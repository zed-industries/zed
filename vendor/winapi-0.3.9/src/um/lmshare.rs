// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! This module defines the API function prototypes and data structures
use shared::basetsd::PDWORD_PTR;
use shared::guiddef::GUID;
use shared::lmcons::{LMSTR, NET_API_STATUS, PARMNUM_BASE_INFOLEVEL};
use shared::minwindef::{DWORD, LPBYTE, LPDWORD, ULONG};
use um::winnt::{BOOLEAN, PSECURITY_DESCRIPTOR};
extern "system" {
    pub fn NetShareAdd(
        servername: LMSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareEnum(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareEnumSticky(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareGetInfo(
        servername: LMSTR,
        netname: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetShareSetInfo(
        servername: LMSTR,
        netname: LMSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareDel(
        servername: LMSTR,
        netname: LMSTR,
        reserved: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareDelSticky(
        servername: LMSTR,
        netname: LMSTR,
        reserved: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareCheck(
        servername: LMSTR,
        device: LMSTR,
        _type: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetShareDelEx(
        servername: LMSTR,
        level: DWORD,
        buf: LPBYTE,
    ) -> NET_API_STATUS;
}
STRUCT!{struct SHARE_INFO_0 {
    shi0_netname: LMSTR,
}}
pub type PSHARE_INFO_0 = *mut SHARE_INFO_0;
pub type LPSHARE_INFO_0 = *mut SHARE_INFO_0;
STRUCT!{struct SHARE_INFO_1 {
    shi1_netname: LMSTR,
    shi1_type: DWORD,
    shi1_remark: LMSTR,
}}
pub type PSHARE_INFO_1 = *mut SHARE_INFO_1;
pub type LPSHARE_INFO_1 = *mut SHARE_INFO_1;
STRUCT!{struct SHARE_INFO_2 {
    shi2_netname: LMSTR,
    shi2_type: DWORD,
    shi2_remark: LMSTR,
    shi2_permissions: DWORD,
    shi2_max_uses: DWORD,
    shi2_current_uses: DWORD,
    shi2_path: LMSTR,
    shi2_passwd: LMSTR,
}}
pub type PSHARE_INFO_2 = *mut SHARE_INFO_2;
pub type LPSHARE_INFO_2 = *mut SHARE_INFO_2;
STRUCT!{struct SHARE_INFO_501 {
    shi501_netname: LMSTR,
    shi501_type: DWORD,
    shi501_remark: LMSTR,
    shi501_flags: DWORD,
}}
pub type PSHARE_INFO_501 = *mut SHARE_INFO_501;
pub type LPSHARE_INFO_501 = *mut SHARE_INFO_501;
STRUCT!{struct SHARE_INFO_502 {
    shi502_netname: LMSTR,
    shi502_type: DWORD,
    shi502_remark: LMSTR,
    shi502_permissions: DWORD,
    shi502_max_uses: DWORD,
    shi502_current_uses: DWORD,
    shi502_path: LMSTR,
    shi502_passwd: LMSTR,
    shi502_reserved: DWORD,
    shi502_security_descriptor: PSECURITY_DESCRIPTOR,
}}
pub type PSHARE_INFO_502 = *mut SHARE_INFO_502;
pub type LPSHARE_INFO_502 = *mut SHARE_INFO_502;
STRUCT!{struct SHARE_INFO_503 {
    shi503_netname: LMSTR,
    shi503_type: DWORD,
    shi503_remark: LMSTR,
    shi503_permissions: DWORD,
    shi503_max_uses: DWORD,
    shi503_current_uses: DWORD,
    shi503_path: LMSTR,
    shi503_passwd: LMSTR,
    shi503_servername: LMSTR,
    shi503_reserved: DWORD,
    shi503_security_descriptor: PSECURITY_DESCRIPTOR,
}}
pub type PSHARE_INFO_503 = *mut SHARE_INFO_503;
pub type LPSHARE_INFO_503 = *mut SHARE_INFO_503;
STRUCT!{struct SHARE_INFO_1004 {
    shi1004_remark: LMSTR,
}}
pub type PSHARE_INFO_1004 = *mut SHARE_INFO_1004;
pub type LPSHARE_INFO_1004 = *mut SHARE_INFO_1004;
STRUCT!{struct SHARE_INFO_1005 {
    shi1005_flags: DWORD,
}}
pub type PSHARE_INFO_1005 = *mut SHARE_INFO_1005;
pub type LPSHARE_INFO_1005 = *mut SHARE_INFO_1005;
STRUCT!{struct SHARE_INFO_1006 {
    shi1006_max_uses: DWORD,
}}
pub type PSHARE_INFO_1006 = *mut SHARE_INFO_1006;
pub type LPSHARE_INFO_1006 = *mut SHARE_INFO_1006;
STRUCT!{struct SHARE_INFO_1501 {
    shi1501_reserved: DWORD,
    shi1501_security_descriptor: PSECURITY_DESCRIPTOR,
}}
pub type PSHARE_INFO_1501 = *mut SHARE_INFO_1501;
pub type LPSHARE_INFO_1501 = *mut SHARE_INFO_1501;
STRUCT!{struct SHARE_INFO_1503 {
    shi1503_sharefilter: GUID,
}}
pub type PSHARE_INFO_1503 = *mut SHARE_INFO_1503;
pub type LPSHARE_INFO_1503 = *mut SHARE_INFO_1503;
extern "system" {
    pub fn NetServerAliasAdd(
        servername: LMSTR,
        level: DWORD,
        buf: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerAliasDel(
        servername: LMSTR,
        level: DWORD,
        buf: LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetServerAliasEnum(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct SERVER_ALIAS_INFO_0 {
    srvai0_alias: LMSTR,
    srvai0_target: LMSTR,
    srvai0_default: BOOLEAN,
    srvai0_reserved: ULONG,
}}
pub type PSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub type LPSERVER_ALIAS_INFO_0 = *mut SERVER_ALIAS_INFO_0;
pub const SHARE_NETNAME_PARMNUM: DWORD = 1;
pub const SHARE_TYPE_PARMNUM: DWORD = 3;
pub const SHARE_REMARK_PARMNUM: DWORD = 4;
pub const SHARE_PERMISSIONS_PARMNUM: DWORD = 5;
pub const SHARE_MAX_USES_PARMNUM: DWORD = 6;
pub const SHARE_CURRENT_USES_PARMNUM: DWORD = 7;
pub const SHARE_PATH_PARMNUM: DWORD = 8;
pub const SHARE_PASSWD_PARMNUM: DWORD = 9;
pub const SHARE_FILE_SD_PARMNUM: DWORD = 501;
pub const SHARE_SERVER_PARMNUM: DWORD = 503;
pub const SHARE_REMARK_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SHARE_REMARK_PARMNUM;
pub const SHARE_MAX_USES_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SHARE_MAX_USES_PARMNUM;
pub const SHARE_FILE_SD_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + SHARE_FILE_SD_PARMNUM;
pub const SHI1_NUM_ELEMENTS: DWORD = 4;
pub const SHI2_NUM_ELEMENTS: DWORD = 10;
pub const STYPE_DISKTREE: DWORD = 0;
pub const STYPE_PRINTQ: DWORD = 1;
pub const STYPE_DEVICE: DWORD = 2;
pub const STYPE_IPC: DWORD = 3;
pub const STYPE_MASK: DWORD = 0x000000FF;
pub const STYPE_RESERVED1: DWORD = 0x01000000;
pub const STYPE_RESERVED2: DWORD = 0x02000000;
pub const STYPE_RESERVED3: DWORD = 0x04000000;
pub const STYPE_RESERVED4: DWORD = 0x08000000;
pub const STYPE_RESERVED_ALL: DWORD = 0x3FFFFF00;
pub const STYPE_TEMPORARY: DWORD = 0x40000000;
pub const STYPE_SPECIAL: DWORD = 0x80000000;
pub const SHI_USES_UNLIMITED: DWORD = -1i32 as u32;
pub const SHI1005_FLAGS_DFS: DWORD = 0x0001;
pub const SHI1005_FLAGS_DFS_ROOT: DWORD = 0x0002;
pub const CSC_MASK_EXT: DWORD = 0x2030;
pub const CSC_MASK: DWORD = 0x0030;
pub const CSC_CACHE_MANUAL_REINT: DWORD = 0x0000;
pub const CSC_CACHE_AUTO_REINT: DWORD = 0x0010;
pub const CSC_CACHE_VDO: DWORD = 0x0020;
pub const CSC_CACHE_NONE: DWORD = 0x0030;
pub const SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS: DWORD = 0x00100;
pub const SHI1005_FLAGS_FORCE_SHARED_DELETE: DWORD = 0x00200;
pub const SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING: DWORD = 0x00400;
pub const SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM: DWORD = 0x00800;
pub const SHI1005_FLAGS_FORCE_LEVELII_OPLOCK: DWORD = 0x01000;
pub const SHI1005_FLAGS_ENABLE_HASH: DWORD = 0x02000;
pub const SHI1005_FLAGS_ENABLE_CA: DWORD = 0x04000;
pub const SHI1005_FLAGS_ENCRYPT_DATA: DWORD = 0x08000;
pub const SHI1005_FLAGS_RESERVED: DWORD = 0x10000;
pub const SHI1005_VALID_FLAGS_SET: DWORD = CSC_MASK | SHI1005_FLAGS_RESTRICT_EXCLUSIVE_OPENS
    | SHI1005_FLAGS_FORCE_SHARED_DELETE | SHI1005_FLAGS_ALLOW_NAMESPACE_CACHING
    | SHI1005_FLAGS_ACCESS_BASED_DIRECTORY_ENUM | SHI1005_FLAGS_FORCE_LEVELII_OPLOCK
    | SHI1005_FLAGS_ENABLE_HASH | SHI1005_FLAGS_ENABLE_CA | SHI1005_FLAGS_ENCRYPT_DATA
    | SHI1005_FLAGS_RESERVED;
extern "system" {
    pub fn NetSessionEnum(
        servername: LMSTR,
        UncClientName: LMSTR,
        username: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetSessionDel(
        servername: LMSTR,
        UncClientName: LMSTR,
        username: LMSTR,
    ) -> NET_API_STATUS;
    pub fn NetSessionGetInfo(
        servername: LMSTR,
        UncClientName: LMSTR,
        username: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
STRUCT!{struct SESSION_INFO_0 {
    sesi0_cname: LMSTR,
}}
pub type PSESSION_INFO_0 = *mut SESSION_INFO_0;
pub type LPSESSION_INFO_0 = *mut SESSION_INFO_0;
STRUCT!{struct SESSION_INFO_1 {
    sesi1_cname: LMSTR,
    sesi1_username: LMSTR,
    sesi1_num_opens: DWORD,
    sesi1_time: DWORD,
    sesi1_idle_time: DWORD,
    sesi1_user_flags: DWORD,
}}
pub type PSESSION_INFO_1 = *mut SESSION_INFO_1;
pub type LPSESSION_INFO_1 = *mut SESSION_INFO_1;
STRUCT!{struct SESSION_INFO_2 {
    sesi2_cname: LMSTR,
    sesi2_username: LMSTR,
    sesi2_num_opens: DWORD,
    sesi2_time: DWORD,
    sesi2_idle_time: DWORD,
    sesi2_user_flags: DWORD,
    sesi2_cltype_name: LMSTR,
}}
pub type PSESSION_INFO_2 = *mut SESSION_INFO_2;
pub type LPSESSION_INFO_2 = *mut SESSION_INFO_2;
STRUCT!{struct SESSION_INFO_10 {
    sesi10_cname: LMSTR,
    sesi10_username: LMSTR,
    sesi10_time: DWORD,
    sesi10_idle_time: DWORD,
}}
pub type PSESSION_INFO_10 = *mut SESSION_INFO_10;
pub type LPSESSION_INFO_10 = *mut SESSION_INFO_10;
STRUCT!{struct SESSION_INFO_502 {
    sesi502_cname: LMSTR,
    sesi502_username: LMSTR,
    sesi502_num_opens: DWORD,
    sesi502_time: DWORD,
    sesi502_idle_time: DWORD,
    sesi502_user_flags: DWORD,
    sesi502_cltype_name: LMSTR,
    sesi502_transport: LMSTR,
}}
pub type PSESSION_INFO_502 = *mut SESSION_INFO_502;
pub type LPSESSION_INFO_502 = *mut SESSION_INFO_502;
pub const SESS_GUEST: DWORD = 0x00000001;
pub const SESS_NOENCRYPTION: DWORD = 0x00000002;
pub const SESI1_NUM_ELEMENTS: DWORD = 8;
pub const SESI2_NUM_ELEMENTS: DWORD = 9;
extern "system" {
    pub fn NetConnectionEnum(
        servername: LMSTR,
        qualifier: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resume_handle: LPDWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct CONNECTION_INFO_0 {
    coni0_id: DWORD,
}}
pub type PCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
pub type LPCONNECTION_INFO_0 = *mut CONNECTION_INFO_0;
STRUCT!{struct CONNECTION_INFO_1 {
    coni1_id: DWORD,
    coni1_type: DWORD,
    coni1_num_opens: DWORD,
    coni1_num_users: DWORD,
    coni1_time: DWORD,
    coni1_username: LMSTR,
    coni1_netname: LMSTR,
}}
pub type PCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
pub type LPCONNECTION_INFO_1 = *mut CONNECTION_INFO_1;
extern "system" {
    pub fn NetFileClose(
        servername: LMSTR,
        fileid: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetFileEnum(
        servername: LMSTR,
        basepath: LMSTR,
        username: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resume_handle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetFileGetInfo(
        servername: LMSTR,
        fileid: DWORD,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
STRUCT!{struct FILE_INFO_2 {
    fi2_id: DWORD,
}}
pub type PFILE_INFO_2 = *mut FILE_INFO_2;
pub type LPFILE_INFO_2 = *mut FILE_INFO_2;
STRUCT!{struct FILE_INFO_3 {
    fi3_id: DWORD,
    fi3_permissions: DWORD,
    fi3_num_locks: DWORD,
    fi3_pathname: LMSTR,
    fi3_username: LMSTR,
}}
pub type PFILE_INFO_3 = *mut FILE_INFO_3;
pub type LPFILE_INFO_3 = *mut FILE_INFO_3;
pub const PERM_FILE_READ: DWORD = 0x1;
pub const PERM_FILE_WRITE: DWORD = 0x2;
pub const PERM_FILE_CREATE: DWORD = 0x4;
