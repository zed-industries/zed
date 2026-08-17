//! The [VVAR (Vertical Metrics Variation)](https://docs.microsoft.com/en-us/typography/opentype/spec/vvar) table

use super::variations::{self, DeltaSetIndexMap, ItemVariationStore};

include!("../../generated/generated_vvar.rs");

impl Vvar<'_> {
    /// Returns the advance height delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn advance_height_delta(
        &self,
        glyph_id: GlyphId,
        coords: &[F2Dot14],
    ) -> Result<Fixed, ReadError> {
        variations::advance_delta(
            self.advance_height_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }

    /// Returns the top side bearing delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn tsb_delta(&self, glyph_id: GlyphId, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        variations::item_delta(
            self.tsb_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }

    /// Returns the bottom side bearing delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn bsb_delta(&self, glyph_id: GlyphId, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        variations::item_delta(
            self.bsb_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }

    /// Returns the vertical origin delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn v_org_delta(&self, glyph_id: GlyphId, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        variations::item_delta(
            self.v_org_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }
}
