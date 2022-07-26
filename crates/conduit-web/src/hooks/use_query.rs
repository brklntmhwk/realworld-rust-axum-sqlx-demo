use std::{error::Error, future::Future};

use gloo::net::http::Method;
use yew::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum FetchError<E>
where
    E: Error,
{
    Inner(E),
}

#[derive(Default, Debug, Copy, Clone)]
pub struct FetchOptions<'a> {
    pub fetch_key: &'a str,
    pub revalidate_if_stale: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct FetchResult<T, E>
where
    T: Copy,
    E: Error,
{
    pub data: Option<T>,
    pub error: Option<FetchError<E>>,
    pub loading: bool,
}

impl<T, E> Default for FetchResult<T, E>
where
    T: Copy,
    E: Error,
{
    fn default() -> Self {
        Self {
            data: None,
            error: None,
            loading: false,
        }
    }
}

impl<T, E> FetchResult<T, E>
where
    T: Copy,
    E: Error,
{
    pub fn set_result(&mut self, value: T) {
        self.data = Some(value);
    }

    pub fn set_error(&mut self, error: E)
    where
        E: Error,
    {
        self.error = Some(FetchError::Inner(error));
    }
}

impl<'a> FetchOptions<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            fetch_key: key,
            revalidate_if_stale: true,
        }
    }

    pub fn with_revalidation(&mut self, revalidate: bool) -> Self {
        self.revalidate_if_stale = revalidate;
        *self
    }

    pub fn build(self) -> Self {
        Self {
            fetch_key: self.fetch_key,
            revalidate_if_stale: self.revalidate_if_stale,
        }
    }
}

pub fn use_query<T, K, E, F>(key: &str, fetcher: F, mut options: Option<FetchOptions>) -> FetchResult<T, E>
where
    T: for<'a> serde::de::Deserialize<'a> + Copy,
    K: serde::Serialize,
    F: Future<Output = K> + 'static,
    E: Error,
{
    FetchResult::default()
}