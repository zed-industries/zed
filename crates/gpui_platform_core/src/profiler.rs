//! Profiler hooks used by the executor runtime.

use std::sync::{
    Arc,
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
