use std::num::NonZeroU16;
use ttf_parser::GlyphId;
use ttf_parser::hmtx::Table;
use crate::{convert, Unit::*};

macro_rules! nzu16 {
    ($n:expr) => { NonZeroU16::new($n).unwrap() };
}

#[test]
fn simple_case() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]
    ]);

    let table = Table::parse(1, nzu16!(1), &data).unwrap();
    assert_eq!(table.advance(GlyphId(0)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(0)), Some(2));
}

#[test]
fn empty() {
    assert!(Table::parse(1, nzu16!(1), &[]).is_none());
}

#[test]
fn zero_metrics() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]
    ]);

    assert!(Table::parse(0, nzu16!(1), &data).is_none());
}

#[test]
fn smaller_than_glyphs_count() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]

        Int16(3), // side bearing [1]
    ]);

    let table = Table::parse(1, nzu16!(2), &data).unwrap();
    assert_eq!(table.advance(GlyphId(0)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(0)), Some(2));
    assert_eq!(table.advance(GlyphId(1)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(1)), Some(3));
}

#[test]
fn no_additional_side_bearings() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]

        // A single side bearing should be present here.
        // We should simply ignore it and not return None during Table parsing.
    ]);

    let table = Table::parse(1, nzu16!(2), &data).unwrap();
    assert_eq!(table.advance(GlyphId(0)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(0)), Some(2));
}

#[test]
fn less_metrics_than_glyphs() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]

        UInt16(3), // advance width [1]
        Int16(4), // side bearing [1]

        Int16(5), // side bearing [2]
    ]);

    let table = Table::parse(2, nzu16!(1), &data).unwrap();
    assert_eq!(table.side_bearing(GlyphId(0)), Some(2));
    assert_eq!(table.side_bearing(GlyphId(1)), Some(4));
    assert_eq!(table.side_bearing(GlyphId(2)), None);
}

#[test]
fn glyph_out_of_bounds_0() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]
    ]);

    let table = Table::parse(1, nzu16!(1), &data).unwrap();
    assert_eq!(table.advance(GlyphId(0)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(0)), Some(2));
    assert_eq!(table.advance(GlyphId(1)), None);
    assert_eq!(table.side_bearing(GlyphId(1)), None);
}

#[test]
fn glyph_out_of_bounds_1() {
    let data = convert(&[
        UInt16(1), // advance width [0]
        Int16(2), // side bearing [0]

        Int16(3), // side bearing [1]
    ]);

    let table = Table::parse(1, nzu16!(2), &data).unwrap();
    assert_eq!(table.advance(GlyphId(1)), Some(1));
    assert_eq!(table.side_bearing(GlyphId(1)), Some(3));
    assert_eq!(table.advance(GlyphId(2)), None);
    assert_eq!(table.side_bearing(GlyphId(2)), None);
}
