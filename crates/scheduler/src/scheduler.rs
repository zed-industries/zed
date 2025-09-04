mod clock;
mod executor;
mod test_scheduler;
#[cfg(test)]
mod tests;

use chrono::{DateTime, Utc};
pub use clock::*;
pub use executor::*;
pub use test_scheduler::*;

use async_task::Runnable;
use futures::{FutureExt as _, channel::oneshot, future::LocalBoxFuture};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub trait Scheduler: Send + Sync {
    fn block(&self, session_id: SessionId, future: LocalBoxFuture<()>, timeout: Option<Duration>);
    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable);
    fn schedule_background(&self, runnable: Runnable);
    fn is_main_thread(&self) -> bool;
    fn timer(&self, timeout: Duration) -> Timer;
    fn now(&self) -> DateTime<Utc>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SessionId(u16);

pub struct Timer(oneshot::Receiver<()>);

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        match self.0.poll_unpin(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}
