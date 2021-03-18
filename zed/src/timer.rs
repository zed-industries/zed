use smol::prelude::*;
use std::{
    pin::Pin,
    task::Poll,
    time::{Duration, Instant},
};

pub struct Repeat {
    timer: smol::Timer,
    period: Duration,
}

impl Stream for Repeat {
    type Item = Instant;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.as_mut().timer().poll(cx) {
            Poll::Ready(instant) => {
                let period = self.as_ref().period;
                self.as_mut().timer().set_after(period);
                Poll::Ready(Some(instant))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Repeat {
    fn timer(self: std::pin::Pin<&mut Self>) -> Pin<&mut smol::Timer> {
        unsafe { self.map_unchecked_mut(|s| &mut s.timer) }
    }
}

pub fn repeat(period: Duration) -> Repeat {
    Repeat {
        timer: smol::Timer::after(period),
        period,
    }
}
