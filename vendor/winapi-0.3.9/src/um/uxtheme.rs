// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! theming API
use ctypes::{c_float, c_int, c_void};
use shared::minwindef::{BOOL, BYTE, DWORD, HINSTANCE, HRGN, LPARAM, UINT, ULONG, WORD};
use shared::windef::{COLORREF, HBITMAP, HBRUSH, HDC, HWND, LPCRECT, LPRECT, POINT, RECT, SIZE};
use um::commctrl::HIMAGELIST;
use um::wingdi::{BLENDFUNCTION, LOGFONTW, RGBQUAD, TEXTMETRICW};
use um::winnt::{HANDLE, HRESULT, LONG, LPCWSTR, LPWSTR, PVOID, VOID};
pub type HTHEME = HANDLE;
//pub const MAX_THEMECOLOR: u32 = 64;
//pub const MAX_THEMESIZE: u32 = 64;
extern "system" {
    pub fn BeginPanningFeedback(
        hwnd: HWND,
    ) -> BOOL;
    pub fn UpdatePanningFeedback(
        hwnd: HWND,
        lTotalOverpanOffsetX: LONG,
        lTotalOverpanOffsetY: LONG,
        fInInertia: BOOL,
    ) -> BOOL;
    pub fn EndPanningFeedback(
        hwnd: HWND,
        fAnimateBack: BOOL,
    ) -> BOOL;
}
ENUM!{enum TA_PROPERTY {
    TAP_FLAGS = 0,
    TAP_TRANSFORMCOUNT = 1,
    TAP_STAGGERDELAY = 2,
    TAP_STAGGERDELAYCAP = 3,
    TAP_STAGGERDELAYFACTOR = 4,
    TAP_ZORDER = 5,
}}
ENUM!{enum TA_PROPERTY_FLAG {
    TAPF_NONE = 0x0,
    TAPF_HASSTAGGER = 0x1,
    TAPF_ISRTLAWARE = 0x2,
    TAPF_ALLOWCOLLECTION = 0x4,
    TAPF_HASBACKGROUND = 0x8,
    TAPF_HASPERSPECTIVE = 0x10,
}}
extern "system" {
    pub fn GetThemeAnimationProperty(
        hTheme: HTHEME,
        iStoryboardId: c_int,
        iTargetId: c_int,
        eProperty: TA_PROPERTY,
        pvProperty: *mut VOID,
        cbSize: DWORD,
        pcbSizeOut: *mut DWORD,
    ) -> HRESULT;
}
ENUM!{enum TA_TRANSFORM_TYPE {
    TATT_TRANSLATE_2D = 0,
    TATT_SCALE_2D = 1,
    TATT_OPACITY = 2,
    TATT_CLIP = 3,
}}
ENUM!{enum TA_TRANSFORM_FLAG {
    TATF_NONE = 0x0,
    TATF_TARGETVALUES_USER = 0x1,
    TATF_HASINITIALVALUES = 0x2,
    TATF_HASORIGINVALUES = 0x4,
}}
STRUCT!{struct TA_TRANSFORM {
    eTransformType: TA_TRANSFORM_TYPE,
    dwTimingFunctionId: DWORD,
    dwStartTime: DWORD,
    dwDurationTime: DWORD,
    eFlags: TA_TRANSFORM_FLAG,
}}
pub type PTA_TRANSFORM = *mut TA_TRANSFORM;
STRUCT!{struct TA_TRANSFORM_2D {
    header: TA_TRANSFORM,
    rX: c_float,
    rY: c_float,
    rInitialX: c_float,
    rInitialY: c_float,
    rOriginX: c_float,
    rOriginY: c_float,
}}
pub type PTA_TRANSFORM_2D = *mut TA_TRANSFORM_2D;
STRUCT!{struct TA_TRANSFORM_OPACITY {
    header: TA_TRANSFORM,
    rOpacity: c_float,
    rInitialOpacity: c_float,
}}
pub type PTA_TRANSFORM_OPACITY = *mut TA_TRANSFORM_OPACITY;
STRUCT!{struct TA_TRANSFORM_CLIP {
    header: TA_TRANSFORM,
    rLeft: c_float,
    rTop: c_float,
    rRight: c_float,
    rBottom: c_float,
    rInitialLeft: c_float,
    rInitialTop: c_float,
    rInitialRight: c_float,
    rInitialBottom: c_float,
}}
pub type PTA_TRANSFORM_CLIP = *mut TA_TRANSFORM_CLIP;
extern "system" {
    pub fn GetThemeAnimationTransform(
        hTheme: HTHEME,
        iStoryboardId: c_int,
        iTargetId: c_int,
        dwTransformIndex: DWORD,
        pTransform: *mut TA_TRANSFORM,
        cbSize: DWORD,
        pcbSizeOut: *mut DWORD,
    ) -> HRESULT;
}
ENUM!{enum TA_TIMINGFUNCTION_TYPE {
    TTFT_UNDEFINED = 0,
    TTFT_CUBIC_BEZIER = 1,
}}
STRUCT!{struct TA_TIMINGFUNCTION {
    eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}}
pub type PTA_TIMINGFUNCTION = *mut TA_TIMINGFUNCTION;
STRUCT!{struct TA_CUBIC_BEZIER {
    header: TA_TIMINGFUNCTION,
    rX0: c_float,
    rY0: c_float,
    rX1: c_float,
    rY1: c_float,
}}
pub type PTA_CUBIC_BEZIER = *mut TA_CUBIC_BEZIER;
extern "system" {
    pub fn GetThemeTimingFunction(
        hTheme: HTHEME,
        iTimingFunctionId: c_int,
        pTimingFunction: *mut TA_TIMINGFUNCTION,
        cbSize: DWORD,
        pcbSizeOut: *mut DWORD,
    ) -> HRESULT;
    pub fn OpenThemeData(
        hwnd: HWND,
        pszClassList: LPCWSTR,
    ) -> HTHEME;
}
pub const OTD_FORCE_RECT_SIZING: DWORD = 0x00000001;
pub const OTD_NONCLIENT: DWORD = 0x00000002;
pub const OTD_VALIDBITS: DWORD = OTD_FORCE_RECT_SIZING | OTD_NONCLIENT;
extern "system" {
    pub fn OpenThemeDataForDpi(
        hwnd: HWND,
        pszClassList: LPCWSTR,
        dpi: UINT,
    ) -> HTHEME;
    pub fn OpenThemeDataEx(
        hwnd: HWND,
        pszClassList: LPCWSTR,
        dwFlags: DWORD,
    ) -> HTHEME;
    pub fn CloseThemeData(
        hTheme: HTHEME,
    ) -> HRESULT;
    pub fn DrawThemeBackground(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pRect: LPCRECT,
        pClipRect: LPCRECT,
    ) -> HRESULT;
}
pub const DTBG_CLIPRECT: DWORD = 0x00000001;
pub const DTBG_DRAWSOLID: DWORD = 0x00000002;
pub const DTBG_OMITBORDER: DWORD = 0x00000004;
pub const DTBG_OMITCONTENT: DWORD = 0x00000008;
pub const DTBG_COMPUTINGREGION: DWORD = 0x00000010;
pub const DTBG_MIRRORDC: DWORD = 0x00000020;
pub const DTBG_NOMIRROR: DWORD = 0x00000040;
pub const DTBG_VALIDBITS: DWORD = DTBG_CLIPRECT | DTBG_DRAWSOLID | DTBG_OMITBORDER
    | DTBG_OMITCONTENT | DTBG_COMPUTINGREGION | DTBG_MIRRORDC | DTBG_NOMIRROR;
STRUCT!{struct DTBGOPTS {
    dwSize: DWORD,
    dwFlags: DWORD,
    rcClip: RECT,
}}
pub type PDTBGOPTS = *mut DTBGOPTS;
extern "system" {
    pub fn DrawThemeBackgroundEx(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pRect: LPCRECT,
        pOptions: *const DTBGOPTS,
    ) -> HRESULT;
}
//pub const DTT_GRAYED: u32 = 0x00000001;
//pub const DTT_FLAGS2VALIDBITS: u32 = DTT_GRAYED;
extern "system" {
    pub fn DrawThemeText(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pszText: LPCWSTR,
        cchText: c_int,
        dwTextFlags: DWORD,
        dwTextFlags2: DWORD,
        pRect: LPCRECT,
    ) -> HRESULT;
    pub fn GetThemeBackgroundContentRect(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pBoundingRect: LPCRECT,
        pContentRect: LPRECT,
    ) -> HRESULT;
    pub fn GetThemeBackgroundExtent(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pContentRect: LPCRECT,
        pExtentRect: LPRECT,
    ) -> HRESULT;
    pub fn GetThemeBackgroundRegion(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pRect: LPCRECT,
        pRegion: *mut HRGN,
    ) -> HRESULT;
}
ENUM!{enum THEMESIZE {
    TS_MIN = 0,
    TS_TRUE = 1,
    TS_DRAW = 2,
}}
extern "system" {
    pub fn GetThemePartSize(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        prc: LPCRECT,
        eSize: THEMESIZE,
        psz: *mut SIZE,
    ) -> HRESULT;
    pub fn GetThemeTextExtent(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pszText: LPCWSTR,
        cchCharCount: c_int,
        dwTextFlags: DWORD,
        pBoundingRect: LPCRECT,
        pExtentRect: LPRECT,
    ) -> HRESULT;
    pub fn GetThemeTextMetrics(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        ptm: *mut TEXTMETRICW,
    ) -> HRESULT;
}
pub const HTTB_BACKGROUNDSEG: DWORD = 0x00000000;
pub const HTTB_FIXEDBORDER: DWORD = 0x00000002;
pub const HTTB_CAPTION: DWORD = 0x00000004;
pub const HTTB_RESIZINGBORDER_LEFT: DWORD = 0x00000010;
pub const HTTB_RESIZINGBORDER_TOP: DWORD = 0x00000020;
pub const HTTB_RESIZINGBORDER_RIGHT: DWORD = 0x00000040;
pub const HTTB_RESIZINGBORDER_BOTTOM: DWORD = 0x00000080;
pub const HTTB_RESIZINGBORDER: DWORD = HTTB_RESIZINGBORDER_LEFT | HTTB_RESIZINGBORDER_TOP
    | HTTB_RESIZINGBORDER_RIGHT | HTTB_RESIZINGBORDER_BOTTOM;
pub const HTTB_SIZINGTEMPLATE: DWORD = 0x00000100;
pub const HTTB_SYSTEMSIZINGMARGINS: DWORD = 0x00000200;
extern "system" {
    pub fn HitTestThemeBackground(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        dwOptions: DWORD,
        pRect: LPCRECT,
        hrgn: HRGN,
        ptTest: POINT,
        pwHitTestCode: *mut WORD,
    ) -> HRESULT;
    pub fn DrawThemeEdge(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pDestRect: LPCRECT,
        uEdge: UINT,
        uFlags: UINT,
        pContentRect: LPRECT,
    ) -> HRESULT;
    pub fn DrawThemeIcon(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pRect: LPCRECT,
        himl: HIMAGELIST,
        iImageIndex: c_int,
    ) -> HRESULT;
    pub fn IsThemePartDefined(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
    ) -> BOOL;
    pub fn IsThemeBackgroundPartiallyTransparent(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
    ) -> BOOL;
    pub fn GetThemeColor(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pColor: *mut COLORREF,
    ) -> HRESULT;
    pub fn GetThemeMetric(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        piVal: *mut c_int,
    ) -> HRESULT;
    pub fn GetThemeString(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pszBuff: LPWSTR,
        cchMaxBuffChars: c_int,
    ) -> HRESULT;
    pub fn GetThemeBool(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pfVal: *mut BOOL,
    ) -> HRESULT;
    pub fn GetThemeInt(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        piVal: *mut c_int,
    ) -> HRESULT;
    pub fn GetThemeEnumValue(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        piVal: *mut c_int,
    ) -> HRESULT;
    pub fn GetThemePosition(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pPoint: *mut POINT,
    ) -> HRESULT;
    pub fn GetThemeFont(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pFont: *mut LOGFONTW,
    ) -> HRESULT;
    pub fn GetThemeRect(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pRect: LPRECT,
    ) -> HRESULT;
}
STRUCT!{struct MARGINS {
    cxLeftWidth: c_int,
    cxRightWidth: c_int,
    cyTopHeight: c_int,
    cyBottomHeight: c_int,
}}
pub type PMARGINS = *mut MARGINS;
extern "system" {
    pub fn GetThemeMargins(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        prc: LPCRECT,
        pMargins: *mut MARGINS,
    ) -> HRESULT;
}
pub const MAX_INTLIST_COUNT: usize = 402;
STRUCT!{struct INTLIST {
    iValueCount: c_int,
    iValues: [c_int; MAX_INTLIST_COUNT],
}}
pub type PINTLIST = *mut INTLIST;
extern "system" {
    pub fn GetThemeIntList(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pIntList: *mut INTLIST,
    ) -> HRESULT;
}
ENUM!{enum PROPERTYORIGIN {
    PO_STATE = 0,
    PO_PART = 1,
    PO_CLASS = 2,
    PO_GLOBAL = 3,
    PO_NOTFOUND = 4,
}}
extern "system" {
    pub fn GetThemePropertyOrigin(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pOrigin: *mut PROPERTYORIGIN,
    ) -> HRESULT;
    pub fn SetWindowTheme(
        hwnd: HWND,
        pszSubAppName: LPCWSTR,
        pszSubIdList: LPCWSTR,
    ) -> HRESULT;
    pub fn GetThemeFilename(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        pszThemeFileName: LPWSTR,
        cchMaxBuffChars: c_int,
    ) -> HRESULT;
    pub fn GetThemeSysColor(
        hTheme: HTHEME,
        iColorId: c_int,
    ) -> COLORREF;
    pub fn GetThemeSysColorBrush(
        hTheme: HTHEME,
        iColorId: c_int,
    ) -> HBRUSH;
    pub fn GetThemeSysBool(
        hTheme: HTHEME,
        iBoolId: c_int,
    ) -> BOOL;
    pub fn GetThemeSysSize(
        hTheme: HTHEME,
        iSizeId: c_int,
    ) -> c_int;
    pub fn GetThemeSysFont(
        hTheme: HTHEME,
        iFontId: c_int,
        plf: *mut LOGFONTW,
    ) -> HRESULT;
    pub fn GetThemeSysString(
        hTheme: HTHEME,
        iStringId: c_int,
        pszStringBuff: LPWSTR,
        cchMaxStringChars: c_int,
    ) -> HRESULT;
    pub fn GetThemeSysInt(
        hTheme: HTHEME,
        iIntId: c_int,
        piValue: *mut c_int,
    ) -> HRESULT;
    pub fn IsThemeActive() -> BOOL;
    pub fn IsAppThemed() -> BOOL;
    pub fn GetWindowTheme(
        hwnd: HWND,
    ) -> HTHEME;
}
pub const ETDT_DISABLE: DWORD = 0x00000001;
pub const ETDT_ENABLE: DWORD = 0x00000002;
pub const ETDT_USETABTEXTURE: DWORD = 0x00000004;
pub const ETDT_ENABLETAB: DWORD = ETDT_ENABLE | ETDT_USETABTEXTURE;
pub const ETDT_USEAEROWIZARDTABTEXTURE: DWORD = 0x00000008;
pub const ETDT_ENABLEAEROWIZARDTAB: DWORD = ETDT_ENABLE | ETDT_USEAEROWIZARDTABTEXTURE;
pub const ETDT_VALIDBITS: DWORD = ETDT_DISABLE | ETDT_ENABLE | ETDT_USETABTEXTURE
    | ETDT_USEAEROWIZARDTABTEXTURE;
extern "system" {
    pub fn EnableThemeDialogTexture(
        hwnd: HWND,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn IsThemeDialogTextureEnabled(
        hwnd: HWND,
    ) -> BOOL;
}
pub const STAP_ALLOW_NONCLIENT: DWORD = 1 << 0;
pub const STAP_ALLOW_CONTROLS: DWORD = 1 << 1;
pub const STAP_ALLOW_WEBCONTENT: DWORD = 1 << 2;
pub const STAP_VALIDBITS: DWORD = STAP_ALLOW_NONCLIENT | STAP_ALLOW_CONTROLS
    | STAP_ALLOW_WEBCONTENT;
extern "system" {
    pub fn GetThemeAppProperties() -> DWORD;
    pub fn SetThemeAppProperties(
        dwFlags: DWORD,
    );
    pub fn GetCurrentThemeName(
        pszThemeFileName: LPWSTR,
        cchMaxNameChars: c_int,
        pszColorBuff: LPWSTR,
        cchMaxColorChars: c_int,
        pszSizeBuff: LPWSTR,
        cchMaxSizeChars: c_int,
    ) -> HRESULT;
}
pub const SZ_THDOCPROP_DISPLAYNAME: &'static str = "DisplayName";
pub const SZ_THDOCPROP_CANONICALNAME: &'static str = "ThemeName";
pub const SZ_THDOCPROP_TOOLTIP: &'static str = "ToolTip";
pub const SZ_THDOCPROP_AUTHOR: &'static str = "author";
extern "system" {
    pub fn GetThemeDocumentationProperty(
        pszThemeName: LPCWSTR,
        pszPropertyName: LPCWSTR,
        pszValueBuff: LPWSTR,
        cchMaxValChars: c_int,
    ) -> HRESULT;
    pub fn DrawThemeParentBackground(
        hwnd: HWND,
        hdc: HDC,
        prc: *const RECT,
    ) -> HRESULT;
    pub fn EnableTheming(
        fEnable: BOOL,
    ) -> HRESULT;
}
pub const GBF_DIRECT: ULONG = 0x00000001;
pub const GBF_COPY: ULONG = 0x00000002;
pub const GBF_VALIDBITS: ULONG = GBF_DIRECT | GBF_COPY;
pub const DTPB_WINDOWDC: DWORD = 0x00000001;
pub const DTPB_USECTLCOLORSTATIC: DWORD = 0x00000002;
pub const DTPB_USEERASEBKGND: DWORD = 0x00000004;
extern "system" {
    pub fn DrawThemeParentBackgroundEx(
        hwnd: HWND,
        hdc: HDC,
        dwFlags: DWORD,
        prc: *const RECT,
    ) -> HRESULT;
}
ENUM!{enum WINDOWTHEMEATTRIBUTETYPE {
    WTA_NONCLIENT = 1,
}}
STRUCT!{struct WTA_OPTIONS {
    dwFlags: DWORD,
    dwMask: DWORD,
}}
pub type PWTA_OPTIONS = *mut WTA_OPTIONS;
pub const WTNCA_NODRAWCAPTION: DWORD = 0x00000001;
pub const WTNCA_NODRAWICON: DWORD = 0x00000002;
pub const WTNCA_NOSYSMENU: DWORD = 0x00000004;
pub const WTNCA_NOMIRRORHELP: DWORD = 0x00000008;
pub const WTNCA_VALIDBITS: DWORD = WTNCA_NODRAWCAPTION | WTNCA_NODRAWICON | WTNCA_NOSYSMENU
    | WTNCA_NOMIRRORHELP;
extern "system" {
    pub fn SetWindowThemeAttribute(
        hwnd: HWND,
        eAttribute: WINDOWTHEMEATTRIBUTETYPE,
        pvAttribute: PVOID,
        cbAttribute: DWORD,
    ) -> HRESULT;
}
#[inline]
pub unsafe fn SetWindowThemeNonClientAttributes(
    hwnd: HWND,
    dwMask: DWORD,
    dwAttributes: DWORD,
) -> HRESULT {
    use core::mem::{size_of, zeroed};
    let mut wta: WTA_OPTIONS = zeroed();
    wta.dwFlags = dwAttributes;
    wta.dwMask = dwMask;
    SetWindowThemeAttribute(
        hwnd,
        WTA_NONCLIENT,
        &mut wta as *mut WTA_OPTIONS as *mut c_void,
        size_of::<WTA_OPTIONS>() as u32,
    )
}
FN!{stdcall DTT_CALLBACK_PROC(
    hdc: HDC,
    pszText: LPWSTR,
    cchText: c_int,
    prc: LPRECT,
    dwFlags: UINT,
    lParam: LPARAM,
) -> c_int}
pub const DTT_TEXTCOLOR: DWORD = 1 << 0;
pub const DTT_BORDERCOLOR: DWORD = 1 << 1;
pub const DTT_SHADOWCOLOR: DWORD = 1 << 2;
pub const DTT_SHADOWTYPE: DWORD = 1 << 3;
pub const DTT_SHADOWOFFSET: DWORD = 1 << 4;
pub const DTT_BORDERSIZE: DWORD = 1 << 5;
pub const DTT_FONTPROP: DWORD = 1 << 6;
pub const DTT_COLORPROP: DWORD = 1 << 7;
pub const DTT_STATEID: DWORD = 1 << 8;
pub const DTT_CALCRECT: DWORD = 1 << 9;
pub const DTT_APPLYOVERLAY: DWORD = 1 << 10;
pub const DTT_GLOWSIZE: DWORD = 1 << 11;
pub const DTT_CALLBACK: DWORD = 1 << 12;
pub const DTT_COMPOSITED: DWORD = 1 << 13;
pub const DTT_VALIDBITS: DWORD = DTT_TEXTCOLOR | DTT_BORDERCOLOR | DTT_SHADOWCOLOR
    | DTT_SHADOWTYPE | DTT_SHADOWOFFSET | DTT_BORDERSIZE | DTT_FONTPROP | DTT_COLORPROP
    | DTT_STATEID | DTT_CALCRECT | DTT_APPLYOVERLAY | DTT_GLOWSIZE | DTT_COMPOSITED;
STRUCT!{struct DTTOPTS {
    dwSize: DWORD,
    dwFlags: DWORD,
    crText: COLORREF,
    crBorder: COLORREF,
    crShadow: COLORREF,
    iTextShadowType: c_int,
    ptShadowOffset: POINT,
    iBorderSize: c_int,
    iFontPropId: c_int,
    iColorPropId: c_int,
    iStateId: c_int,
    fApplyOverlay: BOOL,
    iGlowSize: c_int,
    pfnDrawTextCallback: DTT_CALLBACK_PROC,
    lParam: LPARAM,
}}
pub type PDTTOPTS = *mut DTTOPTS;
extern "system" {
    pub fn DrawThemeTextEx(
        hTheme: HTHEME,
        hdc: HDC,
        iPartId: c_int,
        iStateId: c_int,
        pszText: LPCWSTR,
        cchText: c_int,
        dwTextFlags: DWORD,
        pRect: LPRECT,
        pOptions: *const DTTOPTS,
    ) -> HRESULT;
    pub fn GetThemeBitmap(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        dwFlags: ULONG,
        phBitmap: *mut HBITMAP,
    ) -> HRESULT;
    pub fn GetThemeStream(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateId: c_int,
        iPropId: c_int,
        ppvStream: *mut *mut VOID,
        pcbStream: *mut DWORD,
        hInst: HINSTANCE,
    ) -> HRESULT;
    pub fn BufferedPaintInit() -> HRESULT;
    pub fn BufferedPaintUnInit() -> HRESULT;
}
pub type HPAINTBUFFER = HANDLE;
ENUM!{enum BP_BUFFERFORMAT {
    BPBF_COMPATIBLEBITMAP = 0,
    BPBF_DIB = 1,
    BPBF_TOPDOWNDIB = 2,
    BPBF_TOPDOWNMONODIB = 3,
}}
pub const BPBF_COMPOSITED: BP_BUFFERFORMAT = BPBF_TOPDOWNDIB;
ENUM!{enum BP_ANIMATIONSTYLE {
    BPAS_NONE = 0,
    BPAS_LINEAR = 1,
    BPAS_CUBIC = 2,
    BPAS_SINE = 3,
}}
STRUCT!{struct BP_ANIMATIONPARAMS {
    cbSize: DWORD,
    dwFlags: DWORD,
    style: BP_ANIMATIONSTYLE,
    dwDuration: DWORD,
}}
pub type PBP_ANIMATIONPARAMS = *mut BP_ANIMATIONPARAMS;
pub const BPPF_ERASE: DWORD = 0x0001;
pub const BPPF_NOCLIP: DWORD = 0x0002;
pub const BPPF_NONCLIENT: DWORD = 0x0004;
STRUCT!{struct BP_PAINTPARAMS {
    cbSize: DWORD,
    dwFlags: DWORD,
    prcExclude: *const RECT,
    pBlendFunction: *const BLENDFUNCTION,
}}
pub type PBP_PAINTPARAMS = *mut BP_PAINTPARAMS;
extern "system" {
    pub fn BeginBufferedPaint(
        hdcTarget: HDC,
        prcTarget: *const RECT,
        dwFormat: BP_BUFFERFORMAT,
        pPaintParams: *mut BP_PAINTPARAMS,
        phdc: *mut HDC,
    ) -> HPAINTBUFFER;
    pub fn EndBufferedPaint(
        hBufferedPaint: HPAINTBUFFER,
        fUpdateTarget: BOOL,
    ) -> HRESULT;
    pub fn GetBufferedPaintTargetRect(
        hBufferedPaint: HPAINTBUFFER,
        prc: *mut RECT,
    ) -> HRESULT;
    pub fn GetBufferedPaintTargetDC(
        hBufferedPaint: HPAINTBUFFER,
    ) -> HDC;
    pub fn GetBufferedPaintDC(
        hBufferedPaint: HPAINTBUFFER,
    ) -> HDC;
    pub fn GetBufferedPaintBits(
        hBufferedPaint: HPAINTBUFFER,
        ppbBuffer: *mut *mut RGBQUAD,
        pcxRow: *mut c_int,
    ) -> HRESULT;
    pub fn BufferedPaintClear(
        hBufferedPaint: HPAINTBUFFER,
        prc: *const RECT,
    ) -> HRESULT;
    pub fn BufferedPaintSetAlpha(
        hBufferedPaint: HPAINTBUFFER,
        prc: *const RECT,
        alpha: BYTE,
    ) -> HRESULT;
    pub fn BufferedPaintStopAllAnimations(
        hwnd: HWND,
    ) -> HRESULT;
}
pub type HANIMATIONBUFFER = HANDLE;
extern "system" {
    pub fn BeginBufferedAnimation(
        hwnd: HWND,
        hdcTarget: HDC,
        prcTarget: *const RECT,
        dwFormat: BP_BUFFERFORMAT,
        pPaintParams: *mut BP_PAINTPARAMS,
        pAnimationParams: *mut BP_ANIMATIONPARAMS,
        phdcFrom: *mut HDC,
        phdcTo: *mut HDC,
    ) -> HANIMATIONBUFFER;
    pub fn EndBufferedAnimation(
        hbpAnimation: HANIMATIONBUFFER,
        fUpdateTarget: BOOL,
    ) -> HRESULT;
    pub fn BufferedPaintRenderAnimation(
        hwnd: HWND,
        hdcTarget: HDC,
    ) -> BOOL;
    pub fn IsCompositionActive() -> BOOL;
    pub fn GetThemeTransitionDuration(
        hTheme: HTHEME,
        iPartId: c_int,
        iStateIdFrom: c_int,
        iStateIdTo: c_int,
        iPropId: c_int,
        pdwDuration: *mut DWORD,
    ) -> HRESULT;
}
