use ttf_parser::{RasterGlyphImage, RasterImageFormat};

// NOTE: Bitmap.otb is an incomplete example font that was created specifically for this test.
// It is under the same license as the other source files in the project.
static FONT_DATA: &[u8] = include_bytes!("fonts/bitmap.otb");

#[test]
fn bitmap_font() {
    let face = ttf_parser::Face::parse(FONT_DATA, 0).unwrap();
    assert_eq!(face.units_per_em(), 800);
    assert_eq!(
        face.glyph_hor_advance(face.glyph_index('a').unwrap()),
        Some(500)
    );
    const W: u8 = 0;
    const B: u8 = 255;
    assert_eq!(
        face.glyph_raster_image(face.glyph_index('a').unwrap(), 1),
        Some(RasterGlyphImage {
            x: 0,
            y: 0,
            width: 4,
            height: 4,
            pixels_per_em: 8,
            format: RasterImageFormat::BitmapGray8,
            #[rustfmt::skip]
            data: &[
                W, B, B, B,
                B, W, W, B,
                B, W, W, B,
                W, B, B, B
            ]
        })
    );
    assert_eq!(
        face.glyph_raster_image(face.glyph_index('d').unwrap(), 1),
        Some(RasterGlyphImage {
            x: 0,
            y: 0,
            width: 4,
            height: 6,
            pixels_per_em: 8,
            format: RasterImageFormat::BitmapGray8,
            #[rustfmt::skip]
            data: &[
                W, W, W, B,
                W, W, W, B,
                W, B, B, B,
                B, W, W, B,
                B, W, W, B,
                W, B, B, B
            ]
        })
    );
    assert_eq!(
        face.glyph_raster_image(face.glyph_index('\"').unwrap(), 1),
        Some(RasterGlyphImage {
            x: 1,
            y: 4,
            width: 3,
            height: 2,
            pixels_per_em: 8,
            format: RasterImageFormat::BitmapGray8,
            #[rustfmt::skip]
            data: &[
                B, W, B,
                B, W, B,
            ]
        })
    );
}
