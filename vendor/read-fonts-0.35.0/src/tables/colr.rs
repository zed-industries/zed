//! The [COLR](https://docs.microsoft.com/en-us/typography/opentype/spec/colr) table

#[cfg(feature = "std")]
mod closure;

use super::variations::{DeltaSetIndexMap, ItemVariationStore};

include!("../../generated/generated_colr.rs");

/// Unique paint identifier used for detecting cycles in the paint graph.
pub type PaintId = usize;

impl<'a> Colr<'a> {
    /// Returns the COLRv0 base glyph for the given glyph identifier.
    ///
    /// The return value is a range of layer indices that can be passed to
    /// [`v0_layer`](Self::v0_layer) to retrieve the layer glyph identifiers
    /// and palette color indices.
    pub fn v0_base_glyph(&self, glyph_id: GlyphId) -> Result<Option<Range<usize>>, ReadError> {
        let records = self.base_glyph_records().ok_or(ReadError::NullOffset)??;
        let Ok(glyph_id) = glyph_id.try_into() else {
            return Ok(None);
        };
        let record = match records.binary_search_by(|rec| rec.glyph_id().cmp(&glyph_id)) {
            Ok(ix) => &records[ix],
            _ => return Ok(None),
        };
        let start = record.first_layer_index() as usize;
        let end = start + record.num_layers() as usize;
        Ok(Some(start..end))
    }

    /// Returns the COLRv0 layer at the given index.
    ///
    /// The layer is represented by a tuple containing the glyph identifier of
    /// the associated outline and the palette color index.
    pub fn v0_layer(&self, index: usize) -> Result<(GlyphId16, u16), ReadError> {
        let layers = self.layer_records().ok_or(ReadError::NullOffset)??;
        let layer = layers.get(index).ok_or(ReadError::OutOfBounds)?;
        Ok((layer.glyph_id(), layer.palette_index()))
    }

    /// Returns the COLRv1 base glyph for the given glyph identifier.
    ///
    /// The second value in the tuple is a unique identifier for the paint that
    /// may be used to detect recursion in the paint graph.
    pub fn v1_base_glyph(
        &self,
        glyph_id: GlyphId,
    ) -> Result<Option<(Paint<'a>, PaintId)>, ReadError> {
        let Ok(glyph_id) = glyph_id.try_into() else {
            return Ok(None);
        };
        let list = self.base_glyph_list().ok_or(ReadError::NullOffset)??;
        let records = list.base_glyph_paint_records();
        let record = match records.binary_search_by(|rec| rec.glyph_id().cmp(&glyph_id)) {
            Ok(ix) => &records[ix],
            _ => return Ok(None),
        };
        let offset_data = list.offset_data();
        // Use the address of the paint as an identifier for the recursion
        // blacklist.
        let id = record.paint_offset().to_u32() as usize + offset_data.as_ref().as_ptr() as usize;
        Ok(Some((record.paint(offset_data)?, id)))
    }

    /// Returns the COLRv1 layer at the given index.
    ///
    /// The second value in the tuple is a unique identifier for the paint that
    /// may be used to detect recursion in the paint graph.
    pub fn v1_layer(&self, index: usize) -> Result<(Paint<'a>, PaintId), ReadError> {
        let list = self.layer_list().ok_or(ReadError::NullOffset)??;
        let offset = list
            .paint_offsets()
            .get(index)
            .ok_or(ReadError::OutOfBounds)?
            .get();
        let offset_data = list.offset_data();
        // Use the address of the paint as an identifier for the recursion
        // blacklist.
        let id = offset.to_u32() as usize + offset_data.as_ref().as_ptr() as usize;
        Ok((offset.resolve(offset_data)?, id))
    }

    /// Returns the COLRv1 clip box for the given glyph identifier.
    pub fn v1_clip_box(&self, glyph_id: GlyphId) -> Result<Option<ClipBox<'a>>, ReadError> {
        use core::cmp::Ordering;
        let Ok(glyph_id): Result<GlyphId16, _> = glyph_id.try_into() else {
            return Ok(None);
        };
        let list = self.clip_list().ok_or(ReadError::NullOffset)??;
        let clips = list.clips();
        let clip = match clips.binary_search_by(|clip| {
            if glyph_id < clip.start_glyph_id() {
                Ordering::Greater
            } else if glyph_id > clip.end_glyph_id() {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(ix) => &clips[ix],
            _ => return Ok(None),
        };
        Ok(Some(clip.clip_box(list.offset_data())?))
    }
}
