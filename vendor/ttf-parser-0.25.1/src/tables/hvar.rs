//! A [Horizontal Metrics Variations Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/hvar) implementation.

use crate::delta_set::DeltaSetIndexMap;
use crate::parser::{Offset, Offset32, Stream};
use crate::var_store::ItemVariationStore;
use crate::{GlyphId, NormalizedCoordinate};

/// A [Horizontal Metrics Variations Table](
/// https://docs.microsoft.com/en-us/typography/opentype/spec/hvar).
#[derive(Clone, Copy)]
pub struct Table<'a> {
    data: &'a [u8],
    variation_store: ItemVariationStore<'a>,
    advance_width_mapping_offset: Option<Offset32>,
    lsb_mapping_offset: Option<Offset32>,
    rsb_mapping_offset: Option<Offset32>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);

        let version = s.read::<u32>()?;
        if version != 0x00010000 {
            return None;
        }

        let variation_store_offset = s.read::<Offset32>()?;
        let var_store_s = Stream::new_at(data, variation_store_offset.to_usize())?;
        let variation_store = ItemVariationStore::parse(var_store_s)?;

        Some(Table {
            data,
            variation_store,
            advance_width_mapping_offset: s.read::<Option<Offset32>>()?,
            lsb_mapping_offset: s.read::<Option<Offset32>>()?,
            rsb_mapping_offset: s.read::<Option<Offset32>>()?,
        })
    }

    /// Returns the advance width offset for a glyph.
    #[inline]
    pub fn advance_offset(
        &self,
        glyph_id: GlyphId,
        coordinates: &[NormalizedCoordinate],
    ) -> Option<f32> {
        let (outer_idx, inner_idx) = if let Some(offset) = self.advance_width_mapping_offset {
            DeltaSetIndexMap::new(self.data.get(offset.to_usize()..)?).map(glyph_id.0 as u32)?
        } else {
            // 'If there is no delta-set index mapping table for advance widths,
            // then glyph IDs implicitly provide the indices:
            // for a given glyph ID, the delta-set outer-level index is zero,
            // and the glyph ID is the delta-set inner-level index.'
            (0, glyph_id.0)
        };

        self.variation_store
            .parse_delta(outer_idx, inner_idx, coordinates)
    }

    /// Returns the left side bearing offset for a glyph.
    #[inline]
    pub fn left_side_bearing_offset(
        &self,
        glyph_id: GlyphId,
        coordinates: &[NormalizedCoordinate],
    ) -> Option<f32> {
        let set_data = self.data.get(self.lsb_mapping_offset?.to_usize()..)?;
        self.side_bearing_offset(glyph_id, coordinates, set_data)
    }

    /// Returns the right side bearing offset for a glyph.
    #[inline]
    pub fn right_side_bearing_offset(
        &self,
        glyph_id: GlyphId,
        coordinates: &[NormalizedCoordinate],
    ) -> Option<f32> {
        let set_data = self.data.get(self.rsb_mapping_offset?.to_usize()..)?;
        self.side_bearing_offset(glyph_id, coordinates, set_data)
    }

    fn side_bearing_offset(
        &self,
        glyph_id: GlyphId,
        coordinates: &[NormalizedCoordinate],
        set_data: &[u8],
    ) -> Option<f32> {
        let (outer_idx, inner_idx) = DeltaSetIndexMap::new(set_data).map(glyph_id.0 as u32)?;
        self.variation_store
            .parse_delta(outer_idx, inner_idx, coordinates)
    }
}

impl core::fmt::Debug for Table<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Table {{ ... }}")
    }
}
