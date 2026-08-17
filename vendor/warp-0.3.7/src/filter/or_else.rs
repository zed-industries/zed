use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::{ready, TryFuture};
use pin_project::pin_project;

use super::{Filter, FilterBase, Func, Internal};
use crate::reject::IsReject;
use crate::route;

#[derive(Clone, Copy, Debug)]
pub struct OrElse<T, F> {
    pub(super) filter: T,
    pub(super) callback: F,
}

impl<T, F> FilterBase for OrElse<T, F>
where
    T: Filter,
    F: Func<T::Error> + Clone + Send,
    F::Output: TryFuture<Ok = T::Extract> + Send,
    <F::Output as TryFuture>::Error: IsReject,
{
    type Extract = <F::Output as TryFuture>::Ok;
    type Error = <F::Output as TryFuture>::Error;
    type Future = OrElseFuture<T, F>;
    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        let idx = route::with(|route| route.matched_path_index());
        OrElseFuture {
            state: State::First(self.filter.filter(Internal), self.callback.clone()),
            original_path_index: PathIndex(idx),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct OrElseFuture<T: Filter, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture<Ok = T::Extract> + Send,
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
    F::Output: TryFuture<Ok = T::Extract> + Send,
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

impl<T, F> Future for OrElseFuture<T, F>
where
    T: Filter,
    F: Func<T::Error>,
    F::Output: TryFuture<Ok = T::Extract> + Send,
{
    type Output = Result<<F::Output as TryFuture>::Ok, <F::Output as TryFuture>::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let pin = self.as_mut().project();
            let (err, second) = match pin.state.project() {
                StateProj::First(first, second) => match ready!(first.try_poll(cx)) {
                    Ok(ex) => return Poll::Ready(Ok(ex)),
                    Err(err) => (err, second),
                },
                StateProj::Second(second) => {
                    let ex2 = ready!(second.try_poll(cx));
                    self.set(OrElseFuture {
                        state: State::Done,
                        ..*self
                    });
                    return Poll::Ready(ex2);
                }
                StateProj::Done => panic!("polled after complete"),
            };

            pin.original_path_index.reset_path();
            let fut2 = second.call(err);
            self.set(OrElseFuture {
                state: State::Second(fut2),
                ..*self
            });
        }
    }
}
