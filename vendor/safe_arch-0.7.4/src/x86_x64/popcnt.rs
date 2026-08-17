#![cfg(target_feature = "popcnt")]

use super::*;

/// Count the number of bits set within an `i32`
///
/// * **Intrinsic:** [`_popcnt32`]
/// * **Assembly:** `popcnt r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "popcnt")))]
pub fn population_count_i32(a: i32) -> i32 {
  unsafe { _popcnt32(a) }
}

/// Count the number of bits set within an `i64`
///
/// * **Intrinsic:** [`_popcnt64`]
/// * **Assembly:** `popcnt r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "popcnt")))]
pub fn population_count_i64(a: i64) -> i32 {
  unsafe { _popcnt64(a) }
}

