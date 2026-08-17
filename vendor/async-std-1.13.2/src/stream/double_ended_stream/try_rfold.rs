use crate::future::Future;
use core::pin::Pin;
use crate::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::stream::DoubleEndedStream;

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct TryRFoldFuture<S, F, T> {
        #[pin]
        stream: S,
        f: F,
        acc: Option<T>,
    }
}

impl<S, F, T> TryRFoldFuture<S, F, T> {
    pub(super) fn new(stream: S, init: T, f: F) -> Self {
        TryRFoldFuture {
            stream,
            f,
            acc: Some(init),
        }
    }
}

impl<S, F, T, E> Future for TryRFoldFuture<S, F, T>
where
    S: DoubleEndedStream + Unpin,
    F: FnMut(T, S::Item) -> Result<T, E>,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next_back(cx));

            match next {
                Some(v) => {
                    let old = this.acc.take().unwrap();
                    let new = (this.f)(old, v);

                    match new {
                        Ok(o) => *this.acc = Some(o),
                        Err(e) => return Poll::Ready(Err(e)),
                    }
                }
                None => return Poll::Ready(Ok(this.acc.take().unwrap())),
            }
        }
    }
}
