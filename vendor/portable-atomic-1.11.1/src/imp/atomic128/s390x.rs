// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementation on s390x.

This architecture provides the following 128-bit atomic instructions:

- LPQ/STPQ: load/store (arch1 or later, i.e., baseline)
- CDSG: CAS (arch1 or later, i.e., baseline)

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#s390x

LLVM's minimal supported architecture level is arch8 (z10):
https://github.com/llvm/llvm-project/blob/llvmorg-20.1.0/llvm/lib/Target/SystemZ/SystemZProcessors.td#L16-L17
This does not appear to have changed since the current s390x backend was added in LLVM 3.3:
https://github.com/llvm/llvm-project/commit/5f613dfd1f7edb0ae95d521b7107b582d9df5103#diff-cbaef692b3958312e80fd5507a7e2aff071f1acb086f10e8a96bc06a7bb289db

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use intrinsics.rs instead.

Refs:
- z/Architecture Principles of Operation, Fourteenth Edition (SA22-7832-13)
  https://publibfp.dhe.ibm.com/epubs/pdf/a227832d.pdf
- atomic-maybe-uninit
  https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- s390x https://godbolt.org/z/oPxYYEvPG
- s390x (z196) https://godbolt.org/z/M69KrKT7Y
- s390x (z15,-vector) https://godbolt.org/z/Wec8b3ada
- s390x (z15) https://godbolt.org/z/KxWcrbfYh
*/

include!("macros.rs");

use core::{arch::asm, sync::atomic::Ordering};

use crate::utils::{Pair, U128};

// bcr 14,0 requires fast-BCR-serialization facility added in arch9 (z196).
#[cfg(any(
    target_feature = "fast-serialization",
    portable_atomic_target_feature = "fast-serialization",
))]
macro_rules! serialization {
    () => {
        "bcr 14, 0"
    };
}
#[cfg(not(any(
    target_feature = "fast-serialization",
    portable_atomic_target_feature = "fast-serialization",
)))]
macro_rules! serialization {
    () => {
        "bcr 15, 0"
    };
}

// Use distinct operands on z196 or later, otherwise split to lgr and $op.
#[cfg(any(target_feature = "distinct-ops", portable_atomic_target_feature = "distinct-ops"))]
macro_rules! distinct_op {
    ($op:tt, $a0:tt, $a1:tt, $a2:tt) => {
        concat!($op, "k ", $a0, ", ", $a1, ", ", $a2)
    };
}
#[cfg(not(any(target_feature = "distinct-ops", portable_atomic_target_feature = "distinct-ops")))]
macro_rules! distinct_op {
    ($op:tt, $a0:tt, $a1:tt, $a2:tt) => {
        concat!("lgr ", $a0, ", ", $a1, "\n", $op, " ", $a0, ", ", $a2)
    };
}

// Use selgr$cond on z15 or later, otherwise split to locgr$cond and $op.
#[cfg(any(
    target_feature = "miscellaneous-extensions-3",
    portable_atomic_target_feature = "miscellaneous-extensions-3",
))]
#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
macro_rules! select_op {
    ($cond:tt, $a0:tt, $a1:tt, $a2:tt) => {
        concat!("selgr", $cond, " ", $a0, ", ", $a1, ", ", $a2)
    };
}
#[cfg(not(any(
    target_feature = "miscellaneous-extensions-3",
    portable_atomic_target_feature = "miscellaneous-extensions-3",
)))]
#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
macro_rules! select_op {
    ($cond:tt, $a0:tt, $a1:tt, $a2:tt) => {
        concat!("lgr ", $a0, ", ", $a2, "\n", "locgr", $cond, " ", $a0, ", ", $a1)
    };
}

// Extracts and checks condition code.
#[inline]
fn extract_cc(r: i64) -> bool {
    r.wrapping_add(-268435456) & (1 << 31) != 0
}

#[inline]
unsafe fn atomic_load(src: *mut u128, _order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    let (out_hi, out_lo);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        // atomic load is always SeqCst.
        asm!(
            "lpq %r0, 0({src})", // atomic { r0:r1 = *src }
            src = in(reg) ptr_reg!(src),
            // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
            out("r0") out_hi,
            out("r1") out_lo,
            options(nostack, preserves_flags),
        );
        U128 { pair: Pair { hi: out_hi, lo: out_lo } }.whole
    }
}

#[inline]
unsafe fn atomic_store(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);
    let val = U128 { whole: val };

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        macro_rules! atomic_store {
            ($acquire:expr) => {
                asm!(
                    "stpq %r0, 0({dst})", // atomic { *dst = r0:r1 }
                    $acquire,             // fence
                    dst = in(reg) ptr_reg!(dst),
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    in("r0") val.pair.hi,
                    in("r1") val.pair.lo,
                    options(nostack, preserves_flags),
                )
            };
        }
        match order {
            // Relaxed and Release stores are equivalent.
            Ordering::Relaxed | Ordering::Release => atomic_store!(""),
            Ordering::SeqCst => atomic_store!(serialization!()),
            _ => unreachable!(),
        }
    }
}

#[inline]
unsafe fn atomic_compare_exchange(
    dst: *mut u128,
    old: u128,
    new: u128,
    _success: Ordering,
    _failure: Ordering,
) -> Result<u128, u128> {
    debug_assert!(dst as usize % 16 == 0);
    let old = U128 { whole: old };
    let new = U128 { whole: new };
    let (prev_hi, prev_lo);
    let r;

    // SAFETY: the caller must uphold the safety contract.
    let prev = unsafe {
        // atomic CAS is always SeqCst.
        asm!(
            "cdsg %r0, %r12, 0({dst})", // atomic { if *dst == r0:r1 { cc = 0; *dst = r12:13 } else { cc = 1; r0:r1 = *dst } }
            "ipm {r}",                  // r[:] = cc
            dst = in(reg) ptr_reg!(dst),
            r = lateout(reg) r,
            // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
            inout("r0") old.pair.hi => prev_hi,
            inout("r1") old.pair.lo => prev_lo,
            in("r12") new.pair.hi,
            in("r13") new.pair.lo,
            // Do not use `preserves_flags` because CDSG modifies the condition code.
            options(nostack),
        );
        U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
    };
    if extract_cc(r) { Ok(prev) } else { Err(prev) }
}

// cdsg is always strong.
use self::atomic_compare_exchange as atomic_compare_exchange_weak;

// 128-bit atomic load by two 64-bit atomic loads.
#[cfg(not(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
)))]
#[inline]
unsafe fn byte_wise_atomic_load(src: *const u128) -> u128 {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let (out_hi, out_lo);
        asm!(
            "lg {out_hi}, 8({src})", // atomic { out_hi = *src.byte_add(8) }
            "lg {out_lo}, 0({src})", // atomic { out_lo = *src }
            src = in(reg) src,
            out_hi = out(reg) out_hi,
            out_lo = out(reg) out_lo,
            options(pure, nostack, preserves_flags, readonly),
        );
        U128 { pair: Pair { hi: out_hi, lo: out_lo } }.whole
    }
}

#[cfg(not(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
)))]
#[inline(always)]
unsafe fn atomic_update<F>(dst: *mut u128, order: Ordering, mut f: F) -> u128
where
    F: FnMut(u128) -> u128,
{
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
            let next = f(prev);
            match atomic_compare_exchange_weak(dst, prev, next, order, Ordering::Relaxed) {
                Ok(x) => return x,
                Err(x) => prev = x,
            }
        }
    }
}

#[inline]
unsafe fn atomic_swap(dst: *mut u128, val: u128, _order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    let val = U128 { whole: val };
    let (mut prev_hi, mut prev_lo);

    // SAFETY: the caller must uphold the safety contract.
    //
    // We could use atomic_update here, but using an inline assembly allows omitting
    // the comparison of results and the storing/comparing of condition flags.
    //
    // Do not use atomic_rmw_cas_3 because it needs extra LGR to implement swap.
    unsafe {
        // atomic swap is always SeqCst.
        asm!(
            "lg %r0, 8({dst})",             // atomic { r0 = *dst.byte_add(8) }
            "lg %r1, 0({dst})",             // atomic { r1 = *dst }
            "2:", // 'retry:
                "cdsg %r0, %r12, 0({dst})", // atomic { if *dst == r0:r1 { cc = 0; *dst = r12:r13 } else { cc = 1; r0:r1 = *dst } }
                "jl 2b",                    // if cc == 1 { jump 'retry }
            dst = in(reg) ptr_reg!(dst),
            // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
            out("r0") prev_hi,
            out("r1") prev_lo,
            in("r12") val.pair.hi,
            in("r13") val.pair.lo,
            // Do not use `preserves_flags` because CDSG modifies the condition code.
            options(nostack),
        );
        U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
    }
}

/// Atomic RMW by CAS loop (3 arguments)
/// `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - val_hi/val_lo pair: val argument (read-only for `$op`)
/// - r0/r1 pair: previous value loaded (read-only for `$op`)
/// - r12/r13 pair: new value that will be stored
// We could use atomic_update here, but using an inline assembly allows omitting
// the comparison of results and the storing/comparing of condition flags.
macro_rules! atomic_rmw_cas_3 {
    ($name:ident, [$($reg:tt)*], $($op:tt)*) => {
        #[inline]
        unsafe fn $name(dst: *mut u128, val: u128, _order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            let val = U128 { whole: val };
            let (mut prev_hi, mut prev_lo);

            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                // atomic RMW is always SeqCst.
                asm!(
                    "lg %r0, 8({dst})",             // atomic { r0 = *dst.byte_add(8) }
                    "lg %r1, 0({dst})",             // atomic { r1 = *dst }
                    "2:", // 'retry:
                        $($op)*
                        "cdsg %r0, %r12, 0({dst})", // atomic { if *dst == r0:r1 { cc = 0; *dst = r12:r13 } else { cc = 1; r0:r1 = *dst } }
                        "jl 2b",                    // if cc == 1 { jump 'retry }
                    dst = in(reg) ptr_reg!(dst),
                    val_hi = in(reg) val.pair.hi,
                    val_lo = in(reg) val.pair.lo,
                    $($reg)*
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    out("r0") prev_hi,
                    out("r1") prev_lo,
                    out("r12") _,
                    out("r13") _,
                    // Do not use `preserves_flags` because CDSG modifies the condition code.
                    options(nostack),
                );
                U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
            }
        }
    };
}
/// Atomic RMW by CAS loop (2 arguments)
/// `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - r0/r1 pair: previous value loaded (read-only for `$op`)
/// - r12/r13 pair: new value that will be stored
// We could use atomic_update here, but using an inline assembly allows omitting
// the comparison of results and the storing/comparing of condition flags.
macro_rules! atomic_rmw_cas_2 {
    ($name:ident, [$($reg:tt)*], $($op:tt)*) => {
        #[inline]
        unsafe fn $name(dst: *mut u128, _order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            let (mut prev_hi, mut prev_lo);

            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                // atomic RMW is always SeqCst.
                asm!(
                    "lg %r0, 8({dst})",             // atomic { r0 = *dst.byte_add(8) }
                    "lg %r1, 0({dst})",             // atomic { r1 = *dst }
                    "2:", // 'retry:
                        $($op)*
                        "cdsg %r0, %r12, 0({dst})", // atomic { if *dst == r0:r1 { cc = 0; *dst = r12:r13 } else { cc = 1; r0:r1 = *dst } }
                        "jl 2b",                    // if cc == 1 { jump 'retry }
                    dst = in(reg) ptr_reg!(dst),
                    $($reg)*
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    out("r0") prev_hi,
                    out("r1") prev_lo,
                    out("r12") _,
                    out("r13") _,
                    // Do not use `preserves_flags` because CDSG modifies the condition code.
                    options(nostack),
                );
                U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
            }
        }
    };
}

atomic_rmw_cas_3! {
    atomic_add, [],
    distinct_op!("algr", "%r13", "%r1", "{val_lo}"), // r13 = r1 + val_lo; cc = zero | carry
    "lgr %r12, %r0",                                 // r12 = r0
    "alcgr %r12, {val_hi}",                          // r12 += val_hi + carry
}
atomic_rmw_cas_3! {
    atomic_sub, [],
    distinct_op!("slgr", "%r13", "%r1", "{val_lo}"), // r13 = r1 - val_lo; cc = zero | borrow
    "lgr %r12, %r0",                                 // r12 = r0
    "slbgr %r12, {val_hi}",                          // r12 -= val_hi + borrow
}
atomic_rmw_cas_3! {
    atomic_and, [],
    distinct_op!("ngr", "%r13", "%r1", "{val_lo}"), // r13 = r1 & val_lo
    distinct_op!("ngr", "%r12", "%r0", "{val_hi}"), // r12 = r0 & val_hi
}

// Use nngrk on z15 or later.
#[cfg(any(
    target_feature = "miscellaneous-extensions-3",
    portable_atomic_target_feature = "miscellaneous-extensions-3",
))]
atomic_rmw_cas_3! {
    atomic_nand, [],
    "nngrk %r13, %r1, {val_lo}", // r13 = !(r1 & val_lo)
    "nngrk %r12, %r0, {val_hi}", // r12 = !(r0 & val_hi)
}
#[cfg(not(any(
    target_feature = "miscellaneous-extensions-3",
    portable_atomic_target_feature = "miscellaneous-extensions-3",
)))]
atomic_rmw_cas_3! {
    atomic_nand, [],
    distinct_op!("ngr", "%r13", "%r1", "{val_lo}"), // r13 = r1 & val_lo
    distinct_op!("ngr", "%r12", "%r0", "{val_hi}"), // r12 = r0 & val_hi
    "lcgr %r13, %r13",                              // r13 = !r13 + 1
    "aghi %r13, -1",                                // r13 -= 1
    "lcgr %r12, %r12",                              // r12 = !r12 + 1
    "aghi %r12, -1",                                // r12 -= 1
}

atomic_rmw_cas_3! {
    atomic_or, [],
    distinct_op!("ogr", "%r13", "%r1", "{val_lo}"), // r13 = r1 | val_lo
    distinct_op!("ogr", "%r12", "%r0", "{val_hi}"), // r12 = r0 | val_hi
}
atomic_rmw_cas_3! {
    atomic_xor, [],
    distinct_op!("xgr", "%r13", "%r1", "{val_lo}"), // r13 = r1 ^ val_lo
    distinct_op!("xgr", "%r12", "%r0", "{val_hi}"), // r12 = r0 ^ val_hi
}

#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
atomic_rmw_cas_3! {
    atomic_max, [],
    "clgr %r1, {val_lo}",                       // if r1(u) < val_lo(u) { cc = 1 } else if r1(u) > val_lo(u) { cc = 2 } else { cc = 0 }
    select_op!("h", "%r12", "%r1", "{val_lo}"), // if cc == 2 { r12 = r1 } else { r12 = val_lo }
    "cgr %r0, {val_hi}",                        // if r0(i) < val_hi(i) { cc = 1 } else if r0(i) > val_hi(i) { cc = 2 } else { cc = 0 }
    select_op!("h", "%r13", "%r1", "{val_lo}"), // if cc == 2 { r13 = r1 } else { r13 = val_lo }
    "locgre %r13, %r12",                        // if cc == 0 { r13 = r12 }
    select_op!("h", "%r12", "%r0", "{val_hi}"), // if cc == 2 { r12 = r0 } else { r12 = val_hi }
}
#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
atomic_rmw_cas_3! {
    atomic_umax, [tmp = out(reg) _,],
    "clgr %r1, {val_lo}",                        // if r1(u) < val_lo(u) { cc = 1 } else if r1(u) > val_lo(u) { cc = 2 } else { cc = 0 }
    select_op!("h", "{tmp}", "%r1", "{val_lo}"), // if cc == 2 { tmp = r1 } else { tmp = val_lo }
    "clgr %r0, {val_hi}",                        // if r0(u) < val_hi(u) { cc = 1 } else if r0(u) > val_hi(u) { cc = 2 } else { cc = 0 }
    select_op!("h", "%r12", "%r0", "{val_hi}"),  // if cc == 2 { r12 = r0 } else { r12 = val_hi }
    select_op!("h", "%r13", "%r1", "{val_lo}"),  // if cc == 2 { r13 = r1 } else { r13 = val_lo }
    "cgr %r0, {val_hi}",                         // if r0(i) < val_hi(i) { cc = 1 } else if r0(i) > val_hi(i) { cc = 2 } else { cc = 0 }
    "locgre %r13, {tmp}",                        // if cc == 0 { r13 = tmp }
}
#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
atomic_rmw_cas_3! {
    atomic_min, [],
    "clgr %r1, {val_lo}",                       // if r1(u) < val_lo(u) { cc = 1 } else if r1(u) > val_lo(u) { cc = 2 } else { cc = 0 }
    select_op!("l", "%r12", "%r1", "{val_lo}"), // if cc == 1 { r12 = r1 } else { r12 = val_lo }
    "cgr %r0, {val_hi}",                        // if r0(i) < val_hi(i) { cc = 1 } else if r0(i) > val_hi(i) { cc = 2 } else { cc = 0 }
    select_op!("l", "%r13", "%r1", "{val_lo}"), // if cc == 1 { r13 = r1 } else { r13 = val_lo }
    "locgre %r13, %r12",                        // if cc == 0 { r13 = r12 }
    select_op!("l", "%r12", "%r0", "{val_hi}"), // if cc == 1 { r12 = r0 } else { r12 = val_hi }
}
#[cfg(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
))]
atomic_rmw_cas_3! {
    atomic_umin, [tmp = out(reg) _,],
    "clgr %r1, {val_lo}",                        // if r1(u) < val_lo(u) { cc = 1 } else if r1(u) > val_lo(u) { cc = 2 } else { cc = 0 }
    select_op!("l", "{tmp}", "%r1", "{val_lo}"), // if cc == 1 { tmp = r1 } else { tmp = val_lo }
    "clgr %r0, {val_hi}",                        // if r0(u) < val_hi(u) { cc = 1 } else if r0(u) > val_hi(u) { cc = 2 } else { cc = 0 }
    select_op!("l", "%r12", "%r0", "{val_hi}"),  // if cc == 1 { r12 = r0 } else { r12 = val_hi }
    select_op!("l", "%r13", "%r1", "{val_lo}"),  // if cc == 1 { r13 = r1 } else { r13 = val_lo }
    "cgr %r0, {val_hi}",                         // if r0(i) < val_hi(i) { cc = 1 } else if r0(i) > val_hi(i) { cc = 2 } else { cc = 0 }
    "locgre %r13, {tmp}",                        // if cc == 0 { r13 = tmp }
}
// We use atomic_update for atomic min/max on pre-z196 because
// z10 doesn't seem to have a good way to implement 128-bit min/max.
// loc{,g}r requires z196 or later.
// https://godbolt.org/z/EqoMEP8b3
#[cfg(not(any(
    target_feature = "load-store-on-cond",
    portable_atomic_target_feature = "load-store-on-cond",
)))]
atomic_rmw_by_atomic_update!(cmp);

atomic_rmw_cas_2! {
    atomic_not, [],
    "lcgr %r13, %r1", // r13 = !r1 + 1
    "aghi %r13, -1",  // r13 -= 1
    "lcgr %r12, %r0", // r12 = !r0 + 1
    "aghi %r12, -1",  // r12 -= 1
}

#[cfg(any(target_feature = "distinct-ops", portable_atomic_target_feature = "distinct-ops"))]
atomic_rmw_cas_2! {
    atomic_neg, [zero = in(reg) 0_u64,],
    "slgrk %r13, {zero}, %r1", // r13 = 0 - r1; cc = zero | borrow
    "lghi %r12, 0",            // r12 = 0
    "slbgr %r12, %r0",         // r12 -= r0 + borrow
}
#[cfg(not(any(target_feature = "distinct-ops", portable_atomic_target_feature = "distinct-ops")))]
atomic_rmw_cas_2! {
    atomic_neg, [],
    "lghi %r13, 0",    // r13 = 0
    "slgr %r13, %r1",  // r13 -= r1; cc = zero | borrow
    "lghi %r12, 0",    // r12 = 0
    "slbgr %r12, %r0", // r12 -= r0 + borrow
}

#[inline]
const fn is_lock_free() -> bool {
    IS_ALWAYS_LOCK_FREE
}
const IS_ALWAYS_LOCK_FREE: bool = true;

atomic128!(AtomicI128, i128, atomic_max, atomic_min);
atomic128!(AtomicU128, u128, atomic_umax, atomic_umin);

#[cfg(test)]
mod tests {
    use super::*;

    test_atomic_int!(i128);
    test_atomic_int!(u128);

    // load/store/swap implementation is not affected by signedness, so it is
    // enough to test only unsigned types.
    stress_test!(u128);
}
