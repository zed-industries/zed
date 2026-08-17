//! The `Accept` trait and supporting types.
//!
//! This module contains:
//!
//! - The [`Accept`](Accept) trait used to asynchronously accept incoming
//!   connections.
//! - Utilities like `poll_fn` to ease creating a custom `Accept`.

use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "stream")]
use futures_core::Stream;
#[cfg(feature = "stream")]
use pin_project_lite::pin_project;

/// Asynchronously accept incoming connections.
pub trait Accept {
    /// The connection type that can be accepted.
    type Conn;
    /// The error type that can occur when accepting a connection.
    type Error;

    /// Poll to accept the next connection.
    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>>;
}

/// Create an `Accept` with a polling function.
///
/// # Example
///
/// ```
/// use std::task::Poll;
/// use hyper::server::{accept, Server};
///
/// # let mock_conn = ();
/// // If we created some mocked connection...
/// let mut conn = Some(mock_conn);
///
/// // And accept just the mocked conn once...
/// let once = accept::poll_fn(move |cx| {
///     Poll::Ready(conn.take().map(Ok::<_, ()>))
/// });
///
/// let builder = Server::builder(once);
/// ```
pub fn poll_fn<F, IO, E>(func: F) -> impl Accept<Conn = IO, Error = E>
where
    F: FnMut(&mut Context<'_>) -> Poll<Option<Result<IO, E>>>,
{
    struct PollFn<F>(F);

    // The closure `F` is never pinned
    impl<F> Unpin for PollFn<F> {}

    impl<F, IO, E> Accept for PollFn<F>
    where
        F: FnMut(&mut Context<'_>) -> Poll<Option<Result<IO, E>>>,
    {
        type Conn = IO;
        type Error = E;
        fn poll_accept(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
            (self.get_mut().0)(cx)
        }
    }

    PollFn(func)
}

/// Adapt a `Stream` of incoming connections into an `Accept`.
///
/// # Optional
///
/// This function requires enabling the `stream` feature in your
/// `Cargo.toml`.
#[cfg(feature = "stream")]
pub fn from_stream<S, IO, E>(stream: S) -> impl Accept<Conn = IO, Error = E>
where
    S: Stream<Item = Result<IO, E>>,
{
    pin_project! {
        struct FromStream<S> {
            #[pin]
            stream: S,
        }
    }

    impl<S, IO, E> Accept for FromStream<S>
    where
        S: Stream<Item = Result<IO, E>>,
    {
        type Conn = IO;
        type Error = E;
        fn poll_accept(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
            self.project().stream.poll_next(cx)
        }
    }

    FromStream { stream }
}
