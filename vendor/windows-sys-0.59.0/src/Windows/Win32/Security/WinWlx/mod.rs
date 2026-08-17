pub const STATUSMSG_OPTION_NOANIMATION: u32 = 1u32;
pub const STATUSMSG_OPTION_SETFOREGROUND: u32 = 2u32;
pub const WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREATE_INSTANCE_ONLY: u32 = 1u32;
pub const WLX_CREATE_USER: u32 = 2u32;
pub const WLX_CREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREDENTIAL_TYPE_V2_0: u32 = 2u32;
pub const WLX_CURRENT_VERSION: u32 = 65540u32;
pub const WLX_DESKTOP_HANDLE: u32 = 2u32;
pub const WLX_DESKTOP_NAME: u32 = 1u32;
pub const WLX_DIRECTORY_LENGTH: u32 = 256u32;
pub const WLX_DLG_INPUT_TIMEOUT: u32 = 102u32;
pub const WLX_DLG_SAS: u32 = 101u32;
pub const WLX_DLG_SCREEN_SAVER_TIMEOUT: u32 = 103u32;
pub const WLX_DLG_USER_LOGOFF: u32 = 104u32;
pub const WLX_LOGON_OPT_NO_PROFILE: u32 = 1u32;
pub const WLX_OPTION_CONTEXT_POINTER: u32 = 2u32;
pub const WLX_OPTION_DISPATCH_TABLE_SIZE: u32 = 65539u32;
pub const WLX_OPTION_FORCE_LOGOFF_TIME: u32 = 4u32;
pub const WLX_OPTION_IGNORE_AUTO_LOGON: u32 = 8u32;
pub const WLX_OPTION_NO_SWITCH_ON_SAS: u32 = 9u32;
pub const WLX_OPTION_SMART_CARD_INFO: u32 = 65538u32;
pub const WLX_OPTION_SMART_CARD_PRESENT: u32 = 65537u32;
pub const WLX_OPTION_USE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_OPTION_USE_SMART_CARD: u32 = 3u32;
pub const WLX_PROFILE_TYPE_V1_0: u32 = 1u32;
pub const WLX_PROFILE_TYPE_V2_0: u32 = 2u32;
pub const WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF: u32 = 16u32;
pub const WLX_SAS_ACTION_FORCE_LOGOFF: u32 = 9u32;
pub const WLX_SAS_ACTION_LOCK_WKSTA: u32 = 3u32;
pub const WLX_SAS_ACTION_LOGOFF: u32 = 4u32;
pub const WLX_SAS_ACTION_LOGON: u32 = 1u32;
pub const WLX_SAS_ACTION_NONE: u32 = 2u32;
pub const WLX_SAS_ACTION_PWD_CHANGED: u32 = 6u32;
pub const WLX_SAS_ACTION_RECONNECTED: u32 = 15u32;
pub const WLX_SAS_ACTION_SHUTDOWN: WLX_SHUTDOWN_TYPE = 5u32;
pub const WLX_SAS_ACTION_SHUTDOWN_HIBERNATE: u32 = 14u32;
pub const WLX_SAS_ACTION_SHUTDOWN_POWER_OFF: WLX_SHUTDOWN_TYPE = 10u32;
pub const WLX_SAS_ACTION_SHUTDOWN_REBOOT: WLX_SHUTDOWN_TYPE = 11u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP: u32 = 12u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP2: u32 = 13u32;
pub const WLX_SAS_ACTION_SWITCH_CONSOLE: u32 = 17u32;
pub const WLX_SAS_ACTION_TASKLIST: u32 = 7u32;
pub const WLX_SAS_ACTION_UNLOCK_WKSTA: u32 = 8u32;
pub const WLX_SAS_TYPE_AUTHENTICATED: u32 = 7u32;
pub const WLX_SAS_TYPE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_SAS_TYPE_MAX_MSFT_VALUE: u32 = 127u32;
pub const WLX_SAS_TYPE_SCRNSVR_ACTIVITY: u32 = 3u32;
pub const WLX_SAS_TYPE_SCRNSVR_TIMEOUT: u32 = 2u32;
pub const WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED: u32 = 8u32;
pub const WLX_SAS_TYPE_SC_INSERT: u32 = 5u32;
pub const WLX_SAS_TYPE_SC_LAST_READER_REMOVED: u32 = 9u32;
pub const WLX_SAS_TYPE_SC_REMOVE: u32 = 6u32;
pub const WLX_SAS_TYPE_SWITCHUSER: u32 = 10u32;
pub const WLX_SAS_TYPE_TIMEOUT: u32 = 0u32;
pub const WLX_SAS_TYPE_USER_LOGOFF: u32 = 4u32;
pub const WLX_VERSION_1_0: u32 = 65536u32;
pub const WLX_VERSION_1_1: u32 = 65537u32;
pub const WLX_VERSION_1_2: u32 = 65538u32;
pub const WLX_VERSION_1_3: u32 = 65539u32;
pub const WLX_VERSION_1_4: u32 = 65540u32;
pub const WLX_WM_SAS: u32 = 1625u32;
pub type WLX_SHUTDOWN_TYPE = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub pszUserName: windows_sys::core::PWSTR,
    pub pszDomain: windows_sys::core::PWSTR,
    pub pszPassword: windows_sys::core::PWSTR,
    pub fPromptForPassword: super::super::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V2_0 {
    pub dwType: u32,
    pub pszUserName: windows_sys::core::PWSTR,
    pub pszDomain: windows_sys::core::PWSTR,
    pub pszPassword: windows_sys::core::PWSTR,
    pub fPromptForPassword: super::super::Foundation::BOOL,
    pub fDisconnectOnLogonFailure: super::super::Foundation::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0 {
    pub dwType: u32,
    pub UserToken: super::super::Foundation::HANDLE,
    pub LogonId: super::super::Foundation::LUID,
    pub Quotas: super::QUOTA_LIMITS,
    pub UserName: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub LogonTime: i64,
    pub SmartCardLogon: super::super::Foundation::BOOL,
    pub ProfileLength: u32,
    pub MessageType: u32,
    pub LogonCount: u16,
    pub BadPasswordCount: u16,
    pub ProfileLogonTime: i64,
    pub LogoffTime: i64,
    pub KickOffTime: i64,
    pub PasswordLastSet: i64,
    pub PasswordCanChange: i64,
    pub PasswordMustChange: i64,
    pub LogonScript: windows_sys::core::PWSTR,
    pub HomeDirectory: windows_sys::core::PWSTR,
    pub FullName: windows_sys::core::PWSTR,
    pub ProfilePath: windows_sys::core::PWSTR,
    pub HomeDirectoryDrive: windows_sys::core::PWSTR,
    pub LogonServer: windows_sys::core::PWSTR,
    pub UserFlags: u32,
    pub PrivateDataLen: u32,
    pub PrivateData: *mut u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_StationsAndDesktops")]
#[derive(Clone, Copy)]
pub struct WLX_DESKTOP {
    pub Size: u32,
    pub Flags: u32,
    pub hDesktop: super::super::System::StationsAndDesktops::HDESK,
    pub pszDesktopName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy)]
pub struct WLX_DISPATCH_VERSION_1_0 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct WLX_DISPATCH_VERSION_1_1 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct WLX_DISPATCH_VERSION_1_2 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct WLX_DISPATCH_VERSION_1_3 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
    pub WlxSetOption: PWLX_SET_OPTION,
    pub WlxGetOption: PWLX_GET_OPTION,
    pub WlxWin31Migrate: PWLX_WIN31_MIGRATE,
    pub WlxQueryClientCredentials: PWLX_QUERY_CLIENT_CREDENTIALS,
    pub WlxQueryInetConnectorCredentials: PWLX_QUERY_IC_CREDENTIALS,
    pub WlxDisconnect: PWLX_DISCONNECT,
    pub WlxQueryTerminalServicesData: PWLX_QUERY_TERMINAL_SERVICES_DATA,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct WLX_DISPATCH_VERSION_1_4 {
    pub WlxUseCtrlAltDel: PWLX_USE_CTRL_ALT_DEL,
    pub WlxSetContextPointer: PWLX_SET_CONTEXT_POINTER,
    pub WlxSasNotify: PWLX_SAS_NOTIFY,
    pub WlxSetTimeout: PWLX_SET_TIMEOUT,
    pub WlxAssignShellProtection: PWLX_ASSIGN_SHELL_PROTECTION,
    pub WlxMessageBox: PWLX_MESSAGE_BOX,
    pub WlxDialogBox: PWLX_DIALOG_BOX,
    pub WlxDialogBoxParam: PWLX_DIALOG_BOX_PARAM,
    pub WlxDialogBoxIndirect: PWLX_DIALOG_BOX_INDIRECT,
    pub WlxDialogBoxIndirectParam: PWLX_DIALOG_BOX_INDIRECT_PARAM,
    pub WlxSwitchDesktopToUser: PWLX_SWITCH_DESKTOP_TO_USER,
    pub WlxSwitchDesktopToWinlogon: PWLX_SWITCH_DESKTOP_TO_WINLOGON,
    pub WlxChangePasswordNotify: PWLX_CHANGE_PASSWORD_NOTIFY,
    pub WlxGetSourceDesktop: PWLX_GET_SOURCE_DESKTOP,
    pub WlxSetReturnDesktop: PWLX_SET_RETURN_DESKTOP,
    pub WlxCreateUserDesktop: PWLX_CREATE_USER_DESKTOP,
    pub WlxChangePasswordNotifyEx: PWLX_CHANGE_PASSWORD_NOTIFY_EX,
    pub WlxCloseUserDesktop: PWLX_CLOSE_USER_DESKTOP,
    pub WlxSetOption: PWLX_SET_OPTION,
    pub WlxGetOption: PWLX_GET_OPTION,
    pub WlxWin31Migrate: PWLX_WIN31_MIGRATE,
    pub WlxQueryClientCredentials: PWLX_QUERY_CLIENT_CREDENTIALS,
    pub WlxQueryInetConnectorCredentials: PWLX_QUERY_IC_CREDENTIALS,
    pub WlxDisconnect: PWLX_DISCONNECT,
    pub WlxQueryTerminalServicesData: PWLX_QUERY_TERMINAL_SERVICES_DATA,
    pub WlxQueryConsoleSwitchCredentials: PWLX_QUERY_CONSOLESWITCH_CREDENTIALS,
    pub WlxQueryTsLogonCredentials: PWLX_QUERY_TS_LOGON_CREDENTIALS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_MPR_NOTIFY_INFO {
    pub pszUserName: windows_sys::core::PWSTR,
    pub pszDomain: windows_sys::core::PWSTR,
    pub pszPassword: windows_sys::core::PWSTR,
    pub pszOldPassword: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_StationsAndDesktops")]
#[derive(Clone, Copy)]
pub struct WLX_NOTIFICATION_INFO {
    pub Size: u32,
    pub Flags: u32,
    pub UserName: windows_sys::core::PWSTR,
    pub Domain: windows_sys::core::PWSTR,
    pub WindowStation: windows_sys::core::PWSTR,
    pub hToken: super::super::Foundation::HANDLE,
    pub hDesktop: super::super::System::StationsAndDesktops::HDESK,
    pub pStatusCallback: PFNMSGECALLBACK,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_PROFILE_V1_0 {
    pub dwType: u32,
    pub pszProfile: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_PROFILE_V2_0 {
    pub dwType: u32,
    pub pszProfile: windows_sys::core::PWSTR,
    pub pszPolicy: windows_sys::core::PWSTR,
    pub pszNetworkDefaultUserProfile: windows_sys::core::PWSTR,
    pub pszServerName: windows_sys::core::PWSTR,
    pub pszEnvironment: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_SC_NOTIFICATION_INFO {
    pub pszCard: windows_sys::core::PWSTR,
    pub pszReader: windows_sys::core::PWSTR,
    pub pszContainer: windows_sys::core::PWSTR,
    pub pszCryptoProvider: windows_sys::core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLX_TERMINAL_SERVICES_DATA {
    pub ProfilePath: [u16; 257],
    pub HomeDir: [u16; 257],
    pub HomeDirDrive: [u16; 4],
}
pub type PFNMSGECALLBACK = Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: windows_sys::core::PCWSTR) -> u32>;
pub type PWLX_ASSIGN_SHELL_PROTECTION = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_CHANGE_PASSWORD_NOTIFY = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32) -> i32>;
pub type PWLX_CHANGE_PASSWORD_NOTIFY_EX = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32, providername: windows_sys::core::PCWSTR, reserved: *mut core::ffi::c_void) -> i32>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_CLOSE_USER_DESKTOP = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP, htoken: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_CREATE_USER_DESKTOP = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, flags: u32, pszdesktopname: windows_sys::core::PCWSTR, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: windows_sys::core::PCWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_INDIRECT = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_INDIRECT_PARAM = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PWLX_DIALOG_BOX_PARAM = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: windows_sys::core::PCWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32>;
pub type PWLX_DISCONNECT = Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type PWLX_GET_OPTION = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: *mut usize) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_GET_SOURCE_DESKTOP = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
pub type PWLX_MESSAGE_BOX = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hwndowner: super::super::Foundation::HWND, lpsztext: windows_sys::core::PCWSTR, lpsztitle: windows_sys::core::PCWSTR, fustyle: u32) -> i32>;
pub type PWLX_QUERY_CLIENT_CREDENTIALS = Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL>;
pub type PWLX_QUERY_CONSOLESWITCH_CREDENTIALS = Option<unsafe extern "system" fn(pcred: *mut WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0) -> u32>;
pub type PWLX_QUERY_IC_CREDENTIALS = Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL>;
pub type PWLX_QUERY_TERMINAL_SERVICES_DATA = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ptsdata: *mut WLX_TERMINAL_SERVICES_DATA, username: windows_sys::core::PCWSTR, domain: windows_sys::core::PCWSTR) -> u32>;
pub type PWLX_QUERY_TS_LOGON_CREDENTIALS = Option<unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V2_0) -> super::super::Foundation::BOOL>;
pub type PWLX_SAS_NOTIFY = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, dwsastype: u32)>;
pub type PWLX_SET_CONTEXT_POINTER = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pwlxcontext: *mut core::ffi::c_void)>;
pub type PWLX_SET_OPTION = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: usize, oldvalue: *mut usize) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_System_StationsAndDesktops")]
pub type PWLX_SET_RETURN_DESKTOP = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP) -> super::super::Foundation::BOOL>;
pub type PWLX_SET_TIMEOUT = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, timeout: u32) -> super::super::Foundation::BOOL>;
pub type PWLX_SWITCH_DESKTOP_TO_USER = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_SWITCH_DESKTOP_TO_WINLOGON = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32>;
pub type PWLX_USE_CTRL_ALT_DEL = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE)>;
pub type PWLX_WIN31_MIGRATE = Option<unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE)>;
