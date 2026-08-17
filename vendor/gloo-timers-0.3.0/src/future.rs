//! `Future`- and `Stream`-backed timers APIs.

use crate::callback::{Interval, Timeout};

use futures_channel::{mpsc, oneshot};
use futures_core::stream::Stream;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use wasm_bindgen::prelude::*;

/// A scheduled timeout as a `Future`.
///
/// See `TimeoutFuture::new` for scheduling new timeouts.
///
/// Once scheduled, if you change your mind and don't want the timeout to fire,
/// you can `drop` the future.
///
/// A timeout future will never resolve to `Err`. Its only failure mode is when
/// the timeout is so long that it is effectively infinite and never fires.
///
/// # Example
///
/// ```no_run
/// use gloo_timers::future::TimeoutFuture;
/// use futures_util::future::{select, Either};
/// use wasm_bindgen_futures::spawn_local;
///
/// spawn_local(async {
///     match select(TimeoutFuture::new(1_000), TimeoutFuture::new(2_000)).await {
///         Either::Left((val, b)) => {
///             // Drop the `2_000` ms timeout to cancel its timeout.
///             drop(b);
///         }
///         Either::Right((a, val)) => {
///             panic!("the `1_000` ms timeout should have won this race");
///         }
///     }
/// });
/// ```
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or spawned"]
pub struct TimeoutFuture {
    _inner: Timeout,
    rx: oneshot::Receiver<()>,
}

impl TimeoutFuture {
    /// Create a new timeout future.
    ///
    /// Remember that futures do nothing unless polled or spawned, so either
    /// pass this future to `wasm_bindgen_futures::spawn_local` or use it inside
    /// another future.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use gloo_timers::future::TimeoutFuture;
    /// use wasm_bindgen_futures::spawn_local;
    ///
    /// spawn_local(async {
    ///     TimeoutFuture::new(1_000).await;
    ///     // Do stuff after one second...
    /// });
    /// ```
    pub fn new(millis: u32) -> TimeoutFuture {
        let (tx, rx) = oneshot::channel();
        let inner = Timeout::new(millis, move || {
            // if the receiver was dropped we do nothing.
            tx.send(()).unwrap_throw();
        });
        TimeoutFuture { _inner: inner, rx }
    }
}

/// Waits until the specified duration has elapsed.
///
/// # Panics
///
/// This function will panic if the specified [`Duration`] cannot be casted into a u32 in
/// milliseconds.
///
/// # Example
///
/// ```compile_fail
/// use std::time::Duration;
/// use gloo_timers::future::sleep;
///
/// sleep(Duration::from_secs(1)).await;
/// ```
pub fn sleep(dur: Duration) -> TimeoutFuture {
    let millis = u32::try_from(dur.as_millis())
        .expect_throw("failed to cast the duration into a u32 with Duration::as_millis.");

    TimeoutFuture::new(millis)
}

impl Future for TimeoutFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut self.rx), cx).map(|t| t.unwrap_throw())
    }
}
/// A scheduled interval as a `Stream`.
///
/// See `IntervalStream::new` for scheduling new intervals.
///
/// Once scheduled, if you want to stop the interval from continuing to fire,
/// you can `drop` the stream.
///
/// An interval stream will never resolve to `Err`.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled or spawned"]
pub struct IntervalStream {
    receiver: mpsc::UnboundedReceiver<()>,
    _inner: Interval,
}

impl IntervalStream {
    /// Create a new interval stream.
    ///
    /// Remember that streams do nothing unless polled or spawned, so either
    /// spawn this stream via `wasm_bindgen_futures::spawn_local` or use it inside
    /// another stream or future.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use futures_util::stream::StreamExt;
    /// use gloo_timers::future::IntervalStream;
    /// use wasm_bindgen_futures::spawn_local;
    ///
    /// spawn_local(async {
    ///     IntervalStream::new(1_000).for_each(|_| {
    ///         // Do stuff every one second...
    ///     }).await;
    /// });
    /// ```
    pub fn new(millis: u32) -> IntervalStream {
        let (sender, receiver) = mpsc::unbounded();
        let inner = Interval::new(millis, move || {
            // if the receiver was dropped we do nothing.
            sender.unbounded_send(()).unwrap_throw();
        });

        IntervalStream {
            receiver,
            _inner: inner,
        }
    }
}

impl Stream for IntervalStream {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Stream::poll_next(Pin::new(&mut self.receiver), cx)
    }
}
