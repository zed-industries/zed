/*!
A complete [harfbuzz](https://github.com/harfbuzz/harfbuzz) shaping algorithm port to Rust.
*/

#![cfg_attr(not(feature = "std"), no_std)]
// Forbidding unsafe code only applies to the lib
// examples continue to use it, so this cannot be placed into Cargo.toml
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

mod hb;

#[cfg(feature = "std")]
pub(crate) type U32Set = read_fonts::collections::int_set::U32Set;
#[cfg(not(feature = "std"))]
mod digest_u32_set;
#[cfg(not(feature = "std"))]
pub(crate) type U32Set = digest_u32_set::DigestU32Set;

pub use read_fonts::{types::Tag, FontRef};

pub use hb::buffer::{GlyphBuffer, GlyphInfo, GlyphPosition, UnicodeBuffer};
pub use hb::common::{script, Direction, Feature, Language, Script, Variation};
pub use hb::face::{hb_font_t as Shaper, ShaperBuilder, ShaperData, ShaperInstance};
pub use hb::ot_shape_plan::{hb_ot_shape_plan_t as ShapePlan, ShapePlanKey};

/// Type alias for a normalized variation coordinate.
pub type NormalizedCoord = read_fonts::types::F2Dot14;

bitflags::bitflags! {
    /// Flags for buffers.
    #[derive(Default, Debug, Clone, Copy)]
    pub struct BufferFlags: u32 {
        /// Indicates that special handling of the beginning of text paragraph can be applied to this buffer. Should usually be set, unless you are passing to the buffer only part of the text without the full context.
        const BEGINNING_OF_TEXT             = 0x0000_0001;
        /// Indicates that special handling of the end of text paragraph can be applied to this buffer, similar to [`BufferFlags::BEGINNING_OF_TEXT`].
        const END_OF_TEXT                   = 0x0000_0002;
        /// Indicates that characters with `Default_Ignorable` Unicode property should use the corresponding glyph from the font, instead of hiding them (done by replacing them with the space glyph and zeroing the advance width.) This flag takes precedence over [`BufferFlags::REMOVE_DEFAULT_IGNORABLES`].
        const PRESERVE_DEFAULT_IGNORABLES   = 0x0000_0004;
        /// Indicates that characters with `Default_Ignorable` Unicode property should be removed from glyph string instead of hiding them (done by replacing them with the space glyph and zeroing the advance width.) [`BufferFlags::PRESERVE_DEFAULT_IGNORABLES`] takes precedence over this flag.
        const REMOVE_DEFAULT_IGNORABLES     = 0x0000_0008;
        /// Indicates that a dotted circle should not be inserted in the rendering of incorrect character sequences (such as `<0905 093E>`).
        const DO_NOT_INSERT_DOTTED_CIRCLE   = 0x0000_0010;
        /// Indicates that the shape() call and its variants should perform various verification processes on the results of the shaping operation on the buffer. If the verification fails, then either a buffer message is sent, if a message handler is installed on the buffer, or a message is written to standard error. In either case, the shaping result might be modified to show the failed output.
        const VERIFY                        = 0x0000_0020;
        /// Indicates that the `UNSAFE_TO_CONCAT` glyph-flag should be produced by the shaper. By default it will not be produced since it incurs a cost.
        const PRODUCE_UNSAFE_TO_CONCAT      = 0x0000_0040;
        /// Indicates that the `SAFE_TO_INSERT_TATWEEL` glyph-flag should be produced by the shaper. By default it will not be produced.
        const PRODUCE_SAFE_TO_INSERT_TATWEEL      = 0x0000_0080;
        /// All currently defined flags
        const DEFINED = 0x0000_00FF;
    }
}

/// A cluster level.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BufferClusterLevel {
    MonotoneGraphemes,
    MonotoneCharacters,
    Characters,
    Graphemes,
}

impl BufferClusterLevel {
    #[inline]
    fn new(level: u32) -> Self {
        match level {
            0 => Self::MonotoneGraphemes,
            1 => Self::MonotoneCharacters,
            2 => Self::Characters,
            3 => Self::Graphemes,
            _ => Self::MonotoneGraphemes,
        }
    }
    #[inline]
    fn is_monotone(self) -> bool {
        matches!(self, Self::MonotoneGraphemes | Self::MonotoneCharacters)
    }
    #[inline]
    fn is_graphemes(self) -> bool {
        matches!(self, Self::MonotoneGraphemes | Self::Graphemes)
    }
    #[inline]
    fn _is_characters(self) -> bool {
        matches!(self, Self::MonotoneCharacters | Self::Characters)
    }
}

impl Default for BufferClusterLevel {
    #[inline]
    fn default() -> Self {
        BufferClusterLevel::MonotoneGraphemes
    }
}

bitflags::bitflags! {
    /// Flags used for serialization with a `BufferSerializer`.
    #[derive(Default)]
    pub struct SerializeFlags: u8 {
        /// Do not serialize glyph cluster.
        const NO_CLUSTERS       = 0b0000_0001;
        /// Do not serialize glyph position information.
        const NO_POSITIONS      = 0b0000_0010;
        /// Do no serialize glyph name.
        const NO_GLYPH_NAMES    = 0b0000_0100;
        /// Serialize glyph extents.
        const GLYPH_EXTENTS     = 0b0000_1000;
        /// Serialize glyph flags.
        const GLYPH_FLAGS       = 0b0001_0000;
        /// Do not serialize glyph advances, glyph offsets will reflect absolute
        /// glyph positions.
        const NO_ADVANCES       = 0b0010_0000;
        /// All currently defined flags.
        const DEFINED = 0b0011_1111;
    }
}
