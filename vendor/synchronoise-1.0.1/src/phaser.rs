//! Support module for `WriterReaderPhaser` and related structs.
//!
//! See the documentation of the [`WriterReaderPhaser`] struct for more information.
//!
//! [`WriterReaderPhaser`]: struct.WriterReaderPhaser.html

use std::sync::{Arc, Mutex, LockResult, MutexGuard};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::isize::MIN as ISIZE_MIN;
use std::time::Duration;
use std::thread;

/// A synchronization primitive that allows for multiple concurrent wait-free "writer critical
/// sections" and a "reader phase flip" that can wait for all currently-active writers to finish.
///
/// The basic interaction setup for a `WriterReaderPhaser` is as follows:
///
/// * Any number of writers can open and close a "writer critical section" with no waiting.
/// * Zero or one readers can be active at one time, by holding a "read lock". Any reader who
///   wishes to open a "read lock" while another one is active is blocked until the previous one
///   finishes.
/// * The holder of a read lock may request a "phase flip", which causes the reader to wait until
///   all current writer critical sections are finished before continuing.
///
/// `WriterReaderPhaser` is a port of the primitive of the same name from `HdrHistogram`. For a
/// summary of the rationale behind its design, see [this post by its author][wrp-blog]. Part of
/// its assumptions is that this primitive is synchronizing access to a double-buffered set of
/// counters, and the readers are expected to swap the buffers while holding a read lock but before
/// flipping the phase. This allows them to access a stable sample to read and perform calculations
/// from, while writers still have wait-free synchronization.
///
/// [wrp-blog]: https://stuff-gil-says.blogspot.com/2014/11/writerreaderphaser-story-about-new.html
///
/// "Writer critical sections" and "read locks" are represented by guard structs that allow
/// scope-based resource management of the counters and locks.
///
/// * The `PhaserCriticalSection` atomically increments and decrements the phase counters upon
///   creation and drop. These operations use `std::sync::atomic::AtomicIsize` from the standard
///   library, and provide no-wait handling for platforms with atomic addition instructions.
/// * The `PhaserReadLock` is kept in the `WriterReaderPhaser` as a Mutex, enforcing the mutual
///   exclusion of the read lock. The "phase flip" operation is defined on the read lock guard
///   itself, enforcing that only the holder of a read lock can execute one.
pub struct WriterReaderPhaser {
    start_epoch: Arc<AtomicIsize>,
    even_end_epoch: Arc<AtomicIsize>,
    odd_end_epoch: Arc<AtomicIsize>,
    read_lock: Mutex<PhaserReadLock>,
}

/// Guard struct that represents a "writer critical section" for a `WriterReaderPhaser`.
///
/// `PhaserCriticalSection` is a scope-based guard to signal the beginning and end of a "writer
/// critical section" to the phaser. Upon calling `writer_critical_section`, the phaser atomically
/// increments a counter, and when the returned `PhaserCriticalSection` drops, the `drop` call
/// atomically increments another counter. On platforms with atomic increment instructions, this
/// should result in wait-free synchronization.
///
/// # Example
///
/// ```
/// # let phaser = synchronoise::WriterReaderPhaser::new();
/// {
///     let _guard = phaser.writer_critical_section();
///     // perform writes
/// } // _guard drops, signaling the end of the section
/// ```
pub struct PhaserCriticalSection {
    end_epoch: Arc<AtomicIsize>,
}

/// Upon drop, a `PhaserCriticalSection` will signal its parent `WriterReaderPhaser` that the
/// critical section has ended.
impl Drop for PhaserCriticalSection {
    fn drop(&mut self) {
        self.end_epoch.fetch_add(1, Ordering::Release);
    }
}

/// Guard struct for a `WriterReaderPhaser` that allows a reader to perform a "phase flip".
///
/// The `PhaserReadLock` struct allows one to perform a "phase flip" on its parent
/// `WriterReaderPhaser`. It is held in a `std::sync::Mutex` in its parent phaser, enforcing that
/// only one reader may be active at once.
///
/// The `flip_phase` call performs a spin-wait while waiting the the currently-active writers to
/// finish. A sleep time may be added between checks by calling `flip_with_sleep` instead.
///
/// # Example
///
/// ```
/// # let phaser = synchronoise::WriterReaderPhaser::new();
/// {
///     let lock = phaser.read_lock().unwrap();
///     // swap buffers
///     lock.flip_phase();
///     // reader now has access to a stable snapshot
/// } // lock drops, relinquishing the read lock and allowing another reader to lock
/// ```
pub struct PhaserReadLock {
    start_epoch: Arc<AtomicIsize>,
    even_end_epoch: Arc<AtomicIsize>,
    odd_end_epoch: Arc<AtomicIsize>,
}

impl WriterReaderPhaser {
    /// Creates a new `WriterReaderPhaser`.
    pub fn new() -> WriterReaderPhaser {
        let start = Arc::new(AtomicIsize::new(0));
        let even = Arc::new(AtomicIsize::new(0));
        let odd = Arc::new(AtomicIsize::new(ISIZE_MIN));
        let read_lock = PhaserReadLock {
            start_epoch: start.clone(),
            even_end_epoch: even.clone(),
            odd_end_epoch: odd.clone(),
        };

        WriterReaderPhaser {
            start_epoch: start,
            even_end_epoch: even,
            odd_end_epoch: odd,
            read_lock: Mutex::new(read_lock),
        }
    }

    /// Enters a writer critical section, returning a guard object that signals the end of the
    /// critical section upon drop.
    pub fn writer_critical_section(&self) -> PhaserCriticalSection {
        let flag = self.start_epoch.fetch_add(1, Ordering::Release);

        if flag < 0 {
            PhaserCriticalSection {
                end_epoch: self.odd_end_epoch.clone(),
            }
        } else {
            PhaserCriticalSection {
                end_epoch: self.even_end_epoch.clone(),
            }
        }
    }

    /// Enter a reader criticial section, potentially blocking until a currently active read
    /// section finishes. Returns a guard object that allows the user to flip the phase of the
    /// `WriterReaderPhaser`, and unlocks the read lock upon drop.
    ///
    /// # Errors
    ///
    /// If another reader critical section panicked while holding the read lock, this call will
    /// return an error once the lock is acquired. See the documentation for
    /// `std::sync::Mutex::lock` for details.
    pub fn read_lock(&self) -> LockResult<MutexGuard<PhaserReadLock>> {
        self.read_lock.lock()
    }
}

impl PhaserReadLock {
    /// Wait until all currently-active writer critical sections have completed.
    pub fn flip_phase(&self) {
        self.flip_with_sleep(Duration::default());
    }

    /// Wait until all currently-active writer critical sections have completed. While waiting,
    /// sleep with the given duration.
    pub fn flip_with_sleep(&self, sleep_time: Duration) {
        let next_phase_even = self.start_epoch.load(Ordering::Relaxed) < 0;

        let start_value = if next_phase_even {
            let tmp = 0;
            self.even_end_epoch.store(tmp, Ordering::Relaxed);
            tmp
        } else {
            let tmp = ISIZE_MIN;
            self.odd_end_epoch.store(tmp, Ordering::Relaxed);
            tmp
        };

        let value_at_flip = self.start_epoch.swap(start_value, Ordering::AcqRel);

        let end_epoch = if next_phase_even {
            self.odd_end_epoch.clone()
        } else {
            self.even_end_epoch.clone()
        };

        while end_epoch.load(Ordering::Relaxed) != value_at_flip {
            if sleep_time == Duration::default() {
                thread::yield_now();
            } else {
                thread::sleep(sleep_time);
            }
        }
    }
}

