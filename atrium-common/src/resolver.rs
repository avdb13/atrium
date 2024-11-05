mod cache_impl;
mod cached_resolver;
mod error;
mod throttled_resolver;

pub use self::cached_resolver::{CachedResolver, CachedResolverConfig};
pub use self::error::{Error, Result};
pub use self::throttled_resolver::ThrottledResolver;
use std::future::Future;
use std::hash::Hash;

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub trait Resolver<E>
where
    E: std::error::Error,
{
    type Input: ?Sized;
    type Output;

    fn resolve(
        &self,
        input: &Self::Input,
    ) -> impl Future<Output = std::result::Result<Option<Self::Output>, E>>;
}

pub trait Cacheable<E>
where
    Self: Sized + Resolver<E>,
    Self::Input: Sized,
    E: std::error::Error,
{
    fn cached(self, config: CachedResolverConfig) -> CachedResolver<Self, E>;
}

impl<R, E> Cacheable<E> for R
where
    R: Sized + Resolver<E>,
    R::Input: Sized + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn cached(self, config: CachedResolverConfig) -> CachedResolver<Self, E> {
        CachedResolver::new(self, config)
    }
}

pub trait Throttleable<E>
where
    Self: Sized + Resolver<E>,
    Self::Input: Sized,
    E: std::error::Error,
{
    fn throttled(self) -> ThrottledResolver<Self, E>;
}

impl<R, E> Throttleable<E> for R
where
    R: Sized + Resolver<E>,
    R::Input: Clone + Hash + Eq + Send + Sync + 'static,
    R::Output: Clone + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn throttled(self) -> ThrottledResolver<Self, E> {
        ThrottledResolver::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg(not(target_arch = "wasm32"))]
    async fn sleep(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    #[cfg(target_arch = "wasm32")]
    async fn sleep(duration: Duration) {
        gloo_timers::future::sleep(duration).await;
    }

    struct MockResolver {
        data: HashMap<String, String>,
        counts: Arc<RwLock<HashMap<String, usize>>>,
    }

    impl Resolver<Error> for MockResolver {
        type Input = String;
        type Output = String;

        async fn resolve(&self, input: &Self::Input) -> Result<Option<Self::Output>> {
            sleep(Duration::from_millis(10)).await;
            *self.counts.write().await.entry(input.clone()).or_default() += 1;
            Ok(self.data.get(input).cloned())
        }
    }

    fn mock_resolver(counts: Arc<RwLock<HashMap<String, usize>>>) -> MockResolver {
        MockResolver {
            data: [
                (String::from("k1"), String::from("v1")),
                (String::from("k2"), String::from("v2")),
            ]
            .into_iter()
            .collect(),
            counts,
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn test_no_cached() {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let resolver = mock_resolver(counts.clone());
        for (input, expected) in [
            ("k1", Some("v1")),
            ("k2", Some("v2")),
            ("k2", Some("v2")),
            ("k1", Some("v1")),
            ("k3", None),
            ("k1", Some("v1")),
            ("k3", None),
        ] {
            let result = resolver.resolve(&input.to_string()).await;
            match expected {
                Some(value) => {
                    assert_eq!(result.expect("failed to resolve").as_deref(), Some(value))
                }
                None => assert!(result.is_err()),
            }
        }
        assert_eq!(
            *counts.read().await,
            [(String::from("k1"), 3), (String::from("k2"), 2), (String::from("k3"), 2),]
                .into_iter()
                .collect()
        );
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn test_cached() {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let resolver = mock_resolver(counts.clone()).cached(Default::default());
        for (input, expected) in [
            ("k1", Some("v1")),
            ("k2", Some("v2")),
            ("k2", Some("v2")),
            ("k1", Some("v1")),
            ("k3", None),
            ("k1", Some("v1")),
            ("k3", None),
        ] {
            let result = resolver.resolve(&input.to_string()).await;
            match expected {
                Some(value) => {
                    assert_eq!(result.expect("failed to resolve").as_deref(), Some(value))
                }
                None => assert!(result.is_err()),
            }
        }
        assert_eq!(
            *counts.read().await,
            [(String::from("k1"), 1), (String::from("k2"), 1), (String::from("k3"), 2),]
                .into_iter()
                .collect()
        );
    }

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    async fn test_cached_with_max_capacity() {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let resolver = mock_resolver(counts.clone())
            .cached(CachedResolverConfig { max_capacity: Some(1), ..Default::default() });
        for (input, expected) in [
            ("k1", Some("v1")),
            ("k2", Some("v2")),
            ("k2", Some("v2")),
            ("k1", Some("v1")),
            ("k3", None),
            ("k1", Some("v1")),
            ("k3", None),
        ] {
            let result = resolver.resolve(&input.to_string()).await;
            match expected {
                Some(value) => {
                    assert_eq!(result.expect("failed to resolve").as_deref(), Some(value))
                }
                None => assert!(result.is_err()),
            }
        }
        assert_eq!(
            *counts.read().await,
            [(String::from("k1"), 2), (String::from("k2"), 1), (String::from("k3"), 2),]
                .into_iter()
                .collect()
        );
    }

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    async fn test_cached_with_time_to_live() {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let resolver = mock_resolver(counts.clone()).cached(CachedResolverConfig {
            time_to_live: Some(Duration::from_millis(10)),
            ..Default::default()
        });
        for _ in 0..10 {
            let result = resolver.resolve(&String::from("k1")).await;
            assert_eq!(result.expect("failed to resolve").as_deref(), Some("v1"));
        }
        sleep(Duration::from_millis(10)).await;
        for _ in 0..10 {
            let result = resolver.resolve(&String::from("k1")).await;
            assert_eq!(result.expect("failed to resolve").as_deref(), Some("v1"));
        }
        assert_eq!(*counts.read().await, [(String::from("k1"), 2)].into_iter().collect());
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn test_throttled() {
        let counts = Arc::new(RwLock::new(HashMap::new()));
        let resolver = Arc::new(mock_resolver(counts.clone()).throttled());

        let mut handles = Vec::new();
        for (input, expected) in [
            ("k1", Some("v1")),
            ("k2", Some("v2")),
            ("k2", Some("v2")),
            ("k1", Some("v1")),
            ("k3", None),
            ("k1", Some("v1")),
            ("k3", None),
        ] {
            let resolver = resolver.clone();
            handles.push(async move { (resolver.resolve(&input.to_string()).await, expected) });
        }
        for (result, expected) in futures::future::join_all(handles).await {
            match expected {
                Some(value) => {
                    assert_eq!(result.expect("failed to resolve").as_deref(), Some(value))
                }
                None => assert!(result.is_err()),
            }
        }
        assert_eq!(
            *counts.read().await,
            [(String::from("k1"), 1), (String::from("k2"), 1), (String::from("k3"), 1),]
                .into_iter()
                .collect()
        );
    }
}
