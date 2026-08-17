use core::future::Future;
use core::pin::Pin;

use crate::stream::Stream;
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct TryForEachFuture<'a, S, F> {
    stream: &'a mut S,
    f: F,
}

impl<'a, S, F> Unpin for TryForEachFuture<'a, S, F> {}

impl<'a, S, F> TryForEachFuture<'a, S, F> {
    pub(crate) fn new(stream: &'a mut S, f: F) -> Self {
        Self { stream, f }
    }
}

impl<'a, S, F, E> Future for TryForEachFuture<'a, S, F>
where
    S: Stream + Unpin,
    F: FnMut(S::Item) -> Result<(), E>,
{
    type Output = Result<(), E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let item = futures_core::ready!(Pin::new(&mut self.stream).poll_next(cx));

            match item {
                None => return Poll::Ready(Ok(())),
                Some(v) => {
                    let res = (&mut self.f)(v);
                    if let Err(e) = res {
                        return Poll::Ready(Err(e));
                    }
                }
            }
        }
    }
}
