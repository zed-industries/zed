use core::cmp::Ordering;
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
    pub struct PartialCmpFuture<L: Stream, R: Stream> {
        #[pin]
        l: Fuse<L>,
        #[pin]
        r: Fuse<R>,
        l_cache: Option<L::Item>,
        r_cache: Option<R::Item>,
    }
}

impl<L: Stream, R: Stream> PartialCmpFuture<L, R> {
    pub(super) fn new(l: L, r: R) -> Self {
        Self {
            l: l.fuse(),
            r: r.fuse(),
            l_cache: None,
            r_cache: None,
        }
    }
}

impl<L: Stream, R: Stream> Future for PartialCmpFuture<L, R>
where
    L: Stream + Sized,
    R: Stream + Sized,
    L::Item: PartialOrd<R::Item>,
{
    type Output = Option<Ordering>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            // Short circuit logic
            // Stream that completes earliest can be considered Less, etc
            let l_complete = this.l.done && this.l_cache.is_none();
            let r_complete = this.r.done && this.r_cache.is_none();

            if l_complete && r_complete {
                return Poll::Ready(Some(Ordering::Equal));
            } else if l_complete {
                return Poll::Ready(Some(Ordering::Less));
            } else if r_complete {
                return Poll::Ready(Some(Ordering::Greater));
            }

            // Get next value if possible and necessary
            if !this.l.done && this.l_cache.is_none() {
                let l_next = futures_core::ready!(this.l.as_mut().poll_next(cx));
                if let Some(item) = l_next {
                    *this.l_cache = Some(item);
                }
            }

            if !this.r.done && this.r_cache.is_none() {
                let r_next = futures_core::ready!(this.r.as_mut().poll_next(cx));
                if let Some(item) = r_next {
                    *this.r_cache = Some(item);
                }
            }

            // Compare if both values are available.
            if this.l_cache.is_some() && this.r_cache.is_some() {
                let l_value = this.l_cache.as_mut().take().unwrap();
                let r_value = this.r_cache.as_mut().take().unwrap();
                let result = l_value.partial_cmp(&r_value);

                if let Some(Ordering::Equal) = result {
                    // Reset cache to prepare for next comparison
                    *this.l_cache = None;
                    *this.r_cache = None;
                } else {
                    // Return non equal value
                    return Poll::Ready(result);
                }
            }
        }
    }
}
