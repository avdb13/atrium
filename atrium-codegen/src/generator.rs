use crate::code_writer::CodeWriter;
use crate::fs::find_dirs;
use crate::schema::find_ref_unions;
use crate::token_stream::LexConverter;
use atrium_lex::lexicon::LexUserType;
use atrium_lex::LexiconDoc;
use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const HEADER: &str = "// This file is generated by atrium-codegen. DO NOT EDIT.";

pub(crate) fn generate_schemas(
    schema: &LexiconDoc,
    outdir: &Path,
) -> Result<Vec<impl AsRef<Path>>, Box<dyn Error>> {
    let mut results = Vec::new();
    let mut paths = schema.id.split('.').collect::<Vec<_>>();
    if let Some(basename) = paths.pop() {
        let mut tokens = Vec::new();
        let mut names = Vec::new();
        let converter = LexConverter::new(schema.id.clone());
        // main def
        for (name, def) in &schema.defs {
            if name == "main" {
                tokens.push(converter.convert(basename, def, true)?);
            } else {
                names.push(name);
            }
        }
        // other defs
        for &name in names.iter().sorted() {
            tokens.push(converter.convert(name, &schema.defs[name], false)?);
        }
        // ref unions
        tokens.push(converter.ref_unions(&find_ref_unions(&schema.defs))?);

        let documentation = {
            let doc = format!("Definitions for the `{}` namespace.", schema.id);
            let description = if let Some(description) = &schema.description {
                quote!(#![doc = #description])
            } else {
                quote!()
            };
            quote! {
                #![doc = #doc]
                #description
            }
        };
        let mut filename = PathBuf::from(basename.to_snake_case());
        filename.set_extension("rs");
        let dir = outdir.join(paths.join("/"));
        create_dir_all(&dir)?;
        let path = dir.join(filename);
        let content = syn::parse2::<syn::File>(quote! {
            #documentation
            #(#tokens)*
        })?
        .to_token_stream();
        write_to_file(File::create(&path)?, content)?;
        results.push(path);
    }
    Ok(results)
}

pub(crate) fn generate_records(
    outdir: &Path,
    schemas: &[LexiconDoc],
) -> Result<(), Box<dyn Error>> {
    let records = schemas
        .iter()
        .filter_map(|schema| {
            if let Some(LexUserType::Record(_)) = schema.defs.get("main") {
                Some(schema.id.clone())
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();
    let mut writer = CodeWriter::default();
    writer.write_header(None, None)?;
    writer.write_records(&records)?;
    writer.write_to_file(&outdir.join("records.rs"))?;
    Ok(())
}

pub(crate) fn generate_traits(outdir: &Path, schemas: &[LexiconDoc]) -> Result<(), Box<dyn Error>> {
    let traits = schemas
        .iter()
        .filter_map(|schema| {
            if let Some(LexUserType::XrpcQuery(_) | LexUserType::XrpcProcedure(_)) =
                schema.defs.get("main")
            {
                Some(schema.id.clone())
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();
    let mut writer = CodeWriter::default();
    writer.write_header(None, None)?;
    writer.write_traits_macro(&traits)?;
    writer.write_to_file(&outdir.join("traits.rs"))?;
    Ok(())
}

pub(crate) fn generate_modules(outdir: &Path) -> Result<(), Box<dyn Error>> {
    let paths = find_dirs(outdir)?;
    let mut files = Vec::with_capacity(paths.len());
    // create ".rs" files
    for path in &paths {
        let mut p = path.as_ref().to_path_buf();
        p.set_extension("rs");
        files.push(p);
    }
    // write "mod" statements
    for (path, filepath) in paths.iter().zip(&files) {
        if path.as_ref() == outdir {
            continue;
        }
        let modules = read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .sorted()
            .collect_vec();

        let mut writer = CodeWriter::default();
        writer.write_header(None, None)?;
        writer.write_mods(&modules)?;
        writer.write_to_file(filepath)?;
    }
    Ok(())
}

fn write_to_file(mut file: impl Write, content: TokenStream) -> Result<(), std::io::Error> {
    writeln!(file, "{HEADER}")?;
    writeln!(file, "{content}")?;
    Ok(())
}
