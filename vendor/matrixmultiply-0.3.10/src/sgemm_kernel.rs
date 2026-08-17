// Copyright 2016 - 2023 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::kernel::GemmKernel;
use crate::kernel::GemmSelect;
use crate::kernel::{U4, U8};
use crate::archparam;

#[cfg(target_arch="x86")]
use core::arch::x86::*;
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::*;
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
use crate::x86::{FusedMulAdd, AvxMulAdd, SMultiplyAdd};

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

type T = f32;

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
macro_rules! loop_m { ($i:ident, $e:expr) => { loop8!($i, $e) }; }
#[cfg(all(test, any(target_arch="x86", target_arch="x86_64")))]
macro_rules! loop_n { ($j:ident, $e:expr) => { loop8!($j, $e) }; }

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelAvx {
    type Elem = T;

    type MRTy = U8;
    type NRTy = U8;

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
    fn nc() -> usize { archparam::S_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::S_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::S_MC }

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
        c: *mut T, rsc: isize, csc: isize) {
        kernel_target_fma(k, alpha, a, b, beta, c, rsc, csc)
    }
}

#[cfg(any(target_arch="x86", target_arch="x86_64"))]
impl GemmKernel for KernelSse2 {
    type Elem = T;

    type MRTy = <KernelFallback as GemmKernel>::MRTy;
    type NRTy = <KernelFallback as GemmKernel>::NRTy;

    #[inline(always)]
    fn align_to() -> usize { 16 }

    #[inline(always)]
    fn always_masked() -> bool { KernelFallback::always_masked() }

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
        kernel_target_sse2(k, alpha, a, b, beta, c, rsc, csc)
    }
}


#[cfg(target_arch="aarch64")]
#[cfg(has_aarch64_simd)]
impl GemmKernel for KernelNeon {
    type Elem = T;

    type MRTy = U8;
    type NRTy = U8;

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

    type MRTy = U8;
    type NRTy = U4;

    #[inline(always)]
    fn align_to() -> usize { 0 }

    #[inline(always)]
    fn always_masked() -> bool { true }

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
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
#[target_feature(enable="sse2")]
unsafe fn kernel_target_sse2(k: usize, alpha: T, a: *const T, b: *const T,
                             beta: T, c: *mut T, rsc: isize, csc: isize)
{
    kernel_fallback_impl(k, alpha, a, b, beta, c, rsc, csc)
}

#[inline(always)]
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
unsafe fn kernel_x86_avx<MA>(k: usize, alpha: T, a: *const T, b: *const T,
                             beta: T, c: *mut T, rsc: isize, csc: isize)
    where MA: SMultiplyAdd,
{
    const MR: usize = KernelAvx::MR;
    const NR: usize = KernelAvx::NR;

    debug_assert_ne!(k, 0);

    let mut ab = [_mm256_setzero_ps(); MR];

    // this kernel can operate in either transposition (C = A B or C^T = B^T A^T)
    let prefer_row_major_c = rsc != 1;

    let (mut a, mut b) = if prefer_row_major_c { (a, b) } else { (b, a) };
    let (rsc, csc) = if prefer_row_major_c { (rsc, csc) } else { (csc, rsc) };

    macro_rules! shuffle_mask {
        ($z:expr, $y:expr, $x:expr, $w:expr) => {
            ($z << 6) | ($y << 4) | ($x << 2) | $w
        }
    }
    macro_rules! permute_mask {
        ($z:expr, $y:expr, $x:expr, $w:expr) => {
            ($z << 6) | ($y << 4) | ($x << 2) | $w
        }
    }

    macro_rules! permute2f128_mask {
        ($y:expr, $x:expr) => {
            (($y << 4) | $x)
        }
    }

    // Start data load before each iteration
    let mut av = _mm256_load_ps(a);
    let mut bv = _mm256_load_ps(b);

    // Compute A B
    unroll_by_with_last!(4 => k, is_last, {
        // We compute abij = ai bj
        //
        // Load b as one contiguous vector
        // Load a as striped vectors
        //
        // Shuffle the abij elements in order after the loop.
        //
        // Note this scheme copied and transposed from the BLIS 8x8 sgemm
        // microkernel.
        //
        // Our a indices are striped and our b indices are linear. In
        // the variable names below, we always have doubled indices so
        // for example a0246 corresponds to a vector of a0 a0 a2 a2 a4 a4 a6 a6.
        //
        // ab0246: ab2064: ab4602: ab6420:
        // ( ab00  ( ab20  ( ab40  ( ab60
        //   ab01    ab21    ab41    ab61
        //   ab22    ab02    ab62    ab42
        //   ab23    ab03    ab63    ab43
        //   ab44    ab64    ab04    ab24
        //   ab45    ab65    ab05    ab25
        //   ab66    ab46    ab26    ab06
        //   ab67 )  ab47 )  ab27 )  ab07 )
        //
        // ab1357: ab3175: ab5713: ab7531:
        // ( ab10  ( ab30  ( ab50  ( ab70
        //   ab11    ab31    ab51    ab71
        //   ab32    ab12    ab72    ab52
        //   ab33    ab13    ab73    ab53
        //   ab54    ab74    ab14    ab34
        //   ab55    ab75    ab15    ab35
        //   ab76    ab56    ab36    ab16
        //   ab77 )  ab57 )  ab37 )  ab17 )

        const PERM32_2301: i32 = permute_mask!(1, 0, 3, 2);
        const PERM128_30: i32 = permute2f128_mask!(0, 3);

        // _mm256_moveldup_ps(av):
        // vmovsldup ymm2, ymmword ptr [rax]
        //
        // Load and duplicate each even word:
        // ymm2 ← [a0 a0 a2 a2 a4 a4 a6 a6]
        //
        // _mm256_movehdup_ps(av):
        // vmovshdup ymm2, ymmword ptr [rax]
        //
        // Load and duplicate each odd word:
        // ymm2 ← [a1 a1 a3 a3 a5 a5 a7 a7]
        //

        let a0246 = _mm256_moveldup_ps(av); // Load: a0 a0 a2 a2 a4 a4 a6 a6
        let a2064 = _mm256_permute_ps(a0246, PERM32_2301);

        let a1357 = _mm256_movehdup_ps(av); // Load: a1 a1 a3 a3 a5 a5 a7 a7
        let a3175 = _mm256_permute_ps(a1357, PERM32_2301);

        let bv_lh = _mm256_permute2f128_ps(bv, bv, PERM128_30);

        ab[0] = MA::multiply_add(a0246, bv, ab[0]);
        ab[1] = MA::multiply_add(a2064, bv, ab[1]);
        ab[2] = MA::multiply_add(a0246, bv_lh, ab[2]);
        ab[3] = MA::multiply_add(a2064, bv_lh, ab[3]);

        ab[4] = MA::multiply_add(a1357, bv, ab[4]);
        ab[5] = MA::multiply_add(a3175, bv, ab[5]);
        ab[6] = MA::multiply_add(a1357, bv_lh, ab[6]);
        ab[7] = MA::multiply_add(a3175, bv_lh, ab[7]);

        if !is_last {
            a = a.add(MR);
            b = b.add(NR);

            bv = _mm256_load_ps(b);
            av = _mm256_load_ps(a);
        }
    });

    let alphav = _mm256_set1_ps(alpha);

    // Permute to put the abij elements in order
    //
    // shufps 0xe4: 22006644 00224466 -> 22226666
    //
    // vperm2 0x30: 00004444 44440000 -> 00000000
    // vperm2 0x12: 00004444 44440000 -> 44444444
    //
    
    let ab0246 = ab[0];
    let ab2064 = ab[1];
    let ab4602 = ab[2]; // reverse order
    let ab6420 = ab[3]; // reverse order

    let ab1357 = ab[4];
    let ab3175 = ab[5];
    let ab5713 = ab[6]; // reverse order
    let ab7531 = ab[7]; // reverse order

    const SHUF_0123: i32 = shuffle_mask!(3, 2, 1, 0);
    debug_assert_eq!(SHUF_0123, 0xE4);

    const PERM128_02: i32 = permute2f128_mask!(2, 0);
    const PERM128_31: i32 = permute2f128_mask!(1, 3);

    // No elements are "shuffled" in truth, they all stay at their index
    // but we combine vectors to de-stripe them.
    //
    // For example, the first shuffle below uses 0 1 2 3 which
    // corresponds to the X0 X1 Y2 Y3 sequence etc:
    //
    //                                             variable
    // X ab00 ab01 ab22 ab23 ab44 ab45 ab66 ab67   ab0246
    // Y ab20 ab21 ab02 ab03 ab64 ab65 ab46 ab47   ab2064
    // 
    //   X0   X1   Y2   Y3   X4   X5   Y6   Y7
    // = ab00 ab01 ab02 ab03 ab44 ab45 ab46 ab47   ab0044

    let ab0044 = _mm256_shuffle_ps(ab0246, ab2064, SHUF_0123);
    let ab2266 = _mm256_shuffle_ps(ab2064, ab0246, SHUF_0123);

    let ab4400 = _mm256_shuffle_ps(ab4602, ab6420, SHUF_0123);
    let ab6622 = _mm256_shuffle_ps(ab6420, ab4602, SHUF_0123);

    let ab1155 = _mm256_shuffle_ps(ab1357, ab3175, SHUF_0123);
    let ab3377 = _mm256_shuffle_ps(ab3175, ab1357, SHUF_0123);

    let ab5511 = _mm256_shuffle_ps(ab5713, ab7531, SHUF_0123);
    let ab7733 = _mm256_shuffle_ps(ab7531, ab5713, SHUF_0123);

    let ab0000 = _mm256_permute2f128_ps(ab0044, ab4400, PERM128_02);
    let ab4444 = _mm256_permute2f128_ps(ab0044, ab4400, PERM128_31);

    let ab2222 = _mm256_permute2f128_ps(ab2266, ab6622, PERM128_02);
    let ab6666 = _mm256_permute2f128_ps(ab2266, ab6622, PERM128_31);

    let ab1111 = _mm256_permute2f128_ps(ab1155, ab5511, PERM128_02);
    let ab5555 = _mm256_permute2f128_ps(ab1155, ab5511, PERM128_31);

    let ab3333 = _mm256_permute2f128_ps(ab3377, ab7733, PERM128_02);
    let ab7777 = _mm256_permute2f128_ps(ab3377, ab7733, PERM128_31);

    ab[0] = ab0000;
    ab[1] = ab1111;
    ab[2] = ab2222;
    ab[3] = ab3333;
    ab[4] = ab4444;
    ab[5] = ab5555;
    ab[6] = ab6666;
    ab[7] = ab7777;

    // Compute α (A B)
    // Compute here if we don't have fma, else pick up α further down
    if !MA::IS_FUSED {
        loop_m!(i, ab[i] = _mm256_mul_ps(alphav, ab[i]));
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // C ← α A B + β C
    let mut cv = [_mm256_setzero_ps(); MR];
    if beta != 0. {
        let betav = _mm256_set1_ps(beta);
        // Read C
        if csc == 1 {
            loop_m!(i, cv[i] = _mm256_loadu_ps(c![i, 0]));
        } else {
            loop_m!(i, cv[i] = _mm256_setr_ps(*c![i, 0], *c![i, 1], *c![i, 2], *c![i, 3],
                                              *c![i, 4], *c![i, 5], *c![i, 6], *c![i, 7]));
        }
        // Compute β C
        loop_m!(i, cv[i] = _mm256_mul_ps(cv[i], betav));
    }

    // Compute (α A B) + (β C)
    if !MA::IS_FUSED {
        loop_m!(i, cv[i] = _mm256_add_ps(cv[i], ab[i]));
    } else {
        loop_m!(i, cv[i] = MA::multiply_add(alphav, ab[i], cv[i]));
    }

    // Store C back to memory
    if csc == 1 {
        loop_m!(i, _mm256_storeu_ps(c![i, 0], cv[i]));
    } else {
        // Permute to bring each element in the vector to the front and store
        loop_m!(i, {
            let cvlo = _mm256_extractf128_ps(cv[i], 0);
            let cvhi = _mm256_extractf128_ps(cv[i], 1);

            _mm_store_ss(c![i, 0], cvlo);
            let cperm = _mm_permute_ps(cvlo, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 1], cperm);
            let cperm = _mm_permute_ps(cperm, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 2], cperm);
            let cperm = _mm_permute_ps(cperm, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 3], cperm);

            _mm_store_ss(c![i, 4], cvhi);
            let cperm = _mm_permute_ps(cvhi, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 5], cperm);
            let cperm = _mm_permute_ps(cperm, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 6], cperm);
            let cperm = _mm_permute_ps(cperm, permute_mask!(0, 3, 2, 1));
            _mm_store_ss(c![i, 7], cperm);
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

    let (mut a, mut b, rsc, csc) = if rsc == 1 { (b, a, csc, rsc) } else { (a, b, rsc, csc) };

    // Kernel 8 x 8 (a x b)
    // Four quadrants of 4 x 4
    let mut ab11 = [vmovq_n_f32(0.); 4];
    let mut ab12 = [vmovq_n_f32(0.); 4];
    let mut ab21 = [vmovq_n_f32(0.); 4];
    let mut ab22 = [vmovq_n_f32(0.); 4];

    // Compute
    // ab_ij = a_i * b_j for all i, j
    macro_rules! ab_ij_equals_ai_bj {
        ($dest:ident, $av:expr, $bv:expr) => {
            $dest[0] = vfmaq_laneq_f32($dest[0], $bv, $av, 0);
            $dest[1] = vfmaq_laneq_f32($dest[1], $bv, $av, 1);
            $dest[2] = vfmaq_laneq_f32($dest[2], $bv, $av, 2);
            $dest[3] = vfmaq_laneq_f32($dest[3], $bv, $av, 3);
        }
    }

    for _ in 0..k {
        let a1 = vld1q_f32(a);
        let b1 = vld1q_f32(b);
        let a2 = vld1q_f32(a.add(4));
        let b2 = vld1q_f32(b.add(4));

        // compute an outer product ab = a (*) b in four quadrants ab11, ab12, ab21, ab22

        // ab11: [a1 a2 a3 a4] (*) [b1 b2 b3 b4]
        // ab11: a1b1 a1b2 a1b3 a1b4
        //       a2b1 a2b2 a2b3 a2b4
        //       a3b1 a3b2 a3b3 a3b4
        //       a4b1 a4b2 a4b3 a4b4
        //  etc
        ab_ij_equals_ai_bj!(ab11, a1, b1);
        ab_ij_equals_ai_bj!(ab12, a1, b2);
        ab_ij_equals_ai_bj!(ab21, a2, b1);
        ab_ij_equals_ai_bj!(ab22, a2, b2);

        a = a.add(MR);
        b = b.add(NR);
    }

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // ab *= alpha
    loop4!(i, ab11[i] = vmulq_n_f32(ab11[i], alpha));
    loop4!(i, ab12[i] = vmulq_n_f32(ab12[i], alpha));
    loop4!(i, ab21[i] = vmulq_n_f32(ab21[i], alpha));
    loop4!(i, ab22[i] = vmulq_n_f32(ab22[i], alpha));

    // load one float32x4_t from four pointers
    macro_rules! loadq_from_pointers {
        ($p0:expr, $p1:expr, $p2:expr, $p3:expr) => (
            {
                let v = vld1q_dup_f32($p0);
                let v = vld1q_lane_f32($p1, v, 1);
                let v = vld1q_lane_f32($p2, v, 2);
                let v = vld1q_lane_f32($p3, v, 3);
                v
            }
        );
    }

    if beta != 0. {
        // load existing value in C
        let mut c11 = [vmovq_n_f32(0.); 4];
        let mut c12 = [vmovq_n_f32(0.); 4];
        let mut c21 = [vmovq_n_f32(0.); 4];
        let mut c22 = [vmovq_n_f32(0.); 4];

        if csc == 1 {
            loop4!(i, c11[i] = vld1q_f32(c![i + 0, 0]));
            loop4!(i, c12[i] = vld1q_f32(c![i + 0, 4]));
            loop4!(i, c21[i] = vld1q_f32(c![i + 4, 0]));
            loop4!(i, c22[i] = vld1q_f32(c![i + 4, 4]));
        } else {
            loop4!(i, c11[i] = loadq_from_pointers!(c![i + 0, 0], c![i + 0, 1], c![i + 0, 2], c![i + 0, 3]));
            loop4!(i, c12[i] = loadq_from_pointers!(c![i + 0, 4], c![i + 0, 5], c![i + 0, 6], c![i + 0, 7]));
            loop4!(i, c21[i] = loadq_from_pointers!(c![i + 4, 0], c![i + 4, 1], c![i + 4, 2], c![i + 4, 3]));
            loop4!(i, c22[i] = loadq_from_pointers!(c![i + 4, 4], c![i + 4, 5], c![i + 4, 6], c![i + 4, 7]));
        }

        let betav = vmovq_n_f32(beta);

        // ab += β C
        loop4!(i, ab11[i] = vfmaq_f32(ab11[i], c11[i], betav));
        loop4!(i, ab12[i] = vfmaq_f32(ab12[i], c12[i], betav));
        loop4!(i, ab21[i] = vfmaq_f32(ab21[i], c21[i], betav));
        loop4!(i, ab22[i] = vfmaq_f32(ab22[i], c22[i], betav));
    }

    // c <- ab
    // which is in full
    //   C <- α A B (+ β C)
    if csc == 1 {
        loop4!(i, vst1q_f32(c![i + 0, 0], ab11[i]));
        loop4!(i, vst1q_f32(c![i + 0, 4], ab12[i]));
        loop4!(i, vst1q_f32(c![i + 4, 0], ab21[i]));
        loop4!(i, vst1q_f32(c![i + 4, 4], ab22[i]));
    } else {
        loop4!(i, vst1q_lane_f32(c![i + 0, 0], ab11[i], 0));
        loop4!(i, vst1q_lane_f32(c![i + 0, 1], ab11[i], 1));
        loop4!(i, vst1q_lane_f32(c![i + 0, 2], ab11[i], 2));
        loop4!(i, vst1q_lane_f32(c![i + 0, 3], ab11[i], 3));

        loop4!(i, vst1q_lane_f32(c![i + 0, 4], ab12[i], 0));
        loop4!(i, vst1q_lane_f32(c![i + 0, 5], ab12[i], 1));
        loop4!(i, vst1q_lane_f32(c![i + 0, 6], ab12[i], 2));
        loop4!(i, vst1q_lane_f32(c![i + 0, 7], ab12[i], 3));

        loop4!(i, vst1q_lane_f32(c![i + 4, 0], ab21[i], 0));
        loop4!(i, vst1q_lane_f32(c![i + 4, 1], ab21[i], 1));
        loop4!(i, vst1q_lane_f32(c![i + 4, 2], ab21[i], 2));
        loop4!(i, vst1q_lane_f32(c![i + 4, 3], ab21[i], 3));

        loop4!(i, vst1q_lane_f32(c![i + 4, 4], ab22[i], 0));
        loop4!(i, vst1q_lane_f32(c![i + 4, 5], ab22[i], 1));
        loop4!(i, vst1q_lane_f32(c![i + 4, 6], ab22[i], 2));
        loop4!(i, vst1q_lane_f32(c![i + 4, 7], ab22[i], 3));
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

    // Compute A B into ab[i][j]
    unroll_by!(4 => k, {
        loop8!(i, loop4!(j, ab[i][j] += at(a, i) * at(b, j)));

        a = a.offset(MR as isize);
        b = b.offset(NR as isize);
    });

    macro_rules! c {
        ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
    }

    // set C = α A B
    loop4!(j, loop8!(i, *c![i, j] = alpha * ab[i][j]));
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
        let mut m = [[0; KernelAvx::NR]; KernelAvx::MR];
        loop_m!(i, loop_n!(j, m[i][j] += 1));
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
            "neon", neon8x8, KernelNeon
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

        #[test]
        fn ensure_target_features_tested() {
            // If enabled, this test ensures that the requested feature actually
            // was enabled on this configuration, so that it was tested.
            let should_ensure_feature = !option_env!("MMTEST_ENSUREFEATURE")
                                                    .unwrap_or("").is_empty();
            if !should_ensure_feature {
                // skip
                return;
            }
            let feature_name = option_env!("MMTEST_FEATURE")
                                          .expect("No MMTEST_FEATURE configured!");
            let detected = match feature_name {
                "avx" => is_x86_feature_detected_!("avx"),
                "fma" => is_x86_feature_detected_!("fma"),
                "sse2" => is_x86_feature_detected_!("sse2"),
                _ => false,
            };
            assert!(detected, "Feature {:?} was not detected, so it could not be tested",
                    feature_name);
        }
    }
}
