// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Helper for outline-atomics.

On architectures where DW atomics are not supported on older CPUs, we use
fallback implementation when DW atomic instructions are not supported and
outline-atomics is enabled.

This module provides helpers to implement them.
*/

use core::sync::atomic::Ordering;

#[cfg(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "riscv64"))]
pub(crate) type Udw = u128;
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "riscv64"))]
pub(crate) type AtomicUdw = super::super::super::fallback::AtomicU128;
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc64", target_arch = "riscv64"))]
pub(crate) type AtomicIdw = super::super::super::fallback::AtomicI128;

#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
pub(crate) type Udw = u64;
#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
pub(crate) type AtomicUdw = super::super::super::fallback::AtomicU64;
#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
pub(crate) type AtomicIdw = super::super::super::fallback::AtomicI64;

// Asserts that the function is called in the correct context.
macro_rules! debug_assert_outline_atomics {
    () => {
        #[cfg(target_arch = "x86_64")]
        {
            debug_assert!(!super::detect::detect().cmpxchg16b());
        }
        #[cfg(target_arch = "powerpc64")]
        {
            debug_assert!(!super::detect::detect().quadword_atomics());
        }
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        {
            debug_assert!(!super::detect::detect().zacas());
        }
        #[cfg(target_arch = "arm")]
        {
            debug_assert!(!super::has_kuser_cmpxchg64());
        }
    };
}

#[cold]
pub(crate) unsafe fn atomic_load(src: *mut Udw, order: Ordering) -> Udw {
    debug_assert_outline_atomics!();
    #[allow(clippy::cast_ptr_alignment)]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        (*(src as *const AtomicUdw)).load(order)
    }
}
fn_alias! {
    #[cold]
    pub(crate) unsafe fn(src: *mut Udw) -> Udw;
    // fallback's atomic load has at least acquire semantics.
    #[cfg(not(any(target_arch = "arm", target_arch = "x86_64")))]
    atomic_load_non_seqcst = atomic_load(Ordering::Acquire);
    atomic_load_seqcst = atomic_load(Ordering::SeqCst);
}

#[cfg(not(any(target_arch = "arm", target_arch = "riscv32", target_arch = "riscv64")))]
#[cold]
pub(crate) unsafe fn atomic_store(dst: *mut Udw, val: Udw, order: Ordering) {
    debug_assert_outline_atomics!();
    #[allow(clippy::cast_ptr_alignment)]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        (*(dst as *const AtomicUdw)).store(val, order);
    }
}
#[cfg(not(any(target_arch = "arm", target_arch = "riscv32", target_arch = "riscv64")))]
fn_alias! {
    #[cold]
    pub(crate) unsafe fn(dst: *mut Udw, val: Udw);
    // fallback's atomic store has at least release semantics.
    atomic_store_non_seqcst = atomic_store(Ordering::Release);
    atomic_store_seqcst = atomic_store(Ordering::SeqCst);
}

#[cold]
pub(crate) unsafe fn atomic_compare_exchange(
    dst: *mut Udw,
    old: Udw,
    new: Udw,
    success: Ordering,
    failure: Ordering,
) -> (Udw, bool) {
    debug_assert_outline_atomics!();
    #[allow(clippy::cast_ptr_alignment)]
    // SAFETY: the caller must uphold the safety contract.
    unsafe {
        match (*(dst as *const AtomicUdw)).compare_exchange(old, new, success, failure) {
            Ok(v) => (v, true),
            Err(v) => (v, false),
        }
    }
}
fn_alias! {
    #[cold]
    pub(crate) unsafe fn(dst: *mut Udw, old: Udw, new: Udw) -> (Udw, bool);
    // fallback's atomic CAS has at least AcqRel semantics.
    #[cfg(not(any(target_arch = "arm", target_arch = "x86_64")))]
    atomic_compare_exchange_non_seqcst
        = atomic_compare_exchange(Ordering::AcqRel, Ordering::Acquire);
    atomic_compare_exchange_seqcst
        = atomic_compare_exchange(Ordering::SeqCst, Ordering::SeqCst);
}

macro_rules! atomic_rmw_3 {
    (
        $name:ident($atomic_type:ident::$method_name:ident),
        $non_seqcst_alias:ident, $seqcst_alias:ident
    ) => {
        #[cold]
        pub(crate) unsafe fn $name(dst: *mut Udw, val: Udw, order: Ordering) -> Udw {
            debug_assert_outline_atomics!();
            #[allow(
                clippy::as_underscore,
                clippy::cast_possible_wrap,
                clippy::cast_ptr_alignment,
                clippy::cast_sign_loss
            )]
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                (*(dst as *const $atomic_type)).$method_name(val as _, order) as Udw
            }
        }
        fn_alias! {
            #[cold]
            pub(crate) unsafe fn(dst: *mut Udw, val: Udw) -> Udw;
            // fallback's atomic RMW has at least AcqRel semantics.
            #[cfg(not(any(target_arch = "arm", target_arch = "x86_64")))]
            $non_seqcst_alias = $name(Ordering::AcqRel);
            $seqcst_alias = $name(Ordering::SeqCst);
        }
    };
}
macro_rules! atomic_rmw_2 {
    (
        $name:ident($atomic_type:ident::$method_name:ident),
        $non_seqcst_alias:ident, $seqcst_alias:ident
    ) => {
        #[cold]
        pub(crate) unsafe fn $name(dst: *mut Udw, order: Ordering) -> Udw {
            debug_assert_outline_atomics!();
            #[allow(clippy::cast_ptr_alignment)]
            // SAFETY: the caller must uphold the safety contract.
            unsafe {
                (*(dst as *const $atomic_type)).$method_name(order) as Udw
            }
        }
        fn_alias! {
            #[cold]
            pub(crate) unsafe fn(dst: *mut Udw) -> Udw;
            // fallback's atomic RMW has at least AcqRel semantics.
            #[cfg(not(any(target_arch = "arm", target_arch = "x86_64")))]
            $non_seqcst_alias = $name(Ordering::AcqRel);
            $seqcst_alias = $name(Ordering::SeqCst);
        }
    };
}

atomic_rmw_3!(atomic_swap(AtomicUdw::swap), atomic_swap_non_seqcst, atomic_swap_seqcst);
atomic_rmw_3!(atomic_add(AtomicUdw::fetch_add), atomic_add_non_seqcst, atomic_add_seqcst);
atomic_rmw_3!(atomic_sub(AtomicUdw::fetch_sub), atomic_sub_non_seqcst, atomic_sub_seqcst);
atomic_rmw_3!(atomic_and(AtomicUdw::fetch_and), atomic_and_non_seqcst, atomic_and_seqcst);
atomic_rmw_3!(atomic_nand(AtomicUdw::fetch_nand), atomic_nand_non_seqcst, atomic_nand_seqcst);
atomic_rmw_3!(atomic_or(AtomicUdw::fetch_or), atomic_or_non_seqcst, atomic_or_seqcst);
atomic_rmw_3!(atomic_xor(AtomicUdw::fetch_xor), atomic_xor_non_seqcst, atomic_xor_seqcst);
atomic_rmw_3!(atomic_max(AtomicIdw::fetch_max), atomic_max_non_seqcst, atomic_max_seqcst);
atomic_rmw_3!(atomic_umax(AtomicUdw::fetch_max), atomic_umax_non_seqcst, atomic_umax_seqcst);
atomic_rmw_3!(atomic_min(AtomicIdw::fetch_min), atomic_min_non_seqcst, atomic_min_seqcst);
atomic_rmw_3!(atomic_umin(AtomicUdw::fetch_min), atomic_umin_non_seqcst, atomic_umin_seqcst);

atomic_rmw_2!(atomic_not(AtomicUdw::fetch_not), atomic_not_non_seqcst, atomic_not_seqcst);
atomic_rmw_2!(atomic_neg(AtomicUdw::fetch_neg), atomic_neg_non_seqcst, atomic_neg_seqcst);
