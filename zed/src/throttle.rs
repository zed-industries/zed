use core::time;
use futures_core::{Future, Stream};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use time::Duration;

pub struct Throttled<S: Stream> {
    period: Duration,
    stream: S,
    timer: Option<smol::Timer>,
}

pub fn throttled<S: Stream + Unpin>(period: Duration, stream: S) -> impl Stream<Item = S::Item> {
    Throttled {
        period,
        stream,
        timer: None,
    }
}

impl<S: Stream + Unpin> Stream for Throttled<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(timer) = self.as_mut().timer() {
            if let Poll::Pending = timer.poll(cx) {
                return Poll::Pending;
            } else {
                self.as_mut().get_mut().timer = None;
            }
        }

        let mut stream = self.as_mut().stream();
        let mut last_item = None;
        loop {
            match stream.as_mut().poll_next(cx) {
                Poll::Ready(None) => {
                    return Poll::Ready(None);
                }
                Poll::Ready(Some(item)) => last_item = Some(item),
                Poll::Pending => break,
            }
        }

        if let Some(last_item) = last_item {
            self.get_mut().timer = Some(smol::Timer::after(self.period));
            Poll::Ready(Some(last_item))
        } else {
            Poll::Pending
        }
    }
}

impl<S: Stream> Throttled<S> {
    fn stream(self: Pin<&mut Self>) -> Pin<&mut S> {
        unsafe { self.map_unchecked_mut(|s| &mut s.stream) }
    }

    fn timer(self: Pin<&mut Self>) -> Option<Pin<&mut smol::Timer>> {
        if self.timer.is_some() {
            Some(unsafe { self.map_unchecked_mut(|s| s.timer.as_mut().unwrap()) })
        } else {
            None
        }
    }
}
