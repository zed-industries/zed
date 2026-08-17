// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! SHELL.DLL functions, types, and definitions
use ctypes::{__int64, c_int, c_void};
use shared::basetsd::{DWORD_PTR, UINT_PTR};
use shared::guiddef::{GUID, REFIID};
use shared::minwindef::{
    BOOL, DWORD, FILETIME, HINSTANCE, HKEY, INT, LPARAM, LPVOID, MAX_PATH, UINT, ULONG, WORD,
};
use shared::windef::{HICON, HWND, POINT, RECT};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::processthreadsapi::{LPPROCESS_INFORMATION, LPSTARTUPINFOW};
use um::winnt::{
    CHAR, HANDLE, HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCSTR, PCWSTR, PCZZSTR, PCZZWSTR, PWSTR,
    PZZSTR, PZZWSTR, ULARGE_INTEGER, WCHAR,
};
use um::winuser::WM_USER;
DECLARE_HANDLE!{HDROP, HDROP__}
extern "system" {
    pub fn DragQueryFileA(
        hDrop: HDROP,
        iFile: UINT,
        lpszFile: LPSTR,
        cch: UINT,
    ) -> UINT;
    pub fn DragQueryFileW(
        hDrop: HDROP,
        iFile: UINT,
        lpszFile: LPWSTR,
        cch: UINT,
    ) -> UINT;
    pub fn DragQueryPoint(
        hDrop: HDROP,
        lppt: *mut POINT,
    ) -> BOOL;
    pub fn DragFinish(
        hDrop: HDROP,
    );
    pub fn DragAcceptFiles(
        hWnd: HWND,
        fAccept: BOOL,
    );
    pub fn ShellExecuteA(
        hwnd: HWND,
        lpOperation: LPCSTR,
        lpFile: LPCSTR,
        lpParameters: LPCSTR,
        lpDirectory: LPCSTR,
        nShowCmd: c_int,
    ) -> HINSTANCE;
    pub fn ShellExecuteW(
        hwnd: HWND,
        lpOperation: LPCWSTR,
        lpFile: LPCWSTR,
        lpParameters: LPCWSTR,
        lpDirectory: LPCWSTR,
        nShowCmd: c_int,
    ) -> HINSTANCE;
    pub fn FindExecutableA(
        lpFile: LPCSTR,
        lpDirectory: LPCSTR,
        lpResult: LPSTR,
    ) -> HINSTANCE;
    pub fn FindExecutableW(
        lpFile: LPCWSTR,
        lpDirectory: LPCWSTR,
        lpResult: LPWSTR,
    ) -> HINSTANCE;
    pub fn CommandLineToArgvW(
        lpCmdLine: LPCWSTR,
        pNumArgs: *mut c_int,
    ) -> *mut LPWSTR;
    pub fn ShellAboutA(
        hWnd: HWND,
        szApp: LPCSTR,
        szOtherStuff: LPCSTR,
        hIcon: HICON,
    ) -> INT;
    pub fn ShellAboutW(
        hWnd: HWND,
        szApp: LPCWSTR,
        szOtherStuff: LPCWSTR,
        hIcon: HICON,
    ) -> INT;
    pub fn DuplicateIcon(
        hInst: HINSTANCE,
        hIcon: HICON,
    ) -> HICON;
    pub fn ExtractAssociatedIconA(
        hInst: HINSTANCE,
        pszIconPath: LPSTR,
        piIcon: *mut WORD,
    ) -> HICON;
    pub fn ExtractAssociatedIconW(
        hInst: HINSTANCE,
        pszIconPath: LPWSTR,
        piIcon: *mut WORD,
    ) -> HICON;
    pub fn ExtractAssociatedIconExA(
        hInst: HINSTANCE,
        pszIconPath: LPSTR,
        piIconIndex: *mut WORD,
        piIconId: *mut WORD,
    ) -> HICON;
    pub fn ExtractAssociatedIconExW(
        hInst: HINSTANCE,
        pszIconPath: LPWSTR,
        piIconIndex: *mut WORD,
        piIconId: *mut WORD,
    ) -> HICON;
    pub fn ExtractIconA(
        hInst: HINSTANCE,
        pszExeFileName: LPCSTR,
        nIconIndex: UINT,
    ) -> HICON;
    pub fn ExtractIconW(
        hInst: HINSTANCE,
        pszExeFileName: LPCWSTR,
        nIconIndex: UINT,
    ) -> HICON;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct DRAGINFOA {
    uSize: UINT,
    pt: POINT,
    fNC: BOOL,
    lpFileList: PZZSTR,
    grfKeyState: DWORD,
}}
pub type LPDRAGINFOA = *mut DRAGINFOA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct DRAGINFOW {
    uSize: UINT,
    pt: POINT,
    fNC: BOOL,
    lpFileList: PZZWSTR,
    grfKeyState: DWORD,
}}
pub type LPDRAGINFOW = *mut DRAGINFOW;
pub const ABM_NEW: DWORD = 0x00000000;
pub const ABM_REMOVE: DWORD = 0x00000001;
pub const ABM_QUERYPOS: DWORD = 0x00000002;
pub const ABM_SETPOS: DWORD = 0x00000003;
pub const ABM_GETSTATE: DWORD = 0x00000004;
pub const ABM_GETTASKBARPOS: DWORD = 0x00000005;
pub const ABM_ACTIVATE: DWORD = 0x00000006;
pub const ABM_GETAUTOHIDEBAR: DWORD = 0x00000007;
pub const ABM_SETAUTOHIDEBAR: DWORD = 0x00000008;
pub const ABM_WINDOWPOSCHANGED: DWORD = 0x0000009;
pub const ABM_SETSTATE: DWORD = 0x0000000a;
pub const ABM_GETAUTOHIDEBAREX: DWORD = 0x0000000b;
pub const ABM_SETAUTOHIDEBAREX: DWORD = 0x0000000c;
pub const ABN_STATECHANGE: DWORD = 0x0000000;
pub const ABN_POSCHANGED: DWORD = 0x0000001;
pub const ABN_FULLSCREENAPP: DWORD = 0x0000002;
pub const ABN_WINDOWARRANGE: DWORD = 0x0000003;
pub const ABS_AUTOHIDE: UINT = 0x0000001;
pub const ABS_ALWAYSONTOP: UINT = 0x0000002;
pub const ABE_LEFT: UINT = 0;
pub const ABE_TOP: UINT = 1;
pub const ABE_RIGHT: UINT = 2;
pub const ABE_BOTTOM: UINT = 3;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct APPBARDATA {
    cbSize: DWORD,
    hWnd: HWND,
    uCallbackMessage: UINT,
    uEdge: UINT,
    rc: RECT,
    lParam: LPARAM,
}}
pub type PAPPBARDATA = *mut APPBARDATA;
extern "system" {
    pub fn SHAppBarMessage(
        dwMessage: DWORD,
        pData: PAPPBARDATA,
    ) -> UINT_PTR;
    pub fn DoEnvironmentSubstA(
        pszSrc: LPSTR,
        cchSrc: UINT,
    ) -> DWORD;
    pub fn DoEnvironmentSubstW(
        pszSrc: LPWSTR,
        cchSrc: UINT,
    ) -> DWORD;
    pub fn ExtractIconExA(
        lpszFile: LPCSTR,
        nIconIndex: c_int,
        phiconLarge: *mut HICON,
        phiconSmall: *mut HICON,
        nIcons: UINT,
    ) -> UINT;
    pub fn ExtractIconExW(
        lpszFile: LPCWSTR,
        nIconIndex: c_int,
        phiconLarge: *mut HICON,
        phiconSmall: *mut HICON,
        nIcons: UINT,
    ) -> UINT;
}
pub const FO_MOVE: WORD = 0x0001;
pub const FO_COPY: WORD = 0x0002;
pub const FO_DELETE: WORD = 0x0003;
pub const FO_RENAME: WORD = 0x0004;
pub const FOF_MULTIDESTFILES: WORD = 0x0001;
pub const FOF_CONFIRMMOUSE: WORD = 0x0002;
pub const FOF_SILENT: WORD = 0x0004;
pub const FOF_RENAMEONCOLLISION: WORD = 0x0008;
pub const FOF_NOCONFIRMATION: WORD = 0x0010;
pub const FOF_WANTMAPPINGHANDLE: WORD = 0x0020;
pub const FOF_ALLOWUNDO: WORD = 0x0040;
pub const FOF_FILESONLY: WORD = 0x0080;
pub const FOF_SIMPLEPROGRESS: WORD = 0x0100;
pub const FOF_NOCONFIRMMKDIR: WORD = 0x0200;
pub const FOF_NOERRORUI: WORD = 0x0400;
pub const FOF_NOCOPYSECURITYATTRIBS: WORD = 0x0800;
pub const FOF_NORECURSION: WORD = 0x1000;
pub const FOF_NO_CONNECTED_ELEMENTS: WORD = 0x2000;
pub const FOF_WANTNUKEWARNING: WORD = 0x4000;
pub const FOF_NORECURSEREPARSE: WORD = 0x8000;
pub const FOF_NO_UI: WORD = FOF_SILENT | FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_NOCONFIRMMKDIR;
pub type FILEOP_FLAGS = WORD;
pub const PO_DELETE: WORD = 0x0013;
pub const PO_RENAME: WORD = 0x0014;
pub const PO_PORTCHANGE: WORD = 0x0020;
pub const PO_REN_PORT: WORD = 0x0034;
pub type PRINTEROP_FLAGS = WORD;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHFILEOPSTRUCTA {
    hwnd: HWND,
    wFunc: UINT,
    pFrom: PCZZSTR,
    pTo: PCZZSTR,
    fFlags: FILEOP_FLAGS,
    fAnyOperationsAborted: BOOL,
    hNameMappings: LPVOID,
    lpszProgressTitle: PCSTR,
}}
pub type LPSHFILEOPSTRUCTA = *mut SHFILEOPSTRUCTA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHFILEOPSTRUCTW {
    hwnd: HWND,
    wFunc: UINT,
    pFrom: PCZZWSTR,
    pTo: PCZZWSTR,
    fFlags: FILEOP_FLAGS,
    fAnyOperationsAborted: BOOL,
    hNameMappings: LPVOID,
    lpszProgressTitle: PCWSTR,
}}
pub type LPSHFILEOPSTRUCTW = *mut SHFILEOPSTRUCTW;
extern "system" {
    pub fn SHFileOperationA(
        lpFileOp: LPSHFILEOPSTRUCTA,
    ) -> c_int;
    pub fn SHFileOperationW(
        lpFileOp: LPSHFILEOPSTRUCTW,
    ) -> c_int;
    pub fn SHFreeNameMappings(
        hNameMappings: HANDLE,
    );
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHNAMEMAPPINGA {
    pszOldPath: LPSTR,
    pszNewPath: LPSTR,
    cchOldPath: c_int,
    cchNewPath: c_int,
}}
pub type LPSHNAMEMAPPINGA = *mut SHNAMEMAPPINGA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHNAMEMAPPINGW {
    pszOldPath: LPWSTR,
    pszNewPath: LPWSTR,
    cchOldPath: c_int,
    cchNewPath: c_int,
}}
pub type LPSHNAMEMAPPINGW = *mut SHNAMEMAPPINGW;
pub const SE_ERR_FNF: DWORD = 2;
pub const SE_ERR_PNF: DWORD = 3;
pub const SE_ERR_ACCESSDENIED: DWORD = 5;
pub const SE_ERR_OOM: DWORD = 8;
pub const SE_ERR_DLLNOTFOUND: DWORD = 32;
pub const SE_ERR_SHARE: DWORD = 26;
pub const SE_ERR_ASSOCINCOMPLETE: DWORD = 27;
pub const SE_ERR_DDETIMEOUT: DWORD = 28;
pub const SE_ERR_DDEFAIL: DWORD = 29;
pub const SE_ERR_DDEBUSY: DWORD = 30;
pub const SE_ERR_NOASSOC: DWORD = 31;
pub const SEE_MASK_DEFAULT: DWORD = 0x00000000;
pub const SEE_MASK_CLASSNAME: DWORD = 0x00000001;
pub const SEE_MASK_CLASSKEY: DWORD = 0x00000003;
pub const SEE_MASK_IDLIST: DWORD = 0x00000004;
pub const SEE_MASK_INVOKEIDLIST: DWORD = 0x0000000c;
pub const SEE_MASK_ICON: DWORD = 0x00000010;
pub const SEE_MASK_HOTKEY: DWORD = 0x00000020;
pub const SEE_MASK_NOCLOSEPROCESS: DWORD = 0x00000040;
pub const SEE_MASK_CONNECTNETDRV: DWORD = 0x00000080;
pub const SEE_MASK_NOASYNC: DWORD = 0x00000100;
pub const SEE_MASK_FLAG_DDEWAIT: DWORD = SEE_MASK_NOASYNC;
pub const SEE_MASK_DOENVSUBST: DWORD = 0x00000200;
pub const SEE_MASK_FLAG_NO_UI: DWORD = 0x00000400;
pub const SEE_MASK_UNICODE: DWORD = 0x00004000;
pub const SEE_MASK_NO_CONSOLE: DWORD = 0x00008000;
pub const SEE_MASK_ASYNCOK: DWORD = 0x00100000;
pub const SEE_MASK_HMONITOR: DWORD = 0x00200000;
pub const SEE_MASK_NOZONECHECKS: DWORD = 0x00800000;
pub const SEE_MASK_NOQUERYCLASSSTORE: DWORD = 0x01000000;
pub const SEE_MASK_WAITFORINPUTIDLE: DWORD = 0x02000000;
pub const SEE_MASK_FLAG_LOG_USAGE: DWORD = 0x04000000;
pub const SEE_MASK_FLAG_HINST_IS_SITE: DWORD = 0x08000000;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHELLEXECUTEINFOA {
    cbSize: DWORD,
    fMask: ULONG,
    hwnd: HWND,
    lpVerb: LPCSTR,
    lpFile: LPCSTR,
    lpParameters: LPCSTR,
    lpDirectory: LPCSTR,
    nShow: c_int,
    hInstApp: HINSTANCE,
    lpIDList: *mut c_void,
    lpClass: LPCSTR,
    hkeyClass: HKEY,
    dwHotKey: DWORD,
    hMonitor: HANDLE,
    hProcess: HANDLE,
}}
pub type LPSHELLEXECUTEINFOA = *mut SHELLEXECUTEINFOA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHELLEXECUTEINFOW {
    cbSize: DWORD,
    fMask: ULONG,
    hwnd: HWND,
    lpVerb: LPCWSTR,
    lpFile: LPCWSTR,
    lpParameters: LPCWSTR,
    lpDirectory: LPCWSTR,
    nShow: c_int,
    hInstApp: HINSTANCE,
    lpIDList: *mut c_void,
    lpClass: LPCWSTR,
    hkeyClass: HKEY,
    dwHotKey: DWORD,
    hMonitor: HANDLE,
    hProcess: HANDLE,
}}
pub type LPSHELLEXECUTEINFOW = *mut SHELLEXECUTEINFOW;
extern "system" {
    pub fn ShellExecuteExA(
        pExecInfo: *mut SHELLEXECUTEINFOA,
    ) -> BOOL;
    pub fn ShellExecuteExW(
        pExecInfo: *mut SHELLEXECUTEINFOW,
    ) -> BOOL;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHCREATEPROCESSINFOW {
    cbSize: DWORD,
    fMask: ULONG,
    hwnd: HWND,
    pszFile: LPCWSTR,
    pszParameters: LPCWSTR,
    pszCurrentDirectory: LPCWSTR,
    hUserToken: HANDLE,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
}}
pub type PSHCREATEPROCESSINFOW = *mut SHCREATEPROCESSINFOW;
extern "system" {
    pub fn SHCreateProcessAsUserW(
        pscpi: PSHCREATEPROCESSINFOW,
    ) -> BOOL;
    pub fn SHEvaluateSystemCommandTemplate(
        pszCmdTemplate: PCWSTR,
        ppszApplication: *mut PWSTR,
        ppszCommandLine: *mut PWSTR,
        ppszParameters: *mut PWSTR,
    ) -> HRESULT;
}
ENUM!{enum ASSOCCLASS {
    ASSOCCLASS_SHELL_KEY = 0,
    ASSOCCLASS_PROGID_KEY,
    ASSOCCLASS_PROGID_STR,
    ASSOCCLASS_CLSID_KEY,
    ASSOCCLASS_CLSID_STR,
    ASSOCCLASS_APP_KEY,
    ASSOCCLASS_APP_STR,
    ASSOCCLASS_SYSTEM_STR,
    ASSOCCLASS_FOLDER,
    ASSOCCLASS_STAR,
    ASSOCCLASS_FIXED_PROGID_STR,
    ASSOCCLASS_PROTOCOL_STR,
}}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct ASSOCIATIONELEMENT {
    ac: ASSOCCLASS,
    hkClass: HKEY,
    pszClass: PCWSTR,
}}
extern "system" {
    pub fn AssocCreateForClasses(
        rgClasses: *const ASSOCIATIONELEMENT,
        cClasses: ULONG,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHQUERYRBINFO {
    cbSize: DWORD,
    i64Size: __int64,
    i64NumItems: __int64,
}}
pub type LPSHQUERYRBINFO = *mut SHQUERYRBINFO;
pub const SHERB_NOCONFIRMATION: DWORD = 0x00000001;
pub const SHERB_NOPROGRESSUI: DWORD = 0x00000002;
pub const SHERB_NOSOUND: DWORD = 0x00000004;
extern "system" {
    pub fn SHQueryRecycleBinA(
        pszRootPath: LPCSTR,
        pSHQueryRBInfo: LPSHQUERYRBINFO,
    ) -> HRESULT;
    pub fn SHQueryRecycleBinW(
        pszRootPath: LPCWSTR,
        pSHQueryRBInfo: LPSHQUERYRBINFO,
    ) -> HRESULT;
    pub fn SHEmptyRecycleBinA(
        hwnd: HWND,
        pszRootPath: LPCSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
    pub fn SHEmptyRecycleBinW(
        hwnd: HWND,
        pszRootPath: LPCWSTR,
        dwFlags: DWORD,
    ) -> HRESULT;
}
ENUM!{enum QUERY_USER_NOTIFICATION_STATE {
    QUNS_NOT_PRESENT = 1,
    QUNS_BUSY = 2,
    QUNS_RUNNING_D3D_FULL_SCREEN = 3,
    QUNS_PRESENTATION_MODE = 4,
    QUNS_ACCEPTS_NOTIFICATIONS = 5,
    QUNS_QUIET_TIME = 6,
    QUNS_APP = 7,
}}
extern "system" {
    pub fn SHQueryUserNotificationState(
        pquns: *mut QUERY_USER_NOTIFICATION_STATE,
    ) -> HRESULT;
    pub fn SHGetPropertyStoreForWindow(
        hwnd: HWND,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
UNION!{#[cfg_attr(target_arch = "x86", repr(packed))] union NOTIFYICONDATAA_u {
    [u32; 1],
    uTimeout uTimeout_mut: UINT,
    uVersion uVersion_mut: UINT,
}}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct NOTIFYICONDATAA {
    cbSize: DWORD,
    hWnd: HWND,
    uID: UINT,
    uFlags: UINT,
    uCallbackMessage: UINT,
    hIcon: HICON,
    szTip: [CHAR; 128],
    dwState: DWORD,
    dwStateMask: DWORD,
    szInfo: [CHAR; 256],
    u: NOTIFYICONDATAA_u,
    szInfoTitle: [CHAR; 64],
    dwInfoFlags: DWORD,
    guidItem: GUID,
    hBalloonIcon: HICON,
}}
pub type PNOTIFYICONDATAA = *mut NOTIFYICONDATAA;
UNION!{#[cfg_attr(target_arch = "x86", repr(packed))] union NOTIFYICONDATAW_u {
    [u32; 1],
    uTimeout uTimeout_mut: UINT,
    uVersion uVersion_mut: UINT,
}}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct NOTIFYICONDATAW {
    cbSize: DWORD,
    hWnd: HWND,
    uID: UINT,
    uFlags: UINT,
    uCallbackMessage: UINT,
    hIcon: HICON,
    szTip: [WCHAR; 128],
    dwState: DWORD,
    dwStateMask: DWORD,
    szInfo: [WCHAR; 256],
    u: NOTIFYICONDATAW_u,
    szInfoTitle: [WCHAR; 64],
    dwInfoFlags: DWORD,
    guidItem: GUID,
    hBalloonIcon: HICON,
}}
pub type PNOTIFYICONDATAW = *mut NOTIFYICONDATAW;
pub const NIN_SELECT: DWORD = WM_USER + 0;
pub const NINF_KEY: DWORD = 0x1;
pub const NIN_KEYSELECT: DWORD = NIN_SELECT | NINF_KEY;
pub const NIN_BALLOONSHOW: DWORD = WM_USER + 2;
pub const NIN_BALLOONHIDE: DWORD = WM_USER + 3;
pub const NIN_BALLOONTIMEOUT: DWORD = WM_USER + 4;
pub const NIN_BALLOONUSERCLICK: DWORD = WM_USER + 5;
pub const NIN_POPUPOPEN: DWORD = WM_USER + 6;
pub const NIN_POPUPCLOSE: DWORD = WM_USER + 7;
pub const NIM_ADD: DWORD = 0x00000000;
pub const NIM_MODIFY: DWORD = 0x00000001;
pub const NIM_DELETE: DWORD = 0x00000002;
pub const NIM_SETFOCUS: DWORD = 0x00000003;
pub const NIM_SETVERSION: DWORD = 0x00000004;
pub const NOTIFYICON_VERSION: DWORD = 3;
pub const NOTIFYICON_VERSION_4: DWORD = 4;
pub const NIF_MESSAGE: DWORD = 0x00000001;
pub const NIF_ICON: DWORD = 0x00000002;
pub const NIF_TIP: DWORD = 0x00000004;
pub const NIF_STATE: DWORD = 0x00000008;
pub const NIF_INFO: DWORD = 0x00000010;
pub const NIF_GUID: DWORD = 0x00000020;
pub const NIF_REALTIME: DWORD = 0x00000040;
pub const NIF_SHOWTIP: DWORD = 0x00000080;
pub const NIS_HIDDEN: DWORD = 0x00000001;
pub const NIS_SHAREDICON: DWORD = 0x00000002;
pub const NIIF_NONE: DWORD = 0x00000000;
pub const NIIF_INFO: DWORD = 0x00000001;
pub const NIIF_WARNING: DWORD = 0x00000002;
pub const NIIF_ERROR: DWORD = 0x00000003;
pub const NIIF_USER: DWORD = 0x00000004;
pub const NIIF_ICON_MASK: DWORD = 0x0000000F;
pub const NIIF_NOSOUND: DWORD = 0x00000010;
pub const NIIF_LARGE_ICON: DWORD = 0x00000020;
pub const NIIF_RESPECT_QUIET_TIME: DWORD = 0x00000080;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct NOTIFYICONIDENTIFIER {
    cbSize: DWORD,
    hWnd: HWND,
    uID: UINT,
    guidItem: GUID,
}}
pub type PNOTIFYICONIDENTIFIER = *mut NOTIFYICONIDENTIFIER;
extern "system" {
    pub fn Shell_NotifyIconA(
        dwMessage: DWORD,
        lpData: PNOTIFYICONDATAA,
    ) -> BOOL;
    pub fn Shell_NotifyIconW(
        dwMessage: DWORD,
        lpData: PNOTIFYICONDATAW,
    ) -> BOOL;
    pub fn Shell_NotifyIconGetRect(
        identifier: *const NOTIFYICONIDENTIFIER,
        iconLocation: *mut RECT,
    ) -> HRESULT;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHFILEINFOA {
    hIcon: HICON,
    iIcon: c_int,
    dwAttributes: DWORD,
    szDisplayName: [CHAR; MAX_PATH],
    szTypeName: [CHAR; 80],
}}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHFILEINFOW {
    hIcon: HICON,
    iIcon: c_int,
    dwAttributes: DWORD,
    szDisplayName: [WCHAR; MAX_PATH],
    szTypeName: [WCHAR; 80],
}}
pub const SHGFI_ICON: DWORD = 0x000000100;
pub const SHGFI_DISPLAYNAME: DWORD = 0x000000200;
pub const SHGFI_TYPENAME: DWORD = 0x000000400;
pub const SHGFI_ATTRIBUTES: DWORD = 0x000000800;
pub const SHGFI_ICONLOCATION: DWORD = 0x000001000;
pub const SHGFI_EXETYPE: DWORD = 0x000002000;
pub const SHGFI_SYSICONINDEX: DWORD = 0x000004000;
pub const SHGFI_LINKOVERLAY: DWORD = 0x000008000;
pub const SHGFI_SELECTED: DWORD = 0x000010000;
pub const SHGFI_ATTR_SPECIFIED: DWORD = 0x000020000;
pub const SHGFI_LARGEICON: DWORD = 0x000000000;
pub const SHGFI_SMALLICON: DWORD = 0x000000001;
pub const SHGFI_OPENICON: DWORD = 0x000000002;
pub const SHGFI_SHELLICONSIZE: DWORD = 0x000000004;
pub const SHGFI_PIDL: DWORD = 0x000000008;
pub const SHGFI_USEFILEATTRIBUTES: DWORD = 0x000000010;
pub const SHGFI_ADDOVERLAYS: DWORD = 0x000000020;
pub const SHGFI_OVERLAYINDEX: DWORD = 0x000000040;
extern "system" {
    pub fn SHGetFileInfoA(
        pszPath: LPCSTR,
        dwFileAttributes: DWORD,
        psfi: *mut SHFILEINFOA,
        cbFileInfo: UINT,
        uFlags: UINT,
    ) -> DWORD_PTR;
    pub fn SHGetFileInfoW(
        pszPath: LPCWSTR,
        dwFileAttributes: DWORD,
        psfi: *mut SHFILEINFOW,
        cbFileInfo: UINT,
        uFlags: UINT,
    ) -> DWORD_PTR;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct SHSTOCKICONINFO {
    cbSize: DWORD,
    hIcon: HICON,
    iSysImageIndex: c_int,
    iIcon: c_int,
    szPath: [WCHAR; MAX_PATH],
}}
pub const SHGSI_ICONLOCATION: DWORD = 0;
pub const SHGSI_ICON: DWORD = SHGFI_ICON;
pub const SHGSI_SYSICONINDEX: DWORD = SHGFI_SYSICONINDEX;
pub const SHGSI_LINKOVERLAY: DWORD = SHGFI_LINKOVERLAY;
pub const SHGSI_SELECTED: DWORD = SHGFI_SELECTED;
pub const SHGSI_LARGEICON: DWORD = SHGFI_LARGEICON;
pub const SHGSI_SMALLICON: DWORD = SHGFI_SMALLICON;
pub const SHGSI_SHELLICONSIZE: DWORD = SHGFI_SHELLICONSIZE;
ENUM!{enum SHSTOCKICONID {
    SIID_DOCNOASSOC = 0,
    SIID_DOCASSOC = 1,
    SIID_APPLICATION = 2,
    SIID_FOLDER = 3,
    SIID_FOLDEROPEN = 4,
    SIID_DRIVE525 = 5,
    SIID_DRIVE35 = 6,
    SIID_DRIVEREMOVE = 7,
    SIID_DRIVEFIXED = 8,
    SIID_DRIVENET = 9,
    SIID_DRIVENETDISABLED = 10,
    SIID_DRIVECD = 11,
    SIID_DRIVERAM = 12,
    SIID_WORLD = 13,
    SIID_SERVER = 15,
    SIID_PRINTER = 16,
    SIID_MYNETWORK = 17,
    SIID_FIND = 22,
    SIID_HELP = 23,
    SIID_SHARE = 28,
    SIID_LINK = 29,
    SIID_SLOWFILE = 30,
    SIID_RECYCLER = 31,
    SIID_RECYCLERFULL = 32,
    SIID_MEDIACDAUDIO = 40,
    SIID_LOCK = 47,
    SIID_AUTOLIST = 49,
    SIID_PRINTERNET = 50,
    SIID_SERVERSHARE = 51,
    SIID_PRINTERFAX = 52,
    SIID_PRINTERFAXNET = 53,
    SIID_PRINTERFILE = 54,
    SIID_STACK = 55,
    SIID_MEDIASVCD = 56,
    SIID_STUFFEDFOLDER = 57,
    SIID_DRIVEUNKNOWN = 58,
    SIID_DRIVEDVD = 59,
    SIID_MEDIADVD = 60,
    SIID_MEDIADVDRAM = 61,
    SIID_MEDIADVDRW = 62,
    SIID_MEDIADVDR = 63,
    SIID_MEDIADVDROM = 64,
    SIID_MEDIACDAUDIOPLUS = 65,
    SIID_MEDIACDRW = 66,
    SIID_MEDIACDR = 67,
    SIID_MEDIACDBURN = 68,
    SIID_MEDIABLANKCD = 69,
    SIID_MEDIACDROM = 70,
    SIID_AUDIOFILES = 71,
    SIID_IMAGEFILES = 72,
    SIID_VIDEOFILES = 73,
    SIID_MIXEDFILES = 74,
    SIID_FOLDERBACK = 75,
    SIID_FOLDERFRONT = 76,
    SIID_SHIELD = 77,
    SIID_WARNING = 78,
    SIID_INFO = 79,
    SIID_ERROR = 80,
    SIID_KEY = 81,
    SIID_SOFTWARE = 82,
    SIID_RENAME = 83,
    SIID_DELETE = 84,
    SIID_MEDIAAUDIODVD = 85,
    SIID_MEDIAMOVIEDVD = 86,
    SIID_MEDIAENHANCEDCD = 87,
    SIID_MEDIAENHANCEDDVD = 88,
    SIID_MEDIAHDDVD = 89,
    SIID_MEDIABLURAY = 90,
    SIID_MEDIAVCD = 91,
    SIID_MEDIADVDPLUSR = 92,
    SIID_MEDIADVDPLUSRW = 93,
    SIID_DESKTOPPC = 94,
    SIID_MOBILEPC = 95,
    SIID_USERS = 96,
    SIID_MEDIASMARTMEDIA = 97,
    SIID_MEDIACOMPACTFLASH = 98,
    SIID_DEVICECELLPHONE = 99,
    SIID_DEVICECAMERA = 100,
    SIID_DEVICEVIDEOCAMERA = 101,
    SIID_DEVICEAUDIOPLAYER = 102,
    SIID_NETWORKCONNECT = 103,
    SIID_INTERNET = 104,
    SIID_ZIPFILE = 105,
    SIID_SETTINGS = 106,
    SIID_DRIVEHDDVD = 132,
    SIID_DRIVEBD = 133,
    SIID_MEDIAHDDVDROM = 134,
    SIID_MEDIAHDDVDR = 135,
    SIID_MEDIAHDDVDRAM = 136,
    SIID_MEDIABDROM = 137,
    SIID_MEDIABDR = 138,
    SIID_MEDIABDRE = 139,
    SIID_CLUSTEREDDRIVE = 140,
    SIID_MAX_ICONS = 181,
}}
pub const SIID_INVALID: SHSTOCKICONID = -1i32 as u32;
extern "system" {
    pub fn SHGetStockIconInfo(
        siid: SHSTOCKICONID,
        uFlags: UINT,
        psii: *mut SHSTOCKICONINFO,
    ) -> HRESULT;
    pub fn SHGetDiskFreeSpaceExA(
        pszDirectoryName: LPCSTR,
        pulFreeBytesAvailableToCaller: *mut ULARGE_INTEGER,
        pulTotalNumberOfBytes: *mut ULARGE_INTEGER,
        pulTotalNumberOfFreeBytes: *mut ULARGE_INTEGER,
    ) -> BOOL;
    pub fn SHGetDiskFreeSpaceExW(
        pszDirectoryName: LPCWSTR,
        pulFreeBytesAvailableToCaller: *mut ULARGE_INTEGER,
        pulTotalNumberOfBytes: *mut ULARGE_INTEGER,
        pulTotalNumberOfFreeBytes: *mut ULARGE_INTEGER,
    ) -> BOOL;
    pub fn SHGetNewLinkInfoA(
        pszLinkTo: LPCSTR,
        pszDir: LPCSTR,
        pszName: LPSTR,
        pfMustCopy: *mut BOOL,
        uFlags: UINT,
    ) -> BOOL;
    pub fn SHGetNewLinkInfoW(
        pszLinkTo: LPCWSTR,
        pszDir: LPCWSTR,
        pszName: LPWSTR,
        pfMustCopy: *mut BOOL,
        uFlags: UINT,
    ) -> BOOL;
}
pub const SHGNLI_PIDL: DWORD = 0x000000001;
pub const SHGNLI_PREFIXNAME: DWORD = 0x000000002;
pub const SHGNLI_NOUNIQUE: DWORD = 0x000000004;
pub const SHGNLI_NOLNK: DWORD = 0x000000008;
pub const SHGNLI_NOLOCNAME: DWORD = 0x000000010;
pub const SHGNLI_USEURLEXT: DWORD = 0x000000020;
pub const PRINTACTION_OPEN: DWORD = 0;
pub const PRINTACTION_PROPERTIES: DWORD = 1;
pub const PRINTACTION_NETINSTALL: DWORD = 2;
pub const PRINTACTION_NETINSTALLLINK: DWORD = 3;
pub const PRINTACTION_TESTPAGE: DWORD = 4;
pub const PRINTACTION_OPENNETPRN: DWORD = 5;
pub const PRINTACTION_DOCUMENTDEFAULTS: DWORD = 6;
pub const PRINTACTION_SERVERPROPERTIES: DWORD = 7;
extern "system" {
    pub fn SHInvokePrinterCommandA(
        hwnd: HWND,
        uAction: UINT,
        lpBuf1: LPCSTR,
        lpBuf2: LPCSTR,
        fModal: BOOL,
    ) -> BOOL;
    pub fn SHInvokePrinterCommandW(
        hwnd: HWND,
        uAction: UINT,
        lpBuf1: LPCWSTR,
        lpBuf2: LPCWSTR,
        fModal: BOOL,
    ) -> BOOL;
}
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPEN_PRINTER_PROPS_INFOA {
    dwSize: DWORD,
    pszSheetName: LPSTR,
    uSheetIndex: UINT,
    dwFlags: DWORD,
    bModal: BOOL,
}}
pub type POPEN_PRINTER_PROPS_INFOA = *mut OPEN_PRINTER_PROPS_INFOA;
STRUCT!{#[cfg_attr(target_arch = "x86", repr(packed))] struct OPEN_PRINTER_PROPS_INFOW {
    dwSize: DWORD,
    pszSheetName: LPWSTR,
    uSheetIndex: UINT,
    dwFlags: DWORD,
    bModal: BOOL,
}}
pub type POPEN_PRINTER_PROPS_INFOW = *mut OPEN_PRINTER_PROPS_INFOW;
pub const PRINT_PROP_FORCE_NAME: DWORD = 0x01;
extern "system" {
    pub fn SHLoadNonloadedIconOverlayIdentifiers() -> HRESULT;
    pub fn SHIsFileAvailableOffline(
        pwszPath: PCWSTR,
        pdwStatus: *mut DWORD,
    ) -> HRESULT;
}
pub const OFFLINE_STATUS_LOCAL: DWORD = 0x0001;
pub const OFFLINE_STATUS_REMOTE: DWORD = 0x0002;
pub const OFFLINE_STATUS_INCOMPLETE: DWORD = 0x0004;
extern "system" {
    pub fn SHSetLocalizedName(
        pszPath: PCWSTR,
        pszResModule: PCWSTR,
        idsRes: c_int,
    ) -> HRESULT;
    pub fn SHRemoveLocalizedName(
        pszPath: PCWSTR,
    ) -> HRESULT;
    pub fn SHGetLocalizedName(
        pszPath: PCWSTR,
        pszResModule: PWSTR,
        cch: UINT,
        pidsRes: *mut c_int,
    ) -> HRESULT;
}
extern "C" {
    pub fn ShellMessageBoxA(
        hAppInst: HINSTANCE,
        hWnd: HWND,
        lpcText: LPCSTR,
        lpcTitle: LPCSTR,
        fuStyle: UINT,
        ...
    ) -> c_int;
    pub fn ShellMessageBoxW(
        hAppInst: HINSTANCE,
        hWnd: HWND,
        lpcText: LPCWSTR,
        lpcTitle: LPCWSTR,
        fuStyle: UINT,
        ...
    ) -> c_int;
}
extern "system" {
    pub fn IsLFNDriveA(
        pszPath: LPCSTR,
    ) -> BOOL;
    pub fn IsLFNDriveW(
        pszPath: LPCWSTR,
    ) -> BOOL;
    pub fn SHEnumerateUnreadMailAccountsA(
        hKeyUser: HKEY,
        dwIndex: DWORD,
        pszMailAddress: LPSTR,
        cchMailAddress: c_int,
    ) -> HRESULT;
    pub fn SHEnumerateUnreadMailAccountsW(
        hKeyUser: HKEY,
        dwIndex: DWORD,
        pszMailAddress: LPWSTR,
        cchMailAddress: c_int,
    ) -> HRESULT;
    pub fn SHGetUnreadMailCountA(
        hKeyUser: HKEY,
        pszMailAddress: LPCSTR,
        pdwCount: *mut DWORD,
        pFileTime: *mut FILETIME,
        pszShellExecuteCommand: LPSTR,
        cchShellExecuteCommand: c_int,
    ) -> HRESULT;
    pub fn SHGetUnreadMailCountW(
        hKeyUser: HKEY,
        pszMailAddress: LPCWSTR,
        pdwCount: *mut DWORD,
        pFileTime: *mut FILETIME,
        pszShellExecuteCommand: LPWSTR,
        cchShellExecuteCommand: c_int,
    ) -> HRESULT;
    pub fn SHSetUnreadMailCountA(
        pszMailAddress: LPCSTR,
        dwCount: DWORD,
        pszShellExecuteCommand: LPCSTR,
    ) -> HRESULT;
    pub fn SHSetUnreadMailCountW(
        pszMailAddress: LPCWSTR,
        dwCount: DWORD,
        pszShellExecuteCommand: LPCWSTR,
    ) -> HRESULT;
    pub fn SHTestTokenMembership(
        hToken: HANDLE,
        ulRID: ULONG,
    ) -> BOOL;
    pub fn SHGetImageList(
        iImageList: c_int,
        riid: REFIID,
        ppvObj: *mut *mut c_void,
    ) -> HRESULT;
}
pub const SHIL_LARGE: DWORD = 0;
pub const SHIL_SMALL: DWORD = 1;
pub const SHIL_EXTRALARGE: DWORD = 2;
pub const SHIL_SYSSMALL: DWORD = 3;
pub const SHIL_JUMBO: DWORD = 4;
pub const SHIL_LAST: DWORD = SHIL_JUMBO;
FN!{stdcall PFNCANSHAREFOLDERW(
    pszPath: PCWSTR,
) -> HRESULT}
FN!{stdcall PFNSHOWSHAREFOLDERUIW(
    hwndParent: HWND,
    pszPath: PCWSTR,
) -> HRESULT}
pub const WC_NETADDRESS: &'static str = "msctls_netaddress";
extern "system" {
    pub fn InitNetworkAddressControl() -> BOOL;
}
// STRUCT!{struct NC_ADDRESS {
//     pAddrInfo: *mut NET_ADDRESS_INFO,
//     PortNumber: USHORT,
//     PrefixLength: BYTE,
// }}
// pub type PNC_ADDRESS = *mut NC_ADDRESS;
extern "system" {
    pub fn SHGetDriveMedia(
        pszDrive: PCWSTR,
        pdwMediaContent: *mut DWORD,
    ) -> HRESULT;
}
