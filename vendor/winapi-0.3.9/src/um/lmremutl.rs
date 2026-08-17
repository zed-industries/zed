// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains structures, function prototypes, and definitions for the NetRemote API
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{DWORD, LPBYTE, LPDWORD};
use um::winnt::{CHAR, LONG, LPCWSTR, LPSTR};
pub type DESC_CHAR = CHAR;
pub type LPDESC = LPSTR;
extern "system" {
    pub fn NetRemoteTOD(
        UncServerName: LPCWSTR,
        BufferPtr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetRemoteComputerSupports(
        UncServerName: LPCWSTR,
        OptionsWanted: DWORD,
        OptionsSupported: LPDWORD,
    ) -> NET_API_STATUS;
}
extern "C" {
    pub fn RxRemoteApi(
        ApiNumber: DWORD,
        UncServerName: LPCWSTR,
        ParmDescString: LPDESC,
        DataDesc16: LPDESC,
        DataDesc32: LPDESC,
        DataDescSmb: LPDESC,
        AuxDesc16: LPDESC,
        AuxDesc32: LPDESC,
        AuxDescSmb: LPDESC,
        Flags: DWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct TIME_OF_DAY_INFO {
    tod_elapsedt: DWORD,
    tod_msecs: DWORD,
    tod_hours: DWORD,
    tod_mins: DWORD,
    tod_secs: DWORD,
    tod_hunds: DWORD,
    tod_timezone: LONG,
    tod_tinterval: DWORD,
    tod_day: DWORD,
    tod_month: DWORD,
    tod_year: DWORD,
    tod_weekday: DWORD,
}}
pub type PTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub type LPTIME_OF_DAY_INFO = *mut TIME_OF_DAY_INFO;
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: DWORD = 0x00000002;
pub const SUPPORTS_RPC: DWORD = 0x00000004;
pub const SUPPORTS_SAM_PROTOCOL: DWORD = 0x00000008;
pub const SUPPORTS_UNICODE: DWORD = 0x00000010;
pub const SUPPORTS_LOCAL: DWORD = 0x00000020;
pub const SUPPORTS_ANY: DWORD = 0xFFFFFFFF;
pub const NO_PERMISSION_REQUIRED: DWORD = 0x00000001;
pub const ALLOCATE_RESPONSE: DWORD = 0x00000002;
pub const USE_SPECIFIC_TRANSPORT: DWORD = 0x80000000;
