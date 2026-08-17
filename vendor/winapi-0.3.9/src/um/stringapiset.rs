// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_int;
use shared::minwindef::{BOOL, DWORD, LPARAM, LPBOOL, LPVOID, LPWORD, UINT};
use um::winnls::LPNLSVERSIONINFO;
use um::winnt::{LCID, LPCSTR, LPCWCH, LPCWSTR, LPSTR, LPWSTR, PCNZWCH};
extern "system" {
    pub fn CompareStringEx(
        lpLocaleName: LPCWSTR,
        dwCmpFlags: DWORD,
        lpString1: LPCWCH,
        cchCount1: c_int,
        lpString2: LPCWCH,
        cchCount2: c_int,
        lpVersionInformation: LPNLSVERSIONINFO,
        lpReserved: LPVOID,
        lParam: LPARAM,
    ) -> c_int;
    pub fn CompareStringOrdinal(
        lpString1: LPCWCH,
        cchCount1: c_int,
        lpString2: LPCWCH,
        cchCount2: c_int,
        bIgnoreCase: BOOL,
    ) -> c_int;
    pub fn CompareStringW(
        Locale: LCID,
        dwCmpFlags: DWORD,
        lpString1: PCNZWCH,
        cchCount1: c_int,
        lpString2: PCNZWCH,
        cchCount2: c_int,
    ) -> c_int;
    pub fn FoldStringW(
        dwMapFlags: DWORD,
        lpSrcStr: LPCWCH,
        cchSrc: c_int,
        lpDestStr: LPWSTR,
        cchDest: c_int,
    ) -> c_int;
    pub fn GetStringTypeExW(
        Locale: LCID,
        dwInfoType: DWORD,
        lpSrcStr: LPCWCH,
        cchSrc: c_int,
        lpCharType: LPWORD,
    ) -> BOOL;
    pub fn GetStringTypeW(
        dwInfoType: DWORD,
        lpSrcStr: LPCWCH,
        cchSrc: c_int,
        lpCharType: LPWORD,
    ) -> BOOL;
    pub fn MultiByteToWideChar(
        CodePage: UINT,
        dwFlags: DWORD,
        lpMultiByteStr: LPCSTR,
        cbMultiByte: c_int,
        lpWideCharStr: LPWSTR,
        cchWideChar: c_int,
    ) -> c_int;
    pub fn WideCharToMultiByte(
        CodePage: UINT,
        dwFlags: DWORD,
        lpWideCharStr: LPCWSTR,
        cchWideChar: c_int,
        lpMultiByteStr: LPSTR,
        cbMultiByte: c_int,
        lpDefaultChar: LPCSTR,
        lpUsedDefaultChar: LPBOOL,
    ) -> c_int;
}
