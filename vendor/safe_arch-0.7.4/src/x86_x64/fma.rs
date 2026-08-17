#![cfg(target_feature = "fma")]

use super::*;

// Note(Lokathor): There's only a few significant operations here but each
// operation has variants for each of the four floating SIMD types (m128, m128d,
// m256, and m256d), as well as possibly working on the scalar forms. We should
// try to keep each operation grouping sorted into the order: m128, m128_s,
// m128d, m128d_s, m256, m256d

//
// MUL ADD
//

/// Lanewise fused `(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_add_m128(a, b, c).to_array();
/// assert_eq!(d, [9.0, 16.0, 25.0, 36.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmadd_ps`]
/// * **Assembly:** one of
///   * `vfmadd132ps xmm, xmm, xmm`
///   * `vfmadd213ps xmm, xmm, xmm`
///   * `vfmadd231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmadd_ps(a.0, b.0, c.0) })
}

/// Low lane fused `(a * b) + c`, other lanes unchanged
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_add_m128_s(a, b, c).to_array();
/// assert_eq!(d, [9.0, 3.0, 4.0, 5.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmadd_ss`]
/// * **Assembly:** one of
///   * `vfmadd132ss xmm, xmm, xmm`
///   * `vfmadd213ss xmm, xmm, xmm`
///   * `vfmadd231ss xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m128_s(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmadd_ss(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_add_m128d(a, b, c).to_array();
/// assert_eq!(d, [9.0, 16.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmadd_pd`]
/// * **Assembly:** one of
///   * `vfmadd132pd xmm, xmm, xmm`
///   * `vfmadd213pd xmm, xmm, xmm`
///   * `vfmadd231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmadd_pd(a.0, b.0, c.0) })
}

/// Low lane fused `(a * b) + c`, other lanes unchanged
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_add_m128d_s(a, b, c).to_array();
/// assert_eq!(d, [9.0, 3.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmadd_sd`]
/// * **Assembly:** one of
///   * `vfmadd132sd xmm, xmm, xmm`
///   * `vfmadd213sd xmm, xmm, xmm`
///   * `vfmadd231sd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmadd_sd(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_add_m256(a, b, c).to_array();
/// assert_eq!(d, [6.0, 13.0, 22.0, 33.0, 6.0, 13.0, 22.0, 33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmadd_ps`]
/// * **Assembly:** one of
///   * `vfmadd132ps ymm, ymm, ymm`
///   * `vfmadd213ps ymm, ymm, ymm`
///   * `vfmadd231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fmadd_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_add_m256d(a, b, c).to_array();
/// assert_eq!(d, [6.0, 13.0, 22.0, 33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmadd_pd`]
/// * **Assembly:** one of
///   * `vfmadd132pd ymm, ymm, ymm`
///   * `vfmadd213pd ymm, ymm, ymm`
///   * `vfmadd231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_add_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fmadd_pd(a.0, b.0, c.0) })
}

//
// MUL SUB
//

/// Lanewise fused `(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_sub_m128(a, b, c).to_array();
/// assert_eq!(d, [7.0, 14.0, 23.0, 34.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsub_ps`]
/// * **Assembly:** one of
///   * `vfmsub132ps xmm, xmm, xmm`
///   * `vfmsub213ps xmm, xmm, xmm`
///   * `vfmsub231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmsub_ps(a.0, b.0, c.0) })
}

/// Low lane fused `(a * b) - c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_sub_m128_s(a, b, c).to_array();
/// assert_eq!(d, [7.0, 3.0, 4.0, 5.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsub_ss`]
/// * **Assembly:** one of
///   * `vfmsub132ss xmm, xmm, xmm`
///   * `vfmsub213ss xmm, xmm, xmm`
///   * `vfmsub231ss xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m128_s(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmsub_ss(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_sub_m128d(a, b, c).to_array();
/// assert_eq!(d, [7.0, 14.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsub_pd`]
/// * **Assembly:** one of
///   * `vfmsub132pd xmm, xmm, xmm`
///   * `vfmsub213pd xmm, xmm, xmm`
///   * `vfmsub231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmsub_pd(a.0, b.0, c.0) })
}

/// Low lane fused `(a * b) - c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_sub_m128d_s(a, b, c).to_array();
/// assert_eq!(d, [7.0, 3.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsub_sd`]
/// * **Assembly:** one of
///   * `vfmsub132sd xmm, xmm, xmm`
///   * `vfmsub213sd xmm, xmm, xmm`
///   * `vfmsub231sd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmsub_sd(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_sub_m256(a, b, c).to_array();
/// assert_eq!(d, [4.0, 11.0, 20.0, 31.0, 4.0, 11.0, 20.0, 31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmsub_ps`]
/// * **Assembly:** one of
///   * `vfmsub132ps ymm, ymm, ymm`
///   * `vfmsub213ps ymm, ymm, ymm`
///   * `vfmsub231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fmsub_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_sub_m256d(a, b, c).to_array();
/// assert_eq!(d, [4.0, 11.0, 20.0, 31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmsub_pd`]
/// * **Assembly:** one of
///   * `vfmsub132pd ymm, ymm, ymm`
///   * `vfmsub213pd ymm, ymm, ymm`
///   * `vfmsub231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_sub_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fmsub_pd(a.0, b.0, c.0) })
}

//
// MUL NEG ADD
//

/// Lanewise fused `-(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_add_m128(a, b, c).to_array();
/// assert_eq!(d, [-7.0, -14.0, -23.0, -34.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmadd_ps`]
/// * **Assembly:** one of
///   * `vfnmadd132ps xmm, xmm, xmm`
///   * `vfnmadd213ps xmm, xmm, xmm`
///   * `vfnmadd231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fnmadd_ps(a.0, b.0, c.0) })
}

/// Low lane `-(a * b) + c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_add_m128_s(a, b, c).to_array();
/// assert_eq!(d, [-7.0, 3.0, 4.0, 5.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmadd_ss`]
/// * **Assembly:** one of
///   * `vfnmadd132ss xmm, xmm, xmm`
///   * `vfnmadd213ss xmm, xmm, xmm`
///   * `vfnmadd231ss xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m128_s(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fnmadd_ss(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_neg_add_m128d(a, b, c).to_array();
/// assert_eq!(d, [-7.0, -14.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmadd_pd`]
/// * **Assembly:** one of
///   * `vfnmadd132pd xmm, xmm, xmm`
///   * `vfnmadd213pd xmm, xmm, xmm`
///   * `vfnmadd231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fnmadd_pd(a.0, b.0, c.0) })
}

/// Low lane `-(a * b) + c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_neg_add_m128d_s(a, b, c).to_array();
/// assert_eq!(d, [-7.0, 3.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmadd_sd`]
/// * **Assembly:** one of
///   * `vfnmadd132sd xmm, xmm, xmm`
///   * `vfnmadd213sd xmm, xmm, xmm`
///   * `vfnmadd231sd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fnmadd_sd(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_add_m256(a, b, c).to_array();
/// assert_eq!(d, [-4.0, -11.0, -20.0, -31.0, -4.0, -11.0, -20.0, -31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fnmadd_ps`]
/// * **Assembly:** one of
///   * `vfnmadd132ps ymm, ymm, ymm`
///   * `vfnmadd213ps ymm, ymm, ymm`
///   * `vfnmadd231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fnmadd_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) + c`
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_add_m256d(a, b, c).to_array();
/// assert_eq!(d, [-4.0, -11.0, -20.0, -31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fnmadd_pd`]
/// * **Assembly:** one of
///   * `vfnmadd132pd ymm, ymm, ymm`
///   * `vfnmadd213pd ymm, ymm, ymm`
///   * `vfnmadd231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_add_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fnmadd_pd(a.0, b.0, c.0) })
}

//
// MUL NEG SUB
//

/// Lanewise fused `-(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_sub_m128(a, b, c).to_array();
/// assert_eq!(d, [-9.0, -16.0, -25.0, -36.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmsub_ps`]
/// * **Assembly:** one of
///   * `vfnmsub132ps xmm, xmm, xmm`
///   * `vfnmsub213ps xmm, xmm, xmm`
///   * `vfnmsub231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fnmsub_ps(a.0, b.0, c.0) })
}

/// Low lane fused `-(a * b) - c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_sub_m128_s(a, b, c).to_array();
/// assert_eq!(d, [-9.0, 3.0, 4.0, 5.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmsub_ss`]
/// * **Assembly:** one of
///   * `vfnmsub132ss xmm, xmm, xmm`
///   * `vfnmsub213ss xmm, xmm, xmm`
///   * `vfnmsub231ss xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m128_s(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fnmsub_ss(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_neg_sub_m128d(a, b, c).to_array();
/// assert_eq!(d, [-9.0, -16.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmsub_pd`]
/// * **Assembly:** one of
///   * `vfnmsub132pd xmm, xmm, xmm`
///   * `vfnmsub213pd xmm, xmm, xmm`
///   * `vfnmsub231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fnmsub_pd(a.0, b.0, c.0) })
}

/// Low lane fused `-(a * b) - c`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_neg_sub_m128d_s(a, b, c).to_array();
/// assert_eq!(d, [-9.0, 3.0]);
/// ```
/// * **Intrinsic:** [`_mm_fnmsub_sd`]
/// * **Assembly:** one of
///   * `vfnmsub132sd xmm, xmm, xmm`
///   * `vfnmsub213sd xmm, xmm, xmm`
///   * `vfnmsub231sd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fnmsub_sd(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_sub_m256(a, b, c).to_array();
/// assert_eq!(d, [-6.0, -13.0, -22.0, -33.0, -6.0, -13.0, -22.0, -33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fnmsub_ps`]
/// * **Assembly:** one of
///   * `vfnmsub132ps ymm, ymm, ymm`
///   * `vfnmsub213ps ymm, ymm, ymm`
///   * `vfnmsub231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fnmsub_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `-(a * b) - c`
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_neg_sub_m256d(a, b, c).to_array();
/// assert_eq!(d, [-6.0, -13.0, -22.0, -33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fnmsub_pd`]
/// * **Assembly:** one of
///   * `vfnmsub132pd ymm, ymm, ymm`
///   * `vfnmsub213pd ymm, ymm, ymm`
///   * `vfnmsub231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_neg_sub_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fnmsub_pd(a.0, b.0, c.0) })
}

//
// MUL ADDSUB
//

/// Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_addsub_m128(a, b, c).to_array();
/// assert_eq!(d, [7.0, 16.0, 23.0, 36.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmaddsub_ps`]
/// * **Assembly:** one of
///   * `vfmaddsub132ps xmm, xmm, xmm`
///   * `vfmaddsub213ps xmm, xmm, xmm`
///   * `vfmaddsub231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_addsub_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmaddsub_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_addsub_m128d(a, b, c).to_array();
/// assert_eq!(d, [7.0, 16.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmaddsub_pd`]
/// * **Assembly:** one of
///   * `vfmaddsub132pd xmm, xmm, xmm`
///   * `vfmaddsub213pd xmm, xmm, xmm`
///   * `vfmaddsub231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_addsub_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmaddsub_pd(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_addsub_m256(a, b, c).to_array();
/// assert_eq!(d, [4.0, 13.0, 20.0, 33.0, 4.0, 13.0, 20.0, 33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmaddsub_ps`]
/// * **Assembly:** one of
///   * `vfmaddsub132ps ymm, ymm, ymm`
///   * `vfmaddsub213ps ymm, ymm, ymm`
///   * `vfmaddsub231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_addsub_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fmaddsub_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_addsub_m256d(a, b, c).to_array();
/// assert_eq!(d, [4.0, 13.0, 20.0, 33.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmaddsub_pd`]
/// * **Assembly:** one of
///   * `vfmaddsub132pd ymm, ymm, ymm`
///   * `vfmaddsub213pd ymm, ymm, ymm`
///   * `vfmaddsub231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_addsub_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fmaddsub_pd(a.0, b.0, c.0) })
}

//
// MUL SUBADD
//

/// Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 3.0, 4.0, 5.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_subadd_m128(a, b, c).to_array();
/// assert_eq!(d, [9.0, 14.0, 25.0, 34.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsubadd_ps`]
/// * **Assembly:**
///   * `vfmsubadd132ps xmm, xmm, xmm`
///   * `vfmsubadd213ps xmm, xmm, xmm`
///   * `vfmsubadd231ps xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_subadd_m128(a: m128, b: m128, c: m128) -> m128 {
  m128(unsafe { _mm_fmsubadd_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128d::from_array([4.0, 5.0]);
/// let c = m128d::from_array([1.0, 1.0]);
/// let d = fused_mul_subadd_m128d(a, b, c).to_array();
/// assert_eq!(d, [9.0, 14.0]);
/// ```
/// * **Intrinsic:** [`_mm_fmsubadd_pd`]
/// * **Assembly:** one of
///   * `vfmsubadd132pd xmm, xmm, xmm`
///   * `vfmsubadd213pd xmm, xmm, xmm`
///   * `vfmsubadd231pd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_subadd_m128d(a: m128d, b: m128d, c: m128d) -> m128d {
  m128d(unsafe { _mm_fmsubadd_pd(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0]);
/// let c = m256::from_array([1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_subadd_m256(a, b, c).to_array();
/// assert_eq!(d, [6.0, 11.0, 22.0, 31.0, 6.0, 11.0, 22.0, 31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmsubadd_ps`]
/// * **Assembly:** one of
///   * `vfmsubadd132ps ymm, ymm, ymm`
///   * `vfmsubadd213ps ymm, ymm, ymm`
///   * `vfmsubadd231ps ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_subadd_m256(a: m256, b: m256, c: m256) -> m256 {
  m256(unsafe { _mm256_fmsubadd_ps(a.0, b.0, c.0) })
}

/// Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = m256d::from_array([1.0, 1.0, 1.0, 1.0]);
/// let d = fused_mul_subadd_m256d(a, b, c).to_array();
/// assert_eq!(d, [6.0, 11.0, 22.0, 31.0]);
/// ```
/// * **Intrinsic:** [`_mm256_fmsubadd_pd`]
/// * **Assembly:** one of
///   * `vfmsubadd132pd ymm, ymm, ymm`
///   * `vfmsubadd213pd ymm, ymm, ymm`
///   * `vfmsubadd231pd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "fma")))]
pub fn fused_mul_subadd_m256d(a: m256d, b: m256d, c: m256d) -> m256d {
  m256d(unsafe { _mm256_fmsubadd_pd(a.0, b.0, c.0) })
}

