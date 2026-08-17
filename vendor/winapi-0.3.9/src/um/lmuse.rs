// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This file contains structures, function prototypes, and definitions for the NetUse API
use shared::lmcons::{LMSTR, NET_API_STATUS};
use shared::minwindef::{DWORD, LPBYTE, LPDWORD, PBYTE, ULONG};
use um::winnt::LPWSTR;
extern "system" {
    pub fn NetUseAdd(
        servername: LPWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseDel(
        UncServerName: LMSTR,
        UseName: LMSTR,
        ForceCond: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseEnum(
        UncServerName: LMSTR,
        Level: DWORD,
        BufPtr: *mut LPBYTE,
        PreferedMaximumSize: DWORD,
        EntriesRead: LPDWORD,
        TotalEntries: LPDWORD,
        ResumeHandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUseGetInfo(
        UncServerName: LMSTR,
        UseName: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
STRUCT!{struct USE_INFO_0 {
    ui0_local: LMSTR,
    ui0_remote: LMSTR,
}}
pub type PUSE_INFO_0 = *mut USE_INFO_0;
pub type LPUSE_INFO_0 = *mut USE_INFO_0;
STRUCT!{struct USE_INFO_1 {
    ui1_local: LMSTR,
    ui1_remote: LMSTR,
    ui1_password: LMSTR,
    ui1_status: DWORD,
    ui1_asg_type: DWORD,
    ui1_refcount: DWORD,
    ui1_usecount: DWORD,
}}
pub type PUSE_INFO_1 = *mut USE_INFO_1;
pub type LPUSE_INFO_1 = *mut USE_INFO_1;
STRUCT!{struct USE_INFO_2 {
    ui2_local: LMSTR,
    ui2_remote: LMSTR,
    ui2_password: LMSTR,
    ui2_status: DWORD,
    ui2_asg_type: DWORD,
    ui2_refcount: DWORD,
    ui2_usecount: DWORD,
    ui2_username: LMSTR,
    ui2_domainname: LMSTR,
}}
pub type PUSE_INFO_2 = *mut USE_INFO_2;
pub type LPUSE_INFO_2 = *mut USE_INFO_2;
STRUCT!{struct USE_INFO_3 {
    ui3_ui2: USE_INFO_2,
    ui3_flags: ULONG,
}}
pub type PUSE_INFO_3 = *mut USE_INFO_3;
STRUCT!{struct USE_INFO_4 {
    ui4_ui3: USE_INFO_3,
    ui4_auth_identity_length: DWORD,
    ui4_auth_identity: PBYTE,
}}
pub type PUSE_INFO_4 = *mut USE_INFO_4;
pub type LPUSE_INFO_4 = *mut USE_INFO_4;
pub const USE_LOCAL_PARMNUM: DWORD = 1;
pub const USE_REMOTE_PARMNUM: DWORD = 2;
pub const USE_PASSWORD_PARMNUM: DWORD = 3;
pub const USE_ASGTYPE_PARMNUM: DWORD = 4;
pub const USE_USERNAME_PARMNUM: DWORD = 5;
pub const USE_DOMAINNAME_PARMNUM: DWORD = 6;
pub const USE_OK: DWORD = 0;
pub const USE_PAUSED: DWORD = 1;
pub const USE_SESSLOST: DWORD = 2;
pub const USE_DISCONN: DWORD = 2;
pub const USE_NETERR: DWORD = 3;
pub const USE_CONN: DWORD = 4;
pub const USE_RECONN: DWORD = 5;
pub const USE_WILDCARD: DWORD = -1i32 as u32;
pub const USE_DISKDEV: DWORD = 0;
pub const USE_SPOOLDEV: DWORD = 1;
pub const USE_CHARDEV: DWORD = 2;
pub const USE_IPC: DWORD = 3;
pub const CREATE_NO_CONNECT: ULONG = 0x1;
pub const CREATE_BYPASS_CSC: ULONG = 0x2;
pub const CREATE_CRED_RESET: ULONG = 0x4;
pub const USE_DEFAULT_CREDENTIALS: ULONG = 0x4;
