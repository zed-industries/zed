use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

/// Creates a stream that yields the same item repeatedly.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::repeat(7);
///
/// assert_eq!(s.next().await, Some(7));
/// assert_eq!(s.next().await, Some(7));
/// #
/// # })
/// ```
pub fn repeat<T>(item: T) -> Repeat<T>
where
    T: Clone,
{
    Repeat { item }
}

/// A stream that yields the same item repeatedly.
///
/// This stream is created by the [`repeat`] function. See its
/// documentation for more.
///
/// [`repeat`]: fn.repeat.html
#[derive(Clone, Debug)]
pub struct Repeat<T> {
    item: T,
}

impl<T: Clone> Stream for Repeat<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(self.item.clone()))
    }
}
