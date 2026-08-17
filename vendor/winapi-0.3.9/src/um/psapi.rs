// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! FFI bindings to psapi.
use shared::basetsd::{SIZE_T, ULONG_PTR};
use shared::minwindef::{BOOL, DWORD, HMODULE, LPDWORD, LPVOID, PDWORD};
use um::winnt::{HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PVOID};
pub const LIST_MODULES_DEFAULT: DWORD = 0x0;
pub const LIST_MODULES_32BIT: DWORD = 0x01;
pub const LIST_MODULES_64BIT: DWORD = 0x02;
pub const LIST_MODULES_ALL: DWORD = LIST_MODULES_32BIT | LIST_MODULES_64BIT;
extern "system" {
    pub fn K32EnumProcesses(
        lpidProcess: *mut DWORD,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32EnumProcessModules(
        hProcess: HANDLE,
        lphModule: *mut HMODULE,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32EnumProcessModulesEx(
        hProcess: HANDLE,
        lphModule: *mut HMODULE,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
        dwFilterFlag: DWORD,
    ) -> BOOL;
    pub fn K32GetModuleBaseNameA(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpBaseName: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleBaseNameW(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpBaseName: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleFileNameExA(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetModuleFileNameExW(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32EmptyWorkingSet(
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn K32QueryWorkingSet(
        hProcess: HANDLE,
        pv: PVOID,
        cb: DWORD,
    ) -> BOOL;
    pub fn K32QueryWorkingSetEx(
        hProcess: HANDLE,
        pv: PVOID,
        cb: DWORD,
    ) -> BOOL;
    pub fn K32InitializeProcessForWsWatch(
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn K32GetWsChanges(
        hProcess: HANDLE,
        lpWatchInfo: PPSAPI_WS_WATCH_INFORMATION,
        cb: DWORD,
    ) -> BOOL;
    pub fn K32GetWsChangesEx(
        hProcess: HANDLE,
        lpWatchInfoEx: PPSAPI_WS_WATCH_INFORMATION_EX,
        cb: PDWORD,
    ) -> BOOL;
    pub fn K32GetMappedFileNameW(
        hProcess: HANDLE,
        lpv: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetMappedFileNameA(
        hProcess: HANDLE,
        lpv: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32EnumDeviceDrivers(
        lpImageBase: *mut LPVOID,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32GetDeviceDriverBaseNameA(
        ImageBase: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetDeviceDriverBaseNameW(
        ImageBase: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetDeviceDriverFileNameA(
        ImageBase: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetDeviceDriverFileNameW(
        ImageBase: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetPerformanceInfo(
        pPerformanceInformation: PPERFORMANCE_INFORMATION,
        cb: DWORD,
    ) -> BOOL;
    pub fn K32EnumPageFilesW(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKW,
        pContext: LPVOID,
    ) -> BOOL;
    pub fn K32EnumPageFilesA(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKA,
        pContext: LPVOID,
    ) -> BOOL;
    pub fn K32GetProcessImageFileNameA(
        hProcess: HANDLE,
        lpImageFileName: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn K32GetProcessImageFileNameW(
        hProcess: HANDLE,
        lpImageFileName: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn EnumProcesses(
        lpidProcess: *mut DWORD,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn K32GetProcessMemoryInfo(
        Process: HANDLE,
        ppsmemCounters: PPROCESS_MEMORY_COUNTERS,
        cb: DWORD,
    ) -> BOOL;
    pub fn K32GetModuleInformation(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpmodinfo: LPMODULEINFO,
        cb: DWORD,
    ) -> BOOL;
}
pub type LPMODULEINFO = *mut MODULEINFO;
pub type PPSAPI_WORKING_SET_INFORMATION = *mut PSAPI_WORKING_SET_INFORMATION;
pub type PPSAPI_WORKING_SET_EX_INFORMATION = *mut PSAPI_WORKING_SET_EX_INFORMATION;
pub type PPSAPI_WS_WATCH_INFORMATION = *mut PSAPI_WS_WATCH_INFORMATION;
pub type PPSAPI_WS_WATCH_INFORMATION_EX = *mut PSAPI_WS_WATCH_INFORMATION_EX;
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;
pub type PPERFORMANCE_INFORMATION = *mut PERFORMANCE_INFORMATION;
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;
FN!{stdcall PENUM_PAGE_FILE_CALLBACKA(
    pContext: LPVOID,
    pPageFileInfo: PENUM_PAGE_FILE_INFORMATION,
    lpFilename: LPCSTR,
) -> BOOL}
FN!{stdcall PENUM_PAGE_FILE_CALLBACKW(
    pContext: LPVOID,
    pPageFileInfo: PENUM_PAGE_FILE_INFORMATION,
    lpFilename: LPCWSTR,
) -> BOOL}
STRUCT!{struct MODULEINFO {
    lpBaseOfDll: LPVOID,
    SizeOfImage: DWORD,
    EntryPoint: LPVOID,
}}
STRUCT!{struct ENUM_PAGE_FILE_INFORMATION {
    cb: DWORD,
    Reserved: DWORD,
    TotalSize: SIZE_T,
    TotalInUse: SIZE_T,
    PeakUsage: SIZE_T,
}}
STRUCT!{struct PERFORMANCE_INFORMATION {
    cb: DWORD,
    CommitTotal: SIZE_T,
    CommitLimit: SIZE_T,
    CommitPeak: SIZE_T,
    PhysicalTotal: SIZE_T,
    PhysicalAvailable: SIZE_T,
    SystemCache: SIZE_T,
    KernelTotal: SIZE_T,
    KernelPaged: SIZE_T,
    KernelNonpaged: SIZE_T,
    PageSize: SIZE_T,
    HandleCount: DWORD,
    ProcessCount: DWORD,
    ThreadCount: DWORD,
}}
STRUCT!{struct PROCESS_MEMORY_COUNTERS {
    cb: DWORD,
    PageFaultCount: DWORD,
    PeakWorkingSetSize: SIZE_T,
    WorkingSetSize: SIZE_T,
    QuotaPeakPagedPoolUsage: SIZE_T,
    QuotaPagedPoolUsage: SIZE_T,
    QuotaPeakNonPagedPoolUsage: SIZE_T,
    QuotaNonPagedPoolUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PeakPagefileUsage: SIZE_T,
}}
STRUCT!{struct PROCESS_MEMORY_COUNTERS_EX {
    cb: DWORD,
    PageFaultCount: DWORD,
    PeakWorkingSetSize: SIZE_T,
    WorkingSetSize: SIZE_T,
    QuotaPeakPagedPoolUsage: SIZE_T,
    QuotaPagedPoolUsage: SIZE_T,
    QuotaPeakNonPagedPoolUsage: SIZE_T,
    QuotaNonPagedPoolUsage: SIZE_T,
    PagefileUsage: SIZE_T,
    PeakPagefileUsage: SIZE_T,
    PrivateUsage: SIZE_T,
}}
STRUCT!{struct PSAPI_WORKING_SET_BLOCK {
    Flags: ULONG_PTR,
}}
BITFIELD!{PSAPI_WORKING_SET_BLOCK Flags: ULONG_PTR [
    Protection set_Protection[0..5],
    ShareCount set_ShareCount[5..8],
    Shared set_Shared[8..9],
    Reserved set_Reserved[9..12],
    VirtualPage set_VirtualPage[12..32],
]}
pub type PPSAPI_WORKING_SET_BLOCK = *mut PSAPI_WORKING_SET_BLOCK;
STRUCT!{struct PSAPI_WORKING_SET_EX_BLOCK {
    Flags: ULONG_PTR,
}}
#[cfg(not(target_arch="x86_64"))]
BITFIELD!{PSAPI_WORKING_SET_EX_BLOCK Flags: ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Reserved set_Reserved[24..31],
    Bad set_Bad[31..32],
]}
#[cfg(target_arch="x86_64")]
BITFIELD!{PSAPI_WORKING_SET_EX_BLOCK Flags: ULONG_PTR [
    Valid set_Valid[0..1],
    ShareCount set_ShareCount[1..4],
    Win32Protection set_Win32Protection[4..15],
    Shared set_Shared[15..16],
    Node set_Node[16..22],
    Locked set_Locked[22..23],
    LargePage set_LargePage[23..24],
    Reserved set_Reserved[24..31],
    Bad set_Bad[31..32],
    ReservedUlong set_ReservedULong[32..64],
]}
pub type PPSAPI_WORKING_SET_EX_BLOCK = *mut PSAPI_WORKING_SET_EX_BLOCK;
STRUCT!{struct PSAPI_WORKING_SET_INFORMATION {
    NumberOfEntries: ULONG_PTR,
    WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}}
STRUCT!{struct PSAPI_WORKING_SET_EX_INFORMATION {
    VirtualAddress: PVOID,
    VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}}
STRUCT!{struct PSAPI_WS_WATCH_INFORMATION {
    FaultingPc: LPVOID,
    FaultingVa: LPVOID,
}}
STRUCT!{struct PSAPI_WS_WATCH_INFORMATION_EX {
    BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    FaultingThreadId: ULONG_PTR,
    Flags: ULONG_PTR,
}}
extern "system" {
    pub fn EmptyWorkingSet(
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn EnumDeviceDrivers(
        lpImageBase: *mut LPVOID,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn EnumPageFilesA(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKA,
        pContext: LPVOID,
    ) -> BOOL;
    pub fn EnumPageFilesW(
        pCallBackRoutine: PENUM_PAGE_FILE_CALLBACKW,
        pContext: LPVOID,
    ) -> BOOL;
    pub fn EnumProcessModules(
        hProcess: HANDLE,
        lphModule: *mut HMODULE,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
    ) -> BOOL;
    pub fn EnumProcessModulesEx(
        hProcess: HANDLE,
        lphModule: *mut HMODULE,
        cb: DWORD,
        lpcbNeeded: LPDWORD,
        dwFilterFlag: DWORD,
    ) -> BOOL;
    pub fn GetDeviceDriverBaseNameA(
        ImageBase: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetDeviceDriverBaseNameW(
        ImageBase: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetDeviceDriverFileNameA(
        ImageBase: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetDeviceDriverFileNameW(
        ImageBase: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetMappedFileNameA(
        hProcess: HANDLE,
        lpv: LPVOID,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetMappedFileNameW(
        hProcess: HANDLE,
        lpv: LPVOID,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleBaseNameA(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpBaseName: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleBaseNameW(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpBaseName: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleFileNameExA(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleFileNameExW(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleInformation(
        hProcess: HANDLE,
        hModule: HMODULE,
        lpmodinfo: LPMODULEINFO,
        cb: DWORD,
    ) -> BOOL;
    pub fn GetPerformanceInfo(
        pPerformanceInformation: PPERFORMANCE_INFORMATION,
        cb: DWORD,
    ) -> BOOL;
    pub fn GetProcessImageFileNameA(
        hProcess: HANDLE,
        lpImageFileName: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetProcessImageFileNameW(
        hProcess: HANDLE,
        lpImageFileName: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetProcessMemoryInfo(
        hProcess: HANDLE,
        ppsmemCounters: PPROCESS_MEMORY_COUNTERS,
        cb: DWORD,
    ) -> BOOL;
    pub fn GetWsChanges(
        hProcess: HANDLE,
        lpWatchInfo: PPSAPI_WS_WATCH_INFORMATION,
        cb: DWORD,
    ) -> BOOL;
    pub fn GetWsChangesEx(
        hProcess: HANDLE,
        lpWatchInfoEx: PPSAPI_WS_WATCH_INFORMATION_EX,
        cb: PDWORD,
    ) -> BOOL;
    pub fn InitializeProcessForWsWatch(
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn QueryWorkingSet(
        hProcess: HANDLE,
        pv: PVOID,
        cb: DWORD,
    ) -> BOOL;
    pub fn QueryWorkingSetEx(
        hProcess: HANDLE,
        pv: PVOID,
        cb: DWORD,
    ) -> BOOL;
}
