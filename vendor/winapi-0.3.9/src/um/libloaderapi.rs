// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-libraryloader-l1
use ctypes::c_int;
use shared::basetsd::LONG_PTR;
use shared::minwindef::{
    BOOL, DWORD, FARPROC, HGLOBAL, HINSTANCE, HMODULE, HRSRC, LPVOID, UINT, WORD
};
use um::winnt::{HANDLE, LANGID, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PCWSTR, PVOID};
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: DWORD = 0x00000001;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: DWORD = 0x00000002;
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: DWORD = 0x00000004;
pub const DONT_RESOLVE_DLL_REFERENCES: DWORD = 0x00000001;
pub const LOAD_LIBRARY_AS_DATAFILE: DWORD = 0x00000002;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: DWORD = 0x00000008;
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: DWORD = 0x00000010;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: DWORD = 0x00000020;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: DWORD = 0x00000040;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: DWORD = 0x00000080;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: DWORD = 0x00000100;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: DWORD = 0x00000200;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: DWORD = 0x00000400;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: DWORD = 0x00000800;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: DWORD = 0x00001000;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: DWORD = 0x00002000;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: DWORD = 0x00004000;
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: DWORD = 0x00008000;
FN!{stdcall ENUMRESLANGPROCA(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpName: LPCSTR,
    wLanguage: WORD,
    lParam: LONG_PTR,
) -> BOOL}
FN!{stdcall ENUMRESLANGPROCW(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpName: LPCWSTR,
    wLanguage: WORD,
    lParam: LONG_PTR,
) -> BOOL}
FN!{stdcall ENUMRESNAMEPROCA(
    hModule: HMODULE,
    lpType: LPCSTR,
    lpName: LPSTR,
    lParam: LONG_PTR,
) -> BOOL}
FN!{stdcall ENUMRESNAMEPROCW(
    hModule: HMODULE,
    lpType: LPCWSTR,
    lpName: LPWSTR,
    lParam: LONG_PTR,
) -> BOOL}
FN!{stdcall ENUMRESTYPEPROCA(
    hModule: HMODULE,
    lpType: LPSTR,
    lParam: LONG_PTR,
) -> BOOL}
FN!{stdcall ENUMRESTYPEPROCW(
    hModule: HMODULE,
    lpType: LPWSTR,
    lParam: LONG_PTR,
) -> BOOL}
extern "system" {
    pub fn DisableThreadLibraryCalls(
        hLibModule: HMODULE,
    ) -> BOOL;
    pub fn FindResourceExW(
        hModule: HMODULE,
        lpName: LPCWSTR,
        lpType: LPCWSTR,
        wLanguage: WORD,
    ) -> HRSRC;
    pub fn FindStringOrdinal(
        dwFindStringOrdinalFlags: DWORD,
        lpStringSource: LPCWSTR,
        cchSource: c_int,
        lpStringValue: LPCWSTR,
        cchValue: c_int,
        bIgnoreCase: BOOL,
    ) -> c_int;
    pub fn FreeLibrary(
        hLibModule: HMODULE,
    ) -> BOOL;
    pub fn FreeLibraryAndExitThread(
        hLibModule: HMODULE,
        dwExitCode: DWORD,
    );
    pub fn FreeResource(
        hResData: HGLOBAL,
    ) -> BOOL;
    pub fn GetModuleFileNameA(
        hModule: HMODULE,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleFileNameW(
        hModule: HMODULE,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleHandleA(
        lpModuleName: LPCSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleW(
        lpModuleName: LPCWSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleExA(
        dwFlags: DWORD,
        lpModuleName: LPCSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleExW(
        dwFlags: DWORD,
        lpModuleName: LPCWSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetProcAddress(
        hModule: HMODULE,
        lpProcName: LPCSTR,
    ) -> FARPROC;
    pub fn LoadLibraryExA(
        lpLibFileName: LPCSTR,
        hFile: HANDLE,
        dwFlags: DWORD,
    ) -> HMODULE;
    pub fn LoadLibraryExW(
        lpLibFileName: LPCWSTR,
        hFile: HANDLE,
        dwFlags: DWORD,
    ) -> HMODULE;
    pub fn LoadResource(
        hModule: HMODULE,
        hResInfo: HRSRC,
    ) -> HGLOBAL;
    pub fn LoadStringA(
        hInstance: HINSTANCE,
        uID: UINT,
        lpBuffer: LPSTR,
        cchBufferMax: c_int,
    ) -> c_int;
    pub fn LoadStringW(
        hInstance: HINSTANCE,
        uID: UINT,
        lpBuffer: LPWSTR,
        cchBufferMax: c_int,
    ) -> c_int;
    pub fn LockResource(
        hResData: HGLOBAL,
    ) -> LPVOID;
    pub fn SizeofResource(
        hModule: HMODULE,
        hResInfo: HRSRC,
    ) -> DWORD;
}
pub type DLL_DIRECTORY_COOKIE = PVOID;
pub type PDLL_DIRECTORY_COOKIE = *mut PVOID;
extern "system" {
    pub fn AddDllDirectory(
        NewDirectory: PCWSTR,
    ) -> DLL_DIRECTORY_COOKIE;
    pub fn RemoveDllDirectory(
        Cookie: DLL_DIRECTORY_COOKIE,
    ) -> BOOL;
    pub fn SetDefaultDllDirectories(
        DirectoryFlags: DWORD,
    ) -> BOOL;
    pub fn EnumResourceLanguagesExA(
        hModule: HMODULE,
        lpType: LPCSTR,
        lpName: LPCSTR,
        lpEnumFunc: ENUMRESLANGPROCA,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn EnumResourceLanguagesExW(
        hModule: HMODULE,
        lpType: LPCWSTR,
        lpName: LPCWSTR,
        lpEnumFunc: ENUMRESLANGPROCW,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn EnumResourceNamesExA(
        hModule: HMODULE,
        lpType: LPCSTR,
        lpEnumFunc: ENUMRESNAMEPROCA,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn EnumResourceNamesExW(
        hModule: HMODULE,
        lpType: LPCWSTR,
        lpEnumFunc: ENUMRESNAMEPROCW,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn EnumResourceTypesExA(
        hModule: HMODULE,
        lpEnumFunc: ENUMRESTYPEPROCA,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn EnumResourceTypesExW(
        hModule: HMODULE,
        lpEnumFunc: ENUMRESTYPEPROCW,
        lParam: LONG_PTR,
        dwFlags: DWORD,
        LangId: LANGID,
    ) -> BOOL;
    pub fn FindResourceW(
        hModule: HMODULE,
        lpName: LPCWSTR,
        lpType: LPCWSTR,
    ) -> HRSRC;
    pub fn LoadLibraryA(
        lpFileName: LPCSTR,
    ) -> HMODULE;
    pub fn LoadLibraryW(
        lpFileName: LPCWSTR,
    ) -> HMODULE;
    pub fn EnumResourceNamesW(
        hModule: HMODULE,
        lpType: LPCWSTR,
        lpEnumFunc: ENUMRESNAMEPROCW,
        lParam: LONG_PTR,
    ) -> BOOL;
}
