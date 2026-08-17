//! The [EBDT (Embedded Bitmap Data)](https://docs.microsoft.com/en-us/typography/opentype/spec/ebdt) table

use super::bitmap::{BitmapData, BitmapLocation};

include!("../../generated/generated_ebdt.rs");

impl<'a> Ebdt<'a> {
    pub fn data(&self, location: &BitmapLocation) -> Result<BitmapData<'a>, ReadError> {
        super::bitmap::bitmap_data(self.offset_data(), location, false)
    }
}

#[cfg(test)]
mod tests {
    use super::super::bitmap::{
        BigGlyphMetrics, BitmapContent, BitmapData, BitmapDataFormat, BitmapMetrics,
        SmallGlyphMetrics,
    };
    use crate::{
        types::{GlyphId, GlyphId16},
        FontRef, TableProvider,
    };

    impl<'a> BitmapContent<'a> {
        pub(crate) fn extract_data(&self) -> (BitmapDataFormat, &'a [u8]) {
            match self {
                BitmapContent::Data(fmt, data) => (*fmt, *data),
                _ => panic!("expected data content"),
            }
        }
    }

    impl BitmapData<'_> {
        pub(crate) fn extract_small_metrics(&self) -> &SmallGlyphMetrics {
            match &self.metrics {
                BitmapMetrics::Small(small) => small,
                _ => panic!("expected small glyph metrics"),
            }
        }
    }

    #[test]
    fn read_eblc_3_ebdt_2() {
        let font = FontRef::new(font_test_data::EMBEDDED_BITMAPS).unwrap();
        let eblc = font.eblc().unwrap();
        let ebdt = font.ebdt().unwrap();
        let size = &eblc.bitmap_sizes()[0];
        // Metrics for size at index 0
        assert!(
            size.hori.ascender() == 6
                && size.hori.descender() == 2
                && size.hori.width_max() == 4
                && size.hori.max_before_bl() == 6
                && size.hori.min_after_bl() == -2
                && size.vert.ascender() == 6
                && size.vert.descender() == 2
                && size.start_glyph_index() == GlyphId16::new(1)
                && size.end_glyph_index() == GlyphId16::new(2)
                && size.ppem_x() == 7
                && size.ppem_y() == 7
                && size.bit_depth() == 1
        );
        // Bit aligned formats in this strike:
        let expected: &[(GlyphId, &[u8], SmallGlyphMetrics)] = &[
            (
                GlyphId::new(1),
                &[0xee, 0xae, 0xea],
                SmallGlyphMetrics {
                    height: 8,
                    width: 3,
                    bearing_x: 1.into(),
                    bearing_y: 6.into(),
                    advance: 4,
                },
            ),
            (
                GlyphId::new(2),
                &[0xf0, 0xf0, 0xf0, 0xf0],
                SmallGlyphMetrics {
                    height: 8,
                    width: 4,
                    bearing_x: 0.into(),
                    bearing_y: 6.into(),
                    advance: 4,
                },
            ),
        ];
        for (gid, data, metrics) in expected {
            let location = size.location(eblc.offset_data(), *gid).unwrap();
            assert_eq!(location.format, 2);
            let bitmap_data = ebdt.data(&location).unwrap();
            let (img_fmt, img_data) = bitmap_data.content.extract_data();
            assert_eq!(img_fmt, BitmapDataFormat::BitAligned);
            assert_eq!(img_data, *data);
            assert_eq!(bitmap_data.extract_small_metrics(), metrics);
        }
    }

    #[test]
    fn read_eblc_2_ebdt_5() {
        let font = FontRef::new(font_test_data::EMBEDDED_BITMAPS).unwrap();
        let eblc = font.eblc().unwrap();
        let ebdt = font.ebdt().unwrap();
        let size = &eblc.bitmap_sizes()[1];
        // Metrics for size at index 1
        assert!(
            size.hori.ascender() == 12
                && size.hori.descender() == 5
                && size.hori.width_max() == 9
                && size.hori.max_before_bl() == 12
                && size.hori.min_after_bl() == -5
                && size.vert.ascender() == 12
                && size.vert.descender() == 5
                && size.start_glyph_index() == GlyphId16::new(3)
                && size.end_glyph_index() == GlyphId16::new(3)
                && size.ppem_x() == 15
                && size.ppem_y() == 15
                && size.bit_depth() == 1
        );
        let expected: &[(GlyphId, &[u8])] = &[(
            GlyphId::new(3),
            &[
                0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x11, 0x22, 0x33, 0xff, 0xee, 0x12, 0x34, 0x42, 0x42,
                0x42, 0xaa, 0x88, 0x99, 0x00, 0x11,
            ],
        )];
        for (gid, data) in expected {
            let location = size.location(eblc.offset_data(), *gid).unwrap();
            // Metrics are in EBLC, so the same for all glyphs
            assert_eq!(
                &location.metrics,
                &Some(BigGlyphMetrics {
                    height: 17,
                    width: 9,
                    hori_bearing_x: 0.into(),
                    hori_bearing_y: 12.into(),
                    hori_advance: 9,
                    vert_bearing_x: (-4).into(),
                    vert_bearing_y: (-9).into(),
                    vert_advance: 0,
                })
            );
            assert_eq!(location.format, 5);
            let bitmap_data = ebdt.data(&location).unwrap();
            let (img_fmt, img_data) = bitmap_data.content.extract_data();
            assert_eq!(img_fmt, BitmapDataFormat::BitAligned);
            assert_eq!(img_data, *data);
        }
    }
}
