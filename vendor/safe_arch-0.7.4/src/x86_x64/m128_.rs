//! This module is for the `m128` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 128-bit SSE register of four `f32` lanes.
///
/// * This is _very similar to_ having `[f32; 4]`. The main difference is that
///   it's aligned to 16 instead of just 4, and of course you can perform
///   various intrinsic operations on it.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m128(pub __m128);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for m128 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for m128 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<__m128> for m128 {}

impl m128 {
  /// Transmutes the `m128` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f32; 4] {
    self.into()
  }

  /// Transmutes an array into `m128`.
  ///
  /// Same as `m128::from(arr)`, it just lets you be more explicit about what's
  /// happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f32; 4]) -> Self {
    f.into()
  }

  //

  /// Converts into the bit patterns of these floats (`[u32;4]`).
  ///
  /// Like [`f32::to_bits`](f32::to_bits), but all four lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn to_bits(self) -> [u32; 4] {
    unsafe { core::mem::transmute(self) }
  }

  /// Converts from the bit patterns of these floats (`[u32;4]`).
  ///
  /// Like [`f32::from_bits`](f32::from_bits), but all four lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn from_bits(bits: [u32; 4]) -> Self {
    unsafe { core::mem::transmute(bits) }
  }
}

impl Clone for m128 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m128 {}

impl Default for m128 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[f32; 4]> for m128 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [f32; 4]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128> for [f32; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m128 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "m128(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for m128 {
  /// Display formats each float, and leaves the type name off of the font.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Display::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Binary for m128 {
  /// Binary formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl LowerExp for m128 {
  /// LowerExp formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerExp::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl UpperExp for m128 {
  /// UpperExp formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperExp::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl LowerHex for m128 {
  /// LowerHex formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl UpperHex for m128 {
  /// UpperHex formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl Octal for m128 {
  /// Octal formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}
