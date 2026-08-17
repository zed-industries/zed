// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic load/store implementation on RISC-V.

This is for RISC-V targets without A extension. (pre-1.76 rustc doesn't provide atomics
at all on such targets. https://github.com/rust-lang/rust/pull/114499)

Also, optionally provides RMW implementation when Zaamo extension or force-amo feature is enabled.

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#risc-v

Refs:
- RISC-V Instruction Set Manual
  "Zaamo" Extension for Atomic Memory Operations
  https://github.com/riscv/riscv-isa-manual/blob/riscv-isa-release-8b9dc50-2024-08-30/src/a-st-ext.adoc#zaamo-extension-for-atomic-memory-operations
  "Zabha" Extension for Byte and Halfword Atomic Memory Operations
  https://github.com/riscv/riscv-isa-manual/blob/riscv-isa-release-8b9dc50-2024-08-30/src/zabha.adoc
- RISC-V Atomics ABI Specification
  https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/draft-20240829-13bfa9f54634cb60d86b9b333e109f077805b4b3/riscv-atomic.adoc
- atomic-maybe-uninit
  https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- riscv64gc https://godbolt.org/z/Ws933n9jE
- riscv64gc (+zabha) https://godbolt.org/z/zEKPPW11f
- riscv32imac https://godbolt.org/z/TKbYdbaE9
- riscv32imac (+zabha) https://godbolt.org/z/TnePfK6co
*/

// TODO: Zacas/Zalrsc extension

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::{cell::UnsafeCell, sync::atomic::Ordering};

#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
#[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
#[cfg(target_arch = "riscv32")]
macro_rules! w {
    () => {
        ""
    };
}
#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
#[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
#[cfg(target_arch = "riscv64")]
macro_rules! w {
    () => {
        "w"
    };
}

#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
macro_rules! atomic_rmw_amo_ext {
    // Use +a also for zaamo because `option arch +zaamo` requires LLVM 19.
    // https://github.com/llvm/llvm-project/commit/8be079cdddfd628d356d9ddb5ab397ea95fb1030
    ("w") => {
        "+a"
    };
    ("d") => {
        "+a"
    };
    ("b") => {
        "+a,+zabha"
    };
    ("h") => {
        "+a,+zabha"
    };
}
#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
macro_rules! atomic_rmw_amo {
    ($op:ident, $dst:ident, $val:ident $(as $cast:ty)?, $order:ident, $size:tt) => {{
        let out $(: $cast)?;
        macro_rules! op {
            ($asm_order:tt) => {
                // SAFETY: The user guaranteed that the AMO instruction is available in this
                // system by setting the portable_atomic_force_amo/target_feature and
                // portable_atomic_unsafe_assume_single_core.
                // The caller of this macro must guarantee the validity of the pointer.
                asm!(
                    ".option push",
                    // https://github.com/riscv-non-isa/riscv-asm-manual/blob/ad0de8c004e29c9a7ac33cfd054f4d4f9392f2fb/src/asm-manual.adoc#arch
                    // LLVM supports `.option arch` directive on LLVM 17+.
                    // https://github.com/llvm/llvm-project/commit/9e8ed3403c191ab9c4903e8eeb8f732ff8a43cb4
                    // Note that `.insn <value>` directive requires LLVM 19.
                    // https://github.com/llvm/llvm-project/commit/2a086dce691e3cc34a2fc27f4fb255bb2cbbfac9
                    concat!(".option arch, ", atomic_rmw_amo_ext!($size)),
                    concat!("amo", stringify!($op), ".", $size, $asm_order, " {out}, {val}, 0({dst})"), // atomic { _x = *dst; *dst = op(_x, val); out = _x }
                    ".option pop",
                    dst = in(reg) ptr_reg!($dst),
                    val = in(reg) $val $(as $cast)?,
                    out = lateout(reg) out,
                    options(nostack, preserves_flags),
                )
            };
        }
        match $order {
            Ordering::Relaxed => op!(""),
            Ordering::Acquire => op!(".aq"),
            Ordering::Release => op!(".rl"),
            // AcqRel and SeqCst RMWs are equivalent.
            Ordering::AcqRel | Ordering::SeqCst => op!(".aqrl"),
            _ => unreachable!(),
        }
        out
    }};
}
// 32-bit val.wrapping_shl(shift) but no extra `& (u32::BITS - 1)`
#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
#[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
#[inline(always)]
fn sllw(val: u32, shift: u32) -> u32 {
    // SAFETY: Calling sll{,w} is safe.
    unsafe {
        let out;
        asm!(
            concat!("sll", w!(), " {out}, {val}, {shift}"), // out = val << shift & 31
            out = lateout(reg) out,
            val = in(reg) val,
            shift = in(reg) shift,
            options(pure, nomem, nostack, preserves_flags),
        );
        out
    }
}
// 32-bit val.wrapping_shr(shift) but no extra `& (u32::BITS - 1)`
#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
#[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
macro_rules! srlw {
    ($val:expr, $shift:expr) => {
        // SAFETY: Calling srl{,w} is safe.
        unsafe {
            let val: u32 = $val;
            let shift: u32 = $shift;
            let out;
            asm!(
                concat!("srl", w!(), " {out}, {val}, {shift}"), // out = val >> shift & 31
                out = lateout(reg) out,
                val = in(reg) val,
                shift = in(reg) shift,
                options(pure, nomem, nostack, preserves_flags),
            );
            out
        }
    };
}

macro_rules! atomic_load_store {
    ($([$($generics:tt)*])? $atomic_type:ident, $value_type:ty $(as $cast:ty)?, $size:tt) => {
        #[repr(transparent)]
        pub(crate) struct $atomic_type $(<$($generics)*>)? {
            v: UnsafeCell<$value_type>,
        }

        // Send is implicitly implemented for atomic integers, but not for atomic pointers.
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl $(<$($generics)*>)? Send for $atomic_type $(<$($generics)*>)? {}
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl $(<$($generics)*>)? Sync for $atomic_type $(<$($generics)*>)? {}

        #[cfg(any(test, not(portable_atomic_unsafe_assume_single_core)))]
        impl $(<$($generics)*>)? $atomic_type $(<$($generics)*>)? {
            #[inline]
            pub(crate) const fn new(v: $value_type) -> Self {
                Self { v: UnsafeCell::new(v) }
            }

            #[inline]
            pub(crate) fn is_lock_free() -> bool {
                Self::IS_ALWAYS_LOCK_FREE
            }
            pub(crate) const IS_ALWAYS_LOCK_FREE: bool = true;

            #[inline]
            pub(crate) const fn as_ptr(&self) -> *mut $value_type {
                self.v.get()
            }
        }
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
                    macro_rules! atomic_load {
                        ($acquire:tt, $release:tt) => {
                            asm!(
                                $release,                                // fence
                                concat!("l", $size, " {out}, 0({src})"), // atomic { out = *src }
                                $acquire,                                // fence
                                src = in(reg) ptr_reg!(src),
                                out = lateout(reg) out,
                                options(nostack, preserves_flags),
                            )
                        };
                    }
                    match order {
                        Ordering::Relaxed => atomic_load!("", ""),
                        Ordering::Acquire => atomic_load!("fence r, rw", ""),
                        Ordering::SeqCst => atomic_load!("fence r, rw", "fence rw, rw"),
                        _ => unreachable!(),
                    }
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
                    macro_rules! atomic_store {
                        ($acquire:tt, $release:tt) => {
                            asm!(
                                $release,                                // fence
                                concat!("s", $size, " {val}, 0({dst})"), // atomic { *dst = val }
                                $acquire,                                // fence
                                dst = in(reg) ptr_reg!(dst),
                                val = in(reg) val $(as $cast)?,
                                options(nostack, preserves_flags),
                            )
                        };
                    }
                    match order {
                        Ordering::Relaxed => atomic_store!("", ""),
                        Ordering::Release => atomic_store!("", "fence rw, w"),
                        // https://github.com/llvm/llvm-project/commit/3ea8f2526541884e03d5bd4f4e46f4eb190990b6
                        Ordering::SeqCst => atomic_store!("fence rw, rw", "fence rw, w"),
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
}

macro_rules! atomic_ptr {
    ($([$($generics:tt)*])? $atomic_type:ident, $value_type:ty $(as $cast:ty)?, $size:tt) => {
        atomic_load_store!($([$($generics)*])? $atomic_type, $value_type $(as $cast)?, $size);
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        impl $(<$($generics)*>)? $atomic_type $(<$($generics)*>)? {
            #[inline]
            pub(crate) fn swap(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    atomic_rmw_amo!(swap, dst, val $(as $cast)?, order, $size)
                        $(as $cast as $value_type)?
                }
            }
        }
    };
}

macro_rules! atomic {
    ($atomic_type:ident, $value_type:ty, $size:tt, $max:tt, $min:tt) => {
        atomic_load_store!($atomic_type, $value_type, $size);
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(portable_atomic_unsafe_assume_single_core, feature = "critical-section")))]
        impl_default_no_fetch_ops!($atomic_type, $value_type);
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(portable_atomic_unsafe_assume_single_core, feature = "critical-section")))]
        impl_default_bit_opts!($atomic_type, $value_type);
        // There is no amo{sub,nand,neg}.
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        impl $atomic_type {
            #[inline]
            pub(crate) fn swap(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(swap, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_add(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(add, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_sub(&self, val: $value_type, order: Ordering) -> $value_type {
                self.fetch_add(val.wrapping_neg(), order)
            }

            #[inline]
            pub(crate) fn fetch_and(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(and, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(or, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_xor(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(xor, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_not(&self, order: Ordering) -> $value_type {
                let dst = self.v.get();
                #[cfg(target_arch = "riscv32")]
                let val: u32 = !0;
                #[cfg(target_arch = "riscv64")]
                let val: u64 = !0;
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!(xor, dst, val, order, $size) }
            }
            #[cfg(not(any(
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
            )))]
            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }

            #[inline]
            pub(crate) fn fetch_max(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!($max, dst, val, order, $size) }
            }

            #[inline]
            pub(crate) fn fetch_min(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe { atomic_rmw_amo!($min, dst, val, order, $size) }
            }
        }
    };
}

#[cfg(any(
    test,
    portable_atomic_force_amo,
    target_feature = "zaamo",
    portable_atomic_target_feature = "zaamo",
))]
#[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
trait ZeroExtend: Copy {
    /// Zero-extends `self` to `u32` if it is smaller than 32-bit.
    fn zero_extend(self) -> u32;
}
macro_rules! zero_extend {
    ($int:ident, $uint:ident) => {
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        impl ZeroExtend for $uint {
            #[inline(always)]
            fn zero_extend(self) -> u32 {
                self as u32
            }
        }
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        impl ZeroExtend for $int {
            #[allow(clippy::cast_sign_loss)]
            #[inline(always)]
            fn zero_extend(self) -> u32 {
                self as $uint as u32
            }
        }
    };
}
zero_extend!(i8, u8);
zero_extend!(i16, u16);

macro_rules! atomic_sub_word {
    ($atomic_type:ident, $value_type:ty, $size:tt, $max:tt, $min:tt) => {
        #[cfg(any(target_feature = "zabha", portable_atomic_target_feature = "zabha"))]
        atomic!($atomic_type, $value_type, $size, $max, $min);
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        atomic_load_store!($atomic_type, $value_type, $size);
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        #[cfg(not(any(portable_atomic_unsafe_assume_single_core, feature = "critical-section")))]
        impl_default_bit_opts!($atomic_type, $value_type);
        #[cfg(any(
            test,
            portable_atomic_force_amo,
            target_feature = "zaamo",
            portable_atomic_target_feature = "zaamo",
        ))]
        #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
        impl $atomic_type {
            #[inline]
            pub(crate) fn fetch_and(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                let (dst, shift, mut mask) = crate::utils::create_sub_word_mask_values(dst);
                mask = !sllw(mask, shift);
                let mut val = sllw(ZeroExtend::zero_extend(val), shift);
                val |= mask;
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                let out: u32 = unsafe { atomic_rmw_amo!(and, dst, val, order, "w") };
                srlw!(out, shift)
            }
            #[cfg(not(any(
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
            )))]
            #[inline]
            pub(crate) fn and(&self, val: $value_type, order: Ordering) {
                self.fetch_and(val, order);
            }

            #[inline]
            pub(crate) fn fetch_or(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                let (dst, shift, _mask) = crate::utils::create_sub_word_mask_values(dst);
                let val = sllw(ZeroExtend::zero_extend(val), shift);
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                let out: u32 = unsafe { atomic_rmw_amo!(or, dst, val, order, "w") };
                srlw!(out, shift)
            }
            #[cfg(not(any(
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
            )))]
            #[inline]
            pub(crate) fn or(&self, val: $value_type, order: Ordering) {
                self.fetch_or(val, order);
            }

            #[inline]
            pub(crate) fn fetch_xor(&self, val: $value_type, order: Ordering) -> $value_type {
                let dst = self.v.get();
                let (dst, shift, _mask) = crate::utils::create_sub_word_mask_values(dst);
                let val = sllw(ZeroExtend::zero_extend(val), shift);
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                let out: u32 = unsafe { atomic_rmw_amo!(xor, dst, val, order, "w") };
                srlw!(out, shift)
            }
            #[cfg(not(any(
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
            )))]
            #[inline]
            pub(crate) fn xor(&self, val: $value_type, order: Ordering) {
                self.fetch_xor(val, order);
            }

            #[inline]
            pub(crate) fn fetch_not(&self, order: Ordering) -> $value_type {
                self.fetch_xor(!0, order)
            }
            #[cfg(not(any(
                portable_atomic_unsafe_assume_single_core,
                feature = "critical-section",
            )))]
            #[inline]
            pub(crate) fn not(&self, order: Ordering) {
                self.fetch_not(order);
            }
        }
    };
}

atomic_sub_word!(AtomicI8, i8, "b", max, min);
atomic_sub_word!(AtomicU8, u8, "b", maxu, minu);
atomic_sub_word!(AtomicI16, i16, "h", max, min);
atomic_sub_word!(AtomicU16, u16, "h", maxu, minu);
atomic!(AtomicI32, i32, "w", max, min);
atomic!(AtomicU32, u32, "w", maxu, minu);
#[cfg(target_arch = "riscv64")]
atomic!(AtomicI64, i64, "d", max, min);
#[cfg(target_arch = "riscv64")]
atomic!(AtomicU64, u64, "d", maxu, minu);
#[cfg(target_pointer_width = "32")]
atomic!(AtomicIsize, isize, "w", max, min);
#[cfg(target_pointer_width = "32")]
atomic!(AtomicUsize, usize, "w", maxu, minu);
#[cfg(target_pointer_width = "32")]
atomic_ptr!([T] AtomicPtr, *mut T as *mut u8, "w");
#[cfg(target_pointer_width = "64")]
atomic!(AtomicIsize, isize, "d", max, min);
#[cfg(target_pointer_width = "64")]
atomic!(AtomicUsize, usize, "d", maxu, minu);
#[cfg(target_pointer_width = "64")]
atomic_ptr!([T] AtomicPtr, *mut T as *mut u8, "d");

#[cfg(test)]
mod tests {
    use super::*;

    test_atomic_ptr_load_store!();
    test_atomic_int_load_store!(i8);
    test_atomic_int_load_store!(u8);
    test_atomic_int_load_store!(i16);
    test_atomic_int_load_store!(u16);
    test_atomic_int_load_store!(i32);
    test_atomic_int_load_store!(u32);
    #[cfg(target_arch = "riscv64")]
    test_atomic_int_load_store!(i64);
    #[cfg(target_arch = "riscv64")]
    test_atomic_int_load_store!(u64);
    test_atomic_int_load_store!(isize);
    test_atomic_int_load_store!(usize);

    macro_rules! test_atomic_ptr_amo {
        () => {
            #[allow(
                clippy::alloc_instead_of_core,
                clippy::std_instead_of_alloc,
                clippy::std_instead_of_core,
                clippy::undocumented_unsafe_blocks
            )]
            mod test_atomic_ptr_amo {
                use super::*;
                test_atomic_ptr_amo!(AtomicPtr<u8>);
            }
        };
        ($atomic_type:ty) => {
            use crate::tests::helper;
            ::quickcheck::quickcheck! {
                fn quickcheck_swap(x: usize, y: usize) -> bool {
                    let x = sptr::invalid_mut(x);
                    let y = sptr::invalid_mut(y);
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.swap(y, order), x);
                        assert_eq!(a.swap(x, order), y);
                    }
                    true
                }
            }
        };
    }
    macro_rules! test_atomic_int_amo {
        ($int_type:ident) => {
            paste::paste! {
                #[allow(
                    clippy::alloc_instead_of_core,
                    clippy::std_instead_of_alloc,
                    clippy::std_instead_of_core,
                    clippy::undocumented_unsafe_blocks
                )]
                mod [<test_atomic_ $int_type _amo>] {
                    use super::*;
                    test_atomic_int_amo!([<Atomic $int_type:camel>], $int_type);
                }
            }
        };
        ($atomic_type:ty, $int_type:ident) => {
            use crate::tests::helper;
            ::quickcheck::quickcheck! {
                fn quickcheck_swap(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.swap(y, order), x);
                        assert_eq!(a.swap(x, order), y);
                    }
                    true
                }
                fn quickcheck_fetch_add(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_add(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), x.wrapping_add(y));
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_add(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), y.wrapping_add(x));
                    }
                    true
                }
                fn quickcheck_fetch_sub(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_sub(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), x.wrapping_sub(y));
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_sub(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), y.wrapping_sub(x));
                    }
                    true
                }
                fn quickcheck_fetch_and(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_and(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), x & y);
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_and(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), y & x);
                    }
                    true
                }
                fn quickcheck_fetch_or(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_or(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), x | y);
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_or(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), y | x);
                    }
                    true
                }
                fn quickcheck_fetch_xor(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_xor(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), x ^ y);
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_xor(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), y ^ x);
                    }
                    true
                }
                fn quickcheck_fetch_max(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_max(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), core::cmp::max(x, y));
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_max(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), core::cmp::max(y, x));
                    }
                    true
                }
                fn quickcheck_fetch_min(x: $int_type, y: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_min(y, order), x);
                        assert_eq!(a.load(Ordering::Relaxed), core::cmp::min(x, y));
                        let a = <$atomic_type>::new(y);
                        assert_eq!(a.fetch_min(x, order), y);
                        assert_eq!(a.load(Ordering::Relaxed), core::cmp::min(y, x));
                    }
                    true
                }
                fn quickcheck_fetch_not(x: $int_type) -> bool {
                    for &order in &helper::SWAP_ORDERINGS {
                        let a = <$atomic_type>::new(x);
                        assert_eq!(a.fetch_not(order), x);
                        assert_eq!(a.load(Ordering::Relaxed), !x);
                        assert_eq!(a.fetch_not(order), !x);
                        assert_eq!(a.load(Ordering::Relaxed), x);
                    }
                    true
                }
            }
        };
    }
    macro_rules! test_atomic_int_amo_sub_word {
        ($int_type:ident) => {
            paste::paste! {
                #[allow(
                    clippy::alloc_instead_of_core,
                    clippy::std_instead_of_alloc,
                    clippy::std_instead_of_core,
                    clippy::undocumented_unsafe_blocks
                )]
                mod [<test_atomic_ $int_type _amo>] {
                    use super::*;
                    #[cfg(any(target_feature = "zabha", portable_atomic_target_feature = "zabha"))]
                    test_atomic_int_amo!([<Atomic $int_type:camel>], $int_type);
                    #[cfg(not(any(target_feature = "zabha", portable_atomic_target_feature = "zabha")))]
                    test_atomic_int_amo_sub_word!([<Atomic $int_type:camel>], $int_type);
                }
            }
        };
        ($atomic_type:ty, $int_type:ident) => {
            use crate::tests::helper::{self, *};
            ::quickcheck::quickcheck! {
                fn quickcheck_fetch_and(x: $int_type, y: $int_type) -> bool {
                    let mut rng = fastrand::Rng::new();
                    for &order in &helper::SWAP_ORDERINGS {
                        for base in [0, !0] {
                            let mut arr = Align16([
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                            ]);
                            let a_idx = rng.usize(3..=6);
                            arr.0[a_idx] = <$atomic_type>::new(x);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_and(y, order), x);
                            assert_eq!(a.load(Ordering::Relaxed), x & y);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            arr.0[a_idx] = <$atomic_type>::new(y);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_and(x, order), y);
                            assert_eq!(a.load(Ordering::Relaxed), y & x);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                        }
                    }
                    true
                }
                fn quickcheck_fetch_or(x: $int_type, y: $int_type) -> bool {
                    let mut rng = fastrand::Rng::new();
                    for &order in &helper::SWAP_ORDERINGS {
                        for base in [0, !0] {
                            let mut arr = Align16([
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                            ]);
                            let a_idx = rng.usize(3..=6);
                            arr.0[a_idx] = <$atomic_type>::new(x);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_or(y, order), x);
                            assert_eq!(a.load(Ordering::Relaxed), x | y);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            arr.0[a_idx] = <$atomic_type>::new(y);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_or(x, order), y);
                            assert_eq!(a.load(Ordering::Relaxed), y | x);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                        }
                    }
                    true
                }
                fn quickcheck_fetch_xor(x: $int_type, y: $int_type) -> bool {
                    let mut rng = fastrand::Rng::new();
                    for &order in &helper::SWAP_ORDERINGS {
                        for base in [0, !0] {
                            let mut arr = Align16([
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                            ]);
                            let a_idx = rng.usize(3..=6);
                            arr.0[a_idx] = <$atomic_type>::new(x);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_xor(y, order), x);
                            assert_eq!(a.load(Ordering::Relaxed), x ^ y);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            arr.0[a_idx] = <$atomic_type>::new(y);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_xor(x, order), y);
                            assert_eq!(a.load(Ordering::Relaxed), y ^ x);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                        }
                    }
                    true
                }
                fn quickcheck_fetch_not(x: $int_type) -> bool {
                    let mut rng = fastrand::Rng::new();
                    for &order in &helper::SWAP_ORDERINGS {
                        for base in [0, !0] {
                            let mut arr = Align16([
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                                <$atomic_type>::new(base),
                            ]);
                            let a_idx = rng.usize(3..=6);
                            arr.0[a_idx] = <$atomic_type>::new(x);
                            let a = &arr.0[a_idx];
                            assert_eq!(a.fetch_not(order), x);
                            assert_eq!(a.load(Ordering::Relaxed), !x);
                            assert_eq!(a.fetch_not(order), !x);
                            assert_eq!(a.load(Ordering::Relaxed), x);
                            for i in 0..a_idx {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                            for i in a_idx + 1..arr.0.len() {
                                assert_eq!(arr.0[i].load(Ordering::Relaxed), base, "invalid value written");
                            }
                        }
                    }
                    true
                }
            }
        };
    }
    test_atomic_ptr_amo!();
    test_atomic_int_amo_sub_word!(i8);
    test_atomic_int_amo_sub_word!(u8);
    test_atomic_int_amo_sub_word!(i16);
    test_atomic_int_amo_sub_word!(u16);
    test_atomic_int_amo!(i32);
    test_atomic_int_amo!(u32);
    #[cfg(target_arch = "riscv64")]
    test_atomic_int_amo!(i64);
    #[cfg(target_arch = "riscv64")]
    test_atomic_int_amo!(u64);
    test_atomic_int_amo!(isize);
    test_atomic_int_amo!(usize);
}
