// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::{GUID, LPCGUID};
use shared::minwindef::{DWORD, HKEY, LPBYTE, LPDWORD, PUCHAR, PULONG};
use um::winnt::HANDLE;
use um::winuser::{HPOWERNOTIFY, PHPOWERNOTIFY};
extern "system" {
    pub fn PowerReadACValue(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Type: PULONG,
        Buffer: LPBYTE,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadDCValue(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Type: PULONG,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerWriteACValueIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        AcValueIndex: DWORD,
    ) -> DWORD;
    pub fn PowerWriteDCValueIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        DcValueIndex: DWORD,
    ) -> DWORD;
    pub fn PowerGetActiveScheme(
        UserRootPowerKey: HKEY,
        ActivePolicyGuid: *mut *mut GUID,
    ) -> DWORD;
    pub fn PowerSetActiveScheme(
        UserRootPowerKey: HKEY,
        SchemeGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerSettingRegisterNotification(
        SettingGuid: LPCGUID,
        Flags: DWORD,
        Recipient: HANDLE,
        RegistrationHandle: PHPOWERNOTIFY,
    ) -> DWORD;
    pub fn PowerSettingUnregisterNotification(
        RegistrationHandle: HPOWERNOTIFY,
    ) -> DWORD;
}
