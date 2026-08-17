//! Implementations of locking traits from the [`lock_api`](https://crates.io/crates/lock_api) crate.

use super::Lock;

/// A mutual exclusion primitive for protecting shared data of type `T`.
///
/// # Examples
///
/// ```
/// use saa::Mutex;
///
/// let mutex: Mutex<usize> = Mutex::new(0);
/// ```
pub type Mutex<T> = lock_api::Mutex<Lock, T>;

/// Acquires a mutex, asynchronously.
///
/// # Examples
///
/// ```
/// use saa::{Mutex, MutexGuard, lock_async};
///
/// let mutex: Mutex<usize> = Mutex::new(0);
///
/// async {
///     let mut guard: MutexGuard<usize> = lock_async(&mutex).await;
/// };
/// ```
#[inline]
pub async fn lock_async<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    unsafe {
        mutex.raw().lock_async().await;
        mutex.make_guard_unchecked()
    }
}

/// A scoped mutex guard.
///
/// # Examples
///
/// ```
/// use saa::{Mutex, MutexGuard};
///
/// let mutex: Mutex<usize> = Mutex::new(0);
/// let mut guard: MutexGuard<usize> = mutex.lock();
/// *guard += 1;
/// drop(guard);
///
/// assert_eq!(*mutex.try_lock().unwrap(), 1);
/// ```
pub type MutexGuard<'a, T> = lock_api::MutexGuard<'a, Lock, T>;

/// A reader-writer lock for protecting shared data of type `T`.
///
/// # Examples
///
/// ```
/// use saa::RwLock;
///
/// let rwlock: RwLock<usize> = RwLock::new(0);
/// ```
pub type RwLock<T> = lock_api::RwLock<Lock, T>;

/// A scoped read lock.
///
/// # Examples
///
/// ```
/// use saa::{RwLock, RwLockReadGuard};
///
/// let rwlock: RwLock<usize> = RwLock::new(0);
/// let guard: RwLockReadGuard<usize> = rwlock.read();
/// assert_eq!(*guard, 0);
/// ```
pub type RwLockReadGuard<'a, T> = lock_api::RwLockReadGuard<'a, Lock, T>;

/// Locks the [`RwLock`] with shared read access, asynchronously.
///
/// # Examples
///
/// ```
/// use saa::{RwLock, RwLockReadGuard, read_async};
///
/// let rwlock: RwLock<usize> = RwLock::new(0);
///
/// async {
///     let guard: RwLockReadGuard<usize> = read_async(&rwlock).await;
/// };
/// ```
#[inline]
pub async fn read_async<T>(rwlock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    unsafe {
        rwlock.raw().share_async().await;
        rwlock.make_read_guard_unchecked()
    }
}

/// A scoped write lock.
///
/// # Examples
///
/// ```
/// use saa::{RwLock, RwLockWriteGuard};
///
/// let rwlock: RwLock<usize> = RwLock::new(0);
/// let mut guard: RwLockWriteGuard<usize> = rwlock.write();
/// *guard += 1;
/// drop(guard);
///
/// assert_eq!(*rwlock.read(), 1);
/// ```
pub type RwLockWriteGuard<'a, T> = lock_api::RwLockWriteGuard<'a, Lock, T>;

/// Locks the [`RwLock`] with exclusive write access, asynchronously.
///
/// # Examples
///
/// ```
/// use saa::{RwLock, RwLockWriteGuard, write_async};
///
/// let rwlock: RwLock<usize> = RwLock::new(0);
///
/// async {
///     let guard: RwLockWriteGuard<usize> = write_async(&rwlock).await;
/// };
/// ```
#[inline]
pub async fn write_async<T>(rwlock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    unsafe {
        rwlock.raw().lock_async().await;
        rwlock.make_write_guard_unchecked()
    }
}

unsafe impl lock_api::RawMutex for Lock {
    const INIT: Self = Lock::new();

    type GuardMarker = lock_api::GuardSend;

    #[inline]
    fn lock(&self) {
        self.lock_sync();
    }

    #[inline]
    fn try_lock(&self) -> bool {
        self.try_lock()
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.release_lock();
    }
}

unsafe impl lock_api::RawMutexFair for Lock {
    #[inline]
    unsafe fn unlock_fair(&self) {
        self.release_lock();
    }
}

unsafe impl lock_api::RawRwLock for Lock {
    const INIT: Self = Lock::new();

    type GuardMarker = lock_api::GuardSend;

    #[inline]
    fn lock_shared(&self) {
        self.share_sync();
    }

    #[inline]
    fn try_lock_shared(&self) -> bool {
        self.try_share()
    }

    #[inline]
    unsafe fn unlock_shared(&self) {
        self.release_share();
    }

    #[inline]
    fn lock_exclusive(&self) {
        self.lock_sync();
    }

    #[inline]
    fn try_lock_exclusive(&self) -> bool {
        self.try_lock()
    }

    #[inline]
    unsafe fn unlock_exclusive(&self) {
        self.release_lock();
    }
}

unsafe impl lock_api::RawRwLockFair for Lock {
    #[inline]
    unsafe fn unlock_shared_fair(&self) {
        self.release_share();
    }

    #[inline]
    unsafe fn unlock_exclusive_fair(&self) {
        self.release_lock();
    }
}
