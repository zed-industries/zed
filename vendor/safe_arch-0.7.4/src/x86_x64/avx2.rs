#![cfg(target_feature = "avx2")]

use super::*;

/// Blends the `i32` lanes in `a` and `b` into a single value.
///
/// * The blend is controlled by an immediate mask value (an `i32`).
/// * For each lane `0..=3`, use `0` if you want that lane of the output to be
///   from `a` and use `1` if you want that lane of the output to be from `b`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([10, 20, 30, 40]);
/// let b = m128i::from([100, 200, 300, 400]);
/// //
/// let c: [i32; 4] = blend_imm_i32_m128i::<0b0110>(a, b).into();
/// assert_eq!(c, [10, 200, 300, 40]);
/// ```
/// * **Intrinsic:** [`_mm_blend_epi32`]
/// * **Assembly:** `vpblendd xmm, xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn blend_imm_i32_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_blend_epi32(a.0, b.0, IMM) })
}

/// Splat the lowest 8-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i8; 16] = splat_i8_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i8; 16]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastb_epi8`]
/// * **Assembly:** `vpbroadcastb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i8_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastb_epi8(a.0) })
}

/// Splat the lowest 16-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i16; 8] = splat_i16_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i16; 8]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastw_epi16`]
/// * **Assembly:** `vpbroadcastw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i16_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastw_epi16(a.0) })
}

/// Splat the lowest 32-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i32; 4] = splat_i32_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77; 4]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastd_epi32`]
/// * **Assembly:** `vpbroadcastd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i32_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastd_epi32(a.0) })
}

/// Splat the lowest 64-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i64; 2] = splat_i64_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i64; 2]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastq_epi64`]
/// * **Assembly:** `vpbroadcastq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i64_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastq_epi64(a.0) })
}

/// Splat the lower `f64` across both lanes of `m128d`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([1.0, 2.0]);
/// let b = splat_m128d_s_m128d(a).to_array();
/// assert_eq!(b, [1.0, 1.0]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastsd_pd`]
/// * **Assembly:** `movddup xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128d_s_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_broadcastsd_pd(a.0) })
}

/// Splat the 128-bits across 256-bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(1_i128);
/// let b: [i128; 2] = splat_m128i_m256i(a).into();
/// assert_eq!(b, [1_i128, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastsi128_si256`]
/// * **Assembly:** `vbroadcasti128 ymm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastsi128_si256(a.0) })
}

/// Splat the lowest `f32` across all four lanes.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_s(1.0);
/// let b = splat_m128_s_m128(a).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastss_ps`]
/// * **Assembly:** `vbroadcastss xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128_s_m128(a: m128) -> m128 {
  m128(unsafe { _mm_broadcastss_ps(a.0) })
}

/// Loads the reference given and zeroes any `i32` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m128i(5);
/// let b = load_masked_i32_m128i(&a, m128i::from([-1_i32, 0, 0, -1]));
/// assert_eq!(<[i32; 4]>::from(b), [5, 0, 0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskload_epi32`]
/// * **Assembly:** `vpmaskmovd xmm, xmm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i32_m128i(a: &m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_maskload_epi32(a as *const m128i as *const i32, mask.0) })
}

/// Loads the reference given and zeroes any `i64` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m128i(5);
/// let b = load_masked_i64_m128i(&a, m128i::from([0_i64, -1]));
/// assert_eq!(<[i64; 2]>::from(b), [0_i64, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskload_epi64`]
/// * **Assembly:** `vpmaskmovq xmm, xmm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i64_m128i(a: &m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_maskload_epi64(a as *const m128i as *const i64, mask.0) })
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m128i::default();
/// store_masked_i32_m128i(&mut a, m128i::from([-1_i32, 0, 0, -1]), set_splat_i32_m128i(5));
/// assert_eq!(<[i32; 4]>::from(a), [5, 0, 0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskstore_epi32`]
/// * **Assembly:** `vpmaskmovd m128, xmm, xmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i32_m128i(addr: &mut m128i, mask: m128i, a: m128i) {
  unsafe { _mm_maskstore_epi32(addr as *mut m128i as *mut i32, mask.0, a.0) };
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m128i::default();
/// store_masked_i64_m128i(&mut a, m128i::from([0_i64, -1]), set_splat_i64_m128i(5));
/// assert_eq!(<[i64; 2]>::from(a), [0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskstore_epi64`]
/// * **Assembly:** `vpmaskmovq m128, xmm, xmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i64_m128i(addr: &mut m128i, mask: m128i, a: m128i) {
  unsafe { _mm_maskstore_epi64(addr as *mut m128i as *mut i64, mask.0, a.0) };
}

/// Shift `u32` values to the left by `count` bits.
///
/// * Each `u32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let count = m128i::from([5, 6, 7, 8]);
/// let out: [u32; 4] = shl_each_u32_m128i(a, count).into();
/// assert_eq!(out, [1 << 5, 2 << 6, 3 << 7, 4 << 8]);
/// ```
/// * **Intrinsic:** [`_mm_sllv_epi32`]
/// * **Assembly:** `vpsllvd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_each_u32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sllv_epi32(a.0, count.0) })
}

/// Shift `u64` values to the left by `count` bits.
///
/// * Each `u64` lane in `a` is shifted by the same indexed `u64` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 2]);
/// let count = m128i::from([3_u64, 4]);
/// let out: [u64; 2] = shl_each_u64_m128i(a, count).into();
/// assert_eq!(out, [1_u64 << 3, 2 << 4]);
/// ```
/// * **Intrinsic:** [`_mm_sllv_epi64`]
/// * **Assembly:** `vpsllvq xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_each_u64_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sllv_epi64(a.0, count.0) })
}

/// Shift `i32` values to the right by `count` bits.
///
/// * Each `i32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100, 110, 120, -130]);
/// let count = m128i::from([1, 2, 3, 4]);
/// let out: [i32; 4] = shr_each_i32_m128i(a, count).into();
/// assert_eq!(out, [100 >> 1, 110 >> 2, 120 >> 3, (-130) >> 4]);
/// ```
/// * **Intrinsic:** [`_mm_srav_epi32`]
/// * **Assembly:** `vpsravd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_i32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srav_epi32(a.0, count.0) })
}

/// Shift `u32` values to the left by `count` bits.
///
/// * Each `u32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100, 110, 120, 130]);
/// let count = m128i::from([1, 2, 3, 4]);
/// let out: [u32; 4] = shr_each_u32_m128i(a, count).into();
/// assert_eq!(out, [100 >> 1, 110 >> 2, 120 >> 3, 130 >> 4]);
/// ```
/// * **Intrinsic:** [`_mm_srlv_epi32`]
/// * **Assembly:** `vpsrlvd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_u32_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srlv_epi32(a.0, count.0) })
}

/// Shift `u64` values to the left by `count` bits.
///
/// * Each `u64` lane in `a` is shifted by the same indexed `u64` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100_u64, 110]);
/// let count = m128i::from([1_u64, 2]);
/// let out: [u64; 2] = shr_each_u64_m128i(a, count).into();
/// assert_eq!(out, [100_u64 >> 1, 110 >> 2]);
/// ```
/// * **Intrinsic:** [`_mm_srlv_epi64`]
/// * **Assembly:** `vpsrlvq xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_u64_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srlv_epi64(a.0, count.0) })
}

/// Absolute value of `i8` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i8; 32]);
/// let b: [i8; 32] = abs_i8_m256i(a).into();
/// assert_eq!(b, [7_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi8`]
/// * **Assembly:** `vpabsb ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i8_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi8(a.0) })
}

/// Absolute value of `i16` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i16; 16]);
/// let b: [i16; 16] = abs_i16_m256i(a).into();
/// assert_eq!(b, [7_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi16`]
/// * **Assembly:** `vpabsw ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i16_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi16(a.0) })
}

/// Absolute value of `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i32; 8]);
/// let b: [i32; 8] = abs_i32_m256i(a).into();
/// assert_eq!(b, [7_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi32`]
/// * **Assembly:** `vpabsd ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i32_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi32(a.0) })
}

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([10_i8; 32]);
/// let c: [i8; 32] = add_i8_m256i(a, b).into();
/// assert_eq!(c, [15_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi8`]
/// * **Assembly:** `vpaddb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([10_i16; 16]);
/// let c: [i16; 16] = add_i16_m256i(a, b).into();
/// assert_eq!(c, [15_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi16`]
/// * **Assembly:** `vpaddw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi16(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([10_i32; 8]);
/// let c: [i32; 8] = add_i32_m256i(a, b).into();
/// assert_eq!(c, [15_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi32`]
/// * **Assembly:** `vpaddd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi32(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i64; 4]);
/// let b = m256i::from([10_i64; 4]);
/// let c: [i64; 4] = add_i64_m256i(a, b).into();
/// assert_eq!(c, [15_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi64`]
/// * **Assembly:** `vpaddq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi64(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([126_i8; 32]);
/// let b = m256i::from([125_i8; 32]);
/// let c: [i8; 32] = add_saturating_i8_m256i(a, b).into();
/// assert_eq!(c, [127_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epi8`]
/// * **Assembly:** `vpaddsb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epi8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([32700_i16; 16]);
/// let b = m256i::from([32000_i16; 16]);
/// let c: [i16; 16] = add_saturating_i16_m256i(a, b).into();
/// assert_eq!(c, [32767_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epi16`]
/// * **Assembly:** `vpaddsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epi16(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([126_u8; 32]);
/// let b = m256i::from([125_u8; 32]);
/// let c: [u8; 32] = add_saturating_u8_m256i(a, b).into();
/// assert_eq!(c, [251_u8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epu8`]
/// * **Assembly:** `vpaddusb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epu8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([32700_u16; 16]);
/// let b = m256i::from([32000_u16; 16]);
/// let c: [u16; 16] = add_saturating_u16_m256i(a, b).into();
/// assert_eq!(c, [64700_u16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epu16`]
/// * **Assembly:** `vpaddusw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epu16(a.0, b.0) })
}

/// Works like [`combined_byte_shr_imm_m128i`], but twice as wide.
///
/// The low half of the bytes and high half of the bytes are both processed
/// separately.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([12_i8; 32]);
/// // `a` bytes come in to the _high_ indexes because these are LE bytes.
/// // Also note that the three 5 values at the low half and high half.
/// let c: [i8; 32] = combined_byte_shr_imm_m256i::<3>(a, b).into();
/// assert_eq!(
///   c,
///   [
///     12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 5, 5, 5, 12, 12, 12, 12, 12, 12, 12, 12,
///     12, 12, 12, 12, 12, 5, 5, 5_i8
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_alignr_epi8`]
/// * **Assembly:** `vpalignr ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn combined_byte_shr_imm_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_alignr_epi8(a.0, b.0, IMM) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = bitand_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 0, 0, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_and_si256`]
/// * **Assembly:** `vpand ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn bitand_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_and_si256(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = bitandnot_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_andnot_si256`]
/// * **Assembly:** `vpandn ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn bitandnot_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_andnot_si256(a.0, b.0) })
}

/// Average `u8` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([100_u8; 32]);
/// let b = m256i::from([120_u8; 32]);
/// let c: [u8; 32] = average_u8_m256i(a, b).into();
/// assert_eq!(c, [110_u8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_avg_epu8`]
/// * **Assembly:** `vpavgb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn average_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_avg_epu8(a.0, b.0) })
}

/// Average `u16` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([100_u16; 16]);
/// let b = m256i::from([120_u16; 16]);
/// let c: [u16; 16] = average_u16_m256i(a, b).into();
/// assert_eq!(c, [110_u16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_avg_epu16`]
/// * **Assembly:** `vpavgw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn average_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_avg_epu16(a.0, b.0) })
}

/// Blends the `i16` lanes according to the immediate value.
///
/// * The low 8 lanes and high 8 lanes both use the same immediate.
/// * Each bit in `0..=7` should be set for `$b` and unset for `$a` within that
///   half of the `i16` values.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([10_i16; 16]);
/// //
/// let c: [i16; 16] = blend_imm_i16_m256i::<0b11001000>(a, b).into();
/// assert_eq!(c, [5_i16, 5, 5, 10, 5, 5, 10, 10, 5, 5, 5, 10, 5, 5, 10, 10]);
/// ```
/// * **Intrinsic:** [`_mm256_blend_epi16`]
/// * **Assembly:** `vpblendw ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn blend_imm_i16_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_blend_epi16(a.0, b.0, IMM) })
}

/// Blends the `i32` lanes according to the immediate value.
///
/// * Each bit in `0..=7` should be set for `$b` and unset for `$a`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([10_i32; 8]);
/// //
/// let c: [i32; 8] = blend_imm_i32_m256i::<0b11001000>(a, b).into();
/// assert_eq!(c, [5, 5, 5, 10, 5, 5, 10, 10]);
/// ```
/// * **Intrinsic:** [`_mm256_blend_epi32`]
/// * **Assembly:** `vpblendd ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn blend_imm_i32_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_blend_epi32(a.0, b.0, IMM) })
}

/// Blend `i8` lanes according to a runtime varying mask.
///
/// * Mask lanes should be non-negative for `a` and negative for `b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([10_i8; 32]);
/// let mask = m256i::from([
///   0_i8, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0,
///   -1, -1, -1, 0, 0,
/// ]);
/// let c: [i8; 32] = blend_varying_i8_m256i(a, b, mask).into();
/// assert_eq!(
///   c,
///   [
///     5, 5, 5, 10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5, 5,
///     10, 10, 10, 5, 5
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_blendv_epi8`]
/// * **Assembly:** `vpavgw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn blend_varying_i8_m256i(a: m256i, b: m256i, mask: m256i) -> m256i {
  m256i(unsafe { _mm256_blendv_epi8(a.0, b.0, mask.0) })
}

/// Sets the lowest `i8` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i8 as i128);
/// let b: [i8; 32] = set_splat_i8_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastb_epi8`]
/// * **Assembly:** `vpbroadcastb ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i8_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastb_epi8(a.0) })
}

/// Sets the lowest `i16` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i16 as i128);
/// let b: [i16; 16] = set_splat_i16_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastw_epi16`]
/// * **Assembly:** `vpbroadcastw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i16_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastw_epi16(a.0) })
}

/// Sets the lowest `i32` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i32 as i128);
/// let b: [i32; 8] = set_splat_i32_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastd_epi32`]
/// * **Assembly:** `vpbroadcastd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i32_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastd_epi32(a.0) })
}

/// Sets the lowest `i64` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i64 as i128);
/// let b: [i64; 4] = set_splat_i64_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastq_epi64`]
/// * **Assembly:** `vpbroadcastq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i64_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastq_epi64(a.0) })
}

/// Sets the lowest lane of an `m128d` as all lanes of an `m256d`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128d_s(5.0);
/// let b = set_splat_m128d_s_m256d(a).to_array();
/// assert_eq!(b, [5.0; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastsd_pd`]
/// * **Assembly:** `vbroadcastsd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_m128d_s_m256d(a: m128d) -> m256d {
  m256d(unsafe { _mm256_broadcastsd_pd(a.0) })
}

/// Sets the lowest lane of an `m128` as all lanes of an `m256`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_s(5.0);
/// let b = set_splat_m128_s_m256(a).to_array();
/// assert_eq!(b, [5.0; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastss_ps`]
/// * **Assembly:** `vbroadcastss ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_m128_s_m256(a: m128) -> m256 {
  m256(unsafe { _mm256_broadcastss_ps(a.0) })
}

/// Shifts each `u128` lane left by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0x0000000B_0000000A_0000000F_11111111_u128; 2]);
/// //
/// let b: [u128; 2] = byte_shl_imm_u128_m256i::<1>(a).into();
/// assert_eq!(b, [0x00000B00_00000A00_00000F11_11111100_u128; 2]);
/// ```
/// * **Intrinsic:** [`_mm256_bslli_epi128`]
/// * **Assembly:** `vpslldq ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn byte_shl_imm_u128_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_bslli_epi128(a.0, IMM) })
}

/// Shifts each `u128` lane right by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0x0000000B_0000000A_0000000F_11111111_u128; 2]);
/// //
/// let b: [u128; 2] = byte_shr_imm_u128_m256i::<1>(a).into();
/// assert_eq!(b, [0x00000000_0B000000_0A000000_0F111111; 2]);
/// ```
/// * **Intrinsic:** [`_mm256_bsrli_epi128`]
/// * **Assembly:** `vpsrldq ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn byte_shr_imm_u128_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_bsrli_epi128(a.0, IMM) })
}

/// Compare `i8` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i8; 32]>::from(cmp_eq_mask_i8_m256i(m256i::from([1_i8; 32]), m256i::from([1_i8; 32]))),
///   [-1_i8; 32]
/// );
/// assert_eq!(
///   <[i8; 32]>::from(cmp_eq_mask_i8_m256i(m256i::from([5_i8; 32]), m256i::from([6_i8; 32]))),
///   [0_i8; 32]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi8`]
/// * **Assembly:** `vpcmpeqb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi8(a.0, b.0) })
}

/// Compare `i16` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i16; 16]>::from(cmp_eq_mask_i16_m256i(m256i::from([1_i16; 16]), m256i::from([1_i16; 16]))),
///   [-1_i16; 16]
/// );
/// assert_eq!(
///   <[i16; 16]>::from(cmp_eq_mask_i16_m256i(m256i::from([5_i16; 16]), m256i::from([6_i16; 16]))),
///   [0_i16; 16]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi16`]
/// * **Assembly:** `vpcmpeqw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi16(a.0, b.0) })
}

/// Compare `i32` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i32; 8]>::from(cmp_eq_mask_i32_m256i(m256i::from([1_i32; 8]), m256i::from([1_i32; 8]))),
///   [-1_i32; 8]
/// );
/// assert_eq!(
///   <[i32; 8]>::from(cmp_eq_mask_i32_m256i(m256i::from([5_i32; 8]), m256i::from([6_i32; 8]))),
///   [0_i32; 8]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi32`]
/// * **Assembly:** `vpcmpeqd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi32(a.0, b.0) })
}

/// Compare `i64` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i64; 4]>::from(cmp_eq_mask_i64_m256i(m256i::from([1_i64; 4]), m256i::from([1_i64; 4]))),
///   [-1_i64; 4]
/// );
/// assert_eq!(
///   <[i64; 4]>::from(cmp_eq_mask_i64_m256i(m256i::from([5_i64; 4]), m256i::from([6_i64; 4]))),
///   [0_i64; 4]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi64`]
/// * **Assembly:** `vpcmpeqq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi64(a.0, b.0) })
}

/// Compare `i8` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i8; 32]>::from(cmp_gt_mask_i8_m256i(m256i::from([1_i8; 32]), m256i::from([0_i8; 32]))),
///   [-1_i8; 32]
/// );
/// assert_eq!(
///   <[i8; 32]>::from(cmp_gt_mask_i8_m256i(m256i::from([5_i8; 32]), m256i::from([5_i8; 32]))),
///   [0_i8; 32]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi8`]
/// * **Assembly:** `vpcmpgtb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi8(a.0, b.0) })
}

/// Compare `i16` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i16; 16]>::from(cmp_gt_mask_i16_m256i(m256i::from([1_i16; 16]), m256i::from([0_i16; 16]))),
///   [-1_i16; 16]
/// );
/// assert_eq!(
///   <[i16; 16]>::from(cmp_gt_mask_i16_m256i(m256i::from([5_i16; 16]), m256i::from([5_i16; 16]))),
///   [0_i16; 16]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi16`]
/// * **Assembly:** `vpcmpgtw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi16(a.0, b.0) })
}

/// Compare `i32` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i32; 8]>::from(cmp_gt_mask_i32_m256i(m256i::from([1_i32; 8]), m256i::from([0_i32; 8]))),
///   [-1_i32; 8]
/// );
/// assert_eq!(
///   <[i32; 8]>::from(cmp_gt_mask_i32_m256i(m256i::from([5_i32; 8]), m256i::from([5_i32; 8]))),
///   [0_i32; 8]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi32`]
/// * **Assembly:** `vpcmpgtd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi32(a.0, b.0) })
}

/// Compare `i64` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i64; 4]>::from(cmp_gt_mask_i64_m256i(m256i::from([1_i64; 4]), m256i::from([0_i64; 4]))),
///   [-1_i64; 4]
/// );
/// assert_eq!(
///   <[i64; 4]>::from(cmp_gt_mask_i64_m256i(m256i::from([5_i64; 4]), m256i::from([5_i64; 4]))),
///   [0_i64; 4]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi64`]
/// * **Assembly:** `vpcmpgtq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi64(a.0, b.0) })
}

/// Convert `i16` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i16; 8]);
/// let b: [i32; 8] = convert_to_i32_m256i_from_i16_m128i(a).into();
/// assert_eq!(b, [-5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi16_epi32`]
/// * **Assembly:** `vpmovsxwd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i32_m256i_from_i16_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi16_epi32(a.0) })
}

/// Convert `i16` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i16; 8]);
/// let b: [i64; 4] = convert_to_i64_m256i_from_lower4_i16_m128i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi16_epi64`]
/// * **Assembly:** `vpmovsxwq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i64_m256i_from_lower4_i16_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi16_epi64(a.0) })
}

/// Convert `i32` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i32; 4]);
/// let b: [i64; 4] = convert_to_i64_m256i_from_i32_m128i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi32_epi64`]
/// * **Assembly:** `vpmovsxdq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i64_m256i_from_i32_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi32_epi64(a.0) })
}

/// Convert `i8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i16; 16] = convert_to_i16_m256i_from_i8_m128i(a).into();
/// assert_eq!(b, [-5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi16`]
/// * **Assembly:** `vpmovsxbw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i16_m256i_from_i8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi16(a.0) })
}

/// Convert the lower 8 `i8` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i32; 8] = convert_to_i32_m256i_from_lower8_i8_m128i(a).into();
/// assert_eq!(b, [-5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi32`]
/// * **Assembly:** `vpmovsxbd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i32_m256i_from_lower8_i8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi32(a.0) })
}

/// Convert the lower 4 `i8` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i64; 4] = convert_to_i64_m256i_from_lower4_i8_m128i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi64`]
/// * **Assembly:** `vpmovsxbq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i64_m256i_from_lower4_i8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi64(a.0) })
}

/// Convert `u16` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u16; 8]);
/// let b: [i32; 8] = convert_to_i32_m256i_from_u16_m128i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu16_epi32`]
/// * **Assembly:** `vpmovzxwd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i32_m256i_from_u16_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu16_epi32(a.0) })
}

/// Convert `u16` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u16; 8]);
/// let b: [i64; 4] = convert_to_i64_m256i_from_lower4_u16_m128i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu16_epi64`]
/// * **Assembly:** `vpmovzxwq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i64_m256i_from_lower4_u16_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu16_epi64(a.0) })
}

/// Convert `u32` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u32; 4]);
/// let b: [i64; 4] = convert_to_i64_m256i_from_u32_m128i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu32_epi64`]
/// * **Assembly:** `vpmovzxdq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i64_m256i_from_u32_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu32_epi64(a.0) })
}

/// Convert `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i16; 16] = convert_to_i16_m256i_from_u8_m128i(a).into();
/// assert_eq!(b, [5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi16`]
/// * **Assembly:** `vpmovzxbw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i16_m256i_from_u8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi16(a.0) })
}

/// Convert lower 8 `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i32; 8] = convert_to_i16_m256i_from_lower8_u8_m128i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi32`]
/// * **Assembly:** `vpmovzxbd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i16_m256i_from_lower8_u8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi32(a.0) })
}

/// Convert lower 4 `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i64; 4] = convert_to_i16_m256i_from_lower4_u8_m128i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi64`]
/// * **Assembly:** `vpmovzxbq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn convert_to_i16_m256i_from_lower4_u8_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi64(a.0) })
}

/// Gets an `i16` value out of an `m256i`, returns as `i32`.
///
/// The lane to get must be a constant in the range `0..16`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0xA_i16, 0xB, 0xC, 0xD, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// //
/// assert_eq!(extract_i16_as_i32_m256i::<0>(a), 0xA);
/// assert_eq!(extract_i16_as_i32_m256i::<1>(a), 0xB);
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi16`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn extract_i16_as_i32_m256i<const LANE: i32>(a: m256i) -> i32 {
  unsafe { _mm256_extract_epi16(a.0, LANE) }
}

/// Gets an `i8` value out of an `m256i`, returns as `i32`.
///
/// The lane to get must be a constant in the range `0..32`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0xA_i8, 0xB, 0xC, 0xD, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///   0, 0, 0, 0,
/// ]);
/// //
/// assert_eq!(extract_i8_as_i32_m256i::<0>(a), 0xA);
/// assert_eq!(extract_i8_as_i32_m256i::<1>(a), 0xB);
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi8`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn extract_i8_as_i32_m256i<const LANE: i32>(a: m256i) -> i32 {
  unsafe { _mm256_extract_epi8(a.0, LANE) }
}

/// Gets an `m128i` value out of an `m256i`.
///
/// The lane to get must be a constant 0 or 1.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u128, 6_u128]);
/// //
/// assert_eq!(extract_m128i_m256i::<0>(a), m128i::from(5_u128));
/// assert_eq!(extract_m128i_m256i::<1>(a), m128i::from(6_u128));
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi8`]
/// * **Assembly:** `vextracti128 xmm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn extract_m128i_m256i<const LANE: i32>(a: m256i) -> m128i {
  m128i(unsafe { _mm256_extracti128_si256(a.0, LANE) })
}

/// Horizontal `a + b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([6_i16; 16]);
/// let c: [i16; 16] = add_horizontal_i16_m256i(a, b).into();
/// assert_eq!(c, [10_i16, 10, 10, 10, 12, 12, 12, 12, 10, 10, 10, 10, 12, 12, 12, 12]);
/// ```
/// * **Intrinsic:** [`_mm256_hadd_epi16`]
/// * **Assembly:** `vphaddw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadd_epi16(a.0, b.0) })
}

/// Horizontal saturating `a + b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([i16::MAX; 16]);
/// let b = m256i::from([i16::MIN; 16]);
/// let c: [i16; 16] = add_horizontal_saturating_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hadds_epi16`]
/// * **Assembly:** `vphaddsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
#[rustfmt::skip]
pub fn add_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadds_epi16(a.0, b.0) })
}

/// Horizontal `a + b` with lanes as `i32`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([6_i32; 8]);
/// let c: [i32; 8] = add_horizontal_i32_m256i(a, b).into();
/// assert_eq!(c, [10, 10, 12, 12, 10, 10, 12, 12]);
/// ```
/// * **Intrinsic:** [`_mm256_hadd_epi32`]
/// * **Assembly:** `vphaddd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn add_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadd_epi32(a.0, b.0) })
}

/// Horizontal `a - b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_i16, 13000, -2, -8, 0, 1, 2, 3, 8, 7, 6, 5, 234, 654, 123, 978]);
/// let c: [i16; 16] = add_horizontal_i16_m256i(a, b).into();
/// assert_eq!(c, [11_i16, 7, 7, 1, 25000, -10, 1, 5, 1, 77, 15, 11, 15, 11, 888, 1101]);
/// ```
/// * **Intrinsic:** [`_mm256_hsub_epi16`]
/// * **Assembly:** `vphsubw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsub_epi16(a.0, b.0) })
}

/// Horizontal `a - b` with lanes as `i32`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5, 6, 2, 5, 4, 3, 1, 0]);
/// let b = m256i::from([-12, 13, 56, 21, 8, 7, 6, 5]);
/// let c: [i32; 8] = sub_horizontal_i32_m256i(a, b).into();
/// assert_eq!(c, [-1, -3, -25, 35, 1, 1, 1, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_hsub_epi32`]
/// * **Assembly:** `vphsubd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsub_epi32(a.0, b.0) })
}

/// Horizontal saturating `a - b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([i16::MAX; 16]);
/// let b = m256i::from([i16::MIN; 16]);
/// let c: [i16; 16] = add_horizontal_saturating_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hsubs_epi16`]
/// * **Assembly:** `vphsubsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
#[rustfmt::skip]
pub fn sub_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsubs_epi16(a.0, b.0) })
}

/// Multiply `i16` lanes producing `i32` values, horizontal add pairs of `i32`
/// values to produce the final output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i16, 2, 3, 4, -1, -2, -3, -4, 12, 13, -14, -15, 100, 200, 300, -400]);
/// let b = m256i::from([5_i16, 6, 7, 8, -15, -26, -37, 48, 50, 60, 70, -80, 90, 100, 12, -80]);
/// let c: [i32; 8] = mul_i16_horizontal_add_m256i(a, b).into();
/// assert_eq!(c, [17, 53, 67, -81, 1380, 220, 29000, 35600]);
/// ```
/// * **Intrinsic:** [`_mm256_madd_epi16`]
/// * **Assembly:** `vpmaddwd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i16_horizontal_add_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_madd_epi16(a.0, b.0) })
}

/// This is dumb and weird.
///
/// * Vertically multiplies each `u8` lane from `a` with an `i8` lane from `b`,
///   producing an `i16` intermediate value.
/// * These intermediate `i16` values are horizontally added with saturation.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   255_u8, 255, 0, 0, 255, 255, 1, 1, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
///   18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
/// ]);
/// let b = m256i::from([
///   127_i8, 127, 0, 0, -127, -127, 1, 1, 24, 25, 26, 27, 28, 29, 30, 31, 16,
///   17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
/// ]);
/// let c: [i16; 16] = mul_u8i8_add_horizontal_saturating_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [i16::MAX, 0, i16::MIN, 2, 417, 557, 713, 885,
///   545, 685, 841, 1013, 1201, 1405, 1625, 1861]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_maddubs_epi16`]
/// * **Assembly:** `vpmaddubsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
#[rustfmt::skip]
pub fn mul_u8i8_add_horizontal_saturating_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_maddubs_epi16(a.0, b.0) })
}

/// Loads the reference given and zeroes any `i32` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = load_masked_i32_m256i(&a, m256i::from([-1_i32, 0, 0, -1, -1, -1, 0, 0]));
/// assert_eq!(<[i32; 8]>::from(b), [5, 0, 0, 5, 5, 5, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_maskload_epi32`]
/// * **Assembly:** `vpmaskmovd ymm, ymm, m256`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i32_m256i(a: &m256i, mask: m256i) -> m256i {
  m256i(unsafe { _mm256_maskload_epi32(a as *const m256i as *const i32, mask.0) })
}

/// Loads the reference given and zeroes any `i64` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i64; 4]);
/// let b = load_masked_i64_m256i(&a, m256i::from([0_i64, -1, -1, 0]));
/// assert_eq!(<[i64; 4]>::from(b), [0_i64, 5, 5, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_maskload_epi64`]
/// * **Assembly:** `vpmaskmovq ymm, ymm, m256`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i64_m256i(a: &m256i, mask: m256i) -> m256i {
  m256i(unsafe { _mm256_maskload_epi64(a as *const m256i as *const i64, mask.0) })
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m256i::default();
/// store_masked_i32_m256i(
///   &mut a,
///   m256i::from([-1_i32, 0, 0, -1, -1, -1, 0, 0]),
///   m256i::from([5_i32; 8]),
/// );
/// assert_eq!(<[i32; 8]>::from(a), [5, 0, 0, 5, 5, 5, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_maskstore_epi32`]
/// * **Assembly:** `vpmaskmovd m256, ymm, ymm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i32_m256i(addr: &mut m256i, mask: m256i, a: m256i) {
  unsafe { _mm256_maskstore_epi32(addr as *mut m256i as *mut i32, mask.0, a.0) };
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m256i::default();
/// store_masked_i64_m256i(&mut a, m256i::from([0_i64, -1, -1, 0]), m256i::from([5_i64; 4]));
/// assert_eq!(<[i64; 4]>::from(a), [0, 5, 5, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_maskstore_epi64`]
/// * **Assembly:** `vpmaskmovq m256, ymm, ymm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i64_m256i(addr: &mut m256i, mask: m256i, a: m256i) {
  unsafe { _mm256_maskstore_epi64(addr as *mut m256i as *mut i64, mask.0, a.0) };
}

/// Inserts an `m128i` to an `m256i` at the high or low position.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32; 8]);
/// let b: [i32; 8] = insert_m128i_to_m256i::<1>(a, m128i::from([1, 2, 3, 4])).into();
/// assert_eq!(b, [0, 0, 0, 0, 1, 2, 3, 4]);
/// ```
/// * **Intrinsic:** [`_mm256_inserti128_si256`]
/// * **Assembly:** `vinserti128 ymm, ymm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn insert_m128i_to_m256i<const LANE: i32>(a: m256i, b: m128i) -> m256i {
  m256i(unsafe { _mm256_inserti128_si256(a.0, b.0, LANE) })
}

/// Lanewise `max(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127, 1, 3, 5, 7, 2, 3, 5, 12, 13, 16, 27,
///   28, 29, 30, 31, 32,
/// ]);
/// let b = m256i::from([
///   0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127, 0, -1, 3, 4, 5, 1, -2, -4,
///   -8, 12, 13, 14, 29, 30, -31, -32,
/// ]);
/// let c: [i8; 32] = max_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     0, 11, 2, 3, 4, 15, 6, 7, 8, 19, 10, 21, 22, 13, 24, 127, 1, 3, 5, 7, 5, 3, 5, 12, 13, 16,
///     27, 28, 29, 30, 31, 32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_max_epi8`]
/// * **Assembly:** `vpmaxsb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epi8(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m256i::from([0_i16, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, -24, 25]);
/// let c: [i16; 16] = max_i16_m256i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 3, 4, 15, 6, 7, 8, 19, 10, 21, 22, 13, 14, 127]);
/// ```
/// * **Intrinsic:** [`_mm256_max_epi16`]
/// * **Assembly:** `vpmaxsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epi16(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 11, 2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = max_i32_m256i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 3, 4, 15, 6, 7]);
/// ```
/// * **Intrinsic:** [`_mm256_max_epi32`]
/// * **Assembly:** `vpmaxsd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epi32(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127, 1, 3, 5, 7, 2, 3, 5, 12, 13, 16, 27,
///   28, 29, 30, 31, 32,
/// ]);
/// let b = m256i::from([
///   0_u8, 255, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 0, 1, 3, 4, 5, 1, 2, 4, 8, 12,
///   13, 14, 29, 30, 31, 32,
/// ]);
/// let c: [u8; 32] = max_u8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     0, 255, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 1, 3, 5, 7, 5, 3, 5, 12, 13, 16,
///     27, 28, 29, 30, 31, 32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_max_epu8`]
/// * **Assembly:** `vpmaxub ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epu8(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m256i::from([0_u16, 65535, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 25]);
/// let c: [u16; 16] = max_u16_m256i(a, b).into();
/// assert_eq!(c, [0, 65535, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127]);
/// ```
/// * **Intrinsic:** [`_mm256_max_epu16`]
/// * **Assembly:** `vpmaxuw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epu16(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_u32, 11, 2, 13, 4, 15, 6, 17]);
/// let c: [u32; 8] = max_u32_m256i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 13, 4, 15, 6, 17]);
/// ```
/// * **Intrinsic:** [`_mm256_max_epu32`]
/// * **Assembly:** `vpmaxud ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn max_u32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_max_epu32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127, 1, 3, 5, 7, 2, 3, 5, 12, 13, 16, 27,
///   28, 29, 30, 31, 32,
/// ]);
/// let b = m256i::from([
///   0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127, 0, -1, 3, 4, 5, 1, -2, -4,
///   -8, 12, 13, 14, 29, 30, -31, -32,
/// ]);
/// let c: [i8; 32] = min_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     0, 1, 2, -13, 4, 5, 6, -17, -8, 9, -20, 11, 12, -23, 14, 127, 0, -1, 3, 4, 2, 1, -2, -4, -8,
///     12, 13, 14, 29, 30, -31, -32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_min_epi8`]
/// * **Assembly:** `vpminsb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epi8(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m256i::from([0_i16, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, -24, 25]);
/// let c: [i16; 16] = min_i16_m256i(a, b).into();
/// assert_eq!(c, [0, 1, 2, -13, 4, 5, 6, -17, -8, 9, -20, 11, 12, -23, -24, 25]);
/// ```
/// * **Intrinsic:** [`_mm256_min_epi16`]
/// * **Assembly:** `vpminsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epi16(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 11, 2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = min_i32_m256i(a, b).into();
/// assert_eq!(c, [0, 1, 2, -13, 4, 5, 6, -17]);
/// ```
/// * **Intrinsic:** [`_mm256_min_epi32`]
/// * **Assembly:** `vpminsd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epi32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127, 1, 3, 5, 7, 2, 3, 5, 12, 13, 16, 27,
///   28, 29, 30, 31, 32,
/// ]);
/// let b = m256i::from([
///   0_u8, 255, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 0, 1, 3, 4, 5, 1, 2, 4, 8, 12,
///   13, 14, 29, 30, 31, 32,
/// ]);
/// let c: [u8; 32] = min_u8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127, 0, 1, 3, 4, 2, 1, 2, 4, 8, 12, 13, 14,
///     29, 30, 31, 32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_min_epu8`]
/// * **Assembly:** `vpminub ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epu8(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m256i::from([0_u16, 65535, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 25]);
/// let c: [u16; 16] = min_u16_m256i(a, b).into();
/// assert_eq!(c, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 25]);
/// ```
/// * **Intrinsic:** [`_mm256_min_epu16`]
/// * **Assembly:** `vpminuw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epu16(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_u32, 11, 2, 13, 4, 15, 6, 17]);
/// let c: [u32; 8] = min_u32_m256i(a, b).into();
/// assert_eq!(c, [0, 1, 2, 3, 4, 5, 6, 7]);
/// ```
/// * **Intrinsic:** [`_mm256_min_epu32`]
/// * **Assembly:** `vpminud ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn min_u32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_min_epu32(a.0, b.0) })
}

/// Create an `i32` mask of each sign bit in the `i8` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127, 0, -1, 3, 4, 5, 1, -2, -4,
///   -8, 12, 13, 14, 29, 30, -31, 32,
/// ]);
/// assert_eq!(0b01000001110000100010010110001000, move_mask_i8_m256i(a));
/// ```
/// * **Intrinsic:** [`_mm256_movemask_epi8`]
/// * **Assembly:** `vpmovmskb r32, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn move_mask_i8_m256i(a: m256i) -> i32 {
  unsafe { _mm256_movemask_epi8(a.0) }
}

/// Computes eight `u16` "sum of absolute difference" values according to the
/// bytes selected.
///
/// * This essentially works like two [`multi_packed_sum_abs_diff_u8_m128i`]
///   uses happening at once, the "low" portion works on the lower 128 bits, and
///   the "high" portion works on the upper 128 bits.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u8; 32]);
/// let b = m256i::from([7_u8; 32]);
/// //
/// let c: [u16; 16] = multi_packed_sum_abs_diff_u8_m256i::<0b101000>(a, b).into();
/// assert_eq!(c, [8_u16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_mpsadbw_epu8`]
/// * **Assembly:** ``
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn multi_packed_sum_abs_diff_u8_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mpsadbw_epu8(a.0, b.0, IMM) })
}

/// Multiply the lower `i32` within each `i64` lane, `i64` output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i64, 2, 3, 4]);
/// let b = m256i::from([5_i64, 6, 7, -8]);
/// let c: [i64; 4] = mul_i64_low_bits_m256i(a, b).into();
/// assert_eq!(c, [5_i64, 12, 21, -32]);
/// ```
/// * **Intrinsic:** [`_mm256_mul_epi32`]
/// * **Assembly:** `vpmuldq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i64_low_bits_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mul_epi32(a.0, b.0) })
}

/// Multiply the lower `u32` within each `u64` lane, `u64` output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_u64, 2, 3, 4]);
/// let b = m256i::from([5_u64, 6, 7, 8]);
/// let c: [u64; 4] = mul_u64_low_bits_m256i(a, b).into();
/// assert_eq!(c, [5_u64, 12, 21, 32]);
/// ```
/// * **Intrinsic:** [`_mm256_mul_epu32`]
/// * **Assembly:** `vpmuludq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_u64_low_bits_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mul_epu32(a.0, b.0) })
}

/// Multiply the `i16` lanes and keep the high half of each 32-bit output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_i16, 13000, -2, -8, 0, 1, 2, 3, 8, 7, 6, 5, 234, 654, 123, 978]);
/// let c: [i16; 16] = mul_i16_keep_high_m256i(a, b).into();
/// assert_eq!(c, [0_i16, 1, -1, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_mulhi_epi16`]
/// * **Assembly:** `vpmulhw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i16_keep_high_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mulhi_epi16(a.0, b.0) })
}

/// Multiply the `u16` lanes and keep the high half of each 32-bit output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u16, 6, 2, 5, 4, 3, 1, 0, 12000, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_u16, 13000, 2000, 800, 0, 1, 2, 3, 8, 7, 6, 5, 234, 654, 123, 978]);
/// let c: [u16; 16] = mul_u16_keep_high_m256i(a, b).into();
/// assert_eq!(c, [0_u16, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_mulhi_epu16`]
/// * **Assembly:** `vpmulhuw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_u16_keep_high_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mulhi_epu16(a.0, b.0) })
}

/// Multiply `i16` lanes into `i32` intermediates, keep the high 18 bits, round
/// by adding 1, right shift by 1.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_i16, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500,
/// ]);
/// let b = m256i::from([
///   800_i16, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200,
///   2300,
/// ]);
/// let c: [i16; 16] = mul_i16_scale_round_m256i(a, b).into();
/// assert_eq!(c, [0_i16, 3, 6, 10, 15, 20, 26, 32, 39, 47, 55, 64, 73, 83, 94, 105]);
/// ```
/// * **Intrinsic:** [`_mm256_mulhrs_epi16`]
/// * **Assembly:** `vpmulhrsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i16_scale_round_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mulhrs_epi16(a.0, b.0) })
}

/// Multiply the `i16` lanes and keep the low half of each 32-bit output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([-1_i16, 13000, -2, -8, 0, 1, 2, 3, 8, 7, 6, 5, 234, 654, 123, 978]);
/// let c: [i16; 16] = mul_i16_keep_low_m256i(a, b).into();
/// assert_eq!(c, [-5, 12464, -4, -40, 0, 3, 2, 0, -96, 91, 336, 105, 1872, 4578, 738, 4890]);
/// ```
/// * **Intrinsic:** [`_mm256_mullo_epi16`]
/// * **Assembly:** `vpmullw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i16_keep_low_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mullo_epi16(a.0, b.0) })
}

/// Multiply the `i32` lanes and keep the low half of each 64-bit output.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 11, 2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = mul_i32_keep_low_m256i(a, b).into();
/// assert_eq!(c, [0, 11, 4, -39, 16, 75, 36, -119]);
/// ```
/// * **Intrinsic:** [`_mm256_mullo_epi32`]
/// * **Assembly:** `vpmulld ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn mul_i32_keep_low_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_mullo_epi32(a.0, b.0) })
}

/// Bitwise `a | b`
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = bitor_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 1, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_or_si256`]
/// * **Assembly:** `vpor ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn bitor_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_or_si256(a.0, b.0) })
}

/// Saturating convert `i16` to `i8`, and pack the values.
///
/// * The values are packed 128 bits at a time: `a_low`, `b_low`, `a_high`,
///   `b_high`
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
/// let b = m256i::from([17_i16, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
/// let c: [i8; 32] = pack_i16_to_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     1_i8, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16, 25,
///     26, 27, 28, 29, 30, 31, 32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_packs_epi16`]
/// * **Assembly:** `vpacksswb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn pack_i16_to_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_packs_epi16(a.0, b.0) })
}

/// Saturating convert `i32` to `i16`, and pack the values.
///
/// * The values are packed 128 bits at a time: `a_low`, `b_low`, `a_high`,
///   `b_high`
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i32, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m256i::from([9_i32, 10, 11, 12, 13, 14, 15, 16]);
/// let c: [i16; 16] = pack_i32_to_i16_m256i(a, b).into();
/// assert_eq!(c, [1_i16, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]);
/// ```
/// * **Intrinsic:** [`_mm256_packs_epi32`]
/// * **Assembly:** `vpackssdw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn pack_i32_to_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_packs_epi32(a.0, b.0) })
}

/// Saturating convert `i16` to `u8`, and pack the values.
///
/// * The values are packed 128 bits at a time: `a_low`, `b_low`, `a_high`,
///   `b_high`
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
/// let b = m256i::from([17_i16, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]);
/// let c: [u8; 32] = pack_i16_to_u8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     1_u8, 2, 3, 4, 5, 6, 7, 8, 17, 18, 19, 20, 21, 22, 23, 24, 9, 10, 11, 12, 13, 14, 15, 16, 25,
///     26, 27, 28, 29, 30, 31, 32
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_packus_epi16`]
/// * **Assembly:** `vpackuswb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn pack_i16_to_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_packus_epi16(a.0, b.0) })
}

/// Saturating convert `i32` to `u16`, and pack the values.
///
/// * The values are packed 128 bits at a time: `a_low`, `b_low`, `a_high`,
///   `b_high`
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i32, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m256i::from([9_i32, 10, 11, 12, 13, 14, 15, 16]);
/// let c: [u16; 16] = pack_i32_to_u16_m256i(a, b).into();
/// assert_eq!(c, [1_u16, 2, 3, 4, 9, 10, 11, 12, 5, 6, 7, 8, 13, 14, 15, 16]);
/// ```
/// * **Intrinsic:** [`_mm256_packus_epi32`]
/// * **Assembly:** `vpackusdw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn pack_i32_to_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_packs_epi32(a.0, b.0) })
}

/// Shuffle 128 bits of integer data from `$a` and `$b` using an immediate
/// control value.
///
/// You can pass `A_Low`, `A_High`, `B_Low`, `B_High`, or `Zeroed`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
/// //
/// let c: [i32; 8] = shuffle_abi_i128z_all_m256i::<0b_1000_0010>(a, b).into();
/// assert_eq!(c, [9, 10, 11, 12, 0, 0, 0, 0]);
/// //
/// let c: [i32; 8] = shuffle_abi_i128z_all_m256i::<0b_0001_1000>(a, b).into();
/// assert_eq!(c, [0, 0, 0, 0, 5, 6, 7, 8]);
/// ```
/// * **Intrinsic:** [`_mm256_permute2x128_si256`]
/// * **Assembly:** `vperm2i128 ymm, ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_abi_i128z_all_m256i<const MASK: i32>(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_permute2x128_si256(a.0, b.0, MASK) })
}

/// Shuffle the `f64` lanes in `$a` using an immediate control value.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i64, 6, 7, 8]);
/// let b: [i64; 4] = shuffle_ai_i64_all_m256i::<0b00_01_10_11>(a).into();
/// assert_eq!(b, [8_i64, 7, 6, 5]);
/// ```
/// * **Intrinsic:** [`_mm256_permute4x64_epi64`]
/// * **Assembly:** `vpermq ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_ai_i64_all_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_permute4x64_epi64(a.0, IMM) })
}

/// Shuffle the `f64` lanes from `$a` using an immediate control value.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let b: [f64; 4] = shuffle_ai_f64_all_m256d::<0b00_01_10_11>(a).to_array();
/// assert_eq!(b, [8.0, 7.0, 6.0, 5.0]);
/// ```
/// * **Intrinsic:** [`_mm256_permute4x64_pd`]
/// * **Assembly:** `vpermpd ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_ai_f64_all_m256d<const IMM: i32>(a: m256d) -> m256d {
  m256d(unsafe { _mm256_permute4x64_pd(a.0, IMM) })
}

/// Shuffle `i32` lanes in `a` using `i32` values in `v`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([8, 9, 10, 11, 12, 13, 14, 15]);
/// let v = m256i::from([7, 6, 5, 5, 3, 2, 2, 0]);
/// let c: [i32; 8] = shuffle_av_i32_all_m256i(a, v).into();
/// assert_eq!(c, [15, 14, 13, 13, 11, 10, 10, 8]);
/// ```
/// * **Intrinsic:** [`_mm256_permutevar8x32_epi32`]
/// * **Assembly:** `vpermd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_av_i32_all_m256i(a: m256i, v: m256i) -> m256i {
  m256i(unsafe { _mm256_permutevar8x32_epi32(a.0, v.0) })
}

/// Shuffle `f32` lanes in `a` using `i32` values in `v`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]);
/// let v = m256i::from([7, 6, 5, 5, 3, 2, 2, 0]);
/// let c: [f32; 8] = shuffle_av_i32_all_m256(a, v).to_array();
/// assert_eq!(c, [15.0, 14.0, 13.0, 13.0, 11.0, 10.0, 10.0, 8.0]);
/// ```
/// * **Intrinsic:** [`_mm256_permutevar8x32_ps`]
/// * **Assembly:** `vpermps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_av_i32_all_m256(a: m256, v: m256i) -> m256 {
  m256(unsafe { _mm256_permutevar8x32_ps(a.0, v.0) })
}

/// Compute "sum of `u8` absolute differences".
///
/// * `u8` lanewise `abs(a - b)`, producing `u8` intermediate values.
/// * Sum the first eight and second eight values.
/// * Place into the low 16 bits of four `u64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0_u8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 0, 11, 2, 13, 4, 15, 6, 17, 8,
///   19, 20, 21, 22, 23, 24, 127,
/// ]);
/// let b = m256i::from([
///   20_u8, 110, 250, 103, 34, 105, 60, 217, 8, 19, 210, 201, 202, 203, 204, 127, 2, 3, 4, 5, 6, 7,
///   8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
/// ]);
/// let c: [u64; 4] = sum_of_u8_abs_diff_m256i(a, b).into();
/// assert_eq!(c, [831_u64, 910, 40, 160]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sum_of_u8_abs_diff_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sad_epu8(a.0, b.0) })
}

/// Shuffle the `i32` lanes in `a` using an immediate control value.
///
/// Each lane selection value picks only within that 128-bit half of the overall
/// register.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5, 6, 7, 8, 9, 10, 11, 12]);
/// let b: [i32; 8] = shuffle_ai_i32_half_m256i::<0b00_01_10_11>(a).into();
/// assert_eq!(b, [8, 7, 6, 5, 12, 11, 10, 9]);
/// ```
/// * **Intrinsic:** [`_mm256_shuffle_epi32`]
/// * **Assembly:** `vpshufd ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_ai_i32_half_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_shuffle_epi32(a.0, IMM) })
}

/// Shuffle `i8` lanes in `a` using `i8` values in `v`.
///
/// Each lane selection value picks only within that 128-bit half of the overall
/// register.
///
/// If a lane in `v` is negative, that output is zeroed.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   3_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 7, 11, 2, 13, 4, 15, 6, 17, 8,
///   19, 20, 21, 22, 23, 24, 127,
/// ]);
/// let b = m256i::from([
///   -1_i8, 1, 0, 2, 2, 3, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 12, 11, 10, 9,
///   8, 7, 6, 5, 4,
/// ]);
/// let c: [i8; 32] = shuffle_av_i8z_half_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     0, 11, 3, 2, 2, 13, 4, 15, 6, 6, 17, 8, 8, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 22, 21,
///     20, 19, 8, 17, 6, 15, 4
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_shuffle_epi8`]
/// * **Assembly:** `vpshufb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_av_i8z_half_m256i(a: m256i, v: m256i) -> m256i {
  m256i(unsafe { _mm256_shuffle_epi8(a.0, v.0) })
}

/// Shuffle the high `i16` lanes in `$a` using an immediate control value.
///
/// The lower 128 bits and upper 128 bits have this performed separately.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b: [i16; 16] = shuffle_ai_i16_h64half_m256i::<0b_00_01_10_11>(a).into();
/// assert_eq!(b, [0, 1, 2, 3, 7, 6, 5, 4, 8, 9, 10, 11, 15, 14, 13, 12]);
/// ```
/// * **Intrinsic:** [`_mm256_shufflehi_epi16`]
/// * **Assembly:** `vpshufhw ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_ai_i16_h64half_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_shufflehi_epi16(a.0, IMM) })
}

/// Shuffle the low `i16` lanes in `$a` using an immediate control value.
///
/// The lower 128 bits and upper 128 bits have this performed separately.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b: [i16; 16] = shuffle_ai_i16_l64half_m256i::<0b00_01_10_11>(a).into();
/// assert_eq!(b, [3, 2, 1, 0, 4, 5, 6, 7, 11, 10, 9, 8, 12, 13, 14, 15]);
/// ```
/// * **Intrinsic:** [`_mm256_shufflelo_epi16`]
/// * **Assembly:** `vpshuflw ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shuffle_ai_i16_l64half_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_shufflelo_epi16(a.0, IMM) })
}

/// Lanewise `a * signum(b)` with lanes as `i8`
///
/// * If `b` is positive, the output is `a`.
/// * If `b` is zero, the output is 0.
/// * If `b` is negative, the output is `-a`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   3_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 7, 11, 2, 13, 4, 15, 6, 17, 8,
///   19, 20, 21, 22, 23, 24, 127,
/// ]);
/// let b = m256i::from([
///   -1_i8, -1, 0, 2, 2, 3, 0, 5, 6, 6, -7, 8, 8, 0, 0, 10, 10, -11, 11, 12, 12, 13, 13, 12, 11,
///   -10, 9, 8, 7, 6, 5, -4,
/// ]);
/// let c: [i8; 32] = sign_apply_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     -3, -11, 0, 13, 4, 15, 0, 17, 8, 19, -20, 21, 22, 0, 0, 127, 7, -11, 2, 13, 4, 15, 6, 17, 8,
///     -19, 20, 21, 22, 23, 24, -127
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_sign_epi8`]
/// * **Assembly:** `vpsignb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sign_apply_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sign_epi8(a.0, b.0) })
}

/// Lanewise `a * signum(b)` with lanes as `i16`
///
/// * If `b` is positive, the output is `a`.
/// * If `b` is zero, the output is 0.
/// * If `b` is negative, the output is `-a`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_i16, 13000, -2, -8, 0, 1, 2, 3, -8, -7, 6, 5, 0, 0, 0, 978]);
/// let c: [i16; 16] = sign_apply_i16_m256i(a, b).into();
/// assert_eq!(c, [5, 6, -2, -5, 0, 3, 1, 0, 12, -13, 56, 21, 0, 0, 0, 5]);
/// ```
/// * **Intrinsic:** [`_mm256_sign_epi16`]
/// * **Assembly:** `vpsignw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sign_apply_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sign_epi16(a.0, b.0) })
}

/// Lanewise `a * signum(b)` with lanes as `i32`
///
/// * If `b` is positive, the output is `a`.
/// * If `b` is zero, the output is 0.
/// * If `b` is negative, the output is `-a`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 0, -2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = sign_apply_i32_m256i(a, b).into();
/// assert_eq!(c, [0_i32, 0, -2, -3, 4, 5, 6, -7]);
/// ```
/// * **Intrinsic:** [`_mm256_sign_epi32`]
/// * **Assembly:** `vpsignd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sign_apply_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sign_epi32(a.0, b.0) })
}

/// Lanewise `u16` shift left by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u16, 6, 2, 5, 4, 3, 1, 0, 12, 13, 56, 21, 8, 7, 6, 5]);
/// let count = m128i::from(1_u128);
/// let b: [u16; 16] = shl_all_u16_m256i(a, count).into();
/// assert_eq!(b, [10, 12, 4, 10, 8, 6, 2, 0, 24, 26, 112, 42, 16, 14, 12, 10]);
/// ```
/// * **Intrinsic:** [`_mm256_sll_epi16`]
/// * **Assembly:** `vpsllw ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_all_u16_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_sll_epi16(a.0, count.0) })
}

/// Shift all `u32` lanes left by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1, 2, 13, 4, 15, 6, 17]);
/// let count = m128i::from(1_u128);
/// let b: [u32; 8] = shl_all_u32_m256i(a, count).into();
/// assert_eq!(b, [0, 2, 4, 26, 8, 30, 12, 34]);
/// ```
/// * **Intrinsic:** [`_mm256_sll_epi32`]
/// * **Assembly:** `vpslld ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_all_u32_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_sll_epi32(a.0, count.0) })
}

/// Shift all `u64` lanes left by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u64, 1, 2, 13]);
/// let count = m128i::from(1_u128);
/// let b: [u64; 4] = shl_all_u64_m256i(a, count).into();
/// assert_eq!(b, [0, 2, 4, 26]);
/// ```
/// * **Intrinsic:** [`_mm256_sll_epi64`]
/// * **Assembly:** `vpsllq ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_all_u64_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_sll_epi64(a.0, count.0) })
}

/// Shifts all `u16` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_u16, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]);
/// let c: [u16; 16] = shl_imm_u16_m256i::<1>(a).into();
/// assert_eq!(c, [2, 4, 6, 8, 2, 4, 6, 8, 2, 4, 6, 8, 2, 4, 6, 8]);
/// ```
/// * **Intrinsic:** [`_mm256_slli_epi16`]
/// * **Assembly:** `vpsllw ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_imm_u16_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_slli_epi16(a.0, IMM) })
}

/// Shifts all `u32` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_u32, 2, 3, 4, 1, 2, 3, 4]);
/// let c: [u32; 8] = shl_imm_u32_m256i::<1>(a).into();
/// assert_eq!(c, [1_u32 << 1, 2 << 1, 3 << 1, 4 << 1, 1 << 1, 2 << 1, 3 << 1, 4 << 1]);
/// ```
/// * **Intrinsic:** [`_mm256_slli_epi32`]
/// * **Assembly:** `vpslld ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_imm_u32_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_slli_epi32(a.0, IMM) })
}

/// Shifts all `u64` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_u64, 2, 3, 4]);
/// let c: [u64; 4] = shl_imm_u64_m256i::<1>(a).into();
/// assert_eq!(c, [1_u64 << 1, 2 << 1, 3 << 1, 4 << 1,]);
/// ```
/// * **Intrinsic:** [`_mm256_slli_epi64`]
/// * **Assembly:** `vpsllq ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_imm_u64_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_slli_epi64(a.0, IMM) })
}

/// Lanewise `u32` shift left by the matching `i32` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1, 2, 13, 5, 6, 7, 1]);
/// let count = m256i::from([1_u32, 2, 3, 4, 5, 6, 7, 1]);
/// let b: [u32; 8] = shl_each_u32_m256i(a, count).into();
/// assert_eq!(b, [0, 4, 16, 208, 160, 384, 896, 2]);
/// ```
/// * **Intrinsic:** [`_mm256_sllv_epi32`]
/// * **Assembly:** `vpsllvd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_each_u32_m256i(a: m256i, count: m256i) -> m256i {
  m256i(unsafe { _mm256_sllv_epi32(a.0, count.0) })
}

/// Lanewise `u64` shift left by the matching `u64` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u64, 1, 2, 13]);
/// let count = m256i::from([1_u64, 2, 3, 4]);
/// let b: [u64; 4] = shl_each_u64_m256i(a, count).into();
/// assert_eq!(b, [0, 4, 16, 208]);
/// ```
/// * **Intrinsic:** [`_mm256_sllv_epi64`]
/// * **Assembly:** `vpsllvq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shl_each_u64_m256i(a: m256i, count: m256i) -> m256i {
  m256i(unsafe { _mm256_sllv_epi64(a.0, count.0) })
}

/// Lanewise `i16` shift right by the lower `i64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let count = m128i::from(1_i128);
/// let b: [i16; 16] = shr_all_i16_m256i(a, count).into();
/// assert_eq!(b, [2, 3, 1, 2, 2, 1, 0, 0, -6, 6, 28, 10, 4, 3, 3, 2]);
/// ```
/// * **Intrinsic:** [`_mm256_sra_epi16`]
/// * **Assembly:** `vpsraw ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_all_i16_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_sra_epi16(a.0, count.0) })
}

/// Lanewise `i32` shift right by the lower `i64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, -2, -13, 4, 15, 6, -17]);
/// let count = m128i::from(1_i128);
/// let b: [i32; 8] = shr_all_i32_m256i(a, count).into();
/// assert_eq!(b, [0, 0, -1, -7, 2, 7, 3, -9]);
/// ```
/// * **Intrinsic:** [`_mm256_sra_epi32`]
/// * **Assembly:** `vpsrad ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_all_i32_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_sra_epi32(a.0, count.0) })
}

/// Shifts all `i16` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i16, 2, 3, 4, -1, -2, -3, -4, 1, 2, 3, 4, -1, -2, -3, -4]);
/// let c: [i16; 16] = shr_imm_i16_m256i::<1>(a).into();
/// assert_eq!(c, [0_i16, 1, 1, 2, -1, -1, -2, -2, 0, 1, 1, 2, -1, -1, -2, -2]);
/// ```
/// * **Intrinsic:** [`_mm256_srai_epi16`]
/// * **Assembly:** `vpsraw ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_imm_i16_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_srai_epi16(a.0, IMM) })
}

/// Shifts all `i32` lanes left by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i32, 2, 3, 4, -1, -2, -3, -4]);
/// let c: [i32; 8] = shr_imm_i32_m256i::<1>(a).into();
/// assert_eq!(c, [0, 1, 1, 2, -1, -1, -2, -2]);
/// ```
/// * **Intrinsic:** [`_mm256_srai_epi32`]
/// * **Assembly:** `vpsrad ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_imm_i32_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_srai_epi32(a.0, IMM) })
}

/// Lanewise `i32` shift right by the matching `i32` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1111, -2999, -13888, 5444, 6222, 7333, -11111]);
/// let count = m256i::from([1_i32, 2, 3, 4, 5, 4, 3, 2]);
/// let b: [i32; 8] = shr_each_i32_m256i(a, count).into();
/// assert_eq!(b, [0, 277, -375, -868, 170, 388, 916, -2778]);
/// ```
/// * **Intrinsic:** [`_mm256_srav_epi32`]
/// * **Assembly:** `vpsravd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_i32_m256i(a: m256i, count: m256i) -> m256i {
  m256i(unsafe { _mm256_srav_epi32(a.0, count.0) })
}

/// Lanewise `u16` shift right by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u16, 6, 2, 5, 4, 3, 1, 0, 12, 13, 56, 21, 8, 7, 6, 5]);
/// let count = m128i::from(1_u128);
/// let b: [u16; 16] = shr_all_u16_m256i(a, count).into();
/// assert_eq!(b, [2, 3, 1, 2, 2, 1, 0, 0, 6, 6, 28, 10, 4, 3, 3, 2]);
/// ```
/// * **Intrinsic:** [`_mm256_srl_epi16`]
/// * **Assembly:** `vpsrlw ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_all_u16_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_srl_epi16(a.0, count.0) })
}

/// Lanewise `u32` shift right by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1, 2, 13, 4, 15, 6, 17]);
/// let count = m128i::from(1_u128);
/// let b: [u32; 8] = shr_all_u32_m256i(a, count).into();
/// assert_eq!(b, [0, 0, 1, 6, 2, 7, 3, 8]);
/// ```
/// * **Intrinsic:** [`_mm256_srl_epi32`]
/// * **Assembly:** `vpsrld ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_all_u32_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_srl_epi32(a.0, count.0) })
}

/// Lanewise `u64` shift right by the lower `u64` lane of `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u64, 1, 2, 13]);
/// let count = m128i::from(1_u128);
/// let b: [u64; 4] = shr_all_u64_m256i(a, count).into();
/// assert_eq!(b, [0, 0, 1, 6]);
/// ```
/// * **Intrinsic:** [`_mm256_srl_epi64`]
/// * **Assembly:** `vpsrlq ymm, ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_all_u64_m256i(a: m256i, count: m128i) -> m256i {
  m256i(unsafe { _mm256_srl_epi64(a.0, count.0) })
}

/// Shifts all `u16` lanes right by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
/// let c: [u16; 16] = shr_imm_u16_m256i::<1>(a).into();
/// assert_eq!(c, [0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8]);
/// ```
/// * **Intrinsic:** [`_mm256_srli_epi16`]
/// * **Assembly:** `vpsrlw ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_imm_u16_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_srli_epi16(a.0, IMM) })
}

/// Shifts all `u32` lanes right by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i32, 2, 3, 4, 5, 6, 7, 8]);
/// let c: [u32; 8] = shr_imm_u32_m256i::<1>(a).into();
/// assert_eq!(c, [0, 1, 1, 2, 2, 3, 3, 4]);
/// ```
/// * **Intrinsic:** [`_mm256_srli_epi32`]
/// * **Assembly:** `vpsrld ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_imm_u32_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_srli_epi32(a.0, IMM) })
}

/// Shifts all `u64` lanes right by an immediate.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_u64, 2, 3, 4]);
/// let c: [u64; 4] = shr_imm_u64_m256i::<1>(a).into();
/// assert_eq!(c, [0, 1, 1, 2]);
/// ```
/// * **Intrinsic:** [`_mm256_srli_epi64`]
/// * **Assembly:** `vpsrlq ymm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_imm_u64_m256i<const IMM: i32>(a: m256i) -> m256i {
  m256i(unsafe { _mm256_srli_epi64(a.0, IMM) })
}

/// Lanewise `u32` shift right by the matching `u32` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u32, 1111, 2999, 13888, 5444, 6222, 7333, 11111]);
/// let count = m256i::from([1_i32, 2, 3, 4, 5, 4, 3, 2]);
/// let b: [u32; 8] = shr_each_u32_m256i(a, count).into();
/// assert_eq!(b, [0, 277, 374, 868, 170, 388, 916, 2777]);
/// ```
/// * **Intrinsic:** [`_mm256_srlv_epi32`]
/// * **Assembly:** `vpsrlvd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_u32_m256i(a: m256i, count: m256i) -> m256i {
  m256i(unsafe { _mm256_srlv_epi32(a.0, count.0) })
}

/// Lanewise `u64` shift right by the matching `i64` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_u64, 1111, 2999, 13888]);
/// let count = m256i::from([1_u64, 2, 3, 4]);
/// let b: [u64; 4] = shr_each_u64_m256i(a, count).into();
/// assert_eq!(b, [0, 277, 374, 868]);
/// ```
/// * **Intrinsic:** [`_mm256_srlv_epi64`]
/// * **Assembly:** `vpsrlvq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn shr_each_u64_m256i(a: m256i, count: m256i) -> m256i {
  m256i(unsafe { _mm256_srlv_epi64(a.0, count.0) })
}

/// Lanewise `a - b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([10_i8; 32]);
/// let c: [i8; 32] = sub_i8_m256i(a, b).into();
/// assert_eq!(c, [-5_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_sub_epi8`]
/// * **Assembly:** `vpsubb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sub_epi8(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([10_i16; 16]);
/// let c: [i16; 16] = sub_i16_m256i(a, b).into();
/// assert_eq!(c, [-5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_sub_epi16`]
/// * **Assembly:** `vpsubw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sub_epi16(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([10_i32; 8]);
/// let c: [i32; 8] = sub_i32_m256i(a, b).into();
/// assert_eq!(c, [-5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_sub_epi32`]
/// * **Assembly:** `vpsubd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sub_epi32(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i64; 4]);
/// let b = m256i::from([10_i64; 4]);
/// let c: [i64; 4] = sub_i64_m256i(a, b).into();
/// assert_eq!(c, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_sub_epi64`]
/// * **Assembly:** `vpsubq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_sub_epi64(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([126_i8; 32]);
/// let b = m256i::from([125_i8; 32]);
/// let c: [i8; 32] = sub_saturating_i8_m256i(a, b).into();
/// assert_eq!(c, [1_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_subs_epi8`]
/// * **Assembly:** `vpsubsb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_saturating_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_subs_epi8(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([32700_i16; 16]);
/// let b = m256i::from([32000_i16; 16]);
/// let c: [i16; 16] = sub_saturating_i16_m256i(a, b).into();
/// assert_eq!(c, [700_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_subs_epi16`]
/// * **Assembly:** `vpsubsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_subs_epi16(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([126_u8; 32]);
/// let b = m256i::from([125_u8; 32]);
/// let c: [u8; 32] = sub_saturating_u8_m256i(a, b).into();
/// assert_eq!(c, [1_u8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_subs_epu8`]
/// * **Assembly:** `vpsubusb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_saturating_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_subs_epu8(a.0, b.0) })
}

/// Lanewise saturating `a - b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([32700_u16; 16]);
/// let b = m256i::from([32000_u16; 16]);
/// let c: [u16; 16] = sub_saturating_u16_m256i(a, b).into();
/// assert_eq!(c, [700_u16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_subs_epu16`]
/// * **Assembly:** `vpsubusw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn sub_saturating_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_subs_epu16(a.0, b.0) })
}

/// Unpack and interleave high `i8` lanes of `a` and `b`.
///
/// * Operates on the high half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   3_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 7, 11, 2, 13, 4, 15, 6, 17, 8,
///   19, 20, 21, 22, 23, 24, 127,
/// ]);
/// let b = m256i::from([
///   -1_i8, -1, 0, 2, 2, 3, 0, 5, 6, 6, -7, 8, 8, 0, 0, 10, 10, -11, 11, 12, 12, 13, 13, 12, 11,
///   -10, 9, 8, 7, 6, 5, -4,
/// ]);
/// let c: [i8; 32] = unpack_high_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     8, 6, 19, 6, 20, -7, 21, 8, 22, 8, 23, 0, 24, 0, 127, 10, 8, 11, 19, -10, 20, 9, 21, 8, 22,
///     7, 23, 6, 24, 5, 127, -4
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_unpackhi_epi8`]
/// * **Assembly:** `vpunpckhbw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_high_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpackhi_epi8(a.0, b.0) })
}

/// Unpack and interleave high `i16` lanes of `a` and `b`.
///
/// * Operates on the high half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_i16, 13000, -2, -8, 0, 1, 2, 3, -8, -7, 6, 5, 0, 0, 0, 978]);
/// let c: [i16; 16] = unpack_high_i16_m256i(a, b).into();
/// assert_eq!(c, [4, 0, 3, 1, 1, 2, 0, 3, 8, 0, 7, 0, 6, 0, 5, 978]);
/// ```
/// * **Intrinsic:** [`_mm256_unpackhi_epi16`]
/// * **Assembly:** `vpunpckhwd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_high_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpackhi_epi16(a.0, b.0) })
}

/// Unpack and interleave high `i32` lanes of `a` and `b`.
///
/// * Operates on the high half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 11, 2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = unpack_high_i32_m256i(a, b).into();
/// assert_eq!(c, [2, 2, 3, -13, 6, 6, 7, -17]);
/// ```
/// * **Intrinsic:** [`_mm256_unpackhi_epi32`]
/// * **Assembly:** `vpunpckhdq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_high_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpackhi_epi32(a.0, b.0) })
}

/// Unpack and interleave high `i64` lanes of `a` and `b`.
///
/// * Operates on the high half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i64, 2, 3, 4]);
/// let b = m256i::from([5_i64, 6, 7, -8]);
/// let c: [i64; 4] = unpack_high_i64_m256i(a, b).into();
/// assert_eq!(c, [2, 6, 4, -8]);
/// ```
/// * **Intrinsic:** [`_mm256_unpackhi_epi64`]
/// * **Assembly:** `vpunpckhqdq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_high_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpackhi_epi64(a.0, b.0) })
}

/// Unpack and interleave low `i8` lanes of `a` and `b`.
///
/// * Operates on the low half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   3_i8, 11, 2, 13, 4, 15, 6, 17, 8, 19, 20, 21, 22, 23, 24, 127, 7, 11, 2, 13, 4, 15, 6, 17, 8,
///   19, 20, 21, 22, 23, 24, 127,
/// ]);
/// let b = m256i::from([
///   -1_i8, -1, 0, 2, 2, 3, 0, 5, 6, 6, -7, 8, 8, 0, 0, 10, 10, -11, 11, 12, 12, 13, 13, 12, 11,
///   -10, 9, 8, 7, 6, 5, -4,
/// ]);
/// let c: [i8; 32] = unpack_low_i8_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     3, -1, 11, -1, 2, 0, 13, 2, 4, 2, 15, 3, 6, 0, 17, 5, 7, 10, 11, -11, 2, 11, 13, 12, 4, 12,
///     15, 13, 6, 13, 17, 12
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_unpacklo_epi8`]
/// * **Assembly:** `_mm256_unpacklo_epi8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_low_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpacklo_epi8(a.0, b.0) })
}

/// Unpack and interleave low `i16` lanes of `a` and `b`.
///
/// * Operates on the low half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([12000_i16, 13000, -2, -8, 0, 1, 2, 3, -8, -7, 6, 5, 0, 0, 0, 978]);
/// let c: [i16; 16] = unpack_low_i16_m256i(a, b).into();
/// assert_eq!(c, [5, 12000, 6, 13000, 2, -2, 5, -8, -12, -8, 13, -7, 56, 6, 21, 5]);
/// ```
/// * **Intrinsic:** [`_mm256_unpacklo_epi16`]
/// * **Assembly:** `vpunpcklwd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_low_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpacklo_epi16(a.0, b.0) })
}

/// Unpack and interleave low `i32` lanes of `a` and `b`.
///
/// * Operates on the low half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m256i::from([0_i32, 11, 2, -13, 4, 15, 6, -17]);
/// let c: [i32; 8] = unpack_low_i32_m256i(a, b).into();
/// assert_eq!(c, [0, 0, 1, 11, 4, 4, 5, 15]);
/// ```
/// * **Intrinsic:** [`_mm256_unpacklo_epi32`]
/// * **Assembly:** `vpunpckldq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_low_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpacklo_epi32(a.0, b.0) })
}

/// Unpack and interleave low `i64` lanes of `a` and `b`.
///
/// * Operates on the low half of each 128 bit portion.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i64, 2, 3, 4]);
/// let b = m256i::from([5_i64, 6, 7, -8]);
/// let c: [i64; 4] = unpack_low_i64_m256i(a, b).into();
/// assert_eq!(c, [1, 5, 3, 7]);
/// ```
/// * **Intrinsic:** [`_mm256_unpacklo_epi64`]
/// * **Assembly:** `vpunpcklqdq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn unpack_low_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_unpacklo_epi64(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = bitxor_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 1, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_xor_si256`]
/// * **Assembly:** `vpxor ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx2")))]
pub fn bitxor_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_xor_si256(a.0, b.0) })
}

impl Not for m256i {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  /// ```
  /// # use safe_arch::*;
  /// let a = m256i::from([0_u128, 0]);
  /// let c: [u128; 2] = (!a).into();
  /// assert_eq!(c, [u128::MAX, u128::MAX]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_i16_m256i(-1);
    self ^ all_bits
  }
}

impl BitAnd for m256i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m256i::from([0_i64, 0, 1, 1]);
  /// let b = m256i::from([0_i64, 1, 0, 1]);
  /// let c: [i64; 4] = (a & b).into();
  /// assert_eq!(c, [0_i64, 0, 0, 1]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m256i(self, rhs)
  }
}
impl BitAndAssign for m256i {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m256i::from([0_i64, 0, 1, 1]);
  /// let b = m256i::from([0_i64, 1, 0, 1]);
  /// let c: [i64; 4] = (a | b).into();
  /// assert_eq!(c, [0_i64, 1, 1, 1]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m256i(self, rhs)
  }
}
impl BitOrAssign for m256i {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m256i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m256i::from([0_i64, 0, 1, 1]);
  /// let b = m256i::from([0_i64, 1, 0, 1]);
  /// let c: [i64; 4] = (a ^ b).into();
  /// assert_eq!(c, [0_i64, 1, 1, 0]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m256i(self, rhs)
  }
}
impl BitXorAssign for m256i {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl PartialEq for m256i {
  #[must_use]
  #[inline(always)]
  /// ```
  /// # use safe_arch::*;
  /// let a = m256i::from([0_i64, 0, 1, 1]);
  /// let b = m256i::from([0_i64, 1, 0, 1]);
  /// assert_eq!(a, a);
  /// assert_ne!(a, b);
  /// ```
  fn eq(&self, other: &Self) -> bool {
    let mask = cmp_eq_mask_i8_m256i(*self, *other);
    move_mask_i8_m256i(mask) == -1_i32
  }
}
impl Eq for m256i {}
