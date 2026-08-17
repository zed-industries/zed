use std::num::NonZeroU16;
use ttf_parser::GlyphId;
use ttf_parser::ankr::{Table, Point};
use crate::{convert, Unit::*};

#[test]
fn empty() {
    let data = convert(&[
        UInt16(0), // version
        UInt16(0), // reserved
        UInt32(0), // offset to lookup table
        UInt32(0), // offset to glyphs data
    ]);

    let _ = Table::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
}

#[test]
fn single() {
    let data = convert(&[
        UInt16(0), // version
        UInt16(0), // reserved
        UInt32(12), // offset to lookup table
        UInt32(12 + 16), // offset to glyphs data

        // Lookup Table
        UInt16(6), // format

        // Binary Search Table
        UInt16(4), // segment size
        UInt16(1), // number of segments
        UInt16(0), // search range: we don't use it
        UInt16(0), // entry selector: we don't use it
        UInt16(0), // range shift: we don't use it

        // Segment [0]
        UInt16(0), // glyph
        UInt16(0), // offset

        // Glyphs Data
        UInt32(1), // number of points
        // Point [0]
        Int16(-5), // x
        Int16(11), // y
    ]);

    let table = Table::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
    let points = table.points(GlyphId(0)).unwrap();
    assert_eq!(points.get(0).unwrap(), Point { x: -5, y: 11 });
}

#[test]
fn two_points() {
    let data = convert(&[
        UInt16(0), // version
        UInt16(0), // reserved
        UInt32(12), // offset to lookup table
        UInt32(12 + 16), // offset to glyphs data

        // Lookup Table
        UInt16(6), // format

        // Binary Search Table
        UInt16(4), // segment size
        UInt16(1), // number of segments
        UInt16(0), // search range: we don't use it
        UInt16(0), // entry selector: we don't use it
        UInt16(0), // range shift: we don't use it

        // Segment [0]
        UInt16(0), // glyph
        UInt16(0), // offset

        // Glyphs Data
        // Glyph Data [0]
        UInt32(2), // number of points
        // Point [0]
        Int16(-5), // x
        Int16(11), // y
        // Point [1]
        Int16(10), // x
        Int16(-40), // y
    ]);

    let table = Table::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
    let points = table.points(GlyphId(0)).unwrap();
    assert_eq!(points.get(0).unwrap(), Point { x: -5, y: 11 });
    assert_eq!(points.get(1).unwrap(), Point { x: 10, y: -40 });
}

#[test]
fn two_glyphs() {
    let data = convert(&[
        UInt16(0), // version
        UInt16(0), // reserved
        UInt32(12), // offset to lookup table
        UInt32(12 + 20), // offset to glyphs data

        // Lookup Table
        UInt16(6), // format

        // Binary Search Table
        UInt16(4), // segment size
        UInt16(2), // number of segments
        UInt16(0), // search range: we don't use it
        UInt16(0), // entry selector: we don't use it
        UInt16(0), // range shift: we don't use it

        // Segment [0]
        UInt16(0), // glyph
        UInt16(0), // offset
        // Segment [1]
        UInt16(1), // glyph
        UInt16(8), // offset

        // Glyphs Data
        // Glyph Data [0]
        UInt32(1), // number of points
        // Point [0]
        Int16(-5), // x
        Int16(11), // y
        // Glyph Data [1]
        UInt32(1), // number of points
        // Point [0]
        Int16(40), // x
        Int16(10), // y
    ]);

    let table = Table::parse(NonZeroU16::new(1).unwrap(), &data).unwrap();
    let points = table.points(GlyphId(0)).unwrap();
    assert_eq!(points.get(0).unwrap(), Point { x: -5, y: 11 });
    let points = table.points(GlyphId(1)).unwrap();
    assert_eq!(points.get(0).unwrap(), Point { x: 40, y: 10 });
}
