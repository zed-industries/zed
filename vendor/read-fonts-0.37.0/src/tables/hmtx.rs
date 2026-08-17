//! The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table

include!("../../generated/generated_hmtx.rs");

impl Hmtx<'_> {
    /// Returns the advance width for the given glyph identifier.
    pub fn advance(&self, glyph_id: GlyphId) -> Option<u16> {
        advance(self.h_metrics(), glyph_id)
    }

    /// Returns the left side bearing for the given glyph identifier.
    pub fn side_bearing(&self, glyph_id: GlyphId) -> Option<i16> {
        side_bearing(self.h_metrics(), self.left_side_bearings(), glyph_id)
    }
}

pub(super) fn advance(metrics: &[LongMetric], glyph_id: GlyphId) -> Option<u16> {
    metrics
        .get(glyph_id.to_u32() as usize)
        .or_else(|| metrics.last())
        .map(|metric| metric.advance())
}

pub(super) fn side_bearing(
    metrics: &[LongMetric],
    side_bearings: &[BigEndian<i16>],
    glyph_id: GlyphId,
) -> Option<i16> {
    let ix = glyph_id.to_u32() as usize;
    metrics
        .get(ix)
        .map(|metric| metric.side_bearing())
        .or_else(|| {
            side_bearings
                .get(ix.saturating_sub(metrics.len()))
                .map(|sb| sb.get())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, TableProvider};
    use font_test_data::{be_buffer, bebuffer::BeBuffer};

    /// Test case where "long metric" array is short
    #[test]
    fn trimmed_advances() {
        let font = FontRef::new(font_test_data::CBDT).unwrap();
        let hmtx = font.hmtx().unwrap();
        assert!(
            !hmtx.left_side_bearings().is_empty(),
            "if this fails then the test is no longer accurate"
        );
        let expected_lsbs = [100, 0, 100, 0];
        for (i, lsb) in expected_lsbs.into_iter().enumerate() {
            let gid = GlyphId::new(i as _);
            // All glyphs have 800 advance width
            assert_eq!(hmtx.advance(gid), Some(800));
            assert_eq!(hmtx.side_bearing(gid), Some(lsb));
        }
    }

    #[test]
    fn missing_left_side_bearings() {
        let hmtx_data = be_buffer! {
            500u16, 50u16, // advance width + lsb, glyph 0
            600u16, 60u16 // advance width + lsb, glyph 1
        };

        let hmtx = Hmtx::read(hmtx_data.data().into(), 2).unwrap();

        assert_eq!(hmtx.advance(GlyphId::new(0)), Some(500));
        assert_eq!(hmtx.side_bearing(GlyphId::new(0)), Some(50));

        assert_eq!(hmtx.advance(GlyphId::new(1)), Some(600));
        assert_eq!(hmtx.side_bearing(GlyphId::new(1)), Some(60));

        assert_eq!(hmtx.advance(GlyphId::new(2)), Some(600));
        assert_eq!(hmtx.side_bearing(GlyphId::new(2)), None);

        assert_eq!(hmtx.advance(GlyphId::new(3)), Some(600));
        assert_eq!(hmtx.side_bearing(GlyphId::new(3)), None);
    }

    #[test]
    fn metrics() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let hmtx = font.hmtx().unwrap();
        let expected = [(908, 100), (1336, 29), (1336, 29), (633, 57)];
        for (i, (advance, lsb)) in expected.into_iter().enumerate() {
            let gid = GlyphId::new(i as _);
            assert_eq!(hmtx.advance(gid), Some(advance));
            assert_eq!(hmtx.side_bearing(gid), Some(lsb));
        }
    }
}
