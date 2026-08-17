use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::{ready, TryFuture};
use pin_project::pin_project;

use super::{Filter, FilterBase, Internal};
use crate::generic::Either;
use crate::reject::CombineRejection;
use crate::route;

type Combined<E1, E2> = <E1 as CombineRejection<E2>>::Combined;

#[derive(Clone, Copy, Debug)]
pub struct Or<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U> FilterBase for Or<T, U>
where
    T: Filter,
    U: Filter + Clone + Send,
    U::Error: CombineRejection<T::Error>,
{
    type Extract = (Either<T::Extract, U::Extract>,);
    //type Error = <U::Error as CombineRejection<T::Error>>::Combined;
    type Error = Combined<U::Error, T::Error>;
    type Future = EitherFuture<T, U>;

    fn filter(&self, _: Internal) -> Self::Future {
        let idx = route::with(|route| route.matched_path_index());
        EitherFuture {
            state: State::First(self.first.filter(Internal), self.second.clone()),
            original_path_index: PathIndex(idx),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
pub struct EitherFuture<T: Filter, U: Filter> {
    #[pin]
    state: State<T, U>,
    original_path_index: PathIndex,
}

#[pin_project(project = StateProj)]
enum State<T: Filter, U: Filter> {
    First(#[pin] T::Future, U),
    Second(Option<T::Error>, #[pin] U::Future),
    Done,
}

#[derive(Copy, Clone)]
struct PathIndex(usize);

impl PathIndex {
    fn reset_path(&self) {
        route::with(|route| route.reset_matched_path_index(self.0));
    }
}

impl<T, U> Future for EitherFuture<T, U>
where
    T: Filter,
    U: Filter,
    U::Error: CombineRejection<T::Error>,
{
    type Output = Result<(Either<T::Extract, U::Extract>,), Combined<U::Error, T::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let pin = self.as_mut().project();
            let (err1, fut2) = match pin.state.project() {
                StateProj::First(first, second) => match ready!(first.try_poll(cx)) {
                    Ok(ex1) => {
                        return Poll::Ready(Ok((Either::A(ex1),)));
                    }
                    Err(e) => {
                        pin.original_path_index.reset_path();
                        (e, second.filter(Internal))
                    }
                },
                StateProj::Second(err1, second) => {
                    let ex2 = match ready!(second.try_poll(cx)) {
                        Ok(ex2) => Ok((Either::B(ex2),)),
                        Err(e) => {
                            pin.original_path_index.reset_path();
                            let err1 = err1.take().expect("polled after complete");
                            Err(e.combine(err1))
                        }
                    };
                    self.set(EitherFuture {
                        state: State::Done,
                        ..*self
                    });
                    return Poll::Ready(ex2);
                }
                StateProj::Done => panic!("polled after complete"),
            };

            self.set(EitherFuture {
                state: State::Second(Some(err1), fut2),
                ..*self
            });
        }
    }
}
