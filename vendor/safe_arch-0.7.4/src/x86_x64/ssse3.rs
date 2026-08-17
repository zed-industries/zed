#![cfg(target_feature = "ssse3")]

use super::*;

/// Lanewise absolute value with lanes as `i8`.
///
/// This is a "wrapping" absolute value, so `i8::MIN` stays as `i8::MIN`.
///
/// * **Intrinsic:** [`_mm_abs_epi8`]
/// * **Assembly:** `pabsb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn abs_i8_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_abs_epi8(a.0) })
}

/// Lanewise absolute value with lanes as `i16`.
///
/// This is a "wrapping" absolute value, so `i16::MIN` stays as `i16::MIN`.
///
/// * **Intrinsic:** [`_mm_abs_epi16`]
/// * **Assembly:** `pabsw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn abs_i16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_abs_epi16(a.0) })
}

/// Lanewise absolute value with lanes as `i32`.
///
/// This is a "wrapping" absolute value, so `i32::MIN` stays as `i32::MIN`.
///
/// * **Intrinsic:** [`_mm_abs_epi32`]
/// * **Assembly:** `pabsd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn abs_i32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_abs_epi32(a.0) })
}

/// Counts `$a` as the high bytes and `$b` as the low bytes then performs a
/// **byte** shift to the right by the immediate value.
///
/// Remember that this is all little-endian data.
///
/// * **Intrinsic:** [`_mm_alignr_epi8`]
/// * **Assembly:** `palignr xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn combined_byte_shr_imm_m128i<const IMM: i32>(
  a: m128i, b: m128i,
) -> m128i {
  m128i(unsafe { _mm_alignr_epi8(a.0, b.0, IMM) })
}

/// Add horizontal pairs of `i16` values, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hadd_epi16`]
/// * **Assembly:** `phaddw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn add_horizontal_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hadd_epi16(a.0, b.0) })
}

/// Add horizontal pairs of `i32` values, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hadd_epi32`]
/// * **Assembly:** `phaddd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn add_horizontal_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hadd_epi32(a.0, b.0) })
}

/// Add horizontal pairs of `i16` values, saturating, pack the outputs as `a`
/// then `b`.
///
/// * **Intrinsic:** [`_mm_hadds_epi16`]
/// * **Assembly:** `phaddsw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn add_horizontal_saturating_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hadds_epi16(a.0, b.0) })
}

/// Subtract horizontal pairs of `i16` values, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hsub_epi16`]
/// * **Assembly:** `phsubw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sub_horizontal_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hsub_epi16(a.0, b.0) })
}

/// Subtract horizontal pairs of `i32` values, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hsub_epi32`]
/// * **Assembly:** `phsubd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sub_horizontal_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hsub_epi32(a.0, b.0) })
}

/// Subtract horizontal pairs of `i16` values, saturating, pack the outputs as
/// `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hsubs_epi16`]
/// * **Assembly:** `phsubsw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sub_horizontal_saturating_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_hsubs_epi16(a.0, b.0) })
}

/// This is dumb and weird.
///
/// * Vertically multiplies each `u8` lane from `a` with an `i8` lane from `b`,
///   producing an `i16` intermediate value.
/// * These intermediate `i16` values are horizontally added with saturation.
///
/// * **Intrinsic:** [`_mm_maddubs_epi16`]
/// * **Assembly:** `pmaddubsw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn mul_u8i8_add_horizontal_saturating_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_maddubs_epi16(a.0, b.0) })
}

/// Multiply `i16` lanes into `i32` intermediates, keep the high 18 bits, round
/// by adding 1, right shift by 1.
///
/// This is `_mm_mulhrs_epi16`, which I can only assume is named for something
/// like "high bits rounded and scaled".
///
/// * **Intrinsic:** [`_mm_mulhrs_epi16`]
/// * **Assembly:** `pmulhrsw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn mul_i16_scale_round_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mulhrs_epi16(a.0, b.0) })
}

/// Shuffle `i8` lanes in `a` using `i8` values in `v`.
///
/// If a lane in `v` is negative, that output is zeroed.
///
/// * **Intrinsic:** [`_mm_shuffle_epi8`]
/// * **Assembly:** `pshufb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn shuffle_av_i8z_all_m128i(a: m128i, v: m128i) -> m128i {
  m128i(unsafe { _mm_shuffle_epi8(a.0, v.0) })
}

/// Applies the sign of `i8` values in `b` to the values in `a`.
///
/// * If `b` is negative: the `a` value is negated.
/// * Else If `b` is 0: the `a` value becomes 0.
/// * Else the `a` value is unchanged.
///
/// * **Intrinsic:** [`_mm_sign_epi8`]
/// * **Assembly:** `psignb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sign_apply_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sign_epi8(a.0, b.0) })
}

/// Applies the sign of `i16` values in `b` to the values in `a`.
///
/// * If `b` is negative: the `a` value is negated.
/// * Else If `b` is 0: the `a` value becomes 0.
/// * Else the `a` value is unchanged.
///
/// * **Intrinsic:** [`_mm_sign_epi16`]
/// * **Assembly:** `psignw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sign_apply_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sign_epi16(a.0, b.0) })
}

/// Applies the sign of `i32` values in `b` to the values in `a`.
///
/// * If `b` is negative: the `a` value is negated.
/// * Else If `b` is 0: the `a` value becomes 0.
/// * Else the `a` value is unchanged.
///
/// * **Intrinsic:** [`_mm_sign_epi32`]
/// * **Assembly:** `psignd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "ssse3")))]
pub fn sign_apply_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sign_epi32(a.0, b.0) })
}

