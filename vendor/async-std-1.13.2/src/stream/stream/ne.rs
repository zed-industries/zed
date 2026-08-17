use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use super::fuse::Fuse;
use crate::stream::stream::StreamExt;
use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    // Lexicographically compares the elements of this `Stream` with those
    // of another.
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct NeFuture<L: Stream, R: Stream> {
        #[pin]
        l: Fuse<L>,
        #[pin]
        r: Fuse<R>,
    }
}

impl<L: Stream, R: Stream> NeFuture<L, R>
where
    L::Item: PartialEq<R::Item>,
{
    pub(super) fn new(l: L, r: R) -> Self {
        Self {
            l: l.fuse(),
            r: r.fuse(),
        }
    }
}

impl<L: Stream, R: Stream> Future for NeFuture<L, R>
where
    L: Stream + Sized,
    R: Stream + Sized,
    L::Item: PartialEq<R::Item>,
{
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        loop {
            let l_val = futures_core::ready!(this.l.as_mut().poll_next(cx));
            let r_val = futures_core::ready!(this.r.as_mut().poll_next(cx));

            if this.l.done || this.r.done {
                return Poll::Ready(false);
            }

            match (l_val, r_val) {
                (Some(l), Some(r)) if l == r => {
                    continue;
                }
                _ => {
                    return Poll::Ready(true);
                }
            }
        }
    }
}
