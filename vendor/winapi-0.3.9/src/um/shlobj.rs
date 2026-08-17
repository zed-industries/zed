// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::{c_int, c_void};
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::HWND;
use um::minwinbase::SECURITY_ATTRIBUTES;
use um::shtypes::{PCIDLIST_ABSOLUTE, PCUITEMID_CHILD_ARRAY, PIDLIST_ABSOLUTE, REFKNOWNFOLDERID};
use um::winnt::{HANDLE, HRESULT, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, PWSTR};
pub const IDO_SHGIOI_SHARE: c_int = 0x0FFFFFFF;
pub const IDO_SHGIOI_LINK: c_int = 0x0FFFFFFE;
// Yes, these values are supposed to be 9 digits
pub const IDO_SHGIOI_SLOWFILE: c_int = 0x0FFFFFFFD;
pub const IDO_SHGIOI_DEFAULT: c_int = 0x0FFFFFFFC;
extern "system" {
    pub fn SHGetIconOverlayIndexA(
        pszIconPath: LPCSTR,
        iIconIndex: c_int,
    ) -> c_int;
    pub fn SHGetIconOverlayIndexW(
        pszIconPath: LPCWSTR,
        iIconIndex: c_int,
    ) -> c_int;
}
pub const GPFIDL_DEFAULT: GPFIDL_FLAGS = 0x0000;
pub const GPFIDL_ALTNAME: GPFIDL_FLAGS = 0x0001;
pub const GPFIDL_UNCPRINTER: GPFIDL_FLAGS = 0x0002;
pub type GPFIDL_FLAGS = c_int;
extern "system" {
    pub fn SHGetPathFromIDListEx(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: PWSTR,
        cchPath: DWORD,
        uOpts: GPFIDL_FLAGS,
    ) -> BOOL;
    pub fn SHGetPathFromIDListA(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: LPSTR,
    ) -> BOOL;
    pub fn SHGetPathFromIDListW(
        pidl: PCIDLIST_ABSOLUTE,
        pszPath: LPWSTR,
    ) -> BOOL;
    pub fn SHCreateDirectory(
        hwnd: HWND,
        pszPath: PCWSTR,
    ) -> c_int;
    pub fn SHCreateDirectoryExA(
        hwnd: HWND,
        pszPath: LPCSTR,
        psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
    pub fn SHCreateDirectoryExW(
        hwnd: HWND,
        pszPath: LPCWSTR,
        psa: *const SECURITY_ATTRIBUTES,
    ) -> c_int;
}
pub const OFASI_EDIT: DWORD = 0x0001;
pub const OFASI_OPENDESKTOP: DWORD = 0x0002;
extern "system" {
    pub fn SHOpenFolderAndSelectItems(
        pidlFolder: PCIDLIST_ABSOLUTE,
        cidl: UINT,
        apidl: PCUITEMID_CHILD_ARRAY,
        dwFlags: DWORD,
    ) -> HRESULT;
    //pub fn SHCreateShellItem(
    //    pidlParent: PCIDLIST_ABSOLUTE,
    //    psfParent: *mut IShellFolder,
    //    pidl: PCUITEMID_CHILD,
    //    ppsi: *mut *mut IShellItem,
    //) -> HRESULT;
}
pub const CSIDL_DESKTOP: c_int = 0x0000;
pub const CSIDL_INTERNET: c_int = 0x0001;
pub const CSIDL_PROGRAMS: c_int = 0x0002;
pub const CSIDL_CONTROLS: c_int = 0x0003;
pub const CSIDL_PRINTERS: c_int = 0x0004;
pub const CSIDL_PERSONAL: c_int = 0x0005;
pub const CSIDL_FAVORITES: c_int = 0x0006;
pub const CSIDL_STARTUP: c_int = 0x0007;
pub const CSIDL_RECENT: c_int = 0x0008;
pub const CSIDL_SENDTO: c_int = 0x0009;
pub const CSIDL_BITBUCKET: c_int = 0x000a;
pub const CSIDL_STARTMENU: c_int = 0x000b;
pub const CSIDL_MYDOCUMENTS: c_int = CSIDL_PERSONAL;
pub const CSIDL_MYMUSIC: c_int = 0x000d;
pub const CSIDL_MYVIDEO: c_int = 0x000e;
pub const CSIDL_DESKTOPDIRECTORY: c_int = 0x0010;
pub const CSIDL_DRIVES: c_int = 0x0011;
pub const CSIDL_NETWORK: c_int = 0x0012;
pub const CSIDL_NETHOOD: c_int = 0x0013;
pub const CSIDL_FONTS: c_int = 0x0014;
pub const CSIDL_TEMPLATES: c_int = 0x0015;
pub const CSIDL_COMMON_STARTMENU: c_int = 0x0016;
pub const CSIDL_COMMON_PROGRAMS: c_int = 0x0017;
pub const CSIDL_COMMON_STARTUP: c_int = 0x0018;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: c_int = 0x0019;
pub const CSIDL_APPDATA: c_int = 0x001a;
pub const CSIDL_PRINTHOOD: c_int = 0x001b;
pub const CSIDL_LOCAL_APPDATA: c_int = 0x001c;
pub const CSIDL_ALTSTARTUP: c_int = 0x001d;
pub const CSIDL_COMMON_ALTSTARTUP: c_int = 0x001e;
pub const CSIDL_COMMON_FAVORITES: c_int = 0x001f;
pub const CSIDL_INTERNET_CACHE: c_int = 0x0020;
pub const CSIDL_COOKIES: c_int = 0x0021;
pub const CSIDL_HISTORY: c_int = 0x0022;
pub const CSIDL_COMMON_APPDATA: c_int = 0x0023;
pub const CSIDL_WINDOWS: c_int = 0x0024;
pub const CSIDL_SYSTEM: c_int = 0x0025;
pub const CSIDL_PROGRAM_FILES: c_int = 0x0026;
pub const CSIDL_MYPICTURES: c_int = 0x0027;
pub const CSIDL_PROFILE: c_int = 0x0028;
pub const CSIDL_SYSTEMX86: c_int = 0x0029;
pub const CSIDL_PROGRAM_FILESX86: c_int = 0x002a;
pub const CSIDL_PROGRAM_FILES_COMMON: c_int = 0x002b;
pub const CSIDL_PROGRAM_FILES_COMMONX86: c_int = 0x002c;
pub const CSIDL_COMMON_TEMPLATES: c_int = 0x002d;
pub const CSIDL_COMMON_DOCUMENTS: c_int = 0x002e;
pub const CSIDL_COMMON_ADMINTOOLS: c_int = 0x002f;
pub const CSIDL_ADMINTOOLS: c_int = 0x0030;
pub const CSIDL_CONNECTIONS: c_int = 0x0031;
pub const CSIDL_COMMON_MUSIC: c_int = 0x0035;
pub const CSIDL_COMMON_PICTURES: c_int = 0x0036;
pub const CSIDL_COMMON_VIDEO: c_int = 0x0037;
pub const CSIDL_RESOURCES: c_int = 0x0038;
pub const CSIDL_RESOURCES_LOCALIZED: c_int = 0x0039;
pub const CSIDL_COMMON_OEM_LINKS: c_int = 0x003a;
pub const CSIDL_CDBURN_AREA: c_int = 0x003b;
pub const CSIDL_COMPUTERSNEARME: c_int = 0x003d;
pub const CSIDL_FLAG_CREATE: c_int = 0x8000;
pub const CSIDL_FLAG_DONT_VERIFY: c_int = 0x4000;
pub const CSIDL_FLAG_DONT_UNEXPAND: c_int = 0x2000;
pub const CSIDL_FLAG_NO_ALIAS: c_int = 0x1000;
pub const CSIDL_FLAG_PER_USER_INIT: c_int = 0x0800;
pub const CSIDL_FLAG_MASK: c_int = 0xff00;
extern "system" {
    pub fn SHGetSpecialFolderLocation(
        hwnd: HWND,
        csidl: c_int,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHCloneSpecialIDList(
        hwnd: HWND,
        csidl: c_int,
        fCreate: BOOL,
    ) -> PIDLIST_ABSOLUTE;
    pub fn SHGetSpecialFolderPathA(
        hwnd: HWND,
        pszPath: LPSTR,
        csidl: c_int,
        fCreate: BOOL,
    ) -> BOOL;
    pub fn SHGetSpecialFolderPathW(
        hwnd: HWND,
        pszPath: LPWSTR,
        csidl: c_int,
        fCreate: BOOL,
    ) -> BOOL;
    pub fn SHFlushSFCache();
}
ENUM!{enum SHGFP_TYPE {
    SHGFP_TYPE_CURRENT = 0,
    SHGFP_TYPE_DEFAULT = 1,
}}
extern "system" {
    pub fn SHGetFolderPathA(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathW(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPWSTR,
    ) -> HRESULT;
    pub fn SHGetFolderLocation(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHSetFolderPathA(
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPCSTR,
    ) -> HRESULT;
    pub fn SHSetFolderPathW(
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszPath: LPCWSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirA(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszSubDir: LPCSTR,
        pszPath: LPSTR,
    ) -> HRESULT;
    pub fn SHGetFolderPathAndSubDirW(
        hwnd: HWND,
        csidl: c_int,
        hToken: HANDLE,
        dwFlags: DWORD,
        pszSubDir: LPCWSTR,
        pszPath: LPWSTR,
    ) -> HRESULT;
}
ENUM!{enum KNOWN_FOLDER_FLAG {
    KF_FLAG_DEFAULT = 0x00000000,
    KF_FLAG_NO_APPCONTAINER_REDIRECTION = 0x00010000,
    KF_FLAG_CREATE = 0x00008000,
    KF_FLAG_DONT_VERIFY = 0x00004000,
    KF_FLAG_DONT_UNEXPAND = 0x00002000,
    KF_FLAG_NO_ALIAS = 0x00001000,
    KF_FLAG_INIT = 0x00000800,
    KF_FLAG_DEFAULT_PATH = 0x00000400,
    KF_FLAG_NOT_PARENT_RELATIVE = 0x00000200,
    KF_FLAG_SIMPLE_IDLIST = 0x00000100,
    KF_FLAG_ALIAS_ONLY = 0x80000000,
}}
extern "system" {
    pub fn SHGetKnownFolderIDList(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        ppidl: *mut PIDLIST_ABSOLUTE,
    ) -> HRESULT;
    pub fn SHSetKnownFolderPath(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        pszPath: PCWSTR,
    ) -> HRESULT;
    pub fn SHGetKnownFolderPath(
        rfid: REFKNOWNFOLDERID,
        dwFlags: DWORD,
        hToken: HANDLE,
        pszPath: *mut PWSTR,
    ) -> HRESULT;
    pub fn SHGetKnownFolderItem(
        rfid: REFKNOWNFOLDERID,
        flags: KNOWN_FOLDER_FLAG,
        hToken: HANDLE,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT;
}
