use super::Resolver;
use dashmap::{DashMap, Entry};
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Sender};
use tokio::sync::Mutex;

type SharedSender<T> = Arc<Mutex<Sender<Option<T>>>>;

pub struct ThrottledResolver<R, E>
where
    R: Resolver<E>,
    R::Input: Sized,
    E: Error,
{
    resolver: R,
    senders: Arc<DashMap<R::Input, SharedSender<R::Output>>>,
}

impl<R, E> ThrottledResolver<R, E>
where
    R: Resolver<E>,
    R::Input: Clone + Hash + Eq + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn new(resolver: R) -> Self {
        Self { resolver, senders: Arc::new(DashMap::new()) }
    }
}

impl<R, E> Resolver<E> for ThrottledResolver<R, E>
where
    R: Resolver<E> + Send + Sync + 'static,
    R::Input: Clone + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    type Input = R::Input;
    type Output = R::Output;

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>, E> {
        match self.senders.entry(input.clone()) {
            Entry::Occupied(occupied) => {
                let tx = occupied.get().lock().await.clone();
                drop(occupied);
                Ok(tx.subscribe().recv().await.expect("recv"))
            }
            Entry::Vacant(vacant) => {
                let (tx, _) = channel(1);
                vacant.insert(Arc::new(Mutex::new(tx.clone())));
                let Some(result) = self.resolver.resolve(input).await.transpose() else {
                    return Ok(None);
                };
                tx.send(result.as_ref().ok().cloned()).ok();
                self.senders.remove(input);
                result.map(Some)
            }
        }
    }
}
