use atrium_api::types::string::Did;

use crate::TokenSet;

use super::{memory::MemorySimpleStore, SimpleStore};

pub trait SessionStore: SimpleStore<Did, TokenSet> {}

pub type MemorySessionStore = MemorySimpleStore<Did, TokenSet>;

impl SessionStore for MemorySessionStore {}
