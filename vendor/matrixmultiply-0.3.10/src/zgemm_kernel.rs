// Copyright 2016 - 2021 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::kernel::GemmKernel;
use crate::kernel::GemmSelect;
use crate::kernel::{U2, U4, c64, Element, c64_mul as mul};
use crate::archparam;
use crate::cgemm_common::pack_complex;

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelAvx2;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelFma;

#[cfg(target_arch = "aarch64")]
#[cfg(has_aarch64_simd)]
struct KernelNeon;

struct KernelFallback;

type T = c64;
type TReal = f64;

/// Detect which implementation to use and select it using the selector's
/// .select(Kernel) method.
///
/// This function is called one or more times during a whole program's
/// execution, it may be called for each gemm kernel invocation or fewer times.
#[inline]
pub(crate) fn detect<G>(selector: G) where G: GemmSelect<T> {
    // dispatch to specific compiled versions
    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    {
        if is_x86_feature_detected_!("fma") {
            if is_x86_feature_detected_!("avx2") {
                return selector.select(KernelAvx2);
            }
            return selector.select(KernelFma);
        }
    }
    #[cfg(target_arch = "aarch64")]
    #[cfg(has_aarch64_simd)]
    {
        if is_aarch64_feature_detected_!("neon") {
            return selector.select(KernelNeon);
        }
    }
    return selector.select(KernelFallback);
}

macro_rules! loop_m { ($i:ident, $e:expr) => { loop4!($i, $e) }; }
macro_rules! loop_n { ($j:ident, $e:expr) => { loop2!($j, $e) }; }

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelAvx2 {
    type Elem = T;

    type MRTy = U4;
    type NRTy = U2;

    #[inline(always)]
    fn align_to() -> usize { 32 }

    #[inline(always)]
    fn always_masked() -> bool { KernelFallback::always_masked() }

    #[inline(always)]
    fn nc() -> usize { archparam::Z_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::Z_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::Z_MC }

    pack_methods!{}

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T, rsc: isize, csc: isize) {
        kernel_target_avx2(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelFma {
    type Elem = T;

    type MRTy = <KernelFallback as GemmKernel>::MRTy;
    type NRTy = <KernelFallback as GemmKernel>::NRTy;

    #[inline(always)]
    fn align_to() -> usize { 16 }

    #[inline(always)]
    fn always_masked() -> bool { KernelFallback::always_masked() }

    #[inline(always)]
    fn nc() -> usize { archparam::Z_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::Z_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::Z_MC }

    pack_methods!{}

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T, rsc: isize, csc: isize) {
        kernel_target_fma(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(target_arch = "aarch64")]
#[cfg(has_aarch64_simd)]
impl GemmKernel for KernelNeon {
    type Elem = T;

    type MRTy = U4;
    type NRTy = U2;

    #[inline(always)]
    fn align_to() -> usize { 16 }

    #[inline(always)]
    fn always_masked() -> bool { KernelFallback::always_masked() }

    #[inline(always)]
    fn nc() -> usize { archparam::Z_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::Z_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::Z_MC }

    pack_methods!{}

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T, rsc: isize, csc: isize) {
        kernel_target_neon(k, alpha, a, b, beta, c, rsc, csc)
    }
}

impl GemmKernel for KernelFallback {
    type Elem = T;

    type MRTy = U4;
    type NRTy = U2;

    #[inline(always)]
    fn align_to() -> usize { 0 }

    #[inline(always)]
    fn always_masked() -> bool { true }

    #[inline(always)]
    fn nc() -> usize { archparam::Z_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::Z_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::Z_MC }

    pack_methods!{}

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T, rsc: isize, csc: isize) {
        kernel_fallback_impl(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
kernel_fallback_impl_complex! {
    // instantiate fma separately
    [inline target_feature(enable="fma") target_feature(enable="avx2")] [fma_yes]
    kernel_target_avx2, T, TReal, KernelAvx2::MR, KernelAvx2::NR, 4
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
kernel_fallback_impl_complex! {
    // instantiate fma separately
    [inline target_feature(enable="fma")] [fma_no]
    kernel_target_fma, T, TReal, KernelFma::MR, KernelFma::NR, 2
}

// Kernel neon

#[cfg(target_arch = "aarch64")]
#[cfg(has_aarch64_simd)]
kernel_fallback_impl_complex! {
    [inline target_feature(enable="neon")] [fma_yes]
    kernel_target_neon, T, TReal, KernelNeon::MR, KernelNeon::NR, 1
}

// kernel fallback

kernel_fallback_impl_complex! {
    [inline] [fma_no]
    kernel_fallback_impl, T, TReal, KernelFallback::MR, KernelFallback::NR, 1
}

#[inline(always)]
unsafe fn at(ptr: *const TReal, i: usize) -> TReal {
    *ptr.add(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::test::test_complex_packed_kernel;

    #[test]
    fn test_kernel_fallback_impl() {
        test_complex_packed_kernel::<KernelFallback, _, TReal>("kernel");
    }

    #[cfg(target_arch = "aarch64")]
    #[cfg(has_aarch64_simd)]
    mod test_kernel_aarch64 {
        use super::test_complex_packed_kernel;
        use super::super::*;
        #[cfg(feature = "std")]
        use std::println;
        macro_rules! test_arch_kernels {
            ($($feature_name:tt, $name:ident, $kernel_ty:ty),*) => {
                $(
                #[test]
                fn $name() {
                    if is_aarch64_feature_detected_!($feature_name) {
                        test_complex_packed_kernel::<$kernel_ty, _, TReal>(stringify!($name));
                    } else {
                        #[cfg(feature = "std")]
                        println!("Skipping, host does not have feature: {:?}", $feature_name);
                    }
                }
                )*
            }
        }

        test_arch_kernels! {
            "neon", neon, KernelNeon
        }
    }

    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    mod test_arch_kernels {
        use super::test_complex_packed_kernel;
        use super::super::*;
        #[cfg(feature = "std")]
        use std::println;
        macro_rules! test_arch_kernels_x86 {
            ($($feature_name:tt, $name:ident, $kernel_ty:ty),*) => {
                $(
                #[test]
                fn $name() {
                    if is_x86_feature_detected_!($feature_name) {
                        test_complex_packed_kernel::<$kernel_ty, _, TReal>(stringify!($name));
                    } else {
                        #[cfg(feature = "std")]
                        println!("Skipping, host does not have feature: {:?}", $feature_name);
                    }
                }
                )*
            }
        }

        test_arch_kernels_x86! {
            "fma", fma, KernelFma,
            "avx2", avx2, KernelAvx2
        }
    }
}
