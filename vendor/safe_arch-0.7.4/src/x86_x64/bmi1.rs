#![cfg(target_feature = "bmi1")]

use super::*;

/// Bitwise `(!a) & b` for `u32`
///
/// * **Intrinsic:** [`_andn_u32`]
/// * **Assembly:** `andn r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bitandnot_u32(a: u32, b: u32) -> u32 {
  unsafe { _andn_u32(a, b) }
}

/// Bitwise `(!a) & b` for `u64`
///
/// * **Intrinsic:** [`_andn_u64`]
/// * **Assembly:** `andn r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bitandnot_u64(a: u64, b: u64) -> u64 {
  unsafe { _andn_u64(a, b) }
}

/// Extract a span of bits from the `u32`, start and len style.
///
/// * **Intrinsic:** [`_bextr_u32`]
/// * **Assembly:** `bextr r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract_u32(a: u32, start: u32, len: u32) -> u32 {
  unsafe { _bextr_u32(a, start, len) }
}

/// Extract a span of bits from the `u64`, start and len style.
///
/// * **Intrinsic:** [`_bextr_u64`]
/// * **Assembly:** `bextr r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract_u64(a: u64, start: u32, len: u32) -> u64 {
  unsafe { _bextr_u64(a, start, len) }
}

/// Extract a span of bits from the `u32`, control value style.
///
/// * Bits 0 through 7: start position.
/// * Bits 8 through 15: span length.
///
/// * **Intrinsic:** [`_bextr2_u32`]
/// * **Assembly:** `bextr r32, r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract2_u32(a: u32, control: u32) -> u32 {
  unsafe { _bextr2_u32(a, control) }
}

/// Extract a span of bits from the `u64`, control value style.
///
/// * Bits 0 through 7: start position.
/// * Bits 8 through 15: span length.
///
/// * **Intrinsic:** [`_bextr2_u64`]
/// * **Assembly:** `bextr r64, r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract2_u64(a: u64, control: u64) -> u64 {
  unsafe { _bextr2_u64(a, control) }
}

/// Gets the *value* of the lowest set bit in a `u32`.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(!a) & a`
///
/// * **Intrinsic:** [`_blsi_u32`]
/// * **Assembly:** `blsi r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_value_u32(a: u32) -> u32 {
  unsafe { _blsi_u32(a) }
}

/// Gets the *value* of the lowest set bit in a `u64`.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(!a) & a`
///
/// * **Intrinsic:** [`_blsi_u64`]
/// * **Assembly:** `blsi r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_value_u64(a: u64) -> u64 {
  unsafe { _blsi_u64(a) }
}

/// Gets the mask of all bits up to and including the lowest set bit in a `u32`.
///
/// If the input is 0, you get `u32::MAX`
///
/// * Formula: `(a - 1) ^ a`
///
/// * **Intrinsic:** [`_blsmsk_u32`]
/// * **Assembly:** `blsmsk r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_mask_u32(a: u32) -> u32 {
  unsafe { _blsmsk_u32(a) }
}

/// Gets the mask of all bits up to and including the lowest set bit in a `u64`.
///
/// If the input is 0, you get `u64::MAX`
///
/// * Formula: `(a - 1) ^ a`
///
/// * **Intrinsic:** [`_blsmsk_u64`]
/// * **Assembly:** `blsmsk r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_mask_u64(a: u64) -> u64 {
  unsafe { _blsmsk_u64(a) }
}

/// Resets (clears) the lowest set bit.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(a - 1) & a`
///
/// * **Intrinsic:** [`_blsr_u32`]
/// * **Assembly:** `blsr r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_reset_u32(a: u32) -> u32 {
  unsafe { _blsr_u32(a) }
}

/// Resets (clears) the lowest set bit.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(a - 1) & a`
///
/// * **Intrinsic:** [`_blsr_u64`]
/// * **Assembly:** `blsr r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_reset_u64(a: u64) -> u64 {
  unsafe { _blsr_u64(a) }
}

/// Counts the number of trailing zero bits in a `u32`.
///
/// An input of 0 gives 32.
///
/// * **Intrinsic:** [`_tzcnt_u32`]
/// * **Assembly:** `tzcnt r32, r32`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn trailing_zero_count_u32(a: u32) -> u32 {
  unsafe { _tzcnt_u32(a) }
}

/// Counts the number of trailing zero bits in a `u64`.
///
/// An input of 0 gives 64.
///
/// * **Intrinsic:** [`_tzcnt_u64`]
/// * **Assembly:** `tzcnt r64, r64`
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docsrs, doc(cfg(target_feature = "bmi1")))]
pub fn trailing_zero_count_u64(a: u64) -> u64 {
  unsafe { _tzcnt_u64(a) }
}

