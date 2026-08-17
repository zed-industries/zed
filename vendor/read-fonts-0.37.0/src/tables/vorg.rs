//! The [VORG (Vertical Origin)](https://docs.microsoft.com/en-us/typography/opentype/spec/vorg) table.

include!("../../generated/generated_vorg.rs");

impl Vorg<'_> {
    /// Returns the y coordinate of the of the glyph's vertical origin.
    pub fn vertical_origin_y(&self, glyph_id: GlyphId) -> i16 {
        let gid = glyph_id.to_u32();
        let metrics = self.vert_origin_y_metrics();
        match metrics.binary_search_by(|rec| rec.glyph_index().to_u32().cmp(&gid)) {
            Ok(ix) => metrics
                .get(ix)
                .map(|metric| metric.vert_origin_y())
                .unwrap_or_default(),
            _ => self.default_vert_origin_y(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontRef, TableProvider};

    #[test]
    fn vertical_origins() {
        let font = FontRef::new(font_test_data::VORG).unwrap();
        let vorg = font.vorg().unwrap();
        // Glyphs 1 and 3 have entries while 0 and 2 use the default value
        // of 880
        assert_eq!(vorg.vertical_origin_y(GlyphId::new(0)), 880);
        assert_eq!(vorg.vertical_origin_y(GlyphId::new(1)), 867);
        assert_eq!(vorg.vertical_origin_y(GlyphId::new(2)), 880);
        assert_eq!(vorg.vertical_origin_y(GlyphId::new(3)), 824);
    }
}
