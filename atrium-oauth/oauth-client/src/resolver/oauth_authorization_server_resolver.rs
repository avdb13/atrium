use crate::types::OAuthAuthorizationServerMetadata;
use atrium_common::resolver::{self, Resolver};
use atrium_identity::{Error, IdentityError, Result};
use atrium_xrpc::http::uri::Builder;
use atrium_xrpc::http::{Request, StatusCode, Uri};
use atrium_xrpc::HttpClient;
use std::sync::Arc;

pub struct DefaultOAuthAuthorizationServerResolver<T> {
    http_client: Arc<T>,
}

impl<T> DefaultOAuthAuthorizationServerResolver<T> {
    pub fn new(http_client: Arc<T>) -> Self {
        Self { http_client }
    }
}

impl<T> Resolver<Error> for DefaultOAuthAuthorizationServerResolver<T>
where
    T: HttpClient + Send + Sync + 'static,
{
    type Input = String;
    type Output = OAuthAuthorizationServerMetadata;

    async fn resolve(&self, issuer: &Self::Input) -> Result<Option<Self::Output>> {
        let result = || async {
            let uri = Builder::from(issuer.parse::<Uri>()?)
                .path_and_query("/.well-known/oauth-authorization-server")
                .build()?;
            let res = self
                .http_client
                .send_http(Request::builder().uri(uri).body(Vec::new())?)
                .await
                .map_err(resolver::Error::HttpClient)?;
            // https://datatracker.ietf.org/doc/html/rfc8414#section-3.2
            if res.status() == StatusCode::OK {
                Ok(serde_json::from_slice::<OAuthAuthorizationServerMetadata>(res.body())?)
            } else {
                Err(resolver::Error::HttpStatus(res.status()))
            }
        };
        let metadata = result().await.map_err(Error::Resolver)?;

        // https://datatracker.ietf.org/doc/html/rfc8414#section-3.3
        if &metadata.issuer == issuer {
            Ok(Some(metadata))
        } else {
            Err(IdentityError::AuthorizationServerMetadata(format!(
                "invalid issuer: {}",
                metadata.issuer
            ))
            .into())
        }
    }
}
