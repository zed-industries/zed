// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::basetsd::DWORD_PTR;
use shared::minwindef::{
    BOOL, BYTE, DWORD, HKEY, LPBYTE, LPCVOID, LPDWORD, PFILETIME, PHKEY, ULONG
};
use um::minwinbase::LPSECURITY_ATTRIBUTES;
use um::reason::{
    SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_LEGACY_API, SHTDN_REASON_MAJOR_HARDWARE,
    SHTDN_REASON_MAJOR_OTHER, SHTDN_REASON_MAJOR_SOFTWARE, SHTDN_REASON_MAJOR_SYSTEM,
    SHTDN_REASON_MINOR_HUNG, SHTDN_REASON_MINOR_INSTALLATION, SHTDN_REASON_MINOR_OTHER,
    SHTDN_REASON_MINOR_RECONFIG, SHTDN_REASON_MINOR_UNSTABLE, SHTDN_REASON_UNKNOWN,
};
use um::winnt::{
    ACCESS_MASK, BOOLEAN, HANDLE, LONG, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PBOOLEAN, PLONG,
    PSECURITY_DESCRIPTOR, PVOID, SECURITY_INFORMATION,
};
pub type LSTATUS = LONG;
pub const RRF_RT_REG_NONE: DWORD = 0x00000001;
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;
pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_DWORD;
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_QWORD;
pub const RRF_RT_ANY: DWORD = 0x0000ffff;
pub const RRF_SUBKEY_WOW6464KEY: DWORD = 0x00010000;
pub const RRF_SUBKEY_WOW6432KEY: DWORD = 0x00020000;
pub const RRF_WOW64_MASK: DWORD = 0x00030000;
pub const RRF_NOEXPAND: DWORD = 0x10000000;
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;
pub const REG_PROCESS_APPKEY: DWORD = 0x00000001;
pub type REGSAM = ACCESS_MASK;
pub const HKEY_CLASSES_ROOT: HKEY = 0x80000000i32 as usize as HKEY;
pub const HKEY_CURRENT_USER: HKEY = 0x80000001i32 as usize as HKEY;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002i32 as usize as HKEY;
pub const HKEY_USERS: HKEY = 0x80000003i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_DATA: HKEY = 0x80000004i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_TEXT: HKEY = 0x80000050i32 as usize as HKEY;
pub const HKEY_PERFORMANCE_NLSTEXT: HKEY = 0x80000060i32 as usize as HKEY;
pub const HKEY_CURRENT_CONFIG: HKEY = 0x80000005i32 as usize as HKEY;
pub const HKEY_DYN_DATA: HKEY = 0x80000006i32 as usize as HKEY;
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: HKEY = 0x80000007i32 as usize as HKEY;
// PROVIDER_KEEPS_VALUE_LENGTH
// val_context
// PVALUEA
// PVALUEW
// QUERYHANDLER
// REG_PROVIDER
STRUCT!{struct VALENTA {
    ve_valuename: LPSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
pub type PVALENTA = *mut VALENTA;
STRUCT!{struct VALENTW {
    ve_valuename: LPWSTR,
    ve_valuelen: DWORD,
    ve_valueptr: DWORD_PTR,
    ve_type: DWORD,
}}
pub type PVALENTW = *mut VALENTW;
// WIN31_CLASS
pub const REG_MUI_STRING_TRUNCATE: DWORD = 0x00000001;
pub const REG_SECURE_CONNECTION: DWORD = 1;
extern "system" {
    pub fn RegCloseKey(
        hKey: HKEY,
    ) -> LSTATUS;
    pub fn RegOverridePredefKey(
        hKey: HKEY,
        hNewHKey: HKEY,
    ) -> LSTATUS;
    pub fn RegOpenUserClassesRoot(
        hToken: HANDLE,
        dwOptions: DWORD,
        samDesired: REGSAM,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegOpenCurrentUser(
        samDesired: REGSAM,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegDisablePredefinedCache() -> LSTATUS;
    pub fn RegDisablePredefinedCacheEx() -> LSTATUS;
    pub fn RegConnectRegistryA(
        lpMachineName: LPCSTR,
        hKey: HKEY,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegConnectRegistryW(
        lpMachineName: LPCWSTR,
        hKey: HKEY,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegConnectRegistryExA(
        lpMachineName: LPCSTR,
        hKey: HKEY,
        flags: ULONG,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegConnectRegistryExW(
        lpMachineName: LPCWSTR,
        hKey: HKEY,
        flags: ULONG,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegCreateKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegCreateKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegCreateKeyExA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        Reserved: DWORD,
        lpClass: LPSTR,
        dwOptions: DWORD,
        samDesired: REGSAM,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
    ) -> LSTATUS;
    pub fn RegCreateKeyExW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        Reserved: DWORD,
        lpClass: LPWSTR,
        dwOptions: DWORD,
        samDesired: REGSAM,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
    ) -> LSTATUS;
    pub fn RegCreateKeyTransactedA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        Reserved: DWORD,
        lpClass: LPSTR,
        dwOptions: DWORD,
        samDesired: REGSAM,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegCreateKeyTransactedW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        Reserved: DWORD,
        lpClass: LPWSTR,
        dwOptions: DWORD,
        samDesired: REGSAM,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        phkResult: PHKEY,
        lpdwDisposition: LPDWORD,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegDeleteKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
    ) -> LSTATUS;
    pub fn RegDeleteKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegDeleteKeyExA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        samDesired: REGSAM,
        Reserved: DWORD,
    ) -> LSTATUS;
    pub fn RegDeleteKeyExW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        samDesired: REGSAM,
        Reserved: DWORD,
    ) -> LSTATUS;
    pub fn RegDeleteKeyTransactedA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        samDesired: REGSAM,
        Reserved: DWORD,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegDeleteKeyTransactedW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        samDesired: REGSAM,
        Reserved: DWORD,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegDisableReflectionKey(
        hBase: HKEY,
    ) -> LONG;
    pub fn RegEnableReflectionKey(
        hBase: HKEY,
    ) -> LONG;
    pub fn RegQueryReflectionKey(
        hBase: HKEY,
        bIsReflectionDisabled: *mut BOOL,
    ) -> LONG;
    pub fn RegDeleteValueA(
        hKey: HKEY,
        lpValueName: LPCSTR,
    ) -> LSTATUS;
    pub fn RegDeleteValueW(
        hKey: HKEY,
        lpValueName: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegEnumKeyA(
        hKey: HKEY,
        dwIndex: DWORD,
        lpName: LPSTR,
        cchName: DWORD,
    ) -> LSTATUS;
    pub fn RegEnumKeyW(
        hKey: HKEY,
        dwIndex: DWORD,
        lpName: LPWSTR,
        cchName: DWORD,
    ) -> LSTATUS;
    pub fn RegEnumKeyExA(
        hKey: HKEY,
        dwIndex: DWORD,
        lpName: LPSTR,
        lpcName: LPDWORD,
        lpReserved: LPDWORD,
        lpClass: LPSTR,
        lpcClass: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LSTATUS;
    pub fn RegEnumKeyExW(
        hKey: HKEY,
        dwIndex: DWORD,
        lpName: LPWSTR,
        lpcName: LPDWORD,
        lpReserved: LPDWORD,
        lpClass: LPWSTR,
        lpcClass: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LSTATUS;
    pub fn RegEnumValueA(
        hKey: HKEY,
        dwIndex: DWORD,
        lpValueName: LPSTR,
        lpcchValueName: LPDWORD,
        lpReserved: LPDWORD,
        lpType: LPDWORD,
        lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegEnumValueW(
        hKey: HKEY,
        dwIndex: DWORD,
        lpValueName: LPWSTR,
        lpcchValueName: LPDWORD,
        lpReserved: LPDWORD,
        lpType: LPDWORD,
        lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegFlushKey(
        hKey: HKEY,
    ) -> LSTATUS;
    pub fn RegGetKeySecurity(
        hKey: HKEY,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
        lpcbSecurityDescriptor: LPDWORD,
    ) -> LSTATUS;
    pub fn RegLoadKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        lpFile: LPCSTR,
    ) -> LSTATUS;
    pub fn RegLoadKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        lpFile: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegNotifyChangeKeyValue(
        hKey: HKEY,
        bWatchSubtree: BOOL,
        dwNotifyFilter: DWORD,
        hEvent: HANDLE,
        fAsynchronous: BOOL,
    ) -> LSTATUS;
    pub fn RegOpenKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegOpenKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegOpenKeyExA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        ulOptions: DWORD,
        samDesired: REGSAM,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegOpenKeyExW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        ulOptions: DWORD,
        samDesired: REGSAM,
        phkResult: PHKEY,
    ) -> LSTATUS;
    pub fn RegOpenKeyTransactedA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        ulOptions: DWORD,
        samDesired: REGSAM,
        phkResult: PHKEY,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegOpenKeyTransactedW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        ulOptions: DWORD,
        samDesired: REGSAM,
        phkResult: PHKEY,
        hTransaction: HANDLE,
        pExtendedParemeter: PVOID,
    ) -> LSTATUS;
    pub fn RegQueryInfoKeyA(
        hKey: HKEY,
        lpClass: LPSTR,
        lpcClass: LPDWORD,
        lpReserved: LPDWORD,
        lpcSubKeys: LPDWORD,
        lpcMaxSubKeyLen: LPDWORD,
        lpcMaxClassLen: LPDWORD,
        lpcValues: LPDWORD,
        lpcMaxValueNameLen: LPDWORD,
        lpcMaxValueLen: LPDWORD,
        lpcbSecurityDescriptor: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LSTATUS;
    pub fn RegQueryInfoKeyW(
        hKey: HKEY,
        lpClass: LPWSTR,
        lpcClass: LPDWORD,
        lpReserved: LPDWORD,
        lpcSubKeys: LPDWORD,
        lpcMaxSubKeyLen: LPDWORD,
        lpcMaxClassLen: LPDWORD,
        lpcValues: LPDWORD,
        lpcMaxValueNameLen: LPDWORD,
        lpcMaxValueLen: LPDWORD,
        lpcbSecurityDescriptor: LPDWORD,
        lpftLastWriteTime: PFILETIME,
    ) -> LSTATUS;
    pub fn RegQueryValueA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        lpData: LPSTR,
        lpcbData: PLONG,
    ) -> LSTATUS;
    pub fn RegQueryValueW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        lpData: LPWSTR,
        lpcbData: PLONG,
    ) -> LSTATUS;
    pub fn RegQueryMultipleValuesA(
        hKey: HKEY,
        val_list: PVALENTA,
        num_vals: DWORD,
        lpValueBuf: LPSTR,
        ldwTotsize: LPDWORD,
    ) -> LSTATUS;
    pub fn RegQueryMultipleValuesW(
        hKey: HKEY,
        val_list: PVALENTW,
        num_vals: DWORD,
        lpValueBuf: LPWSTR,
        ldwTotsize: LPDWORD,
    ) -> LSTATUS;
    pub fn RegQueryValueExA(
        hKey: HKEY,
        lpValueName: LPCSTR,
        lpReserved: LPDWORD,
        lpType: LPDWORD,
        lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegQueryValueExW(
        hKey: HKEY,
        lpValueName: LPCWSTR,
        lpReserved: LPDWORD,
        lpType: LPDWORD,
        lpData: LPBYTE,
        lpcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegReplaceKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        lpNewFile: LPCSTR,
        lpOldFile: LPCSTR,
    ) -> LSTATUS;
    pub fn RegReplaceKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        lpNewFile: LPCWSTR,
        lpOldFile: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegRestoreKeyA(
        hKey: HKEY,
        lpFile: LPCSTR,
        dwFlags: DWORD,
    ) -> LSTATUS;
    pub fn RegRestoreKeyW(
        hKey: HKEY,
        lpFile: LPCWSTR,
        dwFlags: DWORD,
    ) -> LSTATUS;
    pub fn RegRenameKey(
        hKey: HKEY,
        lpSubKeyName: LPCWSTR,
        lpNewKeyName: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegSaveKeyA(
        hKey: HKEY,
        lpFile: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> LSTATUS;
    pub fn RegSaveKeyW(
        hKey: HKEY,
        lpFile: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
    ) -> LSTATUS;
    pub fn RegSetKeySecurity(
        hKey: HKEY,
        SecurityInformation: SECURITY_INFORMATION,
        pSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> LSTATUS;
    pub fn RegSetValueA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        dwType: DWORD,
        lpData: LPCSTR,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegSetValueW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        dwType: DWORD,
        lpData: LPCWSTR,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegSetValueExA(
        hKey: HKEY,
        lpValueName: LPCSTR,
        Reserved: DWORD,
        dwType: DWORD,
        lpData: *const BYTE,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegSetValueExW(
        hKey: HKEY,
        lpValueName: LPCWSTR,
        Reserved: DWORD,
        dwType: DWORD,
        lpData: *const BYTE,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegUnLoadKeyA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
    ) -> LSTATUS;
    pub fn RegUnLoadKeyW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegDeleteKeyValueA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        lpValueName: LPCSTR,
    ) -> LSTATUS;
    pub fn RegDeleteKeyValueW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        lpValueName: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegSetKeyValueA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
        lpValueName: LPCSTR,
        dwType: DWORD,
        lpData: LPCVOID,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegSetKeyValueW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
        lpValueName: LPCWSTR,
        dwType: DWORD,
        lpData: LPCVOID,
        cbData: DWORD,
    ) -> LSTATUS;
    pub fn RegDeleteTreeA(
        hKey: HKEY,
        lpSubKey: LPCSTR,
    ) -> LSTATUS;
    pub fn RegDeleteTreeW(
        hKey: HKEY,
        lpSubKey: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegCopyTreeA(
        hKeySrc: HKEY,
        lpSubKey: LPCSTR,
        hKeyDest: HKEY,
    ) -> LSTATUS;
    pub fn RegGetValueA(
        hkey: HKEY,
        lpSubKey: LPCSTR,
        lpValue: LPCSTR,
        dwFlags: DWORD,
        pdwType: LPDWORD,
        pvData: PVOID,
        pcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegGetValueW(
        hkey: HKEY,
        lpSubKey: LPCWSTR,
        lpValue: LPCWSTR,
        dwFlags: DWORD,
        pdwType: LPDWORD,
        pvData: PVOID,
        pcbData: LPDWORD,
    ) -> LSTATUS;
    pub fn RegCopyTreeW(
        hKeySrc: HKEY,
        lpSubKey: LPCWSTR,
        hKeyDest: HKEY,
    ) -> LSTATUS;
    pub fn RegLoadMUIStringA(
        hKey: HKEY,
        pszValue: LPCSTR,
        pszOutBuf: LPSTR,
        cbOutBuf: DWORD,
        pcbData: LPDWORD,
        Flags: DWORD,
        pszDirectory: LPCSTR,
    ) -> LSTATUS;
    pub fn RegLoadMUIStringW(
        hKey: HKEY,
        pszValue: LPCWSTR,
        pszOutBuf: LPWSTR,
        cbOutBuf: DWORD,
        pcbData: LPDWORD,
        Flags: DWORD,
        pszDirectory: LPCWSTR,
    ) -> LSTATUS;
    pub fn RegLoadAppKeyA(
        lpFile: LPCSTR,
        phkResult: PHKEY,
        samDesired: REGSAM,
        dwOptions: DWORD,
        Reserved: DWORD,
    ) -> LSTATUS;
    pub fn RegLoadAppKeyW(
        lpFile: LPCWSTR,
        phkResult: PHKEY,
        samDesired: REGSAM,
        dwOptions: DWORD,
        Reserved: DWORD,
    ) -> LSTATUS;
    pub fn InitiateSystemShutdownA(
        lpMachineName: LPSTR,
        lpMessage: LPSTR,
        dwTimeout: DWORD,
        bForceAppsClosed: BOOL,
        bRebootAfterShutdown: BOOL,
    ) -> BOOL;
    pub fn InitiateSystemShutdownW(
        lpMachineName: LPWSTR,
        lpMessage: LPWSTR,
        dwTimeout: DWORD,
        bForceAppsClosed: BOOL,
        bRebootAfterShutdown: BOOL,
    ) -> BOOL;
    pub fn AbortSystemShutdownA(
        lpMachineName: LPSTR,
    ) -> BOOL;
    pub fn AbortSystemShutdownW(
        lpMachineName: LPWSTR,
    ) -> BOOL;
}
pub const REASON_SWINSTALL: DWORD = SHTDN_REASON_MAJOR_SOFTWARE | SHTDN_REASON_MINOR_INSTALLATION;
pub const REASON_HWINSTALL: DWORD = SHTDN_REASON_MAJOR_HARDWARE | SHTDN_REASON_MINOR_INSTALLATION;
pub const REASON_SERVICEHANG: DWORD = SHTDN_REASON_MAJOR_SOFTWARE | SHTDN_REASON_MINOR_HUNG;
pub const REASON_UNSTABLE: DWORD = SHTDN_REASON_MAJOR_SYSTEM | SHTDN_REASON_MINOR_UNSTABLE;
pub const REASON_SWHWRECONF: DWORD = SHTDN_REASON_MAJOR_SOFTWARE | SHTDN_REASON_MINOR_RECONFIG;
pub const REASON_OTHER: DWORD = SHTDN_REASON_MAJOR_OTHER | SHTDN_REASON_MINOR_OTHER;
pub const REASON_UNKNOWN: DWORD = SHTDN_REASON_UNKNOWN;
pub const REASON_LEGACY_API: DWORD = SHTDN_REASON_LEGACY_API;
pub const REASON_PLANNED_FLAG: DWORD = SHTDN_REASON_FLAG_PLANNED;
pub const MAX_SHUTDOWN_TIMEOUT: DWORD = 10 * 365 * 24 * 60 * 60;
extern "system" {
    pub fn InitiateSystemShutdownExA(
        lpMachineName: LPSTR,
        lpMessage: LPSTR,
        dwTimeout: DWORD,
        bForceAppsClosed: BOOL,
        bRebootAfterShutdown: BOOL,
        dwReason: DWORD,
    ) -> BOOL;
    pub fn InitiateSystemShutdownExW(
        lpMachineName: LPWSTR,
        lpMessage: LPWSTR,
        dwTimeout: DWORD,
        bForceAppsClosed: BOOL,
        bRebootAfterShutdown: BOOL,
        dwReason: DWORD,
    ) -> BOOL;
}
pub const SHUTDOWN_FORCE_OTHERS: DWORD = 0x00000001;
pub const SHUTDOWN_FORCE_SELF: DWORD = 0x00000002;
pub const SHUTDOWN_RESTART: DWORD = 0x00000004;
pub const SHUTDOWN_POWEROFF: DWORD = 0x00000008;
pub const SHUTDOWN_NOREBOOT: DWORD = 0x00000010;
pub const SHUTDOWN_GRACE_OVERRIDE: DWORD = 0x00000020;
pub const SHUTDOWN_INSTALL_UPDATES: DWORD = 0x00000040;
pub const SHUTDOWN_RESTARTAPPS: DWORD = 0x00000080;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: DWORD = 0x00000100;
pub const SHUTDOWN_HYBRID: DWORD = 0x00000200;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: DWORD = 0x00000400;
pub const SHUTDOWN_SOFT_REBOOT: DWORD = 0x00000800;
pub const SHUTDOWN_MOBILE_UI: DWORD = 0x00001000;
pub const SHUTDOWN_ARSO: DWORD = 0x00002000;
extern "system" {
    pub fn InitiateShutdownA(
        lpMachineName: LPSTR,
        lpMessage: LPSTR,
        dwGracePeriod: DWORD,
        dwShutdownFlags: DWORD,
        dwReason: DWORD,
    ) -> DWORD;
    pub fn InitiateShutdownW(
        lpMachineName: LPWSTR,
        lpMessage: LPWSTR,
        dwGracePeriod: DWORD,
        dwShutdownFlags: DWORD,
        dwReason: DWORD,
    ) -> DWORD;
    pub fn CheckForHiberboot(
        pHiberboot: PBOOLEAN,
        bClearFlag: BOOLEAN,
    ) -> DWORD;
    pub fn RegSaveKeyExA(
        hKey: HKEY,
        lpFile: LPCSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        Flags: DWORD,
    ) -> LSTATUS;
    pub fn RegSaveKeyExW(
        hKey: HKEY,
        lpFile: LPCWSTR,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        Flags: DWORD,
    ) -> LSTATUS;
}
