// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{BOOL, DWORD};
use um::winnt::{HANDLE, LPCH, LPCSTR, LPCWSTR, LPSTR, LPWCH, LPWSTR, PHANDLE};
extern "system" {
    pub fn GetEnvironmentStrings() -> LPCH;
    pub fn GetEnvironmentStringsW() -> LPWCH;
    pub fn SetEnvironmentStringsW(
        NewEnvironment: LPWCH,
    ) -> BOOL;
    pub fn FreeEnvironmentStringsA(
        penv: LPCH,
    ) -> BOOL;
    pub fn FreeEnvironmentStringsW(
        penv: LPWCH,
    ) -> BOOL;
    pub fn GetStdHandle(
        nStdHandle: DWORD,
    ) -> HANDLE;
    pub fn SetStdHandle(
        nStdHandle: DWORD,
        hHandle: HANDLE,
    ) -> BOOL;
    pub fn SetStdHandleEx(
        nStdHandle: DWORD,
        hHandle: HANDLE,
        phPrevValue: PHANDLE,
    ) -> BOOL;
    pub fn GetCommandLineA() -> LPSTR;
    pub fn GetCommandLineW() -> LPWSTR;
    pub fn GetEnvironmentVariableA(
        lpName: LPCSTR,
        lpBuffer: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetEnvironmentVariableW(
        lpName: LPCWSTR,
        lpBuffer: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn SetEnvironmentVariableA(
        lpName: LPCSTR,
        lpValue: LPCSTR,
    ) -> BOOL;
    pub fn SetEnvironmentVariableW(
        lpName: LPCWSTR,
        lpValue: LPCWSTR,
    ) -> BOOL;
    pub fn ExpandEnvironmentStringsA(
        lpSrc: LPCSTR,
        lpDst: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn ExpandEnvironmentStringsW(
        lpSrc: LPCWSTR,
        lpDst: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn SetCurrentDirectoryA(
        lpPathName: LPCSTR,
    ) -> BOOL;
    pub fn SetCurrentDirectoryW(
        lpPathName: LPCWSTR,
    ) -> BOOL;
    pub fn GetCurrentDirectoryA(
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
    ) -> DWORD;
    pub fn GetCurrentDirectoryW(
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
    ) -> DWORD;
    pub fn SearchPathW(
        lpPath: LPCWSTR,
        lpFileName: LPCWSTR,
        lpExtension: LPCWSTR,
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
        lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn SearchPathA(
        lpPath: LPCSTR,
        lpFileName: LPCSTR,
        lpExtension: LPCSTR,
        nBufferLength: DWORD,
        lpBuffer: LPSTR,
        lpFilePart: *mut LPSTR,
    ) -> DWORD;
    pub fn NeedCurrentDirectoryForExePathA(
        ExeName: LPCSTR,
    ) -> BOOL;
    pub fn NeedCurrentDirectoryForExePathW(
        ExeName: LPCWSTR,
    ) -> BOOL;
}
