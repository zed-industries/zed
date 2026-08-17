// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::LONG64;
use shared::minwindef::{BOOL, DWORD, LPDWORD, LPVOID, UINT, ULONG};
use shared::ntdef::{HANDLE, LPCWSTR, PCWSTR, VOID};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::winnt::JOBOBJECTINFOCLASS;
STRUCT!{struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    MaxIops: LONG64,
    MaxBandwidth: LONG64,
    ReservationIops: LONG64,
    VolumeName: PCWSTR,
    BaseIoSize: ULONG,
    ControlFlags: ULONG,
}}
extern "system" {
    pub fn CreateJobObjectW(
        lpJobAttributes: LPSECURITY_ATTRIBUTES,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn FreeMemoryJobObject(
        Buffer: *mut VOID,
    ) -> ();
    pub fn OpenJobObjectW(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        lpName: LPCWSTR,
    ) -> HANDLE;
    pub fn AssignProcessToJobObject(
        hJob: HANDLE,
        hProcess: HANDLE,
    ) -> BOOL;
    pub fn TerminateJobObject(
        hJob: HANDLE,
        uExitCode: UINT,
    ) -> BOOL;
    pub fn SetInformationJobObject(
        hJob: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID,
        cbJovObjectInformationLength: DWORD,
    ) -> BOOL;
    pub fn SetIoRateControlInformationJobObject(
        hJob: HANDLE,
        IoRateControlInfo: *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
    ) -> DWORD;
    pub fn QueryInformationJobObject(
        hJob: HANDLE,
        JobObjectInformationClass: JOBOBJECTINFOCLASS,
        lpJobObjectInformation: LPVOID,
        cbJovObjectInformationLength: DWORD,
        lpReturnLength: LPDWORD,
    ) -> BOOL;
    pub fn QueryIoRateControlInformationJobObject(
        hJob: HANDLE,
        VolumeName: PCWSTR,
        InfoBlocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION,
        InfoBlockCount: *mut ULONG,
    ) -> DWORD;
}
