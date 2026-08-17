#![cfg(target_feature = "sse4.1")]

use super::*;

/// Blends the `i16` lanes according to the immediate mask.
///
/// Each bit 0 though 7 controls lane 0 through 7. Use 0 for the `a` value and
/// 1 for the `b` value.
///
/// * **Intrinsic:** [`_mm_blend_epi16`]
/// * **Assembly:** `pblendw xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_imm_i16_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_blend_epi16(a.0, b.0, IMM) })
}

/// Blends the `i16` lanes according to the immediate mask.
///
/// Bits 0 and 1 control where output lane 0 and 1 come from. Use 0 for the `a`
/// value and 1 for the `b` value.
///
/// * **Intrinsic:** [`_mm_blend_pd`]
/// * **Assembly:** `blendpd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_imm_m128d<const IMM: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_blend_pd(a.0, b.0, IMM) })
}

/// Blends the lanes according to the immediate mask.
///
/// Bits 0 to 3 control where output lane 0 to 3 come from. Use 0 for the `a`
/// value and 1 for the `b` value.
///
/// * **Intrinsic:** [`_mm_blend_ps`]
/// * **Assembly:** `blendps xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_imm_m128<const IMM: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_blend_ps(a.0, b.0, IMM) })
}

/// Blend the `i8` lanes according to a runtime varying mask.
///
/// The sign bit of each `i8` lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// * **Intrinsic:** [`_mm_blendv_epi8`]
/// * **Assembly:** `pblendvb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_i8_m128i(a: m128i, b: m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_blendv_epi8(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// * **Intrinsic:** [`_mm_blendv_pd`]
/// * **Assembly:** `blendvpd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_m128d(a: m128d, b: m128d, mask: m128d) -> m128d {
  m128d(unsafe { _mm_blendv_pd(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// * **Intrinsic:** [`_mm_blendv_ps`]
/// * **Assembly:** `blendvps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_m128(a: m128, b: m128, mask: m128) -> m128 {
  m128(unsafe { _mm_blendv_ps(a.0, b.0, mask.0) })
}

/// Round each lane to a whole number, towards positive infinity.
///
/// * **Intrinsic:** [`_mm_ceil_pd`]
/// * **Assembly:** `roundpd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_ceil_pd(a.0) })
}

/// Round each lane to a whole number, towards positive infinity.
///
/// * **Intrinsic:** [`_mm_ceil_ps`]
/// * **Assembly:** `roundps xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128(a: m128) -> m128 {
  m128(unsafe { _mm_ceil_ps(a.0) })
}

/// Round the low lane of `b` toward positive infinity, high lane is `a`.
///
/// * **Intrinsic:** [`_mm_ceil_sd`]
/// * **Assembly:** `roundsd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_ceil_sd(a.0, b.0) })
}

/// Round the low lane of `b` toward positive infinity, other lanes `a`.
///
/// * **Intrinsic:** [`_mm_ceil_ss`]
/// * **Assembly:** `roundss xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_ceil_ss(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i64`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
///
/// * **Intrinsic:** [`_mm_cmpeq_epi64`]
/// * **Assembly:** `pcmpeqq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn cmp_eq_mask_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi64(a.0, b.0) })
}

/// Convert the lower four `i16` lanes to four `i32` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi16_epi32`]
/// * **Assembly:** `pmovsxwd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i32_m128i_from_lower4_i16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi16_epi32(a.0) })
}

/// Convert the lower two `i64` lanes to two `i32` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi16_epi64`]
/// * **Assembly:** `pmovsxwq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i16_m128i_from_lower2_i16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi16_epi64(a.0) })
}

/// Convert the lower two `i32` lanes to two `i64` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi32_epi64`]
/// * **Assembly:** `_mm_cvtepi32_epi64`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i64_m128i_from_lower2_i32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi32_epi64(a.0) })
}

/// Convert the lower eight `i8` lanes to eight `i16` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi8_epi16`]
/// * **Assembly:** `pmovsxbw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i16_m128i_from_lower8_i8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi16(a.0) })
}

/// Convert the lower four `i8` lanes to four `i32` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi8_epi32`]
/// * **Assembly:** `pmovsxbd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i32_m128i_from_lower4_i8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi32(a.0) })
}

/// Convert the lower two `i8` lanes to two `i64` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepi8_epi64`]
/// * **Assembly:** `pmovsxbq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_i64_m128i_from_lower2_i8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi64(a.0) })
}

/// Convert the lower four `u16` lanes to four `u32` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu16_epi32`]
/// * **Assembly:** `pmovzxwd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u32_m128i_from_lower4_u16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu16_epi32(a.0) })
}

/// Convert the lower two `u16` lanes to two `u64` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu16_epi64`]
/// * **Assembly:** `pmovzxwq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u64_m128i_from_lower2_u16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu16_epi64(a.0) })
}

/// Convert the lower two `u32` lanes to two `u64` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu32_epi64`]
/// * **Assembly:** `pmovzxdq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u64_m128i_from_lower2_u32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu32_epi64(a.0) })
}

/// Convert the lower eight `u8` lanes to eight `u16` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu8_epi16`]
/// * **Assembly:** `pmovzxbw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u16_m128i_from_lower8_u8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi16(a.0) })
}

/// Convert the lower four `u8` lanes to four `u32` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu8_epi32`]
/// * **Assembly:** `pmovzxbd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u32_m128i_from_lower4_u8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi32(a.0) })
}

/// Convert the lower two `u8` lanes to two `u64` lanes.
///
/// * **Intrinsic:** [`_mm_cvtepu8_epi64`]
/// * **Assembly:** `pmovzxbq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_to_u64_m128i_from_lower2_u8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi64(a.0) })
}

/// Performs a dot product of two `m128d` registers.
///
/// The output details are determined by the constant:
/// * For each lane, you can multiply that lane from `a` and `b` or you can take
///   a default of 0.0
/// * Bits 4 and 5 determines if we mul lanes 0 in `a` and `b`, and lanes 1 in
///   `a` and `b`.
/// * This forms two temporary `f64` values which are summed to a single `f64`.
/// * For each output lane, you can have the sum in that lane or 0.0.
/// * Bits 0 and 1 determines if an output lane is our sum or 0.0.
///
/// * **Intrinsic:** [`_mm_dp_pd`]
/// * **Assembly:** `dppd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn dot_product_m128d<const IMM: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_dp_pd(a.0, b.0, IMM) })
}

/// Performs a dot product of two `m128` registers.
///
/// The output details are determined by a control mask:
/// * For each lane, you can multiply that lane from `a` and `b` or you can take
///   a default of 0.0
/// * Bits 4 through 7 determine if we should mul lanes 0 through 3.
/// * This forms four temporary `f32` values which are summed to a single `f32`.
/// * For each output lane, you can have the sum in that lane or 0.0.
/// * Bits 0 through 3 determines if the `sum` is in lanes 0 through 3.
///
/// * **Intrinsic:** [`_mm_dp_ps`]
/// * **Assembly:** `dpps xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn dot_product_m128<const IMM: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_dp_ps(a.0, b.0, IMM) })
}

/// Gets the `i32` lane requested. Only the lowest 2 bits are considered.
///
/// * **Intrinsic:** [`_mm_extract_epi32`]
/// * **Assembly:** `pextrd r32, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn extract_i32_imm_m128i<const IMM: i32>(a: m128i) -> i32 {
  unsafe { _mm_extract_epi32(a.0, IMM) }
}

/// Gets the `i64` lane requested. Only the lowest bit is considered.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_i64, 6]);
/// assert_eq!(extract_i64_imm_m128i::<1>(a), 6_i64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn extract_i64_imm_m128i<const IMM: i32>(a: m128i) -> i64 {
  unsafe { _mm_extract_epi64(a.0, IMM) }
}

/// Gets the `i8` lane requested. Only the lowest 4 bits are considered.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 101, 8, 9, 10, 11, 12, 13, 14, 15]);
/// assert_eq!(extract_i8_as_i32_imm_m128i::<7>(a), 101_i32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn extract_i8_as_i32_imm_m128i<const IMM: i32>(a: m128i) -> i32 {
  unsafe { _mm_extract_epi8(a.0, IMM) }
}

/// Gets the `f32` lane requested. Returns as an `i32` bit pattern.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// assert_eq!(extract_f32_as_i32_bits_imm_m128::<3>(a), 8_f32.to_bits() as i32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn extract_f32_as_i32_bits_imm_m128<const IMM: i32>(a: m128) -> i32 {
  unsafe { _mm_extract_ps(a.0, IMM) }
}

/// Round each lane to a whole number, towards negative infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// assert_eq!(floor_m128d(a).to_array(), [-1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_floor_pd(a.0) })
}

/// Round each lane to a whole number, towards negative infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 2.5, 3.0]);
/// assert_eq!(floor_m128(a).to_array(), [-1.0, 1.0, 2.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128(a: m128) -> m128 {
  m128(unsafe { _mm_floor_ps(a.0) })
}

/// Round the low lane of `b` toward negative infinity, high lane is `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// let b = m128d::from_array([2.5, 3.0]);
/// assert_eq!(floor_m128d_s(a, b).to_array(), [2.0, 1.8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_floor_sd(a.0, b.0) })
}

/// Round the low lane of `b` toward negative infinity, other lanes `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 5.0, 6.0]);
/// let b = m128::from_array([2.5, 3.0, 10.0, 20.0]);
/// assert_eq!(floor_m128_s(a, b).to_array(), [2.0, 1.8, 5.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_floor_ss(a.0, b.0) })
}

/// Inserts a new value for the `i32` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5, 6, 7, 8]);
/// let b: [i32; 4] = insert_i32_imm_m128i::<1>(a, 23).into();
/// assert_eq!(b, [5, 23, 7, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn insert_i32_imm_m128i<const IMM: i32>(a: m128i, new: i32) -> m128i {
  m128i(unsafe { _mm_insert_epi32(a.0, new, IMM) })
}

/// Inserts a new value for the `i64` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_i64, 6]);
/// let b: [i64; 2] = insert_i64_imm_m128i::<1>(a, 23).into();
/// assert_eq!(b, [5_i64, 23]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn insert_i64_imm_m128i<const IMM: i32>(a: m128i, new: i64) -> m128i {
  m128i(unsafe { _mm_insert_epi64(a.0, new, IMM) })
}

/// Inserts a new value for the `i64` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b: [i8; 16] = insert_i8_imm_m128i::<1>(a, 23).into();
/// assert_eq!(b, [0_i8, 23, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn insert_i8_imm_m128i<const IMM: i32>(a: m128i, new: i32) -> m128i {
  m128i(unsafe { _mm_insert_epi8(a.0, new, IMM) })
}

/// Inserts a lane from `$b` into `$a`, optionally at a new position.
///
/// Also, you can zero out any lanes you like for free as part of the same
/// operation. If you don't specify the mask argument then no lanes are zeroed.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = insert_f32_imm_m128::<0b00_11_0000>(a, b).to_array();
/// assert_eq!(c, [1.0, 2.0, 3.0, 5.0]);
/// //
/// let c = insert_f32_imm_m128::<0b00_11_0110>(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 5.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn insert_f32_imm_m128<const IMM: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_insert_ps(a.0, b.0, IMM) })
}

/// Lanewise `max(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, -7, 8]);
/// let c: [i32; 4] = max_i32_m128i(a, b).into();
/// assert_eq!(c, [5, 6, 3, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epi32(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = max_i8_m128i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 3, 4, 15, 6, 7, 8, 19, 10, 21, 22, 13, 24, 127]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epi8(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 300, 400, 1, 2, 3, 4]);
/// let b = m128i::from([5_u16, 6, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = max_u16_m128i(a, b).into();
/// assert_eq!(c, [5_u16, 6, 300, 400, 15, 26, 37, 48]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epu16(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 200, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [u32; 4] = max_u32_m128i(a, b).into();
/// assert_eq!(c, [5, 200, 7, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_u32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epu32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, -7, 8]);
/// let c: [i32; 4] = min_i32_m128i(a, b).into();
/// assert_eq!(c, [1, 2, -7, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epi32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = min_i8_m128i(a, b).into();
/// assert_eq!(c, [0_i8, 1, 2, -13, 4, 5, 6, -17, -8, 9, -20, 11, 12, -23, 14, 127]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epi8(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 300, 400, 1, 2, 3, 4]);
/// let b = m128i::from([5_u16, 6, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = min_u16_m128i(a, b).into();
/// assert_eq!(c, [1_u16, 2, 7, 8, 1, 2, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epu16(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 200, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [u32; 4] = min_u32_m128i(a, b).into();
/// assert_eq!(c, [1, 6, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_u32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epu32(a.0, b.0) })
}

/// Min `u16` value, position, and other lanes zeroed.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([120_u16, 24, 300, 400, 90, 129, 31, 114]);
/// let c: [u16; 8] = min_position_u16_m128i(a).into();
/// assert_eq!(c, [24_u16, 1, 0, 0, 0, 0, 0, 0]);
///
/// // the position favors the leftmost minimum
/// let a = m128i::from([120_u16, 24, 24, 400, 90, 129, 31, 114]);
/// let c: [u16; 8] = min_position_u16_m128i(a).into();
/// assert_eq!(c, [24_u16, 1, 0, 0, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_position_u16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_minpos_epu16(a.0) })
}

/// Computes eight `u16` "sum of absolute difference" values according to the
/// bytes selected.
///
/// * `a` can be 0 or 1, and specifies to skip the first fur `$a` values or not.
/// * `b` can be 0, 1, 2, or 3 and specifies to skip the first four times that
///   many values in `$b`.
///
/// This is used for some HD codec thing, and I don't really get what the point
/// is, but I'm sure someone uses it. If you can write better docs about what
/// this does please file a PR.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_u8, 1, 56, 3, 255, 5, 127, 7, 128, 9, 100, 101, 123, 13, 154, 125]);
/// let b = m128i::from([12_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b00_00>(a, b).into();
/// assert_eq!(c, [66, 319, 301, 390, 376, 263, 253, 236]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b00_01>(a, b).into();
/// assert_eq!(c, [62, 305, 305, 372, 372, 245, 249, 222]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b00_10>(a, b).into();
/// assert_eq!(c, [70, 305, 305, 372, 372, 241, 241, 210]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b00_11>(a, b).into();
/// assert_eq!(c, [78, 305, 305, 372, 372, 241, 241, 210]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b01_00>(a, b).into();
/// assert_eq!(c, [376, 263, 253, 236, 320, 321, 319, 373]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b01_01>(a, b).into();
/// assert_eq!(c, [372, 245, 249, 222, 316, 311, 315, 369]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b01_10>(a, b).into();
/// assert_eq!(c, [372, 241, 241, 210, 300, 295, 299, 353]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i::<0b01_11>(a, b).into();
/// assert_eq!(c, [372, 241, 241, 210, 292, 285, 287, 339]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn multi_packed_sum_abs_diff_u8_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mpsadbw_epu8(a.0, b.0, IMM) })
}

/// Multiplies the odd `i32` lanes and gives the widened (`i64`) results.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 7, i32::MAX, 7]);
/// let b = m128i::from([-5, 7, i32::MAX, 7]);
/// let c: [i64; 2] = mul_widen_i32_odd_m128i(a, b).into();
/// assert_eq!(c, [(-1 * 5), (i32::MAX as i64 * i32::MAX as i64)]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn mul_widen_i32_odd_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mul_epi32(a.0, b.0) })
}

/// Lanewise `a * b` with 32-bit lanes.
///
/// This keeps the low 32-bits from each 64-bit output,
/// so it actually works for both `i32` and `u32`.
/// ```
/// # use safe_arch::*;
/// let ai = m128i::from([1, 2000000, -300, 45689]);
/// let bi = m128i::from([5, 6000000, 700, -89109]);
/// let ci: [i32; 4] = mul_32_m128i(ai, bi).into();
/// assert_eq!(ci, [5, -138625024, -210000, 223666195]);
///
/// let au = m128i::from([u32::MAX, 26, 5678, 1234567890]);
/// let bu = m128i::from([u32::MAX, 74, 9101112, 765]);
/// let cu: [u32; 4] = mul_32_m128i(au, bu).into();
/// assert_eq!(cu, [1, 1924, 136506384, 3846598026]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn mul_32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mullo_epi32(a.0, b.0) })
}

/// Saturating convert `i32` to `u16`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([9, -10, -11, i32::MAX]);
/// let c: [u16; 8] = pack_i32_to_u16_m128i(a, b).into();
/// assert_eq!(c, [1, 2, 3, 4, 9, 0, 0, u16::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn pack_i32_to_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_packus_epi32(a.0, b.0) })
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.6]);
/// //
/// assert_eq!(round_m128d::<{ round_op!(Nearest) }>(a).to_array(), [0.0, 2.0]);
/// //
/// assert_eq!(round_m128d::<{ round_op!(NegInf) }>(a).to_array(), [-1.0, 1.0]);
/// //
/// assert_eq!(round_m128d::<{ round_op!(PosInf) }>(a).to_array(), [0.0, 2.0]);
/// //
/// assert_eq!(round_m128d::<{ round_op!(Zero) }>(a).to_array(), [0.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn round_m128d<const MODE: i32>(a: m128d) -> m128d {
  m128d(unsafe { _mm_round_pd(a.0, MODE) })
}

/// Rounds `$b` low as specified, keeps `$a` high.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 900.0]);
/// //
/// let b = m128d::from_array([-0.1, f64::NAN]);
/// //
/// assert_eq!(round_m128d_s::<{ round_op!(Nearest) }>(a, b).to_array(), [0.0, 900.0]);
/// assert_eq!(round_m128d_s::<{ round_op!(NegInf) }>(a, b).to_array(), [-1.0, 900.0]);
/// //
/// let b = m128d::from_array([2.4, f64::NAN]);
/// //
/// assert_eq!(round_m128d_s::<{ round_op!(PosInf) }>(a, b).to_array(), [3.0, 900.0]);
/// assert_eq!(round_m128d_s::<{ round_op!(Zero) }>(a, b).to_array(), [2.0, 900.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn round_m128d_s<const MODE: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_round_sd(a.0, b.0, MODE) })
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.6, 3.3, 4.5]);
/// //
/// assert_eq!(round_m128::<{ round_op!(Nearest) }>(a).to_array(), [0.0, 2.0, 3.0, 4.0]);
/// //
/// assert_eq!(round_m128::<{ round_op!(NegInf) }>(a).to_array(), [-1.0, 1.0, 3.0, 4.0]);
/// //
/// assert_eq!(round_m128::<{ round_op!(PosInf) }>(a).to_array(), [0.0, 2.0, 4.0, 5.0]);
/// //
/// assert_eq!(round_m128::<{ round_op!(Zero) }>(a).to_array(), [0.0, 1.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn round_m128<const MODE: i32>(a: m128) -> m128 {
  m128(unsafe { _mm_round_ps(a.0, MODE) })
}

/// Rounds `$b` low as specified, other lanes use `$a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([f32::NAN, 6.0, 7.0, 8.0]);
/// //
/// let b = m128::from_array([-0.1, f32::NAN, f32::NAN, f32::NAN]);
/// //
/// assert_eq!(round_m128_s::<{ round_op!(Nearest) }>(a, b).to_array(), [0.0, 6.0, 7.0, 8.0]);
/// assert_eq!(round_m128_s::<{ round_op!(NegInf) }>(a, b).to_array(), [-1.0, 6.0, 7.0, 8.0]);
/// //
/// let b = m128::from_array([2.4, f32::NAN, f32::NAN, f32::NAN]);
/// //
/// assert_eq!(round_m128_s::<{ round_op!(PosInf) }>(a, b).to_array(), [3.0, 6.0, 7.0, 8.0]);
/// assert_eq!(round_m128_s::<{ round_op!(Zero) }>(a, b).to_array(), [2.0, 6.0, 7.0, 8.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn round_m128_s<const MODE: i32>(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_round_ss(a.0, b.0, MODE) })
}

/// Computes the bitwise AND of 256 bits in `a` and
/// `b`, returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testz_si128`]
/// * **Assembly:** ptest xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn testz_m128i(a: m128i, b: m128i) -> i32 {
  unsafe { _mm_testz_si128(a.0, b.0) }
}

/// Compute the bitwise NOT of `a` and then AND with `b`,
/// returns 1 if the result is zero, otherwise 0.
/// * **Intrinsic:** [`_mm_testc_si128`]
/// * **Assembly:** ptest xmm, xmm
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn testc_m128i(a: m128i, b: m128i) -> i32 {
  unsafe { _mm_testc_si128(a.0, b.0) }
}

/// Tests if all bits are 1.
/// * **Intrinsic:** [`_mm_test_all_ones`]
/// * **Assembly:** pcmpeqd xmm, xmm / ptest xmm, xmm
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0_u128);
/// let b = m128i::from(u128::MAX);
/// assert_eq!(test_all_ones_m128i(a), 0);
/// assert_eq!(test_all_ones_m128i(b), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_all_ones_m128i(a: m128i) -> i32 {
  unsafe { _mm_test_all_ones(a.0) }
}

/// Returns if all masked bits are 0, `(a & mask) as u128 == 0`
/// * **Intrinsic:** [`_mm_test_all_zeros`]
/// * **Assembly:** ptest xmm, xmm
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0b111_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b0_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 1);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b0_1111_u128);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_all_zeroes_m128i(a: m128i, mask: m128i) -> i32 {
  unsafe { _mm_test_all_zeros(a.0, mask.0) }
}

/// Returns if, among the masked bits, there's both 0s and 1s
/// * **Intrinsic:** [`_mm_test_mix_ones_zeros`]
/// * **Assembly:** ptest xmm, xmm
/// 
/// * Zero Flag = `(a & mask) as u128 == 0`
/// * Carry Flag = `((!a) & mask) as u128 == 0`
/// * Return `ZeroFlag == 0 && Carry Flag == 0`
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0b111_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 1);
/// //
/// let a = m128i::from(0b0_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b0_1111_u128);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b1_1111_u128);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_mixed_ones_and_zeroes_m128i(a: m128i, mask: m128i) -> i32 {
  unsafe { _mm_test_mix_ones_zeros(a.0, mask.0) }
}

