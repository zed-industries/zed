use std::future::Future;
use std::pin::Pin;

use crate::future::MaybeDone;
use pin_project_lite::pin_project;

use crate::task::{Context, Poll};

pin_project! {
    #[allow(missing_docs)]
    #[allow(missing_debug_implementations)]
    pub struct Race<L, R>
    where
        L: Future,
        R: Future<Output = L::Output>
    {
        #[pin] left: MaybeDone<L>,
        #[pin] right: MaybeDone<R>,
    }
}

impl<L, R> Race<L, R>
where
    L: Future,
    R: Future<Output = L::Output>,
{
    pub(crate) fn new(left: L, right: R) -> Self {
        Self {
            left: MaybeDone::new(left),
            right: MaybeDone::new(right),
        }
    }
}

impl<L, R> Future for Race<L, R>
where
    L: Future,
    R: Future<Output = L::Output>,
{
    type Output = L::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let mut left = this.left;
        if Future::poll(Pin::new(&mut left), cx).is_ready() {
            return Poll::Ready(left.take().unwrap());
        }

        let mut right = this.right;
        if Future::poll(Pin::new(&mut right), cx).is_ready() {
            return Poll::Ready(right.take().unwrap());
        }

        Poll::Pending
    }
}
