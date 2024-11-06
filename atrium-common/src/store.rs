pub mod memory;

use std::error::Error;
use std::future::Future;
use std::hash::Hash;

#[cfg_attr(not(target_arch = "wasm32"), trait_variant::make(Send))]
pub trait SimpleStore<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    type Error: Error;

    fn get(&self, key: &K) -> impl Future<Output = Result<Option<V>, Self::Error>>;
    fn set(&self, key: K, value: V) -> impl Future<Output = Result<(), Self::Error>>;
    fn del(&self, key: &K) -> impl Future<Output = Result<(), Self::Error>>;
    fn clear(&self) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<T> SimpleStore<(), T> for std::sync::Mutex<Option<T>>
where
    T: Clone + Send,
{
    type Error = std::convert::Infallible;

    async fn get(&self, _: &()) -> Result<Option<T>, Self::Error> {
        Ok(self.lock().as_deref().cloned().expect("todo"))
    }

    async fn set(&self, _: (), value: T) -> Result<(), Self::Error> {
        *self.lock().expect("todo") = Some(value);
        Ok(())
    }

    async fn del(&self, _: &()) -> Result<(), Self::Error> {
        self.clear().await.expect("todo");
        Ok(())
    }

    async fn clear(&self) -> Result<(), Self::Error> {
        *self.lock().expect("todo") = None;
        Ok(())
    }
}

impl<K, V> SimpleStore<K, V> for dashmap::DashMap<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    type Error = std::convert::Infallible;

    async fn get(&self, key: &K) -> Result<Option<V>, Self::Error> {
        Ok(self.get(key).as_deref().cloned())
    }

    async fn set(&self, key: K, value: V) -> Result<(), Self::Error> {
        self.insert(key, value);
        Ok(())
    }

    async fn del(&self, key: &K) -> Result<(), Self::Error> {
        self.remove(key);
        Ok(())
    }

    async fn clear(&self) -> Result<(), Self::Error> {
        self.clear();
        Ok(())
    }
}
