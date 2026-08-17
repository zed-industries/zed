use {Async, Poll};
use stream::Stream;

/// A stream which is just a shim over an underlying instance of `Iterator`.
///
/// This stream will never block and is always ready.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct IterResult<I> {
    iter: I,
}

/// Converts an `Iterator` over `Result`s into a `Stream` which is always ready
/// to yield the next value.
///
/// Iterators in Rust don't express the ability to block, so this adapter simply
/// always calls `iter.next()` and returns that.
///
/// ```rust
/// use futures::*;
///
/// let mut stream = stream::iter_result(vec![Ok(17), Err(false), Ok(19)]);
/// assert_eq!(Ok(Async::Ready(Some(17))), stream.poll());
/// assert_eq!(Err(false), stream.poll());
/// assert_eq!(Ok(Async::Ready(Some(19))), stream.poll());
/// assert_eq!(Ok(Async::Ready(None)), stream.poll());
/// ```
pub fn iter_result<J, T, E>(i: J) -> IterResult<J::IntoIter>
where
    J: IntoIterator<Item = Result<T, E>>,
{
    IterResult {
        iter: i.into_iter(),
    }
}

impl<I, T, E> Stream for IterResult<I>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<T>, E> {
        match self.iter.next() {
            Some(Ok(e)) => Ok(Async::Ready(Some(e))),
            Some(Err(e)) => Err(e),
            None => Ok(Async::Ready(None)),
        }
    }
}
