use crate::future::Join;
use crate::future::Race;
use core::future::IntoFuture;
use futures_core::Future;

use super::join::tuple::Join2;
use super::race::tuple::Race2;
use super::WaitUntil;

/// An extension trait for the `Future` trait.
pub trait FutureExt: Future {
    /// Wait for both futures to complete.
    fn join<S2>(self, other: S2) -> Join2<Self, S2::IntoFuture>
    where
        Self: Future + Sized,
        S2: IntoFuture;

    /// Wait for the first future to complete.
    fn race<T, S2>(self, other: S2) -> Race2<T, Self, S2::IntoFuture>
    where
        Self: Future<Output = T> + Sized,
        S2: IntoFuture<Output = T>;

    /// Delay resolving the future until the given deadline.
    ///
    /// The underlying future will not be polled until the deadline has expired. In addition
    /// to using a time source as a deadline, any future can be used as a
    /// deadline too. When used in combination with a multi-consumer channel,
    /// this method can be used to synchronize the start of multiple futures and streams.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(miri)]fn main() {}
    /// # #[cfg(not(miri))]
    /// # fn main() {
    /// use async_io::Timer;
    /// use futures_concurrency::prelude::*;
    /// use futures_lite::future::block_on;
    /// use std::time::{Duration, Instant};
    ///
    /// block_on(async {
    ///     let now = Instant::now();
    ///     let duration = Duration::from_millis(100);
    ///
    ///     async { "meow" }
    ///         .wait_until(Timer::after(duration))
    ///         .await;
    ///
    ///     assert!(now.elapsed() >= duration);
    /// });
    /// # }
    /// ```
    fn wait_until<D>(self, deadline: D) -> WaitUntil<Self, D::IntoFuture>
    where
        Self: Sized,
        D: IntoFuture,
    {
        WaitUntil::new(self, deadline.into_future())
    }
}

impl<F1> FutureExt for F1
where
    F1: Future,
{
    fn join<F2>(self, other: F2) -> Join2<Self, F2::IntoFuture>
    where
        Self: Future + Sized,
        F2: IntoFuture,
    {
        Join::join((self, other))
    }

    fn race<T, S2>(self, other: S2) -> Race2<T, Self, S2::IntoFuture>
    where
        Self: Future<Output = T> + Sized,
        S2: IntoFuture<Output = T>,
    {
        Race::race((self, other))
    }
}
