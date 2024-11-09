use std::hash::Hasher;

use jose_jwk::Key;
use serde::{Deserialize, Serialize};

use crate::TokenSet;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stage {
    Authorization(AuthorizationData),
    Session(SessionData),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationData {
    pub iss: String,
    pub dpop_key: Key,
    pub verifier: String,
}

impl From<AuthorizationData> for Stage {
    fn from(authorization_data: AuthorizationData) -> Self {
        Stage::Authorization(authorization_data)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionData {
    pub dpop_key: Key,
    pub token_set: TokenSet,
}

impl From<SessionData> for Stage {
    fn from(session_data: SessionData) -> Self {
        Stage::Session(session_data)
    }
}
