use std::num::NonZeroU16;
use ttf_parser::{GlyphId, RasterImageFormat};
use ttf_parser::sbix::Table;
use crate::{convert, Unit::*};

#[test]
fn single_glyph() {
    let data = convert(&[
        UInt16(1), // version
        UInt16(0), // flags
        UInt32(1), // number of strikes
        UInt32(12), // strike offset [0]

        // Strike [0]
        UInt16(20), // pixels_per_em
        UInt16(72), // ppi
        UInt32(12), // glyph data offset [0]
        UInt32(44), // glyph data offset [1]

        // Glyph Data [0]
        UInt16(1), // x
        UInt16(2), // y
        Raw(b"png "), // type tag
        // PNG data, just the part we need
        Raw(&[0x89, 0x50, 0x4E, 0x47]),
        Raw(&[0x0D, 0x0A, 0x1A, 0x0A]),
        Raw(&[0x00, 0x00, 0x00, 0x0D]),
        Raw(&[0x49, 0x48, 0x44, 0x52]),
        UInt32(20), // width
        UInt32(30), // height
    ]);

    let table = Table::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
    assert_eq!(table.strikes.len(), 1);

    let strike = table.strikes.get(0).unwrap();
    assert_eq!(strike.pixels_per_em, 20);
    assert_eq!(strike.ppi, 72);
    assert_eq!(strike.len(), 1);

    let glyph_data = strike.get(GlyphId(0)).unwrap();
    assert_eq!(glyph_data.x, 1);
    assert_eq!(glyph_data.y, 2);
    assert_eq!(glyph_data.width, 20);
    assert_eq!(glyph_data.height, 30);
    assert_eq!(glyph_data.pixels_per_em, 20);
    assert_eq!(glyph_data.format, RasterImageFormat::PNG);
    assert_eq!(glyph_data.data.len(), 24);
}

#[test]
fn duplicate_glyph() {
    let data = convert(&[
        UInt16(1), // version
        UInt16(0), // flags
        UInt32(1), // number of strikes
        UInt32(12), // strike offset [0]

        // Strike [0]
        UInt16(20), // pixels_per_em
        UInt16(72), // ppi
        UInt32(16), // glyph data offset [0]
        UInt32(48), // glyph data offset [1]
        UInt32(58), // glyph data offset [2]

        // Glyph Data [0]
        UInt16(1), // x
        UInt16(2), // y
        Raw(b"png "), // type tag
        // PNG data, just the part we need
        Raw(&[0x89, 0x50, 0x4E, 0x47]),
        Raw(&[0x0D, 0x0A, 0x1A, 0x0A]),
        Raw(&[0x00, 0x00, 0x00, 0x0D]),
        Raw(&[0x49, 0x48, 0x44, 0x52]),
        UInt32(20), // width
        UInt32(30), // height

        // Glyph Data [1]
        UInt16(3), // x
        UInt16(4), // y
        Raw(b"dupe"), // type tag
        UInt16(0), // glyph id
    ]);

    let table = Table::parse(NonZeroU16::new(2).unwrap(), &data).unwrap();
    assert_eq!(table.strikes.len(), 1);

    let strike = table.strikes.get(0).unwrap();
    assert_eq!(strike.pixels_per_em, 20);
    assert_eq!(strike.ppi, 72);
    assert_eq!(strike.len(), 2);

    let glyph_data = strike.get(GlyphId(1)).unwrap();
    assert_eq!(glyph_data.x, 1);
    assert_eq!(glyph_data.y, 2);
    assert_eq!(glyph_data.width, 20);
    assert_eq!(glyph_data.height, 30);
    assert_eq!(glyph_data.pixels_per_em, 20);
    assert_eq!(glyph_data.format, RasterImageFormat::PNG);
    assert_eq!(glyph_data.data.len(), 24);
}

#[test]
fn recursive() {
    let data = convert(&[
        UInt16(1), // version
        UInt16(0), // flags
        UInt32(1), // number of strikes
        UInt32(12), // strike offset [0]

        // Strike [0]
        UInt16(20), // pixels_per_em
        UInt16(72), // ppi
        UInt32(16), // glyph data offset [0]
        UInt32(26), // glyph data offset [1]
        UInt32(36), // glyph data offset [2]

        // Glyph Data [0]
        UInt16(1), // x
        UInt16(2), // y
        Raw(b"dupe"), // type tag
        UInt16(0), // glyph id

        // Glyph Data [1]
        UInt16(1), // x
        UInt16(2), // y
        Raw(b"dupe"), // type tag
        UInt16(0), // glyph id
    ]);

    let table = Table::parse(NonZeroU16::new(2).unwrap(), &data).unwrap();
    let strike = table.strikes.get(0).unwrap();
    assert!(strike.get(GlyphId(0)).is_none());
    assert!(strike.get(GlyphId(1)).is_none());
}
