use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::stream::Stream;
use pin_project::pin_project;

/// Delay execution of a stream once for the specified duration.
///
/// This `struct` is created by the [`wait_until`] method on [`StreamExt`]. See its
/// documentation for more.
///
/// [`wait_until`]: crate::stream::StreamExt::wait_until
/// [`StreamExt`]: crate::stream::StreamExt
#[derive(Debug)]
#[must_use = "streams do nothing unless polled or .awaited"]
#[pin_project]
pub struct WaitUntil<S, D> {
    #[pin]
    stream: S,
    #[pin]
    deadline: D,
    state: State,
}

#[derive(Debug)]
enum State {
    Timer,
    Streaming,
}

impl<S, D> WaitUntil<S, D> {
    pub(crate) fn new(stream: S, deadline: D) -> Self {
        WaitUntil {
            stream,
            deadline,
            state: State::Timer,
        }
    }
}

impl<S, D> Stream for WaitUntil<S, D>
where
    S: Stream,
    D: Future,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match this.state {
            State::Timer => match this.deadline.poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(_) => {
                    *this.state = State::Streaming;
                    this.stream.poll_next(cx)
                }
            },
            State::Streaming => this.stream.poll_next(cx),
        }
    }
}
