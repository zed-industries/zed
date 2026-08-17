use super::{LockResult, MutexGuard};
use crate::rt;

use std::sync::PoisonError;
use std::time::Duration;

/// Mock implementation of `std::sync::Condvar`.
#[derive(Debug)]
pub struct Condvar {
    object: rt::Condvar,
}

/// A type indicating whether a timed wait on a condition variable returned due
/// to a time out or not.
#[derive(Debug)]
pub struct WaitTimeoutResult(bool);

impl Condvar {
    /// Creates a new condition variable which is ready to be waited on and notified.
    pub fn new() -> Condvar {
        Condvar {
            object: rt::Condvar::new(),
        }
    }

    /// Blocks the current thread until this condition variable receives a notification.
    #[track_caller]
    pub fn wait<'a, T>(&self, mut guard: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>> {
        // Release the RefCell borrow guard allowing another thread to lock the
        // data
        guard.unborrow();

        // Wait until notified
        self.object.wait(guard.rt(), location!());

        // Borrow the mutex guarded data again
        guard.reborrow();

        Ok(guard)
    }

    /// Waits on this condition variable for a notification, timing out after a
    /// specified duration.
    pub fn wait_timeout<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        _dur: Duration,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)> {
        // TODO: implement timing out
        self.wait(guard)
            .map(|guard| (guard, WaitTimeoutResult(false)))
            .map_err(|err| PoisonError::new((err.into_inner(), WaitTimeoutResult(false))))
    }

    /// Wakes up one blocked thread on this condvar.
    #[track_caller]
    pub fn notify_one(&self) {
        self.object.notify_one(location!());
    }

    /// Wakes up all blocked threads on this condvar.
    #[track_caller]
    pub fn notify_all(&self) {
        self.object.notify_all(location!());
    }
}

impl WaitTimeoutResult {
    /// Returns `true` if the wait was known to have timed out.
    pub fn timed_out(&self) -> bool {
        self.0
    }
}

impl Default for Condvar {
    fn default() -> Self {
        Self::new()
    }
}
