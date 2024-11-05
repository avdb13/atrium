use super::DidResolver;
use crate::error::{Error, Result};
use crate::IdentityError;
use atrium_api::did_doc::DidDocument;
use atrium_api::types::string::Did;
use atrium_common::resolver::{self, Resolver};
use atrium_xrpc::http::{header::ACCEPT, Request, Uri};
use atrium_xrpc::HttpClient;
use std::sync::Arc;

const DID_WEB_PREFIX: &str = "did:web:";

#[derive(Clone, Debug)]
pub struct WebDidResolverConfig<T> {
    pub http_client: Arc<T>,
}

pub struct WebDidResolver<T> {
    http_client: Arc<T>,
}

impl<T> WebDidResolver<T> {
    pub fn new(config: WebDidResolverConfig<T>) -> Self {
        Self { http_client: config.http_client }
    }
}

impl<T> Resolver<Error> for WebDidResolver<T>
where
    T: HttpClient + Send + Sync + 'static,
{
    type Input = Did;
    type Output = DidDocument;

    async fn resolve(&self, did: &Self::Input) -> Result<Option<Self::Output>> {
        let location = did
            .as_str()
            .strip_prefix(DID_WEB_PREFIX)
            .ok_or_else(|| IdentityError::Did(did.as_str().to_string()))?;
        let result = || async {
            let document_url =
                format!("https://{location}/.well-known/did.json",).parse::<Uri>()?;
            let res = self
                .http_client
                .send_http(
                    Request::builder()
                        .header(ACCEPT, "application/did+ld+json,application/json")
                        .uri(document_url)
                        .body(Vec::new())?,
                )
                .await
                .map_err(resolver::Error::HttpClient)?;
            if res.status().is_success() {
                Ok(Some(serde_json::from_slice::<DidDocument>(res.body())?))
            } else {
                Err(resolver::Error::HttpStatus(res.status()))
            }
        };
        result().await.map_err(Error::Resolver)
    }
}

impl<T> DidResolver for WebDidResolver<T> where T: HttpClient + Send + Sync + 'static {}
