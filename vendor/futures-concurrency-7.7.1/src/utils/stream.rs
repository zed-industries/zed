use core::pin::Pin;

use pin_project::pin_project;

use core::task::{Context, Poll};
use futures_core::stream::Stream;

/// A stream that was created from iterator.
#[pin_project]
#[derive(Clone, Debug)]
pub(crate) struct FromIter<I> {
    iter: I,
}

/// Converts an iterator into a stream.
pub(crate) fn from_iter<I: IntoIterator>(iter: I) -> FromIter<I::IntoIter> {
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
