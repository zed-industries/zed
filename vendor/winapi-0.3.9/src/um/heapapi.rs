// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-heap-l1
use shared::basetsd::{PSIZE_T, SIZE_T};
use shared::minwindef::{BOOL, DWORD, LPCVOID, LPVOID};
use um::minwinbase::LPPROCESS_HEAP_ENTRY;
use um::winnt::{HANDLE, HEAP_INFORMATION_CLASS, PHANDLE, PVOID};
STRUCT!{struct HEAP_SUMMARY {
    cb: DWORD,
    cbAllocated: SIZE_T,
    cbCommitted: SIZE_T,
    cbReserved: SIZE_T,
    cbMaxReserve: SIZE_T,
}}
pub type PHEAP_SUMMARY = *mut HEAP_SUMMARY;
pub type LPHEAP_SUMMARY = PHEAP_SUMMARY;
extern "system" {
    pub fn HeapCreate(
        flOptions: DWORD,
        dwInitialSize: SIZE_T,
        dwMaximumSize: SIZE_T,
    ) -> HANDLE;
    pub fn HeapDestroy(
        hHeap: HANDLE,
    ) -> BOOL;
    pub fn HeapAlloc(
        hHeap: HANDLE,
        dwFlags: DWORD,
        dwBytes: SIZE_T,
    ) -> LPVOID;
    pub fn HeapReAlloc(
        hHeap: HANDLE,
        dwFlags: DWORD,
        lpMem: LPVOID,
        dwBytes: SIZE_T,
    ) -> LPVOID;
    pub fn HeapFree(
        hHeap: HANDLE,
        dwFlags: DWORD,
        lpMem: LPVOID,
    ) -> BOOL;
    pub fn HeapSize(
        hHeap: HANDLE,
        dwFlags: DWORD,
        lpMem: LPCVOID,
    ) -> SIZE_T;
    pub fn GetProcessHeap() -> HANDLE;
    pub fn HeapCompact(
        hHeap: HANDLE,
        dwFlags: DWORD,
    ) -> SIZE_T;
    pub fn HeapSetInformation(
        HeapHandle: HANDLE,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
    ) -> BOOL;
    pub fn HeapValidate(
        hHeap: HANDLE,
        dwFlags: DWORD,
        lpMem: LPCVOID,
    ) -> BOOL;
    pub fn HeapSummary(
        hHeap: HANDLE,
        dwFlags: DWORD,
        lpSummary: LPHEAP_SUMMARY,
    ) -> BOOL;
    pub fn GetProcessHeaps(
        NumberOfHeaps: DWORD,
        ProcessHeaps: PHANDLE,
    ) -> DWORD;
    pub fn HeapLock(
        hHeap: HANDLE,
    ) -> BOOL;
    pub fn HeapUnlock(
        hHeap: HANDLE,
    ) -> BOOL;
    pub fn HeapWalk(
        hHeap: HANDLE,
        lpEntry: LPPROCESS_HEAP_ENTRY,
    ) -> BOOL;
    pub fn HeapQueryInformation(
        HeapHandle: HANDLE,
        HeapInformationClass: HEAP_INFORMATION_CLASS,
        HeapInformation: PVOID,
        HeapInformationLength: SIZE_T,
        ReturnLength: PSIZE_T,
    ) -> BOOL;
}
