// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{c_char, c_double, c_int, c_short, c_uint, c_ulong, c_ushort};

use super::xlib::{Atom, Bool, Colormap, Cursor, Display, Pixmap, Region, Visual, XRectangle, XID};

//
// functions
//

x11_link! { Xrender, xrender, ["libXrender.so.1", "libXrender.so"], 44,
  pub fn XRenderAddGlyphs (_7: *mut Display, _6: c_ulong, _5: *const c_ulong, _4: *const XGlyphInfo, _3: c_int, _2: *const c_char, _1: c_int) -> (),
  pub fn XRenderAddTraps (_6: *mut Display, _5: c_ulong, _4: c_int, _3: c_int, _2: *const XTrap, _1: c_int) -> (),
  pub fn XRenderChangePicture (_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: *const XRenderPictureAttributes) -> (),
  pub fn XRenderComposite (_13: *mut Display, _12: c_int, _11: c_ulong, _10: c_ulong, _9: c_ulong, _8: c_int, _7: c_int, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> (),
  pub fn XRenderCompositeDoublePoly (_12: *mut Display, _11: c_int, _10: c_ulong, _9: c_ulong, _8: *const XRenderPictFormat, _7: c_int, _6: c_int, _5: c_int, _4: c_int, _3: *const XPointDouble, _2: c_int, _1: c_int) -> (),
  pub fn XRenderCompositeString16 (_12: *mut Display, _11: c_int, _10: c_ulong, _9: c_ulong, _8: *const XRenderPictFormat, _7: c_ulong, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const c_ushort, _1: c_int) -> (),
  pub fn XRenderCompositeString32 (_12: *mut Display, _11: c_int, _10: c_ulong, _9: c_ulong, _8: *const XRenderPictFormat, _7: c_ulong, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const c_uint, _1: c_int) -> (),
  pub fn XRenderCompositeString8 (_12: *mut Display, _11: c_int, _10: c_ulong, _9: c_ulong, _8: *const XRenderPictFormat, _7: c_ulong, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> (),
  pub fn XRenderCompositeText16 (_11: *mut Display, _10: c_int, _9: c_ulong, _8: c_ulong, _7: *const XRenderPictFormat, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const XGlyphElt16, _1: c_int) -> (),
  pub fn XRenderCompositeText32 (_11: *mut Display, _10: c_int, _9: c_ulong, _8: c_ulong, _7: *const XRenderPictFormat, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const XGlyphElt32, _1: c_int) -> (),
  pub fn XRenderCompositeText8 (_11: *mut Display, _10: c_int, _9: c_ulong, _8: c_ulong, _7: *const XRenderPictFormat, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: *const XGlyphElt8, _1: c_int) -> (),
  pub fn XRenderCompositeTrapezoids (_9: *mut Display, _8: c_int, _7: c_ulong, _6: c_ulong, _5: *const XRenderPictFormat, _4: c_int, _3: c_int, _2: *const XTrapezoid, _1: c_int) -> (),
  pub fn XRenderCompositeTriangles (_9: *mut Display, _8: c_int, _7: c_ulong, _6: c_ulong, _5: *const XRenderPictFormat, _4: c_int, _3: c_int, _2: *const XTriangle, _1: c_int) -> (),
  pub fn XRenderCompositeTriFan (_9: *mut Display, _8: c_int, _7: c_ulong, _6: c_ulong, _5: *const XRenderPictFormat, _4: c_int, _3: c_int, _2: *const XPointFixed, _1: c_int) -> (),
  pub fn XRenderCompositeTriStrip (_9: *mut Display, _8: c_int, _7: c_ulong, _6: c_ulong, _5: *const XRenderPictFormat, _4: c_int, _3: c_int, _2: *const XPointFixed, _1: c_int) -> (),
  pub fn XRenderCreateAnimCursor (_3: *mut Display, _2: c_int, _1: *mut XAnimCursor) -> c_ulong,
  pub fn XRenderCreateConicalGradient (_5: *mut Display, _4: *const XConicalGradient, _3: *const c_int, _2: *const XRenderColor, _1: c_int) -> c_ulong,
  pub fn XRenderCreateCursor (_4: *mut Display, _3: c_ulong, _2: c_uint, _1: c_uint) -> c_ulong,
  pub fn XRenderCreateGlyphSet (_2: *mut Display, _1: *const XRenderPictFormat) -> c_ulong,
  pub fn XRenderCreateLinearGradient (_5: *mut Display, _4: *const XLinearGradient, _3: *const c_int, _2: *const XRenderColor, _1: c_int) -> c_ulong,
  pub fn XRenderCreatePicture (_5: *mut Display, _4: c_ulong, _3: *const XRenderPictFormat, _2: c_ulong, _1: *const XRenderPictureAttributes) -> c_ulong,
  pub fn XRenderCreateRadialGradient (_5: *mut Display, _4: *const XRadialGradient, _3: *const c_int, _2: *const XRenderColor, _1: c_int) -> c_ulong,
  pub fn XRenderCreateSolidFill (_2: *mut Display, _1: *const XRenderColor) -> c_ulong,
  pub fn XRenderFillRectangle (_8: *mut Display, _7: c_int, _6: c_ulong, _5: *const XRenderColor, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> (),
  pub fn XRenderFillRectangles (_6: *mut Display, _5: c_int, _4: c_ulong, _3: *const XRenderColor, _2: *const XRectangle, _1: c_int) -> (),
  pub fn XRenderFindFormat (_4: *mut Display, _3: c_ulong, _2: *const XRenderPictFormat, _1: c_int) -> *mut XRenderPictFormat,
  pub fn XRenderFindStandardFormat (_2: *mut Display, _1: c_int) -> *mut XRenderPictFormat,
  pub fn XRenderFindVisualFormat (_2: *mut Display, _1: *const Visual) -> *mut XRenderPictFormat,
  pub fn XRenderFreeGlyphs (_4: *mut Display, _3: c_ulong, _2: *const c_ulong, _1: c_int) -> (),
  pub fn XRenderFreeGlyphSet (_2: *mut Display, _1: c_ulong) -> (),
  pub fn XRenderFreePicture (_2: *mut Display, _1: c_ulong) -> (),
  pub fn XRenderParseColor (_3: *mut Display, _2: *mut c_char, _1: *mut XRenderColor) -> c_int,
  pub fn XRenderQueryExtension (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XRenderQueryFilters (_2: *mut Display, _1: c_ulong) -> *mut XFilters,
  pub fn XRenderQueryFormats (_1: *mut Display) -> c_int,
  pub fn XRenderQueryPictIndexValues (_3: *mut Display, _2: *const XRenderPictFormat, _1: *mut c_int) -> *mut XIndexValue,
  pub fn XRenderQuerySubpixelOrder (_2: *mut Display, _1: c_int) -> c_int,
  pub fn XRenderQueryVersion (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XRenderReferenceGlyphSet (_2: *mut Display, _1: c_ulong) -> c_ulong,
  pub fn XRenderSetPictureClipRectangles (_6: *mut Display, _5: c_ulong, _4: c_int, _3: c_int, _2: *const XRectangle, _1: c_int) -> (),
  pub fn XRenderSetPictureClipRegion (_3: *mut Display, _2: c_ulong, _1: Region) -> (),
  pub fn XRenderSetPictureFilter (_5: *mut Display, _4: c_ulong, _3: *const c_char, _2: *mut c_int, _1: c_int) -> (),
  pub fn XRenderSetPictureTransform (_3: *mut Display, _2: c_ulong, _1: *mut XTransform) -> (),
  pub fn XRenderSetSubpixelOrder (_3: *mut Display, _2: c_int, _1: c_int) -> c_int,
variadic:
globals:
}

//
// types
//

pub type Glyph = XID;
pub type GlyphSet = XID;
pub type PictFormat = XID;
pub type Picture = XID;
pub type XDouble = c_double;
pub type XFixed = c_int;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XAnimCursor {
    pub cursor: Cursor,
    pub delay: c_ulong,
}
pub type XAnimCursor = _XAnimCursor;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XCircle {
    pub x: XFixed,
    pub y: XFixed,
    pub radius: XFixed,
}
pub type XCircle = _XCircle;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XConicalGradient {
    pub center: XPointFixed,
    pub angle: XFixed,
}
pub type XConicalGradient = _XConicalGradient;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XFilters {
    pub nfilter: c_int,
    pub filter: *mut *mut c_char,
    pub nalias: c_int,
    pub alias: *mut c_short,
}
pub type XFilters = _XFilters;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XGlyphElt8 {
    pub glyphset: GlyphSet,
    pub chars: *mut c_char,
    pub nchars: c_int,
    pub xOff: c_int,
    pub yOff: c_int,
}
pub type XGlyphElt8 = _XGlyphElt8;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XGlyphElt16 {
    pub glyphset: GlyphSet,
    pub chars: *mut c_ushort,
    pub nchars: c_int,
    pub xOff: c_int,
    pub yOff: c_int,
}
pub type XGlyphElt16 = _XGlyphElt16;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XGlyphElt32 {
    pub glyphset: GlyphSet,
    pub chars: *mut c_uint,
    pub nchars: c_int,
    pub xOff: c_int,
    pub yOff: c_int,
}
pub type XGlyphElt32 = _XGlyphElt32;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XGlyphInfo {
    pub width: c_ushort,
    pub height: c_ushort,
    pub x: c_short,
    pub y: c_short,
    pub xOff: c_short,
    pub yOff: c_short,
}
pub type XGlyphInfo = _XGlyphInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XIndexValue {
    pub pixel: c_ulong,
    pub red: c_ushort,
    pub green: c_ushort,
    pub blue: c_ushort,
    pub alpha: c_ushort,
}
pub type XIndexValue = _XIndexValue;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XLinearGradient {
    pub p1: XPointFixed,
    pub p2: XPointFixed,
}
pub type XLinearGradient = _XLinearGradient;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XLineFixed {
    pub p1: XPointFixed,
    pub p2: XPointFixed,
}
pub type XLineFixed = _XLineFixed;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XPointDouble {
    pub x: XDouble,
    pub y: XDouble,
}
pub type XPointDouble = _XPointDouble;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XPointFixed {
    pub x: XFixed,
    pub y: XFixed,
}
pub type XPointFixed = _XPointFixed;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XRadialGradient {
    pub inner: XCircle,
    pub outer: XCircle,
}
pub type XRadialGradient = _XRadialGradient;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRenderColor {
    pub red: c_ushort,
    pub green: c_ushort,
    pub blue: c_ushort,
    pub alpha: c_ushort,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRenderDirectFormat {
    pub red: c_short,
    pub redMask: c_short,
    pub green: c_short,
    pub greenMask: c_short,
    pub blue: c_short,
    pub blueMask: c_short,
    pub alpha: c_short,
    pub alphaMask: c_short,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRenderPictFormat {
    pub id: PictFormat,
    pub type_: c_int,
    pub depth: c_int,
    pub direct: XRenderDirectFormat,
    pub colormap: Colormap,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XRenderPictureAttributes {
    pub repeat: c_int,
    pub alpha_map: Picture,
    pub alpha_x_origin: c_int,
    pub alpha_y_origin: c_int,
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,
    pub clip_mask: Pixmap,
    pub graphics_exposures: Bool,
    pub subwindow_mode: c_int,
    pub poly_edge: c_int,
    pub poly_mode: c_int,
    pub dither: Atom,
    pub component_alpha: Bool,
}
pub type XRenderPictureAttributes = _XRenderPictureAttributes;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XSpanFix {
    pub left: XFixed,
    pub right: XFixed,
    pub y: XFixed,
}
pub type XSpanFix = _XSpanFix;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XTrap {
    pub top: XSpanFix,
    pub bottom: XSpanFix,
}
pub type XTrap = _XTrap;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XTrapezoid {
    pub top: XFixed,
    pub bottom: XFixed,
    pub left: XLineFixed,
    pub right: XLineFixed,
}
pub type XTrapezoid = _XTrapezoid;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XTriangle {
    pub p1: XPointFixed,
    pub p2: XPointFixed,
    pub p3: XPointFixed,
}
pub type XTriangle = _XTriangle;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XTransform {
    pub matrix: [[XFixed; 3]; 3],
}
pub type XTransform = _XTransform;

//
// constants
//

// pict format mask
pub const PictFormatID: c_ulong = 1 << 0;
pub const PictFormatType: c_ulong = 1 << 1;
pub const PictFormatDepth: c_ulong = 1 << 2;
pub const PictFormatRed: c_ulong = 1 << 3;
pub const PictFormatRedMask: c_ulong = 1 << 4;
pub const PictFormatGreen: c_ulong = 1 << 5;
pub const PictFormatGreenMask: c_ulong = 1 << 6;
pub const PictFormatBlue: c_ulong = 1 << 7;
pub const PictFormatBlueMask: c_ulong = 1 << 8;
pub const PictFormatAlpha: c_ulong = 1 << 9;
pub const PictFormatAlphaMask: c_ulong = 1 << 10;
pub const PictFormatColormap: c_ulong = 1 << 11;

// error codes
pub const BadPictFormat: c_int = 0;
pub const BadPicture: c_int = 1;
pub const BadPictOp: c_int = 2;
pub const BadGlyphSet: c_int = 3;
pub const BadGlyph: c_int = 4;
pub const RenderNumberErrors: c_int = BadGlyph + 1;

// pict types
pub const PictTypeIndexed: c_int = 0;
pub const PictTypeDirect: c_int = 1;

// ops
pub const PictOpMinimum: c_int = 0;
pub const PictOpClear: c_int = 0;
pub const PictOpSrc: c_int = 1;
pub const PictOpDst: c_int = 2;
pub const PictOpOver: c_int = 3;
pub const PictOpOverReverse: c_int = 4;
pub const PictOpIn: c_int = 5;
pub const PictOpInReverse: c_int = 6;
pub const PictOpOut: c_int = 7;
pub const PictOpOutReverse: c_int = 8;
pub const PictOpAtop: c_int = 9;
pub const PictOpAtopReverse: c_int = 10;
pub const PictOpXor: c_int = 11;
pub const PictOpAdd: c_int = 12;
pub const PictOpSaturate: c_int = 13;
pub const PictOpMaximum: c_int = 13;

pub const PictOpDisjointMinimum: c_int = 0x10;
pub const PictOpDisjointClear: c_int = 0x10;
pub const PictOpDisjointSrc: c_int = 0x11;
pub const PictOpDisjointDst: c_int = 0x12;
pub const PictOpDisjointOver: c_int = 0x13;
pub const PictOpDisjointOverReverse: c_int = 0x14;
pub const PictOpDisjointIn: c_int = 0x15;
pub const PictOpDisjointInReverse: c_int = 0x16;
pub const PictOpDisjointOut: c_int = 0x17;
pub const PictOpDisjointOutReverse: c_int = 0x18;
pub const PictOpDisjointAtop: c_int = 0x19;
pub const PictOpDisjointAtopReverse: c_int = 0x1a;
pub const PictOpDisjointXor: c_int = 0x1b;
pub const PictOpDisjointMaximum: c_int = 0x1b;

pub const PictOpConjointMinimum: c_int = 0x20;
pub const PictOpConjointClear: c_int = 0x20;
pub const PictOpConjointSrc: c_int = 0x21;
pub const PictOpConjointDst: c_int = 0x22;
pub const PictOpConjointOver: c_int = 0x23;
pub const PictOpConjointOverReverse: c_int = 0x24;
pub const PictOpConjointIn: c_int = 0x25;
pub const PictOpConjointInReverse: c_int = 0x26;
pub const PictOpConjointOut: c_int = 0x27;
pub const PictOpConjointOutReverse: c_int = 0x28;
pub const PictOpConjointAtop: c_int = 0x29;
pub const PictOpConjointAtopReverse: c_int = 0x2a;
pub const PictOpConjointXor: c_int = 0x2b;
pub const PictOpConjointMaximum: c_int = 0x2b;

pub const PictOpBlendMinimum: c_int = 0x30;
pub const PictOpMultiply: c_int = 0x30;
pub const PictOpScreen: c_int = 0x31;
pub const PictOpOverlay: c_int = 0x32;
pub const PictOpDarken: c_int = 0x33;
pub const PictOpLighten: c_int = 0x34;
pub const PictOpColorDodge: c_int = 0x35;
pub const PictOpColorBurn: c_int = 0x36;
pub const PictOpHardLight: c_int = 0x37;
pub const PictOpSoftLight: c_int = 0x38;
pub const PictOpDifference: c_int = 0x39;
pub const PictOpExclusion: c_int = 0x3a;
pub const PictOpHSLHue: c_int = 0x3b;
pub const PictOpHSLSaturation: c_int = 0x3c;
pub const PictOpHSLColor: c_int = 0x3d;
pub const PictOpHSLLuminosity: c_int = 0x3e;
pub const PictOpBlendMaximum: c_int = 0x3e;

pub const PictStandardARGB32: c_int = 0;
pub const PictStandardRGB24: c_int = 0;
pub const PictStandardA8: c_int = 2;
pub const PictStandardA4: c_int = 3;
pub const PictStandardA1: c_int = 4;

// poly edge types
pub const PolyEdgeSharp: c_int = 0;
pub const PolyEdgeSmooth: c_int = 1;

// poly modes
pub const PolyModePrecise: c_int = 0;
pub const PolyModeImprecise: c_int = 1;

// picture attributes mask
pub const CPRepeat: c_int = 1 << 0;
pub const CPAlphaMap: c_int = 1 << 1;
pub const CPAlphaXOrigin: c_int = 1 << 2;
pub const CPAlphaYOrigin: c_int = 1 << 3;
pub const CPClipXOrigin: c_int = 1 << 4;
pub const CPClipYOrigin: c_int = 1 << 5;
pub const CPClipMask: c_int = 1 << 6;
pub const CPGraphicsExposure: c_int = 1 << 7;
pub const CPSubwindowMode: c_int = 1 << 8;
pub const CPPolyEdge: c_int = 1 << 9;
pub const CPPolyMode: c_int = 1 << 10;
pub const CPDither: c_int = 1 << 11;
pub const CPComponentAlpha: c_int = 1 << 12;
pub const CPLastBit: c_int = 12;

// filter methods
pub const FilterNearest: &str = "nearest";
pub const FilterBilinear: &str = "bilinear";
pub const FilterConvolution: &str = "convolution";
pub const FilterFast: &str = "fast";
pub const FilterGood: &str = "good";
pub const FilterBest: &str = "best";

// subpixel orders
pub const SubPixelUnknown: c_int = 0;
pub const SubPixelHorizontalRGB: c_int = 1;
pub const SubPixelHorizontalBGR: c_int = 2;
pub const SubPixelVerticalRGB: c_int = 3;
pub const SubPixelVerticalBGR: c_int = 4;
pub const SubPixelNone: c_int = 5;

// repeat attributes
pub const RepeatNone: c_int = 0;
pub const RepeatNormal: c_int = 1;
pub const RepeatPad: c_int = 2;
pub const RepeatReflect: c_int = 3;
