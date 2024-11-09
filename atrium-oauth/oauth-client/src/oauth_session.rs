use std::sync::Arc;

use atrium_api::{
    agent::{atp_agent::inner, SessionManager},
    client::com::Service as AtprotoService,
    types::string::Did,
};
use atrium_identity::{did::DidResolver, handle::HandleResolver};
use atrium_xrpc::{
    http::{Request, Response},
    HttpClient, XrpcClient,
};
use thiserror::Error;

use crate::{
    server_agent::OAuthServerAgent,
    store::{AtpStageCellStore, AtpStageCellStore},
    types::stage::{SessionData, Stage},
    TokenSet,
};

#[derive(Clone, Debug, Error)]
pub enum Error {}

pub struct OAuthSession<S, T, D, H>
where
    S: AtpStageCellStore + Send + Sync,
    T: XrpcClient + Send + Sync,
    // S: SessionStore,
    // T: HttpClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    // session_store: S,
    pub server: Arc<OAuthServerAgent<T, D, H>>,
    // pub sub: Did,
    store: Arc<inner::Store<S>>,
    inner: Arc<inner::Client<S, T>>,
    atproto_service: AtprotoService<inner::Client<S, T>>,
}

impl<S, T, D, H> OAuthSession<S, T, D, H>
where
    S: AtpStageCellStore + Send + Sync,
    T: XrpcClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    pub fn new(server: OAuthServerAgent<T, D, H>, sub: String, store: S) -> Self {
        todo!()
        // Self { server: Arc::new(server), sub, session_store }
    }

    pub async fn get_session(
        &self,
        sub: &String,
        _refresh: Option<bool>,
    ) -> crate::Result<SessionData> {
        let Some(Stage::Session(SessionData { dpop_key, token_set })) =
            self.store.get_stage(sub).await
        else {
            todo!()
        };
        let token_set = self.server.refresh_token(token_set).await?;
        Ok(SessionData { dpop_key, token_set })
    }

    // pub async fn get_token_info(&self, refresh: Option<bool>) -> crate::Result<TokenInfo> {
    //     let TokenSet { iss, sub, aud, scope, expires_at, .. } = self.get_token_set(refresh).await?;
    //     let expires_at = expires_at.as_ref().map(AsRef::as_ref).cloned();

    //     Ok(TokenInfo::new(iss, sub.parse().expect("valid Did"), aud, scope, expires_at))
    // }

    // pub async fn logout(&self, _refresh: Option<bool>) -> crate::Result<()> {
    //     let token_set = self.get_session(&"todo".to_owned(), Some(false)).await?;

    //     self.server.revoke(&token_set.access_token).await?;

    //     let _ = self.session_store.del(&self.sub).await;
    //     Ok(())
    // }
}

impl<S, T, D, H> HttpClient for OAuthSession<S, T, D, H>
where
    S: AtpStageCellStore + Send + Sync + 'static,
    T: XrpcClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    async fn send_http(
        &self,
        request: Request<Vec<u8>>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.inner.send_http(request).await
    }
}

impl<S, T, D, H> XrpcClient for OAuthSession<S, T, D, H>
where
    S: AtpStageCellStore + Send + Sync + 'static,
    T: XrpcClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    fn base_uri(&self) -> String {
        todo!()
    }
}

impl<S, T, D, H> SessionManager for OAuthSession<S, T, D, H>
where
    S: AtpStageCellStore + Send + Sync + 'static,
    T: XrpcClient + Send + Sync + 'static,
    D: DidResolver + Send + Sync + 'static,
    H: HandleResolver + Send + Sync + 'static,
{
    async fn did(&self) -> Option<Did> {
        self.store.get_stage().await.map(|session| session.data.did)
    }
}
