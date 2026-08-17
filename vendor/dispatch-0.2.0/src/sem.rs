use std::os::raw::c_long;
use std::time::Duration;

use crate::ffi::*;
use crate::{time_after_delay, WaitTimeout};

/// A counting semaphore.
#[derive(Debug)]
pub struct Semaphore {
    ptr: dispatch_semaphore_t,
}

impl Semaphore {
    /// Creates a new `Semaphore` with an initial value.
    ///
    /// A `Semaphore` created with a value greater than 0 cannot be disposed if
    /// it has been decremented below its original value. If there are more
    /// successful calls to `wait` than `signal`, the system assumes the
    /// `Semaphore` is still in use and will abort if it is disposed.
    pub fn new(value: u32) -> Self {
        let ptr = unsafe {
            dispatch_semaphore_create(value as c_long)
        };
        Semaphore { ptr }
    }

    /// Wait for (decrement) self.
    pub fn wait(&self) {
        let result = unsafe {
            dispatch_semaphore_wait(self.ptr, DISPATCH_TIME_FOREVER)
        };
        assert!(result == 0, "Dispatch semaphore wait errored");
    }

    /// Wait for (decrement) self until the specified timeout has elapsed.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<(), WaitTimeout> {
        let when = time_after_delay(timeout);
        let result = unsafe {
            dispatch_semaphore_wait(self.ptr, when)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WaitTimeout { duration: timeout })
        }
    }

    /// Signal (increment) self.
    ///
    /// If the previous value was less than zero, this method wakes a waiting thread.
    /// Returns `true` if a thread is woken or `false` otherwise.
    pub fn signal(&self) -> bool {
        unsafe {
            dispatch_semaphore_signal(self.ptr) != 0
        }
    }

    /// Wait to access a resource protected by self.
    /// This decrements self and returns a guard that increments when dropped.
    pub fn access(&self) -> SemaphoreGuard {
        self.wait();
        SemaphoreGuard::new(self.clone())
    }

    /// Wait until the specified timeout to access a resource protected by self.
    /// This decrements self and returns a guard that increments when dropped.
    pub fn access_timeout(&self, timeout: Duration)
    -> Result<SemaphoreGuard, WaitTimeout> {
        self.wait_timeout(timeout)?;
        Ok(SemaphoreGuard::new(self.clone()))
    }
}

unsafe impl Sync for Semaphore {}
unsafe impl Send for Semaphore {}

impl Clone for Semaphore {
    fn clone(&self) -> Self {
        unsafe {
            dispatch_retain(self.ptr);
        }
        Semaphore { ptr: self.ptr }
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.ptr);
        }
    }
}

/// An RAII guard which will signal a `Semaphore` when dropped.
#[derive(Debug)]
pub struct SemaphoreGuard {
    sem: Semaphore,
}

impl SemaphoreGuard {
    fn new(sem: Semaphore) -> SemaphoreGuard {
        SemaphoreGuard { sem }
    }

    /// Drops self, signaling the `Semaphore`.
    pub fn signal(self) { }
}

impl Drop for SemaphoreGuard {
    fn drop(&mut self) {
        self.sem.signal();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semaphore() {
        let sem = Semaphore::new(0);

        assert!(!sem.signal());
        sem.wait();

        assert!(sem.wait_timeout(Duration::from_millis(5)).is_err());
    }

    #[test]
    fn test_semaphore_guard() {
        let sem = Semaphore::new(1);

        let guard = sem.access();
        assert!(sem.access_timeout(Duration::from_millis(5)).is_err());
        drop(guard);
        assert!(sem.access_timeout(Duration::from_millis(5)).is_ok());
    }
}
