//! The [HVAR (Horizontal Metrics Variation)](https://docs.microsoft.com/en-us/typography/opentype/spec/hvar) table

use super::variations::{self, DeltaSetIndexMap, ItemVariationStore};

include!("../../generated/generated_hvar.rs");

impl Hvar<'_> {
    /// Returns the advance width delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn advance_width_delta(
        &self,
        glyph_id: GlyphId,
        coords: &[F2Dot14],
    ) -> Result<Fixed, ReadError> {
        variations::advance_delta(
            self.advance_width_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }

    /// Returns the left side bearing delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn lsb_delta(&self, glyph_id: GlyphId, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        variations::item_delta(
            self.lsb_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }

    /// Returns the left side bearing delta for the specified glyph identifier and
    /// normalized variation coordinates.
    pub fn rsb_delta(&self, glyph_id: GlyphId, coords: &[F2Dot14]) -> Result<Fixed, ReadError> {
        variations::item_delta(
            self.rsb_mapping(),
            self.item_variation_store(),
            glyph_id,
            coords,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{tables::variations::DeltaSetIndexMap, FontRef, TableProvider};
    use types::{F2Dot14, Fixed, GlyphId};

    #[test]
    fn advance_deltas() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let hvar = font.hvar().unwrap();
        let gid_a = GlyphId::new(1);
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(-1.0)])
                .unwrap(),
            Fixed::from_f64(-113.0)
        );
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(-0.75)])
                .unwrap(),
            Fixed::from_f64(-85.0)
        );
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(-0.5)])
                .unwrap(),
            Fixed::from_f64(-56.0)
        );
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(0.0)])
                .unwrap(),
            Fixed::from_f64(0.0)
        );
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(0.5)])
                .unwrap(),
            Fixed::from_f64(30.0)
        );
        assert_eq!(
            hvar.advance_width_delta(gid_a, &[F2Dot14::from_f32(1.0)])
                .unwrap(),
            Fixed::from_f64(59.0)
        );
    }

    #[test]
    fn advance_deltas_from_hvar_with_truncated_adv_index_map() {
        let font = FontRef::new(font_test_data::HVAR_WITH_TRUNCATED_ADVANCE_INDEX_MAP).unwrap();
        let maxp = font.maxp().unwrap();
        let num_glyphs = maxp.num_glyphs();
        let hvar = font.hvar().unwrap();
        let Ok(DeltaSetIndexMap::Format0(adv_index_map)) = hvar.advance_width_mapping().unwrap()
        else {
            panic!("Expected DeltaSetIndexMap::Format0 for hvar.advance_width_mapping()");
        };
        assert!(adv_index_map.map_count() < num_glyphs);
        assert_eq!(num_glyphs, 24);
        assert_eq!(adv_index_map.map_count(), 15);
        let last_mapped_gid = adv_index_map.map_count() - 1;
        // We expect the last 10 glyphs to have the same advance width delta as the last mapped glyph.
        // Crucially, hvar.advance_width_delta() should not return OutOfBounds for these glyphs.
        for idx in last_mapped_gid..num_glyphs {
            let gid = GlyphId::new(idx as _);
            assert_eq!(
                hvar.advance_width_delta(gid, &[F2Dot14::from_f32(1.0)])
                    .unwrap(),
                Fixed::from_f64(100.0)
            );
        }
    }
}
