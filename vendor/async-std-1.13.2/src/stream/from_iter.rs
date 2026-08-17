use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
#[cfg(feature = "unstable")]
use crate::stream::double_ended_stream::DoubleEndedStream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that was created from iterator.
    ///
    /// This stream is created by the [`from_iter`] function.
    /// See it documentation for more.
    ///
    /// [`from_iter`]: fn.from_iter.html
    #[derive(Clone, Debug)]
    pub struct FromIter<I> {
        iter: I,
    }
}

/// Converts an iterator into a stream.
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use async_std::prelude::*;
/// use async_std::stream;
///
/// let mut s = stream::from_iter(vec![0, 1, 2, 3]);
///
/// assert_eq!(s.next().await, Some(0));
/// assert_eq!(s.next().await, Some(1));
/// assert_eq!(s.next().await, Some(2));
/// assert_eq!(s.next().await, Some(3));
/// assert_eq!(s.next().await, None);
/// #
/// # })
/// ```
pub fn from_iter<I: IntoIterator>(iter: I) -> FromIter<I::IntoIter> {
    FromIter {
        iter: iter.into_iter(),
    }
}

impl<I: Iterator> Stream for FromIter<I> {
    type Item = I::Item;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.iter.next())
    }
}

#[cfg(feature = "unstable")]
impl<T: DoubleEndedIterator> DoubleEndedStream for FromIter<T> {
    fn poll_next_back(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<T::Item>> {
        Poll::Ready(self.iter.next_back())
    }
}
