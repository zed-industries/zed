use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct LastFuture<S, T> {
        #[pin]
        stream: S,
        last: Option<T>,
    }
}

impl<S, T> LastFuture<S, T> {
    pub(crate) fn new(stream: S) -> Self {
        Self { stream, last: None }
    }
}

impl<S> Future for LastFuture<S, S::Item>
where
    S: Stream + Unpin + Sized,
    S::Item: Copy,
{
    type Output = Option<S::Item>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));

        match next {
            Some(new) => {
                cx.waker().wake_by_ref();
                *this.last = Some(new);
                Poll::Pending
            }
            None => Poll::Ready(*this.last),
        }
    }
}
