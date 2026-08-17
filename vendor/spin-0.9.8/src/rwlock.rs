//! A lock that provides data access to either one writer or many readers.

use crate::{
    atomic::{AtomicUsize, Ordering},
    RelaxStrategy, Spin,
};
use core::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    mem,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

/// A lock that provides data access to either one writer or many readers.
///
/// This lock behaves in a similar manner to its namesake `std::sync::RwLock` but uses
/// spinning for synchronisation instead. Unlike its namespace, this lock does not
/// track lock poisoning.
///
/// This type of lock allows a number of readers or at most one writer at any
/// point in time. The write portion of this lock typically allows modification
/// of the underlying data (exclusive access) and the read portion of this lock
/// typically allows for read-only access (shared access).
///
/// The type parameter `T` represents the data that this lock protects. It is
/// required that `T` satisfies `Send` to be shared across tasks and `Sync` to
/// allow concurrent access through readers. The RAII guards returned from the
/// locking methods implement `Deref` (and `DerefMut` for the `write` methods)
/// to allow access to the contained of the lock.
///
/// An [`RwLockUpgradableGuard`](RwLockUpgradableGuard) can be upgraded to a
/// writable guard through the [`RwLockUpgradableGuard::upgrade`](RwLockUpgradableGuard::upgrade)
/// [`RwLockUpgradableGuard::try_upgrade`](RwLockUpgradableGuard::try_upgrade) functions.
/// Writable or upgradeable guards can be downgraded through their respective `downgrade`
/// functions.
///
/// Based on Facebook's
/// [`folly/RWSpinLock.h`](https://github.com/facebook/folly/blob/a0394d84f2d5c3e50ebfd0566f9d3acb52cfab5a/folly/synchronization/RWSpinLock.h).
/// This implementation is unfair to writers - if the lock always has readers, then no writers will
/// ever get a chance. Using an upgradeable lock guard can *somewhat* alleviate this issue as no
/// new readers are allowed when an upgradeable guard is held, but upgradeable guards can be taken
/// when there are existing readers. However if the lock is that highly contended and writes are
/// crucial then this implementation may be a poor choice.
///
/// # Examples
///
/// ```
/// use spin;
///
/// let lock = spin::RwLock::new(5);
///
/// // many reader locks can be held at once
/// {
///     let r1 = lock.read();
///     let r2 = lock.read();
///     assert_eq!(*r1, 5);
///     assert_eq!(*r2, 5);
/// } // read locks are dropped at this point
///
/// // only one write lock may be held, however
/// {
///     let mut w = lock.write();
///     *w += 1;
///     assert_eq!(*w, 6);
/// } // write lock is dropped here
/// ```
pub struct RwLock<T: ?Sized, R = Spin> {
    phantom: PhantomData<R>,
    lock: AtomicUsize,
    data: UnsafeCell<T>,
}

const READER: usize = 1 << 2;
const UPGRADED: usize = 1 << 1;
const WRITER: usize = 1;

/// A guard that provides immutable data access.
///
/// When the guard falls out of scope it will decrement the read count,
/// potentially releasing the lock.
pub struct RwLockReadGuard<'a, T: 'a + ?Sized> {
    lock: &'a AtomicUsize,
    data: *const T,
}

/// A guard that provides mutable data access.
///
/// When the guard falls out of scope it will release the lock.
pub struct RwLockWriteGuard<'a, T: 'a + ?Sized, R = Spin> {
    phantom: PhantomData<R>,
    inner: &'a RwLock<T, R>,
    data: *mut T,
}

/// A guard that provides immutable data access but can be upgraded to [`RwLockWriteGuard`].
///
/// No writers or other upgradeable guards can exist while this is in scope. New reader
/// creation is prevented (to alleviate writer starvation) but there may be existing readers
/// when the lock is acquired.
///
/// When the guard falls out of scope it will release the lock.
pub struct RwLockUpgradableGuard<'a, T: 'a + ?Sized, R = Spin> {
    phantom: PhantomData<R>,
    inner: &'a RwLock<T, R>,
    data: *const T,
}

// Same unsafe impls as `std::sync::RwLock`
unsafe impl<T: ?Sized + Send, R> Send for RwLock<T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for RwLock<T, R> {}

unsafe impl<T: ?Sized + Send + Sync, R> Send for RwLockWriteGuard<'_, T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for RwLockWriteGuard<'_, T, R> {}

unsafe impl<T: ?Sized + Sync> Send for RwLockReadGuard<'_, T> {}
unsafe impl<T: ?Sized + Sync> Sync for RwLockReadGuard<'_, T> {}

unsafe impl<T: ?Sized + Send + Sync, R> Send for RwLockUpgradableGuard<'_, T, R> {}
unsafe impl<T: ?Sized + Send + Sync, R> Sync for RwLockUpgradableGuard<'_, T, R> {}

impl<T, R> RwLock<T, R> {
    /// Creates a new spinlock wrapping the supplied data.
    ///
    /// May be used statically:
    ///
    /// ```
    /// use spin;
    ///
    /// static RW_LOCK: spin::RwLock<()> = spin::RwLock::new(());
    ///
    /// fn demo() {
    ///     let lock = RW_LOCK.read();
    ///     // do something with lock
    ///     drop(lock);
    /// }
    /// ```
    #[inline]
    pub const fn new(data: T) -> Self {
        RwLock {
            phantom: PhantomData,
            lock: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Consumes this `RwLock`, returning the underlying data.
    #[inline]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let RwLock { data, .. } = self;
        data.into_inner()
    }
    /// Returns a mutable pointer to the underying data.
    ///
    /// This is mostly meant to be used for applications which require manual unlocking, but where
    /// storing both the lock and the pointer to the inner data gets inefficient.
    ///
    /// While this is safe, writing to the data is undefined behavior unless the current thread has
    /// acquired a write lock, and reading requires either a read or write lock.
    ///
    /// # Example
    /// ```
    /// let lock = spin::RwLock::new(42);
    ///
    /// unsafe {
    ///     core::mem::forget(lock.write());
    ///
    ///     assert_eq!(lock.as_mut_ptr().read(), 42);
    ///     lock.as_mut_ptr().write(58);
    ///
    ///     lock.force_write_unlock();
    /// }
    ///
    /// assert_eq!(*lock.read(), 58);
    ///
    /// ```
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.get()
    }
}

impl<T: ?Sized, R: RelaxStrategy> RwLock<T, R> {
    /// Locks this rwlock with shared read access, blocking the current thread
    /// until it can be acquired.
    ///
    /// The calling thread will be blocked until there are no more writers which
    /// hold the lock. There may be other readers currently inside the lock when
    /// this method returns. This method does not provide any guarantees with
    /// respect to the ordering of whether contentious readers or writers will
    /// acquire the lock first.
    ///
    /// Returns an RAII guard which will release this thread's shared access
    /// once it is dropped.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    /// {
    ///     let mut data = mylock.read();
    ///     // The lock is now locked and the data can be read
    ///     println!("{}", *data);
    ///     // The lock is dropped
    /// }
    /// ```
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<T> {
        loop {
            match self.try_read() {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }

    /// Lock this rwlock with exclusive write access, blocking the current
    /// thread until it can be acquired.
    ///
    /// This function will not return while other writers or other readers
    /// currently have access to the lock.
    ///
    /// Returns an RAII guard which will drop the write access of this rwlock
    /// when dropped.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    /// {
    ///     let mut data = mylock.write();
    ///     // The lock is now locked and the data can be written
    ///     *data += 1;
    ///     // The lock is dropped
    /// }
    /// ```
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<T, R> {
        loop {
            match self.try_write_internal(false) {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }

    /// Obtain a readable lock guard that can later be upgraded to a writable lock guard.
    /// Upgrades can be done through the [`RwLockUpgradableGuard::upgrade`](RwLockUpgradableGuard::upgrade) method.
    #[inline]
    pub fn upgradeable_read(&self) -> RwLockUpgradableGuard<T, R> {
        loop {
            match self.try_upgradeable_read() {
                Some(guard) => return guard,
                None => R::relax(),
            }
        }
    }
}

impl<T: ?Sized, R> RwLock<T, R> {
    // Acquire a read lock, returning the new lock value.
    fn acquire_reader(&self) -> usize {
        // An arbitrary cap that allows us to catch overflows long before they happen
        const MAX_READERS: usize = core::usize::MAX / READER / 2;

        let value = self.lock.fetch_add(READER, Ordering::Acquire);

        if value > MAX_READERS * READER {
            self.lock.fetch_sub(READER, Ordering::Relaxed);
            panic!("Too many lock readers, cannot safely proceed");
        } else {
            value
        }
    }

    /// Attempt to acquire this lock with shared read access.
    ///
    /// This function will never block and will return immediately if `read`
    /// would otherwise succeed. Returns `Some` of an RAII guard which will
    /// release the shared access of this thread when dropped, or `None` if the
    /// access could not be granted. This method does not provide any
    /// guarantees with respect to the ordering of whether contentious readers
    /// or writers will acquire the lock first.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    /// {
    ///     match mylock.try_read() {
    ///         Some(data) => {
    ///             // The lock is now locked and the data can be read
    ///             println!("{}", *data);
    ///             // The lock is dropped
    ///         },
    ///         None => (), // no cigar
    ///     };
    /// }
    /// ```
    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        let value = self.acquire_reader();

        // We check the UPGRADED bit here so that new readers are prevented when an UPGRADED lock is held.
        // This helps reduce writer starvation.
        if value & (WRITER | UPGRADED) != 0 {
            // Lock is taken, undo.
            self.lock.fetch_sub(READER, Ordering::Release);
            None
        } else {
            Some(RwLockReadGuard {
                lock: &self.lock,
                data: unsafe { &*self.data.get() },
            })
        }
    }

    /// Return the number of readers that currently hold the lock (including upgradable readers).
    ///
    /// # Safety
    ///
    /// This function provides no synchronization guarantees and so its result should be considered 'out of date'
    /// the instant it is called. Do not use it for synchronization purposes. However, it may be useful as a heuristic.
    pub fn reader_count(&self) -> usize {
        let state = self.lock.load(Ordering::Relaxed);
        state / READER + (state & UPGRADED) / UPGRADED
    }

    /// Return the number of writers that currently hold the lock.
    ///
    /// Because [`RwLock`] guarantees exclusive mutable access, this function may only return either `0` or `1`.
    ///
    /// # Safety
    ///
    /// This function provides no synchronization guarantees and so its result should be considered 'out of date'
    /// the instant it is called. Do not use it for synchronization purposes. However, it may be useful as a heuristic.
    pub fn writer_count(&self) -> usize {
        (self.lock.load(Ordering::Relaxed) & WRITER) / WRITER
    }

    /// Force decrement the reader count.
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if there are outstanding `RwLockReadGuard`s
    /// live, or if called more times than `read` has been called, but can be
    /// useful in FFI contexts where the caller doesn't know how to deal with
    /// RAII. The underlying atomic operation uses `Ordering::Release`.
    #[inline]
    pub unsafe fn force_read_decrement(&self) {
        debug_assert!(self.lock.load(Ordering::Relaxed) & !WRITER > 0);
        self.lock.fetch_sub(READER, Ordering::Release);
    }

    /// Force unlock exclusive write access.
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if there are outstanding `RwLockWriteGuard`s
    /// live, or if called when there are current readers, but can be useful in
    /// FFI contexts where the caller doesn't know how to deal with RAII. The
    /// underlying atomic operation uses `Ordering::Release`.
    #[inline]
    pub unsafe fn force_write_unlock(&self) {
        debug_assert_eq!(self.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED), 0);
        self.lock.fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }

    #[inline(always)]
    fn try_write_internal(&self, strong: bool) -> Option<RwLockWriteGuard<T, R>> {
        if compare_exchange(
            &self.lock,
            0,
            WRITER,
            Ordering::Acquire,
            Ordering::Relaxed,
            strong,
        )
        .is_ok()
        {
            Some(RwLockWriteGuard {
                phantom: PhantomData,
                inner: self,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }

    /// Attempt to lock this rwlock with exclusive write access.
    ///
    /// This function does not ever block, and it will return `None` if a call
    /// to `write` would otherwise block. If successful, an RAII guard is
    /// returned.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    /// {
    ///     match mylock.try_write() {
    ///         Some(mut data) => {
    ///             // The lock is now locked and the data can be written
    ///             *data += 1;
    ///             // The lock is implicitly dropped
    ///         },
    ///         None => (), // no cigar
    ///     };
    /// }
    /// ```
    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<T, R>> {
        self.try_write_internal(true)
    }

    /// Tries to obtain an upgradeable lock guard.
    #[inline]
    pub fn try_upgradeable_read(&self) -> Option<RwLockUpgradableGuard<T, R>> {
        if self.lock.fetch_or(UPGRADED, Ordering::Acquire) & (WRITER | UPGRADED) == 0 {
            Some(RwLockUpgradableGuard {
                phantom: PhantomData,
                inner: self,
                data: unsafe { &*self.data.get() },
            })
        } else {
            // We can't unflip the UPGRADED bit back just yet as there is another upgradeable or write lock.
            // When they unlock, they will clear the bit.
            None
        }
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the `RwLock` mutably, no actual locking needs to
    /// take place -- the mutable borrow statically guarantees no locks exist.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lock = spin::RwLock::new(0);
    /// *lock.get_mut() = 10;
    /// assert_eq!(*lock.read(), 10);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        // We know statically that there are no other references to `self`, so
        // there's no need to lock the inner lock.
        unsafe { &mut *self.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for RwLock<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_read() {
            Some(guard) => write!(f, "RwLock {{ data: ")
                .and_then(|()| (&*guard).fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "RwLock {{ <locked> }}"),
        }
    }
}

impl<T: ?Sized + Default, R> Default for RwLock<T, R> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, R> From<T> for RwLock<T, R> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<'rwlock, T: ?Sized> RwLockReadGuard<'rwlock, T> {
    /// Leak the lock guard, yielding a reference to the underlying data.
    ///
    /// Note that this function will permanently lock the original lock for all but reading locks.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let data: &i32 = spin::RwLockReadGuard::leak(mylock.read());
    ///
    /// assert_eq!(*data, 0);
    /// ```
    #[inline]
    pub fn leak(this: Self) -> &'rwlock T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &*this.data }
    }
}

impl<'rwlock, T: ?Sized + fmt::Debug> fmt::Debug for RwLockReadGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Display> fmt::Display for RwLockReadGuard<'rwlock, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized, R: RelaxStrategy> RwLockUpgradableGuard<'rwlock, T, R> {
    /// Upgrades an upgradeable lock guard to a writable lock guard.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let upgradeable = mylock.upgradeable_read(); // Readable, but not yet writable
    /// let writable = upgradeable.upgrade();
    /// ```
    #[inline]
    pub fn upgrade(mut self) -> RwLockWriteGuard<'rwlock, T, R> {
        loop {
            self = match self.try_upgrade_internal(false) {
                Ok(guard) => return guard,
                Err(e) => e,
            };

            R::relax();
        }
    }
}

impl<'rwlock, T: ?Sized, R> RwLockUpgradableGuard<'rwlock, T, R> {
    #[inline(always)]
    fn try_upgrade_internal(self, strong: bool) -> Result<RwLockWriteGuard<'rwlock, T, R>, Self> {
        if compare_exchange(
            &self.inner.lock,
            UPGRADED,
            WRITER,
            Ordering::Acquire,
            Ordering::Relaxed,
            strong,
        )
        .is_ok()
        {
            let inner = self.inner;

            // Forget the old guard so its destructor doesn't run (before mutably aliasing data below)
            mem::forget(self);

            // Upgrade successful
            Ok(RwLockWriteGuard {
                phantom: PhantomData,
                inner,
                data: unsafe { &mut *inner.data.get() },
            })
        } else {
            Err(self)
        }
    }

    /// Tries to upgrade an upgradeable lock guard to a writable lock guard.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    /// let upgradeable = mylock.upgradeable_read(); // Readable, but not yet writable
    ///
    /// match upgradeable.try_upgrade() {
    ///     Ok(writable) => /* upgrade successful - use writable lock guard */ (),
    ///     Err(upgradeable) => /* upgrade unsuccessful */ (),
    /// };
    /// ```
    #[inline]
    pub fn try_upgrade(self) -> Result<RwLockWriteGuard<'rwlock, T, R>, Self> {
        self.try_upgrade_internal(true)
    }

    #[inline]
    /// Downgrades the upgradeable lock guard to a readable, shared lock guard. Cannot fail and is guaranteed not to spin.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(1);
    ///
    /// let upgradeable = mylock.upgradeable_read();
    /// assert!(mylock.try_read().is_none());
    /// assert_eq!(*upgradeable, 1);
    ///
    /// let readable = upgradeable.downgrade(); // This is guaranteed not to spin
    /// assert!(mylock.try_read().is_some());
    /// assert_eq!(*readable, 1);
    /// ```
    pub fn downgrade(self) -> RwLockReadGuard<'rwlock, T> {
        // Reserve the read guard for ourselves
        self.inner.acquire_reader();

        let inner = self.inner;

        // Dropping self removes the UPGRADED bit
        mem::drop(self);

        RwLockReadGuard {
            lock: &inner.lock,
            data: unsafe { &*inner.data.get() },
        }
    }

    /// Leak the lock guard, yielding a reference to the underlying data.
    ///
    /// Note that this function will permanently lock the original lock.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let data: &i32 = spin::RwLockUpgradableGuard::leak(mylock.upgradeable_read());
    ///
    /// assert_eq!(*data, 0);
    /// ```
    #[inline]
    pub fn leak(this: Self) -> &'rwlock T {
        let this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &*this.data }
    }
}

impl<'rwlock, T: ?Sized + fmt::Debug, R> fmt::Debug for RwLockUpgradableGuard<'rwlock, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Display, R> fmt::Display for RwLockUpgradableGuard<'rwlock, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized, R> RwLockWriteGuard<'rwlock, T, R> {
    /// Downgrades the writable lock guard to a readable, shared lock guard. Cannot fail and is guaranteed not to spin.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let mut writable = mylock.write();
    /// *writable = 1;
    ///
    /// let readable = writable.downgrade(); // This is guaranteed not to spin
    /// # let readable_2 = mylock.try_read().unwrap();
    /// assert_eq!(*readable, 1);
    /// ```
    #[inline]
    pub fn downgrade(self) -> RwLockReadGuard<'rwlock, T> {
        // Reserve the read guard for ourselves
        self.inner.acquire_reader();

        let inner = self.inner;

        // Dropping self removes the UPGRADED bit
        mem::drop(self);

        RwLockReadGuard {
            lock: &inner.lock,
            data: unsafe { &*inner.data.get() },
        }
    }

    /// Downgrades the writable lock guard to an upgradable, shared lock guard. Cannot fail and is guaranteed not to spin.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let mut writable = mylock.write();
    /// *writable = 1;
    ///
    /// let readable = writable.downgrade_to_upgradeable(); // This is guaranteed not to spin
    /// assert_eq!(*readable, 1);
    /// ```
    #[inline]
    pub fn downgrade_to_upgradeable(self) -> RwLockUpgradableGuard<'rwlock, T, R> {
        debug_assert_eq!(
            self.inner.lock.load(Ordering::Acquire) & (WRITER | UPGRADED),
            WRITER
        );

        // Reserve the read guard for ourselves
        self.inner.lock.store(UPGRADED, Ordering::Release);

        let inner = self.inner;

        // Dropping self removes the UPGRADED bit
        mem::forget(self);

        RwLockUpgradableGuard {
            phantom: PhantomData,
            inner,
            data: unsafe { &*inner.data.get() },
        }
    }

    /// Leak the lock guard, yielding a mutable reference to the underlying data.
    ///
    /// Note that this function will permanently lock the original lock.
    ///
    /// ```
    /// let mylock = spin::RwLock::new(0);
    ///
    /// let data: &mut i32 = spin::RwLockWriteGuard::leak(mylock.write());
    ///
    /// *data = 1;
    /// assert_eq!(*data, 1);
    /// ```
    #[inline]
    pub fn leak(this: Self) -> &'rwlock mut T {
        let mut this = ManuallyDrop::new(this);
        // Safety: We know statically that only we are referencing data
        unsafe { &mut *this.data }
    }
}

impl<'rwlock, T: ?Sized + fmt::Debug, R> fmt::Debug for RwLockWriteGuard<'rwlock, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized + fmt::Display, R> fmt::Display for RwLockWriteGuard<'rwlock, T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'rwlock, T: ?Sized> Deref for RwLockReadGuard<'rwlock, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'rwlock, T: ?Sized, R> Deref for RwLockUpgradableGuard<'rwlock, T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'rwlock, T: ?Sized, R> Deref for RwLockWriteGuard<'rwlock, T, R> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'rwlock, T: ?Sized, R> DerefMut for RwLockWriteGuard<'rwlock, T, R> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'rwlock, T: ?Sized> Drop for RwLockReadGuard<'rwlock, T> {
    fn drop(&mut self) {
        debug_assert!(self.lock.load(Ordering::Relaxed) & !(WRITER | UPGRADED) > 0);
        self.lock.fetch_sub(READER, Ordering::Release);
    }
}

impl<'rwlock, T: ?Sized, R> Drop for RwLockUpgradableGuard<'rwlock, T, R> {
    fn drop(&mut self) {
        debug_assert_eq!(
            self.inner.lock.load(Ordering::Relaxed) & (WRITER | UPGRADED),
            UPGRADED
        );
        self.inner.lock.fetch_sub(UPGRADED, Ordering::AcqRel);
    }
}

impl<'rwlock, T: ?Sized, R> Drop for RwLockWriteGuard<'rwlock, T, R> {
    fn drop(&mut self) {
        debug_assert_eq!(self.inner.lock.load(Ordering::Relaxed) & WRITER, WRITER);

        // Writer is responsible for clearing both WRITER and UPGRADED bits.
        // The UPGRADED bit may be set if an upgradeable lock attempts an upgrade while this lock is held.
        self.inner
            .lock
            .fetch_and(!(WRITER | UPGRADED), Ordering::Release);
    }
}

#[inline(always)]
fn compare_exchange(
    atomic: &AtomicUsize,
    current: usize,
    new: usize,
    success: Ordering,
    failure: Ordering,
    strong: bool,
) -> Result<usize, usize> {
    if strong {
        atomic.compare_exchange(current, new, success, failure)
    } else {
        atomic.compare_exchange_weak(current, new, success, failure)
    }
}

#[cfg(feature = "lock_api")]
unsafe impl<R: RelaxStrategy> lock_api_crate::RawRwLock for RwLock<(), R> {
    type GuardMarker = lock_api_crate::GuardSend;

    const INIT: Self = Self::new(());

    #[inline(always)]
    fn lock_exclusive(&self) {
        // Prevent guard destructor running
        core::mem::forget(self.write());
    }

    #[inline(always)]
    fn try_lock_exclusive(&self) -> bool {
        // Prevent guard destructor running
        self.try_write().map(|g| core::mem::forget(g)).is_some()
    }

    #[inline(always)]
    unsafe fn unlock_exclusive(&self) {
        drop(RwLockWriteGuard {
            inner: self,
            data: &mut (),
            phantom: PhantomData,
        });
    }

    #[inline(always)]
    fn lock_shared(&self) {
        // Prevent guard destructor running
        core::mem::forget(self.read());
    }

    #[inline(always)]
    fn try_lock_shared(&self) -> bool {
        // Prevent guard destructor running
        self.try_read().map(|g| core::mem::forget(g)).is_some()
    }

    #[inline(always)]
    unsafe fn unlock_shared(&self) {
        drop(RwLockReadGuard {
            lock: &self.lock,
            data: &(),
        });
    }

    #[inline(always)]
    fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed) != 0
    }
}

#[cfg(feature = "lock_api")]
unsafe impl<R: RelaxStrategy> lock_api_crate::RawRwLockUpgrade for RwLock<(), R> {
    #[inline(always)]
    fn lock_upgradable(&self) {
        // Prevent guard destructor running
        core::mem::forget(self.upgradeable_read());
    }

    #[inline(always)]
    fn try_lock_upgradable(&self) -> bool {
        // Prevent guard destructor running
        self.try_upgradeable_read()
            .map(|g| core::mem::forget(g))
            .is_some()
    }

    #[inline(always)]
    unsafe fn unlock_upgradable(&self) {
        drop(RwLockUpgradableGuard {
            inner: self,
            data: &(),
            phantom: PhantomData,
        });
    }

    #[inline(always)]
    unsafe fn upgrade(&self) {
        let tmp_guard = RwLockUpgradableGuard {
            inner: self,
            data: &(),
            phantom: PhantomData,
        };
        core::mem::forget(tmp_guard.upgrade());
    }

    #[inline(always)]
    unsafe fn try_upgrade(&self) -> bool {
        let tmp_guard = RwLockUpgradableGuard {
            inner: self,
            data: &(),
            phantom: PhantomData,
        };
        tmp_guard
            .try_upgrade()
            .map(|g| core::mem::forget(g))
            .is_ok()
    }
}

#[cfg(feature = "lock_api")]
unsafe impl<R: RelaxStrategy> lock_api_crate::RawRwLockDowngrade for RwLock<(), R> {
    unsafe fn downgrade(&self) {
        let tmp_guard = RwLockWriteGuard {
            inner: self,
            data: &mut (),
            phantom: PhantomData,
        };
        core::mem::forget(tmp_guard.downgrade());
    }
}

#[cfg(feature = "lock_api1")]
unsafe impl lock_api::RawRwLockUpgradeDowngrade for RwLock<()> {
    unsafe fn downgrade_upgradable(&self) {
        let tmp_guard = RwLockUpgradableGuard {
            inner: self,
            data: &(),
            phantom: PhantomData,
        };
        core::mem::forget(tmp_guard.downgrade());
    }

    unsafe fn downgrade_to_upgradable(&self) {
        let tmp_guard = RwLockWriteGuard {
            inner: self,
            data: &mut (),
            phantom: PhantomData,
        };
        core::mem::forget(tmp_guard.downgrade_to_upgradeable());
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    type RwLock<T> = super::RwLock<T>;

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let l = RwLock::new(());
        drop(l.read());
        drop(l.write());
        drop((l.read(), l.read()));
        drop(l.write());
    }

    // TODO: needs RNG
    //#[test]
    //fn frob() {
    //    static R: RwLock = RwLock::new();
    //    const N: usize = 10;
    //    const M: usize = 1000;
    //
    //    let (tx, rx) = channel::<()>();
    //    for _ in 0..N {
    //        let tx = tx.clone();
    //        thread::spawn(move|| {
    //            let mut rng = rand::thread_rng();
    //            for _ in 0..M {
    //                if rng.gen_weighted_bool(N) {
    //                    drop(R.write());
    //                } else {
    //                    drop(R.read());
    //                }
    //            }
    //            drop(tx);
    //        });
    //    }
    //    drop(tx);
    //    let _ = rx.recv();
    //    unsafe { R.destroy(); }
    //}

    #[test]
    fn test_rw_arc() {
        let arc = Arc::new(RwLock::new(0));
        let arc2 = arc.clone();
        let (tx, rx) = channel();

        let t = thread::spawn(move || {
            let mut lock = arc2.write();
            for _ in 0..10 {
                let tmp = *lock;
                *lock = -1;
                thread::yield_now();
                *lock = tmp + 1;
            }
            tx.send(()).unwrap();
        });

        // Readers try to catch the writer in the act
        let mut children = Vec::new();
        for _ in 0..5 {
            let arc3 = arc.clone();
            children.push(thread::spawn(move || {
                let lock = arc3.read();
                assert!(*lock >= 0);
            }));
        }

        // Wait for children to pass their asserts
        for r in children {
            assert!(r.join().is_ok());
        }

        // Wait for writer to finish
        rx.recv().unwrap();
        let lock = arc.read();
        assert_eq!(*lock, 10);

        assert!(t.join().is_ok());
    }

    #[test]
    fn test_rw_access_in_unwind() {
        let arc = Arc::new(RwLock::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move || -> () {
            struct Unwinder {
                i: Arc<RwLock<isize>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    let mut lock = self.i.write();
                    *lock += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        })
        .join();
        let lock = arc.read();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_rwlock_unsized() {
        let rw: &RwLock<[i32]> = &RwLock::new([1, 2, 3]);
        {
            let b = &mut *rw.write();
            b[0] = 4;
            b[2] = 5;
        }
        let comp: &[i32] = &[4, 2, 5];
        assert_eq!(&*rw.read(), comp);
    }

    #[test]
    fn test_rwlock_try_write() {
        use std::mem::drop;

        let lock = RwLock::new(0isize);
        let read_guard = lock.read();

        let write_result = lock.try_write();
        match write_result {
            None => (),
            Some(_) => assert!(
                false,
                "try_write should not succeed while read_guard is in scope"
            ),
        }

        drop(read_guard);
    }

    #[test]
    fn test_rw_try_read() {
        let m = RwLock::new(0);
        ::std::mem::forget(m.write());
        assert!(m.try_read().is_none());
    }

    #[test]
    fn test_into_inner() {
        let m = RwLock::new(NonCopy(10));
        assert_eq!(m.into_inner(), NonCopy(10));
    }

    #[test]
    fn test_into_inner_drop() {
        struct Foo(Arc<AtomicUsize>);
        impl Drop for Foo {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let num_drops = Arc::new(AtomicUsize::new(0));
        let m = RwLock::new(Foo(num_drops.clone()));
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        {
            let _inner = m.into_inner();
            assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        }
        assert_eq!(num_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_force_read_decrement() {
        let m = RwLock::new(());
        ::std::mem::forget(m.read());
        ::std::mem::forget(m.read());
        ::std::mem::forget(m.read());
        assert!(m.try_write().is_none());
        unsafe {
            m.force_read_decrement();
            m.force_read_decrement();
        }
        assert!(m.try_write().is_none());
        unsafe {
            m.force_read_decrement();
        }
        assert!(m.try_write().is_some());
    }

    #[test]
    fn test_force_write_unlock() {
        let m = RwLock::new(());
        ::std::mem::forget(m.write());
        assert!(m.try_read().is_none());
        unsafe {
            m.force_write_unlock();
        }
        assert!(m.try_read().is_some());
    }

    #[test]
    fn test_upgrade_downgrade() {
        let m = RwLock::new(());
        {
            let _r = m.read();
            let upg = m.try_upgradeable_read().unwrap();
            assert!(m.try_read().is_none());
            assert!(m.try_write().is_none());
            assert!(upg.try_upgrade().is_err());
        }
        {
            let w = m.write();
            assert!(m.try_upgradeable_read().is_none());
            let _r = w.downgrade();
            assert!(m.try_upgradeable_read().is_some());
            assert!(m.try_read().is_some());
            assert!(m.try_write().is_none());
        }
        {
            let _u = m.upgradeable_read();
            assert!(m.try_upgradeable_read().is_none());
        }

        assert!(m.try_upgradeable_read().unwrap().try_upgrade().is_ok());
    }
}
