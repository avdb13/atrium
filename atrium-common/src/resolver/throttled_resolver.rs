use crate::store::SimpleStore;

use super::Resolver;
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Sender};
use tokio::sync::Mutex;

type SharedSender<T> = Arc<Mutex<Sender<Option<T>>>>;

pub struct ThrottledResolver<R, S> {
    resolver: R,
    senders: Arc<S>,
}

impl<R, S> ThrottledResolver<R, S> {
    pub fn new(resolver: R, senders: S) -> Self {
        Self { resolver, senders: Arc::new(senders) }
    }
}

impl<R, S> Resolver for ThrottledResolver<R, S>
where
    R: Resolver + Send + Sync,
    R::Input: Clone + Hash + Eq + Send + Sync,
    R::Output: Clone + Send + Sync,
    R::Error: Error + Send + Sync,
    S: SimpleStore<R::Input, SharedSender<R::Output>> + Sync,
{
    type Input = R::Input;
    type Output = R::Output;
    type Error = R::Error;

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>, Self::Error> {
        match self.senders.get(input).await? {
            Some(occupied) => {
                let tx = occupied.lock().await.clone();
                drop(occupied);
                Ok(tx.subscribe().recv().await.expect("recv"))
            }
            None => {
                let (tx, _) = channel(1);
                self.senders.set(input.clone(), Arc::new(Mutex::new(tx.clone())));
                let Some(result) = self.resolver.resolve(input).await.transpose() else {
                    return Ok(None);
                };
                tx.send(result.as_ref().ok().cloned()).ok();
                self.senders.del(input);
                result.map(Some)
            }
        }
    }
}
