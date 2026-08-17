// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! WIN32 tool help functions, types, and definitions
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::minwindef::{BOOL, BYTE, DWORD, HMODULE, LPCVOID, LPVOID, MAX_PATH};
use um::winnt::{CHAR, HANDLE, LONG, WCHAR};
pub const MAX_MODULE_NAME32: usize = 255;
extern "system" {
    pub fn CreateToolhelp32Snapshot(
        dwFlags: DWORD,
        th32ProcessID: DWORD,
    ) -> HANDLE;
}
pub const TH32CS_SNAPHEAPLIST: DWORD = 0x00000001;
pub const TH32CS_SNAPPROCESS: DWORD = 0x00000002;
pub const TH32CS_SNAPTHREAD: DWORD = 0x00000004;
pub const TH32CS_SNAPMODULE: DWORD = 0x00000008;
pub const TH32CS_SNAPMODULE32: DWORD = 0x00000010;
pub const TH32CS_SNAPALL: DWORD =
    TH32CS_SNAPHEAPLIST | TH32CS_SNAPPROCESS | TH32CS_SNAPTHREAD | TH32CS_SNAPMODULE;
pub const TH32CS_INHERIT: DWORD = 0x80000000;
STRUCT!{struct HEAPLIST32 {
    dwSize: SIZE_T,
    th32ProcessID: DWORD,
    th32HeapID: ULONG_PTR,
    dwFlags: DWORD,
}}
pub type PHEAPLIST32 = *mut HEAPLIST32;
pub type LPHEAPLIST32 = *mut HEAPLIST32;
pub const HF32_DEFAULT: DWORD = 1;
pub const HF32_SHARED: DWORD = 2;
extern "system" {
    pub fn Heap32ListFirst(
        hSnapshot: HANDLE,
        lphl: LPHEAPLIST32,
    ) -> BOOL;
    pub fn Heap32ListNext(
        hSnapshot: HANDLE,
        lphl: LPHEAPLIST32,
    ) -> BOOL;
}
STRUCT!{struct HEAPENTRY32 {
    dwSize: SIZE_T,
    hHandle: HANDLE,
    dwAddress: ULONG_PTR,
    dwBlockSize: SIZE_T,
    dwFlags: DWORD,
    dwLockCount: DWORD,
    dwResvd: DWORD,
    th32ProcessID: DWORD,
    th32HeapID: ULONG_PTR,
}}
pub type PHEAPENTRY32 = *mut HEAPENTRY32;
pub type LPHEAPENTRY32 = *mut HEAPENTRY32;
pub const LF32_FIXED: DWORD = 0x00000001;
pub const LF32_FREE: DWORD = 0x00000002;
pub const LF32_MOVEABLE: DWORD = 0x00000004;
extern "system" {
    pub fn Heap32First(
        lphe: LPHEAPENTRY32,
        th32ProcessID: DWORD,
        th32HeapID: ULONG_PTR,
    ) -> BOOL;
    pub fn Heap32Next(
        lphe: LPHEAPENTRY32,
    ) -> BOOL;
    pub fn Toolhelp32ReadProcessMemory(
        th32ProcessID: DWORD,
        lpBaseAddress: LPCVOID,
        lpBuffer: LPVOID,
        cbRead: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
}
STRUCT!{struct PROCESSENTRY32W {
    dwSize: DWORD,
    cntUsage: DWORD,
    th32ProcessID: DWORD,
    th32DefaultHeapID: ULONG_PTR,
    th32ModuleID: DWORD,
    cntThreads: DWORD,
    th32ParentProcessID: DWORD,
    pcPriClassBase: LONG,
    dwFlags: DWORD,
    szExeFile: [WCHAR; MAX_PATH],
}}
pub type PPROCESSENTRY32W = *mut PROCESSENTRY32W;
pub type LPPROCESSENTRY32W = *mut PROCESSENTRY32W;
extern "system" {
    pub fn Process32FirstW(
        hSnapshot: HANDLE,
        lppe: LPPROCESSENTRY32W,
    ) -> BOOL;
    pub fn Process32NextW(
        hSnapshot: HANDLE,
        lppe: LPPROCESSENTRY32W,
    ) -> BOOL;
}
STRUCT!{struct PROCESSENTRY32 {
    dwSize: DWORD,
    cntUsage: DWORD,
    th32ProcessID: DWORD,
    th32DefaultHeapID: ULONG_PTR,
    th32ModuleID: DWORD,
    cntThreads: DWORD,
    th32ParentProcessID: DWORD,
    pcPriClassBase: LONG,
    dwFlags: DWORD,
    szExeFile: [CHAR; MAX_PATH],
}}
pub type PPROCESSENTRY32 = *mut PROCESSENTRY32;
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;
extern "system" {
    pub fn Process32First(
        hSnapshot: HANDLE,
        lppe: LPPROCESSENTRY32,
    ) -> BOOL;
    pub fn Process32Next(
        hSnapshot: HANDLE,
        lppe: LPPROCESSENTRY32,
    ) -> BOOL;
}
STRUCT!{struct THREADENTRY32 {
    dwSize: DWORD,
    cntUsage: DWORD,
    th32ThreadID: DWORD,
    th32OwnerProcessID: DWORD,
    tpBasePri: LONG,
    tpDeltaPri: LONG,
    dwFlags: DWORD,
}}
pub type PTHREADENTRY32 = *mut THREADENTRY32;
pub type LPTHREADENTRY32 = *mut THREADENTRY32;
extern "system" {
    pub fn Thread32First(
        hSnapshot: HANDLE,
        lpte: LPTHREADENTRY32,
    ) -> BOOL;
    pub fn Thread32Next(
        hSnapshot: HANDLE,
        lpte: LPTHREADENTRY32,
    ) -> BOOL;
}
STRUCT!{struct MODULEENTRY32W {
    dwSize: DWORD,
    th32ModuleID: DWORD,
    th32ProcessID: DWORD,
    GlblcntUsage: DWORD,
    ProccntUsage: DWORD,
    modBaseAddr: *mut BYTE,
    modBaseSize: DWORD,
    hModule: HMODULE,
    szModule: [WCHAR; MAX_MODULE_NAME32 + 1],
    szExePath: [WCHAR; MAX_PATH],
}}
pub type PMODULEENTRY32W = *mut MODULEENTRY32W;
pub type LPMODULEENTRY32W = *mut MODULEENTRY32W;
extern "system" {
    pub fn Module32FirstW(
        hSnapshot: HANDLE,
        lpme: LPMODULEENTRY32W,
    ) -> BOOL;
    pub fn Module32NextW(
        hSnapshot: HANDLE,
        lpme: LPMODULEENTRY32W,
    ) -> BOOL;
}
STRUCT!{struct MODULEENTRY32 {
    dwSize: DWORD,
    th32ModuleID: DWORD,
    th32ProcessID: DWORD,
    GlblcntUsage: DWORD,
    ProccntUsage: DWORD,
    modBaseAddr: *mut BYTE,
    modBaseSize: DWORD,
    hModule: HMODULE,
    szModule: [CHAR; MAX_MODULE_NAME32 + 1],
    szExePath: [CHAR; MAX_PATH],
}}
pub type PMODULEENTRY32 = *mut MODULEENTRY32;
pub type LPMODULEENTRY32 = *mut MODULEENTRY32;
extern "system" {
    pub fn Module32First(
        hSnapshot: HANDLE,
        lpme: LPMODULEENTRY32,
    ) -> BOOL;
    pub fn Module32Next(
        hSnapshot: HANDLE,
        lpme: LPMODULEENTRY32,
    ) -> BOOL;
}
