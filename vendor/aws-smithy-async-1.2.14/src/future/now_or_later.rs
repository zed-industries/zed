/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Provides the [`NowOrLater`] future with an explicit `Now` variant
//!
//! When a future is immediately, ready, this enables avoiding an unnecessary allocation.
//! This is intended to be used with `Pin<Box<dyn Future>>` or similar as the future variant. For
//! convenience, [`BoxFuture`] is provided for this use case.
//!
//! Typically, this is used when creating a manual async trait. In this case, it's critical that the
//! lifetime is captured to enable interop with the async-trait macro.
//!
//! # Examples
//!
//! ```rust
//! mod future {
//!   use aws_smithy_async::future::now_or_later::{NowOrLater, BoxFuture};
//!   use std::future::Future;
//!   pub struct ProvideRegion<'a>(NowOrLater<Option<String>, BoxFuture<'a, Option<String>>>);
//!   impl<'a> ProvideRegion<'a> {
//!       pub fn new(f: impl Future<Output = Option<String>> + Send + 'a) -> Self {
//!           Self(NowOrLater::new(Box::pin(f)))
//!       }
//!
//!       pub fn ready(region: Option<String>) -> Self {
//!           Self(NowOrLater::ready(region))
//!       }
//!   }
//! }
//!
//! pub trait ProvideRegion {
//!     fn provide_region<'a>(&'a self) -> future::ProvideRegion<'a> where Self: 'a;
//! }
//!
//! struct AsyncRegionProvider;
//! impl AsyncRegionProvider {
//!     async fn region(&self) -> Option<String> {
//!         todo!()
//!     }
//! }
//!
//! impl ProvideRegion for AsyncRegionProvider {
//!     fn provide_region<'a>(&'a self) -> future::ProvideRegion<'a> where Self: 'a {
//!       future::ProvideRegion::new(self.region())
//!     }
//! }
//! ```

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

/// Boxed future type alias
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug)]
/// Zero sized type for using NowOrLater when no future variant exists.
pub enum OnlyReady {}

pin_project! {
    /// Future with an explicit `Now` variant
    ///
    /// See the [module documentation](crate::future::now_or_later) for more information.
    pub struct NowOrLater<T, F> {
        #[pin]
        inner: Inner<T, F>
    }
}

impl<T, F> fmt::Debug for NowOrLater<T, F>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NowOrLater")
            .field("inner", &self.inner)
            .finish()
    }
}

pin_project! {
    #[project = NowOrLaterProj]
    enum Inner<T, F> {
        #[non_exhaustive]
        Now { value: Option<T> },
        #[non_exhaustive]
        Later { #[pin] future: F },
    }
}

impl<T, F> fmt::Debug for Inner<T, F>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Now { value } => f.debug_struct("Now").field("value", value).finish(),
            Self::Later { .. } => f
                .debug_struct("Later")
                .field("future", &"<future>")
                .finish(),
        }
    }
}

impl<T, F> NowOrLater<T, F> {
    /// Creates a future that will resolve when `future` resolves
    pub fn new(future: F) -> Self {
        Self {
            inner: Inner::Later { future },
        }
    }

    /// Creates a future that immediately resolves to `value`
    pub fn ready(value: T) -> NowOrLater<T, F> {
        let value = Some(value);
        Self {
            inner: Inner::Now { value },
        }
    }
}

impl<T, F> Future for NowOrLater<T, F>
where
    F: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().inner.project() {
            NowOrLaterProj::Now { value } => {
                Poll::Ready(value.take().expect("cannot be called twice"))
            }
            NowOrLaterProj::Later { future } => future.poll(cx),
        }
    }
}

impl<T> Future for NowOrLater<T, OnlyReady> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().inner.project() {
            NowOrLaterProj::Now { value } => {
                Poll::Ready(value.take().expect("cannot be called twice"))
            }
            NowOrLaterProj::Later { .. } => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::future::now_or_later::{NowOrLater, OnlyReady};
    use futures_util::FutureExt;

    #[test]
    fn ready_future_immediately_returns() {
        let a = true;
        let f = if a {
            NowOrLater::ready(5)
        } else {
            NowOrLater::new(async { 5 })
        };
        use futures_util::FutureExt;
        assert_eq!(f.now_or_never().expect("future was ready"), 5);
    }

    #[test]
    fn only_ready_instantiation() {
        assert_eq!(
            NowOrLater::<i32, OnlyReady>::ready(5)
                .now_or_never()
                .expect("ready"),
            5
        );
    }

    #[tokio::test]
    async fn box_dyn_future() {
        let f = async { 5 };
        let f = Box::pin(f);
        let wrapped = NowOrLater::new(f);
        assert_eq!(wrapped.await, 5);
    }

    #[tokio::test]
    async fn async_fn_future() {
        let wrapped = NowOrLater::new(async { 5 });
        assert_eq!(wrapped.await, 5);
    }
}
