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
    any::Any,
    future::Future,
    panic::Location,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    thread,
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

#[derive(Clone, Copy, Debug)]
pub struct SpawnTime(pub Instant);

/// Metadata attached to runnables for debugging and profiling.
#[derive(Clone, Debug)]
pub struct RunnableMeta {
    /// The source location where the task was spawned.
    pub location: &'static Location<'static>,
    /// The moment the task was spawned.
    pub spawned: SpawnTime,
}

impl RunnableMeta {
    #[track_caller]
    pub fn new_with_callers_location() -> Self {
        Self {
            location: core::panic::Location::caller(),
            spawned: SpawnTime(Instant::now()),
        }
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

    /// Schedule a runnable on the local (session-pinned) queue for `session_id`.
    /// Runnables scheduled here run in order on whichever thread drains the
    /// session — the main thread for ordinary sessions, or a dedicated OS
    /// thread for sessions created via `spawn_dedicated_thread`.
    fn schedule_local(&self, session_id: SessionId, runnable: Runnable<RunnableMeta>);

    /// Schedule a background task with the given priority.
    fn schedule_background_with_priority(
        &self,
        runnable: Runnable<RunnableMeta>,
        priority: Priority,
    );

    /// Spawn a closure on a dedicated realtime thread for audio processing.
    fn spawn_realtime(&self, f: Box<dyn FnOnce() + Send>);

    /// Schedule a background task with default (medium) priority.
    fn schedule_background(&self, runnable: Runnable<RunnableMeta>) {
        self.schedule_background_with_priority(runnable, Priority::default());
    }

    #[track_caller]
    fn timer(&self, timeout: Duration) -> Timer;
    fn clock(&self) -> Arc<dyn Clock>;

    /// Spawn a closure on a fresh session pinned to its own [`LocalExecutor`].
    ///
    /// `PlatformScheduler` runs the closure on a new OS thread (see
    /// [`spawn_dedicated_thread`]). `TestScheduler` runs it on the test
    /// scheduler's loop alongside everything else so determinism under
    /// `TestScheduler::many` is preserved.
    ///
    /// This is the dyn-safe entry point: the closure's output is type-erased
    /// as `Box<dyn Any + Send + Sync>` so the trait stays object-safe.
    /// Callers typically reach for the type-safe wrappers on
    /// [`LocalExecutor::spawn_dedicated`] and
    /// [`BackgroundExecutor::spawn_dedicated`], which compose this method
    /// with [`Task::downcast`] to recover the closure's concrete return type.
    fn spawn_dedicated(
        self: Arc<Self>,
        f: Box<
            dyn FnOnce(
                    LocalExecutor,
                )
                    -> Pin<Box<dyn Future<Output = Box<dyn Any + Send + Sync>> + 'static>>
                + Send
                + 'static,
        >,
    ) -> Task<Box<dyn Any + Send + Sync>>;

    fn as_test(&self) -> Option<&TestScheduler> {
        None
    }
}

/// Spawn work on a fresh OS thread that's exclusive to the returned task and
/// anything spawned on the executor it provides. Blocking syscalls inside that
/// work don't disturb any other executor in the process.
///
/// `f` is called on the dedicated thread with a [`LocalExecutor`] pinned
/// to it. The future `f` returns may freely be `!Send`. The returned `Task` is
/// that future's task: dropping it cancels the root, but detached children
/// keep running until they finish. The thread shuts down once the executor and
/// every task on it are gone.
///
/// The caller is responsible for supplying a `session_id` that's distinct from
/// every other live session on `scheduler`. Concrete schedulers typically wrap
/// this in an inherent method that allocates the id from their own counter.
pub fn spawn_dedicated_thread<F, Fut>(
    session_id: SessionId,
    scheduler: Arc<dyn Scheduler>,
    f: F,
) -> Task<Fut::Output>
where
    F: FnOnce(LocalExecutor) -> Fut + Send + 'static,
    Fut: Future + 'static,
    Fut::Output: Send + 'static,
{
    let (runnable_sender, runnable_receiver) = flume::unbounded::<Runnable<RunnableMeta>>();
    let (task_sender, task_receiver) = flume::bounded::<Task<Fut::Output>>(1);

    thread::Builder::new()
        .name(format!("spawn_dedicated session {:?}", session_id))
        .spawn(move || {
            let dispatch = move |runnable: Runnable<RunnableMeta>| {
                let _ = runnable_sender.send(runnable);
            };
            let executor = LocalExecutor::new(session_id, scheduler, dispatch);
            let root_task = executor.spawn(f(executor.clone()));
            let _ = task_sender.send(root_task);
            // After this drop, every strong reference to the runnable sender
            // lives inside a spawned task or a user-held executor clone. The
            // recv loop exits once all of those are gone.
            drop(executor);

            while let Ok(runnable) = runnable_receiver.recv() {
                runnable.run();
            }
        })
        .expect("failed to spawn dedicated thread");

    task_receiver
        .recv()
        .expect("dedicated thread failed to produce root task")
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
