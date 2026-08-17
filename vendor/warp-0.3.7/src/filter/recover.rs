use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::{ready, TryFuture};
use pin_project::pin_project;

use super::{Filter, FilterBase, Func, Internal};
use crate::generic::Either;
use crate::reject::IsReject;
use crate::route;

#[derive(Clone, Copy, Debug)]
pub struct Recover<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}

impl<T, F> FilterBase for Recover<T, F>
where
    T: Filter,
    F: Func<T::Error> + Clone + Send,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    type Extract = (Either<T::Extract, (<F::Output as TryFuture>::Ok,)>,);
    type Error = <F::Output as TryFuture>::Error;
    type Future = RecoverFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        let idx = route::with(|route| route.matched_path_index());
        RecoverFuture {
            state: State::First(self.filter.filter(Internal), self.callback.clone()),
            original_path_index: PathIndex(idx),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct RecoverFuture<T: Filter, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    #[pin]
    state: State<T, F>,
    original_path_index: PathIndex,
}

#[pin_project(project = StateProj)]
enum State<T, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    First(#[pin] T::Future, F),
    Second(#[pin] F::Output),
    Done,
}

#[derive(Copy, Clone)]
struct PathIndex(usize);

impl PathIndex {
    fn reset_path(&self) {
        route::with(|route| route.reset_matched_path_index(self.0));
    }
}

impl<T, F> Future for RecoverFuture<T, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    type Output = Result<
        (Either<T::Extract, (<F::Output as TryFuture>::Ok,)>,),
        <F::Output as TryFuture>::Error,
    >;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let pin = self.as_mut().project();
            let (err, second) = match pin.state.project() {
                StateProj::First(first, second) => match ready!(first.try_poll(cx)) {
                    Ok(ex) => return Poll::Ready(Ok((Either::A(ex),))),
                    Err(err) => (err, second),
                },
                StateProj::Second(second) => {
                    let ex2 = match ready!(second.try_poll(cx)) {
                        Ok(ex2) => Ok((Either::B((ex2,)),)),
                        Err(e) => Err(e),
                    };
                    self.set(RecoverFuture {
                        state: State::Done,
                        ..*self
                    });
                    return Poll::Ready(ex2);
                }
                StateProj::Done => panic!("polled after complete"),
            };

            pin.original_path_index.reset_path();
            let fut2 = second.call(err);
            self.set(RecoverFuture {
                state: State::Second(fut2),
                ..*self
            });
        }
    }
}
