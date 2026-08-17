//! A sink for values which are asynchronously accepted, until the target is closed.
//!
//! Postage channel senders implement Sink:
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::Sink;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (mut tx, rx) = channel(16);
//!     assert_eq!(Ok(()), tx.send(true).await);
//! }
//! ```
//!
//! Sinks return an error if the channel is closed, and the message cannot be accepted by the receiver:
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::{SendError, Sink};
//!
//! #[tokio::main]
//! async fn main() {
//!     let (mut tx, rx) = channel(16);
//!     drop(rx);
//!     assert_eq!(Err(SendError(true)), tx.send(true).await);
//! }
//! ```
//!
//! Note that `Sink::send` returns an `Err` type, unlike `Stream::recv` which returns an option.
//! This is because the failure to send a message sometimes needs to be interpreted as an application error:
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::{SendError, Sink};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), SendError<bool>> {
//!     let (mut tx, rx) = channel(16);
//!     tx.send(true).await?;
//!     Ok(())
//! }
//! ```
//!
//! Tasks can ignore send errors by using `Result::ok`:
//! ```rust
//! use postage::mpsc::channel;
//! use postage::sink::Sink;
//!
//! #[tokio::main]
//! async fn main() {
//!     let (mut tx, rx) = channel(16);
//!     tx.send(true).await.ok();
//! }
//! ```
use std::marker::PhantomPinned;
use std::{future::Future, ops::DerefMut, pin::Pin, task::Poll};

use crate::Context;
use pin_project::pin_project;

mod chain;
mod errors;
mod filter;

#[cfg(feature = "logging")]
mod sink_log;

pub use errors::*;

/// A sink which can asynchronously accept messages, and at some point may refuse to accept any further messages.
///
/// Sinks implement `poll_send`, a poll-based method very similar to `std::future::Future`.
///
/// Sinks can be used in async code with `stream.send(value).await`, or with `stream.try_send(value)`.  Note that
/// `send` returns an error if the sink has been closed.  And `try_send` returns an error if the sink is full, or it is closed.
///
/// Send errors can be ignored using `Result::ok`.
///
/// ```rust
/// use postage::mpsc::channel;
/// use postage::sink::{Sink, TrySendError};
///
/// #[tokio::main]
/// async fn main() -> Result<(), TrySendError<bool>> {
///     let (mut tx, mut rx) = channel(16);
///     tx.send(true).await.ok();
///     tx.try_send(true)?;
///     drop(tx);
///     Ok(())
/// }
/// ```
///
/// Sinks also support combinators, such as map, filter, chain, and log.
/// ```rust
/// use postage::mpsc::channel;
/// use postage::sink::{Sink, SendError, TrySendError};
/// use postage::stream::Stream;
///
/// #[tokio::main]
/// async fn main() {
///     let (mut tx, mut rx) = channel(16);
///     let (tx2, mut rx2) = channel(16);
///
///     let mut combo = tx2
///         .after(tx)
///         .filter(|i| *i >= 2);
///     
///     // The `logging` feature enables a combinator that logs values using the Debug trait.
///     #[cfg(feature = "logging")]
///     let mut combo = combo
///         .log(log::Level::Info);
///
///     combo.send(1usize).await.ok();
///     combo.send(2usize).await.ok();

///     assert_eq!(Some(2usize), rx.recv().await);
///     drop(rx);
///
///     combo.send(3usize).await.ok();
///     combo.send(4usize).await.ok();
///     assert_eq!(Some(3usize), rx2.recv().await);
///     assert_eq!(Some(4usize), rx2.recv().await);
///
///     drop(rx2);
///     assert_eq!(Err(SendError(5usize)), combo.send(5usize).await);
/// }
/// ```
pub trait Sink {
    type Item;

    /// Attempts to accept the message, without blocking.  
    ///
    /// Returns:
    /// - `PollSend::Ready` if the value was sent
    /// - `PollSend::Pending(value)` if the channel is full.  The channel will call the waker in `cx` when the item may be accepted in the future.
    /// - `PollSend::Rejected(value)` if the channel is closed, and will never accept the item.
    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item>;

    /// Attempts to send a message into the sink.  
    ///
    /// Returns:
    /// - `Ok(())` if the value was accepted.
    /// - `Err(SendError(value))` if the sink rejected the message.
    fn send(&mut self, value: Self::Item) -> SendFuture<Self> {
        SendFuture::new(self, value)
    }

    /// Attempts to send a message over the sink, without blocking.
    ///
    /// Returns:
    /// - `Ok(())` if the value was accepted.
    /// - `Err(TrySendError::Pending(value))` if the channel is full, and cannot accept the item at this time.
    /// - `Err(TrySendError::Rejected(value))` if the channel is closed, and will never accept the item.
    fn try_send(&mut self, value: Self::Item) -> Result<(), TrySendError<Self::Item>>
    where
        Self: Unpin,
    {
        let pin = Pin::new(self);

        match pin.poll_send(&mut Context::empty(), value) {
            PollSend::Ready => Ok(()),
            PollSend::Pending(value) => Err(TrySendError::Pending(value)),
            PollSend::Rejected(value) => Err(TrySendError::Rejected(value)),
        }
    }

    /// Sends a message over the channel, blocking the current thread until the message is sent.
    ///
    /// Requires the `blocking` feature (enabled by default).
    #[cfg(feature = "blocking")]
    fn blocking_send(&mut self, value: Self::Item) -> Result<(), SendError<Self::Item>>
    where
        Self: Unpin,
    {
        pollster::block_on(self.send(value))
    }

    /// Chains two sink implementations.  Messages will be transmitted to the argument until it rejects a message.
    /// Then messages will be transmitted to self.
    fn after<Before>(self, before: Before) -> chain::ChainSink<Before, Self>
    where
        Before: Sink<Item = Self::Item>,
        Self: Sized,
    {
        chain::ChainSink::new(before, self)
    }

    /// Filters messages, forwarding them to the sink if the filter returns true
    fn filter<Filter>(self, filter: Filter) -> filter::FilterSink<Filter, Self>
    where
        Filter: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        filter::FilterSink::new(filter, self)
    }

    /// Logs messages that are accepted by the sink using the Debug trait, at the provided log level.
    ///
    /// Requires the `logging` feature
    #[cfg(feature = "logging")]
    fn log(self, level: log::Level) -> sink_log::SinkLog<Self>
    where
        Self: Sized,
        Self::Item: std::fmt::Debug,
    {
        sink_log::SinkLog::new(self, level)
    }
}

impl<S> Sink for &mut S
where
    S: Sink + Unpin + ?Sized,
{
    type Item = S::Item;

    fn poll_send(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        S::poll_send(Pin::new(&mut **self), cx, value)
    }
}

impl<P, S> Sink for Pin<P>
where
    P: DerefMut<Target = S> + Unpin,
    S: Sink + Unpin + ?Sized,
{
    type Item = <S as Sink>::Item;

    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        value: Self::Item,
    ) -> PollSend<Self::Item> {
        Pin::get_mut(self).as_mut().poll_send(cx, value)
    }
}

/// An enum of poll responses that are produced by Sink implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PollSend<T> {
    /// The item was accepted and sent
    Ready,
    /// The sender is pending, and has registered with the waker context
    Pending(T),
    /// The sender has been closed, and will never accept the item
    Rejected(T),
}

/// A future returned by `Sink::send`, which wraps an item.
/// The item is sent to the sink, or returned if the sink is closed.
#[pin_project]
#[must_use = "futures do nothing unless polled"]
pub struct SendFuture<'s, S>
where
    S: Sink + ?Sized,
{
    #[pin]
    send: &'s mut S,
    value: Option<S::Item>,
    #[pin]
    _pin: PhantomPinned,
}

impl<'s, S> SendFuture<'s, S>
where
    S: Sink + ?Sized,
{
    pub fn new(send: &'s mut S, value: S::Item) -> SendFuture<S> {
        Self {
            send,
            value: Some(value),
            _pin: PhantomPinned,
        }
    }
}

impl<'s, S> Future for SendFuture<'s, S>
where
    S: Sink + Unpin + ?Sized,
{
    type Output = Result<(), SendError<S::Item>>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.value.is_none() {
            return Poll::Ready(Ok(()));
        }

        let this = self.project();

        let mut cx: crate::Context<'_> = cx.into();
        match this.send.poll_send(&mut cx, this.value.take().unwrap()) {
            PollSend::Ready => Poll::Ready(Ok(())),
            PollSend::Pending(value) => {
                *this.value = Some(value);
                Poll::Pending
            }
            PollSend::Rejected(value) => Poll::Ready(Err(SendError(value))),
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "blocking")]
    #[test]
    fn test_blocking() {
        use super::Sink;
        use crate::test::sink::ready;

        let mut stream = ready();
        assert_eq!(Ok(()), stream.blocking_send(1usize));
    }
}
