// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementation on PowerPC64.

This architecture provides the following 128-bit atomic instructions:

- lq/stq: load/store (ISA 2.07 or later, included in the Linux Compliancy subset and AIX Compliancy subset)
- lqarx/stqcx.: LL/SC (ISA 2.07 or later, included in the Linux Compliancy subset and AIX Compliancy subset)
- plq/pstq: load/store (ISA 3.1 or later, included in the Linux Compliancy subset and AIX Compliancy subset)

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#powerpc

Note that we do not separate LL and SC into separate functions, but handle
them within a single asm block. This is because it is theoretically possible
for the compiler to insert operations that might clear the reservation between
LL and SC. See aarch64.rs for details.

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use intrinsics.rs instead.

Refs:
- Power ISA
  https://openpowerfoundation.org/specifications/isa
- AIX Assembler language reference
  https://www.ibm.com/docs/en/aix/7.3?topic=aix-assembler-language-reference
- atomic-maybe-uninit
  https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- powerpc64 (pwr8) https://godbolt.org/z/TjKsPbWc6
- powerpc64le https://godbolt.org/z/5WqPGhb3Y
*/

include!("macros.rs");

#[cfg(not(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
)))]
#[path = "../fallback/outline_atomics.rs"]
mod fallback;

// On musl with static linking, it seems that getauxval is not always available.
// See detect/auxv.rs for more.
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(
    test,
    not(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    )),
))]
#[cfg(any(
    all(
        target_os = "linux",
        any(
            all(
                target_env = "gnu",
                any(target_endian = "little", not(target_feature = "crt-static")),
            ),
            all(target_env = "musl", any(not(target_feature = "crt-static"), feature = "std")),
            target_env = "ohos",
            all(target_env = "uclibc", not(target_feature = "crt-static")),
            portable_atomic_outline_atomics,
        ),
    ),
    target_os = "android",
    all(
        target_os = "freebsd",
        any(
            target_endian = "little",
            not(target_feature = "crt-static"),
            portable_atomic_outline_atomics,
        ),
    ),
    target_os = "openbsd",
))]
#[path = "../detect/auxv.rs"]
mod detect;
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(
    test,
    not(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    )),
))]
#[cfg(target_os = "aix")]
#[cfg(not(portable_atomic_pre_llvm_20))] // SIGTRAP on LLVM 19
#[cfg(any(test, portable_atomic_outline_atomics))] // TODO(aix): currently disabled by default
#[path = "../detect/powerpc64_aix.rs"]
mod detect;

use core::{arch::asm, sync::atomic::Ordering};

use crate::utils::{Pair, U128};

macro_rules! debug_assert_pwr8 {
    () => {
        #[cfg(not(any(
            target_feature = "quadword-atomics",
            portable_atomic_target_feature = "quadword-atomics",
        )))]
        {
            debug_assert!(detect::detect().quadword_atomics());
        }
    };
}

// Refs: https://www.ibm.com/docs/en/aix/7.3?topic=ops-machine-pseudo-op
//
// This is similar to #[target_feature(enable = "quadword-atomics")], except that there are
// no compiler guarantees regarding (un)inlining, and the scope is within an asm
// block rather than a function. We use this directive because #[target_feature(enable = "quadword-atomics")]
// is unstable and unavailable on old nightly and incompatible with rustc_codegen_cranelift:
// https://github.com/rust-lang/rustc_codegen_cranelift/issues/1400#issuecomment-1774599775
//
// Note: start_pwr8 and end_pwr8 must be used in pairs.
//
// Note: If power8 instructions are not available at compile-time, we must guarantee that
// the function that uses it is not inlined into a function where it is not
// clear whether power8 instructions are available. Otherwise, (even if we checked whether
// power8 instructions are available at run-time) optimizations that reorder its
// instructions across the if condition might introduce undefined behavior.
// (see also https://rust-lang.github.io/rfcs/2045-target-feature.html#safely-inlining-target_feature-functions-on-more-contexts)
// However, our code uses the ifunc helper macro that works with function pointers,
// so we don't have to worry about this unless calling without helper macro.
macro_rules! start_pwr8 {
    () => {
        ".machine push\n.machine power8"
    };
}
macro_rules! end_pwr8 {
    () => {
        ".machine pop"
    };
}

macro_rules! atomic_rmw {
    ($op:ident, $order:ident) => {
        match $order {
            Ordering::Relaxed => $op!("", ""),
            Ordering::Acquire => $op!("isync", ""),
            Ordering::Release => $op!("", "lwsync"),
            Ordering::AcqRel => $op!("isync", "lwsync"),
            Ordering::SeqCst => $op!("isync", "sync"),
            _ => unreachable!(),
        }
    };
}
macro_rules! atomic_cas {
    ($op:ident, $success:ident, $failure:ident) => {
        if $failure == Ordering::Relaxed {
            match $success {
                Ordering::Relaxed => $op!("", "", ""),
                Ordering::Acquire => $op!("", "isync", ""),
                Ordering::Release => $op!("", "", "lwsync"),
                Ordering::AcqRel => $op!("", "isync", "lwsync"),
                Ordering::SeqCst => $op!("", "isync", "sync"),
                _ => unreachable!(),
            }
        } else {
            let order = crate::utils::upgrade_success_ordering($success, $failure);
            match order {
                // Relaxed and Release are covered in $failure == Relaxed branch.
                Ordering::Acquire => $op!("isync", "", ""),
                Ordering::AcqRel => $op!("isync", "", "lwsync"),
                Ordering::SeqCst => $op!("isync", "", "sync"),
                _ => unreachable!(),
            }
        }
    };
}

// Extracts and checks the EQ bit of cr0.
#[inline]
fn test_cr0_eq(cr: u64) -> bool {
    cr & 0x20000000 != 0
}

// If quadword-atomics is available at compile-time, we can always use pwr8_fn.
#[cfg(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
))]
use self::atomic_load_pwr8 as atomic_load;
// Otherwise, we need to do run-time detection and can use pwr8_fn only if quadword-atomics is available.
#[cfg(not(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
)))]
#[inline]
unsafe fn atomic_load(src: *mut u128, order: Ordering) -> u128 {
    fn_alias! {
        // inline(never) is just a hint and also not strictly necessary
        // because we use ifunc helper macro, but used for clarity.
        #[inline(never)]
        unsafe fn(src: *mut u128) -> u128;
        atomic_load_pwr8_relaxed = atomic_load_pwr8(Ordering::Relaxed);
        atomic_load_pwr8_acquire = atomic_load_pwr8(Ordering::Acquire);
        atomic_load_pwr8_seqcst = atomic_load_pwr8(Ordering::SeqCst);
    }
    // SAFETY: the caller must uphold the safety contract.
    // we only calls atomic_load_pwr8 if quadword-atomics is available.
    unsafe {
        match order {
            Ordering::Relaxed => {
                ifunc!(unsafe fn(src: *mut u128) -> u128 {
                    if detect::detect().quadword_atomics() {
                        atomic_load_pwr8_relaxed
                    } else {
                        fallback::atomic_load_non_seqcst
                    }
                })
            }
            Ordering::Acquire => {
                ifunc!(unsafe fn(src: *mut u128) -> u128 {
                    if detect::detect().quadword_atomics() {
                        atomic_load_pwr8_acquire
                    } else {
                        fallback::atomic_load_non_seqcst
                    }
                })
            }
            Ordering::SeqCst => {
                ifunc!(unsafe fn(src: *mut u128) -> u128 {
                    if detect::detect().quadword_atomics() {
                        atomic_load_pwr8_seqcst
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
unsafe fn atomic_load_pwr8(src: *mut u128, order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_pwr8!();
    let (out_hi, out_lo);

    // SAFETY: the caller must uphold the safety contract.
    //
    // Refs: Section 3.3.4 "Fixed Point Load and Store Quadword Instructions" of Power ISA 3.1C Book I
    unsafe {
        macro_rules! atomic_load_acquire {
            ($release:tt) => {
                asm!(
                    start_pwr8!(),
                    $release,
                    "lq %r4, 0({src})", // atomic { r4:r5 = *src }
                    "cmpw %r4, %r4",    // if r4 == r4 { cr0.EQ = 1 } else { cr0.EQ = 0 }
                    "bne- %cr0, 2f",    // if unlikely(cr0.EQ == 0) { jump 'never }
                    "2:", // 'never:
                    "isync",            // fence (works in combination with a branch that depends on the loaded value)
                    end_pwr8!(),
                    src = in(reg_nonzero) ptr_reg!(src),
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    out("r4") out_hi,
                    out("r5") out_lo,
                    out("cr0") _,
                    options(nostack, preserves_flags),
                )
            };
        }
        match order {
            Ordering::Relaxed => {
                asm!(
                    start_pwr8!(),
                    "lq %r4, 0({src})", // atomic { r4:r5 = *src }
                    end_pwr8!(),
                    src = in(reg_nonzero) ptr_reg!(src),
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    out("r4") out_hi,
                    out("r5") out_lo,
                    options(nostack, preserves_flags),
                );
            }
            Ordering::Acquire => atomic_load_acquire!(""),
            Ordering::SeqCst => atomic_load_acquire!("sync"),
            _ => unreachable!(),
        }
        U128 { pair: Pair { hi: out_hi, lo: out_lo } }.whole
    }
}

// If quadword-atomics is available at compile-time, we can always use pwr8_fn.
#[cfg(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
))]
use self::atomic_store_pwr8 as atomic_store;
// Otherwise, we need to do run-time detection and can use pwr8_fn only if quadword-atomics is available.
#[cfg(not(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
)))]
#[inline]
unsafe fn atomic_store(dst: *mut u128, val: u128, order: Ordering) {
    fn_alias! {
        // inline(never) is just a hint and also not strictly necessary
        // because we use ifunc helper macro, but used for clarity.
        #[inline(never)]
        unsafe fn(dst: *mut u128, val: u128);
        atomic_store_pwr8_relaxed = atomic_store_pwr8(Ordering::Relaxed);
        atomic_store_pwr8_release = atomic_store_pwr8(Ordering::Release);
        atomic_store_pwr8_seqcst = atomic_store_pwr8(Ordering::SeqCst);
    }
    // SAFETY: the caller must uphold the safety contract.
    // we only calls atomic_store_pwr8 if quadword-atomics is available.
    unsafe {
        match order {
            Ordering::Relaxed => {
                ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                    if detect::detect().quadword_atomics() {
                        atomic_store_pwr8_relaxed
                    } else {
                        fallback::atomic_store_non_seqcst
                    }
                });
            }
            Ordering::Release => {
                ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                    if detect::detect().quadword_atomics() {
                        atomic_store_pwr8_release
                    } else {
                        fallback::atomic_store_non_seqcst
                    }
                });
            }
            Ordering::SeqCst => {
                ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                    if detect::detect().quadword_atomics() {
                        atomic_store_pwr8_seqcst
                    } else {
                        fallback::atomic_store_seqcst
                    }
                });
            }
            _ => unreachable!(),
        }
    }
}
#[inline]
unsafe fn atomic_store_pwr8(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_pwr8!();
    let val = U128 { whole: val };

    // SAFETY: the caller must uphold the safety contract.
    //
    // Refs: Section 3.3.4 "Fixed Point Load and Store Quadword Instructions" of Power ISA 3.1C Book I
    unsafe {
        macro_rules! atomic_store {
            ($release:tt) => {
                asm!(
                    start_pwr8!(),
                    $release,            // fence
                    "stq %r4, 0({dst})", // atomic { *dst = r4:r5 }
                    end_pwr8!(),
                    dst = in(reg_nonzero) ptr_reg!(dst),
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    in("r4") val.pair.hi,
                    in("r5") val.pair.lo,
                    options(nostack, preserves_flags),
                )
            };
        }
        match order {
            Ordering::Relaxed => atomic_store!(""),
            Ordering::Release => atomic_store!("lwsync"),
            Ordering::SeqCst => atomic_store!("sync"),
            _ => unreachable!(),
        }
    }
}

#[inline]
unsafe fn atomic_compare_exchange(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> Result<u128, u128> {
    #[cfg(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    ))]
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantees that quadword atomics instructions are available at compile-time.
    let (prev, ok) = unsafe { atomic_compare_exchange_pwr8(dst, old, new, success, failure) };
    #[cfg(not(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    )))]
    // SAFETY: the caller must uphold the safety contract.
    let (prev, ok) = {
        fn_alias! {
            // inline(never) is just a hint and also not strictly necessary
            // because we use ifunc helper macro, but used for clarity.
            #[inline(never)]
            unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool);
            pwr8_relaxed_fn = atomic_compare_exchange_pwr8(Ordering::Relaxed, Ordering::Relaxed);
            pwr8_acquire_fn = atomic_compare_exchange_pwr8(Ordering::Acquire, Ordering::Acquire);
            pwr8_release_fn = atomic_compare_exchange_pwr8(Ordering::Release, Ordering::Relaxed);
            pwr8_acqrel_fn = atomic_compare_exchange_pwr8(Ordering::AcqRel, Ordering::Acquire);
            pwr8_seqcst_fn = atomic_compare_exchange_pwr8(Ordering::SeqCst, Ordering::SeqCst);
        }
        // SAFETY: the caller must uphold the safety contract.
        // we only calls pwr8_fn if quadword-atomics is available.
        unsafe {
            let success = crate::utils::upgrade_success_ordering(success, failure);
            match success {
                Ordering::Relaxed => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
                        if detect::detect().quadword_atomics() {
                            pwr8_relaxed_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::Acquire => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
                        if detect::detect().quadword_atomics() {
                            pwr8_acquire_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::Release => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
                        if detect::detect().quadword_atomics() {
                            pwr8_release_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::AcqRel => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
                        if detect::detect().quadword_atomics() {
                            pwr8_acqrel_fn
                        } else {
                            fallback::atomic_compare_exchange_non_seqcst
                        }
                    })
                }
                Ordering::SeqCst => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
                        if detect::detect().quadword_atomics() {
                            pwr8_seqcst_fn
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
unsafe fn atomic_compare_exchange_pwr8(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> (u128, bool) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_pwr8!();
    let old = U128 { whole: old };
    let new = U128 { whole: new };
    let (mut prev_hi, mut prev_lo);
    let mut r;

    // SAFETY: the caller must uphold the safety contract.
    //
    // Refs: Section 4.6.2.2 "128-bit Load And Reserve and Store Conditional Instructions" of Power ISA 3.1C Book II
    unsafe {
        macro_rules! cmpxchg {
            ($acquire_always:tt, $acquire_success:tt, $release:tt) => {
                asm!(
                    start_pwr8!(),
                    $release,                               // fence
                    "2:", // 'retry:
                        "lqarx %r8, 0, {dst}",              // atomic { RESERVE = (dst, 16); r8:r9 = *dst }
                        "xor {tmp_lo}, %r9, {old_lo}",      // tmp_lo = r9 ^ old_lo
                        "xor {tmp_hi}, %r8, {old_hi}",      // tmp_hi = r8 ^ old_hi
                        "or. {tmp_lo}, {tmp_lo}, {tmp_hi}", // tmp_lo |= tmp_hi; if tmp_lo == 0 { cr0.EQ = 1 } else { cr0.EQ = 0 }
                        "bne %cr0, 3f",                     // if cr0.EQ == 0 { jump 'cmp-fail }
                        "stqcx. %r6, 0, {dst}",             // atomic { if RESERVE == (dst, 16) { *dst = r6:r7; cr0.EQ = 1 } else { cr0.EQ = 0 }; RESERVE = None }
                        "bne %cr0, 2b",                     // if cr0.EQ == 0 { jump 'retry }
                        $acquire_success,                   // fence
                    "3:", // 'cmp-fail:
                    $acquire_always,                        // fence
                    "mfcr {tmp_lo}",                        // tmp_lo = zero_extend(cr)
                    end_pwr8!(),
                    dst = in(reg_nonzero) ptr_reg!(dst),
                    old_hi = in(reg) old.pair.hi,
                    old_lo = in(reg) old.pair.lo,
                    tmp_hi = out(reg) _,
                    tmp_lo = out(reg) r,
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    in("r6") new.pair.hi,
                    in("r7") new.pair.lo,
                    out("r8") prev_hi,
                    out("r9") prev_lo,
                    out("cr0") _,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_cas!(cmpxchg, success, failure);
        // if compare failed EQ bit is cleared, if store succeeds EQ bit is set.
        (U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole, test_cr0_eq(r))
    }
}

// Always use strong CAS for outline-atomics.
#[cfg(not(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
)))]
use self::atomic_compare_exchange as atomic_compare_exchange_weak;
#[cfg(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
))]
#[inline]
unsafe fn atomic_compare_exchange_weak(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> Result<u128, u128> {
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantees that quadword atomics instructions are available at compile-time.
    let (prev, ok) = unsafe { atomic_compare_exchange_weak_pwr8(dst, old, new, success, failure) };
    if ok { Ok(prev) } else { Err(prev) }
}
#[cfg(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
))]
#[inline]
unsafe fn atomic_compare_exchange_weak_pwr8(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> (u128, bool) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_pwr8!();
    let old = U128 { whole: old };
    let new = U128 { whole: new };
    let (mut prev_hi, mut prev_lo);
    let mut r;

    // SAFETY: the caller must uphold the safety contract.
    //
    // Refs: Section 4.6.2.2 "128-bit Load And Reserve and Store Conditional Instructions" of Power ISA 3.1C Book II
    unsafe {
        macro_rules! cmpxchg_weak {
            ($acquire_always:tt, $acquire_success:tt, $release:tt) => {
                asm!(
                    start_pwr8!(),
                    $release,                           // fence
                    "lqarx %r8, 0, {dst}",              // atomic { RESERVE = (dst, 16); r8:r9 = *dst }
                    "xor {tmp_lo}, %r9, {old_lo}",      // tmp_lo = r9 ^ old_lo
                    "xor {tmp_hi}, %r8, {old_hi}",      // tmp_hi = r8 ^ old_hi
                    "or. {tmp_lo}, {tmp_lo}, {tmp_hi}", // tmp_lo |= tmp_hi; if tmp_lo == 0 { cr0.EQ = 1 } else { cr0.EQ = 0 }
                    "bne %cr0, 3f",                     // if cr0.EQ == 0 { jump 'cmp-fail }
                    "stqcx. %r6, 0, {dst}",             // atomic { if RESERVE == (dst, 16) { *dst = r6:r7; cr0.EQ = 1 } else { cr0.EQ = 0 }; RESERVE = None }
                    $acquire_success,                   // fence
                    "3:", // 'cmp-fail:
                    $acquire_always,                    // fence
                    "mfcr {tmp_lo}",                    // tmp_lo = zero_extend(cr)
                    end_pwr8!(),
                    dst = in(reg_nonzero) ptr_reg!(dst),
                    old_hi = in(reg) old.pair.hi,
                    old_lo = in(reg) old.pair.lo,
                    tmp_hi = out(reg) _,
                    tmp_lo = out(reg) r,
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    in("r6") new.pair.hi,
                    in("r7") new.pair.lo,
                    out("r8") prev_hi,
                    out("r9") prev_lo,
                    out("cr0") _,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_cas!(cmpxchg_weak, success, failure);
        // if compare or store failed EQ bit is cleared, if store succeeds EQ bit is set.
        (U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole, test_cr0_eq(r))
    }
}

// Do not use atomic_rmw_ll_sc_3 because it needs extra MR to implement swap.
#[inline]
unsafe fn atomic_swap_pwr8(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_pwr8!();
    let val = U128 { whole: val };
    let (mut prev_hi, mut prev_lo);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        macro_rules! swap {
            ($acquire:tt, $release:tt) => {
                asm!(
                    start_pwr8!(),
                    $release,                   // fence
                    "2:", // 'retry:
                        "lqarx %r6, 0, {dst}",  // atomic { RESERVE = (dst, 16); r6:r7 = *dst }
                        "stqcx. %r8, 0, {dst}", // atomic { if RESERVE == (dst, 16) { *dst = r8:r9; cr0.EQ = 1 } else { cr0.EQ = 0 }; RESERVE = None }
                        "bne %cr0, 2b",         // if cr0.EQ == 0 { jump 'retry }
                    $acquire,                   // fence
                    end_pwr8!(),
                    dst = in(reg_nonzero) ptr_reg!(dst),
                    // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                    // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                    out("r6") prev_hi,
                    out("r7") prev_lo,
                    in("r8") val.pair.hi,
                    in("r9") val.pair.lo,
                    out("cr0") _,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_rmw!(swap, order);
        U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
    }
}

/// Atomic RMW by LL/SC loop (3 arguments)
/// `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`
///
/// $op can use the following registers:
/// - val_hi/val_lo pair: val argument (read-only for `$op`)
/// - r6/r7 pair: previous value loaded by ll (read-only for `$op`)
/// - r8/r9 pair: new value that will be stored by sc
macro_rules! atomic_rmw_ll_sc_3 {
    ($name:ident, [$($reg:tt)*], $($op:tt)*) => {
        #[inline]
        unsafe fn $name(dst: *mut u128, val: u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_pwr8!();
            let val = U128 { whole: val };
            let (mut prev_hi, mut prev_lo);

            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                macro_rules! op {
                    ($acquire:tt, $release:tt) => {
                        asm!(
                            start_pwr8!(),
                            $release,                   // fence
                            "2:", // 'retry:
                                "lqarx %r6, 0, {dst}",  // atomic { RESERVE = (dst, 16); r6:r7 = *dst }
                                $($op)*
                                "stqcx. %r8, 0, {dst}", // atomic { if RESERVE == (dst, 16) { *dst = r8:r9; cr0.EQ = 1 } else { cr0.EQ = 0 }; RESERVE = None }
                                "bne %cr0, 2b",         // if cr0.EQ == 0 { jump 'retry }
                            $acquire,                   // fence
                            end_pwr8!(),
                            dst = in(reg_nonzero) ptr_reg!(dst),
                            val_hi = in(reg) val.pair.hi,
                            val_lo = in(reg) val.pair.lo,
                            $($reg)*
                            // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                            // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                            out("r6") prev_hi,
                            out("r7") prev_lo,
                            out("r8") _, // new (hi)
                            out("r9") _, // new (lo)
                            out("cr0") _,
                            options(nostack, preserves_flags),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
            }
        }
    };
}
/// Atomic RMW by LL/SC loop (2 arguments)
/// `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`
///
/// $op can use the following registers:
/// - r6/r7 pair: previous value loaded by ll (read-only for `$op`)
/// - r8/r9 pair: new value that will be stored by sc
macro_rules! atomic_rmw_ll_sc_2 {
    ($name:ident, [$($reg:tt)*], $($op:tt)*) => {
        #[inline]
        unsafe fn $name(dst: *mut u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_pwr8!();
            let (mut prev_hi, mut prev_lo);

            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                macro_rules! op {
                    ($acquire:tt, $release:tt) => {
                        asm!(
                            start_pwr8!(),
                            $release,                   // fence
                            "2:", // 'retry:
                                "lqarx %r6, 0, {dst}",  // atomic { RESERVE = (dst, 16); r6:r7 = *dst }
                                $($op)*
                                "stqcx. %r8, 0, {dst}", // atomic { if RESERVE == (dst, 16) { *dst = r8:r9; cr0.EQ = 1 } else { cr0.EQ = 0 }; RESERVE = None }
                                "bne %cr0, 2b",         // if cr0.EQ == 0 { jump 'retry }
                            $acquire,                   // fence
                            end_pwr8!(),
                            dst = in(reg_nonzero) ptr_reg!(dst),
                            $($reg)*
                            // Quadword atomic instructions work with even/odd pair of specified register and subsequent register.
                            // We cannot use r1 (sp) and r2 (system reserved), so start with r4 or grater.
                            out("r6") prev_hi,
                            out("r7") prev_lo,
                            out("r8") _, // new (hi)
                            out("r9") _, // new (lo)
                            out("cr0") _,
                            options(nostack, preserves_flags),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { hi: prev_hi, lo: prev_lo } }.whole
            }
        }
    };
}

atomic_rmw_ll_sc_3! {
    atomic_add_pwr8, [out("xer") _,],
    "addc %r9, {val_lo}, %r7", // r9 = val_lo + r7; xer.CA = carry
    "adde %r8, {val_hi}, %r6", // r8 = val_hi + r6 + xer.CA
}
atomic_rmw_ll_sc_3! {
    atomic_sub_pwr8, [out("xer") _,],
    "subc %r9, %r7, {val_lo}",  // r9 = val_lo - r7; xer.CA = borrow
    "subfe %r8, {val_hi}, %r6", // r8 = val_hi - r6 - xer.CA
}
atomic_rmw_ll_sc_3! {
    atomic_and_pwr8, [],
    "and %r9, {val_lo}, %r7", // r9 = val_lo & r7
    "and %r8, {val_hi}, %r6", // r8 = val_hi & r6
}
atomic_rmw_ll_sc_3! {
    atomic_nand_pwr8, [],
    "nand %r9, {val_lo}, %r7", // r9 = !(val_lo & r7)
    "nand %r8, {val_hi}, %r6", // r8 = !(val_hi & r6)
}
atomic_rmw_ll_sc_3! {
    atomic_or_pwr8, [],
    "or %r9, {val_lo}, %r7", // r9 = val_lo | r7
    "or %r8, {val_hi}, %r6", // r8 = val_hi | r6
}
atomic_rmw_ll_sc_3! {
    atomic_xor_pwr8, [],
    "xor %r9, {val_lo}, %r7", // r9 = val_lo ^ r7
    "xor %r8, {val_hi}, %r6", // r8 = val_hi ^ r6
}
atomic_rmw_ll_sc_3! {
    atomic_max_pwr8, [out("cr1") _,],
    "cmpld %r7, {val_lo}",        // if r7(u) < val_lo(u) { cr0 = { LT: 1, ..0 } } else if r7(u) > val_lo(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "iselgt %r9, %r7, {val_lo}",  // if cr0.GT == 1 { r9 = r7 } else { r9 = val_lo }
    "cmpd %cr1, %r6, {val_hi}",   // if r6(i) < val_hi(i) { cr1 = { LT: 1, ..0 } } else if r6(i) > val_hi(i) { cr1 = { GT: 1, ..0 } } else { cr1 = { EQ: 1, ..0 } }
    "isel %r8, %r7, {val_lo}, 5", // if cr1.GT == 1 { r8 = r7 } else { r8 = val_lo }
    "cmpld %r6, {val_hi}",        // if r6(u) < val_hi(u) { cr0 = { LT: 1, ..0 } } else if r6(u) > val_hi(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "iseleq %r9, %r9, %r8",       // if cr0.EQ == 1 { r9 = r9 } else { r9 = r8 }
    "isel %r8, %r6, {val_hi}, 5", // if cr1.GT == 1 { r8 = r6 } else { r8 = val_hi }
}
atomic_rmw_ll_sc_3! {
    atomic_umax_pwr8, [],
    "cmpld %r7, {val_lo}",       // if r7(u) < val_lo(u) { cr0 = { LT: 1, ..0 } } else if r7(u) > val_lo(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "iselgt %r9, %r7, {val_lo}", // if cr0.GT == 1 { r9 = r7 } else { r9 = val_lo }
    "cmpld %r6, {val_hi}",       // if r6(u) < val_hi(u) { cr0 = { LT: 1, ..0 } } else if r6(u) > val_hi(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "iselgt %r8, %r7, {val_lo}", // if cr0.GT == 1 { r8 = r7 } else { r8 = val_lo }
    "iseleq %r9, %r9, %r8",      // if cr0.EQ == 1 { r9 = r9 } else { r9 = r8 }
    "iselgt %r8, %r6, {val_hi}", // if cr0.GT == 1 { r8 = r6 } else { r8 = val_hi }
}
atomic_rmw_ll_sc_3! {
    atomic_min_pwr8, [out("cr1") _,],
    "cmpld %r7, {val_lo}",        // if r7(u) < val_lo(u) { cr0 = { LT: 1, ..0 } } else if r7(u) > val_lo(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "isellt %r9, %r7, {val_lo}",  // if cr0.LT == 1 { r9 = r7 } else { r9 = val_lo }
    "cmpd %cr1, %r6, {val_hi}",   // if r6(i) < val_hi(i) { cr1 = { LT: 1, ..0 } } else if r6(i) > val_hi(i) { cr1 = { GT: 1, ..0 } } else { cr1 = { EQ: 1, ..0 } }
    "isel %r8, %r7, {val_lo}, 4", // if cr1.LT == 1 { r8 = r7 } else { r8 = val_lo }
    "cmpld %r6, {val_hi}",        // if r6(u) < val_hi(u) { cr0 = { LT: 1, ..0 } } else if r6(u) > val_hi(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "iseleq %r9, %r9, %r8",       // if cr0.EQ == 1 { r9 = r9 } else { r9 = r8 }
    "isel %r8, %r6, {val_hi}, 4", // if cr1.LT == 1 { r8 = r6 } else { r8 = val_hi }
}
atomic_rmw_ll_sc_3! {
    atomic_umin_pwr8, [],
    "cmpld %r7, {val_lo}",       // if r7(u) < val_lo(u) { cr0 = { LT: 1, ..0 } } else if r7(u) > val_lo(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "isellt %r9, %r7, {val_lo}", // if cr0.LT == 1 { r9 = r7 } else { r9 = val_lo }
    "cmpld %r6, {val_hi}",       // if r6(u) < val_hi(u) { cr0 = { LT: 1, ..0 } } else if r6(u) > val_hi(u) { cr0 = { GT: 1, ..0 } } else { cr0 = { EQ: 1, ..0 } }
    "isellt %r8, %r7, {val_lo}", // if cr0.LT == 1 { r8 = r7 } else { r8 = val_lo }
    "iseleq %r9, %r9, %r8",      // if cr0.EQ == 1 { r9 = r9 } else { r9 = r8 }
    "isellt %r8, %r6, {val_hi}", // if cr0.LT == 1 { r8 = r6 } else { r8 = val_hi }
}

#[inline]
unsafe fn atomic_not_pwr8(dst: *mut u128, order: Ordering) -> u128 {
    // SAFETY: the caller must uphold the safety contract.
    unsafe { atomic_xor_pwr8(dst, !0, order) }
}

#[cfg(not(portable_atomic_pre_llvm_16))]
atomic_rmw_ll_sc_2! {
    atomic_neg_pwr8, [out("xer") _,],
    "subfic %r9, %r7, 0", // r9 = 0 - r7; xer.CA = borrow
    "subfze %r8, %r6",    // r8 = 0 - r6 - xer.CA
}
// LLVM 15 miscompiles subfic.
#[cfg(portable_atomic_pre_llvm_16)]
atomic_rmw_ll_sc_2! {
    atomic_neg_pwr8, [zero = in(reg) 0_u64, out("xer") _,],
    "subc %r9, {zero}, %r7", // r9 = 0 - r7; xer.CA = borrow
    "subfze %r8, %r6",       // r8 = 0 - r6 - xer.CA
}

macro_rules! select_atomic_rmw {
    (
        unsafe fn $name:ident($($arg:tt)*) $(-> $ret_ty:ty)?;
        pwr8 = $pwr8_fn:ident;
        non_seqcst_fallback = $non_seqcst_fallback_fn:ident;
        seqcst_fallback = $seqcst_fallback_fn:ident;
    ) => {
        // If quadword-atomics is available at compile-time, we can always use pwr8_fn.
        #[cfg(any(
            target_feature = "quadword-atomics",
            portable_atomic_target_feature = "quadword-atomics",
        ))]
        use self::$pwr8_fn as $name;
        // Otherwise, we need to do run-time detection and can use pwr8_fn only if quadword-atomics is available.
        #[cfg(not(any(
            target_feature = "quadword-atomics",
            portable_atomic_target_feature = "quadword-atomics",
        )))]
        #[inline]
        unsafe fn $name($($arg)*, order: Ordering) $(-> $ret_ty)? {
            fn_alias! {
                // inline(never) is just a hint and also not strictly necessary
                // because we use ifunc helper macro, but used for clarity.
                #[inline(never)]
                unsafe fn($($arg)*) $(-> $ret_ty)?;
                pwr8_relaxed_fn = $pwr8_fn(Ordering::Relaxed);
                pwr8_acquire_fn = $pwr8_fn(Ordering::Acquire);
                pwr8_release_fn = $pwr8_fn(Ordering::Release);
                pwr8_acqrel_fn = $pwr8_fn(Ordering::AcqRel);
                pwr8_seqcst_fn = $pwr8_fn(Ordering::SeqCst);
            }
            // SAFETY: the caller must uphold the safety contract.
            // we only calls pwr8_fn if quadword-atomics is available.
            unsafe {
                match order {
                    Ordering::Relaxed => {
                        ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                            if detect::detect().quadword_atomics() {
                                pwr8_relaxed_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::Acquire => {
                        ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                            if detect::detect().quadword_atomics() {
                                pwr8_acquire_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::Release => {
                        ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                            if detect::detect().quadword_atomics() {
                                pwr8_release_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::AcqRel => {
                        ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                            if detect::detect().quadword_atomics() {
                                pwr8_acqrel_fn
                            } else {
                                fallback::$non_seqcst_fallback_fn
                            }
                        })
                    }
                    Ordering::SeqCst => {
                        ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                            if detect::detect().quadword_atomics() {
                                pwr8_seqcst_fn
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
    unsafe fn atomic_swap(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_swap_pwr8;
    non_seqcst_fallback = atomic_swap_non_seqcst;
    seqcst_fallback = atomic_swap_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_add(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_add_pwr8;
    non_seqcst_fallback = atomic_add_non_seqcst;
    seqcst_fallback = atomic_add_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_sub(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_sub_pwr8;
    non_seqcst_fallback = atomic_sub_non_seqcst;
    seqcst_fallback = atomic_sub_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_and(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_and_pwr8;
    non_seqcst_fallback = atomic_and_non_seqcst;
    seqcst_fallback = atomic_and_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_nand(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_nand_pwr8;
    non_seqcst_fallback = atomic_nand_non_seqcst;
    seqcst_fallback = atomic_nand_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_or(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_or_pwr8;
    non_seqcst_fallback = atomic_or_non_seqcst;
    seqcst_fallback = atomic_or_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_xor(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_xor_pwr8;
    non_seqcst_fallback = atomic_xor_non_seqcst;
    seqcst_fallback = atomic_xor_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_max(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_max_pwr8;
    non_seqcst_fallback = atomic_max_non_seqcst;
    seqcst_fallback = atomic_max_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umax(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_umax_pwr8;
    non_seqcst_fallback = atomic_umax_non_seqcst;
    seqcst_fallback = atomic_umax_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_min(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_min_pwr8;
    non_seqcst_fallback = atomic_min_non_seqcst;
    seqcst_fallback = atomic_min_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umin(dst: *mut u128, val: u128) -> u128;
    pwr8 = atomic_umin_pwr8;
    non_seqcst_fallback = atomic_umin_non_seqcst;
    seqcst_fallback = atomic_umin_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_not(dst: *mut u128) -> u128;
    pwr8 = atomic_not_pwr8;
    non_seqcst_fallback = atomic_not_non_seqcst;
    seqcst_fallback = atomic_not_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_neg(dst: *mut u128) -> u128;
    pwr8 = atomic_neg_pwr8;
    non_seqcst_fallback = atomic_neg_non_seqcst;
    seqcst_fallback = atomic_neg_seqcst;
}

#[inline]
fn is_lock_free() -> bool {
    #[cfg(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    ))]
    {
        // lqarx and stqcx. instructions are statically available.
        true
    }
    #[cfg(not(any(
        target_feature = "quadword-atomics",
        portable_atomic_target_feature = "quadword-atomics",
    )))]
    {
        detect::detect().quadword_atomics()
    }
}
const IS_ALWAYS_LOCK_FREE: bool = cfg!(any(
    target_feature = "quadword-atomics",
    portable_atomic_target_feature = "quadword-atomics",
));

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
