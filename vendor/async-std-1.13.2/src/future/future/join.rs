use std::pin::Pin;

use crate::future::MaybeDone;
use pin_project_lite::pin_project;

use crate::task::{Context, Poll};
use std::future::Future;

pin_project! {
    #[allow(missing_docs)]
    #[allow(missing_debug_implementations)]
    pub struct Join<L, R>
    where
        L: Future,
        R: Future,
    {
        #[pin] left: MaybeDone<L>,
        #[pin] right: MaybeDone<R>,
    }
}

impl<L, R> Join<L, R>
where
    L: Future,
    R: Future,
{
    pub(crate) fn new(left: L, right: R) -> Self {
        Self {
            left: MaybeDone::new(left),
            right: MaybeDone::new(right),
        }
    }
}

impl<L, R> Future for Join<L, R>
where
    L: Future,
    R: Future,
{
    type Output = (L::Output, R::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let mut left = this.left;
        let mut right = this.right;

        let is_left_ready = Future::poll(Pin::new(&mut left), cx).is_ready();
        if is_left_ready && right.as_ref().output().is_some() {
            return Poll::Ready((left.take().unwrap(), right.take().unwrap()));
        }

        let is_right_ready = Future::poll(Pin::new(&mut right), cx).is_ready();
        if is_right_ready && left.as_ref().output().is_some() {
            return Poll::Ready((left.take().unwrap(), right.take().unwrap()));
        }

        Poll::Pending
    }
}
