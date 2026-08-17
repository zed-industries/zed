/*!
A complete [harfbuzz](https://github.com/harfbuzz/harfbuzz) shaping algorithm port to Rust.
*/

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod hb;

pub use ttf_parser;

pub use hb::buffer::hb_glyph_info_t as GlyphInfo;
pub use hb::buffer::{GlyphBuffer, GlyphPosition, UnicodeBuffer};
pub use hb::common::{script, Direction, Feature, Language, Script, Variation};
pub use hb::face::hb_font_t as Face;
pub use hb::ot_shape_plan::hb_ot_shape_plan_t as ShapePlan;
pub use hb::shape::{shape, shape_with_plan};

bitflags::bitflags! {
    /// Flags for buffers.
    #[derive(Default, Debug, Clone, Copy)]
    pub struct BufferFlags: u32 {
        /// Indicates that special handling of the beginning of text paragraph can be applied to this buffer. Should usually be set, unless you are passing to the buffer only part of the text without the full context.
        const BEGINNING_OF_TEXT             = 0x00000001;
        /// Indicates that special handling of the end of text paragraph can be applied to this buffer, similar to [`BufferFlags::BEGINNING_OF_TEXT`].
        const END_OF_TEXT                   = 0x00000002;
        /// Indicates that characters with `Default_Ignorable` Unicode property should use the corresponding glyph from the font, instead of hiding them (done by replacing them with the space glyph and zeroing the advance width.) This flag takes precedence over [`BufferFlags::REMOVE_DEFAULT_IGNORABLES`].
        const PRESERVE_DEFAULT_IGNORABLES   = 0x00000004;
        /// Indicates that characters with `Default_Ignorable` Unicode property should be removed from glyph string instead of hiding them (done by replacing them with the space glyph and zeroing the advance width.) [`BufferFlags::PRESERVE_DEFAULT_IGNORABLES`] takes precedence over this flag.
        const REMOVE_DEFAULT_IGNORABLES     = 0x00000008;
        /// Indicates that a dotted circle should not be inserted in the rendering of incorrect character sequences (such as `<0905 093E>`).
        const DO_NOT_INSERT_DOTTED_CIRCLE   = 0x00000010;
        /// Indicates that the shape() call and its variants should perform various verification processes on the results of the shaping operation on the buffer. If the verification fails, then either a buffer message is sent, if a message handler is installed on the buffer, or a message is written to standard error. In either case, the shaping result might be modified to show the failed output.
        const VERIFY                        = 0x00000020;
        /// Indicates that the `UNSAFE_TO_CONCAT` glyph-flag should be produced by the shaper. By default it will not be produced since it incurs a cost.
        const PRODUCE_UNSAFE_TO_CONCAT      = 0x00000040;
        /// Indicates that the `SAFE_TO_INSERT_TATWEEL` glyph-flag should be produced by the shaper. By default it will not be produced.
        const PRODUCE_SAFE_TO_INSERT_TATWEEL      = 0x00000040;
        /// All currently defined flags
        const DEFINED = 0x000000FF;
    }
}

/// A cluster level.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BufferClusterLevel {
    MonotoneGraphemes,
    MonotoneCharacters,
    Characters,
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
        const NO_CLUSTERS       = 0b00000001;
        /// Do not serialize glyph position information.
        const NO_POSITIONS      = 0b00000010;
        /// Do no serialize glyph name.
        const NO_GLYPH_NAMES    = 0b00000100;
        /// Serialize glyph extents.
        const GLYPH_EXTENTS     = 0b00001000;
        /// Serialize glyph flags.
        const GLYPH_FLAGS       = 0b00010000;
        /// Do not serialize glyph advances, glyph offsets will reflect absolute
        /// glyph positions.
        const NO_ADVANCES       = 0b00100000;
        /// All currently defined flags.
        const DEFINED = 0b00111111;
    }
}
