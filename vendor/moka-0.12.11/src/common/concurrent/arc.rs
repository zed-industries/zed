// This module's source code was written by us, the `moka` developers, referring to
// the following book and code:
//
// - Chapter 6. Building Our Own "Arc" of the Rust Atomics and Locks book.
//     - Rust Atomics and Locks by Mara Bos (O’Reilly). Copyright 2023 Mara Bos,
//       ISBN: 978-1-098-11944-7
//     - https://marabos.nl/atomics/
// - The `triomphe` crate v0.1.13 and v0.1.11 by Manish Goregaokar (Manishearth)
//     - MIT or Apache-2.0 License
//     - https://github.com/Manishearth/triomphe
// - `std::sync::Arc` in the Rust Standard Library (1.81.0).
//     -  MIT or Apache-2.0 License

use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    ptr::NonNull,
};

#[cfg(not(moka_loom))]
use std::sync::atomic::{self, AtomicU32};

#[cfg(moka_loom)]
use loom::sync::atomic::{self, AtomicU32};

/// A thread-safe reference-counting pointer. `MiniArc` is similar to
/// `std::sync::Arc`, Atomically Reference Counted shared pointer, but with a few
/// differences:
///
/// - Smaller memory overhead:
///     - `MiniArc` does not support weak references, so it does not need to store a
///       weak reference count.
///     - `MiniArc` uses `AtomicU32` for the reference count, while `std::sync::Arc`
///       uses `AtomicUsize`. On a 64-bit system, `AtomicU32` is half the size of
///       `AtomicUsize`.
///         - Note: Depending on the value type `T`, the Rust compiler may add
///           padding to the internal struct of `MiniArc<T>`, so the actual memory
///           overhead may vary.
/// - Smaller code size:
///     - Only about 100 lines of code.
///         - This is because `MiniArc` provides only the methods needed for the
///           `moka` and `mini-moka` crates.
///     - Smaller code size means less chance of bugs.
pub(crate) struct MiniArc<T: ?Sized> {
    ptr: NonNull<ArcData<T>>,
}

struct ArcData<T: ?Sized> {
    ref_count: AtomicU32,
    data: T,
}

/// A soft limit on the amount of references that may be made to an `MiniArc`.
///
/// Going above this limit will abort your program (although not necessarily)
/// at _exactly_ `MAX_REFCOUNT + 1` references.
const MAX_REFCOUNT: u32 = (i32::MAX) as u32;

unsafe impl<T: ?Sized + Send + Sync> Send for MiniArc<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for MiniArc<T> {}

impl<T> MiniArc<T> {
    pub(crate) fn new(data: T) -> MiniArc<T> {
        MiniArc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicU32::new(1),
                data,
            }))),
        }
    }
}

impl<T: ?Sized> MiniArc<T> {
    /// Gets the number of [`MiniArc`] pointers to this allocation
    pub(crate) fn count(this: &Self) -> u32 {
        use atomic::Ordering::Acquire;

        this.data().ref_count.load(Acquire)
    }

    /// Returns `true` if the two `MiniArc`s point to the same allocation in a
    /// vein similar to [`ptr::eq`].
    ///
    /// # Safety
    ///
    /// This function is unreliable when `T` is a `dyn Trait`. Currently
    /// coercing `MiniArc<SomeTime>` to `MiniArc<dyn Trait>` is not possible, so
    /// this is not a problem in practice. However, if this coercion becomes
    /// possible in the future, this function may return incorrect results when
    /// comparing `MiniArc<dyn Trait>` instances.
    ///
    /// To fix this, we must rise the minimum supported Rust version (MSRV) to
    /// 1.76 and use `std::ptr::addr_eq` internally instead of `eq` (`==`).
    /// `addr_eq` compares the _addresses_ of the pointers for equality,
    /// ignoring any metadata in fat pointers.
    ///
    /// See the following `triomphe` issue for more information:
    /// https://github.com/Manishearth/triomphe/pull/84
    ///
    /// Note that `triomphe` has a feature called `unsize`, which enables the
    /// coercion by using the `unsize` crate. `MiniArc` does not have such a
    /// feature, so we are safe for now.
    #[inline]
    #[allow(ambiguous_wide_pointer_comparisons)] // Remove this when MSRV is 1.76 or newer.
    #[allow(clippy::ptr_eq)] // Remove this when MSRV is 1.76 or newer.
    pub(crate) fn ptr_eq(this: &Self, other: &Self) -> bool {
        // `addr_eq` requires Rust 1.76 or newer.
        // ptr::addr_eq(this.ptr.as_ptr(), other.ptr.as_ptr())
        this.ptr.as_ptr() == other.ptr.as_ptr()
    }

    #[inline]
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> Deref for MiniArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T: ?Sized> Clone for MiniArc<T> {
    fn clone(&self) -> Self {
        use atomic::Ordering::Relaxed;

        if self.data().ref_count.fetch_add(1, Relaxed) > MAX_REFCOUNT {
            std::process::abort();
        }

        MiniArc { ptr: self.ptr }
    }
}

impl<T: ?Sized> Drop for MiniArc<T> {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering::{Acquire, Release};

        if self.data().ref_count.fetch_sub(1, Release) == 1 {
            atomic::fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T: Default> Default for MiniArc<T> {
    /// Creates a new `MiniArc<T>`, with the `Default` value for `T`.
    fn default() -> MiniArc<T> {
        MiniArc::new(Default::default())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for MiniArc<T> {
    fn eq(&self, other: &MiniArc<T>) -> bool {
        // TODO: pointer equality is incorrect if `T` is not `Eq`.
        // See: https://github.com/Manishearth/triomphe/pull/88
        Self::ptr_eq(self, other) || *(*self) == *(*other)
    }

    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &MiniArc<T>) -> bool {
        !Self::ptr_eq(self, other) && *(*self) != *(*other)
    }
}

impl<T: ?Sized + Eq> Eq for MiniArc<T> {}

impl<T: ?Sized + fmt::Display> fmt::Display for MiniArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for MiniArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for MiniArc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

impl<T: ?Sized + Hash> Hash for MiniArc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

#[cfg(all(test, not(moka_loom)))]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

    use super::*;

    #[test]
    fn test_drop() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Relaxed);
            }
        }

        // Create two MiniArcs sharing an object containing a string
        // and a DetectDrop, to detect when it is dropped.
        let x = MiniArc::new(("hello", DetectDrop));
        let y = x.clone();

        // Send x to another thread, and use it there.
        let t = std::thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });

        // In parallel, y should still be usable here.
        assert_eq!(y.0, "hello");
        assert!(MiniArc::count(&y) >= 1);

        // Wait for the thread to finish.
        t.join().unwrap();

        // One MiniArc, x, should be dropped by now.
        // We still have y, so the object should not have been dropped yet.
        assert_eq!(NUM_DROPS.load(Relaxed), 0);
        assert_eq!(MiniArc::count(&y), 1);

        // Drop the remaining `MiniArc`.
        drop(y);

        // Now that `y` is dropped too,
        // the object should have been dropped.
        assert_eq!(NUM_DROPS.load(Relaxed), 1);
    }

    #[test]
    fn test_eq() {
        let w = MiniArc::new(6502);
        let x = w.clone();
        let y = MiniArc::new(6502);
        let z = MiniArc::new(8086);

        assert_eq!(w, x);
        assert_eq!(x, w);
        assert_eq!(w, y);
        assert_eq!(y, w);
        assert_ne!(y, z);
        assert_ne!(z, y);
    }

    #[test]
    fn test_partial_eq_bug() {
        let float = f32::NAN;
        assert_ne!(float, float);
        let arc = MiniArc::new(f32::NAN);
        // TODO: this is a bug.
        // See: https://github.com/Manishearth/triomphe/pull/88
        assert_eq!(arc, arc);
    }

    #[allow(dead_code)]
    const fn is_partial_eq<T: ?Sized + PartialEq>() {}

    #[allow(dead_code)]
    const fn is_eq<T: ?Sized + Eq>() {}

    // compile-time check that PartialEq/Eq is correctly derived
    const _: () = is_partial_eq::<MiniArc<i32>>();
    const _: () = is_eq::<MiniArc<i32>>();
}

#[cfg(all(test, moka_loom))]
mod loom_tests {
    use super::*;

    #[test]
    fn test_drop() {
        use loom::sync::atomic::{AtomicUsize, Ordering::Relaxed};

        struct DetectDrop(loom::sync::Arc<AtomicUsize>);

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                self.0.fetch_add(1, Relaxed);
            }
        }

        loom::model(move || {
            let num_drops = loom::sync::Arc::new(AtomicUsize::new(0));

            // Create two MiniArcs sharing an object containing a string
            // and a DetectDrop, to detect when it is dropped.
            let x = MiniArc::new(("hello", DetectDrop(loom::sync::Arc::clone(&num_drops))));
            let y = x.clone();

            // Send x to another thread, and use it there.
            let t = loom::thread::spawn(move || {
                assert_eq!(x.0, "hello");
            });

            // In parallel, y should still be usable here.
            assert_eq!(y.0, "hello");
            assert!(MiniArc::count(&y) >= 1);

            // Wait for the thread to finish.
            t.join().unwrap();

            // One MiniArc, x, should be dropped by now.
            // We still have y, so the object should not have been dropped yet.
            assert_eq!(num_drops.load(Relaxed), 0);
            assert_eq!(MiniArc::count(&y), 1);

            // Drop the remaining `MiniArc`.
            drop(y);

            // Now that `y` is dropped too,
            // the object should have been dropped.
            assert_eq!(num_drops.load(Relaxed), 1);
        });
    }
}
