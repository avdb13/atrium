#[cfg(not(target_arch = "wasm32"))]
mod moka;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use self::moka::MokaCache as CacheImpl;
#[cfg(target_arch = "wasm32")]
pub use self::wasm::WasmCache as CacheImpl;

use std::error::Error;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use std::{collections::hash_map::Entry, time::Duration};
use tokio::sync::broadcast::{channel, Sender};

use crate::resolver::Resolver;

#[derive(Clone, Debug, Default)]
pub struct CacheConfig {
    pub max_capacity: Option<u64>,
    pub time_to_live: Option<Duration>,
}

pub trait Cacheable
where
    Self: Sized + Resolver,
    Self::Input: Sized,
    Self::Error: Error,
{
    type Input: ?Sized;
    type Output;
    type Error;

    fn cached(self, config: CacheConfig) -> Cached<Self>;
}

impl<T> Cacheable for T
where
    T: Resolver,
    T::Input: Sized + Hash + Eq,
    T::Output: Clone,
    T::Error: Error,
    // T: Sized + Resolver,
    // T::Input: Sized + Hash + Eq + Send + Sync + 'static,
    // T::Output: Clone + Send + Sync + 'static,
    // T::Error: Error + Send + Sync + 'static,
{
    type Input = T::Input;
    type Output = T::Output;
    type Error = T::Error;

    fn cached(self, config: CacheConfig) -> Cached<Self> {
        Cached::new(self, config)
    }
}

pub struct Cached<T, K, V>
// where
//     R: Resolver,
//     R::Input: Sized,
{
    inner: T,
    cache: CacheImpl<K, V>,
}

impl<T, K, V> Cached<T, K, V>
// where
//     R: Resolver,
//     R::Input: Sized + Hash + Eq + Send + Sync + 'static,
//     R::Output: Clone + Send + Sync + 'static,
//     R::Error: Error,
{
    pub fn new(inner: T, config: CacheConfig) -> Self {
        Self { inner, cache: CacheImpl::new(config) }
    }
    // R: Send + FnOnce(Option<K>) -> Resolution<'static, Result<V, E>> + 'static,
}

impl<T> Resolver for Cached<T>
where
    T: Resolver + Sync,
    T::Input: Sized + Clone + Hash + Eq + Send + Sync + 'static,
    T::Output: Clone + Send + Sync + 'static,
    T::Error: Error,
{
    type Input = T::Input;
    type Output = T::Output;
    type Error = T::Error;

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>, Self::Error> {
        // let output = self.resolver.resolve(input).await?;
        // self.cache.set(input.clone(), output.clone()).await;
        // Ok(output)

        match dashmap::DashMap::new().entry(input.clone()) {
            dashmap::Entry::Occupied(occupied) => {
                Ok(Some(*occupied))
            }
            dashmap::Entry::Vacant(vacant) => {
                vacant.insert(input.clone(), );
                let result = self.resolver.resolve(input).await;
                tx.send(result.as_ref().ok().cloned()).ok();
                self.senders.remove(input);
                result
            }
        }
    }
}
