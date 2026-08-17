// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! Version management functions, types, and definitions
use ctypes::c_void;
use shared::minwindef::{BOOL, DWORD, LPCVOID, LPVOID, PUINT};
use um::winnt::{LPCSTR, LPCWSTR, LPSTR, LPWSTR};
extern "system" {
    pub fn GetFileVersionInfoSizeA(
        lptstrFilename: LPCSTR,
        lpdwHandle: *mut DWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoSizeW(
        lptstrFilename: LPCWSTR,
        lpdwHandle: *mut DWORD,
    ) -> DWORD;
    pub fn GetFileVersionInfoA(
        lptstrFilename: LPCSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: *mut c_void,
    ) -> BOOL;
    pub fn GetFileVersionInfoW(
        lptstrFilename: LPCWSTR,
        dwHandle: DWORD,
        dwLen: DWORD,
        lpData: *mut c_void,
    ) -> BOOL;
    pub fn VerQueryValueA(
        pBlock: LPCVOID,
        lpSubBlock: LPCSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;
    pub fn VerQueryValueW(
        pBlock: LPCVOID,
        lpSubBlock: LPCWSTR,
        lplpBuffer: &mut LPVOID,
        puLen: PUINT,
    ) -> BOOL;
    pub fn VerLanguageNameA(
        wLang: DWORD,
        szLang: LPSTR,
        cchLang: DWORD,
    ) -> DWORD;
    pub fn VerLanguageNameW(
        wLang: DWORD,
        szLang: LPWSTR,
        cchLang: DWORD,
    ) -> DWORD;
}
