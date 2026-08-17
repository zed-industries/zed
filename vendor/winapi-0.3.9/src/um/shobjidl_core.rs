// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::guiddef::{REFGUID, REFIID};
use shared::minwindef::{BOOL, DWORD, UINT, ULONG, WORD};
use shared::windef::{COLORREF, HICON, HWND, RECT};
use um::commctrl::HIMAGELIST;
use um::minwinbase::{WIN32_FIND_DATAA, WIN32_FIND_DATAW};
use um::objidl::IBindCtx;
use um::propkeydef::REFPROPERTYKEY;
use um::propsys::GETPROPERTYSTOREFLAGS;
use um::shtypes::{PCIDLIST_ABSOLUTE, PIDLIST_ABSOLUTE};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, ULONGLONG, WCHAR};
DEFINE_GUID!{CLSID_DesktopWallpaper,
    0xc2cf3110, 0x460e, 0x4fc1, 0xb9, 0xd0, 0x8a, 0x1c, 0x0c, 0x9c, 0xc4, 0xbd}
DEFINE_GUID!{CLSID_TaskbarList,
    0x56fdf344, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90}
DEFINE_GUID!{CLSID_FileOpenDialog,
    0xdc1c5a9c, 0xe88a, 0x4dde, 0xa5, 0xa1, 0x60, 0xf8, 0x2a, 0x20, 0xae, 0xf7}
DEFINE_GUID!{CLSID_FileSaveDialog,
    0xc0b4e2f3, 0xba21, 0x4773, 0x8d, 0xba, 0x33, 0x5e, 0xc9, 0x46, 0xeb, 0x8b}
//4498
ENUM!{enum SHCONTF {
    SHCONTF_CHECKING_FOR_CHILDREN = 0x10,
    SHCONTF_FOLDERS = 0x20,
    SHCONTF_NONFOLDERS = 0x40,
    SHCONTF_INCLUDEHIDDEN = 0x80,
    SHCONTF_INIT_ON_FIRST_NEXT = 0x100,
    SHCONTF_NETPRINTERSRCH = 0x200,
    SHCONTF_SHAREABLE = 0x400,
    SHCONTF_STORAGE = 0x800,
    SHCONTF_NAVIGATION_ENUM = 0x1000,
    SHCONTF_FASTITEMS = 0x2000,
    SHCONTF_FLATLIST = 0x4000,
    SHCONTF_ENABLE_ASYNC = 0x8000,
    SHCONTF_INCLUDESUPERHIDDEN = 0x10000,
}}
pub type SFGAOF = ULONG;
//9466
ENUM!{enum SIGDN {
    SIGDN_NORMALDISPLAY = 0,
    SIGDN_PARENTRELATIVEPARSING = 0x80018001,
    SIGDN_DESKTOPABSOLUTEPARSING = 0x80028000,
    SIGDN_PARENTRELATIVEEDITING = 0x80031001,
    SIGDN_DESKTOPABSOLUTEEDITING = 0x8004c000,
    SIGDN_FILESYSPATH = 0x80058000,
    SIGDN_URL = 0x80068000,
    SIGDN_PARENTRELATIVEFORADDRESSBAR = 0x8007c001,
    SIGDN_PARENTRELATIVE = 0x80080001,
    SIGDN_PARENTRELATIVEFORUI = 0x80094001,
}}
ENUM!{enum SICHINTF {
    SICHINT_DISPLAY = 0,
    SICHINT_ALLFIELDS = 0x80000000,
    SICHINT_CANONICAL = 0x10000000,
    SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL = 0x20000000,
}}
RIDL!{#[uuid(0x43826d1e, 0xe718, 0x42ee, 0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe)]
interface IShellItem(IShellItemVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetParent(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetDisplayName(
        sigdnName: SIGDN,
        ppszName: *mut LPWSTR,
    ) -> HRESULT,
    fn GetAttributes(
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> HRESULT,
    fn Compare(
        psi: *mut IShellItem,
        hint: SICHINTF,
        piOrder: *mut c_int,
    ) -> HRESULT,
}}
ENUM!{enum SIATTRIBFLAGS {
    SIATTRIBFLAGS_AND = 0x1,
    SIATTRIBFLAGS_OR = 0x2,
    SIATTRIBFLAGS_APPCOMPAT = 0x3,
    SIATTRIBFLAGS_MASK = 0x3,
    SIATTRIBFLAGS_ALLITEMS = 0x4000,
}}
RIDL!{#[uuid(0xb63ea76d, 0x1f85, 0x456f, 0xa1, 0x9c, 0x48, 0x15, 0x9e, 0xfa, 0x85, 0x8b)]
interface IShellItemArray(IShellItemArrayVtbl): IUnknown(IUnknownVtbl) {
    fn BindToHandler(
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppvOut: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyStore(
        flags: GETPROPERTYSTOREFLAGS,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetPropertyDescriptionList(
        keyType: REFPROPERTYKEY,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT,
    fn GetAttributes(
        AttribFlags: SIATTRIBFLAGS,
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF,
    ) -> HRESULT,
    fn GetCount(
        pdwNumItems: *mut DWORD,
    ) -> HRESULT,
    fn GetItemAt(
        dwIndex: DWORD,
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    // TODO: Add IEnumShellItems
    //fn EnumItems(
    //    ppenumShellItems: *mut *mut IEnumShellItems,
    //) -> HRESULT,
}}
//20869
RIDL!{#[uuid(0xb4db1657, 0x70d7, 0x485e, 0x8e, 0x3e, 0x6f, 0xcb, 0x5a, 0x5c, 0x18, 0x02)]
interface IModalWindow(IModalWindowVtbl): IUnknown(IUnknownVtbl) {
    fn Show(
        hwndOwner: HWND,
    ) -> HRESULT,
}}
//22307
//27457
RIDL!{#[uuid(0x2659b475, 0xeeb8, 0x48b7, 0x8f, 0x07, 0xb3, 0x78, 0x81, 0x0f, 0x48, 0xcf)]
interface IShellItemFilter(IShellItemFilterVtbl): IUnknown(IUnknownVtbl) {
    fn IncludeItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetEnumFlagsForItem(
        psi: *mut IShellItem,
        pgrfFlags: *mut SHCONTF,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x56fdf342, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
interface ITaskbarList(ITaskbarListVtbl): IUnknown(IUnknownVtbl) {
    fn HrInit() -> HRESULT,
    fn AddTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn DeleteTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn ActivateTab(
        hwnd: HWND,
    ) -> HRESULT,
    fn SetActiveAlt(
        hwnd: HWND,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x602d4995, 0xb13a, 0x429b, 0xa6, 0x6e, 0x19, 0x35, 0xe4, 0x4f, 0x43, 0x17)]
interface ITaskbarList2(ITaskbarList2Vtbl): ITaskbarList(ITaskbarListVtbl) {
    fn MarkFullscreenWindow(
        hwnd: HWND,
        fFullscreen: BOOL,
    ) -> HRESULT,
}}
ENUM!{enum THUMBBUTTONFLAGS {
    THBF_ENABLED = 0,
    THBF_DISABLED = 0x1,
    THBF_DISMISSONCLICK = 0x2,
    THBF_NOBACKGROUND = 0x4,
    THBF_HIDDEN = 0x8,
    THBF_NONINTERACTIVE = 0x10,
}}
ENUM!{enum THUMBBUTTONMASK {
    THB_BITMAP = 0x1,
    THB_ICON = 0x2,
    THB_TOOLTIP = 0x4,
    THB_FLAGS = 0x8,
}}
STRUCT!{struct THUMBBUTTON {
    dwMask: THUMBBUTTONMASK,
    iId: UINT,
    iBitmap: UINT,
    hIcon: HICON,
    szTip: [WCHAR; 260],
    dwFlags: THUMBBUTTONFLAGS,
}}
pub type LPTHUMBBUTTON = *mut THUMBBUTTON;
pub const THBN_CLICKED: WORD = 0x1800;
ENUM!{enum TBPFLAG {
    TBPF_NOPROGRESS = 0,
    TBPF_INDETERMINATE = 0x1,
    TBPF_NORMAL = 0x2,
    TBPF_ERROR = 0x4,
    TBPF_PAUSED = 0x8,
}}
RIDL!{#[uuid(0xea1afb91, 0x9e28, 0x4b86, 0x90, 0xe9, 0x9e, 0x9f, 0x8a, 0x5e, 0xef, 0xaf)]
interface ITaskbarList3(ITaskbarList3Vtbl): ITaskbarList2(ITaskbarList2Vtbl) {
    fn SetProgressValue(
        hwnd: HWND,
        ullCompleted: ULONGLONG,
        ullTotal: ULONGLONG,
    ) -> HRESULT,
    fn SetProgressState(
        hwnd: HWND,
        tbpFlags: TBPFLAG,
    ) -> HRESULT,
    fn RegisterTab(
        hwndTab: HWND,
        hwndMDI: HWND,
    ) -> HRESULT,
    fn UnregisterTab(
        hwndTab: HWND,
    ) -> HRESULT,
    fn SetTabOrder(
        hwndTab: HWND,
        hwndInsertBefore: HWND,
    ) -> HRESULT,
    fn SetTabActive(
        hwndTab: HWND,
        hwndMDI: HWND,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn ThumbBarAddButtons(
        hwnd: HWND,
        cButtons: UINT,
        pButton: LPTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarUpdateButtons(
        hwnd: HWND,
        cButtons: UINT,
        pButton: LPTHUMBBUTTON,
    ) -> HRESULT,
    fn ThumbBarSetImageList(
        hwnd: HWND,
        himl: HIMAGELIST,
    ) -> HRESULT,
    fn SetOverlayIcon(
        hwnd: HWND,
        hIcon: HICON,
        pszDescription: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailTooltip(
        hwnd: HWND,
        pszTip: LPCWSTR,
    ) -> HRESULT,
    fn SetThumbnailClip(
        hwnd: HWND,
        prcClip: *mut RECT,
    ) -> HRESULT,
}}
ENUM!{enum STPFLAG {
    STPF_NONE = 0,
    STPF_USEAPPTHUMBNAILALWAYS = 0x1,
    STPF_USEAPPTHUMBNAILWHENACTIVE = 0x2,
    STPF_USEAPPPEEKALWAYS = 0x4,
    STPF_USEAPPPEEKWHENACTIVE = 0x8,
}}
RIDL!{#[uuid(0xc43dc798, 0x95d1, 0x4bea, 0x90, 0x30, 0xbb, 0x99, 0xe2, 0x98, 0x3a, 0x1a)]
interface ITaskbarList4(ITaskbarList4Vtbl): ITaskbarList3(ITaskbarList3Vtbl) {
    fn SetTabProperties(
        hwndTab: HWND,
        stpFlags: STPFLAG,
    ) -> HRESULT,
}}
ENUM!{enum DESKTOP_SLIDESHOW_OPTIONS {
    DSO_SHUFFLEIMAGES = 0x1,
}}
ENUM!{enum DESKTOP_SLIDESHOW_STATE {
    DSS_ENABLED = 0x1,
    DSS_SLIDESHOW = 0x2,
    DSS_DISABLED_BY_REMOTE_SESSION = 0x4,
}}
ENUM!{enum DESKTOP_SLIDESHOW_DIRECTION {
    DSD_FORWARD = 0,
    DSD_BACKWARD = 1,
}}
ENUM!{enum DESKTOP_WALLPAPER_POSITION {
    DWPOS_CENTER = 0,
    DWPOS_TILE = 1,
    DWPOS_STRETCH = 2,
    DWPOS_FIT = 3,
    DWPOS_FILL = 4,
    DWPOS_SPAN = 5,
}}
RIDL!{#[uuid(0xb92b56a9, 0x8b55, 0x4e14, 0x9a, 0x89, 0x01, 0x99, 0xbb, 0xb6, 0xf9, 0x3b)]
interface IDesktopWallpaper(IDesktopWallpaperVtbl): IUnknown(IUnknownVtbl) {
    fn SetWallpaper(
        monitorID: LPCWSTR,
        wallpaper: LPCWSTR,
    ) -> HRESULT,
    fn GetWallpaper(
        monitorID: LPCWSTR,
        wallpaper: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMonitorDevicePathAt(
        monitorIndex: UINT,
        monitorID: *mut LPWSTR,
    ) -> HRESULT,
    fn GetMonitorDevicePathCount(
        count: *mut UINT,
    ) -> HRESULT,
    fn GetMonitorRECT(
        monitorID: LPCWSTR,
        displayRect: *mut RECT,
    ) -> HRESULT,
    fn SetBackgroundColor(
        color: COLORREF,
    ) -> HRESULT,
    fn GetBackgroundColor(
        color: *mut COLORREF,
    ) -> HRESULT,
    fn SetPosition(
        position: DESKTOP_WALLPAPER_POSITION,
    ) -> HRESULT,
    fn GetPosition(
        position: *mut DESKTOP_WALLPAPER_POSITION,
    ) -> HRESULT,
    fn SetSlideshow(
        items: *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSlideshow(
        items: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn SetSlideshowOptions(
        options: DESKTOP_SLIDESHOW_OPTIONS,
        slideshowTick: UINT,
    ) -> HRESULT,
    fn GetSlideshowOptions(
        options: *mut DESKTOP_SLIDESHOW_OPTIONS,
        slideshowTick: *mut UINT,
    ) -> HRESULT,
    fn AdvanceSlideshow(
        monitorID: LPCWSTR,
        direction: DESKTOP_SLIDESHOW_DIRECTION,
    ) -> HRESULT,
    fn GetStatus(
        state: *mut DESKTOP_SLIDESHOW_STATE,
    ) -> HRESULT,
    fn Enable(
        enable: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x000214ee, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkA(IShellLinkAVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAA,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x000214f9, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IShellLinkW(IShellLinkWVtbl): IUnknown(IUnknownVtbl) {
    fn GetPath(
        pszFile: LPWSTR,
        cch: c_int,
        pfd: *mut WIN32_FIND_DATAW,
        fFlags: DWORD,
    ) -> HRESULT,
    fn GetIDList(
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn SetIDList(
        pidl: PCIDLIST_ABSOLUTE,
    ) -> HRESULT,
    fn GetDescription(
        pszName: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetDescription(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetWorkingDirectory(
        pszDir: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetWorkingDirectory(
        pszDir: LPCWSTR,
    ) -> HRESULT,
    fn GetArguments(
        pszArgs: LPWSTR,
        cch: c_int,
    ) -> HRESULT,
    fn SetArguments(
        pszArgs: LPCWSTR,
    ) -> HRESULT,
    fn GetHotkey(
        pwHotkey: *mut WORD,
    ) -> HRESULT,
    fn SetHotkey(
        wHotkey: WORD,
    ) -> HRESULT,
    fn GetShowCmd(
        piShowCmd: *mut c_int,
    ) -> HRESULT,
    fn SetShowCmd(
        iShowCmd: c_int,
    ) -> HRESULT,
    fn GetIconLocation(
        pszIconPath: LPWSTR,
        cch: c_int,
        piIcon: *mut c_int,
    ) -> HRESULT,
    fn SetIconLocation(
        pszIconPath: LPCWSTR,
        iIcon: c_int,
    ) -> HRESULT,
    fn SetRelativePath(
        pszPathRel: LPCWSTR,
        dwReserved: DWORD,
    ) -> HRESULT,
    fn Resolve(
        hwnd: HWND,
        fFlags: DWORD,
    ) -> HRESULT,
    fn SetPath(
        pszFile: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xc2cf3110, 0x460e, 0x4fc1, 0xb9, 0xd0, 0x8a, 0x1c, 0x0c, 0x9c, 0xc4, 0xbd)]
class DesktopWallpaper;}
RIDL!{#[uuid(0x00021400, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
class ShellDesktop;}
RIDL!{#[uuid(0xf3364ba0, 0x65b9, 0x11ce, 0xa9, 0xba, 0x00, 0xaa, 0x00, 0x4a, 0xe8, 0x37)]
class ShellFSFolder;}
RIDL!{#[uuid(0x208d2c60, 0x3aea, 0x1069, 0xa2, 0xd7, 0x08, 0x00, 0x2b, 0x30, 0x30, 0x9d)]
class NetworkPlaces;}
RIDL!{#[uuid(0x00021401, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
class ShellLink;}
RIDL!{#[uuid(0x94357b53, 0xca29, 0x4b78, 0x83, 0xae, 0xe8, 0xfe, 0x74, 0x09, 0x13, 0x4f)]
class DriveSizeCategorizer;}
RIDL!{#[uuid(0xb0a8f3cf, 0x4333, 0x4bab, 0x88, 0x73, 0x1c, 0xcb, 0x1c, 0xad, 0xa4, 0x8b)]
class DriveTypeCategorizer;}
RIDL!{#[uuid(0xb5607793, 0x24ac, 0x44c7, 0x82, 0xe2, 0x83, 0x17, 0x26, 0xaa, 0x6c, 0xb7)]
class FreeSpaceCategorizer;}
RIDL!{#[uuid(0x55d7b852, 0xf6d1, 0x42f2, 0xaa, 0x75, 0x87, 0x28, 0xa1, 0xb2, 0xd2, 0x64)]
class SizeCategorizer;}
RIDL!{#[uuid(0xd912f8cf, 0x0396, 0x4915, 0x88, 0x4e, 0xfb, 0x42, 0x5d, 0x32, 0x94, 0x3b)]
class PropertiesUI;}
RIDL!{#[uuid(0x0010890e, 0x8789, 0x413c, 0xad, 0xbc, 0x48, 0xf5, 0xb5, 0x11, 0xb3, 0xaf)]
class UserNotification;}
RIDL!{#[uuid(0x56fdf344, 0xfd6d, 0x11d0, 0x95, 0x8a, 0x00, 0x60, 0x97, 0xc9, 0xa0, 0x90)]
class TaskbarList;}
RIDL!{#[uuid(0x9ac9fbe1, 0xe0a2, 0x4ad6, 0xb4, 0xee, 0xe2, 0x12, 0x01, 0x3e, 0xa9, 0x17)]
class ShellItem;}
RIDL!{#[uuid(0x72eb61e0, 0x8672, 0x4303, 0x91, 0x75, 0xf2, 0xe4, 0xc6, 0x8b, 0x2e, 0x7c)]
class NamespaceWalker;}
RIDL!{#[uuid(0x3ad05575, 0x8857, 0x4850, 0x92, 0x77, 0x11, 0xb8, 0x5b, 0xdb, 0x8e, 0x09)]
class FileOperation;}
RIDL!{#[uuid(0xdc1c5a9c, 0xe88a, 0x4dde, 0xa5, 0xa1, 0x60, 0xf8, 0x2a, 0x20, 0xae, 0xf7)]
class FileOpenDialog;}
RIDL!{#[uuid(0xc0b4e2f3, 0xba21, 0x4773, 0x8d, 0xba, 0x33, 0x5e, 0xc9, 0x46, 0xeb, 0x8b)]
class FileSaveDialog;}
RIDL!{#[uuid(0x4df0c730, 0xdf9d, 0x4ae3, 0x91, 0x53, 0xaa, 0x6b, 0x82, 0xe9, 0x79, 0x5a)]
class KnownFolderManager;}
RIDL!{#[uuid(0x49f371e1, 0x8c5c, 0x4d9c, 0x9a, 0x3b, 0x54, 0xa6, 0x82, 0x7f, 0x51, 0x3c)]
class SharingConfigurationManager;}
RIDL!{#[uuid(0x7007acc7, 0x3202, 0x11d1, 0xaa, 0xd2, 0x00, 0x80, 0x5f, 0xc1, 0x27, 0x0e)]
class NetworkConnections;}
RIDL!{#[uuid(0xd6277990, 0x4c6a, 0x11cf, 0x8d, 0x87, 0x00, 0xaa, 0x00, 0x60, 0xf5, 0xbf)]
class ScheduledTasks;}
RIDL!{#[uuid(0x591209c7, 0x767b, 0x42b2, 0x9f, 0xba, 0x44, 0xee, 0x46, 0x15, 0xf2, 0xc7)]
class ApplicationAssociationRegistration;}
RIDL!{#[uuid(0x14010e02, 0xbbbd, 0x41f0, 0x88, 0xe3, 0xed, 0xa3, 0x71, 0x21, 0x65, 0x84)]
class SearchFolderItemFactory;}
RIDL!{#[uuid(0x06622d85, 0x6856, 0x4460, 0x8d, 0xe1, 0xa8, 0x19, 0x21, 0xb4, 0x1c, 0x4b)]
class OpenControlPanel;}
RIDL!{#[uuid(0x9e56be60, 0xc50f, 0x11cf, 0x9a, 0x2c, 0x00, 0xa0, 0xc9, 0x0a, 0x90, 0xce)]
class MailRecipient;}
RIDL!{#[uuid(0xf02c1a0d, 0xbe21, 0x4350, 0x88, 0xb0, 0x73, 0x67, 0xfc, 0x96, 0xef, 0x3c)]
class NetworkExplorerFolder;}
RIDL!{#[uuid(0x77f10cf0, 0x3db5, 0x4966, 0xb5, 0x20, 0xb7, 0xc5, 0x4f, 0xd3, 0x5e, 0xd6)]
class DestinationList;}
RIDL!{#[uuid(0x86c14003, 0x4d6b, 0x4ef3, 0xa7, 0xb4, 0x05, 0x06, 0x66, 0x3b, 0x2e, 0x68)]
class ApplicationDestinations;}
RIDL!{#[uuid(0x86bec222, 0x30f2, 0x47e0, 0x9f, 0x25, 0x60, 0xd1, 0x1c, 0xd7, 0x5c, 0x28)]
class ApplicationDocumentLists;}
RIDL!{#[uuid(0xde77ba04, 0x3c92, 0x4d11, 0xa1, 0xa5, 0x42, 0x35, 0x2a, 0x53, 0xe0, 0xe3)]
class HomeGroup;}
RIDL!{#[uuid(0xd9b3211d, 0xe57f, 0x4426, 0xaa, 0xef, 0x30, 0xa8, 0x06, 0xad, 0xd3, 0x97)]
class ShellLibrary;}
RIDL!{#[uuid(0x273eb5e7, 0x88b0, 0x4843, 0xbf, 0xef, 0xe2, 0xc8, 0x1d, 0x43, 0xaa, 0xe5)]
class AppStartupLink;}
RIDL!{#[uuid(0x2d3468c1, 0x36a7, 0x43b6, 0xac, 0x24, 0xd3, 0xf0, 0x2f, 0xd9, 0x60, 0x7a)]
class EnumerableObjectCollection;}
RIDL!{#[uuid(0xd5120aa3, 0x46ba, 0x44c5, 0x82, 0x2d, 0xca, 0x80, 0x92, 0xc1, 0xfc, 0x72)]
class FrameworkInputPane;}
RIDL!{#[uuid(0xc63382be, 0x7933, 0x48d0, 0x9a, 0xc8, 0x85, 0xfb, 0x46, 0xbe, 0x2f, 0xdd)]
class DefFolderMenu;}
RIDL!{#[uuid(0x7e5fe3d9, 0x985f, 0x4908, 0x91, 0xf9, 0xee, 0x19, 0xf9, 0xfd, 0x15, 0x14)]
class AppVisibility;}
RIDL!{#[uuid(0x4ed3a719, 0xcea8, 0x4bd9, 0x91, 0x0d, 0xe2, 0x52, 0xf9, 0x97, 0xaf, 0xc2)]
class AppShellVerbHandler;}
RIDL!{#[uuid(0xe44e9428, 0xbdbc, 0x4987, 0xa0, 0x99, 0x40, 0xdc, 0x8f, 0xd2, 0x55, 0xe7)]
class ExecuteUnknown;}
RIDL!{#[uuid(0xb1aec16f, 0x2383, 0x4852, 0xb0, 0xe9, 0x8f, 0x0b, 0x1d, 0xc6, 0x6b, 0x4d)]
class PackageDebugSettings;}
RIDL!{#[uuid(0x6b273fc5, 0x61fd, 0x4918, 0x95, 0xa2, 0xc3, 0xb5, 0xe9, 0xd7, 0xf5, 0x81)]
class SuspensionDependencyManager;}
RIDL!{#[uuid(0x45ba127d, 0x10a8, 0x46ea, 0x8a, 0xb7, 0x56, 0xea, 0x90, 0x78, 0x94, 0x3c)]
class ApplicationActivationManager;}
RIDL!{#[uuid(0x958a6fb5, 0xdcb2, 0x4faf, 0xaa, 0xfd, 0x7f, 0xb0, 0x54, 0xad, 0x1a, 0x3b)]
class ApplicationDesignModeSettings;}
extern "system" {
    pub fn SHCreateItemFromParsingName(
        pszPath: PCWSTR,
        pbc: *mut IBindCtx,
        riid: REFIID,
        ppv: *mut *mut c_void
    ) -> HRESULT;
}
