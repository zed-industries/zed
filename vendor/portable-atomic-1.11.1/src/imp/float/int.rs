// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic float implementation based on atomic integer.

This module provides atomic float implementations using atomic integer.

Note that most of `fetch_*` operations of atomic floats are implemented using
CAS loops, which can be slower than equivalent operations of atomic integers.

AArch64 with FEAT_LSFE and GPU targets have atomic instructions for float.
See aarch64.rs for AArch64 with FEAT_LSFE.
GPU targets will also use architecture-specific implementations instead of this implementation in the
future: https://github.com/taiki-e/portable-atomic/issues/34 / https://github.com/taiki-e/portable-atomic/pull/45
*/

// TODO: fetch_{minimum,maximum}* https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2024/p3008r2.html / https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2024/p0493r5.pdf

#![cfg_attr(
    all(target_pointer_width = "16", not(feature = "fallback")),
    allow(unused_imports, unused_macros)
)]

use core::{cell::UnsafeCell, sync::atomic::Ordering};

macro_rules! atomic_float {
    (
        $atomic_type:ident, $float_type:ident, $atomic_int_type:ident, $int_type:ident,
        $align:literal
    ) => {
        #[repr(C, align($align))]
        pub(crate) struct $atomic_type {
            v: UnsafeCell<$float_type>,
        }

        // Send is implicitly implemented.
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl Sync for $atomic_type {}

        impl $atomic_type {
            #[inline]
            pub(crate) const fn new(v: $float_type) -> Self {
                Self { v: UnsafeCell::new(v) }
            }

            #[inline]
            pub(crate) fn is_lock_free() -> bool {
                crate::$atomic_int_type::is_lock_free()
            }
            pub(crate) const IS_ALWAYS_LOCK_FREE: bool =
                crate::$atomic_int_type::is_always_lock_free();

            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn load(&self, order: Ordering) -> $float_type {
                $float_type::from_bits(self.as_bits().load(order))
            }

            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn store(&self, val: $float_type, order: Ordering) {
                self.as_bits().store(val.to_bits(), order)
            }

            const_fn! {
                const_if: #[cfg(not(portable_atomic_no_const_raw_ptr_deref))];
                #[inline(always)]
                pub(crate) const fn as_bits(&self) -> &crate::$atomic_int_type {
                    // SAFETY: $atomic_type and $atomic_int_type have the same layout,
                    // and there is no concurrent access to the value that does not go through this method.
                    unsafe { &*(self as *const Self as *const crate::$atomic_int_type) }
                }
            }

            #[inline]
            pub(crate) const fn as_ptr(&self) -> *mut $float_type {
                self.v.get()
            }
        }

        cfg_has_atomic_cas_or_amo32! {
        impl $atomic_type {
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn swap(&self, val: $float_type, order: Ordering) -> $float_type {
                $float_type::from_bits(self.as_bits().swap(val.to_bits(), order))
            }

            cfg_has_atomic_cas! {
            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn compare_exchange(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type> {
                match self.as_bits().compare_exchange(
                    current.to_bits(),
                    new.to_bits(),
                    success,
                    failure,
                ) {
                    Ok(v) => Ok($float_type::from_bits(v)),
                    Err(v) => Err($float_type::from_bits(v)),
                }
            }

            #[inline]
            #[cfg_attr(
                any(all(debug_assertions, not(portable_atomic_no_track_caller)), miri),
                track_caller
            )]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $float_type,
                new: $float_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$float_type, $float_type> {
                match self.as_bits().compare_exchange_weak(
                    current.to_bits(),
                    new.to_bits(),
                    success,
                    failure,
                ) {
                    Ok(v) => Ok($float_type::from_bits(v)),
                    Err(v) => Err($float_type::from_bits(v)),
                }
            }

            #[cfg(not(all(
                any(target_arch = "aarch64", target_arch = "arm64ec"),
                any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
                target_feature = "neon", // for vreg
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_add(&self, val: $float_type, order: Ordering) -> $float_type {
                self.fetch_update_(order, |x| x + val)
            }

            #[cfg(not(all(
                any(target_arch = "aarch64", target_arch = "arm64ec"),
                any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
                target_feature = "neon", // for vreg
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_sub(&self, val: $float_type, order: Ordering) -> $float_type {
                self.fetch_update_(order, |x| x - val)
            }

            #[cfg(not(all(
                any(target_arch = "aarch64", target_arch = "arm64ec"),
                any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
                target_feature = "neon", // for vreg
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            fn fetch_update_<F>(&self, order: Ordering, mut f: F) -> $float_type
            where
                F: FnMut($float_type) -> $float_type,
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

            #[cfg(not(all(
                any(target_arch = "aarch64", target_arch = "arm64ec"),
                any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
                target_feature = "neon", // for vreg
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_max(&self, val: $float_type, order: Ordering) -> $float_type {
                self.fetch_update_(order, |x| x.max(val))
            }

            #[cfg(not(all(
                any(target_arch = "aarch64", target_arch = "arm64ec"),
                any(target_feature = "lsfe", portable_atomic_target_feature = "lsfe"),
                target_feature = "neon", // for vreg
                not(any(miri, portable_atomic_sanitize_thread)),
                any(not(portable_atomic_no_asm), portable_atomic_unstable_asm),
            )))]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_min(&self, val: $float_type, order: Ordering) -> $float_type {
                self.fetch_update_(order, |x| x.min(val))
            }
            } // cfg_has_atomic_cas!

            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_neg(&self, order: Ordering) -> $float_type {
                const NEG_MASK: $int_type = !0 / 2 + 1;
                $float_type::from_bits(self.as_bits().fetch_xor(NEG_MASK, order))
            }

            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub(crate) fn fetch_abs(&self, order: Ordering) -> $float_type {
                const ABS_MASK: $int_type = !0 / 2;
                $float_type::from_bits(self.as_bits().fetch_and(ABS_MASK, order))
            }
        }
        } // cfg_has_atomic_cas_or_amo32!
    };
}

#[cfg(portable_atomic_unstable_f16)]
cfg_has_atomic_16! {
    atomic_float!(AtomicF16, f16, AtomicU16, u16, 2);
}
cfg_has_atomic_32! {
    atomic_float!(AtomicF32, f32, AtomicU32, u32, 4);
}
cfg_has_atomic_64! {
    atomic_float!(AtomicF64, f64, AtomicU64, u64, 8);
}
#[cfg(portable_atomic_unstable_f128)]
cfg_has_atomic_128! {
    atomic_float!(AtomicF128, f128, AtomicU128, u128, 16);
}
