use crate::stream::Stream;
use crate::task::{Context, Poll};
use pin_project_lite::pin_project;
use core::pin::Pin;

pin_project! {
    /// A stream that copies the elements of an underlying stream.
    #[derive(Debug)]
    pub struct Copied<S> {
        #[pin]
        stream: S,
    }
}

impl<S> Copied<S> {
    pub(super) fn new(stream: S) -> Self {
        Self { stream }
    }
}

impl<'a, S, T: 'a> Stream for Copied<S>
where
    S: Stream<Item = &'a T>,
    T: Copy,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));
        Poll::Ready(next.copied())
    }
}
