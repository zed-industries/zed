#![deprecated(note = "implementation moved to `iter_ok` and `iter_result`")]
#![allow(deprecated)]

use Poll;
use stream::{iter_result, IterResult, Stream};

/// A stream which is just a shim over an underlying instance of `Iterator`.
///
/// This stream will never block and is always ready.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Iter<I>(IterResult<I>);

/// Converts an `Iterator` over `Result`s into a `Stream` which is always ready
/// to yield the next value.
///
/// Iterators in Rust don't express the ability to block, so this adapter simply
/// always calls `iter.next()` and returns that.
///
/// ```rust
/// use futures::*;
///
/// let mut stream = stream::iter(vec![Ok(17), Err(false), Ok(19)]);
/// assert_eq!(Ok(Async::Ready(Some(17))), stream.poll());
/// assert_eq!(Err(false), stream.poll());
/// assert_eq!(Ok(Async::Ready(Some(19))), stream.poll());
/// assert_eq!(Ok(Async::Ready(None)), stream.poll());
/// ```
#[inline]
pub fn iter<J, T, E>(i: J) -> Iter<J::IntoIter>
    where J: IntoIterator<Item=Result<T, E>>,
{
    Iter(iter_result(i))
}

impl<I, T, E> Stream for Iter<I>
    where I: Iterator<Item=Result<T, E>>,
{
    type Item = T;
    type Error = E;

    #[inline]
    fn poll(&mut self) -> Poll<Option<T>, E> {
        self.0.poll()
    }
}
