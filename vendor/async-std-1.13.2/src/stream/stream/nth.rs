use core::pin::Pin;
use core::task::{Context, Poll};
use core::future::Future;

use crate::stream::Stream;

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct NthFuture<'a, S> {
    stream: &'a mut S,
    n: usize,
}

impl<S: Unpin> Unpin for NthFuture<'_, S> {}

impl<'a, S> NthFuture<'a, S> {
    pub(crate) fn new(stream: &'a mut S, n: usize) -> Self {
        Self { stream, n }
    }
}

impl<'a, S> Future for NthFuture<'a, S>
where
    S: Stream + Unpin + Sized,
{
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let next = futures_core::ready!(Pin::new(&mut *self.stream).poll_next(cx));
        match next {
            Some(v) => match self.n {
                0 => Poll::Ready(Some(v)),
                _ => {
                    self.n -= 1;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            },
            None => Poll::Ready(None),
        }
    }
}
