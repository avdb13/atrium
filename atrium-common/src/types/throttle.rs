use std::collections::hash_map::Entry;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Sender};

use crate::resolver::Resolver;

pub trait Throttleable
where
    Self: Sized + Resolver,
    Self::Input: Sized,
{
    fn throttled(self) -> Throttled<Self, Self::Error>;
}

impl<R> Throttleable for R
where
    R: Resolver,
    R::Input: Clone + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    R::Error: std::error::Error + Send + Sync + 'static,
{
    fn throttled(self) -> Throttled<Self, Self::Error> {
        Throttled::new(self, Default::default())
    }
}

type SharedSender<T> = Arc<Mutex<Sender<Option<T>>>>;

pub struct Throttled<T, S> {
    inner: T,
    pending: Arc<S>,
}

// pub type ThrottledResolver<S> = Throttled<R, S>;

impl<T, S> Throttled<T, S>
where
    S: Default,
{
    pub fn new(inner: T) -> Self {
        Self { inner, pending: Arc::new(S::default()) }
    }
}

// impl<'a, K, V> Enterable<'a, K, V> for DashMap<T, S>
// where
//     T: Resolver + Send + Sync,
//     T::Input: Clone + Hash + Eq + Send + Sync,
//     T::Output: Clone + Send + Sync,
//     T::Error: Error + Send + Sync,
// {
// }

impl<T> Resolver for Throttled<T, dashmap::DashMap<(), SharedSender<()>>>
where
    T::Input: Send + Sync + Clone + Hash + Eq,
    T::Output: Send + Sync + Clone,
    T::Error: Send + Sync + std::error::Error,
    T: Send + Sync + Resolver<T::Input, T::Output>,
{
    type Input = T::Input;
    type Output = T::Output;
    type Error = T::Error;

    async fn resolve(&self, input: &()) -> Result<Option<Self::Output>, Self::Error> {
        match self.pending.entry(input.clone()).await {
            dashmap::Entry::Occupied(occupied) => {
                let tx = occupied.get().lock().await.clone();
                drop(occupied);
                match tx.subscribe().recv().await.expect("recv") {
                    Some(result) => Ok(Entry::Occupied(result)),
                    None => Err(Error::NotFound),
                }
            }
            dashmap::Entry::Vacant(vacant) => {
                let (tx, _) = channel(1);
                vacant.insert(Arc::new(std::sync::Mutex::new(tx.clone())));
                let result = self.inner.resolve(input).await;
                tx.send(result.as_ref().ok().cloned()).ok();
                self.senders.remove(input);
                result
            }
        }
    }
}
