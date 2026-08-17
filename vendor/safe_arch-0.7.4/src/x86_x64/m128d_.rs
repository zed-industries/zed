//! This module is for the `m128d` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 128-bit SSE register of two `f64` values.
///
/// * This is _very similar to_ having `[f64; 2]`. The main difference is that
///   it's aligned to 16 instead of just 4, and of course you can perform
///   various intrinsic operations on it.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m128d(pub __m128d);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for m128d {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for m128d {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<__m128d> for m128d {}

impl m128d {
  /// Transmutes the `m128d` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f64; 2] {
    self.into()
  }

  /// Transmutes an array into `m128d`.
  ///
  /// Same as `m128d::from(arr)`, it just lets you be more explicit about what's
  /// happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f64; 2]) -> Self {
    f.into()
  }

  //

  /// Converts into the bit patterns of these doubles (`[u64;2]`).
  ///
  /// Like [`f64::to_bits`](f64::to_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn to_bits(self) -> [u64; 2] {
    unsafe { core::mem::transmute(self) }
  }

  /// Converts from the bit patterns of these doubles (`[u64;2]`).
  ///
  /// Like [`f64::from_bits`](f64::from_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn from_bits(bits: [u64; 2]) -> Self {
    unsafe { core::mem::transmute(bits) }
  }
}

impl Clone for m128d {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m128d {}

impl Default for m128d {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[f64; 2]> for m128d {
  #[must_use]
  #[inline(always)]
  fn from(arr: [f64; 2]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align8) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128d> for [f64; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128d) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m128d {
  /// Debug formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:?}", m128d::default());
  /// assert_eq!(&f, "m128d(0.0, 0.0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "m128d(")?;
    Debug::fmt(&a[0], f)?;
    write!(f, ", ")?;
    Debug::fmt(&a[1], f)?;
    write!(f, ")")
  }
}

impl Display for m128d {
  /// Display formats each double, and leaves the type name off of the font.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{}", m128d::default());
  /// assert_eq!(&f, "(0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    Display::fmt(&a[0], f)?;
    write!(f, ", ")?;
    Display::fmt(&a[1], f)?;
    write!(f, ")")
  }
}

impl Binary for m128d {
  /// Binary formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:b}", m128d::default());
  /// assert_eq!(&f, "(0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    Binary::fmt(&a[0].to_bits(), f)?;
    write!(f, ", ")?;
    Binary::fmt(&a[1].to_bits(), f)?;
    write!(f, ")")
  }
}

impl LowerExp for m128d {
  /// LowerExp formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:e}", m128d::default());
  /// assert_eq!(&f, "(0e0, 0e0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    LowerExp::fmt(&a[0], f)?;
    write!(f, ", ")?;
    LowerExp::fmt(&a[1], f)?;
    write!(f, ")")
  }
}

impl UpperExp for m128d {
  /// UpperExp formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:E}", m128d::default());
  /// assert_eq!(&f, "(0E0, 0E0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    UpperExp::fmt(&a[0], f)?;
    write!(f, ", ")?;
    UpperExp::fmt(&a[1], f)?;
    write!(f, ")")
  }
}

impl LowerHex for m128d {
  /// LowerHex formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:x}", m128d::default());
  /// assert_eq!(&f, "(0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    LowerHex::fmt(&a[0].to_bits(), f)?;
    write!(f, ", ")?;
    LowerHex::fmt(&a[1].to_bits(), f)?;
    write!(f, ")")
  }
}

impl UpperHex for m128d {
  /// UpperHex formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:X}", m128d::default());
  /// assert_eq!(&f, "(0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    UpperHex::fmt(&a[0].to_bits(), f)?;
    write!(f, ", ")?;
    UpperHex::fmt(&a[1].to_bits(), f)?;
    write!(f, ")")
  }
}

impl Octal for m128d {
  /// Octal formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:o}", m128d::default());
  /// assert_eq!(&f, "(0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "(")?;
    Debug::fmt(&a[0].to_bits(), f)?;
    write!(f, ", ")?;
    Debug::fmt(&a[1].to_bits(), f)?;
    write!(f, ")")
  }
}
