use super::cache_impl::CacheImpl;
use super::Resolver;
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::Duration;

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub(crate) trait Cache {
    type Input: Hash + Eq + Sync + 'static;
    type Output: Clone + Sync + 'static;

    fn new(config: CachedResolverConfig) -> Self;
    async fn get(&self, key: &Self::Input) -> Option<Self::Output>;
    async fn set(&self, key: Self::Input, value: Self::Output);
}

#[derive(Clone, Debug, Default)]
pub struct CachedResolverConfig {
    pub max_capacity: Option<u64>,
    pub time_to_live: Option<Duration>,
}

pub struct CachedResolver<R>
where
    R: Resolver,
    R::Input: Sized,
{
    resolver: R,
    cache: CacheImpl<R::Input, R::Output>,
}

impl<R> CachedResolver<R>
where
    R: Resolver,
    R::Input: Sized + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    R::Error: Error,
{
    pub fn new(resolver: R, config: CachedResolverConfig) -> Self {
        Self { resolver, cache: CacheImpl::new(config) }
    }
}

impl<R> Resolver for CachedResolver<R>
where
    R: Resolver + Sync,
    R::Input: Sized + Clone + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    R::Error: Error,
{
    type Input = R::Input;
    type Output = R::Output;
    type Error = R::Error;

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>, Self::Error> {
        if let Some(output) = self.cache.get(input).await {
            return Ok(Some(output));
        }
        let Some(output) = self.resolver.resolve(input).await? else {
            return Ok(None);
        };
        self.cache.set(input.clone(), output.clone()).await;
        Ok(Some(output))
    }
}
