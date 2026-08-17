use core::pin::Pin;

use crate::future::Future;
use crate::stream::Stream;
use crate::task::{Context, Poll};

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct TryFoldFuture<'a, S, F, T> {
    stream: &'a mut S,
    f: F,
    acc: Option<T>,
}

impl<'a, S, F, T> Unpin for TryFoldFuture<'a, S, F, T> {}

impl<'a, S, F, T> TryFoldFuture<'a, S, F, T> {
    pub(super) fn new(stream: &'a mut S, init: T, f: F) -> Self {
        Self {
            stream,
            f,
            acc: Some(init),
        }
    }
}

impl<'a, S, F, T, E> Future for TryFoldFuture<'a, S, F, T>
where
    S: Stream + Unpin,
    F: FnMut(T, S::Item) -> Result<T, E>,
{
    type Output = Result<T, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let next = futures_core::ready!(Pin::new(&mut self.stream).poll_next(cx));

            match next {
                Some(v) => {
                    let old = self.acc.take().unwrap();
                    let new = (&mut self.f)(old, v);

                    match new {
                        Ok(o) => self.acc = Some(o),
                        Err(e) => return Poll::Ready(Err(e)),
                    }
                }
                None => return Poll::Ready(Ok(self.acc.take().unwrap())),
            }
        }
    }
}
