// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms
// This file contains structures, function prototypes, and definitions
// for the NetUser, NetUserModals, NetGroup, NetAccess, and NetLogon API.
use shared::basetsd::PDWORD_PTR;
use shared::lmcons::{ENCRYPTED_PWLEN, NET_API_STATUS, PARMNUM_BASE_INFOLEVEL, PWLEN};
use shared::minwindef::{BOOL, BYTE, DWORD, FILETIME, LPBYTE, LPDWORD, LPVOID, PBYTE, ULONG};
use um::winnt::{BOOLEAN, LONG, LPCWSTR, LPWSTR, PSID, PVOID, PZPWSTR, SID_NAME_USE};
extern "system" {
    pub fn NetUserAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserEnum(
        servername: LPCWSTR,
        level: DWORD,
        filter: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserGetInfo(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUserSetInfo(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserDel(
        servername: LPCWSTR,
        username: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetUserGetGroups(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserSetGroups(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        num_entries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserGetLocalGroups(
        servername: LPCWSTR,
        username: LPCWSTR,
        level: DWORD,
        flags: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserModalsGet(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetUserModalsSet(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetUserChangePassword(
        domainname: LPCWSTR,
        username: LPCWSTR,
        oldpassword: LPCWSTR,
        newpassword: LPCWSTR,
    ) -> NET_API_STATUS;
}
STRUCT!{struct USER_INFO_0 {
    usri0_name: LPWSTR,
}}
pub type PUSER_INFO_0 = *mut USER_INFO_0;
pub type LPUSER_INFO_0 = *mut USER_INFO_0;
STRUCT!{struct USER_INFO_1 {
    usri1_name: LPWSTR,
    usri1_password: LPWSTR,
    usri1_password_age: DWORD,
    usri1_priv: DWORD,
    usri1_home_dir: LPWSTR,
    usri1_comment: LPWSTR,
    usri1_flags: DWORD,
    usri1_script_path: LPWSTR,
}}
pub type PUSER_INFO_1 = *mut USER_INFO_1;
pub type LPUSER_INFO_1 = *mut USER_INFO_1;
STRUCT!{struct USER_INFO_2 {
    usri2_name: LPWSTR,
    usri2_password: LPWSTR,
    usri2_password_age: DWORD,
    usri2_priv: DWORD,
    usri2_home_dir: LPWSTR,
    usri2_comment: LPWSTR,
    usri2_flags: DWORD,
    usri2_script_path: LPWSTR,
    usri2_auth_flags: DWORD,
    usri2_full_name: LPWSTR,
    usri2_usr_comment: LPWSTR,
    usri2_parms: LPWSTR,
    usri2_workstations: LPWSTR,
    usri2_last_logon: DWORD,
    usri2_last_logoff: DWORD,
    usri2_acct_expires: DWORD,
    usri2_max_storage: DWORD,
    usri2_units_per_week: DWORD,
    usri2_logon_hours: PBYTE,
    usri2_bad_pw_count: DWORD,
    usri2_num_logons: DWORD,
    usri2_logon_server: LPWSTR,
    usri2_country_code: DWORD,
    usri2_code_page: DWORD,
}}
pub type PUSER_INFO_2 = *mut USER_INFO_2;
pub type LPUSER_INFO_2 = *mut USER_INFO_2;
STRUCT!{struct USER_INFO_3 {
    usri3_name: LPWSTR,
    usri3_password: LPWSTR,
    usri3_password_age: DWORD,
    usri3_priv: DWORD,
    usri3_home_dir: LPWSTR,
    usri3_comment: LPWSTR,
    usri3_flags: DWORD,
    usri3_script_path: LPWSTR,
    usri3_auth_flags: DWORD,
    usri3_full_name: LPWSTR,
    usri3_usr_comment: LPWSTR,
    usri3_parms: LPWSTR,
    usri3_workstations: LPWSTR,
    usri3_last_logon: DWORD,
    usri3_last_logoff: DWORD,
    usri3_acct_expires: DWORD,
    usri3_max_storage: DWORD,
    usri3_units_per_week: DWORD,
    usri3_logon_hours: PBYTE,
    usri3_bad_pw_count: DWORD,
    usri3_num_logons: DWORD,
    usri3_logon_server: LPWSTR,
    usri3_country_code: DWORD,
    usri3_code_page: DWORD,
    usri3_user_id: DWORD,
    usri3_primary_group_id: DWORD,
    usri3_profile: LPWSTR,
    usri3_home_dir_drive: LPWSTR,
    usri3_password_expired: DWORD,
}}
pub type PUSER_INFO_3 = *mut USER_INFO_3;
pub type LPUSER_INFO_3 = *mut USER_INFO_3;
STRUCT!{struct USER_INFO_4 {
    usri4_name: LPWSTR,
    usri4_password: LPWSTR,
    usri4_password_age: DWORD,
    usri4_priv: DWORD,
    usri4_home_dir: LPWSTR,
    usri4_comment: LPWSTR,
    usri4_flags: DWORD,
    usri4_script_path: LPWSTR,
    usri4_auth_flags: DWORD,
    usri4_full_name: LPWSTR,
    usri4_usr_comment: LPWSTR,
    usri4_parms: LPWSTR,
    usri4_workstations: LPWSTR,
    usri4_last_logon: DWORD,
    usri4_last_logoff: DWORD,
    usri4_acct_expires: DWORD,
    usri4_max_storage: DWORD,
    usri4_units_per_week: DWORD,
    usri4_logon_hours: PBYTE,
    usri4_bad_pw_count: DWORD,
    usri4_num_logons: DWORD,
    usri4_logon_server: LPWSTR,
    usri4_country_code: DWORD,
    usri4_code_page: DWORD,
    usri4_user_sid: PSID,
    usri4_primary_group_id: DWORD,
    usri4_profile: LPWSTR,
    usri4_home_dir_drive: LPWSTR,
    usri4_password_expired: DWORD,
}}
pub type PUSER_INFO_4 = *mut USER_INFO_4;
pub type LPUSER_INFO_4 = *mut USER_INFO_4;
STRUCT!{struct USER_INFO_10 {
    usri10_name: LPWSTR,
    usri10_comment: LPWSTR,
    usri10_usr_comment: LPWSTR,
    usri10_full_name: LPWSTR,
}}
pub type PUSER_INFO_10 = *mut USER_INFO_10;
pub type LPUSER_INFO_10 = *mut USER_INFO_10;
STRUCT!{struct USER_INFO_11 {
    usri11_name: LPWSTR,
    usri11_comment: LPWSTR,
    usri11_usr_comment: LPWSTR,
    usri11_full_name: LPWSTR,
    usri11_priv: DWORD,
    usri11_auth_flags: DWORD,
    usri11_password_age: DWORD,
    usri11_home_dir: LPWSTR,
    usri11_parms: LPWSTR,
    usri11_last_logon: DWORD,
    usri11_last_logoff: DWORD,
    usri11_bad_pw_count: DWORD,
    usri11_num_logons: DWORD,
    usri11_logon_server: LPWSTR,
    usri11_country_code: DWORD,
    usri11_workstations: LPWSTR,
    usri11_max_storage: DWORD,
    usri11_units_per_week: DWORD,
    usri11_logon_hours: PBYTE,
    usri11_code_page: DWORD,
}}
pub type PUSER_INFO_11 = *mut USER_INFO_11;
pub type LPUSER_INFO_11 = *mut USER_INFO_11;
STRUCT!{struct USER_INFO_20 {
    usri20_name: LPWSTR,
    usri20_full_name: LPWSTR,
    usri20_comment: LPWSTR,
    usri20_flags: DWORD,
    usri20_user_id: DWORD,
}}
pub type PUSER_INFO_20 = *mut USER_INFO_20;
pub type LPUSER_INFO_20 = *mut USER_INFO_20;
STRUCT!{struct USER_INFO_21 {
    usri21_password: [BYTE; ENCRYPTED_PWLEN],
}}
pub type PUSER_INFO_21 = *mut USER_INFO_21;
pub type LPUSER_INFO_21 = *mut USER_INFO_21;
STRUCT!{struct USER_INFO_22 {
    usri22_name: LPWSTR,
    usri22_password: [BYTE; ENCRYPTED_PWLEN],
    usri22_password_age: DWORD,
    usri22_priv: DWORD,
    usri22_home_dir: LPWSTR,
    usri22_comment: LPWSTR,
    usri22_flags: DWORD,
    usri22_script_path: LPWSTR,
    usri22_auth_flags: DWORD,
    usri22_full_name: LPWSTR,
    usri22_usr_comment: LPWSTR,
    usri22_parms: LPWSTR,
    usri22_workstations: LPWSTR,
    usri22_last_logon: DWORD,
    usri22_last_logoff: DWORD,
    usri22_acct_expires: DWORD,
    usri22_max_storage: DWORD,
    usri22_units_per_week: DWORD,
    usri22_logon_hours: PBYTE,
    usri22_bad_pw_count: DWORD,
    usri22_num_logons: DWORD,
    usri22_logon_server: LPWSTR,
    usri22_country_code: DWORD,
    usri22_code_page: DWORD,
}}
pub type PUSER_INFO_22 = *mut USER_INFO_22;
pub type LPUSER_INFO_22 = *mut USER_INFO_22;
STRUCT!{struct USER_INFO_23 {
    usri23_name: LPWSTR,
    usri23_full_name: LPWSTR,
    usri23_comment: LPWSTR,
    usri23_flags: DWORD,
    usri23_user_sid: PSID,
}}
pub type PUSER_INFO_23 = *mut USER_INFO_23;
pub type LPUSER_INFO_23 = *mut USER_INFO_23;
STRUCT!{struct USER_INFO_24 {
    usri24_internet_identity: BOOL,
    usri24_flags: DWORD,
    usri24_internet_provider_name: LPWSTR,
    usri24_internet_principal_name: LPWSTR,
    usri24_user_sid: PSID,
}}
pub type PUSER_INFO_24 = *mut USER_INFO_24;
pub type LPUSER_INFO_24 = *mut USER_INFO_24;
STRUCT!{struct USER_INFO_1003 {
    usri1003_password: LPWSTR,
}}
pub type PUSER_INFO_1003 = *mut USER_INFO_1003;
pub type LPUSER_INFO_1003 = *mut USER_INFO_1003;
STRUCT!{struct USER_INFO_1005 {
    usri1005_priv: DWORD,
}}
pub type PUSER_INFO_1005 = *mut USER_INFO_1005;
pub type LPUSER_INFO_1005 = *mut USER_INFO_1005;
STRUCT!{struct USER_INFO_1006 {
    usri1006_home_dir: LPWSTR,
}}
pub type PUSER_INFO_1006 = *mut USER_INFO_1006;
pub type LPUSER_INFO_1006 = *mut USER_INFO_1006;
STRUCT!{struct USER_INFO_1007 {
    usri1007_comment: LPWSTR,
}}
pub type PUSER_INFO_1007 = *mut USER_INFO_1007;
pub type LPUSER_INFO_1007 = *mut USER_INFO_1007;
STRUCT!{struct USER_INFO_1008 {
    usri1008_flags: DWORD,
}}
pub type PUSER_INFO_1008 = *mut USER_INFO_1008;
pub type LPUSER_INFO_1008 = *mut USER_INFO_1008;
STRUCT!{struct USER_INFO_1009 {
    usri1009_script_path: LPWSTR,
}}
pub type PUSER_INFO_1009 = *mut USER_INFO_1009;
pub type LPUSER_INFO_1009 = *mut USER_INFO_1009;
STRUCT!{struct USER_INFO_1010 {
    usri1010_auth_flags: DWORD,
}}
pub type PUSER_INFO_1010 = *mut USER_INFO_1010;
pub type LPUSER_INFO_1010 = *mut USER_INFO_1010;
STRUCT!{struct USER_INFO_1011 {
    usri1011_full_name: LPWSTR,
}}
pub type PUSER_INFO_1011 = *mut USER_INFO_1011;
pub type LPUSER_INFO_1011 = *mut USER_INFO_1011;
STRUCT!{struct USER_INFO_1012 {
    usri1012_usr_comment: LPWSTR,
}}
pub type PUSER_INFO_1012 = *mut USER_INFO_1012;
pub type LPUSER_INFO_1012 = *mut USER_INFO_1012;
STRUCT!{struct USER_INFO_1013 {
    usri1013_parms: LPWSTR,
}}
pub type PUSER_INFO_1013 = *mut USER_INFO_1013;
pub type LPUSER_INFO_1013 = *mut USER_INFO_1013;
STRUCT!{struct USER_INFO_1014 {
    usri1014_workstations: LPWSTR,
}}
pub type PUSER_INFO_1014 = *mut USER_INFO_1014;
pub type LPUSER_INFO_1014 = *mut USER_INFO_1014;
STRUCT!{struct USER_INFO_1017 {
    usri1017_acct_expires: DWORD,
}}
pub type PUSER_INFO_1017 = *mut USER_INFO_1017;
pub type LPUSER_INFO_1017 = *mut USER_INFO_1017;
STRUCT!{struct USER_INFO_1018 {
    usri1018_max_storage: DWORD,
}}
pub type PUSER_INFO_1018 = *mut USER_INFO_1018;
pub type LPUSER_INFO_1018 = *mut USER_INFO_1018;
STRUCT!{struct USER_INFO_1020 {
    usri1020_units_per_week: DWORD,
    usri1020_logon_hours: LPBYTE,
}}
pub type PUSER_INFO_1020 = *mut USER_INFO_1020;
pub type LPUSER_INFO_1020 = *mut USER_INFO_1020;
STRUCT!{struct USER_INFO_1023 {
    usri1023_logon_server: LPWSTR,
}}
pub type PUSER_INFO_1023 = *mut USER_INFO_1023;
pub type LPUSER_INFO_1023 = *mut USER_INFO_1023;
STRUCT!{struct USER_INFO_1024 {
    usri1024_country_code: DWORD,
}}
pub type PUSER_INFO_1024 = *mut USER_INFO_1024;
pub type LPUSER_INFO_1024 = *mut USER_INFO_1024;
STRUCT!{struct USER_INFO_1025 {
    usri1025_code_page: DWORD,
}}
pub type PUSER_INFO_1025 = *mut USER_INFO_1025;
pub type LPUSER_INFO_1025 = *mut USER_INFO_1025;
STRUCT!{struct USER_INFO_1051 {
    usri1051_primary_group_id: DWORD,
}}
pub type PUSER_INFO_1051 = *mut USER_INFO_1051;
pub type LPUSER_INFO_1051 = *mut USER_INFO_1051;
STRUCT!{struct USER_INFO_1052 {
    usri1052_profile: LPWSTR,
}}
pub type PUSER_INFO_1052 = *mut USER_INFO_1052;
pub type LPUSER_INFO_1052 = *mut USER_INFO_1052;
STRUCT!{struct USER_INFO_1053 {
    usri1053_home_dir_drive: LPWSTR,
}}
pub type PUSER_INFO_1053 = *mut USER_INFO_1053;
pub type LPUSER_INFO_1053 = *mut USER_INFO_1053;
STRUCT!{struct USER_MODALS_INFO_0 {
    usrmod0_min_passwd_len: DWORD,
    usrmod0_max_passwd_age: DWORD,
    usrmod0_min_passwd_age: DWORD,
    usrmod0_force_logoff: DWORD,
    usrmod0_password_hist_len: DWORD,
}}
pub type PUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
pub type LPUSER_MODALS_INFO_0 = *mut USER_MODALS_INFO_0;
STRUCT!{struct USER_MODALS_INFO_1 {
    usrmod1_role: DWORD,
    usrmod1_primary: LPWSTR,
}}
pub type PUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
pub type LPUSER_MODALS_INFO_1 = *mut USER_MODALS_INFO_1;
STRUCT!{struct USER_MODALS_INFO_2 {
    usrmod2_domain_name: LPWSTR,
    usrmod2_domain_id: PSID,
}}
pub type PUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
pub type LPUSER_MODALS_INFO_2 = *mut USER_MODALS_INFO_2;
STRUCT!{struct USER_MODALS_INFO_3 {
    usrmod3_lockout_duration: DWORD,
    usrmod3_lockout_observation_window: DWORD,
    usrmod3_lockout_threshold: DWORD,
}}
pub type PUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
pub type LPUSER_MODALS_INFO_3 = *mut USER_MODALS_INFO_3;
STRUCT!{struct USER_MODALS_INFO_1001 {
    usrmod1001_min_passwd_len: DWORD,
}}
pub type PUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
pub type LPUSER_MODALS_INFO_1001 = *mut USER_MODALS_INFO_1001;
STRUCT!{struct USER_MODALS_INFO_1002 {
    usrmod1002_max_passwd_age: DWORD,
}}
pub type PUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
pub type LPUSER_MODALS_INFO_1002 = *mut USER_MODALS_INFO_1002;
STRUCT!{struct USER_MODALS_INFO_1003 {
    usrmod1003_min_passwd_age: DWORD,
}}
pub type PUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
pub type LPUSER_MODALS_INFO_1003 = *mut USER_MODALS_INFO_1003;
STRUCT!{struct USER_MODALS_INFO_1004 {
    usrmod1004_force_logoff: DWORD,
}}
pub type PUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
pub type LPUSER_MODALS_INFO_1004 = *mut USER_MODALS_INFO_1004;
STRUCT!{struct USER_MODALS_INFO_1005 {
    usrmod1005_password_hist_len: DWORD,
}}
pub type PUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
pub type LPUSER_MODALS_INFO_1005 = *mut USER_MODALS_INFO_1005;
STRUCT!{struct USER_MODALS_INFO_1006 {
    usrmod1006_role: DWORD,
}}
pub type PUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
pub type LPUSER_MODALS_INFO_1006 = *mut USER_MODALS_INFO_1006;
STRUCT!{struct USER_MODALS_INFO_1007 {
    usrmod1007_primary: LPWSTR,
}}
pub type PUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
pub type LPUSER_MODALS_INFO_1007 = *mut USER_MODALS_INFO_1007;
pub const UF_SCRIPT: DWORD = 0x0001;
pub const UF_ACCOUNTDISABLE: DWORD = 0x0002;
pub const UF_HOMEDIR_REQUIRED: DWORD = 0x0008;
pub const UF_LOCKOUT: DWORD = 0x0010;
pub const UF_PASSWD_NOTREQD: DWORD = 0x0020;
pub const UF_PASSWD_CANT_CHANGE: DWORD = 0x0040;
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: DWORD = 0x0080;
pub const UF_TEMP_DUPLICATE_ACCOUNT: DWORD = 0x0100;
pub const UF_NORMAL_ACCOUNT: DWORD = 0x0200;
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: DWORD = 0x0800;
pub const UF_WORKSTATION_TRUST_ACCOUNT: DWORD = 0x1000;
pub const UF_SERVER_TRUST_ACCOUNT: DWORD = 0x2000;
pub const UF_MACHINE_ACCOUNT_MASK: DWORD = UF_INTERDOMAIN_TRUST_ACCOUNT
    | UF_WORKSTATION_TRUST_ACCOUNT | UF_SERVER_TRUST_ACCOUNT;
pub const UF_ACCOUNT_TYPE_MASK: DWORD = UF_TEMP_DUPLICATE_ACCOUNT | UF_NORMAL_ACCOUNT
    | UF_INTERDOMAIN_TRUST_ACCOUNT | UF_WORKSTATION_TRUST_ACCOUNT | UF_SERVER_TRUST_ACCOUNT;
pub const UF_DONT_EXPIRE_PASSWD: DWORD = 0x10000;
pub const UF_MNS_LOGON_ACCOUNT: DWORD = 0x20000;
pub const UF_SMARTCARD_REQUIRED: DWORD = 0x40000;
pub const UF_TRUSTED_FOR_DELEGATION: DWORD = 0x80000;
pub const UF_NOT_DELEGATED: DWORD = 0x100000;
pub const UF_USE_DES_KEY_ONLY: DWORD = 0x200000;
pub const UF_DONT_REQUIRE_PREAUTH: DWORD = 0x400000;
pub const UF_PASSWORD_EXPIRED: DWORD = 0x800000;
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: DWORD = 0x1000000;
pub const UF_NO_AUTH_DATA_REQUIRED: DWORD = 0x2000000;
pub const UF_PARTIAL_SECRETS_ACCOUNT: DWORD = 0x4000000;
pub const UF_USE_AES_KEYS: DWORD = 0x8000000;
pub const UF_SETTABLE_BITS: DWORD = UF_SCRIPT | UF_ACCOUNTDISABLE | UF_LOCKOUT
    | UF_HOMEDIR_REQUIRED | UF_PASSWD_NOTREQD | UF_PASSWD_CANT_CHANGE | UF_ACCOUNT_TYPE_MASK
    | UF_DONT_EXPIRE_PASSWD | UF_MNS_LOGON_ACCOUNT | UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED
    | UF_SMARTCARD_REQUIRED | UF_TRUSTED_FOR_DELEGATION | UF_NOT_DELEGATED | UF_USE_DES_KEY_ONLY
    | UF_DONT_REQUIRE_PREAUTH | UF_PASSWORD_EXPIRED | UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION
    | UF_NO_AUTH_DATA_REQUIRED | UF_USE_AES_KEYS | UF_PARTIAL_SECRETS_ACCOUNT;
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: DWORD = 0x0001;
pub const FILTER_NORMAL_ACCOUNT: DWORD = 0x0002;
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: DWORD = 0x0008;
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: DWORD = 0x0010;
pub const FILTER_SERVER_TRUST_ACCOUNT: DWORD = 0x0020;
pub const LG_INCLUDE_INDIRECT: DWORD = 0x0001;
pub const AF_OP_PRINT: DWORD = 0x1;
pub const AF_OP_COMM: DWORD = 0x2;
pub const AF_OP_SERVER: DWORD = 0x4;
pub const AF_OP_ACCOUNTS: DWORD = 0x8;
pub const AF_SETTABLE_BITS: DWORD = AF_OP_PRINT | AF_OP_COMM | AF_OP_SERVER | AF_OP_ACCOUNTS;
pub const UAS_ROLE_STANDALONE: DWORD = 0;
pub const UAS_ROLE_MEMBER: DWORD = 1;
pub const UAS_ROLE_BACKUP: DWORD = 2;
pub const UAS_ROLE_PRIMARY: DWORD = 3;
pub const USER_NAME_PARMNUM: DWORD = 1;
pub const USER_PASSWORD_PARMNUM: DWORD = 3;
pub const USER_PASSWORD_AGE_PARMNUM: DWORD = 4;
pub const USER_PRIV_PARMNUM: DWORD = 5;
pub const USER_HOME_DIR_PARMNUM: DWORD = 6;
pub const USER_COMMENT_PARMNUM: DWORD = 7;
pub const USER_FLAGS_PARMNUM: DWORD = 8;
pub const USER_SCRIPT_PATH_PARMNUM: DWORD = 9;
pub const USER_AUTH_FLAGS_PARMNUM: DWORD = 10;
pub const USER_FULL_NAME_PARMNUM: DWORD = 11;
pub const USER_USR_COMMENT_PARMNUM: DWORD = 12;
pub const USER_PARMS_PARMNUM: DWORD = 13;
pub const USER_WORKSTATIONS_PARMNUM: DWORD = 14;
pub const USER_LAST_LOGON_PARMNUM: DWORD = 15;
pub const USER_LAST_LOGOFF_PARMNUM: DWORD = 16;
pub const USER_ACCT_EXPIRES_PARMNUM: DWORD = 17;
pub const USER_MAX_STORAGE_PARMNUM: DWORD = 18;
pub const USER_UNITS_PER_WEEK_PARMNUM: DWORD = 19;
pub const USER_LOGON_HOURS_PARMNUM: DWORD = 20;
pub const USER_PAD_PW_COUNT_PARMNUM: DWORD = 21;
pub const USER_NUM_LOGONS_PARMNUM: DWORD = 22;
pub const USER_LOGON_SERVER_PARMNUM: DWORD = 23;
pub const USER_COUNTRY_CODE_PARMNUM: DWORD = 24;
pub const USER_CODE_PAGE_PARMNUM: DWORD = 25;
pub const USER_PRIMARY_GROUP_PARMNUM: DWORD = 51;
pub const USER_PROFILE: DWORD = 52;
pub const USER_PROFILE_PARMNUM: DWORD = 52;
pub const USER_HOME_DIR_DRIVE_PARMNUM: DWORD = 53;
pub const USER_NAME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_NAME_PARMNUM;
pub const USER_PASSWORD_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_PASSWORD_PARMNUM;
pub const USER_PASSWORD_AGE_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_PASSWORD_AGE_PARMNUM;
pub const USER_PRIV_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_PRIV_PARMNUM;
pub const USER_HOME_DIR_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_HOME_DIR_PARMNUM;
pub const USER_COMMENT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_COMMENT_PARMNUM;
pub const USER_FLAGS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_FLAGS_PARMNUM;
pub const USER_SCRIPT_PATH_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_SCRIPT_PATH_PARMNUM;
pub const USER_AUTH_FLAGS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_AUTH_FLAGS_PARMNUM;
pub const USER_FULL_NAME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_FULL_NAME_PARMNUM;
pub const USER_USR_COMMENT_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_USR_COMMENT_PARMNUM;
pub const USER_PARMS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_PARMS_PARMNUM;
pub const USER_WORKSTATIONS_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_WORKSTATIONS_PARMNUM;
pub const USER_LAST_LOGON_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_LAST_LOGON_PARMNUM;
pub const USER_LAST_LOGOFF_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_LAST_LOGOFF_PARMNUM;
pub const USER_ACCT_EXPIRES_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_ACCT_EXPIRES_PARMNUM;
pub const USER_MAX_STORAGE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_MAX_STORAGE_PARMNUM;
pub const USER_UNITS_PER_WEEK_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_UNITS_PER_WEEK_PARMNUM;
pub const USER_LOGON_HOURS_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_LOGON_HOURS_PARMNUM;
pub const USER_PAD_PW_COUNT_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_PAD_PW_COUNT_PARMNUM;
pub const USER_NUM_LOGONS_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_NUM_LOGONS_PARMNUM;
pub const USER_LOGON_SERVER_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_LOGON_SERVER_PARMNUM;
pub const USER_COUNTRY_CODE_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_COUNTRY_CODE_PARMNUM;
pub const USER_CODE_PAGE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + USER_CODE_PAGE_PARMNUM;
pub const USER_PRIMARY_GROUP_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_PRIMARY_GROUP_PARMNUM;
pub const USER_HOME_DIR_DRIVE_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + USER_HOME_DIR_DRIVE_PARMNUM;
pub const NULL_USERSETINFO_PASSWD: &'static str = "              ";
pub const TIMEQ_FOREVER: DWORD = -1i32 as u32;
pub const USER_MAXSTORAGE_UNLIMITED: DWORD = -1i32 as u32;
pub const USER_NO_LOGOFF: DWORD = -1i32 as u32;
pub const UNITS_PER_DAY: DWORD = 24;
pub const UNITS_PER_WEEK: DWORD = UNITS_PER_DAY * 7;
pub const USER_PRIV_MASK: DWORD = 0x3;
pub const USER_PRIV_GUEST: DWORD = 0;
pub const USER_PRIV_USER: DWORD = 1;
pub const USER_PRIV_ADMIN: DWORD = 2;
pub const MAX_PASSWD_LEN: DWORD = PWLEN;
pub const DEF_MIN_PWLEN: DWORD = 6;
pub const DEF_PWUNIQUENESS: DWORD = 5;
pub const DEF_MAX_PWHIST: DWORD = 8;
pub const DEF_MAX_PWAGE: DWORD = TIMEQ_FOREVER;
pub const DEF_MIN_PWAGE: DWORD = 0;
pub const DEF_FORCE_LOGOFF: DWORD = 0xffffffff;
pub const DEF_MAX_BADPW: DWORD = 0;
pub const ONE_DAY: DWORD = 1 * 24 * 3600;
pub const VALIDATED_LOGON: DWORD = 0;
pub const PASSWORD_EXPIRED: DWORD = 2;
pub const NON_VALIDATED_LOGON: DWORD = 3;
pub const VALID_LOGOFF: DWORD = 1;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: DWORD = 1;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: DWORD = 2;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: DWORD = 3;
pub const MODALS_FORCE_LOGOFF_PARMNUM: DWORD = 4;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: DWORD = 5;
pub const MODALS_ROLE_PARMNUM: DWORD = 6;
pub const MODALS_PRIMARY_PARMNUM: DWORD = 7;
pub const MODALS_DOMAIN_NAME_PARMNUM: DWORD = 8;
pub const MODALS_DOMAIN_ID_PARMNUM: DWORD = 9;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: DWORD = 10;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: DWORD = 11;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: DWORD = 12;
pub const MODALS_MIN_PASSWD_LEN_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_MIN_PASSWD_LEN_PARMNUM;
pub const MODALS_MAX_PASSWD_AGE_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_MAX_PASSWD_AGE_PARMNUM;
pub const MODALS_MIN_PASSWD_AGE_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_MIN_PASSWD_AGE_PARMNUM;
pub const MODALS_FORCE_LOGOFF_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_FORCE_LOGOFF_PARMNUM;
pub const MODALS_PASSWD_HIST_LEN_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_PASSWD_HIST_LEN_PARMNUM;
pub const MODALS_ROLE_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + MODALS_ROLE_PARMNUM;
pub const MODALS_PRIMARY_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + MODALS_PRIMARY_PARMNUM;
pub const MODALS_DOMAIN_NAME_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + MODALS_DOMAIN_NAME_PARMNUM;
pub const MODALS_DOMAIN_ID_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + MODALS_DOMAIN_ID_PARMNUM;
extern "system" {
    pub fn NetGroupAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetGroupAddUser(
        servername: LPCWSTR,
        GroupName: LPCWSTR,
        username: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resume_handle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupGetInfo(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGroupSetInfo(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetGroupDel(
        servername: LPCWSTR,
        groupname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupDelUser(
        servername: LPCWSTR,
        GroupName: LPCWSTR,
        Username: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupGetUsers(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        ResumeHandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetGroupSetUsers(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        totalentries: DWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct GROUP_INFO_0 {
    grpi0_name: LPWSTR,
}}
pub type PGROUP_INFO_0 = *mut GROUP_INFO_0;
pub type LPGROUP_INFO_0 = *mut GROUP_INFO_0;
STRUCT!{struct GROUP_INFO_1 {
    grpi1_name: LPWSTR,
    grpi1_comment: LPWSTR,
}}
pub type PGROUP_INFO_1 = *mut GROUP_INFO_1;
pub type LPGROUP_INFO_1 = *mut GROUP_INFO_1;
STRUCT!{struct GROUP_INFO_2 {
    grpi2_name: LPWSTR,
    grpi2_comment: LPWSTR,
    grpi2_group_id: DWORD,
    grpi2_attributes: DWORD,
}}
pub type PGROUP_INFO_2 = *mut GROUP_INFO_2;
STRUCT!{struct GROUP_INFO_3 {
    grpi3_name: LPWSTR,
    grpi3_comment: LPWSTR,
    grpi3_group_sid: PSID,
    grpi3_attributes: DWORD,
}}
pub type PGROUP_INFO_3 = *mut GROUP_INFO_3;
STRUCT!{struct GROUP_INFO_1002 {
    grpi1002_comment: LPWSTR,
}}
pub type PGROUP_INFO_1002 = *mut GROUP_INFO_1002;
pub type LPGROUP_INFO_1002 = *mut GROUP_INFO_1002;
STRUCT!{struct GROUP_INFO_1005 {
    grpi1005_attributes: DWORD,
}}
pub type PGROUP_INFO_1005 = *mut GROUP_INFO_1005;
pub type LPGROUP_INFO_1005 = *mut GROUP_INFO_1005;
STRUCT!{struct GROUP_USERS_INFO_0 {
    grui0_name: LPWSTR,
}}
pub type PGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
pub type LPGROUP_USERS_INFO_0 = *mut GROUP_USERS_INFO_0;
STRUCT!{struct GROUP_USERS_INFO_1 {
    grui1_name: LPWSTR,
    grui1_attributes: DWORD,
}}
pub type PGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub type LPGROUP_USERS_INFO_1 = *mut GROUP_USERS_INFO_1;
pub const GROUPIDMASK: DWORD = 0x8000;
pub const GROUP_SPECIALGRP_USERS: &'static str = "USERS";
pub const GROUP_SPECIALGRP_ADMINS: &'static str = "ADMINS";
pub const GROUP_SPECIALGRP_GUESTS: &'static str = "GUESTS";
pub const GROUP_SPECIALGRP_LOCAL: &'static str = "LOCAL";
pub const GROUP_ALL_PARMNUM: DWORD = 0;
pub const GROUP_NAME_PARMNUM: DWORD = 1;
pub const GROUP_COMMENT_PARMNUM: DWORD = 2;
pub const GROUP_ATTRIBUTES_PARMNUM: DWORD = 3;
pub const GROUP_ALL_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + GROUP_ALL_PARMNUM;
pub const GROUP_NAME_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + GROUP_NAME_PARMNUM;
pub const GROUP_COMMENT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + GROUP_COMMENT_PARMNUM;
pub const GROUP_ATTRIBUTES_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + GROUP_ATTRIBUTES_PARMNUM;
extern "system" {
    pub fn NetLocalGroupAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupAddMember(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        membersid: PSID,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupEnum(
        servername: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupGetInfo(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupSetInfo(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupDel(
        servername: LPCWSTR,
        groupname: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupDelMember(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        membersid: PSID,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupGetMembers(
        servername: LPCWSTR,
        localgroupname: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resumehandle: PDWORD_PTR,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupSetMembers(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        totalentries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupAddMembers(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        totalentries: DWORD,
    ) -> NET_API_STATUS;
    pub fn NetLocalGroupDelMembers(
        servername: LPCWSTR,
        groupname: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        totalentries: DWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct LOCALGROUP_INFO_0 {
    lgrpi0_name: LPWSTR,
}}
pub type PLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
pub type LPLOCALGROUP_INFO_0 = *mut LOCALGROUP_INFO_0;
STRUCT!{struct LOCALGROUP_INFO_1 {
    lgrpi1_name: LPWSTR,
    lgrpi1_comment: LPWSTR,
}}
pub type PLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
pub type LPLOCALGROUP_INFO_1 = *mut LOCALGROUP_INFO_1;
STRUCT!{struct LOCALGROUP_INFO_1002 {
    lgrpi1002_comment: LPWSTR,
}}
pub type PLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
pub type LPLOCALGROUP_INFO_1002 = *mut LOCALGROUP_INFO_1002;
STRUCT!{struct LOCALGROUP_MEMBERS_INFO_0 {
    lgrmi0_sid: PSID,
}}
pub type PLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
pub type LPLOCALGROUP_MEMBERS_INFO_0 = *mut LOCALGROUP_MEMBERS_INFO_0;
STRUCT!{struct LOCALGROUP_MEMBERS_INFO_1 {
    lgrmi1_sid: PSID,
    lgrmi1_sidusage: SID_NAME_USE,
    lgrmi1_name: LPWSTR,
}}
pub type PLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
pub type LPLOCALGROUP_MEMBERS_INFO_1 = *mut LOCALGROUP_MEMBERS_INFO_1;
STRUCT!{struct LOCALGROUP_MEMBERS_INFO_2 {
    lgrmi2_sid: PSID,
    lgrmi2_sidusage: SID_NAME_USE,
    lgrmi2_domainandname: LPWSTR,
}}
pub type PLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
pub type LPLOCALGROUP_MEMBERS_INFO_2 = *mut LOCALGROUP_MEMBERS_INFO_2;
STRUCT!{struct LOCALGROUP_MEMBERS_INFO_3 {
    lgrmi3_domainandname: LPWSTR,
}}
pub type PLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
pub type LPLOCALGROUP_MEMBERS_INFO_3 = *mut LOCALGROUP_MEMBERS_INFO_3;
STRUCT!{struct LOCALGROUP_USERS_INFO_0 {
    lgrui0_name: LPWSTR,
}}
pub type PLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub type LPLOCALGROUP_USERS_INFO_0 = *mut LOCALGROUP_USERS_INFO_0;
pub const LOCALGROUP_NAME_PARMNUM: DWORD = 1;
pub const LOCALGROUP_COMMENT_PARMNUM: DWORD = 2;
extern "system" {
    pub fn NetQueryDisplayInformation(
        ServerName: LPCWSTR,
        Level: DWORD,
        Index: DWORD,
        EntriesRequested: DWORD,
        PreferredMaximumLength: DWORD,
        ReturnedEntryCount: LPDWORD,
        SortedBuffer: *mut PVOID,
    ) -> NET_API_STATUS;
    pub fn NetGetDisplayInformationIndex(
        ServerName: LPCWSTR,
        Level: DWORD,
        Prefix: LPCWSTR,
        Index: LPDWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct NET_DISPLAY_USER {
    usri1_name: LPWSTR,
    usri1_comment: LPWSTR,
    usri1_flags: DWORD,
    usri1_full_name: LPWSTR,
    usri1_user_id: DWORD,
    usri1_next_index: DWORD,
}}
pub type PNET_DISPLAY_USER = *mut NET_DISPLAY_USER;
STRUCT!{struct NET_DISPLAY_MACHINE {
    usri2_name: LPWSTR,
    usri2_comment: LPWSTR,
    usri2_flags: DWORD,
    usri2_user_id: DWORD,
    usri2_next_index: DWORD,
}}
pub type PNET_DISPLAY_MACHINE = *mut NET_DISPLAY_MACHINE;
STRUCT!{struct NET_DISPLAY_GROUP {
    usri3_name: LPWSTR,
    usri3_comment: LPWSTR,
    grpi3_group_id: DWORD,
    grpi3_attributes: DWORD,
    grpi3_next_index: DWORD,
}}
pub type PNET_DISPLAY_GROUP = *mut NET_DISPLAY_GROUP;
extern "system" {
    pub fn NetAccessAdd(
        servername: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessEnum(
        servername: LPCWSTR,
        BasePath: LPCWSTR,
        Recursive: DWORD,
        level: DWORD,
        bufptr: *mut LPBYTE,
        prefmaxlen: DWORD,
        entriesread: LPDWORD,
        totalentries: LPDWORD,
        resume_handle: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessGetInfo(
        servername: LPCWSTR,
        resource: LPCWSTR,
        level: DWORD,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetAccessSetInfo(
        servername: LPCWSTR,
        resource: LPCWSTR,
        level: DWORD,
        buf: LPBYTE,
        parm_err: LPDWORD,
    ) -> NET_API_STATUS;
    pub fn NetAccessDel(
        servername: LPCWSTR,
        resource: LPCWSTR,
    ) -> NET_API_STATUS;
    pub fn NetAccessGetUserPerms(
        servername: LPCWSTR,
        UGname: LPCWSTR,
        resource: LPCWSTR,
        Perms: LPDWORD,
    ) -> NET_API_STATUS;
}
STRUCT!{struct ACCESS_INFO_0 {
    acc0_resource_name: LPWSTR,
}}
pub type PACCESS_INFO_0 = *mut ACCESS_INFO_0;
pub type LPACCESS_INFO_0 = *mut ACCESS_INFO_0;
STRUCT!{struct ACCESS_INFO_1 {
    acc1_resource_name: LPWSTR,
    acc1_attr: DWORD,
    acc1_count: DWORD,
}}
pub type PACCESS_INFO_1 = *mut ACCESS_INFO_1;
pub type LPACCESS_INFO_1 = *mut ACCESS_INFO_1;
STRUCT!{struct ACCESS_INFO_1002 {
    acc1002_attr: DWORD,
}}
pub type PACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
pub type LPACCESS_INFO_1002 = *mut ACCESS_INFO_1002;
STRUCT!{struct ACCESS_LIST {
    acl_ugname: LPWSTR,
    acl_access: DWORD,
}}
pub type PACCESS_LIST = *mut ACCESS_LIST;
pub type LPACCESS_LIST = *mut ACCESS_LIST;
pub const MAXPERMENTRIES: DWORD = 64;
pub const ACCESS_NONE: DWORD = 0;
pub const ACCESS_ALL: DWORD = ACCESS_READ | ACCESS_WRITE | ACCESS_CREATE | ACCESS_EXEC
    | ACCESS_DELETE | ACCESS_ATRIB | ACCESS_PERM;
pub const ACCESS_READ: DWORD = 0x01;
pub const ACCESS_WRITE: DWORD = 0x02;
pub const ACCESS_CREATE: DWORD = 0x04;
pub const ACCESS_EXEC: DWORD = 0x08;
pub const ACCESS_DELETE: DWORD = 0x10;
pub const ACCESS_ATRIB: DWORD = 0x20;
pub const ACCESS_PERM: DWORD = 0x40;
pub const ACCESS_GROUP: DWORD = 0x8000;
pub const ACCESS_AUDIT: DWORD = 0x1;
pub const ACCESS_SUCCESS_OPEN: DWORD = 0x10;
pub const ACCESS_SUCCESS_WRITE: DWORD = 0x20;
pub const ACCESS_SUCCESS_DELETE: DWORD = 0x40;
pub const ACCESS_SUCCESS_ACL: DWORD = 0x80;
pub const ACCESS_SUCCESS_MASK: DWORD = 0xF0;
pub const ACCESS_FAIL_OPEN: DWORD = 0x100;
pub const ACCESS_FAIL_WRITE: DWORD = 0x200;
pub const ACCESS_FAIL_DELETE: DWORD = 0x400;
pub const ACCESS_FAIL_ACL: DWORD = 0x800;
pub const ACCESS_FAIL_MASK: DWORD = 0xF00;
pub const ACCESS_FAIL_SHIFT: DWORD = 4;
pub const ACCESS_RESOURCE_NAME_PARMNUM: DWORD = 1;
pub const ACCESS_ATTR_PARMNUM: DWORD = 2;
pub const ACCESS_COUNT_PARMNUM: DWORD = 3;
pub const ACCESS_ACCESS_LIST_PARMNUM: DWORD = 4;
pub const ACCESS_RESOURCE_NAME_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + ACCESS_RESOURCE_NAME_PARMNUM;
pub const ACCESS_ATTR_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + ACCESS_ATTR_PARMNUM;
pub const ACCESS_COUNT_INFOLEVEL: DWORD = PARMNUM_BASE_INFOLEVEL + ACCESS_COUNT_PARMNUM;
pub const ACCESS_ACCESS_LIST_INFOLEVEL: DWORD =
    PARMNUM_BASE_INFOLEVEL + ACCESS_ACCESS_LIST_PARMNUM;
pub const ACCESS_LETTERS: &'static str = "RWCXDAP         ";
ENUM!{enum NET_VALIDATE_PASSWORD_TYPE {
    NetValidateAuthentication = 1,
    NetValidatePasswordChange,
    NetValidatePasswordReset,
}}
pub type PNET_VALIDATE_PASSWORD_TYPE = *mut NET_VALIDATE_PASSWORD_TYPE;
STRUCT!{struct NET_VALIDATE_PASSWORD_HASH {
    Length: ULONG,
    Hash: LPBYTE,
}}
pub type PNET_VALIDATE_PASSWORD_HASH = *mut NET_VALIDATE_PASSWORD_HASH;
pub const NET_VALIDATE_PASSWORD_LAST_SET: ULONG = 0x00000001;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: ULONG = 0x00000002;
pub const NET_VALIDATE_LOCKOUT_TIME: ULONG = 0x00000004;
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: ULONG = 0x00000008;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: ULONG = 0x00000010;
pub const NET_VALIDATE_PASSWORD_HISTORY: ULONG = 0x00000020;
STRUCT!{struct NET_VALIDATE_PERSISTED_FIELDS {
    PresentFields: ULONG,
    PasswordLastSet: FILETIME,
    BadPasswordTime: FILETIME,
    LockoutTime: FILETIME,
    BadPasswordCount: ULONG,
    PasswordHistoryLength: ULONG,
    PasswordHistory: PNET_VALIDATE_PASSWORD_HASH,
}}
pub type PNET_VALIDATE_PERSISTED_FIELDS = *mut NET_VALIDATE_PERSISTED_FIELDS;
STRUCT!{struct NET_VALIDATE_OUTPUT_ARG {
    ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    ValidationStatus: NET_API_STATUS,
}}
pub type PNET_VALIDATE_OUTPUT_ARG = *mut NET_VALIDATE_OUTPUT_ARG;
STRUCT!{struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    PasswordMatched: BOOLEAN,
}}
pub type PNET_VALIDATE_AUTHENTICATION_INPUT_ARG = *mut NET_VALIDATE_AUTHENTICATION_INPUT_ARG;
STRUCT!{struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    ClearPassword: LPWSTR,
    UserAccountName: LPWSTR,
    HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    PasswordMatch: BOOLEAN,
}}
pub type PNET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG;
STRUCT!{struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    ClearPassword: LPWSTR,
    UserAccountName: LPWSTR,
    HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    PasswordMustChangeAtNextLogon: BOOLEAN,
    ClearLockout: BOOLEAN,
}}
pub type PNET_VALIDATE_PASSWORD_RESET_INPUT_ARG = *mut NET_VALIDATE_PASSWORD_RESET_INPUT_ARG;
extern "system" {
    pub fn NetValidatePasswordPolicy(
        ServerName: LPCWSTR,
        Qualifier: LPVOID,
        ValidationType: NET_VALIDATE_PASSWORD_TYPE,
        InputArg: LPVOID,
        OutputArg: *mut LPVOID,
    ) -> NET_API_STATUS;
    pub fn NetValidatePasswordPolicyFree(
        OutputArg: *mut LPVOID,
    ) -> NET_API_STATUS;
    pub fn NetGetDCName(
        servername: LPCWSTR,
        domainname: LPCWSTR,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn NetGetAnyDCName(
        servername: LPCWSTR,
        domainname: LPCWSTR,
        bufptr: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn I_NetLogonControl(
        ServerName: LPCWSTR,
        FunctionCode: DWORD,
        QueryLevel: DWORD,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
    pub fn I_NetLogonControl2(
        ServerName: LPCWSTR,
        FunctionCode: DWORD,
        QueryLevel: DWORD,
        Data: LPBYTE,
        Buffer: *mut LPBYTE,
    ) -> NET_API_STATUS;
}
pub type NTSTATUS = LONG;
pub type PNTSTATUS = *mut LONG;
extern "system" {
    pub fn NetEnumerateTrustedDomains(
        ServerName: LPWSTR,
        DomainNames: *mut LPWSTR,
    ) -> NTSTATUS;
}
pub const NETLOGON_CONTROL_QUERY: DWORD = 1;
pub const NETLOGON_CONTROL_REPLICATE: DWORD = 2;
pub const NETLOGON_CONTROL_SYNCHRONIZE: DWORD = 3;
pub const NETLOGON_CONTROL_PDC_REPLICATE: DWORD = 4;
pub const NETLOGON_CONTROL_REDISCOVER: DWORD = 5;
pub const NETLOGON_CONTROL_TC_QUERY: DWORD = 6;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: DWORD = 7;
pub const NETLOGON_CONTROL_FIND_USER: DWORD = 8;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: DWORD = 9;
pub const NETLOGON_CONTROL_TC_VERIFY: DWORD = 10;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: DWORD = 11;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: DWORD = 12;
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: DWORD = 13;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: DWORD = 0xFFFB;
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: DWORD = 0xFFFC;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: DWORD = 0xFFFD;
pub const NETLOGON_CONTROL_SET_DBFLAG: DWORD = 0xFFFE;
pub const NETLOGON_CONTROL_BREAKPOINT: DWORD = 0xFFFF;
STRUCT!{struct NETLOGON_INFO_1 {
    netlog1_flags: DWORD,
    netlog1_pdc_connection_status: NET_API_STATUS,
}}
pub type PNETLOGON_INFO_1 = *mut NETLOGON_INFO_1;
STRUCT!{struct NETLOGON_INFO_2 {
    netlog2_flags: DWORD,
    netlog2_pdc_connection_status: NET_API_STATUS,
    netlog2_trusted_dc_name: LPWSTR,
    netlog2_tc_connection_status: NET_API_STATUS,
}}
pub type PNETLOGON_INFO_2 = *mut NETLOGON_INFO_2;
STRUCT!{struct NETLOGON_INFO_3 {
    netlog3_flags: DWORD,
    netlog3_logon_attempts: DWORD,
    netlog3_reserved1: DWORD,
    netlog3_reserved2: DWORD,
    netlog3_reserved3: DWORD,
    netlog3_reserved4: DWORD,
    netlog3_reserved5: DWORD,
}}
pub type PNETLOGON_INFO_3 = *mut NETLOGON_INFO_3;
STRUCT!{struct NETLOGON_INFO_4 {
    netlog4_trusted_dc_name: LPWSTR,
    netlog4_trusted_domain_name: LPWSTR,
}}
pub type PNETLOGON_INFO_4 = *mut NETLOGON_INFO_4;
pub const NETLOGON_REPLICATION_NEEDED: DWORD = 0x01;
pub const NETLOGON_REPLICATION_IN_PROGRESS: DWORD = 0x02;
pub const NETLOGON_FULL_SYNC_REPLICATION: DWORD = 0x04;
pub const NETLOGON_REDO_NEEDED: DWORD = 0x08;
pub const NETLOGON_HAS_IP: DWORD = 0x10;
pub const NETLOGON_HAS_TIMESERV: DWORD = 0x20;
pub const NETLOGON_DNS_UPDATE_FAILURE: DWORD = 0x40;
pub const NETLOGON_VERIFY_STATUS_RETURNED: DWORD = 0x80;
pub const SERVICE_ACCOUNT_PASSWORD: &'static str = "_SA_{262E99C9-6160-4871-ACEC-4E61736B6F21}";
pub const SERVICE_ACCOUNT_SECRET_PREFIX: &'static str
    = "_SC_{262E99C9-6160-4871-ACEC-4E61736B6F21}_";
DEFINE_GUID!{ServiceAccountPasswordGUID,
    0x262E99C9, 0x6160, 0x4871, 0xAC, 0xEC, 0x4E, 0x61, 0x73, 0x6B, 0x6F, 0x21}
extern "system" {
    pub fn NetAddServiceAccount(
        ServerName: LPWSTR,
        AccountName: LPWSTR,
        Password: LPWSTR,
        Flags: DWORD,
    ) -> NTSTATUS;
}
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: DWORD = 0x00000001;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: DWORD = 0x00000002;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: DWORD = 0x00000001;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: DWORD = 0x00000002;
extern "system" {
    pub fn NetRemoveServiceAccount(
        ServerName: LPWSTR,
        AccountName: LPWSTR,
        Flags: DWORD,
    ) -> NTSTATUS;
    pub fn NetEnumerateServiceAccounts(
        ServerName: LPWSTR,
        Flags: DWORD,
        AccountsCount: *mut DWORD,
        Accounts: *mut PZPWSTR,
    ) -> NTSTATUS;
    pub fn NetIsServiceAccount(
        ServerName: LPWSTR,
        AccountName: LPWSTR,
        IsService: *mut BOOL,
    ) -> NTSTATUS;
    pub fn NetQueryServiceAccount(
        ServerName: LPWSTR,
        AccountName: LPWSTR,
        InfoLevel: DWORD,
        Buffer: *mut PBYTE,
    ) -> NTSTATUS;
}
ENUM!{enum MSA_INFO_LEVEL {
    MsaInfoLevel0 = 0,
    MsaInfoLevelMax,
}}
pub type PMSA_INFO_LEVEL = *mut MSA_INFO_LEVEL;
ENUM!{enum MSA_INFO_STATE {
    MsaInfoNotExist = 1,
    MsaInfoNotService,
    MsaInfoCannotInstall,
    MsaInfoCanInstall,
    MsaInfoInstalled,
}}
pub type PMSA_INFO_STATE = *mut MSA_INFO_STATE;
STRUCT!{struct MSA_INFO_0 {
    State: MSA_INFO_STATE,
}}
pub type PMSA_INFO_0 = *mut MSA_INFO_0;
pub type LPMSA_INFO_0 = *mut MSA_INFO_0;
