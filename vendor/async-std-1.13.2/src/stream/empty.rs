use core::marker::PhantomData;
use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

/// Creates a stream that doesn't yield any items.
///
/// This `struct` is created by the [`empty`] function. See its
/// documentation for more.
///
/// [`empty`]: fn.empty.html
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::empty::<i32>();
///
/// assert_eq!(s.next().await, None);
/// #
/// # })
/// ```
pub fn empty<T>() -> Empty<T> {
    Empty {
        _marker: PhantomData,
    }
}

/// A stream that doesn't yield any items.
///
/// This stream is constructed by the [`empty`] function.
///
/// [`empty`]: fn.empty.html
#[derive(Debug)]
pub struct Empty<T> {
    _marker: PhantomData<T>,
}

impl<T> Stream for Empty<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}
