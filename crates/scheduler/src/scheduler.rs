mod clock;
mod executor;
mod test_scheduler;
#[cfg(test)]
mod tests;

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
    fn block(&self, future: LocalBoxFuture<()>, timeout: Option<Duration>);
    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable);
    fn schedule_background(&self, runnable: Runnable);
    fn timer(&self, timeout: Duration) -> Timer;
    fn is_main_thread(&self) -> bool;
}

impl dyn Scheduler {
    pub fn block_on<Fut: Future>(&self, future: Fut) -> Fut::Output {
        let mut output = None;
        self.block(async { output = Some(future.await) }.boxed_local(), None);
        output.unwrap()
    }

    pub fn block_with_timeout<Fut: Unpin + Future>(
        &self,
        future: &mut Fut,
        timeout: Duration,
    ) -> Option<Fut::Output> {
        let mut output = None;
        self.block(
            async { output = Some(future.await) }.boxed_local(),
            Some(timeout),
        );
        output
    }
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
