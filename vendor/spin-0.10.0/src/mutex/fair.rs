//! A spinning mutex with a fairer unlock algorithm.
//!
//! This mutex is similar to the `SpinMutex` in that it uses spinning to avoid
//! context switches. However, it uses a fairer unlock algorithm that avoids
//! starvation of threads that are waiting for the lock.

use crate::{
    atomic::{AtomicUsize, Ordering},
    RelaxStrategy, Spin,
};
use core::{
    cell::UnsafeCell,
    fmt,
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};

// The lowest bit of `lock` is used to indicate whether the mutex is locked or not. The rest of the bits are used to
// store the number of starving threads.
const LOCKED: usize = 1;
const STARVED: usize = 2;

/// Number chosen by fair roll of the dice, adjust as needed.
const STARVATION_SPINS: usize = 1024;

/// A [spin lock](https://en.m.wikipedia.org/wiki/Spinlock) providing mutually exclusive access to data, but with a fairer
/// algorithm.
///
/// # Example
///
/// ```
/// use spin;
///
/// let lock = spin::mutex::FairMutex::<_>::new(0);
///
/// // Modify the data
/// *lock.lock() = 2;
///
/// // Read the data
/// let answer = *lock.lock();
/// assert_eq!(answer, 2);
/// ```
///
/// # Thread safety example
///
/// ```
/// use spin;
/// use std::sync::{Arc, Barrier};
///
/// let thread_count = 1000;
/// let spin_mutex = Arc::new(spin::mutex::FairMutex::<_>::new(0));
///
/// // We use a barrier to ensure the readout happens after all writing
/// let barrier = Arc::new(Barrier::new(thread_count + 1));
///
/// for _ in (0..thread_count) {
///     let my_barrier = barrier.clone();
///     let my_lock = spin_mutex.clone();
///     std::thread::spawn(move || {
///         let mut guard = my_lock.lock();
///         *guard += 1;
///
///         // Release the lock to prevent a deadlock
///         drop(guard);
///         my_barrier.wait();
///     });
/// }
///
/// barrier.wait();
///
/// let answer = { *spin_mutex.lock() };
/// assert_eq!(answer, thread_count);
/// ```
pub struct FairMutex<T: ?Sized, R = Spin> {
    phantom: PhantomData<R>,
    pub(crate) lock: AtomicUsize,
    data: UnsafeCell<T>,
}

/// A guard that provides mutable data access.
///
/// When the guard falls out of scope it will release the lock.
pub struct FairMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicUsize,
    data: *mut T,
}

/// A handle that indicates that we have been trying to acquire the lock for a while.
///
/// This handle is used to prevent starvation.
pub struct Starvation<'a, T: ?Sized + 'a, R> {
    lock: &'a FairMutex<T, R>,
}

/// Indicates whether a lock was rejected due to the lock being held by another thread or due to starvation.
#[derive(Debug)]
pub enum LockRejectReason {
    /// The lock was rejected due to the lock being held by another thread.
    Locked,

    /// The lock was rejected due to starvation.
    Starved,
}

// Same unsafe impls as `std::sync::Mutex`
unsafe impl<T: ?Sized + Send, R> Sync for FairMutex<T, R> {}
unsafe impl<T: ?Sized + Send, R> Send for FairMutex<T, R> {}

unsafe impl<T: ?Sized + Sync> Sync for FairMutexGuard<'_, T> {}
unsafe impl<T: ?Sized + Send> Send for FairMutexGuard<'_, T> {}

impl<T, R> FairMutex<T, R> {
    /// Creates a new [`FairMutex`] wrapping the supplied data.
    ///
    /// # Example
    ///
    /// ```
    /// use spin::mutex::FairMutex;
    ///
    /// static MUTEX: FairMutex<()> = FairMutex::<_>::new(());
    ///
    /// fn demo() {
    ///     let lock = MUTEX.lock();
    ///     // do something with lock
    ///     drop(lock);
    /// }
    /// ```
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        FairMutex {
            lock: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
            phantom: PhantomData,
        }
    }

    /// Consumes this [`FairMutex`] and unwraps the underlying data.
    ///
    /// # Example
    ///
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(42);
    /// assert_eq!(42, lock.into_inner());
    /// ```
    #[inline(always)]
    pub fn into_inner(self) -> T {
        // We know statically that there are no outstanding references to
        // `self` so there's no need to lock.
        let FairMutex { data, .. } = self;
        data.into_inner()
    }

    /// Returns a mutable pointer to the underlying data.
    ///
    /// This is mostly meant to be used for applications which require manual unlocking, but where
    /// storing both the lock and the pointer to the inner data gets inefficient.
    ///
    /// # Example
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(42);
    ///
    /// unsafe {
    ///     core::mem::forget(lock.lock());
    ///
    ///     assert_eq!(lock.as_mut_ptr().read(), 42);
    ///     lock.as_mut_ptr().write(58);
    ///
    ///     lock.force_unlock();
    /// }
    ///
    /// assert_eq!(*lock.lock(), 58);
    ///
    /// ```
    #[inline(always)]
    pub fn as_mut_ptr(&self) -> *mut T {
        self.data.get()
    }
}

impl<T: ?Sized, R: RelaxStrategy> FairMutex<T, R> {
    /// Locks the [`FairMutex`] and returns a guard that permits access to the inner data.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    ///
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(0);
    /// {
    ///     let mut data = lock.lock();
    ///     // The lock is now locked and the data can be accessed
    ///     *data += 1;
    ///     // The lock is implicitly dropped at the end of the scope
    /// }
    /// ```
    #[inline(always)]
    pub fn lock(&self) -> FairMutexGuard<T> {
        // Can fail to lock even if the spinlock is not locked. May be more efficient than `try_lock`
        // when called in a loop.
        let mut spins = 0;
        while self
            .lock
            .compare_exchange_weak(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Wait until the lock looks unlocked before retrying
            while self.is_locked() {
                R::relax();

                // If we've been spinning for a while, switch to a fairer strategy that will prevent
                // newer users from stealing our lock from us.
                if spins > STARVATION_SPINS {
                    return self.starve().lock();
                }
                spins += 1;
            }
        }

        FairMutexGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<T: ?Sized, R> FairMutex<T, R> {
    /// Returns `true` if the lock is currently held.
    ///
    /// # Safety
    ///
    /// This function provides no synchronization guarantees and so its result should be considered 'out of date'
    /// the instant it is called. Do not use it for synchronization purposes. However, it may be useful as a heuristic.
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed) & LOCKED != 0
    }

    /// Force unlock this [`FairMutex`].
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if the lock is not held by the current
    /// thread. However, this can be useful in some instances for exposing the
    /// lock to FFI that doesn't know how to deal with RAII.
    #[inline(always)]
    pub unsafe fn force_unlock(&self) {
        self.lock.fetch_and(!LOCKED, Ordering::Release);
    }

    /// Try to lock this [`FairMutex`], returning a lock guard if successful.
    ///
    /// # Example
    ///
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(42);
    ///
    /// let maybe_guard = lock.try_lock();
    /// assert!(maybe_guard.is_some());
    ///
    /// // `maybe_guard` is still held, so the second call fails
    /// let maybe_guard2 = lock.try_lock();
    /// assert!(maybe_guard2.is_none());
    /// ```
    #[inline(always)]
    pub fn try_lock(&self) -> Option<FairMutexGuard<T>> {
        self.try_lock_starver().ok()
    }

    /// Tries to lock this [`FairMutex`] and returns a result that indicates whether the lock was
    /// rejected due to a starver or not.
    #[inline(always)]
    pub fn try_lock_starver(&self) -> Result<FairMutexGuard<T>, LockRejectReason> {
        match self
            .lock
            .compare_exchange(0, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .unwrap_or_else(|x| x)
        {
            0 => Ok(FairMutexGuard {
                lock: &self.lock,
                data: unsafe { &mut *self.data.get() },
            }),
            LOCKED => Err(LockRejectReason::Locked),
            _ => Err(LockRejectReason::Starved),
        }
    }

    /// Indicates that the current user has been waiting for the lock for a while
    /// and that the lock should yield to this thread over a newly arriving thread.
    ///
    /// # Example
    ///
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(42);
    ///
    /// // Lock the mutex to simulate it being used by another user.
    /// let guard1 = lock.lock();
    ///
    /// // Try to lock the mutex.
    /// let guard2 = lock.try_lock();
    /// assert!(guard2.is_none());
    ///
    /// // Wait for a while.
    /// wait_for_a_while();
    ///
    /// // We are now starved, indicate as such.
    /// let starve = lock.starve();
    ///
    /// // Once the lock is released, another user trying to lock it will
    /// // fail.
    /// drop(guard1);
    /// let guard3 = lock.try_lock();
    /// assert!(guard3.is_none());
    ///
    /// // However, we will be able to lock it.
    /// let guard4 = starve.try_lock();
    /// assert!(guard4.is_ok());
    ///
    /// # fn wait_for_a_while() {}
    /// ```
    pub fn starve(&self) -> Starvation<'_, T, R> {
        // Add a new starver to the state.
        if self.lock.fetch_add(STARVED, Ordering::Relaxed) > (core::isize::MAX - 1) as usize {
            // In the event of a potential lock overflow, abort.
            crate::abort();
        }

        Starvation { lock: self }
    }

    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this call borrows the [`FairMutex`] mutably, and a mutable reference is guaranteed to be exclusive in
    /// Rust, no actual locking needs to take place -- the mutable borrow statically guarantees no locks exist. As
    /// such, this is a 'zero-cost' operation.
    ///
    /// # Example
    ///
    /// ```
    /// let mut lock = spin::mutex::FairMutex::<_>::new(0);
    /// *lock.get_mut() = 10;
    /// assert_eq!(*lock.lock(), 10);
    /// ```
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        // We know statically that there are no other references to `self`, so
        // there's no need to lock the inner mutex.
        unsafe { &mut *self.data.get() }
    }
}

impl<T: ?Sized + fmt::Debug, R> fmt::Debug for FairMutex<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct LockWrapper<'a, T: ?Sized + fmt::Debug>(Option<FairMutexGuard<'a, T>>);

        impl<T: ?Sized + fmt::Debug> fmt::Debug for LockWrapper<'_, T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self.0 {
                    Some(guard) => fmt::Debug::fmt(guard, f),
                    None => f.write_str("<locked>"),
                }
            }
        }

        f.debug_struct("FairMutex")
            .field("data", &LockWrapper(self.try_lock()))
            .finish()
    }
}

impl<T: ?Sized + Default, R> Default for FairMutex<T, R> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, R> From<T> for FairMutex<T, R> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<'a, T: ?Sized> FairMutexGuard<'a, T> {
    /// Leak the lock guard, yielding a mutable reference to the underlying data.
    ///
    /// Note that this function will permanently lock the original [`FairMutex`].
    ///
    /// ```
    /// let mylock = spin::mutex::FairMutex::<_>::new(0);
    ///
    /// let data: &mut i32 = spin::mutex::FairMutexGuard::leak(mylock.lock());
    ///
    /// *data = 1;
    /// assert_eq!(*data, 1);
    /// ```
    #[inline(always)]
    pub fn leak(this: Self) -> &'a mut T {
        // Use ManuallyDrop to avoid stacked-borrow invalidation
        let mut this = ManuallyDrop::new(this);
        // We know statically that only we are referencing data
        unsafe { &mut *this.data }
    }
}

impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for FairMutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized + fmt::Display> fmt::Display for FairMutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized> Deref for FairMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We know statically that only we are referencing data
        unsafe { &*self.data }
    }
}

impl<'a, T: ?Sized> DerefMut for FairMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        // We know statically that only we are referencing data
        unsafe { &mut *self.data }
    }
}

impl<'a, T: ?Sized> Drop for FairMutexGuard<'a, T> {
    /// The dropping of the MutexGuard will release the lock it was created from.
    fn drop(&mut self) {
        self.lock.fetch_and(!LOCKED, Ordering::Release);
    }
}

impl<'a, T: ?Sized, R> Starvation<'a, T, R> {
    /// Attempts the lock the mutex if we are the only starving user.
    ///
    /// This allows another user to lock the mutex if they are starving as well.
    pub fn try_lock_fair(self) -> Result<FairMutexGuard<'a, T>, Self> {
        // Try to lock the mutex.
        if self
            .lock
            .lock
            .compare_exchange(
                STARVED,
                STARVED | LOCKED,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_ok()
        {
            // We are the only starving user, lock the mutex.
            Ok(FairMutexGuard {
                lock: &self.lock.lock,
                data: self.lock.data.get(),
            })
        } else {
            // Another user is starving, fail.
            Err(self)
        }
    }

    /// Attempts to lock the mutex.
    ///
    /// If the lock is currently held by another thread, this will return `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let lock = spin::mutex::FairMutex::<_>::new(42);
    ///
    /// // Lock the mutex to simulate it being used by another user.
    /// let guard1 = lock.lock();
    ///
    /// // Try to lock the mutex.
    /// let guard2 = lock.try_lock();
    /// assert!(guard2.is_none());
    ///
    /// // Wait for a while.
    /// wait_for_a_while();
    ///
    /// // We are now starved, indicate as such.
    /// let starve = lock.starve();
    ///
    /// // Once the lock is released, another user trying to lock it will
    /// // fail.
    /// drop(guard1);
    /// let guard3 = lock.try_lock();
    /// assert!(guard3.is_none());
    ///
    /// // However, we will be able to lock it.
    /// let guard4 = starve.try_lock();
    /// assert!(guard4.is_ok());
    ///
    /// # fn wait_for_a_while() {}
    /// ```
    pub fn try_lock(self) -> Result<FairMutexGuard<'a, T>, Self> {
        // Try to lock the mutex.
        if self.lock.lock.fetch_or(LOCKED, Ordering::Acquire) & LOCKED == 0 {
            // We have successfully locked the mutex.
            // By dropping `self` here, we decrement the starvation count.
            Ok(FairMutexGuard {
                lock: &self.lock.lock,
                data: self.lock.data.get(),
            })
        } else {
            Err(self)
        }
    }
}

impl<'a, T: ?Sized, R: RelaxStrategy> Starvation<'a, T, R> {
    /// Locks the mutex.
    pub fn lock(mut self) -> FairMutexGuard<'a, T> {
        // Try to lock the mutex.
        loop {
            match self.try_lock() {
                Ok(lock) => return lock,
                Err(starve) => self = starve,
            }

            // Relax until the lock is released.
            while self.lock.is_locked() {
                R::relax();
            }
        }
    }
}

impl<'a, T: ?Sized, R> Drop for Starvation<'a, T, R> {
    fn drop(&mut self) {
        // As there is no longer a user being starved, we decrement the starver count.
        self.lock.lock.fetch_sub(STARVED, Ordering::Release);
    }
}

impl fmt::Display for LockRejectReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LockRejectReason::Locked => write!(f, "locked"),
            LockRejectReason::Starved => write!(f, "starved"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LockRejectReason {}

#[cfg(feature = "lock_api")]
unsafe impl<R: RelaxStrategy> lock_api_crate::RawMutex for FairMutex<(), R> {
    type GuardMarker = lock_api_crate::GuardSend;

    const INIT: Self = Self::new(());

    fn lock(&self) {
        // Prevent guard destructor running
        core::mem::forget(Self::lock(self));
    }

    fn try_lock(&self) -> bool {
        // Prevent guard destructor running
        Self::try_lock(self).map(core::mem::forget).is_some()
    }

    unsafe fn unlock(&self) {
        self.force_unlock();
    }

    fn is_locked(&self) -> bool {
        Self::is_locked(self)
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    type FairMutex<T> = super::FairMutex<T>;

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let m = FairMutex::<_>::new(());
        drop(m.lock());
        drop(m.lock());
    }

    #[test]
    fn lots_and_lots() {
        static M: FairMutex<()> = FairMutex::<_>::new(());
        static mut CNT: u32 = 0;
        const J: u32 = 1000;
        const K: u32 = 3;

        fn inc() {
            for _ in 0..J {
                unsafe {
                    let _g = M.lock();
                    CNT += 1;
                }
            }
        }

        let (tx, rx) = channel();
        for _ in 0..K {
            let tx2 = tx.clone();
            thread::spawn(move || {
                inc();
                tx2.send(()).unwrap();
            });
            let tx2 = tx.clone();
            thread::spawn(move || {
                inc();
                tx2.send(()).unwrap();
            });
        }

        drop(tx);
        for _ in 0..2 * K {
            rx.recv().unwrap();
        }
        assert_eq!(unsafe { CNT }, J * K * 2);
    }

    #[test]
    fn try_lock() {
        let mutex = FairMutex::<_>::new(42);

        // First lock succeeds
        let a = mutex.try_lock();
        assert_eq!(a.as_ref().map(|r| **r), Some(42));

        // Additional lock fails
        let b = mutex.try_lock();
        assert!(b.is_none());

        // After dropping lock, it succeeds again
        ::core::mem::drop(a);
        let c = mutex.try_lock();
        assert_eq!(c.as_ref().map(|r| **r), Some(42));
    }

    #[test]
    fn test_into_inner() {
        let m = FairMutex::<_>::new(NonCopy(10));
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
        let m = FairMutex::<_>::new(Foo(num_drops.clone()));
        assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        {
            let _inner = m.into_inner();
            assert_eq!(num_drops.load(Ordering::SeqCst), 0);
        }
        assert_eq!(num_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_mutex_arc_nested() {
        // Tests nested mutexes and access
        // to underlying data.
        let arc = Arc::new(FairMutex::<_>::new(1));
        let arc2 = Arc::new(FairMutex::<_>::new(arc));
        let (tx, rx) = channel();
        let _t = thread::spawn(move || {
            let lock = arc2.lock();
            let lock2 = lock.lock();
            assert_eq!(*lock2, 1);
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    fn test_mutex_arc_access_in_unwind() {
        let arc = Arc::new(FairMutex::<_>::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move || -> () {
            struct Unwinder {
                i: Arc<FairMutex<i32>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    *self.i.lock() += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        })
        .join();
        let lock = arc.lock();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_mutex_unsized() {
        let mutex: &FairMutex<[i32]> = &FairMutex::<_>::new([1, 2, 3]);
        {
            let b = &mut *mutex.lock();
            b[0] = 4;
            b[2] = 5;
        }
        let comp: &[i32] = &[4, 2, 5];
        assert_eq!(&*mutex.lock(), comp);
    }

    #[test]
    fn test_mutex_force_lock() {
        let lock = FairMutex::<_>::new(());
        ::std::mem::forget(lock.lock());
        unsafe {
            lock.force_unlock();
        }
        assert!(lock.try_lock().is_some());
    }
}
