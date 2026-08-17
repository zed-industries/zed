// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Wrap the standard library's atomic types in newtype.

This is not a reexport, because we want to backport changes like
https://github.com/rust-lang/rust/pull/98383 to old compilers.
*/

use core::{cell::UnsafeCell, marker::PhantomData, sync::atomic::Ordering};

// core::panic::RefUnwindSafe is only available on Rust 1.56+, so on pre-1.56
// Rust, we implement RefUnwindSafe when "std" feature is enabled.
// However, on pre-1.56 Rust, the standard library's atomic types implement
// RefUnwindSafe when "linked to std", and that's behavior that our other atomic
// implementations can't emulate, so use PhantomData<NotRefUnwindSafe> to match
// conditions where our other atomic implementations implement RefUnwindSafe.
//
// If we do not do this, for example, downstream that is only tested on x86_64
// may incorrectly assume that AtomicU64 always implements RefUnwindSafe even on
// older rustc, and may be broken on platforms where std AtomicU64 is not available.
struct NotRefUnwindSafe(UnsafeCell<()>);
// SAFETY: this is a marker type and we'll never access the value.
unsafe impl Sync for NotRefUnwindSafe {}

#[repr(transparent)]
pub(crate) struct AtomicPtr<T> {
    inner: core::sync::atomic::AtomicPtr<T>,
    // Prevent RefUnwindSafe from being propagated from the std atomic type. See NotRefUnwindSafe for more.
    _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
}
impl<T> AtomicPtr<T> {
    #[inline]
    pub(crate) const fn new(v: *mut T) -> Self {
        Self { inner: core::sync::atomic::AtomicPtr::new(v), _not_ref_unwind_safe: PhantomData }
    }
    #[inline]
    pub(crate) fn is_lock_free() -> bool {
        Self::IS_ALWAYS_LOCK_FREE
    }
    pub(crate) const IS_ALWAYS_LOCK_FREE: bool = true;
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub(crate) fn load(&self, order: Ordering) -> *mut T {
        crate::utils::assert_load_ordering(order); // for track_caller (compiler can omit double check)
        self.inner.load(order)
    }
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub(crate) fn store(&self, ptr: *mut T, order: Ordering) {
        crate::utils::assert_store_ordering(order); // for track_caller (compiler can omit double check)
        self.inner.store(ptr, order);
    }
    const_fn! {
        const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
        #[inline]
        pub(crate) const fn as_ptr(&self) -> *mut *mut T {
            // SAFETY: Self is #[repr(C)] and internally UnsafeCell<*mut T>.
            // See also https://github.com/rust-lang/rust/pull/66705 and
            // https://github.com/rust-lang/rust/issues/66136#issuecomment-557867116.
            unsafe { (*(self as *const Self as *const UnsafeCell<*mut T>)).get() }
        }
    }
}
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_cas)))]
#[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
impl<T> AtomicPtr<T> {
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub(crate) fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        crate::utils::assert_compare_exchange_ordering(success, failure); // for track_caller (compiler can omit double check)
        #[cfg(portable_atomic_no_stronger_failure_ordering)]
        let success = crate::utils::upgrade_success_ordering(success, failure);
        self.inner.compare_exchange(current, new, success, failure)
    }
    #[inline]
    #[cfg_attr(
        any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
        track_caller
    )]
    pub(crate) fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        crate::utils::assert_compare_exchange_ordering(success, failure); // for track_caller (compiler can omit double check)
        #[cfg(portable_atomic_no_stronger_failure_ordering)]
        let success = crate::utils::upgrade_success_ordering(success, failure);
        self.inner.compare_exchange_weak(current, new, success, failure)
    }
}
impl<T> core::ops::Deref for AtomicPtr<T> {
    type Target = core::sync::atomic::AtomicPtr<T>;
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

macro_rules! atomic_int {
    ($atomic_type:ident, $int_type:ident) => {
        #[repr(transparent)]
        pub(crate) struct $atomic_type {
            inner: core::sync::atomic::$atomic_type,
            // Prevent RefUnwindSafe from being propagated from the std atomic type. See NotRefUnwindSafe for more.
            _not_ref_unwind_safe: PhantomData<NotRefUnwindSafe>,
        }
        #[cfg_attr(
            portable_atomic_no_cfg_target_has_atomic,
            cfg(not(portable_atomic_no_atomic_cas))
        )]
        #[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
        impl_default_no_fetch_ops!($atomic_type, $int_type);
        #[cfg(not(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            not(any(miri, portable_atomic_sanitize_thread)),
            any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
        )))]
        #[cfg_attr(
            portable_atomic_no_cfg_target_has_atomic,
            cfg(not(portable_atomic_no_atomic_cas))
        )]
        #[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
        impl_default_bit_opts!($atomic_type, $int_type);
        impl $atomic_type {
            #[inline]
            pub(crate) const fn new(v: $int_type) -> Self {
                Self {
                    inner: core::sync::atomic::$atomic_type::new(v),
                    _not_ref_unwind_safe: PhantomData,
                }
            }
            #[inline]
            pub(crate) fn is_lock_free() -> bool {
                Self::IS_ALWAYS_LOCK_FREE
            }
            // ESP-IDF targets' 64-bit atomics are not lock-free.
            // https://github.com/rust-lang/rust/pull/115577#issuecomment-1732259297
            pub(crate) const IS_ALWAYS_LOCK_FREE: bool = cfg!(not(all(
                any(target_arch = "riscv32", target_arch = "xtensa"),
                target_os = "espidf",
            ))) | (core::mem::size_of::<$int_type>()
                < 8);
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn load(&self, order: Ordering) -> $int_type {
                crate::utils::assert_load_ordering(order); // for track_caller (compiler can omit double check)
                self.inner.load(order)
            }
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn store(&self, val: $int_type, order: Ordering) {
                crate::utils::assert_store_ordering(order); // for track_caller (compiler can omit double check)
                self.inner.store(val, order);
            }
            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
                #[inline]
                pub(crate) const fn as_ptr(&self) -> *mut $int_type {
                    // SAFETY: Self is #[repr(C)] and internally UnsafeCell<$int_type>.
                    // See also https://github.com/rust-lang/rust/pull/66705 and
                    // https://github.com/rust-lang/rust/issues/66136#issuecomment-557867116.
                    unsafe {
                        (*(self as *const Self as *const UnsafeCell<$int_type>)).get()
                    }
                }
            }
        }
        #[cfg_attr(
            portable_atomic_no_cfg_target_has_atomic,
            cfg(not(portable_atomic_no_atomic_cas))
        )]
        #[cfg_attr(not(portable_atomic_no_cfg_target_has_atomic), cfg(target_has_atomic = "ptr"))]
        impl $atomic_type {
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure); // for track_caller (compiler can omit double check)
                #[cfg(portable_atomic_no_stronger_failure_ordering)]
                let success = crate::utils::upgrade_success_ordering(success, failure);
                self.inner.compare_exchange(current, new, success, failure)
            }
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure); // for track_caller (compiler can omit double check)
                #[cfg(portable_atomic_no_stronger_failure_ordering)]
                let success = crate::utils::upgrade_success_ordering(success, failure);
                self.inner.compare_exchange_weak(current, new, success, failure)
            }
            #[allow(dead_code)]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            fn fetch_update_<F>(&self, order: Ordering, mut f: F) -> $int_type
            where
                F: FnMut($int_type) -> $int_type,
            {
                // This is a private function and all instances of `f` only operate on the value
                // loaded, so there is no need to synchronize the first load/failed CAS.
                let mut prev = self.load(Ordering::Relaxed);
                loop {
                    let next = f(prev);
                    match self.compare_exchange_weak(prev, next, order, Ordering::Relaxed) {
                        Ok(x) => return x,
                        Err(next_prev) => prev = next_prev,
                    }
                }
            }
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                #[cfg(not(portable_atomic_no_atomic_min_max))]
                {
                    #[cfg(any(
                        all(
                            any(target_arch = "aarch64", target_arch = "arm64ec"),
                            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
                        ),
                        all(
                            target_arch = "arm",
                            not(any(
                                target_feature = "v6",
                                portable_atomic_target_feature = "v6",
                            )),
                        ),
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "powerpc",
                        target_arch = "powerpc64",
                    ))]
                    {
                        // HACK: the following operations are currently broken (at least on qemu-user):
                        // - aarch64's `AtomicI{8,16}::fetch_{max,min}` (release mode + lse)
                        // - armv5te's `Atomic{I,U}{8,16}::fetch_{max,min}`
                        // - mips's `AtomicI8::fetch_{max,min}` (release mode)
                        // - mipsel's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - mips64's `AtomicI8::fetch_{max,min}` (release mode)
                        // - mips64el's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - powerpc's `AtomicI{8,16}::fetch_{max,min}`
                        // - powerpc64's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - powerpc64le's `AtomicU{8,16}::fetch_{max,min}` (release mode + fat LTO)
                        // See also:
                        // https://github.com/llvm/llvm-project/issues/61880
                        // https://github.com/llvm/llvm-project/issues/61881
                        // https://github.com/llvm/llvm-project/issues/61882
                        // https://github.com/taiki-e/portable-atomic/issues/2
                        // https://github.com/rust-lang/rust/issues/100650
                        if core::mem::size_of::<$int_type>() <= 2 {
                            return self.fetch_update_(order, |x| core::cmp::max(x, val));
                        }
                    }
                    self.inner.fetch_max(val, order)
                }
                #[cfg(portable_atomic_no_atomic_min_max)]
                {
                    self.fetch_update_(order, |x| core::cmp::max(x, val))
                }
            }
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type {
                #[cfg(not(portable_atomic_no_atomic_min_max))]
                {
                    #[cfg(any(
                        all(
                            any(target_arch = "aarch64", target_arch = "arm64ec"),
                            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
                        ),
                        all(
                            target_arch = "arm",
                            not(any(
                                target_feature = "v6",
                                portable_atomic_target_feature = "v6",
                            )),
                        ),
                        target_arch = "mips",
                        target_arch = "mips32r6",
                        target_arch = "mips64",
                        target_arch = "mips64r6",
                        target_arch = "powerpc",
                        target_arch = "powerpc64",
                    ))]
                    {
                        // HACK: the following operations are currently broken (at least on qemu-user):
                        // - aarch64's `AtomicI{8,16}::fetch_{max,min}` (release mode + lse)
                        // - armv5te's `Atomic{I,U}{8,16}::fetch_{max,min}`
                        // - mips's `AtomicI8::fetch_{max,min}` (release mode)
                        // - mipsel's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - mips64's `AtomicI8::fetch_{max,min}` (release mode)
                        // - mips64el's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - powerpc's `AtomicI{8,16}::fetch_{max,min}`
                        // - powerpc64's `AtomicI{8,16}::fetch_{max,min}` (debug mode, at least)
                        // - powerpc64le's `AtomicU{8,16}::fetch_{max,min}` (release mode + fat LTO)
                        // See also:
                        // https://github.com/llvm/llvm-project/issues/61880
                        // https://github.com/llvm/llvm-project/issues/61881
                        // https://github.com/llvm/llvm-project/issues/61882
                        // https://github.com/taiki-e/portable-atomic/issues/2
                        // https://github.com/rust-lang/rust/issues/100650
                        if core::mem::size_of::<$int_type>() <= 2 {
                            return self.fetch_update_(order, |x| core::cmp::min(x, val));
                        }
                    }
                    self.inner.fetch_min(val, order)
                }
                #[cfg(portable_atomic_no_atomic_min_max)]
                {
                    self.fetch_update_(order, |x| core::cmp::min(x, val))
                }
            }
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_not(&self, order: Ordering) -> $int_type {
                self.fetch_xor(!0, order)
            }
            #[cfg(not(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }
            // TODO: provide asm-based implementation on AArch64 without FEAT_LSE, Armv7, RISC-V, etc.
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_neg(&self, order: Ordering) -> $int_type {
                self.fetch_update_(order, $int_type::wrapping_neg)
            }
            #[cfg(not(all(
                any(target_arch = "x86", target_arch = "x86_64"),
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn neg(&self, order: Ordering) {
                self.fetch_neg(order);
            }
        }
        impl core::ops::Deref for $atomic_type {
            type Target = core::sync::atomic::$atomic_type;
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

atomic_int!(AtomicIsize, isize);
atomic_int!(AtomicUsize, usize);
#[cfg(not(portable_atomic_no_atomic_load_store))]
atomic_int!(AtomicI8, i8);
#[cfg(not(portable_atomic_no_atomic_load_store))]
atomic_int!(AtomicU8, u8);
#[cfg(not(portable_atomic_no_atomic_load_store))]
atomic_int!(AtomicI16, i16);
#[cfg(not(portable_atomic_no_atomic_load_store))]
atomic_int!(AtomicU16, u16);
#[cfg(not(portable_atomic_no_atomic_load_store))]
#[cfg(not(target_pointer_width = "16"))]
atomic_int!(AtomicI32, i32);
#[cfg(not(portable_atomic_no_atomic_load_store))]
#[cfg(not(target_pointer_width = "16"))]
atomic_int!(AtomicU32, u32);
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_64)))]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        target_has_atomic = "64",
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
    ))
)]
atomic_int!(AtomicI64, i64);
#[cfg_attr(portable_atomic_no_cfg_target_has_atomic, cfg(not(portable_atomic_no_atomic_64)))]
#[cfg_attr(
    not(portable_atomic_no_cfg_target_has_atomic),
    cfg(any(
        target_has_atomic = "64",
        not(any(target_pointer_width = "16", target_pointer_width = "32")),
    ))
)]
atomic_int!(AtomicU64, u64);
