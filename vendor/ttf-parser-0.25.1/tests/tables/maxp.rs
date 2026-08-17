use std::num::NonZeroU16;
use ttf_parser::maxp::Table;
use crate::{convert, Unit::*};

#[test]
fn version_05() {
    let table = Table::parse(&convert(&[
        Fixed(0.3125), // version
        UInt16(1), // number of glyphs
    ])).unwrap();
    assert_eq!(table.number_of_glyphs, NonZeroU16::new(1).unwrap());
}

#[test]
fn version_1_full() {
    let table = Table::parse(&convert(&[
        Fixed(1.0), // version
        UInt16(1), // number of glyphs
        UInt16(0), // maximum points in a non-composite glyph
        UInt16(0), // maximum contours in a non-composite glyph
        UInt16(0), // maximum points in a composite glyph
        UInt16(0), // maximum contours in a composite glyph
        UInt16(0), // maximum zones
        UInt16(0), // maximum twilight points
        UInt16(0), // number of Storage Area locations
        UInt16(0), // number of FDEFs
        UInt16(0), // number of IDEFs
        UInt16(0), // maximum stack depth
        UInt16(0), // maximum byte count for glyph instructions
        UInt16(0), // maximum number of components
        UInt16(0), // maximum levels of recursion
    ])).unwrap();
    assert_eq!(table.number_of_glyphs, NonZeroU16::new(1).unwrap());
}

#[test]
fn version_1_trimmed() {
    // We don't really care about the data after the number of glyphs.
    let table = Table::parse(&convert(&[
        Fixed(1.0), // version
        UInt16(1), // number of glyphs
    ])).unwrap();
    assert_eq!(table.number_of_glyphs, NonZeroU16::new(1).unwrap());
}

#[test]
fn unknown_version() {
    let table = Table::parse(&convert(&[
        Fixed(0.0), // version
        UInt16(1), // number of glyphs
    ]));
    assert!(table.is_none());
}

#[test]
fn zero_glyphs() {
    let table = Table::parse(&convert(&[
        Fixed(0.3125), // version
        UInt16(0), // number of glyphs
    ]));
    assert!(table.is_none());
}

// TODO: what to do when the number of glyphs is 0xFFFF?
//       we're actually checking this in loca
