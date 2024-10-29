use std::{
    error::Error,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, FixedOffset, Utc};
use tokio::sync::broadcast;

pub type Getter<'f, T> = Pin<Box<dyn Future<Output = T> + Send + 'f>>;

pub struct CachedStore<S>
where
// S: SimpleStore<K, Cached<V, E>>,
// K: Clone + Eq + Hash,
// V: Expired + Clone + Send + Sync + 'static,
// E: Error + Clone + Send + Sync + 'static,
{
    pub store: S,
}

impl<S> Default for CachedStore<S>
where
    S: Default,
{
    fn default() -> Self {
        Self { store: Default::default() }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Cached<T, E>(Arc<Mutex<Locked<T, E>>>)
where
    T: Expired + Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static;

#[derive(Clone, Debug)]
pub struct Locked<T, E>
where
    T: Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    inner: Option<T>,
    pending: Option<broadcast::Sender<Result<T, E>>>,
}

impl<T, E> Default for Locked<T, E>
where
    T: Expired + Clone + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    fn default() -> Self {
        Self { inner: None, pending: None }
    }
}

impl<T, E> Cached<T, E>
where
    T: Expired + Clone + Send + Sync + 'static,
    E: Error + Clone + Send + Sync + 'static,
{
    pub fn new(inner: T) -> Self {
        Cached(Arc::new(Mutex::new(Locked { inner: Some(inner), ..Default::default() })))
    }

    pub async fn get_cached<G>(&self, getter: G) -> Result<T, E>
    where
        G: FnOnce() -> Getter<'static, Result<T, E>> + Send + 'static,
    {
        let mut rx = {
            let mut _self = self.0.lock().unwrap();

            if let Some(value) = _self.inner.as_ref() {
                if value.expires_at().map_or(true, |expires_at| Utc::now() <= expires_at.to_utc()) {
                    return Ok(value.clone());
                }
            }

            if let Some(pending) = _self.pending.as_ref() {
                pending.subscribe()
            } else {
                let (tx, rx) = broadcast::channel::<Result<T, _>>(1);
                _self.pending = Some(tx.clone());
                let cloned = self.0.clone();

                let fut = getter();

                tokio::spawn(async move {
                    let response = fut.await;

                    {
                        let mut _self = cloned.lock().unwrap();
                        _self.pending = None;

                        match response {
                            Ok(value) => {
                                _self.inner.replace(value.clone());

                                let _ = tx.send(Ok(value));
                            }
                            Err(_error) => {
                                // let _ = tx.send(Err(error));
                            }
                        };
                    }
                });

                rx
            }
        };

        Ok(rx.recv().await.unwrap().unwrap())
    }
}

pub trait Expired {
    fn expires_at(&self) -> Option<DateTime<FixedOffset>>;
}
