use super::HandleResolver;
use crate::error::{Error, Result};
use crate::IdentityError;
use atrium_api::types::string::{Did, Handle};
use atrium_common::resolver::{self, Resolver};
use atrium_xrpc::http::Request;
use atrium_xrpc::HttpClient;
use std::sync::Arc;

const WELL_KNWON_PATH: &str = "/.well-known/atproto-did";

#[derive(Clone, Debug)]
pub struct WellKnownHandleResolverConfig<T> {
    pub http_client: Arc<T>,
}

pub struct WellKnownHandleResolver<T> {
    http_client: Arc<T>,
}

impl<T> WellKnownHandleResolver<T> {
    pub fn new(config: WellKnownHandleResolverConfig<T>) -> Self {
        Self { http_client: config.http_client }
    }
}

impl<T> Resolver<Error> for WellKnownHandleResolver<T>
where
    T: HttpClient + Send + Sync + 'static,
{
    type Input = Handle;
    type Output = Did;

    async fn resolve(&self, handle: &Self::Input) -> Result<Option<Self::Output>> {
        let result = || async {
            let url = format!("https://{}{WELL_KNWON_PATH}", handle.as_str());
            // TODO: no-cache?
            let res = self
                .http_client
                .send_http(Request::builder().uri(url).body(Vec::new())?)
                .await
                .map_err(resolver::Error::HttpClient)?;
            if res.status().is_success() {
                Ok(String::from_utf8_lossy(res.body()).to_string())
            } else {
                Err(resolver::Error::HttpStatus(res.status()))
            }
        };
        let text = result().await.map_err(Error::Resolver)?;

        Ok(Some(text.parse::<Did>().map_err(|e| IdentityError::Did(e.to_string()))?))
    }
}

impl<T> HandleResolver for WellKnownHandleResolver<T> where T: HttpClient + Send + Sync + 'static {}
