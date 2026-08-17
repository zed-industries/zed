#![cfg(feature = "with-deprecated")]
#![deprecated(since = "0.1.4", note = "use sync::mpsc::channel instead")]
#![allow(deprecated)]

use std::any::Any;
use std::error::Error;
use std::fmt;

use {Poll, Async, Stream, Future, Sink};
use sink::Send;
use sync::mpsc;

/// Creates an in-memory channel implementation of the `Stream` trait.
///
/// This method creates a concrete implementation of the `Stream` trait which
/// can be used to send values across threads in a streaming fashion. This
/// channel is unique in that it implements back pressure to ensure that the
/// sender never outpaces the receiver. The `Sender::send` method will only
/// allow sending one message and the next message can only be sent once the
/// first was consumed.
///
/// The `Receiver` returned implements the `Stream` trait and has access to any
/// number of the associated combinators for transforming the result.
pub fn channel<T, E>() -> (Sender<T, E>, Receiver<T, E>) {
    let (tx, rx) = mpsc::channel(0);
    (Sender { inner: tx }, Receiver { inner: rx })
}

/// The transmission end of a channel which is used to send values.
///
/// This is created by the `channel` method in the `stream` module.
#[derive(Debug)]
pub struct Sender<T, E> {
    inner: mpsc::Sender<Result<T, E>>,
}

/// The receiving end of a channel which implements the `Stream` trait.
///
/// This is a concrete implementation of a stream which can be used to represent
/// a stream of values being computed elsewhere. This is created by the
/// `channel` method in the `stream` module.
#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct Receiver<T, E> {
    inner: mpsc::Receiver<Result<T, E>>,
}

/// Error type for sending, used when the receiving end of the channel is dropped
pub struct SendError<T, E>(Result<T, E>);

/// Future returned by `Sender::send`.
#[derive(Debug)]
pub struct FutureSender<T, E> {
    inner: Send<mpsc::Sender<Result<T, E>>>,
}

impl<T, E> fmt::Debug for SendError<T, E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("SendError")
            .field(&"...")
            .finish()
    }
}

impl<T, E> fmt::Display for SendError<T, E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "send failed because receiver is gone")
    }
}

impl<T, E> Error for SendError<T, E>
    where T: Any, E: Any
{
    fn description(&self) -> &str {
        "send failed because receiver is gone"
    }
}


impl<T, E> Stream for Receiver<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<T>, E> {
        match self.inner.poll().expect("cannot fail") {
            Async::Ready(Some(Ok(e))) => Ok(Async::Ready(Some(e))),
            Async::Ready(Some(Err(e))) => Err(e),
            Async::Ready(None) => Ok(Async::Ready(None)),
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}

impl<T, E> Sender<T, E> {
    /// Sends a new value along this channel to the receiver.
    ///
    /// This method consumes the sender and returns a future which will resolve
    /// to the sender again when the value sent has been consumed.
    pub fn send(self, t: Result<T, E>) -> FutureSender<T, E> {
        FutureSender { inner: self.inner.send(t) }
    }
}

impl<T, E> Future for FutureSender<T, E> {
    type Item = Sender<T, E>;
    type Error = SendError<T, E>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.inner.poll() {
            Ok(a) => Ok(a.map(|a| Sender { inner: a })),
            Err(e) => Err(SendError(e.into_inner())),
        }
    }
}
