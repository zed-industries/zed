use crate::rt;
use crate::thread;

use std::sync::Mutex;
use std::task::Waker;

/// Mock implementation of `tokio::sync::AtomicWaker`.
#[derive(Debug)]
pub struct AtomicWaker {
    waker: Mutex<Option<Waker>>,
    object: rt::Mutex,
}

impl AtomicWaker {
    /// Create a new instance of `AtomicWaker`.
    pub fn new() -> AtomicWaker {
        AtomicWaker {
            waker: Mutex::new(None),
            object: rt::Mutex::new(false),
        }
    }

    /// Registers the current task to be notified on calls to `wake`.
    #[track_caller]
    pub fn register(&self, waker: Waker) {
        if dbg!(!self.object.try_acquire_lock(location!())) {
            waker.wake();
            // yield the task and try again... this is a spin lock.
            thread::yield_now();
            return;
        }

        *self.waker.lock().unwrap() = Some(waker);
        dbg!(self.object.release_lock());
    }

    /// Registers the current task to be woken without consuming the value.
    pub fn register_by_ref(&self, waker: &Waker) {
        self.register(waker.clone());
    }

    /// Notifies the task that last called `register`.
    pub fn wake(&self) {
        if let Some(waker) = self.take_waker() {
            waker.wake();
        }
    }

    /// Attempts to take the `Waker` value out of the `AtomicWaker` with the
    /// intention that the caller will wake the task later.
    #[track_caller]
    pub fn take_waker(&self) -> Option<Waker> {
        dbg!(self.object.acquire_lock(location!()));

        let ret = self.waker.lock().unwrap().take();

        dbg!(self.object.release_lock());

        ret
    }
}

impl Default for AtomicWaker {
    fn default() -> Self {
        AtomicWaker::new()
    }
}
