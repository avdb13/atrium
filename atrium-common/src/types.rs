mod cache;
mod throttle;

use std::collections::hash_map::Entry;
use std::future::Future;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Sender};

use crate::resolver::Resolver;

pub type Resolution<'res, T> = Pin<Box<dyn Future<Output = T> + Send + 'res>>;

pub trait Enterable<'a, K, V> {
    fn entry(&'a mut self, key: K) -> impl Future<Output = Entry<'a, K, V>> + Send;
}

pub enum Entry<T, V> {
    Occupied(T),
    Vacant(V),
}
