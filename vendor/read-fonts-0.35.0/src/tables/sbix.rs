//! The [sbix (Standard Bitmap Graphics)](https://docs.microsoft.com/en-us/typography/opentype/spec/sbix) table

include!("../../generated/generated_sbix.rs");

impl<'a> Strike<'a> {
    pub fn glyph_data(&self, glyph_id: GlyphId) -> Result<Option<GlyphData<'a>>, ReadError> {
        let offsets = self.glyph_data_offsets();
        let start_ix = glyph_id.to_u32() as usize;
        let start = offsets.get(start_ix).ok_or(ReadError::OutOfBounds)?.get() as usize;
        let end = offsets
            .get(start_ix + 1)
            .ok_or(ReadError::OutOfBounds)?
            .get() as usize;
        if start == end {
            // Empty glyphs are okay
            return Ok(None);
        }
        let data = self
            .offset_data()
            .slice(start..end)
            .ok_or(ReadError::OutOfBounds)?;
        Ok(Some(GlyphData::read(data)?))
    }
}

#[cfg(test)]
mod tests {
    use font_test_data::bebuffer::BeBuffer;

    use crate::tables::sbix::Sbix;

    #[test]
    fn sbix_strikes_count_overflow_table() {
        // Contains an invalid `num_strikes` values which would move the cursor outside the able.
        // See https://issues.chromium.org/issues/347835680 for the ClusterFuzz report.
        // Failure only reproduces on 32-bit, for example, run with:
        // cargo test --target=i686-unknown-linux-gnu "sbix_strikes_count_overflow_table"
        let sbix = BeBuffer::new()
            .push(1u16) // version
            .push(0u16) // flags
            .push(u32::MAX); // num_strikes

        let table = Sbix::read(sbix.data().into(), 5);
        // Must not panic with "attempt to multiply with overflow".
        assert!(table.is_err());
    }
}
