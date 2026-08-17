// TODO: simplify/rewrite

use std::fmt::Write;

use ttf_parser::{cff, GlyphId, CFFError, Rect};

struct Builder(String);
impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        write!(&mut self.0, "M {} {} ", x, y).unwrap();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        write!(&mut self.0, "L {} {} ", x, y).unwrap();
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        write!(&mut self.0, "Q {} {} {} {} ", x1, y1, x, y).unwrap();
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        write!(&mut self.0, "C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y).unwrap();
    }

    fn close(&mut self) {
        write!(&mut self.0, "Z ").unwrap();
    }
}

#[allow(dead_code)]
mod operator {
    pub const HORIZONTAL_STEM: u8           = 1;
    pub const VERTICAL_STEM: u8             = 3;
    pub const VERTICAL_MOVE_TO: u8          = 4;
    pub const LINE_TO: u8                   = 5;
    pub const HORIZONTAL_LINE_TO: u8        = 6;
    pub const VERTICAL_LINE_TO: u8          = 7;
    pub const CURVE_TO: u8                  = 8;
    pub const CALL_LOCAL_SUBROUTINE: u8     = 10;
    pub const RETURN: u8                    = 11;
    pub const ENDCHAR: u8                   = 14;
    pub const HORIZONTAL_STEM_HINT_MASK: u8 = 18;
    pub const HINT_MASK: u8                 = 19;
    pub const COUNTER_MASK: u8              = 20;
    pub const MOVE_TO: u8                   = 21;
    pub const HORIZONTAL_MOVE_TO: u8        = 22;
    pub const VERTICAL_STEM_HINT_MASK: u8   = 23;
    pub const CURVE_LINE: u8                = 24;
    pub const LINE_CURVE: u8                = 25;
    pub const VV_CURVE_TO: u8               = 26;
    pub const HH_CURVE_TO: u8               = 27;
    pub const SHORT_INT: u8                 = 28;
    pub const CALL_GLOBAL_SUBROUTINE: u8    = 29;
    pub const VH_CURVE_TO: u8               = 30;
    pub const HV_CURVE_TO: u8               = 31;
    pub const HFLEX: u8                     = 34;
    pub const FLEX: u8                      = 35;
    pub const HFLEX1: u8                    = 36;
    pub const FLEX1: u8                     = 37;
    pub const FIXED_16_16: u8               = 255;
}

#[allow(dead_code)]
mod top_dict_operator {
    pub const CHARSET_OFFSET: u16               = 15;
    pub const CHAR_STRINGS_OFFSET: u16          = 17;
    pub const PRIVATE_DICT_SIZE_AND_OFFSET: u16 = 18;
    pub const ROS: u16                          = 1230;
    pub const FD_ARRAY: u16                     = 1236;
    pub const FD_SELECT: u16                    = 1237;
}

mod private_dict_operator {
    pub const LOCAL_SUBROUTINES_OFFSET: u16 = 19;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum TtfType {
    Raw(&'static [u8]),
    TrueTypeMagic,
    OpenTypeMagic,
    FontCollectionMagic,
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    CFFInt(i32),
}

use TtfType::*;

fn convert(values: &[TtfType]) -> Vec<u8> {
    let mut data = Vec::with_capacity(256);
    for v in values {
        convert_type(*v, &mut data);
    }

    data
}

fn convert_type(value: TtfType, data: &mut Vec<u8>) {
    match value {
        TtfType::Raw(bytes) => {
            data.extend_from_slice(bytes);
        }
        TtfType::TrueTypeMagic => {
            data.extend_from_slice(&[0x00, 0x01, 0x00, 0x00]);
        }
        TtfType::OpenTypeMagic => {
            data.extend_from_slice(&[0x4F, 0x54, 0x54, 0x4F]);
        }
        TtfType::FontCollectionMagic => {
            data.extend_from_slice(&[0x74, 0x74, 0x63, 0x66]);
        }
        TtfType::Int8(n) => {
            data.extend_from_slice(&i8::to_be_bytes(n));
        }
        TtfType::UInt8(n) => {
            data.extend_from_slice(&u8::to_be_bytes(n));
        }
        TtfType::Int16(n) => {
            data.extend_from_slice(&i16::to_be_bytes(n));
        }
        TtfType::UInt16(n) => {
            data.extend_from_slice(&u16::to_be_bytes(n));
        }
        TtfType::Int32(n) => {
            data.extend_from_slice(&i32::to_be_bytes(n));
        }
        TtfType::UInt32(n) => {
            data.extend_from_slice(&u32::to_be_bytes(n));
        }
        TtfType::CFFInt(n) => {
            match n {
                -107..=107 => {
                    data.push((n as i16 + 139) as u8);
                }
                108..=1131 => {
                    let n = n - 108;
                    data.push(((n >> 8) + 247) as u8);
                    data.push((n & 0xFF) as u8);
                }
                -1131..=-108 => {
                    let n = -n - 108;
                    data.push(((n >> 8) + 251) as u8);
                    data.push((n & 0xFF) as u8);
                }
                -32768..=32767 => {
                    data.push(28);
                    data.extend_from_slice(&i16::to_be_bytes(n as i16));
                }
                _ => {
                    data.push(29);
                    data.extend_from_slice(&i32::to_be_bytes(n));
                }
            }
        }
    }
}

#[derive(Debug)]
struct Writer {
    data: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Writer { data: Vec::with_capacity(256) }
    }

    fn offset(&self) -> usize {
        self.data.len()
    }

    fn write(&mut self, value: TtfType) {
        convert_type(value, &mut self.data);
    }
}


fn gen_cff(
    global_subrs: &[&[TtfType]],
    local_subrs: &[&[TtfType]],
    chars: &[TtfType],
) -> Vec<u8> {
    fn gen_global_subrs(subrs: &[&[TtfType]]) -> Vec<u8> {
        let mut w = Writer::new();
        for v1 in subrs {
            for v2 in v1.iter() {
                w.write(*v2);
            }
        }
        w.data
    }

    fn gen_local_subrs(subrs: &[&[TtfType]]) -> Vec<u8> {
        let mut w = Writer::new();
        for v1 in subrs {
            for v2 in v1.iter() {
                w.write(*v2);
            }
        }
        w.data
    }

    const EMPTY_INDEX_SIZE: usize = 2;
    const INDEX_HEADER_SIZE: usize = 5;

    // TODO: support multiple subrs
    assert!(global_subrs.len() <= 1);
    assert!(local_subrs.len() <= 1);

    let global_subrs_data = gen_global_subrs(global_subrs);
    let local_subrs_data = gen_local_subrs(local_subrs);
    let chars_data = convert(chars);

    assert!(global_subrs_data.len() < 255);
    assert!(local_subrs_data.len() < 255);
    assert!(chars_data.len() < 255);

    let mut w = Writer::new();
    // Header
    w.write(UInt8(1)); // major version
    w.write(UInt8(0)); // minor version
    w.write(UInt8(4)); // header size
    w.write(UInt8(0)); // absolute offset

    // Name INDEX
    w.write(UInt16(0)); // count

    // Top DICT
    // INDEX
    w.write(UInt16(1)); // count
    w.write(UInt8(1)); // offset size
    w.write(UInt8(1)); // index[0]

    let top_dict_idx2 = if local_subrs.is_empty() { 3 } else { 6 };
    w.write(UInt8(top_dict_idx2)); // index[1]
    // Item 0
    let mut charstr_offset = w.offset() + 2;
    charstr_offset += EMPTY_INDEX_SIZE; // String INDEX

    // Global Subroutines INDEX
    if !global_subrs_data.is_empty() {
        charstr_offset += INDEX_HEADER_SIZE + global_subrs_data.len();
    } else {
        charstr_offset += EMPTY_INDEX_SIZE;
    }

    if !local_subrs_data.is_empty() {
        charstr_offset += 3;
    }

    w.write(CFFInt(charstr_offset as i32));
    w.write(UInt8(top_dict_operator::CHAR_STRINGS_OFFSET as u8));

    if !local_subrs_data.is_empty() {
        // Item 1
        w.write(CFFInt(2)); // length
        w.write(CFFInt((charstr_offset + INDEX_HEADER_SIZE + chars_data.len()) as i32)); // offset
        w.write(UInt8(top_dict_operator::PRIVATE_DICT_SIZE_AND_OFFSET as u8));
    }

    // String INDEX
    w.write(UInt16(0)); // count

    // Global Subroutines INDEX
    if global_subrs_data.is_empty() {
        w.write(UInt16(0)); // count
    } else {
        w.write(UInt16(1)); // count
        w.write(UInt8(1)); // offset size
        w.write(UInt8(1)); // index[0]
        w.write(UInt8(global_subrs_data.len() as u8 + 1)); // index[1]
        w.data.extend_from_slice(&global_subrs_data);
    }

    // CharString INDEX
    w.write(UInt16(1)); // count
    w.write(UInt8(1)); // offset size
    w.write(UInt8(1)); // index[0]
    w.write(UInt8(chars_data.len() as u8 + 1)); // index[1]
    w.data.extend_from_slice(&chars_data);

    if !local_subrs_data.is_empty() {
        // The local subroutines offset is relative to the beginning of the Private DICT data.

        // Private DICT
        w.write(CFFInt(2));
        w.write(UInt8(private_dict_operator::LOCAL_SUBROUTINES_OFFSET as u8));

        // Local Subroutines INDEX
        w.write(UInt16(1)); // count
        w.write(UInt8(1)); // offset size
        w.write(UInt8(1)); // index[0]
        w.write(UInt8(local_subrs_data.len() as u8 + 1)); // index[1]
        w.data.extend_from_slice(&local_subrs_data);
    }

    w.data
}

#[test]
fn unsupported_version() {
    let data = convert(&[
        UInt8(10), // major version, only 1 is supported
        UInt8(0), // minor version
        UInt8(4), // header size
        UInt8(0), // absolute offset
    ]);

    assert!(cff::Table::parse(&data).is_none());
}

#[test]
fn non_default_header_size() {
    let data = convert(&[
        // Header
        UInt8(1), // major version
        UInt8(0), // minor version
        UInt8(8), // header size
        UInt8(0), // absolute offset

        // no-op, should be skipped
        UInt8(0),
        UInt8(0),
        UInt8(0),
        UInt8(0),

        // Name INDEX
        UInt16(0), // count

        // Top DICT
        // INDEX
        UInt16(1), // count
        UInt8(1), // offset size
        UInt8(1), // index[0]
        UInt8(3), // index[1]
        // Data
        CFFInt(21),
        UInt8(top_dict_operator::CHAR_STRINGS_OFFSET as u8),

        // String INDEX
        UInt16(0), // count

        // Global Subroutines INDEX
        UInt16(0), // count

        // CharString INDEX
        UInt16(1), // count
        UInt8(1), // offset size
        UInt8(1), // index[0]
        UInt8(4), // index[1]
        // Data
        CFFInt(10),
        UInt8(operator::HORIZONTAL_MOVE_TO),
        UInt8(operator::ENDCHAR),
    ]);

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let rect = table.outline(GlyphId(0), &mut builder).unwrap();

    assert_eq!(builder.0, "M 10 0 Z ");
    assert_eq!(rect, Rect { x_min: 10, y_min: 0, x_max: 10, y_max: 0 });
}

fn rect(x_min: i16, y_min: i16, x_max: i16, y_max: i16) -> Rect {
    Rect { x_min, y_min, x_max, y_max }
}

macro_rules! test_cs_with_subrs {
    ($name:ident, $glob:expr, $loc:expr, $values:expr, $path:expr, $rect_res:expr) => {
        #[test]
        fn $name() {
            let data = gen_cff($glob, $loc, $values);
            let table = cff::Table::parse(&data).unwrap();
            let mut builder = Builder(String::new());
            let rect = table.outline(GlyphId(0), &mut builder).unwrap();

            assert_eq!(builder.0, $path);
            assert_eq!(rect, $rect_res);
        }
    };
}

macro_rules! test_cs {
    ($name:ident, $values:expr, $path:expr, $rect_res:expr) => {
        test_cs_with_subrs!($name, &[], &[], $values, $path, $rect_res);
    };
}

macro_rules! test_cs_err {
    ($name:ident, $values:expr, $err:expr) => {
        #[test]
        fn $name() {
            let data = gen_cff(&[], &[], $values);
            let table = cff::Table::parse(&data).unwrap();
            let mut builder = Builder(String::new());
            let res = table.outline(GlyphId(0), &mut builder);
            assert_eq!(res.unwrap_err(), $err);
        }
    };
}

test_cs!(move_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 Z ",
    rect(10, 20, 10, 20)
);

test_cs!(move_to_with_width, &[
    CFFInt(5), CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 Z ",
    rect(10, 20, 10, 20)
);

test_cs!(hmove_to, &[
    CFFInt(10), UInt8(operator::HORIZONTAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 0 Z ",
    rect(10, 0, 10, 0)
);

test_cs!(hmove_to_with_width, &[
    CFFInt(10), CFFInt(20), UInt8(operator::HORIZONTAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 20 0 Z ",
    rect(20, 0, 20, 0)
);

test_cs!(vmove_to, &[
    CFFInt(10), UInt8(operator::VERTICAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 0 10 Z ",
    rect(0, 10, 0, 10)
);

test_cs!(vmove_to_with_width, &[
    CFFInt(10), CFFInt(20), UInt8(operator::VERTICAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 0 20 Z ",
    rect(0, 20, 0, 20)
);

// Use only the first width.
test_cs!(two_vmove_to_with_width, &[
    CFFInt(10), CFFInt(20), UInt8(operator::VERTICAL_MOVE_TO),
    CFFInt(10), CFFInt(20), UInt8(operator::VERTICAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], "M 0 20 Z M 0 40 Z ",
    rect(0, 20, 0, 40)
);

test_cs!(line_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), UInt8(operator::LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 40 60 Z ",
    rect(10, 20, 40, 60)
);

test_cs!(line_to_with_multiple_pairs, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), UInt8(operator::LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 40 60 L 90 120 Z ",
    rect(10, 20, 90, 120)
);

test_cs!(hline_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), UInt8(operator::HORIZONTAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 40 20 Z ",
    rect(10, 20, 40, 20)
);

test_cs!(hline_to_with_two_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), UInt8(operator::HORIZONTAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 40 20 L 40 60 Z ",
    rect(10, 20, 40, 60)
);

test_cs!(hline_to_with_three_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), UInt8(operator::HORIZONTAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 40 20 L 40 60 L 90 60 Z ",
    rect(10, 20, 90, 60)
);

test_cs!(vline_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), UInt8(operator::VERTICAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 10 50 Z ",
    rect(10, 20, 10, 50)
);

test_cs!(vline_to_with_two_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), UInt8(operator::VERTICAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 10 50 L 50 50 Z ",
    rect(10, 20, 50, 50)
);

test_cs!(vline_to_with_three_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), UInt8(operator::VERTICAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 L 10 50 L 50 50 L 50 100 Z ",
    rect(10, 20, 50, 100)
);

test_cs!(curve_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), CFFInt(70), CFFInt(80),
    UInt8(operator::CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 40 60 90 120 160 200 Z ",
    rect(10, 20, 160, 200)
);

test_cs!(curve_to_with_two_sets_of_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), CFFInt(70), CFFInt(80),
    CFFInt(90), CFFInt(100), CFFInt(110), CFFInt(120), CFFInt(130), CFFInt(140),
    UInt8(operator::CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 40 60 90 120 160 200 C 250 300 360 420 490 560 Z ",
    rect(10, 20, 490, 560)
);

test_cs!(hh_curve_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), UInt8(operator::HH_CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 40 20 80 70 140 70 Z ",
    rect(10, 20, 140, 70)
);

test_cs!(hh_curve_to_with_y, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), CFFInt(70), UInt8(operator::HH_CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 50 50 100 110 170 110 Z ",
    rect(10, 20, 170, 110)
);

test_cs!(vv_curve_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), UInt8(operator::VV_CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 10 50 50 100 50 160 Z ",
    rect(10, 20, 50, 160)
);

test_cs!(vv_curve_to_with_x, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), CFFInt(70), UInt8(operator::VV_CURVE_TO),
    UInt8(operator::ENDCHAR),
], "M 10 20 C 40 60 90 120 90 190 Z ",
    rect(10, 20, 90, 190)
);

#[test]
fn only_endchar() {
    let data = gen_cff(&[], &[], &[UInt8(operator::ENDCHAR)]);
    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    assert!(table.outline(GlyphId(0), &mut builder).is_err());
}

test_cs_with_subrs!(local_subr,
    &[],
    &[&[
        CFFInt(30),
        CFFInt(40),
        UInt8(operator::LINE_TO),
        UInt8(operator::RETURN),
    ]],
    &[
        CFFInt(10),
        UInt8(operator::HORIZONTAL_MOVE_TO),
        CFFInt(0 - 107), // subr index - subr bias
        UInt8(operator::CALL_LOCAL_SUBROUTINE),
        UInt8(operator::ENDCHAR),
    ],
    "M 10 0 L 40 40 Z ",
    rect(10, 0, 40, 40)
);

test_cs_with_subrs!(endchar_in_subr,
    &[],
    &[&[
        CFFInt(30),
        CFFInt(40),
        UInt8(operator::LINE_TO),
        UInt8(operator::ENDCHAR),
    ]],
    &[
        CFFInt(10),
        UInt8(operator::HORIZONTAL_MOVE_TO),
        CFFInt(0 - 107), // subr index - subr bias
        UInt8(operator::CALL_LOCAL_SUBROUTINE),
    ],
    "M 10 0 L 40 40 Z ",
    rect(10, 0, 40, 40)
);

test_cs_with_subrs!(global_subr,
    &[&[
        CFFInt(30),
        CFFInt(40),
        UInt8(operator::LINE_TO),
        UInt8(operator::RETURN),
    ]],
    &[],
    &[
        CFFInt(10),
        UInt8(operator::HORIZONTAL_MOVE_TO),
        CFFInt(0 - 107), // subr index - subr bias
        UInt8(operator::CALL_GLOBAL_SUBROUTINE),
        UInt8(operator::ENDCHAR),
    ],
    "M 10 0 L 40 40 Z ",
    rect(10, 0, 40, 40)
);

test_cs_err!(reserved_operator, &[
    CFFInt(10), UInt8(2),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidOperator);

test_cs_err!(line_to_without_move_to, &[
    CFFInt(10), CFFInt(20), UInt8(operator::LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::MissingMoveTo);

test_cs_err!(move_to_with_too_many_coords, &[
    CFFInt(10), CFFInt(10), CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(move_to_with_not_enough_coords, &[
    CFFInt(10), UInt8(operator::MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(hmove_to_with_too_many_coords, &[
    CFFInt(10), CFFInt(10), CFFInt(10), UInt8(operator::HORIZONTAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(hmove_to_with_not_enough_coords, &[
    UInt8(operator::HORIZONTAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(vmove_to_with_too_many_coords, &[
    CFFInt(10), CFFInt(10), CFFInt(10), UInt8(operator::VERTICAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(vmove_to_with_not_enough_coords, &[
    UInt8(operator::VERTICAL_MOVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(line_to_with_single_coord, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), UInt8(operator::LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(line_to_with_odd_number_of_coord, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), UInt8(operator::LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(hline_to_without_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    UInt8(operator::HORIZONTAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(vline_to_without_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    UInt8(operator::VERTICAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(curve_to_with_invalid_num_of_coords_1, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), UInt8(operator::CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(curve_to_with_invalid_num_of_coords_2, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(60), CFFInt(70), CFFInt(80), CFFInt(90),
    UInt8(operator::CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(hh_curve_to_with_not_enough_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), UInt8(operator::HH_CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(hh_curve_to_with_too_many_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(30), CFFInt(40), CFFInt(50),
    UInt8(operator::HH_CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(vv_curve_to_with_not_enough_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), UInt8(operator::VV_CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(vv_curve_to_with_too_many_coords, &[
    CFFInt(10), CFFInt(20), UInt8(operator::MOVE_TO),
    CFFInt(30), CFFInt(40), CFFInt(50), CFFInt(30), CFFInt(40), CFFInt(50),
    UInt8(operator::VV_CURVE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::InvalidArgumentsStackLength);

test_cs_err!(multiple_endchar, &[
    UInt8(operator::ENDCHAR),
    UInt8(operator::ENDCHAR),
], CFFError::DataAfterEndChar);

test_cs_err!(seac_with_not_enough_data, &[
    CFFInt(0),
    CFFInt(0),
    CFFInt(0),
    CFFInt(0),
    UInt8(operator::ENDCHAR),
], CFFError::NestingLimitReached);

test_cs_err!(operands_overflow, &[
    CFFInt(0), CFFInt(1), CFFInt(2), CFFInt(3), CFFInt(4), CFFInt(5), CFFInt(6), CFFInt(7), CFFInt(8), CFFInt(9),
    CFFInt(0), CFFInt(1), CFFInt(2), CFFInt(3), CFFInt(4), CFFInt(5), CFFInt(6), CFFInt(7), CFFInt(8), CFFInt(9),
    CFFInt(0), CFFInt(1), CFFInt(2), CFFInt(3), CFFInt(4), CFFInt(5), CFFInt(6), CFFInt(7), CFFInt(8), CFFInt(9),
    CFFInt(0), CFFInt(1), CFFInt(2), CFFInt(3), CFFInt(4), CFFInt(5), CFFInt(6), CFFInt(7), CFFInt(8), CFFInt(9),
    CFFInt(0), CFFInt(1), CFFInt(2), CFFInt(3), CFFInt(4), CFFInt(5), CFFInt(6), CFFInt(7), CFFInt(8), CFFInt(9),
], CFFError::ArgumentsStackLimitReached);

test_cs_err!(operands_overflow_with_4_byte_ints, &[
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
    CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000), CFFInt(30000),
], CFFError::ArgumentsStackLimitReached);

test_cs_err!(bbox_overflow, &[
    CFFInt(32767), UInt8(operator::HORIZONTAL_MOVE_TO),
    CFFInt(32767), UInt8(operator::HORIZONTAL_LINE_TO),
    UInt8(operator::ENDCHAR),
], CFFError::BboxOverflow);

#[test]
fn endchar_in_subr_with_extra_data_1() {
    let data = gen_cff(
        &[],
        &[&[
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
            UInt8(operator::ENDCHAR),
        ]],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::DataAfterEndChar);
}

#[test]
fn endchar_in_subr_with_extra_data_2() {
    let data = gen_cff(
        &[],
        &[&[
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
            UInt8(operator::ENDCHAR),
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
        ]],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::DataAfterEndChar);
}

#[test]
fn subr_without_return() {
    let data = gen_cff(
        &[],
        &[&[
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
            UInt8(operator::ENDCHAR),
            CFFInt(30),
            CFFInt(40),
            UInt8(operator::LINE_TO),
        ]],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::DataAfterEndChar);
}

#[test]
fn recursive_local_subr() {
    let data = gen_cff(
        &[],
        &[&[
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
        ]],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::NestingLimitReached);
}

#[test]
fn recursive_global_subr() {
    let data = gen_cff(
        &[&[
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_GLOBAL_SUBROUTINE),
        ]],
        &[],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_GLOBAL_SUBROUTINE),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::NestingLimitReached);
}

#[test]
fn recursive_mixed_subr() {
    let data = gen_cff(
        &[&[
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_LOCAL_SUBROUTINE),
        ]],
        &[&[
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_GLOBAL_SUBROUTINE),
        ]],
        &[
            CFFInt(10),
            UInt8(operator::HORIZONTAL_MOVE_TO),
            CFFInt(0 - 107), // subr index - subr bias
            UInt8(operator::CALL_GLOBAL_SUBROUTINE),
        ]
    );

    let table = cff::Table::parse(&data).unwrap();
    let mut builder = Builder(String::new());
    let res = table.outline(GlyphId(0), &mut builder);
    assert_eq!(res.unwrap_err(), CFFError::NestingLimitReached);
}

#[test]
fn zero_char_string_offset() {
    let data = convert(&[
        // Header
        UInt8(1), // major version
        UInt8(0), // minor version
        UInt8(4), // header size
        UInt8(0), // absolute offset

        // Name INDEX
        UInt16(0), // count

        // Top DICT
        // INDEX
        UInt16(1), // count
        UInt8(1), // offset size
        UInt8(1), // index[0]
        UInt8(3), // index[1]
        // Data
        CFFInt(0), // zero offset!
        UInt8(top_dict_operator::CHAR_STRINGS_OFFSET as u8),
    ]);

    assert!(cff::Table::parse(&data).is_none());
}

#[test]
fn invalid_char_string_offset() {
    let data = convert(&[
        // Header
        UInt8(1), // major version
        UInt8(0), // minor version
        UInt8(4), // header size
        UInt8(0), // absolute offset

        // Name INDEX
        UInt16(0), // count

        // Top DICT
        // INDEX
        UInt16(1), // count
        UInt8(1), // offset size
        UInt8(1), // index[0]
        UInt8(3), // index[1]
        // Data
        CFFInt(2), // invalid offset!
        UInt8(top_dict_operator::CHAR_STRINGS_OFFSET as u8),
    ]);

    assert!(cff::Table::parse(&data).is_none());
}

// TODO: return from main
// TODO: return without endchar
// TODO: data after return
// TODO: recursive subr
// TODO: HORIZONTAL_STEM
// TODO: VERTICAL_STEM
// TODO: HORIZONTAL_STEM_HINT_MASK
// TODO: HINT_MASK
// TODO: COUNTER_MASK
// TODO: VERTICAL_STEM_HINT_MASK
// TODO: CURVE_LINE
// TODO: LINE_CURVE
// TODO: VH_CURVE_TO
// TODO: HFLEX
// TODO: FLEX
// TODO: HFLEX1
// TODO: FLEX1
