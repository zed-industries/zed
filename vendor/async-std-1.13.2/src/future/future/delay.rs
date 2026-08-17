use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pin_project_lite::pin_project;

use crate::task::{Context, Poll};
use crate::utils::{timer_after, Timer};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct DelayFuture<F> {
        #[pin]
        future: F,
        #[pin]
        delay: Timer,
    }
}

impl<F> DelayFuture<F> {
    pub fn new(future: F, dur: Duration) -> DelayFuture<F> {
        let delay = timer_after(dur);

        DelayFuture { future, delay }
    }
}

impl<F: Future> Future for DelayFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.delay.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => match this.future.poll(cx) {
                Poll::Ready(v) => Poll::Ready(v),
                Poll::Pending => Poll::Pending,
            },
        }
    }
}
