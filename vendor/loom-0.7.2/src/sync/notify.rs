use crate::rt;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

/// Implements the park / unpark pattern directly using Loom's internal
/// primitives.
///
/// Notification establishes an acquire / release synchronization point.
///
/// Using this type is useful to mock out constructs when using loom tests.
#[derive(Debug)]
pub struct Notify {
    object: rt::Notify,

    /// Enforces the single waiter invariant
    waiting: AtomicBool,
}

impl Notify {
    /// Create a new `Notify`.
    pub fn new() -> Notify {
        Notify {
            object: rt::Notify::new(false, true),
            waiting: AtomicBool::new(false),
        }
    }

    /// Notify the waiter
    #[track_caller]
    pub fn notify(&self) {
        self.object.notify(location!());
    }

    /// Wait for a notification
    #[track_caller]
    pub fn wait(&self) {
        self.waiting
            .compare_exchange(false, true, SeqCst, SeqCst)
            .expect("only a single thread may wait on `Notify`");

        self.object.wait(location!());
        self.waiting.store(false, SeqCst);
    }
}

impl Default for Notify {
    fn default() -> Self {
        Self::new()
    }
}
