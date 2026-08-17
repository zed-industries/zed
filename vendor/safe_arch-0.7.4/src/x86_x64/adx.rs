#![cfg(target_feature = "adx")]

use super::*;

/// Add two `u32` with a carry value.
///
/// Writes the sum to the reference, and returns the new carry flag.
///
/// * **Intrinsic:** [`_addcarryx_u32`]
/// * **Assembly:**
/// ```asm
/// adcx r32, r32
/// adox r32, r32
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "adx")))]
pub fn add_carry_u32(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8 {
  unsafe { _addcarryx_u32(c_in, a, b, out) }
}

/// Add two `u64` with a carry value.
///
/// Writes the sum to the reference and returns the new carry flag.
///
/// * **Intrinsic:** [`_addcarryx_u64`]
/// * **Assembly:**
/// ```asm
/// adcx r64, r64
/// adox r64, r64
/// ```
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "adx")))]
#[cfg(target_arch = "x86_64")]
pub fn add_carry_u64(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8 {
  unsafe { _addcarryx_u64(c_in, a, b, out) }
}
