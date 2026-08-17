// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This file contains structures, function prototypes, and definitions for the replicator APIs
use shared::lmcons::{NET_API_STATUS, PARMNUM_BASE_INFOLEVEL};
use shared::minwindef::{DWORD, LPBYTE, LPDWORD};
use um::winnt::{LPCWSTR, LPWSTR};
pub const REPL_ROLE_EXPORT: DWORD = 1;
pub const REPL_ROLE_IMPORT: DWORD = 2;
pub const REPL_ROLE_BOTH: DWORD = 3;
pub const REPL_INTERVAL_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 0;
pub const REPL_PULSE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 1;
pub const REPL_GUARDTIME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 2;
pub const REPL_RANDOM_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 3;
STRUCT!{struct REPL_INFO_0 {
    rp0_role: DWORD,
    rp0_exportpath: LPWSTR,
    rp0_exportlist: LPWSTR,
    rp0_importpath: LPWSTR,
    rp0_importlist: LPWSTR,
    rp0_logonusername: LPWSTR,
    rp0_interval: DWORD,
    rp0_pulse: DWORD,
    rp0_guardtime: DWORD,
    rp0_random: DWORD,
}}
pub type PREPL_INFO_0 = *mut REPL_INFO_0;
pub type LPREPL_INFO_0 = *mut REPL_INFO_0;
STRUCT!{struct REPL_INFO_1000 {
    rp1000_interval: DWORD,
}}
pub type PREPL_INFO_1000 = *mut REPL_INFO_1000;
pub type LPREPL_INFO_1000 = *mut REPL_INFO_1000;
STRUCT!{struct REPL_INFO_1001 {
    rp1001_pulse: DWORD,
}}
pub type PREPL_INFO_1001 = *mut REPL_INFO_1001;
pub type LPREPL_INFO_1001 = *mut REPL_INFO_1001;
STRUCT!{struct REPL_INFO_1002 {
    rp1002_guardtime: DWORD,
}}
pub type PREPL_INFO_1002 = *mut REPL_INFO_1002;
pub type LPREPL_INFO_1002 = *mut REPL_INFO_1002;
STRUCT!{struct REPL_INFO_1003 {
    rp1003_random: DWORD,
}}
pub type PREPL_INFO_1003 = *mut REPL_INFO_1003;
pub type LPREPL_INFO_1003 = *mut REPL_INFO_1003;
extern "system" {
    pub fn NetReplGetInfo(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplSetInfo(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
}
pub const REPL_INTEGRITY_FILE: DWORD = 1;
pub const REPL_INTEGRITY_TREE: DWORD = 2;
pub const REPL_EXTENT_FILE: DWORD = 1;
pub const REPL_EXTENT_TREE: DWORD = 2;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 0;
pub const REPL_EXPORT_EXTENT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + 1;
STRUCT!{struct REPL_EDIR_INFO_0 {
    rped0_dirname: LPWSTR,
}}
pub type PREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
pub type LPREPL_EDIR_INFO_0 = *mut REPL_EDIR_INFO_0;
STRUCT!{struct REPL_EDIR_INFO_1 {
    rped1_dirname: LPWSTR,
    rped1_integrity: DWORD,
    rped1_extent: DWORD,
}}
pub type PREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
pub type LPREPL_EDIR_INFO_1 = *mut REPL_EDIR_INFO_1;
STRUCT!{struct REPL_EDIR_INFO_2 {
    rped2_dirname: LPWSTR,
    rped2_integrity: DWORD,
    rped2_extent: DWORD,
    rped2_lockcount: DWORD,
    rped2_locktime: DWORD,
}}
pub type PREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
pub type LPREPL_EDIR_INFO_2 = *mut REPL_EDIR_INFO_2;
STRUCT!{struct REPL_EDIR_INFO_1000 {
    rped1000_integrity: DWORD,
}}
pub type PREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
pub type LPREPL_EDIR_INFO_1000 = *mut REPL_EDIR_INFO_1000;
STRUCT!{struct REPL_EDIR_INFO_1001 {
    rped1001_extent: DWORD,
}}
pub type PREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
pub type LPREPL_EDIR_INFO_1001 = *mut REPL_EDIR_INFO_1001;
extern "system" {
    pub fn NetReplExportDirAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirDel(
        servername: LPCWSTR,
        dirname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirGetInfo(
        servername: LPCWSTR,
        dirname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirSetInfo(
        servername: LPCWSTR,
        dirname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirLock(
        servername: LPCWSTR,
        dirname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetReplExportDirUnlock(
        servername: LPCWSTR,
        dirname: LPCWSTR,
        unlockforce: DWORD,
    ) -> NET_API_STATUS;
}
pub const REPL_UNLOCK_NOFORCE: DWORD = 0;
pub const REPL_UNLOCK_FORCE: DWORD = 1;
STRUCT!{struct REPL_IDIR_INFO_0 {
    rpid0_dirname: LPWSTR,
}}
pub type PREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
pub type LPREPL_IDIR_INFO_0 = *mut REPL_IDIR_INFO_0;
STRUCT!{struct REPL_IDIR_INFO_1 {
    rpid1_dirname: LPWSTR,
    rpid1_state: DWORD,
    rpid1_mastername: LPWSTR,
    rpid1_last_update_time: DWORD,
    rpid1_lockcount: DWORD,
    rpid1_locktime: DWORD,
}}
pub type PREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
pub type LPREPL_IDIR_INFO_1 = *mut REPL_IDIR_INFO_1;
extern "system" {
    pub fn NetReplImportDirAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirDel(
        servername: LPCWSTR,
        dirname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirGetInfo(
        servername: LPCWSTR,
        dirname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirLock(
        servername: LPCWSTR,
        dirname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetReplImportDirUnlock(
        servername: LPCWSTR,
        dirname: LPCWSTR,
        unlockforce: DWORD,
    ) -> NET_API_STATUS;
}
pub const REPL_STATE_OK: DWORD = 0;
pub const REPL_STATE_NO_MASTER: DWORD = 1;
pub const REPL_STATE_NO_SYNC: DWORD = 2;
pub const REPL_STATE_NEVER_REPLICATED: DWORD = 3;
