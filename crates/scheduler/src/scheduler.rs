mod clock;
mod executor;
mod test_scheduler;
#[cfg(test)]
mod tests;

pub use clock::*;
pub use executor::*;
pub use test_scheduler::*;

use async_task::Runnable;
use futures::channel::oneshot;
use std::{
    future::Future,
    panic::Location,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

/// Task priority for background tasks.
///
/// Higher priority tasks are more likely to be scheduled before lower priority tasks,
/// but this is not a strict guarantee - the scheduler may interleave tasks of different
/// priorities to prevent starvation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Priority {
    /// Realtime priority
    ///
    /// Spawning a task with this priority will spin it off on a separate thread dedicated just to that task. Only use for audio.
    RealtimeAudio,
    /// High priority - use for tasks critical to user experience/responsiveness.
    High,
    /// Medium priority - suitable for most use cases.
    #[default]
    Medium,
    /// Low priority - use for background work that can be deprioritized.
    Low,
}

impl Priority {
    /// Returns the relative probability weight for this priority level.
    /// Used by schedulers to determine task selection probability.
    pub const fn weight(self) -> u32 {
        match self {
            Priority::High => 60,
            Priority::Medium => 30,
            Priority::Low => 10,
            // realtime priorities are not considered for probability scheduling
            Priority::RealtimeAudio => 0,
        }
    }
}

/// Metadata attached to runnables for debugging and profiling.
#[derive(Clone)]
pub struct RunnableMeta {
    /// The source location where the task was spawned.
    pub location: &'static Location<'static>,
    /// Shared flag indicating whether the scheduler has been closed.
    /// When true, tasks should be dropped without running.
    pub closed: Arc<AtomicBool>,
}

impl RunnableMeta {
    /// Returns true if the scheduler has been closed and this task should not run.
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }
}

impl std::fmt::Debug for RunnableMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunnableMeta")
            .field("location", &self.location)
            .field("closed", &self.is_closed())
            .finish()
    }
}

pub trait Scheduler: Send + Sync {
    /// Block until the given future completes or timeout occurs.
    ///
    /// Returns `true` if the future completed, `false` if it timed out.
    /// The future is passed as a pinned mutable reference so the caller
    /// retains ownership and can continue polling or return it on timeout.
    fn block(
        &self,
        session_id: Option<SessionId>,
        future: Pin<&mut dyn Future<Output = ()>>,
        timeout: Option<Duration>,
    ) -> bool;

    fn schedule_foreground(&self, session_id: SessionId, runnable: Runnable<RunnableMeta>);

    /// Schedule a background task with the given priority.
    fn schedule_background_with_priority(
        &self,
        runnable: Runnable<RunnableMeta>,
        priority: Priority,
    );

    /// Schedule a background task with default (medium) priority.
    fn schedule_background(&self, runnable: Runnable<RunnableMeta>) {
        self.schedule_background_with_priority(runnable, Priority::default());
    }

    fn timer(&self, timeout: Duration) -> Timer;
    fn clock(&self) -> Arc<dyn Clock>;

    fn as_test(&self) -> Option<&TestScheduler> {
        None
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SessionId(u16);

impl SessionId {
    pub fn new(id: u16) -> Self {
        SessionId(id)
    }
}

pub struct Timer(oneshot::Receiver<()>);

impl Timer {
    pub fn new(rx: oneshot::Receiver<()>) -> Self {
        Timer(rx)
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<()> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Ready(_) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        }
    }
}
