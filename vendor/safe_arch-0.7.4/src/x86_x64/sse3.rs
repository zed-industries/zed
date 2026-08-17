#![cfg(target_feature = "sse3")]

use super::*;

/// Add the high lane and subtract the low lane.
///
/// * **Intrinsic:** [`_mm_addsub_pd`]
/// * **Assembly:** `addsubpd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn addsub_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_addsub_pd(a.0, b.0) })
}

/// Alternately, from the top, add a lane and then subtract a lane.
///
/// * **Intrinsic:** [`_mm_addsub_ps`]
/// * **Assembly:** `addsubps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn addsub_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_addsub_ps(a.0, b.0) })
}

/// Add each lane horizontally, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hadd_pd`]
/// * **Assembly:** `haddpd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn add_horizontal_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_hadd_pd(a.0, b.0) })
}

/// Add each lane horizontally, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hadd_ps`]
/// * **Assembly:** `haddps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn add_horizontal_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_hadd_ps(a.0, b.0) })
}

/// Subtract each lane horizontally, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hsub_pd`]
/// * **Assembly:** `hsubpd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn sub_horizontal_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_hsub_pd(a.0, b.0) })
}

/// Subtract each lane horizontally, pack the outputs as `a` then `b`.
///
/// * **Intrinsic:** [`_mm_hsub_ps`]
/// * **Assembly:** `hsubps xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn sub_horizontal_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_hsub_ps(a.0, b.0) })
}

/// Copy the low lane of the input to both lanes of the output.
///
/// * **Intrinsic:** [`_mm_movedup_pd`]
/// * **Assembly:** `movddup xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_low_lane_m128d_s(a: m128d) -> m128d {
  m128d(unsafe { _mm_movedup_pd(a.0) })
}

/// Duplicate the odd lanes to the even lanes.
///
/// * **Intrinsic:** [`_mm_movehdup_ps`]
/// * **Assembly:** `movshdup xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_odd_lanes_m128(a: m128) -> m128 {
  m128(unsafe { _mm_movehdup_ps(a.0) })
}

/// Duplicate the odd lanes to the even lanes.
///
/// * **Intrinsic:** [`_mm_moveldup_ps`]
/// * **Assembly:** `movsldup xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_even_lanes_m128(a: m128) -> m128 {
  m128(unsafe { _mm_moveldup_ps(a.0) })
}

