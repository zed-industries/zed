/*!
Rendered glyph image.
*/

use super::Source;
use alloc::vec::Vec;
use zeno::Placement;

/// Content of a scaled glyph image.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Content {
    /// 8-bit alpha mask.
    Mask,
    /// 32-bit RGBA subpixel mask.
    SubpixelMask,
    /// 32-bit RGBA bitmap.
    Color,
}

impl Default for Content {
    fn default() -> Self {
        Self::Mask
    }
}

/// Scaled glyph image.
#[derive(Clone, Default)]
pub struct Image {
    /// Source of the image.
    pub source: Source,
    /// Content of the image.
    pub content: Content,
    /// Offset and size of the image.
    pub placement: Placement,
    /// Raw image data.
    pub data: Vec<u8>,
}

impl Image {
    /// Creates a new empty scaled image.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the image to a default state.
    pub fn clear(&mut self) {
        self.source = Source::default();
        self.content = Content::default();
        self.placement = Placement::default();
        self.data.clear();
    }
}
