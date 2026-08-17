use std::time::Duration;

use crate::ffi::*;
use crate::{context_and_function, time_after_delay, WaitTimeout};
use crate::queue::Queue;

/// A Grand Central Dispatch group.
///
/// A `Group` is a mechanism for grouping closures and monitoring them. This
/// allows for aggregate synchronization, so you can track when all the
/// closures complete, even if they are running on different queues.
#[derive(Debug)]
pub struct Group {
    ptr: dispatch_group_t,
}

impl Group {
    /// Creates a new dispatch `Group`.
    pub fn create() -> Group {
        unsafe {
            Group { ptr: dispatch_group_create() }
        }
    }

    /// Indicates that a closure has entered self, and increments the current
    /// count of outstanding tasks. Returns a `GroupGuard` that should be
    /// dropped when the closure leaves self, decrementing the count.
    pub fn enter(&self) -> GroupGuard {
        GroupGuard::new(self)
    }

    /// Submits a closure asynchronously to the given `Queue` and associates it
    /// with self.
    pub fn exec_async<F>(&self, queue: &Queue, work: F)
            where F: 'static + Send + FnOnce() {
        let (context, work) = context_and_function(work);
        unsafe {
            dispatch_group_async_f(self.ptr, queue.ptr, context, work);
        }
    }

    /// Schedules a closure to be submitted to the given `Queue` when all tasks
    /// associated with self have completed.
    /// If self is empty, the closure is submitted immediately.
    pub fn notify<F>(&self, queue: &Queue, work: F)
            where F: 'static + Send + FnOnce() {
        let (context, work) = context_and_function(work);
        unsafe {
            dispatch_group_notify_f(self.ptr, queue.ptr, context, work);
        }
    }

    /// Waits synchronously for all tasks associated with self to complete.
    pub fn wait(&self) {
        let result = unsafe {
            dispatch_group_wait(self.ptr, DISPATCH_TIME_FOREVER)
        };
        assert!(result == 0, "Dispatch group wait errored");
    }

    /// Waits for all tasks associated with self to complete within the
    /// specified duration.
    /// Returns true if the tasks completed or false if the timeout elapsed.
    pub fn wait_timeout(&self, timeout: Duration) -> Result<(), WaitTimeout> {
        let when = time_after_delay(timeout);
        let result = unsafe {
            dispatch_group_wait(self.ptr, when)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WaitTimeout { duration: timeout })
        }
    }

    /// Returns whether self is currently empty.
    pub fn is_empty(&self) -> bool {
        let result = unsafe {
            dispatch_group_wait(self.ptr, DISPATCH_TIME_NOW)
        };
        result == 0
    }
}

unsafe impl Sync for Group { }
unsafe impl Send for Group { }

impl Clone for Group {
    fn clone(&self) -> Self {
        unsafe {
            dispatch_retain(self.ptr);
        }
        Group { ptr: self.ptr }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.ptr);
        }
    }
}

/// An RAII guard which will leave a `Group` when dropped.
#[derive(Debug)]
pub struct GroupGuard {
    group: Group,
}

impl GroupGuard {
    fn new(group: &Group) -> GroupGuard {
        unsafe {
            dispatch_group_enter(group.ptr);
        }
        GroupGuard { group: group.clone() }
    }

    /// Drops self, leaving the `Group`.
    pub fn leave(self) { }
}

impl Clone for GroupGuard {
    fn clone(&self) -> Self {
        GroupGuard::new(&self.group)
    }
}

impl Drop for GroupGuard {
    fn drop(&mut self) {
        unsafe {
            dispatch_group_leave(self.group.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::{Queue, QueueAttribute};
    use super::Group;

    #[test]
    fn test_group() {
        let group = Group::create();
        let q = Queue::create("", QueueAttribute::Serial);
        let num = Arc::new(Mutex::new(0));

        let num2 = num.clone();
        group.exec_async(&q, move || {
            let mut num = num2.lock().unwrap();
            *num += 1;
        });

        let guard = group.enter();
        assert!(!group.is_empty());
        let num3 = num.clone();
        q.exec_async(move || {
            let mut num = num3.lock().unwrap();
            *num += 1;
            guard.leave();
        });

        let notify_group = Group::create();
        let guard = notify_group.enter();
        let num4 = num.clone();
        group.notify(&q, move || {
            let mut num = num4.lock().unwrap();
            *num *= 5;
            guard.leave();
        });

        // Wait for the notify block to finish
        notify_group.wait();
        // If the notify ran, the group should be empty
        assert!(group.is_empty());
        // The notify must have run after the two blocks of the group
        assert_eq!(*num.lock().unwrap(), 10);
    }
}
