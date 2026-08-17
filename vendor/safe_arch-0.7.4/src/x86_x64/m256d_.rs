//! This module is for the `m256d` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 256-bit AVX register of four `f64` values.
///
/// * This is _very similar to_ having `[f64; 4]`. The main difference is that
///   it's aligned to 32 instead of just 4, and of course you can perform
///   various intrinsic operations on it.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m256d(pub __m256d);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for m256d {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for m256d {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<__m256d> for m256d {}

impl m256d {
  /// Transmutes the `m256d` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f64; 4] {
    self.into()
  }

  /// Transmutes an array into `m256d`.
  ///
  /// Same as `m256d::from(arr)`, it just lets you be more explicit about what's
  /// happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f64; 4]) -> Self {
    f.into()
  }

  //

  /// Converts into the bit patterns of these doubles (`[u64;4]`).
  ///
  /// Like [`f64::to_bits`](f64::to_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn to_bits(self) -> [u64; 4] {
    unsafe { core::mem::transmute(self) }
  }

  /// Converts from the bit patterns of these doubles (`[u64;4]`).
  ///
  /// Like [`f64::from_bits`](f64::from_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn from_bits(bits: [u64; 4]) -> Self {
    unsafe { core::mem::transmute(bits) }
  }
}

impl Clone for m256d {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m256d {}

impl Default for m256d {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[f64; 4]> for m256d {
  #[must_use]
  #[inline(always)]
  fn from(arr: [f64; 4]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align8) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256d> for [f64; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256d) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m256d {
  /// Debug formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:?}", m256d::default());
  /// assert_eq!(&f, "m256d(0.0, 0.0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "m256d(")?;
    Debug::fmt(&a[0], f)?;
    write!(f, ", ")?;
    Debug::fmt(&a[1], f)?;
    write!(f, ")")
  }
}

impl Display for m256d {
  /// Display formats each double, and leaves the type name off of the font.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{}", m256d::default());
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

impl Binary for m256d {
  /// Binary formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:b}", m256d::default());
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

impl LowerExp for m256d {
  /// LowerExp formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:e}", m256d::default());
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

impl UpperExp for m256d {
  /// UpperExp formats each double.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:E}", m256d::default());
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

impl LowerHex for m256d {
  /// LowerHex formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:x}", m256d::default());
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

impl UpperHex for m256d {
  /// UpperHex formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:X}", m256d::default());
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

impl Octal for m256d {
  /// Octal formats each double's bit pattern (via [`f64::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:o}", m256d::default());
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
