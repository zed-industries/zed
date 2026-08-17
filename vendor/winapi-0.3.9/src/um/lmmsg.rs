// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains structures, function prototypes, and definitions for the NetMessage API
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{DWORD, LPBYTE, LPDWORD};
use um::winnt::{LPCWSTR, LPWSTR};
extern "system" {
    pub fn NetMessageNameAdd(
        servername: LPCWSTR,
        msgname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetMessageNameEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetMessageNameGetInfo(
        servername: LPCWSTR,
        msgname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetMessageNameDel(
        servername: LPCWSTR,
        msgname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetMessageBufferSend(
        servername: LPCWSTR,
        msgname: LPCWSTR,
        fromname: LPCWSTR,
        buf: LPBYTE,
        buflen: DWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct MSG_INFO_0 {
    msgi0_name: LPWSTR,
}}
pub type PMSG_INFO_0 = *mut MSG_INFO_0;
pub type LPMSG_INFO_0 = *mut MSG_INFO_0;
STRUCT!{struct MSG_INFO_1 {
    msgi1_name: LPWSTR,
    msgi1_forward_flag: DWORD,
    msgi1_forward: LPWSTR,
}}
pub type PMSG_INFO_1 = *mut MSG_INFO_1;
pub type LPMSG_INFO_1 = *mut MSG_INFO_1;
pub const MSGNAME_NOT_FORWARDED: DWORD = 0;
pub const MSGNAME_FORWARDED_TO: DWORD = 0x04;
pub const MSGNAME_FORWARDED_FROM: DWORD = 0x10;
