//! Profiler hooks used by the executor runtime.
//!
//! Platform backends bracket each dispatched runnable with
//! [`update_running_task`] and [`save_task_timing`]. The executor runtime in
//! this crate has no notion of task timing, so the framework installs its own
//! implementation through [`set_task_profiler`]; until then the calls are
//! no-ops.

use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

/// Tracks how many foreground runnables are queued on the current thread.
///
/// The executor increments this when it queues a runnable on the main thread and
/// the profiler decrements it when that runnable is polled, so the journal can
/// distinguish an idle foreground from one with outstanding work.
#[derive(Clone)]
pub struct ForegroundRunnableCounter(Arc<AtomicUsize>);

impl ForegroundRunnableCounter {
    /// Creates a counter with no queued runnables.
    pub fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    /// Records that a runnable was queued.
    pub fn queued(&self) {
        self.0.fetch_add(1, Ordering::Release);
    }

    /// Records that a queued runnable finished.
    pub fn finished(&self) {
        let _ = self
            .0
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
    }

    /// Whether any runnables are currently queued.
    pub fn has_runnables(&self) -> bool {
        self.0.load(Ordering::Acquire) > 0
    }
}

impl Default for ForegroundRunnableCounter {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static FOREGROUND_RUNNABLES: ForegroundRunnableCounter = ForegroundRunnableCounter::new();
}

/// Returns the current thread's foreground runnable counter.
pub fn foreground_runnable_counter() -> ForegroundRunnableCounter {
    FOREGROUND_RUNNABLES.with(Clone::clone)
}

/// Records that a queued foreground runnable finished on the current thread.
pub fn foreground_runnable_finished() {
    FOREGROUND_RUNNABLES.with(ForegroundRunnableCounter::finished);
}

/// Signature for recording that a runnable began polling.
pub type UpdateRunningTaskFn = fn(scheduler::SpawnTime, &'static std::panic::Location<'static>);

/// Signature for recording that the running runnable yielded.
pub type SaveTaskTimingFn = fn();

struct TaskProfilerHooks {
    update_running_task: UpdateRunningTaskFn,
    save_task_timing: SaveTaskTimingFn,
}

static TASK_PROFILER_HOOKS: OnceLock<TaskProfilerHooks> = OnceLock::new();

/// Installs the framework's task profiler hooks.
///
/// Only the first call takes effect. Until a profiler is installed,
/// [`update_running_task`] and [`save_task_timing`] do nothing.
pub fn set_task_profiler(
    update_running_task: UpdateRunningTaskFn,
    save_task_timing: SaveTaskTimingFn,
) {
    if TASK_PROFILER_HOOKS
        .set(TaskProfilerHooks {
            update_running_task,
            save_task_timing,
        })
        .is_err()
    {
        log::debug!("task profiler hooks were already installed");
    }
}

/// Records that a runnable began polling.
///
/// Platform backends call this immediately before running a dispatched runnable.
#[doc(hidden)]
#[inline]
pub fn update_running_task(
    spawned: scheduler::SpawnTime,
    location: &'static std::panic::Location<'static>,
) {
    if let Some(hooks) = TASK_PROFILER_HOOKS.get() {
        (hooks.update_running_task)(spawned, location);
    }
}

/// Records that the running runnable yielded.
///
/// Platform backends call this immediately after running a dispatched runnable.
#[doc(hidden)]
#[inline]
pub fn save_task_timing() {
    if let Some(hooks) = TASK_PROFILER_HOOKS.get() {
        (hooks.save_task_timing)();
    }
}
