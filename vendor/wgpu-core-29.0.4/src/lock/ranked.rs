//! Lock types that enforce well-ranked lock acquisition order.
//!
//! This module's [`Mutex`] and [`RwLock` types are instrumented to check that
//! `wgpu-core` acquires locks according to their rank, to prevent deadlocks. To
//! use it, put `--cfg wgpu_validate_locks` in `RUSTFLAGS`.
//!
//! The [`LockRank`] constants in the [`lock::rank`] module describe edges in a
//! directed graph of lock acquisitions: each lock's rank says, if this is the most
//! recently acquired lock that you are still holding, then these are the locks you
//! are allowed to acquire next.
//!
//! As long as this graph doesn't have cycles, any number of threads can acquire
//! locks along paths through the graph without deadlock:
//!
//! - Assume that if a thread is holding a lock, then it will either release it,
//!   or block trying to acquire another one. No thread just sits on its locks
//!   forever for unrelated reasons. If it did, then that would be a source of
//!   deadlock "outside the system" that we can't do anything about.
//!
//! - This module asserts that threads acquire and release locks in a stack-like
//!   order: a lock is dropped only when it is the *most recently acquired* lock
//!   *still held* - call this the "youngest" lock. This stack-like ordering
//!   isn't a Rust requirement; Rust lets you drop guards in any order you like.
//!   This is a restriction we impose.
//!
//! - Consider the directed graph whose nodes are locks, and whose edges go from
//!   each lock to its permitted followers, the locks in its [`LockRank::followers`]
//!   set. The definition of the [`lock::rank`] module's [`LockRank`] constants
//!   ensures that this graph has no cycles, including trivial cycles from a node to
//!   itself.
//!
//! - This module then asserts that each thread attempts to acquire a lock only if
//!   it is among its youngest lock's permitted followers. Thus, as a thread
//!   acquires locks, it must be traversing a path through the graph along its
//!   edges.
//!
//! - Because there are no cycles in the graph, whenever one thread is blocked
//!   waiting to acquire a lock, that lock must be held by a different thread: if
//!   you were allowed to acquire a lock you already hold, that would be a cycle in
//!   the graph.
//!
//! - Furthermore, because the graph has no cycles, as we work our way from each
//!   thread to the thread it is blocked waiting for, we must eventually reach an
//!   end point: there must be some thread that is able to acquire its next lock, or
//!   that is about to release a lock.
//!
//! Thus, the system as a whole is always able to make progress: it is free of
//! deadlocks.
//!
//! Note that this validation only monitors each thread's behavior in isolation:
//! there's only thread-local state, nothing communicated between threads. So we
//! don't detect deadlocks, per se, only the potential to cause deadlocks. This
//! means that the validation is conservative, but more reproducible, since it's not
//! dependent on any particular interleaving of execution.
//!
//! [`lock::rank`]: crate::lock::rank

use core::{cell::Cell, fmt, ops, panic::Location};

use super::rank::LockRank;

pub use LockState as RankData;

/// A `Mutex` instrumented for deadlock prevention.
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
    saved: LockStateGuard,
}

std::thread_local! {
    static LOCK_STATE: Cell<LockState> = const { Cell::new(LockState::INITIAL) };
}

/// Per-thread state for the deadlock checker.
#[derive(Debug, Copy, Clone)]
pub struct LockState {
    /// The last lock we acquired, and where.
    last_acquired: Option<(LockRank, &'static Location<'static>)>,

    /// The number of locks currently held.
    ///
    /// This is used to enforce stack-like lock acquisition and release.
    depth: u32,
}

impl LockState {
    const INITIAL: LockState = LockState {
        last_acquired: None,
        depth: 0,
    };
}

/// A container that restores a [`LockState`] when dropped.
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
struct LockStateGuard(LockState);

impl Drop for LockStateGuard {
    fn drop(&mut self) {
        release(self.0)
    }
}

/// Check and record the acquisition of a lock with `new_rank`.
///
/// Check that acquiring a lock with `new_rank` is permitted at this point, and
/// update the per-thread state accordingly.
///
/// Return the `LockState` that must be restored when this thread is released.
fn acquire(new_rank: LockRank, location: &'static Location<'static>) -> LockState {
    let state = LOCK_STATE.get();
    // Initially, it's fine to acquire any lock. So we only
    // need to check when `last_acquired` is `Some`.
    if let Some((ref last_rank, ref last_location)) = state.last_acquired {
        assert!(
            last_rank.followers.contains(new_rank.bit),
            "Attempt to acquire nested mutexes in wrong order:\n\
             last locked {:<35} at {}\n\
             now locking {:<35} at {}\n\
             Locking {} after locking {} is not permitted.",
            last_rank.bit.member_name(),
            last_location,
            new_rank.bit.member_name(),
            location,
            new_rank.bit.member_name(),
            last_rank.bit.member_name(),
        );
    }
    LOCK_STATE.set(LockState {
        last_acquired: Some((new_rank, location)),
        depth: state.depth + 1,
    });
    state
}

/// Record the release of a lock whose saved state was `saved`.
///
/// Check that locks are being acquired in stacking order, and update the
/// per-thread state accordingly.
fn release(saved: LockState) {
    let prior = LOCK_STATE.replace(saved);

    // Although Rust allows mutex guards to be dropped in any
    // order, this analysis requires that locks be acquired and
    // released in stack order: the next lock to be released must be
    // the most recently acquired lock still held.
    assert_eq!(
        prior.depth,
        saved.depth + 1,
        "Lock not released in stacking order"
    );
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
            saved: LockStateGuard(saved),
        }
    }
}

impl<'a, T> ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

/// An `RwLock` instrumented for deadlock prevention.
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
    saved: LockStateGuard,
}

/// A write guard produced by locking [`RwLock`] for writing.
///
/// This is just a wrapper around a [`parking_lot::RwLockWriteGuard`], along
/// with the state needed to track lock acquisition.
///
/// For details, see [the module documentation][self].
pub struct RwLockWriteGuard<'a, T> {
    inner: parking_lot::RwLockWriteGuard<'a, T>,
    saved: LockStateGuard,
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
            saved: LockStateGuard(saved),
        }
    }

    #[track_caller]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        let saved = acquire(self.rank, Location::caller());
        RwLockWriteGuard {
            inner: self.inner.write(),
            saved: LockStateGuard(saved),
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

        this.saved.0
    }
}

impl<'a, T> RwLockWriteGuard<'a, T> {
    pub fn downgrade(this: Self) -> RwLockReadGuard<'a, T> {
        RwLockReadGuard {
            inner: parking_lot::RwLockWriteGuard::downgrade(this.inner),
            saved: this.saved,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, T> ops::Deref for RwLockReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> ops::Deref for RwLockWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> ops::DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

/// Locks can be acquired in the order indicated by their ranks.
#[test]
fn permitted() {
    use super::rank;

    let lock1 = Mutex::new(rank::PAWN, ());
    let lock2 = Mutex::new(rank::ROOK, ());

    let _guard1 = lock1.lock();
    let _guard2 = lock2.lock();
}

/// Locks can only be acquired in the order indicated by their ranks.
#[test]
#[should_panic(expected = "Locking pawn after locking rook")]
fn forbidden_unrelated() {
    use super::rank;

    let lock1 = Mutex::new(rank::ROOK, ());
    let lock2 = Mutex::new(rank::PAWN, ());

    let _guard1 = lock1.lock();
    let _guard2 = lock2.lock();
}

/// Lock acquisitions can't skip ranks.
///
/// These two locks *could* be acquired in this order, but only if other locks
/// are acquired in between them. Skipping ranks isn't allowed.
#[test]
#[should_panic(expected = "Locking knight after locking pawn")]
fn forbidden_skip() {
    use super::rank;

    let lock1 = Mutex::new(rank::PAWN, ());
    let lock2 = Mutex::new(rank::KNIGHT, ());

    let _guard1 = lock1.lock();
    let _guard2 = lock2.lock();
}

/// Locks can be acquired and released in a stack-like order.
#[test]
fn stack_like() {
    use super::rank;

    let lock1 = Mutex::new(rank::PAWN, ());
    let lock2 = Mutex::new(rank::ROOK, ());
    let lock3 = Mutex::new(rank::BISHOP, ());

    let guard1 = lock1.lock();
    let guard2 = lock2.lock();
    drop(guard2);

    let guard3 = lock3.lock();
    drop(guard3);
    drop(guard1);
}

/// Locks can only be acquired and released in a stack-like order.
#[test]
#[should_panic(expected = "Lock not released in stacking order")]
fn non_stack_like() {
    use super::rank;

    let lock1 = Mutex::new(rank::PAWN, ());
    let lock2 = Mutex::new(rank::ROOK, ());

    let guard1 = lock1.lock();
    let guard2 = lock2.lock();

    // Avoid a double panic from dropping this while unwinding due to the panic
    // we're testing for.
    core::mem::forget(guard2);

    drop(guard1);
}
