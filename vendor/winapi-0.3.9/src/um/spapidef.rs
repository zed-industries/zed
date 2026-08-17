// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Public header file for Windows NT Setup and Device Installer services Dlls
use shared::minwindef::DWORD;
use um::winnt::DWORDLONG;
pub type SP_LOG_TOKEN = DWORDLONG;
pub type PSP_LOG_TOKEN = *mut DWORDLONG;
pub const LOGTOKEN_TYPE_MASK: SP_LOG_TOKEN = 3;
pub const LOGTOKEN_UNSPECIFIED: SP_LOG_TOKEN = 0;
pub const LOGTOKEN_NO_LOG: SP_LOG_TOKEN = 1;
pub const LOGTOKEN_SETUPAPI_APPLOG: SP_LOG_TOKEN = 2;
pub const LOGTOKEN_SETUPAPI_DEVLOG: SP_LOG_TOKEN = 3;
pub const TXTLOG_SETUPAPI_DEVLOG: DWORD = 0x00000001;
pub const TXTLOG_SETUPAPI_CMDLINE: DWORD = 0x00000002;
pub const TXTLOG_SETUPAPI_BITS: DWORD = 0x00000003;
pub const TXTLOG_ERROR: DWORD = 0x1;
pub const TXTLOG_WARNING: DWORD = 0x2;
pub const TXTLOG_SYSTEM_STATE_CHANGE: DWORD = 0x3;
pub const TXTLOG_SUMMARY: DWORD = 0x4;
pub const TXTLOG_DETAILS: DWORD = 0x5;
pub const TXTLOG_VERBOSE: DWORD = 0x6;
pub const TXTLOG_VERY_VERBOSE: DWORD = 0x7;
pub const TXTLOG_RESERVED_FLAGS: DWORD = 0x0000FFF0;
pub const TXTLOG_TIMESTAMP: DWORD = 0x00010000;
pub const TXTLOG_DEPTH_INCR: DWORD = 0x00020000;
pub const TXTLOG_DEPTH_DECR: DWORD = 0x00040000;
pub const TXTLOG_TAB_1: DWORD = 0x00080000;
pub const TXTLOG_FLUSH_FILE: DWORD = 0x00100000;
#[inline]
pub fn TXTLOG_LEVEL(flags: DWORD) -> DWORD {
    flags & 0xf
}
pub const TXTLOG_DEVINST: DWORD = 0x00000001;
pub const TXTLOG_INF: DWORD = 0x00000002;
pub const TXTLOG_FILEQ: DWORD = 0x00000004;
pub const TXTLOG_COPYFILES: DWORD = 0x00000008;
pub const TXTLOG_SIGVERIF: DWORD = 0x00000020;
pub const TXTLOG_BACKUP: DWORD = 0x00000080;
pub const TXTLOG_UI: DWORD = 0x00000100;
pub const TXTLOG_UTIL: DWORD = 0x00000200;
pub const TXTLOG_INFDB: DWORD = 0x00000400;
pub const TXTLOG_POLICY: DWORD = 0x00800000;
pub const TXTLOG_NEWDEV: DWORD = 0x01000000;
pub const TXTLOG_UMPNPMGR: DWORD = 0x02000000;
pub const TXTLOG_DRIVER_STORE: DWORD = 0x04000000;
pub const TXTLOG_SETUP: DWORD = 0x08000000;
pub const TXTLOG_CMI: DWORD = 0x10000000;
pub const TXTLOG_DEVMGR: DWORD = 0x20000000;
pub const TXTLOG_INSTALLER: DWORD = 0x40000000;
pub const TXTLOG_VENDOR: DWORD = 0x80000000;
