use std::{
    arch::wasm32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Mutex,
    },
    task::{Poll, Waker},
};

use futures::future::poll_fn;

use super::utils::SpinLockMutex;

/// A combined sync/async synchronization primitive that allows waiting for a condition.
pub struct Signal {
    waiters: Mutex<Vec<Waker>>,
    // Starts with 0 and changes to 1 when signaled
    value: AtomicU32,
}

impl Signal {
    pub fn new() -> Self {
        Self {
            waiters: Mutex::new(Default::default()),
            value: AtomicU32::new(0),
        }
    }

    /// Sends a signal and unlocks all waiters.
    pub fn signal(&self) {
        self.value.store(1, Ordering::SeqCst);

        // Wake all blocking waiters
        unsafe {
            wasm32::memory_atomic_notify(&self.value as *const AtomicU32 as *mut i32, i32::MAX as u32);
        }

        // Wake all async waiters
        for waiter in self.waiters.lock_spin().unwrap().drain(..) {
            waiter.wake();
        }
    }

    /// Synchronously waits until [Self::signal] is called.
    pub fn wait(&self) {
        while self.value.load(Ordering::Relaxed) == 0 {
            unsafe {
                wasm32::memory_atomic_wait32(&self.value as *const AtomicU32 as *mut i32, 0, -1);
            }
        }
    }

    /// Asynchronously waits until [Self::signal] is called.
    pub async fn wait_async(&self) {
        poll_fn(|cx| {
            self.waiters.lock_spin().unwrap().push(cx.waker().clone());

            if self.value.load(Ordering::Relaxed) == 1 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}
