use core::task::{Context, Poll};
use core::future::Future;
use core::pin::Pin;

use crate::stream::DoubleEndedStream;

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct RFindFuture<'a, S, P> {
    stream: &'a mut S,
    p: P,
}

impl<'a, S, P> RFindFuture<'a, S, P> {
    pub(super) fn new(stream: &'a mut S, p: P) -> Self {
        RFindFuture { stream, p }
    }
}

impl<S: Unpin, P> Unpin for RFindFuture<'_, S, P> {}

impl<'a, S, P> Future for RFindFuture<'a, S, P>
where
    S: DoubleEndedStream + Unpin + Sized,
    P: FnMut(&S::Item) -> bool,
{
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let item = futures_core::ready!(Pin::new(&mut *self.stream).poll_next_back(cx));

        match item {
            Some(v) if (&mut self.p)(&v) => Poll::Ready(Some(v)),
            Some(_) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            None => Poll::Ready(None),
        }
    }
}
