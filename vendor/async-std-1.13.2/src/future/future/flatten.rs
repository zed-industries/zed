use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;

use crate::future::IntoFuture;
use crate::task::{ready, Context, Poll};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct FlattenFuture<Fut1, Fut2> {
        #[pin]
        state: State<Fut1, Fut2>,
    }
}

pin_project! {
    #[project = StateProj]
    #[derive(Debug)]
    enum State<Fut1, Fut2> {
        First {
            #[pin]
            fut1: Fut1,
        },
        Second {
            #[pin]
            fut2: Fut2,
        },
        Empty,
    }
}

impl<Fut1, Fut2> FlattenFuture<Fut1, Fut2> {
    pub(crate) fn new(fut1: Fut1) -> FlattenFuture<Fut1, Fut2> {
        FlattenFuture {
            state: State::First { fut1 },
        }
    }
}

impl<Fut1> Future for FlattenFuture<Fut1, <Fut1::Output as IntoFuture>::Future>
where
    Fut1: Future,
    Fut1::Output: IntoFuture,
{
    type Output = <Fut1::Output as IntoFuture>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.project().state;
        loop {
            match state.as_mut().project() {
                StateProj::First { fut1 } => {
                    let fut2 = ready!(fut1.poll(cx)).into_future();
                    state.set(State::Second { fut2 });
                }
                StateProj::Second { fut2 } => {
                    let v = ready!(fut2.poll(cx));
                    state.set(State::Empty);
                    return Poll::Ready(v);
                }
                StateProj::Empty => panic!("polled a completed future"),
            }
        }
    }
}
