// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Header file for the Service Control Manager
use shared::minwindef::{BOOL, DWORD, LPBYTE, LPDWORD, LPVOID};
use um::winnt::{
    HANDLE, LPCSTR, LPCWSTR, LPSTR, LPWSTR, PSECURITY_DESCRIPTOR, PVOID,
    SECURITY_INFORMATION, STANDARD_RIGHTS_REQUIRED
};
pub const SERVICE_NO_CHANGE: DWORD = 0xffffffff;
pub const SERVICE_ACTIVE: DWORD = 0x00000001;
pub const SERVICE_INACTIVE: DWORD = 0x00000002;
pub const SERVICE_STATE_ALL: DWORD = SERVICE_ACTIVE | SERVICE_INACTIVE;
pub const SERVICE_CONTROL_STOP: DWORD = 0x00000001;
pub const SERVICE_CONTROL_PAUSE: DWORD = 0x00000002;
pub const SERVICE_CONTROL_CONTINUE: DWORD = 0x00000003;
pub const SERVICE_CONTROL_INTERROGATE: DWORD = 0x00000004;
pub const SERVICE_CONTROL_SHUTDOWN: DWORD = 0x00000005;
pub const SERVICE_CONTROL_PARAMCHANGE: DWORD = 0x00000006;
pub const SERVICE_CONTROL_NETBINDADD: DWORD = 0x00000007;
pub const SERVICE_CONTROL_NETBINDREMOVE: DWORD = 0x00000008;
pub const SERVICE_CONTROL_NETBINDENABLE: DWORD = 0x00000009;
pub const SERVICE_CONTROL_NETBINDDISABLE: DWORD = 0x0000000A;
pub const SERVICE_CONTROL_DEVICEEVENT: DWORD = 0x0000000B;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: DWORD = 0x0000000C;
pub const SERVICE_CONTROL_POWEREVENT: DWORD = 0x0000000D;
pub const SERVICE_CONTROL_SESSIONCHANGE: DWORD = 0x0000000E;
pub const SERVICE_CONTROL_PRESHUTDOWN: DWORD = 0x0000000F;
pub const SERVICE_CONTROL_TIMECHANGE: DWORD = 0x00000010;
pub const SERVICE_CONTROL_TRIGGEREVENT: DWORD = 0x00000020;
pub const SERVICE_STOPPED: DWORD = 0x00000001;
pub const SERVICE_START_PENDING: DWORD = 0x00000002;
pub const SERVICE_STOP_PENDING: DWORD = 0x00000003;
pub const SERVICE_RUNNING: DWORD = 0x00000004;
pub const SERVICE_CONTINUE_PENDING: DWORD = 0x00000005;
pub const SERVICE_PAUSE_PENDING: DWORD = 0x00000006;
pub const SERVICE_PAUSED: DWORD = 0x00000007;
pub const SERVICE_ACCEPT_STOP: DWORD = 0x00000001;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: DWORD = 0x00000002;
pub const SERVICE_ACCEPT_SHUTDOWN: DWORD = 0x00000004;
pub const SERVICE_ACCEPT_PARAMCHANGE: DWORD = 0x00000008;
pub const SERVICE_ACCEPT_NETBINDCHANGE: DWORD = 0x00000010;
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: DWORD = 0x00000020;
pub const SERVICE_ACCEPT_POWEREVENT: DWORD = 0x00000040;
pub const SERVICE_ACCEPT_SESSIONCHANGE: DWORD = 0x00000080;
pub const SERVICE_ACCEPT_PRESHUTDOWN: DWORD = 0x00000100;
pub const SERVICE_ACCEPT_TIMECHANGE: DWORD = 0x00000200;
pub const SERVICE_ACCEPT_TRIGGEREVENT: DWORD = 0x00000400;
// SERVICE_ACCEPT_USER_LOGOFF
pub const SC_MANAGER_CONNECT: DWORD = 0x0001;
pub const SC_MANAGER_CREATE_SERVICE: DWORD = 0x0002;
pub const SC_MANAGER_ENUMERATE_SERVICE: DWORD = 0x0004;
pub const SC_MANAGER_LOCK: DWORD = 0x0008;
pub const SC_MANAGER_QUERY_LOCK_STATUS: DWORD = 0x0010;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: DWORD = 0x0020;
pub const SC_MANAGER_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | SC_MANAGER_CONNECT
    | SC_MANAGER_CREATE_SERVICE | SC_MANAGER_ENUMERATE_SERVICE | SC_MANAGER_LOCK
    | SC_MANAGER_QUERY_LOCK_STATUS | SC_MANAGER_MODIFY_BOOT_CONFIG;
pub const SERVICE_QUERY_CONFIG: DWORD = 0x0001;
pub const SERVICE_CHANGE_CONFIG: DWORD = 0x0002;
pub const SERVICE_QUERY_STATUS: DWORD = 0x0004;
pub const SERVICE_ENUMERATE_DEPENDENTS: DWORD = 0x0008;
pub const SERVICE_START: DWORD = 0x0010;
pub const SERVICE_STOP: DWORD = 0x0020;
pub const SERVICE_PAUSE_CONTINUE: DWORD = 0x0040;
pub const SERVICE_INTERROGATE: DWORD = 0x0080;
pub const SERVICE_USER_DEFINED_CONTROL: DWORD = 0x0100;
pub const SERVICE_ALL_ACCESS: DWORD = STANDARD_RIGHTS_REQUIRED | SERVICE_QUERY_CONFIG
    | SERVICE_CHANGE_CONFIG | SERVICE_QUERY_STATUS | SERVICE_ENUMERATE_DEPENDENTS | SERVICE_START
    | SERVICE_STOP | SERVICE_PAUSE_CONTINUE | SERVICE_INTERROGATE | SERVICE_USER_DEFINED_CONTROL;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: DWORD = 0x00000001;
pub const SERVICE_CONFIG_DESCRIPTION: DWORD = 1;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: DWORD = 2;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: DWORD = 3;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: DWORD = 4;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: DWORD = 5;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: DWORD = 6;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: DWORD = 7;
pub const SERVICE_CONFIG_TRIGGER_INFO: DWORD = 8;
pub const SERVICE_CONFIG_PREFERRED_NODE: DWORD = 9;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: DWORD = 12;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: DWORD = 1;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: DWORD = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE: DWORD = SERVICE_NOTIFY_STATUS_CHANGE_2;
pub const SERVICE_NOTIFY_STOPPED: DWORD = 0x00000001;
pub const SERVICE_NOTIFY_START_PENDING: DWORD = 0x00000002;
pub const SERVICE_NOTIFY_STOP_PENDING: DWORD = 0x00000004;
pub const SERVICE_NOTIFY_RUNNING: DWORD = 0x00000008;
pub const SERVICE_NOTIFY_CONTINUE_PENDING: DWORD = 0x00000010;
pub const SERVICE_NOTIFY_PAUSE_PENDING: DWORD = 0x00000020;
pub const SERVICE_NOTIFY_PAUSED: DWORD = 0x00000040;
pub const SERVICE_NOTIFY_CREATED: DWORD = 0x00000080;
pub const SERVICE_NOTIFY_DELETED: DWORD = 0x00000100;
pub const SERVICE_NOTIFY_DELETE_PENDING: DWORD = 0x00000200;
pub const SERVICE_STOP_REASON_FLAG_MIN: DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: DWORD = 0x10000000;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: DWORD = 0x20000000;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: DWORD = 0x40000000;
pub const SERVICE_STOP_REASON_FLAG_MAX: DWORD = 0x80000000;
pub const SERVICE_STOP_REASON_MAJOR_MIN: DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: DWORD = 0x00010000;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: DWORD = 0x00020000;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: DWORD = 0x00030000;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: DWORD = 0x00040000;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: DWORD = 0x00050000;
pub const SERVICE_STOP_REASON_MAJOR_NONE: DWORD = 0x00060000;
pub const SERVICE_STOP_REASON_MAJOR_MAX: DWORD = 0x00070000;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: DWORD = 0x00400000;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: DWORD = 0x00ff0000;
pub const SERVICE_STOP_REASON_MINOR_MIN: DWORD = 0x00000000;
pub const SERVICE_STOP_REASON_MINOR_OTHER: DWORD = 0x00000001;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: DWORD = 0x00000002;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: DWORD = 0x00000003;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: DWORD = 0x00000004;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: DWORD = 0x00000005;
pub const SERVICE_STOP_REASON_MINOR_HUNG: DWORD = 0x00000006;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: DWORD = 0x00000007;
pub const SERVICE_STOP_REASON_MINOR_DISK: DWORD = 0x00000008;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: DWORD = 0x00000009;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: DWORD = 0x0000000a;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: DWORD = 0x0000000b;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: DWORD = 0x0000000c;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: DWORD = 0x0000000d;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: DWORD = 0x0000000e;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: DWORD = 0x0000000f;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: DWORD = 0x00000010;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: DWORD = 0x00000011;
pub const SERVICE_STOP_REASON_MINOR_WMI: DWORD = 0x00000012;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: DWORD = 0x00000013;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: DWORD = 0x00000014;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: DWORD = 0x00000015;
pub const SERVICE_STOP_REASON_MINOR_MMC: DWORD = 0x00000016;
pub const SERVICE_STOP_REASON_MINOR_NONE: DWORD = 0x00000017;
pub const SERVICE_STOP_REASON_MINOR_MAX: DWORD = 0x00000018;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: DWORD = 0x00000100;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: DWORD = 0x0000FFFF;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: DWORD = 1;
pub const SERVICE_SID_TYPE_NONE: DWORD = 0x00000000;
pub const SERVICE_SID_TYPE_UNRESTRICTED: DWORD = 0x00000001;
pub const SERVICE_SID_TYPE_RESTRICTED: DWORD = 0x00000002 | SERVICE_SID_TYPE_UNRESTRICTED;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: DWORD = 1;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: DWORD = 2;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: DWORD = 3;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: DWORD = 4;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: DWORD = 5;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: DWORD = 6;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: DWORD = 7;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: DWORD = 20;
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: DWORD = 1;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: DWORD = 2;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: DWORD = 3;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: DWORD = 4;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: DWORD = 5;
pub const SERVICE_START_REASON_DEMAND: DWORD = 0x00000001;
pub const SERVICE_START_REASON_AUTO: DWORD = 0x00000002;
pub const SERVICE_START_REASON_TRIGGER: DWORD = 0x00000004;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: DWORD = 0x00000008;
pub const SERVICE_START_REASON_DELAYEDAUTO: DWORD = 0x00000010;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: DWORD = 1;
pub const SERVICE_LAUNCH_PROTECTED_NONE: DWORD = 0;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: DWORD = 1;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: DWORD = 2;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: DWORD = 3;
DEFINE_GUID!{NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID,
    0x4f27f2de, 0x14e2, 0x430b, 0xa5, 0x49, 0x7c, 0xd4, 0x8c, 0xbc, 0x82, 0x45}
DEFINE_GUID!{NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID,
    0xcc4ba62a, 0x162e, 0x4648, 0x84, 0x7a, 0xb6, 0xbd, 0xf9, 0x93, 0xe3, 0x35}
DEFINE_GUID!{DOMAIN_JOIN_GUID,
    0x1ce20aba, 0x9851, 0x4421, 0x94, 0x30, 0x1d, 0xde, 0xb7, 0x66, 0xe8, 0x09}
DEFINE_GUID!{DOMAIN_LEAVE_GUID,
    0xddaf516e, 0x58c2, 0x4866, 0x95, 0x74, 0xc3, 0xb6, 0x15, 0xd4, 0x2e, 0xa1}
DEFINE_GUID!{FIREWALL_PORT_OPEN_GUID,
    0xb7569e07, 0x8421, 0x4ee0, 0xad, 0x10, 0x86, 0x91, 0x5a, 0xfd, 0xad, 0x09}
DEFINE_GUID!{FIREWALL_PORT_CLOSE_GUID,
    0xa144ed38, 0x8e12, 0x4de4, 0x9d, 0x96, 0xe6, 0x47, 0x40, 0xb1, 0xa5, 0x24}
DEFINE_GUID!{MACHINE_POLICY_PRESENT_GUID,
    0x659fcae6, 0x5bdb, 0x4da9, 0xb1, 0xff, 0xca, 0x2a, 0x17, 0x8d, 0x46, 0xe0}
DEFINE_GUID!{USER_POLICY_PRESENT_GUID,
    0x54fb46c8, 0xf089, 0x464c, 0xb1, 0xfd, 0x59, 0xd1, 0xb6, 0x2c, 0x3b, 0x50}
DEFINE_GUID!{RPC_INTERFACE_EVENT_GUID,
    0xbc90d167, 0x9470, 0x4139, 0xa9, 0xba, 0xbe, 0x0b, 0xbb, 0xf5, 0xb7, 0x4d}
DEFINE_GUID!{NAMED_PIPE_EVENT_GUID,
    0x1f81d131, 0x3fac, 0x4537, 0x9e, 0x0c, 0x7e, 0x7b, 0x0c, 0x2f, 0x4b, 0x55}
DEFINE_GUID!{CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID,
    0x2d7a2816, 0x0c5e, 0x45fc, 0x9c, 0xe7, 0x57, 0x0e, 0x5e, 0xcd, 0xe9, 0xc9}
ENUM!{enum SC_ACTION_TYPE {
    SC_ACTION_NONE = 0,
    SC_ACTION_RESTART = 1,
    SC_ACTION_REBOOT = 2,
    SC_ACTION_RUN_COMMAND = 3,
}}
STRUCT!{struct SC_ACTION {
    Type: SC_ACTION_TYPE,
    Delay: DWORD,
}}
pub type LPSC_ACTION = *mut SC_ACTION;
STRUCT!{struct SERVICE_FAILURE_ACTIONSW {
    dwResetPeriod: DWORD,
    lpRebootMsg: LPWSTR,
    lpCommand: LPWSTR,
    cActions: DWORD,
    lpsaActions: LPSC_ACTION,
}}
pub type LPSERVICE_FAILURE_ACTIONSW = *mut SERVICE_FAILURE_ACTIONSW;
STRUCT!{struct SERVICE_FAILURE_ACTIONS_FLAG {
    fFailureActionsOnNonCrashFailures: BOOL,
}}
DECLARE_HANDLE!{SC_HANDLE, SC_HANDLE__}
pub type LPSC_HANDLE = *mut SC_HANDLE;
DECLARE_HANDLE!{SERVICE_STATUS_HANDLE, SERVICE_STATUS_HANDLE__}
ENUM!{enum SC_STATUS_TYPE {
    SC_STATUS_PROCESS_INFO = 0,
}}
ENUM!{enum SC_ENUM_TYPE {
    SC_ENUM_PROCESS_INFO = 0,
}}
STRUCT!{struct SERVICE_STATUS {
    dwServiceType: DWORD,
    dwCurrentState: DWORD,
    dwControlsAccepted: DWORD,
    dwWin32ExitCode: DWORD,
    dwServiceSpecificExitCode: DWORD,
    dwCheckPoint: DWORD,
    dwWaitHint: DWORD,
}}
pub type LPSERVICE_STATUS = *mut SERVICE_STATUS;
STRUCT!{struct SERVICE_STATUS_PROCESS {
    dwServiceType: DWORD,
    dwCurrentState: DWORD,
    dwControlsAccepted: DWORD,
    dwWin32ExitCode: DWORD,
    dwServiceSpecificExitCode: DWORD,
    dwCheckPoint: DWORD,
    dwWaitHint: DWORD,
    dwProcessId: DWORD,
    dwServiceFlags: DWORD,
}}
pub type LPSERVICE_STATUS_PROCESS = *mut SERVICE_STATUS_PROCESS;
STRUCT!{struct ENUM_SERVICE_STATUSA {
    lpServiceName: LPSTR,
    lpDisplayName: LPSTR,
    ServiceStatus: SERVICE_STATUS,
}}
pub type LPENUM_SERVICE_STATUSA = *mut ENUM_SERVICE_STATUSA;
STRUCT!{struct ENUM_SERVICE_STATUSW {
    lpServiceName: LPWSTR,
    lpDisplayName: LPWSTR,
    ServiceStatus: SERVICE_STATUS,
}}
pub type LPENUM_SERVICE_STATUSW = *mut ENUM_SERVICE_STATUSW;
STRUCT!{struct ENUM_SERVICE_STATUS_PROCESSA {
    lpServiceName: LPSTR,
    lpDisplayName: LPSTR,
    ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}}
pub type LPENUM_SERVICE_STATUS_PROCESSA = *mut ENUM_SERVICE_STATUS_PROCESSA;
STRUCT!{struct ENUM_SERVICE_STATUS_PROCESSW {
    lpServiceName: LPWSTR,
    lpDisplayName: LPWSTR,
    ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}}
pub type LPENUM_SERVICE_STATUS_PROCESSW = *mut ENUM_SERVICE_STATUS_PROCESSW;
pub type SC_LOCK = LPVOID;
STRUCT!{struct QUERY_SERVICE_LOCK_STATUSA {
    fIsLocked: DWORD,
    lpLockOwner: LPSTR,
    dwLockDuration: DWORD,
}}
pub type LPQUERY_SERVICE_LOCK_STATUSA = *mut QUERY_SERVICE_LOCK_STATUSA;
STRUCT!{struct QUERY_SERVICE_LOCK_STATUSW {
    fIsLocked: DWORD,
    lpLockOwner: LPWSTR,
    dwLockDuration: DWORD,
}}
pub type LPQUERY_SERVICE_LOCK_STATUSW = *mut QUERY_SERVICE_LOCK_STATUSW;
STRUCT!{struct QUERY_SERVICE_CONFIGA {
    dwServiceType: DWORD,
    dwStartType: DWORD,
    dwErrorControl: DWORD,
    lpBinaryPathName: LPSTR,
    lpLoadOrderGroup: LPSTR,
    dwTagId: DWORD,
    lpDependencies: LPSTR,
    lpServiceStartName: LPSTR,
    lpDisplayName: LPSTR,
}}
pub type LPQUERY_SERVICE_CONFIGA = *mut QUERY_SERVICE_CONFIGA;
STRUCT!{struct QUERY_SERVICE_CONFIGW {
    dwServiceType: DWORD,
    dwStartType: DWORD,
    dwErrorControl: DWORD,
    lpBinaryPathName: LPWSTR,
    lpLoadOrderGroup: LPWSTR,
    dwTagId: DWORD,
    lpDependencies: LPWSTR,
    lpServiceStartName: LPWSTR,
    lpDisplayName: LPWSTR,
}}
pub type LPQUERY_SERVICE_CONFIGW = *mut QUERY_SERVICE_CONFIGW;
STRUCT!{struct SERVICE_DESCRIPTIONA {
    lpDescription: LPSTR,
}}
pub type LPSERVICE_DESCRIPTIONA = *mut SERVICE_DESCRIPTIONA;
STRUCT!{struct SERVICE_DESCRIPTIONW {
    lpDescription: LPWSTR,
}}
pub type LPSERVICE_DESCRIPTIONW = *mut SERVICE_DESCRIPTIONW;
FN!{stdcall LPSERVICE_MAIN_FUNCTIONW(
    dwNumServicesArgs: DWORD,
    lpServiceArgVectors: *mut LPWSTR,
) -> ()}
FN!{stdcall LPSERVICE_MAIN_FUNCTIONA(
    dwNumServicesArgs: DWORD,
    lpServiceArgVectors: *mut LPSTR,
) -> ()}
STRUCT!{struct SERVICE_TABLE_ENTRYA {
    lpServiceName: LPCSTR,
    lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}}
pub type LPSERVICE_TABLE_ENTRYA = *mut SERVICE_TABLE_ENTRYA;
STRUCT!{struct SERVICE_TABLE_ENTRYW {
    lpServiceName: LPCWSTR,
    lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}}
pub type LPSERVICE_TABLE_ENTRYW = *mut SERVICE_TABLE_ENTRYW;
FN!{stdcall LPHANDLER_FUNCTION(
    dwControl: DWORD,
) -> ()}
FN!{stdcall LPHANDLER_FUNCTION_EX(
    dwControl: DWORD,
    dwEventType: DWORD,
    lpEventData: LPVOID,
    lpContext: LPVOID,
) -> DWORD}
FN!{stdcall PFN_SC_NOTIFY_CALLBACK(
    pParameter: PVOID,
) -> ()}
STRUCT!{struct SERVICE_NOTIFY_1 {
    dwVersion: DWORD,
    pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pContext: PVOID,
    dwNotificationStatus: DWORD,
    ServiceStatus: SERVICE_STATUS_PROCESS,
}}
pub type PSERVICE_NOTIFY_1 = *mut SERVICE_NOTIFY_1;
STRUCT!{struct SERVICE_NOTIFY_2A {
    dwVersion: DWORD,
    pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pContext: PVOID,
    dwNotificationStatus: DWORD,
    ServiceStatus: SERVICE_STATUS_PROCESS,
    dwNotificationTriggered: DWORD,
    pszServiceNames: LPSTR,
}}
pub type PSERVICE_NOTIFY_2A = *mut SERVICE_NOTIFY_2A;
STRUCT!{struct SERVICE_NOTIFY_2W {
    dwVersion: DWORD,
    pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pContext: PVOID,
    dwNotificationStatus: DWORD,
    ServiceStatus: SERVICE_STATUS_PROCESS,
    dwNotificationTriggered: DWORD,
    pszServiceNames: LPWSTR,
}}
pub type PSERVICE_NOTIFY_2W = *mut SERVICE_NOTIFY_2W;
pub type SERVICE_NOTIFYA = SERVICE_NOTIFY_2A;
pub type PSERVICE_NOTIFYA = PSERVICE_NOTIFY_2A;
pub type SERVICE_NOTIFYW = SERVICE_NOTIFY_2W;
pub type PSERVICE_NOTIFYW = PSERVICE_NOTIFY_2W;
extern "system" {
    pub fn ChangeServiceConfigA(
        hService: SC_HANDLE,
        dwServiceType: DWORD,
        dsStartType: DWORD,
        dwErrorControl: DWORD,
        lpBinaryPathName: LPCSTR,
        lpLoadOrderGroup: LPCSTR,
        lpdwTagId: LPDWORD,
        lpDependencies: LPCSTR,
        lpServiceStartName: LPCSTR,
        lpPassword: LPCSTR,
        lpDisplayName: LPCSTR,
    ) -> BOOL;
    pub fn ChangeServiceConfigW(
        hService: SC_HANDLE,
        dwServiceType: DWORD,
        dsStartType: DWORD,
        dwErrorControl: DWORD,
        lpBinaryPathName: LPCWSTR,
        lpLoadOrderGroup: LPCWSTR,
        lpdwTagId: LPDWORD,
        lpDependencies: LPCWSTR,
        lpServiceStartName: LPCWSTR,
        lpPassword: LPCWSTR,
        lpDisplayName: LPCWSTR,
    ) -> BOOL;
    pub fn ChangeServiceConfig2A(
        hService: SC_HANDLE,
        dwInfoLevel: DWORD,
        lpInfo: LPVOID,
    ) -> BOOL;
    pub fn ChangeServiceConfig2W(
        hService: SC_HANDLE,
        dwInfoLevel: DWORD,
        lpInfo: LPVOID,
    ) -> BOOL;
    pub fn CloseServiceHandle(
        hSCObject: SC_HANDLE,
    ) -> BOOL;
    pub fn ControlService(
        hService: SC_HANDLE,
        dwControl: DWORD,
        lpServiceStatus: LPSERVICE_STATUS,
    ) -> BOOL;
    pub fn CreateServiceA(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCSTR,
        lpDisplayName: LPCSTR,
        dwDesiredAccess: DWORD,
        dwServiceType: DWORD,
        dwStartType: DWORD,
        dwErrorControl: DWORD,
        lpBinaryPathName: LPCSTR,
        lpLoadOrderGroup: LPCSTR,
        lpdwTagId: LPDWORD,
        lpDependencies: LPCSTR,
        lpServiceStartName: LPCSTR,
        lpPassword: LPCSTR,
    ) -> SC_HANDLE;
    pub fn CreateServiceW(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCWSTR,
        lpDisplayName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwServiceType: DWORD,
        dwStartType: DWORD,
        dwErrorControl: DWORD,
        lpBinaryPathName: LPCWSTR,
        lpLoadOrderGroup: LPCWSTR,
        lpdwTagId: LPDWORD,
        lpDependencies: LPCWSTR,
        lpServiceStartName: LPCWSTR,
        lpPassword: LPCWSTR,
    ) -> SC_HANDLE;
    pub fn DeleteService(
        hService: SC_HANDLE,
    ) -> BOOL;
    pub fn EnumDependentServicesA(
        hService: SC_HANDLE,
        dwServiceState: DWORD,
        lpServices: LPENUM_SERVICE_STATUSA,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumDependentServicesW(
        hService: SC_HANDLE,
        dwServiceState: DWORD,
        lpServices: LPENUM_SERVICE_STATUSW,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
    ) -> BOOL;
    pub fn EnumServicesStatusA(
        hSCManager: SC_HANDLE,
        dwServiceType: DWORD,
        dwServiceState: DWORD,
        lpServices: LPENUM_SERVICE_STATUSA,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
        lpResumeHandle: LPDWORD,
    ) -> BOOL;
    pub fn EnumServicesStatusW(
        hSCManager: SC_HANDLE,
        dwServiceType: DWORD,
        dwServiceState: DWORD,
        lpServices: LPENUM_SERVICE_STATUSW,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
        lpResumeHandle: LPDWORD,
    ) -> BOOL;
    pub fn EnumServicesStatusExA(
        hSCManager: SC_HANDLE,
        InfoLevel: SC_ENUM_TYPE,
        dwServiceType: DWORD,
        dwServiceState: DWORD,
        lpServices: LPBYTE,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
        lpResumeHandle: LPDWORD,
        pszGroupName: LPCSTR,
    ) -> BOOL;
    pub fn EnumServicesStatusExW(
        hSCManager: SC_HANDLE,
        InfoLevel: SC_ENUM_TYPE,
        dwServiceType: DWORD,
        dwServiceState: DWORD,
        lpServices: LPBYTE,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
        lpServicesReturned: LPDWORD,
        lpResumeHandle: LPDWORD,
        pszGroupName: LPCWSTR,
    ) -> BOOL;
    pub fn GetServiceKeyNameA(
        hSCManager: SC_HANDLE,
        lpDisplayName: LPCSTR,
        lpServiceName: LPSTR,
        lpcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn GetServiceKeyNameW(
        hSCManager: SC_HANDLE,
        lpDisplayName: LPCWSTR,
        lpServiceName: LPWSTR,
        lpcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn GetServiceDisplayNameA(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCSTR,
        lpDisplayName: LPSTR,
        lpcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn GetServiceDisplayNameW(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCWSTR,
        lpDisplayName: LPWSTR,
        lpcchBuffer: LPDWORD,
    ) -> BOOL;
    pub fn LockServiceDatabase(
        hSCManager: SC_HANDLE,
    ) -> SC_LOCK;
    pub fn NotifyBootConfigStatus(
        BootAcceptable: BOOL,
    ) -> BOOL;
    pub fn OpenSCManagerA(
        lpMachineName: LPCSTR,
        lpDatabaseName: LPCSTR,
        dwDesiredAccess: DWORD,
    ) -> SC_HANDLE;
    pub fn OpenSCManagerW(
        lpMachineName: LPCWSTR,
        lpDatabaseName: LPCWSTR,
        dwDesiredAccess: DWORD,
    ) -> SC_HANDLE;
    pub fn OpenServiceA(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCSTR,
        dwDesiredAccess: DWORD,
    ) -> SC_HANDLE;
    pub fn OpenServiceW(
        hSCManager: SC_HANDLE,
        lpServiceName: LPCWSTR,
        dwDesiredAccess: DWORD,
    ) -> SC_HANDLE;
    pub fn QueryServiceConfigA(
        hService: SC_HANDLE,
        lpServiceConfig: LPQUERY_SERVICE_CONFIGA,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceConfigW(
        hService: SC_HANDLE,
        lpServiceConfig: LPQUERY_SERVICE_CONFIGW,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceConfig2A(
        hService: SC_HANDLE,
        dwInfoLevel: DWORD,
        lpBuffer: LPBYTE,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceConfig2W(
        hService: SC_HANDLE,
        dwInfoLevel: DWORD,
        lpBuffer: LPBYTE,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceLockStatusA(
        hSCManager: SC_HANDLE,
        lpLockStatus: LPQUERY_SERVICE_LOCK_STATUSA,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceLockStatusW(
        hSCManager: SC_HANDLE,
        lpLockStatus: LPQUERY_SERVICE_LOCK_STATUSW,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceObjectSecurity(
        hService: SC_HANDLE,
        dwSecurityInformation: SECURITY_INFORMATION,
        lpSecurityDescriptor: PSECURITY_DESCRIPTOR,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn QueryServiceStatus(
        hService: SC_HANDLE,
        lpServiceStatus: LPSERVICE_STATUS,
    ) -> BOOL;
    pub fn QueryServiceStatusEx(
        hService: SC_HANDLE,
        InfoLevel: SC_STATUS_TYPE,
        lpBuffer: LPBYTE,
        cbBufSize: DWORD,
        pcbBytesNeeded: LPDWORD,
    ) -> BOOL;
    pub fn RegisterServiceCtrlHandlerA(
        lpServiceName: LPCSTR,
        lpHandlerProc: LPHANDLER_FUNCTION,
    ) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerW(
        lpServiceName: LPCWSTR,
        lpHandlerProc: LPHANDLER_FUNCTION,
    ) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerExA(
        lpServiceName: LPCSTR,
        lpHandlerProc: LPHANDLER_FUNCTION_EX,
        lpContext: LPVOID,
    ) -> SERVICE_STATUS_HANDLE;
    pub fn RegisterServiceCtrlHandlerExW(
        lpServiceName: LPCWSTR,
        lpHandlerProc: LPHANDLER_FUNCTION_EX,
        lpContext: LPVOID,
    ) -> SERVICE_STATUS_HANDLE;
    pub fn SetServiceObjectSecurity(
        hService: SC_HANDLE,
        dwSecurityInformation: SECURITY_INFORMATION,
        lpSecurityDescriptor: PSECURITY_DESCRIPTOR,
    ) -> BOOL;
    pub fn SetServiceStatus(
        hServiceStatus: SERVICE_STATUS_HANDLE,
        lpServiceStatus: LPSERVICE_STATUS,
    ) -> BOOL;
    pub fn StartServiceCtrlDispatcherA(
        lpServiceStartTable: *const SERVICE_TABLE_ENTRYA,
    ) -> BOOL;
    pub fn StartServiceCtrlDispatcherW(
        lpServiceStartTable: *const SERVICE_TABLE_ENTRYW,
    ) -> BOOL;
    pub fn StartServiceA(
        hService: SC_HANDLE,
        dwNumServiceArgs: DWORD,
        lpServiceArgVectors: *mut LPCSTR,
    ) -> BOOL;
    pub fn StartServiceW(
        hService: SC_HANDLE,
        dwNumServiceArgs: DWORD,
        lpServiceArgVectors: *mut LPCWSTR,
    ) -> BOOL;
    pub fn UnlockServiceDatabase(
        ScLock: SC_LOCK,
    ) -> BOOL;
    pub fn NotifyServiceStatusChangeA(
        hService: SC_HANDLE,
        dwNotifyMask: DWORD,
        pNotifyBuffer: PSERVICE_NOTIFYA,
    ) -> DWORD;
    pub fn NotifyServiceStatusChangeW(
        hService: SC_HANDLE,
        dwNotifyMask: DWORD,
        pNotifyBuffer: PSERVICE_NOTIFYW,
    ) -> DWORD;
    pub fn ControlServiceExA(
        hService: SC_HANDLE,
        dwControl: DWORD,
        dwInfoLevel: DWORD,
        pControlParams: PVOID,
    ) -> BOOL;
    pub fn ControlServiceExW(
        hService: SC_HANDLE,
        dwControl: DWORD,
        dwInfoLevel: DWORD,
        pControlParams: PVOID,
    ) -> BOOL;
    pub fn QueryServiceDynamicInformation(
        hServiceStatus: SERVICE_STATUS_HANDLE,
        dwInfoLevel: DWORD,
        ppDynamicInfo: *mut PVOID,
    ) -> BOOL;
    pub fn WaitServiceState (
        hService: SC_HANDLE,
        dwNotify: DWORD,
        dwTimeout: DWORD,
        hCancelEvent: HANDLE,
    ) -> DWORD;
}
