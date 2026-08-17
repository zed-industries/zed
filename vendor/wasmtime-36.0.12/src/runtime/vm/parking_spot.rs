//! Implements thread wait and notify primitives with `std::sync` primitives.
//!
//! This is a simplified version of the `parking_lot_core` crate.
//!
//! There are two main operations that can be performed:
//!
//! - *Parking* refers to suspending the thread while simultaneously enqueuing it
//!   on a queue keyed by some address.
//! - *Unparking* refers to dequeuing a thread from a queue keyed by some address
//!   and resuming it.

#![deny(missing_docs)]

use crate::prelude::*;
use crate::runtime::vm::{SendSyncPtr, WaitResult};
use std::collections::BTreeMap;
use std::ptr::NonNull;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::SeqCst};
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

#[derive(Default, Debug)]
struct Spot {
    head: Option<SendSyncPtr<WaiterInner>>,
    tail: Option<SendSyncPtr<WaiterInner>>,
}

/// The thread global `ParkingSpot`.
#[derive(Default, Debug)]
pub struct ParkingSpot {
    inner: Mutex<BTreeMap<u64, Spot>>,
}

#[derive(Default)]
pub struct Waiter {
    inner: Option<Box<WaiterInner>>,
}

struct WaiterInner {
    // NB: this field may be read concurrently, but is only written under the
    // lock of a `ParkingSpot`.
    thread: Thread,

    // NB: these fields are only modified/read under the lock of a
    // `ParkingSpot`.
    notified: bool,
    next: Option<SendSyncPtr<WaiterInner>>,
    prev: Option<SendSyncPtr<WaiterInner>>,
}

impl ParkingSpot {
    /// Atomically validates if `atomic == expected` and, if so, blocks the
    /// current thread.
    ///
    /// This method will first check to see if `atomic == expected` using a
    /// `SeqCst` load ordering. If the values are not equal then the method
    /// immediately returns with `WaitResult::Mismatch`. Otherwise the thread
    /// will be blocked and can only be woken up with `notify` on the same
    /// address. Note that the check-and-block operation is atomic with respect
    /// to `notify`.
    ///
    /// The optional `deadline` specified can indicate a point in time after
    /// which this thread will be unblocked. If this thread is not notified and
    /// `deadline` is reached then `WaitResult::TimedOut` is returned. If
    /// `deadline` is `None` then this thread will block forever waiting for
    /// `notify`.
    ///
    /// The `waiter` argument is metadata used by this structure to block
    /// the current thread.
    ///
    /// This method will not spuriously wake up one blocked.
    pub fn wait32(
        &self,
        atomic: &AtomicU32,
        expected: u32,
        deadline: impl Into<Option<Instant>>,
        waiter: &mut Waiter,
    ) -> WaitResult {
        self.wait(
            atomic.as_ptr() as u64,
            || atomic.load(SeqCst) == expected,
            deadline.into(),
            waiter,
        )
    }

    /// Same as `wait32`, but for 64-bit values.
    pub fn wait64(
        &self,
        atomic: &AtomicU64,
        expected: u64,
        deadline: impl Into<Option<Instant>>,
        waiter: &mut Waiter,
    ) -> WaitResult {
        self.wait(
            atomic.as_ptr() as u64,
            || atomic.load(SeqCst) == expected,
            deadline.into(),
            waiter,
        )
    }

    fn wait(
        &self,
        key: u64,
        validate: impl FnOnce() -> bool,
        deadline: Option<Instant>,
        waiter: &mut Waiter,
    ) -> WaitResult {
        let mut inner = self
            .inner
            .lock()
            .expect("failed to lock inner parking table");

        // This is the "atomic" part of the `validate` check which ensure that
        // the memory location still indicates that we're allowed to block.
        if !validate() {
            return WaitResult::Mismatch;
        }

        // Lazily initialize the `waiter` node if it hasn't been already, and
        // additionally ensure it's not accidentally in some other queue.
        let waiter = waiter.inner.get_or_insert_with(|| {
            Box::new(WaiterInner {
                next: None,
                prev: None,
                notified: false,
                thread: thread::current(),
            })
        });
        assert!(waiter.next.is_none());
        assert!(waiter.prev.is_none());

        // Clear the `notified` flag if it was previously notified and
        // configure the thread to wakeup as our own.
        waiter.notified = false;
        waiter.thread = thread::current();

        let ptr = SendSyncPtr::new(NonNull::from(&mut **waiter));
        let spot = inner.entry(key).or_insert_with(Spot::default);
        unsafe {
            // Enqueue our `waiter` in the internal queue for this spot.
            spot.push(ptr);

            // Wait for a notification to arrive. This is done through
            // `std::thread::park_timeout` by dropping the lock that is held.
            // This loop is somewhat similar to a condition variable.
            //
            // If no timeout was given then the maximum duration is effectively
            // infinite (500 billion years), otherwise the timeout is
            // calculated relative to the `deadline` specified.
            //
            // To handle spurious wakeups if the thread wakes up but a
            // notification wasn't received then the thread goes back to sleep.
            let timed_out = loop {
                let timeout = match deadline {
                    Some(deadline) => {
                        let now = Instant::now();
                        if deadline <= now {
                            break true;
                        } else {
                            deadline - now
                        }
                    }
                    None => Duration::MAX,
                };

                drop(inner);
                thread::park_timeout(timeout);
                inner = self.inner.lock().unwrap();

                if ptr.as_ref().notified {
                    break false;
                }
            };

            if timed_out {
                // If this thread timed out then it is still present in the
                // waiter queue, so remove it.
                inner.get_mut(&key).unwrap().remove(ptr);
                WaitResult::TimedOut
            } else {
                // If this node was notified then we should not be in a queue
                // at this point.
                assert!(ptr.as_ref().next.is_none());
                assert!(ptr.as_ref().prev.is_none());
                WaitResult::Ok
            }
        }
    }

    /// Notify at most `n` threads that are blocked on the given address.
    ///
    /// Returns the number of threads that were actually unparked.
    pub fn notify<T>(&self, addr: &T, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let mut unparked = 0;

        // It's known here that `n > 0` so dequeue items until `unparked`
        // equals `n` or the queue runs out. Each thread dequeued is signaled
        // that it's been notified and then woken up.
        self.with_lot(addr, |spot| unsafe {
            while let Some(mut head) = spot.pop() {
                let head = head.as_mut();
                assert!(head.next.is_none());
                head.notified = true;
                head.thread.unpark();
                unparked += 1;
                if unparked == n {
                    break;
                }
            }
        });

        unparked
    }

    fn with_lot<T, F: FnMut(&mut Spot)>(&self, addr: &T, mut f: F) {
        let key = addr as *const _ as u64;
        let mut inner = self
            .inner
            .lock()
            .expect("failed to lock inner parking table");
        if let Some(spot) = inner.get_mut(&key) {
            f(spot);
        }
    }
}

impl Waiter {
    pub const fn new() -> Waiter {
        Waiter { inner: None }
    }
}

impl Spot {
    /// Adds `waiter` to the queue at the end.
    ///
    /// # Unsafety
    ///
    /// This method is `unsafe` as it can only be invoked under the parking
    /// spot's mutex. Additionally `waiter` must be a valid pointer not already
    /// in any other queue and additionally only exclusively used by this queue
    /// now.
    unsafe fn push(&mut self, mut waiter: SendSyncPtr<WaiterInner>) {
        unsafe {
            assert!(waiter.as_ref().next.is_none());
            assert!(waiter.as_ref().prev.is_none());

            waiter.as_mut().prev = self.tail;
            match self.tail {
                Some(mut tail) => tail.as_mut().next = Some(waiter),
                None => self.head = Some(waiter),
            }
            self.tail = Some(waiter);
        }
    }

    /// Removes `waiter` from the queue.
    ///
    /// # Unsafety
    ///
    /// This method is `unsafe` as it can only be invoked under the parking
    /// spot's mutex. Additionally `waiter` must be a valid pointer in this
    /// queue.
    unsafe fn remove(&mut self, mut waiter: SendSyncPtr<WaiterInner>) {
        unsafe {
            let w = waiter.as_mut();
            match w.prev {
                Some(mut prev) => prev.as_mut().next = w.next,
                None => self.head = w.next,
            }
            match w.next {
                Some(mut next) => next.as_mut().prev = w.prev,
                None => self.tail = w.prev,
            }
            w.prev = None;
            w.next = None;
        }
    }

    /// Pops the head of the queue from this linked list to wake up a waiter.
    ///
    /// # Unsafety
    ///
    /// This method is `unsafe` as it can only be invoked under the parking
    /// spot's mutex.
    unsafe fn pop(&mut self) -> Option<SendSyncPtr<WaiterInner>> {
        let ret = self.head?;
        unsafe {
            self.remove(ret);
        }
        Some(ret)
    }

    #[cfg(test)]
    fn num_parked(&self) -> u32 {
        let mut ret = 0;
        let mut cur = self.head;
        while let Some(next) = cur {
            ret += 1;
            cur = unsafe { next.as_ref().next };
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::{ParkingSpot, Waiter};
    use crate::prelude::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn atomic_wait_notify() {
        let parking_spot = ParkingSpot::default();
        let atomic = AtomicU64::new(0);

        let wait_until_value = |val: u64, waiter: &mut Waiter| loop {
            let cur = atomic.load(Ordering::SeqCst);
            if cur == val {
                break;
            } else {
                parking_spot.wait64(&atomic, cur, None, waiter);
            }
        };

        thread::scope(|s| {
            let thread1 = s.spawn(|| {
                let mut waiter = Waiter::default();
                atomic.store(1, Ordering::SeqCst);
                parking_spot.notify(&atomic, u32::MAX);
                parking_spot.wait64(&atomic, 1, None, &mut waiter);
            });

            let thread2 = s.spawn(|| {
                let mut waiter = Waiter::default();
                wait_until_value(1, &mut waiter);
                atomic.store(2, Ordering::SeqCst);
                parking_spot.notify(&atomic, u32::MAX);
                parking_spot.wait64(&atomic, 2, None, &mut waiter);
            });

            let thread3 = s.spawn(|| {
                let mut waiter = Waiter::default();
                wait_until_value(2, &mut waiter);
                atomic.store(3, Ordering::SeqCst);
                parking_spot.notify(&atomic, u32::MAX);
                parking_spot.wait64(&atomic, 3, None, &mut waiter);
            });

            let mut waiter = Waiter::default();
            wait_until_value(3, &mut waiter);
            atomic.store(4, Ordering::SeqCst);
            parking_spot.notify(&atomic, u32::MAX);

            thread1.join().unwrap();
            thread2.join().unwrap();
            thread3.join().unwrap();
        });
    }

    mod parking_lot {
        // This is a modified version of the parking_lot_core tests,
        // which are licensed under the MIT and Apache 2.0 licenses.
        use super::*;
        use std::sync::Arc;
        use std::sync::atomic::AtomicU32;

        macro_rules! test {
            ( $( $name:ident(
                repeats: $repeats:expr,
                latches: $latches:expr,
                delay: $delay:expr,
                threads: $threads:expr,
                single_unparks: $single_unparks:expr);
            )* ) => {
                $(
                #[test]
                #[cfg_attr(miri, ignore)]
                fn $name() {
                    if std::env::var("WASMTIME_TEST_NO_HOG_MEMORY").is_ok() {
                        return;
                    }
                    let delay = Duration::from_micros($delay);
                    for _ in 0..$repeats {
                        run_parking_test($latches, delay, $threads, $single_unparks);
                    }
                })*
            };
        }

        test! {
            unpark_all_one_fast(
                repeats: 10000, latches: 1, delay: 0, threads: 1, single_unparks: 0
            );
            unpark_all_hundred_fast(
                repeats: 100, latches: 1, delay: 0, threads: 100, single_unparks: 0
            );
            unpark_one_one_fast(
                repeats: 1000, latches: 1, delay: 0, threads: 1, single_unparks: 1
            );
            unpark_one_hundred_fast(
                repeats: 20, latches: 1, delay: 0, threads: 100, single_unparks: 100
            );
            unpark_one_fifty_then_fifty_all_fast(
                repeats: 50, latches: 1, delay: 0, threads: 100, single_unparks: 50
            );
            unpark_all_one(
                repeats: 100, latches: 1, delay: 10000, threads: 1, single_unparks: 0
            );
            unpark_all_hundred(
                repeats: 100, latches: 1, delay: 10000, threads: 100, single_unparks: 0
            );
            unpark_one_one(
                repeats: 10, latches: 1, delay: 10000, threads: 1, single_unparks: 1
            );
            unpark_one_fifty(
                repeats: 1, latches: 1, delay: 10000, threads: 50, single_unparks: 50
            );
            unpark_one_fifty_then_fifty_all(
                repeats: 2, latches: 1, delay: 10000, threads: 100, single_unparks: 50
            );
            hundred_unpark_all_one_fast(
                repeats: 100, latches: 100, delay: 0, threads: 1, single_unparks: 0
            );
            hundred_unpark_all_one(
                repeats: 1, latches: 100, delay: 10000, threads: 1, single_unparks: 0
            );
        }

        fn run_parking_test(
            num_latches: usize,
            delay: Duration,
            num_threads: u32,
            num_single_unparks: u32,
        ) {
            let spot = ParkingSpot::default();

            thread::scope(|s| {
                let mut tests = Vec::with_capacity(num_latches);

                for _ in 0..num_latches {
                    let test = Arc::new(SingleLatchTest::new(num_threads, &spot));
                    let mut threads = Vec::with_capacity(num_threads as _);
                    for _ in 0..num_threads {
                        let test = test.clone();
                        threads.push(s.spawn(move || test.run()));
                    }
                    tests.push((test, threads));
                }

                for unpark_index in 0..num_single_unparks {
                    thread::sleep(delay);
                    for (test, _) in &tests {
                        test.unpark_one(unpark_index);
                    }
                }

                for (test, threads) in tests {
                    test.finish(num_single_unparks);
                    for thread in threads {
                        thread.join().expect("Test thread panic");
                    }
                }
            });
        }

        struct SingleLatchTest<'a> {
            semaphore: AtomicU32,
            num_awake: AtomicU32,
            /// Total number of threads participating in this test.
            num_threads: u32,
            spot: &'a ParkingSpot,
        }

        impl<'a> SingleLatchTest<'a> {
            pub fn new(num_threads: u32, spot: &'a ParkingSpot) -> Self {
                Self {
                    // This implements a fair (FIFO) semaphore, and it starts out unavailable.
                    semaphore: AtomicU32::new(0),
                    num_awake: AtomicU32::new(0),
                    num_threads,
                    spot,
                }
            }

            pub fn run(&self) {
                // Get one slot from the semaphore
                self.down();

                self.num_awake.fetch_add(1, Ordering::SeqCst);
            }

            pub fn unpark_one(&self, _single_unpark_index: u32) {
                let num_awake_before_up = self.num_awake.load(Ordering::SeqCst);

                self.up();

                // Wait for a parked thread to wake up and update num_awake + last_awoken.
                while self.num_awake.load(Ordering::SeqCst) != num_awake_before_up + 1 {
                    thread::yield_now();
                }
            }

            pub fn finish(&self, num_single_unparks: u32) {
                // The amount of threads not unparked via unpark_one
                let mut num_threads_left =
                    self.num_threads.checked_sub(num_single_unparks).unwrap();

                // Wake remaining threads up with unpark_all. Has to be in a loop, because there might
                // still be threads that has not yet parked.
                while num_threads_left > 0 {
                    let mut num_waiting_on_address = 0;
                    self.spot.with_lot(&self.semaphore, |thread_data| {
                        num_waiting_on_address = thread_data.num_parked();
                    });
                    assert!(num_waiting_on_address <= num_threads_left);

                    let num_awake_before_unpark = self.num_awake.load(Ordering::SeqCst);

                    let num_unparked = self.spot.notify(&self.semaphore, u32::MAX);
                    assert!(num_unparked >= num_waiting_on_address);
                    assert!(num_unparked <= num_threads_left);

                    // Wait for all unparked threads to wake up and update num_awake + last_awoken.
                    while self.num_awake.load(Ordering::SeqCst)
                        != num_awake_before_unpark + num_unparked
                    {
                        thread::yield_now();
                    }

                    num_threads_left = num_threads_left.checked_sub(num_unparked).unwrap();
                }
                // By now, all threads should have been woken up
                assert_eq!(self.num_awake.load(Ordering::SeqCst), self.num_threads);

                // Make sure no thread is parked on our semaphore address
                let mut num_waiting_on_address = 0;
                self.spot.with_lot(&self.semaphore, |thread_data| {
                    num_waiting_on_address = thread_data.num_parked();
                });
                assert_eq!(num_waiting_on_address, 0);
            }

            pub fn down(&self) {
                let mut old_semaphore_value = self.semaphore.fetch_sub(1, Ordering::SeqCst);

                if (old_semaphore_value as i32) > 0 {
                    // We acquired the semaphore. Done.
                    return;
                }

                // Force this thread to go to sleep.
                let mut waiter = Waiter::new();
                loop {
                    match self
                        .spot
                        .wait32(&self.semaphore, old_semaphore_value, None, &mut waiter)
                    {
                        crate::runtime::vm::WaitResult::Mismatch => {}
                        _ => break,
                    }
                    old_semaphore_value = self.semaphore.load(Ordering::SeqCst);
                }
            }

            pub fn up(&self) {
                let old_semaphore_value = self.semaphore.fetch_add(1, Ordering::SeqCst) as i32;

                // Check if anyone was waiting on the semaphore. If they were, then pass ownership to them.
                if old_semaphore_value < 0 {
                    // We need to continue until we have actually unparked someone. It might be that
                    // the thread we want to pass ownership to has decremented the semaphore counter,
                    // but not yet parked.
                    loop {
                        match self.spot.notify(&self.semaphore, 1) {
                            1 => break,
                            0 => (),
                            i => panic!("Should not wake up {i} threads"),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn wait_with_timeout() {
        let parking_spot = ParkingSpot::default();
        let atomic = AtomicU64::new(0);

        thread::scope(|s| {
            const N: u64 = 5;
            const M: u64 = if cfg!(miri) { 10 } else { 1000 };

            let thread = s.spawn(|| {
                let mut waiter = Waiter::new();
                loop {
                    let cur = atomic.load(Ordering::SeqCst);
                    if cur == N * M {
                        break;
                    }
                    let timeout = Instant::now() + Duration::from_millis(1);
                    parking_spot.wait64(&atomic, cur, Some(timeout), &mut waiter);
                }
            });

            let mut threads = vec![thread];
            for _ in 0..N {
                threads.push(s.spawn(|| {
                    for _ in 0..M {
                        atomic.fetch_add(1, Ordering::SeqCst);
                        parking_spot.notify(&atomic, 1);
                    }
                }));
            }

            for thread in threads {
                thread.join().unwrap();
            }
        });
    }
}
