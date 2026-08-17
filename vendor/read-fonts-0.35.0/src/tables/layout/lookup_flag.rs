//! The lookup flag type.
//!
//! This is kind-of-but-not-quite-exactly a bit enumeration, and so we implement
//! it manually.

use core::ops::{BitOr, BitOrAssign};

/// The [LookupFlag](https://learn.microsoft.com/en-us/typography/opentype/spec/chapter2#lookupFlag) bit enumeration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LookupFlag(u16);

//NOTE: this impl has the potential to make garbage if used on two lookupflag
//instances which have different mark attachment masks set, but as that field
//is not really used in compilation, which is where this impl will be helpful,
//the risk that this is the source of an actual bug seems very low,
impl BitOr for LookupFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for LookupFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl LookupFlag {
    /// This bit relates only to the correct processing of GPOS type 3 (cursive attachment) lookups
    ///
    /// When this bit is set, the last glyph in a given sequence to which the cursive
    /// attachment lookup is applied, will be positioned on the baseline.
    pub const RIGHT_TO_LEFT: Self = LookupFlag(0x0001);
    /// If set, skips over base glyphs
    pub const IGNORE_BASE_GLYPHS: Self = LookupFlag(0x002);
    /// If set, skips over ligatures
    pub const IGNORE_LIGATURES: Self = LookupFlag(0x004);
    /// If set, skips over all combining marks
    pub const IGNORE_MARKS: Self = LookupFlag(0x008);
    /// If set, indicates that the lookup table structure is followed by a MarkFilteringSet field.
    ///
    /// The layout engine skips over all mark glyphs not in the mark filtering set indicated.
    pub const USE_MARK_FILTERING_SET: Self = LookupFlag(0x010);

    // union of all flags, above
    const FLAG_MASK: Self = LookupFlag(0x1F);

    /// Return new, empty flags
    pub fn empty() -> Self {
        Self(0)
    }

    /// Construct a LookupFlag from a raw value, discarding invalid bits
    pub fn from_bits_truncate(bits: u16) -> Self {
        const VALID_BITS: u16 = !0x00E0;
        Self(bits & VALID_BITS)
    }

    /// Raw transmutation to u16.
    pub fn to_bits(self) -> u16 {
        self.0
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        // only count flag bits
        let other = other.0 & Self::FLAG_MASK.0;
        (self.0 & other) == other
    }

    /// If not zero, skips over all marks of attachment type different from specified.
    pub fn mark_attachment_class(self) -> Option<u16> {
        let val = self.0 & 0xff00;
        if val == 0 {
            None
        } else {
            Some(val >> 8)
        }
    }

    /// If not zero, skips over all marks of attachment type different from specified.
    pub fn set_mark_attachment_class(&mut self, val: u16) {
        let val = (val & 0xff) << 8;
        self.0 = (self.0 & 0xff) | val;
    }
}

impl types::Scalar for LookupFlag {
    type Raw = <u16 as types::Scalar>::Raw;
    fn to_raw(self) -> Self::Raw {
        self.0.to_raw()
    }
    fn from_raw(raw: Self::Raw) -> Self {
        let t = <u16>::from_raw(raw);
        Self(t)
    }
}
