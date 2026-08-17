// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::{DWORD, ULONG};
use um::winnt::{
    BOOLEAN, HANDLE, LONG, POWER_INFORMATION_LEVEL, POWER_PLATFORM_ROLE,
    PSYSTEM_POWER_CAPABILITIES, PVOID,
};
use um::winuser::{HPOWERNOTIFY, PHPOWERNOTIFY};
pub type NTSTATUS = LONG;
extern "system" {
    pub fn CallNtPowerInformation(
        InformationLevel: POWER_INFORMATION_LEVEL,
        InputBuffer: PVOID,
        InputBufferLength: ULONG,
        OutputBuffer: PVOID,
        OutputBufferLength: ULONG,
    ) -> NTSTATUS;
    pub fn GetPwrCapabilities(
        lpspc: PSYSTEM_POWER_CAPABILITIES,
    ) -> BOOLEAN;
    pub fn PowerDeterminePlatformRoleEx(
        Version: ULONG,
    ) -> POWER_PLATFORM_ROLE;
    pub fn PowerRegisterSuspendResumeNotification(
        Flags: DWORD,
        Recipient: HANDLE,
        RegistrationHandle: PHPOWERNOTIFY,
    ) -> DWORD;
    pub fn PowerUnregisterSuspendResumeNotification(
        RegistrationHandle: HPOWERNOTIFY,
    ) -> DWORD;
}
