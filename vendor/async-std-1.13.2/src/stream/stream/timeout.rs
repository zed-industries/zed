use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};
use crate::utils::{timer_after, Timer};

pin_project! {
    /// A stream with timeout time set
    #[derive(Debug)]
    pub struct Timeout<S: Stream> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        duration: Duration,
    }
}

impl<S: Stream> Timeout<S> {
    pub(crate) fn new(stream: S, dur: Duration) -> Self {
        let delay = timer_after(dur);

        Self { stream, delay, duration: dur }
    }
}

impl<S: Stream> Stream for Timeout<S> {
    type Item = Result<S::Item, TimeoutError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        let r = match this.stream.poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => match this.delay.as_mut().poll(cx) {
                Poll::Ready(_) => Poll::Ready(Some(Err(TimeoutError { _private: () }))),
                Poll::Pending => return Poll::Pending,
            },
        };

        *this.delay.as_mut() = timer_after(*this.duration);

        r
    }
}

/// An error returned when a stream times out.
#[cfg_attr(feature = "docs", doc(cfg(unstable)))]
#[cfg(any(feature = "unstable", feature = "docs"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeoutError {
    _private: (),
}

impl Error for TimeoutError {}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "stream has timed out".fmt(f)
    }
}
