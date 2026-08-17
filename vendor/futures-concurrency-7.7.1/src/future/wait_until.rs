use core::future::Future;
use core::pin::Pin;
use core::task::{ready, Context, Poll};

/// Suspends a future until the specified deadline.
///
/// This `struct` is created by the [`wait_until`] method on [`FutureExt`]. See its
/// documentation for more.
///
/// [`wait_until`]: crate::future::FutureExt::wait_until
/// [`FutureExt`]: crate::future::FutureExt
#[derive(Debug)]
#[pin_project::pin_project]
#[must_use = "futures do nothing unless polled or .awaited"]
pub struct WaitUntil<F, D> {
    #[pin]
    future: F,
    #[pin]
    deadline: D,
    state: State,
}

/// The internal state
#[derive(Debug)]
enum State {
    Started,
    PollFuture,
    Completed,
}

impl<F, D> WaitUntil<F, D> {
    pub(super) fn new(future: F, deadline: D) -> Self {
        Self {
            future,
            deadline,
            state: State::Started,
        }
    }
}

impl<F: Future, D: Future> Future for WaitUntil<F, D> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.state {
                State::Started => {
                    ready!(this.deadline.as_mut().poll(cx));
                    *this.state = State::PollFuture;
                }
                State::PollFuture => {
                    let value = ready!(this.future.as_mut().poll(cx));
                    *this.state = State::Completed;
                    return Poll::Ready(value);
                }
                State::Completed => panic!("future polled after completing"),
            }
        }
    }
}
