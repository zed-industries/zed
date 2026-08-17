// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! GDI procedure declarations, constant definitions and macros
use ctypes::{c_char, c_int, c_long, c_short, c_ushort, c_void};
use shared::basetsd::{UINT16, UINT32, UINT64, ULONG_PTR};
use shared::minwindef::{
    BOOL, BYTE, DWORD, FLOAT, HGLOBAL, HMETAFILE, HMODULE, HRGN, INT, LOBYTE, LPARAM, LPBYTE,
    LPDWORD, LPINT, LPVOID, LPWORD, MAX_PATH, PFLOAT, PROC, UINT, ULONG, USHORT, WORD,
};
use shared::windef::{
    COLORREF, HBITMAP, HBRUSH, HCOLORSPACE, HDC, HENHMETAFILE, HFONT, HGDIOBJ, HGLRC, HPALETTE,
    HPEN, HWND, LPPOINT, LPRECT, LPSIZE, POINT, POINTL, POINTS, RECT, RECTL, SIZEL,
};
use um::winnt::{
    CHAR, HANDLE, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, LUID, PSTR, PVOID, SHORT, VOID, WCHAR,
};
pub const R2_BLACK: c_int = 1;
pub const R2_NOTMERGEPEN: c_int = 2;
pub const R2_MASKNOTPEN: c_int = 3;
pub const R2_NOTCOPYPEN: c_int = 4;
pub const R2_MASKPENNOT: c_int = 5;
pub const R2_NOT: c_int = 6;
pub const R2_XORPEN: c_int = 7;
pub const R2_NOTMASKPEN: c_int = 8;
pub const R2_MASKPEN: c_int = 9;
pub const R2_NOTXORPEN: c_int = 10;
pub const R2_NOP: c_int = 11;
pub const R2_MERGENOTPEN: c_int = 12;
pub const R2_COPYPEN: c_int = 13;
pub const R2_MERGEPENNOT: c_int = 14;
pub const R2_MERGEPEN: c_int = 15;
pub const R2_WHITE: c_int = 16;
pub const R2_LAST: c_int = 16;
pub const SRCCOPY: DWORD = 0x00CC0020;
pub const SRCPAINT: DWORD = 0x00EE0086;
pub const SRCAND: DWORD = 0x008800C6;
pub const SRCINVERT: DWORD = 0x00660046;
pub const SRCERASE: DWORD = 0x00440328;
pub const NOTSRCCOPY: DWORD = 0x00330008;
pub const NOTSRCERASE: DWORD = 0x001100A6;
pub const MERGECOPY: DWORD = 0x00C000CA;
pub const MERGEPAINT: DWORD = 0x00BB0226;
pub const PATCOPY: DWORD = 0x00F00021;
pub const PATPAINT: DWORD = 0x00FB0A09;
pub const PATINVERT: DWORD = 0x005A0049;
pub const DSTINVERT: DWORD = 0x00550009;
pub const BLACKNESS: DWORD = 0x00000042;
pub const WHITENESS: DWORD = 0x00FF0062;
pub const NOMIRRORBITMAP: DWORD = 0x80000000;
pub const CAPTUREBLT: DWORD = 0x40000000;
#[inline]
pub fn MAKEROP4(fore: DWORD, back: DWORD) -> DWORD {
    ((back << 8) & 0xFF000000) | fore
}
pub const GDI_ERROR: ULONG = 0xFFFFFFFF;
pub const HGDI_ERROR: HANDLE = -1isize as HANDLE;
pub const ERROR: c_int = 0;
pub const NULLREGION: c_int = 1;
pub const SIMPLEREGION: c_int = 2;
pub const COMPLEXREGION: c_int = 3;
pub const RGN_ERROR: c_int = ERROR;
pub const RGN_AND: c_int = 1;
pub const RGN_OR: c_int = 2;
pub const RGN_XOR: c_int = 3;
pub const RGN_DIFF: c_int = 4;
pub const RGN_COPY: c_int = 5;
pub const RGN_MIN: c_int = RGN_AND;
pub const RGN_MAX: c_int = RGN_COPY;
pub const BLACKONWHITE: c_int = 1;
pub const WHITEONBLACK: c_int = 2;
pub const COLORONCOLOR: c_int = 3;
pub const HALFTONE: c_int = 4;
pub const MAXSTRETCHBLTMODE: c_int = 4;
pub const STRETCH_ANDSCANS: c_int = BLACKONWHITE;
pub const STRETCH_ORSCANS: c_int = WHITEONBLACK;
pub const STRETCH_DELETESCANS: c_int = COLORONCOLOR;
pub const STRETCH_HALFTONE: c_int = HALFTONE;
pub const ALTERNATE: c_int = 1;
pub const WINDING: c_int = 2;
pub const POLYFILL_LAST: c_int = 2;
pub const LAYOUT_RTL: DWORD = 0x00000001;
pub const LAYOUT_BTT: DWORD = 0x00000002;
pub const LAYOUT_VBH: DWORD = 0x00000004;
pub const LAYOUT_ORIENTATIONMASK: DWORD = LAYOUT_RTL | LAYOUT_BTT | LAYOUT_VBH;
pub const LAYOUT_BITMAPORIENTATIONPRESERVED: DWORD = 0x00000008;
pub const TA_NOUPDATECP: UINT = 0;
pub const TA_UPDATECP: UINT = 1;
pub const TA_LEFT: UINT = 0;
pub const TA_RIGHT: UINT = 2;
pub const TA_CENTER: UINT = 6;
pub const TA_TOP: UINT = 0;
pub const TA_BOTTOM: UINT = 8;
pub const TA_BASELINE: UINT = 24;
pub const TA_RTLREADING: UINT = 256;
pub const TA_MASK: UINT = TA_BASELINE + TA_CENTER + TA_UPDATECP + TA_RTLREADING;
pub const VTA_BASELINE: UINT = TA_BASELINE;
pub const VTA_LEFT: UINT = TA_BOTTOM;
pub const VTA_RIGHT: UINT = TA_TOP;
pub const VTA_CENTER: UINT = TA_CENTER;
pub const VTA_BOTTOM: UINT = TA_RIGHT;
pub const VTA_TOP: UINT = TA_LEFT;
pub const ETO_OPAQUE: UINT = 0x0002;
pub const ETO_CLIPPED: UINT = 0x0004;
pub const ETO_GLYPH_INDEX: UINT = 0x0010;
pub const ETO_RTLREADING: UINT = 0x0080;
pub const ETO_NUMERICSLOCAL: UINT = 0x0400;
pub const ETO_NUMERICSLATIN: UINT = 0x0800;
pub const ETO_IGNORELANGUAGE: UINT = 0x1000;
pub const ETO_PDY: UINT = 0x2000;
pub const ETO_REVERSE_INDEX_MAP: UINT = 0x10000;
pub const ASPECT_FILTERING: UINT = 0x0001;
pub const DCB_RESET: UINT = 0x0001;
pub const DCB_ACCUMULATE: UINT = 0x0002;
pub const DCB_DIRTY: UINT = DCB_ACCUMULATE;
pub const DCB_SET: UINT = DCB_RESET | DCB_ACCUMULATE;
pub const DCB_ENABLE: UINT = 0x0004;
pub const DCB_DISABLE: UINT = 0x0008;
pub const META_SETBKCOLOR: WORD = 0x0201;
pub const META_SETBKMODE: WORD = 0x0102;
pub const META_SETMAPMODE: WORD = 0x0103;
pub const META_SETROP2: WORD = 0x0104;
pub const META_SETRELABS: WORD = 0x0105;
pub const META_SETPOLYFILLMODE: WORD = 0x0106;
pub const META_SETSTRETCHBLTMODE: WORD = 0x0107;
pub const META_SETTEXTCHAREXTRA: WORD = 0x0108;
pub const META_SETTEXTCOLOR: WORD = 0x0209;
pub const META_SETTEXTJUSTIFICATION: WORD = 0x020A;
pub const META_SETWINDOWORG: WORD = 0x020B;
pub const META_SETWINDOWEXT: WORD = 0x020C;
pub const META_SETVIEWPORTORG: WORD = 0x020D;
pub const META_SETVIEWPORTEXT: WORD = 0x020E;
pub const META_OFFSETWINDOWORG: WORD = 0x020F;
pub const META_SCALEWINDOWEXT: WORD = 0x0410;
pub const META_OFFSETVIEWPORTORG: WORD = 0x0211;
pub const META_SCALEVIEWPORTEXT: WORD = 0x0412;
pub const META_LINETO: WORD = 0x0213;
pub const META_MOVETO: WORD = 0x0214;
pub const META_EXCLUDECLIPRECT: WORD = 0x0415;
pub const META_INTERSECTCLIPRECT: WORD = 0x0416;
pub const META_ARC: WORD = 0x0817;
pub const META_ELLIPSE: WORD = 0x0418;
pub const META_FLOODFILL: WORD = 0x0419;
pub const META_PIE: WORD = 0x081A;
pub const META_RECTANGLE: WORD = 0x041B;
pub const META_ROUNDRECT: WORD = 0x061C;
pub const META_PATBLT: WORD = 0x061D;
pub const META_SAVEDC: WORD = 0x001E;
pub const META_SETPIXEL: WORD = 0x041F;
pub const META_OFFSETCLIPRGN: WORD = 0x0220;
pub const META_TEXTOUT: WORD = 0x0521;
pub const META_BITBLT: WORD = 0x0922;
pub const META_STRETCHBLT: WORD = 0x0B23;
pub const META_POLYGON: WORD = 0x0324;
pub const META_POLYLINE: WORD = 0x0325;
pub const META_ESCAPE: WORD = 0x0626;
pub const META_RESTOREDC: WORD = 0x0127;
pub const META_FILLREGION: WORD = 0x0228;
pub const META_FRAMEREGION: WORD = 0x0429;
pub const META_INVERTREGION: WORD = 0x012A;
pub const META_PAINTREGION: WORD = 0x012B;
pub const META_SELECTCLIPREGION: WORD = 0x012C;
pub const META_SELECTOBJECT: WORD = 0x012D;
pub const META_SETTEXTALIGN: WORD = 0x012E;
pub const META_CHORD: WORD = 0x0830;
pub const META_SETMAPPERFLAGS: WORD = 0x0231;
pub const META_EXTTEXTOUT: WORD = 0x0a32;
pub const META_SETDIBTODEV: WORD = 0x0d33;
pub const META_SELECTPALETTE: WORD = 0x0234;
pub const META_REALIZEPALETTE: WORD = 0x0035;
pub const META_ANIMATEPALETTE: WORD = 0x0436;
pub const META_SETPALENTRIES: WORD = 0x0037;
pub const META_POLYPOLYGON: WORD = 0x0538;
pub const META_RESIZEPALETTE: WORD = 0x0139;
pub const META_DIBBITBLT: WORD = 0x0940;
pub const META_DIBSTRETCHBLT: WORD = 0x0b41;
pub const META_DIBCREATEPATTERNBRUSH: WORD = 0x0142;
pub const META_STRETCHDIB: WORD = 0x0f43;
pub const META_EXTFLOODFILL: WORD = 0x0548;
pub const META_SETLAYOUT: WORD = 0x0149;
pub const META_DELETEOBJECT: WORD = 0x01f0;
pub const META_CREATEPALETTE: WORD = 0x00f7;
pub const META_CREATEPATTERNBRUSH: WORD = 0x01F9;
pub const META_CREATEPENINDIRECT: WORD = 0x02FA;
pub const META_CREATEFONTINDIRECT: WORD = 0x02FB;
pub const META_CREATEBRUSHINDIRECT: WORD = 0x02FC;
pub const META_CREATEREGION: WORD = 0x06FF;
STRUCT!{struct DRAWPATRECT {
    ptPosition: POINT,
    ptSize: POINT,
    wStyle: WORD,
    wPattern: WORD,
}}
pub type PDRAWPATRECT = *mut DRAWPATRECT;
pub const NEWFRAME: c_int = 1;
pub const ABORTDOC: c_int = 2;
pub const NEXTBAND: c_int = 3;
pub const SETCOLORTABLE: c_int = 4;
pub const GETCOLORTABLE: c_int = 5;
pub const FLUSHOUTPUT: c_int = 6;
pub const DRAFTMODE: c_int = 7;
pub const QUERYESCSUPPORT: c_int = 8;
pub const SETABORTPROC: c_int = 9;
pub const STARTDOC: c_int = 10;
pub const ENDDOC: c_int = 11;
pub const GETPHYSPAGESIZE: c_int = 12;
pub const GETPRINTINGOFFSET: c_int = 13;
pub const GETSCALINGFACTOR: c_int = 14;
pub const MFCOMMENT: c_int = 15;
pub const GETPENWIDTH: c_int = 16;
pub const SETCOPYCOUNT: c_int = 17;
pub const SELECTPAPERSOURCE: c_int = 18;
pub const DEVICEDATA: c_int = 19;
pub const PASSTHROUGH: c_int = 19;
pub const GETTECHNOLGY: c_int = 20;
pub const GETTECHNOLOGY: c_int = 20;
pub const SETLINECAP: c_int = 21;
pub const SETLINEJOIN: c_int = 22;
pub const SETMITERLIMIT: c_int = 23;
pub const BANDINFO: c_int = 24;
pub const DRAWPATTERNRECT: c_int = 25;
pub const GETVECTORPENSIZE: c_int = 26;
pub const GETVECTORBRUSHSIZE: c_int = 27;
pub const ENABLEDUPLEX: c_int = 28;
pub const GETSETPAPERBINS: c_int = 29;
pub const GETSETPRINTORIENT: c_int = 30;
pub const ENUMPAPERBINS: c_int = 31;
pub const SETDIBSCALING: c_int = 32;
pub const EPSPRINTING: c_int = 33;
pub const ENUMPAPERMETRICS: c_int = 34;
pub const GETSETPAPERMETRICS: c_int = 35;
pub const POSTSCRIPT_DATA: c_int = 37;
pub const POSTSCRIPT_IGNORE: c_int = 38;
pub const MOUSETRAILS: c_int = 39;
pub const GETDEVICEUNITS: c_int = 42;
pub const GETEXTENDEDTEXTMETRICS: c_int = 256;
pub const GETEXTENTTABLE: c_int = 257;
pub const GETPAIRKERNTABLE: c_int = 258;
pub const GETTRACKKERNTABLE: c_int = 259;
pub const EXTTEXTOUT: c_int = 512;
pub const GETFACENAME: c_int = 513;
pub const DOWNLOADFACE: c_int = 514;
pub const ENABLERELATIVEWIDTHS: c_int = 768;
pub const ENABLEPAIRKERNING: c_int = 769;
pub const SETKERNTRACK: c_int = 770;
pub const SETALLJUSTVALUES: c_int = 771;
pub const SETCHARSET: c_int = 772;
pub const STRETCHBLT: c_int = 2048;
pub const METAFILE_DRIVER: c_int = 2049;
pub const GETSETSCREENPARAMS: c_int = 3072;
pub const QUERYDIBSUPPORT: c_int = 3073;
pub const BEGIN_PATH: c_int = 4096;
pub const CLIP_TO_PATH: c_int = 4097;
pub const END_PATH: c_int = 4098;
pub const EXT_DEVICE_CAPS: c_int = 4099;
pub const RESTORE_CTM: c_int = 4100;
pub const SAVE_CTM: c_int = 4101;
pub const SET_ARC_DIRECTION: c_int = 4102;
pub const SET_BACKGROUND_COLOR: c_int = 4103;
pub const SET_POLY_MODE: c_int = 4104;
pub const SET_SCREEN_ANGLE: c_int = 4105;
pub const SET_SPREAD: c_int = 4106;
pub const TRANSFORM_CTM: c_int = 4107;
pub const SET_CLIP_BOX: c_int = 4108;
pub const SET_BOUNDS: c_int = 4109;
pub const SET_MIRROR_MODE: c_int = 4110;
pub const OPENCHANNEL: c_int = 4110;
pub const DOWNLOADHEADER: c_int = 4111;
pub const CLOSECHANNEL: c_int = 4112;
pub const POSTSCRIPT_PASSTHROUGH: c_int = 4115;
pub const ENCAPSULATED_POSTSCRIPT: c_int = 4116;
pub const POSTSCRIPT_IDENTIFY: c_int = 4117;
pub const POSTSCRIPT_INJECTION: c_int = 4118;
pub const CHECKJPEGFORMAT: c_int = 4119;
pub const CHECKPNGFORMAT: c_int = 4120;
pub const GET_PS_FEATURESETTING: c_int = 4121;
pub const GDIPLUS_TS_QUERYVER: c_int = 4122;
pub const GDIPLUS_TS_RECORD: c_int = 4123;
pub const MILCORE_TS_QUERYVER_RESULT_FALSE: c_int = 0x0;
pub const MILCORE_TS_QUERYVER_RESULT_TRUE: c_int = 0x7FFFFFFF;
pub const SPCLPASSTHROUGH2: c_int = 4568;
pub const PSIDENT_GDICENTRIC: c_int = 0;
pub const PSIDENT_PSCENTRIC: c_int = 1;
STRUCT!{struct PSINJECTDATA {
    DataBytes: DWORD,
    InjectionPoint: WORD,
    PageNumber: WORD,
}}
pub type PPSINJECTDATA = *mut PSINJECTDATA;
pub const PSINJECT_BEGINSTREAM: WORD = 1;
pub const PSINJECT_PSADOBE: WORD = 2;
pub const PSINJECT_PAGESATEND: WORD = 3;
pub const PSINJECT_PAGES: WORD = 4;
pub const PSINJECT_DOCNEEDEDRES: WORD = 5;
pub const PSINJECT_DOCSUPPLIEDRES: WORD = 6;
pub const PSINJECT_PAGEORDER: WORD = 7;
pub const PSINJECT_ORIENTATION: WORD = 8;
pub const PSINJECT_BOUNDINGBOX: WORD = 9;
pub const PSINJECT_DOCUMENTPROCESSCOLORS: WORD = 10;
pub const PSINJECT_COMMENTS: WORD = 11;
pub const PSINJECT_BEGINDEFAULTS: WORD = 12;
pub const PSINJECT_ENDDEFAULTS: WORD = 13;
pub const PSINJECT_BEGINPROLOG: WORD = 14;
pub const PSINJECT_ENDPROLOG: WORD = 15;
pub const PSINJECT_BEGINSETUP: WORD = 16;
pub const PSINJECT_ENDSETUP: WORD = 17;
pub const PSINJECT_TRAILER: WORD = 18;
pub const PSINJECT_EOF: WORD = 19;
pub const PSINJECT_ENDSTREAM: WORD = 20;
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: WORD = 21;
pub const PSINJECT_PAGENUMBER: WORD = 100;
pub const PSINJECT_BEGINPAGESETUP: WORD = 101;
pub const PSINJECT_ENDPAGESETUP: WORD = 102;
pub const PSINJECT_PAGETRAILER: WORD = 103;
pub const PSINJECT_PLATECOLOR: WORD = 104;
pub const PSINJECT_SHOWPAGE: WORD = 105;
pub const PSINJECT_PAGEBBOX: WORD = 106;
pub const PSINJECT_ENDPAGECOMMENTS: WORD = 107;
pub const PSINJECT_VMSAVE: WORD = 200;
pub const PSINJECT_VMRESTORE: WORD = 201;
pub const PSINJECT_DLFONT: DWORD = 0xdddddddd;
pub const FEATURESETTING_NUP: WORD = 0;
pub const FEATURESETTING_OUTPUT: WORD = 1;
pub const FEATURESETTING_PSLEVEL: WORD = 2;
pub const FEATURESETTING_CUSTPAPER: WORD = 3;
pub const FEATURESETTING_MIRROR: WORD = 4;
pub const FEATURESETTING_NEGATIVE: WORD = 5;
pub const FEATURESETTING_PROTOCOL: WORD = 6;
pub const FEATURESETTING_PRIVATE_BEGIN: WORD = 0x1000;
pub const FEATURESETTING_PRIVATE_END: WORD = 0x1FFF;
STRUCT!{struct PSFEATURE_OUTPUT {
    bPageIndependent: BOOL,
    bSetPageDevice: BOOL,
}}
pub type PPSFEATURE_OUTPUT = *mut PSFEATURE_OUTPUT;
STRUCT!{struct PSFEATURE_CUSTPAPER {
    lOrientation: LONG,
    lWidth: LONG,
    lHeight: LONG,
    lWidthOffset: LONG,
    lHeightOffset: LONG,
}}
pub type PPSFEATURE_CUSTPAPER = *mut PSFEATURE_CUSTPAPER;
pub const PSPROTOCOL_ASCII: c_int = 0;
pub const PSPROTOCOL_BCP: c_int = 1;
pub const PSPROTOCOL_TBCP: c_int = 2;
pub const PSPROTOCOL_BINARY: c_int = 3;
pub const QDI_SETDIBITS: c_int = 1;
pub const QDI_GETDIBITS: c_int = 2;
pub const QDI_DIBTOSCREEN: c_int = 4;
pub const QDI_STRETCHDIB: c_int = 8;
pub const SP_NOTREPORTED: c_int = 0x4000;
pub const SP_ERROR: c_int = -1;
pub const SP_APPABORT: c_int = -2;
pub const SP_USERABORT: c_int = -3;
pub const SP_OUTOFDISK: c_int = -4;
pub const SP_OUTOFMEMORY: c_int = -5;
pub const PR_JOBSTATUS: c_int = 0x0000;
pub const OBJ_PEN: UINT = 1;
pub const OBJ_BRUSH: UINT = 2;
pub const OBJ_DC: UINT = 3;
pub const OBJ_METADC: UINT = 4;
pub const OBJ_PAL: UINT = 5;
pub const OBJ_FONT: UINT = 6;
pub const OBJ_BITMAP: UINT = 7;
pub const OBJ_REGION: UINT = 8;
pub const OBJ_METAFILE: UINT = 9;
pub const OBJ_MEMDC: UINT = 10;
pub const OBJ_EXTPEN: UINT = 11;
pub const OBJ_ENHMETADC: UINT = 12;
pub const OBJ_ENHMETAFILE: UINT = 13;
pub const OBJ_COLORSPACE: UINT = 14;
pub const GDI_OBJ_LAST: UINT = OBJ_COLORSPACE;
pub const MWT_IDENTITY: c_int = 1;
pub const MWT_LEFTMULTIPLY: c_int = 2;
pub const MWT_RIGHTMULTIPLY: c_int = 3;
pub const MWT_MIN: c_int = MWT_IDENTITY;
pub const MWT_MAX: c_int = MWT_RIGHTMULTIPLY;
STRUCT!{struct XFORM {
    eM11: FLOAT,
    eM12: FLOAT,
    eM21: FLOAT,
    eM22: FLOAT,
    eDx: FLOAT,
    eDy: FLOAT,
}}
pub type PXFORM = *mut XFORM;
pub type LPXFORM = *mut XFORM;
STRUCT!{struct BITMAP {
    bmType: LONG,
    bmWidth: LONG,
    bmHeight: LONG,
    bmWidthBytes: LONG,
    bmPlanes: WORD,
    bmBitsPixel: WORD,
    bmBits: LPVOID,
}}
pub type PBITMAP = *mut BITMAP;
pub type NPBITMAP = *mut BITMAP;
pub type LPBITMAP = *mut BITMAP;
STRUCT!{#[debug] struct RGBTRIPLE {
    rgbtBlue: BYTE,
    rgbtGreen: BYTE,
    rgbtRed: BYTE,
}}
pub type PRGBTRIPLE = *mut RGBTRIPLE;
pub type NPRGBTRIPLE = *mut RGBTRIPLE;
pub type LPRGBTRIPLE = *mut RGBTRIPLE;
STRUCT!{#[debug] struct RGBQUAD {
    rgbBlue: BYTE,
    rgbGreen: BYTE,
    rgbRed: BYTE,
    rgbReserved: BYTE,
}}
pub type LPRGBQUAD = *mut RGBQUAD;
pub const CS_ENABLE: DWORD = 0x00000001;
pub const CS_DISABLE: DWORD = 0x00000002;
pub const CS_DELETE_TRANSFORM: DWORD = 0x00000003;
pub const LCS_SIGNATURE: DWORD = 0x5053_4F43; // 'PSOC'
pub const LCS_sRGB: LCSCSTYPE = 0x7352_4742; // 'sRGB'
pub const LCS_WINDOWS_COLOR_SPACE: LCSCSTYPE = 0x5769_6E20; // 'Win '
pub type LCSCSTYPE = LONG;
pub const LCS_CALIBRATED_RGB: LCSCSTYPE = 0x00000000;
pub type LCSGAMUTMATCH = LONG;
pub const LCS_GM_BUSINESS: LCSGAMUTMATCH = 0x00000001;
pub const LCS_GM_GRAPHICS: LCSGAMUTMATCH = 0x00000002;
pub const LCS_GM_IMAGES: LCSGAMUTMATCH = 0x00000004;
pub const LCS_GM_ABS_COLORIMETRIC: LCSGAMUTMATCH = 0x00000008;
pub const CM_OUT_OF_GAMUT: BYTE = 255;
pub const CM_IN_GAMUT: BYTE = 0;
pub const ICM_ADDPROFILE: UINT = 1;
pub const ICM_DELETEPROFILE: UINT = 2;
pub const ICM_QUERYPROFILE: UINT = 3;
pub const ICM_SETDEFAULTPROFILE: UINT = 4;
pub const ICM_REGISTERICMATCHER: UINT = 5;
pub const ICM_UNREGISTERICMATCHER: UINT = 6;
pub const ICM_QUERYMATCH: UINT = 7;
#[inline]
pub fn GetKValue(cmyk: COLORREF) -> BYTE {
    cmyk as BYTE
}
#[inline]
pub fn GetYValue(cmyk: COLORREF) -> BYTE {
    (cmyk >> 8) as BYTE
}
#[inline]
pub fn GetMValue(cmyk: COLORREF) -> BYTE {
    (cmyk >> 16) as BYTE
}
#[inline]
pub fn GetCValue(cmyk: COLORREF) -> BYTE {
    (cmyk >> 24) as BYTE
}
#[inline]
pub fn CMYK(c: BYTE, m: BYTE, y: BYTE, k: BYTE) -> COLORREF {
    (k as COLORREF) | ((y as COLORREF) << 8) | ((m as COLORREF) << 16) | ((c as COLORREF) << 24)
}
pub type FXPT16DOT16 = c_long;
pub type LPFXPT16DOT16 = *mut c_long;
pub type FXPT2DOT30 = c_long;
pub type LPFXPT2DOT30 = *mut c_long;
STRUCT!{#[debug] struct CIEXYZ {
    ciexyzX: FXPT2DOT30,
    ciexyzY: FXPT2DOT30,
    ciexyzZ: FXPT2DOT30,
}}
pub type LPCIEXYZ = *mut CIEXYZ;
STRUCT!{#[debug] struct CIEXYZTRIPLE {
    ciexyzRed: CIEXYZ,
    ciexyzGreen: CIEXYZ,
    ciexyzBlue: CIEXYZ,
}}
pub type LPCIEXYZTRIPLE = *mut CIEXYZTRIPLE;
STRUCT!{struct LOGCOLORSPACEA {
    lcsSignature: DWORD,
    lcsVersion: DWORD,
    lcsSize: DWORD,
    lcsCSType: LCSCSTYPE,
    lcsIntent: LCSGAMUTMATCH,
    lcsEndpoints: CIEXYZTRIPLE,
    lcsGammaRed: DWORD,
    lcsGammaGreen: DWORD,
    lcsGammaBlue: DWORD,
    lcsFilename: [CHAR; MAX_PATH],
}}
pub type LPLOGCOLORSPACEA = *mut LOGCOLORSPACEA;
STRUCT!{struct LOGCOLORSPACEW {
    lcsSignature: DWORD,
    lcsVersion: DWORD,
    lcsSize: DWORD,
    lcsCSType: LCSCSTYPE,
    lcsIntent: LCSGAMUTMATCH,
    lcsEndpoints: CIEXYZTRIPLE,
    lcsGammaRed: DWORD,
    lcsGammaGreen: DWORD,
    lcsGammaBlue: DWORD,
    lcsFilename: [WCHAR; MAX_PATH],
}}
pub type LPLOGCOLORSPACEW = *mut LOGCOLORSPACEW;
STRUCT!{#[debug] struct BITMAPCOREHEADER {
    bcSize: DWORD,
    bcWidth: WORD,
    bcHeight: WORD,
    bcPlanes: WORD,
    bcBitCount: WORD,
}}
pub type LPBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
pub type PBITMAPCOREHEADER = *mut BITMAPCOREHEADER;
STRUCT!{#[debug] struct BITMAPINFOHEADER {
    biSize: DWORD,
    biWidth: LONG,
    biHeight: LONG,
    biPlanes: WORD,
    biBitCount: WORD,
    biCompression: DWORD,
    biSizeImage: DWORD,
    biXPelsPerMeter: LONG,
    biYPelsPerMeter: LONG,
    biClrUsed: DWORD,
    biClrImportant: DWORD,
}}
pub type LPBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
pub type PBITMAPINFOHEADER = *mut BITMAPINFOHEADER;
STRUCT!{#[debug] struct BITMAPV4HEADER {
    bV4Size: DWORD,
    bV4Width: LONG,
    bV4Height: LONG,
    bV4Planes: WORD,
    bV4BitCount: WORD,
    bV4V4Compression: DWORD,
    bV4SizeImage: DWORD,
    bV4XPelsPerMeter: LONG,
    bV4YPelsPerMeter: LONG,
    bV4ClrUsed: DWORD,
    bV4ClrImportant: DWORD,
    bV4RedMask: DWORD,
    bV4GreenMask: DWORD,
    bV4BlueMask: DWORD,
    bV4AlphaMask: DWORD,
    bV4CSType: DWORD,
    bV4Endpoints: CIEXYZTRIPLE,
    bV4GammaRed: DWORD,
    bV4GammaGreen: DWORD,
    bV4GammaBlue: DWORD,
}}
pub type LPBITMAPV4HEADER = *mut BITMAPV4HEADER;
pub type PBITMAPV4HEADER = *mut BITMAPV4HEADER;
STRUCT!{#[debug] struct BITMAPV5HEADER {
    bV5Size: DWORD,
    bV5Width: LONG,
    bV5Height: LONG,
    bV5Planes: WORD,
    bV5BitCount: WORD,
    bV5Compression: DWORD,
    bV5SizeImage: DWORD,
    bV5XPelsPerMeter: LONG,
    bV5YPelsPerMeter: LONG,
    bV5ClrUsed: DWORD,
    bV5ClrImportant: DWORD,
    bV5RedMask: DWORD,
    bV5GreenMask: DWORD,
    bV5BlueMask: DWORD,
    bV5AlphaMask: DWORD,
    bV5CSType: DWORD,
    bV5Endpoints: CIEXYZTRIPLE,
    bV5GammaRed: DWORD,
    bV5GammaGreen: DWORD,
    bV5GammaBlue: DWORD,
    bV5Intent: DWORD,
    bV5ProfileData: DWORD,
    bV5ProfileSize: DWORD,
    bV5Reserved: DWORD,
}}
pub type LPBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub type PBITMAPV5HEADER = *mut BITMAPV5HEADER;
pub const PROFILE_LINKED: LONG = 0x4C49_4E4B; // 'LINK'
pub const PROFILE_EMBEDDED: LONG = 0x4D42_4544; // 'MBED'
pub const BI_RGB: DWORD = 0;
pub const BI_RLE8: DWORD = 1;
pub const BI_RLE4: DWORD = 2;
pub const BI_BITFIELDS: DWORD = 3;
pub const BI_JPEG: DWORD = 4;
pub const BI_PNG: DWORD = 5;
STRUCT!{#[debug] struct BITMAPINFO {
    bmiHeader: BITMAPINFOHEADER,
    bmiColors: [RGBQUAD; 1],
}}
pub type LPBITMAPINFO = *mut BITMAPINFO;
pub type PBITMAPINFO = *mut BITMAPINFO;
STRUCT!{#[debug] struct BITMAPCOREINFO {
    bmciHeader: BITMAPCOREHEADER,
    bmciColors: [RGBTRIPLE; 1],
}}
pub type LPBITMAPCOREINFO = *mut BITMAPCOREINFO;
pub type PBITMAPCOREINFO = *mut BITMAPCOREINFO;
STRUCT!{#[debug] #[repr(packed)] struct BITMAPFILEHEADER {
    bfType: WORD,
    bfSize: DWORD,
    bfReserved1: WORD,
    bfReserved2: WORD,
    bfOffBits: DWORD,
}}
pub type LPBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
pub type PBITMAPFILEHEADER = *mut BITMAPFILEHEADER;
#[inline]
pub fn MAKEPOINTS(l: DWORD) -> POINTS {
    unsafe { ::core::mem::transmute::<DWORD, POINTS>(l) }
}
STRUCT!{#[debug] struct FONTSIGNATURE {
    fsUsb: [DWORD; 4],
    fsCsb: [DWORD; 2],
}}
pub type LPFONTSIGNATURE = *mut FONTSIGNATURE;
pub type PFONTSIGNATURE = *mut FONTSIGNATURE;
STRUCT!{#[debug] struct CHARSETINFO {
    ciCharset: UINT,
    ciACP: UINT,
    fs: FONTSIGNATURE,
}}
pub type PCHARSETINFO = *mut CHARSETINFO;
pub type NPCHARSETINFO = *mut CHARSETINFO;
pub type LPCHARSETINFO = *mut CHARSETINFO;
pub const TCI_SRCCHARSET: c_int = 1;
pub const TCI_SRCCODEPAGE: c_int = 2;
pub const TCI_SRCFONTSIG: c_int = 3;
pub const TCI_SRCLOCALE: c_int = 0x1000;
STRUCT!{#[debug] struct LOCALESIGNATURE {
    lsUsb: [DWORD; 4],
    lsCsbDefault: [DWORD; 2],
    lsCsbSupported: [DWORD; 2],
}}
pub type PLOCALESIGNATURE = *mut LOCALESIGNATURE;
pub type LPLOCALESIGNATURE = *mut LOCALESIGNATURE;
STRUCT!{struct HANDLETABLE {
    objectHandle: [HGDIOBJ; 1],
}}
pub type LPHANDLETABLE = *mut HANDLETABLE;
pub type PHANDLETABLE = *mut HANDLETABLE;
STRUCT!{struct METARECORD {
    rdSize: DWORD,
    rdFunction: WORD,
    rdParm: [WORD; 1],
}}
pub type PMETARECORD = *mut METARECORD;
pub type LPMETARECORD = *mut METARECORD;
STRUCT!{struct METAFILEPICT {
    mm: LONG,
    xExt: LONG,
    yExt: LONG,
    hMF: HMETAFILE,
}}
pub type LPMETAFILEPICT = *mut METAFILEPICT;
STRUCT!{struct METAHEADER {
    mtType: WORD,
    mtHeaderSize: WORD,
    mtVersion: WORD,
    mtSize: DWORD,
    mtNoObjects: WORD,
    mtMaxRecord: DWORD,
    mtNoParameters: WORD,
}}
pub type PMETAHEADER = *mut METAHEADER;
pub type LPMETAHEADER = *mut METAHEADER;
STRUCT!{struct ENHMETARECORD {
    iType: DWORD,
    nSize: DWORD,
    dParm: [DWORD; 1],
}}
pub type PENHMETARECORD = *mut ENHMETARECORD;
pub type LPENHMETARECORD = *mut ENHMETARECORD;
STRUCT!{struct ENHMETAHEADER {
    iType: DWORD,
    nSize: DWORD,
    rclBounds: RECTL,
    rclFrame: RECTL,
    dSignature: DWORD,
    nVersion: DWORD,
    nBytes: DWORD,
    nRecords: DWORD,
    nHandles: WORD,
    sReserved: WORD,
    nDescription: DWORD,
    offDescription: DWORD,
    nPalEntries: DWORD,
    szlDevice: SIZEL,
    szlMillimeters: SIZEL,
    cbPixelFormat: DWORD,
    offPixelFormat: DWORD,
    bOpenGL: DWORD,
    szlMicrometers: SIZEL,
}}
pub type PENHMETAHEADER = *mut ENHMETAHEADER;
pub type LPENHMETAHEADER = *mut ENHMETAHEADER;
pub const TMPF_FIXED_PITCH: BYTE = 0x01;
pub const TMPF_VECTOR: BYTE = 0x02;
pub const TMPF_DEVICE: BYTE = 0x08;
pub const TMPF_TRUETYPE: BYTE = 0x04;
// BCHAR
STRUCT!{struct TEXTMETRICA {
    tmHeight: LONG,
    tmAscent: LONG,
    tmDescent: LONG,
    tmInternalLeading: LONG,
    tmExternalLeading: LONG,
    tmAveCharWidth: LONG,
    tmMaxCharWidth: LONG,
    tmWeight: LONG,
    tmOverhang: LONG,
    tmDigitizedAspectX: LONG,
    tmDigitizedAspectY: LONG,
    tmFirstChar: BYTE,
    tmLastChar: BYTE,
    tmDefaultChar: BYTE,
    tmBreakChar: BYTE,
    tmItalic: BYTE,
    tmUnderlined: BYTE,
    tmStruckOut: BYTE,
    tmPitchAndFamily: BYTE,
    tmCharSet: BYTE,
}}
pub type PTEXTMETRICA = *mut TEXTMETRICA;
pub type NPTEXTMETRICA = *mut TEXTMETRICA;
pub type LPTEXTMETRICA = *mut TEXTMETRICA;
STRUCT!{struct TEXTMETRICW {
    tmHeight: LONG,
    tmAscent: LONG,
    tmDescent: LONG,
    tmInternalLeading: LONG,
    tmExternalLeading: LONG,
    tmAveCharWidth: LONG,
    tmMaxCharWidth: LONG,
    tmWeight: LONG,
    tmOverhang: LONG,
    tmDigitizedAspectX: LONG,
    tmDigitizedAspectY: LONG,
    tmFirstChar: WCHAR,
    tmLastChar: WCHAR,
    tmDefaultChar: WCHAR,
    tmBreakChar: WCHAR,
    tmItalic: BYTE,
    tmUnderlined: BYTE,
    tmStruckOut: BYTE,
    tmPitchAndFamily: BYTE,
    tmCharSet: BYTE,
}}
pub type PTEXTMETRICW = *mut TEXTMETRICW;
pub type NPTEXTMETRICW = *mut TEXTMETRICW;
pub type LPTEXTMETRICW = *mut TEXTMETRICW;
pub const NTM_REGULAR: DWORD = 0x00000040;
pub const NTM_BOLD: DWORD = 0x00000020;
pub const NTM_ITALIC: DWORD = 0x00000001;
pub const NTM_NONNEGATIVE_AC: DWORD = 0x00010000;
pub const NTM_PS_OPENTYPE: DWORD = 0x00020000;
pub const NTM_TT_OPENTYPE: DWORD = 0x00040000;
pub const NTM_MULTIPLEMASTER: DWORD = 0x00080000;
pub const NTM_TYPE1: DWORD = 0x00100000;
pub const NTM_DSIG: DWORD = 0x00200000;
STRUCT!{struct NEWTEXTMETRICA {
    tmHeight: LONG,
    tmAscent: LONG,
    tmDescent: LONG,
    tmInternalLeading: LONG,
    tmExternalLeading: LONG,
    tmAveCharWidth: LONG,
    tmMaxCharWidth: LONG,
    tmWeight: LONG,
    tmOverhang: LONG,
    tmDigitizedAspectX: LONG,
    tmDigitizedAspectY: LONG,
    tmFirstChar: BYTE,
    tmLastChar: BYTE,
    tmDefaultChar: BYTE,
    tmBreakChar: BYTE,
    tmItalic: BYTE,
    tmUnderlined: BYTE,
    tmStruckOut: BYTE,
    tmPitchAndFamily: BYTE,
    tmCharSet: BYTE,
    ntmFlags: DWORD,
    ntmSizeEM: UINT,
    ntmCellHeight: UINT,
    ntmAvgWidth: UINT,
}}
pub type PNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
pub type NPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
pub type LPNEWTEXTMETRICA = *mut NEWTEXTMETRICA;
STRUCT!{struct NEWTEXTMETRICW {
    tmHeight: LONG,
    tmAscent: LONG,
    tmDescent: LONG,
    tmInternalLeading: LONG,
    tmExternalLeading: LONG,
    tmAveCharWidth: LONG,
    tmMaxCharWidth: LONG,
    tmWeight: LONG,
    tmOverhang: LONG,
    tmDigitizedAspectX: LONG,
    tmDigitizedAspectY: LONG,
    tmFirstChar: WCHAR,
    tmLastChar: WCHAR,
    tmDefaultChar: WCHAR,
    tmBreakChar: WCHAR,
    tmItalic: BYTE,
    tmUnderlined: BYTE,
    tmStruckOut: BYTE,
    tmPitchAndFamily: BYTE,
    tmCharSet: BYTE,
    ntmFlags: DWORD,
    ntmSizeEM: UINT,
    ntmCellHeight: UINT,
    ntmAvgWidth: UINT,
}}
pub type PNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
pub type NPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
pub type LPNEWTEXTMETRICW = *mut NEWTEXTMETRICW;
STRUCT!{struct NEWTEXTMETRICEXA {
    ntmTm: NEWTEXTMETRICA,
    ntmFontSig: FONTSIGNATURE,
}}
STRUCT!{struct NEWTEXTMETRICEXW {
    ntmTm: NEWTEXTMETRICW,
    ntmFontSig: FONTSIGNATURE,
}}
STRUCT!{struct PELARRAY {
    paXCount: LONG,
    paYCount: LONG,
    paXExt: LONG,
    paYExt: LONG,
    paRGBs: BYTE,
}}
pub type PPELARRAY = *mut PELARRAY;
pub type NPPELARRAY = *mut PELARRAY;
pub type LPPELARRAY = *mut PELARRAY;
STRUCT!{struct LOGBRUSH {
    lbStyle: UINT,
    lbColor: COLORREF,
    lbHatch: ULONG_PTR,
}}
pub type PLOGBRUSH = *mut LOGBRUSH;
pub type NPLOGBRUSH = *mut LOGBRUSH;
pub type LPLOGBRUSH = *mut LOGBRUSH;
STRUCT!{struct LOGBRUSH32 {
    lbStyle: UINT,
    lbColor: COLORREF,
    lbHatch: ULONG,
}}
pub type PLOGBRUSH32 = *mut LOGBRUSH32;
pub type NPLOGBRUSH32 = *mut LOGBRUSH32;
pub type LPLOGBRUSH32 = *mut LOGBRUSH32;
pub type PATTERN = LOGBRUSH;
pub type PPATTERN = *mut PATTERN;
pub type NPPATTERN = *mut PATTERN;
pub type LPPATTERN = *mut PATTERN;
STRUCT!{struct LOGPEN {
    lopnStyle: UINT,
    lopnWidth: POINT,
    lopnColor: COLORREF,
}}
pub type PLOGPEN = *mut LOGPEN;
pub type NPLOGPEN = *mut LOGPEN;
pub type LPLOGPEN = *mut LOGPEN;
STRUCT!{struct EXTLOGPEN {
    elpPenStyle: DWORD,
    elpWidth: DWORD,
    elpBrushStyle: UINT,
    elpColor: COLORREF,
    elpHatch: ULONG_PTR,
    elpNumEntries: DWORD,
    elpStyleEntry: [DWORD; 1],
}}
pub type PEXTLOGPEN = *mut EXTLOGPEN;
pub type NPEXTLOGPEN = *mut EXTLOGPEN;
pub type LPEXTLOGPEN = *mut EXTLOGPEN;
STRUCT!{struct EXTLOGPEN32 {
    elpPenStyle: DWORD,
    elpWidth: DWORD,
    elpBrushStyle: UINT,
    elpColor: COLORREF,
    elpHatch: ULONG,
    elpNumEntries: DWORD,
    elpStyleEntry: [DWORD; 1],
}}
pub type PEXTLOGPEN32 = *mut EXTLOGPEN32;
pub type NPEXTLOGPEN32 = *mut EXTLOGPEN32;
pub type LPEXTLOGPEN32 = *mut EXTLOGPEN32;
STRUCT!{struct PALETTEENTRY {
    peRed: BYTE,
    peGreen: BYTE,
    peBlue: BYTE,
    peFlags: BYTE,
}}
pub type PPALETTEENTRY = *mut PALETTEENTRY;
pub type LPPALETTEENTRY = *mut PALETTEENTRY;
STRUCT!{struct LOGPALETTE {
    palVersion: WORD,
    palNumEntries: WORD,
    palPalEntry: [PALETTEENTRY; 1],
}}
pub type PLOGPALETTE = *mut LOGPALETTE;
pub type NPLOGPALETTE = *mut LOGPALETTE;
pub type LPLOGPALETTE = *mut LOGPALETTE;
pub const LF_FACESIZE: usize = 32;
STRUCT!{struct LOGFONTA {
    lfHeight: LONG,
    lfWidth: LONG,
    lfEscapement: LONG,
    lfOrientation: LONG,
    lfWeight: LONG,
    lfItalic: BYTE,
    lfUnderline: BYTE,
    lfStrikeOut: BYTE,
    lfCharSet: BYTE,
    lfOutPrecision: BYTE,
    lfClipPrecision: BYTE,
    lfQuality: BYTE,
    lfPitchAndFamily: BYTE,
    lfFaceName: [CHAR; LF_FACESIZE],
}}
pub type PLOGFONTA = *mut LOGFONTA;
pub type NPLOGFONTA = *mut LOGFONTA;
pub type LPLOGFONTA = *mut LOGFONTA;
STRUCT!{struct LOGFONTW {
    lfHeight: LONG,
    lfWidth: LONG,
    lfEscapement: LONG,
    lfOrientation: LONG,
    lfWeight: LONG,
    lfItalic: BYTE,
    lfUnderline: BYTE,
    lfStrikeOut: BYTE,
    lfCharSet: BYTE,
    lfOutPrecision: BYTE,
    lfClipPrecision: BYTE,
    lfQuality: BYTE,
    lfPitchAndFamily: BYTE,
    lfFaceName: [WCHAR; LF_FACESIZE],
}}
pub type PLOGFONTW = *mut LOGFONTW;
pub type NPLOGFONTW = *mut LOGFONTW;
pub type LPLOGFONTW = *mut LOGFONTW;
pub const LF_FULLFACESIZE: usize = 64;
STRUCT!{struct ENUMLOGFONTA {
    elfLogFont: LOGFONTA,
    elfFullName: [BYTE; LF_FULLFACESIZE],
    elfStyle: [BYTE; LF_FACESIZE],
}}
pub type LPENUMLOGFONTA = *mut ENUMLOGFONTA;
STRUCT!{struct ENUMLOGFONTW {
    elfLogFont: LOGFONTW,
    elfFullName: [WCHAR; LF_FULLFACESIZE],
    elfStyle: [WCHAR; LF_FACESIZE],
}}
pub type LPENUMLOGFONTW = *mut ENUMLOGFONTW;
STRUCT!{struct ENUMLOGFONTEXA {
    elfLogFont: LOGFONTA,
    elfFullName: [BYTE; LF_FULLFACESIZE],
    elfStyle: [BYTE; LF_FACESIZE],
    elfScript: [BYTE; LF_FACESIZE],
}}
pub type LPENUMLOGFONTEXA = *mut ENUMLOGFONTEXA;
STRUCT!{struct ENUMLOGFONTEXW {
    elfLogFont: LOGFONTW,
    elfFullName: [WCHAR; LF_FULLFACESIZE],
    elfStyle: [WCHAR; LF_FACESIZE],
    elfScript: [WCHAR; LF_FACESIZE],
}}
pub type LPENUMLOGFONTEXW = *mut ENUMLOGFONTEXW;
pub const OUT_DEFAULT_PRECIS: DWORD = 0;
pub const OUT_STRING_PRECIS: DWORD = 1;
pub const OUT_CHARACTER_PRECIS: DWORD = 2;
pub const OUT_STROKE_PRECIS: DWORD = 3;
pub const OUT_TT_PRECIS: DWORD = 4;
pub const OUT_DEVICE_PRECIS: DWORD = 5;
pub const OUT_RASTER_PRECIS: DWORD = 6;
pub const OUT_TT_ONLY_PRECIS: DWORD = 7;
pub const OUT_OUTLINE_PRECIS: DWORD = 8;
pub const OUT_SCREEN_OUTLINE_PRECIS: DWORD = 9;
pub const OUT_PS_ONLY_PRECIS: DWORD = 10;
pub const CLIP_DEFAULT_PRECIS: DWORD = 0;
pub const CLIP_CHARACTER_PRECIS: DWORD = 1;
pub const CLIP_STROKE_PRECIS: DWORD = 2;
pub const CLIP_MASK: DWORD = 0xf;
pub const CLIP_LH_ANGLES: DWORD = 1 << 4;
pub const CLIP_TT_ALWAYS: DWORD = 2 << 4;
pub const CLIP_DFA_DISABLE: DWORD = 4 << 4;
pub const CLIP_EMBEDDED: DWORD = 8 << 4;
pub const DEFAULT_QUALITY: DWORD = 0;
pub const DRAFT_QUALITY: DWORD = 1;
pub const PROOF_QUALITY: DWORD = 2;
pub const NONANTIALIASED_QUALITY: DWORD = 3;
pub const ANTIALIASED_QUALITY: DWORD = 4;
pub const CLEARTYPE_QUALITY: DWORD = 5;
pub const CLEARTYPE_NATURAL_QUALITY: DWORD = 6;
pub const DEFAULT_PITCH: DWORD = 0;
pub const FIXED_PITCH: DWORD = 1;
pub const VARIABLE_PITCH: DWORD = 2;
pub const MONO_FONT: DWORD = 8;
pub const ANSI_CHARSET: DWORD = 0;
pub const DEFAULT_CHARSET: DWORD = 1;
pub const SYMBOL_CHARSET: DWORD = 2;
pub const SHIFTJIS_CHARSET: DWORD = 128;
pub const HANGEUL_CHARSET: DWORD = 129;
pub const HANGUL_CHARSET: DWORD = 129;
pub const GB2312_CHARSET: DWORD = 134;
pub const CHINESEBIG5_CHARSET: DWORD = 136;
pub const OEM_CHARSET: DWORD = 255;
pub const JOHAB_CHARSET: DWORD = 130;
pub const HEBREW_CHARSET: DWORD = 177;
pub const ARABIC_CHARSET: DWORD = 178;
pub const GREEK_CHARSET: DWORD = 161;
pub const TURKISH_CHARSET: DWORD = 162;
pub const VIETNAMESE_CHARSET: DWORD = 163;
pub const THAI_CHARSET: DWORD = 222;
pub const EASTEUROPE_CHARSET: DWORD = 238;
pub const RUSSIAN_CHARSET: DWORD = 204;
pub const MAC_CHARSET: DWORD = 77;
pub const BALTIC_CHARSET: DWORD = 186;
pub const FS_LATIN1: DWORD = 0x00000001;
pub const FS_LATIN2: DWORD = 0x00000002;
pub const FS_CYRILLIC: DWORD = 0x00000004;
pub const FS_GREEK: DWORD = 0x00000008;
pub const FS_TURKISH: DWORD = 0x00000010;
pub const FS_HEBREW: DWORD = 0x00000020;
pub const FS_ARABIC: DWORD = 0x00000040;
pub const FS_BALTIC: DWORD = 0x00000080;
pub const FS_VIETNAMESE: DWORD = 0x00000100;
pub const FS_THAI: DWORD = 0x00010000;
pub const FS_JISJAPAN: DWORD = 0x00020000;
pub const FS_CHINESESIMP: DWORD = 0x00040000;
pub const FS_WANSUNG: DWORD = 0x00080000;
pub const FS_CHINESETRAD: DWORD = 0x00100000;
pub const FS_JOHAB: DWORD = 0x00200000;
pub const FS_SYMBOL: DWORD = 0x80000000;
pub const FF_DONTCARE: DWORD = 0 << 4;
pub const FF_ROMAN: DWORD = 1 << 4;
pub const FF_SWISS: DWORD = 2 << 4;
pub const FF_MODERN: DWORD = 3 << 4;
pub const FF_SCRIPT: DWORD = 4 << 4;
pub const FF_DECORATIVE: DWORD = 5 << 4;
pub const FW_DONTCARE: c_int = 0;
pub const FW_THIN: c_int = 100;
pub const FW_EXTRALIGHT: c_int = 200;
pub const FW_LIGHT: c_int = 300;
pub const FW_NORMAL: c_int = 400;
pub const FW_MEDIUM: c_int = 500;
pub const FW_SEMIBOLD: c_int = 600;
pub const FW_BOLD: c_int = 700;
pub const FW_EXTRABOLD: c_int = 800;
pub const FW_HEAVY: c_int = 900;
pub const FW_ULTRALIGHT: c_int = FW_EXTRALIGHT;
pub const FW_REGULAR: c_int = FW_NORMAL;
pub const FW_DEMIBOLD: c_int = FW_SEMIBOLD;
pub const FW_ULTRABOLD: c_int = FW_EXTRABOLD;
pub const FW_BLACK: c_int = FW_HEAVY;
pub const PANOSE_COUNT: DWORD = 10;
pub const PAN_FAMILYTYPE_INDEX: DWORD = 0;
pub const PAN_SERIFSTYLE_INDEX: DWORD = 1;
pub const PAN_WEIGHT_INDEX: DWORD = 2;
pub const PAN_PROPORTION_INDEX: DWORD = 3;
pub const PAN_CONTRAST_INDEX: DWORD = 4;
pub const PAN_STROKEVARIATION_INDEX: DWORD = 5;
pub const PAN_ARMSTYLE_INDEX: DWORD = 6;
pub const PAN_LETTERFORM_INDEX: DWORD = 7;
pub const PAN_MIDLINE_INDEX: DWORD = 8;
pub const PAN_XHEIGHT_INDEX: DWORD = 9;
pub const PAN_CULTURE_LATIN: DWORD = 0;
STRUCT!{struct PANOSE {
    bFamilyType: BYTE,
    bSerifStyle: BYTE,
    bWeight: BYTE,
    bProportion: BYTE,
    bContrast: BYTE,
    bStrokeVariation: BYTE,
    bArmStyle: BYTE,
    bLetterform: BYTE,
    bMidline: BYTE,
    bXHeight: BYTE,
}}
pub type LPPANOSE = *mut PANOSE;
pub const PAN_ANY: BYTE = 0;
pub const PAN_NO_FIT: BYTE = 1;
pub const PAN_FAMILY_TEXT_DISPLAY: BYTE = 2;
pub const PAN_FAMILY_SCRIPT: BYTE = 3;
pub const PAN_FAMILY_DECORATIVE: BYTE = 4;
pub const PAN_FAMILY_PICTORIAL: BYTE = 5;
pub const PAN_SERIF_COVE: BYTE = 2;
pub const PAN_SERIF_OBTUSE_COVE: BYTE = 3;
pub const PAN_SERIF_SQUARE_COVE: BYTE = 4;
pub const PAN_SERIF_OBTUSE_SQUARE_COVE: BYTE = 5;
pub const PAN_SERIF_SQUARE: BYTE = 6;
pub const PAN_SERIF_THIN: BYTE = 7;
pub const PAN_SERIF_BONE: BYTE = 8;
pub const PAN_SERIF_EXAGGERATED: BYTE = 9;
pub const PAN_SERIF_TRIANGLE: BYTE = 10;
pub const PAN_SERIF_NORMAL_SANS: BYTE = 11;
pub const PAN_SERIF_OBTUSE_SANS: BYTE = 12;
pub const PAN_SERIF_PERP_SANS: BYTE = 13;
pub const PAN_SERIF_FLARED: BYTE = 14;
pub const PAN_SERIF_ROUNDED: BYTE = 15;
pub const PAN_WEIGHT_VERY_LIGHT: BYTE = 2;
pub const PAN_WEIGHT_LIGHT: BYTE = 3;
pub const PAN_WEIGHT_THIN: BYTE = 4;
pub const PAN_WEIGHT_BOOK: BYTE = 5;
pub const PAN_WEIGHT_MEDIUM: BYTE = 6;
pub const PAN_WEIGHT_DEMI: BYTE = 7;
pub const PAN_WEIGHT_BOLD: BYTE = 8;
pub const PAN_WEIGHT_HEAVY: BYTE = 9;
pub const PAN_WEIGHT_BLACK: BYTE = 10;
pub const PAN_WEIGHT_NORD: BYTE = 11;
pub const PAN_PROP_OLD_STYLE: BYTE = 2;
pub const PAN_PROP_MODERN: BYTE = 3;
pub const PAN_PROP_EVEN_WIDTH: BYTE = 4;
pub const PAN_PROP_EXPANDED: BYTE = 5;
pub const PAN_PROP_CONDENSED: BYTE = 6;
pub const PAN_PROP_VERY_EXPANDED: BYTE = 7;
pub const PAN_PROP_VERY_CONDENSED: BYTE = 8;
pub const PAN_PROP_MONOSPACED: BYTE = 9;
pub const PAN_CONTRAST_NONE: BYTE = 2;
pub const PAN_CONTRAST_VERY_LOW: BYTE = 3;
pub const PAN_CONTRAST_LOW: BYTE = 4;
pub const PAN_CONTRAST_MEDIUM_LOW: BYTE = 5;
pub const PAN_CONTRAST_MEDIUM: BYTE = 6;
pub const PAN_CONTRAST_MEDIUM_HIGH: BYTE = 7;
pub const PAN_CONTRAST_HIGH: BYTE = 8;
pub const PAN_CONTRAST_VERY_HIGH: BYTE = 9;
pub const PAN_STROKE_GRADUAL_DIAG: BYTE = 2;
pub const PAN_STROKE_GRADUAL_TRAN: BYTE = 3;
pub const PAN_STROKE_GRADUAL_VERT: BYTE = 4;
pub const PAN_STROKE_GRADUAL_HORZ: BYTE = 5;
pub const PAN_STROKE_RAPID_VERT: BYTE = 6;
pub const PAN_STROKE_RAPID_HORZ: BYTE = 7;
pub const PAN_STROKE_INSTANT_VERT: BYTE = 8;
pub const PAN_STRAIGHT_ARMS_HORZ: BYTE = 2;
pub const PAN_STRAIGHT_ARMS_WEDGE: BYTE = 3;
pub const PAN_STRAIGHT_ARMS_VERT: BYTE = 4;
pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: BYTE = 5;
pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: BYTE = 6;
pub const PAN_BENT_ARMS_HORZ: BYTE = 7;
pub const PAN_BENT_ARMS_WEDGE: BYTE = 8;
pub const PAN_BENT_ARMS_VERT: BYTE = 9;
pub const PAN_BENT_ARMS_SINGLE_SERIF: BYTE = 10;
pub const PAN_BENT_ARMS_DOUBLE_SERIF: BYTE = 11;
pub const PAN_LETT_NORMAL_CONTACT: BYTE = 2;
pub const PAN_LETT_NORMAL_WEIGHTED: BYTE = 3;
pub const PAN_LETT_NORMAL_BOXED: BYTE = 4;
pub const PAN_LETT_NORMAL_FLATTENED: BYTE = 5;
pub const PAN_LETT_NORMAL_ROUNDED: BYTE = 6;
pub const PAN_LETT_NORMAL_OFF_CENTER: BYTE = 7;
pub const PAN_LETT_NORMAL_SQUARE: BYTE = 8;
pub const PAN_LETT_OBLIQUE_CONTACT: BYTE = 9;
pub const PAN_LETT_OBLIQUE_WEIGHTED: BYTE = 10;
pub const PAN_LETT_OBLIQUE_BOXED: BYTE = 11;
pub const PAN_LETT_OBLIQUE_FLATTENED: BYTE = 12;
pub const PAN_LETT_OBLIQUE_ROUNDED: BYTE = 13;
pub const PAN_LETT_OBLIQUE_OFF_CENTER: BYTE = 14;
pub const PAN_LETT_OBLIQUE_SQUARE: BYTE = 15;
pub const PAN_MIDLINE_STANDARD_TRIMMED: BYTE = 2;
pub const PAN_MIDLINE_STANDARD_POINTED: BYTE = 3;
pub const PAN_MIDLINE_STANDARD_SERIFED: BYTE = 4;
pub const PAN_MIDLINE_HIGH_TRIMMED: BYTE = 5;
pub const PAN_MIDLINE_HIGH_POINTED: BYTE = 6;
pub const PAN_MIDLINE_HIGH_SERIFED: BYTE = 7;
pub const PAN_MIDLINE_CONSTANT_TRIMMED: BYTE = 8;
pub const PAN_MIDLINE_CONSTANT_POINTED: BYTE = 9;
pub const PAN_MIDLINE_CONSTANT_SERIFED: BYTE = 10;
pub const PAN_MIDLINE_LOW_TRIMMED: BYTE = 11;
pub const PAN_MIDLINE_LOW_POINTED: BYTE = 12;
pub const PAN_MIDLINE_LOW_SERIFED: BYTE = 13;
pub const PAN_XHEIGHT_CONSTANT_SMALL: BYTE = 2;
pub const PAN_XHEIGHT_CONSTANT_STD: BYTE = 3;
pub const PAN_XHEIGHT_CONSTANT_LARGE: BYTE = 4;
pub const PAN_XHEIGHT_DUCKING_SMALL: BYTE = 5;
pub const PAN_XHEIGHT_DUCKING_STD: BYTE = 6;
pub const PAN_XHEIGHT_DUCKING_LARGE: BYTE = 7;
pub const ELF_VENDOR_SIZE: usize = 4;
STRUCT!{struct EXTLOGFONTA {
    elfLogFont: LOGFONTA,
    elfFullName: [BYTE; LF_FULLFACESIZE],
    elfStyle: [BYTE; LF_FACESIZE],
    elfVersion: DWORD,
    elfStyleSize: DWORD,
    elfMatch: DWORD,
    elfReserved: DWORD,
    elfVendorId: [BYTE; ELF_VENDOR_SIZE],
    elfCulture: DWORD,
    elfPanose: PANOSE,
}}
pub type PEXTLOGFONTA = *mut EXTLOGFONTA;
pub type NPEXTLOGFONTA = *mut EXTLOGFONTA;
pub type LPEXTLOGFONTA = *mut EXTLOGFONTA;
STRUCT!{struct EXTLOGFONTW {
    elfLogFont: LOGFONTW,
    elfFullNam: [WCHAR; LF_FULLFACESIZE],
    elfStyle: [WCHAR; LF_FACESIZE],
    elfVersion: DWORD,
    elfStyleSize: DWORD,
    elfMatch: DWORD,
    elfReserved: DWORD,
    elfVendorId: [BYTE; ELF_VENDOR_SIZE],
    elfCulture: DWORD,
    elfPanose: PANOSE,
}}
pub type PEXTLOGFONTW = *mut EXTLOGFONTW;
pub type NPEXTLOGFONTW = *mut EXTLOGFONTW;
pub type LPEXTLOGFONTW = *mut EXTLOGFONTW;
pub const ELF_VERSION: DWORD = 0;
pub const ELF_CULTURE_LATIN: DWORD = 0;
pub const RASTER_FONTTYPE: DWORD = 0x0001;
pub const DEVICE_FONTTYPE: DWORD = 0x0002;
pub const TRUETYPE_FONTTYPE: DWORD = 0x0004;
#[inline]
pub fn RGB(r: BYTE, g: BYTE, b: BYTE) -> COLORREF {
    r as COLORREF | ((g as COLORREF) << 8) | ((b as COLORREF) << 16)
}
#[inline]
pub fn PALETTERGB(r: BYTE, g: BYTE, b: BYTE) -> COLORREF {
    0x02000000 | RGB(r, g, b)
}
#[inline]
pub fn PALETTEINDEX(i: WORD) -> COLORREF {
    0x01000000 | i as DWORD
}
pub const PC_RESERVED: DWORD = 0x01;
pub const PC_EXPLICIT: DWORD = 0x02;
pub const PC_NOCOLLAPSE: DWORD = 0x04;
#[inline]
pub fn GetRValue(rgb: COLORREF) -> BYTE {
    LOBYTE(rgb as WORD)
}
#[inline]
pub fn GetGValue(rgb: COLORREF) -> BYTE {
    LOBYTE((rgb as WORD) >> 8)
}
#[inline]
pub fn GetBValue(rgb: COLORREF) -> BYTE {
    LOBYTE((rgb >> 16) as WORD)
}
pub const TRANSPARENT: DWORD = 1;
pub const OPAQUE: DWORD = 2;
pub const BKMODE_LAST: DWORD = 2;
pub const GM_COMPATIBLE: DWORD = 1;
pub const GM_ADVANCED: DWORD = 2;
pub const GM_LAST: DWORD = 2;
pub const PT_CLOSEFIGURE: DWORD = 0x01;
pub const PT_LINETO: DWORD = 0x02;
pub const PT_BEZIERTO: DWORD = 0x04;
pub const PT_MOVETO: DWORD = 0x06;
pub const MM_TEXT: DWORD = 1;
pub const MM_LOMETRIC: DWORD = 2;
pub const MM_HIMETRIC: DWORD = 3;
pub const MM_LOENGLISH: DWORD = 4;
pub const MM_HIENGLISH: DWORD = 5;
pub const MM_TWIPS: DWORD = 6;
pub const MM_ISOTROPIC: DWORD = 7;
pub const MM_ANISOTROPIC: DWORD = 8;
pub const MM_MIN: DWORD = MM_TEXT;
pub const MM_MAX: DWORD = MM_ANISOTROPIC;
pub const MM_MAX_FIXEDSCALE: DWORD = MM_TWIPS;
pub const ABSOLUTE: DWORD = 1;
pub const RELATIVE: DWORD = 2;
pub const WHITE_BRUSH: DWORD = 0;
pub const LTGRAY_BRUSH: DWORD = 1;
pub const GRAY_BRUSH: DWORD = 2;
pub const DKGRAY_BRUSH: DWORD = 3;
pub const BLACK_BRUSH: DWORD = 4;
pub const NULL_BRUSH: DWORD = 5;
pub const HOLLOW_BRUSH: DWORD = NULL_BRUSH;
pub const WHITE_PEN: DWORD = 6;
pub const BLACK_PEN: DWORD = 7;
pub const NULL_PEN: DWORD = 8;
pub const OEM_FIXED_FONT: DWORD = 10;
pub const ANSI_FIXED_FONT: DWORD = 11;
pub const ANSI_VAR_FONT: DWORD = 12;
pub const SYSTEM_FONT: DWORD = 13;
pub const DEVICE_DEFAULT_FONT: DWORD = 14;
pub const DEFAULT_PALETTE: DWORD = 15;
pub const SYSTEM_FIXED_FONT: DWORD = 16;
pub const DEFAULT_GUI_FONT: DWORD = 17;
pub const DC_BRUSH: DWORD = 18;
pub const DC_PEN: DWORD = 19;
pub const STOCK_LAST: DWORD = 19;
pub const CLR_INVALID: COLORREF = 0xFFFFFFFF;
pub const BS_SOLID: DWORD = 0;
pub const BS_NULL: DWORD = 1;
pub const BS_HOLLOW: DWORD = BS_NULL;
pub const BS_HATCHED: DWORD = 2;
pub const BS_PATTERN: DWORD = 3;
pub const BS_INDEXED: DWORD = 4;
pub const BS_DIBPATTERN: DWORD = 5;
pub const BS_DIBPATTERNPT: DWORD = 6;
pub const BS_PATTERN8X8: DWORD = 7;
pub const BS_DIBPATTERN8X8: DWORD = 8;
pub const BS_MONOPATTERN: DWORD = 9;
pub const HS_HORIZONTAL: DWORD = 0;
pub const HS_VERTICAL: DWORD = 1;
pub const HS_FDIAGONAL: DWORD = 2;
pub const HS_BDIAGONAL: DWORD = 3;
pub const HS_CROSS: DWORD = 4;
pub const HS_DIAGCROSS: DWORD = 5;
pub const HS_API_MAX: DWORD = 12;
pub const PS_SOLID: DWORD = 0;
pub const PS_DASH: DWORD = 1;
pub const PS_DOT: DWORD = 2;
pub const PS_DASHDOT: DWORD = 3;
pub const PS_DASHDOTDOT: DWORD = 4;
pub const PS_NULL: DWORD = 5;
pub const PS_INSIDEFRAME: DWORD = 6;
pub const PS_USERSTYLE: DWORD = 7;
pub const PS_ALTERNATE: DWORD = 8;
pub const PS_STYLE_MASK: DWORD = 0x0000000F;
pub const PS_ENDCAP_ROUND: DWORD = 0x00000000;
pub const PS_ENDCAP_SQUARE: DWORD = 0x00000100;
pub const PS_ENDCAP_FLAT: DWORD = 0x00000200;
pub const PS_ENDCAP_MASK: DWORD = 0x00000F00;
pub const PS_JOIN_ROUND: DWORD = 0x00000000;
pub const PS_JOIN_BEVEL: DWORD = 0x00001000;
pub const PS_JOIN_MITER: DWORD = 0x00002000;
pub const PS_JOIN_MASK: DWORD = 0x0000F000;
pub const PS_COSMETIC: DWORD = 0x00000000;
pub const PS_GEOMETRIC: DWORD = 0x00010000;
pub const PS_TYPE_MASK: DWORD = 0x000F0000;
pub const AD_COUNTERCLOCKWISE: DWORD = 1;
pub const AD_CLOCKWISE: DWORD = 2;
pub const DRIVERVERSION: c_int = 0;
pub const TECHNOLOGY: c_int = 2;
pub const HORZSIZE: c_int = 4;
pub const VERTSIZE: c_int = 6;
pub const HORZRES: c_int = 8;
pub const VERTRES: c_int = 10;
pub const BITSPIXEL: c_int = 12;
pub const PLANES: c_int = 14;
pub const NUMBRUSHES: c_int = 16;
pub const NUMPENS: c_int = 18;
pub const NUMMARKERS: c_int = 20;
pub const NUMFONTS: c_int = 22;
pub const NUMCOLORS: c_int = 24;
pub const PDEVICESIZE: c_int = 26;
pub const CURVECAPS: c_int = 28;
pub const LINECAPS: c_int = 30;
pub const POLYGONALCAPS: c_int = 32;
pub const TEXTCAPS: c_int = 34;
pub const CLIPCAPS: c_int = 36;
pub const RASTERCAPS: c_int = 38;
pub const ASPECTX: c_int = 40;
pub const ASPECTY: c_int = 42;
pub const ASPECTXY: c_int = 44;
pub const LOGPIXELSX: c_int = 88;
pub const LOGPIXELSY: c_int = 90;
pub const SIZEPALETTE: c_int = 104;
pub const NUMRESERVED: c_int = 106;
pub const COLORRES: c_int = 108;
pub const PHYSICALWIDTH: c_int = 110;
pub const PHYSICALHEIGHT: c_int = 111;
pub const PHYSICALOFFSETX: c_int = 112;
pub const PHYSICALOFFSETY: c_int = 113;
pub const SCALINGFACTORX: c_int = 114;
pub const SCALINGFACTORY: c_int = 115;
pub const VREFRESH: c_int = 116;
pub const DESKTOPVERTRES: c_int = 117;
pub const DESKTOPHORZRES: c_int = 118;
pub const BLTALIGNMENT: c_int = 119;
pub const SHADEBLENDCAPS: c_int = 120;
pub const COLORMGMTCAPS: c_int = 121;
pub const DT_PLOTTER: DWORD = 0;
pub const DT_RASDISPLAY: DWORD = 1;
pub const DT_RASPRINTER: DWORD = 2;
pub const DT_RASCAMERA: DWORD = 3;
pub const DT_CHARSTREAM: DWORD = 4;
pub const DT_METAFILE: DWORD = 5;
pub const DT_DISPFILE: DWORD = 6;
pub const CC_NONE: DWORD = 0;
pub const CC_CIRCLES: DWORD = 1;
pub const CC_PIE: DWORD = 2;
pub const CC_CHORD: DWORD = 4;
pub const CC_ELLIPSES: DWORD = 8;
pub const CC_WIDE: DWORD = 16;
pub const CC_STYLED: DWORD = 32;
pub const CC_WIDESTYLED: DWORD = 64;
pub const CC_INTERIORS: DWORD = 128;
pub const CC_ROUNDRECT: DWORD = 256;
pub const LC_NONE: DWORD = 0;
pub const LC_POLYLINE: DWORD = 2;
pub const LC_MARKER: DWORD = 4;
pub const LC_POLYMARKER: DWORD = 8;
pub const LC_WIDE: DWORD = 16;
pub const LC_STYLED: DWORD = 32;
pub const LC_WIDESTYLED: DWORD = 64;
pub const LC_INTERIORS: DWORD = 128;
pub const PC_NONE: DWORD = 0;
pub const PC_POLYGON: DWORD = 1;
pub const PC_RECTANGLE: DWORD = 2;
pub const PC_WINDPOLYGON: DWORD = 4;
pub const PC_TRAPEZOID: DWORD = 4;
pub const PC_SCANLINE: DWORD = 8;
pub const PC_WIDE: DWORD = 16;
pub const PC_STYLED: DWORD = 32;
pub const PC_WIDESTYLED: DWORD = 64;
pub const PC_INTERIORS: DWORD = 128;
pub const PC_POLYPOLYGON: DWORD = 256;
pub const PC_PATHS: DWORD = 512;
pub const CP_NONE: DWORD = 0;
pub const CP_RECTANGLE: DWORD = 1;
pub const CP_REGION: DWORD = 2;
pub const TC_OP_CHARACTER: DWORD = 0x00000001;
pub const TC_OP_STROKE: DWORD = 0x00000002;
pub const TC_CP_STROKE: DWORD = 0x00000004;
pub const TC_CR_90: DWORD = 0x00000008;
pub const TC_CR_ANY: DWORD = 0x00000010;
pub const TC_SF_X_YINDEP: DWORD = 0x00000020;
pub const TC_SA_DOUBLE: DWORD = 0x00000040;
pub const TC_SA_INTEGER: DWORD = 0x00000080;
pub const TC_SA_CONTIN: DWORD = 0x00000100;
pub const TC_EA_DOUBLE: DWORD = 0x00000200;
pub const TC_IA_ABLE: DWORD = 0x00000400;
pub const TC_UA_ABLE: DWORD = 0x00000800;
pub const TC_SO_ABLE: DWORD = 0x00001000;
pub const TC_RA_ABLE: DWORD = 0x00002000;
pub const TC_VA_ABLE: DWORD = 0x00004000;
pub const TC_RESERVED: DWORD = 0x00008000;
pub const TC_SCROLLBLT: DWORD = 0x00010000;
pub const RC_BITBLT: DWORD = 1;
pub const RC_BANDING: DWORD = 2;
pub const RC_SCALING: DWORD = 4;
pub const RC_BITMAP64: DWORD = 8;
pub const RC_GDI20_OUTPUT: DWORD = 0x0010;
pub const RC_GDI20_STATE: DWORD = 0x0020;
pub const RC_SAVEBITMAP: DWORD = 0x0040;
pub const RC_DI_BITMAP: DWORD = 0x0080;
pub const RC_PALETTE: DWORD = 0x0100;
pub const RC_DIBTODEV: DWORD = 0x0200;
pub const RC_BIGFONT: DWORD = 0x0400;
pub const RC_STRETCHBLT: DWORD = 0x0800;
pub const RC_FLOODFILL: DWORD = 0x1000;
pub const RC_STRETCHDIB: DWORD = 0x2000;
pub const RC_OP_DX_OUTPUT: DWORD = 0x4000;
pub const RC_DEVBITS: DWORD = 0x8000;
pub const SB_NONE: DWORD = 0x00000000;
pub const SB_CONST_ALPHA: DWORD = 0x00000001;
pub const SB_PIXEL_ALPHA: DWORD = 0x00000002;
pub const SB_PREMULT_ALPHA: DWORD = 0x00000004;
pub const SB_GRAD_RECT: DWORD = 0x00000010;
pub const SB_GRAD_TRI: DWORD = 0x00000020;
pub const CM_NONE: DWORD = 0x00000000;
pub const CM_DEVICE_ICM: DWORD = 0x00000001;
pub const CM_GAMMA_RAMP: DWORD = 0x00000002;
pub const CM_CMYK_COLOR: DWORD = 0x00000004;
pub const DIB_RGB_COLORS: DWORD = 0;
pub const DIB_PAL_COLORS: DWORD = 1;
pub const SYSPAL_ERROR: DWORD = 0;
pub const SYSPAL_STATIC: DWORD = 1;
pub const SYSPAL_NOSTATIC: DWORD = 2;
pub const SYSPAL_NOSTATIC256: DWORD = 3;
pub const CBM_INIT: DWORD = 0x04;
pub const FLOODFILLBORDER: DWORD = 0;
pub const FLOODFILLSURFACE: DWORD = 1;
pub const CCHDEVICENAME: usize = 32;
pub const CCHFORMNAME: usize = 32;
STRUCT!{struct DEVMODE_u1_s1 {
    dmOrientation: c_short,
    dmPaperSize: c_short,
    dmPaperLength: c_short,
    dmPaperWidth: c_short,
    dmScale: c_short,
    dmCopies: c_short,
    dmDefaultSource: c_short,
    dmPrintQuality: c_short,
}}
STRUCT!{struct DEVMODE_u1_s2 {
    dmPosition: POINTL,
    dmDisplayOrientation: DWORD,
    dmDisplayFixedOutput: DWORD,
}}
UNION!{union DEVMODE_u1 {
    [u32; 4],
    s1 s1_mut: DEVMODE_u1_s1,
    s2 s2_mut: DEVMODE_u1_s2,
}}
UNION!{union DEVMODE_u2 {
    [u32; 1],
    dmDisplayFlags dmDisplayFlags_mut: DWORD,
    dmNup dmNup_mut: DWORD,
}}
STRUCT!{struct DEVMODEA {
    dmDeviceName: [CHAR; CCHDEVICENAME],
    dmSpecVersion: WORD,
    dmDriverVersion: WORD,
    dmSize: WORD,
    dmDriverExtra: WORD,
    dmFields: DWORD,
    u1: DEVMODE_u1,
    dmColor: c_short,
    dmDuplex: c_short,
    dmYResolution: c_short,
    dmTTOption: c_short,
    dmCollate: c_short,
    dmFormName: [CHAR; CCHFORMNAME],
    dmLogPixels: WORD,
    dmBitsPerPel: DWORD,
    dmPelsWidth: DWORD,
    dmPelsHeight: DWORD,
    u2: DEVMODE_u2,
    dmDisplayFrequency: DWORD,
    dmICMMethod: DWORD,
    dmICMIntent: DWORD,
    dmMediaType: DWORD,
    dmDitherType: DWORD,
    dmReserved1: DWORD,
    dmReserved2: DWORD,
    dmPanningWidth: DWORD,
    dmPanningHeight: DWORD,
}}
pub type PDEVMODEA = *mut DEVMODEA;
pub type NPDEVMODEA = *mut DEVMODEA;
pub type LPDEVMODEA = *mut DEVMODEA;
STRUCT!{struct DEVMODEW {
    dmDeviceName: [WCHAR; CCHDEVICENAME],
    dmSpecVersion: WORD,
    dmDriverVersion: WORD,
    dmSize: WORD,
    dmDriverExtra: WORD,
    dmFields: DWORD,
    u1: DEVMODE_u1,
    dmColor: c_short,
    dmDuplex: c_short,
    dmYResolution: c_short,
    dmTTOption: c_short,
    dmCollate: c_short,
    dmFormName: [WCHAR; CCHFORMNAME],
    dmLogPixels: WORD,
    dmBitsPerPel: DWORD,
    dmPelsWidth: DWORD,
    dmPelsHeight: DWORD,
    u2: DEVMODE_u2,
    dmDisplayFrequency: DWORD,
    dmICMMethod: DWORD,
    dmICMIntent: DWORD,
    dmMediaType: DWORD,
    dmDitherType: DWORD,
    dmReserved1: DWORD,
    dmReserved2: DWORD,
    dmPanningWidth: DWORD,
    dmPanningHeight: DWORD,
}}
pub type PDEVMODEW = *mut DEVMODEW;
pub type NPDEVMODEW = *mut DEVMODEW;
pub type LPDEVMODEW = *mut DEVMODEW;
pub const DM_SPECVERSION: DWORD = 0x0401;
pub const DM_ORIENTATION: DWORD = 0x00000001;
pub const DM_PAPERSIZE: DWORD = 0x00000002;
pub const DM_PAPERLENGTH: DWORD = 0x00000004;
pub const DM_PAPERWIDTH: DWORD = 0x00000008;
pub const DM_SCALE: DWORD = 0x00000010;
pub const DM_POSITION: DWORD = 0x00000020;
pub const DM_NUP: DWORD = 0x00000040;
pub const DM_DISPLAYORIENTATION: DWORD = 0x00000080;
pub const DM_COPIES: DWORD = 0x00000100;
pub const DM_DEFAULTSOURCE: DWORD = 0x00000200;
pub const DM_PRINTQUALITY: DWORD = 0x00000400;
pub const DM_COLOR: DWORD = 0x00000800;
pub const DM_DUPLEX: DWORD = 0x00001000;
pub const DM_YRESOLUTION: DWORD = 0x00002000;
pub const DM_TTOPTION: DWORD = 0x00004000;
pub const DM_COLLATE: DWORD = 0x00008000;
pub const DM_FORMNAME: DWORD = 0x00010000;
pub const DM_LOGPIXELS: DWORD = 0x00020000;
pub const DM_BITSPERPEL: DWORD = 0x00040000;
pub const DM_PELSWIDTH: DWORD = 0x00080000;
pub const DM_PELSHEIGHT: DWORD = 0x00100000;
pub const DM_DISPLAYFLAGS: DWORD = 0x00200000;
pub const DM_DISPLAYFREQUENCY: DWORD = 0x00400000;
pub const DM_ICMMETHOD: DWORD = 0x00800000;
pub const DM_ICMINTENT: DWORD = 0x01000000;
pub const DM_MEDIATYPE: DWORD = 0x02000000;
pub const DM_DITHERTYPE: DWORD = 0x04000000;
pub const DM_PANNINGWIDTH: DWORD = 0x08000000;
pub const DM_PANNINGHEIGHT: DWORD = 0x10000000;
pub const DM_DISPLAYFIXEDOUTPUT: DWORD = 0x20000000;
pub const DMORIENT_PORTRAIT: DWORD = 1;
pub const DMORIENT_LANDSCAPE: DWORD = 2;
pub const DMPAPER_FIRST: DWORD = DMPAPER_LETTER;
pub const DMPAPER_LETTER: DWORD = 1;
pub const DMPAPER_LETTERSMALL: DWORD = 2;
pub const DMPAPER_TABLOID: DWORD = 3;
pub const DMPAPER_LEDGER: DWORD = 4;
pub const DMPAPER_LEGAL: DWORD = 5;
pub const DMPAPER_STATEMENT: DWORD = 6;
pub const DMPAPER_EXECUTIVE: DWORD = 7;
pub const DMPAPER_A3: DWORD = 8;
pub const DMPAPER_A4: DWORD = 9;
pub const DMPAPER_A4SMALL: DWORD = 10;
pub const DMPAPER_A5: DWORD = 11;
pub const DMPAPER_B4: DWORD = 12;
pub const DMPAPER_B5: DWORD = 13;
pub const DMPAPER_FOLIO: DWORD = 14;
pub const DMPAPER_QUARTO: DWORD = 15;
pub const DMPAPER_10X14: DWORD = 16;
pub const DMPAPER_11X17: DWORD = 17;
pub const DMPAPER_NOTE: DWORD = 18;
pub const DMPAPER_ENV_9: DWORD = 19;
pub const DMPAPER_ENV_10: DWORD = 20;
pub const DMPAPER_ENV_11: DWORD = 21;
pub const DMPAPER_ENV_12: DWORD = 22;
pub const DMPAPER_ENV_14: DWORD = 23;
pub const DMPAPER_CSHEET: DWORD = 24;
pub const DMPAPER_DSHEET: DWORD = 25;
pub const DMPAPER_ESHEET: DWORD = 26;
pub const DMPAPER_ENV_DL: DWORD = 27;
pub const DMPAPER_ENV_C5: DWORD = 28;
pub const DMPAPER_ENV_C3: DWORD = 29;
pub const DMPAPER_ENV_C4: DWORD = 30;
pub const DMPAPER_ENV_C6: DWORD = 31;
pub const DMPAPER_ENV_C65: DWORD = 32;
pub const DMPAPER_ENV_B4: DWORD = 33;
pub const DMPAPER_ENV_B5: DWORD = 34;
pub const DMPAPER_ENV_B6: DWORD = 35;
pub const DMPAPER_ENV_ITALY: DWORD = 36;
pub const DMPAPER_ENV_MONARCH: DWORD = 37;
pub const DMPAPER_ENV_PERSONAL: DWORD = 38;
pub const DMPAPER_FANFOLD_US: DWORD = 39;
pub const DMPAPER_FANFOLD_STD_GERMAN: DWORD = 40;
pub const DMPAPER_FANFOLD_LGL_GERMAN: DWORD = 41;
pub const DMPAPER_ISO_B4: DWORD = 42;
pub const DMPAPER_JAPANESE_POSTCARD: DWORD = 43;
pub const DMPAPER_9X11: DWORD = 44;
pub const DMPAPER_10X11: DWORD = 45;
pub const DMPAPER_15X11: DWORD = 46;
pub const DMPAPER_ENV_INVITE: DWORD = 47;
pub const DMPAPER_RESERVED_48: DWORD = 48;
pub const DMPAPER_RESERVED_49: DWORD = 49;
pub const DMPAPER_LETTER_EXTRA: DWORD = 50;
pub const DMPAPER_LEGAL_EXTRA: DWORD = 51;
pub const DMPAPER_TABLOID_EXTRA: DWORD = 52;
pub const DMPAPER_A4_EXTRA: DWORD = 53;
pub const DMPAPER_LETTER_TRANSVERSE: DWORD = 54;
pub const DMPAPER_A4_TRANSVERSE: DWORD = 55;
pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: DWORD = 56;
pub const DMPAPER_A_PLUS: DWORD = 57;
pub const DMPAPER_B_PLUS: DWORD = 58;
pub const DMPAPER_LETTER_PLUS: DWORD = 59;
pub const DMPAPER_A4_PLUS: DWORD = 60;
pub const DMPAPER_A5_TRANSVERSE: DWORD = 61;
pub const DMPAPER_B5_TRANSVERSE: DWORD = 62;
pub const DMPAPER_A3_EXTRA: DWORD = 63;
pub const DMPAPER_A5_EXTRA: DWORD = 64;
pub const DMPAPER_B5_EXTRA: DWORD = 65;
pub const DMPAPER_A2: DWORD = 66;
pub const DMPAPER_A3_TRANSVERSE: DWORD = 67;
pub const DMPAPER_A3_EXTRA_TRANSVERSE: DWORD = 68;
pub const DMPAPER_DBL_JAPANESE_POSTCARD: DWORD = 69;
pub const DMPAPER_A6: DWORD = 70;
pub const DMPAPER_JENV_KAKU2: DWORD = 71;
pub const DMPAPER_JENV_KAKU3: DWORD = 72;
pub const DMPAPER_JENV_CHOU3: DWORD = 73;
pub const DMPAPER_JENV_CHOU4: DWORD = 74;
pub const DMPAPER_LETTER_ROTATED: DWORD = 75;
pub const DMPAPER_A3_ROTATED: DWORD = 76;
pub const DMPAPER_A4_ROTATED: DWORD = 77;
pub const DMPAPER_A5_ROTATED: DWORD = 78;
pub const DMPAPER_B4_JIS_ROTATED: DWORD = 79;
pub const DMPAPER_B5_JIS_ROTATED: DWORD = 80;
pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: DWORD = 81;
pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: DWORD = 82;
pub const DMPAPER_A6_ROTATED: DWORD = 83;
pub const DMPAPER_JENV_KAKU2_ROTATED: DWORD = 84;
pub const DMPAPER_JENV_KAKU3_ROTATED: DWORD = 85;
pub const DMPAPER_JENV_CHOU3_ROTATED: DWORD = 86;
pub const DMPAPER_JENV_CHOU4_ROTATED: DWORD = 87;
pub const DMPAPER_B6_JIS: DWORD = 88;
pub const DMPAPER_B6_JIS_ROTATED: DWORD = 89;
pub const DMPAPER_12X11: DWORD = 90;
pub const DMPAPER_JENV_YOU4: DWORD = 91;
pub const DMPAPER_JENV_YOU4_ROTATED: DWORD = 92;
pub const DMPAPER_P16K: DWORD = 93;
pub const DMPAPER_P32K: DWORD = 94;
pub const DMPAPER_P32KBIG: DWORD = 95;
pub const DMPAPER_PENV_1: DWORD = 96;
pub const DMPAPER_PENV_2: DWORD = 97;
pub const DMPAPER_PENV_3: DWORD = 98;
pub const DMPAPER_PENV_4: DWORD = 99;
pub const DMPAPER_PENV_5: DWORD = 100;
pub const DMPAPER_PENV_6: DWORD = 101;
pub const DMPAPER_PENV_7: DWORD = 102;
pub const DMPAPER_PENV_8: DWORD = 103;
pub const DMPAPER_PENV_9: DWORD = 104;
pub const DMPAPER_PENV_10: DWORD = 105;
pub const DMPAPER_P16K_ROTATED: DWORD = 106;
pub const DMPAPER_P32K_ROTATED: DWORD = 107;
pub const DMPAPER_P32KBIG_ROTATED: DWORD = 108;
pub const DMPAPER_PENV_1_ROTATED: DWORD = 109;
pub const DMPAPER_PENV_2_ROTATED: DWORD = 110;
pub const DMPAPER_PENV_3_ROTATED: DWORD = 111;
pub const DMPAPER_PENV_4_ROTATED: DWORD = 112;
pub const DMPAPER_PENV_5_ROTATED: DWORD = 113;
pub const DMPAPER_PENV_6_ROTATED: DWORD = 114;
pub const DMPAPER_PENV_7_ROTATED: DWORD = 115;
pub const DMPAPER_PENV_8_ROTATED: DWORD = 116;
pub const DMPAPER_PENV_9_ROTATED: DWORD = 117;
pub const DMPAPER_PENV_10_ROTATED: DWORD = 118;
pub const DMPAPER_LAST: DWORD = DMPAPER_PENV_10_ROTATED;
pub const DMPAPER_USER: DWORD = 256;
pub const DMBIN_FIRST: DWORD = DMBIN_UPPER;
pub const DMBIN_UPPER: DWORD = 1;
pub const DMBIN_ONLYONE: DWORD = 1;
pub const DMBIN_LOWER: DWORD = 2;
pub const DMBIN_MIDDLE: DWORD = 3;
pub const DMBIN_MANUAL: DWORD = 4;
pub const DMBIN_ENVELOPE: DWORD = 5;
pub const DMBIN_ENVMANUAL: DWORD = 6;
pub const DMBIN_AUTO: DWORD = 7;
pub const DMBIN_TRACTOR: DWORD = 8;
pub const DMBIN_SMALLFMT: DWORD = 9;
pub const DMBIN_LARGEFMT: DWORD = 10;
pub const DMBIN_LARGECAPACITY: DWORD = 11;
pub const DMBIN_CASSETTE: DWORD = 14;
pub const DMBIN_FORMSOURCE: DWORD = 15;
pub const DMBIN_LAST: DWORD = DMBIN_FORMSOURCE;
pub const DMBIN_USER: DWORD = 256;
pub const DMRES_DRAFT: c_int = -1;
pub const DMRES_LOW: c_int = -2;
pub const DMRES_MEDIUM: c_int = -3;
pub const DMRES_HIGH: c_int = -4;
pub const DMCOLOR_MONOCHROME: DWORD = 1;
pub const DMCOLOR_COLOR: DWORD = 2;
pub const DMDUP_SIMPLEX: DWORD = 1;
pub const DMDUP_VERTICAL: DWORD = 2;
pub const DMDUP_HORIZONTAL: DWORD = 3;
pub const DMTT_BITMAP: DWORD = 1;
pub const DMTT_DOWNLOAD: DWORD = 2;
pub const DMTT_SUBDEV: DWORD = 3;
pub const DMTT_DOWNLOAD_OUTLINE: DWORD = 4;
pub const DMCOLLATE_FALSE: DWORD = 0;
pub const DMCOLLATE_TRUE: DWORD = 1;
pub const DMDO_DEFAULT: DWORD = 0;
pub const DMDO_90: DWORD = 1;
pub const DMDO_180: DWORD = 2;
pub const DMDO_270: DWORD = 3;
pub const DMDFO_DEFAULT: DWORD = 0;
pub const DMDFO_STRETCH: DWORD = 1;
pub const DMDFO_CENTER: DWORD = 2;
pub const DM_INTERLACED: DWORD = 0x00000002;
pub const DMDISPLAYFLAGS_TEXTMODE: DWORD = 0x00000004;
pub const DMNUP_SYSTEM: DWORD = 1;
pub const DMNUP_ONEUP: DWORD = 2;
pub const DMICMMETHOD_NONE: DWORD = 1;
pub const DMICMMETHOD_SYSTEM: DWORD = 2;
pub const DMICMMETHOD_DRIVER: DWORD = 3;
pub const DMICMMETHOD_DEVICE: DWORD = 4;
pub const DMICMMETHOD_USER: DWORD = 256;
pub const DMICM_SATURATE: DWORD = 1;
pub const DMICM_CONTRAST: DWORD = 2;
pub const DMICM_COLORIMETRIC: DWORD = 3;
pub const DMICM_ABS_COLORIMETRIC: DWORD = 4;
pub const DMICM_USER: DWORD = 256;
pub const DMMEDIA_STANDARD: DWORD = 1;
pub const DMMEDIA_TRANSPARENCY: DWORD = 2;
pub const DMMEDIA_GLOSSY: DWORD = 3;
pub const DMMEDIA_USER: DWORD = 256;
pub const DMDITHER_NONE: DWORD = 1;
pub const DMDITHER_COARSE: DWORD = 2;
pub const DMDITHER_FINE: DWORD = 3;
pub const DMDITHER_LINEART: DWORD = 4;
pub const DMDITHER_ERRORDIFFUSION: DWORD = 5;
pub const DMDITHER_RESERVED6: DWORD = 6;
pub const DMDITHER_RESERVED7: DWORD = 7;
pub const DMDITHER_RESERVED8: DWORD = 8;
pub const DMDITHER_RESERVED9: DWORD = 9;
pub const DMDITHER_GRAYSCALE: DWORD = 10;
pub const DMDITHER_USER: DWORD = 256;
STRUCT!{struct DISPLAY_DEVICEA {
    cb: DWORD,
    DeviceName: [CHAR; 32],
    DeviceString: [CHAR; 128],
    StateFlags: DWORD,
    DeviceID: [CHAR; 128],
    DeviceKey: [CHAR; 128],
}}
pub type PDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
pub type LPDISPLAY_DEVICEA = *mut DISPLAY_DEVICEA;
STRUCT!{struct DISPLAY_DEVICEW {
    cb: DWORD,
    DeviceName: [WCHAR; 32],
    DeviceString: [WCHAR; 128],
    StateFlags: DWORD,
    DeviceID: [WCHAR; 128],
    DeviceKey: [WCHAR; 128],
}}
pub type PDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub type LPDISPLAY_DEVICEW = *mut DISPLAY_DEVICEW;
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_MULTI_DRIVER: DWORD = 0x00000002;
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: DWORD = 0x00000004;
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: DWORD = 0x00000008;
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: DWORD = 0x00000010;
pub const DISPLAY_DEVICE_REMOVABLE: DWORD = 0x00000020;
pub const DISPLAY_DEVICE_ACC_DRIVER: DWORD = 0x00000040;
pub const DISPLAY_DEVICE_MODESPRUNED: DWORD = 0x08000000;
pub const DISPLAY_DEVICE_RDPUDD: DWORD = 0x01000000;
pub const DISPLAY_DEVICE_REMOTE: DWORD = 0x04000000;
pub const DISPLAY_DEVICE_DISCONNECT: DWORD = 0x02000000;
pub const DISPLAY_DEVICE_TS_COMPATIBLE: DWORD = 0x00200000;
pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: DWORD = 0x00080000;
pub const DISPLAY_DEVICE_ACTIVE: DWORD = 0x00000001;
pub const DISPLAY_DEVICE_ATTACHED: DWORD = 0x00000002;
pub const DISPLAYCONFIG_MAXPATH: usize = 1024;
STRUCT!{struct DISPLAYCONFIG_RATIONAL {
    Numerator: UINT32,
    Denominator: UINT32,
}}
ENUM!{enum DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER = -1i32 as u32,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15 = 0,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO = 1,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO = 2,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO = 3,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI = 4,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI = 5,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS = 6,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN = 8,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI = 9,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL = 10,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED = 11,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL = 12,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED = 13,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE = 14,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST = 15,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED = 16,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL = 0x80000000,
    DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32 = 0xFFFFFFFF,
}}
ENUM!{enum DISPLAYCONFIG_SCANLINE_ORDERING {
    DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED = 0,
    DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE = 1,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED = 2,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST =
        DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED,
    DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST = 3,
    DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32 = 0xFFFFFFFF,
}}
STRUCT!{struct DISPLAYCONFIG_2DREGION {
    cx: UINT32,
    cy: UINT32,
}}
STRUCT!{struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_AdditionalSignalInfo {
    bitfield: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_VIDEO_SIGNAL_INFO_AdditionalSignalInfo bitfield: UINT32 [
    videoStandard set_videoStandard[0..16],
    vSyncFreqDivider set_vSyncFreqDivider[16..22],
]}
UNION!{union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_u {
    [u32; 1],
    AdditionalSignalInfo AdditionalSignalInfo_mut:
        DISPLAYCONFIG_VIDEO_SIGNAL_INFO_AdditionalSignalInfo,
    videoStandard videoStandard_mut: UINT32,
}}
STRUCT!{struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    pixelRate: UINT64,
    hSyncFreq: DISPLAYCONFIG_RATIONAL,
    vSyncFreq: DISPLAYCONFIG_RATIONAL,
    activeSize: DISPLAYCONFIG_2DREGION,
    totalSize: DISPLAYCONFIG_2DREGION,
    u: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_u,
    scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
}}
ENUM!{enum DISPLAYCONFIG_SCALING {
    DISPLAYCONFIG_SCALING_IDENTITY = 1,
    DISPLAYCONFIG_SCALING_CENTERED = 2,
    DISPLAYCONFIG_SCALING_STRETCHED = 3,
    DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX = 4,
    DISPLAYCONFIG_SCALING_CUSTOM = 5,
    DISPLAYCONFIG_SCALING_PREFERRED = 128,
    DISPLAYCONFIG_SCALING_FORCE_UINT32 = 0xFFFFFFFF,
}}
ENUM!{enum DISPLAYCONFIG_ROTATION {
    DISPLAYCONFIG_ROTATION_IDENTITY = 1,
    DISPLAYCONFIG_ROTATION_ROTATE90 = 2,
    DISPLAYCONFIG_ROTATION_ROTATE180 = 3,
    DISPLAYCONFIG_ROTATION_ROTATE270 = 4,
    DISPLAYCONFIG_ROTATION_FORCE_UINT32 = 0xFFFFFFFF,
}}
ENUM!{enum DISPLAYCONFIG_MODE_INFO_TYPE {
    DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE = 1,
    DISPLAYCONFIG_MODE_INFO_TYPE_TARGET = 2,
    DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE = 3,
    DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32 = 0xFFFFFFFF,
}}
ENUM!{enum DISPLAYCONFIG_PIXELFORMAT {
    DISPLAYCONFIG_PIXELFORMAT_8BPP = 1,
    DISPLAYCONFIG_PIXELFORMAT_16BPP = 2,
    DISPLAYCONFIG_PIXELFORMAT_24BPP = 3,
    DISPLAYCONFIG_PIXELFORMAT_32BPP = 4,
    DISPLAYCONFIG_PIXELFORMAT_NONGDI = 5,
    DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32 = 0xffffffff,
}}
STRUCT!{struct DISPLAYCONFIG_SOURCE_MODE {
    width: UINT32,
    height: UINT32,
    pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    position: POINTL,
}}
STRUCT!{struct DISPLAYCONFIG_TARGET_MODE {
    targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}}
STRUCT!{struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    PathSourceSize: POINTL,
    DesktopImageRegion: RECTL,
    DesktopImageClip: RECTL,
}}
UNION!{union DISPLAYCONFIG_MODE_INFO_u {
    [u64; 6],
    targetMode targetMode_mut: DISPLAYCONFIG_TARGET_MODE,
    sourceMode sourceMode_mut: DISPLAYCONFIG_SOURCE_MODE,
    desktopImageInfo desktopImageInfo_mut: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}}
STRUCT!{struct DISPLAYCONFIG_MODE_INFO {
    infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    id: UINT32,
    adapterId: LUID,
    u: DISPLAYCONFIG_MODE_INFO_u,
}}
pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: DWORD = 0xffffffff;
pub const DISPLAYCONFIG_PATH_TARGET_MODE_IDX_INVALID: DWORD = 0xffff;
pub const DISPLAYCONFIG_PATH_DESKTOP_IMAGE_IDX_INVALID: DWORD = 0xffff;
pub const DISPLAYCONFIG_PATH_SOURCE_MODE_IDX_INVALID: DWORD = 0xffff;
pub const DISPLAYCONFIG_PATH_CLONE_GROUP_INVALID: DWORD = 0xffff;
STRUCT!{struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    adapterId: LUID,
    id: UINT32,
    modeInfoIdx: UINT32,
    statusFlags: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_PATH_SOURCE_INFO modeInfoIdx: UINT32 [
    cloneGroupId set_cloneGroupId[0..16],
    sourceModeInfoIdx set_sourceModeInfoIdx[16..32],
]}
pub const DISPLAYCONFIG_SOURCE_IN_USE: DWORD = 0x00000001;
STRUCT!{struct DISPLAYCONFIG_PATH_TARGET_INFO {
    adapterId: LUID,
    id: UINT32,
    modeInfoIdx: UINT32,
    outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    rotation: DISPLAYCONFIG_ROTATION,
    scaling: DISPLAYCONFIG_SCALING,
    refreshRate: DISPLAYCONFIG_RATIONAL,
    scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    targetAvailable: BOOL,
    statusFlags: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_PATH_TARGET_INFO modeInfoIdx: UINT32 [
    desktopModeInfoIdx set_desktopModeInfoIdx[0..16],
    targetModeInfoIdx set_targetModeInfoIdx[16..32],
]}
pub const DISPLAYCONFIG_TARGET_IN_USE: DWORD = 0x00000001;
pub const DISPLAYCONFIG_TARGET_FORCIBLE: DWORD = 0x00000002;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: DWORD = 0x00000004;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: DWORD = 0x00000008;
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: DWORD = 0x00000010;
pub const DISPLAYCONFIG_TARGET_IS_HMD: DWORD = 0x00000020;
STRUCT!{struct DISPLAYCONFIG_PATH_INFO {
    sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    flags: UINT32,
}}
pub const DISPLAYCONFIG_PATH_ACTIVE: DWORD = 0x00000001;
pub const DISPLAYCONFIG_PATH_PREFERRED_UNSCALED: DWORD = 0x00000004;
pub const DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE: DWORD = 0x00000008;
pub const DISPLAYCONFIG_PATH_VALID_FLAGS: DWORD = 0x0000000D;
ENUM!{enum DISPLAYCONFIG_TOPOLOGY_ID {
    DISPLAYCONFIG_TOPOLOGY_INTERNAL = 0x00000001,
    DISPLAYCONFIG_TOPOLOGY_CLONE = 0x00000002,
    DISPLAYCONFIG_TOPOLOGY_EXTEND = 0x00000004,
    DISPLAYCONFIG_TOPOLOGY_EXTERNAL = 0x00000008,
    DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32 = 0xFFFFFFFF,
}}
ENUM!{enum DISPLAYCONFIG_DEVICE_INFO_TYPE {
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME = 1,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME = 2,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE = 3,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME = 4,
    DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE = 5,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE = 6,
    DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION = 7,
    DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION = 8,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO = 9,
    DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE = 10,
    DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32 = 0xFFFFFFFF,
}}
STRUCT!{struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    _type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    size: UINT32,
    adapterId: LUID,
    id: UINT32,
}}
STRUCT!{struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    viewGdiDeviceName: [WCHAR; CCHDEVICENAME],
}}
STRUCT!{struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    value: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS value: UINT32 [
    friendlyNameFromEdid set_friendlyNameFromEdid[0..1],
    friendlyNameForced set_friendlyNameForced[1..2],
    edidIdsValid set_edidIdsValid[2..3],
]}
STRUCT!{struct DISPLAYCONFIG_TARGET_DEVICE_NAME {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,
    outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    edidManufactureId: UINT16,
    edidProductCodeId: UINT16,
    connectorInstance: UINT32,
    monitorFriendlyDeviceName: [WCHAR; 64],
    monitorDevicePath: [WCHAR; 128],
}}
STRUCT!{struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    width: UINT32,
    height: UINT32,
    targetMode: DISPLAYCONFIG_TARGET_MODE,
}}
STRUCT!{struct DISPLAYCONFIG_ADAPTER_NAME {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    adapterDevicePath: [WCHAR; 128],
}}
STRUCT!{struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}}
STRUCT!{struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    value: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_SET_TARGET_PERSISTENCE value: UINT32 [
    bootPersistenceOn set_bootPersistenceOn[0..1],
]}
STRUCT!{struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    value: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION value: UINT32 [
    disableMonitorVirtualResolution set_disableMonitorVirtualResolution[0..1],
]}
ENUM!{enum DISPLAYCONFIG_COLOR_ENCODING {
    DISPLAYCONFIG_COLOR_ENCODING_RGB = 0,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR444 = 1,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR422 = 2,
    DISPLAYCONFIG_COLOR_ENCODING_YCBCR420 = 3,
    DISPLAYCONFIG_COLOR_ENCODING_INTENSITY = 4,
    DISPLAYCONFIG_COLOR_ENCODING_FORCE_UINT32 = 0xFFFFFFFF,
}}
STRUCT!{struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    value: UINT32,
    colorEncoding: DISPLAYCONFIG_COLOR_ENCODING,
    bitsPerColorChannel: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO value: UINT32 [
    advancedColorSupported set_advancedColorSupported[0..1],
    advancedColorEnabled set_advancedColorEnabled[1..2],
    reserved set_reserved[2..32],
]}
STRUCT!{struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    value: UINT32,
}}
BITFIELD!{DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE value: UINT32 [
    enableAdvancedColor set_enableAdvancedColor[0..1],
    reserved set_reserved[1..32],
]}
pub const QDC_ALL_PATHS: DWORD = 0x00000001;
pub const QDC_ONLY_ACTIVE_PATHS: DWORD = 0x00000002;
pub const QDC_DATABASE_CURRENT: DWORD = 0x00000004;
pub const QDC_VIRTUAL_MODE_AWARE: DWORD = 0x00000010;
pub const QDC_INCLUDE_HMD: DWORD = 0x00000020;
pub const SDC_TOPOLOGY_INTERNAL: DWORD = 0x00000001;
pub const SDC_TOPOLOGY_CLONE: DWORD = 0x00000002;
pub const SDC_TOPOLOGY_EXTEND: DWORD = 0x00000004;
pub const SDC_TOPOLOGY_EXTERNAL: DWORD = 0x00000008;
pub const SDC_TOPOLOGY_SUPPLIED: DWORD = 0x00000010;
pub const SDC_USE_DATABASE_CURRENT: DWORD = SDC_TOPOLOGY_INTERNAL | SDC_TOPOLOGY_CLONE
    | SDC_TOPOLOGY_EXTEND | SDC_TOPOLOGY_EXTERNAL;
pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: DWORD = 0x00000020;
pub const SDC_VALIDATE: DWORD = 0x00000040;
pub const SDC_APPLY: DWORD = 0x00000080;
pub const SDC_NO_OPTIMIZATION: DWORD = 0x00000100;
pub const SDC_SAVE_TO_DATABASE: DWORD = 0x00000200;
pub const SDC_ALLOW_CHANGES: DWORD = 0x00000400;
pub const SDC_PATH_PERSIST_IF_REQUIRED: DWORD = 0x00000800;
pub const SDC_FORCE_MODE_ENUMERATION: DWORD = 0x00001000;
pub const SDC_ALLOW_PATH_ORDER_CHANGES: DWORD = 0x00002000;
pub const SDC_VIRTUAL_MODE_AWARE: DWORD = 0x00008000;
pub const RDH_RECTANGLES: DWORD = 1;
STRUCT!{struct RGNDATAHEADER {
    dwSize: DWORD,
    iType: DWORD,
    nCount: DWORD,
    nRgnSize: DWORD,
    rcBound: RECT,
}}
pub type PRGNDATAHEADER = *mut RGNDATAHEADER;
STRUCT!{struct RGNDATA {
    rdh: RGNDATAHEADER,
    Buffer: [c_char; 1],
}}
pub type PRGNDATA = *mut RGNDATA;
pub type NPRGNDATA = *mut RGNDATA;
pub type LPRGNDATA = *mut RGNDATA;
pub const SYSRGN: INT = 4;
STRUCT!{struct ABC {
    abcA: c_int,
    abcB: UINT,
    abcC: c_int,
}}
pub type PABC = *mut ABC;
pub type NPABC = *mut ABC;
pub type LPABC = *mut ABC;
STRUCT!{struct ABCFLOAT {
    abcfA: FLOAT,
    abcfB: FLOAT,
    abcfC: FLOAT,
}}
pub type PABCFLOAT = *mut ABCFLOAT;
pub type NPABCFLOAT = *mut ABCFLOAT;
pub type LPABCFLOAT = *mut ABCFLOAT;
STRUCT!{struct OUTLINETEXTMETRICA {
    otmSize: UINT,
    otmTextMetrics: TEXTMETRICA,
    otmFiller: BYTE,
    otmPanoseNumber: PANOSE,
    otmfsSelection: UINT,
    otmfsType: UINT,
    otmsCharSlopeRise: c_int,
    otmsCharSlopeRun: c_int,
    otmItalicAngle: c_int,
    otmEMSquare: UINT,
    otmAscent: c_int,
    otmDescent: c_int,
    otmLineGap: UINT,
    otmsCapEmHeight: UINT,
    otmsXHeight: UINT,
    otmrcFontBox: RECT,
    otmMacAscent: c_int,
    otmMacDescent: c_int,
    otmMacLineGap: UINT,
    otmusMinimumPPEM: UINT,
    otmptSubscriptSize: POINT,
    otmptSubscriptOffset: POINT,
    otmptSuperscriptSize: POINT,
    otmptSuperscriptOffset: POINT,
    otmsStrikeoutSize: UINT,
    otmsStrikeoutPosition: c_int,
    otmsUnderscoreSize: c_int,
    otmsUnderscorePosition: c_int,
    otmpFamilyName: PSTR,
    otmpFaceName: PSTR,
    otmpStyleName: PSTR,
    otmpFullName: PSTR,
}}
pub type POUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
pub type NPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
pub type LPOUTLINETEXTMETRICA = *mut OUTLINETEXTMETRICA;
STRUCT!{struct OUTLINETEXTMETRICW {
    otmSize: UINT,
    otmTextMetrics: TEXTMETRICW,
    otmFiller: BYTE,
    otmPanoseNumber: PANOSE,
    otmfsSelection: UINT,
    otmfsType: UINT,
    otmsCharSlopeRise: c_int,
    otmsCharSlopeRun: c_int,
    otmItalicAngle: c_int,
    otmEMSquare: UINT,
    otmAscent: c_int,
    otmDescent: c_int,
    otmLineGap: UINT,
    otmsCapEmHeight: UINT,
    otmsXHeight: UINT,
    otmrcFontBox: RECT,
    otmMacAscent: c_int,
    otmMacDescent: c_int,
    otmMacLineGap: UINT,
    otmusMinimumPPEM: UINT,
    otmptSubscriptSize: POINT,
    otmptSubscriptOffset: POINT,
    otmptSuperscriptSize: POINT,
    otmptSuperscriptOffset: POINT,
    otmsStrikeoutSize: UINT,
    otmsStrikeoutPosition: c_int,
    otmsUnderscoreSize: c_int,
    otmsUnderscorePosition: c_int,
    otmpFamilyName: PSTR,
    otmpFaceName: PSTR,
    otmpStyleName: PSTR,
    otmpFullName: PSTR,
}}
pub type POUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type NPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
pub type LPOUTLINETEXTMETRICW = *mut OUTLINETEXTMETRICW;
STRUCT!{struct POLYTEXTA {
    x: c_int,
    y: c_int,
    n: UINT,
    lpstr: LPCSTR,
    uiFlags: UINT,
    rcl: RECT,
    pdx: *mut c_int,
}}
pub type PPOLYTEXTA = *mut POLYTEXTA;
pub type NPPOLYTEXTA = *mut POLYTEXTA;
pub type LPPOLYTEXTA = *mut POLYTEXTA;
STRUCT!{struct POLYTEXTW {
    x: c_int,
    y: c_int,
    n: UINT,
    lpstr: LPCWSTR,
    uiFlags: UINT,
    rcl: RECT,
    pdx: *mut c_int,
}}
pub type PPOLYTEXTW = *mut POLYTEXTW;
pub type NPPOLYTEXTW = *mut POLYTEXTW;
pub type LPPOLYTEXTW = *mut POLYTEXTW;
STRUCT!{struct FIXED {
    fract: WORD,
    value: c_short,
}}
STRUCT!{struct MAT2 {
    eM11: FIXED,
    eM12: FIXED,
    eM21: FIXED,
    eM22: FIXED,
}}
pub type LPMAT2 = *mut MAT2;
STRUCT!{struct GLYPHMETRICS {
    gmBlackBoxX: UINT,
    gmBlackBoxY: UINT,
    gmptGlyphOrigin: POINT,
    gmCellIncX: c_short,
    gmCellIncY: c_short,
}}
pub type LPGLYPHMETRICS = *mut GLYPHMETRICS;
pub const GGO_METRICS: DWORD = 0;
pub const GGO_BITMAP: DWORD = 1;
pub const GGO_NATIVE: DWORD = 2;
pub const GGO_BEZIER: DWORD = 3;
pub const GGO_GRAY2_BITMAP: DWORD = 4;
pub const GGO_GRAY4_BITMAP: DWORD = 5;
pub const GGO_GRAY8_BITMAP: DWORD = 6;
pub const GGO_GLYPH_INDEX: DWORD = 0x0080;
pub const GGO_UNHINTED: DWORD = 0x0100;
pub const TT_POLYGON_TYPE: DWORD = 24;
pub const TT_PRIM_LINE: DWORD = 1;
pub const TT_PRIM_QSPLINE: DWORD = 2;
pub const TT_PRIM_CSPLINE: DWORD = 3;
STRUCT!{struct POINTFX {
    x: FIXED,
    y: FIXED,
}}
pub type LPPOINTFX = *mut POINTFX;
STRUCT!{struct TTPOLYCURVE {
    wType: WORD,
    cpfx: WORD,
    apfx: [POINTFX; 1],
}}
pub type LPTTPOLYCURVE = *mut TTPOLYCURVE;
STRUCT!{struct TTPOLYGONHEADER {
    cb: DWORD,
    dwType: DWORD,
    pfxStart: POINTFX,
}}
pub type LPTTPOLYGONHEADER = *mut TTPOLYGONHEADER;
pub const GCP_DBCS: DWORD = 0x0001;
pub const GCP_REORDER: DWORD = 0x0002;
pub const GCP_USEKERNING: DWORD = 0x0008;
pub const GCP_GLYPHSHAPE: DWORD = 0x0010;
pub const GCP_LIGATE: DWORD = 0x0020;
pub const GCP_DIACRITIC: DWORD = 0x0100;
pub const GCP_KASHIDA: DWORD = 0x0400;
pub const GCP_ERROR: DWORD = 0x8000;
pub const FLI_MASK: DWORD = 0x103B;
pub const GCP_JUSTIFY: DWORD = 0x00010000;
pub const FLI_GLYPHS: DWORD = 0x00040000;
pub const GCP_CLASSIN: DWORD = 0x00080000;
pub const GCP_MAXEXTENT: DWORD = 0x00100000;
pub const GCP_JUSTIFYIN: DWORD = 0x00200000;
pub const GCP_DISPLAYZWG: DWORD = 0x00400000;
pub const GCP_SYMSWAPOFF: DWORD = 0x00800000;
pub const GCP_NUMERICOVERRIDE: DWORD = 0x01000000;
pub const GCP_NEUTRALOVERRIDE: DWORD = 0x02000000;
pub const GCP_NUMERICSLATIN: DWORD = 0x04000000;
pub const GCP_NUMERICSLOCAL: DWORD = 0x08000000;
pub const GCPCLASS_LATIN: DWORD = 1;
pub const GCPCLASS_HEBREW: DWORD = 2;
pub const GCPCLASS_ARABIC: DWORD = 2;
pub const GCPCLASS_NEUTRAL: DWORD = 3;
pub const GCPCLASS_LOCALNUMBER: DWORD = 4;
pub const GCPCLASS_LATINNUMBER: DWORD = 5;
pub const GCPCLASS_LATINNUMERICTERMINATOR: DWORD = 6;
pub const GCPCLASS_LATINNUMERICSEPARATOR: DWORD = 7;
pub const GCPCLASS_NUMERICSEPARATOR: DWORD = 8;
pub const GCPCLASS_PREBOUNDLTR: DWORD = 0x80;
pub const GCPCLASS_PREBOUNDRTL: DWORD = 0x40;
pub const GCPCLASS_POSTBOUNDLTR: DWORD = 0x20;
pub const GCPCLASS_POSTBOUNDRTL: DWORD = 0x10;
pub const GCPGLYPH_LINKBEFORE: DWORD = 0x8000;
pub const GCPGLYPH_LINKAFTER: DWORD = 0x4000;
STRUCT!{struct GCP_RESULTSA {
    lStructSize: DWORD,
    lpOutString: LPSTR,
    lpOrder: *mut UINT,
    lpDx: *mut c_int,
    lpCaretPos: *mut c_int,
    lpClass: LPSTR,
    lpGlyphs: LPWSTR,
    nGlyphs: UINT,
    nMaxFit: c_int,
}}
pub type LPGCP_RESULTSA = *mut GCP_RESULTSA;
STRUCT!{struct GCP_RESULTSW {
    lStructSize: DWORD,
    lpOutString: LPWSTR,
    lpOrder: *mut UINT,
    lpDx: *mut c_int,
    lpCaretPos: *mut c_int,
    lpClass: LPSTR,
    lpGlyphs: LPWSTR,
    nGlyphs: UINT,
    nMaxFit: c_int,
}}
pub type LPGCP_RESULTSW = *mut GCP_RESULTSW;
STRUCT!{struct RASTERIZER_STATUS {
    nSize: c_short,
    wFlags: c_short,
    nLanguageID: c_short,
}}
pub type LPRASTERIZER_STATUS = *mut RASTERIZER_STATUS;
pub const TT_AVAILABLE: DWORD = 0x0001;
pub const TT_ENABLED: DWORD = 0x0002;
STRUCT!{struct PIXELFORMATDESCRIPTOR {
    nSize: WORD,
    nVersion: WORD,
    dwFlags: DWORD,
    iPixelType: BYTE,
    cColorBits: BYTE,
    cRedBits: BYTE,
    cRedShift: BYTE,
    cGreenBits: BYTE,
    cGreenShift: BYTE,
    cBlueBits: BYTE,
    cBlueShift: BYTE,
    cAlphaBits: BYTE,
    cAlphaShift: BYTE,
    cAccumBits: BYTE,
    cAccumRedBits: BYTE,
    cAccumGreenBits: BYTE,
    cAccumBlueBits: BYTE,
    cAccumAlphaBits: BYTE,
    cDepthBits: BYTE,
    cStencilBits: BYTE,
    cAuxBuffers: BYTE,
    iLayerType: BYTE,
    bReserved: BYTE,
    dwLayerMask: DWORD,
    dwVisibleMask: DWORD,
    dwDamageMask: DWORD,
}}
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub const PFD_TYPE_RGBA: BYTE = 0;
pub const PFD_TYPE_COLORINDEX: BYTE = 1;
pub const PFD_MAIN_PLANE: BYTE = 0;
pub const PFD_OVERLAY_PLANE: BYTE = 1;
pub const PFD_UNDERLAY_PLANE: BYTE = -1i8 as u8;
pub const PFD_DOUBLEBUFFER: DWORD = 0x00000001;
pub const PFD_STEREO: DWORD = 0x00000002;
pub const PFD_DRAW_TO_WINDOW: DWORD = 0x00000004;
pub const PFD_DRAW_TO_BITMAP: DWORD = 0x00000008;
pub const PFD_SUPPORT_GDI: DWORD = 0x00000010;
pub const PFD_SUPPORT_OPENGL: DWORD = 0x00000020;
pub const PFD_GENERIC_FORMAT: DWORD = 0x00000040;
pub const PFD_NEED_PALETTE: DWORD = 0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE: DWORD = 0x00000100;
pub const PFD_SWAP_EXCHANGE: DWORD = 0x00000200;
pub const PFD_SWAP_COPY: DWORD = 0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS: DWORD = 0x00000800;
pub const PFD_GENERIC_ACCELERATED: DWORD = 0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW: DWORD = 0x00002000;
pub const PFD_DIRECT3D_ACCELERATED: DWORD = 0x00004000;
pub const PFD_SUPPORT_COMPOSITION: DWORD = 0x00008000;
pub const PFD_DEPTH_DONTCARE: DWORD = 0x20000000;
pub const PFD_DOUBLEBUFFER_DONTCARE: DWORD = 0x40000000;
pub const PFD_STEREO_DONTCARE: DWORD = 0x80000000;
FN!{stdcall OLDFONTENUMPROCA(
    *const LOGFONTA,
    *const TEXTMETRICA,
    DWORD,
    LPARAM,
) -> c_int}
FN!{stdcall OLDFONTENUMPROCW(
    *const LOGFONTW,
    *const TEXTMETRICW,
    DWORD,
    LPARAM,
) -> c_int}
pub type FONTENUMPROCA = OLDFONTENUMPROCA;
pub type FONTENUMPROCW = OLDFONTENUMPROCW;
FN!{stdcall GOBJENUMPROC(
    LPVOID,
    LPARAM,
) -> c_int}
FN!{stdcall LINEDDAPROC(
    c_int,
    c_int,
    LPARAM,
) -> ()}
extern "system" {
    pub fn AddFontResourceA(
        _: LPCSTR,
    ) -> c_int;
    pub fn AddFontResourceW(
        _: LPCWSTR,
    ) -> c_int;
    pub fn AnimatePalette(
        hPal: HPALETTE,
        iStartIndex: UINT,
        cEntries: UINT,
        ppe: *const PALETTEENTRY,
    ) -> BOOL;
    pub fn Arc(
        hdc: HDC,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
        x4: c_int,
        y4: c_int,
    ) -> BOOL;
    pub fn BitBlt(
        hdc: HDC,
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        hdcSrc: HDC,
        x1: c_int,
        y1: c_int,
        rop: DWORD,
    ) -> BOOL;
    pub fn CancelDC(
        hdc: HDC,
    ) -> BOOL;
    pub fn Chord(
        hdc: HDC,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
        x4: c_int,
        y4: c_int,
    ) -> BOOL;
    pub fn ChoosePixelFormat(
        hdc: HDC,
        ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> c_int;
    pub fn CloseMetaFile(
        hdc: HDC,
    ) -> HMETAFILE;
    pub fn CombineRgn(
        hrgnDst: HRGN,
        hrgnSrc1: HRGN,
        hrgnSrc2: HRGN,
        iMode: c_int,
    ) -> c_int;
    pub fn CopyMetaFileA(
        _: HMETAFILE,
        _: LPCSTR,
    ) -> HMETAFILE;
    pub fn CopyMetaFileW(
        _: HMETAFILE,
        _: LPCWSTR,
    ) -> HMETAFILE;
    pub fn CreateBitmap(
        nWidth: c_int,
        nHeight: c_int,
        nPlanes: UINT,
        nBitCount: UINT,
        lpBits: *const c_void,
    ) -> HBITMAP;
    pub fn CreateBitmapIndirect(
        pbm: *const BITMAP,
    ) -> HBITMAP;
    pub fn CreateBrushIndirect(
        plbrush: *const LOGBRUSH,
    ) -> HBRUSH;
    pub fn CreateCompatibleBitmap(
        hdc: HDC,
        cx: c_int,
        cy: c_int,
    ) -> HBITMAP;
    pub fn CreateDiscardableBitmap(
        hdc: HDC,
        cx: c_int,
        cy: c_int,
    ) -> HBITMAP;
    pub fn CreateCompatibleDC(
        hdc: HDC,
    ) -> HDC;
    pub fn CreateDCA(
        pwszDriver: LPCSTR,
        pwszDevice: LPCSTR,
        pszPort: LPCSTR,
        pdm: *const DEVMODEA,
    ) -> HDC;
    pub fn CreateDCW(
        pwszDriver: LPCWSTR,
        pwszDevice: LPCWSTR,
        pszPort: LPCWSTR,
        pdm: *const DEVMODEW,
    ) -> HDC;
    pub fn CreateDIBitmap(
        hdc: HDC,
        pbmih: *const BITMAPINFOHEADER,
        flInit: DWORD,
        pjBits: *const c_void,
        pbmi: *const BITMAPINFO,
        iUsage: UINT,
    ) -> HBITMAP;
    pub fn CreateDIBPatternBrush(
        h: HGLOBAL,
        iUsage: UINT,
    ) -> HBRUSH;
    pub fn CreateDIBPatternBrushPt(
        lpPackedDIB: *const c_void,
        iUsage: UINT,
    ) -> HBRUSH;
    pub fn CreateEllipticRgn(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
    ) -> HRGN;
    pub fn CreateEllipticRgnIndirect(
        lprect: *const RECT,
    ) -> HRGN;
    pub fn CreateFontIndirectA(
        lplf: *const LOGFONTA,
    ) -> HFONT;
    pub fn CreateFontIndirectW(
        lplf: *const LOGFONTW,
    ) -> HFONT;
    pub fn CreateFontA(
        cHeight: c_int,
        cWidth: c_int,
        cEscapement: c_int,
        cOrientation: c_int,
        cWeight: c_int,
        bItalic: DWORD,
        bUnderline: DWORD,
        bStrikeOut: DWORD,
        iCharSet: DWORD,
        iOutPrecision: DWORD,
        iClipPrecision: DWORD,
        iQuality: DWORD,
        iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> HFONT;
    pub fn CreateFontW(
        cHeight: c_int,
        cWidth: c_int,
        cEscapement: c_int,
        cOrientation: c_int,
        cWeight: c_int,
        bItalic: DWORD,
        bUnderline: DWORD,
        bStrikeOut: DWORD,
        iCharSet: DWORD,
        iOutPrecision: DWORD,
        iClipPrecision: DWORD,
        iQuality: DWORD,
        iPitchAndFamily: DWORD,
        pszFaceName: LPCWSTR,
    ) -> HFONT;
    pub fn CreateHatchBrush(
        iHatch: c_int,
        color: COLORREF,
    ) -> HBRUSH;
    pub fn CreateICA(
        pszDriver: LPCSTR,
        pszDevice: LPCSTR,
        pszPort: LPCSTR,
        pdm: *const DEVMODEA,
    ) -> HDC;
    pub fn CreateICW(
        pszDriver: LPCWSTR,
        pszDevice: LPCWSTR,
        pszPort: LPCWSTR,
        pdm: *const DEVMODEW,
    ) -> HDC;
    pub fn CreateMetaFileA(
        pszFile: LPCSTR,
    ) -> HDC;
    pub fn CreateMetaFileW(
        pszFile: LPCWSTR,
    ) -> HDC;
    pub fn CreatePalette(
        plpal: *const LOGPALETTE,
    ) -> HPALETTE;
    pub fn CreatePen(
        iStyle: c_int,
        cWidth: c_int,
        color: COLORREF,
    ) -> HPEN;
    pub fn CreatePenIndirect(
        plpen: *const LOGPEN,
    ) -> HPEN;
    pub fn CreatePolyPolygonRgn(
        pptl: *const POINT,
        pc: *const INT,
        cPoly: c_int,
        iMode: c_int,
    ) -> HRGN;
    pub fn CreatePatternBrush(
        hbm: HBITMAP,
    ) -> HBRUSH;
    pub fn CreateRectRgn(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
    ) -> HRGN;
    pub fn CreateRectRgnIndirect(
        lprect: *const RECT,
    ) -> HRGN;
    pub fn CreateRoundRectRgn(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        w: c_int,
        h: c_int,
    ) -> HRGN;
    pub fn CreateScalableFontResourceA(
        fdwHidden: DWORD,
        lpszFont: LPCSTR,
        lpszFile: LPCSTR,
        lpszPath: LPCSTR,
    ) -> BOOL;
    pub fn CreateScalableFontResourceW(
        fdwHidden: DWORD,
        lpszFont: LPCWSTR,
        lpszFile: LPCWSTR,
        lpszPath: LPCWSTR,
    ) -> BOOL;
    pub fn CreateSolidBrush(
        color: COLORREF,
    ) -> HBRUSH;
    pub fn DeleteDC(
        hdc: HDC,
    ) -> BOOL;
    pub fn DeleteMetaFile(
        hmf: HMETAFILE,
    ) -> BOOL;
    pub fn DeleteObject(
        ho: HGDIOBJ,
    ) -> BOOL;
    pub fn DescribePixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        nBytes: UINT,
        ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;
}
FN!{stdcall LPFNDEVMODE(
    HWND,
    HMODULE,
    LPDEVMODEA,
    LPSTR,
    LPSTR,
    LPDEVMODEA,
    LPSTR,
    UINT,
) -> UINT}
FN!{stdcall LPFNDEVCAPS(
    LPSTR,
    LPSTR,
    UINT,
    LPSTR,
    LPDEVMODEA,
) -> DWORD}
pub const DM_UPDATE: DWORD = 1;
pub const DM_COPY: DWORD = 2;
pub const DM_PROMPT: DWORD = 4;
pub const DM_MODIFY: DWORD = 8;
pub const DM_IN_BUFFER: DWORD = DM_MODIFY;
pub const DM_IN_PROMPT: DWORD = DM_PROMPT;
pub const DM_OUT_BUFFER: DWORD = DM_COPY;
pub const DM_OUT_DEFAULT: DWORD = DM_UPDATE;
pub const DC_FIELDS: WORD = 1;
pub const DC_PAPERS: WORD = 2;
pub const DC_PAPERSIZE: WORD = 3;
pub const DC_MINEXTENT: WORD = 4;
pub const DC_MAXEXTENT: WORD = 5;
pub const DC_BINS: WORD = 6;
pub const DC_DUPLEX: WORD = 7;
pub const DC_SIZE: WORD = 8;
pub const DC_EXTRA: WORD = 9;
pub const DC_VERSION: WORD = 10;
pub const DC_DRIVER: WORD = 11;
pub const DC_BINNAMES: WORD = 12;
pub const DC_ENUMRESOLUTIONS: WORD = 13;
pub const DC_FILEDEPENDENCIES: WORD = 14;
pub const DC_TRUETYPE: WORD = 15;
pub const DC_PAPERNAMES: WORD = 16;
pub const DC_ORIENTATION: WORD = 17;
pub const DC_COPIES: WORD = 18;
pub const DC_BINADJUST: WORD = 19;
pub const DC_EMF_COMPLIANT: WORD = 20;
pub const DC_DATATYPE_PRODUCED: WORD = 21;
pub const DC_COLLATE: WORD = 22;
pub const DC_MANUFACTURER: WORD = 23;
pub const DC_MODEL: WORD = 24;
pub const DC_PERSONALITY: WORD = 25;
pub const DC_PRINTRATE: WORD = 26;
pub const DC_PRINTRATEUNIT: WORD = 27;
pub const PRINTRATEUNIT_PPM: WORD = 1;
pub const PRINTRATEUNIT_CPS: WORD = 2;
pub const PRINTRATEUNIT_LPM: WORD = 3;
pub const PRINTRATEUNIT_IPM: WORD = 4;
pub const DC_PRINTERMEM: WORD = 28;
pub const DC_MEDIAREADY: WORD = 29;
pub const DC_STAPLE: WORD = 30;
pub const DC_PRINTRATEPPM: WORD = 31;
pub const DC_COLORDEVICE: WORD = 32;
pub const DC_NUP: WORD = 33;
pub const DC_MEDIATYPENAMES: WORD = 34;
pub const DC_MEDIATYPES: WORD = 35;
pub const DCTT_BITMAP: DWORD = 0x0000001;
pub const DCTT_DOWNLOAD: DWORD = 0x0000002;
pub const DCTT_SUBDEV: DWORD = 0x0000004;
pub const DCTT_DOWNLOAD_OUTLINE: DWORD = 0x0000008;
pub const DCBA_FACEUPNONE: DWORD = 0x0000;
pub const DCBA_FACEUPCENTER: DWORD = 0x0001;
pub const DCBA_FACEUPLEFT: DWORD = 0x0002;
pub const DCBA_FACEUPRIGHT: DWORD = 0x0003;
pub const DCBA_FACEDOWNNONE: DWORD = 0x0100;
pub const DCBA_FACEDOWNCENTER: DWORD = 0x0101;
pub const DCBA_FACEDOWNLEFT: DWORD = 0x0102;
pub const DCBA_FACEDOWNRIGHT: DWORD = 0x0103;
extern "system" {
    pub fn DeviceCapabilitiesA(
        pDevice: LPCSTR,
        pPort: LPCSTR,
        fwCapability: WORD,
        pOutput: LPSTR,
        pDevMode: *const DEVMODEA,
    ) -> c_int;
    pub fn DeviceCapabilitiesW(
        pDevice: LPCWSTR,
        pPort: LPCWSTR,
        fwCapability: WORD,
        pOutput: LPWSTR,
        pDevMode: *const DEVMODEW,
    ) -> c_int;
    pub fn DrawEscape(
        hdc: HDC,
        iEscape: c_int,
        cjIn: c_int,
        lpIn: LPCSTR,
    ) -> c_int;
    pub fn Ellipse(
        hdc: HDC,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> BOOL;
    pub fn EnumFontFamiliesExA(
        hdc: HDC,
        lpLogfont: LPLOGFONTA,
        lpProc: FONTENUMPROCA,
        lParam: LPARAM,
        dwFlags: DWORD,
    ) -> c_int;
    pub fn EnumFontFamiliesExW(
        hdc: HDC,
        lpLogfont: LPLOGFONTW,
        lpProc: FONTENUMPROCW,
        lParam: LPARAM,
        dwFlags: DWORD,
    ) -> c_int;
    pub fn EnumFontFamiliesA(
        hdc: HDC,
        lpLogfont: LPCSTR,
        lpProc: FONTENUMPROCA,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumFontFamiliesW(
        hdc: HDC,
        lpLogfont: LPCWSTR,
        lpProc: FONTENUMPROCW,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumFontsA(
        hdc: HDC,
        lpLogfont: LPCSTR,
        lpProc: FONTENUMPROCA,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumFontsW(
        hdc: HDC,
        lpLogfont: LPCWSTR,
        lpProc: FONTENUMPROCW,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumObjects(
        hdc: HDC,
        nType: c_int,
        lpFunc: GOBJENUMPROC,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EqualRgn(
        hrgn1: HRGN,
        hrgn2: HRGN,
    ) -> BOOL;
    pub fn Escape(
        hdc: HDC,
        iEscape: c_int,
        cjIn: c_int,
        pvIn: LPCSTR,
        pvOut: LPVOID,
    ) -> c_int;
    pub fn ExtEscape(
        hdc: HDC,
        iEscape: c_int,
        cjInput: c_int,
        lpInData: LPCSTR,
        cjOutput: c_int,
        lpOutData: LPSTR,
    ) -> c_int;
    pub fn ExcludeClipRect(
        hdc: HDC,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> c_int;
    pub fn ExtCreateRegion(
        lpx: *const XFORM,
        nCount: DWORD,
        lpData: *const RGNDATA,
    ) -> HRGN;
    pub fn ExtFloodFill(
        hdc: HDC,
        x: c_int,
        y: c_int,
        color: COLORREF,
        utype: UINT,
    ) -> BOOL;
    pub fn FillRgn(
        hdc: HDC,
        hrgn: HRGN,
        hbr: HBRUSH,
    ) -> BOOL;
    pub fn FloodFill(
        hdc: HDC,
        x: c_int,
        y: c_int,
        color: COLORREF,
    ) -> BOOL;
    pub fn FrameRgn(
        hdc: HDC,
        hrgn: HRGN,
        hbr: HBRUSH,
        w: c_int,
        h: c_int,
    ) -> BOOL;
    pub fn GetROP2(
        hdc: HDC,
    ) -> c_int;
    pub fn GetAspectRatioFilterEx(
        hdc: HDC,
        lpsize: LPSIZE,
    ) -> BOOL;
    pub fn GetBkColor(
        hdc: HDC,
    ) -> COLORREF;
    pub fn GetDCBrushColor(
        hdc: HDC,
    ) -> COLORREF;
    pub fn GetDCPenColor(
        hdc: HDC,
    ) -> COLORREF;
    pub fn GetBkMode(
        hdc: HDC,
    ) -> c_int;
    pub fn GetBitmapBits(
        hbit: HBITMAP,
        cb: LONG,
        lpvBits: LPVOID,
    ) -> LONG;
    pub fn GetBitmapDimensionEx(
        hbit: HBITMAP,
        lpsize: LPSIZE,
    ) -> BOOL;
    pub fn GetBoundsRect(
        hdc: HDC,
        lprect: LPRECT,
        flags: UINT,
    ) -> UINT;
    pub fn GetBrushOrgEx(
        hdc: HDC,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn GetCharWidthA(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: LPINT,
    ) -> BOOL;
    pub fn GetCharWidthW(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: LPINT,
    ) -> BOOL;
    pub fn GetCharWidth32A(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: LPINT,
    ) -> BOOL;
    pub fn GetCharWidth32W(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: LPINT,
    ) -> BOOL;
    pub fn GetCharWidthFloatA(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: PFLOAT,
    ) -> BOOL;
    pub fn GetCharWidthFloatW(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpBuffer: PFLOAT,
    ) -> BOOL;
    pub fn GetCharABCWidthsA(
        hdc: HDC,
        wFirst: UINT,
        wLast: UINT,
        lpABC: LPABC,
    ) -> BOOL;
    pub fn GetCharABCWidthsW(
        hdc: HDC,
        wFirst: UINT,
        wLast: UINT,
        lpABC: LPABC,
    ) -> BOOL;
    pub fn GetCharABCWidthsFloatA(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpABC: LPABCFLOAT,
    ) -> BOOL;
    pub fn GetCharABCWidthsFloatW(
        hdc: HDC,
        iFirst: UINT,
        iLast: UINT,
        lpABC: LPABCFLOAT,
    ) -> BOOL;
    pub fn GetClipBox(
        hdc: HDC,
        lprect: LPRECT,
    ) -> c_int;
    pub fn GetClipRgn(
        hdc: HDC,
        hrgn: HRGN,
    ) -> c_int;
    pub fn GetMetaRgn(
        hdc: HDC,
        hrgn: HRGN,
    ) -> c_int;
    pub fn GetCurrentObject(
        hdc: HDC,
        tp: UINT,
    ) -> HGDIOBJ;
    pub fn GetCurrentPositionEx(
        hdc: HDC,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn GetDeviceCaps(
        hdc: HDC,
        nIndex: c_int,
    ) -> c_int;
    pub fn GetDIBits(
        hdc: HDC,
        hbm: HBITMAP,
        start: UINT,
        cLines: UINT,
        lpvBits: LPVOID,
        lpbmi: LPBITMAPINFO,
        usage: UINT,
    ) -> c_int;
    pub fn GetFontData(
        hdc: HDC,
        dwTable: DWORD,
        dwOffset: DWORD,
        pvBuffer: PVOID,
        cjBuffer: DWORD,
    ) -> DWORD;
    pub fn GetGlyphOutlineA(
        hdc: HDC,
        uChar: UINT,
        fuFormat: UINT,
        lpgm: LPGLYPHMETRICS,
        cjBuffer: DWORD,
        pvBuffer: LPVOID,
        lpmat2: *const MAT2,
    ) -> DWORD;
    pub fn GetGlyphOutlineW(
        hdc: HDC,
        uChar: UINT,
        fuFormat: UINT,
        lpgm: LPGLYPHMETRICS,
        cjBuffer: DWORD,
        pvBuffer: LPVOID,
        lpmat2: *const MAT2,
    ) -> DWORD;
    pub fn GetGraphicsMode(
        hdc: HDC,
    ) -> c_int;
    pub fn GetMapMode(
        hdc: HDC,
    ) -> c_int;
    pub fn GetMetaFileBitsEx(
        hMF: HMETAFILE,
        cbBuffer: UINT,
        lpData: LPVOID,
    ) -> UINT;
    pub fn GetMetaFileA(
        lpName: LPCSTR,
    ) -> HMETAFILE;
    pub fn GetMetaFileW(
        lpName: LPCWSTR,
    ) -> HMETAFILE;
    pub fn GetNearestColor(
        hdc: HDC,
        color: COLORREF,
    ) -> COLORREF;
    pub fn GetNearestPaletteIndex(
        h: HPALETTE,
        color: COLORREF,
    ) -> UINT;
    pub fn GetObjectType(
        h: HGDIOBJ,
    ) -> DWORD;
    pub fn GetOutlineTextMetricsA(
        hdc: HDC,
        cjCopy: UINT,
        potm: LPOUTLINETEXTMETRICA,
    ) -> UINT;
    pub fn GetOutlineTextMetricsW(
        hdc: HDC,
        cjCopy: UINT,
        potm: LPOUTLINETEXTMETRICW,
    ) -> UINT;
    pub fn GetPaletteEntries(
        hpal: HPALETTE,
        iStart: UINT,
        cEntries: UINT,
        pPalEntries: LPPALETTEENTRY,
    ) -> UINT;
    pub fn GetPixel(
        hdc: HDC,
        x: c_int,
        y: c_int,
    ) -> COLORREF;
    pub fn GetPixelFormat(
        hdc: HDC,
    ) -> c_int;
    pub fn GetPolyFillMode(
        hdc: HDC,
    ) -> c_int;
    pub fn GetRasterizerCaps(
        lpraststat: LPRASTERIZER_STATUS,
        cjBytes: UINT,
    ) -> BOOL;
    pub fn GetRandomRgn (
        hdc: HDC,
        hrgn: HRGN,
        i: INT,
    ) -> c_int;
    pub fn GetRegionData(
        hrgn: HRGN,
        nCount: DWORD,
        lpRgnData: LPRGNDATA,
    ) -> DWORD;
    pub fn GetRgnBox(
        hrgn: HRGN,
        lprc: LPRECT,
    ) -> c_int;
    pub fn GetStockObject(
        i: c_int,
    ) -> HGDIOBJ;
    pub fn GetStretchBltMode(
        hdc: HDC,
    ) -> c_int;
    pub fn GetSystemPaletteEntries(
        hdc: HDC,
        iStart: UINT,
        cEntries: UINT,
        pPalEntries: LPPALETTEENTRY,
    ) -> UINT;
    pub fn GetSystemPaletteUse(
        hdc: HDC,
    ) -> UINT;
    pub fn GetTextCharacterExtra(
        hdc: HDC,
    ) -> c_int;
    pub fn GetTextAlign(
        hdc: HDC,
    ) -> UINT;
    pub fn GetTextColor(
        hdc: HDC,
    ) -> COLORREF;
    pub fn GetTextExtentPointA(
        hdc: HDC,
        lpString: LPCSTR,
        c: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentPointW(
        hdc: HDC,
        lpString: LPCWSTR,
        c: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentPoint32A(
        hdc: HDC,
        lpString: LPCSTR,
        c: c_int,
        psizl: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentPoint32W(
        hdc: HDC,
        lpString: LPCWSTR,
        c: c_int,
        psizl: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentExPointA(
        hdc: HDC,
        lpszString: LPCSTR,
        cchString: c_int,
        nMaxExtent: c_int,
        lpnFit: LPINT,
        lpnDx: LPINT,
        lpSize: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentExPointW(
        hdc: HDC,
        lpszString: LPCWSTR,
        cchString: c_int,
        nMaxExtent: c_int,
        lpnFit: LPINT,
        lpnDx: LPINT,
        lpSize: LPSIZE,
    ) -> BOOL;
    pub fn GetTextCharset(
        hdc: HDC,
    ) -> c_int;
    pub fn GetTextCharsetInfo(
        hdc: HDC,
        lpSig: LPFONTSIGNATURE,
        dwFlags: DWORD,
    ) -> c_int;
    pub fn TranslateCharsetInfo(
        lpSrc: *const DWORD,
        lpCs: LPCHARSETINFO,
        dwFlags: DWORD,
    ) -> BOOL;
    pub fn GetFontLanguageInfo(
        hdc: HDC,
    ) -> DWORD;
    pub fn GetCharacterPlacementA(
        hdc: HDC,
        lpString: LPCSTR,
        nCount: c_int,
        nMexExtent: c_int,
        lpResults: LPGCP_RESULTSA,
        dwFlags: DWORD,
    ) -> DWORD;
    pub fn GetCharacterPlacementW(
        hdc: HDC,
        lpString: LPCWSTR,
        nCount: c_int,
        nMexExtent: c_int,
        lpResults: LPGCP_RESULTSW,
        dwFlags: DWORD,
    ) -> DWORD;
}
STRUCT!{struct WCRANGE {
    wcLow: WCHAR,
    cGlyphs: USHORT,
}}
pub type PWCRANGE = *mut WCRANGE;
pub type LPWCRANGE = *mut WCRANGE;
STRUCT!{struct GLYPHSET {
    cbThis: DWORD,
    flAccel: DWORD,
    cGlyphsSupported: DWORD,
    cRanges: DWORD,
    ranges: [WCRANGE;1],
}}
pub type PGLYPHSET = *mut GLYPHSET;
pub type LPGLYPHSET = *mut GLYPHSET;
pub const GS_8BIT_INDICES: DWORD = 0x00000001;
pub const GGI_MARK_NONEXISTING_GLYPHS: DWORD = 0x0001;
extern "system" {
    pub fn GetFontUnicodeRanges(
        hdc: HDC,
        lpgs: LPGLYPHSET,
    ) -> DWORD;
    pub fn GetGlyphIndicesA(
        hdc: HDC,
        lpstr: LPCSTR,
        c: c_int,
        pgi: LPWORD,
        fl: DWORD,
    ) -> DWORD;
    pub fn GetGlyphIndicesW(
        hdc: HDC,
        lpstr: LPCWSTR,
        c: c_int,
        pgi: LPWORD,
        fl: DWORD,
    ) -> DWORD;
    pub fn GetTextExtentPointI(
        hdc: HDC,
        pgiIn: LPWORD,
        cgi: c_int,
        psize: LPSIZE,
    ) -> BOOL;
    pub fn GetTextExtentExPointI(
        hdc: HDC,
        lpwszString: LPWORD,
        cwchString: c_int,
        nMaxExtent: c_int,
        lpnFit: LPINT,
        lpnDx: LPINT,
        lpSize: LPSIZE,
    ) -> BOOL;
    pub fn GetCharWidthI(
        hdc: HDC,
        giFirst: UINT,
        cgi: UINT,
        pgi: LPWORD,
        piWidths: LPINT,
    ) -> BOOL;
    pub fn GetCharABCWidthsI(
        hdc: HDC,
        giFirst: UINT,
        cgi: UINT,
        pgi: LPWORD,
        pabc: LPABC,
    ) -> BOOL;
}
pub const STAMP_DESIGNVECTOR: DWORD = 0x8000000 + 0x64 + (0x76 << 8);
pub const STAMP_AXESLIST: DWORD = 0x8000000 + 0x61 + (0x6c << 8);
pub const STAMP_TRUETYPE_VARIATION: DWORD = 0x8000000 + 0x74 + (0x76 << 8);
pub const MM_MAX_NUMAXES: usize = 16;
STRUCT!{struct DESIGNVECTOR {
    dvReserved: DWORD,
    dvNumAxes: DWORD,
    dvValues: [LONG; MM_MAX_NUMAXES],
}}
pub type PDESIGNVECTOR = *mut DESIGNVECTOR;
pub type LPDESIGNVECTOR = *mut DESIGNVECTOR;
extern "system" {
    pub fn AddFontResourceExA(
        lpszFilename: LPCSTR,
        fl: DWORD,
        pdv: PVOID,
    ) -> c_int;
    pub fn AddFontResourceExW(
        lpszFilename: LPCWSTR,
        fl: DWORD,
        pdv: PVOID,
    ) -> c_int;
    pub fn RemoveFontResourceExA(
        name: LPCSTR,
        fl: DWORD,
        pdv: PVOID,
    ) -> BOOL;
    pub fn RemoveFontResourceExW(
        name: LPCWSTR,
        fl: DWORD,
        pdv: PVOID,
    ) -> BOOL;
    pub fn AddFontMemResourceEx(
        pbFont: PVOID,
        cbSize: DWORD,
        pdv: PVOID,
        pcFonts: *mut DWORD,
    ) -> HANDLE;
    pub fn RemoveFontMemResourceEx(
        h: HANDLE,
    ) -> BOOL;
}
pub const FR_PRIVATE: DWORD = 0x10;
pub const FR_NOT_ENUM: DWORD = 0x20;
pub const MM_MAX_AXES_NAMELEN: usize = 16;
STRUCT!{struct AXISINFOA {
    axMinValue: LONG,
    axMaxValue: LONG,
    axAxisName: [BYTE; MM_MAX_AXES_NAMELEN],
}}
pub type PAXISINFOA = *mut AXISINFOA;
pub type LPAXISINFOA = *mut AXISINFOA;
STRUCT!{struct AXISINFOW {
    axMinValue: LONG,
    axMaxValue: LONG,
    axAxisName: [WCHAR; MM_MAX_AXES_NAMELEN],
}}
pub type PAXISINFOW = *mut AXISINFOW;
pub type LPAXISINFOW = *mut AXISINFOW;
STRUCT!{struct AXESLISTA {
    axlReserved: DWORD,
    axlNumAxes: DWORD,
    axlAxisInfo: [AXISINFOA; MM_MAX_AXES_NAMELEN],
}}
pub type PAXESLISTA = *mut AXESLISTA;
pub type LPAXESLISTA = *mut AXESLISTA;
STRUCT!{struct AXESLISTW {
    axlReserved: DWORD,
    axlNumAxes: DWORD,
    axlAxisInfo: [AXISINFOW; MM_MAX_AXES_NAMELEN],
}}
pub type PAXESLISTW = *mut AXESLISTW;
pub type LPAXESLISTW = *mut AXESLISTW;
STRUCT!{struct ENUMLOGFONTEXDVA {
    elfEnumLogfontEx: ENUMLOGFONTEXA,
    elfDesignVector: DESIGNVECTOR,
}}
pub type PENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
pub type LPENUMLOGFONTEXDVA = *mut ENUMLOGFONTEXDVA;
STRUCT!{struct ENUMLOGFONTEXDVW {
    elfEnumLogfontEx: ENUMLOGFONTEXW,
    elfDesignVector: DESIGNVECTOR,
}}
pub type PENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
pub type LPENUMLOGFONTEXDVW = *mut ENUMLOGFONTEXDVW;
extern "system" {
    pub fn CreateFontIndirectExA(
        penumlfex: *const ENUMLOGFONTEXDVA,
    ) -> HFONT;
    pub fn CreateFontIndirectExW(
        penumlfex: *const ENUMLOGFONTEXDVW,
    ) -> HFONT;
}
STRUCT!{struct ENUMTEXTMETRICA {
    etmNewTextMetricEx: NEWTEXTMETRICEXA,
    etmAxesList: AXESLISTA,
}}
pub type PENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
pub type LPENUMTEXTMETRICA = *mut ENUMTEXTMETRICA;
STRUCT!{struct ENUMTEXTMETRICW {
    etmNewTextMetricEx: NEWTEXTMETRICEXW,
    etmAxesList: AXESLISTW,
}}
pub type PENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
pub type LPENUMTEXTMETRICW = *mut ENUMTEXTMETRICW;
extern "system" {
    pub fn GetViewportExtEx(
        hdc: HDC,
        lpsize: LPSIZE,
    ) -> BOOL;
    pub fn GetViewportOrgEx(
        hdc: HDC,
        lppoint: LPPOINT,
    ) -> BOOL;
    pub fn GetWindowExtEx(
        hdc: HDC,
        lpsize: LPSIZE,
    ) -> BOOL;
    pub fn GetWindowOrgEx(
        hdc: HDC,
        lppoint: LPPOINT,
    ) -> BOOL;
    pub fn IntersectClipRect(
        hdc: HDC,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> c_int;
    pub fn InvertRgn(
        hdc: HDC,
        hrgn: HRGN,
    ) -> BOOL;
    pub fn LineDDA(
        nXStart: c_int,
        nYStart: c_int,
        nXEnd: c_int,
        nYEnd: c_int,
        lpLineFunc: LINEDDAPROC,
        lpData: LPARAM,
    ) -> BOOL;
    pub fn LineTo(
        hdc: HDC,
        nXEnd: c_int,
        nYEnd: c_int,
    ) -> BOOL;
    pub fn MaskBlt(
        hdcDest: HDC,
        xDest: c_int,
        yDest: c_int,
        width: c_int,
        height: c_int,
        hdcSrc: HDC,
        xSrc: c_int,
        ySrc: c_int,
        hbmMask: HBITMAP,
        xMask: c_int,
        yMask: c_int,
        rop: DWORD,
    ) -> BOOL;
    pub fn PlgBlt(
        hdcDest: HDC,
        lpPoint: *const POINT,
        hdcSrc: HDC,
        xSrc: c_int,
        ySrc: c_int,
        width: c_int,
        height: c_int,
        hbmMask: HBITMAP,
        xMask: c_int,
        yMask: c_int,
    ) -> BOOL;
    pub fn OffsetClipRgn(
        hdc: HDC,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn OffsetRgn(
        hrgn: HRGN,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn PatBlt(
        hdc: HDC,
        nXLeft: c_int,
        nYLeft: c_int,
        nWidth: c_int,
        nHeight: c_int,
        dwRop: DWORD,
    ) -> BOOL;
    pub fn Pie(
        hdc: HDC,
        nLeftRect: c_int,
        nTopRect: c_int,
        nRightRect: c_int,
        nBottomRect: c_int,
        nXRadial1: c_int,
        nYRadial1: c_int,
        nXRadial2: c_int,
        nYRadial2: c_int,
    ) -> BOOL;
    pub fn PlayMetaFile(
        hdc: HDC,
        hmf: HMETAFILE,
    ) -> BOOL;
    pub fn PaintRgn(
        hdc: HDC,
        hrgn: HRGN,
    ) -> BOOL;
    pub fn PolyPolygon(
        hdc: HDC,
        lpPoints: *const POINT,
        lpPolyCounts: *const INT,
        cCount: DWORD,
    ) -> BOOL;
    pub fn PtInRegion(
        hrgn: HRGN,
        x: c_int,
        y: c_int,
    ) -> BOOL;
    pub fn PtVisible(
        hdc: HDC,
        x: c_int,
        y: c_int,
    ) -> BOOL;
    pub fn RectInRegion(
        hrgn: HRGN,
        lprect: *const RECT,
    ) -> BOOL;
    pub fn RectVisible(
        hdc: HDC,
        lprect: *const RECT,
    ) -> BOOL;
    pub fn Rectangle(
        hdc: HDC,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> BOOL;
    pub fn RestoreDC(
        hdc: HDC,
        nSavedDC: c_int,
    ) -> BOOL;
    pub fn ResetDCA(
        hdc: HDC,
        lpdm: *const DEVMODEA,
    ) -> HDC;
    pub fn ResetDCW(
        hdc: HDC,
        lpdm: *const DEVMODEW,
    ) -> HDC;
    pub fn RealizePalette(
        hdc: HDC,
    ) -> UINT;
    pub fn RemoveFontResourceA(
        lpFileName: LPCSTR,
    ) -> BOOL;
    pub fn RemoveFontResourceW(
        lpFileName: LPCWSTR,
    ) -> BOOL;
    pub fn RoundRect(
        hdc: HDC,
        nLeftRect: c_int,
        nTopRect: c_int,
        nRightRect: c_int,
        nBottomRect: c_int,
        nWidth: c_int,
        nHeight: c_int,
    ) -> BOOL;
    pub fn ResizePalette(
        hpal: HPALETTE,
        n: UINT,
    ) -> BOOL;
    pub fn SaveDC(
        hdc: HDC,
    ) -> c_int;
    pub fn SelectClipRgn(
        hdc: HDC,
        hrgn: HRGN,
    ) -> c_int;
    pub fn ExtSelectClipRgn(
        hdc: HDC,
        hrgn: HRGN,
        mode: c_int,
    ) -> c_int;
    pub fn SetMetaRgn(
        hdc: HDC,
    ) -> c_int;
    pub fn SelectObject(
        hdc: HDC,
        h: HGDIOBJ,
    ) -> HGDIOBJ;
    pub fn SelectPalette(
        hdc: HDC,
        hPal: HPALETTE,
        bForceBkgd: BOOL,
    ) -> HPALETTE;
    pub fn SetBkColor(
        hdc: HDC,
        color: COLORREF,
    ) -> COLORREF;
    pub fn SetDCBrushColor(
        hdc: HDC,
        color: COLORREF,
    ) -> COLORREF;
    pub fn SetDCPenColor(
        hdc: HDC,
        color: COLORREF,
    ) -> COLORREF;
    pub fn SetBkMode(
        hdc: HDC,
        mode: c_int,
    ) -> c_int;
    pub fn SetBitmapBits(
        hbm: HBITMAP,
        cb: DWORD,
        pvBits: *const VOID,
    ) -> LONG;
    pub fn SetBoundsRect(
        hdc: HDC,
        lprect: *const RECT,
        flags: UINT,
    ) -> UINT;
    pub fn SetDIBits(
        hdc: HDC,
        hbm: HBITMAP,
        start: UINT,
        cLines: UINT,
        lpBits: *const VOID,
        lpbmi: *const BITMAPINFO,
        ColorUse: UINT,
    ) -> c_int;
    pub fn SetDIBitsToDevice(
        hdc: HDC,
        xDest: c_int,
        yDest: c_int,
        w: DWORD,
        h: DWORD,
        xSrc: c_int,
        ySrc: c_int,
        StartScan: UINT,
        cLines: UINT,
        lpvBits: *const VOID,
        lpbmi: *const BITMAPINFO,
        ColorUse: UINT,
    ) -> c_int;
    pub fn SetMapperFlags(
        hdc: HDC,
        flags: DWORD,
    ) -> DWORD;
    pub fn SetGraphicsMode(
        hdc: HDC,
        iMode: c_int,
    ) -> c_int;
    pub fn SetMapMode(
        hdc: HDC,
        mode: c_int,
    ) -> c_int;
    pub fn SetLayout(
        hdc: HDC,
        l: DWORD,
    ) -> DWORD;
    pub fn GetLayout(
        hdc: HDC,
    ) -> DWORD;
    pub fn SetMetaFileBitsEx(
        cbBuffer: UINT,
        lpData: *const BYTE,
    ) -> HMETAFILE;
    pub fn SetPaletteEntries(
        hpal: HPALETTE,
        iStart: UINT,
        cEntries: UINT,
        pPalEntries: *const PALETTEENTRY,
    ) -> UINT;
    pub fn SetPixel(
        hdc: HDC,
        x: c_int,
        y: c_int,
        color: COLORREF,
    ) -> COLORREF;
    pub fn SetPixelV(
        hdc: HDC,
        x: c_int,
        y: c_int,
        color: COLORREF,
    ) -> BOOL;
    pub fn SetPixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> BOOL;
    pub fn SetPolyFillMode(
        hdc: HDC,
        iPolyFillMode: c_int,
    ) -> c_int;
    pub fn StretchBlt(
        hdcDest: HDC,
        xDest: c_int,
        yDest: c_int,
        wDest: c_int,
        hDest: c_int,
        hdcSrc: HDC,
        xSrc: c_int,
        ySrc: c_int,
        wSrc: c_int,
        hSrc: c_int,
        rop: DWORD,
    ) -> BOOL;
    pub fn SetRectRgn(
        hrgn: HRGN,
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
    ) -> BOOL;
    pub fn StretchDIBits(
        hdc: HDC,
        XDest: c_int,
        YDest: c_int,
        nDestWidth: c_int,
        nDestHeight: c_int,
        XSrc: c_int,
        YSrc: c_int,
        nSrcWidth: c_int,
        nSrcHeight: c_int,
        lpBits: *const VOID,
        lpBitsInfo: *const BITMAPINFO,
        iUsage: UINT,
        dwRop: DWORD,
    ) -> c_int;
    pub fn SetROP2(
        hdc: HDC,
        rop2: c_int,
    ) -> c_int;
    pub fn SetStretchBltMode(
        hdc: HDC,
        mode: c_int,
    ) -> c_int;
    pub fn SetSystemPaletteUse(
        hdc: HDC,
        uuse: UINT,
    ) -> UINT;
    pub fn SetTextCharacterExtra(
        hdc: HDC,
        extra: c_int,
    ) -> c_int;
    pub fn SetTextColor(
        hdc: HDC,
        color: COLORREF,
    ) -> COLORREF;
    pub fn SetTextAlign(
        hdc: HDC,
        align: UINT,
    ) -> UINT;
    pub fn SetTextJustification(
        hdc: HDC,
        extra: c_int,
        count: c_int,
    ) -> BOOL;
    pub fn UpdateColors(
        hdc: HDC,
    ) -> BOOL;
}
pub type COLOR16 = c_ushort;
STRUCT!{struct TRIVERTEX {
    x: LONG,
    y: LONG,
    Red: COLOR16,
    Green: COLOR16,
    Blue: COLOR16,
    Alpha: COLOR16,
}}
pub type PTRIVERTEX = *mut TRIVERTEX;
pub type LPTRIVERTEX = *mut TRIVERTEX;
STRUCT!{struct GRADIENT_RECT {
    UpperLeft: ULONG,
    LowerRight: ULONG,
}}
pub type PGRADIENT_RECT = *mut GRADIENT_RECT;
pub type LPGRADIENT_RECT = *mut GRADIENT_RECT;
STRUCT!{struct BLENDFUNCTION {
    BlendOp: BYTE,
    BlendFlags: BYTE,
    SourceConstantAlpha: BYTE,
    AlphaFormat: BYTE,
}}
pub type PBLENDFUNCTION = *mut BLENDFUNCTION;
pub const AC_SRC_OVER: BYTE = 0x00;
pub const AC_SRC_ALPHA: BYTE = 0x01;
extern "system" {
    pub fn AlphaBlend(
        hdcDest: HDC,
        xoriginDest: c_int,
        yoriginDest: c_int,
        wDest: c_int,
        hDest: c_int,
        hdcSrc: HDC,
        xoriginSrc: c_int,
        yoriginSrc: c_int,
        wSrc: c_int,
        hSrc: c_int,
        ftn: BLENDFUNCTION,
    ) -> BOOL;
    pub fn TransparentBlt(
        hdcDest: HDC,
        xoriginDest: c_int,
        yoriginDest: c_int,
        wDest: c_int,
        hDest: c_int,
        hdcSrc: HDC,
        xoriginSrc: c_int,
        yoriginSrc: c_int,
        wSrc: c_int,
        hSrc: c_int,
        crTransparent: UINT,
    ) -> BOOL;
}
pub const GRADIENT_FILL_RECT_H: ULONG = 0x00000000;
pub const GRADIENT_FILL_RECT_V: ULONG = 0x00000001;
pub const GRADIENT_FILL_TRIANGLE: ULONG = 0x00000002;
pub const GRADIENT_FILL_OP_FLAG: ULONG = 0x000000ff;
extern "system" {
    pub fn GradientFill(
        hdc: HDC,
        pVertex: PTRIVERTEX,
        nVertex: ULONG,
        pMesh: PVOID,
        nMesh: ULONG,
        ulMode: ULONG,
    ) -> BOOL;
    pub fn GdiAlphaBlend(
        hdcDest: HDC,
        xoriginDest: c_int,
        yoriginDest: c_int,
        wDest: c_int,
        hDest: c_int,
        hdcSrc: HDC,
        xoriginSrc: c_int,
        yoriginSrc: c_int,
        wSrc: c_int,
        hSrc: c_int,
        ftn: BLENDFUNCTION,
    ) -> BOOL;
    pub fn GdiTransparentBlt(
        hdcDest: HDC,
        xoriginDest: c_int,
        yoriginDest: c_int,
        wDest: c_int,
        hDest: c_int,
        hdcSrc: HDC,
        xoriginSrc: c_int,
        yoriginSrc: c_int,
        wSrc: c_int,
        hSrc: c_int,
        crTransparent: UINT,
    ) -> BOOL;
    pub fn GdiGradientFill(
        hdc: HDC,
        pVertex: PTRIVERTEX,
        nVertex: ULONG,
        pMesh: PVOID,
        nCount: ULONG,
        ulMode: ULONG,
    ) -> BOOL;
    pub fn PlayMetaFileRecord(
        hdc: HDC,
        lpHandleTable: LPHANDLETABLE,
        lpMR: LPMETARECORD,
        noObjs: UINT,
    ) -> BOOL;
}
FN!{stdcall MFENUMPROC(
    hdc: HDC,
    lpht: *mut HANDLETABLE,
    lpMR: *mut METARECORD,
    nObj: c_int,
    param: LPARAM,
) -> c_int}
extern "system" {
    pub fn EnumMetaFile(
        hdc: HDC,
        hmf: HMETAFILE,
        mproc: MFENUMPROC,
        param: LPARAM,
    ) -> BOOL;
}
FN!{stdcall ENHMFENUMPROC(
    hdc: HDC,
    lpht: *mut HANDLETABLE,
    lpmr: *const ENHMETARECORD,
    nHandles: c_int,
    data: LPARAM,
) -> c_int}
extern "system" {
    pub fn CloseEnhMetaFile(
        hdc: HDC,
    ) -> HENHMETAFILE;
    pub fn CopyEnhMetaFileA(
        hemfSrc: HENHMETAFILE,
        lpszFile: LPCSTR,
    ) -> HENHMETAFILE;
    pub fn CopyEnhMetaFileW(
        hemfSrc: HENHMETAFILE,
        lpszFile: LPCWSTR,
    ) -> HENHMETAFILE;
    pub fn CreateEnhMetaFileA(
        hdcRef: HDC,
        lpFilename: LPCSTR,
        lpRect: *const RECT,
        lpDescription: LPCSTR,
    ) -> HDC;
    pub fn CreateEnhMetaFileW(
        hdcRef: HDC,
        lpFilename: LPCWSTR,
        lpRect: *const RECT,
        lpDescription: LPCWSTR,
    ) -> HDC;
    pub fn DeleteEnhMetaFile(
        hmf: HENHMETAFILE,
    ) -> BOOL;
    pub fn EnumEnhMetaFile(
        hdc: HDC,
        hmf: HENHMETAFILE,
        lpProc: ENHMFENUMPROC,
        param: LPVOID,
        lpRect: *const RECT,
    ) -> BOOL;
    pub fn GetEnhMetaFileA(
        lpName: LPCSTR,
    ) -> HENHMETAFILE;
    pub fn GetEnhMetaFileW(
        lpName: LPCWSTR,
    ) -> HENHMETAFILE;
    pub fn GetEnhMetaFileBits(
        hEMF: HENHMETAFILE,
        nSize: UINT,
        lpData: LPBYTE,
    ) -> UINT;
    pub fn GetEnhMetaFileDescriptionA(
        hemf: HENHMETAFILE,
        cchBuffer: UINT,
        lpDescription: LPSTR,
    ) -> UINT;
    pub fn GetEnhMetaFileDescriptionW(
        hemf: HENHMETAFILE,
        cchBuffer: UINT,
        lpDescription: LPWSTR,
    ) -> UINT;
    pub fn GetEnhMetaFileHeader(
        hemf: HENHMETAFILE,
        nSize: UINT,
        lpEnhMetaHeader: LPENHMETAHEADER,
    ) -> UINT;
    pub fn GetEnhMetaFilePaletteEntries(
        hemf: HENHMETAFILE,
        nNumEntries: UINT,
        lpPaletteEntries: LPPALETTEENTRY,
    ) -> UINT;
    pub fn GetEnhMetaFilePixelFormat(
        hemf: HENHMETAFILE,
        cbBuffer: UINT,
        ppfd: *mut PIXELFORMATDESCRIPTOR,
    ) -> UINT;
    pub fn GetWinMetaFileBits(
        hemf: HENHMETAFILE,
        cbData16: UINT,
        pData16: LPBYTE,
        iMapMode: INT,
        hdcRef: HDC,
    ) -> UINT;
    pub fn PlayEnhMetaFile(
        hdc: HDC,
        hmf: HENHMETAFILE,
        lprect: *const RECT,
    ) -> BOOL;
    pub fn PlayEnhMetaFileRecord(
        hdc: HDC,
        pht: LPHANDLETABLE,
        pmr: *const ENHMETARECORD,
        cht: UINT,
    ) -> BOOL;
    pub fn SetEnhMetaFileBits(
        nSize: UINT,
        pb: *const BYTE,
    ) -> HENHMETAFILE;
    pub fn SetWinMetaFileBits(
        nSize: UINT,
        lpMeta16Data: *const BYTE,
        hdcRef: HDC,
        lpMFP: *const METAFILEPICT,
    ) -> HENHMETAFILE;
    pub fn GdiComment(
        hdc: HDC,
        nSize: UINT,
        lpData: *const BYTE,
    ) -> BOOL;
    pub fn GetTextMetricsA(
        hdc: HDC,
        lptm: LPTEXTMETRICA,
    ) -> BOOL;
    pub fn GetTextMetricsW(
        hdc: HDC,
        lptm: *mut TEXTMETRICW,
    ) -> BOOL;
}
STRUCT!{struct DIBSECTION {
    dsBm: BITMAP,
    dsBmih: BITMAPINFOHEADER,
    dsBitfields: [DWORD; 3],
    dshSection: HANDLE,
    dsOffset: DWORD,
}}
pub type PDIBSECTION = *mut DIBSECTION;
pub type LPDIBSECTION = *mut DIBSECTION;
extern "system" {
    pub fn AngleArc(
        hdc: HDC,
        X: c_int,
        Y: c_int,
        dwRadius: DWORD,
        eStartAngle: FLOAT,
        eSweepAngle: FLOAT,
    ) -> BOOL;
    pub fn PolyPolyline(
        hdc: HDC,
        lppt: *const POINT,
        lpdwPolyPoints: *const DWORD,
        cCount: DWORD,
    ) -> BOOL;
    pub fn GetWorldTransform(
        hdc: HDC,
        lpxf: LPXFORM,
    ) -> BOOL;
    pub fn SetWorldTransform(
        hdc: HDC,
        lpxf: *const XFORM,
    ) -> BOOL;
    pub fn ModifyWorldTransform(
        hdc: HDC,
        lpxf: *const XFORM,
        mode: DWORD,
    ) -> BOOL;
    pub fn CombineTransform(
        lpxformResult: LPXFORM,
        lpxform1: *const XFORM,
        lpxform2: *const XFORM,
    ) -> BOOL;
}
#[inline]
pub fn GDI_WIDTHBYTES(bits: DWORD) -> DWORD {
    ((bits + 31) & !31) / 8
}
#[inline]
pub fn GDI_DIBWIDTHBYTES(bi: &BITMAPINFOHEADER) -> DWORD {
    GDI_WIDTHBYTES((bi.biWidth as DWORD) * (bi.biBitCount as DWORD))
}
#[inline]
pub fn GDI__DIBSIZE(bi: &BITMAPINFOHEADER) -> DWORD {
    GDI_DIBWIDTHBYTES(bi) * bi.biHeight as DWORD
}
#[inline]
pub fn GDI_DIBSIZE(bi: &BITMAPINFOHEADER) -> DWORD {
    if bi.biHeight < 0 {
        GDI__DIBSIZE(bi) * -1i32 as u32
    } else {
        GDI__DIBSIZE(bi)
    }
}
extern "system" {
    pub fn CreateDIBSection(
        hdc: HDC,
        lpbmi: *const BITMAPINFO,
        usage: UINT,
        ppvBits: *mut *mut c_void,
        hSection: HANDLE,
        offset: DWORD,
    ) -> HBITMAP;
    pub fn GetDIBColorTable(
        hdc: HDC,
        iStart: UINT,
        cEntries: UINT,
        prgbq: *mut RGBQUAD,
    ) -> UINT;
    pub fn SetDIBColorTable(
        hdc: HDC,
        iStart: UINT,
        cEntries: UINT,
        prgbq: *const RGBQUAD,
    ) -> UINT;
}
pub const CA_NEGATIVE: WORD = 0x0001;
pub const CA_LOG_FILTER: WORD = 0x0002;
pub const ILLUMINANT_DEVICE_DEFAULT: WORD = 0;
pub const ILLUMINANT_A: WORD = 1;
pub const ILLUMINANT_B: WORD = 2;
pub const ILLUMINANT_C: WORD = 3;
pub const ILLUMINANT_D50: WORD = 4;
pub const ILLUMINANT_D55: WORD = 5;
pub const ILLUMINANT_D65: WORD = 6;
pub const ILLUMINANT_D75: WORD = 7;
pub const ILLUMINANT_F2: WORD = 8;
pub const ILLUMINANT_MAX_INDEX: WORD = ILLUMINANT_F2;
pub const ILLUMINANT_TUNGSTEN: WORD = ILLUMINANT_A;
pub const ILLUMINANT_DAYLIGHT: WORD = ILLUMINANT_C;
pub const ILLUMINANT_FLUORESCENT: WORD = ILLUMINANT_F2;
pub const ILLUMINANT_NTSC: WORD = ILLUMINANT_C;
pub const RGB_GAMMA_MIN: WORD = 0o2500; // FIXME It is octal in the headers but are the headers actually right?
pub const RGB_GAMMA_MAX: WORD = 65000;
pub const REFERENCE_WHITE_MIN: WORD = 6000;
pub const REFERENCE_WHITE_MAX: WORD = 10000;
pub const REFERENCE_BLACK_MIN: WORD = 0;
pub const REFERENCE_BLACK_MAX: WORD = 4000;
pub const COLOR_ADJ_MIN: SHORT = -100;
pub const COLOR_ADJ_MAX: SHORT = 100;
STRUCT!{struct COLORADJUSTMENT {
    caSize: WORD,
    caFlags: WORD,
    caIlluminantIndex: WORD,
    caRedGamma: WORD,
    caGreenGamma: WORD,
    caBlueGamma: WORD,
    caReferenceBlack: WORD,
    caReferenceWhite: WORD,
    caContrast: SHORT,
    caBrightness: SHORT,
    caColorfulness: SHORT,
    caRedGreenTint: SHORT,
}}
pub type PCOLORADJUSTMENT = *mut COLORADJUSTMENT;
pub type LPCOLORADJUSTMENT = *mut COLORADJUSTMENT;
extern "system" {
    pub fn SetColorAdjustment(
        hdc: HDC,
        lpca: *const COLORADJUSTMENT,
    ) -> BOOL;
    pub fn GetColorAdjustment(
        hdc: HDC,
        lpca: LPCOLORADJUSTMENT,
    ) -> BOOL;
    pub fn CreateHalftonePalette(
        hdc: HDC,
    ) -> HPALETTE;
}
FN!{stdcall ABORTPROC(
    HDC,
    c_int,
) -> BOOL}
STRUCT!{struct DOCINFOA {
    cbSize: c_int,
    lpszDocName: LPCSTR,
    lpszOutput: LPCSTR,
    lpszDatatype: LPCSTR,
    fwType: DWORD,
}}
pub type LPDOCINFOA = *mut DOCINFOA;
STRUCT!{struct DOCINFOW {
    cbSize: c_int,
    lpszDocName: LPCWSTR,
    lpszOutput: LPCWSTR,
    lpszDatatype: LPCWSTR,
    fwType: DWORD,
}}
pub type LPDOCINFOW = *mut DOCINFOW;
pub const DI_APPBANDING: DWORD = 0x00000001;
pub const DI_ROPS_READ_DESTINATION: DWORD = 0x00000002;
extern "system" {
    pub fn StartDocA(
        hdc: HDC,
        lpdi: *const DOCINFOA,
    ) -> c_int;
    pub fn StartDocW(
        hdc: HDC,
        lpdi: *const DOCINFOW,
    ) -> c_int;
    pub fn EndDoc(
        hdc: HDC,
    ) -> c_int;
    pub fn StartPage(
        hdc: HDC,
    ) -> c_int;
    pub fn EndPage(
        hdc: HDC,
    ) -> c_int;
    pub fn AbortDoc(
        hdc: HDC,
    ) -> c_int;
    pub fn SetAbortProc(
        hdc: HDC,
        aproc: ABORTPROC,
    ) -> c_int;
    pub fn AbortPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn ArcTo(
        hdc: HDC,
        nLeftRect: c_int,
        nTopRect: c_int,
        nRightRect: c_int,
        nBottomRect: c_int,
        nXRadial1: c_int,
        nYRadial1: c_int,
        nXRadial2: c_int,
        nYRadial2: c_int,
    ) -> BOOL;
    pub fn BeginPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn CloseFigure(
        hdc: HDC,
    ) -> BOOL;
    pub fn EndPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn FillPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn FlattenPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn GetPath(
        hdc: HDC,
        apt: LPPOINT,
        aj: LPBYTE,
        cpt: c_int,
    ) -> c_int;
    pub fn PathToRegion(
        hdc: HDC,
    ) -> HRGN;
    pub fn PolyDraw(
        hdc: HDC,
        lppt: *const POINT,
        lpbTypes: *const BYTE,
        cCount: c_int,
    ) -> BOOL;
    pub fn SelectClipPath(
        hdc: HDC,
        mode: c_int,
    ) -> BOOL;
    pub fn SetArcDirection(
        hdc: HDC,
        ArcDirection: c_int,
    ) -> c_int;
    pub fn SetMiterLimit(
        hdc: HDC,
        limit: FLOAT,
        old: PFLOAT,
    ) -> BOOL;
    pub fn StrokeAndFillPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn StrokePath(
        hdc: HDC,
    ) -> BOOL;
    pub fn WidenPath(
        hdc: HDC,
    ) -> BOOL;
    pub fn ExtCreatePen(
        iPenStyle: DWORD,
        cWidth: DWORD,
        plbrush: *const LOGBRUSH,
        cStyle: DWORD,
        pstyle: *const DWORD,
    ) -> HPEN;
    pub fn GetMiterLimit(
        hdc: HDC,
        plimit: PFLOAT,
    ) -> BOOL;
    pub fn GetArcDirection(
        hdc: HDC,
    ) -> c_int;
    pub fn GetObjectA(
        h: HANDLE,
        c: c_int,
        pv: LPVOID,
    ) -> c_int;
    pub fn GetObjectW(
        h: HANDLE,
        c: c_int,
        pv: LPVOID,
    ) -> c_int;
    pub fn MoveToEx(
        hdc: HDC,
        X: c_int,
        Y: c_int,
        lpPoint:LPPOINT,
    ) -> BOOL;
    pub fn TextOutA(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lpString: LPCSTR,
        c: c_int,
    ) -> BOOL;
    pub fn TextOutW(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lpString: LPCWSTR,
        c: c_int,
    ) -> BOOL;
    pub fn ExtTextOutA(
        hdc: HDC,
        x: c_int,
        y: c_int,
        options: UINT,
        lprect: *const RECT,
        lpString: LPCSTR,
        c: UINT,
        lpDx: *const INT,
    ) -> BOOL;
    pub fn ExtTextOutW(
        hdc: HDC,
        x: c_int,
        y: c_int,
        options: UINT,
        lprect: *const RECT,
        lpString: LPCWSTR,
        c: UINT,
        lpDx: *const INT,
    ) -> BOOL;
    pub fn PolyTextOutA(
        hdc: HDC,
        ppt: *const POLYTEXTA,
        nstrings: c_int,
    ) -> BOOL;
    pub fn PolyTextOutW(
        hdc: HDC,
        ppt: *const POLYTEXTW,
        nstrings: c_int,
    ) -> BOOL;
    pub fn CreatePolygonRgn(
        lppt: *const POINT,
        cPoints: c_int,
        fnPolyFillMode: c_int,
    ) -> HRGN;
    pub fn DPtoLP(
        hdc: HDC,
        lppt: *mut POINT,
        c: c_int,
    ) -> BOOL;
    pub fn LPtoDP(
        hdc: HDC,
        lppt: LPPOINT,
        c: c_int,
    ) -> BOOL;
    pub fn Polygon(
        hdc: HDC,
        lpPoints: *const POINT,
        nCount: c_int,
    ) -> BOOL;
    pub fn Polyline(
        hdc: HDC,
        lppt: *const POINT,
        cCount: c_int,
    ) -> BOOL;
    pub fn PolyBezier(
        hdc: HDC,
        lppt: *const POINT,
        cPoints: DWORD,
    ) -> BOOL;
    pub fn PolyBezierTo(
        hdc: HDC,
        lppt: *const POINT,
        cPoints: DWORD,
    ) -> BOOL;
    pub fn PolylineTo(
        hdc: HDC,
        lppt: *const POINT,
        cCount: DWORD,
    ) -> BOOL;
    pub fn SetViewportExtEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn SetViewportOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: *mut POINT,
    ) -> BOOL;
    pub fn SetWindowExtEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: LPSIZE,
    ) -> BOOL;
    pub fn SetWindowOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn OffsetViewportOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn OffsetWindowOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn ScaleViewportExtEx(
        hdc: HDC,xn: c_int,
        dx: c_int,
        yn: c_int,
        yd: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn ScaleWindowExtEx(
        hdc: HDC,
        xn: c_int,
        xd: c_int,
        yn: c_int,
        yd: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn SetBitmapDimensionEx(
        hbm: HBITMAP,
        w: c_int,
        h: c_int,
        lpsz: LPSIZE,
    ) -> BOOL;
    pub fn SetBrushOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn GetTextFaceA(
        hdc: HDC,
        c: c_int,
        lpName: LPSTR,
    ) -> c_int;
    pub fn GetTextFaceW(
        hdc: HDC,
        c: c_int,
        lpName: LPWSTR,
    ) -> c_int;
}
STRUCT!{struct KERNINGPAIR {
    wFirst: WORD,
    wSecond: WORD,
    iKernAmount: c_int,
}}
pub type LPKERNINGPAIR = *mut KERNINGPAIR;
extern "system" {
    pub fn GetKerningPairsA(
        hdc: HDC,
        nPairs: DWORD,
        lpKernPair: LPKERNINGPAIR,
    ) -> DWORD;
    pub fn GetKerningPairsW(
        hdc: HDC,
        nPairs: DWORD,
        lpKernPair: LPKERNINGPAIR,
    ) -> DWORD;
    pub fn GetDCOrgEx(
        hdc: HDC,
        lppt: LPPOINT,
    ) -> BOOL;
    pub fn FixBrushOrgEx(
        hdc: HDC,
        x: c_int,
        y: c_int,
        ptl: LPPOINT,
    ) -> BOOL;
    pub fn UnrealizeObject(
        h: HGDIOBJ,
    ) -> BOOL;
    pub fn GdiFlush() -> BOOL;
    pub fn GdiSetBatchLimit(
        dw: DWORD,
    ) -> DWORD;
    pub fn GdiGetBatchLimit() -> DWORD;
}
pub const ICM_OFF: c_int = 1;
pub const ICM_ON: c_int = 2;
pub const ICM_QUERY: c_int = 3;
pub const ICM_DONE_OUTSIDEDC: c_int = 4;
FN!{stdcall ICMENUMPROCA(
    LPSTR,
    LPARAM,
) -> c_int}
FN!{stdcall ICMENUMPROCW(
    LPWSTR,
    LPARAM,
) -> c_int}
extern "system" {
    pub fn SetICMMode(
        hdc: HDC,
        mode: c_int,
    ) -> c_int;
    pub fn CheckColorsInGamut(
        hDC: HDC,
        lpRGBTriples: LPVOID,
        lpBuffer: LPVOID,
        nCount: UINT,
    ) -> BOOL;
    pub fn GetColorSpace(
        hdc: HDC,
    ) -> HCOLORSPACE;
    pub fn GetLogColorSpaceA(
        hColorSpace: HCOLORSPACE,
        lpBuffer: LPLOGCOLORSPACEA,
        nSize: DWORD,
    ) -> BOOL;
    pub fn GetLogColorSpaceW(
        hColorSpace: HCOLORSPACE,
        lpBuffer: LPLOGCOLORSPACEW,
        nSize: DWORD,
    ) -> BOOL;
    pub fn CreateColorSpaceA(
        lpLogColorSpace: LPLOGCOLORSPACEA,
    ) -> HCOLORSPACE;
    pub fn CreateColorSpaceW(
        lpLogColorSpace: LPLOGCOLORSPACEW,
    ) -> HCOLORSPACE;
    pub fn SetColorSpace(
        hdc: HDC,
        hcs: HCOLORSPACE,
    ) -> HCOLORSPACE;
    pub fn DeleteColorSpace(
        hcs: HCOLORSPACE,
    ) -> BOOL;
    pub fn GetICMProfileA(
        hdc: HDC,
        pBufSize: LPDWORD,
        pszFilename: LPSTR,
    ) -> BOOL;
    pub fn GetICMProfileW(
        hdc: HDC,
        pBufSize: LPDWORD,
        pszFilename: LPWSTR,
    ) -> BOOL;
    pub fn SetICMProfileA(
        hdc: HDC,
        lpFileName: LPSTR,
    ) -> BOOL;
    pub fn SetICMProfileW(
        hdc: HDC,
        lpFileName: LPWSTR,
    ) -> BOOL;
    pub fn GetDeviceGammaRamp(
        hdc: HDC,
        lpRamp: LPVOID,
    ) -> BOOL;
    pub fn SetDeviceGammaRamp(
        hdc: HDC,
        lpRamp: LPVOID,
    ) -> BOOL;
    pub fn ColorMatchToTarget(
        hDC: HDC,
        hdcTarget: HDC,
        uiAction: UINT,
    ) -> BOOL;
    pub fn EnumICMProfilesA(
        hdc: HDC,
        iproc: ICMENUMPROCA,
        param: LPARAM,
    ) -> c_int;
    pub fn EnumICMProfilesW(
        hdc: HDC,
        iproc: ICMENUMPROCW,
        param: LPARAM,
    ) -> c_int;
    pub fn UpdateICMRegKeyA(
        reserved: DWORD,
        lpszCMID: LPSTR,
        lpszFileName: LPSTR,
        command: UINT,
    ) -> BOOL;
    pub fn UpdateICMRegKeyW(
        reserved: DWORD,
        lpszCMID: LPWSTR,
        lpszFileName: LPWSTR,
        command: UINT,
    ) -> BOOL;
    pub fn ColorCorrectPalette(
        hDC: HDC,
        hPalette: HPALETTE,
        dwFirstEntry: DWORD,
        dwNumOfEntries: DWORD,
    ) -> BOOL;
}
pub const ENHMETA_SIGNATURE: DWORD = 0x464D4520;
pub const ENHMETA_STOCK_OBJECT: DWORD = 0x80000000;
pub const EMR_HEADER: DWORD = 1;
pub const EMR_POLYBEZIER: DWORD = 2;
pub const EMR_POLYGON: DWORD = 3;
pub const EMR_POLYLINE: DWORD = 4;
pub const EMR_POLYBEZIERTO: DWORD = 5;
pub const EMR_POLYLINETO: DWORD = 6;
pub const EMR_POLYPOLYLINE: DWORD = 7;
pub const EMR_POLYPOLYGON: DWORD = 8;
pub const EMR_SETWINDOWEXTEX: DWORD = 9;
pub const EMR_SETWINDOWORGEX: DWORD = 10;
pub const EMR_SETVIEWPORTEXTEX: DWORD = 11;
pub const EMR_SETVIEWPORTORGEX: DWORD = 12;
pub const EMR_SETBRUSHORGEX: DWORD = 13;
pub const EMR_EOF: DWORD = 14;
pub const EMR_SETPIXELV: DWORD = 15;
pub const EMR_SETMAPPERFLAGS: DWORD = 16;
pub const EMR_SETMAPMODE: DWORD = 17;
pub const EMR_SETBKMODE: DWORD = 18;
pub const EMR_SETPOLYFILLMODE: DWORD = 19;
pub const EMR_SETROP2: DWORD = 20;
pub const EMR_SETSTRETCHBLTMODE: DWORD = 21;
pub const EMR_SETTEXTALIGN: DWORD = 22;
pub const EMR_SETCOLORADJUSTMENT: DWORD = 23;
pub const EMR_SETTEXTCOLOR: DWORD = 24;
pub const EMR_SETBKCOLOR: DWORD = 25;
pub const EMR_OFFSETCLIPRGN: DWORD = 26;
pub const EMR_MOVETOEX: DWORD = 27;
pub const EMR_SETMETARGN: DWORD = 28;
pub const EMR_EXCLUDECLIPRECT: DWORD = 29;
pub const EMR_INTERSECTCLIPRECT: DWORD = 30;
pub const EMR_SCALEVIEWPORTEXTEX: DWORD = 31;
pub const EMR_SCALEWINDOWEXTEX: DWORD = 32;
pub const EMR_SAVEDC: DWORD = 33;
pub const EMR_RESTOREDC: DWORD = 34;
pub const EMR_SETWORLDTRANSFORM: DWORD = 35;
pub const EMR_MODIFYWORLDTRANSFORM: DWORD = 36;
pub const EMR_SELECTOBJECT: DWORD = 37;
pub const EMR_CREATEPEN: DWORD = 38;
pub const EMR_CREATEBRUSHINDIRECT: DWORD = 39;
pub const EMR_DELETEOBJECT: DWORD = 40;
pub const EMR_ANGLEARC: DWORD = 41;
pub const EMR_ELLIPSE: DWORD = 42;
pub const EMR_RECTANGLE: DWORD = 43;
pub const EMR_ROUNDRECT: DWORD = 44;
pub const EMR_ARC: DWORD = 45;
pub const EMR_CHORD: DWORD = 46;
pub const EMR_PIE: DWORD = 47;
pub const EMR_SELECTPALETTE: DWORD = 48;
pub const EMR_CREATEPALETTE: DWORD = 49;
pub const EMR_SETPALETTEENTRIES: DWORD = 50;
pub const EMR_RESIZEPALETTE: DWORD = 51;
pub const EMR_REALIZEPALETTE: DWORD = 52;
pub const EMR_EXTFLOODFILL: DWORD = 53;
pub const EMR_LINETO: DWORD = 54;
pub const EMR_ARCTO: DWORD = 55;
pub const EMR_POLYDRAW: DWORD = 56;
pub const EMR_SETARCDIRECTION: DWORD = 57;
pub const EMR_SETMITERLIMIT: DWORD = 58;
pub const EMR_BEGINPATH: DWORD = 59;
pub const EMR_ENDPATH: DWORD = 60;
pub const EMR_CLOSEFIGURE: DWORD = 61;
pub const EMR_FILLPATH: DWORD = 62;
pub const EMR_STROKEANDFILLPATH: DWORD = 63;
pub const EMR_STROKEPATH: DWORD = 64;
pub const EMR_FLATTENPATH: DWORD = 65;
pub const EMR_WIDENPATH: DWORD = 66;
pub const EMR_SELECTCLIPPATH: DWORD = 67;
pub const EMR_ABORTPATH: DWORD = 68;
pub const EMR_GDICOMMENT: DWORD = 70;
pub const EMR_FILLRGN: DWORD = 71;
pub const EMR_FRAMERGN: DWORD = 72;
pub const EMR_INVERTRGN: DWORD = 73;
pub const EMR_PAINTRGN: DWORD = 74;
pub const EMR_EXTSELECTCLIPRGN: DWORD = 75;
pub const EMR_BITBLT: DWORD = 76;
pub const EMR_STRETCHBLT: DWORD = 77;
pub const EMR_MASKBLT: DWORD = 78;
pub const EMR_PLGBLT: DWORD = 79;
pub const EMR_SETDIBITSTODEVICE: DWORD = 80;
pub const EMR_STRETCHDIBITS: DWORD = 81;
pub const EMR_EXTCREATEFONTINDIRECTW: DWORD = 82;
pub const EMR_EXTTEXTOUTA: DWORD = 83;
pub const EMR_EXTTEXTOUTW: DWORD = 84;
pub const EMR_POLYBEZIER16: DWORD = 85;
pub const EMR_POLYGON16: DWORD = 86;
pub const EMR_POLYLINE16: DWORD = 87;
pub const EMR_POLYBEZIERTO16: DWORD = 88;
pub const EMR_POLYLINETO16: DWORD = 89;
pub const EMR_POLYPOLYLINE16: DWORD = 90;
pub const EMR_POLYPOLYGON16: DWORD = 91;
pub const EMR_POLYDRAW16: DWORD = 92;
pub const EMR_CREATEMONOBRUSH: DWORD = 93;
pub const EMR_CREATEDIBPATTERNBRUSHPT: DWORD = 94;
pub const EMR_EXTCREATEPEN: DWORD = 95;
pub const EMR_POLYTEXTOUTA: DWORD = 96;
pub const EMR_POLYTEXTOUTW: DWORD = 97;
pub const EMR_SETICMMODE: DWORD = 98;
pub const EMR_CREATECOLORSPACE: DWORD = 99;
pub const EMR_SETCOLORSPACE: DWORD = 100;
pub const EMR_DELETECOLORSPACE: DWORD = 101;
pub const EMR_GLSRECORD: DWORD = 102;
pub const EMR_GLSBOUNDEDRECORD: DWORD = 103;
pub const EMR_PIXELFORMAT: DWORD = 104;
pub const EMR_RESERVED_105: DWORD = 105;
pub const EMR_RESERVED_106: DWORD = 106;
pub const EMR_RESERVED_107: DWORD = 107;
pub const EMR_RESERVED_108: DWORD = 108;
pub const EMR_RESERVED_109: DWORD = 109;
pub const EMR_RESERVED_110: DWORD = 110;
pub const EMR_COLORCORRECTPALETTE: DWORD = 111;
pub const EMR_SETICMPROFILEA: DWORD = 112;
pub const EMR_SETICMPROFILEW: DWORD = 113;
pub const EMR_ALPHABLEND: DWORD = 114;
pub const EMR_SETLAYOUT: DWORD = 115;
pub const EMR_TRANSPARENTBLT: DWORD = 116;
pub const EMR_RESERVED_117: DWORD = 117;
pub const EMR_GRADIENTFILL: DWORD = 118;
pub const EMR_RESERVED_119: DWORD = 119;
pub const EMR_RESERVED_120: DWORD = 120;
pub const EMR_COLORMATCHTOTARGETW: DWORD = 121;
pub const EMR_CREATECOLORSPACEW: DWORD = 122;
pub const EMR_MIN: DWORD = 1;
pub const EMR_MAX: DWORD = 122;
STRUCT!{struct EMR {
    iType: DWORD,
    nSize: DWORD,
}}
pub type PEMR = *mut EMR;
STRUCT!{struct EMRTEXT {
    ptlReference: POINTL,
    nChars: DWORD,
    offString: DWORD,
    fOptions: DWORD,
    rcl: RECTL,
    offDx: DWORD,
}}
pub type PEMRTEXT = *mut EMRTEXT;
STRUCT!{struct EMRABORTPATH {
    emr: EMR,
}}
pub type PEMRABORTPATH = *mut EMRABORTPATH;
pub type EMRBEGINPATH = EMRABORTPATH;
pub type PEMRBEGINPATH = *mut EMRABORTPATH;
pub type EMRENDPATH = EMRABORTPATH;
pub type PEMRENDPATH = *mut EMRABORTPATH;
pub type EMRCLOSEFIGURE = EMRABORTPATH;
pub type PEMRCLOSEFIGURE = *mut EMRABORTPATH;
pub type EMRFLATTENPATH = EMRABORTPATH;
pub type PEMRFLATTENPATH = *mut EMRABORTPATH;
pub type EMRWIDENPATH = EMRABORTPATH;
pub type PEMRWIDENPATH = *mut EMRABORTPATH;
pub type EMRSETMETARGN = EMRABORTPATH;
pub type PEMRSETMETARGN = *mut EMRABORTPATH;
pub type EMRSAVEDC = EMRABORTPATH;
pub type PEMRSAVEDC = *mut EMRABORTPATH;
pub type EMRREALIZEPALETTE = EMRABORTPATH;
pub type PEMRREALIZEPALETTE = *mut EMRABORTPATH;
STRUCT!{struct EMRSELECTCLIPPATH {
    emr: EMR,
    iMode: DWORD,
}}
pub type PEMRSELECTCLIPPATH = *mut EMRSELECTCLIPPATH;
pub type EMRSETBKMODE = EMRSELECTCLIPPATH;
pub type PEMRSETBKMODE = *mut EMRSELECTCLIPPATH;
pub type EMRSETMAPMODE = EMRSELECTCLIPPATH;
pub type PEMRSETMAPMODE = *mut EMRSELECTCLIPPATH;
pub type EMRSETLAYOUT = EMRSELECTCLIPPATH;
pub type PEMRSETLAYOUT = *mut EMRSELECTCLIPPATH;
pub type EMRSETPOLYFILLMODE = EMRSELECTCLIPPATH;
pub type PEMRSETPOLYFILLMODE = *mut EMRSELECTCLIPPATH;
pub type EMRSETROP2 = EMRSELECTCLIPPATH;
pub type PEMRSETROP2 = *mut EMRSELECTCLIPPATH;
pub type EMRSETSTRETCHBLTMODE = EMRSELECTCLIPPATH;
pub type PEMRSETSTRETCHBLTMODE = *mut EMRSELECTCLIPPATH;
pub type EMRSETICMMODE = EMRSELECTCLIPPATH;
pub type PEMRSETICMMODE = *mut EMRSELECTCLIPPATH;
pub type EMRSETTEXTALIGN = EMRSELECTCLIPPATH;
pub type PEMRSETTEXTALIGN = *mut EMRSELECTCLIPPATH;
STRUCT!{struct EMRSETMITERLIMIT {
    emr: EMR,
    eMiterLimit: FLOAT,
}}
pub type PEMRSETMITERLIMIT = *mut EMRSETMITERLIMIT;
STRUCT!{struct EMRRESTOREDC {
    emr: EMR,
    iRelative: LONG,
}}
pub type PEMRRESTOREDC = *mut EMRRESTOREDC;
STRUCT!{struct EMRSETARCDIRECTION {
    emr: EMR,
    iArcDirection: DWORD,
}}
pub type PEMRSETARCDIRECTION = *mut EMRSETARCDIRECTION;
STRUCT!{struct EMRSETMAPPERFLAGS {
    emr: EMR,
    dwFlags: DWORD,
}}
pub type PEMRSETMAPPERFLAGS = *mut EMRSETMAPPERFLAGS;
STRUCT!{struct EMRSETBKCOLOR {
    emr: EMR,
    crColor: COLORREF,
}}
pub type PEMRSETBKCOLOR = *mut EMRSETBKCOLOR;
pub type EMRSETTEXTCOLOR = EMRSETBKCOLOR;
pub type PEMRSETTEXTCOLOR = *mut EMRSETBKCOLOR;
STRUCT!{struct EMRSELECTOBJECT {
    emr: EMR,
    ihObject: DWORD,
}}
pub type PEMRSELECTOBJECT = *mut EMRSELECTOBJECT;
pub type EMRDELETEOBJECT = EMRSELECTOBJECT;
pub type PEMRDELETEOBJECT = *mut EMRSELECTOBJECT;
STRUCT!{struct EMRSELECTPALETTE {
    emr: EMR,
    ihPal: DWORD,
}}
pub type PEMRSELECTPALETTE = *mut EMRSELECTPALETTE;
STRUCT!{struct EMRRESIZEPALETTE {
    emr: EMR,
    ihPal: DWORD,
    cEntries: DWORD,
}}
pub type PEMRRESIZEPALETTE = *mut EMRRESIZEPALETTE;
STRUCT!{struct EMRSETPALETTEENTRIES {
    emr: EMR,
    ihPal: DWORD,
    iStart: DWORD,
    cEntries: DWORD,
    aPalEntries: [PALETTEENTRY; 1],
}}
pub type PEMRSETPALETTEENTRIES = *mut EMRSETPALETTEENTRIES;
STRUCT!{struct EMRSETCOLORADJUSTMENT {
    emr: EMR,
    ColorAdjustment: COLORADJUSTMENT,
}}
pub type PEMRSETCOLORADJUSTMENT = *mut EMRSETCOLORADJUSTMENT;
STRUCT!{struct EMRGDICOMMENT {
    emr: EMR,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRGDICOMMENT = *mut EMRGDICOMMENT;
STRUCT!{struct EMREOF {
    emr: EMR,
    nPalEntries: DWORD,
    offPalEntries: DWORD,
    nSizeLast: DWORD,
}}
pub type PEMREOF = *mut EMREOF;
STRUCT!{struct EMRLINETO {
    emr: EMR,
    ptl: POINTL,
}}
pub type PEMRLINETO = *mut EMRLINETO;
pub type EMRMOVETOEX = EMRLINETO;
pub type PEMRMOVETOEX = *mut EMRLINETO;
STRUCT!{struct EMROFFSETCLIPRGN {
    emr: EMR,
    ptlOffset: POINTL,
}}
pub type PEMROFFSETCLIPRGN = *mut EMROFFSETCLIPRGN;
STRUCT!{struct EMRFILLPATH {
    emr: EMR,
    rclBounds: RECTL,
}}
pub type PEMRFILLPATH = *mut EMRFILLPATH;
pub type EMRSTROKEANDFILLPATH = EMRFILLPATH;
pub type PEMRSTROKEANDFILLPATH = *mut EMRFILLPATH;
pub type EMRSTROKEPATH = EMRFILLPATH;
pub type PEMRSTROKEPATH = *mut EMRFILLPATH;
STRUCT!{struct EMREXCLUDECLIPRECT {
    emr: EMR,
    rclClip: RECTL,
}}
pub type PEMREXCLUDECLIPRECT = *mut EMREXCLUDECLIPRECT;
pub type EMRINTERSECTCLIPRECT = EMREXCLUDECLIPRECT;
pub type PEMRINTERSECTCLIPRECT = *mut EMREXCLUDECLIPRECT;
STRUCT!{struct EMRSETVIEWPORTORGEX {
    emr: EMR,
    ptlOrigin: POINTL,
}}
pub type PEMRSETVIEWPORTORGEX = *mut EMRSETVIEWPORTORGEX;
pub type EMRSETWINDOWORGEX = EMRSETVIEWPORTORGEX;
pub type PEMRSETWINDOWORGEX = *mut EMRSETVIEWPORTORGEX;
pub type EMRSETBRUSHORGEX = EMRSETVIEWPORTORGEX;
pub type PEMRSETBRUSHORGEX = *mut EMRSETVIEWPORTORGEX;
STRUCT!{struct EMRSETVIEWPORTEXTEX {
    emr: EMR,
    szlExtent: SIZEL,
}}
pub type PEMRSETVIEWPORTEXTEX = *mut EMRSETVIEWPORTEXTEX;
pub type EMRSETWINDOWEXTEX = EMRSETVIEWPORTEXTEX;
pub type PEMRSETWINDOWEXTEX = *mut EMRSETVIEWPORTEXTEX;
STRUCT!{struct EMRSCALEVIEWPORTEXTEX {
    emr: EMR,
    xNum: LONG,
    xDenom: LONG,
    yNum: LONG,
    yDenom: LONG,
}}
pub type PEMRSCALEVIEWPORTEXTEX = *mut EMRSCALEVIEWPORTEXTEX;
pub type EMRSCALEWINDOWEXTEX = EMRSCALEVIEWPORTEXTEX;
pub type PEMRSCALEWINDOWEXTEX = *mut EMRSCALEVIEWPORTEXTEX;
STRUCT!{struct EMRSETWORLDTRANSFORM {
    emr: EMR,
    xform: XFORM,
}}
pub type PEMRSETWORLDTRANSFORM = *mut EMRSETWORLDTRANSFORM;
STRUCT!{struct EMRMODIFYWORLDTRANSFORM {
    emr: EMR,
    xform: XFORM,
    iMode: DWORD,
}}
pub type PEMRMODIFYWORLDTRANSFORM = *mut EMRMODIFYWORLDTRANSFORM;
STRUCT!{struct EMRSETPIXELV {
    emr: EMR,
    ptlPixel: POINTL,
    crColor: COLORREF,
}}
pub type PEMRSETPIXELV = *mut EMRSETPIXELV;
STRUCT!{struct EMREXTFLOODFILL {
    emr: EMR,
    ptlStart: POINTL,
    crColor: COLORREF,
    iMode: DWORD,
}}
pub type PEMREXTFLOODFILL = *mut EMREXTFLOODFILL;
STRUCT!{struct EMRELLIPSE {
    emr: EMR,
    rclBox: RECTL,
}}
pub type PEMRELLIPSE = *mut EMRELLIPSE;
pub type EMRRECTANGLE = EMRELLIPSE;
pub type PEMRRECTANGLE = *mut EMRELLIPSE;
STRUCT!{struct EMRROUNDRECT {
    emr: EMR,
    rclBox: RECTL,
    szlCorner: SIZEL,
}}
pub type PEMRROUNDRECT = *mut EMRROUNDRECT;
STRUCT!{struct EMRARC {
    emr: EMR,
    rclBox: RECTL,
    ptlStart: POINTL,
    ptlEnd: POINTL,
}}
pub type PEMRARC = *mut EMRARC;
pub type EMRARCTO = EMRARC;
pub type PEMRARCTO = *mut EMRARC;
pub type EMRCHORD = EMRARC;
pub type PEMRCHORD = *mut EMRARC;
pub type EMRPIE = EMRARC;
pub type PEMRPIE = *mut EMRARC;
STRUCT!{struct EMRANGLEARC {
    emr: EMR,
    ptlCenter: POINTL,
    nRadius: DWORD,
    eStartAngle: FLOAT,
    eSweepAngle: FLOAT,
}}
pub type PEMRANGLEARC = *mut EMRANGLEARC;
STRUCT!{struct EMRPOLYLINE {
    emr: EMR,
    rclBounds: RECTL,
    cptl: DWORD,
    aptl: [POINTL; 1],
}}
pub type PEMRPOLYLINE = *mut EMRPOLYLINE;
pub type EMRPOLYBEZIER = EMRPOLYLINE;
pub type PEMRPOLYBEZIER = *mut EMRPOLYLINE;
pub type EMRPOLYGON = EMRPOLYLINE;
pub type PEMRPOLYGON = *mut EMRPOLYLINE;
pub type EMRPOLYBEZIERTO = EMRPOLYLINE;
pub type PEMRPOLYBEZIERTO = *mut EMRPOLYLINE;
pub type EMRPOLYLINETO = EMRPOLYLINE;
pub type PEMRPOLYLINETO = *mut EMRPOLYLINE;
STRUCT!{struct EMRPOLYLINE16 {
    emr: EMR,
    rclBounds: RECTL,
    cpts: DWORD,
    apts: [POINTS; 1],
}}
pub type PEMRPOLYLINE16 = *mut EMRPOLYLINE16;
pub type EMRPOLYBEZIER16 = EMRPOLYLINE16;
pub type PEMRPOLYBEZIER16 = *mut EMRPOLYLINE16;
pub type EMRPOLYGON16 = EMRPOLYLINE16;
pub type PEMRPOLYGON16 = *mut EMRPOLYLINE16;
pub type EMRPOLYBEZIERTO16 = EMRPOLYLINE16;
pub type PEMRPOLYBEZIERTO16 = *mut EMRPOLYLINE16;
pub type EMRPOLYLINETO16 = EMRPOLYLINE16;
pub type PEMRPOLYLINETO16 = *mut EMRPOLYLINE16;
STRUCT!{struct EMRPOLYDRAW {
    emr: EMR,
    rclBounds: RECTL,
    cptl: DWORD,
    aptl: [POINTL; 1],
    abTypes: [BYTE; 1],
}}
pub type PEMRPOLYDRAW = *mut EMRPOLYDRAW;
STRUCT!{struct EMRPOLYDRAW16 {
    emr: EMR,
    rclBounds: RECTL,
    cpts: DWORD,
    apts: [POINTS; 1],
    abTypes: [BYTE; 1],
}}
pub type PEMRPOLYDRAW16 = *mut EMRPOLYDRAW16;
STRUCT!{struct EMRPOLYPOLYLINE {
    emr: EMR,
    rclBounds: RECTL,
    nPolys: DWORD,
    cptl: DWORD,
    aPolyCounts: [DWORD; 1],
    aptl: [POINTL; 1],
}}
pub type PEMRPOLYPOLYLINE = *mut EMRPOLYPOLYLINE;
pub type EMRPOLYPOLYGON = EMRPOLYPOLYLINE;
pub type PEMRPOLYPOLYGON = *mut EMRPOLYPOLYLINE;
STRUCT!{struct EMRPOLYPOLYLINE16 {
    emr: EMR,
    rclBounds: RECTL,
    nPolys: DWORD,
    cpts: DWORD,
    aPolyCounts: [DWORD; 1],
    apts: [POINTS; 1],
}}
pub type PEMRPOLYPOLYLINE16 = *mut EMRPOLYPOLYLINE16;
pub type EMRPOLYPOLYGON16 = EMRPOLYPOLYLINE16;
pub type PEMRPOLYPOLYGON16 = *mut EMRPOLYPOLYLINE16;
STRUCT!{struct EMRINVERTRGN {
    emr: EMR,
    rclBounds: RECTL,
    cbRgnData: DWORD,
    RgnData: [BYTE; 1],
}}
pub type PEMRINVERTRGN = *mut EMRINVERTRGN;
pub type EMRPAINTRGN = EMRINVERTRGN;
pub type PEMRPAINTRGN = *mut EMRINVERTRGN;
STRUCT!{struct EMRFILLRGN {
    emr: EMR,
    rclBounds: RECTL,
    cbRgnData: DWORD,
    ihBrush: DWORD,
    RgnData: [BYTE; 1],
}}
pub type PEMRFILLRGN = *mut EMRFILLRGN;
STRUCT!{struct EMRFRAMERGN {
    emr: EMR,
    rclBounds: RECTL,
    cbRgnData: DWORD,
    ihBrush: DWORD,
    szlStroke: SIZEL,
    RgnData: [BYTE; 1],
}}
pub type PEMRFRAMERGN = *mut EMRFRAMERGN;
STRUCT!{struct EMREXTSELECTCLIPRGN {
    emr: EMR,
    cbRgnData: DWORD,
    iMode: DWORD,
    RgnData: [BYTE; 1],
}}
pub type PEMREXTSELECTCLIPRGN = *mut EMREXTSELECTCLIPRGN;
STRUCT!{struct EMREXTTEXTOUTA {
    emr: EMR,
    rclBounds: RECTL,
    iGraphicsMode: DWORD,
    exScale: FLOAT,
    eyScale: FLOAT,
    emrtext: EMRTEXT,
}}
pub type PEMREXTTEXTOUTA = *mut EMREXTTEXTOUTA;
pub type EMREXTTEXTOUTW = EMREXTTEXTOUTA;
pub type PEMREXTTEXTOUTW = *mut EMREXTTEXTOUTA;
STRUCT!{struct EMRPOLYTEXTOUTA {
    emr: EMR,
    rclBounds: RECTL,
    iGraphicsMode: DWORD,
    exScale: FLOAT,
    eyScale: FLOAT,
    cStrings: LONG,
    aemrtext: [EMRTEXT; 1],
}}
pub type PEMRPOLYTEXTOUTA = *mut EMRPOLYTEXTOUTA;
pub type EMRPOLYTEXTOUTW = EMRPOLYTEXTOUTA;
pub type PEMRPOLYTEXTOUTW = *mut EMRPOLYTEXTOUTA;
STRUCT!{struct EMRBITBLT {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    cxDest: LONG,
    cyDest: LONG,
    dwRop: DWORD,
    xSrc: LONG,
    ySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
}}
pub type PEMRBITBLT = *mut EMRBITBLT;
STRUCT!{struct EMRSTRETCHBLT {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    cxDest: LONG,
    cyDest: LONG,
    dwRop: DWORD,
    xSrc: LONG,
    ySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    cxSrc: LONG,
    cySrc: LONG,
}}
pub type PEMRSTRETCHBLT = *mut EMRSTRETCHBLT;
STRUCT!{struct EMRMASKBLT {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    cxDest: LONG,
    cyDest: LONG,
    dwRop: DWORD,
    xSrc: LONG,
    ySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    xMask: LONG,
    yMask: LONG,
    iUsageMask: DWORD,
    offBmiMask: DWORD,
    cbBmiMask: DWORD,
    offBitsMask: DWORD,
    cbBitsMask: DWORD,
}}
pub type PEMRMASKBLT = *mut EMRMASKBLT;
STRUCT!{struct EMRPLGBLT {
    emr: EMR,
    rclBounds: RECTL,
    aptlDest: [POINTL; 3],
    xSrc: LONG,
    ySrc: LONG,
    cxSrc: LONG,
    cySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    xMask: LONG,
    yMask: LONG,
    iUsageMask: DWORD,
    offBmiMask: DWORD,
    cbBmiMask: DWORD,
    offBitsMask: DWORD,
    cbBitsMask: DWORD,
}}
pub type PEMRPLGBLT = *mut EMRPLGBLT;
STRUCT!{struct EMRSETDIBITSTODEVICE {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    xSrc: LONG,
    ySrc: LONG,
    cxSrc: LONG,
    cySrc: LONG,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    iUsageSrc: DWORD,
    iStartScan: DWORD,
    cScans: DWORD,
}}
pub type PEMRSETDIBITSTODEVICE = *mut EMRSETDIBITSTODEVICE;
STRUCT!{struct EMRSTRETCHDIBITS {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    xSrc: LONG,
    ySrc: LONG,
    cxSrc: LONG,
    cySrc: LONG,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    iUsageSrc: DWORD,
    dwRop: DWORD,
    cxDest: LONG,
    cyDest: LONG,
}}
pub type PEMRSTRETCHDIBITS = *mut EMRSTRETCHDIBITS;
STRUCT!{struct EMREXTCREATEFONTINDIRECTW {
    emr: EMR,
    ihFont: DWORD,
    elfw: EXTLOGFONTW,
}}
pub type PEMREXTCREATEFONTINDIRECTW = *mut EMREXTCREATEFONTINDIRECTW;
STRUCT!{struct EMRCREATEPALETTE {
    emr: EMR,
    ihPal: DWORD,
    lgpl: LOGPALETTE,
}}
pub type PEMRCREATEPALETTE = *mut EMRCREATEPALETTE;
STRUCT!{struct EMRCREATEPEN {
    emr: EMR,
    ihPen: DWORD,
    lopn: LOGPEN,
}}
pub type PEMRCREATEPEN = *mut EMRCREATEPEN;
STRUCT!{struct EMREXTCREATEPEN {
    emr: EMR,
    ihPen: DWORD,
    offBmi: DWORD,
    cbBmi: DWORD,
    offBits: DWORD,
    cbBits: DWORD,
    elp: EXTLOGPEN32,
}}
pub type PEMREXTCREATEPEN = *mut EMREXTCREATEPEN;
STRUCT!{struct EMRCREATEBRUSHINDIRECT {
    emr: EMR,
    ihBrush: DWORD,
    lb: LOGBRUSH32,
}}
pub type PEMRCREATEBRUSHINDIRECT = *mut EMRCREATEBRUSHINDIRECT;
STRUCT!{struct EMRCREATEMONOBRUSH {
    emr: EMR,
    ihBrush: DWORD,
    iUsage: DWORD,
    offBmi: DWORD,
    cbBmi: DWORD,
    offBits: DWORD,
    cbBits: DWORD,
}}
pub type PEMRCREATEMONOBRUSH = *mut EMRCREATEMONOBRUSH;
STRUCT!{struct EMRCREATEDIBPATTERNBRUSHPT {
    emr: EMR,
    ihBrush: DWORD,
    iUsage: DWORD,
    offBmi: DWORD,
    cbBmi: DWORD,
    offBits: DWORD,
    cbBits: DWORD,
}}
pub type PEMRCREATEDIBPATTERNBRUSHPT = *mut EMRCREATEDIBPATTERNBRUSHPT;
STRUCT!{struct EMRFORMAT {
    dSignature: DWORD,
    nVersion: DWORD,
    cbData: DWORD,
    offData: DWORD,
}}
pub type PEMRFORMAT = *mut EMRFORMAT;
STRUCT!{struct EMRGLSRECORD {
    emr: EMR,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRGLSRECORD = *mut EMRGLSRECORD;
STRUCT!{struct EMRGLSBOUNDEDRECORD {
    emr: EMR,
    rclBounds: RECTL,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRGLSBOUNDEDRECORD = *mut EMRGLSBOUNDEDRECORD;
STRUCT!{struct EMRPIXELFORMAT {
    emr: EMR,
    pfd: PIXELFORMATDESCRIPTOR,
}}
pub type PEMRPIXELFORMAT = *mut EMRPIXELFORMAT;
STRUCT!{struct EMRCREATECOLORSPACE {
    emr: EMR,
    ihCS: DWORD,
    lcs: LOGCOLORSPACEA,
}}
pub type PEMRCREATECOLORSPACE = *mut EMRCREATECOLORSPACE;
STRUCT!{struct EMRSETCOLORSPACE {
    emr: EMR,
    ihCS: DWORD,
}}
pub type PEMRSETCOLORSPACE = *mut EMRSETCOLORSPACE;
pub type EMRSELECTCOLORSPACE = EMRSETCOLORSPACE;
pub type PEMRSELECTCOLORSPACE = *mut EMRSETCOLORSPACE;
pub type EMRDELETECOLORSPACE = EMRSETCOLORSPACE;
pub type PEMRDELETECOLORSPACE = *mut EMRSETCOLORSPACE;
STRUCT!{struct EMREXTESCAPE {
    emr: EMR,
    iEscape: INT,
    cbEscData: INT,
    EscData: [BYTE; 1],
}}
pub type PEMREXTESCAPE = *mut EMREXTESCAPE;
pub type EMRDRAWESCAPE = EMREXTESCAPE;
pub type PEMRDRAWESCAPE = *mut EMREXTESCAPE;
STRUCT!{struct EMRNAMEDESCAPE {
    emr: EMR,
    iEscape: INT,
    cbDriver: INT,
    cbEscData: INT,
    EscData: [BYTE; 1],
}}
pub type PEMRNAMEDESCAPE = *mut EMRNAMEDESCAPE;
pub const SETICMPROFILE_EMBEDED: DWORD = 0x00000001;
STRUCT!{struct EMRSETICMPROFILE {
    emr: EMR,
    dwFlags: DWORD,
    cbName: DWORD,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRSETICMPROFILE = *mut EMRSETICMPROFILE;
pub type EMRSETICMPROFILEA = EMRSETICMPROFILE;
pub type PEMRSETICMPROFILEA = *mut EMRSETICMPROFILE;
pub type EMRSETICMPROFILEW = EMRSETICMPROFILE;
pub type PEMRSETICMPROFILEW = *mut EMRSETICMPROFILE;
pub const CREATECOLORSPACE_EMBEDED: DWORD = 0x00000001;
STRUCT!{struct EMRCREATECOLORSPACEW {
    emr: EMR,
    ihCS: DWORD,
    lcs: LOGCOLORSPACEW,
    dwFlags: DWORD,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRCREATECOLORSPACEW = *mut EMRCREATECOLORSPACEW;
pub const COLORMATCHTOTARGET_EMBEDED: DWORD = 0x00000001;
STRUCT!{struct EMRCOLORMATCHTOTARGET {
    emr: EMR,
    dwAction: DWORD,
    dwFlags: DWORD,
    cbName: DWORD,
    cbData: DWORD,
    Data: [BYTE; 1],
}}
pub type PEMRCOLORMATCHTOTARGET = *mut EMRCOLORMATCHTOTARGET;
STRUCT!{struct EMRCOLORCORRECTPALETTE {
    emr: EMR,
    ihPalette: DWORD,
    nFirstEntry: DWORD,
    nPalEntries: DWORD,
    nReserved: DWORD,
}}
pub type PEMRCOLORCORRECTPALETTE = *mut EMRCOLORCORRECTPALETTE;
STRUCT!{struct EMRALPHABLEND {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    cxDest: LONG,
    cyDest: LONG,
    dwRop: DWORD,
    xSrc: LONG,
    ySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    cxSrc: LONG,
    cySrc: LONG,
}}
pub type PEMRALPHABLEND = *mut EMRALPHABLEND;
STRUCT!{struct EMRGRADIENTFILL {
    emr: EMR,
    rclBounds: RECTL,
    nVer: DWORD,
    nTri: DWORD,
    ulMode: ULONG,
    Ver: [TRIVERTEX; 1],
}}
pub type PEMRGRADIENTFILL = *mut EMRGRADIENTFILL;
STRUCT!{struct EMRTRANSPARENTBLT {
    emr: EMR,
    rclBounds: RECTL,
    xDest: LONG,
    yDest: LONG,
    cxDest: LONG,
    cyDest: LONG,
    dwRop: DWORD,
    xSrc: LONG,
    ySrc: LONG,
    xformSrc: XFORM,
    crBkColorSrc: COLORREF,
    iUsageSrc: DWORD,
    offBmiSrc: DWORD,
    cbBmiSrc: DWORD,
    offBitsSrc: DWORD,
    cbBitsSrc: DWORD,
    cxSrc: LONG,
    cySrc: LONG,
}}
pub type PEMRTRANSPARENTBLT = *mut EMRTRANSPARENTBLT;
pub const GDICOMMENT_IDENTIFIER: DWORD = 0x43494447;
pub const GDICOMMENT_WINDOWS_METAFILE: DWORD = 0x80000001;
pub const GDICOMMENT_BEGINGROUP: DWORD = 0x00000002;
pub const GDICOMMENT_ENDGROUP: DWORD = 0x00000003;
pub const GDICOMMENT_MULTIFORMATS: DWORD = 0x40000004;
pub const EPS_SIGNATURE: DWORD = 0x46535045;
pub const GDICOMMENT_UNICODE_STRING: DWORD = 0x00000040;
pub const GDICOMMENT_UNICODE_END: DWORD = 0x00000080;
extern "system" {
    pub fn wglCopyContext(
        hglrcSrc: HGLRC,
        hglrcDst: HGLRC,
        mask: UINT,
    ) -> BOOL;
    pub fn wglCreateContext(
        hdc: HDC,
    ) -> HGLRC;
    pub fn wglCreateLayerContext(
        hdc: HDC,
        iLayerPlane: c_int,
    ) -> HGLRC;
    pub fn wglDeleteContext(
        hglrc: HGLRC,
    ) -> BOOL;
    pub fn wglGetCurrentContext() -> HGLRC;
    pub fn wglGetCurrentDC() -> HDC;
    pub fn wglGetProcAddress(
        lpszProc: LPCSTR,
    ) -> PROC;
    pub fn wglMakeCurrent(
        hdc: HDC,
        hglrc: HGLRC,
    ) -> BOOL;
    pub fn wglShareLists(
        hglrc1: HGLRC,
        hglrc2: HGLRC,
    ) -> BOOL;
    pub fn wglUseFontBitmapsA(
        hdc: HDC,
        first: DWORD,
        count: DWORD,
        listBase: DWORD,
    ) -> BOOL;
    pub fn wglUseFontBitmapsW(
        hdc: HDC,
        first: DWORD,
        count: DWORD,
        listBase: DWORD,
    ) -> BOOL;
    pub fn SwapBuffers(
        hdc: HDC,
    ) -> BOOL;
}
STRUCT!{struct POINTFLOAT {
    x: FLOAT,
    y: FLOAT,
}}
pub type PPOINTFLOAT = *mut POINTFLOAT;
STRUCT!{struct GLYPHMETRICSFLOAT {
    gmfBlackBoxX: FLOAT,
    gmfBlackBoxY: FLOAT,
    gmfptGlyphOrigin: POINTFLOAT,
    gmfCellIncX: FLOAT,
    gmfCellIncY: FLOAT,
}}
pub type PGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub type LPGLYPHMETRICSFLOAT = *mut GLYPHMETRICSFLOAT;
pub const WGL_FONT_LINES: DWORD = 0;
pub const WGL_FONT_POLYGONS: DWORD = 1;
extern "system" {
    pub fn wglUseFontOutlinesA(
        hdc: HDC,
        first: DWORD,
        count: DWORD,
        listBase: DWORD,
        deviation: FLOAT,
        extrusion: FLOAT,
        format: c_int,
        lpgmf: LPGLYPHMETRICSFLOAT,
    ) -> BOOL;
    pub fn wglUseFontOutlinesW(
        hdc: HDC,
        first: DWORD,
        count: DWORD,
        listBase: DWORD,
        deviation: FLOAT,
        extrusion: FLOAT,
        format: c_int,
        lpgmf: LPGLYPHMETRICSFLOAT,
    ) -> BOOL;
}
STRUCT!{struct LAYERPLANEDESCRIPTOR {
    nSize: WORD,
    nVersion: WORD,
    dwFlags: DWORD,
    iPixelType: BYTE,
    cColorBits: BYTE,
    cRedBits: BYTE,
    cRedShift: BYTE,
    cGreenBits: BYTE,
    cGreenShift: BYTE,
    cBlueBits: BYTE,
    cBlueShift: BYTE,
    cAlphaBits: BYTE,
    cAlphaShift: BYTE,
    cAccumBits: BYTE,
    cAccumRedBits: BYTE,
    cAccumGreenBits: BYTE,
    cAccumBlueBits: BYTE,
    cAccumAlphaBits: BYTE,
    cDepthBits: BYTE,
    cStencilBits: BYTE,
    cAuxBuffers: BYTE,
    iLayerPlane: BYTE,
    bReserved: BYTE,
    crTransparent: COLORREF,
}}
pub type PLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
pub type LPLAYERPLANEDESCRIPTOR = *mut LAYERPLANEDESCRIPTOR;
pub const LPD_DOUBLEBUFFER: DWORD = 0x00000001;
pub const LPD_STEREO: DWORD = 0x00000002;
pub const LPD_SUPPORT_GDI: DWORD = 0x00000010;
pub const LPD_SUPPORT_OPENGL: DWORD = 0x00000020;
pub const LPD_SHARE_DEPTH: DWORD = 0x00000040;
pub const LPD_SHARE_STENCIL: DWORD = 0x00000080;
pub const LPD_SHARE_ACCUM: DWORD = 0x00000100;
pub const LPD_SWAP_EXCHANGE: DWORD = 0x00000200;
pub const LPD_SWAP_COPY: DWORD = 0x00000400;
pub const LPD_TRANSPARENT: DWORD = 0x00001000;
pub const LPD_TYPE_RGBA: BYTE = 0;
pub const LPD_TYPE_COLORINDEX: BYTE = 1;
pub const WGL_SWAP_MAIN_PLANE: UINT = 0x00000001;
pub const WGL_SWAP_OVERLAY1: UINT = 0x00000002;
pub const WGL_SWAP_OVERLAY2: UINT = 0x00000004;
pub const WGL_SWAP_OVERLAY3: UINT = 0x00000008;
pub const WGL_SWAP_OVERLAY4: UINT = 0x00000010;
pub const WGL_SWAP_OVERLAY5: UINT = 0x00000020;
pub const WGL_SWAP_OVERLAY6: UINT = 0x00000040;
pub const WGL_SWAP_OVERLAY7: UINT = 0x00000080;
pub const WGL_SWAP_OVERLAY8: UINT = 0x00000100;
pub const WGL_SWAP_OVERLAY9: UINT = 0x00000200;
pub const WGL_SWAP_OVERLAY10: UINT = 0x00000400;
pub const WGL_SWAP_OVERLAY11: UINT = 0x00000800;
pub const WGL_SWAP_OVERLAY12: UINT = 0x00001000;
pub const WGL_SWAP_OVERLAY13: UINT = 0x00002000;
pub const WGL_SWAP_OVERLAY14: UINT = 0x00004000;
pub const WGL_SWAP_OVERLAY15: UINT = 0x00008000;
pub const WGL_SWAP_UNDERLAY1: UINT = 0x00010000;
pub const WGL_SWAP_UNDERLAY2: UINT = 0x00020000;
pub const WGL_SWAP_UNDERLAY3: UINT = 0x00040000;
pub const WGL_SWAP_UNDERLAY4: UINT = 0x00080000;
pub const WGL_SWAP_UNDERLAY5: UINT = 0x00100000;
pub const WGL_SWAP_UNDERLAY6: UINT = 0x00200000;
pub const WGL_SWAP_UNDERLAY7: UINT = 0x00400000;
pub const WGL_SWAP_UNDERLAY8: UINT = 0x00800000;
pub const WGL_SWAP_UNDERLAY9: UINT = 0x01000000;
pub const WGL_SWAP_UNDERLAY10: UINT = 0x02000000;
pub const WGL_SWAP_UNDERLAY11: UINT = 0x04000000;
pub const WGL_SWAP_UNDERLAY12: UINT = 0x08000000;
pub const WGL_SWAP_UNDERLAY13: UINT = 0x10000000;
pub const WGL_SWAP_UNDERLAY14: UINT = 0x20000000;
pub const WGL_SWAP_UNDERLAY15: UINT = 0x40000000;
extern "system" {
    pub fn wglDescribeLayerPlane(
        hdc: HDC,
        iPixelFormat: c_int,
        iLayerPlane: c_int,
        nBytes: UINT,
        plpd: LPLAYERPLANEDESCRIPTOR,
    ) -> BOOL;
    pub fn wglSetLayerPaletteEntries(
        hdc: HDC,
        iLayerPlane: c_int,
        iStart: c_int,
        cEntries: c_int,
        pcr: *const COLORREF,
    ) -> c_int;
    pub fn wglGetLayerPaletteEntries(
        hdc: HDC,
        iLayerPlane: c_int,
        iStart: c_int,
        cEntries: c_int,
        pcr: *const COLORREF,
    ) -> c_int;
    pub fn wglRealizeLayerPalette(
        hdc: HDC,
        iLayerPlane: c_int,
        bRealize: BOOL,
    ) -> BOOL;
    pub fn wglSwapLayerBuffers(
        hdc: HDC,
        fuPlanes: UINT,
    ) -> BOOL;
}
STRUCT!{struct WGLSWAP {
    hdc: HDC,
    uiFlags: UINT,
}}
pub type PWGLSWAP = *mut WGLSWAP;
pub type LPWGLSWAP = *mut WGLSWAP;
pub const WGL_SWAPMULTIPLE_MAX: usize = 16;
extern "system" {
    pub fn wglSwapMultipleBuffers(
        n: UINT,
        ps: *const WGLSWAP,
    ) -> DWORD;
}
