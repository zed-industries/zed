//! Definition of the `PollFn` combinator

use {Stream, Poll};

/// A stream which adapts a function returning `Poll`.
///
/// Created by the `poll_fn` function.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct PollFn<F> {
    inner: F,
}

/// Creates a new stream wrapping around a function returning `Poll`.
///
/// Polling the returned stream delegates to the wrapped function.
///
/// # Examples
///
/// ```
/// use futures::stream::poll_fn;
/// use futures::{Async, Poll};
///
/// let mut counter = 1usize;
///
/// let read_stream = poll_fn(move || -> Poll<Option<String>, std::io::Error> {
///     if counter == 0 { return Ok(Async::Ready(None)); }
///     counter -= 1;
///     Ok(Async::Ready(Some("Hello, World!".to_owned())))
/// });
/// ```
pub fn poll_fn<T, E, F>(f: F) -> PollFn<F>
where
    F: FnMut() -> Poll<Option<T>, E>,
{
    PollFn { inner: f }
}

impl<T, E, F> Stream for PollFn<F>
where
    F: FnMut() -> Poll<Option<T>, E>,
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<T>, E> {
        (self.inner)()
    }
}
