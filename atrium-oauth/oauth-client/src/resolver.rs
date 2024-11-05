mod oauth_authorization_server_resolver;
mod oauth_protected_resource_resolver;

use self::oauth_authorization_server_resolver::DefaultOAuthAuthorizationServerResolver;
use self::oauth_protected_resource_resolver::DefaultOAuthProtectedResourceResolver;
use crate::types::{OAuthAuthorizationServerMetadata, OAuthProtectedResourceMetadata};
use atrium_common::resolver::{
    self, Cacheable, CachedResolver, CachedResolverConfig, Resolver, Throttleable,
    ThrottledResolver,
};
use atrium_identity::identity_resolver::{
    IdentityResolver, IdentityResolverConfig, ResolvedIdentity,
};
use atrium_identity::{did::DidResolver, handle::HandleResolver};
use atrium_identity::{Error, IdentityError, Result};
use atrium_xrpc::HttpClient;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct OAuthAuthorizationServerMetadataResolverConfig {
    pub cache: CachedResolverConfig,
}

impl Default for OAuthAuthorizationServerMetadataResolverConfig {
    fn default() -> Self {
        Self {
            cache: CachedResolverConfig {
                max_capacity: Some(100),
                time_to_live: Some(Duration::from_secs(60)),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuthProtectedResourceMetadataResolverConfig {
    pub cache: CachedResolverConfig,
}

impl Default for OAuthProtectedResourceMetadataResolverConfig {
    fn default() -> Self {
        Self {
            cache: CachedResolverConfig {
                max_capacity: Some(100),
                time_to_live: Some(Duration::from_secs(60)),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuthResolverConfig<D, H> {
    pub did_resolver: D,
    pub handle_resolver: H,
    pub authorization_server_metadata: OAuthAuthorizationServerMetadataResolverConfig,
    pub protected_resource_metadata: OAuthProtectedResourceMetadataResolverConfig,
}

pub struct OAuthResolver<
    T,
    D,
    H,
    PR = DefaultOAuthProtectedResourceResolver<T>,
    AS = DefaultOAuthAuthorizationServerResolver<T>,
> where
    PR: Resolver<Error, Input = String, Output = OAuthProtectedResourceMetadata>
        + Send
        + Sync
        + 'static,
    AS: Resolver<Error, Input = String, Output = OAuthAuthorizationServerMetadata>
        + Send
        + Sync
        + 'static,
{
    identity_resolver: IdentityResolver<D, H>,
    protected_resource_resolver: CachedResolver<ThrottledResolver<PR, Error>, Error>,
    authorization_server_resolver: CachedResolver<ThrottledResolver<AS, Error>, Error>,
    _phantom: PhantomData<T>,
}

impl<T, D, H> OAuthResolver<T, D, H>
where
    T: HttpClient + Send + Sync + 'static,
{
    pub fn new(config: OAuthResolverConfig<D, H>, http_client: Arc<T>) -> Self {
        let protected_resource_resolver =
            DefaultOAuthProtectedResourceResolver::new(http_client.clone())
                .throttled()
                .cached(config.authorization_server_metadata.cache);
        let authorization_server_resolver =
            DefaultOAuthAuthorizationServerResolver::new(http_client.clone())
                .throttled()
                .cached(config.protected_resource_metadata.cache);
        Self {
            identity_resolver: IdentityResolver::new(IdentityResolverConfig {
                did_resolver: config.did_resolver,
                handle_resolver: config.handle_resolver,
            }),
            protected_resource_resolver,
            authorization_server_resolver,
            _phantom: PhantomData,
        }
    }
}

impl<T, D, H> OAuthResolver<T, D, H>
where
    T: HttpClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    pub async fn get_authorization_server_metadata(
        &self,
        issuer: impl AsRef<str>,
    ) -> Result<OAuthAuthorizationServerMetadata> {
        self.authorization_server_resolver
            .resolve(&issuer.as_ref().to_string())
            .await
            .and_then(|res| res.ok_or_else(|| resolver::Error::NotFound.into()))
    }
    async fn resolve_from_service(&self, input: &str) -> Result<OAuthAuthorizationServerMetadata> {
        // Assume first that input is a PDS URL (as required by ATPROTO)
        if let Ok(metadata) = self.get_resource_server_metadata(input).await {
            return Ok(metadata);
        }
        // Fallback to trying to fetch as an issuer (Entryway)
        self.get_authorization_server_metadata(input).await
    }
    pub(crate) async fn resolve_from_identity(
        &self,
        input: &str,
    ) -> Result<(OAuthAuthorizationServerMetadata, ResolvedIdentity)> {
        let Some(identity) = self.identity_resolver.resolve(input).await? else {
            return Err(resolver::Error::NotFound.into());
        };
        let metadata = self.get_resource_server_metadata(&identity.pds).await?;
        Ok((metadata, identity))
    }
    async fn get_resource_server_metadata(
        &self,
        pds: &str,
    ) -> Result<OAuthAuthorizationServerMetadata> {
        let Some(rs_metadata) = self.protected_resource_resolver.resolve(&pds.to_string()).await?
        else {
            return Err(resolver::Error::NotFound.into());
        };
        // ATPROTO requires one, and only one, authorization server entry
        // > That document MUST contain a single item in the authorization_servers array.
        // https://github.com/bluesky-social/proposals/tree/main/0004-oauth#server-metadata
        let issuer = match &rs_metadata.authorization_servers {
            Some(servers) if !servers.is_empty() => {
                if servers.len() > 1 {
                    return Err(IdentityError::ProtectedResourceMetadata(format!(
                        "unable to determine authorization server for PDS: {pds}"
                    ))
                    .into());
                }
                &servers[0]
            }
            _ => {
                return Err(IdentityError::ProtectedResourceMetadata(format!(
                    "no authorization server found for PDS: {pds}"
                ))
                .into())
            }
        };
        let as_metadata = self.get_authorization_server_metadata(issuer).await?;
        // https://datatracker.ietf.org/doc/html/draft-ietf-oauth-resource-metadata-08#name-authorization-server-metada
        if let Some(protected_resources) = &as_metadata.protected_resources {
            if !protected_resources.contains(&rs_metadata.resource) {
                return Err(IdentityError::AuthorizationServerMetadata(format!(
                    "pds {pds} does not protected by issuer: {issuer}",
                ))
                .into());
            }
        }

        // TODO: atproot specific validation?
        // https://github.com/bluesky-social/proposals/tree/main/0004-oauth#server-metadata
        //
        // eg.
        // https://drafts.aaronpk.com/draft-parecki-oauth-client-id-metadata-document/draft-parecki-oauth-client-id-metadata-document.html
        // if as_metadata.client_id_metadata_document_supported != Some(true) {
        //     return Err(Error::AuthorizationServerMetadata(format!(
        //         "authorization server does not support client_id_metadata_document: {issuer}"
        //     )));
        // }

        Ok(as_metadata)
    }
}

impl<T, D, H> Resolver<Error> for OAuthResolver<T, D, H>
where
    T: HttpClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    type Input = str;
    type Output = (OAuthAuthorizationServerMetadata, Option<ResolvedIdentity>);

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
        // Allow using an entryway, or PDS url, directly as login input (e.g.
        // when the user forgot their handle, or when the handle does not
        // resolve to a DID)
        Ok(Some(if input.starts_with("https://") {
            (self.resolve_from_service(input.as_ref()).await?, None)
        } else {
            let (metadata, identity) = self.resolve_from_identity(input).await?;
            (metadata, Some(identity))
        }))
    }
}
