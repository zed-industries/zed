// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
64-bit atomic implementation on riscv32.

This architecture provides the following 64-bit atomic instructions:

- amocas.d: CAS (Zacas extension)

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#risc-v

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use fallback implementation instead.

Refs:
- RISC-V Instruction Set Manual
  "Zacas" Extension for Atomic Compare-and-Swap (CAS) Instructions
  https://github.com/riscv/riscv-isa-manual/blob/riscv-isa-release-8b9dc50-2024-08-30/src/zacas.adoc
- RISC-V Atomics ABI Specification
  https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/draft-20240829-13bfa9f54634cb60d86b9b333e109f077805b4b3/riscv-atomic.adoc

Generated asm:
- riscv32imac (+zacas) https://godbolt.org/z/9bTdfhKre
*/

// TODO: merge duplicated code with atomic128/riscv64.rs

include!("macros.rs");

#[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
#[path = "../fallback/outline_atomics.rs"]
mod fallback;

#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(test, not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"))))]
#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "../detect/riscv_linux.rs"]
mod detect;

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::utils::{Pair, U64};

macro_rules! debug_assert_zacas {
    () => {
        #[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
        {
            debug_assert!(detect::detect().zacas());
        }
    };
}

// `.option arch, +zacas` directive requires LLVM 20, so we use .4byte directive for old LLVM.
// Note that `.insn <value>` directive requires LLVM 19.
// https://github.com/llvm/llvm-project/commit/2a086dce691e3cc34a2fc27f4fb255bb2cbbfac9
// https://github.com/riscv-non-isa/riscv-asm-manual/blob/ad0de8c004e29c9a7ac33cfd054f4d4f9392f2fb/src/asm-manual.adoc#arch
#[cfg(not(portable_atomic_pre_llvm_20))]
macro_rules! start_zacas {
    () => {
        ".option push\n.option arch, +zacas"
    };
}
#[cfg(not(portable_atomic_pre_llvm_20))]
macro_rules! end_zacas {
    () => {
        ".option pop"
    };
}

#[cfg(not(portable_atomic_pre_llvm_20))]
macro_rules! atomic_rmw_amocas_order {
    ($op:ident, $order:ident) => {
        atomic_rmw_amocas_order!($op, $order, failure = $order)
    };
    ($op:ident, $order:ident, failure = $failure:ident) => {
        match $order {
            Ordering::Relaxed => $op!("", ""),
            Ordering::Acquire => $op!("", ".aq"),
            Ordering::Release => $op!("", ".rl"),
            Ordering::AcqRel => $op!("", ".aqrl"),
            Ordering::SeqCst if $failure == Ordering::SeqCst => $op!("fence rw,rw", ".aqrl"),
            Ordering::SeqCst => $op!("", ".aqrl"),
            _ => unreachable!(),
        }
    };
}
#[cfg(portable_atomic_pre_llvm_20)]
macro_rules! atomic_rmw_amocas_order_insn {
    ($op:ident, $order:ident) => {
        atomic_rmw_amocas_order_insn!($op, $order, failure = $order)
    };
    ($op:ident, $order:ident, failure = $failure:ident) => {
        match $order {
            Ordering::Relaxed => $op!("", "8"),
            Ordering::Acquire => $op!("", "c"),
            Ordering::Release => $op!("", "a"),
            Ordering::AcqRel => $op!("", "e"),
            Ordering::SeqCst if $failure == Ordering::SeqCst => $op!("fence rw,rw", "e"),
            Ordering::SeqCst => $op!("", "e"),
            _ => unreachable!(),
        }
    };
}

// If zacas is available at compile-time, we can always use zacas_fn.
#[cfg(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"))]
use self::atomic_load_zacas as atomic_load;
// Otherwise, we need to do run-time detection and can use zacas_fn only if zacas is available.
#[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
#[inline]
unsafe fn atomic_load(src: *mut u64, order: Ordering) -> u64 {
    fn_alias! {
        // inline(never) is just a hint and also not strictly necessary
        // because we use ifunc helper macro, but used for clarity.
        #[inline(never)]
        unsafe fn(src: *mut u64) -> u64;
        atomic_load_zacas_relaxed = atomic_load_zacas(Ordering::Relaxed);
        atomic_load_zacas_acquire = atomic_load_zacas(Ordering::Acquire);
        atomic_load_zacas_seqcst = atomic_load_zacas(Ordering::SeqCst);
    }
    // SAFETY: the caller must uphold the safety contract.
    // we only calls atomic_load_zacas if zacas is available.
    unsafe {
        match order {
            Ordering::Relaxed => {
                ifunc!(unsafe fn(src: *mut u64) -> u64 {
                    if detect::detect().zacas() {
                        atomic_load_zacas_relaxed
                    } else {
                        fallback::atomic_load_non_seqcst
                    }
                })
            }
            Ordering::Acquire => {
                ifunc!(unsafe fn(src: *mut u64) -> u64 {
                    if detect::detect().zacas() {
                        atomic_load_zacas_acquire
                    } else {
                        fallback::atomic_load_non_seqcst
                    }
                })
            }
            Ordering::SeqCst => {
                ifunc!(unsafe fn(src: *mut u64) -> u64 {
                    if detect::detect().zacas() {
                        atomic_load_zacas_seqcst
                    } else {
                        fallback::atomic_load_seqcst
                    }
                })
            }
            _ => unreachable!(),
        }
    }
}
#[inline]
unsafe fn atomic_load_zacas(src: *mut u64, order: Ordering) -> u64 {
    debug_assert!(src as usize % 8 == 0);
    debug_assert_zacas!();
    let (out_lo, out_hi);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        #[cfg(not(portable_atomic_pre_llvm_20))]
        macro_rules! load {
            ($fence:tt, $asm_order:tt) => {
                asm!(
                    start_zacas!(),
                    $fence,                                               // fence
                    concat!("amocas.d", $asm_order, " a2, a2, 0({src})"), // atomic { if *dst == a2:a3 { *dst = a2:a3 } else { a2:a3 = *dst } }
                    end_zacas!(),
                    src = in(reg) ptr_reg!(src),
                    inout("a2") 0_u32 => out_lo,
                    inout("a3") 0_u32 => out_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(not(portable_atomic_pre_llvm_20))]
        atomic_rmw_amocas_order!(load, order);
        #[cfg(portable_atomic_pre_llvm_20)]
        macro_rules! load {
            ($fence:tt, $insn_order:tt) => {
                asm!(
                    $fence,                                  // fence
                    // amocas.d{,.aq,.rl,.aqrl} a2, a2, (a0) // atomic { if *a0 == a2:a3 { *a0 = a2:a3 } else { a2:a3 = *a0 } }
                    concat!(".4byte 0x2", $insn_order, "c5362f"),
                    in("a0") ptr_reg!(src),
                    inout("a2") 0_u32 => out_lo,
                    inout("a3") 0_u32 => out_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(portable_atomic_pre_llvm_20)]
        atomic_rmw_amocas_order_insn!(load, order);
        U64 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}

#[inline]
unsafe fn atomic_store(dst: *mut u64, val: u64, order: Ordering) {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        atomic_swap(dst, val, order);
    }
}

#[inline]
unsafe fn atomic_compare_exchange(
    dst: *mut u64,
    old: u64,
    new: u64,
    success: Ordering,
    failure: Ordering,
) -> Result<u64, u64> {
    #[cfg(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"))]
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantees that zacas instructions are available at compile-time.
    let (prev, ok) = unsafe { atomic_compare_exchange_zacas(dst, old, new, success, failure) };
    #[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
    let (prev, ok) = {
        fn_alias! {
            // inline(never) is just a hint and also not strictly necessary
            // because we use ifunc helper macro, but used for clarity.
            #[inline(never)]
            unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool);
            zacas_relaxed_fn = atomic_compare_exchange_zacas(Ordering::Relaxed, Ordering::Relaxed);
            zacas_acquire_fn = atomic_compare_exchange_zacas(Ordering::Acquire, Ordering::Acquire);
            zacas_release_fn = atomic_compare_exchange_zacas(Ordering::Release, Ordering::Relaxed);
            zacas_acqrel_fn = atomic_compare_exchange_zacas(Ordering::AcqRel, Ordering::Acquire);
            zacas_seqcst_fn = atomic_compare_exchange_zacas(Ordering::SeqCst, Ordering::SeqCst);
        }
        let order = crate::utils::upgrade_success_ordering(success, failure);
        // SAFETY: the caller must uphold the safety contract.
        // we only calls atomic_compare_exchange_zacas if zacas is available.
        unsafe {
            match order {
                Ordering::Relaxed => {
                    ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
                        if detect::detect().zacas() {
                            zacas_relaxed_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::Acquire => {
                    ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
                        if detect::detect().zacas() {
                            zacas_acquire_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::Release => {
                    ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
                        if detect::detect().zacas() {
                            zacas_release_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::AcqRel => {
                    ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
                        if detect::detect().zacas() {
                            zacas_acqrel_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::SeqCst => {
                    ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
                        if detect::detect().zacas() {
                            zacas_seqcst_fn
                        } else {
                            fallback::atomic_compare_exchange_seqcst
                        }
                    })
                }
                _ => unreachable!(),
            }
        }
    };
    if ok { Ok(prev) } else { Err(prev) }
}
#[inline]
unsafe fn atomic_compare_exchange_zacas(
    dst: *mut u64,
    old: u64,
    new: u64,
    success: Ordering,
    failure: Ordering,
) -> (u64, bool) {
    debug_assert!(dst as usize % 8 == 0);
    debug_assert_zacas!();
    let order = crate::utils::upgrade_success_ordering(success, failure);
    let old = U64 { whole: old };
    let new = U64 { whole: new };
    let (prev_lo, prev_hi);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        #[cfg(not(portable_atomic_pre_llvm_20))]
        macro_rules! cmpxchg {
            ($fence:tt, $asm_order:tt) => {
                asm!(
                    start_zacas!(),
                    $fence,                                               // fence
                    concat!("amocas.d", $asm_order, " a4, a2, 0({dst})"), // atomic { if *dst == a4:a5 { *dst = a2:a3 } else { a4:a5 = *dst } }
                    end_zacas!(),
                    dst = in(reg) ptr_reg!(dst),
                    // must be allocated to even/odd register pair
                    inout("a4") old.pair.lo => prev_lo,
                    inout("a5") old.pair.hi => prev_hi,
                    // must be allocated to even/odd register pair
                    in("a2") new.pair.lo,
                    in("a3") new.pair.hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(not(portable_atomic_pre_llvm_20))]
        atomic_rmw_amocas_order!(cmpxchg, order, failure = failure);
        #[cfg(portable_atomic_pre_llvm_20)]
        macro_rules! cmpxchg {
            ($fence:tt, $insn_order:tt) => {
                asm!(
                    $fence,                                  // fence
                    // amocas.d{,.aq,.rl,.aqrl} a4, a2, (a0) // atomic { if *a0 == a4:a5 { *a0 = a2:a3 } else { a4:a5 = *a0 } }
                    concat!(".4byte 0x2", $insn_order, "c5372f"),
                    in("a0") ptr_reg!(dst),
                    // must be allocated to even/odd register pair
                    inout("a4") old.pair.lo => prev_lo,
                    inout("a5") old.pair.hi => prev_hi,
                    // must be allocated to even/odd register pair
                    in("a2") new.pair.lo,
                    in("a3") new.pair.hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(portable_atomic_pre_llvm_20)]
        atomic_rmw_amocas_order_insn!(cmpxchg, order, failure = failure);
        let prev = U64 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole;
        (prev, prev == old.whole)
    }
}

// amocas is always strong.
use self::atomic_compare_exchange as atomic_compare_exchange_weak;

// 64-bit atomic load by two 32-bit atomic loads.
#[inline]
unsafe fn byte_wise_atomic_load(src: *const u64) -> u64 {
    let (out_lo, out_hi);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        asm!(
            "lw {out_lo}, ({src})",  // atomic { out_lo = *src }
            "lw {out_hi}, 4({src})", // atomic { out_hi = *src.byte_add(4) }
            src = in(reg) ptr_reg!(src),
            out_lo = out(reg) out_lo,
            out_hi = out(reg) out_hi,
            options(pure, nostack, preserves_flags, readonly),
        );
        U64 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}

macro_rules! select_atomic_rmw {
    (
        unsafe fn $name:ident(dst: *mut u64 $(, $($arg:tt)*)?) $(-> $ret_ty:ty)? {
            |$zacas_fn_binding:ident| $($zacas_fn_body:tt)*
        }
        zacas = $zacas_fn:ident;
        non_seqcst_fallback = $non_seqcst_fallback_fn:ident;
        seqcst_fallback = $seqcst_fallback_fn:ident;
    ) => {
        #[inline]
        unsafe fn $zacas_fn(dst: *mut u64 $(, $($arg)*)?, order: Ordering) $(-> $ret_ty)? {
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                // This is not single-copy atomic reads, but this is ok because subsequent
                // CAS will check for consistency.
                //
                // Note that the C++20 memory model does not allow mixed-sized atomic access,
                // so we must use inline assembly to implement byte_wise_atomic_load.
                // (i.e., byte-wise atomic based on the standard library's atomic types
                // cannot be used here).
                let mut prev = byte_wise_atomic_load(dst);
                loop {
                    let next = {
                        let $zacas_fn_binding = prev;
                        $($zacas_fn_body)*
                    };
                    match atomic_compare_exchange_weak(dst, prev, next, order, Ordering::Relaxed) {
                        Ok(x) => return x,
                        Err(x) => prev = x,
                    }
                }
            }
        }
        // If zacas is available at compile-time, we can always use zacas_fn.
        #[cfg(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"))]
        use self::$zacas_fn as $name;
        // Otherwise, we need to do run-time detection and can use zacas_fn only if zacas is available.
        #[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
        #[inline]
        unsafe fn $name(dst: *mut u64 $(, $($arg)*)?, order: Ordering) $(-> $ret_ty)? {
            fn_alias! {
                // inline(never) is just a hint and also not strictly necessary
                // because we use ifunc helper macro, but used for clarity.
                #[inline(never)]
                unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)?;
                zacas_relaxed_fn = $zacas_fn(Ordering::Relaxed);
                zacas_acquire_fn = $zacas_fn(Ordering::Acquire);
                zacas_release_fn = $zacas_fn(Ordering::Release);
                zacas_acqrel_fn = $zacas_fn(Ordering::AcqRel);
                zacas_seqcst_fn = $zacas_fn(Ordering::SeqCst);
            }
            // SAFETY: the caller must uphold the safety contract.
            // we only calls zacas_fn if zacas is available.
            unsafe {
                match order {
                    Ordering::Relaxed => {
                        ifunc!(unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                            if detect::detect().zacas() {
                                zacas_relaxed_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::Acquire => {
                        ifunc!(unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                            if detect::detect().zacas() {
                                zacas_acquire_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::Release => {
                        ifunc!(unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                            if detect::detect().zacas() {
                                zacas_release_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::AcqRel => {
                        ifunc!(unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                            if detect::detect().zacas() {
                                zacas_acqrel_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::SeqCst => {
                        ifunc!(unsafe fn(dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                            if detect::detect().zacas() {
                                zacas_seqcst_fn
                            } else {
                                fallback::$seqcst_fallback_fn
                            }
                        })
                    }
                    _ => unreachable!(),
                }
            }
        }
    };
}

select_atomic_rmw! {
    unsafe fn atomic_swap(dst: *mut u64, val: u64) -> u64 {
        |_x| val
    }
    zacas = atomic_swap_zacas;
    non_seqcst_fallback = atomic_swap_non_seqcst;
    seqcst_fallback = atomic_swap_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_add(dst: *mut u64, val: u64) -> u64 {
        |x| x.wrapping_add(val)
    }
    zacas = atomic_add_zacas;
    non_seqcst_fallback = atomic_add_non_seqcst;
    seqcst_fallback = atomic_add_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_sub(dst: *mut u64, val: u64) -> u64 {
        |x| x.wrapping_sub(val)
    }
    zacas = atomic_sub_zacas;
    non_seqcst_fallback = atomic_sub_non_seqcst;
    seqcst_fallback = atomic_sub_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_and(dst: *mut u64, val: u64) -> u64 {
        |x| x & val
    }
    zacas = atomic_and_zacas;
    non_seqcst_fallback = atomic_and_non_seqcst;
    seqcst_fallback = atomic_and_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_nand(dst: *mut u64, val: u64) -> u64 {
        |x| !(x & val)
    }
    zacas = atomic_nand_zacas;
    non_seqcst_fallback = atomic_nand_non_seqcst;
    seqcst_fallback = atomic_nand_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_or(dst: *mut u64, val: u64) -> u64 {
        |x| x | val
    }
    zacas = atomic_or_zacas;
    non_seqcst_fallback = atomic_or_non_seqcst;
    seqcst_fallback = atomic_or_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_xor(dst: *mut u64, val: u64) -> u64 {
        |x| x ^ val
    }
    zacas = atomic_xor_zacas;
    non_seqcst_fallback = atomic_xor_non_seqcst;
    seqcst_fallback = atomic_xor_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_max(dst: *mut u64, val: u64) -> u64 {
        |x| {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            { core::cmp::max(x as i64, val as i64) as u64 }
        }
    }
    zacas = atomic_max_zacas;
    non_seqcst_fallback = atomic_max_non_seqcst;
    seqcst_fallback = atomic_max_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umax(dst: *mut u64, val: u64) -> u64 {
        |x| core::cmp::max(x, val)
    }
    zacas = atomic_umax_zacas;
    non_seqcst_fallback = atomic_umax_non_seqcst;
    seqcst_fallback = atomic_umax_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_min(dst: *mut u64, val: u64) -> u64 {
        |x| {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            { core::cmp::min(x as i64, val as i64) as u64 }
        }
    }
    zacas = atomic_min_zacas;
    non_seqcst_fallback = atomic_min_non_seqcst;
    seqcst_fallback = atomic_min_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umin(dst: *mut u64, val: u64) -> u64 {
        |x| core::cmp::min(x, val)
    }
    zacas = atomic_umin_zacas;
    non_seqcst_fallback = atomic_umin_non_seqcst;
    seqcst_fallback = atomic_umin_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_not(dst: *mut u64) -> u64 {
        |x| !x
    }
    zacas = atomic_not_zacas;
    non_seqcst_fallback = atomic_not_non_seqcst;
    seqcst_fallback = atomic_not_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_neg(dst: *mut u64) -> u64 {
        |x| x.wrapping_neg()
    }
    zacas = atomic_neg_zacas;
    non_seqcst_fallback = atomic_neg_non_seqcst;
    seqcst_fallback = atomic_neg_seqcst;
}

#[inline]
fn is_lock_free() -> bool {
    #[cfg(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"))]
    {
        // zacas is available at compile-time.
        true
    }
    #[cfg(not(any(target_feature = "zacas", portable_atomic_target_feature = "zacas")))]
    {
        detect::detect().zacas()
    }
}
const IS_ALWAYS_LOCK_FREE: bool =
    cfg!(any(target_feature = "zacas", portable_atomic_target_feature = "zacas"));

atomic64!(AtomicI64, i64, atomic_max, atomic_min);
atomic64!(AtomicU64, u64, atomic_umax, atomic_umin);

#[allow(clippy::undocumented_unsafe_blocks, clippy::wildcard_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    test_atomic_int!(i64);
    test_atomic_int!(u64);

    // load/store/swap implementation is not affected by signedness, so it is
    // enough to test only unsigned types.
    stress_test!(u64);
}
