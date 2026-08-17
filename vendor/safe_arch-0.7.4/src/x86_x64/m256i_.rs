//! This module is for the `m256i` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 256-bit AVX register of integer data.
///
/// * The exact layout to view the type as depends on the operation used.
/// * `From` and `Into` impls are provided for all the relevant signed integer
///   array types.
/// * Formatting impls print as four `i32` values just because they have to pick
///   something. If you want an alternative you can turn it into an array and
///   print as you like.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m256i(pub __m256i);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for m256i {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for m256i {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<__m256i> for m256i {}

impl Clone for m256i {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m256i {}

impl Default for m256i {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

// 8-bit

impl From<[i8; 32]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i8; 32]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [i8; 32] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

impl From<[u8; 32]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [u8; 32]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [u8; 32] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// 16-bit

impl From<[i16; 16]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i16; 16]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [i16; 16] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

impl From<[u16; 16]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [u16; 16]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [u16; 16] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// 32-bit

impl From<[i32; 8]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i32; 8]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [i32; 8] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

impl From<[u32; 8]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [u32; 8]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [u32; 8] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// 64-bit

impl From<[i64; 4]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i64; 4]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [i64; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

impl From<[u64; 4]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [u64; 4]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m256i> for [u64; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// 256-bit

impl From<[i128; 2]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(i: [i128; 2]) -> Self {
    unsafe { core::mem::transmute(i) }
  }
}

impl From<m256i> for [i128; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

impl From<[u128; 2]> for m256i {
  #[must_use]
  #[inline(always)]
  fn from(u: [u128; 2]) -> Self {
    unsafe { core::mem::transmute(u) }
  }
}

impl From<m256i> for [u128; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: m256i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m256i {
  /// Debug formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:?}", m256i::default());
  /// assert_eq!(&f, "m256i(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "m256i(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Display for m256i {
  /// Display formats each `i32`, and leaves the type name off of the font.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{}", m256i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Display::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Binary for m256i {
  /// Binary formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:b}", m256i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl LowerExp for m256i {
  /// LowerExp formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:e}", m256i::default());
  /// assert_eq!(&f, "(0e0, 0e0, 0e0, 0e0, 0e0, 0e0, 0e0, 0e0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerExp::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl UpperExp for m256i {
  /// UpperExp formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:E}", m256i::default());
  /// assert_eq!(&f, "(0E0, 0E0, 0E0, 0E0, 0E0, 0E0, 0E0, 0E0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperExp::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl LowerHex for m256i {
  /// LowerHex formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:x}", m256i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl UpperHex for m256i {
  /// UpperHex formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:X}", m256i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Octal for m256i {
  /// Octal formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:o}", m256i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0, 0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 8]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(int, f)?;
    }
    write!(f, ")")
  }
}
