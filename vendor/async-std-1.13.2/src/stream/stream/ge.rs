use core::cmp::Ordering;
use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use super::partial_cmp::PartialCmpFuture;
use crate::stream::stream::StreamExt;
use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    // Determines if the elements of this `Stream` are lexicographically
    // greater than or equal to those of another.
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct GeFuture<L: Stream, R: Stream> {
        #[pin]
        partial_cmp: PartialCmpFuture<L, R>,
    }
}

impl<L: Stream, R: Stream> GeFuture<L, R>
where
    L::Item: PartialOrd<R::Item>,
{
    pub(super) fn new(l: L, r: R) -> Self {
        Self {
            partial_cmp: l.partial_cmp(r),
        }
    }
}

impl<L: Stream, R: Stream> Future for GeFuture<L, R>
where
    L: Stream,
    R: Stream,
    L::Item: PartialOrd<R::Item>,
{
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = futures_core::ready!(self.project().partial_cmp.poll(cx));

        match result {
            Some(Ordering::Greater) | Some(Ordering::Equal) => Poll::Ready(true),
            _ => Poll::Ready(false),
        }
    }
}
