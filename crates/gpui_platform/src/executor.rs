//! Executor and dispatcher vocabulary shared by `gpui` and its platform backends.

use async_task::Runnable;
use gpui_util::Deferred;
use scheduler::RunnableMeta;

/// Keeps an operating system activity, such as an idle sleep inhibitor, alive until dropped.
pub struct ActivityGuard {
    _release: Deferred<Box<dyn FnOnce() + Send>>,
}

impl ActivityGuard {
    /// Runs `release` when the guard is dropped.
    pub fn new(release: impl FnOnce() + Send + 'static) -> Self {
        Self {
            _release: gpui_util::defer(Box::new(release)),
        }
    }

    /// A guard for platforms without a corresponding activity.
    pub fn noop() -> Self {
        Self::new(|| {})
    }
}

/// Type alias for runnables with metadata.
/// Previously an enum with a single variant, now simplified to a direct type alias.
#[doc(hidden)]
pub type RunnableVariant = Runnable<RunnableMeta>;

#[doc(hidden)]
pub type TimerResolutionGuard = Deferred<Box<dyn FnOnce() + Send>>;
