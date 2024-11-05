use super::Session;
use crate::did_doc::DidDocument;
use atrium_common::store::SimpleStore;
use std::sync::RwLock;

pub struct Store<S> {
    inner: S,
    pub endpoint: RwLock<String>,
}

impl<S> Store<S>
where
    S: SimpleStore<(), Session>,
{
    pub fn new(inner: S, initial_endpoint: String) -> Self {
        Self { inner, endpoint: RwLock::new(initial_endpoint) }
    }
    pub fn get_endpoint(&self) -> String {
        self.endpoint.read().expect("failed to read endpoint").clone()
    }
    pub fn update_endpoint(&self, did_doc: &DidDocument) {
        if let Some(endpoint) = did_doc.get_pds_endpoint() {
            *self.endpoint.write().expect("failed to write endpoint") = endpoint;
        }
    }
    pub async fn get_session(&self) -> Option<Session> {
        self.inner.get(&()).await.expect("todo")
    }
    pub async fn set_session(&self, session: Session) {
        self.inner.set((), session).await.expect("todo")
    }
    pub async fn clear_session(&self) {
        self.inner.del(&()).await.expect("todo")
    }
}
