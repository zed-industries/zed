use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::stream::stream::StreamExt;
use crate::stream::Fuse;
use crate::stream::Stream;
use crate::utils;

pin_project! {
    /// A stream that merges two other streams into a single stream.
    ///
    /// This `struct` is created by the [`merge`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`merge`]: trait.Stream.html#method.merge
    /// [`Stream`]: trait.Stream.html
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    #[derive(Debug)]
    pub struct Merge<L, R> {
        #[pin]
        left: Fuse<L>,
        #[pin]
        right: Fuse<R>,
    }
}

impl<L: Stream, R: Stream> Merge<L, R> {
    pub(crate) fn new(left: L, right: R) -> Self {
        Self {
            left: left.fuse(),
            right: right.fuse(),
        }
    }
}

impl<L, R, T> Stream for Merge<L, R>
where
    L: Stream<Item = T>,
    R: Stream<Item = T>,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if utils::random(2) == 0 {
            poll_next_in_order(this.left, this.right, cx)
        } else {
            poll_next_in_order(this.right, this.left, cx)
        }
    }
}

fn poll_next_in_order<F, S, I>(
    first: Pin<&mut F>,
    second: Pin<&mut S>,
    cx: &mut Context<'_>,
) -> Poll<Option<I>>
where
    F: Stream<Item = I>,
    S: Stream<Item = I>,
{
    match first.poll_next(cx) {
        Poll::Ready(None) => second.poll_next(cx),
        Poll::Ready(item) => Poll::Ready(item),
        Poll::Pending => match second.poll_next(cx) {
            Poll::Ready(None) | Poll::Pending => Poll::Pending,
            Poll::Ready(item) => Poll::Ready(item),
        },
    }
}
