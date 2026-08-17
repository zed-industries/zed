// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
use shared::lmcons::{LMSTR, NET_API_STATUS};
use shared::minwindef::{BOOL, DWORD, LPBYTE, LPDWORD};
use um::winnt::LPCWSTR;
extern "system" {
    pub fn NetWkstaGetInfo(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetWkstaSetInfo(
        servername: LMSTR,
        level: DWORD,
        buffer: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserGetInfo(
        reserved: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserSetInfo(
        reserved: LMSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaUserEnum(
        servername: LMSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportDel(
        servername: LMSTR,
        transportname: LMSTR,
        ucond: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetWkstaTransportEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct WKSTA_INFO_100 {
    wki100_platform_id: DWORD,
    wki100_computername: LMSTR,
    wki100_langroup: LMSTR,
    wki100_ver_major: DWORD,
    wki100_ver_minor: DWORD,
}}
pub type PWKSTA_INFO_100 = *mut WKSTA_INFO_100;
pub type LPWKSTA_INFO_100 = *mut WKSTA_INFO_100;
STRUCT!{struct WKSTA_INFO_101 {
    wki101_platform_id: DWORD,
    wki101_computername: LMSTR,
    wki101_langroup: LMSTR,
    wki101_ver_major: DWORD,
    wki101_ver_minor: DWORD,
    wki101_lanroot: LMSTR,
}}
pub type PWKSTA_INFO_101 = *mut WKSTA_INFO_101;
pub type LPWKSTA_INFO_101 = *mut WKSTA_INFO_101;
STRUCT!{struct WKSTA_INFO_102 {
    wki102_platform_id: DWORD,
    wki102_computername: LMSTR,
    wki102_langroup: LMSTR,
    wki102_ver_major: DWORD,
    wki102_ver_minor: DWORD,
    wki102_lanroot: LMSTR,
    wki102_logged_on_users: DWORD,
}}
pub type PWKSTA_INFO_102 = *mut WKSTA_INFO_102;
pub type LPWKSTA_INFO_102 = *mut WKSTA_INFO_102;
STRUCT!{struct WKSTA_INFO_302 {
    wki302_char_wait: DWORD,
    wki302_collection_time: DWORD,
    wki302_maximum_collection_count: DWORD,
    wki302_keep_conn: DWORD,
    wki302_keep_search: DWORD,
    wki302_max_cmds: DWORD,
    wki302_num_work_buf: DWORD,
    wki302_siz_work_buf: DWORD,
    wki302_max_wrk_cache: DWORD,
    wki302_sess_timeout: DWORD,
    wki302_siz_error: DWORD,
    wki302_num_alerts: DWORD,
    wki302_num_services: DWORD,
    wki302_errlog_sz: DWORD,
    wki302_print_buf_time: DWORD,
    wki302_num_char_buf: DWORD,
    wki302_siz_char_buf: DWORD,
    wki302_wrk_heuristics: LMSTR,
    wki302_mailslots: DWORD,
    wki302_num_dgram_buf: DWORD,
}}
pub type PWKSTA_INFO_302 = *mut WKSTA_INFO_302;
pub type LPWKSTA_INFO_302 = *mut WKSTA_INFO_302;
STRUCT!{struct WKSTA_INFO_402 {
    wki402_char_wait: DWORD,
    wki402_collection_time: DWORD,
    wki402_maximum_collection_count: DWORD,
    wki402_keep_conn: DWORD,
    wki402_keep_search: DWORD,
    wki402_max_cmds: DWORD,
    wki402_num_work_buf: DWORD,
    wki402_siz_work_buf: DWORD,
    wki402_max_wrk_cache: DWORD,
    wki402_sess_timeout: DWORD,
    wki402_siz_error: DWORD,
    wki402_num_alerts: DWORD,
    wki402_num_services: DWORD,
    wki402_errlog_sz: DWORD,
    wki402_print_buf_time: DWORD,
    wki402_num_char_buf: DWORD,
    wki402_siz_char_buf: DWORD,
    wki402_wrk_heuristics: LMSTR,
    wki402_mailslots: DWORD,
    wki402_num_dgram_buf: DWORD,
    wki402_max_threads: DWORD,
}}
pub type PWKSTA_INFO_402 = *mut WKSTA_INFO_402;
pub type LPWKSTA_INFO_402 = *mut WKSTA_INFO_402;
STRUCT!{struct WKSTA_INFO_502 {
    wki502_char_wait: DWORD,
    wki502_collection_time: DWORD,
    wki502_maximum_collection_count: DWORD,
    wki502_keep_conn: DWORD,
    wki502_max_cmds: DWORD,
    wki502_sess_timeout: DWORD,
    wki502_siz_char_buf: DWORD,
    wki502_max_threads: DWORD,
    wki502_lock_quota: DWORD,
    wki502_lock_increment: DWORD,
    wki502_lock_maximum: DWORD,
    wki502_pipe_increment: DWORD,
    wki502_pipe_maximum: DWORD,
    wki502_cache_file_timeout: DWORD,
    wki502_dormant_file_limit: DWORD,
    wki502_read_ahead_throughput: DWORD,
    wki502_num_mailslot_buffers: DWORD,
    wki502_num_srv_announce_buffers: DWORD,
    wki502_max_illegal_datagram_events: DWORD,
    wki502_illegal_datagram_event_reset_frequency: DWORD,
    wki502_log_election_packets: BOOL,
    wki502_use_opportunistic_locking: BOOL,
    wki502_use_unlock_behind: BOOL,
    wki502_use_close_behind: BOOL,
    wki502_buf_named_pipes: BOOL,
    wki502_use_lock_read_unlock: BOOL,
    wki502_utilize_nt_caching: BOOL,
    wki502_use_raw_read: BOOL,
    wki502_use_raw_write: BOOL,
    wki502_use_write_raw_data: BOOL,
    wki502_use_encryption: BOOL,
    wki502_buf_files_deny_write: BOOL,
    wki502_buf_read_only_files: BOOL,
    wki502_force_core_create_mode: BOOL,
    wki502_use_512_byte_max_transfer: BOOL,
}}
pub type PWKSTA_INFO_502 = *mut WKSTA_INFO_502;
pub type LPWKSTA_INFO_502 = *mut WKSTA_INFO_502;
STRUCT!{struct WKSTA_INFO_1010 {
    wki1010_char_wait: DWORD,
}}
pub type PWKSTA_INFO_1010 = *mut WKSTA_INFO_1010;
pub type LPWKSTA_INFO_1010 = *mut WKSTA_INFO_1010;
STRUCT!{struct WKSTA_INFO_1011 {
    wki1011_collection_time: DWORD,
}}
pub type PWKSTA_INFO_1011 = *mut WKSTA_INFO_1011;
pub type LPWKSTA_INFO_1011 = *mut WKSTA_INFO_1011;
STRUCT!{struct WKSTA_INFO_1012 {
    wki1012_maximum_collection_count: DWORD,
}}
pub type PWKSTA_INFO_1012 = *mut WKSTA_INFO_1012;
pub type LPWKSTA_INFO_1012 = *mut WKSTA_INFO_1012;
STRUCT!{struct WKSTA_INFO_1027 {
    wki1027_errlog_sz: DWORD,
}}
pub type PWKSTA_INFO_1027 = *mut WKSTA_INFO_1027;
pub type LPWKSTA_INFO_1027 = *mut WKSTA_INFO_1027;
STRUCT!{struct WKSTA_INFO_1028 {
    wki1028_print_buf_time: DWORD,
}}
pub type PWKSTA_INFO_1028 = *mut WKSTA_INFO_1028;
pub type LPWKSTA_INFO_1028 = *mut WKSTA_INFO_1028;
STRUCT!{struct WKSTA_INFO_1032 {
    wki1032_wrk_heuristics: DWORD,
}}
pub type PWKSTA_INFO_1032 = *mut WKSTA_INFO_1032;
pub type LPWKSTA_INFO_1032 = *mut WKSTA_INFO_1032;
STRUCT!{struct WKSTA_INFO_1013 {
    wki1013_keep_conn: DWORD,
}}
pub type PWKSTA_INFO_1013 = *mut WKSTA_INFO_1013;
pub type LPWKSTA_INFO_1013 = *mut WKSTA_INFO_1013;
STRUCT!{struct WKSTA_INFO_1018 {
    wki1018_sess_timeout: DWORD,
}}
pub type PWKSTA_INFO_1018 = *mut WKSTA_INFO_1018;
pub type LPWKSTA_INFO_1018 = *mut WKSTA_INFO_1018;
STRUCT!{struct WKSTA_INFO_1023 {
    wki1023_siz_char_buf: DWORD,
}}
pub type PWKSTA_INFO_1023 = *mut WKSTA_INFO_1023;
pub type LPWKSTA_INFO_1023 = *mut WKSTA_INFO_1023;
STRUCT!{struct WKSTA_INFO_1033 {
    wki1033_max_threads: DWORD,
}}
pub type PWKSTA_INFO_1033 = *mut WKSTA_INFO_1033;
pub type LPWKSTA_INFO_1033 = *mut WKSTA_INFO_1033;
STRUCT!{struct WKSTA_INFO_1041 {
    wki1041_lock_quota: DWORD,
}}
pub type PWKSTA_INFO_1041 = *mut WKSTA_INFO_1041;
pub type LPWKSTA_INFO_1041 = *mut WKSTA_INFO_1041;
STRUCT!{struct WKSTA_INFO_1042 {
    wki1042_lock_increment: DWORD,
}}
pub type PWKSTA_INFO_1042 = *mut WKSTA_INFO_1042;
pub type LPWKSTA_INFO_1042 = *mut WKSTA_INFO_1042;
STRUCT!{struct WKSTA_INFO_1043 {
    wki1043_lock_maximum: DWORD,
}}
pub type PWKSTA_INFO_1043 = *mut WKSTA_INFO_1043;
pub type LPWKSTA_INFO_1043 = *mut WKSTA_INFO_1043;
STRUCT!{struct WKSTA_INFO_1044 {
    wki1044_pipe_increment: DWORD,
}}
pub type PWKSTA_INFO_1044 = *mut WKSTA_INFO_1044;
pub type LPWKSTA_INFO_1044 = *mut WKSTA_INFO_1044;
STRUCT!{struct WKSTA_INFO_1045 {
    wki1045_pipe_maximum: DWORD,
}}
pub type PWKSTA_INFO_1045 = *mut WKSTA_INFO_1045;
pub type LPWKSTA_INFO_1045 = *mut WKSTA_INFO_1045;
STRUCT!{struct WKSTA_INFO_1046 {
    wki1046_dormant_file_limit: DWORD,
}}
pub type PWKSTA_INFO_1046 = *mut WKSTA_INFO_1046;
pub type LPWKSTA_INFO_1046 = *mut WKSTA_INFO_1046;
STRUCT!{struct WKSTA_INFO_1047 {
    wki1047_cache_file_timeout: DWORD,
}}
pub type PWKSTA_INFO_1047 = *mut WKSTA_INFO_1047;
pub type LPWKSTA_INFO_1047 = *mut WKSTA_INFO_1047;
STRUCT!{struct WKSTA_INFO_1048 {
    wki1048_use_opportunistic_locking: BOOL,
}}
pub type PWKSTA_INFO_1048 = *mut WKSTA_INFO_1048;
pub type LPWKSTA_INFO_1048 = *mut WKSTA_INFO_1048;
STRUCT!{struct WKSTA_INFO_1049 {
    wki1049_use_unlock_behind: BOOL,
}}
pub type PWKSTA_INFO_1049 = *mut WKSTA_INFO_1049;
pub type LPWKSTA_INFO_1049 = *mut WKSTA_INFO_1049;
STRUCT!{struct WKSTA_INFO_1050 {
    wki1050_use_close_behind: BOOL,
}}
pub type PWKSTA_INFO_1050 = *mut WKSTA_INFO_1050;
pub type LPWKSTA_INFO_1050 = *mut WKSTA_INFO_1050;
STRUCT!{struct WKSTA_INFO_1051 {
    wki1051_buf_named_pipes: BOOL,
}}
pub type PWKSTA_INFO_1051 = *mut WKSTA_INFO_1051;
pub type LPWKSTA_INFO_1051 = *mut WKSTA_INFO_1051;
STRUCT!{struct WKSTA_INFO_1052 {
    wki1052_use_lock_read_unlock: BOOL,
}}
pub type PWKSTA_INFO_1052 = *mut WKSTA_INFO_1052;
pub type LPWKSTA_INFO_1052 = *mut WKSTA_INFO_1052;
STRUCT!{struct WKSTA_INFO_1053 {
    wki1053_utilize_nt_caching: BOOL,
}}
pub type PWKSTA_INFO_1053 = *mut WKSTA_INFO_1053;
pub type LPWKSTA_INFO_1053 = *mut WKSTA_INFO_1053;
STRUCT!{struct WKSTA_INFO_1054 {
    wki1054_use_raw_read: BOOL,
}}
pub type PWKSTA_INFO_1054 = *mut WKSTA_INFO_1054;
pub type LPWKSTA_INFO_1054 = *mut WKSTA_INFO_1054;
STRUCT!{struct WKSTA_INFO_1055 {
    wki1055_use_raw_write: BOOL,
}}
pub type PWKSTA_INFO_1055 = *mut WKSTA_INFO_1055;
pub type LPWKSTA_INFO_1055 = *mut WKSTA_INFO_1055;
STRUCT!{struct WKSTA_INFO_1056 {
    wki1056_use_write_raw_data: BOOL,
}}
pub type PWKSTA_INFO_1056 = *mut WKSTA_INFO_1056;
pub type LPWKSTA_INFO_1056 = *mut WKSTA_INFO_1056;
STRUCT!{struct WKSTA_INFO_1057 {
    wki1057_use_encryption: BOOL,
}}
pub type PWKSTA_INFO_1057 = *mut WKSTA_INFO_1057;
pub type LPWKSTA_INFO_1057 = *mut WKSTA_INFO_1057;
STRUCT!{struct WKSTA_INFO_1058 {
    wki1058_buf_files_deny_write: BOOL,
}}
pub type PWKSTA_INFO_1058 = *mut WKSTA_INFO_1058;
pub type LPWKSTA_INFO_1058 = *mut WKSTA_INFO_1058;
STRUCT!{struct WKSTA_INFO_1059 {
    wki1059_buf_read_only_files: BOOL,
}}
pub type PWKSTA_INFO_1059 = *mut WKSTA_INFO_1059;
pub type LPWKSTA_INFO_1059 = *mut WKSTA_INFO_1059;
STRUCT!{struct WKSTA_INFO_1060 {
    wki1060_force_core_create_mode: BOOL,
}}
pub type PWKSTA_INFO_1060 = *mut WKSTA_INFO_1060;
pub type LPWKSTA_INFO_1060 = *mut WKSTA_INFO_1060;
STRUCT!{struct WKSTA_INFO_1061 {
    wki1061_use_512_byte_max_transfer: BOOL,
}}
pub type PWKSTA_INFO_1061 = *mut WKSTA_INFO_1061;
pub type LPWKSTA_INFO_1061 = *mut WKSTA_INFO_1061;
STRUCT!{struct WKSTA_INFO_1062 {
    wki1062_read_ahead_throughput: DWORD,
}}
pub type PWKSTA_INFO_1062 = *mut WKSTA_INFO_1062;
pub type LPWKSTA_INFO_1062 = *mut WKSTA_INFO_1062;
STRUCT!{struct WKSTA_USER_INFO_0 {
    wkui0_username: LMSTR,
}}
pub type PWKSTA_USER_INFO_0 = *mut WKSTA_USER_INFO_0;
pub type LPWKSTA_USER_INFO_0 = *mut WKSTA_USER_INFO_0;
STRUCT!{struct WKSTA_USER_INFO_1 {
    wkui1_username: LMSTR,
    wkui1_logon_domain: LMSTR,
    wkui1_oth_domains: LMSTR,
    wkui1_logon_server: LMSTR,
}}
pub type PWKSTA_USER_INFO_1 = *mut WKSTA_USER_INFO_1;
pub type LPWKSTA_USER_INFO_1 = *mut WKSTA_USER_INFO_1;
STRUCT!{struct WKSTA_USER_INFO_1101 {
    wkui1101_oth_domains: LMSTR,
}}
pub type PWKSTA_USER_INFO_1101 = *mut WKSTA_USER_INFO_1101;
pub type LPWKSTA_USER_INFO_1101 = *mut WKSTA_USER_INFO_1101;
STRUCT!{struct WKSTA_TRANSPORT_INFO_0 {
    wkti0_quality_of_service: DWORD,
    wkti0_number_of_vcs: DWORD,
    wkti0_transport_name: LMSTR,
    wkti0_transport_address: LMSTR,
    wkti0_wan_ish: BOOL,
}}
pub type PWKSTA_TRANSPORT_INFO_0 = *mut WKSTA_TRANSPORT_INFO_0;
pub type LPWKSTA_TRANSPORT_INFO_0 = *mut WKSTA_TRANSPORT_INFO_0;
pub const WKSTA_PLATFORM_ID_PARMNUM: DWORD = 100;
pub const WKSTA_COMPUTERNAME_PARMNUM: DWORD = 1;
pub const WKSTA_LANGROUP_PARMNUM: DWORD = 2;
pub const WKSTA_VER_MAJOR_PARMNUM: DWORD = 4;
pub const WKSTA_VER_MINOR_PARMNUM: DWORD = 5;
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: DWORD = 6;
pub const WKSTA_LANROOT_PARMNUM: DWORD = 7;
pub const WKSTA_LOGON_DOMAIN_PARMNUM: DWORD = 8;
pub const WKSTA_LOGON_SERVER_PARMNUM: DWORD = 9;
pub const WKSTA_CHARWAIT_PARMNUM: DWORD = 10;
pub const WKSTA_CHARTIME_PARMNUM: DWORD = 11;
pub const WKSTA_CHARCOUNT_PARMNUM: DWORD = 12;
pub const WKSTA_KEEPCONN_PARMNUM: DWORD = 13;
pub const WKSTA_KEEPSEARCH_PARMNUM: DWORD = 14;
pub const WKSTA_MAXCMDS_PARMNUM: DWORD = 15;
pub const WKSTA_NUMWORKBUF_PARMNUM: DWORD = 16;
pub const WKSTA_MAXWRKCACHE_PARMNUM: DWORD = 17;
pub const WKSTA_SESSTIMEOUT_PARMNUM: DWORD = 18;
pub const WKSTA_SIZERROR_PARMNUM: DWORD = 19;
pub const WKSTA_NUMALERTS_PARMNUM: DWORD = 20;
pub const WKSTA_NUMSERVICES_PARMNUM: DWORD = 21;
pub const WKSTA_NUMCHARBUF_PARMNUM: DWORD = 22;
pub const WKSTA_SIZCHARBUF_PARMNUM: DWORD = 23;
pub const WKSTA_ERRLOGSZ_PARMNUM: DWORD = 27;
pub const WKSTA_PRINTBUFTIME_PARMNUM: DWORD = 28;
pub const WKSTA_SIZWORKBUF_PARMNUM: DWORD = 29;
pub const WKSTA_MAILSLOTS_PARMNUM: DWORD = 30;
pub const WKSTA_NUMDGRAMBUF_PARMNUM: DWORD = 31;
pub const WKSTA_WRKHEURISTICS_PARMNUM: DWORD = 32;
pub const WKSTA_MAXTHREADS_PARMNUM: DWORD = 33;
pub const WKSTA_LOCKQUOTA_PARMNUM: DWORD = 41;
pub const WKSTA_LOCKINCREMENT_PARMNUM: DWORD = 42;
pub const WKSTA_LOCKMAXIMUM_PARMNUM: DWORD = 43;
pub const WKSTA_PIPEINCREMENT_PARMNUM: DWORD = 44;
pub const WKSTA_PIPEMAXIMUM_PARMNUM: DWORD = 45;
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: DWORD = 46;
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: DWORD = 47;
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: DWORD = 48;
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: DWORD = 49;
pub const WKSTA_USECLOSEBEHIND_PARMNUM: DWORD = 50;
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: DWORD = 51;
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: DWORD = 52;
pub const WKSTA_UTILIZENTCACHING_PARMNUM: DWORD = 53;
pub const WKSTA_USERAWREAD_PARMNUM: DWORD = 54;
pub const WKSTA_USERAWWRITE_PARMNUM: DWORD = 55;
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: DWORD = 56;
pub const WKSTA_USEENCRYPTION_PARMNUM: DWORD = 57;
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: DWORD = 58;
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: DWORD = 59;
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: DWORD = 60;
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: DWORD = 61;
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: DWORD = 62;
pub const WKSTA_OTH_DOMAINS_PARMNUM: DWORD = 101;
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: DWORD = 201;
pub const TRANSPORT_NAME_PARMNUM: DWORD = 202;
