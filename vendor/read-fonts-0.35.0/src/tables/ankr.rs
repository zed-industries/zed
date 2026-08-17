//! The [anchor point](https://developer.apple.com/fonts/TrueType-Reference-Manual/RM06/Chap6ankr.html) table.

use super::aat::LookupU16;

include!("../../generated/generated_ankr.rs");

impl<'a> Ankr<'a> {
    /// Returns the set of anchor points for the given glyph.
    pub fn anchor_points(&self, glyph_id: GlyphId) -> Result<&'a [AnchorPoint], ReadError> {
        let glyph_id: GlyphId16 = glyph_id.try_into().map_err(|_| ReadError::OutOfBounds)?;
        let entry_offset = self.lookup_table()?.value(glyph_id.to_u16())?;
        let full_offset = (self.glyph_data_table_offset() as usize)
            .checked_add(entry_offset as usize)
            .ok_or(ReadError::OutOfBounds)?;
        let data = self
            .offset_data()
            .split_off(full_offset)
            .ok_or(ReadError::OutOfBounds)?;
        Ok(GlyphDataEntry::read(data)?.anchor_points())
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use super::*;

    #[test]
    fn anchor_points() {
        let mut buf = BeBuffer::new();
        // lookup table (glyph_id -> offset)
        #[rustfmt::skip]
        let lookup = [
            0_u16, // format
            0, 8, 24, 32 // offsets to anchor points
        ];
        let lookup_size = lookup.len() as u32 * 2;
        // header
        buf = buf.extend([0u32, 0x0000000C, 12 + lookup_size]);
        buf = buf.extend(lookup);
        // glyph entry data
        #[rustfmt::skip]
        let expected_anchor_points: [&[(i16, i16)]; 4] = [
            &[(-20, 20)],
            &[(42, -10), (-200, 300), (i16::MIN, i16::MAX)],
            &[(0, 4)],
            &[(0, 0), (64, -64)],
        ];
        for entry in &expected_anchor_points {
            buf = buf
                .push(entry.len() as u32)
                .extend(entry.iter().flat_map(|x| [x.0, x.1]));
        }
        let ankr = Ankr::read(buf.data().into()).unwrap();
        let anchor_points = (0..4)
            .map(|gid| {
                let points = ankr.anchor_points(GlyphId::new(gid)).unwrap();
                points
                    .iter()
                    .map(|point| (point.x(), point.y()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(expected_anchor_points.iter().eq(anchor_points.iter()));
    }
}
