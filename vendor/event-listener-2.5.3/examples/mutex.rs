//! A simple mutex implementation.
//!
//! This mutex exposes both blocking and async methods for acquiring a lock.

#![allow(dead_code)]

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use event_listener::Event;

/// A simple mutex.
struct Mutex<T> {
    /// Set to `true` when the mutex is locked.
    locked: AtomicBool,

    /// Blocked lock operations.
    lock_ops: Event,

    /// The inner protected data.
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a mutex.
    fn new(t: T) -> Mutex<T> {
        Mutex {
            locked: AtomicBool::new(false),
            lock_ops: Event::new(),
            data: UnsafeCell::new(t),
        }
    }

    /// Attempts to acquire a lock.
    fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if !self.locked.swap(true, Ordering::Acquire) {
            Some(MutexGuard(self))
        } else {
            None
        }
    }

    /// Blocks until a lock is acquired.
    fn lock(&self) -> MutexGuard<'_, T> {
        let mut listener = None;

        loop {
            // Attempt grabbing a lock.
            if let Some(guard) = self.try_lock() {
                return guard;
            }

            // Set up an event listener or wait for a notification.
            match listener.take() {
                None => {
                    // Start listening and then try locking again.
                    listener = Some(self.lock_ops.listen());
                }
                Some(l) => {
                    // Wait until a notification is received.
                    l.wait();
                }
            }
        }
    }

    /// Blocks until a lock is acquired or the timeout is reached.
    fn lock_timeout(&self, timeout: Duration) -> Option<MutexGuard<'_, T>> {
        let deadline = Instant::now() + timeout;
        let mut listener = None;

        loop {
            // Attempt grabbing a lock.
            if let Some(guard) = self.try_lock() {
                return Some(guard);
            }

            // Set up an event listener or wait for an event.
            match listener.take() {
                None => {
                    // Start listening and then try locking again.
                    listener = Some(self.lock_ops.listen());
                }
                Some(l) => {
                    // Wait until a notification is received.
                    if !l.wait_deadline(deadline) {
                        return None;
                    }
                }
            }
        }
    }

    /// Acquires a lock asynchronously.
    async fn lock_async(&self) -> MutexGuard<'_, T> {
        let mut listener = None;

        loop {
            // Attempt grabbing a lock.
            if let Some(guard) = self.try_lock() {
                return guard;
            }

            // Set up an event listener or wait for an event.
            match listener.take() {
                None => {
                    // Start listening and then try locking again.
                    listener = Some(self.lock_ops.listen());
                }
                Some(l) => {
                    // Wait until a notification is received.
                    l.await;
                }
            }
        }
    }
}

/// A guard holding a lock.
struct MutexGuard<'a, T>(&'a Mutex<T>);

unsafe impl<T: Send> Send for MutexGuard<'_, T> {}
unsafe impl<T: Sync> Sync for MutexGuard<'_, T> {}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.0.locked.store(false, Ordering::Release);
        self.0.lock_ops.notify(1);
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.data.get() }
    }
}

fn main() {
    const N: usize = 10;

    // A shared counter.
    let counter = Arc::new(Mutex::new(0));

    // A channel that signals when all threads are done.
    let (tx, rx) = mpsc::channel();

    // Spawn a bunch of threads incrementing the counter.
    for _ in 0..N {
        let counter = counter.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            let mut counter = counter.lock();
            *counter += 1;

            // If this is the last increment, signal that we're done.
            if *counter == N {
                tx.send(()).unwrap();
            }
        });
    }

    // Wait until the last thread increments the counter.
    rx.recv().unwrap();

    // The counter must equal the number of threads.
    assert_eq!(*counter.lock(), N);

    println!("Done!");
}
