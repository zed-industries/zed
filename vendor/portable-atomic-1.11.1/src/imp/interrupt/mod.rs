// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Critical section based fallback implementations

This module supports two different critical section implementations:
- Built-in "disable all interrupts".
- Call into the `critical-section` crate (which allows the user to plug any implementation).

The `critical-section`-based fallback is enabled when the user asks for it with the `critical-section`
Cargo feature.

The "disable interrupts" fallback is not sound on multi-core systems.
Also, this uses privileged instructions to disable interrupts, so it usually
doesn't work on unprivileged mode. Using this fallback in an environment where privileged
instructions are not available is also usually considered **unsound**,
although the details are system-dependent.

Therefore, this implementation will only be enabled in one of the following cases:

- When the user explicitly declares that the system is single-core and that
  privileged instructions are available using an unsafe cfg.
- When we can safely assume that the system is single-core and that
  privileged instructions are available on the system.

AVR, which is single core[^avr1] and LLVM also generates code that disables
interrupts [^avr2] in atomic ops by default, is considered the latter.
MSP430 as well.

See also README.md of this directory.

[^avr1]: https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/AVR/AVRExpandPseudoInsts.cpp#L1074
[^avr2]: https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/test/CodeGen/AVR/atomics/load16.ll#L5
*/

// On some platforms, atomic load/store can be implemented in a more efficient
// way than disabling interrupts. On MSP430, some RMWs that do not return the
// previous value can also be optimized.
//
// Note: On single-core systems, it is okay to use critical session-based
// CAS together with atomic load/store. The load/store will not be
// called while interrupts are disabled, and since the load/store is
// atomic, it is not affected by interrupts even if interrupts are enabled.
#[cfg(not(any(
    all(target_arch = "avr", portable_atomic_no_asm),
    feature = "critical-section",
)))]
use self::arch::atomic;

#[cfg(not(feature = "critical-section"))]
#[cfg_attr(
    all(
        target_arch = "arm",
        any(target_feature = "mclass", portable_atomic_target_feature = "mclass"),
    ),
    path = "armv6m.rs"
)]
#[cfg_attr(
    all(
        target_arch = "arm",
        not(any(target_feature = "mclass", portable_atomic_target_feature = "mclass")),
    ),
    path = "armv4t.rs"
)]
#[cfg_attr(target_arch = "avr", path = "avr.rs")]
#[cfg_attr(target_arch = "msp430", path = "msp430.rs")]
#[cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), path = "riscv.rs")]
#[cfg_attr(target_arch = "xtensa", path = "xtensa.rs")]
mod arch;

use core::{cell::UnsafeCell, ptr, sync::atomic::Ordering};

// Critical section implementations might use locks internally.
#[cfg(feature = "critical-section")]
const IS_ALWAYS_LOCK_FREE: bool = false;
// Consider atomic operations based on disabling interrupts on single-core
// systems are lock-free. (We consider the pre-v6 Arm Linux's atomic operations
// provided in a similar way by the Linux kernel to be lock-free.)
#[cfg(not(feature = "critical-section"))]
const IS_ALWAYS_LOCK_FREE: bool = true;

#[cfg(feature = "critical-section")]
#[inline]
fn with<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    critical_section::with(|_| f())
}
#[cfg(not(feature = "critical-section"))]
#[inline(always)]
fn with<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // Get current interrupt state and disable interrupts
    let state = arch::disable();

    let r = f();

    // Restore interrupt state
    // SAFETY: the state was retrieved by the previous `disable`.
    unsafe { arch::restore(state) }

    r
}

#[cfg_attr(target_pointer_width = "16", repr(C, align(2)))]
#[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
#[cfg_attr(target_pointer_width = "128", repr(C, align(16)))]
pub(crate) struct AtomicPtr<T> {
    p: UnsafeCell<*mut T>,
}

// SAFETY: any data races are prevented by disabling interrupts or
// atomic intrinsics (see module-level comments).
unsafe impl<T> Send for AtomicPtr<T> {}
// SAFETY: any data races are prevented by disabling interrupts or
// atomic intrinsics (see module-level comments).
unsafe impl<T> Sync for AtomicPtr<T> {}

impl<T> AtomicPtr<T> {
    #[inline]
    pub(crate) const fn new(p: *mut T) -> Self {
        Self { p: UnsafeCell::new(p) }
    }

    #[inline]
    pub(crate) fn is_lock_free() -> bool {
        Self::IS_ALWAYS_LOCK_FREE
    }
    pub(crate) const IS_ALWAYS_LOCK_FREE: bool = IS_ALWAYS_LOCK_FREE;

    #[inline]
    #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
    pub(crate) fn load(&self, order: Ordering) -> *mut T {
        crate::utils::assert_load_ordering(order);
        #[cfg(not(any(target_arch = "avr", feature = "critical-section")))]
        {
            self.as_native().load(order)
        }
        #[cfg(any(target_arch = "avr", feature = "critical-section"))]
        // SAFETY: any data races are prevented by disabling interrupts (see
        // module-level comments) and the raw pointer is valid because we got it
        // from a reference.
        with(|| unsafe { self.p.get().read() })
    }

    #[inline]
    #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
    pub(crate) fn store(&self, ptr: *mut T, order: Ordering) {
        crate::utils::assert_store_ordering(order);
        #[cfg(not(any(target_arch = "avr", feature = "critical-section")))]
        {
            self.as_native().store(ptr, order);
        }
        #[cfg(any(target_arch = "avr", feature = "critical-section"))]
        // SAFETY: any data races are prevented by disabling interrupts (see
        // module-level comments) and the raw pointer is valid because we got it
        // from a reference.
        with(|| unsafe { self.p.get().write(ptr) });
    }

    #[inline]
    pub(crate) fn swap(&self, ptr: *mut T, order: Ordering) -> *mut T {
        let _ = order;
        #[cfg(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            not(feature = "critical-section"),
            any(
                portable_atomic_force_amo,
                target_feature = "zaamo",
                portable_atomic_target_feature = "zaamo",
            ),
        ))]
        {
            self.as_native().swap(ptr, order)
        }
        #[cfg(not(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            not(feature = "critical-section"),
            any(
                portable_atomic_force_amo,
                target_feature = "zaamo",
                portable_atomic_target_feature = "zaamo",
            ),
        )))]
        // SAFETY: any data races are prevented by disabling interrupts (see
        // module-level comments) and the raw pointer is valid because we got it
        // from a reference.
        with(|| unsafe {
            let prev = self.p.get().read();
            self.p.get().write(ptr);
            prev
        })
    }

    #[inline]
    #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
    pub(crate) fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        crate::utils::assert_compare_exchange_ordering(success, failure);
        // SAFETY: any data races are prevented by disabling interrupts (see
        // module-level comments) and the raw pointer is valid because we got it
        // from a reference.
        with(|| unsafe {
            let prev = self.p.get().read();
            if ptr::eq(prev, current) {
                self.p.get().write(new);
                Ok(prev)
            } else {
                Err(prev)
            }
        })
    }

    #[inline]
    #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
    pub(crate) fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange(current, new, success, failure)
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *mut *mut T {
        self.p.get()
    }

    #[cfg(not(any(target_arch = "avr", feature = "critical-section")))]
    #[inline(always)]
    fn as_native(&self) -> &atomic::AtomicPtr<T> {
        // SAFETY: AtomicPtr and atomic::AtomicPtr have the same layout and
        // guarantee atomicity in a compatible way. (see module-level comments)
        unsafe { &*(self as *const Self as *const atomic::AtomicPtr<T>) }
    }
}

macro_rules! atomic_int {
    (base, $atomic_type:ident, $int_type:ident, $align:literal) => {
        #[repr(C, align($align))]
        pub(crate) struct $atomic_type {
            v: UnsafeCell<$int_type>,
        }

        // Send is implicitly implemented.
        // SAFETY: any data races are prevented by disabling interrupts or
        // atomic intrinsics (see module-level comments).
        unsafe impl Sync for $atomic_type {}

        impl $atomic_type {
            #[inline]
            pub(crate) const fn new(v: $int_type) -> Self {
                Self { v: UnsafeCell::new(v) }
            }

            #[inline]
            pub(crate) fn is_lock_free() -> bool {
                Self::IS_ALWAYS_LOCK_FREE
            }
            pub(crate) const IS_ALWAYS_LOCK_FREE: bool = IS_ALWAYS_LOCK_FREE;

            #[inline]
            pub(crate) const fn as_ptr(&self) -> *mut $int_type {
                self.v.get()
            }
        }
    };
    (load_store_atomic $([$kind:ident])?, $atomic_type:ident, $int_type:ident, $align:literal) => {
        atomic_int!(base, $atomic_type, $int_type, $align);
        #[cfg(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            not(feature = "critical-section"),
            any(
                portable_atomic_force_amo,
                target_feature = "zaamo",
                portable_atomic_target_feature = "zaamo",
            ),
        ))]
        atomic_int!(cas $([$kind])?, $atomic_type, $int_type);
        #[cfg(not(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            not(feature = "critical-section"),
            any(
                portable_atomic_force_amo,
                target_feature = "zaamo",
                portable_atomic_target_feature = "zaamo",
            ),
        )))]
        atomic_int!(cas[emulate], $atomic_type, $int_type);
        impl $atomic_type {
            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn load(&self, order: Ordering) -> $int_type {
                crate::utils::assert_load_ordering(order);
                #[cfg(not(any(
                    all(target_arch = "avr", portable_atomic_no_asm),
                    feature = "critical-section",
                )))]
                {
                    self.as_native().load(order)
                }
                #[cfg(any(
                    all(target_arch = "avr", portable_atomic_no_asm),
                    feature = "critical-section",
                ))]
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe { self.v.get().read() })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn store(&self, val: $int_type, order: Ordering) {
                crate::utils::assert_store_ordering(order);
                #[cfg(not(any(
                    all(target_arch = "avr", portable_atomic_no_asm),
                    feature = "critical-section",
                )))]
                {
                    self.as_native().store(val, order);
                }
                #[cfg(any(
                    all(target_arch = "avr", portable_atomic_no_asm),
                    feature = "critical-section",
                ))]
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe { self.v.get().write(val) });
            }

            #[cfg(not(any(
                all(target_arch = "avr", portable_atomic_no_asm),
                feature = "critical-section",
            )))]
            #[inline(always)]
            fn as_native(&self) -> &atomic::$atomic_type {
                // SAFETY: $atomic_type and atomic::$atomic_type have the same layout and
                // guarantee atomicity in a compatible way. (see module-level comments)
                unsafe { &*(self as *const Self as *const atomic::$atomic_type) }
            }
        }

        #[cfg(not(all(target_arch = "msp430", not(feature = "critical-section"))))]
        impl_default_no_fetch_ops!($atomic_type, $int_type);
        impl_default_bit_opts!($atomic_type, $int_type);
        #[cfg(not(all(target_arch = "msp430", not(feature = "critical-section"))))]
        impl $atomic_type {
            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }
        }
        #[cfg(all(target_arch = "msp430", not(feature = "critical-section")))]
        impl $atomic_type {
            #[inline]
            pub(crate) fn add(&self, val: $int_type, order: Ordering) {
                self.as_native().add(val, order);
            }
            #[inline]
            pub(crate) fn sub(&self, val: $int_type, order: Ordering) {
                self.as_native().sub(val, order);
            }
            #[inline]
            pub(crate) fn and(&self, val: $int_type, order: Ordering) {
                self.as_native().and(val, order);
            }
            #[inline]
            pub(crate) fn or(&self, val: $int_type, order: Ordering) {
                self.as_native().or(val, order);
            }
            #[inline]
            pub(crate) fn xor(&self, val: $int_type, order: Ordering) {
                self.as_native().xor(val, order);
            }
            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.as_native().not(order);
            }
        }
    };
    (all_critical_session, $atomic_type:ident, $int_type:ident, $align:literal) => {
        atomic_int!(base, $atomic_type, $int_type, $align);
        atomic_int!(cas[emulate], $atomic_type, $int_type);
        impl_default_no_fetch_ops!($atomic_type, $int_type);
        impl_default_bit_opts!($atomic_type, $int_type);
        impl $atomic_type {
            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn load(&self, order: Ordering) -> $int_type {
                crate::utils::assert_load_ordering(order);
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe { self.v.get().read() })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn store(&self, val: $int_type, order: Ordering) {
                crate::utils::assert_store_ordering(order);
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe { self.v.get().write(val) });
            }

            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }
        }
    };
    (cas[emulate], $atomic_type:ident, $int_type:ident) => {
        impl $atomic_type {
            #[inline]
            pub(crate) fn swap(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(val);
                    prev
                })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure);
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    if prev == current {
                        self.v.get().write(new);
                        Ok(prev)
                    } else {
                        Err(prev)
                    }
                })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                self.compare_exchange(current, new, success, failure)
            }

            #[inline]
            pub(crate) fn fetch_add(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_add(val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_sub(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_sub(val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_and(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev & val);
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_nand(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(!(prev & val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev | val);
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_xor(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev ^ val);
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_max(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(core::cmp::max(prev, val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_min(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(core::cmp::min(prev, val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_not(&self, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(!prev);
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_neg(&self, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_neg());
                    prev
                })
            }
            #[inline]
            pub(crate) fn neg(&self, order: Ordering) {
                self.fetch_neg(order);
            }
        }
    };
    // RISC-V 32-bit(RV32)/{32,64}-bit(RV64) RMW with Zaamo extension
    // RISC-V 8-bit/16-bit RMW with Zabha extension
    (cas, $atomic_type:ident, $int_type:ident) => {
        impl $atomic_type {
            #[inline]
            pub(crate) fn swap(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().swap(val, order)
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure);
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    if prev == current {
                        self.v.get().write(new);
                        Ok(prev)
                    } else {
                        Err(prev)
                    }
                })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                self.compare_exchange(current, new, success, failure)
            }

            #[inline]
            pub(crate) fn fetch_add(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_add(val, order)
            }
            #[inline]
            pub(crate) fn fetch_sub(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_sub(val, order)
            }
            #[inline]
            pub(crate) fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_and(val, order)
            }

            #[inline]
            pub(crate) fn fetch_nand(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(!(prev & val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_or(val, order)
            }
            #[inline]
            pub(crate) fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_xor(val, order)
            }
            #[inline]
            pub(crate) fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_max(val, order)
            }
            #[inline]
            pub(crate) fn fetch_min(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_min(val, order)
            }
            #[inline]
            pub(crate) fn fetch_not(&self, order: Ordering) -> $int_type {
                self.as_native().fetch_not(order)
            }

            #[inline]
            pub(crate) fn fetch_neg(&self, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_neg());
                    prev
                })
            }
            #[inline]
            pub(crate) fn neg(&self, order: Ordering) {
                self.fetch_neg(order);
            }
        }
    };
    // RISC-V 8-bit/16-bit RMW with Zaamo extension
    (cas[sub_word], $atomic_type:ident, $int_type:ident) => {
        #[cfg(any(target_feature = "zabha", portable_atomic_target_feature = "zabha"))]
        atomic_int!(cas, $atomic_type, $int_type);
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        impl $atomic_type {
            #[inline]
            pub(crate) fn swap(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(val);
                    prev
                })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                crate::utils::assert_compare_exchange_ordering(success, failure);
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    if prev == current {
                        self.v.get().write(new);
                        Ok(prev)
                    } else {
                        Err(prev)
                    }
                })
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$int_type, $int_type> {
                self.compare_exchange(current, new, success, failure)
            }

            #[inline]
            pub(crate) fn fetch_add(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_add(val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_sub(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_sub(val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_and(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_and(val, order)
            }

            #[inline]
            pub(crate) fn fetch_nand(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(!(prev & val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_or(val, order)
            }
            #[inline]
            pub(crate) fn fetch_xor(&self, val: $int_type, order: Ordering) -> $int_type {
                self.as_native().fetch_xor(val, order)
            }

            #[inline]
            pub(crate) fn fetch_max(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(core::cmp::max(prev, val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_min(&self, val: $int_type, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(core::cmp::min(prev, val));
                    prev
                })
            }

            #[inline]
            pub(crate) fn fetch_not(&self, order: Ordering) -> $int_type {
                self.as_native().fetch_not(order)
            }

            #[inline]
            pub(crate) fn fetch_neg(&self, _order: Ordering) -> $int_type {
                // SAFETY: any data races are prevented by disabling interrupts (see
                // module-level comments) and the raw pointer is valid because we got it
                // from a reference.
                with(|| unsafe {
                    let prev = self.v.get().read();
                    self.v.get().write(prev.wrapping_neg());
                    prev
                })
            }
            #[inline]
            pub(crate) fn neg(&self, order: Ordering) {
                self.fetch_neg(order);
            }
        }
    };
}

#[cfg(target_pointer_width = "16")]
#[cfg(not(target_arch = "avr"))]
atomic_int!(load_store_atomic, AtomicIsize, isize, 2);
#[cfg(target_pointer_width = "16")]
#[cfg(not(target_arch = "avr"))]
atomic_int!(load_store_atomic, AtomicUsize, usize, 2);
#[cfg(target_arch = "avr")]
atomic_int!(all_critical_session, AtomicIsize, isize, 2);
#[cfg(target_arch = "avr")]
atomic_int!(all_critical_session, AtomicUsize, usize, 2);
#[cfg(target_pointer_width = "32")]
atomic_int!(load_store_atomic, AtomicIsize, isize, 4);
#[cfg(target_pointer_width = "32")]
atomic_int!(load_store_atomic, AtomicUsize, usize, 4);
#[cfg(target_pointer_width = "64")]
atomic_int!(load_store_atomic, AtomicIsize, isize, 8);
#[cfg(target_pointer_width = "64")]
atomic_int!(load_store_atomic, AtomicUsize, usize, 8);
#[cfg(target_pointer_width = "128")]
atomic_int!(load_store_atomic, AtomicIsize, isize, 16);
#[cfg(target_pointer_width = "128")]
atomic_int!(load_store_atomic, AtomicUsize, usize, 16);

#[cfg(not(all(target_arch = "avr", portable_atomic_no_asm)))]
atomic_int!(load_store_atomic[sub_word], AtomicI8, i8, 1);
#[cfg(not(all(target_arch = "avr", portable_atomic_no_asm)))]
atomic_int!(load_store_atomic[sub_word], AtomicU8, u8, 1);
#[cfg(all(target_arch = "avr", portable_atomic_no_asm))]
atomic_int!(all_critical_session, AtomicI8, i8, 1);
#[cfg(all(target_arch = "avr", portable_atomic_no_asm))]
atomic_int!(all_critical_session, AtomicU8, u8, 1);
#[cfg(not(target_arch = "avr"))]
atomic_int!(load_store_atomic[sub_word], AtomicI16, i16, 2);
#[cfg(not(target_arch = "avr"))]
atomic_int!(load_store_atomic[sub_word], AtomicU16, u16, 2);
#[cfg(target_arch = "avr")]
atomic_int!(all_critical_session, AtomicI16, i16, 2);
#[cfg(target_arch = "avr")]
atomic_int!(all_critical_session, AtomicU16, u16, 2);

#[cfg(not(target_pointer_width = "16"))]
atomic_int!(load_store_atomic, AtomicI32, i32, 4);
#[cfg(not(target_pointer_width = "16"))]
atomic_int!(load_store_atomic, AtomicU32, u32, 4);
#[cfg(target_pointer_width = "16")]
#[cfg(any(test, feature = "fallback"))]
atomic_int!(all_critical_session, AtomicI32, i32, 4);
#[cfg(target_pointer_width = "16")]
#[cfg(any(test, feature = "fallback"))]
atomic_int!(all_critical_session, AtomicU32, u32, 4);

cfg_has_fast_atomic_64! {
    atomic_int!(load_store_atomic, AtomicI64, i64, 8);
    atomic_int!(load_store_atomic, AtomicU64, u64, 8);
}
#[cfg(any(test, feature = "fallback"))]
cfg_no_fast_atomic_64! {
    atomic_int!(all_critical_session, AtomicI64, i64, 8);
    atomic_int!(all_critical_session, AtomicU64, u64, 8);
}

#[cfg(any(test, feature = "fallback"))]
atomic_int!(all_critical_session, AtomicI128, i128, 16);
#[cfg(any(test, feature = "fallback"))]
atomic_int!(all_critical_session, AtomicU128, u128, 16);

#[cfg(test)]
mod tests {
    use super::*;

    test_atomic_ptr_single_thread!();
    test_atomic_int_single_thread!(i8);
    test_atomic_int_single_thread!(u8);
    test_atomic_int_single_thread!(i16);
    test_atomic_int_single_thread!(u16);
    test_atomic_int_single_thread!(i32);
    test_atomic_int_single_thread!(u32);
    test_atomic_int_single_thread!(i64);
    test_atomic_int_single_thread!(u64);
    test_atomic_int_single_thread!(i128);
    test_atomic_int_single_thread!(u128);
    test_atomic_int_single_thread!(isize);
    test_atomic_int_single_thread!(usize);
}
