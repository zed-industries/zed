#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(missing_copy_implementations)]

use libc::{
    c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void, ptrdiff_t, size_t,
};

mod tt_tables;
pub use crate::tt_tables::*;

// Basic Data Types
pub type FT_Byte = c_uchar;
pub type FT_Bytes = *const FT_Byte;
pub type FT_Char = c_char;
pub type FT_Int = c_int;
pub type FT_UInt = c_uint;
pub type FT_Int16 = c_short;
pub type FT_UInt16 = c_ushort;
pub type FT_Int32 = i32;
pub type FT_UInt32 = u32;
pub type FT_Int64 = i64;
pub type FT_UInt64 = u64;
pub type FT_Short = c_short;
pub type FT_UShort = c_ushort;
pub type FT_Long = c_long;
pub type FT_ULong = c_ulong;
pub type FT_Bool = c_uchar;
pub type FT_Offset = size_t;
pub type FT_PtrDist = ptrdiff_t;
pub type FT_String = c_char;
pub type FT_Tag = FT_UInt32;
pub type FT_Error = c_int;
pub type FT_Fixed = c_long;
pub type FT_Pointer = *mut c_void;
pub type FT_Pos = c_long;
pub type FT_FWord = c_short;
pub type FT_UFWord = c_ushort;
pub type FT_F2Dot14 = c_short;
pub type FT_F26Dot6 = c_long;
pub type FT_Generic_Finalizer = extern "C" fn(*mut c_void);
pub type FT_StreamDesc = *mut c_void;
pub type FT_Stream_IoFunc = extern "C" fn(FT_Stream, c_ulong, *mut c_uchar, c_ulong) -> c_ulong;
pub type FT_Stream_CloseFunc = extern "C" fn(FT_Stream);
pub type FT_Alloc_Func = extern "C" fn(FT_Memory, c_long) -> *mut c_void;
pub type FT_Free_Func = extern "C" fn(FT_Memory, *mut c_void);
pub type FT_Realloc_Func = extern "C" fn(FT_Memory, c_long, c_long, *mut c_void) -> *mut c_void;
pub type FT_Outline_MoveToFunc = extern "C" fn(to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_LineToFunc = extern "C" fn(to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_ConicToFunc =
    extern "C" fn(control: *const FT_Vector, to: *const FT_Vector, user: *mut c_void) -> c_int;
pub type FT_Outline_CubicToFunc = extern "C" fn(
    control1: *const FT_Vector,
    control2: *const FT_Vector,
    to: *const FT_Vector,
    user: *mut c_void,
) -> c_int;
pub type FT_SpanFunc =
    extern "C" fn(y: c_int, count: c_int, spans: *const FT_Span, user: *mut c_void);
pub type FT_Raster_BitTest_Func = extern "C" fn(y: c_int, x: c_int, user: *mut c_void) -> c_int;
pub type FT_Raster_BitSet_Func = extern "C" fn(y: c_int, x: c_int, user: *mut c_void);

pub trait FTErrorMethods {
    fn succeeded(&self) -> bool;
}

impl FTErrorMethods for FT_Error {
    fn succeeded(&self) -> bool {
        *self == 0
    }
}

// Structs
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct FT_Vector {
    pub x: FT_Pos,
    pub y: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct FT_BBox {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Matrix {
    pub xx: FT_Fixed,
    pub xy: FT_Fixed,
    pub yx: FT_Fixed,
    pub yy: FT_Fixed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_UnitVector {
    pub x: FT_F2Dot14,
    pub y: FT_F2Dot14,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Bitmap {
    pub rows: c_int,
    pub width: c_int,
    pub pitch: c_int,
    pub buffer: *mut c_uchar,
    pub num_grays: c_short,
    pub pixel_mode: c_char,
    pub palette_mode: c_char,
    pub palette: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Data {
    pub pointer: *const FT_Byte,
    pub length: FT_Int,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Generic {
    pub data: *mut c_void,
    pub finalizer: FT_Generic_Finalizer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Size_Metrics {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,

    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,

    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Outline {
    pub n_contours: c_short,
    pub n_points: c_short,

    pub points: *mut FT_Vector,
    pub tags: *mut c_char,
    pub contours: *mut c_short,

    pub flags: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Glyph_Metrics {
    pub width: FT_Pos,
    pub height: FT_Pos,

    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,

    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Parameter {
    pub tag: FT_ULong,
    pub data: FT_Pointer,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Open_Args {
    pub flags: FT_UInt,
    pub memory_base: *const FT_Byte,
    pub memory_size: FT_Long,
    pub pathname: *mut FT_String,
    pub stream: FT_Stream,
    pub driver: FT_Module,
    pub num_params: FT_Int,
    pub params: *mut FT_Parameter,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Bitmap_Size {
    pub height: FT_Short,
    pub width: FT_Short,

    pub size: FT_Pos,

    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TT_OS2 {
    pub version: FT_UShort,
    pub xAvgCharWidth: FT_Short,
    pub usWeightClass: FT_UShort,
    pub usWidthClass: FT_UShort,
    pub fsType: FT_UShort,
    pub ySubscriptXSize: FT_Short,
    pub ySubscriptYSize: FT_Short,
    pub ySubscriptXOffset: FT_Short,
    pub ySubscriptYOffset: FT_Short,
    pub ySuperscriptXSize: FT_Short,
    pub ySuperscriptYSize: FT_Short,
    pub ySuperscriptXOffset: FT_Short,
    pub ySuperscriptYOffset: FT_Short,
    pub yStrikeoutSize: FT_Short,
    pub yStrikeoutPosition: FT_Short,
    pub sFamilyClass: FT_Short,

    pub panose: [FT_Byte; 10],

    pub ulUnicodeRange1: FT_ULong, /* Bits 0-31   */
    pub ulUnicodeRange2: FT_ULong, /* Bits 32-63  */
    pub ulUnicodeRange3: FT_ULong, /* Bits 64-95  */
    pub ulUnicodeRange4: FT_ULong, /* Bits 96-127 */

    pub achVendID: [FT_Char; 4],

    pub fsSelection: FT_UShort,
    pub usFirstCharIndex: FT_UShort,
    pub usLastCharIndex: FT_UShort,
    pub sTypoAscender: FT_Short,
    pub sTypoDescender: FT_Short,
    pub sTypoLineGap: FT_Short,
    pub usWinAscent: FT_UShort,
    pub usWinDescent: FT_UShort,

    /* only version 1 tables */
    pub ulCodePageRange1: FT_ULong, /* Bits 0-31  */
    pub ulCodePageRange2: FT_ULong, /* Bits 32-63 */

    /* only version 2 tables */
    pub sxHeight: FT_Short,
    pub sCapHeight: FT_Short,
    pub usDefaultChar: FT_UShort,
    pub usBreakChar: FT_UShort,
    pub usMaxContext: FT_UShort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TT_Postscript {
    pub formatType: FT_Fixed,
    pub italicAngle: FT_Fixed,
    pub underlinePosition: FT_Short,
    pub underlineThickness: FT_Short,
    pub isFixedPitch: FT_ULong,
    pub minMemType42: FT_ULong,
    pub maxMemType42: FT_ULong,
    pub minMemType1: FT_ULong,
    pub maxMemType1: FT_ULong,
    /* Glyph names follow in the 'post' table, but we don't */
    /* load them by default.                                */
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Span {
    pub x: c_short,
    pub len: c_ushort,
    pub coverage: c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_MM_Axis {
    pub name: *mut FT_String,
    pub minimum: FT_Long,
    pub maximum: FT_Long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Multi_Master {
    pub num_axis: FT_UInt,
    pub num_designs: FT_UInt,
    pub axis: [FT_MM_Axis; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Var_Axis {
    pub name: *mut FT_String,
    pub minimum: FT_Fixed,
    pub def: FT_Fixed,
    pub maximum: FT_Fixed,
    pub tag: FT_ULong,
    pub strid: FT_UInt,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Var_Named_Style {
    pub coords: *mut FT_Fixed,
    pub strid: FT_UInt,
    pub psid: FT_UInt,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_MM_Var {
    pub num_axis: FT_UInt,
    pub num_designs: FT_UInt,
    pub num_namedstyles: FT_UInt,
    pub axis: *mut FT_Var_Axis,
    pub namedstyle: *mut FT_Var_Named_Style,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_SfntName {
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
    pub language_id: FT_UShort,
    pub name_id: FT_UShort,

    pub string: *mut FT_Byte, /* this string is *not* null-terminated! */
    pub string_len: FT_UInt,  /* in bytes                              */
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_SfntLangTag {
    pub string: *mut FT_Byte,
    pub string_len: FT_UInt,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FT_LayerIterator {
    pub num_layers: FT_UInt,
    pub layer: FT_UInt,
    pub p: *mut FT_Byte,
}

// Enums

pub type enum_FT_Sfnt_Tag_ = c_uint;
pub const ft_sfnt_head: u32 = 0_u32;
pub const ft_sfnt_maxp: u32 = 1_u32;
pub const ft_sfnt_os2: u32 = 2_u32;
pub const ft_sfnt_hhea: u32 = 3_u32;
pub const ft_sfnt_vhea: u32 = 4_u32;
pub const ft_sfnt_post: u32 = 5_u32;
pub const ft_sfnt_pclt: u32 = 6_u32;
pub const ft_sfnt_max: u32 = 7_u32;
pub type FT_Sfnt_Tag = enum_FT_Sfnt_Tag_;

pub type FT_Pixel_Mode = c_uint;
pub const FT_PIXEL_MODE_NONE: FT_Pixel_Mode = 0;
pub const FT_PIXEL_MODE_MONO: FT_Pixel_Mode = 1;
pub const FT_PIXEL_MODE_GRAY: FT_Pixel_Mode = 2;
pub const FT_PIXEL_MODE_GRAY2: FT_Pixel_Mode = 3;
pub const FT_PIXEL_MODE_GRAY4: FT_Pixel_Mode = 4;
pub const FT_PIXEL_MODE_LCD: FT_Pixel_Mode = 5;
pub const FT_PIXEL_MODE_LCD_V: FT_Pixel_Mode = 6;
pub const FT_PIXEL_MODE_BGRA: FT_Pixel_Mode = 7;
pub const FT_PIXEL_MODE_MAX: FT_Pixel_Mode = 8;

pub type FT_Glyph_Format = c_uint;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format = 0;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format = 1668246896;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format = 1651078259;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format = 1869968492;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format = 1886154612;

pub type FT_Render_Mode = c_uint;
pub const FT_RENDER_MODE_NORMAL: FT_Render_Mode = 0;
pub const FT_RENDER_MODE_LIGHT: FT_Render_Mode = 1;
pub const FT_RENDER_MODE_MONO: FT_Render_Mode = 2;
pub const FT_RENDER_MODE_LCD: FT_Render_Mode = 3;
pub const FT_RENDER_MODE_LCD_V: FT_Render_Mode = 4;
pub const FT_RENDER_MODE_SDF: FT_Render_Mode = 5;
pub const FT_RENDER_MODE_MAX: FT_Render_Mode = FT_RENDER_MODE_SDF + 1;

pub type FT_LcdFilter = c_uint;
pub const FT_LCD_FILTER_NONE: FT_LcdFilter = 0;
pub const FT_LCD_FILTER_DEFAULT: FT_LcdFilter = 1;
pub const FT_LCD_FILTER_LIGHT: FT_LcdFilter = 3;
pub const FT_LCD_FILTER_LEGACY1: FT_LcdFilter = 3;
pub const FT_LCD_FILTER_LEGACY: FT_LcdFilter = 16;
pub const FT_LCD_FILTER_MAX: FT_LcdFilter = 17;

pub type FT_Encoding = c_uint;
pub const FT_ENCODING_NONE: FT_Encoding = 0;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding = 1937337698;
pub const FT_ENCODING_UNICODE: FT_Encoding = 1970170211;
pub const FT_ENCODING_SJIS: FT_Encoding = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding = 1734484000;
pub const FT_ENCODING_BIG5: FT_Encoding = 1651074869;
pub const FT_ENCODING_WANSUNG: FT_Encoding = 2002873971;
pub const FT_ENCODING_JOHAB: FT_Encoding = 1785686113;
pub const FT_ENCODING_MS_SJIS: FT_Encoding = 1936353651;
pub const FT_ENCODING_MS_GB2312: FT_Encoding = 1734484000;
pub const FT_ENCODING_MS_BIG5: FT_Encoding = 1651074869;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding = 2002873971;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding = 1785686113;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding = 1094995778;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding = 1094992453;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding = 1094992451;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding = 1818326065;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding = 1818326066;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding = 1634889070;

pub type FT_Size_Request_Type = c_uint;
pub const FT_SIZE_REQUEST_TYPE_NOMINAL: FT_Size_Request_Type = 0;
pub const FT_SIZE_REQUEST_TYPE_REAL_DIM: FT_Size_Request_Type = 1;
pub const FT_SIZE_REQUEST_TYPE_BBOX: FT_Size_Request_Type = 2;
pub const FT_SIZE_REQUEST_TYPE_CELL: FT_Size_Request_Type = 3;
pub const FT_SIZE_REQUEST_TYPE_SCALES: FT_Size_Request_Type = 4;
pub const FT_SIZE_REQUEST_TYPE_MAX: FT_Size_Request_Type = 5;

pub type FT_Kerning_Mode = c_uint;
pub const FT_KERNING_DEFAULT: FT_Kerning_Mode = 0;
pub const FT_KERNING_UNFITTED: FT_Kerning_Mode = 1;
pub const FT_KERNING_UNSCALED: FT_Kerning_Mode = 2;

pub type FT_Glyph_BBox_Mode = c_uint;
pub const FT_GLYPH_BBOX_UNSCALED: FT_Glyph_BBox_Mode = 0;
pub const FT_GLYPH_BBOX_SUBPIXELS: FT_Glyph_BBox_Mode = 0;
pub const FT_GLYPH_BBOX_GRIDFIT: FT_Glyph_BBox_Mode = 1;
pub const FT_GLYPH_BBOX_TRUNCATE: FT_Glyph_BBox_Mode = 2;
pub const FT_GLYPH_BBOX_PIXELS: FT_Glyph_BBox_Mode = 3;

pub type FT_Stroker_LineJoin = c_uint;
pub const FT_STROKER_LINEJOIN_ROUND: FT_Stroker_LineJoin = 0;
pub const FT_STROKER_LINEJOIN_BEVEL: FT_Stroker_LineJoin = 1;
pub const FT_STROKER_LINEJOIN_MITER_VARIABLE: FT_Stroker_LineJoin = 2;
pub const FT_STROKER_LINEJOIN_MITER: FT_Stroker_LineJoin = 2;
pub const FT_STROKER_LINEJOIN_MITER_FIXED: FT_Stroker_LineJoin = 3;

pub type FT_Stroker_LineCap = c_uint;
pub const FT_STROKER_LINECAP_BUTT: FT_Stroker_LineCap = 0;
pub const FT_STROKER_LINECAP_ROUND: FT_Stroker_LineCap = 1;
pub const FT_STROKER_LINECAP_SQUARE: FT_Stroker_LineCap = 2;

pub type FT_StrokerBorder = c_uint;
pub const FT_STROKER_BORDER_LEFT: FT_StrokerBorder = 0;
pub const FT_STROKER_BORDER_RIGHT: FT_StrokerBorder = 1;

pub type FT_Orientation = c_uint;
pub const FT_ORIENTATION_TRUETYPE: FT_Orientation = 0;
pub const FT_ORIENTATION_POSTSCRIPT: FT_Orientation = 1;
pub const FT_ORIENTATION_NONE: FT_Orientation = 2;
pub const FT_ORIENTATION_FILL_RIGHT: FT_Orientation = FT_ORIENTATION_TRUETYPE;
pub const FT_ORIENTATION_FILL_LEFT: FT_Orientation = FT_ORIENTATION_POSTSCRIPT;

// Constants
pub const FT_FACE_FLAG_SCALABLE: FT_Long = 1;
pub const FT_FACE_FLAG_FIXED_SIZES: FT_Long = 1 << 1;
pub const FT_FACE_FLAG_FIXED_WIDTH: FT_Long = 1 << 2;
pub const FT_FACE_FLAG_SFNT: FT_Long = 1 << 3;
pub const FT_FACE_FLAG_HORIZONTAL: FT_Long = 1 << 4;
pub const FT_FACE_FLAG_VERTICAL: FT_Long = 1 << 5;
pub const FT_FACE_FLAG_KERNING: FT_Long = 1 << 6;
pub const FT_FACE_FLAG_FAST_GLYPHS: FT_Long = 1 << 7;
pub const FT_FACE_FLAG_MULTIPLE_MASTERS: FT_Long = 1 << 8;
pub const FT_FACE_FLAG_GLYPH_NAMES: FT_Long = 1 << 9;
pub const FT_FACE_FLAG_EXTERNAL_STREAM: FT_Long = 1 << 10;
pub const FT_FACE_FLAG_HINTER: FT_Long = 1 << 11;
pub const FT_FACE_FLAG_CID_KEYED: FT_Long = 1 << 12;
pub const FT_FACE_FLAG_TRICKY: FT_Long = 1 << 13;
pub const FT_FACE_FLAG_COLOR: FT_Long = 1 << 14;

pub const FT_STYLE_FLAG_ITALIC: FT_Long = 1;
pub const FT_STYLE_FLAG_BOLD: FT_Long = 1 << 1;

pub const FT_OPEN_MEMORY: FT_UInt = 0x1;
pub const FT_OPEN_STREAM: FT_UInt = 0x2;
pub const FT_OPEN_PATHNAME: FT_UInt = 0x4;
pub const FT_OPEN_DRIVER: FT_UInt = 0x8;
pub const FT_OPEN_PARAMS: FT_UInt = 0x10;

pub const FT_SUBGLYPH_FLAG_ARGS_ARE_WORDS: FT_UInt = 1;
pub const FT_SUBGLYPH_FLAG_ARGS_ARE_XY_VALUES: FT_UInt = 2;
pub const FT_SUBGLYPH_FLAG_ROUND_XY_TO_GRID: FT_UInt = 4;
pub const FT_SUBGLYPH_FLAG_SCALE: FT_UInt = 8;
pub const FT_SUBGLYPH_FLAG_XY_SCALE: FT_UInt = 0x40;
pub const FT_SUBGLYPH_FLAG_2X2: FT_UInt = 0x80;
pub const FT_SUBGLYPH_FLAG_USE_MY_METRICS: FT_UInt = 0x200;

pub const FT_LOAD_DEFAULT: FT_Int32 = 0x0;
pub const FT_LOAD_NO_SCALE: FT_Int32 = 0x1;
pub const FT_LOAD_NO_HINTING: FT_Int32 = 0x1 << 1;
pub const FT_LOAD_RENDER: FT_Int32 = 0x1 << 2;
pub const FT_LOAD_NO_BITMAP: FT_Int32 = 0x1 << 3;
pub const FT_LOAD_VERTICAL_LAYOUT: FT_Int32 = 0x1 << 4;
pub const FT_LOAD_FORCE_AUTOHINT: FT_Int32 = 0x1 << 5;
pub const FT_LOAD_CROP_BITMAP: FT_Int32 = 0x1 << 6;
pub const FT_LOAD_PEDANTIC: FT_Int32 = 0x1 << 7;
pub const FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH: FT_Int32 = 0x1 << 9;
pub const FT_LOAD_NO_RECURSE: FT_Int32 = 0x1 << 10;
pub const FT_LOAD_IGNORE_TRANSFORM: FT_Int32 = 0x1 << 11;
pub const FT_LOAD_MONOCHROME: FT_Int32 = 0x1 << 12;
pub const FT_LOAD_LINEAR_DESIGN: FT_Int32 = 0x1 << 13;
pub const FT_LOAD_NO_AUTOHINT: FT_Int32 = 0x1 << 15;
// Bits 16..19 are used by `FT_LOAD_TARGET`
pub const FT_LOAD_COLOR: FT_Int32 = 0x1 << 20;

pub const FT_LOAD_TARGET_NORMAL: FT_Int32 = (FT_RENDER_MODE_NORMAL << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LIGHT: FT_Int32 = (FT_RENDER_MODE_LIGHT << 16) as FT_Int32;
pub const FT_LOAD_TARGET_MONO: FT_Int32 = (FT_RENDER_MODE_MONO << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LCD: FT_Int32 = (FT_RENDER_MODE_LCD << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LCD_V: FT_Int32 = (FT_RENDER_MODE_LCD_V << 16) as FT_Int32;

pub const FT_FSTYPE_INSTALLABLE_EMBEDDING: FT_UShort = 0x0000;
pub const FT_FSTYPE_RESTRICTED_LICENSE_EMBEDDING: FT_UShort = 0x0002;
pub const FT_FSTYPE_PREVIEW_AND_PRINT_EMBEDDING: FT_UShort = 0x0004;
pub const FT_FSTYPE_EDITABLE_EMBEDDING: FT_UShort = 0x0008;
pub const FT_FSTYPE_NO_SUBSETTING: FT_UShort = 0x0100;
pub const FT_FSTYPE_BITMAP_EMBEDDING_ONLY: FT_UShort = 0x0200;

pub const FT_OUTLINE_NONE: c_int = 0x0;
pub const FT_OUTLINE_OWNER: c_int = 0x1;
pub const FT_OUTLINE_EVEN_ODD_FILL: c_int = 0x2;
pub const FT_OUTLINE_REVERSE_FILL: c_int = 0x4;
pub const FT_OUTLINE_IGNORE_DROPOUTS: c_int = 0x8;
pub const FT_OUTLINE_SMART_DROPOUTS: c_int = 0x10;
pub const FT_OUTLINE_INCLUDE_STUBS: c_int = 0x20;
pub const FT_OUTLINE_HIGH_PRECISION: c_int = 0x100;
pub const FT_OUTLINE_SINGLE_PASS: c_int = 0x200;

pub const FT_VAR_AXIS_FLAG_HIDDEN: FT_UInt = 1;

pub const FT_Err_Ok: FT_Error = 0;
pub const FT_Err_Cannot_Open_Resource: FT_Error = 1;
pub const FT_Err_Unknown_File_Format: FT_Error = 2;
pub const FT_Err_Invalid_File_Format: FT_Error = 3;
pub const FT_Err_Invalid_Version: FT_Error = 4;
pub const FT_Err_Lower_Module_Version: FT_Error = 5;
pub const FT_Err_Invalid_Argument: FT_Error = 6;
pub const FT_Err_Unimplemented_Feature: FT_Error = 7;
pub const FT_Err_Invalid_Table: FT_Error = 8;
pub const FT_Err_Invalid_Offset: FT_Error = 9;
pub const FT_Err_Array_Too_Large: FT_Error = 10;
pub const FT_Err_Missing_Module: FT_Error = 11;
pub const FT_Err_Missing_Property: FT_Error = 12;
pub const FT_Err_Invalid_Glyph_Index: FT_Error = 16;
pub const FT_Err_Invalid_Character_Code: FT_Error = 17;
pub const FT_Err_Invalid_Glyph_Format: FT_Error = 18;
pub const FT_Err_Cannot_Render_Glyph: FT_Error = 19;
pub const FT_Err_Invalid_Outline: FT_Error = 20;
pub const FT_Err_Invalid_Composite: FT_Error = 21;
pub const FT_Err_Too_Many_Hints: FT_Error = 22;
pub const FT_Err_Invalid_Pixel_Size: FT_Error = 23;
pub const FT_Err_Invalid_Handle: FT_Error = 32;
pub const FT_Err_Invalid_Library_Handle: FT_Error = 33;
pub const FT_Err_Invalid_Driver_Handle: FT_Error = 34;
pub const FT_Err_Invalid_Face_Handle: FT_Error = 35;
pub const FT_Err_Invalid_Size_Handle: FT_Error = 36;
pub const FT_Err_Invalid_Slot_Handle: FT_Error = 37;
pub const FT_Err_Invalid_CharMap_Handle: FT_Error = 38;
pub const FT_Err_Invalid_Cache_Handle: FT_Error = 39;
pub const FT_Err_Invalid_Stream_Handle: FT_Error = 40;
pub const FT_Err_Too_Many_Drivers: FT_Error = 48;
pub const FT_Err_Too_Many_Extensions: FT_Error = 49;
pub const FT_Err_Out_Of_Memory: FT_Error = 64;
pub const FT_Err_Unlisted_Object: FT_Error = 65;
pub const FT_Err_Cannot_Open_Stream: FT_Error = 81;
pub const FT_Err_Invalid_Stream_Seek: FT_Error = 82;
pub const FT_Err_Invalid_Stream_Skip: FT_Error = 83;
pub const FT_Err_Invalid_Stream_Read: FT_Error = 84;
pub const FT_Err_Invalid_Stream_Operation: FT_Error = 85;
pub const FT_Err_Invalid_Frame_Operation: FT_Error = 86;
pub const FT_Err_Nested_Frame_Access: FT_Error = 87;
pub const FT_Err_Invalid_Frame_Read: FT_Error = 88;
pub const FT_Err_Raster_Uninitialized: FT_Error = 96;
pub const FT_Err_Raster_Corrupted: FT_Error = 97;
pub const FT_Err_Raster_Overflow: FT_Error = 98;
pub const FT_Err_Raster_Negative_Height: FT_Error = 99;
pub const FT_Err_Too_Many_Caches: FT_Error = 112;
pub const FT_Err_Invalid_Opcode: FT_Error = 128;
pub const FT_Err_Too_Few_Arguments: FT_Error = 129;
pub const FT_Err_Stack_Overflow: FT_Error = 130;
pub const FT_Err_Code_Overflow: FT_Error = 131;
pub const FT_Err_Bad_Argument: FT_Error = 132;
pub const FT_Err_Divide_By_Zero: FT_Error = 133;
pub const FT_Err_Invalid_Reference: FT_Error = 134;
pub const FT_Err_Debug_OpCode: FT_Error = 135;
pub const FT_Err_ENDF_In_Exec_Stream: FT_Error = 136;
pub const FT_Err_Nested_DEFS: FT_Error = 137;
pub const FT_Err_Invalid_CodeRange: FT_Error = 138;
pub const FT_Err_Execution_Too_Long: FT_Error = 139;
pub const FT_Err_Too_Many_Function_Defs: FT_Error = 140;
pub const FT_Err_Too_Many_Instruction_Defs: FT_Error = 141;
pub const FT_Err_Table_Missing: FT_Error = 142;
pub const FT_Err_Horiz_Header_Missing: FT_Error = 143;
pub const FT_Err_Locations_Missing: FT_Error = 144;
pub const FT_Err_Name_Table_Missing: FT_Error = 145;
pub const FT_Err_CMap_Table_Missing: FT_Error = 146;
pub const FT_Err_Hmtx_Table_Missing: FT_Error = 147;
pub const FT_Err_Post_Table_Missing: FT_Error = 148;
pub const FT_Err_Invalid_Horiz_Metrics: FT_Error = 149;
pub const FT_Err_Invalid_CharMap_Format: FT_Error = 150;
pub const FT_Err_Invalid_PPem: FT_Error = 151;
pub const FT_Err_Invalid_Vert_Metrics: FT_Error = 152;
pub const FT_Err_Could_Not_Find_Context: FT_Error = 153;
pub const FT_Err_Invalid_Post_Table_Format: FT_Error = 154;
pub const FT_Err_Invalid_Post_Table: FT_Error = 155;
pub const FT_Err_Syntax_Error: FT_Error = 160;
pub const FT_Err_Stack_Underflow: FT_Error = 161;
pub const FT_Err_Ignore: FT_Error = 162;
pub const FT_Err_No_Unicode_Glyph_Name: FT_Error = 163;
pub const FT_Err_Missing_Startfont_Field: FT_Error = 176;
pub const FT_Err_Missing_Font_Field: FT_Error = 177;
pub const FT_Err_Missing_Size_Field: FT_Error = 178;
pub const FT_Err_Missing_Fontboundingbox_Field: FT_Error = 179;
pub const FT_Err_Missing_Chars_Field: FT_Error = 180;
pub const FT_Err_Missing_Startchar_Field: FT_Error = 181;
pub const FT_Err_Missing_Encoding_Field: FT_Error = 182;
pub const FT_Err_Missing_Bbx_Field: FT_Error = 183;
pub const FT_Err_Bbx_Too_Big: FT_Error = 184;
pub const FT_Err_Corrupted_Font_Header: FT_Error = 185;
pub const FT_Err_Corrupted_Font_Glyphs: FT_Error = 186;
pub const FT_Err_Max: FT_Error = 187;

// Objects
pub type FT_Library = *mut FT_LibraryRec;
pub type FT_Face = *mut FT_FaceRec;
pub type FT_Size = *mut FT_SizeRec;
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec;
pub type FT_CharMap = *mut FT_CharMapRec;
pub type FT_Module = *mut FT_ModuleRec;
pub type FT_Driver = *mut FT_DriverRec;
pub type FT_Renderer = *mut FT_RendererRec;
pub type FT_Size_Internal = *mut FT_Size_InternalRec;
pub type FT_SubGlyph = *mut FT_SubGlyphRec;
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec;
pub type FT_Size_Request = *mut FT_Size_RequestRec;
pub type FT_Face_Internal = *mut FT_Face_InternalRec;
pub type FT_Stream = *mut FT_StreamRec;
pub type FT_Memory = *mut FT_MemoryRec;
pub type FT_ListNode = *mut FT_ListNodeRec;
pub type FT_Glyph = *mut FT_GlyphRec;
pub type FT_BitmapGlyph = *mut FT_BitmapGlyphRec;
pub type FT_OutlineGlyph = *mut FT_OutlineGlyphRec;
pub type FT_Stroker = *mut FT_StrokerRec;
pub type TT_OS2_Internal = *mut TT_OS2;
pub type TT_Postscript_Internal = *mut TT_Postscript;

// Internal Types
pub type FT_LibraryRec = c_void;
pub type FT_ModuleRec = c_void;
pub type FT_DriverRec = c_void;
pub type FT_RendererRec = c_void;
pub type FT_Size_InternalRec = c_void;
pub type FT_SubGlyphRec = c_void;
pub type FT_Slot_InternalRec = c_void;
pub type FT_Face_InternalRec = c_void;
pub type FT_StrokerRec = c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_CharMapRec {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_FaceRec {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,

    pub face_flags: FT_Long,
    pub style_flags: FT_Long,

    pub num_glyphs: FT_Long,

    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,

    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,

    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,

    pub generic: FT_Generic,

    pub bbox: FT_BBox,

    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,

    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,

    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,

    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,

    /* @private begin */
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,

    pub sizes_list: FT_ListRec,

    pub autohint: FT_Generic,
    pub extensions: *mut c_void,

    pub internal: FT_Face_Internal,
    /* @private end */
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_GlyphSlotRec {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub reserved: FT_UInt,
    pub generic: FT_Generic,

    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,

    pub format: FT_Glyph_Format,

    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,

    pub outline: FT_Outline,

    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,

    pub control_data: *mut c_void,
    pub control_len: c_long,

    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,

    pub other: *mut c_void,

    pub internal: FT_Slot_Internal,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FT_SizeRec {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_StreamRec {
    pub base: *mut c_uchar,
    pub size: c_ulong,
    pub pos: c_ulong,

    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,

    pub memory: FT_Memory,
    pub cursor: *mut c_uchar,
    pub limit: *mut c_uchar,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_MemoryRec {
    pub user: *mut c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}

unsafe impl Sync for FT_MemoryRec {}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_ListRec {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_ListNodeRec {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Size_RequestRec {
    pub size_request_type: FT_Size_Request_Type, // type
    pub width: FT_Long,
    pub height: FT_Long,
    pub horiResolution: FT_UInt,
    pub vertResolution: FT_UInt,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_GlyphRec {
    pub library: FT_Library,
    pub clazz: *const c_void, // FT_Glyph_Class
    pub format: FT_Glyph_Format,
    pub advance: FT_Vector,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_BitmapGlyphRec {
    pub root: FT_GlyphRec,
    pub left: FT_Int,
    pub top: FT_Int,
    pub bitmap: FT_Bitmap,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_OutlineGlyphRec {
    pub root: FT_GlyphRec,
    pub outline: FT_Outline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FT_Outline_Funcs {
    pub move_to: FT_Outline_MoveToFunc,
    pub line_to: FT_Outline_LineToFunc,
    pub conic_to: FT_Outline_ConicToFunc,
    pub cubic_to: FT_Outline_CubicToFunc,
    pub shift: c_int,
    pub delta: FT_Pos,
}

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
pub struct FT_Raster_Params {
    pub target: *const FT_Bitmap,
    pub source: *const c_void,
    pub flags: c_int,
    pub gray_spans: FT_SpanFunc,
    pub black_spans: FT_SpanFunc,
    pub bit_test: FT_Raster_BitTest_Func,
    pub bit_set: FT_Raster_BitSet_Func,
    pub user: *mut c_void,
    pub clip_box: FT_BBox,
}

// Macro functions
#[inline(always)]
pub fn FT_HAS_HORIZONTAL(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_HORIZONTAL != 0 }
}

#[inline(always)]
pub fn FT_HAS_VERTICAL(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_VERTICAL != 0 }
}

#[inline(always)]
pub fn FT_HAS_KERNING(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_KERNING != 0 }
}

#[inline(always)]
pub fn FT_IS_SCALABLE(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_SCALABLE != 0 }
}

#[inline(always)]
pub fn FT_IS_SFNT(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_SFNT != 0 }
}

#[inline(always)]
pub fn FT_IS_FIXED_WIDTH(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_FIXED_WIDTH != 0 }
}

#[inline(always)]
pub fn FT_HAS_FIXED_SIZES(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_FIXED_SIZES != 0 }
}

#[inline(always)]
pub fn FT_HAS_GLYPH_NAMES(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_GLYPH_NAMES != 0 }
}

#[inline(always)]
pub fn FT_HAS_MULTIPLE_MASTERS(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_MULTIPLE_MASTERS != 0 }
}

#[inline(always)]
pub fn FT_IS_CID_KEYED(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_CID_KEYED != 0 }
}

#[inline(always)]
pub fn FT_IS_TRICKY(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_TRICKY != 0 }
}

#[inline(always)]
pub fn FT_HAS_COLOR(face: FT_Face) -> bool {
    unsafe { (*face).face_flags & FT_FACE_FLAG_COLOR != 0 }
}

extern "C" {
    pub fn FT_Get_Sfnt_Table(face: FT_Face, tag: FT_Sfnt_Tag) -> *mut c_void;
}

extern "C" {
    pub fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    pub fn FT_Done_FreeType(library: FT_Library) -> FT_Error;
    pub fn FT_Set_Default_Properties(library: FT_Library);
    pub fn FT_Property_Get(
        library: FT_Library,
        module_name: *const FT_String,
        property_name: *const FT_String,
        value: *mut c_void,
    ) -> FT_Error;
    pub fn FT_Property_Set(
        library: FT_Library,
        module_name: *const FT_String,
        property_name: *const FT_String,
        value: *const c_void,
    ) -> FT_Error;
    pub fn FT_New_Library(memory: FT_Memory, alibrary: *mut FT_Library) -> FT_Error;
    pub fn FT_Done_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Reference_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Add_Default_Modules(library: FT_Library);
    pub fn FT_New_Face(
        library: FT_Library,
        filepathname: *const c_char,
        face_index: FT_Long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_New_Memory_Face(
        library: FT_Library,
        file_base: *const FT_Byte,
        file_size: FT_Long,
        face_index: FT_Long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_Open_Face(
        library: FT_Library,
        args: *const FT_Open_Args,
        face_index: FT_Long,
        aface: *mut FT_Face,
    ) -> FT_Error;
    pub fn FT_Attach_File(face: FT_Face, filepathname: *const c_char) -> FT_Error;
    pub fn FT_Attach_Stream(face: FT_Face, parameters: *mut FT_Open_Args) -> FT_Error;
    pub fn FT_Reference_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Done_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Select_Size(face: FT_Face, strike_index: FT_Int) -> FT_Error;
    pub fn FT_Request_Size(face: FT_Face, req: FT_Size_Request) -> FT_Error;
    pub fn FT_Set_Char_Size(
        face: FT_Face,
        char_width: FT_F26Dot6,
        char_height: FT_F26Dot6,
        horz_resolution: FT_UInt,
        vert_resolution: FT_UInt,
    ) -> FT_Error;
    pub fn FT_Set_Pixel_Sizes(
        face: FT_Face,
        pixel_width: FT_UInt,
        pixel_height: FT_UInt,
    ) -> FT_Error;
    pub fn FT_Library_SetLcdFilter(library: FT_Library, filter: FT_LcdFilter) -> FT_Error;
    pub fn FT_Load_Glyph(face: FT_Face, glyph_index: FT_UInt, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Load_Char(face: FT_Face, char_code: FT_ULong, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Set_Transform(face: FT_Face, matrix: *mut FT_Matrix, delta: *mut FT_Vector);
    pub fn FT_Render_Glyph(slot: FT_GlyphSlot, render_mode: FT_Render_Mode) -> FT_Error;
    pub fn FT_Get_Kerning(
        face: FT_Face,
        left_glyph: FT_UInt,
        right_glyph: FT_UInt,
        kern_mode: FT_UInt,
        akerning: *mut FT_Vector,
    ) -> FT_Error;
    pub fn FT_Get_Track_Kerning(
        face: FT_Face,
        point_size: FT_Fixed,
        degree: FT_Int,
        akerning: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_Glyph_Name(
        face: FT_Face,
        glyph_index: FT_UInt,
        buffer: FT_Pointer,
        buffer_max: FT_UInt,
    ) -> FT_Error;
    pub fn FT_Get_Postscript_Name(face: FT_Face) -> *const c_char;
    pub fn FT_Select_Charmap(face: FT_Face, encoding: FT_Encoding) -> FT_Error;
    pub fn FT_Set_Charmap(face: FT_Face, charmap: FT_CharMap) -> FT_Error;
    pub fn FT_Get_Charmap_Index(charmap: FT_CharMap) -> FT_Int;
    pub fn FT_Get_Char_Index(face: FT_Face, charcode: FT_ULong) -> FT_UInt;
    pub fn FT_Get_First_Char(face: FT_Face, agindex: *mut FT_UInt) -> FT_ULong;
    pub fn FT_Get_Next_Char(face: FT_Face, char_code: FT_ULong, agindex: *mut FT_UInt) -> FT_ULong;
    pub fn FT_Get_Name_Index(face: FT_Face, glyph_name: *const c_char) -> FT_UInt;
    pub fn FT_Get_SubGlyph_Info(
        glyph: FT_GlyphSlot,
        sub_index: FT_UInt,
        p_index: *mut FT_Int,
        p_flags: *mut FT_UInt,
        p_arg1: *mut FT_Int,
        p_arg2: *mut FT_Int,
        p_transform: *mut FT_Matrix,
    ) -> FT_Error;
    pub fn FT_Get_FSType_Flags(face: FT_Face) -> FT_UShort;
    pub fn FT_Get_Glyph(slot: FT_GlyphSlot, aglyph: *mut FT_Glyph) -> FT_Error;
    pub fn FT_Glyph_Copy(source: FT_Glyph, target: *mut FT_Glyph) -> FT_Error;
    pub fn FT_Glyph_Transform(
        glyph: FT_Glyph,
        matrix: *mut FT_Matrix,
        delta: *mut FT_Vector,
    ) -> FT_Error;
    pub fn FT_Glyph_Get_CBox(glyph: FT_Glyph, bbox_mode: FT_UInt, acbox: *mut FT_BBox);
    pub fn FT_Glyph_To_Bitmap(
        the_glyph: *mut FT_Glyph,
        render_mode: FT_Render_Mode,
        origin: *mut FT_Vector,
        destroy: FT_Bool,
    ) -> FT_Error;
    pub fn FT_Done_Glyph(glyph: FT_Glyph);
    pub fn FT_Outline_GetInsideBorder(outline: *mut FT_Outline) -> FT_StrokerBorder;
    pub fn FT_Outline_GetOutsideBorder(outline: *mut FT_Outline) -> FT_StrokerBorder;
    pub fn FT_Glyph_Stroke(
        pglyph: *mut FT_Glyph,
        stroker: FT_Stroker,
        destroy: FT_Bool,
    ) -> FT_Error;
    pub fn FT_Glyph_StrokeBorder(
        pglyph: *mut FT_Glyph,
        stroker: FT_Stroker,
        inside: FT_Bool,
        outline: FT_Bool,
    ) -> FT_Error;
    pub fn FT_Stroker_New(library: FT_Library, stroker: *mut FT_Stroker) -> FT_Error;
    pub fn FT_Stroker_Set(
        stroker: FT_Stroker,
        radius: FT_Fixed,
        line_cap: FT_Stroker_LineCap,
        line_join: FT_Stroker_LineJoin,
        miter_limit: FT_Fixed,
    );
    pub fn FT_Stroker_Rewind(stroker: FT_Stroker);
    pub fn FT_Stroker_ParseOutline(
        stroker: FT_Stroker,
        outline: *mut FT_Outline,
        opened: FT_Bool,
    ) -> FT_Error;
    pub fn FT_Stroker_Done(stroker: FT_Stroker);
    pub fn FT_Stroker_BeginSubPath(
        stroker: FT_Stroker,
        to: *mut FT_Vector,
        open: FT_Bool,
    ) -> FT_Error;
    pub fn FT_Stroker_EndSubPath(stroker: FT_Stroker) -> FT_Error;
    pub fn FT_Stroker_LineTo(stroker: FT_Stroker, to: *mut FT_Vector) -> FT_Error;
    pub fn FT_Stroker_ConicTo(
        stroker: FT_Stroker,
        control: *mut FT_Vector,
        to: *mut FT_Vector,
    ) -> FT_Error;
    pub fn FT_Stroker_CubicTo(
        stroker: FT_Stroker,
        control1: *mut FT_Vector,
        control2: *mut FT_Vector,
        to: *mut FT_Vector,
    ) -> FT_Error;
    pub fn FT_Stroker_GetBorderCounts(
        stroker: FT_Stroker,
        border: FT_StrokerBorder,
        anum_points: *mut FT_UInt,
        anum_contours: *mut FT_UInt,
    ) -> FT_Error;
    pub fn FT_Stroker_ExportBorder(
        stroker: FT_Stroker,
        border: FT_StrokerBorder,
        outline: *mut FT_Outline,
    );
    pub fn FT_Stroker_GetCounts(
        stroker: FT_Stroker,
        anum_points: *mut FT_UInt,
        anum_contours: *mut FT_UInt,
    ) -> FT_Error;
    pub fn FT_Stroker_Export(stroker: FT_Stroker, outline: *mut FT_Outline);
    pub fn FT_MulDiv(a: FT_Long, b: FT_Long, c: FT_Long) -> FT_Long;
    pub fn FT_MulFix(a: FT_Long, b: FT_Long) -> FT_Long;
    pub fn FT_DivFix(a: FT_Long, b: FT_Long) -> FT_Long;
    pub fn FT_RoundFix(a: FT_Fixed) -> FT_Fixed;
    pub fn FT_CeilFix(a: FT_Fixed) -> FT_Fixed;
    pub fn FT_FloorFix(a: FT_Fixed) -> FT_Fixed;

    pub fn FT_Outline_New(
        library: FT_Library,
        num_points: FT_UInt,
        num_contours: FT_Int,
        anoutline: *mut FT_Outline,
    ) -> FT_Error;
    pub fn FT_Outline_Done(library: FT_Library, outline: *mut FT_Outline) -> FT_Error;
    pub fn FT_Outline_Copy(source: *const FT_Outline, target: *mut FT_Outline) -> FT_Error;
    pub fn FT_Outline_Translate(outline: *const FT_Outline, xOffset: FT_Pos, yOffset: FT_Pos);
    pub fn FT_Outline_Transform(outline: *const FT_Outline, matrix: *const FT_Matrix);
    pub fn FT_Outline_Embolden(outline: *mut FT_Outline, strength: FT_Pos) -> FT_Error;
    pub fn FT_Outline_EmboldenXY(
        outline: *mut FT_Outline,
        xstrength: FT_Pos,
        ystrength: FT_Pos,
    ) -> FT_Error;
    pub fn FT_Outline_Reverse(outline: *mut FT_Outline);
    pub fn FT_Outline_Check(outline: *mut FT_Outline) -> FT_Error;

    pub fn FT_Outline_Get_CBox(outline: *const FT_Outline, acbox: *mut FT_BBox);
    pub fn FT_Outline_Get_BBox(outline: *const FT_Outline, abbox: *mut FT_BBox) -> FT_Error;

    pub fn FT_Outline_Get_Bitmap(
        library: FT_Library,
        outline: *mut FT_Outline,
        abitmap: *const FT_Bitmap,
    ) -> FT_Error;
    pub fn FT_Outline_Render(
        library: FT_Library,
        outline: *mut FT_Outline,
        params: *mut FT_Raster_Params,
    ) -> FT_Error;
    pub fn FT_Outline_Decompose(
        outline: *mut FT_Outline,
        func_interface: *const FT_Outline_Funcs,
        user: *mut c_void,
    ) -> FT_Error;

    pub fn FT_Outline_Get_Orientation(outline: *mut FT_Outline) -> FT_Orientation;

    pub fn FT_GlyphSlot_Embolden(slot: FT_GlyphSlot);
    pub fn FT_GlyphSlot_Oblique(slot: FT_GlyphSlot);

    pub fn FT_Get_Multi_Master(face: FT_Face, amaster: *mut FT_Multi_Master) -> FT_Error;
    pub fn FT_Get_MM_Var(face: FT_Face, amaster: *mut *mut FT_MM_Var) -> FT_Error;
    pub fn FT_Done_MM_Var(library: FT_Library, amaster: *mut FT_MM_Var) -> FT_Error;
    pub fn FT_Set_MM_Design_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *const FT_Long,
    ) -> FT_Error;
    pub fn FT_Set_Var_Design_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *const FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_Var_Design_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Set_MM_Blend_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *const FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_MM_Blend_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Set_Var_Blend_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *const FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_Var_Blend_Coordinates(
        face: FT_Face,
        num_coords: FT_UInt,
        coords: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Set_MM_WeightVector(
        face: FT_Face,
        len: FT_UInt,
        weightvector: *const FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_MM_WeightVector(
        face: FT_Face,
        len: *mut FT_UInt,
        weightvector: *mut FT_Fixed,
    ) -> FT_Error;
    pub fn FT_Get_Var_Axis_Flags(
        master: *const FT_MM_Var,
        axis_index: FT_UInt,
        flags: *mut FT_UInt,
    ) -> FT_Error;
    pub fn FT_Set_Named_Instance(face: FT_Face, instance_index: FT_UInt) -> FT_Error;

    pub fn FT_Get_Sfnt_Name_Count(face: FT_Face) -> FT_UInt;
    pub fn FT_Get_Sfnt_Name(face: FT_Face, idx: FT_UInt, aname: *mut FT_SfntName) -> FT_Error;
    pub fn FT_Get_Sfnt_LangTag(
        face: FT_Face,
        langID: FT_UInt,
        alangtag: *mut FT_SfntName,
    ) -> FT_Error;

    pub fn FT_Face_GetCharVariantIndex(
        face: FT_Face,
        charcode: FT_ULong,
        variant_selector: FT_ULong,
    ) -> FT_UInt;
    pub fn FT_Face_GetCharVariantIsDefault(
        face: FT_Face,
        charcode: FT_ULong,
        variant_selector: FT_ULong,
    ) -> FT_Int;
    pub fn FT_Face_GetVariantSelectors(face: FT_Face) -> *mut FT_UInt;
    pub fn FT_Face_GetVariantsOfChar(face: FT_Face, charcode: FT_ULong) -> *mut FT_UInt32;
    pub fn FT_Face_GetCharsOfVariant(face: FT_Face, variant_selector: FT_ULong) -> *mut FT_UInt32;

    pub fn FT_Get_Color_Glyph_Layer(
        face: FT_Face,
        base_glyph: FT_UInt,
        aglyph_index: *mut FT_UInt,
        acolor_index: *mut FT_UInt,
        iterator: *mut FT_LayerIterator,
    ) -> bool;
}
