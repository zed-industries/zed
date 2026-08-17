//! Synchronization primitives for one-time evaluation.

use crate::{
    atomic::{AtomicU8, Ordering},
    RelaxStrategy, Spin,
};
use core::{cell::UnsafeCell, fmt, marker::PhantomData, mem::MaybeUninit};

/// A primitive that provides lazy one-time initialization.
///
/// Unlike its `std::sync` equivalent, this is generalized such that the closure returns a
/// value to be stored by the [`Once`] (`std::sync::Once` can be trivially emulated with
/// `Once`).
///
/// Because [`Once::new`] is `const`, this primitive may be used to safely initialize statics.
///
/// # Examples
///
/// ```
/// use spin;
///
/// static START: spin::Once = spin::Once::new();
///
/// START.call_once(|| {
///     // run initialization here
/// });
/// ```
pub struct Once<T = (), R = Spin> {
    phantom: PhantomData<R>,
    status: AtomicStatus,
    data: UnsafeCell<MaybeUninit<T>>,
}

impl<T, R> Default for Once<T, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Debug, R> fmt::Debug for Once<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.get() {
            Some(s) => write!(f, "Once {{ data: ")
                .and_then(|()| s.fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "Once {{ <uninitialized> }}"),
        }
    }
}

// Same unsafe impls as `std::sync::RwLock`, because this also allows for
// concurrent reads.
unsafe impl<T: Send + Sync, R> Sync for Once<T, R> {}
unsafe impl<T: Send, R> Send for Once<T, R> {}

mod status {
    use super::*;

    // SAFETY: This structure has an invariant, namely that the inner atomic u8 must *always* have
    // a value for which there exists a valid Status. This means that users of this API must only
    // be allowed to load and store `Status`es.
    #[repr(transparent)]
    pub struct AtomicStatus(AtomicU8);

    // Four states that a Once can be in, encoded into the lower bits of `status` in
    // the Once structure.
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Status {
        Incomplete = 0x00,
        Running = 0x01,
        Complete = 0x02,
        Panicked = 0x03,
    }
    impl Status {
        // Construct a status from an inner u8 integer.
        //
        // # Safety
        //
        // For this to be safe, the inner number must have a valid corresponding enum variant.
        unsafe fn new_unchecked(inner: u8) -> Self {
            core::mem::transmute(inner)
        }
    }

    impl AtomicStatus {
        #[inline(always)]
        pub const fn new(status: Status) -> Self {
            // SAFETY: We got the value directly from status, so transmuting back is fine.
            Self(AtomicU8::new(status as u8))
        }
        #[inline(always)]
        pub fn load(&self, ordering: Ordering) -> Status {
            // SAFETY: We know that the inner integer must have been constructed from a Status in
            // the first place.
            unsafe { Status::new_unchecked(self.0.load(ordering)) }
        }
        #[inline(always)]
        pub fn store(&self, status: Status, ordering: Ordering) {
            // SAFETY: While not directly unsafe, this is safe because the value was retrieved from
            // a status, thus making transmutation safe.
            self.0.store(status as u8, ordering);
        }
        #[inline(always)]
        pub fn compare_exchange(
            &self,
            old: Status,
            new: Status,
            success: Ordering,
            failure: Ordering,
        ) -> Result<Status, Status> {
            match self
                .0
                .compare_exchange(old as u8, new as u8, success, failure)
            {
                // SAFETY: A compare exchange will always return a value that was later stored into
                // the atomic u8, but due to the invariant that it must be a valid Status, we know
                // that both Ok(_) and Err(_) will be safely transmutable.
                Ok(ok) => Ok(unsafe { Status::new_unchecked(ok) }),
                Err(err) => Err(unsafe { Status::new_unchecked(err) }),
            }
        }
        #[inline(always)]
        pub fn get_mut(&mut self) -> &mut Status {
            // SAFETY: Since we know that the u8 inside must be a valid Status, we can safely cast
            // it to a &mut Status.
            unsafe { &mut *((self.0.get_mut() as *mut u8).cast::<Status>()) }
        }
    }
}
use self::status::{AtomicStatus, Status};

impl<T, R: RelaxStrategy> Once<T, R> {
    /// Performs an initialization routine once and only once. The given closure
    /// will be executed if this is the first time `call_once` has been called,
    /// and otherwise the routine will *not* be invoked.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    ///
    /// When this function returns, it is guaranteed that some initialization
    /// has run and completed (it may not be the closure specified). The
    /// returned pointer will point to the result from the closure that was
    /// run.
    ///
    /// # Panics
    ///
    /// This function will panic if the [`Once`] previously panicked while attempting
    /// to initialize. This is similar to the poisoning behaviour of `std::sync`'s
    /// primitives.
    ///
    /// # Examples
    ///
    /// ```
    /// use spin;
    ///
    /// static INIT: spin::Once<usize> = spin::Once::new();
    ///
    /// fn get_cached_val() -> usize {
    ///     *INIT.call_once(expensive_computation)
    /// }
    ///
    /// fn expensive_computation() -> usize {
    ///     // ...
    /// # 2
    /// }
    /// ```
    pub fn call_once<F: FnOnce() -> T>(&self, f: F) -> &T {
        match self.try_call_once(|| Ok::<T, core::convert::Infallible>(f())) {
            Ok(x) => x,
            Err(void) => match void {},
        }
    }

    /// This method is similar to `call_once`, but allows the given closure to
    /// fail, and lets the `Once` in a uninitialized state if it does.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    ///
    /// When this function returns without error, it is guaranteed that some
    /// initialization has run and completed (it may not be the closure
    /// specified). The returned reference will point to the result from the
    /// closure that was run.
    ///
    /// # Panics
    ///
    /// This function will panic if the [`Once`] previously panicked while attempting
    /// to initialize. This is similar to the poisoning behaviour of `std::sync`'s
    /// primitives.
    ///
    /// # Examples
    ///
    /// ```
    /// use spin;
    ///
    /// static INIT: spin::Once<usize> = spin::Once::new();
    ///
    /// fn get_cached_val() -> Result<usize, String> {
    ///     INIT.try_call_once(expensive_fallible_computation).map(|x| *x)
    /// }
    ///
    /// fn expensive_fallible_computation() -> Result<usize, String> {
    ///     // ...
    /// # Ok(2)
    /// }
    /// ```
    pub fn try_call_once<F: FnOnce() -> Result<T, E>, E>(&self, f: F) -> Result<&T, E> {
        if let Some(value) = self.get() {
            Ok(value)
        } else {
            self.try_call_once_slow(f)
        }
    }

    #[cold]
    fn try_call_once_slow<F: FnOnce() -> Result<T, E>, E>(&self, f: F) -> Result<&T, E> {
        loop {
            let xchg = self.status.compare_exchange(
                Status::Incomplete,
                Status::Running,
                Ordering::Acquire,
                Ordering::Acquire,
            );

            match xchg {
                Ok(_must_be_state_incomplete) => {
                    // Impl is defined after the match for readability
                }
                Err(Status::Panicked) => panic!("Once panicked"),
                Err(Status::Running) => match self.poll() {
                    Some(v) => return Ok(v),
                    None => continue,
                },
                Err(Status::Complete) => {
                    return Ok(unsafe {
                        // SAFETY: The status is Complete
                        self.force_get()
                    });
                }
                Err(Status::Incomplete) => {
                    // The compare_exchange failed, so this shouldn't ever be reached,
                    // however if we decide to switch to compare_exchange_weak it will
                    // be safer to leave this here than hit an unreachable
                    continue;
                }
            }

            // The compare-exchange succeeded, so we shall initialize it.

            // We use a guard (Finish) to catch panics caused by builder
            let finish = Finish {
                status: &self.status,
            };
            let val = match f() {
                Ok(val) => val,
                Err(err) => {
                    // If an error occurs, clean up everything and leave.
                    core::mem::forget(finish);
                    self.status.store(Status::Incomplete, Ordering::Release);
                    return Err(err);
                }
            };
            unsafe {
                // SAFETY:
                // `UnsafeCell`/deref: currently the only accessor, mutably
                // and immutably by cas exclusion.
                // `write`: pointer comes from `MaybeUninit`.
                (*self.data.get()).as_mut_ptr().write(val);
            };
            // If there were to be a panic with unwind enabled, the code would
            // short-circuit and never reach the point where it writes the inner data.
            // The destructor for Finish will run, and poison the Once to ensure that other
            // threads accessing it do not exhibit unwanted behavior, if there were to be
            // any inconsistency in data structures caused by the panicking thread.
            //
            // However, f() is expected in the general case not to panic. In that case, we
            // simply forget the guard, bypassing its destructor. We could theoretically
            // clear a flag instead, but this eliminates the call to the destructor at
            // compile time, and unconditionally poisons during an eventual panic, if
            // unwinding is enabled.
            core::mem::forget(finish);

            // SAFETY: Release is required here, so that all memory accesses done in the
            // closure when initializing, become visible to other threads that perform Acquire
            // loads.
            //
            // And, we also know that the changes this thread has done will not magically
            // disappear from our cache, so it does not need to be AcqRel.
            self.status.store(Status::Complete, Ordering::Release);

            // This next line is mainly an optimization.
            return unsafe { Ok(self.force_get()) };
        }
    }

    /// Spins until the [`Once`] contains a value.
    ///
    /// Note that in releases prior to `0.7`, this function had the behaviour of [`Once::poll`].
    ///
    /// # Panics
    ///
    /// This function will panic if the [`Once`] previously panicked while attempting
    /// to initialize. This is similar to the poisoning behaviour of `std::sync`'s
    /// primitives.
    pub fn wait(&self) -> &T {
        loop {
            match self.poll() {
                Some(x) => break x,
                None => R::relax(),
            }
        }
    }

    /// Like [`Once::get`], but will spin if the [`Once`] is in the process of being
    /// initialized. If initialization has not even begun, `None` will be returned.
    ///
    /// Note that in releases prior to `0.7`, this function was named `wait`.
    ///
    /// # Panics
    ///
    /// This function will panic if the [`Once`] previously panicked while attempting
    /// to initialize. This is similar to the poisoning behaviour of `std::sync`'s
    /// primitives.
    pub fn poll(&self) -> Option<&T> {
        loop {
            // SAFETY: Acquire is safe here, because if the status is COMPLETE, then we want to make
            // sure that all memory accessed done while initializing that value, are visible when
            // we return a reference to the inner data after this load.
            match self.status.load(Ordering::Acquire) {
                Status::Incomplete => return None,
                Status::Running => R::relax(), // We spin
                Status::Complete => return Some(unsafe { self.force_get() }),
                Status::Panicked => panic!("Once previously poisoned by a panicked"),
            }
        }
    }
}

impl<T, R> Once<T, R> {
    /// Initialization constant of [`Once`].
    #[allow(clippy::declare_interior_mutable_const)]
    pub const INIT: Self = Self {
        phantom: PhantomData,
        status: AtomicStatus::new(Status::Incomplete),
        data: UnsafeCell::new(MaybeUninit::uninit()),
    };

    /// Creates a new [`Once`].
    pub const fn new() -> Self {
        Self::INIT
    }

    /// Creates a new initialized [`Once`].
    pub const fn initialized(data: T) -> Self {
        Self {
            phantom: PhantomData,
            status: AtomicStatus::new(Status::Complete),
            data: UnsafeCell::new(MaybeUninit::new(data)),
        }
    }

    /// Retrieve a pointer to the inner data.
    ///
    /// While this method itself is safe, accessing the pointer before the [`Once`] has been
    /// initialized is UB, unless this method has already been written to from a pointer coming
    /// from this method.
    pub fn as_mut_ptr(&self) -> *mut T {
        // SAFETY:
        // * MaybeUninit<T> always has exactly the same layout as T
        self.data.get().cast::<T>()
    }

    /// Get a reference to the initialized instance. Must only be called once COMPLETE.
    unsafe fn force_get(&self) -> &T {
        // SAFETY:
        // * `UnsafeCell`/inner deref: data never changes again
        // * `MaybeUninit`/outer deref: data was initialized
        &*(*self.data.get()).as_ptr()
    }

    /// Get a reference to the initialized instance. Must only be called once COMPLETE.
    unsafe fn force_get_mut(&mut self) -> &mut T {
        // SAFETY:
        // * `UnsafeCell`/inner deref: data never changes again
        // * `MaybeUninit`/outer deref: data was initialized
        &mut *(*self.data.get()).as_mut_ptr()
    }

    /// Get a reference to the initialized instance. Must only be called once COMPLETE.
    unsafe fn force_into_inner(self) -> T {
        // SAFETY:
        // * `UnsafeCell`/inner deref: data never changes again
        // * `MaybeUninit`/outer deref: data was initialized
        (*self.data.get()).as_ptr().read()
    }

    /// Returns a reference to the inner value if the [`Once`] has been initialized.
    pub fn get(&self) -> Option<&T> {
        // SAFETY: Just as with `poll`, Acquire is safe here because we want to be able to see the
        // nonatomic stores done when initializing, once we have loaded and checked the status.
        match self.status.load(Ordering::Acquire) {
            Status::Complete => Some(unsafe { self.force_get() }),
            _ => None,
        }
    }

    /// Returns a reference to the inner value on the unchecked assumption that the  [`Once`] has been initialized.
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if the `Once` has not already been initialized because a reference to uninitialized
    /// memory will be returned, immediately triggering undefined behaviour (even if the reference goes unused).
    /// However, this can be useful in some instances for exposing the `Once` to FFI or when the overhead of atomically
    /// checking initialization is unacceptable and the `Once` has already been initialized.
    pub unsafe fn get_unchecked(&self) -> &T {
        debug_assert_eq!(
            self.status.load(Ordering::SeqCst),
            Status::Complete,
            "Attempted to access an uninitialized Once. If this was run without debug checks, this would be undefined behaviour. This is a serious bug and you must fix it.",
        );
        self.force_get()
    }

    /// Returns a mutable reference to the inner value if the [`Once`] has been initialized.
    ///
    /// Because this method requires a mutable reference to the [`Once`], no synchronization
    /// overhead is required to access the inner value. In effect, it is zero-cost.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match *self.status.get_mut() {
            Status::Complete => Some(unsafe { self.force_get_mut() }),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner value
    ///
    /// # Safety
    ///
    /// This is *extremely* unsafe if the `Once` has not already been initialized because a reference to uninitialized
    /// memory will be returned, immediately triggering undefined behaviour (even if the reference goes unused).
    /// However, this can be useful in some instances for exposing the `Once` to FFI or when the overhead of atomically
    /// checking initialization is unacceptable and the `Once` has already been initialized.
    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        debug_assert_eq!(
            self.status.load(Ordering::SeqCst),
            Status::Complete,
            "Attempted to access an unintialized Once.  If this was to run without debug checks, this would be undefined behavior.  This is a serious bug and you must fix it.",
        );
        self.force_get_mut()
    }

    /// Returns a the inner value if the [`Once`] has been initialized.
    ///
    /// Because this method requires ownership of the [`Once`], no synchronization overhead
    /// is required to access the inner value. In effect, it is zero-cost.
    pub fn try_into_inner(mut self) -> Option<T> {
        match *self.status.get_mut() {
            Status::Complete => Some(unsafe { self.force_into_inner() }),
            _ => None,
        }
    }

    /// Returns a the inner value if the [`Once`] has been initialized.  
    /// # Safety
    ///
    /// This is *extremely* unsafe if the `Once` has not already been initialized because a reference to uninitialized
    /// memory will be returned, immediately triggering undefined behaviour (even if the reference goes unused)
    /// This can be useful, if `Once` has already been initialized, and you want to bypass an
    /// option check.
    pub unsafe fn into_inner_unchecked(self) -> T {
        debug_assert_eq!(
            self.status.load(Ordering::SeqCst),
            Status::Complete,
            "Attempted to access an unintialized Once.  If this was to run without debug checks, this would be undefined behavior.  This is a serious bug and you must fix it.",
        );
        self.force_into_inner()
    }

    /// Checks whether the value has been initialized.
    ///
    /// This is done using [`Acquire`](core::sync::atomic::Ordering::Acquire) ordering, and
    /// therefore it is safe to access the value directly via
    /// [`get_unchecked`](Self::get_unchecked) if this returns true.
    pub fn is_completed(&self) -> bool {
        // TODO: Add a similar variant for Relaxed?
        self.status.load(Ordering::Acquire) == Status::Complete
    }
}

impl<T, R> From<T> for Once<T, R> {
    fn from(data: T) -> Self {
        Self::initialized(data)
    }
}

impl<T, R> Drop for Once<T, R> {
    fn drop(&mut self) {
        // No need to do any atomic access here, we have &mut!
        if *self.status.get_mut() == Status::Complete {
            unsafe {
                //TODO: Use MaybeUninit::assume_init_drop once stabilised
                core::ptr::drop_in_place((*self.data.get()).as_mut_ptr());
            }
        }
    }
}

struct Finish<'a> {
    status: &'a AtomicStatus,
}

impl<'a> Drop for Finish<'a> {
    fn drop(&mut self) {
        // While using Relaxed here would most likely not be an issue, we use SeqCst anyway.
        // This is mainly because panics are not meant to be fast at all, but also because if
        // there were to be a compiler bug which reorders accesses within the same thread,
        // where it should not, we want to be sure that the panic really is handled, and does
        // not cause additional problems. SeqCst will therefore help guarding against such
        // bugs.
        self.status.store(Status::Panicked, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::atomic::AtomicU32;
    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[test]
    fn smoke_once() {
        static O: Once = Once::new();
        let mut a = 0;
        O.call_once(|| a += 1);
        assert_eq!(a, 1);
        O.call_once(|| a += 1);
        assert_eq!(a, 1);
    }

    #[test]
    fn smoke_once_value() {
        static O: Once<usize> = Once::new();
        let a = O.call_once(|| 1);
        assert_eq!(*a, 1);
        let b = O.call_once(|| 2);
        assert_eq!(*b, 1);
    }

    #[test]
    fn stampede_once() {
        static O: Once = Once::new();
        static mut RUN: bool = false;

        let (tx, rx) = channel();
        let mut ts = Vec::new();
        for _ in 0..10 {
            let tx = tx.clone();
            ts.push(thread::spawn(move || {
                for _ in 0..4 {
                    thread::yield_now()
                }
                unsafe {
                    O.call_once(|| {
                        assert!(!RUN);
                        RUN = true;
                    });
                    assert!(RUN);
                }
                tx.send(()).unwrap();
            }));
        }

        unsafe {
            O.call_once(|| {
                assert!(!RUN);
                RUN = true;
            });
            assert!(RUN);
        }

        for _ in 0..10 {
            rx.recv().unwrap();
        }

        for t in ts {
            t.join().unwrap();
        }
    }

    #[test]
    fn get() {
        static INIT: Once<usize> = Once::new();

        assert!(INIT.get().is_none());
        INIT.call_once(|| 2);
        assert_eq!(INIT.get().map(|r| *r), Some(2));
    }

    #[test]
    fn get_no_wait() {
        static INIT: Once<usize> = Once::new();

        assert!(INIT.get().is_none());
        let t = thread::spawn(move || {
            INIT.call_once(|| {
                thread::sleep(std::time::Duration::from_secs(3));
                42
            });
        });
        assert!(INIT.get().is_none());

        t.join().unwrap();
    }

    #[test]
    fn poll() {
        static INIT: Once<usize> = Once::new();

        assert!(INIT.poll().is_none());
        INIT.call_once(|| 3);
        assert_eq!(INIT.poll().map(|r| *r), Some(3));
    }

    #[test]
    fn wait() {
        static INIT: Once<usize> = Once::new();

        let t = std::thread::spawn(|| {
            assert_eq!(*INIT.wait(), 3);
            assert!(INIT.is_completed());
        });

        for _ in 0..4 {
            thread::yield_now()
        }

        assert!(INIT.poll().is_none());
        INIT.call_once(|| 3);

        t.join().unwrap();
    }

    #[test]
    fn panic() {
        use std::panic;

        static INIT: Once = Once::new();

        // poison the once
        let t = panic::catch_unwind(|| {
            INIT.call_once(|| panic!());
        });
        assert!(t.is_err());

        // poisoning propagates
        let t = panic::catch_unwind(|| {
            INIT.call_once(|| {});
        });
        assert!(t.is_err());
    }

    #[test]
    fn init_constant() {
        static O: Once = Once::INIT;
        let mut a = 0;
        O.call_once(|| a += 1);
        assert_eq!(a, 1);
        O.call_once(|| a += 1);
        assert_eq!(a, 1);
    }

    static mut CALLED: bool = false;

    struct DropTest {}

    impl Drop for DropTest {
        fn drop(&mut self) {
            unsafe {
                CALLED = true;
            }
        }
    }

    #[test]
    fn try_call_once_err() {
        let once = Once::<_, Spin>::new();
        let shared = Arc::new((once, AtomicU32::new(0)));

        let (tx, rx) = channel();

        let t0 = {
            let shared = shared.clone();
            thread::spawn(move || {
                let (once, called) = &*shared;

                once.try_call_once(|| {
                    called.fetch_add(1, Ordering::AcqRel);
                    tx.send(()).unwrap();
                    thread::sleep(std::time::Duration::from_millis(50));
                    Err(())
                })
                .ok();
            })
        };

        let t1 = {
            let shared = shared.clone();
            thread::spawn(move || {
                rx.recv().unwrap();
                let (once, called) = &*shared;
                assert_eq!(
                    called.load(Ordering::Acquire),
                    1,
                    "leader thread did not run first"
                );

                once.call_once(|| {
                    called.fetch_add(1, Ordering::AcqRel);
                });
            })
        };

        t0.join().unwrap();
        t1.join().unwrap();

        assert_eq!(shared.1.load(Ordering::Acquire), 2);
    }

    // This is sort of two test cases, but if we write them as separate test methods
    // they can be executed concurrently and then fail some small fraction of the
    // time.
    #[test]
    fn drop_occurs_and_skip_uninit_drop() {
        unsafe {
            CALLED = false;
        }

        {
            let once = Once::<_>::new();
            once.call_once(|| DropTest {});
        }

        assert!(unsafe { CALLED });
        // Now test that we skip drops for the uninitialized case.
        unsafe {
            CALLED = false;
        }

        let once = Once::<DropTest>::new();
        drop(once);

        assert!(unsafe { !CALLED });
    }

    #[test]
    fn call_once_test() {
        for _ in 0..20 {
            use std::sync::atomic::AtomicUsize;
            use std::sync::Arc;
            use std::time::Duration;
            let share = Arc::new(AtomicUsize::new(0));
            let once = Arc::new(Once::<_, Spin>::new());
            let mut hs = Vec::new();
            for _ in 0..8 {
                let h = thread::spawn({
                    let share = share.clone();
                    let once = once.clone();
                    move || {
                        thread::sleep(Duration::from_millis(10));
                        once.call_once(|| {
                            share.fetch_add(1, Ordering::SeqCst);
                        });
                    }
                });
                hs.push(h);
            }
            for h in hs {
                h.join().unwrap();
            }
            assert_eq!(1, share.load(Ordering::SeqCst));
        }
    }
}
