// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::DWORD_PTR;
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{DWORD, LPBYTE, LPDWORD, UCHAR};
use um::winnt::{LPCWSTR, LPWSTR};
pub const JOB_RUN_PERIODICALLY: UCHAR = 0x01;
pub const JOB_EXEC_ERROR: UCHAR = 0x02;
pub const JOB_RUNS_TODAY: UCHAR = 0x04;
pub const JOB_ADD_CURRENT_DATE: UCHAR = 0x08;
pub const JOB_NONINTERACTIVE: UCHAR = 0x10;
pub const JOB_INPUT_FLAGS: UCHAR = JOB_RUN_PERIODICALLY | JOB_ADD_CURRENT_DATE
    | JOB_NONINTERACTIVE;
pub const JOB_OUTPUT_FLAGS: UCHAR = JOB_RUN_PERIODICALLY | JOB_EXEC_ERROR | JOB_RUNS_TODAY
    | JOB_NONINTERACTIVE;
STRUCT!{struct AT_INFO {
    JobTime: DWORD_PTR,
    DaysOfMonth: DWORD,
    DaysOfWeek: UCHAR,
    Flags: UCHAR,
    Command: LPWSTR,
}}
pub type PAT_INFO = *mut AT_INFO;
pub type LPAT_INFO = *mut AT_INFO;
STRUCT!{struct AT_ENUM {
    JobId: DWORD,
    JobTime: DWORD_PTR,
    DaysOfMonth: DWORD,
    DaysOfWeek: UCHAR,
    Flags: UCHAR,
    Command: LPWSTR,
}}
pub type PAT_ENUM = *mut AT_ENUM;
pub type LPAT_ENUM = *mut AT_ENUM;
extern "system" {
    pub fn NetScheduleJobAdd(
        Servername: LPCWSTR,
        Buffer: LPBYTE,
        JobId: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobDel(
        Servername: LPCWSTR,
        MinJobId: DWORD,
        MaxJobId: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobEnum(
        Servername: LPCWSTR,
        PointerToBuffer: *mut LPBYTE,
        PointerToBuffer: DWORD,
        EntriesRead: LPDWORD,
        TotalEntries: LPDWORD,
        ResumeHandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetScheduleJobGetInfo(
        Servername: LPCWSTR,
        JobId: DWORD,
        PointerToBuffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
