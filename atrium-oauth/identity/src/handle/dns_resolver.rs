use super::HandleResolver;
use crate::{
    error::{Error, Result},
    IdentityError,
};
use atrium_api::types::string::{Did, Handle};
use atrium_common::resolver::{self, Resolver};
use std::future::Future;

const SUBDOMAIN: &str = "_atproto";
const PREFIX: &str = "did=";

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub trait DnsTxtResolver {
    fn resolve(
        &self,
        query: &str,
    ) -> impl Future<
        Output = core::result::Result<
            Vec<String>,
            Box<dyn std::error::Error + Send + Sync + 'static>,
        >,
    >;
}

#[derive(Clone, Debug)]
pub struct DnsHandleResolverConfig<R> {
    pub dns_txt_resolver: R,
}

pub struct DnsHandleResolver<R> {
    dns_txt_resolver: R,
}

impl<R> DnsHandleResolver<R> {
    pub fn new(config: DnsHandleResolverConfig<R>) -> Self {
        Self { dns_txt_resolver: config.dns_txt_resolver }
    }
}

impl<R> Resolver<Error> for DnsHandleResolver<R>
where
    R: DnsTxtResolver + Send + Sync + 'static,
{
    type Input = Handle;
    type Output = Did;

    async fn resolve(&self, handle: &Self::Input) -> Result<Option<Self::Output>> {
        for result in self
            .dns_txt_resolver
            .resolve(&format!("{SUBDOMAIN}.{}", handle.as_ref()))
            .await
            .map_err(resolver::Error::DnsResolver)?
        {
            if let Some(did) = result.strip_prefix(PREFIX) {
                return Ok(Some(
                    did.parse::<Did>().map_err(|e| IdentityError::Did(e.to_string()))?,
                ));
            }
        }
        Ok(None)
    }
}

impl<R> HandleResolver for DnsHandleResolver<R> where R: DnsTxtResolver + Send + Sync + 'static {}
