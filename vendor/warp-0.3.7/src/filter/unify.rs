use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::{ready, TryFuture};
use pin_project::pin_project;

use super::{Either, Filter, FilterBase, Internal, Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Unify<F> {
    pub(super) filter: F,
}

impl<F, T> FilterBase for Unify<F>
where
    F: Filter<Extract = (Either<T, T>,)>,
    T: Tuple,
{
    type Extract = T;
    type Error = F::Error;
    type Future = UnifyFuture<F::Future>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        UnifyFuture {
            inner: self.filter.filter(Internal),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct UnifyFuture<F> {
    #[pin]
    inner: F,
}

impl<F, T> Future for UnifyFuture<F>
where
    F: TryFuture<Ok = (Either<T, T>,)>,
{
    type Output = Result<T, F::Error>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(match ready!(self.project().inner.try_poll(cx))? {
            (Either::A(x),) | (Either::B(x),) => Ok(x),
        })
    }
}
