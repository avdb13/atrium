use std::{future::Future, pin::Pin, sync::Arc};

use atrium_common::resolver::{Resolver, ThrottledResolver};
use atrium_xrpc::{HttpClient, XrpcClient};

use crate::{
    client::com::Service,
    com::atproto::server::{create_account, create_session},
    error::Error,
};

use super::Session;

pub type Resolution<'f, T> = Pin<Box<dyn Future<Output = T> + Send + 'f>>;

pub struct SessionResolver;

impl<F, T, E> Resolver<E> for SessionResolver
where
    F: FnOnce(Option<T>) -> Resolution<'static, Result<T, E>> + Send + 'static,
    E: std::error::Error,
{
    type Input = dyn FnOnce(Option<T>) -> Resolution<'static, Result<T, E>> + Send + 'static;
    type Output = ();

    async fn resolve(&self, input: Box<Self::Input>) -> Result<Option<Self::Output>> {
        let ok = input;
    }
}

pub type ThrottledSessionResolver<T> = ThrottledResolver<SessionResolver<T>, Error>;
