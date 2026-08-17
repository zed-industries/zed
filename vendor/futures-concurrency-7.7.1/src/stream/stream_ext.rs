use core::future::IntoFuture;

use crate::stream::{IntoStream, Merge};
use futures_core::Stream;

#[cfg(feature = "alloc")]
use crate::concurrent_stream::FromStream;

use super::{chain::tuple::Chain2, merge::tuple::Merge2, zip::tuple::Zip2, Chain, WaitUntil, Zip};

/// An extension trait for the `Stream` trait.
pub trait StreamExt: Stream {
    /// Combines two streams into a single stream of all their outputs.
    fn merge<T, S2>(self, other: S2) -> Merge2<T, Self, S2::IntoStream>
    where
        Self: Stream<Item = T> + Sized,
        S2: IntoStream<Item = T>;

    /// Takes two streams and creates a new stream over all in sequence
    fn chain<T, S2>(self, other: S2) -> Chain2<Self, S2::IntoStream>
    where
        Self: Stream<Item = T> + Sized,
        S2: IntoStream<Item = T>;

    /// ‘Zips up’ multiple streams into a single stream of pairs.
    fn zip<T, S2>(self, other: S2) -> Zip2<Self, S2::IntoStream>
    where
        Self: Stream<Item = T> + Sized,
        S2: IntoStream<Item = T>;

    /// Convert into a concurrent stream.
    #[cfg(feature = "alloc")]
    fn co(self) -> FromStream<Self>
    where
        Self: Sized,
    {
        FromStream::new(self)
    }

    /// Delay the yielding of items from the stream until the given deadline.
    ///
    /// The underlying stream will not be polled until the deadline has expired. In addition
    /// to using a time source as a deadline, any future can be used as a
    /// deadline too. When used in combination with a multi-consumer channel,
    /// this method can be used to synchronize the start of multiple streams and futures.
    ///
    /// # Example
    /// ```
    /// # #[cfg(miri)] fn main() {}
    /// # #[cfg(not(miri))]
    /// # fn main() {
    /// use async_io::Timer;
    /// use futures_concurrency::prelude::*;
    /// use futures_lite::{future::block_on, stream};
    /// use futures_lite::prelude::*;
    /// use std::time::{Duration, Instant};
    ///
    /// block_on(async {
    ///     let now = Instant::now();
    ///     let duration = Duration::from_millis(100);
    ///
    ///     stream::once("meow")
    ///         .wait_until(Timer::after(duration))
    ///         .next()
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

impl<S1> StreamExt for S1
where
    S1: Stream,
{
    fn merge<T, S2>(self, other: S2) -> Merge2<T, S1, S2::IntoStream>
    where
        S1: Stream<Item = T>,
        S2: IntoStream<Item = T>,
    {
        Merge::merge((self, other))
    }

    fn chain<T, S2>(self, other: S2) -> Chain2<Self, S2::IntoStream>
    where
        Self: Stream<Item = T> + Sized,
        S2: IntoStream<Item = T>,
    {
        // TODO(yosh): fix the bounds on the tuple impl
        Chain::chain((self, other.into_stream()))
    }

    fn zip<T, S2>(self, other: S2) -> Zip2<Self, S2::IntoStream>
    where
        Self: Stream<Item = T> + Sized,
        S2: IntoStream<Item = T>,
    {
        // TODO(yosh): fix the bounds on the tuple impl
        Zip::zip((self, other.into_stream()))
    }
}
