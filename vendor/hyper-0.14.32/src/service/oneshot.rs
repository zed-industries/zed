// TODO: Eventually to be replaced with tower_util::Oneshot.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;
use tower_service::Service;

pub(crate) fn oneshot<S, Req>(svc: S, req: Req) -> Oneshot<S, Req>
where
    S: Service<Req>,
{
    Oneshot {
        state: State::NotReady { svc, req },
    }
}

pin_project! {
    // A `Future` consuming a `Service` and request, waiting until the `Service`
    // is ready, and then calling `Service::call` with the request, and
    // waiting for that `Future`.
    #[allow(missing_debug_implementations)]
    pub struct Oneshot<S: Service<Req>, Req> {
        #[pin]
        state: State<S, Req>,
    }
}

pin_project! {
    #[project = StateProj]
    #[project_replace = StateProjOwn]
    enum State<S: Service<Req>, Req> {
        NotReady {
            svc: S,
            req: Req,
        },
        Called {
            #[pin]
            fut: S::Future,
        },
        Tmp,
    }
}

impl<S, Req> Future for Oneshot<S, Req>
where
    S: Service<Req>,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();

        loop {
            match me.state.as_mut().project() {
                StateProj::NotReady { ref mut svc, .. } => {
                    ready!(svc.poll_ready(cx))?;
                    // fallthrough out of the match's borrow
                }
                StateProj::Called { fut } => {
                    return fut.poll(cx);
                }
                StateProj::Tmp => unreachable!(),
            }

            match me.state.as_mut().project_replace(State::Tmp) {
                StateProjOwn::NotReady { mut svc, req } => {
                    me.state.set(State::Called { fut: svc.call(req) });
                }
                _ => unreachable!(),
            }
        }
    }
}
