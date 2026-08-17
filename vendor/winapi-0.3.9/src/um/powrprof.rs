// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Interface for powrprof.dll, the power policy applicator
use shared::guiddef::GUID;
use shared::minwindef::{
    BOOL, DWORD, HKEY, LPARAM, LPDWORD, PBYTE, PUCHAR, PUINT, PULONG, UCHAR, UINT, ULONG,
};
use um::winnt::{
    BOOLEAN, LPCWSTR, LPWSTR, NUM_DISCHARGE_POLICIES, PADMINISTRATOR_POWER_POLICY,
    POWER_ACTION_POLICY, POWER_PLATFORM_ROLE, PROCESSOR_POWER_POLICY, PVOID, SYSTEM_POWER_LEVEL,
    SYSTEM_POWER_STATE,
};
use um::winreg::REGSAM;
STRUCT!{struct GLOBAL_MACHINE_POWER_POLICY {
    Revision: ULONG,
    LidOpenWakeAc: SYSTEM_POWER_STATE,
    LidOpenWakeDc: SYSTEM_POWER_STATE,
    BroadcastCapacityResolution: ULONG,
}}
pub type PGLOBAL_MACHINE_POWER_POLICY = *mut GLOBAL_MACHINE_POWER_POLICY;
STRUCT!{struct GLOBAL_USER_POWER_POLICY {
    Revision: ULONG,
    PowerButtonAc: POWER_ACTION_POLICY,
    PowerButtonDc: POWER_ACTION_POLICY,
    SleepButtonAc: POWER_ACTION_POLICY,
    SleepButtonDc: POWER_ACTION_POLICY,
    LidCloseAc: POWER_ACTION_POLICY,
    LidCloseDc: POWER_ACTION_POLICY,
    DischargePolicy: [SYSTEM_POWER_LEVEL; NUM_DISCHARGE_POLICIES],
    GlobalFlags: ULONG,
}}
pub type PGLOBAL_USER_POWER_POLICY = *mut GLOBAL_USER_POWER_POLICY;
STRUCT!{struct GLOBAL_POWER_POLICY {
    user: GLOBAL_USER_POWER_POLICY,
    mach: GLOBAL_MACHINE_POWER_POLICY,
}}
pub type PGLOBAL_POWER_POLICY = *mut GLOBAL_POWER_POLICY;
STRUCT!{struct MACHINE_POWER_POLICY {
    Revision: ULONG,
    MinSleepAc: SYSTEM_POWER_STATE,
    MinSleepDc: SYSTEM_POWER_STATE,
    ReducedLatencySleepAc: SYSTEM_POWER_STATE,
    ReducedLatencySleepDc: SYSTEM_POWER_STATE,
    DozeTimeoutAc: ULONG,
    DozeTimeoutDc: ULONG,
    DozeS4TimeoutAc: ULONG,
    DozeS4TimeoutDc: ULONG,
    MinThrottleAc: UCHAR,
    MinThrottleDc: UCHAR,
    pad1: [UCHAR; 2],
    OverThrottledAc: POWER_ACTION_POLICY,
    OverThrottledDc: POWER_ACTION_POLICY,
}}
pub type PMACHINE_POWER_POLICY = *mut MACHINE_POWER_POLICY;
STRUCT!{struct MACHINE_PROCESSOR_POWER_POLICY {
    Revision: ULONG,
    ProcessorPolicyAc: PROCESSOR_POWER_POLICY,
    ProcessorPolicyDc: PROCESSOR_POWER_POLICY,
}}
pub type PMACHINE_PROCESSOR_POWER_POLICY = *mut MACHINE_PROCESSOR_POWER_POLICY;
STRUCT!{struct USER_POWER_POLICY {
    Revision: ULONG,
    IdleAc: POWER_ACTION_POLICY,
    IdleDc: POWER_ACTION_POLICY,
    IdleTimeoutAc: ULONG,
    IdleTimeoutDc: ULONG,
    IdleSensitivityAc: UCHAR,
    IdleSensitivityDc: UCHAR,
    ThrottlePolicyAc: UCHAR,
    ThrottlePolicyDc: UCHAR,
    MaxSleepAc: SYSTEM_POWER_STATE,
    MaxSleepDc: SYSTEM_POWER_STATE,
    Reserved: [ULONG; 2],
    VideoTimeoutAc: ULONG,
    VideoTimeoutDc: ULONG,
    SpindownTimeoutAc: ULONG,
    SpindownTimeoutDc: ULONG,
    OptimizeForPowerAc: BOOLEAN,
    OptimizeForPowerDc: BOOLEAN,
    FanThrottleToleranceAc: UCHAR,
    FanThrottleToleranceDc: UCHAR,
    ForcedThrottleAc: UCHAR,
    ForcedThrottleDc: UCHAR,
}}
pub type PUSER_POWER_POLICY = *mut USER_POWER_POLICY;
STRUCT!{struct POWER_POLICY {
    user: USER_POWER_POLICY,
    mach: MACHINE_POWER_POLICY,
}}
pub type PPOWER_POLICY = *mut POWER_POLICY;
pub const EnableSysTrayBatteryMeter: ULONG = 0x01;
pub const EnableMultiBatteryDisplay: ULONG = 0x02;
pub const EnablePasswordLogon: ULONG = 0x04;
pub const EnableWakeOnRing: ULONG = 0x08;
pub const EnableVideoDimDisplay: ULONG = 0x10;
pub const POWER_ATTRIBUTE_HIDE: ULONG = 0x00000001;
pub const POWER_ATTRIBUTE_SHOW_AOAC: ULONG = 0x00000002;
pub const NEWSCHEME: UINT = -1i32 as u32;
FN!{stdcall PWRSCHEMESENUMPROC_V1(
    Index: UINT,
    NameSize: DWORD,
    Name: LPWSTR,
    DescriptionSize: DWORD,
    Description: LPWSTR,
    Policy: PPOWER_POLICY,
    Context: LPARAM,
) -> BOOLEAN}
FN!{stdcall PWRSCHEMESENUMPROC_V2(
    Index: UINT,
    NameSize: DWORD,
    Name: LPWSTR,
    DescriptionSize: DWORD,
    Description: LPWSTR,
    Policy: PPOWER_POLICY,
    Context: LPARAM,
) -> BOOLEAN}
pub type PWRSCHEMESENUMPROC = *mut PWRSCHEMESENUMPROC_V2;
extern "system" {
    pub fn GetPwrDiskSpindownRange(
        puiMax: PUINT,
        puiMin: PUINT,
    ) -> BOOLEAN;
    pub fn EnumPwrSchemes(
        lpfn: PWRSCHEMESENUMPROC,
        lParam: LPARAM,
    ) -> BOOLEAN;
    pub fn ReadGlobalPwrPolicy(
        pGlobalPowerPolicy: PGLOBAL_POWER_POLICY,
    ) -> BOOLEAN;
    pub fn ReadPwrScheme(
        uiID: UINT,
        pPowerPolicy: PPOWER_POLICY,
    ) -> BOOLEAN;
    pub fn WritePwrScheme(
        puiID: PUINT,
        lpszSchemeName: LPCWSTR,
        lpszDescription: LPCWSTR,
        lpScheme: PPOWER_POLICY,
    ) -> BOOLEAN;
    pub fn WriteGlobalPwrPolicy(
        pGlobalPowerPolicy: PGLOBAL_POWER_POLICY,
    ) -> BOOLEAN;
    pub fn DeletePwrScheme(
        uiID: UINT,
    ) -> BOOLEAN;
    pub fn GetActivePwrScheme(
        puiID: PUINT,
    ) -> BOOLEAN;
    pub fn SetActivePwrScheme(
        uiID: UINT,
        pGlobalPowerPolicy: PGLOBAL_POWER_POLICY,
        pPowerPolicy: PPOWER_POLICY,
    ) -> BOOLEAN;
    pub fn IsPwrSuspendAllowed() -> BOOLEAN;
    pub fn IsPwrHibernateAllowed() -> BOOLEAN;
    pub fn IsPwrShutdownAllowed() -> BOOLEAN;
    pub fn IsAdminOverrideActive(
        papp: PADMINISTRATOR_POWER_POLICY,
    ) -> BOOLEAN;
    pub fn SetSuspendState(
        bHibernate: BOOLEAN,
        bForce: BOOLEAN,
        bWakeupEventsDisabled: BOOLEAN,
    ) -> BOOLEAN;
    pub fn GetCurrentPowerPolicies(
        pGlobalPowerPolicy: PGLOBAL_POWER_POLICY,
        pPowerPolicy: PPOWER_POLICY,
    ) -> BOOLEAN;
    pub fn CanUserWritePwrScheme() -> BOOLEAN;
    pub fn ReadProcessorPwrScheme(
        uiID: UINT,
        pMachineProcessorPowerPolicy: PMACHINE_PROCESSOR_POWER_POLICY,
    ) -> BOOLEAN;
    pub fn WriteProcessorPwrScheme(
        uiID: UINT,
        pMachineProcessorPowerPolicy: PMACHINE_PROCESSOR_POWER_POLICY,
    ) -> BOOLEAN;
    pub fn ValidatePowerPolicies(
        pGlobalPowerPolicy: PGLOBAL_POWER_POLICY,
        pPowerPolicy: PPOWER_POLICY,
    ) -> BOOLEAN;
}
ENUM!{enum POWER_DATA_ACCESSOR {
    ACCESS_AC_POWER_SETTING_INDEX = 0,
    ACCESS_DC_POWER_SETTING_INDEX,
    ACCESS_FRIENDLY_NAME,
    ACCESS_DESCRIPTION,
    ACCESS_POSSIBLE_POWER_SETTING,
    ACCESS_POSSIBLE_POWER_SETTING_FRIENDLY_NAME,
    ACCESS_POSSIBLE_POWER_SETTING_DESCRIPTION,
    ACCESS_DEFAULT_AC_POWER_SETTING,
    ACCESS_DEFAULT_DC_POWER_SETTING,
    ACCESS_POSSIBLE_VALUE_MIN,
    ACCESS_POSSIBLE_VALUE_MAX,
    ACCESS_POSSIBLE_VALUE_INCREMENT,
    ACCESS_POSSIBLE_VALUE_UNITS,
    ACCESS_ICON_RESOURCE,
    ACCESS_DEFAULT_SECURITY_DESCRIPTOR,
    ACCESS_ATTRIBUTES,
    ACCESS_SCHEME,
    ACCESS_SUBGROUP,
    ACCESS_INDIVIDUAL_SETTING,
    ACCESS_ACTIVE_SCHEME,
    ACCESS_CREATE_SCHEME,
    ACCESS_AC_POWER_SETTING_MAX,
    ACCESS_DC_POWER_SETTING_MAX,
    ACCESS_AC_POWER_SETTING_MIN,
    ACCESS_DC_POWER_SETTING_MIN,
    ACCESS_PROFILE,
    ACCESS_OVERLAY_SCHEME,
    ACCESS_ACTIVE_OVERLAY_SCHEME,
}}
pub type PPOWER_DATA_ACCESSOR = *mut POWER_DATA_ACCESSOR;
pub const DEVICE_NOTIFY_CALLBACK: ULONG = 2;
FN!{stdcall DEVICE_NOTIFY_CALLBACK_ROUTINE(
    Context: PVOID,
    Type: ULONG,
    Setting: PVOID,
) -> ULONG}
pub type PDEVICE_NOTIFY_CALLBACK_ROUTINE = *mut DEVICE_NOTIFY_CALLBACK_ROUTINE;
STRUCT!{struct DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
    Callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
    Context: PVOID,
}}
pub type PDEVICE_NOTIFY_SUBSCRIBE_PARAMETERS = *mut DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;
extern "system" {
    pub fn PowerIsSettingRangeDefined(
        SubKeyGuid: *const GUID,
        SettingGuid: *const GUID,
    ) -> BOOLEAN;
    pub fn PowerSettingAccessCheckEx(
        AccessFlags: POWER_DATA_ACCESSOR,
        PowerGuid: *const GUID,
        AccessType: REGSAM,
    ) -> DWORD;
    pub fn PowerSettingAccessCheck(
        AccessFlags: POWER_DATA_ACCESSOR,
        PowerGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerReadACValueIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        AcValueIndex: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadDCValueIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        DcValueIndex: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadFriendlyName(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadDescription(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadPossibleValue(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Type: PULONG,
        PossibleSettingIndex: ULONG,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadPossibleFriendlyName(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        PossibleSettingIndex: ULONG,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadPossibleDescription(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        PossibleSettingIndex: ULONG,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadValueMin(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueMinimum: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadValueMax(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueMaximum: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadValueIncrement(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueIncrement: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadValueUnitsSpecifier(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: *mut UCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadACDefaultIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        AcDefaultIndex: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadDCDefaultIndex(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        DcDefaultIndex: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadIconResourceSpecifier(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: PUCHAR,
        BufferSize: LPDWORD,
    ) -> DWORD;
    pub fn PowerReadSettingAttributes(
        SubGroupGuid: *const GUID,
        PowerSettingGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerWriteFriendlyName(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWriteDescription(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWritePossibleValue(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Type: ULONG,
        PossibleSettingIndex: ULONG,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWritePossibleFriendlyName(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        PossibleSettingIndex: ULONG,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWritePossibleDescription(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        PossibleSettingIndex: ULONG,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWriteValueMin(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueMinimum: DWORD,
    ) -> DWORD;
    pub fn PowerWriteValueMax(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueMaximum: DWORD,
    ) -> DWORD;
    pub fn PowerWriteValueIncrement(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        ValueIncrement: DWORD,
    ) -> DWORD;
    pub fn PowerWriteValueUnitsSpecifier(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWriteACDefaultIndex(
        RootSystemPowerKey: HKEY,
        SchemePersonalityGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        DefaultAcIndex: DWORD,
    ) -> DWORD;
    pub fn PowerWriteDCDefaultIndex(
        RootSystemPowerKey: HKEY,
        SchemePersonalityGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        DefaultDcIndex: DWORD,
    ) -> DWORD;
    pub fn PowerWriteIconResourceSpecifier(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Buffer: *mut UCHAR,
        BufferSize: DWORD,
    ) -> DWORD;
    pub fn PowerWriteSettingAttributes(
        SubGroupGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        Attributes: DWORD,
    ) -> DWORD;
    pub fn PowerDuplicateScheme(
        RootPowerKey: HKEY,
        SourceSchemeGuid: *const GUID,
        DestinationSchemeGuid: *mut *mut GUID,
    ) -> DWORD;
    pub fn PowerImportPowerScheme(
        RootPowerKey: HKEY,
        ImportFileNamePath: LPCWSTR,
        DestinationSchemeGuid: *mut *mut GUID,
    ) -> DWORD;
    pub fn PowerDeleteScheme(
        RootPowerKey: HKEY,
        SchemeGuid: *mut GUID,
    ) -> DWORD;
    pub fn PowerRemovePowerSetting(
        PowerSettingSubKeyGuid: *const GUID,
        PowerSettingGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerCreateSetting(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerCreatePossibleSetting(
        RootPowerKey: HKEY,
        SubGroupOfPowerSettingsGuid: *const GUID,
        PowerSettingGuid: *const GUID,
        PossibleSettingIndex: ULONG,
    ) -> DWORD;
    pub fn PowerEnumerate(
        RootPowerKey: HKEY,
        SchemeGuid: *const GUID,
        SubGroupOfPowerSettingsGuid: *const GUID,
        AccessFlags: POWER_DATA_ACCESSOR,
        Index: ULONG,
        Buffer: *mut UCHAR,
        BufferSize: *mut DWORD,
    ) -> DWORD;
    pub fn PowerOpenUserPowerKey(
        phUserPowerKey: *mut HKEY,
        Access: REGSAM,
        OpenExisting: BOOL,
    ) -> DWORD;
    pub fn PowerOpenSystemPowerKey(
        phSystemPowerKey: *mut HKEY,
        Access: REGSAM,
        OpenExisting: BOOL,
    ) -> DWORD;
    pub fn PowerCanRestoreIndividualDefaultPowerScheme(
        SchemeGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerRestoreIndividualDefaultPowerScheme(
        SchemeGuid: *const GUID,
    ) -> DWORD;
    pub fn PowerRestoreDefaultPowerSchemes() -> DWORD;
    pub fn PowerReplaceDefaultPowerSchemes() -> DWORD;
    pub fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE;
}
pub const DEVICEPOWER_HARDWAREID: ULONG = 0x80000000;
pub const DEVICEPOWER_AND_OPERATION: ULONG = 0x40000000;
pub const DEVICEPOWER_FILTER_DEVICES_PRESENT: ULONG = 0x20000000;
pub const DEVICEPOWER_FILTER_HARDWARE: ULONG = 0x10000000;
pub const DEVICEPOWER_FILTER_WAKEENABLED: ULONG = 0x08000000;
pub const DEVICEPOWER_FILTER_WAKEPROGRAMMABLE: ULONG = 0x04000000;
pub const DEVICEPOWER_FILTER_ON_NAME: ULONG = 0x02000000;
pub const DEVICEPOWER_SET_WAKEENABLED: ULONG = 0x00000001;
pub const DEVICEPOWER_CLEAR_WAKEENABLED: ULONG = 0x00000002;
pub const PDCAP_S0_SUPPORTED: ULONG = 0x00010000;
pub const PDCAP_S1_SUPPORTED: ULONG = 0x00020000;
pub const PDCAP_S2_SUPPORTED: ULONG = 0x00040000;
pub const PDCAP_S3_SUPPORTED: ULONG = 0x00080000;
pub const PDCAP_WAKE_FROM_S0_SUPPORTED: ULONG = 0x00100000;
pub const PDCAP_WAKE_FROM_S1_SUPPORTED: ULONG = 0x00200000;
pub const PDCAP_WAKE_FROM_S2_SUPPORTED: ULONG = 0x00400000;
pub const PDCAP_WAKE_FROM_S3_SUPPORTED: ULONG = 0x00800000;
pub const PDCAP_S4_SUPPORTED: ULONG = 0x01000000;
pub const PDCAP_S5_SUPPORTED: ULONG = 0x02000000;
extern "system" {
    pub fn DevicePowerEnumDevices(
        QueryIndex: ULONG,
        QueryInterpretationFlags: ULONG,
        QueryFlags: ULONG,
        pReturnBuffer: PBYTE,
        pBufferSize: PULONG,
    ) -> BOOLEAN;
    pub fn DevicePowerSetDeviceState(
        DeviceDescription: LPCWSTR,
        SetFlags: ULONG,
        SetData: PVOID,
    ) -> DWORD;
    pub fn DevicePowerOpen(
        DebugMask: ULONG,
    ) -> BOOLEAN;
    pub fn DevicePowerClose() -> BOOLEAN;
}
STRUCT!{struct THERMAL_EVENT {
    Version: ULONG,
    Size: ULONG,
    Type: ULONG,
    Temperature: ULONG,
    TripPointTemperature: ULONG,
    Initiator: LPWSTR,
}}
pub type PTHERMAL_EVENT = *mut THERMAL_EVENT;
extern "system" {
    pub fn PowerReportThermalEvent(
        Event: PTHERMAL_EVENT,
    ) -> DWORD;
}
