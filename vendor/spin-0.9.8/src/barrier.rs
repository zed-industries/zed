//! Synchronization primitive allowing multiple threads to synchronize the
//! beginning of some computation.
//!
//! Implementation adapted from the 'Barrier' type of the standard library. See:
//! <https://doc.rust-lang.org/std/sync/struct.Barrier.html>
//!
//! Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//! file at the top-level directory of this distribution and at
//! <http://rust-lang.org/COPYRIGHT>.
//!
//! Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
//! <http://www.apache.org/licenses/LICENSE-2.0>> or the MIT license
//! <LICENSE-MIT or <http://opensource.org/licenses/MIT>>, at your
//! option. This file may not be copied, modified, or distributed
//! except according to those terms.

use crate::{mutex::Mutex, RelaxStrategy, Spin};

/// A primitive that synchronizes the execution of multiple threads.
///
/// # Example
///
/// ```
/// use spin;
/// use std::sync::Arc;
/// use std::thread;
///
/// let mut handles = Vec::with_capacity(10);
/// let barrier = Arc::new(spin::Barrier::new(10));
/// for _ in 0..10 {
///     let c = barrier.clone();
///     // The same messages will be printed together.
///     // You will NOT see any interleaving.
///     handles.push(thread::spawn(move|| {
///         println!("before wait");
///         c.wait();
///         println!("after wait");
///     }));
/// }
/// // Wait for other threads to finish.
/// for handle in handles {
///     handle.join().unwrap();
/// }
/// ```
pub struct Barrier<R = Spin> {
    lock: Mutex<BarrierState, R>,
    num_threads: usize,
}

// The inner state of a double barrier
struct BarrierState {
    count: usize,
    generation_id: usize,
}

/// A `BarrierWaitResult` is returned by [`wait`] when all threads in the [`Barrier`]
/// have rendezvoused.
///
/// [`wait`]: struct.Barrier.html#method.wait
/// [`Barrier`]: struct.Barrier.html
///
/// # Examples
///
/// ```
/// use spin;
///
/// let barrier = spin::Barrier::new(1);
/// let barrier_wait_result = barrier.wait();
/// ```
pub struct BarrierWaitResult(bool);

impl<R: RelaxStrategy> Barrier<R> {
    /// Blocks the current thread until all threads have rendezvoused here.
    ///
    /// Barriers are re-usable after all threads have rendezvoused once, and can
    /// be used continuously.
    ///
    /// A single (arbitrary) thread will receive a [`BarrierWaitResult`] that
    /// returns `true` from [`is_leader`] when returning from this function, and
    /// all other threads will receive a result that will return `false` from
    /// [`is_leader`].
    ///
    /// [`BarrierWaitResult`]: struct.BarrierWaitResult.html
    /// [`is_leader`]: struct.BarrierWaitResult.html#method.is_leader
    ///
    /// # Examples
    ///
    /// ```
    /// use spin;
    /// use std::sync::Arc;
    /// use std::thread;
    ///
    /// let mut handles = Vec::with_capacity(10);
    /// let barrier = Arc::new(spin::Barrier::new(10));
    /// for _ in 0..10 {
    ///     let c = barrier.clone();
    ///     // The same messages will be printed together.
    ///     // You will NOT see any interleaving.
    ///     handles.push(thread::spawn(move|| {
    ///         println!("before wait");
    ///         c.wait();
    ///         println!("after wait");
    ///     }));
    /// }
    /// // Wait for other threads to finish.
    /// for handle in handles {
    ///     handle.join().unwrap();
    /// }
    /// ```
    pub fn wait(&self) -> BarrierWaitResult {
        let mut lock = self.lock.lock();
        lock.count += 1;

        if lock.count < self.num_threads {
            // not the leader
            let local_gen = lock.generation_id;

            while local_gen == lock.generation_id && lock.count < self.num_threads {
                drop(lock);
                R::relax();
                lock = self.lock.lock();
            }
            BarrierWaitResult(false)
        } else {
            // this thread is the leader,
            //   and is responsible for incrementing the generation
            lock.count = 0;
            lock.generation_id = lock.generation_id.wrapping_add(1);
            BarrierWaitResult(true)
        }
    }
}

impl<R> Barrier<R> {
    /// Creates a new barrier that can block a given number of threads.
    ///
    /// A barrier will block `n`-1 threads which call [`wait`] and then wake up
    /// all threads at once when the `n`th thread calls [`wait`]. A Barrier created
    /// with n = 0 will behave identically to one created with n = 1.
    ///
    /// [`wait`]: #method.wait
    ///
    /// # Examples
    ///
    /// ```
    /// use spin;
    ///
    /// let barrier = spin::Barrier::new(10);
    /// ```
    pub const fn new(n: usize) -> Self {
        Self {
            lock: Mutex::new(BarrierState {
                count: 0,
                generation_id: 0,
            }),
            num_threads: n,
        }
    }
}

impl BarrierWaitResult {
    /// Returns whether this thread from [`wait`] is the "leader thread".
    ///
    /// Only one thread will have `true` returned from their result, all other
    /// threads will have `false` returned.
    ///
    /// [`wait`]: struct.Barrier.html#method.wait
    ///
    /// # Examples
    ///
    /// ```
    /// use spin;
    ///
    /// let barrier = spin::Barrier::new(1);
    /// let barrier_wait_result = barrier.wait();
    /// println!("{:?}", barrier_wait_result.is_leader());
    /// ```
    pub fn is_leader(&self) -> bool {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::mpsc::{channel, TryRecvError};
    use std::sync::Arc;
    use std::thread;

    type Barrier = super::Barrier;

    fn use_barrier(n: usize, barrier: Arc<Barrier>) {
        let (tx, rx) = channel();

        let mut ts = Vec::new();
        for _ in 0..n - 1 {
            let c = barrier.clone();
            let tx = tx.clone();
            ts.push(thread::spawn(move || {
                tx.send(c.wait().is_leader()).unwrap();
            }));
        }

        // At this point, all spawned threads should be blocked,
        // so we shouldn't get anything from the port
        assert!(match rx.try_recv() {
            Err(TryRecvError::Empty) => true,
            _ => false,
        });

        let mut leader_found = barrier.wait().is_leader();

        // Now, the barrier is cleared and we should get data.
        for _ in 0..n - 1 {
            if rx.recv().unwrap() {
                assert!(!leader_found);
                leader_found = true;
            }
        }
        assert!(leader_found);

        for t in ts {
            t.join().unwrap();
        }
    }

    #[test]
    fn test_barrier() {
        const N: usize = 10;

        let barrier = Arc::new(Barrier::new(N));

        use_barrier(N, barrier.clone());

        // use barrier twice to ensure it is reusable
        use_barrier(N, barrier.clone());
    }
}
