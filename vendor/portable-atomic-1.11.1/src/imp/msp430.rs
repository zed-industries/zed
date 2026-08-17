// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic implementation on MSP430.

Adapted from https://github.com/pftbest/msp430-atomic.

Operations not supported here are provided by disabling interrupts.
See also src/imp/interrupt/msp430.rs.

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#msp430

Note: Ordering is always SeqCst.

Refs:
- MSP430x5xx and MSP430x6xx Family User's Guide, Rev. Q
  https://www.ti.com/lit/ug/slau208q/slau208q.pdf
- atomic-maybe-uninit
  https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- msp430 https://godbolt.org/z/MGrd4jPoq
*/

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
#[cfg(not(feature = "critical-section"))]
use core::cell::UnsafeCell;
use core::sync::atomic::Ordering;

/// An atomic fence.
///
/// # Panics
///
/// Panics if `order` is [`Relaxed`](Ordering::Relaxed).
#[inline]
#[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
pub fn fence(order: Ordering) {
    match order {
        Ordering::Relaxed => panic!("there is no such thing as a relaxed fence"),
        // MSP430 is single-core and a compiler fence works as an atomic fence.
        _ => compiler_fence(order),
    }
}

/// A compiler memory fence.
///
/// # Panics
///
/// Panics if `order` is [`Relaxed`](Ordering::Relaxed).
#[inline]
#[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
pub fn compiler_fence(order: Ordering) {
    match order {
        Ordering::Relaxed => panic!("there is no such thing as a relaxed compiler fence"),
        _ => {}
    }
    // SAFETY: using an empty asm is safe.
    unsafe {
        // Do not use `nomem` and `readonly` because prevent preceding and subsequent memory accesses from being reordered.
        #[cfg(not(portable_atomic_no_asm))]
        asm!("", options(nostack, preserves_flags));
        #[cfg(portable_atomic_no_asm)]
        llvm_asm!("" ::: "memory" : "volatile");
    }
}

macro_rules! atomic {
    (load_store,
        $([$($generics:tt)*])? $atomic_type:ident, $value_type:ty $(as $cast:ty)?, $size:tt
    ) => {
        #[cfg(not(feature = "critical-section"))]
        #[repr(transparent)]
        pub(crate) struct $atomic_type $(<$($generics)*>)? {
            v: UnsafeCell<$value_type>,
        }

        #[cfg(not(feature = "critical-section"))]
        // Send is implicitly implemented for atomic integers, but not for atomic pointers.
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl $(<$($generics)*>)? Send for $atomic_type $(<$($generics)*>)? {}
        #[cfg(not(feature = "critical-section"))]
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl $(<$($generics)*>)? Sync for $atomic_type $(<$($generics)*>)? {}

        #[cfg(not(feature = "critical-section"))]
        impl $(<$($generics)*>)? $atomic_type $(<$($generics)*>)? {
            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn load(&self, order: Ordering) -> $value_type {
                crate::utils::assert_load_ordering(order);
                let src = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    let out $(: $cast)?;
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("mov.", $size, " @{src}, {out}"), // atomic { out = *src }
                        src = in(reg) src,
                        out = lateout(reg) out,
                        options(nostack, preserves_flags),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("mov.", $size, " $1, $0")
                        : "=r"(out) : "*m"(src) : "memory" : "volatile"
                    );
                    out $(as $cast as $value_type)?
                }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn store(&self, val: $value_type, order: Ordering) {
                crate::utils::assert_store_ordering(order);
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("mov.", $size, " {val}, 0({dst})"), // atomic { *dst = val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        options(nostack, preserves_flags),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("mov.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }
        }
    };
    ($([$($generics:tt)*])? $atomic_type:ident, $value_type:ty $(as $cast:ty)?, $size:tt) => {
        atomic!(load_store, $([$($generics)*])? $atomic_type, $value_type $(as $cast)?, $size);
        #[cfg(not(feature = "critical-section"))]
        impl $(<$($generics)*>)? $atomic_type $(<$($generics)*>)? {
            #[inline]
            pub(crate) fn add(&self, val: $value_type, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("add.", $size, " {val}, 0({dst})"), // atomic { *dst += val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        // Do not use `preserves_flags` because ADD modifies the V, N, Z, and C bits of the status register.
                        options(nostack),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("add.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }

            #[inline]
            pub(crate) fn sub(&self, val: $value_type, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("sub.", $size, " {val}, 0({dst})"), // atomic { *dst -= val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        // Do not use `preserves_flags` because SUB modifies the V, N, Z, and C bits of the status register.
                        options(nostack),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("sub.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }

            #[inline]
            pub(crate) fn and(&self, val: $value_type, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("and.", $size, " {val}, 0({dst})"), // atomic { *dst &= val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        // Do not use `preserves_flags` because AND modifies the V, N, Z, and C bits of the status register.
                        options(nostack),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("and.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }

            #[inline]
            pub(crate) fn or(&self, val: $value_type, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("bis.", $size, " {val}, 0({dst})"), // atomic { *dst |= val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        options(nostack, preserves_flags),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("bis.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }

            #[inline]
            pub(crate) fn xor(&self, val: $value_type, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("xor.", $size, " {val}, 0({dst})"), // atomic { *dst ^= val }
                        dst = in(reg) dst,
                        val = in(reg) val $(as $cast)?,
                        // Do not use `preserves_flags` because XOR modifies the V, N, Z, and C bits of the status register.
                        options(nostack),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("xor.", $size, " $1, $0")
                        :: "*m"(dst), "ir"(val) : "memory" : "volatile"
                    );
                }
            }

            #[inline]
            pub(crate) fn not(&self, _order: Ordering) {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    #[cfg(not(portable_atomic_no_asm))]
                    asm!(
                        concat!("inv.", $size, " 0({dst})"), // atomic { *dst = !*dst }
                        dst = in(reg) dst,
                        // Do not use `preserves_flags` because INV modifies the V, N, Z, and C bits of the status register.
                        options(nostack),
                    );
                    #[cfg(portable_atomic_no_asm)]
                    llvm_asm!(
                        concat!("inv.", $size, " $0")
                        :: "*m"(dst) : "memory" : "volatile"
                    );
                }
            }
        }
    };
}

atomic!(AtomicI8, i8, "b");
atomic!(AtomicU8, u8, "b");
atomic!(AtomicI16, i16, "w");
atomic!(AtomicU16, u16, "w");
atomic!(AtomicIsize, isize, "w");
atomic!(AtomicUsize, usize, "w");
atomic!(load_store, [T] AtomicPtr, *mut T as *mut u8, "w");
