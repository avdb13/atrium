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

pub struct CachedResolver<R, E>
where
    R: Resolver<E>,
    R::Input: Sized,
    E: Error,
{
    resolver: R,
    cache: CacheImpl<R::Input, R::Output>,
}

impl<R, E> CachedResolver<R, E>
where
    R: Resolver<E>,
    R::Input: Sized + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn new(resolver: R, config: CachedResolverConfig) -> Self {
        Self { resolver, cache: CacheImpl::new(config) }
    }
}

impl<R, E> Resolver<E> for CachedResolver<R, E>
where
    R: Resolver<E> + Send + Sync + 'static,
    R::Input: Clone + Hash + Eq + Send + Sync + 'static + Debug,
    R::Output: Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    type Input = R::Input;
    type Output = R::Output;

    async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>, E> {
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
