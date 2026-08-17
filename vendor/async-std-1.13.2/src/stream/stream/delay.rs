use core::future::Future;
use core::pin::Pin;
use core::time::Duration;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};
use crate::utils::{timer_after, Timer};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct Delay<S> {
        #[pin]
        stream: S,
        #[pin]
        delay: Timer,
        delay_done: bool,
    }
}

impl<S> Delay<S> {
    pub(super) fn new(stream: S, dur: Duration) -> Self {
        Delay {
            stream,
            delay: timer_after(dur),
            delay_done: false,
        }
    }
}

impl<S> Stream for Delay<S>
where
    S: Stream,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        if !*this.delay_done {
            futures_core::ready!(this.delay.poll(cx));
            *this.delay_done = true;
        }

        this.stream.poll_next(cx)
    }
}
