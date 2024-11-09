use atrium_api::types::string::Did;
use atrium_common::store::{memory::MemoryMapStore, MapStore};
use jose_jwk::Key;
use serde::{Deserialize, Serialize};

use crate::TokenSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalStateData {
    pub iss: String,
    pub dpop_key: Key,
    pub verifier: String,
}

pub trait StateStore: MapStore<String, InternalStateData> {}

pub type MemoryStateStore = MemoryMapStore<String, InternalStateData>;

impl StateStore for MemoryStateStore {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub dpop_key: Key,
    pub token_set: TokenSet,
}

pub trait SessionStore: MapStore<Did, Session> {}

pub type MemorySessionStore = MemoryMapStore<Did, Session>;

impl SessionStore for MemorySessionStore {}
