#![cfg(target_feature = "pclmulqdq")]

use super::*;

/// Performs a "carryless" multiplication of two `i64` values.
///
/// The `IMM` value selects which lanes of `a` and `b` are multiplied.
/// * Bit 0: the `i64` index from `a` to multiply.
/// * Bit 4: the `i64` index from `b` to multiply.
///
/// The output is always in the low `i64` lane, with the high lane as 0.
///
/// * **Intrinsic:** [`_mm_clmulepi64_si128`]
/// * **Assembly:** `pclmulqdq xmm, xmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "pclmulqdq")))]
pub fn mul_i64_carryless_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_clmulepi64_si128(a.0, b.0, IMM) })
}

