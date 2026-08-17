// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Generic `Atomic<T>` wrapper type
//!
//! Atomic types provide primitive shared-memory communication between
//! threads, and are the building blocks of other concurrent types.
//!
//! This library defines a generic atomic wrapper type `Atomic<T>` for all
//! `T: Copy` types.
//! Atomic types present operations that, when used correctly, synchronize
//! updates between threads.
//!
//! Each method takes an `Ordering` which represents the strength of
//! the memory barrier for that operation. These orderings are the
//! same as [LLVM atomic orderings][1].
//!
//! [1]: http://llvm.org/docs/LangRef.html#memory-model-for-concurrent-operations
//!
//! Atomic variables are safe to share between threads (they implement `Sync`)
//! but they do not themselves provide the mechanism for sharing. The most
//! common way to share an atomic variable is to put it into an `Arc` (an
//! atomically-reference-counted shared pointer).
//!
//! Most atomic types may be stored in static variables, initialized using
//! the `const fn` constructors. Atomic statics are often used for lazy global
//! initialization.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![no_std]
#![cfg_attr(feature = "nightly", feature(integer_atomics))]

#[cfg(any(test, feature = "std"))]
#[macro_use]
extern crate std;

use core::mem::MaybeUninit;
// Re-export some useful definitions from libcore
pub use core::sync::atomic::{fence, Ordering};

use core::cell::UnsafeCell;
use core::fmt;

#[cfg(feature = "std")]
use std::panic::RefUnwindSafe;

#[cfg(feature = "fallback")]
mod fallback;
mod ops;

/// A generic atomic wrapper type which allows an object to be safely shared
/// between threads.
#[repr(transparent)]
pub struct Atomic<T> {
    // The MaybeUninit is here to work around rust-lang/rust#87341.
    v: UnsafeCell<MaybeUninit<T>>,
}

// Atomic<T> is only Sync if T is Send
unsafe impl<T: Copy + Send> Sync for Atomic<T> {}

// Given that atomicity is guaranteed, Atomic<T> is RefUnwindSafe if T is
//
// This is trivially correct for native lock-free atomic types. For those whose
// atomicity is emulated using a spinlock, it is still correct because the
// `Atomic` API does not allow doing any panic-inducing operation after writing
// to the target object.
#[cfg(feature = "std")]
impl<T: Copy + RefUnwindSafe> RefUnwindSafe for Atomic<T> {}

impl<T: Copy + Default> Default for Atomic<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: Copy + fmt::Debug> fmt::Debug for Atomic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Atomic")
            .field(&self.load(Ordering::SeqCst))
            .finish()
    }
}

impl<T> Atomic<T> {
    /// Creates a new `Atomic`.
    #[inline]
    pub const fn new(v: T) -> Atomic<T> {
        Atomic {
            v: UnsafeCell::new(MaybeUninit::new(v)),
        }
    }

    /// Checks if `Atomic` objects of this type are lock-free.
    ///
    /// If an `Atomic` is not lock-free then it may be implemented using locks
    /// internally, which makes it unsuitable for some situations (such as
    /// communicating with a signal handler).
    #[inline]
    pub const fn is_lock_free() -> bool {
        ops::atomic_is_lock_free::<T>()
    }
}

impl<T: Copy> Atomic<T> {
    #[inline]
    fn inner_ptr(&self) -> *mut T {
        self.v.get() as *mut T
    }

    /// Returns a mutable reference to the underlying type.
    ///
    /// This is safe because the mutable reference guarantees that no other threads are
    /// concurrently accessing the atomic data.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner_ptr() }
    }

    /// Consumes the atomic and returns the contained value.
    ///
    /// This is safe because passing `self` by value guarantees that no other threads are
    /// concurrently accessing the atomic data.
    #[inline]
    pub fn into_inner(self) -> T {
        unsafe { self.v.into_inner().assume_init() }
    }

    /// Loads a value from the `Atomic`.
    ///
    /// `load` takes an `Ordering` argument which describes the memory ordering
    /// of this operation.
    ///
    /// # Panics
    ///
    /// Panics if `order` is `Release` or `AcqRel`.
    #[inline]
    pub fn load(&self, order: Ordering) -> T {
        unsafe { ops::atomic_load(self.inner_ptr(), order) }
    }

    /// Stores a value into the `Atomic`.
    ///
    /// `store` takes an `Ordering` argument which describes the memory ordering
    /// of this operation.
    ///
    /// # Panics
    ///
    /// Panics if `order` is `Acquire` or `AcqRel`.
    #[inline]
    pub fn store(&self, val: T, order: Ordering) {
        unsafe {
            ops::atomic_store(self.inner_ptr(), val, order);
        }
    }

    /// Stores a value into the `Atomic`, returning the old value.
    ///
    /// `swap` takes an `Ordering` argument which describes the memory ordering
    /// of this operation.
    #[inline]
    pub fn swap(&self, val: T, order: Ordering) -> T {
        unsafe { ops::atomic_swap(self.inner_ptr(), val, order) }
    }

    /// Stores a value into the `Atomic` if the current value is the same as the
    /// `current` value.
    ///
    /// The return value is a result indicating whether the new value was
    /// written and containing the previous value. On success this value is
    /// guaranteed to be equal to `new`.
    ///
    /// `compare_exchange` takes two `Ordering` arguments to describe the memory
    /// ordering of this operation. The first describes the required ordering if
    /// the operation succeeds while the second describes the required ordering
    /// when the operation fails. The failure ordering can't be `Release` or
    /// `AcqRel` and must be equivalent or weaker than the success ordering.
    #[inline]
    pub fn compare_exchange(
        &self,
        current: T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, T> {
        unsafe { ops::atomic_compare_exchange(self.inner_ptr(), current, new, success, failure) }
    }

    /// Stores a value into the `Atomic` if the current value is the same as the
    /// `current` value.
    ///
    /// Unlike `compare_exchange`, this function is allowed to spuriously fail
    /// even when the comparison succeeds, which can result in more efficient
    /// code on some platforms. The return value is a result indicating whether
    /// the new value was written and containing the previous value.
    ///
    /// `compare_exchange` takes two `Ordering` arguments to describe the memory
    /// ordering of this operation. The first describes the required ordering if
    /// the operation succeeds while the second describes the required ordering
    /// when the operation fails. The failure ordering can't be `Release` or
    /// `AcqRel` and must be equivalent or weaker than the success ordering.
    /// success ordering.
    #[inline]
    pub fn compare_exchange_weak(
        &self,
        current: T,
        new: T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<T, T> {
        unsafe {
            ops::atomic_compare_exchange_weak(self.inner_ptr(), current, new, success, failure)
        }
    }

    /// Fetches the value, and applies a function to it that returns an optional
    /// new value. Returns a `Result` of `Ok(previous_value)` if the function returned `Some(_)`, else
    /// `Err(previous_value)`.
    ///
    /// Note: This may call the function multiple times if the value has been changed from other threads in
    /// the meantime, as long as the function returns `Some(_)`, but the function will have been applied
    /// only once to the stored value.
    ///
    /// `fetch_update` takes two [`Ordering`] arguments to describe the memory ordering of this operation.
    /// The first describes the required ordering for when the operation finally succeeds while the second
    /// describes the required ordering for loads. These correspond to the success and failure orderings of
    /// [`compare_exchange`] respectively.
    ///
    /// Using [`Acquire`] as success ordering makes the store part
    /// of this operation [`Relaxed`], and using [`Release`] makes the final successful load
    /// [`Relaxed`]. The (failed) load ordering can only be [`SeqCst`], [`Acquire`] or [`Relaxed`]
    /// and must be equivalent to or weaker than the success ordering.
    ///
    /// [`compare_exchange`]: #method.compare_exchange
    /// [`Ordering`]: enum.Ordering.html
    /// [`Relaxed`]: enum.Ordering.html#variant.Relaxed
    /// [`Release`]: enum.Ordering.html#variant.Release
    /// [`Acquire`]: enum.Ordering.html#variant.Acquire
    /// [`SeqCst`]: enum.Ordering.html#variant.SeqCst
    ///
    /// # Examples
    ///
    /// ```rust
    /// use atomic::{Atomic, Ordering};
    ///
    /// let x = Atomic::new(7);
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));
    /// assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));
    /// assert_eq!(x.load(Ordering::SeqCst), 9);
    /// ```
    #[inline]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        mut f: F,
    ) -> Result<T, T>
    where
        F: FnMut(T) -> Option<T>,
    {
        let mut prev = self.load(fetch_order);
        while let Some(next) = f(prev) {
            match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
                x @ Ok(_) => return x,
                Err(next_prev) => prev = next_prev,
            }
        }
        Err(prev)
    }
}

impl Atomic<bool> {
    /// Logical "and" with a boolean value.
    ///
    /// Performs a logical "and" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    #[inline]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        unsafe { ops::atomic_and(self.inner_ptr(), val, order) }
    }

    /// Logical "or" with a boolean value.
    ///
    /// Performs a logical "or" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    #[inline]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        unsafe { ops::atomic_or(self.inner_ptr(), val, order) }
    }

    /// Logical "xor" with a boolean value.
    ///
    /// Performs a logical "xor" operation on the current value and the argument
    /// `val`, and sets the new value to the result.
    ///
    /// Returns the previous value.
    #[inline]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        unsafe { ops::atomic_xor(self.inner_ptr(), val, order) }
    }
}

macro_rules! atomic_ops_common {
    ($($t:ty)*) => ($(
        impl Atomic<$t> {
            /// Add to the current value, returning the previous value.
            #[inline]
            pub fn fetch_add(&self, val: $t, order: Ordering) -> $t {
                unsafe { ops::atomic_add(self.inner_ptr(), val, order) }
            }

            /// Subtract from the current value, returning the previous value.
            #[inline]
            pub fn fetch_sub(&self, val: $t, order: Ordering) -> $t {
                unsafe { ops::atomic_sub(self.inner_ptr(), val, order) }
            }

            /// Bitwise and with the current value, returning the previous value.
            #[inline]
            pub fn fetch_and(&self, val: $t, order: Ordering) -> $t {
                unsafe { ops::atomic_and(self.inner_ptr(), val, order) }
            }

            /// Bitwise or with the current value, returning the previous value.
            #[inline]
            pub fn fetch_or(&self, val: $t, order: Ordering) -> $t {
                unsafe { ops::atomic_or(self.inner_ptr(), val, order) }
            }

            /// Bitwise xor with the current value, returning the previous value.
            #[inline]
            pub fn fetch_xor(&self, val: $t, order: Ordering) -> $t {
                unsafe { ops::atomic_xor(self.inner_ptr(), val, order) }
            }
        }
    )*);
}
macro_rules! atomic_ops_signed {
    ($($t:ty)*) => (
        atomic_ops_common!{ $($t)* }
        $(
            impl Atomic<$t> {
                /// Minimum with the current value.
                #[inline]
                pub fn fetch_min(&self, val: $t, order: Ordering) -> $t {
                    unsafe { ops::atomic_min(self.inner_ptr(), val, order) }
                }

                /// Maximum with the current value.
                #[inline]
                pub fn fetch_max(&self, val: $t, order: Ordering) -> $t {
                    unsafe { ops::atomic_max(self.inner_ptr(), val, order) }
                }
            }
        )*
    );
}
macro_rules! atomic_ops_unsigned {
    ($($t:ty)*) => (
        atomic_ops_common!{ $($t)* }
        $(
            impl Atomic<$t> {
                /// Minimum with the current value.
                #[inline]
                pub fn fetch_min(&self, val: $t, order: Ordering) -> $t {
                    unsafe { ops::atomic_umin(self.inner_ptr(), val, order) }
                }

                /// Maximum with the current value.
                #[inline]
                pub fn fetch_max(&self, val: $t, order: Ordering) -> $t {
                    unsafe { ops::atomic_umax(self.inner_ptr(), val, order) }
                }
            }
        )*
    );
}
atomic_ops_signed! { i8 i16 i32 i64 isize i128 }
atomic_ops_unsigned! { u8 u16 u32 u64 usize u128 }

#[cfg(test)]
mod tests {
    use super::{Atomic, Ordering::*};
    use core::mem;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    struct Foo(u8, u8);
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    struct Bar(u64, u64);
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    struct Quux(u32);

    #[test]
    fn atomic_bool() {
        let a = Atomic::new(false);
        assert_eq!(
            Atomic::<bool>::is_lock_free(),
            cfg!(target_has_atomic = "8"),
        );
        assert_eq!(format!("{:?}", a), "Atomic(false)");
        assert_eq!(a.load(SeqCst), false);
        a.store(true, SeqCst);
        assert_eq!(a.swap(false, SeqCst), true);
        assert_eq!(a.compare_exchange(true, false, SeqCst, SeqCst), Err(false));
        assert_eq!(a.compare_exchange(false, true, SeqCst, SeqCst), Ok(false));
        assert_eq!(a.fetch_and(false, SeqCst), true);
        assert_eq!(a.fetch_or(true, SeqCst), false);
        assert_eq!(a.fetch_xor(false, SeqCst), true);
        assert_eq!(a.load(SeqCst), true);
    }

    #[test]
    fn atomic_i8() {
        let a = Atomic::new(0i8);
        assert_eq!(Atomic::<i8>::is_lock_free(), cfg!(target_has_atomic = "8"));
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        // Make sure overflows are handled correctly
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), -74);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_i16() {
        let a = Atomic::new(0i16);
        assert_eq!(
            Atomic::<i16>::is_lock_free(),
            cfg!(target_has_atomic = "16")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 182);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_i32() {
        let a = Atomic::new(0i32);
        assert_eq!(
            Atomic::<i32>::is_lock_free(),
            cfg!(target_has_atomic = "32")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 182);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_i64() {
        let a = Atomic::new(0i64);
        assert_eq!(
            Atomic::<i64>::is_lock_free(),
            cfg!(target_has_atomic = "64") && mem::align_of::<i64>() == 8
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 182);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_i128() {
        let a = Atomic::new(0i128);
        assert_eq!(
            Atomic::<i128>::is_lock_free(),
            cfg!(feature = "nightly") & cfg!(target_has_atomic = "128")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 182);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_isize() {
        let a = Atomic::new(0isize);
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(-56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 182);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(-25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_u8() {
        let a = Atomic::new(0u8);
        assert_eq!(Atomic::<u8>::is_lock_free(), cfg!(target_has_atomic = "8"));
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_u16() {
        let a = Atomic::new(0u16);
        assert_eq!(
            Atomic::<u16>::is_lock_free(),
            cfg!(target_has_atomic = "16")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_u32() {
        let a = Atomic::new(0u32);
        assert_eq!(
            Atomic::<u32>::is_lock_free(),
            cfg!(target_has_atomic = "32")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_u64() {
        let a = Atomic::new(0u64);
        assert_eq!(
            Atomic::<u64>::is_lock_free(),
            cfg!(target_has_atomic = "64") && mem::align_of::<u64>() == 8
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_u128() {
        let a = Atomic::new(0u128);
        assert_eq!(
            Atomic::<u128>::is_lock_free(),
            cfg!(feature = "nightly") & cfg!(target_has_atomic = "128")
        );
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_usize() {
        let a = Atomic::new(0usize);
        assert_eq!(format!("{:?}", a), "Atomic(0)");
        assert_eq!(a.load(SeqCst), 0);
        a.store(1, SeqCst);
        assert_eq!(a.swap(2, SeqCst), 1);
        assert_eq!(a.compare_exchange(5, 45, SeqCst, SeqCst), Err(2));
        assert_eq!(a.compare_exchange(2, 3, SeqCst, SeqCst), Ok(2));
        assert_eq!(a.fetch_add(123, SeqCst), 3);
        assert_eq!(a.fetch_sub(56, SeqCst), 126);
        assert_eq!(a.fetch_and(7, SeqCst), 70);
        assert_eq!(a.fetch_or(64, SeqCst), 6);
        assert_eq!(a.fetch_xor(1, SeqCst), 70);
        assert_eq!(a.fetch_min(30, SeqCst), 71);
        assert_eq!(a.fetch_max(25, SeqCst), 30);
        assert_eq!(a.load(SeqCst), 30);
    }

    #[test]
    fn atomic_foo() {
        let a = Atomic::default();
        assert_eq!(Atomic::<Foo>::is_lock_free(), false);
        assert_eq!(format!("{:?}", a), "Atomic(Foo(0, 0))");
        assert_eq!(a.load(SeqCst), Foo(0, 0));
        a.store(Foo(1, 1), SeqCst);
        assert_eq!(a.swap(Foo(2, 2), SeqCst), Foo(1, 1));
        assert_eq!(
            a.compare_exchange(Foo(5, 5), Foo(45, 45), SeqCst, SeqCst),
            Err(Foo(2, 2))
        );
        assert_eq!(
            a.compare_exchange(Foo(2, 2), Foo(3, 3), SeqCst, SeqCst),
            Ok(Foo(2, 2))
        );
        assert_eq!(a.load(SeqCst), Foo(3, 3));
    }

    #[test]
    fn atomic_bar() {
        let a = Atomic::default();
        assert_eq!(Atomic::<Bar>::is_lock_free(), false);
        assert_eq!(format!("{:?}", a), "Atomic(Bar(0, 0))");
        assert_eq!(a.load(SeqCst), Bar(0, 0));
        a.store(Bar(1, 1), SeqCst);
        assert_eq!(a.swap(Bar(2, 2), SeqCst), Bar(1, 1));
        assert_eq!(
            a.compare_exchange(Bar(5, 5), Bar(45, 45), SeqCst, SeqCst),
            Err(Bar(2, 2))
        );
        assert_eq!(
            a.compare_exchange(Bar(2, 2), Bar(3, 3), SeqCst, SeqCst),
            Ok(Bar(2, 2))
        );
        assert_eq!(a.load(SeqCst), Bar(3, 3));
    }

    #[test]
    fn atomic_quxx() {
        let a = Atomic::default();
        assert_eq!(
            Atomic::<Quux>::is_lock_free(),
            cfg!(target_has_atomic = "32")
        );
        assert_eq!(format!("{:?}", a), "Atomic(Quux(0))");
        assert_eq!(a.load(SeqCst), Quux(0));
        a.store(Quux(1), SeqCst);
        assert_eq!(a.swap(Quux(2), SeqCst), Quux(1));
        assert_eq!(
            a.compare_exchange(Quux(5), Quux(45), SeqCst, SeqCst),
            Err(Quux(2))
        );
        assert_eq!(
            a.compare_exchange(Quux(2), Quux(3), SeqCst, SeqCst),
            Ok(Quux(2))
        );
        assert_eq!(a.load(SeqCst), Quux(3));
    }
}
