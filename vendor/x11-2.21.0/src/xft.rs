// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::*;

use super::xlib::{Display, Region, Visual, XRectangle};
use super::xrender::{XGlyphInfo, XRenderColor};

//
// external types
//

// freetype
pub enum FT_FaceRec {}
pub type FT_UInt = c_uint;

// fontconfig
pub type FcChar32 = c_uint;
pub enum FcCharSet {}
pub enum FcPattern {}

#[repr(C)]
pub enum FcEndian {
    Big,
    Little,
}

#[repr(C)]
pub enum FcResult {
    Match,
    NoMatch,
    TypeMismatch,
    NoId,
    OutOfMemory,
}

//
// functions
//

x11_link! { Xft, xft, ["libXft.so.2", "libXft.so"], 77,
    pub fn XftCharExists (_2: *mut Display, _1: *mut XftFont, _0: c_uint) -> c_int,
    pub fn XftCharFontSpecRender (_7: *mut Display, _6: c_int, _5: c_ulong, _4: c_ulong, _3: c_int, _2: c_int, _1: *const XftCharFontSpec, _0: c_int) -> (),
    pub fn XftCharIndex (_2: *mut Display, _1: *mut XftFont, _0: c_uint) -> c_uint,
    pub fn XftCharSpecRender (_8: *mut Display, _7: c_int, _6: c_ulong, _5: *mut XftFont, _4: c_ulong, _3: c_int, _2: c_int, _1: *const XftCharSpec, _0: c_int) -> (),
    pub fn XftColorAllocName (_4: *mut Display, _3: *const Visual, _2: c_ulong, _1: *const c_char, _0: *mut XftColor) -> c_int,
    pub fn XftColorAllocValue (_4: *mut Display, _3: *mut Visual, _2: c_ulong, _1: *const XRenderColor, _0: *mut XftColor) -> c_int,
    pub fn XftColorFree (_3: *mut Display, _2: *mut Visual, _1: c_ulong, _0: *mut XftColor) -> (),
    pub fn XftDefaultHasRender (_0: *mut Display) -> c_int,
    pub fn XftDefaultSet (_1: *mut Display, _0: *mut FcPattern) -> c_int,
    pub fn XftDefaultSubstitute (_2: *mut Display, _1: c_int, _0: *mut FcPattern) -> (),
    pub fn XftDrawChange (_1: *mut XftDraw, _0: c_ulong) -> (),
    pub fn XftDrawCharFontSpec (_3: *mut XftDraw, _2: *const XftColor, _1: *const XftCharFontSpec, _0: c_int) -> (),
    pub fn XftDrawCharSpec (_4: *mut XftDraw, _3: *const XftColor, _2: *mut XftFont, _1: *const XftCharSpec, _0: c_int) -> (),
    pub fn XftDrawColormap (_0: *mut XftDraw) -> c_ulong,
    pub fn XftDrawCreate (_3: *mut Display, _2: c_ulong, _1: *mut Visual, _0: c_ulong) -> *mut XftDraw,
    pub fn XftDrawCreateAlpha (_2: *mut Display, _1: c_ulong, _0: c_int) -> *mut XftDraw,
    pub fn XftDrawCreateBitmap (_1: *mut Display, _0: c_ulong) -> *mut XftDraw,
    pub fn XftDrawDestroy (_0: *mut XftDraw) -> (),
    pub fn XftDrawDisplay (_0: *mut XftDraw) -> *mut Display,
    pub fn XftDrawDrawable (_0: *mut XftDraw) -> c_ulong,
    pub fn XftDrawGlyphFontSpec (_3: *mut XftDraw, _2: *const XftColor, _1: *const XftGlyphFontSpec, _0: c_int) -> (),
    pub fn XftDrawGlyphs (_6: *mut XftDraw, _5: *const XftColor, _4: *mut XftFont, _3: c_int, _2: c_int, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftDrawGlyphSpec (_4: *mut XftDraw, _3: *const XftColor, _2: *mut XftFont, _1: *const XftGlyphSpec, _0: c_int) -> (),
    pub fn XftDrawPicture (_0: *mut XftDraw) -> c_ulong,
    pub fn XftDrawRect (_5: *mut XftDraw, _4: *const XftColor, _3: c_int, _2: c_int, _1: c_uint, _0: c_uint) -> (),
    pub fn XftDrawSetClip (_1: *mut XftDraw, _0: Region) -> c_int,
    pub fn XftDrawSetClipRectangles (_4: *mut XftDraw, _3: c_int, _2: c_int, _1: *const XRectangle, _0: c_int) -> c_int,
    pub fn XftDrawSetSubwindowMode (_1: *mut XftDraw, _0: c_int) -> (),
    pub fn XftDrawSrcPicture (_1: *mut XftDraw, _0: *const XftColor) -> c_ulong,
    pub fn XftDrawString16 (_6: *mut XftDraw, _5: *const XftColor, _4: *mut XftFont, _3: c_int, _2: c_int, _1: *const c_ushort, _0: c_int) -> (),
    pub fn XftDrawString32 (_6: *mut XftDraw, _5: *const XftColor, _4: *mut XftFont, _3: c_int, _2: c_int, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftDrawString8 (_6: *mut XftDraw, _5: *const XftColor, _4: *mut XftFont, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftDrawStringUtf16 (_7: *mut XftDraw, _6: *const XftColor, _5: *mut XftFont, _4: c_int, _3: c_int, _2: *const c_uchar, _1: FcEndian, _0: c_int) -> (),
    pub fn XftDrawStringUtf8 (_6: *mut XftDraw, _5: *const XftColor, _4: *mut XftFont, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftDrawVisual (_0: *mut XftDraw) -> *mut Visual,
    pub fn XftFontCheckGlyph (_5: *mut Display, _4: *mut XftFont, _3: c_int, _2: c_uint, _1: *mut c_uint, _0: *mut c_int) -> c_int,
    pub fn XftFontClose (_1: *mut Display, _0: *mut XftFont) -> (),
    pub fn XftFontCopy (_1: *mut Display, _0: *mut XftFont) -> *mut XftFont,
    pub fn XftFontInfoCreate (_1: *mut Display, _0: *const FcPattern) -> *mut XftFontInfo,
    pub fn XftFontInfoDestroy (_1: *mut Display, _0: *mut XftFontInfo) -> (),
    pub fn XftFontInfoEqual (_1: *const XftFontInfo, _0: *const XftFontInfo) -> c_int,
    pub fn XftFontInfoHash (_0: *const XftFontInfo) -> c_uint,
    pub fn XftFontLoadGlyphs (_4: *mut Display, _3: *mut XftFont, _2: c_int, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftFontMatch (_3: *mut Display, _2: c_int, _1: *const FcPattern, _0: *mut FcResult) -> *mut FcPattern,
    pub fn XftFontOpenInfo (_2: *mut Display, _1: *mut FcPattern, _0: *mut XftFontInfo) -> *mut XftFont,
    pub fn XftFontOpenName (_2: *mut Display, _1: c_int, _0: *const c_char) -> *mut XftFont,
    pub fn XftFontOpenPattern (_1: *mut Display, _0: *mut FcPattern) -> *mut XftFont,
    pub fn XftFontOpenXlfd (_2: *mut Display, _1: c_int, _0: *const c_char) -> *mut XftFont,
    pub fn XftFontUnloadGlyphs (_3: *mut Display, _2: *mut XftFont, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftGetVersion () -> c_int,
    pub fn XftGlyphExtents (_4: *mut Display, _3: *mut XftFont, _2: *const c_uint, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftGlyphFontSpecRender (_7: *mut Display, _6: c_int, _5: c_ulong, _4: c_ulong, _3: c_int, _2: c_int, _1: *const XftGlyphFontSpec, _0: c_int) -> (),
    pub fn XftGlyphRender (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftGlyphSpecRender (_8: *mut Display, _7: c_int, _6: c_ulong, _5: *mut XftFont, _4: c_ulong, _3: c_int, _2: c_int, _1: *const XftGlyphSpec, _0: c_int) -> (),
    pub fn XftInit (_0: *const c_char) -> c_int,
    pub fn XftInitFtLibrary () -> c_int,
    pub fn XftLockFace (_0: *mut XftFont) -> *mut FT_FaceRec,
    pub fn XftNameParse (_0: *const c_char) -> *mut FcPattern,
    pub fn XftNameUnparse (_2: *mut FcPattern, _1: *mut c_char, _0: c_int) -> c_int,
    pub fn XftTextExtents16 (_4: *mut Display, _3: *mut XftFont, _2: *const c_ushort, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftTextExtents32 (_4: *mut Display, _3: *mut XftFont, _2: *const c_uint, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftTextExtents8 (_4: *mut Display, _3: *mut XftFont, _2: *const c_uchar, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftTextExtentsUtf16 (_5: *mut Display, _4: *mut XftFont, _3: *const c_uchar, _2: FcEndian, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftTextExtentsUtf8 (_4: *mut Display, _3: *mut XftFont, _2: *const c_uchar, _1: c_int, _0: *mut XGlyphInfo) -> (),
    pub fn XftTextRender16 (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_ushort, _0: c_int) -> (),
    pub fn XftTextRender16BE (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftTextRender16LE (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftTextRender32 (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uint, _0: c_int) -> (),
    pub fn XftTextRender32BE (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftTextRender32LE (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftTextRender8 (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftTextRenderUtf16 (_11: *mut Display, _10: c_int, _9: c_ulong, _8: *mut XftFont, _7: c_ulong, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const c_uchar, _1: FcEndian, _0: c_int) -> (),
    pub fn XftTextRenderUtf8 (_10: *mut Display, _9: c_int, _8: c_ulong, _7: *mut XftFont, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: *const c_uchar, _0: c_int) -> (),
    pub fn XftUnlockFace (_0: *mut XftFont) -> (),
    pub fn XftXlfdParse (_2: *const c_char, _1: c_int, _0: c_int) -> *mut FcPattern,
variadic:
    pub fn XftFontOpen (_1: *mut Display, _0: c_int) -> *mut XftFont,
    pub fn XftListFonts (_1: *mut Display, _0: c_int) -> *mut XftFontSet,
globals:
}

//
// types
//

pub enum XftFontInfo {}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftFont {
    pub ascent: c_int,
    pub descent: c_int,
    pub height: c_int,
    pub max_advance_width: c_int,
    pub charset: *mut FcCharSet,
    pub pattern: *mut FcPattern,
}

pub enum XftDraw {}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftColor {
    pub pixel: c_ulong,
    pub color: XRenderColor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftCharSpec {
    pub ucs4: FcChar32,
    pub x: c_short,
    pub y: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftCharFontSpec {
    pub font: *mut XftFont,
    pub ucs4: FcChar32,
    pub x: c_short,
    pub y: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftFontSet {
    pub nfont: c_int,
    pub sfont: c_int,
    pub fonts: *mut *mut XftPattern,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftGlyphSpec {
    pub glyph: FT_UInt,
    pub x: c_short,
    pub y: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XftGlyphFontSpec {
    pub font: *mut XftFont,
    pub glyph: FT_UInt,
    pub x: c_short,
    pub y: c_short,
}

pub enum XftPattern {}

//
// constants
//

// font attributes
pub const XFT_FAMILY: &str = "family";
pub const XFT_STYLE: &str = "style";
pub const XFT_SLANT: &str = "slant";
pub const XFT_WEIGHT: &str = "weight";
pub const XFT_SIZE: &str = "size";
pub const XFT_PIXEL_SIZE: &str = "pixelsize";
pub const XFT_SPACING: &str = "spacing";
pub const XFT_FOUNDRY: &str = "foundry";
pub const XFT_ANTIALIAS: &str = "antialias";

// slant values
pub const XFT_SLANT_ROMAN: c_int = 0;
pub const XFT_SLANT_ITALIC: c_int = 100;
pub const XFT_SLANT_OBLIQUE: c_int = 110;

// attribute types
pub const XftTypeVoid: c_int = 0;
pub const XftTypeInteger: c_int = 1;
pub const XftTypeDouble: c_int = 2;
pub const XftTypeString: c_int = 3;
pub const XftTypeBool: c_int = 4;
pub const XftTypeMatrix: c_int = 5;
