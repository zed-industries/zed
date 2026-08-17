#![cfg(target_feature = "sse2")]

use super::*;

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = add_i8_m128i(a, b).into();
/// assert_eq!(c, [0, 12, 4, 16, 8, 20, 12, 24, 16, 28, -10, 32, 34, -10, 38, -114]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = add_i16_m128i(a, b).into();
/// assert_eq!(c, [6, 8, 10, 12, -16, -28, -40, 44]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi16(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [i32; 4] = add_i32_m128i(a, b).into();
/// assert_eq!(c, [6, 8, 10, 12]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi32(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([92_i64, 87]);
/// let b = m128i::from([-9001_i64, 1]);
/// let c: [i64; 2] = add_i64_m128i(a, b).into();
/// assert_eq!(c, [-8909, 88]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi64(a.0, b.0) })
}

/// Lanewise `a + b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = add_m128d(a, b).to_array();
/// assert_eq!(c, [192.0, 81.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_add_pd(a.0, b.0) })
}

/// Lowest lane `a + b`, high lane unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -600.0]);
/// let c = add_m128d_s(a, b).to_array();
/// assert_eq!(c, [192.0, 87.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_add_sd(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   i8::MAX, i8::MIN, 3, 4, -1, -2, -3, -4,
///   3, 4, -1, -2, -1, -2, -3, -4,
/// ]);
/// let b = m128i::from([
///   i8::MAX, i8::MIN, 7, 8, -15, -26, -37, 48,
///   7, 8, -15, -26, -15, -26, -37, 48,
/// ]);
/// let c: [i8; 16] = add_saturating_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i8::MAX, i8::MIN, 10, 12, -16, -28, -40, 44,
///     10, 12, -16, -28, -16, -28, -40, 44
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_saturating_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epi8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([i16::MAX, i16::MIN, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([i16::MAX, i16::MIN, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = add_saturating_i16_m128i(a, b).into();
/// assert_eq!(c, [i16::MAX, i16::MIN, 10, 12, -16, -28, -40, 44]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_saturating_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epi16(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   u8::MAX, 0, 3, 4, 254, 2, 3, 4,
///   3, 4, 1, 2, 1, 2, 128, 4,
/// ]);
/// let b = m128i::from([
///   u8::MAX, 0, 7, 8, 15, 26, 37, 48,
///   7, 8, 15, 26, 15, 26, 37, 48,
/// ]);
/// let c: [u8; 16] = add_saturating_u8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     u8::MAX, 0, 10, 12, 255, 28, 40, 52,
///     10, 12, 16, 28, 16, 28, 165, 52
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_saturating_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epu8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 0, 3, 4, 1, 2, 3, 4]);
/// let b = m128i::from([u16::MAX, 0, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = add_saturating_u16_m128i(a, b).into();
/// assert_eq!(c, [u16::MAX, 0, 10, 12, 16, 28, 40, 52]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn add_saturating_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epu16(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = bitand_m128d(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitand_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_and_pd(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = bitand_m128i(a, b).into();
/// assert_eq!(c, [1, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitand_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_and_si128(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = bitandnot_m128d(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitandnot_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_andnot_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = bitandnot_m128i(a, b).into();
/// assert_eq!(c, [0, 1, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitandnot_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_andnot_si128(a.0, b.0) })
}

/// Lanewise average of the `u8` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   u8::MAX, 0, 3, 4, 254, 2, 3, 4,
///   3, 4, 1, 2, 1, 2, 128, 4,
/// ]);
/// let b = m128i::from([
///   u8::MAX, 0, 7, 8, 15, 26, 37, 48,
///   7, 8, 15, 26, 15, 26, 37, 48,
/// ]);
/// let c: [u8; 16] = average_u8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     u8::MAX, 0, 5, 6, 135, 14, 20, 26,
///     5, 6, 8, 14, 8, 14, 83, 26
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn average_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_avg_epu8(a.0, b.0) })
}

/// Lanewise average of the `u16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 0, 3, 4, 1, 2, 3, 4]);
/// let b = m128i::from([u16::MAX, 0, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = average_u16_m128i(a, b).into();
/// assert_eq!(c, [u16::MAX, 0, 5, 6, 8, 14, 20, 26]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn average_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_avg_epu16(a.0, b.0) })
}

/// Shifts all bits in the entire register left by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x0000000B_0000000A_0000000F_11111111_u128);
/// //
/// let b: u128 = byte_shl_imm_u128_m128i::<1>(a).into();
/// assert_eq!(b, 0x00000B00_00000A00_00000F11_11111100);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn byte_shl_imm_u128_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_bslli_si128(a.0, IMM) })
}

/// Shifts all bits in the entire register right by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x0000000B_0000000A_0000000F_11111111_u128);
/// //
/// let c: u128 = byte_shr_imm_u128_m128i::<1>(a).into();
/// assert_eq!(c, 0x00000000_0B000000_0A000000_0F111111);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn byte_shr_imm_u128_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_bsrli_si128(a.0, IMM) })
}

/// Bit-preserving cast to `m128` from `m128d`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let c: [u32; 4] = cast_to_m128_from_m128d(a).to_bits();
/// assert_eq!(c, [0, 0x3FF00000, 0, 0x40000000]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128_from_m128d(a: m128d) -> m128 {
  m128(unsafe { _mm_castpd_ps(a.0) })
}

/// Bit-preserving cast to `m128i` from `m128d`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let c: [u32; 4] = cast_to_m128i_from_m128d(a).into();
/// assert_eq!(c, [0, 0x3FF00000, 0, 0x40000000]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128i_from_m128d(a: m128d) -> m128i {
  m128i(unsafe { _mm_castpd_si128(a.0) })
}

/// Bit-preserving cast to `m128d` from `m128`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let c: [u64; 2] = cast_to_m128d_from_m128(a).to_bits();
/// assert_eq!(c, [0x400000003F800000, 0x4080000040400000]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128d_from_m128(a: m128) -> m128d {
  m128d(unsafe { _mm_castps_pd(a.0) })
}

/// Bit-preserving cast to `m128i` from `m128`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let c: [u32; 4] = cast_to_m128i_from_m128(a).into();
/// assert_eq!(c, [0x3F800000, 0x40000000, 0x40400000, 0x40800000]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128i_from_m128(a: m128) -> m128i {
  m128i(unsafe { _mm_castps_si128(a.0) })
}

/// Bit-preserving cast to `m128d` from `m128i`
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let c: [u64; 2] = cast_to_m128d_from_m128i(a).to_bits();
/// assert_eq!(c, [0x200000001, 0x400000003]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128d_from_m128i(a: m128i) -> m128d {
  m128d(unsafe { _mm_castsi128_pd(a.0) })
}

/// Bit-preserving cast to `m128` from `m128i`
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let c: [u32; 4] = cast_to_m128_from_m128i(a).to_bits();
/// assert_eq!(c, [1, 2, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cast_to_m128_from_m128i(a: m128i) -> m128 {
  m128(unsafe { _mm_castsi128_ps(a.0) })
}

/// Lanewise `a == b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = cmp_eq_mask_i8_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, -1, 0, -1, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi8(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_eq_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1, 0, 0, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi16(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_eq_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi32(a.0, b.0) })
}

/// Lanewise `a == b`, mask output.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_eq_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpeq_pd(a.0, b.0) })
}

/// Low lane `a == b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_eq_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpeq_sd(a.0, b.0) })
}

/// Lanewise `a >= b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ge_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ge_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpge_pd(a.0, b.0) })
}

/// Low lane `a >= b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ge_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ge_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpge_sd(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i8, 1, 20, 3, 40, 5, 60, 7, 80, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 120]);
/// let c: [i8; 16] = cmp_gt_mask_i8_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, 0, -1, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi8(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 20, 3, 40, -1, -2, -3, 0]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_gt_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1, -1, -1, -1, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi16(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 20, 7, 40]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_gt_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi32(a.0, b.0) })
}

/// Lanewise `a > b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_gt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpgt_pd(a.0, b.0) })
}

/// Low lane `a > b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_gt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpgt_sd(a.0, b.0) })
}

/// Lanewise `a <= b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_le_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_le_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmple_pd(a.0, b.0) })
}

/// Low lane `a <= b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_le_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_le_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmple_sd(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i8, 1, 20, 3, 40, 5, 60, 7, 80, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 120]);
/// let c: [i8; 16] = cmp_lt_mask_i8_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, 0, -1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi8(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 20, 3, 40, -1, -2, -3, 0]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_lt_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, -1, 0, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi16(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 20, 7, 40]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_lt_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi32(a.0, b.0) })
}

/// Lanewise `a < b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 7.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_lt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmplt_pd(a.0, b.0) })
}

/// Low lane `a < b`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_lt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmplt_sd(a.0, b.0) })
}

/// Lanewise `a != b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_neq_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_neq_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpneq_pd(a.0, b.0) })
}

/// Low lane `a != b`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_neq_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_neq_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpneq_sd(a.0, b.0) })
}

/// Lanewise `!(a >= b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nge_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [0, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nge_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnge_pd(a.0, b.0) })
}

/// Low lane `!(a >= b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nge_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [0, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nge_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnge_sd(a.0, b.0) })
}

/// Lanewise `!(a > b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ngt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [0, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ngt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpngt_pd(a.0, b.0) })
}

/// Low lane `!(a > b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ngt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [0, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ngt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpngt_sd(a.0, b.0) })
}

/// Lanewise `!(a <= b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nle_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nle_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnle_pd(a.0, b.0) })
}

/// Low lane `!(a <= b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nle_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nle_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnle_sd(a.0, b.0) })
}

/// Lanewise `!(a < b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nlt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nlt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnlt_pd(a.0, b.0) })
}

/// Low lane `!(a < b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nlt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_nlt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnlt_sd(a.0, b.0) })
}

/// Lanewise `(!a.is_nan()) & (!b.is_nan())`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, f64::NAN]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ordered_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ordered_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpord_pd(a.0, b.0) })
}

/// Low lane `(!a.is_nan()) & (!b.is_nan())`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ordered_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ordered_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpord_sd(a.0, b.0) })
}

/// Lanewise `a.is_nan() | b.is_nan()`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_unord_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_unord_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpunord_pd(a.0, b.0) })
}

/// Low lane `a.is_nan() | b.is_nan()`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_unord_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_unord_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpunord_sd(a.0, b.0) })
}

/// Low lane `f64` equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_eq_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_eq_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comieq_sd(a.0, b.0) }
}

/// Low lane `f64` greater than or equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_ge_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_ge_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comige_sd(a.0, b.0) }
}

/// Low lane `f64` greater than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_ge_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_gt_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comigt_sd(a.0, b.0) }
}

/// Low lane `f64` less than or equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_le_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_le_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comile_sd(a.0, b.0) }
}

/// Low lane `f64` less than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_lt_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_lt_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comilt_sd(a.0, b.0) }
}

/// Low lane `f64` less than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_neq_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn cmp_neq_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comineq_sd(a.0, b.0) }
}

/// Rounds the lower two `i32` lanes to two `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = convert_to_m128d_from_lower2_i32_m128i(a);
/// let c = m128d::from_array([1.0, 2.0]);
/// assert_eq!(b.to_bits(), c.to_bits());
/// ```
/// * **Intrinsic:** [`_mm_cvtepi32_pd`]
/// * **Assembly:** `cvtdq2pd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_m128d_from_lower2_i32_m128i(a: m128i) -> m128d {
  m128d(unsafe { _mm_cvtepi32_pd(a.0) })
}

/// Rounds the four `i32` lanes to four `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = convert_to_m128_from_i32_m128i(a);
/// let c = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(b.to_bits(), c.to_bits());
/// ```
/// * **Intrinsic:** [`_mm_cvtepi32_ps`]
/// * **Assembly:** `cvtdq2ps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_m128_from_i32_m128i(a: m128i) -> m128 {
  m128(unsafe { _mm_cvtepi32_ps(a.0) })
}

/// Rounds the two `f64` lanes to the low two `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = convert_to_i32_m128i_from_m128d(a);
/// let c: [i32; 4] = b.into();
/// assert_eq!(c, [1, 2, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm_cvtpd_epi32`]
/// * **Assembly:** `cvtpd2dq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_i32_m128i_from_m128d(a: m128d) -> m128i {
  m128i(unsafe { _mm_cvtpd_epi32(a.0) })
}

/// Rounds the two `f64` lanes to the low two `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = convert_to_m128_from_m128d(a);
/// assert_eq!(b.to_bits(), [1_f32.to_bits(), 2.5_f32.to_bits(), 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm_cvtpd_ps`]
/// * **Assembly:** `cvtpd2ps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_m128_from_m128d(a: m128d) -> m128 {
  m128(unsafe { _mm_cvtpd_ps(a.0) })
}

/// Rounds the `f32` lanes to `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.5, 3.0, 4.0]);
/// let b = convert_to_i32_m128i_from_m128(a);
/// let c: [i32; 4] = b.into();
/// assert_eq!(c, [1, 2, 3, 4]);
/// ```
/// * **Intrinsic:** [`_mm_cvtps_epi32`]
/// * **Assembly:** `cvtps2dq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_i32_m128i_from_m128(a: m128) -> m128i {
  m128i(unsafe { _mm_cvtps_epi32(a.0) })
}

/// Rounds the two `f64` lanes to the low two `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.5, 3.6, 4.7]);
/// let b = convert_to_m128d_from_lower2_m128(a);
/// assert_eq!(b.to_bits(), [1_f64.to_bits(), 2.5_f64.to_bits()]);
/// ```
/// * **Intrinsic:** [`_mm_cvtps_pd`]
/// * **Assembly:** `cvtps2pd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_to_m128d_from_lower2_m128(a: m128) -> m128d {
  m128d(unsafe { _mm_cvtps_pd(a.0) })
}

/// Gets the lower lane as an `f64` value.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = get_f64_from_m128d_s(a);
/// assert_eq!(b, 1.0_f64);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn get_f64_from_m128d_s(a: m128d) -> f64 {
  unsafe { _mm_cvtsd_f64(a.0) }
}

/// Converts the lower lane to an `i32` value.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = get_i32_from_m128d_s(a);
/// assert_eq!(b, 1_i32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn get_i32_from_m128d_s(a: m128d) -> i32 {
  unsafe { _mm_cvtsd_si32(a.0) }
}

/// Converts the lower lane to an `i64` value.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = get_i64_from_m128d_s(a);
/// assert_eq!(b, 1_i64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn get_i64_from_m128d_s(a: m128d) -> i64 {
  unsafe { _mm_cvtsd_si64(a.0) }
}

/// Converts the low `f64` to `f32` and replaces the low lane of the input.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([3.0, 4.0, 5.0, 6.0]);
/// let b = m128d::from_array([1.0, 2.5]);
/// let c = convert_m128d_s_replace_m128_s(a, b);
/// assert_eq!(c.to_array(), [1.0, 4.0, 5.0, 6.0]);
/// ```
/// * **Intrinsic:** [`_mm_cvtsd_ss`]
/// * **Assembly:** `cvtsd2ss xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_m128d_s_replace_m128_s(a: m128, b: m128d) -> m128 {
  m128(unsafe { _mm_cvtsd_ss(a.0, b.0) })
}

/// Converts the lower lane to an `i32` value.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 3, 5, 7]);
/// let b = get_i32_from_m128i_s(a);
/// assert_eq!(b, 1_i32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn get_i32_from_m128i_s(a: m128i) -> i32 {
  unsafe { _mm_cvtsi128_si32(a.0) }
}

/// Converts the lower lane to an `i64` value.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 3]);
/// let b = get_i64_from_m128i_s(a);
/// assert_eq!(b, 1_i64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn get_i64_from_m128i_s(a: m128i) -> i64 {
  unsafe { _mm_cvtsi128_si64(a.0) }
}

/// Convert `i32` to `f64` and replace the low lane of the input.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = convert_i32_replace_m128d_s(a, 5_i32);
/// assert_eq!(b.to_array(), [5.0, 2.0]);
/// ```
/// * **Intrinsic:** [`_mm_cvtsi32_sd`]
/// * **Assembly:** `cvtsi2sd xmm, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_i32_replace_m128d_s(a: m128d, i: i32) -> m128d {
  m128d(unsafe { _mm_cvtsi32_sd(a.0, i) })
}

/// Set an `i32` as the low 32-bit lane of an `m128i`, other lanes blank.
/// ```
/// # use safe_arch::*;
/// let a: [i32; 4] = set_i32_m128i_s(1_i32).into();
/// let b: [i32; 4] = m128i::from([1, 0, 0, 0]).into();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i32_m128i_s(i: i32) -> m128i {
  m128i(unsafe { _mm_cvtsi32_si128(i) })
}

/// Convert `i64` to `f64` and replace the low lane of the input.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = convert_i64_replace_m128d_s(a, 5_i64);
/// assert_eq!(b.to_array(), [5.0, 2.0]);
/// ```
/// * **Intrinsic:** [`_mm_cvtsi64_sd`]
/// * **Assembly:** `cvtsi2sd xmm, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_i64_replace_m128d_s(a: m128d, i: i64) -> m128d {
  m128d(unsafe { _mm_cvtsi64_sd(a.0, i) })
}

/// Set an `i64` as the low 64-bit lane of an `m128i`, other lanes blank.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 2] = set_i64_m128i_s(1_i64).into();
/// let b: [i64; 2] = m128i::from([1_i64, 0]).into();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i64_m128i_s(i: i64) -> m128i {
  m128i(unsafe { _mm_cvtsi64_si128(i) })
}

/// Converts the lower `f32` to `f64` and replace the low lane of the input
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.5]);
/// let b = m128::from_array([3.0, 4.0, 5.0, 6.0]);
/// let c = convert_m128_s_replace_m128d_s(a, b);
/// assert_eq!(c.to_array(), [3.0, 2.5]);
/// ```
/// * **Intrinsic:** [`_mm_cvtss_sd`]
/// * **Assembly:** `cvtss2sd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn convert_m128_s_replace_m128d_s(a: m128d, b: m128) -> m128d {
  m128d(unsafe { _mm_cvtss_sd(a.0, b.0) })
}

/// Truncate the `f64` lanes to the lower `i32` lanes (upper `i32` lanes 0).
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.1, 2.6]);
/// let b = truncate_m128d_to_m128i(a);
/// assert_eq!(<[i32; 4]>::from(b), [1, 2, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn truncate_m128d_to_m128i(a: m128d) -> m128i {
  m128i(unsafe { _mm_cvttpd_epi32(a.0) })
}

/// Truncate the `f32` lanes to `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.1, 2.6, 3.5, 4.0]);
/// let b = truncate_m128_to_m128i(a);
/// assert_eq!(<[i32; 4]>::from(b), [1, 2, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn truncate_m128_to_m128i(a: m128) -> m128i {
  m128i(unsafe { _mm_cvttps_epi32(a.0) })
}

/// Truncate the lower lane into an `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.7, 2.6]);
/// assert_eq!(truncate_to_i32_m128d_s(a), 1_i32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn truncate_to_i32_m128d_s(a: m128d) -> i32 {
  unsafe { _mm_cvttsd_si32(a.0) }
}

/// Truncate the lower lane into an `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.7, 2.6]);
/// assert_eq!(truncate_to_i64_m128d_s(a), 1_i64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn truncate_to_i64_m128d_s(a: m128d) -> i64 {
  unsafe { _mm_cvttsd_si64(a.0) }
}

/// Lanewise `a / b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 42.0]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = div_m128d(a, b).to_array();
/// assert_eq!(c, [0.92, -7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn div_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_div_pd(a.0, b.0) })
}

/// Lowest lane `a / b`, high lane unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -600.0]);
/// let c = div_m128d_s(a, b).to_array();
/// assert_eq!(c, [0.92, 87.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn div_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_div_sd(a.0, b.0) })
}

/// Gets an `i16` value out of an `m128i`, returns as `i32`.
///
/// The lane to get must be a constant in `0..8`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0xA_i16, 0xB, 0xC, 0xD, 0, 0, 0, 0]);
/// //
/// assert_eq!(extract_i16_as_i32_m128i::<0>(a), 0xA);
/// assert_eq!(extract_i16_as_i32_m128i::<1>(a), 0xB);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn extract_i16_as_i32_m128i<const LANE: i32>(a: m128i) -> i32 {
  unsafe { _mm_extract_epi16(a.0, LANE) }
}

/// Inserts the low 16 bits of an `i32` value into an `m128i`.
///
/// The lane to get must be a constant in `0..8`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0xA_i16, 0xB, 0xC, 0xD, 0, 0, 0, 0]);
/// //
/// let b = insert_i16_from_i32_m128i::<0>(a, -1);
/// assert_eq!(<[i16; 8]>::from(b), [-1, 0xB, 0xC, 0xD, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn insert_i16_from_i32_m128i<const LANE: i32>(a: m128i, i: i32) -> m128i {
  m128i(unsafe { _mm_insert_epi16(a.0, i, LANE) })
}

/// Loads the reference into a register.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let b = load_m128d(&a);
/// assert_eq!(a.to_bits(), b.to_bits());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_m128d(a: &m128d) -> m128d {
  m128d(unsafe { _mm_load_pd(a as *const m128d as *const f64) })
}

/// Loads the `f64` reference into all lanes of a register.
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_f64_splat_m128d(&a);
/// assert_eq!(m128d::from_array([1.0, 1.0]).to_bits(), b.to_bits());
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::trivially_copy_pass_by_ref)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_f64_splat_m128d(a: &f64) -> m128d {
  m128d(unsafe { _mm_load1_pd(a) })
}

/// Loads the reference into the low lane of the register.
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_f64_m128d_s(&a);
/// assert_eq!(m128d::from_array([1.0, 0.0]).to_bits(), b.to_bits());
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::trivially_copy_pass_by_ref)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_f64_m128d_s(a: &f64) -> m128d {
  m128d(unsafe { _mm_load_sd(a) })
}

/// Loads the reference into a register.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = load_m128i(&a);
/// assert_eq!(<[i32; 4]>::from(a), <[i32; 4]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_m128i(a: &m128i) -> m128i {
  m128i(unsafe { _mm_load_si128(a as *const m128i as *const __m128i) })
}

/// Loads the reference into a register, replacing the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([1.0, 2.0]);
/// let double = 7.0;
/// let b = load_replace_high_m128d(a, &double);
/// assert_eq!(b.to_array(), [1.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_replace_high_m128d(a: m128d, b: &f64) -> m128d {
  m128d(unsafe { _mm_loadh_pd(a.0, b) })
}

/// Loads the low `i64` into a register.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 2]);
/// let b = load_i64_m128i_s(&a);
/// assert_eq!([1_i64, 0], <[i64; 2]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_i64_m128i_s(a: &m128i) -> m128i {
  m128i(unsafe { _mm_loadl_epi64(a as *const m128i as *const __m128i) })
}

/// Loads the reference into a register, replacing the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([1.0, 2.0]);
/// let double = 7.0;
/// let b = load_replace_low_m128d(a, &double);
/// assert_eq!(b.to_array(), [7.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_replace_low_m128d(a: m128d, b: &f64) -> m128d {
  m128d(unsafe { _mm_loadl_pd(a.0, b) })
}

/// Loads the reference into a register with reversed order.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let b = load_reverse_m128d(&a);
/// assert_eq!(m128d::from_array([12.0, 10.0]).to_bits(), b.to_bits());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_reverse_m128d(a: &m128d) -> m128d {
  m128d(unsafe { _mm_loadr_pd(a as *const m128d as *const f64) })
}

/// Loads the reference into a register.
///
/// This generally has no speed penalty if the reference happens to be 16-byte
/// aligned, but there is a slight speed penalty if the reference is only 8-byte
/// aligned.
/// ```
/// # use safe_arch::*;
/// let a = [10.0, 12.0];
/// let b = load_unaligned_m128d(&a);
/// assert_eq!(m128d::from_array(a).to_bits(), b.to_bits());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_unaligned_m128d(a: &[f64; 2]) -> m128d {
  m128d(unsafe { _mm_loadu_pd(a as *const [f64; 2] as *const f64) })
}

/// Loads the reference into a register.
///
/// This generally has no speed penalty if the reference happens to be 16-byte
/// aligned, but there is a slight speed penalty if the reference is less
/// aligned.
/// ```
/// # use safe_arch::*;
/// let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
/// let b = load_unaligned_m128i(&a);
/// assert_eq!(a, <[u8; 16]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::cast_ptr_alignment)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn load_unaligned_m128i(a: &[u8; 16]) -> m128i {
  m128i(unsafe { _mm_loadu_si128(a as *const [u8; 16] as *const __m128i) })
}

/// Multiply `i16` lanes producing `i32` values, horizontal add pairs of `i32`
/// values to produce the final output.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i32; 4] = mul_i16_horizontal_add_m128i(a, b).into();
/// assert_eq!(c, [17, 53, 67, -81]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_i16_horizontal_add_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_madd_epi16(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([0_u8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127]);
/// let c: [u8; 16] = max_u8_m128i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn max_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epu8(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = max_i16_m128i(a, b).into();
/// assert_eq!(c, [5_i16, 6, 7, 8, -1, -2, -3, 48]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn max_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epi16(a.0, b.0) })
}

/// Lanewise `max(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([5.0, 2.0]);
/// let b = m128d::from_array([1.0, 6.0]);
/// let c = max_m128d(a, b).to_array();
/// assert_eq!(c, [5.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn max_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_max_pd(a.0, b.0) })
}

/// Low lane `max(a, b)`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 12.0]);
/// let b = m128d::from_array([5.0, 6.0]);
/// let c = max_m128d_s(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn max_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_max_sd(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([0_u8, 11, 2, 13, 4, 15, 6, 17, 8, 0, 20, 0, 22, 0, 24, 0]);
/// let c: [u8; 16] = min_u8_m128i(a, b).into();
/// assert_eq!(c, [0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 0, 10, 0, 12, 0, 14, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn min_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epu8(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = min_i16_m128i(a, b).into();
/// assert_eq!(c, [1_i16, 2, 3, 4, -15, -26, -37, -4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn min_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epi16(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 12.0]);
/// let b = m128d::from_array([5.0, 6.0]);
/// let c = min_m128d(a, b).to_array();
/// assert_eq!(c, [1.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn min_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_min_pd(a.0, b.0) })
}

/// Low lane `min(a, b)`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 12.0]);
/// let b = m128d::from_array([0.0, 6.0]);
/// let c = min_m128d_s(a, b).to_array();
/// assert_eq!(c, [0.0, 12.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn min_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_min_sd(a.0, b.0) })
}

/// Copy the low `i64` lane to a new register, upper bits 0.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 2]);
/// let b = copy_i64_m128i_s(a);
/// assert_eq!(<[i64; 2]>::from(b), [1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn copy_i64_m128i_s(a: m128i) -> m128i {
  m128i(unsafe { _mm_move_epi64(a.0) })
}

/// Copies the `a` value and replaces the low lane with the low `b` value.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([1.0, 2.0]);
/// let b = m128d::from([3.0, 4.0]);
/// let c = copy_replace_low_f64_m128d(a, b);
/// assert_eq!(c.to_array(), [3.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn copy_replace_low_f64_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_move_sd(a.0, b.0) })
}

/// Gathers the `i8` sign bit of each lane.
///
/// The output has lane 0 as bit 0, lane 1 as bit 1, and so on.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, -11, -2, 13, 4, 15, -6, 17, 8, 19, -20, 21, 22, 23, -24, 127]);
/// let i = move_mask_i8_m128i(a);
/// assert_eq!(i, 0b0100010001000110);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn move_mask_i8_m128i(a: m128i) -> i32 {
  unsafe { _mm_movemask_epi8(a.0) }
}

/// Gathers the sign bit of each lane.
///
/// The output has lane 0 as bit 0, lane 1 as bit 1.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-1.0, 12.0]);
/// let i = move_mask_m128d(a);
/// assert_eq!(i, 0b01);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn move_mask_m128d(a: m128d) -> i32 {
  unsafe { _mm_movemask_pd(a.0) }
}

/// Multiplies the odd `u32` lanes and gives the widened (`u64`) results.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 7, u32::MAX, 7]);
/// let b = m128i::from([5, 7, u32::MAX, 7]);
/// let c: [u64; 2] = mul_widen_u32_odd_m128i(a, b).into();
/// assert_eq!(c, [(1 * 5), (u32::MAX as u64 * u32::MAX as u64)]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_widen_u32_odd_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mul_epu32(a.0, b.0) })
}

/// Lanewise `a * b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = mul_m128d(a, b).to_array();
/// assert_eq!(c, [9200.0, -525.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_mul_pd(a.0, b.0) })
}

/// Lowest lane `a * b`, high lane unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -600.0]);
/// let c = mul_m128d_s(a, b).to_array();
/// assert_eq!(c, [9200.0, 87.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_mul_sd(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `i16`, keep the high bits of the `i32`
/// intermediates.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 200, 300, 4568, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 600, 700, 8910, -15, -26, -37, 48]);
/// let c: [i16; 8] = mul_i16_keep_high_m128i(a, b).into();
/// assert_eq!(c, [0, 1, 3, 621, 0, 0, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_i16_keep_high_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mulhi_epi16(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `u16`, keep the high bits of the `u32`
/// intermediates.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2003, 3005, 45687, 1, 2, 3, 4]);
/// let b = m128i::from([5_u16, 6004, 7006, 8910, 15, 26, 37, 48]);
/// let c: [u16; 8] = mul_u16_keep_high_m128i(a, b).into();
/// assert_eq!(c, [0, 183, 321, 6211, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_u16_keep_high_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mulhi_epu16(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `i16`, keep the low bits of the `i32`
/// intermediates.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 200, 300, 4568, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 600, 700, 8910, -15, -26, -37, 48]);
/// let c: [i16; 8] = mul_i16_keep_low_m128i(a, b).into();
/// assert_eq!(c, [5, -11072, 13392, 3024, 15, 52, 111, -192]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn mul_i16_keep_low_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mullo_epi16(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = bitor_m128d(a, b).to_array();
/// assert_eq!(c, [1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitor_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_or_pd(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = bitor_m128i(a, b).into();
/// assert_eq!(c, [1, 1, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitor_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_or_si128(a.0, b.0) })
}

/// Saturating convert `i16` to `i8`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m128i::from([9_i16, 10, 11, 12, 13, 14, 15, 16]);
/// let c: [i8; 16] = pack_i16_to_i8_m128i(a, b).into();
/// assert_eq!(c, [1_i8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn pack_i16_to_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_packs_epi16(a.0, b.0) })
}

/// Saturating convert `i32` to `i16`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i32, 2, 3, 4]);
/// let b = m128i::from([5_i32, 6, 7, 8]);
/// let c: [i16; 8] = pack_i32_to_i16_m128i(a, b).into();
/// assert_eq!(c, [1_i16, 2, 3, 4, 5, 6, 7, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn pack_i32_to_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_packs_epi32(a.0, b.0) })
}

/// Saturating convert `i16` to `u8`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-1_i16, 2, -3, 4, -5, 6, -7, 8]);
/// let b = m128i::from([9_i16, 10, 11, 12, 13, -14, 15, -16]);
/// let c: [u8; 16] = pack_i16_to_u8_m128i(a, b).into();
/// assert_eq!(c, [0, 2, 0, 4, 0, 6, 0, 8, 9, 10, 11, 12, 13, 0, 15, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn pack_i16_to_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_packus_epi16(a.0, b.0) })
}

/// Compute "sum of `u8` absolute differences".
///
/// * `u8` lanewise `abs(a - b)`, producing `u8` intermediate values.
/// * Sum the first eight and second eight values.
/// * Place into the low 16 bits of two `u64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_u8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127]);
/// let b = m128i::from([20_u8, 110, 250, 103, 34, 105, 60, 217, 8, 19, 210, 201, 202, 203, 204, 127]);
/// let c: [u64; 2] = sum_of_u8_abs_diff_m128i(a, b).into();
/// assert_eq!(c, [831_u64, 910]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sum_of_u8_abs_diff_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sad_epu8(a.0, b.0) })
}

/// Sets the args into an `m128i`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([15_i8, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
/// let b = set_i8_m128i(0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
/// assert_eq!(<[i8; 16]>::from(a), <[i8; 16]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i8_m128i(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> m128i {
  m128i(unsafe { _mm_set_epi8(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) })
}

/// Sets the args into an `m128i`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([7_i16, 6, 5, 4, 3, 2, 1, 0]);
/// let b = set_i16_m128i(0_i16, 1, 2, 3, 4, 5, 6, 7);
/// assert_eq!(<[i16; 8]>::from(a), <[i16; 8]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i16_m128i(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16, h: i16) -> m128i {
  m128i(unsafe { _mm_set_epi16(a, b, c, d, e, f, g, h) })
}

/// Sets the args into an `m128i`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([3, 2, 1, 0]);
/// let b = set_i32_m128i(0, 1, 2, 3);
/// assert_eq!(<[i32; 4]>::from(a), <[i32; 4]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i32_m128i(a: i32, b: i32, c: i32, d: i32) -> m128i {
  m128i(unsafe { _mm_set_epi32(a, b, c, d) })
}

/// Sets the args into an `m128i`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 0]);
/// let b = set_i64_m128i(0, 1);
/// assert_eq!(<[i64; 2]>::from(a), <[i64; 2]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_i64_m128i(a: i64, b: i64) -> m128i {
  m128i(unsafe { _mm_set_epi64x(a, b) })
}

/// Sets the args into an `m128d`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = set_m128d(0.0, 1.0);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_m128d(a: f64, b: f64) -> m128d {
  m128d(unsafe { _mm_set_pd(a, b) })
}

/// Sets the args into the low lane of a `m128d`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = set_m128d_s(1.0);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_m128d_s(a: f64) -> m128d {
  m128d(unsafe { _mm_set_sd(a) })
}

/// Splats the args into both lanes of the `m128d`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 1.0]);
/// let b = set_splat_m128d(1.0);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_splat_m128d(a: f64) -> m128d {
  m128d(unsafe { _mm_set1_pd(a) })
}

/// Splats the `i8` to all lanes of the `m128i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
/// let b = set_splat_i8_m128i(1);
/// assert_eq!(<[i8; 16]>::from(a), <[i8; 16]>::from(a));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_splat_i8_m128i(i: i8) -> m128i {
  m128i(unsafe { _mm_set1_epi8(i) })
}

/// Splats the `i16` to all lanes of the `m128i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 1, 1, 1, 1, 1, 1, 1]);
/// let b = set_splat_i16_m128i(1);
/// assert_eq!(<[i16; 8]>::from(a), <[i16; 8]>::from(a));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_splat_i16_m128i(i: i16) -> m128i {
  m128i(unsafe { _mm_set1_epi16(i) })
}

/// Splats the `i32` to all lanes of the `m128i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 1, 1, 1]);
/// let b = set_splat_i32_m128i(1);
/// assert_eq!(<[i32; 4]>::from(a), <[i32; 4]>::from(a));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_splat_i32_m128i(i: i32) -> m128i {
  m128i(unsafe { _mm_set1_epi32(i) })
}

/// Splats the `i64` to both lanes of the `m128i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 1]);
/// let b = set_splat_i64_m128i(1);
/// assert_eq!(<[i64; 2]>::from(a), <[i64; 2]>::from(a));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_splat_i64_m128i(i: i64) -> m128i {
  m128i(unsafe { _mm_set1_epi64x(i) })
}

/// Sets the args into an `m128i`, first arg is the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = set_reversed_i8_m128i(0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
/// assert_eq!(<[i8; 16]>::from(a), <[i8; 16]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_reversed_i8_m128i(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> m128i {
  m128i(unsafe { _mm_setr_epi8(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) })
}

/// Sets the args into an `m128i`, first arg is the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i16, 1, 2, 3, 4, 5, 6, 7]);
/// let b = set_reversed_i16_m128i(0_i16, 1, 2, 3, 4, 5, 6, 7);
/// assert_eq!(<[i16; 8]>::from(a), <[i16; 8]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::many_single_char_names)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_reversed_i16_m128i(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16, h: i16) -> m128i {
  m128i(unsafe { _mm_setr_epi16(a, b, c, d, e, f, g, h) })
}

/// Sets the args into an `m128i`, first arg is the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0, 1, 2, 3]);
/// let b = set_reversed_i32_m128i(0, 1, 2, 3);
/// assert_eq!(<[i32; 4]>::from(a), <[i32; 4]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_reversed_i32_m128i(a: i32, b: i32, c: i32, d: i32) -> m128i {
  m128i(unsafe { _mm_setr_epi32(a, b, c, d) })
}

/// Sets the args into an `m128d`, first arg is the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = set_reversed_m128d(0.0, 1.0);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn set_reversed_m128d(a: f64, b: f64) -> m128d {
  m128d(unsafe { _mm_setr_pd(a, b) })
}

/// All lanes zero.
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m128i();
/// assert_eq!(u128::from(a), 0);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn zeroed_m128i() -> m128i {
  m128i(unsafe { _mm_setzero_si128() })
}

/// Both lanes zero.
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m128d();
/// assert_eq!(a.to_array(), [0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn zeroed_m128d() -> m128d {
  m128d(unsafe { _mm_setzero_pd() })
}

/// Shuffle the `i32` lanes in `$a` using an immediate
/// control value.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([6, 7, 8, 9]);
/// //
/// let c = shuffle_ai_f32_all_m128i::<0b01_10_10_00>(a);
/// assert_eq!(<[i32; 4]>::from(c), [6, 8, 8, 7]);
/// ```
/// * **Intrinsic:** [`_mm_shuffle_epi32`]
/// * **Assembly:** `pshufd xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shuffle_ai_f32_all_m128i<const MASK: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_shuffle_epi32(a.0, MASK) })
}

/// Shuffle the `f64` lanes from `$a` and `$b` together using an immediate
/// control value.
///
/// The `a:` and `b:` prefixes on the index selection values are literal tokens
/// that you type. It helps keep clear what value comes from where. The first
/// two output lanes come from `$a`, the second two output lanes come from `$b`.
///
/// You can pass the same value as both arguments, but if you want to swizzle
/// within only a single register and you have `avx` available consider using
/// [`shuffle_ai_f64_all_m128d`] instead. You'll get much better performance.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = m128d::from_array([3.0, 4.0]);
/// //
/// let c = shuffle_abi_f64_all_m128d::<0b00>(a, b).to_array();
/// assert_eq!(c, [1.0, 3.0]);
/// //
/// let c = shuffle_abi_f64_all_m128d::<0b10>(a, b).to_array();
/// assert_eq!(c, [1.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shuffle_abi_f64_all_m128d<const MASK: i32>(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_shuffle_pd(a.0, b.0, MASK) })
}

/// Shuffle the high `i16` lanes in `$a` using an immediate control value.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, 5, 6, 7, 8]);
/// let c = shuffle_ai_i16_h64all_m128i::<0b01_00_10_11>(a);
/// assert_eq!(<[i16; 8]>::from(c), [1_i16, 2, 3, 4, 8, 7, 5, 6]);
/// ```
/// * **Intrinsic:** [`_mm_shufflehi_epi16`]
/// * **Assembly:** `pshufhw xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shuffle_ai_i16_h64all_m128i<const MASK: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_shufflehi_epi16(a.0, MASK) })
}

/// Shuffle the low `i16` lanes in `$a` using an immediate control value.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, 5, 6, 7, 8]);
/// //
/// let c = shuffle_ai_i16_l64all_m128i::<0b01_11_10_00>(a);
/// assert_eq!(<[i16; 8]>::from(c), [1_i16, 3, 4, 2, 5, 6, 7, 8]);
/// ```
/// * **Intrinsic:** [`_mm_shufflelo_epi16`]
/// * **Assembly:** `pshuflw xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shuffle_ai_i16_l64all_m128i<const MASK: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_shufflelo_epi16(a.0, MASK) })
}

/// Shift all `u16` lanes to the left by the `count` in the lower `u64` lane.
///
/// New bits are 0s.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 3, 4, 1, 2, 3, 4]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u16; 8] = shl_all_u16_m128i(a, b).into();
/// assert_eq!(c, [1_u16 << 3, 2 << 3, 3 << 3, 4 << 3, 1 << 3, 2 << 3, 3 << 3, 4 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_all_u16_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sll_epi16(a.0, count.0) })
}

/// Shift all `u32` lanes to the left by the `count` in the lower `u64` lane.
///
/// New bits are 0s.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u32, 2, 3, 4]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u32; 4] = shl_all_u32_m128i(a, b).into();
/// assert_eq!(c, [1 << 3, 2 << 3, 3 << 3, 4 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_all_u32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sll_epi32(a.0, count.0) })
}

/// Shift all `u64` lanes to the left by the `count` in the lower `u64` lane.
///
/// New bits are 0s.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 2]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u64; 2] = shl_all_u64_m128i(a, b).into();
/// assert_eq!(c, [1 << 3, 2 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_all_u64_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sll_epi64(a.0, count.0) })
}

/// Shifts all `u16` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 3, 4, 1, 2, 3, 4]);
/// let c: [u16; 8] = shl_imm_u16_m128i::<3>(a).into();
/// assert_eq!(c, [1_u16 << 3, 2 << 3, 3 << 3, 4 << 3, 1 << 3, 2 << 3, 3 << 3, 4 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_imm_u16_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_slli_epi16(a.0, IMM) })
}

/// Shifts all `u32` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let c: [u32; 4] = shl_imm_u32_m128i::<3>(a).into();
/// assert_eq!(c, [1 << 3, 2 << 3, 3 << 3, 4 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_imm_u32_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_slli_epi32(a.0, IMM) })
}

/// Shifts both `u64` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 2]);
/// let c: [u64; 2] = shl_imm_u64_m128i::<3>(a).into();
/// assert_eq!(c, [1_u64 << 3, 2 << 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shl_imm_u64_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_slli_epi64(a.0, IMM) })
}

/// Lanewise `sqrt(a)`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([25.0, 16.0]);
/// let b = sqrt_m128d(a).to_array();
/// assert_eq!(b, [5.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sqrt_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_sqrt_pd(a.0) })
}

/// Low lane `sqrt(b)`, upper lane is unchanged from `a`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = m128d::from_array([25.0, 4.0]);
/// let c = sqrt_m128d_s(a, b);
/// assert_eq!(c.to_array(), [5.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sqrt_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_sqrt_sd(a.0, b.0) })
}

/// Shift each `i16` lane to the right by the `count` in the lower `i64` lane.
///
/// New bits are the sign bit.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([3_i64, 0]);
/// let c: [i16; 8] = shr_all_i16_m128i(a, b).into();
/// assert_eq!(c, [1_i16 >> 3, 2 >> 3, 3 >> 3, 4 >> 3, -1 >> 3, -2 >> 3, -3 >> 3, -4 >> 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_all_i16_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sra_epi16(a.0, count.0) })
}

/// Shift each `i32` lane to the right by the `count` in the lower `i64` lane.
///
/// New bits are the sign bit.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i32, 2, -3, -4]);
/// let b = m128i::from([3_i64, 0]);
/// let c: [i32; 4] = shr_all_i32_m128i(a, b).into();
/// assert_eq!(c, [1 >> 3, 2 >> 3, -3 >> 3, -4 >> 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_all_i32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sra_epi32(a.0, count.0) })
}

/// Shifts all `i16` lanes right by an immediate.
///
/// New bits are the sign bit.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let c: [i16; 8] = shr_imm_i16_m128i::<3>(a).into();
/// assert_eq!(c, [1_i16 >> 3, 2 >> 3, 3 >> 3, 4 >> 3, -1 >> 3, -2 >> 3, -3 >> 3, -4 >> 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_imm_i16_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_srai_epi16(a.0, IMM) })
}

/// Shifts all `i32` lanes right by an immediate.
///
/// New bits are the sign bit.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, -3, -4]);
/// let c: [i32; 4] = shr_imm_i32_m128i::<3>(a).into();
/// assert_eq!(c, [1 >> 3, 2 >> 3, -3 >> 3, -4 >> 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_imm_i32_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_srai_epi32(a.0, IMM) })
}

/// Shift each `u16` lane to the right by the `count` in the lower `u64` lane.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 3, 4, 100, 200, 300, 400]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u16; 8] = shr_all_u16_m128i(a, b).into();
/// assert_eq!(c, [1_u16 >> 3, 2 >> 3, 3 >> 3, 4 >> 3, 100 >> 3, 200 >> 3, 300 >> 3, 400 >> 3,]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_all_u16_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srl_epi16(a.0, count.0) })
}

/// Shift each `u32` lane to the right by the `count` in the lower `u64` lane.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u32, 2, 300, 400]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u32; 4] = shr_all_u32_m128i(a, b).into();
/// assert_eq!(c, [1 >> 3, 2 >> 3, 300 >> 3, 400 >> 3,]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_all_u32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srl_epi32(a.0, count.0) })
}

/// Shift each `u64` lane to the right by the `count` in the lower `u64` lane.
///
/// New bits are 0s.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 56]);
/// let b = m128i::from([3_u64, 0]);
/// let c: [u64; 2] = shr_all_u64_m128i(a, b).into();
/// assert_eq!(c, [1 >> 3, 56 >> 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_all_u64_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srl_epi64(a.0, count.0) })
}

/// Shifts all `u16` lanes right by an immediate.
///
/// New bits are 0s.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 3, 4, 100, 200, 300, 400]);
/// let c: [u16; 8] = shr_imm_u16_m128i::<3>(a).into();
/// assert_eq!(c, [1_u16 >> 3, 2 >> 3, 3 >> 3, 4 >> 3, 100 >> 3, 200 >> 3, 300 >> 3, 400 >> 3,]);
/// ```
/// * **Intrinsic:** [`_mm_srli_epi16`]
/// * **Assembly:** `psrlw xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_imm_u16_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_srli_epi16(a.0, IMM) })
}

/// Shifts all `u32` lanes right by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 300, 400]);
/// let c: [u32; 4] = shr_imm_u32_m128i::<3>(a).into();
/// assert_eq!(c, [1 >> 3, 2 >> 3, 300 >> 3, 400 >> 3]);
/// ```
/// * **Intrinsic:** [`_mm_srli_epi32`]
/// * **Assembly:** `psrld xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_imm_u32_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_srli_epi32(a.0, IMM) })
}

/// Shifts both `u64` lanes right by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 200]);
/// let c: [u64; 2] = shr_imm_u64_m128i::<3>(a).into();
/// assert_eq!(c, [1_u64 >> 3, 200 >> 3]);
/// ```
/// * **Intrinsic:** [`_mm_srli_epi64`]
/// * **Assembly:** `psrlq xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn shr_imm_u64_m128i<const IMM: i32>(a: m128i) -> m128i {
  m128i(unsafe { _mm_srli_epi64(a.0, IMM) })
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut b = zeroed_m128d();
/// store_m128d(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [10.0, 12.0]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_m128d(r: &mut m128d, a: m128d) {
  unsafe { _mm_store_pd(r as *mut m128d as *mut f64, a.0) }
}

/// Stores the low lane value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut f = 0.0;
/// store_m128d_s(&mut f, a);
/// assert_eq!(f, 10.0);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_m128d_s(r: &mut f64, a: m128d) {
  unsafe { _mm_store_sd(r as *mut f64, a.0) }
}

/// Stores the low lane value to all lanes of the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut b = zeroed_m128d();
/// store_splat_m128d(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [10.0, 10.0]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_splat_m128d(r: &mut m128d, a: m128d) {
  unsafe { _mm_store1_pd(r as *mut m128d as *mut f64, a.0) }
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let mut b = zeroed_m128i();
/// store_m128i(&mut b, a);
/// let c: [i32; 4] = b.into();
/// assert_eq!(c, [1, 2, 3, 4]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_m128i(r: &mut m128i, a: m128i) {
  unsafe { _mm_store_si128(&mut r.0, a.0) }
}

/// Stores the high lane value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut f = 0.0;
/// store_high_m128d_s(&mut f, a);
/// assert_eq!(f, 12.0);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_high_m128d_s(r: &mut f64, a: m128d) {
  unsafe { _mm_storeh_pd(r as *mut f64, a.0) }
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 2]);
/// let mut b = 0_i64;
/// store_i64_m128i_s(&mut b, a);
/// assert_eq!(b, 1_i64);
/// ```
#[inline(always)]
#[allow(clippy::cast_ptr_alignment)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_i64_m128i_s(r: &mut i64, a: m128i) {
  unsafe { _mm_storel_epi64(r as *mut i64 as *mut __m128i, a.0) }
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut b = zeroed_m128d();
/// store_reversed_m128d(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [12.0, 10.0]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_reversed_m128d(r: &mut m128d, a: m128d) {
  unsafe { _mm_storer_pd(r as *mut m128d as *mut f64, a.0) }
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 12.0]);
/// let mut b = [0.0, 0.0];
/// store_unaligned_m128d(&mut b, a);
/// assert_eq!(b, [10.0, 12.0]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_unaligned_m128d(r: &mut [f64; 2], a: m128d) {
  unsafe { _mm_storeu_pd(r.as_mut_ptr(), a.0) }
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let mut b = [0_u8; 16];
/// store_unaligned_m128i(&mut b, a);
/// assert_eq!(b, [0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn store_unaligned_m128i(r: &mut [u8; 16], a: m128i) {
  unsafe { _mm_storeu_si128(r.as_mut_ptr().cast(), a.0) }
}

/// Lanewise `a - b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = sub_i8_m128i(a, b).into();
/// assert_eq!(c, [0, -10, 0, -10, 0, -10, 0, -10, 0, -10, 30, -10, -10, 36, -10, -112]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sub_epi8(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([51_i16, 61, 71, 81, -15, -26, -37, 48]);
/// let c: [i16; 8] = sub_i16_m128i(a, b).into();
/// assert_eq!(c, [-50, -59, -68, -77, 14, 24, 34, -52]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sub_epi16(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([50, 60, 70, 87]);
/// let c: [i32; 4] = sub_i32_m128i(a, b).into();
/// assert_eq!(c, [-49, -58, -67, -83]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sub_epi32(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([92_i64, 87]);
/// let b = m128i::from([-9001_i64, 1]);
/// let c: [i64; 2] = sub_i64_m128i(a, b).into();
/// assert_eq!(c, [9093, 86]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_sub_epi64(a.0, b.0) })
}

/// Lanewise `a - b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = sub_m128d(a, b).to_array();
/// assert_eq!(c, [-8.0, 93.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_sub_pd(a.0, b.0) })
}

/// Lowest lane `a - b`, high lane unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -600.0]);
/// let c = sub_m128d_s(a, b).to_array();
/// assert_eq!(c, [-8.0, 87.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_sub_sd(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, -128, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, -127]);
/// let b = m128i::from([0_i8, 1, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = sub_saturating_i8_m128i(a, b).into();
/// assert_eq!(c, [0, -128, 0, -10, 0, -10, 0, -10, 0, -10, 30, -10, -10, 36, -10, -128]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_saturating_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_subs_epi8(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([51_i16, 61, 71, 81, i16::MAX, -26, -37, 48]);
/// let c: [i16; 8] = sub_saturating_i16_m128i(a, b).into();
/// assert_eq!(c, [-50, -59, -68, -77, -32768, 24, 34, -52]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_saturating_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_subs_epi16(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([10_u8, 255, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 255]);
/// let b = m128i::from([1_u8, 1, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127]);
/// let c: [u8; 16] = sub_saturating_u8_m128i(a, b).into();
/// assert_eq!(c, [9_u8, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_saturating_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_subs_epu8(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([51_u16, 61, 3, 4, u16::MAX, 2, 3, u16::MAX]);
/// let b = m128i::from([5_u16, 2, 71, 81, u16::MAX, 26, 37, u16::MIN]);
/// let c: [u16; 8] = sub_saturating_u16_m128i(a, b).into();
/// assert_eq!(c, [46, 59, 0, 0, 0, 0, 0, u16::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn sub_saturating_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_subs_epu16(a.0, b.0) })
}

/// Unpack and interleave high `i8` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([0_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = unpack_high_i8_m128i(a, b).into();
/// assert_eq!(c, [8, 8, 9, 19, 10, -20, 11, 21, 12, 22, 13, -23, 14, 24, 15, 127]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_high_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpackhi_epi8(a.0, b.0) })
}

/// Unpack and interleave high `i16` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = unpack_high_i16_m128i(a, b).into();
/// assert_eq!(c, [-1, -15, -2, -26, -3, -37, -4, 48]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_high_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpackhi_epi16(a.0, b.0) })
}

/// Unpack and interleave high `i32` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [i32; 4] = unpack_high_i32_m128i(a, b).into();
/// assert_eq!(c, [3, 7, 4, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_high_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpackhi_epi32(a.0, b.0) })
}

/// Unpack and interleave high `i64` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([92_i64, 87]);
/// let b = m128i::from([-9001_i64, 1]);
/// let c: [i64; 2] = unpack_high_i64_m128i(a, b).into();
/// assert_eq!(c, [87, 1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_high_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpackhi_epi64(a.0, b.0) })
}

/// Unpack and interleave high lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = unpack_high_m128d(a, b).to_array();
/// assert_eq!(c, [87.5, -6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_high_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_unpackhi_pd(a.0, b.0) })
}

/// Unpack and interleave low `i8` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([12_i8, 11, 22, 13, 99, 15, 16, 17, 8, 19, -20, 21, 22, -23, 24, 127]);
/// let c: [i8; 16] = unpack_low_i8_m128i(a, b).into();
/// assert_eq!(c, [0, 12, 1, 11, 2, 22, 3, 13, 4, 99, 5, 15, 6, 16, 7, 17]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_low_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpacklo_epi8(a.0, b.0) })
}

/// Unpack and interleave low `i16` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = unpack_low_i16_m128i(a, b).into();
/// assert_eq!(c, [1, 5, 2, 6, 3, 7, 4, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_low_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpacklo_epi16(a.0, b.0) })
}

/// Unpack and interleave low `i32` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [i32; 4] = unpack_low_i32_m128i(a, b).into();
/// assert_eq!(c, [1, 5, 2, 6]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_low_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpacklo_epi32(a.0, b.0) })
}

/// Unpack and interleave low `i64` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([92_i64, 87]);
/// let b = m128i::from([-9001_i64, 1]);
/// let c: [i64; 2] = unpack_low_i64_m128i(a, b).into();
/// assert_eq!(c, [92, -9001]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_low_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_unpacklo_epi64(a.0, b.0) })
}

/// Unpack and interleave low lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = unpack_low_m128d(a, b).to_array();
/// assert_eq!(c, [92.0, 100.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn unpack_low_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_unpacklo_pd(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = bitxor_m128d(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitxor_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_xor_pd(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = bitxor_m128i(a, b).into();
/// assert_eq!(c, [0, 1, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse2")))]
pub fn bitxor_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_xor_si128(a.0, b.0) })
}

//
// Here we define the Operator Overloads for `m128`. Each one just calls the
// correct function from above. By putting the impls here and not with the
// `m128` type we theoretically would be able to build the crate safely even if
// there's no `sse` feature enabled. You'd just have a `m128` type without the
// operator overloads is all. Not that the standard Rust distribution can build
// properly without `sse` enabled, but maybe you're using a custom target or
// something. It doesn't really put us out of our way, so it doesn't hurt to try
// and accommodate the potential use case.
//

// First we provide all `m128d` impls.

impl Add for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    add_m128d(self, rhs)
  }
}
impl AddAssign for m128d {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m128d(self, rhs)
  }
}
impl BitAndAssign for m128d {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m128d(self, rhs)
  }
}
impl BitOrAssign for m128d {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m128d(self, rhs)
  }
}
impl BitXorAssign for m128d {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    div_m128d(self, rhs)
  }
}
impl DivAssign for m128d {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    mul_m128d(self, rhs)
  }
}
impl MulAssign for m128d {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn neg(self) -> Self {
    sub_m128d(zeroed_m128d(), self)
  }
}

impl Not for m128d {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m128d(f64::from_bits(u64::MAX));
    self ^ all_bits
  }
}

impl Sub for m128d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    sub_m128d(self, rhs)
  }
}
impl SubAssign for m128d {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl PartialEq for m128d {
  /// Not a direct intrinsic, this is a `cmp_eq_mask` and then a `move_mask`.
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    move_mask_m128d(cmp_eq_mask_m128d(*self, *other)) == 0b11
  }
}

// Next we provide all `m128i` impls. Since the interpretation of the lanes
// depends on the operation used, we only provide the bit ops (which are "lane
// agnostic").

impl BitAnd for m128i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m128i(self, rhs)
  }
}
impl BitAndAssign for m128i {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m128i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m128i(self, rhs)
  }
}
impl BitOrAssign for m128i {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m128i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m128i(self, rhs)
  }
}
impl BitXorAssign for m128i {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Not for m128i {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_i32_m128i(-1);
    self ^ all_bits
  }
}

impl PartialEq for m128i {
  /// Not a direct intrinsic, this is a `cmp_eq_mask_i8_m128i` and then a
  /// `move_mask_i8_m128i`.
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    move_mask_i8_m128i(cmp_eq_mask_i8_m128i(*self, *other)) == 0b11111111_11111111
  }
}
/// Unlike with the floating types, ints have absolute equality.
impl Eq for m128i {}

