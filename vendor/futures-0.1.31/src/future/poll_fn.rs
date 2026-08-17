//! Definition of the `PollFn` adapter combinator

use {Future, Poll};

/// A future which adapts a function returning `Poll`.
///
/// Created by the `poll_fn` function.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct PollFn<F> {
    inner: F,
}

/// Creates a new future wrapping around a function returning `Poll`.
///
/// Polling the returned future delegates to the wrapped function.
///
/// # Examples
///
/// ```
/// use futures::future::poll_fn;
/// use futures::{Async, Poll};
///
/// fn read_line() -> Poll<String, std::io::Error> {
///     Ok(Async::Ready("Hello, World!".into()))
/// }
///
/// let read_future = poll_fn(read_line);
/// ```
pub fn poll_fn<T, E, F>(f: F) -> PollFn<F>
    where F: FnMut() -> ::Poll<T, E>
{
    PollFn { inner: f }
}

impl<T, E, F> Future for PollFn<F>
    where F: FnMut() -> Poll<T, E>
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        (self.inner)()
    }
}
