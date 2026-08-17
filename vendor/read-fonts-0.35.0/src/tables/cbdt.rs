//! The [CBDT (Color Bitmap Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/cbdt) table

use super::bitmap::{BitmapData, BitmapLocation};

include!("../../generated/generated_cbdt.rs");

impl<'a> Cbdt<'a> {
    pub fn data(&self, location: &BitmapLocation) -> Result<BitmapData<'a>, ReadError> {
        super::bitmap::bitmap_data(self.offset_data(), location, true)
    }
}

#[cfg(test)]
mod tests {
    use super::super::bitmap::{BitmapDataFormat, SmallGlyphMetrics};
    use crate::{
        types::{GlyphId, GlyphId16},
        FontRef, TableProvider,
    };

    #[test]
    fn read_cblc_1_cbdt_17() {
        let font = FontRef::new(font_test_data::EMBEDDED_BITMAPS).unwrap();
        let cblc = font.cblc().unwrap();
        let cbdt = font.cbdt().unwrap();
        let size = &cblc.bitmap_sizes()[0];
        // Metrics for size at index 0
        assert!(
            size.hori.ascender() == 101
                && size.hori.descender() == -27
                && size.hori.width_max() == 136
                && size.vert.ascender() == 101
                && size.vert.descender() == -27
                && size.vert.width_max() == 136
                && size.start_glyph_index() == GlyphId16::new(4)
                && size.end_glyph_index() == GlyphId16::new(4)
                && size.ppem_x() == 109
                && size.ppem_y() == 109
                && size.bit_depth() == 32
        );
        let expected: &[(GlyphId, &[u8], SmallGlyphMetrics)] = &[(
            GlyphId::new(4),
            &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a],
            SmallGlyphMetrics {
                height: 128,
                width: 136,
                bearing_x: 0.into(),
                bearing_y: 101.into(),
                advance: 136,
            },
        )];
        for (gid, data, metrics) in expected {
            let location = size.location(cblc.offset_data(), *gid).unwrap();
            assert_eq!(location.format, 17);
            let bitmap_data = cbdt.data(&location).unwrap();
            let (img_fmt, img_data) = bitmap_data.content.extract_data();
            assert_eq!(img_fmt, BitmapDataFormat::Png);
            assert_eq!(img_data, *data);
            assert_eq!(bitmap_data.extract_small_metrics(), metrics);
        }
    }

    #[test]
    fn sparse_glyph_ids() {
        let font = FontRef::new(font_test_data::CBDT).unwrap();
        let cblc = font.cblc().unwrap();
        let cbdt = font.cbdt().unwrap();
        let size = &cblc.bitmap_sizes()[0];
        // this font has a sparse set with gid 1 missing
        for gid in 0..=3 {
            let location = size
                .location(cblc.offset_data(), GlyphId::new(gid))
                .unwrap();
            if gid == 1 {
                assert!(
                    cbdt.data(&location).is_err(),
                    "expected bitmap for {gid} to be empty"
                );
            } else {
                assert!(
                    cbdt.data(&location).is_ok(),
                    "expected bitmap for {gid} to be present"
                );
            }
        }
    }
}
