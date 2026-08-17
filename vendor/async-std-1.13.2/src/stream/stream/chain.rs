use core::pin::Pin;

use pin_project_lite::pin_project;

use super::fuse::Fuse;
use crate::stream::stream::StreamExt;
use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that chains two streams one after another.
    ///
    /// This `struct` is created by the [`chain`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`chain`]: trait.Stream.html#method.chain
    /// [`Stream`]: trait.Stream.html
    #[derive(Debug)]
    pub struct Chain<S, U> {
        #[pin]
        first: Fuse<S>,
        #[pin]
        second: Fuse<U>,
    }
}

impl<S: Stream, U: Stream> Chain<S, U> {
    pub(super) fn new(first: S, second: U) -> Self {
        Self {
            first: first.fuse(),
            second: second.fuse(),
        }
    }
}

impl<S: Stream, U: Stream<Item = S::Item>> Stream for Chain<S, U> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        if !this.first.done {
            let next = futures_core::ready!(this.first.as_mut().poll_next(cx));
            if let Some(next) = next {
                return Poll::Ready(Some(next));
            }
        }

        if !this.second.done {
            let next = futures_core::ready!(this.second.as_mut().poll_next(cx));
            if let Some(next) = next {
                return Poll::Ready(Some(next));
            }
        }

        if this.first.done && this.second.done {
            return Poll::Ready(None);
        }

        Poll::Pending
    }
}
