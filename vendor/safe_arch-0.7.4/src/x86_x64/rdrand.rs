#![cfg(target_feature = "rdrand")]

use super::*;

/// Try to obtain a random `u16` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdrand16_step`]
/// * **Assembly:** `rdrand r16`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdrand")))]
pub fn rdrand_u16(out: &mut u16) -> i32 {
  unsafe { _rdrand16_step(out) }
}

/// Try to obtain a random `u32` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdrand32_step`]
/// * **Assembly:** `rdrand r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdrand")))]
pub fn rdrand_u32(out: &mut u32) -> i32 {
  unsafe { _rdrand32_step(out) }
}

/// Try to obtain a random `u64` from the hardware RNG.
///
/// * **Intrinsic:** [`_rdrand64_step`]
/// * **Assembly:** `rdrand r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "rdrand")))]
pub fn rdrand_u64(out: &mut u64) -> i32 {
  unsafe { _rdrand64_step(out) }
}

