use atrium_api::types::string::Did;
use atrium_common::resolver::Error as ResolverError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("invalid at identifier: {0}")]
    AtIdentifier(String),
    #[error("invalid did: {0}")]
    Did(String),
    #[error("invalid did document: {0}")]
    DidDocument(String),
    #[error("protected resource metadata is invalid: {0}")]
    ProtectedResourceMetadata(String),
    #[error("authorization server metadata is invalid: {0}")]
    AuthorizationServerMetadata(String),
    #[error("unsupported did method: {0:?}")]
    UnsupportedDidMethod(Did),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Identity(#[from] IdentityError),
    #[error(transparent)]
    Resolver(#[from] ResolverError),
}
