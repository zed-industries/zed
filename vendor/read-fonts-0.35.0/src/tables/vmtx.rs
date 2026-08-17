//! The [vmtx (Vertical Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/vmtx) table

use super::hmtx;
pub use super::hmtx::LongMetric;

include!("../../generated/generated_vmtx.rs");

impl Vmtx<'_> {
    /// Returns the advance height for the given glyph identifier.
    pub fn advance(&self, glyph_id: GlyphId) -> Option<u16> {
        hmtx::advance(self.v_metrics(), glyph_id)
    }

    /// Returns the top side bearing for the given glyph identifier.
    pub fn side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        hmtx::side_bearing(self.v_metrics(), self.top_side_bearings(), glyph_id)
    }
}
