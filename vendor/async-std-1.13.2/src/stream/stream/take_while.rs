use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that yields elements based on a predicate.
    ///
    /// This `struct` is created by the [`take_while`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`take_while`]: trait.Stream.html#method.take_while
    /// [`Stream`]: trait.Stream.html
    #[derive(Debug)]
    pub struct TakeWhile<S, P> {
        #[pin]
        stream: S,
        predicate: P,
    }
}

impl<S, P> TakeWhile<S, P> {
    pub(super) fn new(stream: S, predicate: P) -> Self {
        Self {
            stream,
            predicate,
        }
    }
}

impl<S, P> Stream for TakeWhile<S, P>
where
    S: Stream,
    P: FnMut(&S::Item) -> bool,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));

        match next {
            Some(v) => {
                if (this.predicate)(&v) {
                    Poll::Ready(Some(v))
                } else {
                    Poll::Ready(None)
                }
            }
            None => Poll::Ready(None),
        }
    }
}
