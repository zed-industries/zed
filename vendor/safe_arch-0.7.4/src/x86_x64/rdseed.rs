#![cfg(target_feature = "rdseed")]

use super::*;

/// Try to obtain a random `u16` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdseed16_step`]
/// * **Assembly:** `rdseed r16`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdseed")))]
pub fn rdseed_u16(out: &mut u16) -> i32 {
  unsafe { _rdseed16_step(out) }
}

/// Try to obtain a random `u32` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdseed32_step`]
/// * **Assembly:** `rdseed r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdseed")))]
pub fn rdseed_u32(out: &mut u32) -> i32 {
  unsafe { _rdseed32_step(out) }
}

/// Try to obtain a random `u64` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdseed64_step`]
/// * **Assembly:** `rdseed r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdseed")))]
pub fn rdseed_u64(out: &mut u64) -> i32 {
  unsafe { _rdseed64_step(out) }
}

