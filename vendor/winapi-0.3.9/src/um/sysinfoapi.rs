// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! ApiSet Contract for api-ms-win-core-sysinfo-l1.
use shared::basetsd::DWORD_PTR;
use shared::minwindef::{
    BOOL, BYTE, DWORD, LPDWORD, LPFILETIME, LPVOID, PBOOL, PDWORD, UINT, USHORT, WORD,
};
use um::minwinbase::{LPSYSTEMTIME, SYSTEMTIME};
use um::winnt::{
    DWORDLONG, HANDLE, LOGICAL_PROCESSOR_RELATIONSHIP, LPCSTR, LPCWSTR, LPOSVERSIONINFOA,
    LPOSVERSIONINFOW, LPSTR, LPWSTR, PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
    PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, PULONGLONG, PVOID, ULONGLONG,
};
STRUCT!{struct SYSTEM_INFO_u_s {
    wProcessorArchitecture: WORD,
    wReserved: WORD,
}}
UNION!{union SYSTEM_INFO_u {
    [u32; 1],
    dwOemId dwOemId_mut: DWORD,
    s s_mut: SYSTEM_INFO_u_s,
}}
STRUCT!{struct SYSTEM_INFO {
    u: SYSTEM_INFO_u,
    dwPageSize: DWORD,
    lpMinimumApplicationAddress: LPVOID,
    lpMaximumApplicationAddress: LPVOID,
    dwActiveProcessorMask: DWORD_PTR,
    dwNumberOfProcessors: DWORD,
    dwProcessorType: DWORD,
    dwAllocationGranularity: DWORD,
    wProcessorLevel: WORD,
    wProcessorRevision: WORD,
}}
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
STRUCT!{struct MEMORYSTATUSEX {
    dwLength: DWORD,
    dwMemoryLoad: DWORD,
    ullTotalPhys: DWORDLONG,
    ullAvailPhys: DWORDLONG,
    ullTotalPageFile: DWORDLONG,
    ullAvailPageFile: DWORDLONG,
    ullTotalVirtual: DWORDLONG,
    ullAvailVirtual: DWORDLONG,
    ullAvailExtendedVirtual: DWORDLONG,
}}
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
extern "system" {
    pub fn GlobalMemoryStatusEx(
        lpBuffer: LPMEMORYSTATUSEX,
    ) -> BOOL;
    pub fn GetSystemInfo(
        lpSystemInfo: LPSYSTEM_INFO,
    );
    pub fn GetSystemTime(
        lpSystemTime: LPSYSTEMTIME,
    );
    pub fn GetSystemTimeAsFileTime(
        lpSystemTimeAsFileTime: LPFILETIME,
    );
    pub fn GetLocalTime(
        lpSystemTime: LPSYSTEMTIME,
    );
    pub fn GetVersion() -> DWORD;
    pub fn SetLocalTime(
        lpSystemTime: *const SYSTEMTIME,
    ) -> BOOL;
    pub fn GetTickCount() -> DWORD;
    pub fn GetTickCount64() -> ULONGLONG;
    pub fn GetSystemTimeAdjustment(
        lpTimeAdjustment: PDWORD,
        lpTimeIncrement: PDWORD,
        lpTimeAdjustmentDisabled: PBOOL,
    ) -> BOOL;
    pub fn GetSystemDirectoryA(
        lpBuffer: LPSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetSystemDirectoryW(
        lpBuffer: LPWSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetWindowsDirectoryA(
        lpBuffer: LPSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetWindowsDirectoryW(
        lpBuffer: LPWSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetSystemWindowsDirectoryA(
        lpBuffer: LPSTR,
        uSize: UINT,
    ) -> UINT;
    pub fn GetSystemWindowsDirectoryW(
        lpBuffer: LPWSTR,
        uSize: UINT,
    ) -> UINT;
}
ENUM!{enum COMPUTER_NAME_FORMAT {
    ComputerNameNetBIOS,
    ComputerNameDnsHostname,
    ComputerNameDnsDomain,
    ComputerNameDnsFullyQualified,
    ComputerNamePhysicalNetBIOS,
    ComputerNamePhysicalDnsHostname,
    ComputerNamePhysicalDnsDomain,
    ComputerNamePhysicalDnsFullyQualified,
    ComputerNameMax,
}}
extern "system" {
    pub fn GetComputerNameExA(
        NameType: COMPUTER_NAME_FORMAT,
        lpBuffer: LPSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetComputerNameExW(
        NameType: COMPUTER_NAME_FORMAT,
        lpBuffer: LPWSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn SetComputerNameExW(
        NameType: COMPUTER_NAME_FORMAT,
        lpBuffer: LPCWSTR,
    ) -> BOOL;
    pub fn SetSystemTime(
        lpSystemTime: *const SYSTEMTIME,
    ) -> BOOL;
    pub fn GetVersionExA(
        lpVersionInformation: LPOSVERSIONINFOA,
    ) -> BOOL;
    pub fn GetVersionExW(
        lpVersionInformation: LPOSVERSIONINFOW,
    ) -> BOOL;
    pub fn GetLogicalProcessorInformation(
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
        ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetLogicalProcessorInformationEx(
        RelationshipType: LOGICAL_PROCESSOR_RELATIONSHIP,
        Buffer: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION,
        ReturnedLength: PDWORD,
    ) -> BOOL;
    pub fn GetNativeSystemInfo(
        lpSystemInfo: LPSYSTEM_INFO,
    );
    pub fn GetSystemTimePreciseAsFileTime(
        lpSystemTimeAsFileTime: LPFILETIME,
    );
    pub fn GetProductInfo(
        dwOSMajorVersion: DWORD,
        dwOSMinorVersion: DWORD,
        dwSpMajorVersion: DWORD,
        dwSpMinorVersion: DWORD,
        pdwReturnedProductType: PDWORD,
    ) -> BOOL;
    pub fn VerSetConditionMask(
        ConditionMask: ULONGLONG,
        TypeMask: DWORD,
        Condition: BYTE,
    ) -> ULONGLONG;
    // pub fn GetOsSafeBootMode();
    pub fn EnumSystemFirmwareTables(
        FirmwareTableProviderSignature: DWORD,
        pFirmwareTableEnumBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn GetSystemFirmwareTable(
        FirmwareTableProviderSignature: DWORD,
        FirmwareTableID: DWORD,
        pFirmwareTableBuffer: PVOID,
        BufferSize: DWORD,
    ) -> UINT;
    pub fn DnsHostnameToComputerNameExW(
        Hostname: LPCWSTR,
        ComputerName: LPWSTR,
        nSize: LPDWORD,
    ) -> BOOL;
    pub fn GetPhysicallyInstalledSystemMemory(
        TotalMemoryInKilobytes: PULONGLONG,
    ) -> BOOL;
}
pub const SCEX2_ALT_NETBIOS_NAME: DWORD = 0x00000001;
extern "system" {
    pub fn SetComputerNameEx2W(
        NameType: COMPUTER_NAME_FORMAT,
        Flags: DWORD,
        lpBuffer: LPCWSTR,
    ) -> BOOL;
    pub fn SetSystemTimeAdjustment(
        dwTimeAdjustment: DWORD,
        bTimeAdjustmentDisabled: BOOL,
    ) -> BOOL;
    pub fn InstallELAMCertificateInfo(
        ELAMFile: HANDLE,
    ) -> BOOL;
    pub fn GetProcessorSystemCycleTime(
        Group: USHORT,
        Buffer: PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION,
        ReturnedLength: PDWORD,
    ) -> BOOL;
    // pub fn GetOsManufacturingMode();
    // pub fn GetIntegratedDisplaySize();
    pub fn SetComputerNameA(
        lpComputerName: LPCSTR,
    ) -> BOOL;
    pub fn SetComputerNameW(
        lpComputerName: LPCWSTR,
    ) -> BOOL;
    pub fn SetComputerNameExA(
        NameType: COMPUTER_NAME_FORMAT,
        lpBuffer: LPCSTR,
    ) -> BOOL;
}
