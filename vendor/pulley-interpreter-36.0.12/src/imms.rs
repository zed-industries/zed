//! Immediates.

use core::fmt;

/// A PC-relative offset.
///
/// This is relative to the start of this offset's containing instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PcRelOffset(i32);

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for PcRelOffset {
    fn arbitrary(_u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        // We can't possibly choose valid offsets for jumping to, so just use
        // zero as the least dangerous option. It is up to whoever is generating
        // arbitrary ops to clean this up.
        Ok(Self(0))
    }
}

impl From<i32> for PcRelOffset {
    #[inline]
    fn from(offset: i32) -> Self {
        PcRelOffset(offset)
    }
}

impl From<PcRelOffset> for i32 {
    #[inline]
    fn from(offset: PcRelOffset) -> Self {
        offset.0
    }
}

/// A 6-byte unsigned integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U6(u8);

impl U6 {
    /// Attempts to create a new `U6` from the provided byte
    pub fn new(val: u8) -> Option<U6> {
        if val << 2 >> 2 == val {
            Some(U6(val))
        } else {
            None
        }
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for U6 {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let byte = u.arbitrary::<u8>()?;
        Ok(U6(byte << 2 >> 2))
    }
}

impl From<U6> for u8 {
    #[inline]
    fn from(val: U6) -> Self {
        val.0
    }
}

impl fmt::Display for U6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}
