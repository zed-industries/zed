//! Concurrent execution of streams
//!
//! # Examples
//!
//! **Concurrently process items in a collection**
//!
//! ```rust
//! use futures_concurrency::prelude::*;
//!
//! # futures::executor::block_on(async {
//! let v: Vec<_> = vec!["chashu", "nori"]
//!     .into_co_stream()
//!     .map(|msg| async move { format!("hello {msg}") })
//!     .collect()
//!     .await;
//!
//! assert_eq!(v, &["hello chashu", "hello nori"]);
//! # });
//! ```
//!
//! **Concurrently process items in a stream**
//!
//! ```rust
//! use futures_concurrency::prelude::*;
//! use futures_lite::stream;
//!
//! # futures::executor::block_on(async {
//! let v: Vec<_> = stream::repeat("chashu")
//!     .co()
//!     .take(2)
//!     .map(|msg| async move { format!("hello {msg}") })
//!     .collect()
//!     .await;
//!
//! assert_eq!(v, &["hello chashu", "hello chashu"]);
//! # });
//! ```

mod enumerate;
mod for_each;
mod from_concurrent_stream;
mod from_stream;
mod into_concurrent_stream;
mod limit;
mod map;
mod take;
mod try_for_each;

use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;
use for_each::ForEachConsumer;
use try_for_each::TryForEachConsumer;

pub use enumerate::Enumerate;
pub use from_concurrent_stream::FromConcurrentStream;
pub use from_stream::FromStream;
pub use into_concurrent_stream::IntoConcurrentStream;
pub use limit::Limit;
pub use map::Map;
pub use take::Take;

/// Describes a type which can receive data.
///
/// # Type Generics
/// - `Item` in this context means the item that it will  repeatedly receive.
/// - `Future` in this context refers to the future type repeatedly submitted to it.
#[allow(async_fn_in_trait)]
pub trait Consumer<Item, Fut>
where
    Fut: Future<Output = Item>,
{
    /// What is the type of the item we're returning when completed?
    type Output;

    /// Send an item down to the next step in the processing queue.
    async fn send(self: Pin<&mut Self>, fut: Fut) -> ConsumerState;

    /// Make progress on the consumer while doing something else.
    ///
    /// It should always be possible to drop the future returned by this
    /// function. This is solely intended to keep work going on the `Consumer`
    /// while doing e.g. waiting for new futures from a stream.
    async fn progress(self: Pin<&mut Self>) -> ConsumerState;

    /// We have no more data left to send to the `Consumer`; wait for its
    /// output.
    async fn flush(self: Pin<&mut Self>) -> Self::Output;
}

/// Concurrently operate over items in a stream
#[allow(async_fn_in_trait)]
pub trait ConcurrentStream {
    /// Which item will we be yielding?
    type Item;

    /// What's the type of the future containing our items?
    type Future: Future<Output = Self::Item>;

    /// Internal method used to define the behavior of this concurrent iterator.
    /// You should not need to call this directly. This method causes the
    /// iterator self to start producing items and to feed them to the consumer
    /// consumer one by one.
    async fn drive<C>(self, consumer: C) -> C::Output
    where
        C: Consumer<Self::Item, Self::Future>;

    /// How much concurrency should we apply?
    fn concurrency_limit(&self) -> Option<NonZeroUsize>;

    /// How many items could we potentially end up returning?
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Creates a stream which gives the current iteration count as well as
    /// the next value.
    ///
    /// The value is determined by the moment the future is created, not the
    /// moment the future is evaluated.
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }

    /// Obtain a simple pass-through adapter.
    fn limit(self, limit: Option<NonZeroUsize>) -> Limit<Self>
    where
        Self: Sized,
    {
        Limit::new(self, limit)
    }

    /// Creates a stream that yields the first `n` elements, or fewer if the
    /// underlying iterator ends sooner.
    fn take(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, limit)
    }

    /// Convert items from one type into another
    fn map<F, FutB, B>(self, f: F) -> Map<Self, F, Self::Future, Self::Item, FutB, B>
    where
        Self: Sized,
        F: Fn(Self::Item) -> FutB,
        F: Clone,
        FutB: Future<Output = B>,
    {
        Map::new(self, f)
    }

    /// Iterate over each item concurrently
    async fn for_each<F, Fut>(self, f: F)
    where
        Self: Sized,
        F: Fn(Self::Item) -> Fut,
        F: Clone,
        Fut: Future<Output = ()>,
    {
        let limit = self.concurrency_limit();
        self.drive(ForEachConsumer::new(limit, f)).await
    }

    /// Iterate over each item concurrently, short-circuit on error.
    ///
    /// If an error is returned this will cancel all other futures.
    async fn try_for_each<F, Fut, E>(self, f: F) -> Result<(), E>
    where
        Self: Sized,
        F: Fn(Self::Item) -> Fut,
        F: Clone,
        Fut: Future<Output = Result<(), E>>,
    {
        let limit = self.concurrency_limit();
        self.drive(TryForEachConsumer::new(limit, f)).await
    }

    /// Transforms an iterator into a collection.
    async fn collect<B>(self) -> B
    where
        B: FromConcurrentStream<Self::Item>,
        Self: Sized,
    {
        B::from_concurrent_stream(self).await
    }
}

/// The state of the consumer, used to communicate back to the source.
#[derive(Debug)]
pub enum ConsumerState {
    /// The consumer is done making progress, and the `flush` method should be called.
    Break,
    /// The consumer is ready to keep making progress.
    Continue,
    /// The consumer currently holds no values and should not be called until
    /// more values have been provided to it.
    Empty,
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::prelude::*;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn drain() {
        futures_lite::future::block_on(async {
            stream::repeat(1)
                .take(5)
                .co()
                .map(|x| async move {
                    println!("{x:?}");
                })
                .for_each(|_| async {})
                .await;
        });
    }

    #[test]
    fn for_each() {
        futures_lite::future::block_on(async {
            let s = stream::repeat(1).take(2);
            s.co()
                .limit(NonZeroUsize::new(3))
                .for_each(|x| async move {
                    println!("{x:?}");
                })
                .await;
        });
    }
}
