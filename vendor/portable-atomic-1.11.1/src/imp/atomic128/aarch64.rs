// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementation on AArch64.

This architecture provides the following 128-bit atomic instructions:

- LDXP/STXP: LL/SC (Armv8.0 baseline)
- CASP: CAS (added as Armv8.1 FEAT_LSE (optional from Armv8.0, mandatory from Armv8.1))
- LDP/STP: load/store (if Armv8.4 FEAT_LSE2 (optional from Armv8.2, mandatory from Armv8.4) is available)
- LDIAPP/STILP: acquire-load/release-store (added as Armv8.9 FEAT_LRCPC3 (optional from Armv8.2) (if FEAT_LSE2 is also available))
- LDCLRP/LDSETP/SWPP: fetch-and-{clear,or},swap (added as Armv9.4 FEAT_LSE128 (optional from Armv9.3))

This module supports all of these instructions and attempts to select the best
one based on compile-time and run-time information about available CPU features
and platforms. For example:

- If outline-atomics is not enabled and FEAT_LSE is not available at
  compile-time, we use LDXP/STXP loop.
- If outline-atomics is enabled and FEAT_LSE is not available at
  compile-time, we use CASP for CAS if FEAT_LSE is available
  at run-time, otherwise, use LDXP/STXP loop.
- If FEAT_LSE is available at compile-time, we use CASP for load/store/CAS/RMW.
  However, when portable_atomic_ll_sc_rmw cfg is set, use LDXP/STXP loop instead of CASP
  loop for RMW (by default, it is set on Apple hardware where CASP is slow;
  see build script for details).
- If outline-atomics is enabled and FEAT_LSE2 is not available at compile-time,
  we use LDP/STP (and also LDIAPP/STILP/SWPP if FEAT_LRCPC3/FEAT_LSE128 is
  available) for load/store if FEAT_LSE2 is available at run-time, otherwise,
  use LDXP/STXP or CASP depending on whether FEAT_LSE is available.
- If FEAT_LSE2 is available at compile-time, we use LDP/STP for load/store.
- If FEAT_LSE128 is available at compile-time, we use LDCLRP/LDSETP/SWPP for fetch_and/fetch_or/swap/{release,seqcst}-store.
- If FEAT_LSE2 and FEAT_LRCPC3 are available at compile-time, we use LDIAPP/STILP for acquire-load/release-store.

See each "Instruction selection flow for ..." comment in this file for the exact
instruction selection per operation.

Note: FEAT_LSE2 doesn't imply FEAT_LSE. FEAT_LSE128 implies FEAT_LSE but not FEAT_LSE2.

Note that we do not separate LL and SC into separate functions, but handle
them within a single asm block. This is because it is theoretically possible
for the compiler to insert operations that might clear the reservation between
LL and SC. Considering the type of operations we are providing and the fact
that [progress64](https://github.com/ARM-software/progress64) uses such code,
this is probably not a problem for AArch64, but it seems that AArch64 doesn't
guarantee it and hexagon is the only architecture with hardware guarantees
that such code works. See also:

- https://yarchive.net/comp/linux/cmpxchg_ll_sc_portability.html
- https://lists.llvm.org/pipermail/llvm-dev/2016-May/099490.html
- https://lists.llvm.org/pipermail/llvm-dev/2018-June/123993.html

Also, even when using a CAS loop to implement atomic RMW, include the loop itself
in the asm block because it is more efficient for some codegen backends.
https://github.com/rust-lang/compiler-builtins/issues/339#issuecomment-1191260474

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use intrinsics.rs instead.

Refs:
- Arm A-profile A64 Instruction Set Architecture
  https://developer.arm.com/documentation/ddi0602/2024-12
- Arm Compiler armasm User Guide
  https://developer.arm.com/documentation/dui0801/latest
- Arm Architecture Reference Manual for A-profile architecture
  https://developer.arm.com/documentation/ddi0487/latest (PDF)
- atomic-maybe-uninit https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- aarch64 https://godbolt.org/z/aEWe7zhMh
- aarch64 msvc https://godbolt.org/z/Phq7M6MPs
- aarch64 (+lse) https://godbolt.org/z/9Go3dT6sW
- aarch64 msvc (+lse) https://godbolt.org/z/vGvc6bTMT
- aarch64 (+lse,+lse2) https://godbolt.org/z/KddzqsM9o
- aarch64 (+lse,+lse2,+rcpc3) https://godbolt.org/z/sePheahxh
- aarch64 (+lse2,+lse128) https://godbolt.org/z/WPqM9M1r3
- aarch64 (+lse2,+lse128,+rcpc3) https://godbolt.org/z/5Mf8dc88Y
*/

include!("macros.rs");

// On musl with static linking, it seems that getauxval is not always available.
// See detect/auxv.rs for more.
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(
    test,
    not(all(
        any(target_feature = "lse", portable_atomic_target_feature = "lse"),
        any(target_feature = "lse2", portable_atomic_target_feature = "lse2"),
    )),
))]
#[cfg(any(
    all(
        target_os = "linux",
        any(
            target_env = "gnu",
            all(target_env = "musl", any(not(target_feature = "crt-static"), feature = "std")),
            target_env = "ohos",
            all(target_env = "uclibc", not(target_feature = "crt-static")),
            portable_atomic_outline_atomics,
        ),
    ),
    target_os = "android",
    target_os = "freebsd",
))]
#[path = "../detect/auxv.rs"]
mod detect;
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(
    test,
    not(all(
        any(target_feature = "lse", portable_atomic_target_feature = "lse"),
        any(target_feature = "lse2", portable_atomic_target_feature = "lse2"),
    )),
))]
#[cfg(any(target_os = "netbsd", target_os = "openbsd"))]
#[path = "../detect/aarch64_aa64reg.rs"]
mod detect;
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(test, portable_atomic_outline_atomics))] // TODO(aarch64-illumos): currently disabled by default
#[cfg(any(
    test,
    not(all(
        any(target_feature = "lse2", portable_atomic_target_feature = "lse2"),
        any(target_feature = "lse", portable_atomic_target_feature = "lse"),
    )),
))]
#[cfg(target_os = "illumos")]
#[path = "../detect/aarch64_illumos.rs"]
mod detect;
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(test, not(any(target_feature = "lse", portable_atomic_target_feature = "lse"))))]
#[cfg(target_os = "fuchsia")]
#[path = "../detect/aarch64_fuchsia.rs"]
mod detect;
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(test, not(any(target_feature = "lse", portable_atomic_target_feature = "lse"))))]
#[cfg(windows)]
#[path = "../detect/aarch64_windows.rs"]
mod detect;

// test only
#[cfg(test)]
#[cfg(not(valgrind))]
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(any(target_os = "linux", target_os = "android", target_os = "freebsd"))]
#[path = "../detect/aarch64_aa64reg.rs"]
mod detect_aa64reg;
#[cfg(test)]
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(target_vendor = "apple")]
#[path = "../detect/aarch64_apple.rs"]
mod detect_apple;
#[cfg(test)]
#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(target_os = "openbsd")]
#[path = "../detect/auxv.rs"]
mod detect_auxv;

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::utils::{Pair, U128};

#[cfg(any(
    target_feature = "lse",
    portable_atomic_target_feature = "lse",
    not(portable_atomic_no_outline_atomics),
))]
#[rustfmt::skip]
macro_rules! debug_assert_lse {
    () => {
        #[cfg(all(
            not(portable_atomic_no_outline_atomics),
            any(
                all(
                    target_os = "linux",
                    any(
                        target_env = "gnu",
                        all(
                            target_env = "musl",
                            any(not(target_feature = "crt-static"), feature = "std"),
                        ),
                        target_env = "ohos",
                        all(target_env = "uclibc", not(target_feature = "crt-static")),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                all(target_os = "illumos", portable_atomic_outline_atomics),
                target_os = "fuchsia",
                windows,
            ),
        ))]
        #[cfg(not(any(target_feature = "lse", portable_atomic_target_feature = "lse")))]
        {
            debug_assert!(detect::detect().lse());
        }
    };
}
#[cfg(any(
    target_feature = "lse2",
    portable_atomic_target_feature = "lse2",
    not(portable_atomic_no_outline_atomics),
))]
#[rustfmt::skip]
macro_rules! debug_assert_lse2 {
    () => {
        #[cfg(all(
            not(portable_atomic_no_outline_atomics),
            any(
                all(
                    target_os = "linux",
                    any(
                        target_env = "gnu",
                        all(
                            target_env = "musl",
                            any(not(target_feature = "crt-static"), feature = "std"),
                        ),
                        target_env = "ohos",
                        all(target_env = "uclibc", not(target_feature = "crt-static")),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                all(target_os = "illumos", portable_atomic_outline_atomics),
                // These don't support detection of FEAT_LSE2.
                // target_os = "fuchsia",
                // windows,
            ),
        ))]
        #[cfg(not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")))]
        {
            debug_assert!(detect::detect().lse2());
        }
    };
}
#[cfg(any(
    target_feature = "lse128",
    portable_atomic_target_feature = "lse128",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
#[rustfmt::skip]
macro_rules! debug_assert_lse128 {
    () => {
        #[cfg(all(
            not(portable_atomic_no_outline_atomics),
            any(
                all(
                    target_os = "linux",
                    any(
                        target_env = "gnu",
                        all(
                            target_env = "musl",
                            any(not(target_feature = "crt-static"), feature = "std"),
                        ),
                        target_env = "ohos",
                        all(target_env = "uclibc", not(target_feature = "crt-static")),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                all(target_os = "illumos", portable_atomic_outline_atomics),
                // These don't support detection of FEAT_LSE128.
                // target_os = "fuchsia",
                // windows,
            ),
        ))]
        #[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
        {
            debug_assert!(detect::detect().lse128());
        }
    };
}
#[cfg(any(
    target_feature = "rcpc3",
    portable_atomic_target_feature = "rcpc3",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
#[rustfmt::skip]
macro_rules! debug_assert_rcpc3 {
    () => {
        #[cfg(all(
            not(portable_atomic_no_outline_atomics),
            any(
                all(
                    target_os = "linux",
                    any(
                        target_env = "gnu",
                        all(
                            target_env = "musl",
                            any(not(target_feature = "crt-static"), feature = "std"),
                        ),
                        target_env = "ohos",
                        all(target_env = "uclibc", not(target_feature = "crt-static")),
                        portable_atomic_outline_atomics,
                    ),
                ),
                target_os = "android",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd",
                all(target_os = "illumos", portable_atomic_outline_atomics),
                // These don't support detection of FEAT_LRCPC3.
                // target_os = "fuchsia",
                // windows,
            ),
        ))]
        #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
        {
            debug_assert!(detect::detect().rcpc3());
        }
    };
}

// Refs: https://developer.arm.com/documentation/100067/0611/armclang-Integrated-Assembler/AArch32-Target-selection-directives
//
// This is similar to #[target_feature(enable = "lse")], except that there are
// no compiler guarantees regarding (un)inlining, and the scope is within an asm
// block rather than a function. We use this directive because #[target_feature(enable = "lse")]
// is unstable on pre-1.61 rustc and incompatible with rustc_codegen_cranelift:
// https://github.com/rust-lang/rustc_codegen_cranelift/issues/1400#issuecomment-1774599775
//
// The .arch_extension directive in asm! is effective until the end of the assembly block and
// is not propagated to subsequent code, so the end_lse macro is unneeded.
// https://godbolt.org/z/o6EPndP94
// https://github.com/torvalds/linux/commit/e0d5896bd356cd577f9710a02d7a474cdf58426b
// https://github.com/torvalds/linux/commit/dd1f6308b28edf0452dd5dc7877992903ec61e69
// (It seems GCC effectively ignores this directive and always allow FEAT_LSE instructions: https://godbolt.org/z/W9W6rensG)
// Note that the .arch_extension directive in global_asm!/naked_asm! which are
// not used in this crate has different behavior: https://github.com/rust-lang/rust/pull/137720#discussion_r1973608259
//
// The .arch directive has a similar effect, but we don't use it due to the following issue:
// https://github.com/torvalds/linux/commit/dd1f6308b28edf0452dd5dc7877992903ec61e69
//
// Note: If FEAT_LSE is not available at compile-time, we must guarantee that
// the function that uses it is not inlined into a function where it is not
// clear whether FEAT_LSE is available. Otherwise, (even if we checked whether
// FEAT_LSE is available at run-time) optimizations that reorder its
// instructions across the if condition might introduce undefined behavior.
// (see also https://rust-lang.github.io/rfcs/2045-target-feature.html#safely-inlining-target_feature-functions-on-more-contexts)
// However, our code uses the ifunc helper macro that works with function pointers,
// so we don't have to worry about this unless calling without helper macro.
#[cfg(any(
    target_feature = "lse",
    portable_atomic_target_feature = "lse",
    not(portable_atomic_no_outline_atomics),
))]
macro_rules! start_lse {
    () => {
        ".arch_extension lse"
    };
}
#[cfg(not(portable_atomic_pre_llvm_16))]
#[cfg(any(
    target_feature = "lse128",
    portable_atomic_target_feature = "lse128",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
macro_rules! start_lse128 {
    () => {
        ".arch_extension lse128"
    };
}
#[cfg(not(portable_atomic_pre_llvm_16))]
#[cfg(any(
    target_feature = "rcpc3",
    portable_atomic_target_feature = "rcpc3",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
macro_rules! start_rcpc3 {
    () => {
        ".arch_extension rcpc3"
    };
}

#[cfg(target_endian = "little")]
macro_rules! select_le_or_be {
    ($le:expr, $be:expr) => {
        $le
    };
}
#[cfg(target_endian = "big")]
macro_rules! select_le_or_be {
    ($le:expr, $be:expr) => {
        $be
    };
}

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
#[cfg(portable_atomic_pre_llvm_16)]
#[cfg(any(
    target_feature = "lse128",
    portable_atomic_target_feature = "lse128",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
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

// -----------------------------------------------------------------------------
// load

/*

Instruction selection flow for load:
- if compile_time(FEAT_LSE2) => ldp:
  - if compile_time(FEAT_LRCPC3) && order != relaxed => ldiapp
  - else => ldp
- if platform_supports_detection_of(FEAT_LSE2):
  - if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (ldiapp)
  - if detect(FEAT_LSE2) => lse2 (ldp)
- else => no_lse2:
  - if compile_time(FEAT_LSE) => casp
  - else => ldxp_stxp

Note:
- If FEAT_LSE2 is available at compile-time, we don't do run-time detection of
  FEAT_LRCPC3 at this time, since FEAT_LRCPC3 is not yet available for most CPUs.
  (macOS that doesn't have any FEAT_LRCPC3-enabled CPUs as of M4 is only a platform
  that currently enables FEAT_LSE2 at compile-time by default.)
- If FEAT_LSE2 is not available at compile-time, we want to do run-time detection
  of FEAT_LSE2, so we do run-time detection of FEAT_LRCPC3 at the same time.
- We don't do run-time detection of FEAT_LSE for load at this time, but since
  load by CAS is wait-free, it would probably make sense to do run-time detection. (TODO)

*/

// if compile_time(FEAT_LSE2) => ldp:
// cfg guarantee that the CPU supports FEAT_LSE2.
#[cfg(any(target_feature = "lse2", portable_atomic_target_feature = "lse2"))]
use self::_atomic_load_ldp as atomic_load;
#[cfg(not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")))]
#[inline]
unsafe fn atomic_load(src: *mut u128, order: Ordering) -> u128 {
    #[inline]
    unsafe fn atomic_load_no_lse2(src: *mut u128, order: Ordering) -> u128 {
        // if compile_time(FEAT_LSE) => casp
        #[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
        // SAFETY: the caller must uphold the safety contract.
        // cfg guarantee that the CPU supports FEAT_LSE.
        unsafe {
            _atomic_load_casp(src, order)
        }
        // else => ldxp_stxp
        #[cfg(not(any(target_feature = "lse", portable_atomic_target_feature = "lse")))]
        // SAFETY: the caller must uphold the safety contract.
        unsafe {
            _atomic_load_ldxp_stxp(src, order)
        }
    }
    // if platform_supports_detection_of(FEAT_LSE2):
    #[cfg(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            // These don't support detection of FEAT_LSE2.
            // target_os = "fuchsia",
            // windows,
        ),
    ))]
    {
        fn_alias! {
            // inline(never) is just a hint and also not strictly necessary
            // because we use ifunc helper macro, but used for clarity.
            #[inline(never)]
            unsafe fn(src: *mut u128) -> u128;
            atomic_load_lse2_relaxed = _atomic_load_ldp(Ordering::Relaxed);
            atomic_load_lse2_acquire = _atomic_load_ldp(Ordering::Acquire);
            atomic_load_lse2_seqcst = _atomic_load_ldp(Ordering::SeqCst);
            atomic_load_lse2_rcpc3_acquire = _atomic_load_ldiapp(Ordering::Acquire);
            atomic_load_lse2_rcpc3_seqcst = _atomic_load_ldiapp(Ordering::SeqCst);
        }
        fn_alias! {
            unsafe fn(src: *mut u128) -> u128;
            atomic_load_no_lse2_relaxed = atomic_load_no_lse2(Ordering::Relaxed);
            atomic_load_no_lse2_acquire = atomic_load_no_lse2(Ordering::Acquire);
            atomic_load_no_lse2_seqcst = atomic_load_no_lse2(Ordering::SeqCst);
        }
        // SAFETY: the caller must uphold the safety contract.
        // and we've checked if FEAT_LSE2/FEAT_LRCPC3 is available.
        unsafe {
            match order {
                Ordering::Relaxed => {
                    ifunc!(unsafe fn(src: *mut u128) -> u128 {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            // if detect(FEAT_LSE2) => lse2 (ldp)
                            atomic_load_lse2_relaxed
                        } else {
                            // else => no_lse2:
                            atomic_load_no_lse2_relaxed
                        }
                    })
                }
                Ordering::Acquire => {
                    ifunc!(unsafe fn(src: *mut u128) -> u128 {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            if cpuinfo.rcpc3() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (ldiapp)
                                atomic_load_lse2_rcpc3_acquire
                            } else {
                                // if detect(FEAT_LSE2) => lse2 (ldp)
                                atomic_load_lse2_acquire
                            }
                        } else {
                            // else => no_lse2:
                            atomic_load_no_lse2_acquire
                        }
                    })
                }
                Ordering::SeqCst => {
                    ifunc!(unsafe fn(src: *mut u128) -> u128 {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            if cpuinfo.rcpc3() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (ldiapp)
                                atomic_load_lse2_rcpc3_seqcst
                            } else {
                                // if detect(FEAT_LSE2) => lse2 (ldp)
                                atomic_load_lse2_seqcst
                            }
                        } else {
                            // else => no_lse2:
                            atomic_load_no_lse2_seqcst
                        }
                    })
                }
                _ => unreachable!(),
            }
        }
    }
    // else => no_lse2:
    #[cfg(not(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            // These don't support detection of FEAT_LSE2.
            // target_os = "fuchsia",
            // windows,
        ),
    )))]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        atomic_load_no_lse2(src, order)
    }
}
// If CPU supports FEAT_LSE2, LDP/LDIAPP is single-copy atomic reads,
// otherwise it is two single-copy atomic reads.
// Refs: B2.2.1 of the Arm Architecture Reference Manual Armv8, for Armv8-A architecture profile
#[cfg(any(
    target_feature = "lse2",
    portable_atomic_target_feature = "lse2",
    not(portable_atomic_no_outline_atomics),
))]
#[inline]
unsafe fn _atomic_load_ldp(src: *mut u128, order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_lse2!();

    // SAFETY: the caller must guarantee that `dst` is valid for reads,
    // 16-byte aligned, that there are no concurrent non-atomic operations.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDP--Load-pair-of-registers-
    unsafe {
        let (out_lo, out_hi);
        macro_rules! atomic_load_relaxed {
            ($acquire:tt) => {{
                asm!(
                    "ldp {out_lo}, {out_hi}, [{src}]",
                    $acquire,
                    src = in(reg) ptr_reg!(src),
                    out_hi = lateout(reg) out_hi,
                    out_lo = lateout(reg) out_lo,
                    options(nostack, preserves_flags),
                );
                U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
            }};
        }
        match order {
            // if compile_time(FEAT_LRCPC3) && order != relaxed => ldiapp
            // SAFETY: cfg guarantee that the CPU supports FEAT_LRCPC3.
            #[cfg(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3"))]
            Ordering::Acquire | Ordering::SeqCst => _atomic_load_ldiapp(src, order),

            // else => ldp
            Ordering::Relaxed => atomic_load_relaxed!(""),
            #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
            Ordering::Acquire => atomic_load_relaxed!("dmb ishld"),
            #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
            Ordering::SeqCst => {
                asm!(
                    // ldar (or dmb ishld) is required to prevent reordering with preceding stlxp.
                    // See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108891 for details.
                    "ldar {tmp}, [{src}]",
                    "ldp {out_lo}, {out_hi}, [{src}]",
                    "dmb ishld",
                    src = in(reg) ptr_reg!(src),
                    out_hi = lateout(reg) out_hi,
                    out_lo = lateout(reg) out_lo,
                    tmp = out(reg) _,
                    options(nostack, preserves_flags),
                );
                U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
            }
            _ => unreachable!(),
        }
    }
}
#[cfg(any(
    target_feature = "lse2",
    portable_atomic_target_feature = "lse2",
    not(portable_atomic_no_outline_atomics),
))]
#[cfg(any(
    target_feature = "rcpc3",
    portable_atomic_target_feature = "rcpc3",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
#[inline]
unsafe fn _atomic_load_ldiapp(src: *mut u128, order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_lse2!();
    debug_assert_rcpc3!();

    // SAFETY: the caller must guarantee that `dst` is valid for reads,
    // 16-byte aligned, that there are no concurrent non-atomic operations.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDIAPP--Load-Acquire-RCpc-ordered-pair-of-registers-
    unsafe {
        let (out_lo, out_hi);
        match order {
            Ordering::Acquire => {
                #[cfg(not(portable_atomic_pre_llvm_16))]
                asm!(
                    start_rcpc3!(),
                    "ldiapp {out_lo}, {out_hi}, [{src}]",
                    src = in(reg) ptr_reg!(src),
                    out_hi = lateout(reg) out_hi,
                    out_lo = lateout(reg) out_lo,
                    options(nostack, preserves_flags),
                );
                // LLVM supports FEAT_LRCPC3 instructions on LLVM 16+, so use .inst directive on old LLVM.
                // https://github.com/llvm/llvm-project/commit/a6aaa969f7caec58a994142f8d855861cf3a1463
                #[cfg(portable_atomic_pre_llvm_16)]
                asm!(
                    // ldiapp x0, x1, [x0]
                    ".inst 0xd9411800",
                    in("x0") ptr_reg!(src),
                    lateout("x1") out_hi,
                    lateout("x0") out_lo,
                    options(nostack, preserves_flags),
                );
            }
            Ordering::SeqCst => {
                #[cfg(not(portable_atomic_pre_llvm_16))]
                asm!(
                    start_rcpc3!(),
                    // ldar (or dmb ishld) is required to prevent reordering with preceding stlxp.
                    // See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108891 for details.
                    "ldar {tmp}, [{src}]",
                    "ldiapp {out_lo}, {out_hi}, [{src}]",
                    src = in(reg) ptr_reg!(src),
                    out_hi = lateout(reg) out_hi,
                    out_lo = lateout(reg) out_lo,
                    tmp = out(reg) _,
                    options(nostack, preserves_flags),
                );
                // LLVM supports FEAT_LRCPC3 instructions on LLVM 16+, so use .inst directive on old LLVM.
                // https://github.com/llvm/llvm-project/commit/a6aaa969f7caec58a994142f8d855861cf3a1463
                #[cfg(portable_atomic_pre_llvm_16)]
                asm!(
                    // ldar (or dmb ishld) is required to prevent reordering with preceding stlxp.
                    // See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=108891 for details.
                    "ldar {tmp}, [x0]",
                    // ldiapp x0, x1, [x0]
                    ".inst 0xd9411800",
                    tmp = out(reg) _,
                    in("x0") ptr_reg!(src),
                    lateout("x1") out_hi,
                    lateout("x0") out_lo,
                    options(nostack, preserves_flags),
                );
            }
            _ => unreachable!(),
        }
        U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}
// Do not use _atomic_compare_exchange_casp because it needs extra MOV to implement load.
#[cfg(any(test, not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2"))))]
#[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
#[inline]
unsafe fn _atomic_load_casp(src: *mut u128, order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_lse!();

    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantee that the CPU supports FEAT_LSE.
    unsafe {
        let (out_lo, out_hi);
        macro_rules! atomic_load {
            ($acquire:tt, $release:tt) => {
                asm!(
                    start_lse!(),
                    concat!("casp", $acquire, $release, " x2, x3, x2, x3, [{src}]"),
                    src = in(reg) ptr_reg!(src),
                    // must be allocated to even/odd register pair
                    inout("x2") 0_u64 => out_lo,
                    inout("x3") 0_u64 => out_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        match order {
            Ordering::Relaxed => atomic_load!("", ""),
            Ordering::Acquire => atomic_load!("a", ""),
            Ordering::SeqCst => atomic_load!("a", "l"),
            _ => unreachable!(),
        }
        U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}
#[cfg(any(
    test,
    all(
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
        not(any(target_feature = "lse", portable_atomic_target_feature = "lse")),
    ),
))]
#[inline]
unsafe fn _atomic_load_ldxp_stxp(src: *mut u128, order: Ordering) -> u128 {
    debug_assert!(src as usize % 16 == 0);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let (mut out_lo, mut out_hi);
        macro_rules! atomic_load {
            ($acquire:tt, $release:tt) => {
                asm!(
                    "2:",
                        concat!("ld", $acquire, "xp {out_lo}, {out_hi}, [{src}]"),
                        concat!("st", $release, "xp {r:w}, {out_lo}, {out_hi}, [{src}]"),
                        // 0 if the store was successful, 1 if no store was performed
                        "cbnz {r:w}, 2b",
                    src = in(reg) ptr_reg!(src),
                    out_lo = out(reg) out_lo,
                    out_hi = out(reg) out_hi,
                    r = out(reg) _,
                    options(nostack, preserves_flags),
                )
            };
        }
        match order {
            Ordering::Relaxed => atomic_load!("", ""),
            Ordering::Acquire => atomic_load!("a", ""),
            Ordering::SeqCst => atomic_load!("a", "l"),
            _ => unreachable!(),
        }
        U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}

// -----------------------------------------------------------------------------
// store

/*

Instruction selection flow for store:
- if compile_time(FEAT_LSE2) => stp:
  - if compile_time(FEAT_LSE128) && order == seqcst => swpp
  - if compile_time(FEAT_LRCPC3) && order != relaxed => stilp
  - if compile_time(FEAT_LSE128) && order != relaxed => swpp
  - else => stp
- if platform_supports_detection_of(FEAT_LSE2):
  - if detect(FEAT_LSE2) && detect(FEAT_LSE128) && order == seqcst => lse128 (swpp)
  - if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (stilp)
  - if detect(FEAT_LSE2) && detect(FEAT_LSE128) && order != relaxed => lse128 (swpp)
  - if detect(FEAT_LSE2) => lse2 (stp)
- else => no_lse2:
  - if compile_time(FEAT_LSE) && not(ll_sc_rmw) => casp
  - else => ldxp_stxp

Note:
- If FEAT_LSE2 is available at compile-time, we don't do run-time detection of
  FEAT_LRCPC3/FEAT_LSE128 at this time, since FEAT_LRCPC3/FEAT_LSE128 is not yet available for most CPUs.
  (macOS that doesn't have any FEAT_LRCPC3/FEAT_LSE128-enabled CPUs as of M4 is only a platform
  that currently enables FEAT_LSE2 at compile-time by default.)
- If FEAT_LSE2 is not available at compile-time, we want to do run-time detection
  of FEAT_LSE2, so we do run-time detection of FEAT_LRCPC3/FEAT_LSE128 at the same time.
- We don't do run-time detection of FEAT_LSE for store at this time.

*/

// if compile_time(FEAT_LSE2) => stp:
// cfg guarantee that the CPU supports FEAT_LSE2.
#[cfg(any(target_feature = "lse2", portable_atomic_target_feature = "lse2"))]
use self::_atomic_store_stp as atomic_store;
#[cfg(not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")))]
#[inline]
unsafe fn atomic_store(dst: *mut u128, val: u128, order: Ordering) {
    #[inline]
    unsafe fn atomic_store_no_lse2(dst: *mut u128, val: u128, order: Ordering) {
        // if compile_time(FEAT_LSE) && not(ll_sc_rmw) => casp
        // If FEAT_LSE is available at compile-time and portable_atomic_ll_sc_rmw cfg is not set,
        // we use CAS-based atomic RMW.
        #[cfg(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        ))]
        // SAFETY: the caller must uphold the safety contract.
        // cfg guarantee that the CPU supports FEAT_LSE.
        unsafe {
            _atomic_swap_casp(dst, val, order);
        }
        // else => ldxp_stxp
        #[cfg(not(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        )))]
        // SAFETY: the caller must uphold the safety contract.
        unsafe {
            _atomic_store_ldxp_stxp(dst, val, order);
        }
    }
    #[cfg(any(
        target_feature = "lse128",
        portable_atomic_target_feature = "lse128",
        all(
            not(portable_atomic_no_outline_atomics),
            not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
        ),
    ))]
    #[inline]
    unsafe fn _atomic_store_swpp(dst: *mut u128, val: u128, order: Ordering) {
        // SAFETY: the caller must uphold the safety contract.
        unsafe {
            _atomic_swap_swpp(dst, val, order);
        }
    }
    // if platform_supports_detection_of(FEAT_LSE2):
    #[cfg(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            // These don't support detection of FEAT_LSE2.
            // target_os = "fuchsia",
            // windows,
        ),
    ))]
    {
        fn_alias! {
            // inline(never) is just a hint and also not strictly necessary
            // because we use ifunc helper macro, but used for clarity.
            #[inline(never)]
            unsafe fn(dst: *mut u128, val: u128);
            atomic_store_lse2_relaxed = _atomic_store_stp(Ordering::Relaxed);
            atomic_store_lse2_release = _atomic_store_stp(Ordering::Release);
            atomic_store_lse2_seqcst = _atomic_store_stp(Ordering::SeqCst);
            atomic_store_lse2_rcpc3_release = _atomic_store_stilp(Ordering::Release);
            atomic_store_lse2_rcpc3_seqcst = _atomic_store_stilp(Ordering::SeqCst);
            atomic_store_lse128_release = _atomic_store_swpp(Ordering::Release);
            atomic_store_lse128_seqcst = _atomic_store_swpp(Ordering::SeqCst);
        }
        fn_alias! {
            unsafe fn(dst: *mut u128, val: u128);
            atomic_store_no_lse2_relaxed = atomic_store_no_lse2(Ordering::Relaxed);
            atomic_store_no_lse2_release = atomic_store_no_lse2(Ordering::Release);
            atomic_store_no_lse2_seqcst = atomic_store_no_lse2(Ordering::SeqCst);
        }
        // SAFETY: the caller must uphold the safety contract.
        // and we've checked if FEAT_LSE2/FEAT_LRCPC3/FEAT_LSE128 is available.
        unsafe {
            match order {
                Ordering::Relaxed => {
                    ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            // if detect(FEAT_LSE2) => lse2 (stp)
                            atomic_store_lse2_relaxed
                        } else {
                            // else => no_lse2:
                            atomic_store_no_lse2_relaxed
                        }
                    });
                }
                Ordering::Release => {
                    ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            if cpuinfo.rcpc3() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (stilp)
                                atomic_store_lse2_rcpc3_release
                            } else if cpuinfo.lse128() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LSE128) && order != relaxed => lse128 (swpp)
                                atomic_store_lse128_release
                            } else {
                                // if detect(FEAT_LSE2) => lse2 (stp)
                                atomic_store_lse2_release
                            }
                        } else {
                            // else => no_lse2:
                            atomic_store_no_lse2_release
                        }
                    });
                }
                Ordering::SeqCst => {
                    ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                        let cpuinfo = detect::detect();
                        if cpuinfo.lse2() {
                            if cpuinfo.lse128() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LSE128) && order == seqcst => lse128 (swpp)
                                atomic_store_lse128_seqcst
                            } else if cpuinfo.rcpc3() {
                                // if detect(FEAT_LSE2) && detect(FEAT_LRCPC3) && order != relaxed => lse2_rcpc3 (stilp)
                                atomic_store_lse2_rcpc3_seqcst
                            } else {
                                // if detect(FEAT_LSE2) => lse2 (stp)
                                atomic_store_lse2_seqcst
                            }
                        } else {
                            // else => no_lse2:
                            atomic_store_no_lse2_seqcst
                        }
                    });
                }
                _ => unreachable!(),
            }
        }
    }
    // else => no_lse2:
    #[cfg(not(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            // These don't support detection of FEAT_LSE2.
            // target_os = "fuchsia",
            // windows,
        ),
    )))]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        atomic_store_no_lse2(dst, val, order);
    }
}
// If CPU supports FEAT_LSE2, STP/STILP is single-copy atomic writes,
// otherwise it is two single-copy atomic writes.
// Refs: B2.2.1 of the Arm Architecture Reference Manual Armv8, for Armv8-A architecture profile
#[cfg(any(
    target_feature = "lse2",
    portable_atomic_target_feature = "lse2",
    not(portable_atomic_no_outline_atomics),
))]
#[inline]
unsafe fn _atomic_store_stp(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_lse2!();

    // SAFETY: the caller must guarantee that `dst` is valid for writes,
    // 16-byte aligned, that there are no concurrent non-atomic operations.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STP--Store-pair-of-registers-
    unsafe {
        macro_rules! atomic_store {
            ($acquire:tt, $release:tt) => {{
                let val = U128 { whole: val };
                asm!(
                    $release,
                    "stp {val_lo}, {val_hi}, [{dst}]",
                    $acquire,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = in(reg) val.pair.lo,
                    val_hi = in(reg) val.pair.hi,
                    options(nostack, preserves_flags),
                );
            }};
        }
        match order {
            // if compile_time(FEAT_LSE128) && order == seqcst => swpp
            // Prefer swpp if stp requires fences. https://reviews.llvm.org/D143506
            // SAFETY: cfg guarantee that the CPU supports FEAT_LSE128.
            #[cfg(any(target_feature = "lse128", portable_atomic_target_feature = "lse128"))]
            Ordering::SeqCst => {
                _atomic_swap_swpp(dst, val, order);
            }

            // if compile_time(FEAT_LRCPC3) && order != relaxed => stilp
            // SAFETY: cfg guarantee that the CPU supports FEAT_LRCPC3.
            #[cfg(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3"))]
            Ordering::Release => _atomic_store_stilp(dst, val, order),
            #[cfg(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3"))]
            #[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
            Ordering::SeqCst => _atomic_store_stilp(dst, val, order),

            // if compile_time(FEAT_LSE128) && order != relaxed => swpp
            // Prefer swpp if stp requires fences. https://reviews.llvm.org/D143506
            // SAFETY: cfg guarantee that the CPU supports FEAT_LSE128.
            #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
            #[cfg(any(target_feature = "lse128", portable_atomic_target_feature = "lse128"))]
            Ordering::Release => {
                _atomic_swap_swpp(dst, val, order);
            }

            // else => stp
            Ordering::Relaxed => atomic_store!("", ""),
            #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
            #[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
            Ordering::Release => atomic_store!("", "dmb ish"),
            #[cfg(not(any(target_feature = "rcpc3", portable_atomic_target_feature = "rcpc3")))]
            #[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
            Ordering::SeqCst => atomic_store!("dmb ish", "dmb ish"),
            _ => unreachable!(),
        }
    }
}
#[cfg(any(
    target_feature = "lse2",
    portable_atomic_target_feature = "lse2",
    not(portable_atomic_no_outline_atomics),
))]
#[cfg(any(
    target_feature = "rcpc3",
    portable_atomic_target_feature = "rcpc3",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
#[inline]
unsafe fn _atomic_store_stilp(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_lse2!();
    debug_assert_rcpc3!();

    // SAFETY: the caller must guarantee that `dst` is valid for writes,
    // 16-byte aligned, that there are no concurrent non-atomic operations.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STILP--Store-release-ordered-pair-of-registers-
    unsafe {
        macro_rules! atomic_store {
            ($acquire:tt) => {{
                let val = U128 { whole: val };
                #[cfg(not(portable_atomic_pre_llvm_16))]
                asm!(
                    start_rcpc3!(),
                    "stilp {val_lo}, {val_hi}, [{dst}]",
                    $acquire,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = in(reg) val.pair.lo,
                    val_hi = in(reg) val.pair.hi,
                    options(nostack, preserves_flags),
                );
                // LLVM supports FEAT_LRCPC3 instructions on LLVM 16+, so use .inst directive on old LLVM.
                // https://github.com/llvm/llvm-project/commit/a6aaa969f7caec58a994142f8d855861cf3a1463
                #[cfg(portable_atomic_pre_llvm_16)]
                asm!(
                    // stilp x2, x3, [x0]
                    ".inst 0xd9031802",
                    $acquire,
                    in("x0") ptr_reg!(dst),
                    in("x2") val.pair.lo,
                    in("x3") val.pair.hi,
                    options(nostack, preserves_flags),
                );
            }};
        }
        match order {
            Ordering::Release => atomic_store!(""),
            // LLVM uses store-release (dmb ish; stp); dmb ish, GCC (libatomic)
            // uses store-release (stilp) without fence for SeqCst store
            // (https://github.com/gcc-mirror/gcc/commit/7107574958e2bed11d916a1480ef1319f15e5ffe).
            // Considering https://reviews.llvm.org/D141748, LLVM's lowing seems
            // to be the safer option here (I'm not convinced that the libatomic's implementation is wrong).
            Ordering::SeqCst => atomic_store!("dmb ish"),
            _ => unreachable!(),
        }
    }
}
// Do not use _atomic_swap_ldxp_stxp because it needs extra registers to implement store.
#[cfg(any(
    test,
    not(all(
        any(target_feature = "lse", portable_atomic_target_feature = "lse"),
        not(portable_atomic_ll_sc_rmw),
    ))
))]
#[inline]
unsafe fn _atomic_store_ldxp_stxp(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let val = U128 { whole: val };
        macro_rules! store {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    "2:",
                        concat!("ld", $acquire, "xp xzr, {tmp}, [{dst}]"),
                        concat!("st", $release, "xp {tmp:w}, {val_lo}, {val_hi}, [{dst}]"),
                        // 0 if the store was successful, 1 if no store was performed
                        "cbnz {tmp:w}, 2b",
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = in(reg) val.pair.lo,
                    val_hi = in(reg) val.pair.hi,
                    tmp = out(reg) _,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_rmw!(store, order);
    }
}

// -----------------------------------------------------------------------------
// compare_exchange

/*

Instruction selection flow for compare_exchange:
- if compile_time(FEAT_LSE) => casp
- if platform_supports_detection_of(FEAT_LSE):
  - if detect(FEAT_LSE) => casp
- else => ldxp_stxp

*/

#[inline]
unsafe fn atomic_compare_exchange(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> Result<u128, u128> {
    // if compile_time(FEAT_LSE) => casp
    #[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantee that the CPU supports FEAT_LSE.
    let prev = unsafe { _atomic_compare_exchange_casp(dst, old, new, success, failure) };
    // if platform_supports_detection_of(FEAT_LSE):
    #[cfg(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            target_os = "fuchsia",
            windows,
        ),
    ))]
    #[cfg(not(any(target_feature = "lse", portable_atomic_target_feature = "lse")))]
    let prev = {
        fn_alias! {
            // inline(never) is just a hint and also not strictly necessary
            // because we use ifunc helper macro, but used for clarity.
            #[inline(never)]
            unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128;
            atomic_compare_exchange_casp_relaxed
                = _atomic_compare_exchange_casp(Ordering::Relaxed, Ordering::Relaxed);
            atomic_compare_exchange_casp_acquire
                = _atomic_compare_exchange_casp(Ordering::Acquire, Ordering::Acquire);
            atomic_compare_exchange_casp_release
                = _atomic_compare_exchange_casp(Ordering::Release, Ordering::Relaxed);
            atomic_compare_exchange_casp_acqrel
                = _atomic_compare_exchange_casp(Ordering::AcqRel, Ordering::Acquire);
            // AcqRel and SeqCst RMWs are equivalent in non-MSVC environments.
            #[cfg(target_env = "msvc")]
            atomic_compare_exchange_casp_seqcst
                = _atomic_compare_exchange_casp(Ordering::SeqCst, Ordering::SeqCst);
        }
        fn_alias! {
            unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128;
            atomic_compare_exchange_ldxp_stxp_relaxed
                = _atomic_compare_exchange_ldxp_stxp(Ordering::Relaxed, Ordering::Relaxed);
            atomic_compare_exchange_ldxp_stxp_acquire
                = _atomic_compare_exchange_ldxp_stxp(Ordering::Acquire, Ordering::Acquire);
            atomic_compare_exchange_ldxp_stxp_release
                = _atomic_compare_exchange_ldxp_stxp(Ordering::Release, Ordering::Relaxed);
            atomic_compare_exchange_ldxp_stxp_acqrel
                = _atomic_compare_exchange_ldxp_stxp(Ordering::AcqRel, Ordering::Acquire);
            // AcqRel and SeqCst RMWs are equivalent in non-MSVC environments.
            #[cfg(target_env = "msvc")]
            atomic_compare_exchange_ldxp_stxp_seqcst
                = _atomic_compare_exchange_ldxp_stxp(Ordering::SeqCst, Ordering::SeqCst);
        }
        // SAFETY: the caller must guarantee that `dst` is valid for both writes and
        // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
        // and we've checked if FEAT_LSE is available.
        unsafe {
            let success = crate::utils::upgrade_success_ordering(success, failure);
            match success {
                Ordering::Relaxed => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_relaxed
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_relaxed
                        }
                    })
                }
                Ordering::Acquire => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_acquire
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_acquire
                        }
                    })
                }
                Ordering::Release => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_release
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_release
                        }
                    })
                }
                // AcqRel and SeqCst RMWs are equivalent in both implementations in non-MSVC environments.
                #[cfg(not(target_env = "msvc"))]
                Ordering::AcqRel | Ordering::SeqCst => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_acqrel
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_acqrel
                        }
                    })
                }
                #[cfg(target_env = "msvc")]
                Ordering::AcqRel => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_acqrel
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_acqrel
                        }
                    })
                }
                #[cfg(target_env = "msvc")]
                Ordering::SeqCst => {
                    ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> u128 {
                        if detect::detect().lse() {
                            // if detect(FEAT_LSE) => casp
                            atomic_compare_exchange_casp_seqcst
                        } else {
                            // else => ldxp_stxp
                            atomic_compare_exchange_ldxp_stxp_seqcst
                        }
                    })
                }
                _ => unreachable!(),
            }
        }
    };
    // else => ldxp_stxp
    #[cfg(not(all(
        not(portable_atomic_no_outline_atomics),
        any(
            all(
                target_os = "linux",
                any(
                    target_env = "gnu",
                    all(
                        target_env = "musl",
                        any(not(target_feature = "crt-static"), feature = "std"),
                    ),
                    target_env = "ohos",
                    all(target_env = "uclibc", not(target_feature = "crt-static")),
                    portable_atomic_outline_atomics,
                ),
            ),
            target_os = "android",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
            all(target_os = "illumos", portable_atomic_outline_atomics),
            target_os = "fuchsia",
            windows,
        ),
    )))]
    #[cfg(not(any(target_feature = "lse", portable_atomic_target_feature = "lse")))]
    // SAFETY: the caller must uphold the safety contract.
    let prev = unsafe { _atomic_compare_exchange_ldxp_stxp(dst, old, new, success, failure) };
    if prev == old { Ok(prev) } else { Err(prev) }
}
#[cfg(any(
    target_feature = "lse",
    portable_atomic_target_feature = "lse",
    not(portable_atomic_no_outline_atomics),
))]
#[inline]
unsafe fn _atomic_compare_exchange_casp(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_lse!();
    let order = crate::utils::upgrade_success_ordering(success, failure);

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
    // and the CPU supports FEAT_LSE.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/CASP--CASPA--CASPAL--CASPL--Compare-and-swap-pair-of-words-or-doublewords-in-memory-
    unsafe {
        let old = U128 { whole: old };
        let new = U128 { whole: new };
        let (prev_lo, prev_hi);
        macro_rules! cmpxchg {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    start_lse!(),
                    concat!("casp", $acquire, $release, " x6, x7, x4, x5, [{dst}]"),
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    // must be allocated to even/odd register pair
                    inout("x6") old.pair.lo => prev_lo,
                    inout("x7") old.pair.hi => prev_hi,
                    // must be allocated to even/odd register pair
                    in("x4") new.pair.lo,
                    in("x5") new.pair.hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_rmw!(cmpxchg, order, write = success);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}
#[cfg(any(test, not(any(target_feature = "lse", portable_atomic_target_feature = "lse"))))]
#[inline]
unsafe fn _atomic_compare_exchange_ldxp_stxp(
    dst: *mut u128,
    old: u128,
    new: u128,
    success: Ordering,
    failure: Ordering,
) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    let order = crate::utils::upgrade_success_ordering(success, failure);

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, and that there are no concurrent non-atomic operations.
    //
    // Refs:
    // - LDXP: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDXP--Load-exclusive-pair-of-registers-
    // - LDAXP: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDAXP--Load-acquire-exclusive-pair-of-registers-
    // - STXP: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STXP--Store-exclusive-pair-of-registers-
    // - STLXP: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/STLXP--Store-release-exclusive-pair-of-registers-
    //
    // Note: Load-Exclusive pair (by itself) does not guarantee atomicity; to complete an atomic
    // operation (even load/store), a corresponding Store-Exclusive pair must succeed.
    // See Arm Architecture Reference Manual for A-profile architecture
    // Section B2.2.1 "Requirements for single-copy atomicity", and
    // Section B2.9 "Synchronization and semaphores" for more.
    unsafe {
        let old = U128 { whole: old };
        let new = U128 { whole: new };
        let (mut prev_lo, mut prev_hi);
        macro_rules! cmpxchg {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    "2:",
                        concat!("ld", $acquire, "xp {prev_lo}, {prev_hi}, [{dst}]"),
                        "cmp {prev_lo}, {old_lo}",
                        "cset {r:w}, ne",
                        "cmp {prev_hi}, {old_hi}",
                        "cinc {r:w}, {r:w}, ne",
                        "cbz {r:w}, 3f",
                        concat!("st", $release, "xp {r:w}, {prev_lo}, {prev_hi}, [{dst}]"),
                        // 0 if the store was successful, 1 if no store was performed
                        "cbnz {r:w}, 2b",
                        "b 4f",
                    "3:",
                        concat!("st", $release, "xp {r:w}, {new_lo}, {new_hi}, [{dst}]"),
                        // 0 if the store was successful, 1 if no store was performed
                        "cbnz {r:w}, 2b",
                    "4:",
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    old_lo = in(reg) old.pair.lo,
                    old_hi = in(reg) old.pair.hi,
                    new_lo = in(reg) new.pair.lo,
                    new_hi = in(reg) new.pair.hi,
                    prev_lo = out(reg) prev_lo,
                    prev_hi = out(reg) prev_hi,
                    r = out(reg) _,
                    // Do not use `preserves_flags` because CMP modifies the condition flags.
                    options(nostack),
                )
            };
        }
        atomic_rmw!(cmpxchg, order, write = success);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}

// casp is always strong, and ldxp requires a corresponding (succeed) stxp for
// its atomicity (see code comment in _atomic_compare_exchange_ldxp_stxp).
// (i.e., AArch64 doesn't have 128-bit weak CAS)
use self::atomic_compare_exchange as atomic_compare_exchange_weak;

// -----------------------------------------------------------------------------
// RMW

/*

Instruction selection flow for swap/fetch_and/fetch_or:
- if compile_time(FEAT_LSE128) => swpp/ldclrp/ldsetp
- if compile_time(FEAT_LSE) && not(ll_sc_rmw) => casp
- else => ldxp_stxp

Instruction selection flow for other RMWs:
- if compile_time(FEAT_LSE) && not(ll_sc_rmw) => casp
- else => ldxp_stxp

Note:
- We don't do run-time detection of FEAT_LSE128 at this time, because
  FEAT_LSE128 is not yet available for most CPUs, but since
  swpp/ldclrp/ldsetp is wait-free, it would make sense to do run-time
  detection in the future. (TODO)
- We don't do run-time detection of FEAT_LSE for store at this time.

*/

// If FEAT_LSE is available at compile-time and portable_atomic_ll_sc_rmw cfg is not set,
// we use CAS-based atomic RMW.
#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
#[cfg(all(
    any(target_feature = "lse", portable_atomic_target_feature = "lse"),
    not(portable_atomic_ll_sc_rmw),
))]
use self::_atomic_swap_casp as atomic_swap;
#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
#[cfg(not(all(
    any(target_feature = "lse", portable_atomic_target_feature = "lse"),
    not(portable_atomic_ll_sc_rmw),
)))]
use self::_atomic_swap_ldxp_stxp as atomic_swap;
#[cfg(any(target_feature = "lse128", portable_atomic_target_feature = "lse128"))]
use self::_atomic_swap_swpp as atomic_swap;
#[cfg(any(
    target_feature = "lse128",
    portable_atomic_target_feature = "lse128",
    all(
        not(portable_atomic_no_outline_atomics),
        not(any(target_feature = "lse2", portable_atomic_target_feature = "lse2")),
    ),
))]
#[inline]
unsafe fn _atomic_swap_swpp(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_lse128!();

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
    // and the CPU supports FEAT_LSE128.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/SWPP--SWPPA--SWPPAL--SWPPL--Swap-quadword-in-memory-
    unsafe {
        let val = U128 { whole: val };
        let (prev_lo, prev_hi);
        #[cfg(not(portable_atomic_pre_llvm_16))]
        macro_rules! swap {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    start_lse128!(),
                    concat!("swpp", $acquire, $release, " {val_lo}, {val_hi}, [{dst}]"),
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = inout(reg) val.pair.lo => prev_lo,
                    val_hi = inout(reg) val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(not(portable_atomic_pre_llvm_16))]
        atomic_rmw!(swap, order);
        // LLVM supports FEAT_LSE128 instructions on LLVM 16+, so use .inst directive on old LLVM.
        // https://github.com/llvm/llvm-project/commit/7fea6f2e0e606e5339c3359568f680eaf64aa306
        #[cfg(portable_atomic_pre_llvm_16)]
        macro_rules! swap {
            ($order:tt, $fence:tt) => {
                asm!(
                    // swpp{,a,l,al} x2, x1, [x0]
                    concat!(".inst 0x19", $order, "18002"),
                    $fence,
                    in("x0") ptr_reg!(dst),
                    inout("x2") val.pair.lo => prev_lo,
                    inout("x1") val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(portable_atomic_pre_llvm_16)]
        atomic_rmw_inst!(swap, order);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}
// Do not use atomic_rmw_cas_3 because it needs extra MOV to implement swap.
#[cfg(any(test, not(portable_atomic_ll_sc_rmw)))]
#[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
#[inline]
unsafe fn _atomic_swap_casp(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_lse!();

    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantee that the CPU supports FEAT_LSE.
    unsafe {
        let val = U128 { whole: val };
        let (mut prev_lo, mut prev_hi);
        macro_rules! swap {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    start_lse!(),
                    // If FEAT_LSE2 is not supported, this works like byte-wise atomic.
                    // This is not single-copy atomic reads, but this is ok because subsequent
                    // CAS will check for consistency.
                    "ldp x4, x5, [{dst}]",
                    "2:",
                        // casp writes the current value to the first register pair,
                        // so copy the `out`'s value for later comparison.
                        "mov {tmp_lo}, x4",
                        "mov {tmp_hi}, x5",
                        concat!("casp", $acquire, $release, " x4, x5, x2, x3, [{dst}]"),
                        "cmp {tmp_hi}, x5",
                        "ccmp {tmp_lo}, x4, #0, eq",
                        "b.ne 2b",
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    tmp_lo = out(reg) _,
                    tmp_hi = out(reg) _,
                    // must be allocated to even/odd register pair
                    out("x4") prev_lo,
                    out("x5") prev_hi,
                    // must be allocated to even/odd register pair
                    in("x2") val.pair.lo,
                    in("x3") val.pair.hi,
                    // Do not use `preserves_flags` because CMP and CCMP modify the condition flags.
                    options(nostack),
                )
            };
        }
        atomic_rmw!(swap, order);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}
// Do not use atomic_rmw_ll_sc_3 because it needs extra MOV to implement swap.
#[cfg(any(
    test,
    not(all(
        any(target_feature = "lse", portable_atomic_target_feature = "lse"),
        not(portable_atomic_ll_sc_rmw),
    ))
))]
#[inline]
unsafe fn _atomic_swap_ldxp_stxp(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let val = U128 { whole: val };
        let (mut prev_lo, mut prev_hi);
        macro_rules! swap {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    "2:",
                        concat!("ld", $acquire, "xp {prev_lo}, {prev_hi}, [{dst}]"),
                        concat!("st", $release, "xp {r:w}, {val_lo}, {val_hi}, [{dst}]"),
                        // 0 if the store was successful, 1 if no store was performed
                        "cbnz {r:w}, 2b",
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = in(reg) val.pair.lo,
                    val_hi = in(reg) val.pair.hi,
                    prev_lo = out(reg) prev_lo,
                    prev_hi = out(reg) prev_hi,
                    r = out(reg) _,
                    options(nostack, preserves_flags),
                )
            };
        }
        atomic_rmw!(swap, order);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}

/// Atomic RMW by LL/SC loop (3 arguments)
/// `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - val_lo/val_hi pair: val argument (read-only for `$op`)
/// - prev_lo/prev_hi pair: previous value loaded by ll (read-only for `$op`)
/// - new_lo/new_hi pair: new value that will be stored by sc
macro_rules! atomic_rmw_ll_sc_3 {
    ($name:ident as $reexport_name:ident $(($preserves_flags:tt))?, $($op:tt)*) => {
        // If FEAT_LSE is available at compile-time and portable_atomic_ll_sc_rmw cfg is not set,
        // we use CAS-based atomic RMW generated by atomic_rmw_cas_3! macro instead.
        #[cfg(not(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        )))]
        use self::$name as $reexport_name;
        #[cfg(any(
            test,
            not(all(
                any(target_feature = "lse", portable_atomic_target_feature = "lse"),
                not(portable_atomic_ll_sc_rmw),
            ))
        ))]
        #[inline]
        unsafe fn $name(dst: *mut u128, val: u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                let val = U128 { whole: val };
                let (mut prev_lo, mut prev_hi);
                macro_rules! op {
                    ($acquire:tt, $release:tt, $fence:tt) => {
                        asm!(
                            "2:",
                                concat!("ld", $acquire, "xp {prev_lo}, {prev_hi}, [{dst}]"),
                                $($op)*
                                concat!("st", $release, "xp {r:w}, {new_lo}, {new_hi}, [{dst}]"),
                                // 0 if the store was successful, 1 if no store was performed
                                "cbnz {r:w}, 2b",
                            $fence,
                            dst = in(reg) ptr_reg!(dst),
                            val_lo = in(reg) val.pair.lo,
                            val_hi = in(reg) val.pair.hi,
                            prev_lo = out(reg) prev_lo,
                            prev_hi = out(reg) prev_hi,
                            new_lo = out(reg) _,
                            new_hi = out(reg) _,
                            r = out(reg) _,
                            options(nostack $(, $preserves_flags)?),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}
/// Atomic RMW by CAS loop (3 arguments)
/// `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - val_lo/val_hi pair: val argument (read-only for `$op`)
/// - x6/x7 pair: previous value loaded (read-only for `$op`)
/// - x4/x5 pair: new value that will be stored
macro_rules! atomic_rmw_cas_3 {
    ($name:ident as $reexport_name:ident, $($op:tt)*) => {
        // If FEAT_LSE is not available at compile-time or portable_atomic_ll_sc_rmw cfg is set,
        // we use LL/SC-based atomic RMW generated by atomic_rmw_ll_sc_3! macro instead.
        #[cfg(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        ))]
        use self::$name as $reexport_name;
        #[cfg(any(test, not(portable_atomic_ll_sc_rmw)))]
        #[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
        #[inline]
        unsafe fn $name(dst: *mut u128, val: u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_lse!();
            // SAFETY: the caller must uphold the safety contract.
            // cfg guarantee that the CPU supports FEAT_LSE.
            unsafe {
                let val = U128 { whole: val };
                let (mut prev_lo, mut prev_hi);
                macro_rules! op {
                    ($acquire:tt, $release:tt, $fence:tt) => {
                        asm!(
                            start_lse!(),
                            // If FEAT_LSE2 is not supported, this works like byte-wise atomic.
                            // This is not single-copy atomic reads, but this is ok because subsequent
                            // CAS will check for consistency.
                            "ldp x6, x7, [{dst}]",
                            "2:",
                                // casp writes the current value to the first register pair,
                                // so copy the `out`'s value for later comparison.
                                "mov {tmp_lo}, x6",
                                "mov {tmp_hi}, x7",
                                $($op)*
                                concat!("casp", $acquire, $release, " x6, x7, x4, x5, [{dst}]"),
                                "cmp {tmp_hi}, x7",
                                "ccmp {tmp_lo}, x6, #0, eq",
                                "b.ne 2b",
                            $fence,
                            dst = in(reg) ptr_reg!(dst),
                            val_lo = in(reg) val.pair.lo,
                            val_hi = in(reg) val.pair.hi,
                            tmp_lo = out(reg) _,
                            tmp_hi = out(reg) _,
                            // must be allocated to even/odd register pair
                            out("x6") prev_lo,
                            out("x7") prev_hi,
                            // must be allocated to even/odd register pair
                            out("x4") _,
                            out("x5") _,
                            // Do not use `preserves_flags` because CMP and CCMP modify the condition flags.
                            options(nostack),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}

/// Atomic RMW by LL/SC loop (2 arguments)
/// `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - prev_lo/prev_hi pair: previous value loaded by ll (read-only for `$op`)
/// - new_lo/new_hi pair: new value that will be stored by sc
macro_rules! atomic_rmw_ll_sc_2 {
    ($name:ident as $reexport_name:ident $(($preserves_flags:tt))?, $($op:tt)*) => {
        // If FEAT_LSE is available at compile-time and portable_atomic_ll_sc_rmw cfg is not set,
        // we use CAS-based atomic RMW generated by atomic_rmw_cas_2! macro instead.
        #[cfg(not(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        )))]
        use self::$name as $reexport_name;
        #[cfg(any(
            test,
            not(all(
                any(target_feature = "lse", portable_atomic_target_feature = "lse"),
                not(portable_atomic_ll_sc_rmw),
            ))
        ))]
        #[inline]
        unsafe fn $name(dst: *mut u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                let (mut prev_lo, mut prev_hi);
                macro_rules! op {
                    ($acquire:tt, $release:tt, $fence:tt) => {
                        asm!(
                            "2:",
                                concat!("ld", $acquire, "xp {prev_lo}, {prev_hi}, [{dst}]"),
                                $($op)*
                                concat!("st", $release, "xp {r:w}, {new_lo}, {new_hi}, [{dst}]"),
                                // 0 if the store was successful, 1 if no store was performed
                                "cbnz {r:w}, 2b",
                            $fence,
                            dst = in(reg) ptr_reg!(dst),
                            prev_lo = out(reg) prev_lo,
                            prev_hi = out(reg) prev_hi,
                            new_lo = out(reg) _,
                            new_hi = out(reg) _,
                            r = out(reg) _,
                            options(nostack $(, $preserves_flags)?),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}
/// Atomic RMW by CAS loop (2 arguments)
/// `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - x6/x7 pair: previous value loaded (read-only for `$op`)
/// - x4/x5 pair: new value that will be stored
macro_rules! atomic_rmw_cas_2 {
    ($name:ident as $reexport_name:ident, $($op:tt)*) => {
        // If FEAT_LSE is not available at compile-time or portable_atomic_ll_sc_rmw cfg is set,
        // we use LL/SC-based atomic RMW generated by atomic_rmw_ll_sc_3! macro instead.
        #[cfg(all(
            any(target_feature = "lse", portable_atomic_target_feature = "lse"),
            not(portable_atomic_ll_sc_rmw),
        ))]
        use self::$name as $reexport_name;
        #[cfg(any(test, not(portable_atomic_ll_sc_rmw)))]
        #[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
        #[inline]
        unsafe fn $name(dst: *mut u128, order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_lse!();
            // SAFETY: the caller must uphold the safety contract.
            // cfg guarantee that the CPU supports FEAT_LSE.
            unsafe {
                let (mut prev_lo, mut prev_hi);
                macro_rules! op {
                    ($acquire:tt, $release:tt, $fence:tt) => {
                        asm!(
                            start_lse!(),
                            // If FEAT_LSE2 is not supported, this works like byte-wise atomic.
                            // This is not single-copy atomic reads, but this is ok because subsequent
                            // CAS will check for consistency.
                            "ldp x6, x7, [{dst}]",
                            "2:",
                                // casp writes the current value to the first register pair,
                                // so copy the `out`'s value for later comparison.
                                "mov {tmp_lo}, x6",
                                "mov {tmp_hi}, x7",
                                $($op)*
                                concat!("casp", $acquire, $release, " x6, x7, x4, x5, [{dst}]"),
                                "cmp {tmp_hi}, x7",
                                "ccmp {tmp_lo}, x6, #0, eq",
                                "b.ne 2b",
                            $fence,
                            dst = in(reg) ptr_reg!(dst),
                            tmp_lo = out(reg) _,
                            tmp_hi = out(reg) _,
                            // must be allocated to even/odd register pair
                            out("x6") prev_lo,
                            out("x7") prev_hi,
                            // must be allocated to even/odd register pair
                            out("x4") _,
                            out("x5") _,
                            // Do not use `preserves_flags` because CMP and CCMP modify the condition flags.
                            options(nostack),
                        )
                    };
                }
                atomic_rmw!(op, order);
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}

// Do not use `preserves_flags` because ADDS modifies the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_add_ldxp_stxp as atomic_add,
    select_le_or_be!("adds {new_lo}, {prev_lo}, {val_lo}", "adds {new_hi}, {prev_hi}, {val_hi}"),
    select_le_or_be!("adc {new_hi}, {prev_hi}, {val_hi}", "adc {new_lo}, {prev_lo}, {val_lo}"),
}
atomic_rmw_cas_3! {
    _atomic_add_casp as atomic_add,
    select_le_or_be!("adds x4, x6, {val_lo}", "adds x5, x7, {val_hi}"),
    select_le_or_be!("adc x5, x7, {val_hi}", "adc x4, x6, {val_lo}"),
}

// Do not use `preserves_flags` because SUBS modifies the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_sub_ldxp_stxp as atomic_sub,
    select_le_or_be!("subs {new_lo}, {prev_lo}, {val_lo}", "subs {new_hi}, {prev_hi}, {val_hi}"),
    select_le_or_be!("sbc {new_hi}, {prev_hi}, {val_hi}", "sbc {new_lo}, {prev_lo}, {val_lo}"),
}
atomic_rmw_cas_3! {
    _atomic_sub_casp as atomic_sub,
    select_le_or_be!("subs x4, x6, {val_lo}", "subs x5, x7, {val_hi}"),
    select_le_or_be!("sbc x5, x7, {val_hi}", "sbc x4, x6, {val_lo}"),
}

#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
atomic_rmw_ll_sc_3! {
    _atomic_and_ldxp_stxp as atomic_and (preserves_flags),
    "and {new_lo}, {prev_lo}, {val_lo}",
    "and {new_hi}, {prev_hi}, {val_hi}",
}
#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
atomic_rmw_cas_3! {
    _atomic_and_casp as atomic_and,
    "and x4, x6, {val_lo}",
    "and x5, x7, {val_hi}",
}
#[cfg(any(target_feature = "lse128", portable_atomic_target_feature = "lse128"))]
#[inline]
unsafe fn atomic_and(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
    // and the CPU supports FEAT_LSE128.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDCLRP--LDCLRPA--LDCLRPAL--LDCLRPL--Atomic-bit-clear-on-quadword-in-memory-
    unsafe {
        let val = U128 { whole: !val };
        let (prev_lo, prev_hi);
        #[cfg(not(portable_atomic_pre_llvm_16))]
        macro_rules! clear {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    start_lse128!(),
                    concat!("ldclrp", $acquire, $release, " {val_lo}, {val_hi}, [{dst}]"),
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = inout(reg) val.pair.lo => prev_lo,
                    val_hi = inout(reg) val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(not(portable_atomic_pre_llvm_16))]
        atomic_rmw!(clear, order);
        // LLVM supports FEAT_LSE128 instructions on LLVM 16+, so use .inst directive on old LLVM.
        // https://github.com/llvm/llvm-project/commit/7fea6f2e0e606e5339c3359568f680eaf64aa306
        #[cfg(portable_atomic_pre_llvm_16)]
        macro_rules! clear {
            ($order:tt, $fence:tt) => {
                asm!(
                    // ldclrp{,a,l,al} x8, x1, [x0]
                    concat!(".inst 0x19", $order, "11008"),
                    $fence,
                    in("x0") ptr_reg!(dst),
                    inout("x8") val.pair.lo => prev_lo,
                    inout("x1") val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(portable_atomic_pre_llvm_16)]
        atomic_rmw_inst!(clear, order);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}

atomic_rmw_ll_sc_3! {
    _atomic_nand_ldxp_stxp as atomic_nand (preserves_flags),
    "and {new_lo}, {prev_lo}, {val_lo}",
    "and {new_hi}, {prev_hi}, {val_hi}",
    "mvn {new_lo}, {new_lo}",
    "mvn {new_hi}, {new_hi}",
}
atomic_rmw_cas_3! {
    _atomic_nand_casp as atomic_nand,
    "and x4, x6, {val_lo}",
    "and x5, x7, {val_hi}",
    "mvn x4, x4",
    "mvn x5, x5",
}

#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
atomic_rmw_ll_sc_3! {
    _atomic_or_ldxp_stxp as atomic_or (preserves_flags),
    "orr {new_lo}, {prev_lo}, {val_lo}",
    "orr {new_hi}, {prev_hi}, {val_hi}",
}
#[cfg(not(any(target_feature = "lse128", portable_atomic_target_feature = "lse128")))]
atomic_rmw_cas_3! {
    _atomic_or_casp as atomic_or,
    "orr x4, x6, {val_lo}",
    "orr x5, x7, {val_hi}",
}
#[cfg(any(target_feature = "lse128", portable_atomic_target_feature = "lse128"))]
#[inline]
unsafe fn atomic_or(dst: *mut u128, val: u128, order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
    // and the CPU supports FEAT_LSE128.
    //
    // Refs: https://developer.arm.com/documentation/ddi0602/2024-12/Base-Instructions/LDSETP--LDSETPA--LDSETPAL--LDSETPL--Atomic-bit-set-on-quadword-in-memory-
    unsafe {
        let val = U128 { whole: val };
        let (prev_lo, prev_hi);
        #[cfg(not(portable_atomic_pre_llvm_16))]
        macro_rules! or {
            ($acquire:tt, $release:tt, $fence:tt) => {
                asm!(
                    start_lse128!(),
                    concat!("ldsetp", $acquire, $release, " {val_lo}, {val_hi}, [{dst}]"),
                    $fence,
                    dst = in(reg) ptr_reg!(dst),
                    val_lo = inout(reg) val.pair.lo => prev_lo,
                    val_hi = inout(reg) val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(not(portable_atomic_pre_llvm_16))]
        atomic_rmw!(or, order);
        // LLVM supports FEAT_LSE128 instructions on LLVM 16+, so use .inst directive on old LLVM.
        // https://github.com/llvm/llvm-project/commit/7fea6f2e0e606e5339c3359568f680eaf64aa306
        #[cfg(portable_atomic_pre_llvm_16)]
        macro_rules! or {
            ($order:tt, $fence:tt) => {
                asm!(
                    // ldsetp{,a,l,al} x2, x1, [x0]
                    concat!(".inst 0x19", $order, "13002"),
                    $fence,
                    in("x0") ptr_reg!(dst),
                    inout("x2") val.pair.lo => prev_lo,
                    inout("x1") val.pair.hi => prev_hi,
                    options(nostack, preserves_flags),
                )
            };
        }
        #[cfg(portable_atomic_pre_llvm_16)]
        atomic_rmw_inst!(or, order);
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}

atomic_rmw_ll_sc_3! {
    _atomic_xor_ldxp_stxp as atomic_xor (preserves_flags),
    "eor {new_lo}, {prev_lo}, {val_lo}",
    "eor {new_hi}, {prev_hi}, {val_hi}",
}
atomic_rmw_cas_3! {
    _atomic_xor_casp as atomic_xor,
    "eor x4, x6, {val_lo}",
    "eor x5, x7, {val_hi}",
}

atomic_rmw_ll_sc_2! {
    _atomic_not_ldxp_stxp as atomic_not (preserves_flags),
    "mvn {new_lo}, {prev_lo}",
    "mvn {new_hi}, {prev_hi}",
}
atomic_rmw_cas_2! {
    _atomic_not_casp as atomic_not,
    "mvn x4, x6",
    "mvn x5, x7",
}

// Do not use `preserves_flags` because NEGS modifies the condition flags.
atomic_rmw_ll_sc_2! {
    _atomic_neg_ldxp_stxp as atomic_neg,
    select_le_or_be!("negs {new_lo}, {prev_lo}", "negs {new_hi}, {prev_hi}"),
    select_le_or_be!("ngc {new_hi}, {prev_hi}", "ngc {new_lo}, {prev_lo}"),
}
atomic_rmw_cas_2! {
    _atomic_neg_casp as atomic_neg,
    select_le_or_be!("negs x4, x6", "negs x5, x7"),
    select_le_or_be!("ngc x5, x7", "ngc x4, x6"),
}

// Do not use `preserves_flags` because CMP and SBCS modify the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_max_ldxp_stxp as atomic_max,
    select_le_or_be!("cmp {val_lo}, {prev_lo}", "cmp {val_hi}, {prev_hi}"),
    select_le_or_be!("sbcs xzr, {val_hi}, {prev_hi}", "sbcs xzr, {val_lo}, {prev_lo}"),
    "csel {new_hi}, {prev_hi}, {val_hi}, lt", // select hi 64-bit
    "csel {new_lo}, {prev_lo}, {val_lo}, lt", // select lo 64-bit
}
atomic_rmw_cas_3! {
    _atomic_max_casp as atomic_max,
    select_le_or_be!("cmp {val_lo}, x6", "cmp {val_hi}, x7"),
    select_le_or_be!("sbcs xzr, {val_hi}, x7", "sbcs xzr, {val_lo}, x6"),
    "csel x5, x7, {val_hi}, lt", // select hi 64-bit
    "csel x4, x6, {val_lo}, lt", // select lo 64-bit
}

// Do not use `preserves_flags` because CMP and SBCS modify the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_umax_ldxp_stxp as atomic_umax,
    select_le_or_be!("cmp {val_lo}, {prev_lo}", "cmp {val_hi}, {prev_hi}"),
    select_le_or_be!("sbcs xzr, {val_hi}, {prev_hi}", "sbcs xzr, {val_lo}, {prev_lo}"),
    "csel {new_hi}, {prev_hi}, {val_hi}, lo", // select hi 64-bit
    "csel {new_lo}, {prev_lo}, {val_lo}, lo", // select lo 64-bit
}
atomic_rmw_cas_3! {
    _atomic_umax_casp as atomic_umax,
    select_le_or_be!("cmp {val_lo}, x6", "cmp {val_hi}, x7"),
    select_le_or_be!("sbcs xzr, {val_hi}, x7", "sbcs xzr, {val_lo}, x6"),
    "csel x5, x7, {val_hi}, lo", // select hi 64-bit
    "csel x4, x6, {val_lo}, lo", // select lo 64-bit
}

// Do not use `preserves_flags` because CMP and SBCS modify the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_min_ldxp_stxp as atomic_min,
    select_le_or_be!("cmp {val_lo}, {prev_lo}", "cmp {val_hi}, {prev_hi}"),
    select_le_or_be!("sbcs xzr, {val_hi}, {prev_hi}", "sbcs xzr, {val_lo}, {prev_lo}"),
    "csel {new_hi}, {prev_hi}, {val_hi}, ge", // select hi 64-bit
    "csel {new_lo}, {prev_lo}, {val_lo}, ge", // select lo 64-bit
}
atomic_rmw_cas_3! {
    _atomic_min_casp as atomic_min,
    select_le_or_be!("cmp {val_lo}, x6", "cmp {val_hi}, x7"),
    select_le_or_be!("sbcs xzr, {val_hi}, x7", "sbcs xzr, {val_lo}, x6"),
    "csel x5, x7, {val_hi}, ge", // select hi 64-bit
    "csel x4, x6, {val_lo}, ge", // select lo 64-bit
}

// Do not use `preserves_flags` because CMP and SBCS modify the condition flags.
atomic_rmw_ll_sc_3! {
    _atomic_umin_ldxp_stxp as atomic_umin,
    select_le_or_be!("cmp {val_lo}, {prev_lo}", "cmp {val_hi}, {prev_hi}"),
    select_le_or_be!("sbcs xzr, {val_hi}, {prev_hi}", "sbcs xzr, {val_lo}, {prev_lo}"),
    "csel {new_hi}, {prev_hi}, {val_hi}, hs", // select hi 64-bit
    "csel {new_lo}, {prev_lo}, {val_lo}, hs", // select lo 64-bit
}
atomic_rmw_cas_3! {
    _atomic_umin_casp as atomic_umin,
    select_le_or_be!("cmp {val_lo}, x6", "cmp {val_hi}, x7"),
    select_le_or_be!("sbcs xzr, {val_hi}, x7", "sbcs xzr, {val_lo}, x6"),
    "csel x5, x7, {val_hi}, hs", // select hi 64-bit
    "csel x4, x6, {val_lo}, hs", // select lo 64-bit
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
