use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

/// A stream that yields elements by calling a closure.
///
/// This stream is created by the [`from_fn`] function. See its
/// documentation for more.
///
/// [`from_fn`]: fn.from_fn.html
#[derive(Clone, Debug)]
pub struct FromFn<F> {
    f: F,
}

impl<F> Unpin for FromFn<F> {}

/// Creates a new stream where to produce each new element a provided closure is called.
///
/// This allows creating a custom stream with any behaviour without using the more verbose
/// syntax of creating a dedicated type and implementing a `Stream` trait for it.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut count = 0u8;
/// let s = stream::from_fn(|| {
///     count += 1;
///     if count > 3 {
///         None
///     } else {
///         Some(count)
///     }
/// });
///
/// pin_utils::pin_mut!(s);
///
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(2));
/// assert_eq!(s.next().await, Some(3));
/// assert_eq!(s.next().await, None);
/// #
/// # })
/// ```
pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    FromFn { f }
}

impl<T, F> Stream for FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = (&mut self.f)();
        Poll::Ready(item)
    }
}
