#![cfg(target_feature = "lzcnt")]

use super::*;

/// Count the leading zeroes in a `u32`.
///
/// * **Intrinsic:** [`_lzcnt_u32`]
/// * **Assembly:** `lzcnt r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "lzcnt")))]
pub fn leading_zero_count_u32(a: u32) -> u32 {
  unsafe { _lzcnt_u32(a) }
}

/// Count the leading zeroes in a `u64`.
///
/// * **Intrinsic:** [`_lzcnt_u64`]
/// * **Assembly:** `lzcnt r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "lzcnt")))]
pub fn leading_zero_count_u64(a: u64) -> u64 {
  unsafe { _lzcnt_u64(a) }
}

