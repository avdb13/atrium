use super::HandleResolver;
use crate::error::{Error, Result};
use atrium_api::com::atproto::identity::resolve_handle;
use atrium_api::types::string::{Did, Handle};
use atrium_common::resolver::{self, Resolver};
use atrium_xrpc::http::uri::Builder;
use atrium_xrpc::http::{Request, Uri};
use atrium_xrpc::HttpClient;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppViewHandleResolverConfig<T> {
    pub service_url: String,
    pub http_client: Arc<T>,
}

pub struct AppViewHandleResolver<T> {
    service_url: String,
    http_client: Arc<T>,
}

impl<T> AppViewHandleResolver<T> {
    pub fn new(config: AppViewHandleResolverConfig<T>) -> Self {
        Self { service_url: config.service_url, http_client: config.http_client }
    }
}

impl<T> Resolver<Error> for AppViewHandleResolver<T>
where
    T: HttpClient + Send + Sync + 'static,
{
    type Input = Handle;
    type Output = Did;

    async fn resolve(&self, handle: &Self::Input) -> Result<Option<Self::Output>> {
        let result = || async {
            let uri = Builder::from(self.service_url.parse::<Uri>()?)
                .path_and_query(format!(
                    "/xrpc/com.atproto.identity.resolveHandle?{}",
                    serde_html_form::to_string(resolve_handle::ParametersData {
                        handle: handle.clone(),
                    })?
                ))
                .build()?;
            // TODO: no-cache?
            let res = self
                .http_client
                .send_http(Request::builder().uri(uri).body(Vec::new())?)
                .await
                .map_err(resolver::Error::HttpClient)?;
            if res.status().is_success() {
                Ok(Some(serde_json::from_slice::<resolve_handle::OutputData>(res.body())?.did))
            } else {
                Err(resolver::Error::HttpStatus(res.status()))
            }
        };
        result().await.map_err(Error::Resolver)
    }
}

impl<T> HandleResolver for AppViewHandleResolver<T> where T: HttpClient + Send + Sync + 'static {}
