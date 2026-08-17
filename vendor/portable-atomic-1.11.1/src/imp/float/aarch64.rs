// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic float implementation based on AArch64 with FEAT_LSFE.

This module provides atomic float implementations using FEAT_LSFE instructions.

Generated asm:
- aarch64 (+lsfe) https://godbolt.org/z/7vaxeofv1
*/

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::sync::atomic::Ordering;

#[cfg(portable_atomic_unstable_f16)]
use super::int::AtomicF16;
#[cfg(portable_atomic_unstable_f128)]
use super::int::AtomicF128;
use super::int::{AtomicF32, AtomicF64};

// TODO: optimize no return cases:
// https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/STFADD--STFADDL--Floating-point-atomic-add-in-memory--without-return-
// https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/STFMAXNM--STFMAXNML--Floating-point-atomic-maximum-number-in-memory--without-return-
// https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/STFMINNM--STFMINNML--Floating-point-atomic-minimum-number-in-memory--without-return-

#[cfg(not(portable_atomic_pre_llvm_20))]
macro_rules! start_lsfe {
    () => {
        ".arch_extension lsfe"
    };
}

#[cfg(not(portable_atomic_pre_llvm_20))]
macro_rules! atomic_rmw {
    ($op:ident, $order:ident) => {
        atomic_rmw!($op, $order, write = $order)
    };
    ($op:ident, $order:ident, write = $write:ident) => {
        match $order {
            Ordering::Relaxed => $op!("", "", ""),
            Ordering::Acquire => $op!("a", "", ""),
            Ordering::Release => $op!("", "l", ""),
            Ordering::AcqRel => $op!("a", "l", ""),
            // In MSVC environments, SeqCst stores/writes needs fences after writes.
            // https://reviews.llvm.org/D141748
            #[cfg(target_env = "msvc")]
            Ordering::SeqCst if $write == Ordering::SeqCst => $op!("a", "l", "dmb ish"),
            // AcqRel and SeqCst RMWs are equivalent in non-MSVC environments.
            Ordering::SeqCst => $op!("a", "l", ""),
            _ => unreachable!(),
        }
    };
}
#[cfg(portable_atomic_pre_llvm_20)]
macro_rules! atomic_rmw_inst {
    ($op:ident, $order:ident) => {
        atomic_rmw_inst!($op, $order, write = $order)
    };
    ($op:ident, $order:ident, write = $write:ident) => {
        match $order {
            Ordering::Relaxed => $op!("2", ""), // ""
            Ordering::Acquire => $op!("a", ""), // "a"
            Ordering::Release => $op!("6", ""), // "l"
            Ordering::AcqRel => $op!("e", ""),  // "al"
            // In MSVC environments, SeqCst stores/writes needs fences after writes.
            // https://reviews.llvm.org/D141748
            #[cfg(target_env = "msvc")]
            Ordering::SeqCst if $write == Ordering::SeqCst => $op!("e", "dmb ish"),
            // AcqRel and SeqCst RMWs are equivalent in non-MSVC environments.
            Ordering::SeqCst => $op!("e", ""),
            _ => unreachable!(),
        }
    };
}

macro_rules! atomic_float {
    ($atomic_type:ident, $float_type:ident, $modifier:tt, $inst_modifier:tt) => {
        impl $atomic_type {
            #[inline]
            pub(crate) fn fetch_add(&self, val: $float_type, order: Ordering) -> $float_type {
                let dst = self.as_ptr();
                let out;
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                //
                // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/LDFADD--LDFADDA--LDFADDAL--LDFADDL--Floating-point-atomic-add-in-memory-
                unsafe {
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    macro_rules! add {
                        ($acquire:tt, $release:tt, $fence:tt) => {
                            asm!(
                                start_lsfe!(),
                                concat!("ldfadd", $acquire, $release, " {out:", $modifier, "}, {val:", $modifier, "}, [{dst}]"),
                                $fence,
                                dst = in(reg) ptr_reg!(dst),
                                val = in(vreg) val,
                                out = lateout(vreg) out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    atomic_rmw!(add, order);
                    // LLVM supports FEAT_LSFE instructions on LLVM 20+, so use .inst directive on old LLVM.
                    // https://github.com/llvm/llvm-project/commit/67ff5ba9af9754261abe11d762af11532a816126
                    #[cfg(portable_atomic_pre_llvm_20)]
                    macro_rules! add {
                        ($order:tt, $fence:tt) => {
                            asm!(
                                // ldfadd{,a,l,al} {h,s,d}0, {h,s,d}1, [x2]
                                concat!(".inst 0x", $inst_modifier, "c", $order, "00041"),
                                $fence,
                                in("x2") ptr_reg!(dst),
                                in("v1") val,
                                out("v0") out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(portable_atomic_pre_llvm_20)]
                    atomic_rmw_inst!(add, order);
                }
                out
            }
            #[inline]
            pub(crate) fn fetch_sub(&self, val: $float_type, order: Ordering) -> $float_type {
                // There is no atomic sub instruction, so add `-val`.
                self.fetch_add(-val, order)
            }
            #[inline]
            pub(crate) fn fetch_max(&self, val: $float_type, order: Ordering) -> $float_type {
                let dst = self.as_ptr();
                let out;
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                //
                // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/LDFMAXNM--LDFMAXNMA--LDFMAXNMAL--LDFMAXNML--Floating-point-atomic-maximum-number-in-memory-
                unsafe {
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    macro_rules! max {
                        ($acquire:tt, $release:tt, $fence:tt) => {
                            asm!(
                                start_lsfe!(),
                                concat!("ldfmaxnm", $acquire, $release, " {out:", $modifier, "}, {val:", $modifier, "}, [{dst}]"),
                                $fence,
                                dst = in(reg) ptr_reg!(dst),
                                val = in(vreg) val,
                                out = lateout(vreg) out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    atomic_rmw!(max, order);
                    // LLVM supports FEAT_LSFE instructions on LLVM 20+, so use .inst directive on old LLVM.
                    // https://github.com/llvm/llvm-project/commit/67ff5ba9af9754261abe11d762af11532a816126
                    #[cfg(portable_atomic_pre_llvm_20)]
                    macro_rules! max {
                        ($order:tt, $fence:tt) => {
                            asm!(
                                // ldfmaxnm{,a,l,al} {h,s,d}0, {h,s,d}1, [x2]
                                concat!(".inst 0x", $inst_modifier, "c", $order, "06041"),
                                $fence,
                                in("x2") ptr_reg!(dst),
                                in("v1") val,
                                out("v0") out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(portable_atomic_pre_llvm_20)]
                    atomic_rmw_inst!(max, order);
                }
                out
            }
            #[inline]
            pub(crate) fn fetch_min(&self, val: $float_type, order: Ordering) -> $float_type {
                let dst = self.as_ptr();
                let out;
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                //
                // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/SIMD-FP-Instructions/LDFMINNM--LDFMINNMA--LDFMINNMAL--LDFMINNML--Floating-point-atomic-minimum-number-in-memory-
                unsafe {
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    macro_rules! min {
                        ($acquire:tt, $release:tt, $fence:tt) => {
                            asm!(
                                start_lsfe!(),
                                concat!("ldfminnm", $acquire, $release, " {out:", $modifier, "}, {val:", $modifier, "}, [{dst}]"),
                                $fence,
                                dst = in(reg) ptr_reg!(dst),
                                val = in(vreg) val,
                                out = lateout(vreg) out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(not(portable_atomic_pre_llvm_20))]
                    atomic_rmw!(min, order);
                    // LLVM supports FEAT_LSFE instructions on LLVM 20+, so use .inst directive on old LLVM.
                    // https://github.com/llvm/llvm-project/commit/67ff5ba9af9754261abe11d762af11532a816126
                    #[cfg(portable_atomic_pre_llvm_20)]
                    macro_rules! min {
                        ($order:tt, $fence:tt) => {
                            asm!(
                                // ldfminnm{,a,l,al} {h,s,d}0, {h,s,d}1, [x2]
                                concat!(".inst 0x", $inst_modifier, "c", $order, "07041"),
                                $fence,
                                in("x2") ptr_reg!(dst),
                                in("v1") val,
                                out("v0") out,
                                options(nostack),
                            )
                        };
                    }
                    #[cfg(portable_atomic_pre_llvm_20)]
                    atomic_rmw_inst!(min, order);
                }
                out
            }
        }
    };
}

#[cfg(portable_atomic_unstable_f16)]
atomic_float!(AtomicF16, f16, "h", "7");
atomic_float!(AtomicF32, f32, "s", "b");
atomic_float!(AtomicF64, f64, "d", "f");

#[cfg(portable_atomic_unstable_f128)]
impl AtomicF128 {
    #[inline]
    pub(crate) fn fetch_add(&self, val: f128, order: Ordering) -> f128 {
        self.fetch_update_(order, |x| x + val)
    }
    #[inline]
    pub(crate) fn fetch_sub(&self, val: f128, order: Ordering) -> f128 {
        self.fetch_update_(order, |x| x - val)
    }
    #[inline]
    pub(super) fn fetch_update_<F>(&self, order: Ordering, mut f: F) -> f128
    where
        F: FnMut(f128) -> f128,
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
    pub(crate) fn fetch_max(&self, val: f128, order: Ordering) -> f128 {
        self.fetch_update_(order, |x| x.max(val))
    }
    #[inline]
    pub(crate) fn fetch_min(&self, val: f128, order: Ordering) -> f128 {
        self.fetch_update_(order, |x| x.min(val))
    }
}
