use core::marker;


use stream::Stream;

use {Async, Poll};


/// Stream that produces the same element repeatedly.
///
/// This structure is created by the `stream::repeat` function.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Repeat<T, E>
    where T: Clone
{
    item: T,
    error: marker::PhantomData<E>,
}

/// Create a stream which produces the same item repeatedly.
///
/// Stream never produces an error or EOF. Note that you likely want to avoid
/// usage of `collect` or such on the returned stream as it will exhaust
/// available memory as it tries to just fill up all RAM.
///
/// ```rust
/// use futures::*;
///
/// let mut stream = stream::repeat::<_, bool>(10);
/// assert_eq!(Ok(Async::Ready(Some(10))), stream.poll());
/// assert_eq!(Ok(Async::Ready(Some(10))), stream.poll());
/// assert_eq!(Ok(Async::Ready(Some(10))), stream.poll());
/// ```
pub fn repeat<T, E>(item: T) -> Repeat<T, E>
    where T: Clone
{
    Repeat {
        item: item,
        error: marker::PhantomData,
    }
}

impl<T, E> Stream for Repeat<T, E>
    where T: Clone
{
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(Some(self.item.clone())))
    }
}
