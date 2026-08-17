//! Lock types that observe lock acquisition order.
//!
//! This module's [`Mutex`] type is instrumented to observe the
//! nesting of `wgpu-core` lock acquisitions. Whenever `wgpu-core`
//! acquires one lock while it is already holding another, we note
//! that nesting pair. This tells us what the [`LockRank::followers`]
//! set for each lock would need to include to accommodate
//! `wgpu-core`'s observed behavior.
//!
//! When `wgpu-core`'s `observe_locks` feature is enabled, if the
//! `WGPU_CORE_LOCK_OBSERVE_DIR` environment variable is set to the
//! path of an existing directory, then every thread that acquires a
//! lock in `wgpu-core` will write its own log file to that directory.
//! You can then run the `wgpu` workspace's `lock-analyzer` binary to
//! read those files and summarize the results. The output from
//! `lock-analyzer` has the same form as the lock ranks given in
//! [`lock/rank.rs`].
//!
//! If the `WGPU_CORE_LOCK_OBSERVE_DIR` environment variable is not
//! set, then no instrumentation takes place, and the locks behave
//! normally.
//!
//! To make sure we capture all acquisitions regardless of when the
//! program exits, each thread writes events directly to its log file
//! as they occur. A `write` system call is generally just a copy from
//! userspace into the kernel's buffer, so hopefully this approach
//! will still have tolerable performance.
//!
//! [`lock/rank.rs`]: ../../../src/wgpu_core/lock/rank.rs.html

use alloc::{format, string::String};
use core::{cell::RefCell, panic::Location};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

use super::rank::{LockRank, LockRankSet};
use crate::FastHashSet;

pub type RankData = Option<HeldLock>;

/// A `Mutex` instrumented for lock acquisition order observation.
///
/// This is just a wrapper around a [`parking_lot::Mutex`], along with
/// its rank in the `wgpu_core` lock ordering.
///
/// For details, see [the module documentation][self].
pub struct Mutex<T> {
    inner: parking_lot::Mutex<T>,
    rank: LockRank,
}

/// A guard produced by locking [`Mutex`].
///
/// This is just a wrapper around a [`parking_lot::MutexGuard`], along
/// with the state needed to track lock acquisition.
///
/// For details, see [the module documentation][self].
pub struct MutexGuard<'a, T> {
    inner: parking_lot::MutexGuard<'a, T>,
    _state: LockStateGuard,
}

impl<T> Mutex<T> {
    pub fn new(rank: LockRank, value: T) -> Mutex<T> {
        Mutex {
            inner: parking_lot::Mutex::new(value),
            rank,
        }
    }

    #[track_caller]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        let saved = acquire(self.rank, Location::caller());
        MutexGuard {
            inner: self.inner.lock(),
            _state: LockStateGuard { saved },
        }
    }

    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

/// An `RwLock` instrumented for lock acquisition order observation.
///
/// This is just a wrapper around a [`parking_lot::RwLock`], along with
/// its rank in the `wgpu_core` lock ordering.
///
/// For details, see [the module documentation][self].
pub struct RwLock<T> {
    inner: parking_lot::RwLock<T>,
    rank: LockRank,
}

/// A read guard produced by locking [`RwLock`] for reading.
///
/// This is just a wrapper around a [`parking_lot::RwLockReadGuard`], along with
/// the state needed to track lock acquisition.
///
/// For details, see [the module documentation][self].
pub struct RwLockReadGuard<'a, T> {
    inner: parking_lot::RwLockReadGuard<'a, T>,
    _state: LockStateGuard,
}

/// A write guard produced by locking [`RwLock`] for writing.
///
/// This is just a wrapper around a [`parking_lot::RwLockWriteGuard`], along
/// with the state needed to track lock acquisition.
///
/// For details, see [the module documentation][self].
pub struct RwLockWriteGuard<'a, T> {
    inner: parking_lot::RwLockWriteGuard<'a, T>,
    _state: LockStateGuard,
}

impl<T> RwLock<T> {
    pub fn new(rank: LockRank, value: T) -> RwLock<T> {
        RwLock {
            inner: parking_lot::RwLock::new(value),
            rank,
        }
    }

    #[track_caller]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        let saved = acquire(self.rank, Location::caller());
        RwLockReadGuard {
            inner: self.inner.read(),
            _state: LockStateGuard { saved },
        }
    }

    #[track_caller]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        let saved = acquire(self.rank, Location::caller());
        RwLockWriteGuard {
            inner: self.inner.write(),
            _state: LockStateGuard { saved },
        }
    }

    /// Force an read-unlock operation on this lock.
    ///
    /// Safety:
    /// - A read lock must be held which is not held by a guard.
    pub unsafe fn force_unlock_read(&self, data: RankData) {
        release(data);
        unsafe { self.inner.force_unlock_read() };
    }
}

impl<'a, T> RwLockReadGuard<'a, T> {
    // Forget the read guard, leaving the lock in a locked state with no guard.
    //
    // Equivalent to std::mem::forget, but preserves the information about the lock
    // rank.
    pub fn forget(this: Self) -> RankData {
        core::mem::forget(this.inner);

        this._state.saved
    }
}

impl<'a, T> RwLockWriteGuard<'a, T> {
    pub fn downgrade(this: Self) -> RwLockReadGuard<'a, T> {
        RwLockReadGuard {
            inner: parking_lot::RwLockWriteGuard::downgrade(this.inner),
            _state: this._state,
        }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, T> core::ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> core::ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> core::ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

/// A container that restores a prior per-thread lock state when dropped.
///
/// This type serves two purposes:
///
/// - Operations like `RwLockWriteGuard::downgrade` would like to be able to
///   destructure lock guards and reassemble their pieces into new guards, but
///   if the guard type itself implements `Drop`, we can't destructure it
///   without unsafe code or pointless `Option`s whose state is almost always
///   statically known.
///
/// - We can just implement `Drop` for this type once, and then use it in lock
///   guards, rather than implementing `Drop` separately for each guard type.
struct LockStateGuard {
    /// The youngest lock that was already held when we acquired this
    /// one, if any.
    saved: Option<HeldLock>,
}

impl Drop for LockStateGuard {
    fn drop(&mut self) {
        release(self.saved)
    }
}

/// Check and record the acquisition of a lock with `new_rank`.
///
/// Log the acquisition of a lock with `new_rank`, and
/// update the per-thread state accordingly.
///
/// Return the `Option<HeldLock>` state that must be restored when this lock is
/// released.
fn acquire(new_rank: LockRank, location: &'static Location<'static>) -> Option<HeldLock> {
    LOCK_STATE.with_borrow_mut(|state| match *state {
        ThreadState::Disabled => None,
        ThreadState::Initial => {
            let Ok(dir) = std::env::var("WGPU_CORE_LOCK_OBSERVE_DIR") else {
                *state = ThreadState::Disabled;
                return None;
            };

            // Create the observation log file.
            let mut log = ObservationLog::create(dir)
                .expect("Failed to open lock observation file (does the dir exist?)");

            // Log the full set of lock ranks, so that the analysis can even see
            // locks that are only acquired in isolation.
            for rank in LockRankSet::all().iter() {
                log.write_rank(rank);
            }

            // Update our state to reflect that we are logging acquisitions, and
            // that we have acquired this lock.
            *state = ThreadState::Enabled {
                held_lock: Some(HeldLock {
                    rank: new_rank,
                    location,
                }),
                log,
            };

            // Since this is the first acquisition on this thread, we know that
            // there is no prior lock held, and thus nothing to log yet.
            None
        }
        ThreadState::Enabled {
            ref mut held_lock,
            ref mut log,
        } => {
            if let Some(ref held_lock) = held_lock {
                log.write_acquisition(held_lock, new_rank, location);
            }

            held_lock.replace(HeldLock {
                rank: new_rank,
                location,
            })
        }
    })
}

/// Record the release of a lock whose saved state was `saved`.
fn release(saved: Option<HeldLock>) {
    LOCK_STATE.with_borrow_mut(|state| {
        if let ThreadState::Enabled {
            ref mut held_lock, ..
        } = *state
        {
            *held_lock = saved;
        }
    });
}

std::thread_local! {
    static LOCK_STATE: RefCell<ThreadState> = const { RefCell::new(ThreadState::Initial) };
}

/// Thread-local state for lock observation.
enum ThreadState {
    /// This thread hasn't yet checked the environment variable.
    Initial,

    /// This thread checked the environment variable, and it was
    /// unset, so this thread is not observing lock acquisitions.
    Disabled,

    /// Lock observation is enabled for this thread.
    Enabled {
        held_lock: Option<HeldLock>,
        log: ObservationLog,
    },
}

/// Information about a currently held lock.
#[derive(Debug, Copy, Clone)]
pub struct HeldLock {
    /// The lock's rank.
    rank: LockRank,

    /// Where we acquired the lock.
    location: &'static Location<'static>,
}

/// A log to which we can write observations of lock activity.
struct ObservationLog {
    /// The file to which we are logging lock observations.
    log_file: File,

    /// [`Location`]s we've seen so far.
    ///
    /// This is a hashset of raw pointers because raw pointers have
    /// the [`Eq`] and [`Hash`] relations we want: the pointer value, not
    /// the contents. There's no unsafe code in this module.
    locations_seen: FastHashSet<*const Location<'static>>,

    /// Buffer for serializing events, retained for allocation reuse.
    buffer: String,
}

impl ObservationLog {
    /// Create an observation log in `dir` for the current pid and thread.
    fn create(dir: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut path = PathBuf::from(dir.as_ref());
        path.push(format!(
            "locks-{}.{:?}.ron",
            std::process::id(),
            std::thread::current().id()
        ));
        let log_file = File::create(&path)?;
        Ok(ObservationLog {
            log_file,
            locations_seen: FastHashSet::default(),
            buffer: String::new(),
        })
    }

    /// Record the acquisition of one lock while holding another.
    ///
    /// Log that we acquired a lock of `new_rank` at `new_location` while still
    /// holding other locks, the most recently acquired of which has
    /// `older_rank`.
    fn write_acquisition(
        &mut self,
        older_lock: &HeldLock,
        new_rank: LockRank,
        new_location: &'static Location<'static>,
    ) {
        self.write_location(older_lock.location);
        self.write_location(new_location);
        self.write_action(&Action::Acquisition {
            older_rank: older_lock.rank.bit.number(),
            older_location: addr(older_lock.location),
            newer_rank: new_rank.bit.number(),
            newer_location: addr(new_location),
        });
    }

    fn write_location(&mut self, location: &'static Location<'static>) {
        if self.locations_seen.insert(location) {
            self.write_action(&Action::Location {
                address: addr(location),
                file: location.file(),
                line: location.line(),
                column: location.column(),
            });
        }
    }

    fn write_rank(&mut self, rank: LockRankSet) {
        self.write_action(&Action::Rank {
            bit: rank.number(),
            member_name: rank.member_name(),
            const_name: rank.const_name(),
        });
    }

    fn write_action(&mut self, action: &Action) {
        use std::io::Write;

        self.buffer.clear();
        ron::ser::to_writer(&mut self.buffer, &action)
            .expect("error serializing `lock::observing::Action`");
        self.buffer.push('\n');
        self.log_file
            .write_all(self.buffer.as_bytes())
            .expect("error writing `lock::observing::Action`");
    }
}

/// An action logged by a thread that is observing lock acquisition order.
///
/// Each thread's log file is a sequence of these enums, serialized
/// using the [`ron`] crate, one action per line.
///
/// Lock observation cannot assume that there will be any convenient
/// finalization point before the program exits, so in practice,
/// actions must be written immediately when they occur. This means we
/// can't, say, accumulate tables and write them out when they're
/// complete. The `lock-analyzer` binary is then responsible for
/// consolidating the data into a single table of observed transitions.
#[derive(serde::Serialize)]
enum Action {
    /// A location that we will refer to in later actions.
    ///
    /// We write one of these events the first time we see a
    /// particular `Location`. Treating this as a separate action
    /// simply lets us avoid repeating the content over and over
    /// again in every [`Acquisition`] action.
    ///
    /// [`Acquisition`]: Action::Acquisition
    Location {
        address: usize,
        file: &'static str,
        line: u32,
        column: u32,
    },

    /// A lock rank that we will refer to in later actions.
    ///
    /// We write out one these events for every lock rank at the
    /// beginning of each thread's log file. Treating this as a
    /// separate action simply lets us avoid repeating the names over
    /// and over again in every [`Acquisition`] action.
    ///
    /// [`Acquisition`]: Action::Acquisition
    Rank {
        bit: u32,
        member_name: &'static str,
        const_name: &'static str,
    },

    /// An attempt to acquire a lock while holding another lock.
    Acquisition {
        /// The number of the already acquired lock's rank.
        older_rank: u32,

        /// The source position at which we acquired it. Specifically,
        /// its `Location`'s address, as an integer.
        older_location: usize,

        /// The number of the rank of the lock we are acquiring.
        newer_rank: u32,

        /// The source position at which we are acquiring it.
        /// Specifically, its `Location`'s address, as an integer.
        newer_location: usize,
    },
}

impl LockRankSet {
    /// Return the number of this rank's first member.
    fn number(self) -> u32 {
        self.bits().trailing_zeros()
    }
}

/// Convenience for `core::ptr::from_ref(t) as usize`.
fn addr<T>(t: &T) -> usize {
    core::ptr::from_ref(t) as usize
}
