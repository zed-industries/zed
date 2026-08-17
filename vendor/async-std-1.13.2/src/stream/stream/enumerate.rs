use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[derive(Debug)]
    pub struct Enumerate<S> {
        #[pin]
        stream: S,
        i: usize,
    }
}

impl<S> Enumerate<S> {
    pub(super) fn new(stream: S) -> Self {
        Self { stream, i: 0 }
    }
}

impl<S> Stream for Enumerate<S>
where
    S: Stream,
{
    type Item = (usize, S::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));

        match next {
            Some(v) => {
                let ret = (*this.i, v);
                *this.i += 1;
                Poll::Ready(Some(ret))
            }
            None => Poll::Ready(None),
        }
    }
}
