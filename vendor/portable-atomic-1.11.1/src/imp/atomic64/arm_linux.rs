// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
64-bit atomic implementation using kuser_cmpxchg64 on pre-v6 Arm Linux/Android.

Refs:
- https://github.com/torvalds/linux/blob/v6.13/Documentation/arch/arm/kernel_user_helpers.rst
- https://github.com/rust-lang/compiler-builtins/blob/compiler_builtins-v0.1.124/src/arm_linux.rs

Note: __kuser_cmpxchg64 is always SeqCst.
https://github.com/torvalds/linux/blob/v6.13/arch/arm/kernel/entry-armv.S#L700-L707

Note: On Miri and ThreadSanitizer which do not support inline assembly, we don't use
this module and use fallback implementation instead.
*/

// TODO: Since Rust 1.64, the Linux kernel requirement for Rust when using std is 3.2+, so it should
// be possible to omit the dynamic kernel version check if the std feature is enabled on Rust 1.64+.
// https://blog.rust-lang.org/2022/08/01/Increasing-glibc-kernel-requirements.html

include!("macros.rs");

#[path = "../fallback/outline_atomics.rs"]
mod fallback;

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;
use core::{mem, sync::atomic::Ordering};

use crate::utils::{Pair, U64};

// https://github.com/torvalds/linux/blob/v6.13/Documentation/arch/arm/kernel_user_helpers.rst
const KUSER_HELPER_VERSION: usize = 0xFFFF0FFC;
// __kuser_helper_version >= 5 (kernel version 3.1+)
const KUSER_CMPXCHG64: usize = 0xFFFF0F60;
#[inline]
fn __kuser_helper_version() -> i32 {
    use core::sync::atomic::AtomicI32;

    static CACHE: AtomicI32 = AtomicI32::new(0);
    let mut v = CACHE.load(Ordering::Relaxed);
    if v != 0 {
        return v;
    }
    // SAFETY: core assumes that at least __kuser_memory_barrier (__kuser_helper_version >= 3,
    // kernel version 2.6.15+) is available on this platform. __kuser_helper_version
    // is always available on such a platform.
    v = unsafe { crate::utils::ptr::with_exposed_provenance::<i32>(KUSER_HELPER_VERSION).read() };
    CACHE.store(v, Ordering::Relaxed);
    v
}
#[inline]
fn has_kuser_cmpxchg64() -> bool {
    // Note: detect_false cfg is intended to make it easy for developers to test
    // cases where features usually available is not available, and is not a public API.
    if cfg!(portable_atomic_test_detect_false) {
        return false;
    }
    __kuser_helper_version() >= 5
}
#[inline]
unsafe fn __kuser_cmpxchg64(old_val: *const u64, new_val: *const u64, ptr: *mut u64) -> bool {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let f: extern "C" fn(*const u64, *const u64, *mut u64) -> u32 =
            mem::transmute(crate::utils::ptr::with_exposed_provenance::<()>(KUSER_CMPXCHG64));
        f(old_val, new_val, ptr) == 0
    }
}

// 64-bit atomic load by two 32-bit atomic loads.
#[inline]
unsafe fn byte_wise_atomic_load(src: *const u64) -> u64 {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        let (out_lo, out_hi);
        asm!(
            "ldr {out_lo}, [{src}]",     // atomic { out_lo = *src }
            "ldr {out_hi}, [{src}, #4]", // atomic { out_hi = *src.byte_add(4) }
            src = in(reg) src,
            out_lo = out(reg) out_lo,
            out_hi = out(reg) out_hi,
            options(pure, nostack, preserves_flags, readonly),
        );
        U64 { pair: Pair { lo: out_lo, hi: out_hi } }.whole
    }
}

macro_rules! select_atomic {
    (
        unsafe fn $name:ident($dst:ident: *mut u64 $(, $($arg:tt)*)?) $(-> $ret_ty:ty)? {
            |$kuser_cmpxchg64_fn_binding:ident| $($kuser_cmpxchg64_fn_body:tt)*
        }
        fallback = $seqcst_fallback_fn:ident
    ) => {
        #[inline]
        unsafe fn $name($dst: *mut u64 $(, $($arg)*)?, _: Ordering) $(-> $ret_ty)? {
            unsafe fn kuser_cmpxchg64_fn($dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                debug_assert!($dst as usize % 8 == 0);
                debug_assert!(has_kuser_cmpxchg64());
                // SAFETY: the caller must uphold the safety contract.
                unsafe {
                    loop {
                        // This is not single-copy atomic reads, but this is ok because subsequent
                        // CAS will check for consistency.
                        //
                        // Arm's memory model allow mixed-sized atomic access.
                        // https://github.com/rust-lang/unsafe-code-guidelines/issues/345#issuecomment-1172891466
                        //
                        // Note that the C++20 memory model does not allow mixed-sized atomic access,
                        // so we must use inline assembly to implement byte_wise_atomic_load.
                        // (i.e., byte-wise atomic based on the standard library's atomic types
                        // cannot be used here).
                        let prev = byte_wise_atomic_load($dst);
                        let next = {
                            let $kuser_cmpxchg64_fn_binding = prev;
                            $($kuser_cmpxchg64_fn_body)*
                        };
                        if __kuser_cmpxchg64(&prev, &next, $dst) {
                            return prev;
                        }
                    }
                }
            }
            // SAFETY: the caller must uphold the safety contract.
            // we only calls __kuser_cmpxchg64 if it is available.
            unsafe {
                ifunc!(unsafe fn($dst: *mut u64 $(, $($arg)*)?) $(-> $ret_ty)? {
                    if has_kuser_cmpxchg64() {
                        kuser_cmpxchg64_fn
                    } else {
                        // Use SeqCst because __kuser_cmpxchg64 is always SeqCst.
                        fallback::$seqcst_fallback_fn
                    }
                })
            }
        }
    };
}

select_atomic! {
    unsafe fn atomic_load(src: *mut u64) -> u64 {
        |old| old
    }
    fallback = atomic_load_seqcst
}
#[inline]
unsafe fn atomic_store(dst: *mut u64, val: u64, order: Ordering) {
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        atomic_swap(dst, val, order);
    }
}
select_atomic! {
    unsafe fn atomic_swap(dst: *mut u64, val: u64) -> u64 {
        |_x| val
    }
    fallback = atomic_swap_seqcst
}
#[inline]
unsafe fn atomic_compare_exchange(
    dst: *mut u64,
    old: u64,
    new: u64,
    _: Ordering,
    _: Ordering,
) -> Result<u64, u64> {
    unsafe fn kuser_cmpxchg64_fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
        debug_assert!(dst as usize % 8 == 0);
        debug_assert!(has_kuser_cmpxchg64());
        // SAFETY: the caller must uphold the safety contract.
        unsafe {
            loop {
                // See select_atomic! for more.
                let prev = byte_wise_atomic_load(dst);
                let next = if prev == old { new } else { prev };
                if __kuser_cmpxchg64(&prev, &next, dst) {
                    return (prev, prev == old);
                }
            }
        }
    }
    // SAFETY: the caller must uphold the safety contract.
    // we only calls __kuser_cmpxchg64 if it is available.
    let (prev, ok) = unsafe {
        ifunc!(unsafe fn(dst: *mut u64, old: u64, new: u64) -> (u64, bool) {
            if has_kuser_cmpxchg64() {
                kuser_cmpxchg64_fn
            } else {
                // Use SeqCst because __kuser_cmpxchg64 is always SeqCst.
                fallback::atomic_compare_exchange_seqcst
            }
        })
    };
    if ok { Ok(prev) } else { Err(prev) }
}
use self::atomic_compare_exchange as atomic_compare_exchange_weak;
select_atomic! {
    unsafe fn atomic_add(dst: *mut u64, val: u64) -> u64 {
        |x| x.wrapping_add(val)
    }
    fallback = atomic_add_seqcst
}
select_atomic! {
    unsafe fn atomic_sub(dst: *mut u64, val: u64) -> u64 {
        |x| x.wrapping_sub(val)
    }
    fallback = atomic_sub_seqcst
}
select_atomic! {
    unsafe fn atomic_and(dst: *mut u64, val: u64) -> u64 {
        |x| x & val
    }
    fallback = atomic_and_seqcst
}
select_atomic! {
    unsafe fn atomic_nand(dst: *mut u64, val: u64) -> u64 {
        |x| !(x & val)
    }
    fallback = atomic_nand_seqcst
}
select_atomic! {
    unsafe fn atomic_or(dst: *mut u64, val: u64) -> u64 {
        |x| x | val
    }
    fallback = atomic_or_seqcst
}
select_atomic! {
    unsafe fn atomic_xor(dst: *mut u64, val: u64) -> u64 {
        |x| x ^ val
    }
    fallback = atomic_xor_seqcst
}
select_atomic! {
    unsafe fn atomic_max(dst: *mut u64, val: u64) -> u64 {
        |x| {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            { core::cmp::max(x as i64, val as i64) as u64 }
        }
    }
    fallback = atomic_max_seqcst
}
select_atomic! {
    unsafe fn atomic_umax(dst: *mut u64, val: u64) -> u64 {
        |x| core::cmp::max(x, val)
    }
    fallback = atomic_umax_seqcst
}
select_atomic! {
    unsafe fn atomic_min(dst: *mut u64, val: u64) -> u64 {
        |x| {
            #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
            { core::cmp::min(x as i64, val as i64) as u64 }
        }
    }
    fallback = atomic_min_seqcst
}
select_atomic! {
    unsafe fn atomic_umin(dst: *mut u64, val: u64) -> u64 {
        |x| core::cmp::min(x, val)
    }
    fallback = atomic_umin_seqcst
}
select_atomic! {
    unsafe fn atomic_not(dst: *mut u64) -> u64 {
        |x| !x
    }
    fallback = atomic_not_seqcst
}
select_atomic! {
    unsafe fn atomic_neg(dst: *mut u64) -> u64 {
        |x| x.wrapping_neg()
    }
    fallback = atomic_neg_seqcst
}

#[inline]
fn is_lock_free() -> bool {
    has_kuser_cmpxchg64()
}
const IS_ALWAYS_LOCK_FREE: bool = false;

atomic64!(AtomicI64, i64, atomic_max, atomic_min);
atomic64!(AtomicU64, u64, atomic_umax, atomic_umin);

#[allow(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_imports
)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kuser_helper_version() {
        let version = __kuser_helper_version();
        assert!(version >= 5, "{:?}", version);
        assert_eq!(version, unsafe {
            crate::utils::ptr::with_exposed_provenance::<i32>(KUSER_HELPER_VERSION).read()
        });
    }

    test_atomic_int!(i64);
    test_atomic_int!(u64);

    // load/store/swap implementation is not affected by signedness, so it is
    // enough to test only unsigned types.
    stress_test!(u64);
}
