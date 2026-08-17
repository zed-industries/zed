// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_longlong;
use shared::guiddef::GUID;
use shared::minwindef::{BOOL, LPDWORD, PULONG};
use um::winnt::{HANDLE, LPCSTR, LPCWSTR, PHANDLE, PLARGE_INTEGER};
ENUM!{enum AVRT_PRIORITY {
    AVRT_PRIORITY_VERYLOW = -2i32 as u32,
    AVRT_PRIORITY_LOW,
    AVRT_PRIORITY_NORMAL = 0,
    AVRT_PRIORITY_HIGH,
    AVRT_PRIORITY_CRITICAL,
}}
pub const THREAD_ORDER_GROUP_INFINITE_TIMEOUT: c_longlong = -1;
extern "system" {
    pub fn AvSetMmThreadCharacteristicsA(
        TaskName: LPCSTR,
        TaskIndex: LPDWORD,
    ) -> HANDLE;
    pub fn AvSetMmThreadCharacteristicsW(
        TaskName: LPCWSTR,
        TaskIndex: LPDWORD,
    ) -> HANDLE;
    pub fn AvSetMmMaxThreadCharacteristicsA(
        FirstTask: LPCSTR,
        SecondTask: LPCSTR,
        TaskIndex: LPDWORD,
    ) -> HANDLE;
    pub fn AvSetMmMaxThreadCharacteristicsW(
        FirstTask: LPCWSTR,
        SecondTask: LPCWSTR,
        TaskIndex: LPDWORD,
    ) -> HANDLE;
    pub fn AvRevertMmThreadCharacteristics(
        avrt_handle: HANDLE,
    ) -> BOOL;
    pub fn AvSetMmThreadPriority(
        AvrtHandle: HANDLE,
        Priority: AVRT_PRIORITY,
    ) -> BOOL;
    pub fn AvRtCreateThreadOrderingGroup(
        Context: PHANDLE,
        Period: PLARGE_INTEGER,
        ThreadOrderingGuid: *mut GUID,
        Timeout: PLARGE_INTEGER,
    ) -> BOOL;
    pub fn AvRtCreateThreadOrderingGroupExA(
        Context: PHANDLE,
        Period: PLARGE_INTEGER,
        ThreadOrderingGuid: *mut GUID,
        Timeout: PLARGE_INTEGER,
        TaskName: LPCSTR,
    )-> BOOL;
    pub fn AvRtCreateThreadOrderingGroupExW(
        Context: PHANDLE,
        Period: PLARGE_INTEGER,
        ThreadOrderingGuid: *mut GUID,
        Timeout: PLARGE_INTEGER,
        TaskName: LPCWSTR,
    ) -> BOOL;
    pub fn AvRtJoinThreadOrderingGroup(
        Context: PHANDLE,
        ThreadOrderingGuid: *mut GUID,
        Before: BOOL,
    ) -> BOOL;
    pub fn AvRtWaitOnThreadOrderingGroup(
        Context: HANDLE,
    ) -> BOOL;
    pub fn AvRtLeaveThreadOrderingGroup(
        Context: HANDLE,
    ) -> BOOL;
    pub fn AvRtDeleteThreadOrderingGroup(
        Context: HANDLE,
    ) -> BOOL;
    pub fn AvQuerySystemResponsiveness(
        AvrtHandle: HANDLE,
        SystemResponsivenessValue: PULONG,
    ) -> BOOL;
}
