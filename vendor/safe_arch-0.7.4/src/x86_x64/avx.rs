#![cfg(target_feature = "avx")]

use super::*;

/// Lanewise `a + b` with `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn add_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_add_pd(a.0, b.0) })
}

/// Lanewise `a + b` with `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn add_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_add_ps(a.0, b.0) })
}

/// Alternately, from the top, add `f64` then sub `f64`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn addsub_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_addsub_pd(a.0, b.0) })
}

/// Alternately, from the top, add `f32` then sub `f32`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn addsub_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_addsub_ps(a.0, b.0) })
}

/// Bitwise `a & b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitand_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_and_pd(a.0, b.0) })
}

/// Bitwise `a & b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitand_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_and_ps(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitandnot_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_andnot_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitandnot_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_andnot_ps(a.0, b.0) })
}

/// Blends the `f64` lanes according to the immediate mask.
///
/// Each bit 0 though 3 controls output lane 0 through 3. Use 0 for the `a`
/// value and 1 for the `b` value.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn blend_m256d<const IMM: i32>(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_blend_pd(a.0, b.0, IMM) })
}

/// Blends the `f32` lanes according to the immediate mask.
///
/// Each bit 0 though 7 controls lane 0 through 7. Use 0 for the `$a` value and
/// 1 for the `$b` value.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn blend_m256<const IMM: i32>(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_blend_ps(a.0, b.0, IMM) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn blend_varying_m256d(a: m256d, b: m256d, mask: m256d) -> m256d {
  m256d(unsafe { _mm256_blendv_pd(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn blend_varying_m256(a: m256, b: m256, mask: m256) -> m256 {
  m256(unsafe { _mm256_blendv_ps(a.0, b.0, mask.0) })
}

/// Load an `m128d` and splat it to the lower and upper half of an `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_m128d_splat_m256d(a: &m128d) -> m256d {
  m256d(unsafe { _mm256_broadcast_pd(&a.0) })
}

/// Load an `m128` and splat it to the lower and upper half of an `m256`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_m128_splat_m256(a: &m128) -> m256 {
  m256(unsafe { _mm256_broadcast_ps(&a.0) })
}

/// Load an `f64` and splat it to all lanes of an `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_f64_splat_m256d(a: &f64) -> m256d {
  m256d(unsafe { _mm256_broadcast_sd(a) })
}

/// Load an `f32` and splat it to all lanes of an `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_f32_splat_m256(a: &f32) -> m256 {
  m256(unsafe { _mm256_broadcast_ss(a) })
}

/// Bit-preserving cast to `m256` from `m256d`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256_from_m256d(a: m256d) -> m256 {
  m256(unsafe { _mm256_castpd_ps(a.0) })
}

/// Bit-preserving cast to `m256i` from `m256d`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256i_from_m256d(a: m256d) -> m256i {
  m256i(unsafe { _mm256_castpd_si256(a.0) })
}

/// Bit-preserving cast to `m256i` from `m256`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256d_from_m256(a: m256) -> m256d {
  m256d(unsafe { _mm256_castps_pd(a.0) })
}

/// Bit-preserving cast to `m256i` from `m256`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256i_from_m256(a: m256) -> m256i {
  m256i(unsafe { _mm256_castps_si256(a.0) })
}

/// Bit-preserving cast to `m256d` from `m256i`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256d_from_m256i(a: m256i) -> m256d {
  m256d(unsafe { _mm256_castsi256_pd(a.0) })
}

/// Bit-preserving cast to `m256` from `m256i`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m256_from_m256i(a: m256i) -> m256 {
  m256(unsafe { _mm256_castsi256_ps(a.0) })
}

/// Bit-preserving cast to `m128` from `m256`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m128_from_m256(a: m256) -> m128 {
  m128(unsafe { _mm256_castps256_ps128(a.0) })
}

/// Bit-preserving cast to `m128d` from `m256d`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m128d_from_m256d(a: m256d) -> m128d {
  m128d(unsafe { _mm256_castpd256_pd128(a.0) })
}

/// Bit-preserving cast to `m128i` from `m256i`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cast_to_m128i_from_m256i(a: m256i) -> m128i {
  m128i(unsafe { _mm256_castsi256_si128(a.0) })
}

/// Round `f64` lanes towards positive infinity.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn ceil_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_ceil_pd(a.0) })
}

/// Round `f32` lanes towards positive infinity.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn ceil_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_ceil_ps(a.0) })
}

/// Turns a comparison operator token to the correct constant value.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op {
  (EqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_EQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_EQ_OQ;
    _CMP_EQ_OQ
  }};
  (EqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_EQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_EQ_UQ;
    _CMP_EQ_UQ
  }};
  (False) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_FALSE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_FALSE_OQ;
    _CMP_FALSE_OQ
  }};
  (GreaterEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_GE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_GE_OQ;
    _CMP_GE_OQ
  }};
  (GreaterThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_GT_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_GT_OQ;
    _CMP_GT_OQ
  }};
  (LessEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_LE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_LE_OQ;
    _CMP_LE_OQ
  }};
  (LessThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_LT_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_LT_OQ;
    _CMP_LT_OQ
  }};
  (NotEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NEQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NEQ_OQ;
    _CMP_NEQ_OQ
  }};
  (NotEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NEQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NEQ_UQ;
    _CMP_NEQ_UQ
  }};
  (NotGreaterEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NGE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NGE_UQ;
    _CMP_NGE_UQ
  }};
  (NotGreaterThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NGT_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NGT_UQ;
    _CMP_NGT_UQ
  }};
  (NotLessEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NLE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NLE_UQ;
    _CMP_NLE_UQ
  }};
  (NotLessThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NLT_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NLT_UQ;
    _CMP_NLT_UQ
  }};
  (Ordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_ORD_Q;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_ORD_Q;
    _CMP_ORD_Q
  }};
  (True) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_TRUE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_TRUE_UQ;
    _CMP_TRUE_UQ
  }};
  (Unordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_UNORD_Q;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_UNORD_Q;
    _CMP_UNORD_Q
  }};
  ($unknown_op:tt) => {{
    compile_error!("The operation name given is invalid.");
  }};
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m128<const OP: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmp_ps(a.0, b.0, OP) })
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m128_s<const OP: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmp_ss(a.0, b.0, OP) })
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m256<const OP: i32>(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_cmp_ps(a.0, b.0, OP) })
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m128d<const OP: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmp_pd(a.0, b.0, OP) })
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m128d_s<const OP: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmp_sd(a.0, b.0, OP) })
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`cmp_op`] macro.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn cmp_op_mask_m256d<const OP: i32>(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_cmp_pd(a.0, b.0, OP) })
}

/// Convert `i32` lanes to be `f64` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtepi32_pd`]
/// * **Assembly:** `vcvtdq2pd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256d_from_i32_m128i(a: m128i) -> m256d {
  m256d(unsafe { _mm256_cvtepi32_pd(a.0) })
}

/// Convert `i32` lanes to be `f32` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtepi32_ps`]
/// * **Assembly:** `vcvtdq2ps ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256_from_i32_m256i(a: m256i) -> m256 {
  m256(unsafe { _mm256_cvtepi32_ps(a.0) })
}

/// Convert `f64` lanes to be `i32` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtpd_epi32`]
/// * **Assembly:** `vcvtpd2dq xmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_m128i_from_m256d(a: m256d) -> m128i {
  m128i(unsafe { _mm256_cvtpd_epi32(a.0) })
}

/// Convert `f64` lanes to be `f32` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtpd_ps`]
/// * **Assembly:** `vcvtpd2ps xmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m128_from_m256d(a: m256d) -> m128 {
  m128(unsafe { _mm256_cvtpd_ps(a.0) })
}

/// Convert `f32` lanes to be `i32` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtps_epi32`]
/// * **Assembly:** `vcvtps2dq ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_m256i_from_m256(a: m256) -> m256i {
  m256i(unsafe { _mm256_cvtps_epi32(a.0) })
}

/// Convert `f32` lanes to be `f64` lanes.
///
/// * **Intrinsic:** [`_mm256_cvtps_pd`]
/// * **Assembly:** `vcvtps2pd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256d_from_m128(a: m128) -> m256d {
  m256d(unsafe { _mm256_cvtps_pd(a.0) })
}

/// Convert the lowest `f64` lane to a single `f64`.
///
/// * **Intrinsic:** [`_mm256_cvtsd_f64`]
/// * **Assembly:** `vmovsd m64, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_f64_from_m256d_s(a: m256d) -> f64 {
  unsafe { _mm256_cvtsd_f64(a.0) }
}

/// Convert the lowest `i32` lane to a single `i32`.
///
/// * **Intrinsic:** [`_mm256_cvtsi256_si32`]
/// * **Assembly:** `vmovd r32, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_from_m256i_s(a: m256i) -> i32 {
  unsafe { _mm256_cvtsi256_si32(a.0) }
}

/// Convert the lowest `f32` lane to a single `f32`.
///
/// * **Intrinsic:** [`_mm256_cvtss_f32`]
/// * **Assembly:** `vmovss m32, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_f32_from_m256_s(a: m256) -> f32 {
  unsafe { _mm256_cvtss_f32(a.0) }
}

/// Convert `f64` lanes to `i32` lanes with truncation.
///
/// * **Intrinsic:** [`_mm256_cvttpd_epi32`]
/// * **Assembly:** `vcvttpd2dq xmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_truncate_to_i32_m128i_from_m256d(a: m256d) -> m128i {
  m128i(unsafe { _mm256_cvttpd_epi32(a.0) })
}

/// Convert `f32` lanes to `i32` lanes with truncation.
///
/// * **Intrinsic:** [`_mm256_cvttps_epi32`]
/// * **Assembly:** ``
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn convert_truncate_to_i32_m256i_from_m256(a: m256) -> m256i {
  m256i(unsafe { _mm256_cvttps_epi32(a.0) })
}

/// Lanewise `a / b` with `f64`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn div_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_div_pd(a.0, b.0) })
}

/// Lanewise `a / b` with `f32`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn div_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_div_ps(a.0, b.0) })
}

/// This works like [`dot_product_m128`], but twice as wide.
///
/// The given control is used for the lower 4 lanes and then separately also the
/// upper four lanes. See the other macro for more info on how the control
/// works.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn dot_product_m256<const IMM: i32>(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_dp_ps(a.0, b.0, IMM) })
}

/// Extracts an `i32` lane from `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn extract_i32_from_m256i<const IMM: i32>(a: m256i) -> i32 {
  unsafe { _mm256_extract_epi32(a.0, IMM) }
}

/// Extracts an `i64` lane from `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[cfg(target_arch = "x86_64")]
pub fn extract_i64_from_m256i<const IMM: i32>(a: m256i) -> i64 {
  unsafe { _mm256_extract_epi64(a.0, IMM) }
}

/// Extracts an `m128d` from `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn extract_m128d_from_m256d<const IMM: i32>(a: m256d) -> m128d {
  m128d(unsafe { _mm256_extractf128_pd(a.0, IMM) })
}

/// Extracts an `m128` from `m256`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn extract_m128_from_m256<const IMM: i32>(a: m256) -> m128 {
  m128(unsafe { _mm256_extractf128_ps(a.0, IMM) })
}

/// Extracts an `m128i` from `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn extract_m128i_from_m256i<const IMM: i32>(a: m256i) -> m128i {
  m128i(unsafe { _mm256_extractf128_si256(a.0, IMM) })
}

/// Round `f64` lanes towards negative infinity.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn floor_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_floor_pd(a.0) })
}

/// Round `f32` lanes towards negative infinity.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn floor_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_floor_ps(a.0) })
}

/// Add adjacent `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn add_horizontal_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_hadd_pd(a.0, b.0) })
}

/// Add adjacent `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn add_horizontal_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_hadd_ps(a.0, b.0) })
}

/// Subtract adjacent `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sub_horizontal_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_hsub_pd(a.0, b.0) })
}

/// Subtract adjacent `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sub_horizontal_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_hsub_ps(a.0, b.0) })
}

/// Inserts an `i8` to `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_i8_to_m256i<const IMM: i32>(a: m256i, i: i8) -> m256i {
  m256i(unsafe { _mm256_insert_epi8(a.0, i, IMM) })
}

/// Inserts an `i16` to `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_i16_to_m256i<const IMM: i32>(a: m256i, i: i16) -> m256i {
  m256i(unsafe { _mm256_insert_epi16(a.0, i, IMM) })
}

/// Inserts an `i32` to `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_i32_to_m256i<const IMM: i32>(a: m256i, i: i32) -> m256i {
  m256i(unsafe { _mm256_insert_epi32(a.0, i, IMM) })
}

/// Inserts an `i64` to `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[cfg(target_arch = "x86_64")]
pub fn insert_i64_to_m256i<const IMM: i32>(a: m256i, i: i64) -> m256i {
  m256i(unsafe { _mm256_insert_epi64(a.0, i, IMM) })
}

/// Inserts an `m128d` to `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_m128d_to_m256d<const IMM: i32>(a: m256d, b: m128d) -> m256d {
  m256d(unsafe { _mm256_insertf128_pd(a.0, b.0, IMM) })
}

/// Inserts an `m128` to `m256`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_m128_to_m256<const IMM: i32>(a: m256, b: m128) -> m256 {
  m256(unsafe { _mm256_insertf128_ps(a.0, b.0, IMM) })
}

/// Slowly inserts an `m128i` to `m256i`.
///
/// This is a "historical artifact" that was potentially useful if you have AVX
/// but not AVX2. If you plan on having AVX2 available please use
/// [`insert_m128i_to_m256i`], it will do the same task with better performance.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn insert_m128i_to_m256i_slow_avx<const IMM: i32>(a: m256i, b: m128i) -> m256i {
  m256i(unsafe { _mm256_insertf128_si256(a.0, b.0, IMM) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_m256d(a: &m256d) -> m256d {
  m256d(unsafe { _mm256_load_pd(a as *const m256d as *const f64) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_m256(a: &m256) -> m256 {
  m256(unsafe { _mm256_load_ps(a as *const m256 as *const f32) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_m256i(a: &m256i) -> m256i {
  m256i(unsafe { _mm256_load_si256(a as *const m256i as *const __m256i) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256d(a: &[f64; 4]) -> m256d {
  m256d(unsafe { _mm256_loadu_pd(a as *const [f64; 4] as *const f64) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256(a: &[f32; 8]) -> m256 {
  m256(unsafe { _mm256_loadu_ps(a as *const [f32; 8] as *const f32) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256i(a: &[i8; 32]) -> m256i {
  m256i(unsafe { _mm256_loadu_si256(a as *const [i8; 32] as *const __m256i) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256d(a: &[f64; 2], b: &[f64; 2]) -> m256d {
  m256d(unsafe { _mm256_loadu2_m128d(a as *const [f64; 2] as *const f64, b as *const [f64; 2] as *const f64) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256(a: &[f32; 4], b: &[f32; 4]) -> m256 {
  m256(unsafe { _mm256_loadu2_m128(a as *const [f32; 4] as *const f32, b as *const [f32; 4] as *const f32) })
}

/// Load data from memory into a register.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256i(a: &[i8; 16], b: &[i8; 16]) -> m256i {
  m256i(unsafe { _mm256_loadu2_m128i(a as *const [i8; 16] as *const __m128i, b as *const [i8; 16] as *const __m128i) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m128d(a: &m128d, mask: m128i) -> m128d {
  m128d(unsafe { _mm_maskload_pd(a as *const m128d as *const f64, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m256d(a: &m256d, mask: m256i) -> m256d {
  m256d(unsafe { _mm256_maskload_pd(a as *const m256d as *const f64, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m128(a: &m128, mask: m128i) -> m128 {
  m128(unsafe { _mm_maskload_ps(a as *const m128 as *const f32, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m256(a: &m256, mask: m256i) -> m256 {
  m256(unsafe { _mm256_maskload_ps(a as *const m256 as *const f32, mask.0) })
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m128d(addr: &mut m128d, mask: m128i, a: m128d) {
  unsafe { _mm_maskstore_pd(addr as *mut m128d as *mut f64, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m256d(addr: &mut m256d, mask: m256i, a: m256d) {
  unsafe { _mm256_maskstore_pd(addr as *mut m256d as *mut f64, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m128(addr: &mut m128, mask: m128i, a: m128) {
  unsafe { _mm_maskstore_ps(addr as *mut m128 as *mut f32, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m256(addr: &mut m256, mask: m256i, a: m256) {
  unsafe { _mm256_maskstore_ps(addr as *mut m256 as *mut f32, mask.0, a.0) }
}

/// Lanewise `max(a, b)`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn max_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_max_pd(a.0, b.0) })
}

/// Lanewise `max(a, b)`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn max_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_max_ps(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn min_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_min_pd(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn min_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_min_ps(a.0, b.0) })
}

/// Duplicate the odd-indexed lanes to the even lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_odd_lanes_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_movedup_pd(a.0) })
}

/// Duplicate the even-indexed lanes to the odd lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_even_lanes_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_movehdup_ps(a.0) })
}

/// Duplicate the odd-indexed lanes to the even lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_odd_lanes_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_moveldup_ps(a.0) })
}

/// Collects the sign bit of each lane into a 4-bit value.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn move_mask_m256d(a: m256d) -> i32 {
  unsafe { _mm256_movemask_pd(a.0) }
}

/// Computes the bitwise AND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testz_ps`]
/// * **Assembly:** vtestps ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testz_m256(a: m256, b: m256) -> i32 {
  unsafe { _mm256_testz_ps(a.0, b.0) }
}

/// Computes the bitwise AND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testz_ps`]
/// * **Assembly:** vtestps xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testz_m128(a: m128, b: m128) -> i32 {
  unsafe { _mm_testz_ps(a.0, b.0) }
}

/// Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testc_ps`]
/// * **Assembly:** vtestps ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testc_m256(a: m256, b: m256) -> i32 {
  unsafe { _mm256_testc_ps(a.0, b.0) }
}

/// Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testc_ps`]
/// * **Assembly:** vtestps xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testc_m128(a: m128, b: m128) -> i32 {
  unsafe { _mm_testc_ps(a.0, b.0) }
}

/// Computes the bitwise of sign bit AND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testz_pd`]
/// * **Assembly:** vtestpd ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testz_m256d(a: m256d, b: m256d) -> i32 {
  unsafe { _mm256_testz_pd(a.0, b.0) }
}

/// Computes the bitwise of sign bitAND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testz_pd`]
/// * **Assembly:** vtestpd xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testz_m128d(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_testz_pd(a.0, b.0) }
}

/// Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testc_pd`]
/// * **Assembly:** vtestpd ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testc_m256d(a: m256d, b: m256d) -> i32 {
  unsafe { _mm256_testc_pd(a.0, b.0) }
}

/// Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testc_si128`]
/// * **Assembly:** vptest xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testc_m128d(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_testc_pd(a.0, b.0) }
}

/// Computes the bitwise of sign bit AND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testz_si256`]
/// * **Assembly:** vptest ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testz_m256i(a: m256i, b: m256i) -> i32 {
  unsafe { _mm256_testz_si256(a.0, b.0) }
}

/// Compute the bitwise NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm256_testc_si256`]
/// * **Assembly:** vptest ymm, ymm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn testc_m256i(a: m256i, b: m256i) -> i32 {
  unsafe { _mm256_testc_si256(a.0, b.0) }
}

/// Collects the sign bit of each lane into a 4-bit value.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn move_mask_m256(a: m256) -> i32 {
  unsafe { _mm256_movemask_ps(a.0) }
}

/// Lanewise `a * b` with `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn mul_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_mul_pd(a.0, b.0) })
}

/// Lanewise `a * b` with `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn mul_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_mul_ps(a.0, b.0) })
}

/// Bitwise `a | b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitor_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_or_pd(a.0, b.0) })
}

/// Bitwise `a | b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitor_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_or_ps(a.0, b.0) })
}

/// Shuffle the `f64` lanes in `a` using an immediate control value.
///
/// * **Intrinsic:** [`_mm_permute_pd`]
/// * **Assembly:** `vpermilpd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute_m128d<const MASK: i32>(a: m128d) -> m128d {
  m128d(unsafe { _mm_permute_pd(a.0, MASK) })
}

/// Shuffle the `f64` lanes from `a` together using an immediate
/// control value.
///
/// * **Intrinsic:** [`_mm256_permute_pd`]
/// * **Assembly:** `vpermilpd ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute_m256d<const MASK: i32>(a: m256d) -> m256d {
  m256d(unsafe { _mm256_permute_pd(a.0, MASK) })
}

/// Shuffle the `f32` lanes from `a` using an immediate control value.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute_m128<const MASK: i32>(a: m128) -> m128 {
  m128(unsafe { _mm_permute_ps(a.0, MASK) })
}

/// Shuffle the `f32` lanes in `a` using an immediate control value.
///
/// * **Intrinsic:** [`_mm256_permute_ps`]
/// * **Assembly:** `vpermilps ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute_m256<const MASK: i32>(a: m256) -> m256 {
  m256(unsafe { _mm256_permute_ps(a.0, MASK) })
}

/// Shuffle 128 bits of floating point data at a time from `a` and `b` using an
/// immediate control value.
///
/// Each output selection is 4-bit wide, if `1000` is passed, that output is
/// zeroed instead of picking from `a` or `b`.
///
/// * **Intrinsic:** [`_mm256_permute2f128_pd`]
/// * **Assembly:** `vperm2f128 ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute2z_m256d<const MASK: i32>(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_permute2f128_pd(a.0, b.0, MASK) })
}

/// Shuffle 128 bits of floating point data at a time from `$a` and `$b` using
/// an immediate control value.
///
/// Each output selection is 4-bit wide, if `1000` is passed, that output is
/// zeroed instead of picking from `a` or `b`.
///
/// * **Intrinsic:** [`_mm256_permute2f128_ps`]
/// * **Assembly:** `vperm2f128 ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute2z_m256<const MASK: i32>(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_permute2f128_ps(a.0, b.0, MASK) })
}

/// *Slowly* swizzle 128 bits of integer data from `a` and `b` using an
/// immediate control value.
///
/// Each output selection is 4-bit wide, if `1000` is passed, that output is
/// zeroed instead of picking from `a` or `b`.
///
/// If `avx2` is available you should use [`shuffle_abi_i128z_all_m256i`]
/// instead. Only use this if you're targeting `avx` but not `avx2`.
///
/// * **Intrinsic:** [`_mm256_permute2f128_si256`]
/// * **Assembly:** `vperm2f128 ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn permute2z_m256i<const MASK: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_permute2f128_si256(a.0, b.0, MASK) })
}

/// Shuffle `f64` lanes in `a` using **bit 1** of the `i64` lanes in `v`
///
/// * **Intrinsic:** [`_mm_permutevar_pd`]
/// * **Assembly:** `vpermilpd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_av_f64_all_m128d(a: m128d, v: m128i) -> m128d {
  m128d(unsafe { _mm_permutevar_pd(a.0, v.0) })
}

/// Shuffle `f64` lanes in `a` using **bit 1** of the `i64` lanes in `v`.
///
/// Each lane selection value picks only within that 128-bit half of the overall
/// register.
///
/// * **Intrinsic:** [`_mm256_permutevar_pd`]
/// * **Assembly:** `vpermilpd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_av_f64_half_m256d(a: m256d, b: m256i) -> m256d {
  m256d(unsafe { _mm256_permutevar_pd(a.0, b.0) })
}

/// Shuffle `f32` values in `a` using `i32` values in `v`.
///
/// * **Intrinsic:** [`_mm_permutevar_ps`]
/// * **Assembly:** `vpermilps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_av_f32_all_m128(a: m128, v: m128i) -> m128 {
  m128(unsafe { _mm_permutevar_ps(a.0, v.0) })
}

/// Shuffle `f32` values in `a` using `i32` values in `v`.
///
/// Each lane selection value picks only within that 128-bit half of the overall
/// register.
///
/// * **Intrinsic:** [`_mm256_permutevar_ps`]
/// * **Assembly:** `vpermilps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_av_f32_half_m256(a: m256, v: m256i) -> m256 {
  m256(unsafe { _mm256_permutevar_ps(a.0, v.0) })
}

/// Reciprocal of `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn reciprocal_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_rcp_ps(a.0) })
}

/// Rounds each lane in the style specified.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn round_m256d<const OP: i32>(a: m256d) -> m256d {
  m256d(unsafe { _mm256_round_pd(a.0, OP) })
}

/// Rounds each lane in the style specified.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn round_m256<const OP: i32>(a: m256) -> m256 {
  m256(unsafe { _mm256_round_ps(a.0, OP) })
}

/// Reciprocal of `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn reciprocal_sqrt_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_rsqrt_ps(a.0) })
}

/// Set `i8` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i8_m256i(
  e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8, e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8, e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi8(
      e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0
    )
  })
}

/// Set `i16` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i16_m256i(
  e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16, e9: i16, e8: i16,
  e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi16(
      e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    )
  })
}

/// Set `i32` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i32_m256i(
  e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32, e0: i32,
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi32(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Set `i64` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_i64_m256i(e3: i64, e2: i64, e1: i64, e0: i64) -> m256i {
  m256i(unsafe { _mm256_set_epi64x(e3, e2, e1, e0) })
}

/// Set `m128` args into an `m256`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_m128_m256(high: m128, low: m128) -> m256 {
  m256(unsafe { _mm256_set_m128(high.0, low.0) })
}

/// Set `m128d` args into an `m256d`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m128d_m256d(
  high: m128d, low: m128d
) -> m256d {
  m256d(unsafe { _mm256_set_m128d(high.0, low.0) })
}

/// Set `m128i` args into an `m256i`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m128i_m256i(
  hi: m128i, lo: m128i
) -> m256i {
  m256i(unsafe { _mm256_set_m128i(hi.0, lo.0) })
}

/// Set `f64` args into an `m256d` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m256d(
  e3: f64, e2: f64, e1: f64, e0: f64,
) -> m256d {
  m256d(unsafe { _mm256_set_pd(e3, e2, e1, e0) })
}

/// Set `f32` args into an `m256` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m256(
  e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32, e0: f32,
) -> m256 {
  m256(unsafe {
    _mm256_set_ps(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Splat an `i8` arg into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i8_m256i(i: i8) -> m256i {
  m256i(unsafe { _mm256_set1_epi8(i) })
}

/// Splat an `i16` arg into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i16_m256i(i: i16) -> m256i {
  m256i(unsafe { _mm256_set1_epi16(i) })
}

/// Splat an `i32` arg into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i32_m256i(i: i32) -> m256i {
  m256i(unsafe { _mm256_set1_epi32(i) })
}

/// Splat an `i64` arg into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i64_m256i(i: i64) -> m256i {
  m256i(unsafe { _mm256_set1_epi64x(i) })
}

/// Splat an `f64` arg into an `m256d` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_m256d(f: f64) -> m256d {
  m256d(unsafe { _mm256_set1_pd(f) })
}

/// Splat an `f32` arg into an `m256` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_splat_m256(
  f: f32,
) -> m256 {
  m256(unsafe {
    _mm256_set1_ps(f)
  })
}

/// Set `i8` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i8_m256i(
  e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8, e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8, e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi8(
      e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0
    )
  })
}

/// Set `i16` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i16_m256i(
  e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16, e9: i16, e8: i16,
  e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi16(
      e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    )
  })
}

/// Set `i32` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i32_m256i(
  e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32, e0: i32,
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi32(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Set `i64` args into an `m256i` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_reversed_i64_m256i(e3: i64, e2: i64, e1: i64, e0: i64) -> m256i {
  m256i(unsafe { _mm256_setr_epi64x(e3, e2, e1, e0) })
}

/// Set `m128` args into an `m256`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn set_reversed_m128_m256(hi: m128, lo: m128) -> m256 {
  m256(unsafe { _mm256_setr_m128(hi.0, lo.0) })
}

/// Set `m128d` args into an `m256d`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m128d_m256d(
  hi: m128d, lo: m128d
) -> m256d {
  m256d(unsafe { _mm256_setr_m128d(hi.0, lo.0) })
}

/// Set `m128i` args into an `m256i`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m128i_m256i(
  hi: m128i, lo: m128i
) -> m256i {
  m256i(unsafe { _mm256_setr_m128i(hi.0, lo.0) })
}

/// Set `f64` args into an `m256d` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m256d(
  e3: f64, e2: f64, e1: f64, e0: f64,
) -> m256d {
  m256d(unsafe { _mm256_setr_pd(e3, e2, e1, e0) })
}

/// Set `f32` args into an `m256` lane.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m256(
  e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32, e0: f32,
) -> m256 {
  m256(unsafe {
    _mm256_setr_ps(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// A zeroed `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256d() -> m256d {
  m256d(unsafe { _mm256_setzero_pd() })
}

/// A zeroed `m256`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256() -> m256 {
  m256(unsafe { _mm256_setzero_ps() })
}

/// A zeroed `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256i() -> m256i {
  m256i(unsafe { _mm256_setzero_si256() })
}

/// Shuffle the `f64` lanes from `a` and `b` together using an immediate control
/// value.
///
/// The control value uses the lowest 4 bits only.
/// * bit 0 picks between lanes 0 or 1 from A.
/// * bit 1 picks between lanes 0 or 1 from B.
/// * bit 2 picks between lanes 2 or 3 from A.
/// * bit 3 picks between lanes 2 or 3 from B.
///
/// Note that this shuffle cannot move data between the lower half of the lanes
/// and the upper half of the lanes.
///
/// * **Intrinsic:** [`_mm256_shuffle_pd`]
/// * **Assembly:** `vshufpd ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_m256d<const IMM: i32>(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_shuffle_pd(a.0, b.0, IMM) })
}

/// Shuffle the `f32` lanes from `a` and `b` together using an immediate
/// control value.
///
/// This works like [`shuffle_abi_f32_all_m128`], but with the low 128 bits and
/// high 128 bits each doing a shuffle at the same time. Each index (`0..=3`)
/// only refers to a lane within a given 128 bit portion of the 256 bit inputs.
/// You cannot cross data between the two 128 bit halves.
///
/// * **Intrinsic:** [`_mm256_shuffle_ps`]
/// * **Assembly:** `vshufps ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn shuffle_m256<const IMM: i32>(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_shuffle_ps(a.0, b.0, IMM) })
}

/// Lanewise `sqrt` on `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sqrt_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_sqrt_pd(a.0) })
}

/// Lanewise `sqrt` on `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sqrt_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_sqrt_ps(a.0) })
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_m256d(addr: &mut m256d, a: m256d) {
  unsafe { _mm256_store_pd(addr as *mut m256d as *mut f64, a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_m256(addr: &mut m256, a: m256) {
  unsafe { _mm256_store_ps(addr as *mut m256 as *mut f32, a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_m256i(addr: &mut m256i, a: m256i) {
  unsafe { _mm256_store_si256(addr as *mut m256i as *mut __m256i, a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256d(addr: &mut [f64; 4], a: m256d) {
  unsafe { _mm256_storeu_pd(addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256(addr: &mut [f32; 8], a: m256) {
  unsafe { _mm256_storeu_ps(addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256i(addr: &mut [i8; 32], a: m256i) {
  unsafe { _mm256_storeu_si256(addr as *mut [i8; 32] as *mut __m256i, a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256d(hi_addr: &mut [f64; 2], lo_addr: &mut [f64; 2], a: m256d) {
  unsafe { _mm256_storeu2_m128d(hi_addr.as_mut_ptr(), lo_addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256(hi_addr: &mut [f32; 4], lo_addr: &mut [f32; 4], a: m256) {
  unsafe { _mm256_storeu2_m128(hi_addr.as_mut_ptr(), lo_addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256i(hi_addr: &mut [i8; 16], lo_addr: &mut [i8; 16], a: m256i) {
  unsafe { _mm256_storeu2_m128i(hi_addr.as_mut_ptr().cast(), lo_addr.as_mut_ptr().cast(), a.0) }
}

/// Lanewise `a - b` with `f64` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sub_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_sub_pd(a.0, b.0) })
}

/// Lanewise `a - b` with `f32` lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn sub_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_sub_ps(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn unpack_hi_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_unpackhi_pd(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn unpack_hi_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_unpackhi_ps(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn unpack_lo_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_unpacklo_pd(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn unpack_lo_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_unpacklo_ps(a.0, b.0) })
}

/// Bitwise `a ^ b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitxor_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_xor_pd(a.0, b.0) })
}

/// Bitwise `a ^ b`.
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn bitxor_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_xor_ps(a.0, b.0) })
}

/// Zero extend an `m128d` to `m256d`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128d(a: m128d) -> m256d {
  m256d(unsafe { _mm256_zextpd128_pd256(a.0) })
}

/// Zero extend an `m128` to `m256`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128(a: m128) -> m256 {
  m256(unsafe { _mm256_zextps128_ps256(a.0) })
}

/// Zero extend an `m128i` to `m256i`
///
/// * **Intrinsic:** [``]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_zextsi128_si256(a.0) })
}

impl Add for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    add_m256d(self, rhs)
  }
}
impl AddAssign for m256d {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m256d(self, rhs)
  }
}
impl BitAndAssign for m256d {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m256d(self, rhs)
  }
}
impl BitOrAssign for m256d {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m256d(self, rhs)
  }
}
impl BitXorAssign for m256d {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    div_m256d(self, rhs)
  }
}
impl DivAssign for m256d {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    mul_m256d(self, rhs)
  }
}
impl MulAssign for m256d {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn neg(self) -> Self {
    sub_m256d(zeroed_m256d(), self)
  }
}

impl Not for m256d {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-ones bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m256d(f64::from_bits(u64::MAX));
    self ^ all_bits
  }
}

impl Sub for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    sub_m256d(self, rhs)
  }
}
impl SubAssign for m256d {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl PartialEq for m256d {
  /// Performs a comparison to get a mask, then moves the mask and checks for
  /// all true.
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    let mask = m256d(unsafe { _mm256_cmp_pd(self.0, other.0, _CMP_EQ_OQ) });
    move_mask_m256d(mask) == 0b1111
  }
}

impl Add for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    add_m256(self, rhs)
  }
}
impl AddAssign for m256 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m256(self, rhs)
  }
}
impl BitAndAssign for m256 {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m256(self, rhs)
  }
}
impl BitOrAssign for m256 {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m256(self, rhs)
  }
}
impl BitXorAssign for m256 {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    div_m256(self, rhs)
  }
}
impl DivAssign for m256 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    mul_m256(self, rhs)
  }
}
impl MulAssign for m256 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn neg(self) -> Self {
    sub_m256(zeroed_m256(), self)
  }
}

impl Not for m256 {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-ones bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m256(f32::from_bits(u32::MAX));
    self ^ all_bits
  }
}

impl Sub for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    sub_m256(self, rhs)
  }
}
impl SubAssign for m256 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl PartialEq for m256 {
  /// Performs a comparison to get a mask, then moves the mask and checks for
  /// all true.
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    let mask = m256(unsafe { _mm256_cmp_ps(self.0, other.0, _CMP_EQ_OQ) });
    move_mask_m256(mask) == 0b1111_1111
  }
}
