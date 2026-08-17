use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

#[cfg(feature = "unstable")]
use crate::stream::DoubleEndedStream;

/// Creates a stream that yields a single item.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::once(7);
///
/// assert_eq!(s.next().await, Some(7));
/// assert_eq!(s.next().await, None);
/// #
/// # })
/// ```
pub fn once<T>(t: T) -> Once<T> {
    Once { value: Some(t) }
}

pin_project! {
    /// A stream that yields a single item.
    ///
    /// This stream is created by the [`once`] function. See its
    /// documentation for more.
    ///
    /// [`once`]: fn.once.html
    #[derive(Clone, Debug)]
    pub struct Once<T> {
        value: Option<T>,
    }
}

impl<T> Stream for Once<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<T>> {
        Poll::Ready(self.project().value.take())
    }
}

#[cfg(feature = "unstable")]
impl <T> DoubleEndedStream for Once<T> {
    fn poll_next_back(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.project().value.take())
    }
}
