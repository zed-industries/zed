#[rustfmt::skip] mod aat;
#[rustfmt::skip] mod ankr;
#[rustfmt::skip] mod cff1;
#[rustfmt::skip] mod cmap;
#[rustfmt::skip] mod colr;
#[rustfmt::skip] mod feat;
#[rustfmt::skip] mod glyf;
#[rustfmt::skip] mod hmtx;
#[rustfmt::skip] mod maxp;
#[rustfmt::skip] mod sbix;
#[rustfmt::skip] mod trak;

use ttf_parser::{fonts_in_collection, Face, FaceParsingError};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Unit {
    Raw(&'static [u8]),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Fixed(f32),
}

pub fn convert(units: &[Unit]) -> Vec<u8> {
    let mut data = Vec::with_capacity(256);
    for v in units {
        convert_unit(*v, &mut data);
    }

    data
}

fn convert_unit(unit: Unit, data: &mut Vec<u8>) {
    match unit {
        Unit::Raw(bytes) => {
            data.extend_from_slice(bytes);
        }
        Unit::Int8(n) => {
            data.extend_from_slice(&i8::to_be_bytes(n));
        }
        Unit::UInt8(n) => {
            data.extend_from_slice(&u8::to_be_bytes(n));
        }
        Unit::Int16(n) => {
            data.extend_from_slice(&i16::to_be_bytes(n));
        }
        Unit::UInt16(n) => {
            data.extend_from_slice(&u16::to_be_bytes(n));
        }
        Unit::Int32(n) => {
            data.extend_from_slice(&i32::to_be_bytes(n));
        }
        Unit::UInt32(n) => {
            data.extend_from_slice(&u32::to_be_bytes(n));
        }
        Unit::Fixed(n) => {
            data.extend_from_slice(&i32::to_be_bytes((n * 65536.0) as i32));
        }
    }
}

#[test]
fn empty_font() {
    assert_eq!(
        Face::parse(&[], 0).unwrap_err(),
        FaceParsingError::UnknownMagic
    );
}

#[test]
fn zero_tables() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x00, 0x01, 0x00, 0x00]), // magic
        UInt16(0),                      // numTables
        UInt16(0),                      // searchRange
        UInt16(0),                      // entrySelector
        UInt16(0),                      // rangeShift
    ]);

    assert_eq!(
        Face::parse(&data, 0).unwrap_err(),
        FaceParsingError::NoHeadTable
    );
}

#[test]
fn tables_count_overflow() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x00, 0x01, 0x00, 0x00]), // magic
        UInt16(u16::MAX),               // numTables
        UInt16(0),                      // searchRange
        UInt16(0),                      // entrySelector
        UInt16(0),                      // rangeShift
    ]);

    assert_eq!(
        Face::parse(&data, 0).unwrap_err(),
        FaceParsingError::MalformedFont
    );
}

#[test]
fn empty_font_collection() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x74, 0x74, 0x63, 0x66]), // magic
        UInt16(0),                      // majorVersion
        UInt16(0),                      // minorVersion
        UInt32(0),                      // numFonts
    ]);

    assert_eq!(fonts_in_collection(&data), Some(0));
    assert_eq!(
        Face::parse(&data, 0).unwrap_err(),
        FaceParsingError::FaceIndexOutOfBounds
    );
}

#[test]
fn font_collection_num_fonts_overflow_1() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x74, 0x74, 0x63, 0x66]), // magic
        UInt16(0),                      // majorVersion
        UInt16(0),                      // minorVersion
        UInt32(u32::MAX),               // numFonts
    ]);

    assert_eq!(fonts_in_collection(&data), Some(u32::MAX));
}

#[test]
#[should_panic]
fn font_collection_num_fonts_overflow_2() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x74, 0x74, 0x63, 0x66]), // magic
        UInt16(0),                      // majorVersion
        UInt16(0),                      // minorVersion
        UInt32(u32::MAX),               // numFonts
    ]);

    assert_eq!(
        Face::parse(&data, 0).unwrap_err(),
        FaceParsingError::MalformedFont
    );
}

#[test]
fn font_index_overflow() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x74, 0x74, 0x63, 0x66]), // magic
        UInt16(0),                      // majorVersion
        UInt16(0),                      // minorVersion
        UInt32(1),                      // numFonts
        UInt32(12),                     // offset [0]
    ]);

    assert_eq!(fonts_in_collection(&data), Some(1));
    assert_eq!(
        Face::parse(&data, u32::MAX).unwrap_err(),
        FaceParsingError::FaceIndexOutOfBounds
    );
}

#[test]
fn font_index_overflow_on_regular_font() {
    use Unit::*;
    let data = convert(&[
        Raw(&[0x00, 0x01, 0x00, 0x00]), // magic
        UInt16(0),                      // numTables
        UInt16(0),                      // searchRange
        UInt16(0),                      // entrySelector
        UInt16(0),                      // rangeShift
    ]);

    assert_eq!(fonts_in_collection(&data), None);
    assert_eq!(
        Face::parse(&data, 1).unwrap_err(),
        FaceParsingError::FaceIndexOutOfBounds
    );
}
