use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that maps value of another stream with a function.
    #[derive(Debug)]
    pub struct Map<S, F> {
        #[pin]
        stream: S,
        f: F,
    }
}

impl<S, F> Map<S, F> {
    pub(crate) fn new(stream: S, f: F) -> Self {
        Self {
            stream,
            f,
        }
    }
}

impl<S, F, B> Stream for Map<S, F>
where
    S: Stream,
    F: FnMut(S::Item) -> B,
{
    type Item = B;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));
        Poll::Ready(next.map(this.f))
    }
}
