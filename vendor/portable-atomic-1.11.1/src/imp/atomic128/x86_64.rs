// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
128-bit atomic implementation on x86_64.

This architecture provides the following 128-bit atomic instructions:

- CMPXCHG16B: CAS (CMPXCHG16B)
- VMOVDQA: load/store (Intel, AMD, or Zhaoxin CPU with AVX)

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use intrinsics.rs instead.

Refs:
- x86 and amd64 instruction reference https://www.felixcloutier.com/x86
- atomic-maybe-uninit https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- x86_64 (+cmpxchg16b) https://godbolt.org/z/rfs1jxd51
*/

// TODO: use core::arch::x86_64::cmpxchg16b where available and efficient than asm

include!("macros.rs");

#[cfg(not(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b")))]
#[path = "../fallback/outline_atomics.rs"]
mod fallback;

#[cfg(not(portable_atomic_no_outline_atomics))]
#[cfg(not(target_env = "sgx"))]
#[cfg_attr(
    not(target_feature = "sse"),
    cfg(not(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b")))
)]
#[path = "../detect/x86_64.rs"]
mod detect;

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::utils::{Pair, U128};

// Asserts that the function is called in the correct context.
macro_rules! debug_assert_cmpxchg16b {
    () => {
        #[cfg(not(any(
            target_feature = "cmpxchg16b",
            portable_atomic_target_feature = "cmpxchg16b",
        )))]
        {
            debug_assert!(detect::detect().cmpxchg16b());
        }
    };
}
#[cfg(not(any(portable_atomic_no_outline_atomics, target_env = "sgx")))]
#[cfg(target_feature = "sse")]
macro_rules! debug_assert_vmovdqa_atomic {
    () => {{
        debug_assert_cmpxchg16b!();
        debug_assert!(detect::detect().vmovdqa_atomic());
    }};
}

#[cfg(not(any(portable_atomic_no_outline_atomics, target_env = "sgx")))]
#[cfg(target_feature = "sse")]
#[cfg(target_pointer_width = "32")]
macro_rules! ptr_modifier {
    () => {
        ":e"
    };
}
#[cfg(not(any(portable_atomic_no_outline_atomics, target_env = "sgx")))]
#[cfg(target_feature = "sse")]
#[cfg(target_pointer_width = "64")]
macro_rules! ptr_modifier {
    () => {
        ""
    };
}

// Unlike AArch64 and RISC-V, x86's assembler doesn't check instruction
// requirements for the currently enabled target features. In the first place,
// there is no option in the x86 assembly for such case, like Arm .arch_extension,
// RISC-V .option arch, PowerPC .machine, etc.
// However, we set target_feature(enable) when available (Rust 1.69+) in case a
// new codegen backend is added that checks for it in the future, or an option
// is added to the assembler to check for it.
#[cfg_attr(
    not(portable_atomic_no_cmpxchg16b_target_feature),
    target_feature(enable = "cmpxchg16b")
)]
#[inline]
unsafe fn cmpxchg16b(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_cmpxchg16b!();

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned (required by CMPXCHG16B), that there are no
    // concurrent non-atomic operations, and that the CPU supports CMPXCHG16B.
    //
    // If the value at `dst` (destination operand) and rdx:rax are equal, the
    // 128-bit value in rcx:rbx is stored in the `dst`, otherwise the value at
    // `dst` is loaded to rdx:rax.
    //
    // The ZF flag is set if the value at `dst` and rdx:rax are equal,
    // otherwise it is cleared. Other flags are unaffected.
    //
    // Refs: https://www.felixcloutier.com/x86/cmpxchg8b:cmpxchg16b
    unsafe {
        // cmpxchg16b is always SeqCst.
        let r: u8;
        let old = U128 { whole: old };
        let new = U128 { whole: new };
        let (prev_lo, prev_hi);
        macro_rules! cmpxchg16b {
            ($rdi:tt) => {
                asm!(
                    "xchg {rbx_tmp}, rbx", // save rbx which is reserved by LLVM
                    concat!("lock cmpxchg16b xmmword ptr [", $rdi, "]"),
                    "sete cl",
                    "mov rbx, {rbx_tmp}", // restore rbx
                    rbx_tmp = inout(reg) new.pair.lo => _,
                    in("rcx") new.pair.hi,
                    inout("rax") old.pair.lo => prev_lo,
                    inout("rdx") old.pair.hi => prev_hi,
                    in($rdi) dst,
                    lateout("cl") r,
                    // Do not use `preserves_flags` because CMPXCHG16B modifies the ZF flag.
                    options(nostack),
                )
            };
        }
        #[cfg(target_pointer_width = "32")]
        cmpxchg16b!("edi");
        #[cfg(target_pointer_width = "64")]
        cmpxchg16b!("rdi");
        crate::utils::assert_unchecked(r == 0 || r == 1); // needed to remove extra test
        (U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole, r != 0)
    }
}

// VMOVDQA is atomic on Intel, AMD, and Zhaoxin CPUs with AVX.
// See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=104688 for details.
//
// Refs: https://www.felixcloutier.com/x86/movdqa:vmovdqa32:vmovdqa64
//
// Use cfg(target_feature = "sse") here -- SSE is included in the x86_64
// baseline and is always available, but the SSE target feature is disabled for
// use cases such as kernels and firmware that should not use vector registers.
// So, do not use vector registers unless SSE target feature is enabled.
// See also https://github.com/rust-lang/rust/blob/1.84.0/src/doc/rustc/src/platform-support/x86_64-unknown-none.md.
#[cfg(not(any(portable_atomic_no_outline_atomics, target_env = "sgx")))]
#[cfg(target_feature = "sse")]
#[target_feature(enable = "avx")]
#[inline]
unsafe fn atomic_load_vmovdqa(src: *mut u128) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_vmovdqa_atomic!();

    // SAFETY: the caller must uphold the safety contract.
    //
    // atomic load by vmovdqa is always SeqCst.
    unsafe {
        let out: core::arch::x86_64::__m128i;
        asm!(
            concat!("vmovdqa {out}, xmmword ptr [{src", ptr_modifier!(), "}]"),
            src = in(reg) src,
            out = out(xmm_reg) out,
            options(nostack, preserves_flags),
        );
        core::mem::transmute(out)
    }
}
#[cfg(not(any(portable_atomic_no_outline_atomics, target_env = "sgx")))]
#[cfg(target_feature = "sse")]
#[target_feature(enable = "avx")]
#[inline]
unsafe fn atomic_store_vmovdqa(dst: *mut u128, val: u128, order: Ordering) {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_vmovdqa_atomic!();

    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let val: core::arch::x86_64::__m128i = core::mem::transmute(val);
        match order {
            // Relaxed and Release stores are equivalent.
            Ordering::Relaxed | Ordering::Release => {
                asm!(
                    concat!("vmovdqa xmmword ptr [{dst", ptr_modifier!(), "}], {val}"),
                    dst = in(reg) dst,
                    val = in(xmm_reg) val,
                    options(nostack, preserves_flags),
                );
            }
            Ordering::SeqCst => {
                let p = core::cell::UnsafeCell::new(core::mem::MaybeUninit::<u64>::uninit());
                asm!(
                    concat!("vmovdqa xmmword ptr [{dst", ptr_modifier!(), "}], {val}"),
                    // Equivalent to mfence, but is up to 3.1x faster on Coffee Lake and up to 2.4x faster on Raptor Lake-H at least in simple cases.
                    // - https://github.com/taiki-e/portable-atomic/pull/156
                    // - LLVM uses lock or for x86_32 64-bit atomic SeqCst store using SSE https://godbolt.org/z/9sKEr8YWc
                    // - Windows uses xchg for x86_32 for MemoryBarrier https://learn.microsoft.com/en-us/windows/win32/api/winnt/nf-winnt-memorybarrier
                    // - MSVC STL uses lock inc https://github.com/microsoft/STL/pull/740
                    // - boost uses lock or https://github.com/boostorg/atomic/commit/559eba81af71386cedd99f170dc6101c6ad7bf22
                    concat!("xchg qword ptr [{p", ptr_modifier!(), "}], {tmp}"),
                    dst = in(reg) dst,
                    val = in(xmm_reg) val,
                    p = inout(reg) p.get() => _,
                    tmp = lateout(reg) _,
                    options(nostack, preserves_flags),
                );
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(not(all(
    any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
    any(portable_atomic_no_outline_atomics, target_env = "sgx", not(target_feature = "sse")),
)))]
macro_rules! load_store_detect {
    (
        vmovdqa = $vmovdqa:ident
        cmpxchg16b = $cmpxchg16b:ident
        fallback = $fallback:ident
    ) => {{
        let cpuid = detect::detect();
        #[cfg(not(any(
            target_feature = "cmpxchg16b",
            portable_atomic_target_feature = "cmpxchg16b",
        )))]
        {
            // Check CMPXCHG16B first to prevent mixing atomic and non-atomic access.
            if cpuid.cmpxchg16b() {
                // We only use VMOVDQA when SSE is enabled. See atomic_load_vmovdqa() for more.
                #[cfg(target_feature = "sse")]
                {
                    if cpuid.vmovdqa_atomic() { $vmovdqa } else { $cmpxchg16b }
                }
                #[cfg(not(target_feature = "sse"))]
                {
                    $cmpxchg16b
                }
            } else {
                fallback::$fallback
            }
        }
        #[cfg(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"))]
        {
            if cpuid.vmovdqa_atomic() { $vmovdqa } else { $cmpxchg16b }
        }
    }};
}

#[inline]
unsafe fn atomic_load(src: *mut u128, _order: Ordering) -> u128 {
    // We only use VMOVDQA when SSE is enabled. See atomic_load_vmovdqa() for more.
    // SGX doesn't support CPUID.
    #[cfg(all(
        any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        any(portable_atomic_no_outline_atomics, target_env = "sgx", not(target_feature = "sse")),
    ))]
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantees that CMPXCHG16B is available at compile-time.
    unsafe {
        // cmpxchg16b is always SeqCst.
        atomic_load_cmpxchg16b(src)
    }
    #[cfg(not(all(
        any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        any(portable_atomic_no_outline_atomics, target_env = "sgx", not(target_feature = "sse")),
    )))]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        ifunc!(unsafe fn(src: *mut u128) -> u128 {
            load_store_detect! {
                vmovdqa = atomic_load_vmovdqa
                cmpxchg16b = atomic_load_cmpxchg16b
                // Use SeqCst because cmpxchg16b and atomic load by vmovdqa is always SeqCst.
                fallback = atomic_load_seqcst
            }
        })
    }
}
// See cmpxchg16b() for target_feature(enable).
#[cfg_attr(
    not(portable_atomic_no_cmpxchg16b_target_feature),
    target_feature(enable = "cmpxchg16b")
)]
#[inline]
unsafe fn atomic_load_cmpxchg16b(src: *mut u128) -> u128 {
    debug_assert!(src as usize % 16 == 0);
    debug_assert_cmpxchg16b!();

    // SAFETY: the caller must guarantee that `src` is valid for both writes and
    // reads, 16-byte aligned, and that there are no concurrent non-atomic operations.
    // cfg guarantees that the CPU supports CMPXCHG16B.
    //
    // See cmpxchg16b function for more.
    //
    // We could use CAS loop by atomic_compare_exchange here, but using an inline assembly allows
    // omitting the storing of condition flags and avoid use of xchg to handle rbx.
    unsafe {
        // cmpxchg16b is always SeqCst.
        let (out_lo, out_hi);
        macro_rules! cmpxchg16b {
            ($rdi:tt) => {
                asm!(
                    "mov {rbx_tmp}, rbx", // save rbx which is reserved by LLVM
                    "xor rbx, rbx", // zeroed rbx
                    concat!("lock cmpxchg16b xmmword ptr [", $rdi, "]"),
                    "mov rbx, {rbx_tmp}", // restore rbx
                    // set old/new args of cmpxchg16b to 0 (rbx is zeroed after saved to rbx_tmp, to avoid xchg)
                    rbx_tmp = out(reg) _,
                    in("rcx") 0_u64,
                    inout("rax") 0_u64 => out_lo,
                    inout("rdx") 0_u64 => out_hi,
                    in($rdi) src,
                    // Do not use `preserves_flags` because CMPXCHG16B modifies the ZF flag.
                    options(nostack),
                )
            };
        }
        #[cfg(target_pointer_width = "32")]
        cmpxchg16b!("edi");
        #[cfg(target_pointer_width = "64")]
        cmpxchg16b!("rdi");
        U128 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}

#[inline]
unsafe fn atomic_store(dst: *mut u128, val: u128, order: Ordering) {
    // We only use VMOVDQA when SSE is enabled. See atomic_load_vmovdqa() for more.
    // SGX doesn't support CPUID.
    #[cfg(all(
        any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        any(portable_atomic_no_outline_atomics, target_env = "sgx", not(target_feature = "sse")),
    ))]
    // SAFETY: the caller must uphold the safety contract.
    // cfg guarantees that CMPXCHG16B is available at compile-time.
    unsafe {
        // cmpxchg16b is always SeqCst.
        let _ = order;
        atomic_store_cmpxchg16b(dst, val);
    }
    #[cfg(not(all(
        any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"),
        any(portable_atomic_no_outline_atomics, target_env = "sgx", not(target_feature = "sse")),
    )))]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        #[cfg(target_feature = "sse")]
        fn_alias! {
            #[target_feature(enable = "avx")]
            unsafe fn(dst: *mut u128, val: u128);
            // atomic store by vmovdqa has at least release semantics.
            atomic_store_vmovdqa_non_seqcst = atomic_store_vmovdqa(Ordering::Release);
            atomic_store_vmovdqa_seqcst = atomic_store_vmovdqa(Ordering::SeqCst);
        }
        match order {
            // Relaxed and Release stores are equivalent in all implementations
            // that may be called here (vmovdqa, asm-based cmpxchg16b, and fallback).
            // core::arch's cmpxchg16b will never called here.
            Ordering::Relaxed | Ordering::Release => {
                ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                    load_store_detect! {
                        vmovdqa = atomic_store_vmovdqa_non_seqcst
                        cmpxchg16b = atomic_store_cmpxchg16b
                        fallback = atomic_store_non_seqcst
                    }
                });
            }
            Ordering::SeqCst => {
                ifunc!(unsafe fn(dst: *mut u128, val: u128) {
                    load_store_detect! {
                        vmovdqa = atomic_store_vmovdqa_seqcst
                        cmpxchg16b = atomic_store_cmpxchg16b
                        fallback = atomic_store_seqcst
                    }
                });
            }
            _ => unreachable!(),
        }
    }
}
// See cmpxchg16b() for target_feature(enable).
#[cfg_attr(
    not(portable_atomic_no_cmpxchg16b_target_feature),
    target_feature(enable = "cmpxchg16b")
)]
#[inline]
unsafe fn atomic_store_cmpxchg16b(dst: *mut u128, val: u128) {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        // cmpxchg16b is always SeqCst.
        atomic_swap_cmpxchg16b(dst, val, Ordering::SeqCst);
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
    #[cfg(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"))]
    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, that there are no concurrent non-atomic operations,
    // and cfg guarantees that CMPXCHG16B is available at compile-time.
    let (prev, ok) = unsafe { cmpxchg16b(dst, old, new) };
    #[cfg(not(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b")))]
    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, and that there are no different kinds of concurrent accesses.
    let (prev, ok) = unsafe {
        ifunc!(unsafe fn(dst: *mut u128, old: u128, new: u128) -> (u128, bool) {
            if detect::detect().cmpxchg16b() {
                cmpxchg16b
            } else {
                // Use SeqCst because cmpxchg16b is always SeqCst.
                fallback::atomic_compare_exchange_seqcst
            }
        })
    };
    if ok { Ok(prev) } else { Err(prev) }
}

// cmpxchg16b is always strong.
use self::atomic_compare_exchange as atomic_compare_exchange_weak;

// See cmpxchg16b() for target_feature(enable).
#[cfg_attr(
    not(portable_atomic_no_cmpxchg16b_target_feature),
    target_feature(enable = "cmpxchg16b")
)]
#[inline]
unsafe fn atomic_swap_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128 {
    debug_assert!(dst as usize % 16 == 0);
    debug_assert_cmpxchg16b!();

    // SAFETY: the caller must guarantee that `dst` is valid for both writes and
    // reads, 16-byte aligned, and that there are no concurrent non-atomic operations.
    // cfg guarantees that the CPU supports CMPXCHG16B.
    //
    // See cmpxchg16b function for more.
    //
    // We could use CAS loop by atomic_compare_exchange here, but using an inline assembly allows
    // omitting the storing/comparing of condition flags and reducing uses of xchg/mov to handle rbx.
    //
    // Do not use atomic_rmw_cas_3 because it needs extra MOV to implement swap.
    unsafe {
        // cmpxchg16b is always SeqCst.
        let val = U128 { whole: val };
        let (mut prev_lo, mut prev_hi);
        macro_rules! cmpxchg16b {
            ($rdi:tt) => {
                asm!(
                    "xchg {rbx_tmp}, rbx", // save rbx which is reserved by LLVM
                    // This is not single-copy atomic reads, but this is ok because subsequent
                    // CAS will check for consistency.
                    //
                    // This is based on the code generated for the first load in DW RMWs by LLVM.
                    //
                    // Note that the C++20 memory model does not allow mixed-sized atomic access,
                    // so we must use inline assembly to implement this.
                    // (i.e., byte-wise atomic based on the standard library's atomic types
                    // cannot be used here).
                    concat!("mov rax, qword ptr [", $rdi, "]"),
                    concat!("mov rdx, qword ptr [", $rdi, " + 8]"),
                    "2:",
                        concat!("lock cmpxchg16b xmmword ptr [", $rdi, "]"),
                        "jne 2b",
                    "mov rbx, {rbx_tmp}", // restore rbx
                    rbx_tmp = inout(reg) val.pair.lo => _,
                    in("rcx") val.pair.hi,
                    out("rax") prev_lo,
                    out("rdx") prev_hi,
                    in($rdi) dst,
                    // Do not use `preserves_flags` because CMPXCHG16B modifies the ZF flag.
                    options(nostack),
                )
            };
        }
        #[cfg(target_pointer_width = "32")]
        cmpxchg16b!("edi");
        #[cfg(target_pointer_width = "64")]
        cmpxchg16b!("rdi");
        U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
    }
}

/// Atomic RMW by CAS loop (3 arguments)
/// `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - rsi/r8 pair: val argument (read-only for `$op`)
/// - rax/rdx pair: previous value loaded (read-only for `$op`)
/// - rbx/rcx pair: new value that will be stored
// We could use CAS loop by atomic_compare_exchange here, but using an inline assembly allows
// omitting the storing/comparing of condition flags and reducing uses of xchg/mov to handle rbx.
macro_rules! atomic_rmw_cas_3 {
    ($name:ident, $($op:tt)*) => {
        // See cmpxchg16b() for target_feature(enable).
        #[cfg_attr(
            not(portable_atomic_no_cmpxchg16b_target_feature),
            target_feature(enable = "cmpxchg16b")
        )]
        #[inline]
        unsafe fn $name(dst: *mut u128, val: u128, _order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_cmpxchg16b!();
            // SAFETY: the caller must guarantee that `dst` is valid for both writes and
            // reads, 16-byte aligned, and that there are no concurrent non-atomic operations.
            // cfg guarantees that the CPU supports CMPXCHG16B.
            //
            // See cmpxchg16b function for more.
            unsafe {
                // cmpxchg16b is always SeqCst.
                let val = U128 { whole: val };
                let (mut prev_lo, mut prev_hi);
                macro_rules! cmpxchg16b {
                    ($rdi:tt) => {
                        asm!(
                            "mov {rbx_tmp}, rbx", // save rbx which is reserved by LLVM
                            // This is not single-copy atomic reads, but this is ok because subsequent
                            // CAS will check for consistency.
                            //
                            // This is based on the code generated for the first load in DW RMWs by LLVM.
                            //
                            // Note that the C++20 memory model does not allow mixed-sized atomic access,
                            // so we must use inline assembly to implement this.
                            // (i.e., byte-wise atomic based on the standard library's atomic types
                            // cannot be used here).
                            concat!("mov rax, qword ptr [", $rdi, "]"),
                            concat!("mov rdx, qword ptr [", $rdi, " + 8]"),
                            "2:",
                                $($op)*
                                concat!("lock cmpxchg16b xmmword ptr [", $rdi, "]"),
                                "jne 2b",
                            "mov rbx, {rbx_tmp}", // restore rbx
                            rbx_tmp = out(reg) _,
                            out("rcx") _,
                            out("rax") prev_lo,
                            out("rdx") prev_hi,
                            in($rdi) dst,
                            in("rsi") val.pair.lo,
                            in("r8") val.pair.hi,
                            // Do not use `preserves_flags` because CMPXCHG16B modifies the ZF flag.
                            options(nostack),
                        )
                    };
                }
                #[cfg(target_pointer_width = "32")]
                cmpxchg16b!("edi");
                #[cfg(target_pointer_width = "64")]
                cmpxchg16b!("rdi");
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}
/// Atomic RMW by CAS loop (2 arguments)
/// `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`
///
/// `$op` can use the following registers:
/// - rax/rdx pair: previous value loaded (read-only for `$op`)
/// - rbx/rcx pair: new value that will be stored
// We could use CAS loop by atomic_compare_exchange here, but using an inline assembly allows
// omitting the storing of condition flags and avoid use of xchg to handle rbx.
macro_rules! atomic_rmw_cas_2 {
    ($name:ident, $($op:tt)*) => {
        // See cmpxchg16b() for target_feature(enable).
        #[cfg_attr(
            not(portable_atomic_no_cmpxchg16b_target_feature),
            target_feature(enable = "cmpxchg16b")
        )]
        #[inline]
        unsafe fn $name(dst: *mut u128, _order: Ordering) -> u128 {
            debug_assert!(dst as usize % 16 == 0);
            debug_assert_cmpxchg16b!();
            // SAFETY: the caller must guarantee that `dst` is valid for both writes and
            // reads, 16-byte aligned, and that there are no concurrent non-atomic operations.
            // cfg guarantees that the CPU supports CMPXCHG16B.
            //
            // See cmpxchg16b function for more.
            unsafe {
                // cmpxchg16b is always SeqCst.
                let (mut prev_lo, mut prev_hi);
                macro_rules! cmpxchg16b {
                    ($rdi:tt) => {
                        asm!(
                            "mov {rbx_tmp}, rbx", // save rbx which is reserved by LLVM
                            // This is not single-copy atomic reads, but this is ok because subsequent
                            // CAS will check for consistency.
                            //
                            // This is based on the code generated for the first load in DW RMWs by LLVM.
                            //
                            // Note that the C++20 memory model does not allow mixed-sized atomic access,
                            // so we must use inline assembly to implement this.
                            // (i.e., byte-wise atomic based on the standard library's atomic types
                            // cannot be used here).
                            concat!("mov rax, qword ptr [", $rdi, "]"),
                            concat!("mov rdx, qword ptr [", $rdi, " + 8]"),
                            "2:",
                                $($op)*
                                concat!("lock cmpxchg16b xmmword ptr [", $rdi, "]"),
                                "jne 2b",
                            "mov rbx, {rbx_tmp}", // restore rbx
                            rbx_tmp = out(reg) _,
                            out("rcx") _,
                            out("rax") prev_lo,
                            out("rdx") prev_hi,
                            in($rdi) dst,
                            // Do not use `preserves_flags` because CMPXCHG16B modifies the ZF flag.
                            options(nostack),
                        )
                    };
                }
                #[cfg(target_pointer_width = "32")]
                cmpxchg16b!("edi");
                #[cfg(target_pointer_width = "64")]
                cmpxchg16b!("rdi");
                U128 { pair: Pair { lo: prev_lo, hi: prev_hi } }.whole
            }
        }
    };
}

atomic_rmw_cas_3! {
    atomic_add_cmpxchg16b,
    "mov rbx, rax",
    "add rbx, rsi",
    "mov rcx, rdx",
    "adc rcx, r8",
}
atomic_rmw_cas_3! {
    atomic_sub_cmpxchg16b,
    "mov rbx, rax",
    "sub rbx, rsi",
    "mov rcx, rdx",
    "sbb rcx, r8",
}
atomic_rmw_cas_3! {
    atomic_and_cmpxchg16b,
    "mov rbx, rax",
    "and rbx, rsi",
    "mov rcx, rdx",
    "and rcx, r8",
}
atomic_rmw_cas_3! {
    atomic_nand_cmpxchg16b,
    "mov rbx, rax",
    "and rbx, rsi",
    "not rbx",
    "mov rcx, rdx",
    "and rcx, r8",
    "not rcx",
}
atomic_rmw_cas_3! {
    atomic_or_cmpxchg16b,
    "mov rbx, rax",
    "or rbx, rsi",
    "mov rcx, rdx",
    "or rcx, r8",
}
atomic_rmw_cas_3! {
    atomic_xor_cmpxchg16b,
    "mov rbx, rax",
    "xor rbx, rsi",
    "mov rcx, rdx",
    "xor rcx, r8",
}

atomic_rmw_cas_2! {
    atomic_not_cmpxchg16b,
    "mov rbx, rax",
    "not rbx",
    "mov rcx, rdx",
    "not rcx",
}
atomic_rmw_cas_2! {
    atomic_neg_cmpxchg16b,
    "mov rbx, rax",
    "neg rbx",
    "mov rcx, 0",
    "sbb rcx, rdx",
}

atomic_rmw_cas_3! {
    atomic_max_cmpxchg16b,
    "cmp rsi, rax",
    "mov rcx, r8",
    "sbb rcx, rdx",
    "mov rcx, r8",
    "cmovl rcx, rdx",
    "mov rbx, rsi",
    "cmovl rbx, rax",
}
atomic_rmw_cas_3! {
    atomic_umax_cmpxchg16b,
    "cmp rsi, rax",
    "mov rcx, r8",
    "sbb rcx, rdx",
    "mov rcx, r8",
    "cmovb rcx, rdx",
    "mov rbx, rsi",
    "cmovb rbx, rax",
}
atomic_rmw_cas_3! {
    atomic_min_cmpxchg16b,
    "cmp rsi, rax",
    "mov rcx, r8",
    "sbb rcx, rdx",
    "mov rcx, r8",
    "cmovge rcx, rdx",
    "mov rbx, rsi",
    "cmovge rbx, rax",
}
atomic_rmw_cas_3! {
    atomic_umin_cmpxchg16b,
    "cmp rsi, rax",
    "mov rcx, r8",
    "sbb rcx, rdx",
    "mov rcx, r8",
    "cmovae rcx, rdx",
    "mov rbx, rsi",
    "cmovae rbx, rax",
}

macro_rules! select_atomic_rmw {
    (
        unsafe fn $name:ident($($arg:tt)*) $(-> $ret_ty:ty)?;
        cmpxchg16b = $cmpxchg16b_fn:ident;
        fallback = $seqcst_fallback_fn:ident;
    ) => {
        // If cmpxchg16b is available at compile-time, we can always use cmpxchg16b_fn.
        #[cfg(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"))]
        use self::$cmpxchg16b_fn as $name;
        // Otherwise, we need to do run-time detection and can use cmpxchg16b_fn only if cmpxchg16b is available.
        #[cfg(not(any(
            target_feature = "cmpxchg16b",
            portable_atomic_target_feature = "cmpxchg16b",
        )))]
        #[inline]
        unsafe fn $name($($arg)*, _order: Ordering) $(-> $ret_ty)? {
            fn_alias! {
                // See cmpxchg16b() for target_feature(enable).
                #[cfg_attr(
                    not(portable_atomic_no_cmpxchg16b_target_feature),
                    target_feature(enable = "cmpxchg16b")
                )]
                unsafe fn($($arg)*) $(-> $ret_ty)?;
                // cmpxchg16b is always SeqCst.
                cmpxchg16b_seqcst_fn = $cmpxchg16b_fn(Ordering::SeqCst);
            }
            // SAFETY: the caller must uphold the safety contract.
            // we only calls cmpxchg16b_fn if cmpxchg16b is available.
            unsafe {
                ifunc!(unsafe fn($($arg)*) $(-> $ret_ty)? {
                    if detect::detect().cmpxchg16b() {
                        cmpxchg16b_seqcst_fn
                    } else {
                        // Use SeqCst because cmpxchg16b is always SeqCst.
                        fallback::$seqcst_fallback_fn
                    }
                })
            }
        }
    };
}

select_atomic_rmw! {
    unsafe fn atomic_swap(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_swap_cmpxchg16b;
    fallback = atomic_swap_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_add(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_add_cmpxchg16b;
    fallback = atomic_add_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_sub(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_sub_cmpxchg16b;
    fallback = atomic_sub_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_and(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_and_cmpxchg16b;
    fallback = atomic_and_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_nand(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_nand_cmpxchg16b;
    fallback = atomic_nand_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_or(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_or_cmpxchg16b;
    fallback = atomic_or_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_xor(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_xor_cmpxchg16b;
    fallback = atomic_xor_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_max(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_max_cmpxchg16b;
    fallback = atomic_max_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umax(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_umax_cmpxchg16b;
    fallback = atomic_umax_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_min(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_min_cmpxchg16b;
    fallback = atomic_min_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_umin(dst: *mut u128, val: u128) -> u128;
    cmpxchg16b = atomic_umin_cmpxchg16b;
    fallback = atomic_umin_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_not(dst: *mut u128) -> u128;
    cmpxchg16b = atomic_not_cmpxchg16b;
    fallback = atomic_not_seqcst;
}
select_atomic_rmw! {
    unsafe fn atomic_neg(dst: *mut u128) -> u128;
    cmpxchg16b = atomic_neg_cmpxchg16b;
    fallback = atomic_neg_seqcst;
}

#[inline]
fn is_lock_free() -> bool {
    #[cfg(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"))]
    {
        // CMPXCHG16B is available at compile-time.
        true
    }
    #[cfg(not(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b")))]
    {
        detect::detect().cmpxchg16b()
    }
}
const IS_ALWAYS_LOCK_FREE: bool =
    cfg!(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"));

atomic128!(AtomicI128, i128, atomic_max, atomic_min);
atomic128!(AtomicU128, u128, atomic_umax, atomic_umin);

#[allow(clippy::undocumented_unsafe_blocks, clippy::wildcard_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    test_atomic_int!(i128);
    test_atomic_int!(u128);

    // load/store/swap implementation is not affected by signedness, so it is
    // enough to test only unsigned types.
    stress_test!(u128);
}
