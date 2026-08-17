// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! 32-Bit Common Dialog APIs
use ctypes::{c_short, c_void};
use shared::basetsd::UINT_PTR;
use shared::minwindef::{
    BOOL, DWORD, HGLOBAL, HINSTANCE, INT, LPARAM, LPVOID, LRESULT, UINT, WORD, WPARAM,
};
use shared::windef::{COLORREF, HDC, HWND, POINT, RECT};
use um::prsht::HPROPSHEETPAGE;
use um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};
use um::wingdi::{DM_COLLATE, DM_COPIES, LPDEVMODEW, LPLOGFONTA, LPLOGFONTW};
use um::winnt::{HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR};
use um::winuser::{NMHDR, WM_USER};
FN!{stdcall LPOFNHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPENFILENAME_NT4A {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpstrFilter: LPCSTR,
    lpstrCustomFilter: LPSTR,
    nMaxCustFilter: DWORD,
    nFilterIndex: DWORD,
    lpstrFile: LPSTR,
    nMaxFile: DWORD,
    lpstrFileTitle: LPSTR,
    nMaxFileTitle: DWORD,
    lpstrInitialDir: LPCSTR,
    lpstrTitle: LPCSTR,
    Flags: DWORD,
    nFileOffset: WORD,
    nFileExtension: WORD,
    lpstrDefExt: LPCSTR,
    lCustData: LPARAM,
    lpfnHook: LPOFNHOOKPROC,
    lpTemplateName: LPCSTR,
}}
pub type LPOPENFILENAME_NT4A = *mut OPENFILENAME_NT4A;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPENFILENAME_NT4W {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpstrFilter: LPCWSTR,
    lpstrCustomFilter: LPWSTR,
    nMaxCustFilter: DWORD,
    nFilterIndex: DWORD,
    lpstrFile: LPWSTR,
    nMaxFile: DWORD,
    lpstrFileTitle: LPWSTR,
    nMaxFileTitle: DWORD,
    lpstrInitialDir: LPCWSTR,
    lpstrTitle: LPCWSTR,
    Flags: DWORD,
    nFileOffset: WORD,
    nFileExtension: WORD,
    lpstrDefExt: LPCWSTR,
    lCustData: LPARAM,
    lpfnHook: LPOFNHOOKPROC,
    lpTemplateName: LPCWSTR,
}}
pub type LPOPENFILENAME_NT4W = *mut OPENFILENAME_NT4W;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPENFILENAMEA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpstrFilter: LPCSTR,
    lpstrCustomFilter: LPSTR,
    nMaxCustFilter: DWORD,
    nFilterIndex: DWORD,
    lpstrFile: LPSTR,
    nMaxFile: DWORD,
    lpstrFileTitle: LPSTR,
    nMaxFileTitle: DWORD,
    lpstrInitialDir: LPCSTR,
    lpstrTitle: LPCSTR,
    Flags: DWORD,
    nFileOffset: WORD,
    nFileExtension: WORD,
    lpstrDefExt: LPCSTR,
    lCustData: LPARAM,
    lpfnHook: LPOFNHOOKPROC,
    lpTemplateName: LPCSTR,
    pvReserved: *mut c_void,
    dwReserved: DWORD,
    FlagsEx: DWORD,
}}
pub type LPOPENFILENAMEA = *mut OPENFILENAMEA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPENFILENAMEW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    lpstrFilter: LPCWSTR,
    lpstrCustomFilter: LPWSTR,
    nMaxCustFilter: DWORD,
    nFilterIndex: DWORD,
    lpstrFile: LPWSTR,
    nMaxFile: DWORD,
    lpstrFileTitle: LPWSTR,
    nMaxFileTitle: DWORD,
    lpstrInitialDir: LPCWSTR,
    lpstrTitle: LPCWSTR,
    Flags: DWORD,
    nFileOffset: WORD,
    nFileExtension: WORD,
    lpstrDefExt: LPCWSTR,
    lCustData: LPARAM,
    lpfnHook: LPOFNHOOKPROC,
    lpTemplateName: LPCWSTR,
    pvReserved: *mut c_void,
    dwReserved: DWORD,
    FlagsEx: DWORD,
}}
pub type LPOPENFILENAMEW = *mut OPENFILENAMEW;
extern "system" {
    pub fn GetOpenFileNameA(
        lpofn: LPOPENFILENAMEA,
    ) -> BOOL;
    pub fn GetOpenFileNameW(
        lpofn: LPOPENFILENAMEW,
    ) -> BOOL;
    pub fn GetSaveFileNameA(
        lpofn: LPOPENFILENAMEA,
    ) -> BOOL;
    pub fn GetSaveFileNameW(
        lpofn: LPOPENFILENAMEW,
    ) -> BOOL;
    pub fn GetFileTitleA(
        lpszFile: LPCSTR,
        Buf: LPSTR,
        cchSize: WORD,
    ) -> c_short;
    pub fn GetFileTitleW(
        lpszFile: LPCWSTR,
        Buf: LPWSTR,
        cchSize: WORD,
    ) -> c_short;
}
pub const OFN_READONLY: DWORD = 0x00000001;
pub const OFN_OVERWRITEPROMPT: DWORD = 0x00000002;
pub const OFN_HIDEREADONLY: DWORD = 0x00000004;
pub const OFN_NOCHANGEDIR: DWORD = 0x00000008;
pub const OFN_SHOWHELP: DWORD = 0x00000010;
pub const OFN_ENABLEHOOK: DWORD = 0x00000020;
pub const OFN_ENABLETEMPLATE: DWORD = 0x00000040;
pub const OFN_ENABLETEMPLATEHANDLE: DWORD = 0x00000080;
pub const OFN_NOVALIDATE: DWORD = 0x00000100;
pub const OFN_ALLOWMULTISELECT: DWORD = 0x00000200;
pub const OFN_EXTENSIONDIFFERENT: DWORD = 0x00000400;
pub const OFN_PATHMUSTEXIST: DWORD = 0x00000800;
pub const OFN_FILEMUSTEXIST: DWORD = 0x00001000;
pub const OFN_CREATEPROMPT: DWORD = 0x00002000;
pub const OFN_SHAREAWARE: DWORD = 0x00004000;
pub const OFN_NOREADONLYRETURN: DWORD = 0x00008000;
pub const OFN_NOTESTFILECREATE: DWORD = 0x00010000;
pub const OFN_NONETWORKBUTTON: DWORD = 0x00020000;
pub const OFN_NOLONGNAMES: DWORD = 0x00040000;
pub const OFN_EXPLORER: DWORD = 0x00080000;
pub const OFN_NODEREFERENCELINKS: DWORD = 0x00100000;
pub const OFN_LONGNAMES: DWORD = 0x00200000;
pub const OFN_ENABLEINCLUDENOTIFY: DWORD = 0x00400000;
pub const OFN_ENABLESIZING: DWORD = 0x00800000;
pub const OFN_DONTADDTORECENT: DWORD = 0x02000000;
pub const OFN_FORCESHOWHIDDEN: DWORD = 0x10000000;
pub const OFN_EX_NOPLACESBAR: DWORD = 0x00000001;
pub const OFN_SHAREFALLTHROUGH: UINT_PTR = 2;
pub const OFN_SHARENOWARN: UINT_PTR = 1;
pub const OFN_SHAREWARN: UINT_PTR = 0;
FN!{stdcall LPCCHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OFNOTIFYA {
    hdr: NMHDR,
    lpOFN: LPOPENFILENAMEA,
    pszFile: LPSTR,
}}
pub type LPOFNOTIFYA = *mut OFNOTIFYA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OFNOTIFYW {
    hdr: NMHDR,
    lpOFN: LPOPENFILENAMEW,
    pszFile: LPWSTR,
}}
pub type LPOFNOTIFYW = *mut OFNOTIFYW;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OFNOTIFYEXA {
    hdr: NMHDR,
    lpOFN: LPOPENFILENAMEA,
    psf: LPVOID,
    pidl: LPVOID,
}}
pub type LPOFNOTIFYEXA = *mut OFNOTIFYEXA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OFNOTIFYEXW {
    hdr: NMHDR,
    lpOFN: LPOPENFILENAMEW,
    psf: LPVOID,
    pidl: LPVOID,
}}
pub type LPOFNOTIFYEXW = *mut OFNOTIFYEXW;
pub const CDN_FIRST: UINT = -601i32 as u32;
pub const CDN_LAST: UINT = -699i32 as u32;
pub const CDN_INITDONE: UINT = CDN_FIRST - 0x0000;
pub const CDN_SELCHANGE: UINT = CDN_FIRST - 0x0001;
pub const CDN_FOLDERCHANGE: UINT = CDN_FIRST - 0x0002;
pub const CDN_SHAREVIOLATION: UINT = CDN_FIRST - 0x0003;
pub const CDN_HELP: UINT = CDN_FIRST - 0x0004;
pub const CDN_FILEOK: UINT = CDN_FIRST - 0x0005;
pub const CDN_TYPECHANGE: UINT = CDN_FIRST - 0x0006;
pub const CDN_INCLUDEITEM: UINT = CDN_FIRST - 0x0007;
pub const CDM_FIRST: UINT = WM_USER + 100;
pub const CDM_LAST: UINT = WM_USER + 200;
pub const CDM_GETSPEC: UINT = CDM_FIRST + 0x0000;
pub const CDM_GETFILEPATH: UINT = CDM_FIRST + 0x0001;
pub const CDM_GETFOLDERPATH: UINT = CDM_FIRST + 0x0002;
pub const CDM_GETFOLDERIDLIST: UINT = CDM_FIRST + 0x0003;
pub const CDM_SETCONTROLTEXT: UINT = CDM_FIRST + 0x0004;
pub const CDM_HIDECONTROL: UINT = CDM_FIRST + 0x0005;
pub const CDM_SETDEFEXT: UINT = CDM_FIRST + 0x0006;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct CHOOSECOLORA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HWND,
    rgbResult: COLORREF,
    lpCustColors: *mut COLORREF,
    Flags: DWORD,
    lCustData: LPARAM,
    lpfnHook: LPCCHOOKPROC,
    lpTemplateName: LPCSTR,
}}
pub type LPCHOOSECOLORA = *mut CHOOSECOLORA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct CHOOSECOLORW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HWND,
    rgbResult: COLORREF,
    lpCustColors: *mut COLORREF,
    Flags: DWORD,
    lCustData: LPARAM,
    lpfnHook: LPCCHOOKPROC,
    lpTemplateName: LPCWSTR,
}}
pub type LPCHOOSECOLORW = *mut CHOOSECOLORW;
extern "system" {
    pub fn ChooseColorA(
        lpcc: LPCHOOSECOLORA,
    ) -> BOOL;
    pub fn ChooseColorW(
        lpcc: LPCHOOSECOLORW,
    ) -> BOOL;
}
pub const CC_RGBINIT: DWORD = 0x00000001;
pub const CC_FULLOPEN: DWORD = 0x00000002;
pub const CC_PREVENTFULLOPEN: DWORD = 0x00000004;
pub const CC_SHOWHELP: DWORD = 0x00000008;
pub const CC_ENABLEHOOK: DWORD = 0x00000010;
pub const CC_ENABLETEMPLATE: DWORD = 0x00000020;
pub const CC_ENABLETEMPLATEHANDLE: DWORD = 0x00000040;
pub const CC_SOLIDCOLOR: DWORD = 0x00000080;
pub const CC_ANYCOLOR: DWORD = 0x00000100;
FN!{stdcall LPFRHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct FINDREPLACEA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    Flags: DWORD,
    lpstrFindWhat: LPSTR,
    lpstrReplaceWith: LPSTR,
    wFindWhatLen: WORD,
    wReplaceWithLen: WORD,
    lCustData: LPARAM,
    lpfnHook: LPFRHOOKPROC,
    lpTemplateName: LPCSTR,
}}
pub type LPFINDREPLACEA = *mut FINDREPLACEA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct FINDREPLACEW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hInstance: HINSTANCE,
    Flags: DWORD,
    lpstrFindWhat: LPWSTR,
    lpstrReplaceWith: LPWSTR,
    wFindWhatLen: WORD,
    wReplaceWithLen: WORD,
    lCustData: LPARAM,
    lpfnHook: LPFRHOOKPROC,
    lpTemplateName: LPCWSTR,
}}
pub type LPFINDREPLACEW = *mut FINDREPLACEW;
pub const FR_DOWN: DWORD = 0x00000001;
pub const FR_WHOLEWORD: DWORD = 0x00000002;
pub const FR_MATCHCASE: DWORD = 0x00000004;
pub const FR_FINDNEXT: DWORD = 0x00000008;
pub const FR_REPLACE: DWORD = 0x00000010;
pub const FR_REPLACEALL: DWORD = 0x00000020;
pub const FR_DIALOGTERM: DWORD = 0x00000040;
pub const FR_SHOWHELP: DWORD = 0x00000080;
pub const FR_ENABLEHOOK: DWORD = 0x00000100;
pub const FR_ENABLETEMPLATE: DWORD = 0x00000200;
pub const FR_NOUPDOWN: DWORD = 0x00000400;
pub const FR_NOMATCHCASE: DWORD = 0x00000800;
pub const FR_NOWHOLEWORD: DWORD = 0x00001000;
pub const FR_ENABLETEMPLATEHANDLE: DWORD = 0x00002000;
pub const FR_HIDEUPDOWN: DWORD = 0x00004000;
pub const FR_HIDEMATCHCASE: DWORD = 0x00008000;
pub const FR_HIDEWHOLEWORD: DWORD = 0x00010000;
pub const FR_RAW: DWORD = 0x00020000;
pub const FR_MATCHDIAC: DWORD = 0x20000000;
pub const FR_MATCHKASHIDA: DWORD = 0x40000000;
pub const FR_MATCHALEFHAMZA: DWORD = 0x80000000;
extern "system" {
    pub fn FindTextA(
        lpfr: LPFINDREPLACEA,
    ) -> HWND;
    pub fn FindTextW(
        lpfr: LPFINDREPLACEW,
    ) -> HWND;
    pub fn ReplaceTextA(
        lpfr: LPFINDREPLACEA,
    ) -> HWND;
    pub fn ReplaceTextW(
        lpfr: LPFINDREPLACEW,
    ) -> HWND;
}
FN!{stdcall LPCFHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct CHOOSEFONTA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDC: HDC,
    lpLogFont: LPLOGFONTA,
    iPointSize: INT,
    Flags: DWORD,
    rgbColors: COLORREF,
    lCustData: LPARAM,
    lpfnHook: LPCFHOOKPROC,
    lpTemplateName: LPCSTR,
    hInstance: HINSTANCE,
    lpszStyle: LPSTR,
    nFontType: WORD,
    ___MISSING_ALIGNMENT__: WORD,
    nSizeMin: INT,
    nSizeMax: INT,
}}
pub type LPCHOOSEFONTA = *mut CHOOSEFONTA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct CHOOSEFONTW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDC: HDC,
    lpLogFont: LPLOGFONTW,
    iPointSize: INT,
    Flags: DWORD,
    rgbColors: COLORREF,
    lCustData: LPARAM,
    lpfnHook: LPCFHOOKPROC,
    lpTemplateName: LPCWSTR,
    hInstance: HINSTANCE,
    lpszStyle: LPWSTR,
    nFontType: WORD,
    ___MISSING_ALIGNMENT__: WORD,
    nSizeMin: INT,
    nSizeMax: INT,
}}
pub type LPCHOOSEFONTW = *mut CHOOSEFONTW;
extern "system" {
    pub fn ChooseFontA(
        lpcf: LPCHOOSEFONTA,
    ) -> BOOL;
    pub fn ChooseFontW(
        lpcf: LPCHOOSEFONTW,
    ) -> BOOL;
}
pub const CF_SCREENFONTS: DWORD = 0x00000001;
pub const CF_PRINTERFONTS: DWORD = 0x00000002;
pub const CF_BOTH: DWORD = CF_SCREENFONTS | CF_PRINTERFONTS;
pub const CF_SHOWHELP: DWORD = 0x00000004;
pub const CF_ENABLEHOOK: DWORD = 0x00000008;
pub const CF_ENABLETEMPLATE: DWORD = 0x00000010;
pub const CF_ENABLETEMPLATEHANDLE: DWORD = 0x00000020;
pub const CF_INITTOLOGFONTSTRUCT: DWORD = 0x00000040;
pub const CF_USESTYLE: DWORD = 0x00000080;
pub const CF_EFFECTS: DWORD = 0x00000100;
pub const CF_APPLY: DWORD = 0x00000200;
pub const CF_ANSIONLY: DWORD = 0x00000400;
pub const CF_SCRIPTSONLY: DWORD = CF_ANSIONLY;
pub const CF_NOVECTORFONTS: DWORD = 0x00000800;
pub const CF_NOOEMFONTS: DWORD = CF_NOVECTORFONTS;
pub const CF_NOSIMULATIONS: DWORD = 0x00001000;
pub const CF_LIMITSIZE: DWORD = 0x00002000;
pub const CF_FIXEDPITCHONLY: DWORD = 0x00004000;
pub const CF_WYSIWYG: DWORD = 0x00008000;
pub const CF_FORCEFONTEXIST: DWORD = 0x00010000;
pub const CF_SCALABLEONLY: DWORD = 0x00020000;
pub const CF_TTONLY: DWORD = 0x00040000;
pub const CF_NOFACESEL: DWORD = 0x00080000;
pub const CF_NOSTYLESEL: DWORD = 0x00100000;
pub const CF_NOSIZESEL: DWORD = 0x00200000;
pub const CF_SELECTSCRIPT: DWORD = 0x00400000;
pub const CF_NOSCRIPTSEL: DWORD = 0x00800000;
pub const CF_NOVERTFONTS: DWORD = 0x01000000;
pub const CF_INACTIVEFONTS: DWORD = 0x02000000;
pub const SIMULATED_FONTTYPE: WORD = 0x8000;
pub const PRINTER_FONTTYPE: WORD = 0x4000;
pub const SCREEN_FONTTYPE: WORD = 0x2000;
pub const BOLD_FONTTYPE: WORD = 0x0100;
pub const ITALIC_FONTTYPE: WORD = 0x0200;
pub const REGULAR_FONTTYPE: WORD = 0x0400;
pub const PS_OPENTYPE_FONTTYPE: DWORD = 0x10000;
pub const TT_OPENTYPE_FONTTYPE: DWORD = 0x20000;
pub const TYPE1_FONTTYPE: DWORD = 0x40000;
pub const SYMBOL_FONTTYPE: DWORD = 0x80000;
pub const WM_CHOOSEFONT_GETLOGFONT: UINT = WM_USER + 1;
pub const WM_CHOOSEFONT_SETLOGFONT: UINT = WM_USER + 101;
pub const WM_CHOOSEFONT_SETFLAGS: UINT = WM_USER + 102;
pub const CD_LBSELNOITEMS: WORD = -1i16 as u16;
pub const CD_LBSELCHANGE: WORD = 0;
pub const CD_LBSELSUB: WORD = 1;
pub const CD_LBSELADD: WORD = 2;
FN!{stdcall LPPRINTHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
FN!{stdcall LPSETUPHOOKPROC(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PRINTDLGA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    hDC: HDC,
    Flags: DWORD,
    nFromPage: WORD,
    nToPage: WORD,
    nMinPage: WORD,
    nMaxPage: WORD,
    nCopies: WORD,
    hInstance: HINSTANCE,
    lCustData: LPARAM,
    lpfnPrintHook: LPPRINTHOOKPROC,
    lpfnSetupHook: LPSETUPHOOKPROC,
    lpPrintTemplateName: LPCSTR,
    lpSetupTemplateName: LPCSTR,
    hPrintTemplate: HGLOBAL,
    hSetupTemplate: HGLOBAL,
}}
pub type LPPRINTDLGA = *mut PRINTDLGA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PRINTDLGW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    hDC: HDC,
    Flags: DWORD,
    nFromPage: WORD,
    nToPage: WORD,
    nMinPage: WORD,
    nMaxPage: WORD,
    nCopies: WORD,
    hInstance: HINSTANCE,
    lCustData: LPARAM,
    lpfnPrintHook: LPPRINTHOOKPROC,
    lpfnSetupHook: LPSETUPHOOKPROC,
    lpPrintTemplateName: LPCWSTR,
    lpSetupTemplateName: LPCWSTR,
    hPrintTemplate: HGLOBAL,
    hSetupTemplate: HGLOBAL,
}}
pub type LPPRINTDLGW = *mut PRINTDLGW;
extern "system" {
    pub fn PrintDlgA(
        pPD: LPPRINTDLGA,
    ) -> BOOL;
    pub fn PrintDlgW(
        pPD: LPPRINTDLGW,
    ) -> BOOL;
}
RIDL!{#[uuid(0x5852a2c3, 0x6530, 0x11d1, 0xb6, 0xa3, 0x0, 0x0, 0xf8, 0x75, 0x7b, 0xf9)]
interface IPrintDialogCallback(IPrintDialogCallbackVtbl): IUnknown(IUnknownVtbl) {
    fn InitDone() -> HRESULT,
    fn SelectionChange() -> HRESULT,
    fn HandleMessage(
        hDlg: HWND,
        uMsg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
        pResult: *mut LRESULT,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x509aaeda, 0x5639, 0x11d1, 0xb6, 0xa1, 0x0, 0x0, 0xf8, 0x75, 0x7b, 0xf9)]
interface IPrintDialogServices(IPrintDialogServicesVtbl): IUnknown(IUnknownVtbl) {
    fn GetCurrentDevMode(
        pDevMode: LPDEVMODEW,
        pcbSize: *mut UINT,
    ) -> HRESULT,
    fn GetCurrentPrinterName(
        pPrinterName: LPWSTR,
        pcchSize: *mut UINT,
    ) -> HRESULT,
    fn GetCurrentPortName(
        pPortName: LPWSTR,
        pcchSize: *mut UINT,
    ) -> HRESULT,
}}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PRINTPAGERANGE {
    nFromPage: DWORD,
    nToPage: DWORD,
}}
pub type LPPRINTPAGERANGE = *mut PRINTPAGERANGE;
pub type PCPRINTPAGERANGE = *const PRINTPAGERANGE;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PRINTDLGEXA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    hDC: HDC,
    Flags: DWORD,
    Flags2: DWORD,
    ExclusionFlags: DWORD,
    nPageRanges: DWORD,
    nMaxPageRanges: DWORD,
    lpPageRanges: LPPRINTPAGERANGE,
    nMinPage: DWORD,
    nMaxPage: DWORD,
    nCopies: DWORD,
    hInstance: HINSTANCE,
    lpPrintTemplateName: LPCSTR,
    lpCallback: LPUNKNOWN,
    nPropertyPages: DWORD,
    lphPropertyPages: *mut HPROPSHEETPAGE,
    nStartPage: DWORD,
    dwResultAction: DWORD,
}}
pub type LPPRINTDLGEXA = *mut PRINTDLGEXA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PRINTDLGEXW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    hDC: HDC,
    Flags: DWORD,
    Flags2: DWORD,
    ExclusionFlags: DWORD,
    nPageRanges: DWORD,
    nMaxPageRanges: DWORD,
    lpPageRanges: LPPRINTPAGERANGE,
    nMinPage: DWORD,
    nMaxPage: DWORD,
    nCopies: DWORD,
    hInstance: HINSTANCE,
    lpPrintTemplateName: LPCWSTR,
    lpCallback: LPUNKNOWN,
    nPropertyPages: DWORD,
    lphPropertyPages: *mut HPROPSHEETPAGE,
    nStartPage: DWORD,
    dwResultAction: DWORD,
}}
pub type LPPRINTDLGEXW = *mut PRINTDLGEXW;
extern "system" {
    pub fn PrintDlgExA(
        pPD: LPPRINTDLGEXA,
    ) -> HRESULT;
    pub fn PrintDlgExW(
        pPD: LPPRINTDLGEXW,
    ) -> HRESULT;
}
pub const PD_ALLPAGES: DWORD = 0x00000000;
pub const PD_SELECTION: DWORD = 0x00000001;
pub const PD_PAGENUMS: DWORD = 0x00000002;
pub const PD_NOSELECTION: DWORD = 0x00000004;
pub const PD_NOPAGENUMS: DWORD = 0x00000008;
pub const PD_COLLATE: DWORD = 0x00000010;
pub const PD_PRINTTOFILE: DWORD = 0x00000020;
pub const PD_PRINTSETUP: DWORD = 0x00000040;
pub const PD_NOWARNING: DWORD = 0x00000080;
pub const PD_RETURNDC: DWORD = 0x00000100;
pub const PD_RETURNIC: DWORD = 0x00000200;
pub const PD_RETURNDEFAULT: DWORD = 0x00000400;
pub const PD_SHOWHELP: DWORD = 0x00000800;
pub const PD_ENABLEPRINTHOOK: DWORD = 0x00001000;
pub const PD_ENABLESETUPHOOK: DWORD = 0x00002000;
pub const PD_ENABLEPRINTTEMPLATE: DWORD = 0x00004000;
pub const PD_ENABLESETUPTEMPLATE: DWORD = 0x00008000;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: DWORD = 0x00010000;
pub const PD_ENABLESETUPTEMPLATEHANDLE: DWORD = 0x00020000;
pub const PD_USEDEVMODECOPIES: DWORD = 0x00040000;
pub const PD_USEDEVMODECOPIESANDCOLLATE: DWORD = 0x00040000;
pub const PD_DISABLEPRINTTOFILE: DWORD = 0x00080000;
pub const PD_HIDEPRINTTOFILE: DWORD = 0x00100000;
pub const PD_NONETWORKBUTTON: DWORD = 0x00200000;
pub const PD_CURRENTPAGE: DWORD = 0x00400000;
pub const PD_NOCURRENTPAGE: DWORD = 0x00800000;
pub const PD_EXCLUSIONFLAGS: DWORD = 0x01000000;
pub const PD_USELARGETEMPLATE: DWORD = 0x10000000;
pub const PD_EXCL_COPIESANDCOLLATE: DWORD = DM_COPIES | DM_COLLATE;
pub const START_PAGE_GENERAL: DWORD = 0xffffffff;
pub const PD_RESULT_CANCEL: DWORD = 0;
pub const PD_RESULT_PRINT: DWORD = 1;
pub const PD_RESULT_APPLY: DWORD = 2;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct DEVNAMES {
    wDriverOffset: WORD,
    wDeviceOffset: WORD,
    wOutputOffset: WORD,
    wDefault: WORD,
}}
pub type LPDEVNAMES = *mut DEVNAMES;
pub type PCDEVNAMES = *const DEVNAMES;
pub const DN_DEFAULTPRN: WORD = 0x0001;
extern "system" {
    pub fn CommDlgExtendedError() -> DWORD;
}
pub const WM_PSD_PAGESETUPDLG: UINT = WM_USER;
pub const WM_PSD_FULLPAGERECT: UINT = WM_USER + 1;
pub const WM_PSD_MINMARGINRECT: UINT = WM_USER + 2;
pub const WM_PSD_MARGINRECT: UINT = WM_USER + 3;
pub const WM_PSD_GREEKTEXTRECT: UINT = WM_USER + 4;
pub const WM_PSD_ENVSTAMPRECT: UINT = WM_USER + 5;
pub const WM_PSD_YAFULLPAGERECT: UINT = WM_USER + 6;
FN!{stdcall LPPAGEPAINTHOOK(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
FN!{stdcall LPPAGESETUPHOOK(
    HWND,
    UINT,
    WPARAM,
    LPARAM,
) -> UINT_PTR}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PAGESETUPDLGA {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    Flags: DWORD,
    ptPaperSize: POINT,
    rtMinMargin: RECT,
    rtMargin: RECT,
    hInstance: HINSTANCE,
    lCustData: LPARAM,
    lpfnPageSetupHook: LPPAGESETUPHOOK,
    lpfnPagePaintHook: LPPAGEPAINTHOOK,
    lpPageSetupTemplateName: LPCSTR,
    hPageSetupTemplate: HGLOBAL,
}}
pub type LPPAGESETUPDLGA = *mut PAGESETUPDLGA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct PAGESETUPDLGW {
    lStructSize: DWORD,
    hwndOwner: HWND,
    hDevMode: HGLOBAL,
    hDevNames: HGLOBAL,
    Flags: DWORD,
    ptPaperSize: POINT,
    rtMinMargin: RECT,
    rtMargin: RECT,
    hInstance: HINSTANCE,
    lCustData: LPARAM,
    lpfnPageSetupHook: LPPAGESETUPHOOK,
    lpfnPagePaintHook: LPPAGEPAINTHOOK,
    lpPageSetupTemplateName: LPCWSTR,
    hPageSetupTemplate: HGLOBAL,
}}
pub type LPPAGESETUPDLGW = *mut PAGESETUPDLGW;
extern "system" {
    pub fn PageSetupDlgA(
        lppsd: LPPAGESETUPDLGA,
    ) -> BOOL;
    pub fn PageSetupDlgW(
        lppsd: LPPAGESETUPDLGW,
    ) -> BOOL;
}
pub const PSD_DEFAULTMINMARGINS: DWORD = 0x00000000;
pub const PSD_INWININIINTLMEASURE: DWORD = 0x00000000;
pub const PSD_MINMARGINS: DWORD = 0x00000001;
pub const PSD_MARGINS: DWORD = 0x00000002;
pub const PSD_INTHOUSANDTHSOFINCHES: DWORD = 0x00000004;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: DWORD = 0x00000008;
pub const PSD_DISABLEMARGINS: DWORD = 0x00000010;
pub const PSD_DISABLEPRINTER: DWORD = 0x00000020;
pub const PSD_NOWARNING: DWORD = 0x00000080;
pub const PSD_DISABLEORIENTATION: DWORD = 0x00000100;
pub const PSD_RETURNDEFAULT: DWORD = 0x00000400;
pub const PSD_DISABLEPAPER: DWORD = 0x00000200;
pub const PSD_SHOWHELP: DWORD = 0x00000800;
pub const PSD_ENABLEPAGESETUPHOOK: DWORD = 0x00002000;
pub const PSD_ENABLEPAGESETUPTEMPLATE: DWORD = 0x00008000;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: DWORD = 0x00020000;
pub const PSD_ENABLEPAGEPAINTHOOK: DWORD = 0x00040000;
pub const PSD_DISABLEPAGEPAINTING: DWORD = 0x00080000;
pub const PSD_NONETWORKBUTTON: DWORD = 0x00200000;
