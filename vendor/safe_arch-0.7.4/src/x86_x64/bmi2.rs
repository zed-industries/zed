#![cfg(target_feature = "bmi2")]

use super::*;

/// Zero out all high bits in a `u32` starting at the index given.
///
/// * **Intrinsic:** [`_bzhi_u32`]
/// * **Assembly:** `bzhi r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn bit_zero_high_index_u32(a: u32, index: u32) -> u32 {
  unsafe { _bzhi_u32(a, index) }
}

/// Zero out all high bits in a `u64` starting at the index given.
///
/// * **Intrinsic:** [`_bzhi_u64`]
/// * **Assembly:** `bzhi r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn bit_zero_high_index_u64(a: u64, index: u32) -> u64 {
  unsafe { _bzhi_u64(a, index) }
}

/// Multiply two `u32`, outputting the low bits and storing the high bits in the
/// reference.
///
/// This does not read or write arithmetic flags.
///
/// * **Intrinsic:** [`_mulx_u32`]
/// * **Assembly:** `mulx r32, r32, m32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn mul_extended_u32(a: u32, b: u32, extra: &mut u32) -> u32 {
  unsafe { _mulx_u32(a, b, extra) }
}

/// Multiply two `u64`, outputting the low bits and storing the high bits in the
/// reference.
///
/// This does not read or write arithmetic flags.
///
/// * **Intrinsic:** [`_mulx_u64`]
/// * **Assembly:** `mulx r64, r64, m64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn mul_extended_u64(a: u64, b: u64, extra: &mut u64) -> u64 {
  unsafe { _mulx_u64(a, b, extra) }
}

/// Deposit contiguous low bits from a `u32` according to a mask.
///
/// Other bits are zero.
///
/// * **Intrinsic:** [`_pdep_u32`]
/// * **Assembly:** `pdep r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn population_deposit_u32(a: u32, index: u32) -> u32 {
  unsafe { _pdep_u32(a, index) }
}

/// Deposit contiguous low bits from a `u64` according to a mask.
///
/// Other bits are zero.
///
/// * **Intrinsic:** [`_pdep_u64`]
/// * **Assembly:** `pdep r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn population_deposit_u64(a: u64, index: u64) -> u64 {
  unsafe { _pdep_u64(a, index) }
}

/// Extract bits from a `u32` according to a mask.
///
/// * **Intrinsic:** [`_pext_u32`]
/// * **Assembly:** `pext r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn population_extract_u32(a: u32, index: u32) -> u32 {
  unsafe { _pext_u32(a, index) }
}

/// Extract bits from a `u64` according to a mask.
///
/// * **Intrinsic:** [`_pext_u64`]
/// * **Assembly:** `pext r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi2")))]
pub fn population_extract_u64(a: u64, index: u64) -> u64 {
  unsafe { _pext_u64(a, index) }
}

