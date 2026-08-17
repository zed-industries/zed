//! A stream of values which are asynchronously produced, until the source is closed.
//!
//! Postage channel receivers implement Stream:
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::Sink;
//! use postage::stream::Stream;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (mut tx, mut rx) = channel(16);
//!     tx.send(true).await;
//!     drop(tx);
//!     assert_eq!(Some(true), rx.recv().await);
//!     assert_eq!(None, rx.recv().await);
//! }
//! ```
//!
//! Streams produce `Option<T>`.  When a None value is recieved, the stream is closed and
//! will never produce another item.  Loops can be concicely written with `while let Some(v) = rx.recv().await {}`
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::Sink;
//! use postage::stream::Stream;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (mut tx, mut rx) = channel(16);
//!     tx.send(true).await;
//!     tx.send(true).await;
//!     drop(tx);
//!
//!     while let Some(v) = rx.recv().await {
//!         println!("Value received!: {}", v);
//!     }
//! }
//! ```
use std::{future::Future, marker::PhantomPinned, ops::DerefMut, pin::Pin};

use crate::Context;
use pin_project::pin_project;
use std::task::Poll;

use self::{
    chain::ChainStream, filter::FilterStream, find::FindStream, map::MapStream, merge::MergeStream,
    once::OnceStream, repeat::RepeatStream,
};

mod chain;
mod errors;
mod filter;
mod find;
mod map;
mod merge;
mod once;
mod repeat;

#[cfg(feature = "logging")]
mod stream_log;

pub use errors::*;

/// An asynchronous stream, which produces a series of messages until closed.
///
/// Streams implement `poll_recv`, a poll-based method very similar to `std::future::Future`.
///
/// Streams can be used in async code with `stream.recv().await`, or with `stream.try_recv()`.
///
/// ```rust
/// use postage::mpsc::channel;
/// use postage::sink::Sink;
/// use postage::stream::Stream;
///
/// #[tokio::main]
/// async fn main() {
///     let (mut tx, mut rx) = channel(16);
///     tx.send(true).await;
///     tx.send(true).await;
///     drop(tx);
///
///     while let Some(_v) = rx.recv().await {
///         println!("Value received!");
///         if let Ok(_v) = rx.try_recv() {
///             println!("Extra value received!");
///         }
///     }
/// }
/// ```
///
/// Streams also support combinators, such as map, filter, find, and log.
/// ```rust
/// use postage::mpsc::channel;
/// use postage::sink::Sink;
/// use postage::stream::{Stream, TryRecvError};
///
/// #[tokio::main]
/// async fn main() {
///     let (mut tx, rx) = channel(16);
///
///     tx.send(1usize).await;
///     tx.send(2usize).await;
///     tx.send(3usize).await;
///     drop(tx);
///
///     let mut rx = rx
///         .map(|i| i * 2)
///         .filter(|i| *i >= 4)
///         .find(|i| *i == 6);
///
///     // The `logging` feature enables a combinator that logs values using the Debug trait.
///     #[cfg(feature = "logging")]
///     let mut rx = rx
///         .log(log::Level::Info);
///
///     assert_eq!(Ok(6), rx.try_recv());
///     assert_eq!(Err(TryRecvError::Closed), rx.try_recv());
/// }
/// ```
#[must_use = "streams do nothing unless polled"]
pub trait Stream {
    type Item;

    /// Attempts to retrieve an item from the stream, without blocking.
    ///
    /// Returns:
    /// - `PollRecv::Ready(value)` if a message is ready
    /// - `PollRecv::Pending` if the stream is open, but no message is currently available.
    /// - `PollRecv::Closed` if the stream is closed, and no messages are expected.
    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item>;

    /// Retrieves a message from the stream.
    ///
    /// Returns:
    /// - `Some(value)` if the stream is open
    /// - `None` if the stream is closed, and no further messages are expected.
    fn recv(&mut self) -> RecvFuture<'_, Self>
    where
        Self: Unpin,
    {
        RecvFuture::new(self)
    }

    /// Attempts to retrive a message from the stream, without blocking.
    ///
    /// Returns:
    /// - `Ok(value)` if a message is ready.
    /// - `TryRecvError::Pending` if the stream is open, but no messages are available.
    /// - `TryRecvError::Closed` if the stream has been closed, and no items are expected.
    fn try_recv(&mut self) -> Result<Self::Item, TryRecvError>
    where
        Self: Unpin,
    {
        let pin = Pin::new(self);

        match pin.poll_recv(&mut Context::empty()) {
            PollRecv::Ready(value) => Ok(value),
            PollRecv::Pending => Err(TryRecvError::Pending),
            PollRecv::Closed => Err(TryRecvError::Closed),
        }
    }

    /// Retrieves a message from the stream, blocking the current thread until one is available.
    ///
    /// Returns:
    /// - `Some(value)` if the stream is open
    /// - `None` if the stream is closed, and no further messages are expected.
    #[cfg(feature = "blocking")]
    fn blocking_recv(&mut self) -> Option<Self::Item>
    where
        Self: Unpin,
    {
        pollster::block_on(self.recv())
    }

    /// Transforms the stream with a map function.
    fn map<Map, Into>(self, map: Map) -> MapStream<Self, Map, Into>
    where
        Map: Fn(Self::Item) -> Into,
        Self: Sized,
    {
        MapStream::new(self, map)
    }

    /// Filters messages returned by the stream, ignoring messages where `filter` returns false.
    fn filter<Filter>(self, filter: Filter) -> FilterStream<Self, Filter>
    where
        Self: Sized + Unpin,
        Filter: FnMut(&Self::Item) -> bool + Unpin,
    {
        FilterStream::new(self, filter)
    }

    /// Merges two streams, returning values from both at once, until both are closed.
    fn merge<Other>(self, other: Other) -> MergeStream<Self, Other>
    where
        Other: Stream<Item = Self::Item>,
        Self: Sized,
    {
        MergeStream::new(self, other)
    }

    /// Chains two streams, returning values from `self` until it is closed, and then returning values from `other`.
    fn chain<Other>(self, other: Other) -> ChainStream<Self, Other>
    where
        Other: Stream<Item = Self::Item>,
        Self: Sized,
    {
        ChainStream::new(self, other)
    }

    /// Finds a message matching a condition.  When the condition is matched, a single value will be returned.
    /// Then the stream will be closed.
    fn find<Condition>(self, condition: Condition) -> FindStream<Self, Condition>
    where
        Self: Sized + Unpin,
        Condition: Fn(&Self::Item) -> bool + Unpin,
    {
        FindStream::new(self, condition)
    }

    /// Logs messages that are produced by the stream using the Debug trait, at the provided log level.
    ///
    /// Requires the `logging` feature
    #[cfg(feature = "logging")]
    fn log(self, level: log::Level) -> stream_log::StreamLog<Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        stream_log::StreamLog::new(self, level)
    }
}

impl<S> Stream for &mut S
where
    S: Stream + Unpin + ?Sized,
{
    type Item = S::Item;

    fn poll_recv(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        S::poll_recv(Pin::new(&mut **self), cx)
    }
}

impl<P, S> Stream for Pin<P>
where
    P: DerefMut<Target = S> + Unpin,
    S: Stream + Unpin + ?Sized,
{
    type Item = <S as Stream>::Item;

    fn poll_recv(self: Pin<&mut Self>, cx: &mut Context<'_>) -> PollRecv<Self::Item> {
        Pin::get_mut(self).as_mut().poll_recv(cx)
    }
}

/// Returns a stream which produces a single value, and then is closed.
pub fn once<T>(item: T) -> OnceStream<T> {
    OnceStream::new(item)
}

/// Returns a stream which infiniately produces a clonable value.
pub fn repeat<T>(item: T) -> RepeatStream<T>
where
    T: Clone,
{
    RepeatStream::new(item)
}

/// An enum of poll responses that are produced by Stream implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PollRecv<T> {
    /// An item is ready
    Ready(T),
    /// The channel is open, but no messages are ready and the receiver has registered with the waker context
    Pending,
    /// The channel is closed, and no messages will ever be delivered
    Closed,
}

/// A future returned by `Stream::recv`.
#[pin_project]
#[must_use = "futures do nothing unless polled"]
pub struct RecvFuture<'s, S>
where
    S: Stream + ?Sized,
{
    recv: &'s mut S,
    #[pin]
    _pin: PhantomPinned,
}

impl<'s, S: Stream> RecvFuture<'s, S>
where
    S: ?Sized,
{
    pub fn new(recv: &'s mut S) -> RecvFuture<'s, S> {
        Self {
            recv,
            _pin: PhantomPinned,
        }
    }
}

impl<'s, S> Future for RecvFuture<'s, S>
where
    S: Stream + Unpin + ?Sized,
{
    type Output = Option<S::Item>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let mut cx: crate::Context<'_> = cx.into();
        match Pin::new(this.recv).poll_recv(&mut cx) {
            PollRecv::Ready(v) => Poll::Ready(Some(v)),
            PollRecv::Pending => Poll::Pending,
            PollRecv::Closed => Poll::Ready(None),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "blocking")]
    #[test]
    fn test_blocking() {
        use super::Stream;
        use crate::test::stream::ready;

        let mut stream = ready(1usize);
        assert_eq!(Some(1usize), stream.blocking_recv());
    }
}
