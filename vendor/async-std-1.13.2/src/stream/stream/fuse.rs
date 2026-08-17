use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that yields `None` forever after the underlying stream yields `None` once.
    ///
    /// This `struct` is created by the [`fuse`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`fuse`]: trait.Stream.html#method.fuse
    /// [`Stream`]: trait.Stream.html
    #[derive(Clone, Debug)]
    pub struct Fuse<S> {
        #[pin]
        pub(crate) stream: S,
        pub(crate) done: bool,
    }
}

impl<S> Fuse<S> {
    pub(super) fn new(stream: S) -> Self {
        Self {
            stream,
            done: false,
        }
    }
}

impl<S: Stream> Stream for Fuse<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let this = self.project();
        if *this.done {
            Poll::Ready(None)
        } else {
            let next = futures_core::ready!(this.stream.poll_next(cx));
            if next.is_none() {
                *this.done = true;
            }
            Poll::Ready(next)
        }
    }
}
