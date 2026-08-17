windows_link::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::super::Foundation:: HANDLE));
windows_link::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : u32, targetlogonid : u32, ppassword : windows_sys::core::PCSTR, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : u32, targetlogonid : u32, ppassword : windows_sys::core::PCWSTR, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCSTR, pbuffer : *const WTSLISTENERCONFIGA, flag : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCWSTR, pbuffer : *const WTSLISTENERCONFIGW, flag : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnableChildSessions(benable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut i8, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut u16, pcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOA, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_sys::core::PSTR, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_sys::core::PWSTR, pcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOW, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : windows_sys::core::PCSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOA, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : windows_sys::core::PCWSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOW, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOA, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1A, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1W, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOW, pcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : *mut core::ffi::c_void));
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> u32);
windows_link::link!("wtsapi32.dll" "system" fn WTSGetChildSessionId(psessionid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSIsChildSessionsEnabled(pbenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : windows_sys::core::PCWSTR) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCSTR, pbuffer : *mut WTSLISTENERCONFIGA) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCWSTR, pbuffer : *mut WTSLISTENERCONFIGW) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_sys::core::PSTR, pbytesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_sys::core::PWSTR, pbytesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : windows_sys::core::PCSTR, pusername : windows_sys::core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_sys::core::PSTR, pbytesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : windows_sys::core::PCWSTR, pusername : windows_sys::core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_sys::core::PWSTR, pbytesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : u32, phtoken : *mut super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::super::Foundation:: HWND, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_sys::core::PCSTR, titlelength : u32, pmessage : windows_sys::core::PCSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_sys::core::PCWSTR, titlelength : u32, pmessage : windows_sys::core::PCWSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_sys::core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSSetRenderHint(prenderhintid : *mut u64, hwndowner : super::super::Foundation:: HWND, renderhinttype : u32, cbhintdatalength : u32, phintdata : *const u8) -> windows_sys::core::HRESULT);
windows_link::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : windows_sys::core::PCSTR, pusername : windows_sys::core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_sys::core::PCSTR, datalength : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : windows_sys::core::PCWSTR, pusername : windows_sys::core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_sys::core::PCWSTR, datalength : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::super::Foundation:: HANDLE, shutdownflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : windows_sys::core::PCSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : windows_sys::core::PCWSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::super::Foundation:: HANDLE, processid : u32, exitcode : u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::super::Foundation:: HWND) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::super::Foundation:: HANDLE, sessionid : u32, pvirtualname : windows_sys::core::PCSTR) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : u32, pvirtualname : windows_sys::core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::super::Foundation:: HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut *mut core::ffi::c_void, pbytesreturned : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::super::Foundation:: HANDLE, timeout : u32, buffer : windows_sys::core::PSTR, buffersize : u32, pbytesread : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::super::Foundation:: HANDLE, buffer : windows_sys::core::PCSTR, length : u32, pbyteswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::super::Foundation:: HANDLE, eventmask : u32, peventflags : *mut u32) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AAAccountingData {
    pub userName: windows_sys::core::BSTR,
    pub clientName: windows_sys::core::BSTR,
    pub authType: AAAuthSchemes,
    pub resourceName: windows_sys::core::BSTR,
    pub portNumber: i32,
    pub protocolName: windows_sys::core::BSTR,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: windows_sys::core::BSTR,
    pub mainSessionId: windows_sys::core::GUID,
    pub subSessionId: i32,
}
impl Default for AAAccountingData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type AAAccountingDataType = i32;
pub type AAAuthSchemes = i32;
pub type AATrustClassID = i32;
pub const AA_AUTH_ANY: AAAuthSchemes = 6i32;
pub const AA_AUTH_BASIC: AAAuthSchemes = 1i32;
pub const AA_AUTH_CONID: AAAuthSchemes = 10i32;
pub const AA_AUTH_COOKIE: AAAuthSchemes = 7i32;
pub const AA_AUTH_DIGEST: AAAuthSchemes = 8i32;
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = 4i32;
pub const AA_AUTH_MAX: AAAuthSchemes = 12i32;
pub const AA_AUTH_MIN: AAAuthSchemes = 0i32;
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = 5i32;
pub const AA_AUTH_NTLM: AAAuthSchemes = 2i32;
pub const AA_AUTH_ORGID: AAAuthSchemes = 9i32;
pub const AA_AUTH_SC: AAAuthSchemes = 3i32;
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = 11i32;
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = 3i32;
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = 0i32;
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = 2i32;
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = 1i32;
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = 2i32;
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = 1i32;
pub const AA_UNTRUSTED: AATrustClassID = 0i32;
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe2e9cae6_1e7b_4b8e_babd_e9bf6292ac29);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
pub type AE_POSITION_FLAGS = i32;
pub const AllowOnlySDRServers: PolicyAttributeType = 7i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CHANNEL_DEF {
    pub name: [i8; 8],
    pub options: u32,
}
impl Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
pub const CHANNEL_EVENT_CONNECTED: u32 = 1u32;
pub const CHANNEL_EVENT_DATA_RECEIVED: u32 = 10u32;
pub const CHANNEL_EVENT_DISCONNECTED: u32 = 3u32;
pub const CHANNEL_EVENT_INITIALIZED: u32 = 0u32;
pub const CHANNEL_EVENT_TERMINATED: u32 = 4u32;
pub const CHANNEL_EVENT_V1_CONNECTED: u32 = 2u32;
pub const CHANNEL_EVENT_WRITE_CANCELLED: u32 = 12u32;
pub const CHANNEL_EVENT_WRITE_COMPLETE: u32 = 11u32;
pub const CHANNEL_FLAG_FAIL: u32 = 256u32;
pub const CHANNEL_FLAG_FIRST: u32 = 1u32;
pub const CHANNEL_FLAG_LAST: u32 = 2u32;
pub const CHANNEL_FLAG_MIDDLE: u32 = 0u32;
pub const CHANNEL_MAX_COUNT: u32 = 30u32;
pub const CHANNEL_NAME_LEN: u32 = 7u32;
pub const CHANNEL_OPTION_COMPRESS: u32 = 4194304u32;
pub const CHANNEL_OPTION_COMPRESS_RDP: u32 = 8388608u32;
pub const CHANNEL_OPTION_ENCRYPT_CS: u32 = 268435456u32;
pub const CHANNEL_OPTION_ENCRYPT_RDP: u32 = 1073741824u32;
pub const CHANNEL_OPTION_ENCRYPT_SC: u32 = 536870912u32;
pub const CHANNEL_OPTION_INITIALIZED: u32 = 2147483648u32;
pub const CHANNEL_OPTION_PRI_HIGH: u32 = 134217728u32;
pub const CHANNEL_OPTION_PRI_LOW: u32 = 33554432u32;
pub const CHANNEL_OPTION_PRI_MED: u32 = 67108864u32;
pub const CHANNEL_OPTION_REMOTE_CONTROL_PERSISTENT: u32 = 1048576u32;
pub const CHANNEL_OPTION_SHOW_PROTOCOL: u32 = 2097152u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
pub const CHANNEL_RC_ALREADY_CONNECTED: u32 = 3u32;
pub const CHANNEL_RC_ALREADY_INITIALIZED: u32 = 1u32;
pub const CHANNEL_RC_ALREADY_OPEN: u32 = 14u32;
pub const CHANNEL_RC_BAD_CHANNEL: u32 = 6u32;
pub const CHANNEL_RC_BAD_CHANNEL_HANDLE: u32 = 7u32;
pub const CHANNEL_RC_BAD_INIT_HANDLE: u32 = 9u32;
pub const CHANNEL_RC_BAD_PROC: u32 = 11u32;
pub const CHANNEL_RC_INITIALIZATION_ERROR: u32 = 20u32;
pub const CHANNEL_RC_INVALID_INSTANCE: u32 = 18u32;
pub const CHANNEL_RC_NOT_CONNECTED: u32 = 4u32;
pub const CHANNEL_RC_NOT_INITIALIZED: u32 = 2u32;
pub const CHANNEL_RC_NOT_IN_VIRTUALCHANNELENTRY: u32 = 15u32;
pub const CHANNEL_RC_NOT_OPEN: u32 = 10u32;
pub const CHANNEL_RC_NO_BUFFER: u32 = 8u32;
pub const CHANNEL_RC_NO_MEMORY: u32 = 12u32;
pub const CHANNEL_RC_NULL_DATA: u32 = 16u32;
pub const CHANNEL_RC_OK: u32 = 0u32;
pub const CHANNEL_RC_TOO_MANY_CHANNELS: u32 = 5u32;
pub const CHANNEL_RC_UNKNOWN_CHANNEL_NAME: u32 = 13u32;
pub const CHANNEL_RC_UNSUPPORTED_VERSION: u32 = 19u32;
pub const CHANNEL_RC_ZERO_LENGTH: u32 = 17u32;
pub const CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = 2i32;
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = 0i32;
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = 1i32;
pub type CLIENT_MESSAGE_TYPE = i32;
pub type CONNECTION_CHANGE_NOTIFICATION = i32;
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b150580_fea4_4d3c_9de4_7433a66618f7);
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x693f7ff5_0c4e_4d17_b8e0_1f70325e5d58);
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = 5i32;
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = 2i32;
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = 0i32;
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 6i32;
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 8i32;
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = 1i32;
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = 7i32;
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = 4i32;
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = 3i32;
pub const ClipboardRedirectionDisabled: PolicyAttributeType = 5i32;
pub const DISPID_AX_ADMINMESSAGERECEIVED: u32 = 760u32;
pub const DISPID_AX_AUTORECONNECTED: u32 = 756u32;
pub const DISPID_AX_AUTORECONNECTING: u32 = 755u32;
pub const DISPID_AX_CONNECTED: u32 = 751u32;
pub const DISPID_AX_CONNECTING: u32 = 750u32;
pub const DISPID_AX_DIALOGDISMISSED: u32 = 758u32;
pub const DISPID_AX_DIALOGDISPLAYING: u32 = 757u32;
pub const DISPID_AX_DISCONNECTED: u32 = 753u32;
pub const DISPID_AX_KEYCOMBINATIONPRESSED: u32 = 761u32;
pub const DISPID_AX_LOGINCOMPLETED: u32 = 752u32;
pub const DISPID_AX_NETWORKSTATUSCHANGED: u32 = 759u32;
pub const DISPID_AX_REMOTEDESKTOPSIZECHANGED: u32 = 762u32;
pub const DISPID_AX_STATUSCHANGED: u32 = 754u32;
pub const DISPID_AX_TOUCHPOINTERCURSORMOVED: u32 = 800u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_APPLY_SETTINGS: u32 = 722u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_ATTACH_EVENT: u32 = 706u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_CONNECT: u32 = 701u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DELETE_SAVED_CREDENTIALS: u32 = 704u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DETACH_EVENT: u32 = 707u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_DISCONNECT: u32 = 702u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_EXECUTE_REMOTE_ACTION: u32 = 732u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_RDPPROPERTY: u32 = 721u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_GET_SNAPSHOT: u32 = 733u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RECONNECT: u32 = 703u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RESUME_SCREEN_UPDATES: u32 = 731u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_RETRIEVE_SETTINGS: u32 = 723u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SET_RDPPROPERTY: u32 = 720u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_SUSPEND_SCREEN_UPDATES: u32 = 730u32;
pub const DISPID_METHOD_REMOTEDESKTOPCLIENT_UPDATE_SESSION_DISPLAYSETTINGS: u32 = 705u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_ACTIONS: u32 = 711u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_SETTINGS: u32 = 710u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_ENABLED: u32 = 740u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_EVENTSENABLED: u32 = 741u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCHPOINTER_POINTERSPEED: u32 = 742u32;
pub const DISPID_PROP_REMOTEDESKTOPCLIENT_TOUCH_POINTER: u32 = 712u32;
pub const DOMAIN_LENGTH: u32 = 17u32;
pub const DisableAllRedirections: PolicyAttributeType = 1i32;
pub const DriveRedirectionDisabled: PolicyAttributeType = 2i32;
pub const EnableAllRedirections: PolicyAttributeType = 0i32;
pub const FARM: TARGET_TYPE = 1i32;
pub const FORCE_REJOIN: u32 = 2u32;
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub const KeyCombinationDown: KeyCombinationType = 4i32;
pub const KeyCombinationHome: KeyCombinationType = 0i32;
pub const KeyCombinationLeft: KeyCombinationType = 1i32;
pub const KeyCombinationRight: KeyCombinationType = 3i32;
pub const KeyCombinationScroll: KeyCombinationType = 5i32;
pub type KeyCombinationType = i32;
pub const KeyCombinationUp: KeyCombinationType = 2i32;
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = 4i32;
pub const MAX_DATE_TIME_LENGTH: u32 = 56u32;
pub const MAX_ELAPSED_TIME_LENGTH: u32 = 15u32;
pub const MAX_POLICY_ATTRIBUTES: u32 = 20u32;
pub const MaxAppName_Len: u32 = 256u32;
pub const MaxDomainName_Len: u32 = 256u32;
pub const MaxFQDN_Len: u32 = 256u32;
pub const MaxFarm_Len: u32 = 256u32;
pub const MaxNetBiosName_Len: u32 = 16u32;
pub const MaxNumOfExposed_IPs: u32 = 12u32;
pub const MaxUserName_Len: u32 = 104u32;
pub const NONFARM: TARGET_TYPE = 2i32;
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = 16i32;
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = 1i32;
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = 2i32;
pub const OWNER_UNKNOWN: TARGET_OWNER = 0i32;
pub type PCHANNEL_INIT_EVENT_FN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, event: u32, pdata: *mut core::ffi::c_void, datalength: u32)>;
pub type PCHANNEL_OPEN_EVENT_FN = Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32)>;
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = 8i32;
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
pub type PLUGIN_TYPE = i32;
pub const POLICY_PLUGIN: PLUGIN_TYPE = 1i32;
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = 2i32;
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = 1i32;
pub const POSITION_INVALID: AE_POSITION_FLAGS = 0i32;
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = 4i32;
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRODUCT_INFOA {
    pub CompanyName: [i8; 256],
    pub ProductID: [i8; 4],
}
impl Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0cdfd28e_d0b9_4c1f_a5eb_6d1f6c6535b9);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xed2c3fda_338d_4d3f_81a3_e767310d908e);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6212d757_0043_4862_99c3_9f3059ac2a3b);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x197c427a_0135_4b6d_9c5e_e6579a0ab625);
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = 32i32;
pub type PVIRTUALCHANNELCLOSE = Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
pub type PVIRTUALCHANNELENTRY = Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> windows_sys::core::BOOL>;
pub type PVIRTUALCHANNELINIT = Option<unsafe extern "system" fn(ppinithandle: *mut *mut core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELOPEN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, popenhandle: *mut u32, pchannelname: windows_sys::core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELWRITE = Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut core::ffi::c_void, datalength: u32, puserdata: *mut core::ffi::c_void) -> u32>;
pub type PasswordEncodingType = i32;
pub const PasswordEncodingUTF16BE: PasswordEncodingType = 2i32;
pub const PasswordEncodingUTF16LE: PasswordEncodingType = 1i32;
pub const PasswordEncodingUTF8: PasswordEncodingType = 0i32;
pub const PnpRedirectionDisabled: PolicyAttributeType = 6i32;
pub type PolicyAttributeType = i32;
pub const PortRedirectionDisabled: PolicyAttributeType = 4i32;
pub const PrinterRedirectionDisabled: PolicyAttributeType = 3i32;
pub const RDCLIENT_BITMAP_RENDER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
pub type RDV_TASK_STATUS = i32;
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = 3i32;
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = 2i32;
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = 7i32;
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = 5i32;
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = 4i32;
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = 1i32;
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = 6i32;
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = 8i32;
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = 0i32;
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = 5i32;
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = 3i32;
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = 4i32;
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = 2i32;
pub const RD_FARM_RDSH: RD_FARM_TYPE = 0i32;
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = 1i32;
pub type RD_FARM_TYPE = i32;
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = -1i32;
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
pub const RENDER_HINT_CLEAR: u32 = 0u32;
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
pub const RENDER_HINT_VIDEO: u32 = 1u32;
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = 2i32;
pub const RFX_CLIENT_ID_LENGTH: u32 = 32u32;
pub const RFX_GFX_MAX_SUPPORTED_MONITORS: u32 = 16u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub reserved: u32,
    pub monitorCount: u32,
    pub MonitorData: [RFX_GFX_MONITOR_INFO; 16],
    pub clientUniqueId: [u16; 32],
}
impl Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
pub const RemoteActionAppSwitch: RemoteActionType = 4i32;
pub const RemoteActionAppbar: RemoteActionType = 1i32;
pub const RemoteActionCharms: RemoteActionType = 0i32;
pub const RemoteActionSnap: RemoteActionType = 2i32;
pub const RemoteActionStartScreen: RemoteActionType = 3i32;
pub type RemoteActionType = i32;
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = 0i32;
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = 1i32;
pub type SESSION_TIMEOUT_ACTION_TYPE = i32;
pub const SINGLE_SESSION: u32 = 1u32;
pub const STATE_ACTIVE: TSSESSION_STATE = 0i32;
pub const STATE_CONNECTED: TSSESSION_STATE = 1i32;
pub const STATE_CONNECTQUERY: TSSESSION_STATE = 2i32;
pub const STATE_DISCONNECTED: TSSESSION_STATE = 4i32;
pub const STATE_DOWN: TSSESSION_STATE = 8i32;
pub const STATE_IDLE: TSSESSION_STATE = 5i32;
pub const STATE_INIT: TSSESSION_STATE = 9i32;
pub const STATE_INVALID: TSSESSION_STATE = -1i32;
pub const STATE_LISTEN: TSSESSION_STATE = 6i32;
pub const STATE_MAX: TSSESSION_STATE = 10i32;
pub const STATE_RESET: TSSESSION_STATE = 7i32;
pub const STATE_SHADOW: TSSESSION_STATE = 3i32;
pub const SnapshotEncodingDataUri: SnapshotEncodingType = 0i32;
pub type SnapshotEncodingType = i32;
pub const SnapshotFormatBmp: SnapshotFormatType = 2i32;
pub const SnapshotFormatJpeg: SnapshotFormatType = 1i32;
pub const SnapshotFormatPng: SnapshotFormatType = 0i32;
pub type SnapshotFormatType = i32;
pub type TARGET_CHANGE_TYPE = i32;
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = 1i32;
pub const TARGET_CHECKED_OUT: TARGET_STATE = 6i32;
pub const TARGET_DOWN: TARGET_STATE = 4i32;
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = 2i32;
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = 1024i32;
pub const TARGET_HIBERNATED: TARGET_STATE = 5i32;
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = 64i32;
pub const TARGET_INITIALIZING: TARGET_STATE = 2i32;
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = 4i32;
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = 256i32;
pub const TARGET_INVALID: TARGET_STATE = 8i32;
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = 8i32;
pub const TARGET_MAXSTATE: TARGET_STATE = 11i32;
pub type TARGET_OWNER = i32;
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = 3i32;
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = 4i32;
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = 2i32;
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = 1i32;
pub type TARGET_PATCH_STATE = i32;
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = 512i32;
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = 0i32;
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = 128i32;
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = 16i32;
pub const TARGET_RUNNING: TARGET_STATE = 3i32;
pub const TARGET_STARTING: TARGET_STATE = 9i32;
pub type TARGET_STATE = i32;
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = 32i32;
pub const TARGET_STOPPED: TARGET_STATE = 7i32;
pub const TARGET_STOPPING: TARGET_STATE = 10i32;
pub type TARGET_TYPE = i32;
pub const TARGET_UNKNOWN: TARGET_STATE = 1i32;
pub const TASK_PLUGIN: PLUGIN_TYPE = 64i32;
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = 1i32;
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = 0i32;
pub type TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = i32;
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = 1i32;
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = 0i32;
pub type TSPUB_PLUGIN_PD_RESOLUTION_TYPE = i32;
pub type TSSB_NOTIFICATION_TYPE = i32;
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = 4i32;
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = 0i32;
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = 2i32;
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = 1i32;
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = 4i32;
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = 6i32;
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = 0i32;
pub type TSSD_AddrV46Type = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TSSESSION_STATE = i32;
pub const TSUserExInterfaces: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0910dd01_df8c_11d1_ae27_00c04fa35813);
pub type TS_SB_SORT_BY = i32;
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = 1i32;
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = 0i32;
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = 2i32;
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
pub const UNKNOWN: TARGET_TYPE = 0i32;
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = 0i32;
pub const USERNAME_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
pub type VM_HOST_NOTIFY_STATUS = i32;
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = 2i32;
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = 3i32;
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = 1i32;
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type VM_NOTIFY_STATUS = i32;
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = 4i32;
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = 2i32;
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = 3i32;
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = 1i32;
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut windows_sys::core::PWSTR,
}
impl Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
impl Default for WRDS_CONNECTION_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_CONNECTION_SETTINGS {
    pub WRdsConnectionSettingLevel: WRDS_CONNECTION_SETTING_LEVEL,
    pub WRdsConnectionSetting: WRDS_CONNECTION_SETTING,
}
impl Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: bool,
    pub fInheritColorDepth: bool,
    pub fHideTitleBar: bool,
    pub fInheritAutoLogon: bool,
    pub fMaximizeShell: bool,
    pub fDisablePNP: bool,
    pub fPasswordIsScPin: bool,
    pub fPromptForPassword: bool,
    pub fDisableCpm: bool,
    pub fDisableCdm: bool,
    pub fDisableCcm: bool,
    pub fDisableLPT: bool,
    pub fDisableClip: bool,
    pub fResetBroken: bool,
    pub fDisableEncryption: bool,
    pub fDisableAutoReconnect: bool,
    pub fDisableCtrlAltDel: bool,
    pub fDoubleClickDetect: bool,
    pub fEnableWindowsKey: bool,
    pub fUsingSavedCreds: bool,
    pub fMouse: bool,
    pub fNoAudioPlayback: bool,
    pub fRemoteConsoleAudio: bool,
    pub EncryptionLevel: u8,
    pub ColorDepth: u16,
    pub ProtocolType: u16,
    pub HRes: u16,
    pub VRes: u16,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub KeyboardLayout: u32,
    pub MaxConnectionTime: u32,
    pub MaxDisconnectionTime: u32,
    pub MaxIdleTime: u32,
    pub PerformanceFlags: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub ActiveInputLocale: u32,
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientBuildNumber: u32,
    pub ClientSessionId: u32,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserName: [u16; 256],
    pub Domain: [u16; 256],
    pub Password: [u16; 256],
    pub ProtocolName: [u16; 9],
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub imeFileName: [u16; 33],
    pub AudioDriverName: [u16; 9],
    pub ClientName: [u16; 21],
    pub ClientAddress: [u16; 31],
    pub ClientDirectory: [u16; 257],
    pub ClientDigProductId: [u16; 33],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub WRdsListenerSettings: WRDS_LISTENER_SETTINGS,
    pub EventLogActivityId: windows_sys::core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
impl Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WRDS_CONNECTION_SETTING_LEVEL = i32;
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = 1i32;
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = 0i32;
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
    pub TimeZoneKeyName: [u16; 128],
    pub DynamicDaylightTimeDisabled: u16,
}
impl Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl Default for WRDS_LISTENER_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_LISTENER_SETTINGS {
    pub WRdsListenerSettingLevel: WRDS_LISTENER_SETTING_LEVEL,
    pub WRdsListenerSetting: WRDS_LISTENER_SETTING,
}
impl Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WRDS_LISTENER_SETTING_LEVEL = i32;
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = 1i32;
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = 0i32;
pub const WRDS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WRDS_MAX_COUNTERS: u32 = 100u32;
pub const WRDS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WRDS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WRDS_MAX_RESERVED: u32 = 100u32;
pub const WRDS_PASSWORD_LENGTH: u32 = 255u32;
pub const WRDS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WRDS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WRDS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WRDS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WRDS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WRDS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WRDS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WRDS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WRDS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WRDS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
pub const WRDS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2993f4d_02cf_4280_8c48_1624b44f8706);
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
impl Default for WRDS_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_SETTINGS {
    pub WRdsSettingType: WRDS_SETTING_TYPE,
    pub WRdsSettingLevel: WRDS_SETTING_LEVEL,
    pub WRdsSetting: WRDS_SETTING,
}
impl Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WRDS_SETTINGS_1 {
    pub WRdsDisableClipStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableClipValue: u32,
    pub WRdsDisableLPTStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableLPTValue: u32,
    pub WRdsDisableCcmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCcmValue: u32,
    pub WRdsDisableCdmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCdmValue: u32,
    pub WRdsDisableCpmStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableCpmValue: u32,
    pub WRdsDisablePnpStatus: WRDS_SETTING_STATUS,
    pub WRdsDisablePnpValue: u32,
    pub WRdsEncryptionLevelStatus: WRDS_SETTING_STATUS,
    pub WRdsEncryptionValue: u32,
    pub WRdsColorDepthStatus: WRDS_SETTING_STATUS,
    pub WRdsColorDepthValue: u32,
    pub WRdsDisableAutoReconnecetStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableAutoReconnecetValue: u32,
    pub WRdsDisableEncryptionStatus: WRDS_SETTING_STATUS,
    pub WRdsDisableEncryptionValue: u32,
    pub WRdsResetBrokenStatus: WRDS_SETTING_STATUS,
    pub WRdsResetBrokenValue: u32,
    pub WRdsMaxIdleTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxIdleTimeValue: u32,
    pub WRdsMaxDisconnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxDisconnectTimeValue: u32,
    pub WRdsMaxConnectTimeStatus: WRDS_SETTING_STATUS,
    pub WRdsMaxConnectTimeValue: u32,
    pub WRdsKeepAliveStatus: WRDS_SETTING_STATUS,
    pub WRdsKeepAliveStartValue: bool,
    pub WRdsKeepAliveIntervalValue: u32,
}
pub type WRDS_SETTING_LEVEL = i32;
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = 1i32;
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = 0i32;
pub type WRDS_SETTING_STATUS = i32;
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = 0i32;
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = 1i32;
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = -1i32;
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = 2i32;
pub type WRDS_SETTING_TYPE = i32;
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = 0i32;
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = 1i32;
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = 3i32;
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = 2i32;
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
pub type WRdsGraphicsChannelType = i32;
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = 1i32;
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = 0i32;
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = 0i32;
pub const WTSApplicationName: WTS_INFO_CLASS = 1i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSCLIENTA {
    pub ClientName: [i8; 21],
    pub Domain: [i8; 18],
    pub UserName: [i8; 21],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [i8; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [i8; 261],
}
impl Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSCLIENTW {
    pub ClientName: [u16; 21],
    pub Domain: [u16; 18],
    pub UserName: [u16; 21],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub EncryptionLevel: u8,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub ClientDirectory: [u16; 261],
    pub ClientBuildNumber: u32,
    pub ClientHardwareId: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub DeviceId: [u16; 261],
}
impl Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSCONFIGINFOA {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [i8; 21],
    pub LogonDomain: [i8; 18],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
    pub ApplicationName: [i8; 261],
}
impl Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSCONFIGINFOW {
    pub version: u32,
    pub fConnectClientDrivesAtLogon: u32,
    pub fConnectPrinterAtLogon: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub ShadowSettings: u32,
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
    pub ApplicationName: [u16; 261],
}
impl Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSClientAddress: WTS_INFO_CLASS = 14i32;
pub const WTSClientBuildNumber: WTS_INFO_CLASS = 9i32;
pub const WTSClientDirectory: WTS_INFO_CLASS = 11i32;
pub const WTSClientDisplay: WTS_INFO_CLASS = 15i32;
pub const WTSClientHardwareId: WTS_INFO_CLASS = 13i32;
pub const WTSClientInfo: WTS_INFO_CLASS = 23i32;
pub const WTSClientName: WTS_INFO_CLASS = 10i32;
pub const WTSClientProductId: WTS_INFO_CLASS = 12i32;
pub const WTSClientProtocolType: WTS_INFO_CLASS = 16i32;
pub const WTSConfigInfo: WTS_INFO_CLASS = 26i32;
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = 2i32;
pub const WTSConnectState: WTS_INFO_CLASS = 8i32;
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = 1i32;
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = 4i32;
pub const WTSDomainName: WTS_INFO_CLASS = 7i32;
pub const WTSDown: WTS_CONNECTSTATE_CLASS = 8i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOA {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBy: u32,
    pub WinStationName: [i8; 32],
    pub Domain: [i8; 17],
    pub UserName: [i8; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl Default for WTSINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEXA {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_A,
}
impl Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEXW {
    pub Level: u32,
    pub Data: WTSINFOEX_LEVEL_W,
}
impl Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEX_LEVEL1_A {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [i8; 33],
    pub UserName: [i8; 21],
    pub DomainName: [i8; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOEX_LEVEL1_W {
    pub SessionId: u32,
    pub SessionState: WTS_CONNECTSTATE_CLASS,
    pub SessionFlags: i32,
    pub WinStationName: [u16; 33],
    pub UserName: [u16; 21],
    pub DomainName: [u16; 18],
    pub LogonTime: i64,
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub CurrentTime: i64,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
}
impl Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_A {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_A,
}
impl Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTSINFOEX_LEVEL_W {
    pub WTSInfoExLevel1: WTSINFOEX_LEVEL1_W,
}
impl Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSINFOW {
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub IncomingBytes: u32,
    pub OutgoingBytes: u32,
    pub IncomingFrames: u32,
    pub OutgoingFrames: u32,
    pub IncomingCompressedBytes: u32,
    pub OutgoingCompressedBytes: u32,
    pub WinStationName: [u16; 32],
    pub Domain: [u16; 17],
    pub UserName: [u16; 21],
    pub ConnectTime: i64,
    pub DisconnectTime: i64,
    pub LastInputTime: i64,
    pub LogonTime: i64,
    pub CurrentTime: i64,
}
impl Default for WTSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = 5i32;
pub const WTSIdleTime: WTS_INFO_CLASS = 17i32;
pub const WTSIncomingBytes: WTS_INFO_CLASS = 19i32;
pub const WTSIncomingFrames: WTS_INFO_CLASS = 21i32;
pub const WTSInit: WTS_CONNECTSTATE_CLASS = 9i32;
pub const WTSInitialProgram: WTS_INFO_CLASS = 0i32;
pub const WTSIsRemoteSession: WTS_INFO_CLASS = 29i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSLISTENERCONFIGA {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [i8; 61],
    pub LogonUserName: [i8; 21],
    pub LogonDomain: [i8; 18],
    pub WorkDirectory: [i8; 261],
    pub InitialProgram: [i8; 261],
}
impl Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSLISTENERCONFIGW {
    pub version: u32,
    pub fEnableListener: u32,
    pub MaxConnectionCount: u32,
    pub fPromptForPassword: u32,
    pub fInheritColorDepth: u32,
    pub ColorDepth: u32,
    pub fInheritBrokenTimeoutSettings: u32,
    pub BrokenTimeoutSettings: u32,
    pub fDisablePrinterRedirection: u32,
    pub fDisableDriveRedirection: u32,
    pub fDisableComPortRedirection: u32,
    pub fDisableLPTPortRedirection: u32,
    pub fDisableClipboardRedirection: u32,
    pub fDisableAudioRedirection: u32,
    pub fDisablePNPRedirection: u32,
    pub fDisableDefaultMainClientPrinter: u32,
    pub LanAdapter: u32,
    pub PortNumber: u32,
    pub fInheritShadowSettings: u32,
    pub ShadowSettings: u32,
    pub TimeoutSettingsConnection: u32,
    pub TimeoutSettingsDisconnection: u32,
    pub TimeoutSettingsIdle: u32,
    pub SecurityLayer: u32,
    pub MinEncryptionLevel: u32,
    pub UserAuthentication: u32,
    pub Comment: [u16; 61],
    pub LogonUserName: [u16; 21],
    pub LogonDomain: [u16; 18],
    pub WorkDirectory: [u16; 261],
    pub InitialProgram: [u16; 261],
}
impl Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSListen: WTS_CONNECTSTATE_CLASS = 6i32;
pub const WTSLogonTime: WTS_INFO_CLASS = 18i32;
pub const WTSOEMId: WTS_INFO_CLASS = 3i32;
pub const WTSOutgoingBytes: WTS_INFO_CLASS = 20i32;
pub const WTSOutgoingFrames: WTS_INFO_CLASS = 22i32;
pub const WTSReset: WTS_CONNECTSTATE_CLASS = 7i32;
pub type WTSSBX_ADDRESS_FAMILY = i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = 1i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = 2i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = 3i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = 4i32;
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTSSBX_MACHINE_DRAIN = i32;
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = 1i32;
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = 2i32;
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = 0i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSSBX_MACHINE_INFO {
    pub ClientConnectInfo: WTSSBX_MACHINE_CONNECT_INFO,
    pub wczFarmName: [u16; 257],
    pub InternalIPAddress: WTSSBX_IP_ADDRESS,
    pub dwMaxSessionsLimit: u32,
    pub ServerWeight: u32,
    pub SingleSessionMode: WTSSBX_MACHINE_SESSION_MODE,
    pub InDrain: WTSSBX_MACHINE_DRAIN,
    pub MachineState: WTSSBX_MACHINE_STATE,
}
impl Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTSSBX_MACHINE_SESSION_MODE = i32;
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = 2i32;
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = 1i32;
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = 0i32;
pub type WTSSBX_MACHINE_STATE = i32;
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = 1i32;
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = 2i32;
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = 0i32;
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = 4i32;
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = 2i32;
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = 1i32;
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = 8i32;
pub type WTSSBX_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
impl Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTSSBX_SESSION_STATE = i32;
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = 1i32;
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = 2i32;
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
pub const WTSSessionAddressV4: WTS_INFO_CLASS = 28i32;
pub const WTSSessionId: WTS_INFO_CLASS = 4i32;
pub const WTSSessionInfo: WTS_INFO_CLASS = 24i32;
pub const WTSSessionInfoEx: WTS_INFO_CLASS = 25i32;
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = 3i32;
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = 0i32;
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = 1i32;
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSUSERCONFIGA {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [i8; 261],
    pub WorkDirectory: [i8; 261],
    pub TerminalServerProfilePath: [i8; 261],
    pub TerminalServerHomeDir: [i8; 261],
    pub TerminalServerHomeDirDrive: [i8; 4],
}
impl Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTSUSERCONFIGW {
    pub Source: u32,
    pub InheritInitialProgram: u32,
    pub AllowLogonTerminalServer: u32,
    pub TimeoutSettingsConnections: u32,
    pub TimeoutSettingsDisconnections: u32,
    pub TimeoutSettingsIdle: u32,
    pub DeviceClientDrives: u32,
    pub DeviceClientPrinters: u32,
    pub ClientDefaultPrinter: u32,
    pub BrokenTimeoutSettings: u32,
    pub ReconnectSettings: u32,
    pub ShadowingSettings: u32,
    pub TerminalServerRemoteHomeDir: u32,
    pub InitialProgram: [u16; 261],
    pub WorkDirectory: [u16; 261],
    pub TerminalServerProfilePath: [u16; 261],
    pub TerminalServerHomeDir: [u16; 261],
    pub TerminalServerHomeDirDrive: [u16; 4],
}
impl Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = 10i32;
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = 0i32;
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = 13i32;
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = 12i32;
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = 11i32;
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = 14i32;
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = 0i32;
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = 16i32;
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = 17i32;
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = 15i32;
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = 4i32;
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = 5i32;
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = 6i32;
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = 19i32;
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = 1i32;
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = 3i32;
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = 9i32;
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = 7i32;
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = 8i32;
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = 2i32;
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = 18i32;
pub const WTSUserName: WTS_INFO_CLASS = 5i32;
pub const WTSValidationInfo: WTS_INFO_CLASS = 27i32;
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = 0i32;
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = 1i32;
pub const WTSWinStationName: WTS_INFO_CLASS = 6i32;
pub const WTSWorkingDirectory: WTS_INFO_CLASS = 2i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl Default for WTS_CACHE_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTS_CACHE_STATS_UN {
    pub ProtocolCache: [WTS_PROTOCOL_CACHE; 4],
    pub TShareCacheStats: u32,
    pub Reserved: [u32; 20],
}
impl Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTS_CERT_TYPE = i32;
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = 0i32;
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = 1i32;
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = 2i32;
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: bool,
    pub fDoubleClickDetect: bool,
    pub fEnableWindowsKey: bool,
    pub fHideTitleBar: bool,
    pub fInheritAutoLogon: windows_sys::core::BOOL,
    pub fPromptForPassword: bool,
    pub fUsingSavedCreds: bool,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: bool,
    pub fInheritInitialProgram: windows_sys::core::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: bool,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: windows_sys::core::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: bool,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: bool,
    pub fRemoteConsoleAudio: bool,
    pub AudioDriverName: [u16; 9],
    pub ClientTimeZone: WTS_TIME_ZONE_INFORMATION,
    pub ClientName: [u16; 21],
    pub SerialNumber: u32,
    pub ClientAddressFamily: u32,
    pub ClientAddress: [u16; 31],
    pub ClientSockAddress: WTS_SOCKADDR,
    pub ClientDirectory: [u16; 257],
    pub ClientBuildNumber: u32,
    pub ClientProductId: u16,
    pub OutBufCountHost: u16,
    pub OutBufCountClient: u16,
    pub OutBufLength: u16,
    pub ClientSessionId: u32,
    pub ClientDigProductId: [u16; 33],
    pub fDisableCpm: bool,
    pub fDisableCdm: bool,
    pub fDisableCcm: bool,
    pub fDisableLPT: bool,
    pub fDisableClip: bool,
    pub fDisablePNP: bool,
}
impl Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
pub type WTS_CONFIG_CLASS = i32;
pub type WTS_CONFIG_SOURCE = i32;
pub type WTS_CONNECTSTATE_CLASS = i32;
pub const WTS_CURRENT_SERVER: super::super::Foundation::HANDLE = 0i32 as _;
pub const WTS_CURRENT_SERVER_HANDLE: super::super::Foundation::HANDLE = 0i32 as _;
pub const WTS_CURRENT_SERVER_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("");
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_DOMAIN_LENGTH: u32 = 255u32;
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = 1i32;
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = 2i32;
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = 0i32;
pub const WTS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub const WTS_DRIVE_LENGTH: u32 = 3u32;
pub const WTS_EVENT_ALL: u32 = 2147483647u32;
pub const WTS_EVENT_CONNECT: u32 = 8u32;
pub const WTS_EVENT_CREATE: u32 = 1u32;
pub const WTS_EVENT_DELETE: u32 = 2u32;
pub const WTS_EVENT_DISCONNECT: u32 = 16u32;
pub const WTS_EVENT_FLUSH: u32 = 2147483648u32;
pub const WTS_EVENT_LICENSE: u32 = 256u32;
pub const WTS_EVENT_LOGOFF: u32 = 64u32;
pub const WTS_EVENT_LOGON: u32 = 32u32;
pub const WTS_EVENT_NONE: u32 = 0u32;
pub const WTS_EVENT_RENAME: u32 = 4u32;
pub const WTS_EVENT_STATECHANGE: u32 = 128u32;
pub const WTS_IMEFILENAME_LENGTH: u32 = 32u32;
pub type WTS_INFO_CLASS = i32;
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: windows_sys::core::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
impl Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub const WTS_LISTENER_CREATE: u32 = 1u32;
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
pub type WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = i32;
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 3i32;
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 4i32;
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 2i32;
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 0i32;
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = 1i32;
pub const WTS_MAX_CACHE_RESERVED: u32 = 20u32;
pub const WTS_MAX_COUNTERS: u32 = 100u32;
pub const WTS_MAX_DISPLAY_IOCTL_DATA: u32 = 256u32;
pub const WTS_MAX_PROTOCOL_CACHE: u32 = 4u32;
pub const WTS_MAX_RESERVED: u32 = 100u32;
pub const WTS_PASSWORD_LENGTH: u32 = 255u32;
pub const WTS_PERF_DISABLE_CURSORSETTINGS: u32 = 64u32;
pub const WTS_PERF_DISABLE_CURSOR_SHADOW: u32 = 32u32;
pub const WTS_PERF_DISABLE_FULLWINDOWDRAG: u32 = 2u32;
pub const WTS_PERF_DISABLE_MENUANIMATIONS: u32 = 4u32;
pub const WTS_PERF_DISABLE_NOTHING: u32 = 0u32;
pub const WTS_PERF_DISABLE_THEMING: u32 = 8u32;
pub const WTS_PERF_DISABLE_WALLPAPER: u32 = 1u32;
pub const WTS_PERF_ENABLE_DESKTOP_COMPOSITION: u32 = 256u32;
pub const WTS_PERF_ENABLE_ENHANCED_GRAPHICS: u32 = 16u32;
pub const WTS_PERF_ENABLE_FONT_SMOOTHING: u32 = 128u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: bool,
    pub fDisableAutoReconnect: bool,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: bool,
    pub fDisableCdm: bool,
    pub fDisableCcm: bool,
    pub fDisableLPT: bool,
    pub fDisableClip: bool,
    pub fDisablePNPRedir: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_sys::core::PSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_sys::core::PWSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_sys::core::PSTR,
    pub pUserSid: super::super::Security::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_sys::core::PWSTR,
    pub pUserSid: super::super::Security::PSID,
    pub NumberOfThreads: u32,
    pub HandleCount: u32,
    pub PagefileUsage: u32,
    pub PeakPagefileUsage: u32,
    pub WorkingSetSize: u32,
    pub PeakWorkingSetSize: u32,
    pub UserTime: i64,
    pub KernelTime: i64,
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultConfig");
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
impl Default for WTS_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTS_PROPERTY_VALUE_0 {
    pub ulVal: u32,
    pub strVal: WTS_PROPERTY_VALUE_0_0,
    pub bVal: WTS_PROPERTY_VALUE_0_1,
    pub guidVal: windows_sys::core::GUID,
}
impl Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pbVal: windows_sys::core::PSTR,
}
impl Default for WTS_PROPERTY_VALUE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pstrVal: windows_sys::core::PWSTR,
}
impl Default for WTS_PROPERTY_VALUE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROTOCOL_COUNTERS {
    pub WdBytes: u32,
    pub WdFrames: u32,
    pub WaitForOutBuf: u32,
    pub Frames: u32,
    pub Bytes: u32,
    pub CompressedBytes: u32,
    pub CompressFlushes: u32,
    pub Errors: u32,
    pub Timeouts: u32,
    pub AsyncFramingError: u32,
    pub AsyncOverrunError: u32,
    pub AsyncOverflowError: u32,
    pub AsyncParityError: u32,
    pub TdErrors: u32,
    pub ProtocolType: u16,
    pub Length: u16,
    pub Specific: u16,
    pub Reserved: [u32; 100],
}
impl Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROTOCOL_STATUS {
    pub Output: WTS_PROTOCOL_COUNTERS,
    pub Input: WTS_PROTOCOL_COUNTERS,
    pub Cache: WTS_CACHE_STATS,
    pub AsyncSignal: u32,
    pub AsyncSignalMask: u32,
    pub Counters: [i64; 100],
}
impl Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc77d1b30_5be1_4c6b_a0e1_bd6d2e5c9fcc);
pub const WTS_QUERY_AUDIOENUM_DLL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9bf4fa97_c883_4c2a_80ab_5a39c9af00db);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8b8e0fe7_0804_4a0e_b279_8660b1df0049);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41869ad0_6332_4dc8_95d5_db749e2f1d94);
pub type WTS_RCM_DRAIN_STATE = i32;
pub type WTS_RCM_SERVICE_STATE = i32;
pub const WTS_SECURITY_ALL_ACCESS: WTS_SECURITY_FLAGS = 983999u32;
pub const WTS_SECURITY_CONNECT: WTS_SECURITY_FLAGS = 256u32;
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: WTS_SECURITY_FLAGS = 72u32;
pub const WTS_SECURITY_CURRENT_USER_ACCESS: WTS_SECURITY_FLAGS = 590u32;
pub const WTS_SECURITY_DISCONNECT: WTS_SECURITY_FLAGS = 512u32;
pub type WTS_SECURITY_FLAGS = u32;
pub const WTS_SECURITY_GUEST_ACCESS: WTS_SECURITY_FLAGS = 32u32;
pub const WTS_SECURITY_LOGOFF: WTS_SECURITY_FLAGS = 64u32;
pub const WTS_SECURITY_LOGON: WTS_SECURITY_FLAGS = 32u32;
pub const WTS_SECURITY_MESSAGE: WTS_SECURITY_FLAGS = 128u32;
pub const WTS_SECURITY_QUERY_INFORMATION: WTS_SECURITY_FLAGS = 1u32;
pub const WTS_SECURITY_REMOTE_CONTROL: WTS_SECURITY_FLAGS = 16u32;
pub const WTS_SECURITY_RESET: WTS_SECURITY_FLAGS = 4u32;
pub const WTS_SECURITY_SET_INFORMATION: WTS_SECURITY_FLAGS = 2u32;
pub const WTS_SECURITY_USER_ACCESS: WTS_SECURITY_FLAGS = 329u32;
pub const WTS_SECURITY_VIRTUAL_CHANNELS: WTS_SECURITY_FLAGS = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: windows_sys::core::PSTR,
}
impl Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: windows_sys::core::PWSTR,
}
impl Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = 0i32;
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = 2i32;
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: windows_sys::core::GUID,
    pub SessionId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: windows_sys::core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: windows_sys::core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_sys::core::PSTR,
    pub pHostName: windows_sys::core::PSTR,
    pub pUserName: windows_sys::core::PSTR,
    pub pDomainName: windows_sys::core::PSTR,
    pub pFarmName: windows_sys::core::PSTR,
}
impl Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_sys::core::PWSTR,
    pub pHostName: windows_sys::core::PWSTR,
    pub pUserName: windows_sys::core::PWSTR,
    pub pDomainName: windows_sys::core::PWSTR,
    pub pFarmName: windows_sys::core::PWSTR,
}
impl Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl Default for WTS_SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WTS_SOCKADDR_0 {
    pub ipv4: WTS_SOCKADDR_0_0,
    pub ipv6: WTS_SOCKADDR_0_1,
}
impl Default for WTS_SOCKADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl Default for WTS_SOCKADDR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl Default for WTS_SOCKADDR_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WTS_SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WTS_TYPE_CLASS = i32;
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
pub type WTS_VIRTUAL_CLASS = i32;
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
pub const WTS_WSD_LOGOFF: u32 = 1u32;
pub const WTS_WSD_POWEROFF: u32 = 8u32;
pub const WTS_WSD_REBOOT: u32 = 4u32;
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f1dfca6_3aad_48e1_8406_4bc21a501d7c);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: windows_sys::core::PWSTR,
    pub fileExtension: [u16; 256],
    pub resourcePluginType: [u16; 256],
    pub isDiscoverable: u8,
    pub resourceType: i32,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
    pub pcePluginBlobSize: u32,
    pub blobContents: *mut u8,
}
impl Default for pluginResource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: windows_sys::core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl Default for pluginResource2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
