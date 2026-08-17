// Copyright 2016 - 2023 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::kernel::GemmKernel;
use crate::kernel::GemmSelect;
#[allow(unused)]
use crate::kernel::{U4, U8};
use crate::archparam;

#[cfg(target_arch="x86")]
use core::arch::x86::*;
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::*;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
use crate::x86::{FusedMulAdd, AvxMulAdd, DMultiplyAdd};

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelAvx;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelFmaAvx2;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelFma;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
struct KernelSse2;

#[cfg(target_arch="aarch64")]
#[cfg(has_aarch64_simd)]
struct KernelNeon;

struct KernelFallback;

type T = f64;

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
                return selector.select(KernelFmaAvx2);
            }
            return selector.select(KernelFma);
        } else if is_x86_feature_detected_!("avx") {
            return selector.select(KernelAvx);
        } else if is_x86_feature_detected_!("sse2") {
            return selector.select(KernelSse2);
        }
    }

    #[cfg(target_arch="aarch64")]
    #[cfg(has_aarch64_simd)]
    {
        if is_aarch64_feature_detected_!("neon") {
            return selector.select(KernelNeon);
        }
    }

    return selector.select(KernelFallback);
}


#[cfg(any(target_arch="x86", target_arch="x86_64"))]
macro_rules! loop_m {
    ($i:ident, $e:expr) => { loop8!($i, $e) };
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelAvx {
    type Elem = T;

    type MRTy = U8;
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 32 }

    #[inline(always)]
    fn always_masked() -> bool { false }

    #[inline(always)]
    fn nc() -> usize { archparam::D_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::D_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::D_MC }

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T,
        rsc: isize,
        csc: isize)
    {
        kernel_target_avx(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelFma {
    type Elem = T;

    type MRTy = <KernelAvx as GemmKernel>::MRTy;
    type NRTy = <KernelAvx as GemmKernel>::NRTy;

    #[inline(always)]
    fn align_to() -> usize { KernelAvx::align_to() }

    #[inline(always)]
    fn always_masked() -> bool { KernelAvx::always_masked() }

    #[inline(always)]
    fn nc() -> usize { archparam::D_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::D_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::D_MC }

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T,
        rsc: isize,
        csc: isize)
    {
        kernel_target_fma(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelFmaAvx2 {
    type Elem = T;

    type MRTy = <KernelAvx as GemmKernel>::MRTy;
    type NRTy = <KernelAvx as GemmKernel>::NRTy;

    #[inline(always)]
    fn align_to() -> usize { KernelAvx::align_to() }

    #[inline(always)]
    fn always_masked() -> bool { KernelAvx::always_masked() }

    #[inline(always)]
    fn nc() -> usize { archparam::D_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::D_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::D_MC }

    #[inline]
    unsafe fn pack_mr(kc: usize, mc: usize, pack: &mut [Self::Elem],
                      a: *const Self::Elem, rsa: isize, csa: isize)
    {
        // safety: Avx2 is enabled
        crate::packing::pack_avx2::<Self::MRTy, T>(kc, mc, pack, a, rsa, csa)
    }

    #[inline]
    unsafe fn pack_nr(kc: usize, mc: usize, pack: &mut [Self::Elem],
                      a: *const Self::Elem, rsa: isize, csa: isize)
    {
        // safety: Avx2 is enabled
        crate::packing::pack_avx2::<Self::NRTy, T>(kc, mc, pack, a, rsa, csa)
    }


    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T,
        rsc: isize,
        csc: isize)
    {
        kernel_target_fma(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelSse2 {
    type Elem = T;

    type MRTy = U4;
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 16 }

    #[inline(always)]
    fn always_masked() -> bool { true }

    #[inline(always)]
    fn nc() -> usize { archparam::D_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::D_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::D_MC }

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T,
        rsc: isize,
        csc: isize)
    {
        kernel_target_sse2(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(target_arch="aarch64")]
#[cfg(has_aarch64_simd)]
impl GemmKernel for KernelNeon {
    type Elem = T;

    type MRTy = U8;
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 32 }

    #[inline(always)]
    fn always_masked() -> bool { false }

    #[inline(always)]
    fn nc() -> usize { archparam::S_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::S_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::S_MC }

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
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 0 }

    #[inline(always)]
    fn always_masked() -> bool { true }

    #[inline(always)]
    fn nc() -> usize { archparam::D_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::D_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::D_MC }

    #[inline(always)]
    unsafe fn kernel(
        k: usize,
        alpha: T,
        a: *const T,
        b: *const T,
        beta: T,
        c: *mut T,
        rsc: isize,
        csc: isize)
    {
        kernel_fallback_impl(k, alpha, a, b, beta, c, rsc, csc)
    }
}

// no inline for unmasked kernels
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
#[target_feature(enable="fma")]
unsafe fn kernel_target_fma(k: usize, alpha: T, a: *const T, b: *const T,
                            beta: T, c: *mut T, rsc: isize, csc: isize)
{
    kernel_x86_avx::<FusedMulAdd>(k, alpha, a, b, beta, c, rsc, csc)
}

// no inline for unmasked kernels
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
#[target_feature(enable="avx")]
unsafe fn kernel_target_avx(k: usize, alpha: T, a: *const T, b: *const T,
                            beta: T, c: *mut T, rsc: isize, csc: isize)
{
    kernel_x86_avx::<AvxMulAdd>(k, alpha, a, b, beta, c, rsc, csc)
}

#[inline]
#[target_feature(enable="sse2")]
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
unsafe fn kernel_target_sse2(k: usize, alpha: T, a: *const T, b: *const T,
                                 beta: T, c: *mut T, rsc: isize, csc: isize)
{
    kernel_fallback_impl(k, alpha, a, b, beta, c, rsc, csc)
}

#[inline(always)]
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
unsafe fn kernel_x86_avx<MA>(k: usize, alpha: T, a: *const T, b: *const T,
                             beta: T, c: *mut T, rsc: isize, csc: isize)
    where MA: DMultiplyAdd
{
    const MR: usize = KernelAvx::MR;
    const NR: usize = KernelAvx::NR;

    debug_assert_ne!(k, 0);

    let mut ab = [_mm256_setzero_pd(); MR];

    let (mut a, mut b) = (a, b);

    // With MR=8, we load sets of 4 doubles from a
    let mut a_0123 = _mm256_load_pd(a);
    let mut a_4567 = _mm256_load_pd(a.add(4));

    // With NR=4, we load 4 doubles from b
    let mut b_0123 = _mm256_load_pd(b);

    unroll_by_with_last!(4 => k, is_last, {
        // We need to multiply each element of b with each element of a_0
        // and a_1. To do so, we need to generate all possible permutations
        // for the doubles in b, but without two permutations having the
        // same double at the same spot.
        //
        // So, if we are given the permutations (indices of the doubles
        // in the packed 4-vector):
        //
        // 0 1 2 3
        //
        // Then another valid permutation has to shuffle all elements
        // around without a single element remaining at the same index
        // it was before.
        //
        // A possible set of valid combination then are:
        //
        // 0 1 2 3 (the original)
        // 1 0 3 2 (chosen because of _mm256_shuffle_pd)
        // 3 2 1 0 (chosen because of _mm256_permute2f128_pd)
        // 2 3 0 1 (chosen because of _mm256_shuffle_pd)
        let b_1032 = _mm256_shuffle_pd(b_0123, b_0123, 0b0101);

        // Both packed 4-vectors are the same, so one could also perform
        // the selection 0b0000_0001 or 0b0010_0001 or 0b0010_0011.
        // The confusing part is that of the lower 4 bits and upper 4 bits
        // only 2 bits are used in each. The same choice could have been
        // encoded in a nibble (4 bits) total, i.e. 0b1100, had the intrinsics
        // been defined differently. The highest bit in each nibble controls
        // zero-ing behaviour though.
        let b_3210 = _mm256_permute2f128_pd(b_1032, b_1032, 0b0011);
        let b_2301 = _mm256_shuffle_pd(b_3210, b_3210, 0b0101);

        // The ideal distribution of a_i b_j pairs in the resulting panel of
        // c in order to have the matching products / sums of products in the
        // right places would look like this after the first iteration:
        //
        // ab_0 || a0 b0 | a0 b1 | a0 b2 | a0 b3
        // ab_1 || a1 b0 | a1 b1 | a1 b2 | a1 b3
        // ab_2 || a2 b0 | a2 b1 | a2 b2 | a2 b3
        // ab_3 || a3 b0 | a3 b1 | a3 b2 | a3 b3
        //      || -----------------------------
        // ab_4 || a4 b0 | a4 b1 | a4 b2 | a4 b3
        // ab_5 || a5 b0 | a5 b1 | a5 b2 | a5 b3
        // ab_6 || a6 b0 | a6 b1 | a6 b2 | a6 b3
        // ab_7 || a7 b0 | a7 b1 | a7 b2 | a7 b3
        //
        // As this is not possible / would require too many extra variables
        // and thus operations, we get the following configuration, and thus
        // have to be smart about putting the correct values into their
        // respective places at the end.
        //
        // ab_0 || a0 b0 | a1 b1 | a2 b2 | a3 b3
        // ab_1 || a0 b1 | a1 b0 | a2 b3 | a3 b2
        // ab_2 || a0 b2 | a1 b3 | a2 b0 | a3 b1
        // ab_3 || a0 b3 | a1 b2 | a2 b1 | a3 b0
        //      || -----------------------------
        // ab_4 || a4 b0 | a5 b1 | a6 b2 | a7 b3
        // ab_5 || a4 b1 | a5 b0 | a6 b3 | a7 b2
        // ab_6 || a4 b2 | a5 b3 | a6 b0 | a7 b1
        // ab_7 || a4 b3 | a5 b2 | a6 b1 | a7 b0

        // Add and multiply in one go
        ab[0] = MA::multiply_add(a_0123, b_0123, ab[0]);
        ab[1] = MA::multiply_add(a_0123, b_1032, ab[1]);
        ab[2] = MA::multiply_add(a_0123, b_2301, ab[2]);
        ab[3] = MA::multiply_add(a_0123, b_3210, ab[3]);

        ab[4] = MA::multiply_add(a_4567, b_0123, ab[4]);
        ab[5] = MA::multiply_add(a_4567, b_1032, ab[5]);
        ab[6] = MA::multiply_add(a_4567, b_2301, ab[6]);
        ab[7] = MA::multiply_add(a_4567, b_3210, ab[7]);

        if !is_last {
            a = a.add(MR);
            b = b.add(NR);

            a_0123 = _mm256_load_pd(a);
            a_4567 = _mm256_load_pd(a.add(4));
            b_0123 = _mm256_load_pd(b);
        }
    });

    // Our products/sums are currently stored according to the
    // table below. Each row corresponds to one packed simd
    // 4-vector.
    //
    // ab_0 || a0 b0 | a1 b1 | a2 b2 | a3 b3
    // ab_1 || a0 b1 | a1 b0 | a2 b3 | a3 b2
    // ab_2 || a0 b2 | a1 b3 | a2 b0 | a3 b1
    // ab_3 || a0 b3 | a1 b2 | a2 b1 | a3 b0
    //      || -----------------------------
    // ab_4 || a4 b0 | a5 b1 | a6 b2 | a7 b3
    // ab_5 || a4 b1 | a5 b0 | a6 b3 | a7 b2
    // ab_6 || a4 b2 | a5 b3 | a6 b0 | a7 b1
    // ab_7 || a4 b3 | a5 b2 | a6 b1 | a7 b0
    //
    // This is the final results, where indices are stored
    // in their proper location.
    //
    //      || a0 b0 | a0 b1 | a0 b2 | a0 b3
    //      || a1 b0 | a1 b1 | a1 b2 | a1 b3
    //      || a2 b0 | a2 b1 | a2 b2 | a2 b3
    //      || a3 b0 | a3 b1 | a3 b2 | a3 b3
    //      || -----------------------------
    //      || a4 b0 | a4 b1 | a4 b2 | a4 b3
    //      || a5 b0 | a5 b1 | a5 b2 | a5 b3
    //      || a6 b0 | a6 b1 | a6 b2 | a6 b3
    //      || a7 b0 | a7 b1 | a7 b2 | a7 b3
    //
    // Given the simd intrinsics available through avx, we have two
    // ways of achieving this format. By either:
    //
    // a) Creating packed 4-vectors of rows, or
    // b) creating packed 4-vectors of columns.
    //
    // ** We will use option a) because it has slightly cheaper throughput
    // characteristics (see below).
    //
    // # a) Creating packed 4-vectors of columns
    //
    // To create packed 4-vectors of columns, we make us of
    // _mm256_blend_pd operations, followed by _mm256_permute2f128_pd.
    //
    // The first operation has latency 1 (all architectures), and 0.33
    // throughput (Skylake, Broadwell, Haswell), or 0.5 (Ivy Bridge).
    //
    // The second operation has latency 3 (on Skylake, Broadwell, Haswell),
    // or latency 2 (on Ivy Brdige), and throughput 1 (all architectures).
    //
    // We start by applying _mm256_blend_pd on adjacent rows:
    //
    // Step 0.0
    // a0 b0 | a1 b1 | a2 b2 | a3 b3
    // a0 b1 | a1 b0 | a2 b3 | a3 b2
    // => _mm256_blend_pd with 0b1010
    // a0 b0 | a1 b0 | a2 b2 | a3 b2 (only columns 0 and 2)
    //
    // Step 0.1
    // a0 b1 | a1 b0 | a2 b3 | a3 b2 (flipped the order)
    // a0 b0 | a1 b1 | a2 b2 | a3 b3
    // => _mm256_blend_pd with 0b1010
    // a0 b1 | a1 b1 | a2 b3 | a3 b3 (only columns 1 and 3)
    //
    // Step 0.2
    // a0 b2 | a1 b3 | a2 b0 | a3 b1
    // a0 b3 | a1 b2 | a2 b1 | a3 b0
    // => _mm256_blend_pd with 0b1010
    // a0 b2 | a1 b2 | a2 b0 | a3 b0 (only columns 0 and 2)
    //
    // Step 0.3
    // a0 b3 | a1 b2 | a2 b1 | a3 b0 (flipped the order)
    // a0 b2 | a1 b3 | a2 b0 | a3 b1
    // => _mm256_blend_pd with 0b1010
    // a0 b3 | a1 b3 | a2 b1 | a3 b1 (only columns 1 and 3)
    //
    // Step 1.0 (combining steps 0.0 and 0.2)
    //
    // a0 b0 | a1 b0 | a2 b2 | a3 b2
    // a0 b2 | a1 b2 | a2 b0 | a3 b0
    // => _mm256_permute2f128_pd with 0x30 = 0b0011_0000
    // a0 b0 | a1 b0 | a2 b0 | a3 b0
    //
    // Step 1.1 (combining steps 0.0 and 0.2)
    //
    // a0 b0 | a1 b0 | a2 b2 | a3 b2
    // a0 b2 | a1 b2 | a2 b0 | a3 b0
    // => _mm256_permute2f128_pd with 0x12 = 0b0001_0010
    // a0 b2 | a1 b2 | a2 b2 | a3 b2
    //
    // Step 1.2 (combining steps 0.1 and 0.3)
    // a0 b1 | a1 b1 | a2 b3 | a3 b3
    // a0 b3 | a1 b3 | a2 b1 | a3 b1
    // => _mm256_permute2f128_pd with 0x30 = 0b0011_0000
    // a0 b1 | a1 b1 | a2 b1 | a3 b1
    //
    // Step 1.3 (combining steps 0.1 and 0.3)
    // a0 b1 | a1 b1 | a2 b3 | a3 b3
    // a0 b3 | a1 b3 | a2 b1 | a3 b1
    // => _mm256_permute2f128_pd with 0x12 = 0b0001_0010
    // a0 b3 | a1 b3 | a2 b3 | a3 b3
    //
    // # b) Creating packed 4-vectors of rows
    //
    // To create packed 4-vectors of rows, we make use of
    // _mm256_shuffle_pd operations followed by _mm256_permute2f128_pd.
    //
    // The first operation has a latency 1, throughput 1 (on architectures
    // Skylake, Broadwell, Haswell, and Ivy Bridge).
    //
    // The second operation has latency 3 (on Skylake, Broadwell, Haswell),
    // or latency 2 (on Ivy Brdige), and throughput 1 (all architectures).
    //
    // To achieve this, we can execute a _mm256_shuffle_pd on
    // rows 0 and 1 stored in ab_0 and ab_1:
    //
    // Step 0.0
    // a0 b0 | a1 b1 | a2 b2 | a3 b3
    // a0 b1 | a1 b0 | a2 b3 | a3 b2
    // => _mm256_shuffle_pd with 0000
    // a0 b0 | a0 b1 | a2 b2 | a2 b3 (only rows 0 and 2)
    //
    // Step 0.1
    // a0 b1 | a1 b0 | a2 b3 | a3 b2 (flipped the order)
    // a0 b0 | a1 b1 | a2 b2 | a3 b3
    // => _mm256_shuffle_pd with 1111
    // a1 b0 | a1 b1 | a3 b2 | a3 b3 (only rows 1 and 3)
    //
    // Next, we perform the same operation on the other two rows:
    //
    // Step 0.2
    // a0 b2 | a1 b3 | a2 b0 | a3 b1
    // a0 b3 | a1 b2 | a2 b1 | a3 b0
    // => _mm256_shuffle_pd with 0000
    // a0 b2 | a0 b3 | a2 b0 | a2 b1 (only rows 0 and 2)
    //
    // Step 0.3
    // a0 b3 | a1 b2 | a2 b1 | a3 b0
    // a0 b2 | a1 b3 | a2 b0 | a3 b1
    // => _mm256_shuffle_pd with 1111
    // a1 b2 | a1 b3 | a3 b0 | a3 b1 (only rows 1 and 3)
    //
    // Next, we can apply _mm256_permute2f128_pd to select the
    // correct columns on the matching rows:
    //
    // Step 1.0 (combining Steps 0.0 and 0.2):
    // a0 b0 | a0 b1 | a2 b2 | a2 b3
    // a0 b2 | a0 b3 | a2 b0 | a2 b1
    // => _mm256_permute_2f128_pd with 0x20 = 0b0010_0000
    // a0 b0 | a0 b1 | a0 b2 | a0 b3
    //
    // Step 1.1 (combining Steps 0.0 and 0.2):
    // a0 b0 | a0 b1 | a2 b2 | a2 b3
    // a0 b2 | a0 b3 | a2 b0 | a2 b1
    // => _mm256_permute_2f128_pd with 0x03 = 0b0001_0011
    // a2 b0 | a2 b1 | a2 b2 | a2 b3
    //
    // Step 1.2 (combining Steps 0.1 and 0.3):
    // a1 b0 | a1 b1 | a3 b2 | a3 b3
    // a1 b2 | a1 b3 | a3 b0 | a3 b1
    // => _mm256_permute_2f128_pd with 0x20 = 0b0010_0000
    // a1 b0 | a1 b1 | a1 b2 | a1 b3
    //
    // Step 1.3 (combining Steps 0.1 and 0.3):
    // a1 b0 | a1 b1 | a3 b2 | a3 b3
    // a1 b2 | a1 b3 | a3 b0 | a3 b1
    // => _mm256_permute_2f128_pd with 0x03 = 0b0001_0011
    // a3 b0 | a3 b1 | a3 b2 | a3 b3

    // We use scheme a) as the default case, i.e. if c is column-major, rsc==1, or if
    // c is of general form. Row-major c matrices, csc==1, are treated using schema b).
    if csc == 1 {
        // Scheme b), step 0.0
        // a0 b0 | a1 b1 | a2 b2 | a3 b3
        // a0 b1 | a1 b0 | a2 b3 | a3 b2
        let a0b0_a0b1_a2b2_a2b3 = _mm256_shuffle_pd(ab[0], ab[1], 0b0000);

        // Scheme b), step 0.1
        // a0 b1 | a1 b0 | a2 b3 | a3 b2 (flipped the order)
        // a0 b0 | a1 b1 | a2 b2 | a3 b3
        let a1b0_a1b1_a3b2_a3b3 = _mm256_shuffle_pd(ab[1], ab[0], 0b1111);

        // Scheme b), step 0.2
        // a0 b2 | a1 b3 | a2 b0 | a3 b1
        // a0 b3 | a1 b2 | a2 b1 | a3 b0
        let a0b2_a0b3_a2b0_a2b1 = _mm256_shuffle_pd(ab[2], ab[3], 0b0000);

        // Scheme b), step 0.3
        // a0 b3 | a1 b2 | a2 b1 | a3 b0 (flipped the order)
        // a0 b2 | a1 b3 | a2 b0 | a3 b1
        let a1b2_a1b3_a3b0_a3b1 = _mm256_shuffle_pd(ab[3], ab[2], 0b1111);

        let a4b0_a4b1_a6b2_a6b3 = _mm256_shuffle_pd(ab[4], ab[5], 0b0000);
        let a5b0_a5b1_a7b2_a7b3 = _mm256_shuffle_pd(ab[5], ab[4], 0b1111);

        let a4b2_a4b3_a6b0_a6b1 = _mm256_shuffle_pd(ab[6], ab[7], 0b0000);
        let a5b2_a5b3_a7b0_a7b1 = _mm256_shuffle_pd(ab[7], ab[6], 0b1111);

        // Next, we can apply _mm256_permute2f128_pd to select the
        // correct columns on the matching rows:
        //
        // Step 1.0 (combining Steps 0.0 and 0.2):
        // a0 b0 | a0 b1 | a2 b2 | a2 b3
        // a0 b2 | a0 b3 | a2 b0 | a2 b1
        // => _mm256_permute_2f128_pd with 0x20 = 0b0010_0000
        // a0 b0 | a0 b1 | a0 b2 | a0 b3
        //
        // Step 1.1 (combining Steps 0.0 and 0.2):
        // a0 b0 | a0 b1 | a2 b2 | a2 b3
        // a0 b2 | a0 b3 | a2 b0 | a2 b1
        // => _mm256_permute_2f128_pd with 0x03 = 0b0001_0011
        // a2 b0 | a2 b1 | a2 b2 | a2 b3
        //
        // Step 1.2 (combining Steps 0.1 and 0.3):
        // a1 b0 | a1 b1 | a3 b2 | a3 b3
        // a1 b2 | a1 b3 | a3 b0 | a3 b1
        // => _mm256_permute_2f128_pd with 0x20 = 0b0010_0000
        // a1 b0 | a1 b1 | a1 b2 | a1 b3
        //
        // Step 1.3 (combining Steps 0.1 and 0.3):
        // a1 b0 | a1 b1 | a3 b2 | a3 b3
        // a1 b2 | a1 b3 | a3 b0 | a3 b1
        // => _mm256_permute_2f128_pd with 0x03 = 0b0001_0011
        // a3 b0 | a3 b1 | a3 b2 | a3 b3

        // Scheme b), step 1.0
        let a0b0_a0b1_a0b2_a0b3 = _mm256_permute2f128_pd(
            a0b0_a0b1_a2b2_a2b3,
            a0b2_a0b3_a2b0_a2b1,
            0x20
        );
        // Scheme b), step 1.1
        let a2b0_a2b1_a2b2_a2b3 = _mm256_permute2f128_pd(
            a0b0_a0b1_a2b2_a2b3,
            a0b2_a0b3_a2b0_a2b1,
            0x13
        );
        // Scheme b) step 1.2
        let a1b0_a1b1_a1b2_a1b3 = _mm256_permute2f128_pd(
            a1b0_a1b1_a3b2_a3b3,
            a1b2_a1b3_a3b0_a3b1,
            0x20
        );
        // Scheme b) step 1.3
        let a3b0_a3b1_a3b2_a3b3 = _mm256_permute2f128_pd(
            a1b0_a1b1_a3b2_a3b3,
            a1b2_a1b3_a3b0_a3b1,
            0x13
        );

        // As above, but for ab[4..7]
        let a4b0_a4b1_a4b2_a4b3 = _mm256_permute2f128_pd(
            a4b0_a4b1_a6b2_a6b3,
            a4b2_a4b3_a6b0_a6b1,
            0x20
        );

        let a6b0_a6b1_a6b2_a6b3 = _mm256_permute2f128_pd(
            a4b0_a4b1_a6b2_a6b3,
            a4b2_a4b3_a6b0_a6b1,
            0x13
        );

        let a5b0_a5b1_a5b2_a5b3 = _mm256_permute2f128_pd(
            a5b0_a5b1_a7b2_a7b3,
            a5b2_a5b3_a7b0_a7b1,
            0x20
        );

        let a7b0_a7b1_a7b2_a7b3 = _mm256_permute2f128_pd(
            a5b0_a5b1_a7b2_a7b3,
            a5b2_a5b3_a7b0_a7b1,
            0x13
        );

        ab[0] = a0b0_a0b1_a0b2_a0b3;
        ab[1] = a1b0_a1b1_a1b2_a1b3;
        ab[2] = a2b0_a2b1_a2b2_a2b3;
        ab[3] = a3b0_a3b1_a3b2_a3b3;

        ab[4] = a4b0_a4b1_a4b2_a4b3;
        ab[5] = a5b0_a5b1_a5b2_a5b3;
        ab[6] = a6b0_a6b1_a6b2_a6b3;
        ab[7] = a7b0_a7b1_a7b2_a7b3;

    //  rsc == 1 and general matrix orders
    } else {
        // Scheme a), step 0.0
        // ab[0] = a0 b0 | a1 b1 | a2 b2 | a3 b3
        // ab[1] = a0 b1 | a1 b0 | a2 b3 | a3 b2
        let a0b0_a1b0_a2b2_a3b2 = _mm256_blend_pd(ab[0], ab[1], 0b1010);
        // Scheme a), step 0.1
        let a0b1_a1b1_a2b3_a3b3 = _mm256_blend_pd(ab[1], ab[0], 0b1010);

        // Scheme a), steps 0.2
        // ab[2] = a0 b2 | a1 b3 | a2 b0 | a3 b1
        // ab[3] = a0 b3 | a1 b2 | a2 b1 | a3 b0
        let a0b2_a1b2_a2b0_a3b0 = _mm256_blend_pd(ab[2], ab[3], 0b1010);
        // Scheme a), steps 0.3
        let a0b3_a1b3_a2b1_a3b1 = _mm256_blend_pd(ab[3], ab[2], 0b1010);

        // ab[4] = a4 b0 | a5 b1 | a6 b2 | a7 b3
        // ab[5] = a4 b1 | a5 b0 | a6 b3 | a7 b2
        let a4b0_a5b0_a6b2_a7b2 = _mm256_blend_pd(ab[4], ab[5], 0b1010);
        let a4b1_a5b1_a6b3_a7b3 = _mm256_blend_pd(ab[5], ab[4], 0b1010);

        // ab[6] = a0 b2 | a1 b3 | a2 b0 | a3 b1
        // ab[7] = a0 b3 | a1 b2 | a2 b1 | a3 b0
        let a4b2_a5b2_a6b0_a7b0 = _mm256_blend_pd(ab[6], ab[7], 0b1010);
        let a4b3_a5b3_a6b1_a7b1 = _mm256_blend_pd(ab[7], ab[6], 0b1010);

        // Scheme a), step 1.0
        let a0b0_a1b0_a2b0_a3b0 = _mm256_permute2f128_pd(
            a0b0_a1b0_a2b2_a3b2,
            a0b2_a1b2_a2b0_a3b0,
            0x30
        );
        // Scheme a), step 1.1
        let a0b2_a1b2_a2b2_a3b2 = _mm256_permute2f128_pd(
            a0b0_a1b0_a2b2_a3b2,
            a0b2_a1b2_a2b0_a3b0,
            0x12,
        );
        // Scheme a) step 1.2
        let a0b1_a1b1_a2b1_a3b1 = _mm256_permute2f128_pd(
            a0b1_a1b1_a2b3_a3b3,
            a0b3_a1b3_a2b1_a3b1,
            0x30
        );
        // Scheme a) step 1.3
        let a0b3_a1b3_a2b3_a3b3 = _mm256_permute2f128_pd(
            a0b1_a1b1_a2b3_a3b3,
            a0b3_a1b3_a2b1_a3b1,
            0x12
        );

        // As above, but for ab[4..7]
        let a4b0_a5b0_a6b0_a7b0 = _mm256_permute2f128_pd(
            a4b0_a5b0_a6b2_a7b2,
            a4b2_a5b2_a6b0_a7b0,
            0x30
        );
        let a4b2_a5b2_a6b2_a7b2 = _mm256_permute2f128_pd(
            a4b0_a5b0_a6b2_a7b2,
            a4b2_a5b2_a6b0_a7b0,
            0x12,
        );
        let a4b1_a5b1_a6b1_a7b1 = _mm256_permute2f128_pd(
            a4b1_a5b1_a6b3_a7b3,
            a4b3_a5b3_a6b1_a7b1,
            0x30
        );
        let a4b3_a5b3_a6b3_a7b3 = _mm256_permute2f128_pd(
            a4b1_a5b1_a6b3_a7b3,
            a4b3_a5b3_a6b1_a7b1,
            0x12
        );

        ab[0] = a0b0_a1b0_a2b0_a3b0;
        ab[1] = a0b1_a1b1_a2b1_a3b1;
        ab[2] = a0b2_a1b2_a2b2_a3b2;
        ab[3] = a0b3_a1b3_a2b3_a3b3;

        ab[4] = a4b0_a5b0_a6b0_a7b0;
        ab[5] = a4b1_a5b1_a6b1_a7b1;
        ab[6] = a4b2_a5b2_a6b2_a7b2;
        ab[7] = a4b3_a5b3_a6b3_a7b3;
    }

    // Compute α (A B)
    // Compute here if we don't have fma, else pick up α further down

    let alphav = _mm256_broadcast_sd(&alpha);
    if !MA::IS_FUSED {
        loop_m!(i, ab[i] = _mm256_mul_pd(alphav, ab[i]));
    }

    macro_rules! c {
        ($i:expr, $j:expr) =>
            (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // C ← α A B + β C
    let mut cv = [_mm256_setzero_pd(); MR];

    if beta != 0. {
        // Read C
        if rsc == 1 {
            loop4!(i, cv[i] = _mm256_loadu_pd(c![0, i]));
            loop4!(i, cv[i + 4] = _mm256_loadu_pd(c![4, i]));
        } else if csc == 1 {
            loop4!(i, cv[i] = _mm256_loadu_pd(c![i, 0]));
            loop4!(i, cv[i+4] = _mm256_loadu_pd(c![i+4, 0]));
        } else {
            loop4!(i, cv[i] = _mm256_setr_pd(
                    *c![0, i],
                    *c![1, i],
                    *c![2, i],
                    *c![3, i]
            ));
            loop4!(i, cv[i + 4] = _mm256_setr_pd(
                    *c![4, i],
                    *c![5, i],
                    *c![6, i],
                    *c![7, i]
            ));
        }
        // Compute β C
        // _mm256_set1_pd and _mm256_broadcast_sd seem to achieve the same thing.
        let beta_v = _mm256_broadcast_sd(&beta);
        loop_m!(i, cv[i] = _mm256_mul_pd(cv[i], beta_v));
    }

    // Compute (α A B) + (β C)
    if !MA::IS_FUSED {
        loop_m!(i, cv[i] = _mm256_add_pd(cv[i], ab[i]));
    } else {
        loop_m!(i, cv[i] = MA::multiply_add(alphav, ab[i], cv[i]));
    }

    if rsc == 1 {
        loop4!(i, _mm256_storeu_pd(c![0, i], cv[i]));
        loop4!(i, _mm256_storeu_pd(c![4, i], cv[i + 4]));
    } else if csc == 1 {
        loop4!(i, _mm256_storeu_pd(c![i, 0], cv[i]));
        loop4!(i, _mm256_storeu_pd(c![i+4, 0], cv[i + 4]));
    } else {
        // Permute to bring each element in the vector to the front and store
        loop4!(i, {
            // E.g. c_0_lo = a0b0 | a1b0
            let c_lo: __m128d = _mm256_extractf128_pd(cv[i], 0);
            // E.g. c_0_hi = a2b0 | a3b0
            let c_hi: __m128d = _mm256_extractf128_pd(cv[i], 1);

            _mm_storel_pd(c![0, i], c_lo);
            _mm_storeh_pd(c![1, i], c_lo);
            _mm_storel_pd(c![2, i], c_hi);
            _mm_storeh_pd(c![3, i], c_hi);

            // E.g. c_0_lo = a0b0 | a1b0
            let c_lo: __m128d = _mm256_extractf128_pd(cv[i+4], 0);
            // E.g. c_0_hi = a2b0 | a3b0
            let c_hi: __m128d = _mm256_extractf128_pd(cv[i+4], 1);

            _mm_storel_pd(c![4, i], c_lo);
            _mm_storeh_pd(c![5, i], c_lo);
            _mm_storel_pd(c![6, i], c_hi);
            _mm_storeh_pd(c![7, i], c_hi);
        });
    }
}

#[cfg(target_arch="aarch64")]
#[cfg(has_aarch64_simd)]
#[target_feature(enable="neon")]
unsafe fn kernel_target_neon(k: usize, alpha: T, a: *const T, b: *const T,
                             beta: T, c: *mut T, rsc: isize, csc: isize)
{
    use core::arch::aarch64::*;
    const MR: usize = KernelNeon::MR;
    const NR: usize = KernelNeon::NR;

    let (mut a, mut b) = (a, b);

    // Kernel 8 x 4 (a x b)
    // Four quadrants of 4 x 2
    let mut ab11 = [vmovq_n_f64(0.); 4];
    let mut ab12 = [vmovq_n_f64(0.); 4];
    let mut ab21 = [vmovq_n_f64(0.); 4];
    let mut ab22 = [vmovq_n_f64(0.); 4];

    // Compute
    // ab_ij = a_i * b_j for all i, j
    macro_rules! ab_ij_equals_ai_bj_12 {
        ($dest:ident, $av:expr, $bv:expr) => {
            $dest[0] = vfmaq_laneq_f64($dest[0], $bv, $av, 0);
            $dest[1] = vfmaq_laneq_f64($dest[1], $bv, $av, 1);
        }
    }

    macro_rules! ab_ij_equals_ai_bj_23 {
        ($dest:ident, $av:expr, $bv:expr) => {
            $dest[2] = vfmaq_laneq_f64($dest[2], $bv, $av, 0);
            $dest[3] = vfmaq_laneq_f64($dest[3], $bv, $av, 1);
        }
    }

    for _ in 0..k {
        let b1 = vld1q_f64(b);
        let b2 = vld1q_f64(b.add(2));

        let a1 = vld1q_f64(a);
        let a2 = vld1q_f64(a.add(2));

        ab_ij_equals_ai_bj_12!(ab11, a1, b1);
        ab_ij_equals_ai_bj_23!(ab11, a2, b1);
        ab_ij_equals_ai_bj_12!(ab12, a1, b2);
        ab_ij_equals_ai_bj_23!(ab12, a2, b2);

        let a3 = vld1q_f64(a.add(4));
        let a4 = vld1q_f64(a.add(6));

        ab_ij_equals_ai_bj_12!(ab21, a3, b1);
        ab_ij_equals_ai_bj_23!(ab21, a4, b1);
        ab_ij_equals_ai_bj_12!(ab22, a3, b2);
        ab_ij_equals_ai_bj_23!(ab22, a4, b2);

        a = a.add(MR);
        b = b.add(NR);
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // ab *= alpha
    loop4!(i, ab11[i] = vmulq_n_f64(ab11[i], alpha));
    loop4!(i, ab12[i] = vmulq_n_f64(ab12[i], alpha));
    loop4!(i, ab21[i] = vmulq_n_f64(ab21[i], alpha));
    loop4!(i, ab22[i] = vmulq_n_f64(ab22[i], alpha));

    // load one float64x2_t from two pointers
    macro_rules! loadq_from_pointers {
        ($p0:expr, $p1:expr) => (
            {
                let v = vld1q_dup_f64($p0);
                let v = vld1q_lane_f64($p1, v, 1);
                v
            }
        );
    }

    if beta != 0. {
        // load existing value in C
        let mut c11 = [vmovq_n_f64(0.); 4];
        let mut c12 = [vmovq_n_f64(0.); 4];
        let mut c21 = [vmovq_n_f64(0.); 4];
        let mut c22 = [vmovq_n_f64(0.); 4];

        if csc == 1 {
            loop4!(i, c11[i] = vld1q_f64(c![i + 0, 0]));
            loop4!(i, c12[i] = vld1q_f64(c![i + 0, 2]));
            loop4!(i, c21[i] = vld1q_f64(c![i + 4, 0]));
            loop4!(i, c22[i] = vld1q_f64(c![i + 4, 2]));
        } else {
            loop4!(i, c11[i] = loadq_from_pointers!(c![i + 0, 0], c![i + 0, 1]));
            loop4!(i, c12[i] = loadq_from_pointers!(c![i + 0, 2], c![i + 0, 3]));
            loop4!(i, c21[i] = loadq_from_pointers!(c![i + 4, 0], c![i + 4, 1]));
            loop4!(i, c22[i] = loadq_from_pointers!(c![i + 4, 2], c![i + 4, 3]));
        }

        let betav = vmovq_n_f64(beta);

        // ab += β C
        loop4!(i, ab11[i] = vfmaq_f64(ab11[i], c11[i], betav));
        loop4!(i, ab12[i] = vfmaq_f64(ab12[i], c12[i], betav));
        loop4!(i, ab21[i] = vfmaq_f64(ab21[i], c21[i], betav));
        loop4!(i, ab22[i] = vfmaq_f64(ab22[i], c22[i], betav));
    }

    // c <- ab
    // which is in full
    //   C <- α A B (+ β C)
    if csc == 1 {
        loop4!(i, vst1q_f64(c![i + 0, 0], ab11[i]));
        loop4!(i, vst1q_f64(c![i + 0, 2], ab12[i]));
        loop4!(i, vst1q_f64(c![i + 4, 0], ab21[i]));
        loop4!(i, vst1q_f64(c![i + 4, 2], ab22[i]));
    } else {
        loop4!(i, vst1q_lane_f64(c![i + 0, 0], ab11[i], 0));
        loop4!(i, vst1q_lane_f64(c![i + 0, 1], ab11[i], 1));

        loop4!(i, vst1q_lane_f64(c![i + 0, 2], ab12[i], 0));
        loop4!(i, vst1q_lane_f64(c![i + 0, 3], ab12[i], 1));

        loop4!(i, vst1q_lane_f64(c![i + 4, 0], ab21[i], 0));
        loop4!(i, vst1q_lane_f64(c![i + 4, 1], ab21[i], 1));

        loop4!(i, vst1q_lane_f64(c![i + 4, 2], ab22[i], 0));
        loop4!(i, vst1q_lane_f64(c![i + 4, 3], ab22[i], 1));
    }
}

#[inline]
unsafe fn kernel_fallback_impl(k: usize, alpha: T, a: *const T, b: *const T,
                                   beta: T, c: *mut T, rsc: isize, csc: isize)
{
    const MR: usize = KernelFallback::MR;
    const NR: usize = KernelFallback::NR;
    let mut ab: [[T; NR]; MR] = [[0.; NR]; MR];
    let mut a = a;
    let mut b = b;
    debug_assert_eq!(beta, 0., "Beta must be 0 or is not masked");

    // Compute matrix multiplication into ab[i][j]
    unroll_by!(4 => k, {
        loop4!(i, loop4!(j, ab[i][j] += at(a, i) * at(b, j)));

        a = a.offset(MR as isize);
        b = b.offset(NR as isize);
    });

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // set C = α A B
    loop4!(j, loop4!(i, *c![i, j] = alpha * ab[i][j]));
}

#[inline(always)]
unsafe fn at(ptr: *const T, i: usize) -> T {
    *ptr.offset(i as isize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::test::test_a_kernel;

    #[test]
    fn test_kernel_fallback_impl() {
        test_a_kernel::<KernelFallback, _>("kernel");
    }

    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    #[test]
    fn test_loop_m_n() {
        let mut m = [[0; 4]; KernelAvx::MR];
        loop_m!(i, loop4!(j, m[i][j] += 1));
        for arr in &m[..] {
            for elt in &arr[..] {
                assert_eq!(*elt, 1);
            }
        }
    }

    #[cfg(any(target_arch="aarch64"))]
    #[cfg(has_aarch64_simd)]
    mod test_kernel_aarch64 {
        use super::test_a_kernel;
        use super::super::*;
        #[cfg(feature = "std")]
        use std::println;

        macro_rules! test_arch_kernels_aarch64 {
            ($($feature_name:tt, $name:ident, $kernel_ty:ty),*) => {
                $(
                #[test]
                fn $name() {
                    if is_aarch64_feature_detected_!($feature_name) {
                        test_a_kernel::<$kernel_ty, _>(stringify!($name));
                    } else {
                        #[cfg(feature = "std")]
                        println!("Skipping, host does not have feature: {:?}", $feature_name);
                    }
                }
                )*
            }
        }

        test_arch_kernels_aarch64! {
            "neon", neon, KernelNeon
        }
    }

    #[cfg(any(target_arch="x86", target_arch="x86_64"))]
    mod test_kernel_x86 {
        use super::test_a_kernel;
        use super::super::*;
        #[cfg(feature = "std")]
        use std::println;
        macro_rules! test_arch_kernels_x86 {
            ($($feature_name:tt, $name:ident, $kernel_ty:ty),*) => {
                $(
                #[test]
                fn $name() {
                    if is_x86_feature_detected_!($feature_name) {
                        test_a_kernel::<$kernel_ty, _>(stringify!($name));
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
            "avx", avx, KernelAvx,
            "sse2", sse2, KernelSse2
        }
    }
}
