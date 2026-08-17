use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

/// A stream that repeats elements of type `T` endlessly by applying a provided closure.
///
/// This stream is created by the [`repeat_with`] function. See its
/// documentation for more.
///
/// [`repeat_with`]: fn.repeat_with.html
#[derive(Clone, Debug)]
pub struct RepeatWith<F> {
    f: F,
}

impl<F> Unpin for RepeatWith<F> {}

/// Creates a new stream that repeats elements of type `A` endlessly by applying the provided closure.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let s = stream::repeat_with(|| 1);
///
/// pin_utils::pin_mut!(s);
///
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(1));
/// # })
/// ```
///
/// Going finite:
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut n = 1;
/// let s = stream::repeat_with(|| {
///     let item = n;
///     n *= 2;
///     item
/// })
/// .take(4);
///
/// pin_utils::pin_mut!(s);
///
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(2));
/// assert_eq!(s.next().await, Some(4));
/// assert_eq!(s.next().await, Some(8));
/// assert_eq!(s.next().await, None);
/// # })
/// ```
pub fn repeat_with<T, F>(repeater: F) -> RepeatWith<F>
where
    F: FnMut() -> T,
{
    RepeatWith { f: repeater }
}

impl<T, F> Stream for RepeatWith<F>
where
    F: FnMut() -> T,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let item = (&mut self.f)();
        Poll::Ready(Some(item))
    }
}
