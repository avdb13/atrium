use atrium_api::types::string::Did;
use jose_jwk::Key;
use serde::{Deserialize, Serialize};

use crate::TokenSet;

use super::{memory::MemorySimpleStore, SimpleStore};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Session {
    dpop_key: Key,
    token_set: TokenSet,
}

impl Session {
    pub fn new(dpop_key: Key, token_set: TokenSet) -> Self {
        Self { dpop_key, token_set }
    }
}

pub trait SessionStore: SimpleStore<Did, TokenSet> {}

pub type MemorySessionStore = MemorySimpleStore<Did, TokenSet>;

impl SessionStore for MemorySessionStore {}
