use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};
use crate::utils::{timer_after, Timer};

pin_project! {
    /// A stream that only yields one element once every `duration`.
    ///
    /// This `struct` is created by the [`throttle`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`throttle`]: trait.Stream.html#method.throttle
    /// [`Stream`]: trait.Stream.html
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct Throttle<S> {
        #[pin]
        stream: S,
        duration: Duration,
        #[pin]
        blocked: bool,
        #[pin]
        delay: Timer,
    }
}

impl<S: Stream> Throttle<S> {
    pub(super) fn new(stream: S, duration: Duration) -> Self {
        Self {
            stream,
            duration,
            blocked: false,
            delay: timer_after(Duration::default()),
        }
    }
}

impl<S: Stream> Stream for Throttle<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let mut this = self.project();
        if *this.blocked {
            let d = this.delay.as_mut();
            if d.poll(cx).is_ready() {
                *this.blocked = false;
            } else {
                return Poll::Pending;
            }
        }

        match this.stream.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(v)) => {
                *this.blocked = true;
                let _ = std::mem::replace(&mut *this.delay, timer_after(*this.duration));
                Poll::Ready(Some(v))
            }
        }
    }
}
