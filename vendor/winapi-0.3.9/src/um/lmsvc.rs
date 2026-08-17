// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
//! This file contains structures, function prototypes, and definitions for the NetService API
use ctypes::c_long;
use shared::lmcons::NET_API_STATUS;
use shared::minwindef::{DWORD, LPBYTE, LPDWORD};
use um::winnt::{LPCWSTR, LPWSTR};
STRUCT!{struct SERVICE_INFO_0 {
    svci0_name: LPWSTR,
}}
pub type PSERVICE_INFO_0 = *mut SERVICE_INFO_0;
pub type LPSERVICE_INFO_0 = *mut SERVICE_INFO_0;
STRUCT!{struct SERVICE_INFO_1 {
    svci1_name: LPWSTR,
    svci1_status: DWORD,
    svci1_code: DWORD,
    svci1_pid: DWORD,
}}
pub type PSERVICE_INFO_1 = *mut SERVICE_INFO_1;
pub type LPSERVICE_INFO_1 = *mut SERVICE_INFO_1;
STRUCT!{struct SERVICE_INFO_2 {
    svci2_name: LPWSTR,
    svci2_status: DWORD,
    svci2_code: DWORD,
    svci2_pid: DWORD,
    svci2_text: LPWSTR,
    svci2_specific_error: DWORD,
    svci2_display_name: LPWSTR,
}}
pub type PSERVICE_INFO_2 = *mut SERVICE_INFO_2;
pub type LPSERVICE_INFO_2 = *mut SERVICE_INFO_2;
extern "system" {
    pub fn NetServiceControl(
        servername: LPCWSTR,
        service: LPCWSTR,
        opcode: DWORD,
        arg: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServiceEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetServiceGetInfo(
        servername: LPCWSTR,
        service: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServiceInstall(
        servername: LPCWSTR,
        service: LPCWSTR,
        argc: DWORD,
        argv: *mut LPCWSTR,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
pub const SERVICE_INSTALL_STATE: DWORD = 0x03;
pub const SERVICE_UNINSTALLED: DWORD = 0x00;
pub const SERVICE_INSTALL_PENDING: DWORD = 0x01;
pub const SERVICE_UNINSTALL_PENDING: DWORD = 0x02;
pub const SERVICE_INSTALLED: DWORD = 0x03;
pub const SERVICE_PAUSE_STATE: DWORD = 0x0C;
pub const LM20_SERVICE_ACTIVE: DWORD = 0x00;
pub const LM20_SERVICE_CONTINUE_PENDING: DWORD = 0x04;
pub const LM20_SERVICE_PAUSE_PENDING: DWORD = 0x08;
pub const LM20_SERVICE_PAUSED: DWORD = 0x0C;
pub const SERVICE_NOT_UNINSTALLABLE: DWORD = 0x00;
pub const SERVICE_UNINSTALLABLE: DWORD = 0x10;
pub const SERVICE_NOT_PAUSABLE: DWORD = 0x00;
pub const SERVICE_PAUSABLE: DWORD = 0x20;
pub const SERVICE_REDIR_PAUSED: DWORD = 0x700;
pub const SERVICE_REDIR_DISK_PAUSED: DWORD = 0x100;
pub const SERVICE_REDIR_PRINT_PAUSED: DWORD = 0x200;
pub const SERVICE_REDIR_COMM_PAUSED: DWORD = 0x400;
pub const SERVICE_DOS_ENCRYPTION: &'static str = "ENCRYPT";
pub const SERVICE_CTRL_INTERROGATE: DWORD = 0;
pub const SERVICE_CTRL_PAUSE: DWORD = 1;
pub const SERVICE_CTRL_CONTINUE: DWORD = 2;
pub const SERVICE_CTRL_UNINSTALL: DWORD = 3;
pub const SERVICE_CTRL_REDIR_DISK: DWORD = 0x1;
pub const SERVICE_CTRL_REDIR_PRINT: DWORD = 0x2;
pub const SERVICE_CTRL_REDIR_COMM: DWORD = 0x4;
pub const SERVICE_IP_NO_HINT: DWORD = 0x0;
pub const SERVICE_CCP_NO_HINT: DWORD = 0x0;
pub const SERVICE_IP_QUERY_HINT: DWORD = 0x10000;
pub const SERVICE_CCP_QUERY_HINT: DWORD = 0x10000;
pub const SERVICE_IP_CHKPT_NUM: DWORD = 0x0FF;
pub const SERVICE_CCP_CHKPT_NUM: DWORD = 0x0FF;
pub const SERVICE_IP_WAIT_TIME: DWORD = 0x0FF00;
pub const SERVICE_CCP_WAIT_TIME: DWORD = 0x0FF00;
pub const SERVICE_IP_WAITTIME_SHIFT: DWORD = 8;
pub const SERVICE_NTIP_WAITTIME_SHIFT: DWORD = 12;
pub const UPPER_HINT_MASK: DWORD = 0x0000FF00;
pub const LOWER_HINT_MASK: DWORD = 0x000000FF;
pub const UPPER_GET_HINT_MASK: DWORD = 0x0FF00000;
pub const LOWER_GET_HINT_MASK: DWORD = 0x0000FF00;
pub const SERVICE_NT_MAXTIME: DWORD = 0x0000FFFF;
pub const SERVICE_RESRV_MASK: DWORD = 0x0001FFFF;
pub const SERVICE_MAXTIME: DWORD = 0x000000FF;
pub const SERVICE_BASE: DWORD = 3050;
pub const SERVICE_UIC_NORMAL: DWORD = 0;
pub const SERVICE_UIC_BADPARMVAL: DWORD = SERVICE_BASE + 1;
pub const SERVICE_UIC_MISSPARM: DWORD = SERVICE_BASE + 2;
pub const SERVICE_UIC_UNKPARM: DWORD = SERVICE_BASE + 3;
pub const SERVICE_UIC_RESOURCE: DWORD = SERVICE_BASE + 4;
pub const SERVICE_UIC_CONFIG: DWORD = SERVICE_BASE + 5;
pub const SERVICE_UIC_SYSTEM: DWORD = SERVICE_BASE + 6;
pub const SERVICE_UIC_INTERNAL: DWORD = SERVICE_BASE + 7;
pub const SERVICE_UIC_AMBIGPARM: DWORD = SERVICE_BASE + 8;
pub const SERVICE_UIC_DUPPARM: DWORD = SERVICE_BASE + 9;
pub const SERVICE_UIC_KILL: DWORD = SERVICE_BASE + 10;
pub const SERVICE_UIC_EXEC: DWORD = SERVICE_BASE + 11;
pub const SERVICE_UIC_SUBSERV: DWORD = SERVICE_BASE + 12;
pub const SERVICE_UIC_CONFLPARM: DWORD = SERVICE_BASE + 13;
pub const SERVICE_UIC_FILE: DWORD = SERVICE_BASE + 14;
pub const SERVICE_UIC_M_NULL: DWORD = 0;
pub const SERVICE_UIC_M_MEMORY: DWORD = SERVICE_BASE + 20;
pub const SERVICE_UIC_M_DISK: DWORD = SERVICE_BASE + 21;
pub const SERVICE_UIC_M_THREADS: DWORD = SERVICE_BASE + 22;
pub const SERVICE_UIC_M_PROCESSES: DWORD = SERVICE_BASE + 23;
pub const SERVICE_UIC_M_SECURITY: DWORD = SERVICE_BASE + 24;
pub const SERVICE_UIC_M_LANROOT: DWORD = SERVICE_BASE + 25;
pub const SERVICE_UIC_M_REDIR: DWORD = SERVICE_BASE + 26;
pub const SERVICE_UIC_M_SERVER: DWORD = SERVICE_BASE + 27;
pub const SERVICE_UIC_M_SEC_FILE_ERR: DWORD = SERVICE_BASE + 28;
pub const SERVICE_UIC_M_FILES: DWORD = SERVICE_BASE + 29;
pub const SERVICE_UIC_M_LOGS: DWORD = SERVICE_BASE + 30;
pub const SERVICE_UIC_M_LANGROUP: DWORD = SERVICE_BASE + 31;
pub const SERVICE_UIC_M_MSGNAME: DWORD = SERVICE_BASE + 32;
pub const SERVICE_UIC_M_ANNOUNCE: DWORD = SERVICE_BASE + 33;
pub const SERVICE_UIC_M_UAS: DWORD = SERVICE_BASE + 34;
pub const SERVICE_UIC_M_SERVER_SEC_ERR: DWORD = SERVICE_BASE + 35;
pub const SERVICE_UIC_M_WKSTA: DWORD = SERVICE_BASE + 37;
pub const SERVICE_UIC_M_ERRLOG: DWORD = SERVICE_BASE + 38;
pub const SERVICE_UIC_M_FILE_UW: DWORD = SERVICE_BASE + 39;
pub const SERVICE_UIC_M_ADDPAK: DWORD = SERVICE_BASE + 40;
pub const SERVICE_UIC_M_LAZY: DWORD = SERVICE_BASE + 41;
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: DWORD = SERVICE_BASE + 42;
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: DWORD = SERVICE_BASE + 43;
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: DWORD = SERVICE_BASE + 44;
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: DWORD = SERVICE_BASE + 45;
pub const SERVICE_UIC_M_NETLOGON_NO_DC: DWORD = SERVICE_BASE + 46;
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: DWORD = SERVICE_BASE + 47;
pub const SERVICE_UIC_M_NETLOGON_AUTH: DWORD = SERVICE_BASE + 48;
pub const SERVICE_UIC_M_UAS_PROLOG: DWORD = SERVICE_BASE + 49;
pub const SERVICE2_BASE: DWORD = 5600;
pub const SERVICE_UIC_M_NETLOGON_MPATH: DWORD = SERVICE2_BASE + 0;
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: DWORD = SERVICE2_BASE + 1;
pub const SERVICE_UIC_M_DATABASE_ERROR: DWORD = SERVICE2_BASE + 2;
#[inline]
pub fn SERVICE_IP_CODE(tt: DWORD, nn: DWORD) -> c_long {
    (SERVICE_IP_QUERY_HINT | (nn | (tt << SERVICE_IP_WAITTIME_SHIFT))) as c_long
}
#[inline]
pub fn SERVICE_CCP_CODE(tt: DWORD, nn: DWORD) -> c_long {
    (SERVICE_CCP_QUERY_HINT | (nn | (tt << SERVICE_IP_WAITTIME_SHIFT))) as c_long
}
#[inline]
pub fn SERVICE_UIC_CODE(cc: DWORD, mm: DWORD) -> c_long {
    ((cc << 16) | mm) as c_long
}
#[inline]
pub fn SERVICE_NT_CCP_CODE(tt: DWORD, nn: DWORD) -> c_long {
    (SERVICE_CCP_QUERY_HINT | nn | ((tt & LOWER_HINT_MASK) << SERVICE_IP_WAITTIME_SHIFT)
    | ((tt & UPPER_HINT_MASK) << SERVICE_NTIP_WAITTIME_SHIFT)) as c_long
}
#[inline]
pub fn SERVICE_NT_WAIT_GET(code: DWORD) -> DWORD {
    ((code & UPPER_GET_HINT_MASK) >> SERVICE_NTIP_WAITTIME_SHIFT)
    | ((code & LOWER_GET_HINT_MASK) >> SERVICE_IP_WAITTIME_SHIFT)
}
