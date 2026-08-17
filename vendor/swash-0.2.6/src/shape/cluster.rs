/*!
Glyph cluster modeling-- output from the shaper.
*/

use super::buffer::MARK_ATTACH;
use crate::text::cluster::{ClusterInfo, SourceRange, UserData};
use crate::GlyphId;

/// Information for a glyph.
#[derive(Copy, Clone, Default, Debug)]
pub struct GlyphInfo(pub u16);

impl GlyphInfo {
    /// Returns true if the glyph is an attached mark.
    pub fn is_mark(self) -> bool {
        self.0 & MARK_ATTACH != 0
    }
}

/// Glyph identifier and positioning information as a result of shaping.
#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    /// Glyph identifier.
    pub id: GlyphId,
    /// Glyph flags.
    pub info: GlyphInfo,
    /// Horizontal offset.
    pub x: f32,
    /// Vertical offset.
    pub y: f32,
    /// Advance width or height.
    pub advance: f32,
    /// Arbitrary user data.
    pub data: UserData,
}

/// Collection of glyphs and associated metadata corresponding to one or
/// more source clusters.
#[derive(Copy, Clone, Debug)]
pub struct GlyphCluster<'a> {
    /// Full source range of the cluster in original units supplied to the
    /// shaper.
    pub source: SourceRange,
    /// Information about the textual content of the cluster.
    pub info: ClusterInfo,
    /// Sequence of glyphs for the cluster. May be empty for clusters whose
    /// source consisted entirely of control characters.
    pub glyphs: &'a [Glyph],
    /// If the cluster is a ligature, this contains the source range
    /// of each ligature component. Empty otherwise.
    pub components: &'a [SourceRange],
    /// Arbitrary user data-- taken from the initial character of the cluster.
    pub data: UserData,
}

impl<'a> GlyphCluster<'a> {
    /// Returns true if the cluster is empty. Empty clusters still represent
    /// characters in the source text, but contain no glyphs. This will be
    /// true, for example, with newline sequences (\n or \r\n) as well as other
    /// control characters.
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Returns true if the cluster contains a single glyph. Note that a simple
    /// cluster can also be a ligature.
    pub fn is_simple(&self) -> bool {
        self.glyphs.len() == 1
    }

    /// Returns true if the cluster corresponds to multiple source clusters.
    /// Note that a ligature cluster can also be complex.
    pub fn is_ligature(&self) -> bool {
        !self.components.is_empty()
    }

    /// Returns true if the cluster is complex-- that is if it contains more
    /// than one glyph. This will be true for clusters containing marks and is
    /// also commonly true for syllabic languages such as those in the Indic
    /// family.
    pub fn is_complex(&self) -> bool {
        self.glyphs.len() > 1
    }

    /// Computes the full advance width or height of the cluster.
    pub fn advance(&self) -> f32 {
        let mut advance = 0.;
        for g in self.glyphs {
            advance += g.advance;
        }
        advance
    }
}
