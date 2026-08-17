// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-memory-l1-1-0
use ctypes::c_void;
use shared::basetsd::{PSIZE_T, PULONG_PTR, SIZE_T, ULONG64, ULONG_PTR};
use shared::minwindef::{
    BOOL, DWORD, LPCVOID, LPDWORD, LPVOID, PBOOL, PDWORD, PULONG, UINT, ULONG,
};
use um::minwinbase::{LPSECURITY_ATTRIBUTES, PSECURITY_ATTRIBUTES};
use um::winnt::{
    HANDLE, LPCWSTR, PCWSTR, PMEMORY_BASIC_INFORMATION, PVOID, SECTION_ALL_ACCESS,
    SECTION_MAP_EXECUTE_EXPLICIT, SECTION_MAP_READ, SECTION_MAP_WRITE,
};
pub const FILE_MAP_WRITE: DWORD = SECTION_MAP_WRITE;
pub const FILE_MAP_READ: DWORD = SECTION_MAP_READ;
pub const FILE_MAP_ALL_ACCESS: DWORD = SECTION_ALL_ACCESS;
pub const FILE_MAP_EXECUTE: DWORD = SECTION_MAP_EXECUTE_EXPLICIT;
pub const FILE_MAP_COPY: DWORD = 0x00000001;
pub const FILE_MAP_RESERVE: DWORD = 0x80000000;
pub const FILE_MAP_TARGETS_INVALID: DWORD = 0x40000000;
pub const FILE_MAP_LARGE_PAGES: DWORD = 0x20000000;
extern "system" {
    pub fn VirtualAlloc(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;
    pub fn VirtualProtect(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flNewProtect: DWORD,
        lpflOldProtect: PDWORD,
    ) -> BOOL;
    pub fn VirtualFree(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        dwFreeType: DWORD,
    ) -> BOOL;
    pub fn VirtualQuery(
        lpAddress: LPCVOID,
        lpBuffer: PMEMORY_BASIC_INFORMATION,
        dwLength: SIZE_T,
    ) -> SIZE_T;
    pub fn VirtualAllocEx(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;
    pub fn VirtualFreeEx(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        dwFreeType: DWORD,
    ) -> BOOL;
    pub fn VirtualProtectEx(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flNewProtect: DWORD,
        lpflOldProtect: PDWORD,
    ) -> BOOL;
    pub fn VirtualQueryEx(
        hProcess: HANDLE,
        lpAddress: LPCVOID,
        lpBuffer: PMEMORY_BASIC_INFORMATION,
        dwLength: SIZE_T,
    ) -> SIZE_T;
    pub fn ReadProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPCVOID,
        lpBuffer: LPVOID,
        nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;
    pub fn WriteProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPVOID,
        lpBuffer: LPCVOID,
        nSize: SIZE_T,
        lpNumberOfBytesWritten: *mut SIZE_T,
    ) -> BOOL;
    pub fn CreateFileMappingW(
        hFile: HANDLE,
        lpFileMappingAttributes: LPSECURITY_ATTRIBUTES,
        flProtect: DWORD,
        dwMaximumSizeHigh: DWORD,
        dwMaximumSizeLow: DWORD,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn OpenFileMappingW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn MapViewOfFile(
        hFileMappingObject: HANDLE,
        dwDesiredAccess: DWORD,
        dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD,
        dwNumberOfBytesToMap: SIZE_T,
    ) -> LPVOID;
    pub fn MapViewOfFileEx(
        hFileMappingObject: HANDLE,
        dwDesiredAccess: DWORD,
        dwFileOffsetHigh: DWORD,
        dwFileOffsetLow: DWORD,
        dwNumberOfBytesToMap: SIZE_T,
        lpBaseAddress: LPVOID,
    ) -> LPVOID;
    pub fn FlushViewOfFile(
        lpBaseAddress: LPCVOID,
        dwNumberOfBytesToFlush: SIZE_T,
    ) -> BOOL;
    pub fn UnmapViewOfFile(
        lpBaseAddress: LPCVOID,
    ) -> BOOL;
    pub fn GetLargePageMinimum() -> SIZE_T;
    pub fn GetProcessWorkingSetSizeEx(
        hProcess: HANDLE,
        lpMinimumWorkingSetSize: PSIZE_T,
        lpMaximumWorkingSetSize: PSIZE_T,
        Flags: PDWORD,
    ) -> BOOL;
    pub fn SetProcessWorkingSetSizeEx(
        hProcess: HANDLE,
        dwMinimumWorkingSetSize: SIZE_T,
        dwMaximumWorkingSetSize: SIZE_T,
        Flags: DWORD,
    ) -> BOOL;
    pub fn VirtualLock(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
    ) -> BOOL;
    pub fn VirtualUnlock(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
    ) -> BOOL;
    pub fn GetWriteWatch(
        dwFlags: DWORD,
        lpBaseAddress: PVOID,
        dwRegionSize: SIZE_T,
        lpAddresses: *mut PVOID,
        lpdwCount: *mut ULONG_PTR,
        lpdwGranularity: LPDWORD,
    ) -> UINT;
    pub fn ResetWriteWatch(
        lpBaseAddress: LPVOID,
        dwRegionSize: SIZE_T,
    ) -> UINT;
}
ENUM!{enum MEMORY_RESOURCE_NOTIFICATION_TYPE {
    LowMemoryResourceNotification,
    HighMemoryResourceNotification,
}}
extern "system" {
    pub fn CreateMemoryResourceNotification(
        NotificationType: MEMORY_RESOURCE_NOTIFICATION_TYPE,
    ) -> HANDLE;
    pub fn QueryMemoryResourceNotification(
        ResourceNotificationHandle: HANDLE,
        ResourceState: PBOOL,
    ) -> BOOL;
}
pub const FILE_CACHE_MAX_HARD_ENABLE: DWORD = 0x00000001;
pub const FILE_CACHE_MAX_HARD_DISABLE: DWORD = 0x00000002;
pub const FILE_CACHE_MIN_HARD_ENABLE: DWORD = 0x00000004;
pub const FILE_CACHE_MIN_HARD_DISABLE: DWORD = 0x00000008;
extern "system" {
    pub fn GetSystemFileCacheSize(
        lpMinimumFileCacheSize: PSIZE_T,
        lpMaximumFileCacheSize: PSIZE_T,
        lpFlags: PDWORD,
    ) -> BOOL;
    pub fn SetSystemFileCacheSize(
        MinimumFileCacheSize: SIZE_T,
        MaximumFileCacheSize: SIZE_T,
        Flags: DWORD,
    ) -> BOOL;
    pub fn CreateFileMappingNumaW(
        hFile: HANDLE,
        lpFileMappingAttributes: LPSECURITY_ATTRIBUTES,
        flProtect: DWORD,
        dwMaximumSizeHigh: DWORD,
        dwMaximumSizeLow: DWORD,
        lpName: LPCWSTR,
        nndPreferred: DWORD,
    ) -> HANDLE;
}
STRUCT!{struct WIN32_MEMORY_RANGE_ENTRY {
    VirtualAddress: PVOID,
    NumberOfBytes: SIZE_T,
}}
pub type PWIN32_MEMORY_RANGE_ENTRY = *mut WIN32_MEMORY_RANGE_ENTRY;
extern "system" {
    pub fn PrefetchVirtualMemory(
        hProcess: HANDLE,
        NumberOfEntries: ULONG_PTR,
        VirtualAddresses: PWIN32_MEMORY_RANGE_ENTRY,
        Flags: ULONG,
    ) -> BOOL;
    pub fn CreateFileMappingFromApp(
        hFile: HANDLE,
        SecurityAttributes: PSECURITY_ATTRIBUTES,
        PageProtection: ULONG,
        MaximumSize: ULONG64,
        Name: PCWSTR,
    ) -> HANDLE;
    pub fn MapViewOfFileFromApp(
        hFileMappingObject: HANDLE,
        DesiredAccess: ULONG,
        FileOffset: ULONG64,
        NumberOfBytesToMap: SIZE_T,
    ) -> PVOID;
    pub fn UnmapViewOfFileEx(
        BaseAddress: PVOID,
        UnmapFlags: ULONG,
    ) -> BOOL;
    pub fn AllocateUserPhysicalPages(
        hProcess: HANDLE,
        NumberOfPages: PULONG_PTR,
        PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn FreeUserPhysicalPages(
        hProcess: HANDLE,
        NumberOfPages: PULONG_PTR,
        PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn MapUserPhysicalPages(
        VirtualAddress: PVOID,
        NumberOfPages: ULONG_PTR,
        PageArray: PULONG_PTR,
    ) -> BOOL;
    pub fn AllocateUserPhysicalPagesNuma(
        hProcess: HANDLE,
        NumberOfPages: PULONG_PTR,
        PageArray: PULONG_PTR,
        nndPreferred: DWORD,
    ) -> BOOL;
    pub fn VirtualAllocExNuma(
        hProcess: HANDLE,
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flAllocationType: DWORD,
        flProtect: DWORD,
        nndPreferred: DWORD,
    ) -> LPVOID;
}
pub const MEHC_PATROL_SCRUBBER_PRESENT: ULONG = 0x1;
extern "system" {
    pub fn GetMemoryErrorHandlingCapabilities(
        Capabilities: PULONG,
    ) -> BOOL;
}
FN!{stdcall PBAD_MEMORY_CALLBACK_ROUTINE() -> ()}
extern "system" {
    pub fn RegisterBadMemoryNotification(
        Callback: PBAD_MEMORY_CALLBACK_ROUTINE,
    ) -> PVOID;
    pub fn UnregisterBadMemoryNotification(
        RegistrationHandle: PVOID,
    ) -> BOOL;
}
ENUM!{enum OFFER_PRIORITY {
    VmOfferPriorityVeryLow = 1,
    VmOfferPriorityLow,
    VmOfferPriorityBelowNormal,
    VmOfferPriorityNormal,
}}
extern "system" {
    pub fn OfferVirtualMemory(
        VirtualAddress: PVOID,
        Size: SIZE_T,
        Priority: OFFER_PRIORITY,
    ) -> DWORD;
    pub fn ReclaimVirtualMemory(
        VirtualAddress: *const c_void,
        Size: SIZE_T,
    ) -> DWORD;
    pub fn DiscardVirtualMemory(
        VirtualAddress: PVOID,
        Size: SIZE_T,
    ) -> DWORD;
// TODO: Needs winnt::PCFG_CALL_TARGET_INFO.
/*  pub fn SetProcessValidCallTargets(
        hProcess: HANDLE,
        VirtualAddress: PVOID,
        RegionSize: SIZE_T,
        NumberOfOffsets: ULONG,
        OffsetInformation: PCFG_CALL_TARGET_INFO,
    ) -> BOOL; */
    pub fn VirtualAllocFromApp(
        BaseAddress: PVOID,
        Size: SIZE_T,
        AllocationType: ULONG,
        Protection: ULONG,
    ) -> PVOID;
    pub fn VirtualProtectFromApp(
        Address: PVOID,
        Size: SIZE_T,
        NewProtection: ULONG,
        OldProtection: PULONG,
    ) -> BOOL;
    pub fn OpenFileMappingFromApp(
        DesiredAccess: ULONG,
        InheritHandle: BOOL,
        Name: PCWSTR,
    ) -> HANDLE;
}
// TODO: Under WINAPI_PARTITION_APP, define CreateFileMappingW, MapViewOfFile, VirtualAlloc,
// VirtualProtect, and OpenFileMappingW as wrappers around the *FromApp functions.
ENUM!{enum WIN32_MEMORY_INFORMATION_CLASS {
    MemoryRegionInfo,
}}
STRUCT!{struct WIN32_MEMORY_REGION_INFORMATION {
    AllocationBase: PVOID,
    AllocationProtect: ULONG,
    u: WIN32_MEMORY_REGION_INFORMATION_u,
    RegionSize: SIZE_T,
    CommitSize: SIZE_T,
}}
UNION!{union WIN32_MEMORY_REGION_INFORMATION_u {
    [u32; 1],
    Flags Flags_mut: ULONG,
    s s_mut: WIN32_MEMORY_REGION_INFORMATION_u_s,
}}
STRUCT!{struct WIN32_MEMORY_REGION_INFORMATION_u_s {
    Bitfield: ULONG,
}}
BITFIELD!{WIN32_MEMORY_REGION_INFORMATION_u_s Bitfield: ULONG [
    Private set_Private[0..1],
    MappedDataFile set_MappedDataFile[1..2],
    MappedImage set_MappedImage[2..3],
    MappedPageFile set_MappedPageFile[3..4],
    MappedPhysical set_MappedPhysical[4..5],
    DirectMapped set_DirectMapped[5..6],
    Reserved set_Reserved[6..32],
]}
// TODO: Need to resolve issue #323 first.
/*extern "system" {
    pub fn QueryVirtualMemoryInformation(
        Process: HANDLE,
        VirtualAddress: *const VOID,
        MemoryInformationClass: WIN32_MEMORY_INFORMATION_CLASS,
        MemoryInformation: PVOID,
        MemoryInformationSize: SIZE_T,
        ReturnSize: PSIZE_T,
    ) -> BOOL;
    pub fn MapViewOfFileNuma2(
        FileMappingHandle: HANDLE,
        ProcessHandle: HANDLE,
        Offset: ULONG64,
        BaseAddress: PVOID,
        ViewSize: SIZE_T,
        AllocationType: ULONG,
        PageProtection: ULONG,
        PreferredNode: ULONG,
    ) -> PVOID;
}
#[inline]
pub unsafe fn MapViewOfFile2(
    FileMappingHandle: HANDLE,
    ProcessHandle: HANDLE,
    Offset: ULONG64,
    BaseAddress: PVOID,
    ViewSize: SIZE_T,
    AllocationType: ULONG,
    PageProtection: ULONG,
) -> PVOID {
    MapViewOfFileNuma2(FileMappingHandle,
        ProcessHandle,
        Offset,
        BaseAddress,
        ViewSize,
        AllocationType,
        PageProtection,
        NUMA_NO_PREFERRED_NODE)
}*/
extern "system" {
    pub fn UnmapViewOfFile2(
        ProcessHandle: HANDLE,
        BaseAddress: PVOID,
        UnmapFlags: ULONG,
    ) -> BOOL;
}
