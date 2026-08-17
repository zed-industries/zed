//! Glyph Identifiers.
//!
//! Although these are treated as u16s in the spec, we choose to represent them
//! as a distinct type.

/// A 16-bit glyph identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct GlyphId16(u16);

impl GlyphId16 {
    /// The identifier reserved for unknown glyphs
    pub const NOTDEF: GlyphId16 = GlyphId16(0);

    /// Construct a new `GlyphId16`.
    pub const fn new(raw: u16) -> Self {
        GlyphId16(raw)
    }

    /// The identifier as a u16.
    pub const fn to_u16(self) -> u16 {
        self.0
    }

    /// The identifier as a u32.
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl Default for GlyphId16 {
    fn default() -> Self {
        GlyphId16::NOTDEF
    }
}

impl From<u16> for GlyphId16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<GlyphId16> for usize {
    fn from(value: GlyphId16) -> Self {
        value.0 as usize
    }
}

impl std::fmt::Display for GlyphId16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GID_{}", self.0)
    }
}

impl From<GlyphId16> for u32 {
    fn from(value: GlyphId16) -> u32 {
        value.to_u32()
    }
}

crate::newtype_scalar!(GlyphId16, [u8; 2]);

/// A 32-bit glyph identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct GlyphId(u32);

impl GlyphId {
    /// The identifier reserved for unknown glyphs.
    pub const NOTDEF: GlyphId = GlyphId(0);

    /// Construct a new `GlyphId`.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// The identifier as a u32.
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl Default for GlyphId {
    fn default() -> Self {
        GlyphId::NOTDEF
    }
}

impl From<u16> for GlyphId {
    fn from(value: u16) -> Self {
        Self(value as u32)
    }
}

impl From<u32> for GlyphId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for GlyphId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GID_{}", self.0)
    }
}

impl From<GlyphId> for u32 {
    fn from(value: GlyphId) -> u32 {
        value.to_u32()
    }
}

impl From<GlyphId16> for GlyphId {
    fn from(value: GlyphId16) -> GlyphId {
        Self(value.to_u32())
    }
}

impl PartialEq<GlyphId16> for GlyphId {
    fn eq(&self, other: &GlyphId16) -> bool {
        self.0 == other.0 as u32
    }
}

impl PartialOrd<GlyphId16> for GlyphId {
    fn partial_cmp(&self, other: &GlyphId16) -> Option<core::cmp::Ordering> {
        Some(self.0.cmp(&(other.0 as u32)))
    }
}

impl PartialEq<GlyphId> for GlyphId16 {
    fn eq(&self, other: &GlyphId) -> bool {
        self.0 as u32 == other.0
    }
}

impl PartialOrd<GlyphId> for GlyphId16 {
    fn partial_cmp(&self, other: &GlyphId) -> Option<core::cmp::Ordering> {
        Some((self.0 as u32).cmp(&other.0))
    }
}

impl TryFrom<GlyphId> for GlyphId16 {
    type Error = TryFromGlyphIdError;

    fn try_from(value: GlyphId) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .0
                .try_into()
                .map_err(|_| TryFromGlyphIdError(value.0))?,
        ))
    }
}

/// The error type returned when a glyph identifier conversion fails.
#[derive(Debug)]
pub struct TryFromGlyphIdError(u32);

impl core::fmt::Display for TryFromGlyphIdError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "glyph identifier {} too large for conversion", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromGlyphIdError {}
