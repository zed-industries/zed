//! Support for static variables that are initialized at startup time.
#![allow(unsafe_code)]

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::{AtomicU8, Ordering};

/// A static variable intended to be initialized before `main`.
///
/// ## Expected usage
///
/// This type is designed for the "startup is single-threaded" model: it is
/// expected that no other threads access the value until initialization has
/// completed. After that point, the value is treated as initialized and
/// immutable.
///
/// ## Concurrency
///
/// If the value is accessed concurrently while it is still being
/// initialized (for example, from another thread during startup, including
/// when used from a dynamic library), this is not undefined behavior, but
/// this implementation does not wait for initialization to complete: it
/// will panic instead.
///
/// If you need concurrent first access to be handled by blocking/spinning
/// rather than panicking, use [`std::sync::OnceLock`] or
/// [`std::sync::LazyLock`] (when `std` is available).
///
/// ## Panics / poisoning
///
/// If the initializer panics, the static becomes permanently poisoned: all
/// subsequent accesses will panic, and it cannot be reset.
pub struct Static<T: Sync> {
    storage: UnsafeCell<MaybeUninit<T>>,
    initializer: fn() -> T,
    initialized: AtomicU8,
}

const UNINITIALIZED: u8 = 0;
const INITIALIZING: u8 = 1;
const FINISHED_INITIALIZING: u8 = 2;
const INITIALIZED: u8 = 3;
const DROPPING: u8 = 4;
const POISONED: u8 = 0x10;
const DROPPED: u8 = 0xff;

/// SAFETY: This is safe because the static variable is either initialized
/// and read-only, poisoned and panicing, or initializing (and will panic if
/// multiple threads try to initialize it at the same time).
unsafe impl<T: Sync> Sync for Static<T> {}

impl<T: Sync> Static<T> {
    /// Create a new ctor-initialized static variable.
    ///
    /// # Safety
    ///
    /// See the documentation for `Static` for more information.
    #[doc(hidden)]
    pub const unsafe fn new(initializer: fn() -> T) -> Self {
        Self {
            storage: UnsafeCell::new(MaybeUninit::uninit()),
            initializer,
            initialized: AtomicU8::new(UNINITIALIZED),
        }
    }

    /// Get the underlying value of the static variable without checking the
    /// initialized state.
    ///
    /// # Safety
    ///
    /// This must only be called if the initialized state is `INITIALIZED`.
    #[inline(always)]
    unsafe fn get_unchecked(&self) -> &T {
        unsafe {
            (UnsafeCell::raw_get(&self.storage) as *const T)
                .as_ref()
                .unwrap_unchecked()
        }
    }
}

// Common trait delegates

impl<T: Sync + ::core::fmt::Debug> ::core::fmt::Debug for Static<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Sync + ::core::fmt::Display> ::core::fmt::Display for Static<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: Sync> Deref for Static<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        if self.initialized.load(Ordering::Acquire) == INITIALIZED {
            // SAFETY: We only access the static variable if the initialized
            // state is `INITIALIZED`
            unsafe { self.get_unchecked() }
        } else {
            self.deref_slow()
        }
    }
}

impl<T: Sync> Static<T> {
    #[cold]
    fn deref_slow(&self) -> &T {
        struct PanicGuard<'a> {
            initialized: &'a AtomicU8,
        }
        impl<'a> Drop for PanicGuard<'a> {
            fn drop(&mut self) {
                self.initialized.fetch_or(POISONED, Ordering::AcqRel);
            }
        }

        // SAFETY: We only access the static variable if the initialized
        // state is `INITIALIZED`, or if we are `UNINITIALIZED` and put the
        // state into `INITIALIZED`.
        unsafe {
            match self.initialized.fetch_or(INITIALIZING, Ordering::AcqRel) {
                UNINITIALIZED => {
                    let panic_guard = PanicGuard {
                        initialized: &self.initialized,
                    };
                    let value = (self.initializer)();
                    core::mem::forget(panic_guard);
                    ptr::write(self.storage.get() as _, value);
                    self.initialized
                        .fetch_or(FINISHED_INITIALIZING, Ordering::AcqRel);
                    self.get_unchecked()
                }
                INITIALIZING => {
                    panic!("Recursive or overlapping initialization of static variable");
                }
                INITIALIZED => self.get_unchecked(),
                x if x & POISONED != 0 => panic!("Static variable has been poisoned"),
                _ => panic!("Invalid state for static variable"),
            }
        }
    }
}

impl<T: Sync> Drop for Static<T> {
    fn drop(&mut self) {
        // SAFETY: We only drop if the static is in the `INITIALIZED` state,
        // which is can never leave unless going through this drop path. We
        // leak in all other cases (though nothing should be written in any
        // of those cases).
        unsafe {
            if INITIALIZED == self.initialized.fetch_or(DROPPING, Ordering::AcqRel) {
                (UnsafeCell::raw_get(&self.storage) as *mut T).drop_in_place();
                self.initialized.fetch_or(DROPPED, Ordering::AcqRel);
            }
        }
    }
}
