#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> windows_core::BOOL);
    unsafe { ProcessIdToSessionId(dwprocessid, psessionid as _).ok() }
}
#[inline]
pub unsafe fn WTSCloseServer(hserver: super::super::Foundation::HANDLE) {
    windows_core::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::super::Foundation:: HANDLE));
    unsafe { WTSCloseServer(hserver) }
}
#[inline]
pub unsafe fn WTSConnectSessionA<P2>(logonid: u32, targetlogonid: u32, ppassword: P2, bwait: bool) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCSTR, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSConnectSessionA(logonid, targetlogonid, ppassword.param().abi(), bwait.into()).ok() }
}
#[inline]
pub unsafe fn WTSConnectSessionW<P2>(logonid: u32, targetlogonid: u32, ppassword: P2, bwait: bool) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCWSTR, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSConnectSessionW(logonid, targetlogonid, ppassword.param().abi(), bwait.into()).ok() }
}
#[inline]
pub unsafe fn WTSCreateListenerA<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *const WTSLISTENERCONFIGA, flag : u32) -> windows_core::BOOL);
    unsafe { WTSCreateListenerA(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), pbuffer, flag).ok() }
}
#[inline]
pub unsafe fn WTSCreateListenerW<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *const WTSLISTENERCONFIGW, flag : u32) -> windows_core::BOOL);
    unsafe { WTSCreateListenerW(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), pbuffer, flag).ok() }
}
#[inline]
pub unsafe fn WTSDisconnectSession(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, bwait: bool) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSDisconnectSession(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, bwait.into()).ok() }
}
#[inline]
pub unsafe fn WTSEnableChildSessions(benable: bool) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnableChildSessions(benable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSEnableChildSessions(benable.into()) }
}
#[inline]
pub unsafe fn WTSEnumerateListenersA(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut *mut i8>, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut i8, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateListenersA(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plisteners.unwrap_or(core::mem::zeroed()) as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateListenersW(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut *mut u16>, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut u16, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateListenersW(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plisteners.unwrap_or(core::mem::zeroed()) as _, pcount as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA(hserver: Option<super::super::Foundation::HANDLE>, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesA(hserver.unwrap_or(core::mem::zeroed()) as _, reserved, version, ppprocessinfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateProcessesExA(hserver: Option<super::super::Foundation::HANDLE>, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PSTR, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PSTR, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesExA(hserver.unwrap_or(core::mem::zeroed()) as _, plevel as _, sessionid, ppprocessinfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateProcessesExW(hserver: Option<super::super::Foundation::HANDLE>, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PWSTR, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PWSTR, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesExW(hserver.unwrap_or(core::mem::zeroed()) as _, plevel as _, sessionid, ppprocessinfo as _, pcount as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW(hserver: Option<super::super::Foundation::HANDLE>, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateProcessesW(hserver.unwrap_or(core::mem::zeroed()) as _, reserved, version, ppprocessinfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateServersA<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : windows_core::PCSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateServersA(pdomainname.param().abi(), reserved, version, ppserverinfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateServersW<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : windows_core::PCWSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateServersW(pdomainname.param().abi(), reserved, version, ppserverinfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateSessionsA(hserver: Option<super::super::Foundation::HANDLE>, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOA, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsA(hserver.unwrap_or(core::mem::zeroed()) as _, reserved, version, ppsessioninfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateSessionsExA(hserver: Option<super::super::Foundation::HANDLE>, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1A, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsExA(hserver.unwrap_or(core::mem::zeroed()) as _, plevel as _, filter, ppsessioninfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateSessionsExW(hserver: Option<super::super::Foundation::HANDLE>, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1W, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsExW(hserver.unwrap_or(core::mem::zeroed()) as _, plevel as _, filter, ppsessioninfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSEnumerateSessionsW(hserver: Option<super::super::Foundation::HANDLE>, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOW, pcount : *mut u32) -> windows_core::BOOL);
    unsafe { WTSEnumerateSessionsW(hserver.unwrap_or(core::mem::zeroed()) as _, reserved, version, ppsessioninfo as _, pcount as _).ok() }
}
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : *mut core::ffi::c_void));
    unsafe { WTSFreeMemory(pmemory as _) }
}
#[inline]
pub unsafe fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_core::BOOL);
    unsafe { WTSFreeMemoryExA(wtstypeclass, pmemory, numberofentries).ok() }
}
#[inline]
pub unsafe fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> windows_core::BOOL);
    unsafe { WTSFreeMemoryExW(wtstypeclass, pmemory, numberofentries).ok() }
}
#[inline]
pub unsafe fn WTSGetActiveConsoleSessionId() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> u32);
    unsafe { WTSGetActiveConsoleSessionId() }
}
#[inline]
pub unsafe fn WTSGetChildSessionId(psessionid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSGetChildSessionId(psessionid : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetChildSessionId(psessionid as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: Option<super::super::Security::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetListenerSecurityA(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: Option<super::super::Security::PSECURITY_DESCRIPTOR>, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::BOOL);
    unsafe { WTSGetListenerSecurityW(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _).ok() }
}
#[inline]
pub unsafe fn WTSIsChildSessionsEnabled(pbenabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSIsChildSessionsEnabled(pbenabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSIsChildSessionsEnabled(pbenabled as _) }
}
#[inline]
pub unsafe fn WTSLogoffSession(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, bwait: bool) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSLogoffSession(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, bwait.into()).ok() }
}
#[inline]
pub unsafe fn WTSOpenServerA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    unsafe { WTSOpenServerA(pservername.param().abi()) }
}
#[inline]
pub unsafe fn WTSOpenServerExA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    unsafe { WTSOpenServerExA(pservername.param().abi()) }
}
#[inline]
pub unsafe fn WTSOpenServerExW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    unsafe { WTSOpenServerExW(pservername.param().abi()) }
}
#[inline]
pub unsafe fn WTSOpenServerW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    unsafe { WTSOpenServerW(pservername.param().abi()) }
}
#[inline]
pub unsafe fn WTSQueryListenerConfigA<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *mut WTSLISTENERCONFIGA) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *mut WTSLISTENERCONFIGA) -> windows_core::BOOL);
    unsafe { WTSQueryListenerConfigA(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), pbuffer as _).ok() }
}
#[inline]
pub unsafe fn WTSQueryListenerConfigW<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, pbuffer: *mut WTSLISTENERCONFIGW) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *mut WTSLISTENERCONFIGW) -> windows_core::BOOL);
    unsafe { WTSQueryListenerConfigW(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), pbuffer as _).ok() }
}
#[inline]
pub unsafe fn WTSQuerySessionInformationA(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQuerySessionInformationA(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, wtsinfoclass, ppbuffer as _, pbytesreturned as _).ok() }
}
#[inline]
pub unsafe fn WTSQuerySessionInformationW(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQuerySessionInformationW(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, wtsinfoclass, ppbuffer as _, pbytesreturned as _).ok() }
}
#[inline]
pub unsafe fn WTSQueryUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQueryUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer as _, pbytesreturned as _).ok() }
}
#[inline]
pub unsafe fn WTSQueryUserConfigW<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSQueryUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer as _, pbytesreturned as _).ok() }
}
#[inline]
pub unsafe fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : u32, phtoken : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { WTSQueryUserToken(sessionid, phtoken as _).ok() }
}
#[inline]
pub unsafe fn WTSRegisterSessionNotification(hwnd: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { WTSRegisterSessionNotification(hwnd, dwflags).ok() }
}
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx(hserver: Option<super::super::Foundation::HANDLE>, hwnd: super::super::Foundation::HWND, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { WTSRegisterSessionNotificationEx(hserver.unwrap_or(core::mem::zeroed()) as _, hwnd, dwflags).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn WTSSendMessageA(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, ptitle: &[u8], pmessage: &[u8], style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: bool) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_core::PCSTR, titlelength : u32, pmessage : windows_core::PCSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSSendMessageA(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, core::mem::transmute(ptitle.as_ptr()), ptitle.len().try_into().unwrap(), core::mem::transmute(pmessage.as_ptr()), pmessage.len().try_into().unwrap(), style, timeout, presponse as _, bwait.into()).ok() }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn WTSSendMessageW<P2, P4>(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, ptitle: P2, titlelength: u32, pmessage: P4, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: bool) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_core::PCWSTR, titlelength : u32, pmessage : windows_core::PCWSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WTSSendMessageW(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, ptitle.param().abi(), titlelength, pmessage.param().abi(), messagelength, style, timeout, presponse as _, bwait.into()).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { WTSSetListenerSecurityA(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<P3>(hserver: Option<super::super::Foundation::HANDLE>, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P3, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { WTSSetListenerSecurityW(hserver.unwrap_or(core::mem::zeroed()) as _, preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor).ok() }
}
#[inline]
pub unsafe fn WTSSetRenderHint(prenderhintid: *mut u64, hwndowner: super::super::Foundation::HWND, renderhinttype: u32, phintdata: Option<&[u8]>) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetRenderHint(prenderhintid : *mut u64, hwndowner : super::super::Foundation:: HWND, renderhinttype : u32, cbhintdatalength : u32, phintdata : *const u8) -> windows_core::HRESULT);
    unsafe { WTSSetRenderHint(prenderhintid as _, hwndowner, renderhinttype, phintdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(phintdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok() }
}
#[inline]
pub unsafe fn WTSSetUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCSTR, datalength : u32) -> windows_core::BOOL);
    unsafe { WTSSetUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn WTSSetUserConfigW<P0, P1, P3>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: P3, datalength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCWSTR, datalength : u32) -> windows_core::BOOL);
    unsafe { WTSSetUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, pbuffer.param().abi(), datalength).ok() }
}
#[inline]
pub unsafe fn WTSShutdownSystem(hserver: Option<super::super::Foundation::HANDLE>, shutdownflag: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::super::Foundation:: HANDLE, shutdownflag : u32) -> windows_core::BOOL);
    unsafe { WTSShutdownSystem(hserver.unwrap_or(core::mem::zeroed()) as _, shutdownflag).ok() }
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : windows_core::PCSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_core::BOOL);
    unsafe { WTSStartRemoteControlSessionA(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok() }
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : windows_core::PCWSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> windows_core::BOOL);
    unsafe { WTSStartRemoteControlSessionW(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok() }
}
#[inline]
pub unsafe fn WTSStopRemoteControlSession(logonid: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : u32) -> windows_core::BOOL);
    unsafe { WTSStopRemoteControlSession(logonid).ok() }
}
#[inline]
pub unsafe fn WTSTerminateProcess(hserver: Option<super::super::Foundation::HANDLE>, processid: u32, exitcode: u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::super::Foundation:: HANDLE, processid : u32, exitcode : u32) -> windows_core::BOOL);
    unsafe { WTSTerminateProcess(hserver.unwrap_or(core::mem::zeroed()) as _, processid, exitcode).ok() }
}
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification(hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { WTSUnRegisterSessionNotification(hwnd).ok() }
}
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx(hserver: Option<super::super::Foundation::HANDLE>, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND) -> windows_core::BOOL);
    unsafe { WTSUnRegisterSessionNotificationEx(hserver.unwrap_or(core::mem::zeroed()) as _, hwnd).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelClose(hchannelhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelClose(hchannelhandle).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelOpen<P2>(hserver: Option<super::super::Foundation::HANDLE>, sessionid: u32, pvirtualname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::super::Foundation:: HANDLE, sessionid : u32, pvirtualname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { WTSVirtualChannelOpen(hserver.unwrap_or(core::mem::zeroed()) as _, sessionid, pvirtualname.param().abi()) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<P1>(sessionid: u32, pvirtualname: P1, flags: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : u32, pvirtualname : windows_core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
    let result__ = unsafe { WTSVirtualChannelOpenEx(sessionid, pvirtualname.param().abi(), flags) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_thread)
}
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput(hchannelhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelPurgeInput(hchannelhandle).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput(hchannelhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelPurgeOutput(hchannelhandle).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelQuery(hchannelhandle: super::super::Foundation::HANDLE, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut core::ffi::c_void, pbytesreturned: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::super::Foundation:: HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut *mut core::ffi::c_void, pbytesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelQuery(hchannelhandle, param1, ppbuffer as _, pbytesreturned as _).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelRead(hchannelhandle: super::super::Foundation::HANDLE, timeout: u32, buffer: &mut [u8], pbytesread: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::super::Foundation:: HANDLE, timeout : u32, buffer : windows_core::PSTR, buffersize : u32, pbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelRead(hchannelhandle, timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), pbytesread as _).ok() }
}
#[inline]
pub unsafe fn WTSVirtualChannelWrite(hchannelhandle: super::super::Foundation::HANDLE, buffer: &[u8], pbyteswritten: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::super::Foundation:: HANDLE, buffer : windows_core::PCSTR, length : u32, pbyteswritten : *mut u32) -> windows_core::BOOL);
    unsafe { WTSVirtualChannelWrite(hchannelhandle, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), pbyteswritten as _).ok() }
}
#[inline]
pub unsafe fn WTSWaitSystemEvent(hserver: Option<super::super::Foundation::HANDLE>, eventmask: u32, peventflags: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::super::Foundation:: HANDLE, eventmask : u32, peventflags : *mut u32) -> windows_core::BOOL);
    unsafe { WTSWaitSystemEvent(hserver.unwrap_or(core::mem::zeroed()) as _, eventmask, peventflags as _).ok() }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AAAccountingData {
    pub userName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub clientName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub authType: AAAuthSchemes,
    pub resourceName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub portNumber: i32,
    pub protocolName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub numberOfBytesReceived: i32,
    pub numberOfBytesTransfered: i32,
    pub reasonForDisconnect: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub mainSessionId: windows_core::GUID,
    pub subSessionId: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AAAccountingDataType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AAAuthSchemes(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AATrustClassID(pub i32);
pub const AA_AUTH_ANY: AAAuthSchemes = AAAuthSchemes(6i32);
pub const AA_AUTH_BASIC: AAAuthSchemes = AAAuthSchemes(1i32);
pub const AA_AUTH_CONID: AAAuthSchemes = AAAuthSchemes(10i32);
pub const AA_AUTH_COOKIE: AAAuthSchemes = AAAuthSchemes(7i32);
pub const AA_AUTH_DIGEST: AAAuthSchemes = AAAuthSchemes(8i32);
pub const AA_AUTH_LOGGEDONCREDENTIALS: AAAuthSchemes = AAAuthSchemes(4i32);
pub const AA_AUTH_MAX: AAAuthSchemes = AAAuthSchemes(12i32);
pub const AA_AUTH_MIN: AAAuthSchemes = AAAuthSchemes(0i32);
pub const AA_AUTH_NEGOTIATE: AAAuthSchemes = AAAuthSchemes(5i32);
pub const AA_AUTH_NTLM: AAAuthSchemes = AAAuthSchemes(2i32);
pub const AA_AUTH_ORGID: AAAuthSchemes = AAAuthSchemes(9i32);
pub const AA_AUTH_SC: AAAuthSchemes = AAAuthSchemes(3i32);
pub const AA_AUTH_SSPI_NTLM: AAAuthSchemes = AAAuthSchemes(11i32);
pub const AA_MAIN_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(3i32);
pub const AA_MAIN_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(0i32);
pub const AA_SUB_SESSION_CLOSED: AAAccountingDataType = AAAccountingDataType(2i32);
pub const AA_SUB_SESSION_CREATION: AAAccountingDataType = AAAccountingDataType(1i32);
pub const AA_TRUSTEDUSER_TRUSTEDCLIENT: AATrustClassID = AATrustClassID(2i32);
pub const AA_TRUSTEDUSER_UNTRUSTEDCLIENT: AATrustClassID = AATrustClassID(1i32);
pub const AA_UNTRUSTED: AATrustClassID = AATrustClassID(0i32);
pub const ACQUIRE_TARGET_LOCK_TIMEOUT: u32 = 300000u32;
pub const ADsTSUserEx: windows_core::GUID = windows_core::GUID::from_u128(0xe2e9cae6_1e7b_4b8e_babd_e9bf6292ac29);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AE_POSITION_FLAGS(pub i32);
pub const AllowOnlySDRServers: PolicyAttributeType = PolicyAttributeType(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(2i32);
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(0i32);
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIENT_MESSAGE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONNECTION_CHANGE_NOTIFICATION(pub i32);
pub const CONNECTION_PROPERTY_CURSOR_BLINK_DISABLED: windows_core::GUID = windows_core::GUID::from_u128(0x4b150580_fea4_4d3c_9de4_7433a66618f7);
pub const CONNECTION_PROPERTY_IDLE_TIME_WARNING: windows_core::GUID = windows_core::GUID::from_u128(0x693f7ff5_0c4e_4d17_b8e0_1f70325e5d58);
pub const CONNECTION_REQUEST_CANCELLED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(5i32);
pub const CONNECTION_REQUEST_FAILED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(2i32);
pub const CONNECTION_REQUEST_INVALID: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(0i32);
pub const CONNECTION_REQUEST_LB_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(6i32);
pub const CONNECTION_REQUEST_ORCH_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(8i32);
pub const CONNECTION_REQUEST_PENDING: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(1i32);
pub const CONNECTION_REQUEST_QUERY_PL_COMPLETED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(7i32);
pub const CONNECTION_REQUEST_SUCCEEDED: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(4i32);
pub const CONNECTION_REQUEST_TIMEDOUT: CONNECTION_CHANGE_NOTIFICATION = CONNECTION_CHANGE_NOTIFICATION(3i32);
pub const ClipboardRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(5i32);
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
pub const DisableAllRedirections: PolicyAttributeType = PolicyAttributeType(1i32);
pub const DriveRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(2i32);
pub const EnableAllRedirections: PolicyAttributeType = PolicyAttributeType(0i32);
pub const FARM: TARGET_TYPE = TARGET_TYPE(1i32);
pub const FORCE_REJOIN: u32 = 2u32;
pub const FORCE_REJOIN_IN_CLUSTERMODE: u32 = 3u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsTSUserEx, IADsTSUserEx_Vtbl, 0xc4930e79_2989_4462_8a60_2fcf2f2955ef);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsTSUserEx {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsTSUserEx, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsTSUserEx {
    pub unsafe fn TerminalServicesProfilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TerminalServicesProfilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTerminalServicesProfilePath(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTerminalServicesProfilePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewval)).ok() }
    }
    pub unsafe fn TerminalServicesHomeDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TerminalServicesHomeDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTerminalServicesHomeDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTerminalServicesHomeDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewval)).ok() }
    }
    pub unsafe fn TerminalServicesHomeDrive(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TerminalServicesHomeDrive)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTerminalServicesHomeDrive(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTerminalServicesHomeDrive)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewval)).ok() }
    }
    pub unsafe fn AllowLogon(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllowLogon(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowLogon)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn EnableRemoteControl(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableRemoteControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableRemoteControl(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableRemoteControl)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn MaxDisconnectionTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxDisconnectionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxDisconnectionTime(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxDisconnectionTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn MaxConnectionTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxConnectionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxConnectionTime(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxConnectionTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn MaxIdleTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxIdleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxIdleTime(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxIdleTime)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ReconnectionAction(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReconnectionAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetReconnectionAction(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetReconnectionAction)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn BrokenConnectionAction(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BrokenConnectionAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBrokenConnectionAction(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBrokenConnectionAction)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ConnectClientDrivesAtLogon(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectClientDrivesAtLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectClientDrivesAtLogon)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn ConnectClientPrintersAtLogon(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectClientPrintersAtLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectClientPrintersAtLogon)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn DefaultToMainPrinter(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultToMainPrinter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefaultToMainPrinter(&self, newval: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultToMainPrinter)(windows_core::Interface::as_raw(self), newval).ok() }
    }
    pub unsafe fn TerminalServicesWorkDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TerminalServicesWorkDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTerminalServicesWorkDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTerminalServicesWorkDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewval)).ok() }
    }
    pub unsafe fn TerminalServicesInitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TerminalServicesInitialProgram)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTerminalServicesInitialProgram(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTerminalServicesInitialProgram)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pnewval)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IADsTSUserEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TerminalServicesProfilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTerminalServicesProfilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminalServicesHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTerminalServicesHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminalServicesHomeDrive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTerminalServicesHomeDrive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowLogon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAllowLogon: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EnableRemoteControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEnableRemoteControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxDisconnectionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxDisconnectionTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxConnectionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxConnectionTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxIdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxIdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReconnectionAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetReconnectionAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BrokenConnectionAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBrokenConnectionAction: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ConnectClientDrivesAtLogon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetConnectClientDrivesAtLogon: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ConnectClientPrintersAtLogon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetConnectClientPrintersAtLogon: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DefaultToMainPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultToMainPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TerminalServicesWorkDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTerminalServicesWorkDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TerminalServicesInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTerminalServicesInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IADsTSUserEx_Impl: super::Com::IDispatch_Impl {
    fn TerminalServicesProfilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesProfilePath(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesHomeDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesHomeDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesHomeDrive(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesHomeDrive(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowLogon(&self) -> windows_core::Result<i32>;
    fn SetAllowLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn EnableRemoteControl(&self) -> windows_core::Result<i32>;
    fn SetEnableRemoteControl(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxDisconnectionTime(&self) -> windows_core::Result<i32>;
    fn SetMaxDisconnectionTime(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxConnectionTime(&self) -> windows_core::Result<i32>;
    fn SetMaxConnectionTime(&self, newval: i32) -> windows_core::Result<()>;
    fn MaxIdleTime(&self) -> windows_core::Result<i32>;
    fn SetMaxIdleTime(&self, newval: i32) -> windows_core::Result<()>;
    fn ReconnectionAction(&self) -> windows_core::Result<i32>;
    fn SetReconnectionAction(&self, newval: i32) -> windows_core::Result<()>;
    fn BrokenConnectionAction(&self) -> windows_core::Result<i32>;
    fn SetBrokenConnectionAction(&self, newval: i32) -> windows_core::Result<()>;
    fn ConnectClientDrivesAtLogon(&self) -> windows_core::Result<i32>;
    fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn ConnectClientPrintersAtLogon(&self) -> windows_core::Result<i32>;
    fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> windows_core::Result<()>;
    fn DefaultToMainPrinter(&self) -> windows_core::Result<i32>;
    fn SetDefaultToMainPrinter(&self, newval: i32) -> windows_core::Result<()>;
    fn TerminalServicesWorkDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesWorkDirectory(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TerminalServicesInitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTerminalServicesInitialProgram(&self, pnewval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IADsTSUserEx_Vtbl {
    pub const fn new<Identity: IADsTSUserEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TerminalServicesProfilePath<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::TerminalServicesProfilePath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTerminalServicesProfilePath<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetTerminalServicesProfilePath(this, core::mem::transmute(&pnewval)).into()
            }
        }
        unsafe extern "system" fn TerminalServicesHomeDirectory<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::TerminalServicesHomeDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDirectory<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetTerminalServicesHomeDirectory(this, core::mem::transmute(&pnewval)).into()
            }
        }
        unsafe extern "system" fn TerminalServicesHomeDrive<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::TerminalServicesHomeDrive(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTerminalServicesHomeDrive<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetTerminalServicesHomeDrive(this, core::mem::transmute(&pnewval)).into()
            }
        }
        unsafe extern "system" fn AllowLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::AllowLogon(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetAllowLogon(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn EnableRemoteControl<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::EnableRemoteControl(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableRemoteControl<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetEnableRemoteControl(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MaxDisconnectionTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::MaxDisconnectionTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxDisconnectionTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetMaxDisconnectionTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MaxConnectionTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::MaxConnectionTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxConnectionTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetMaxConnectionTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MaxIdleTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::MaxIdleTime(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxIdleTime<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetMaxIdleTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ReconnectionAction<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::ReconnectionAction(this) {
                    Ok(ok__) => {
                        pnewval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReconnectionAction<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetReconnectionAction(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn BrokenConnectionAction<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::BrokenConnectionAction(this) {
                    Ok(ok__) => {
                        pnewval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBrokenConnectionAction<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetBrokenConnectionAction(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ConnectClientDrivesAtLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::ConnectClientDrivesAtLogon(this) {
                    Ok(ok__) => {
                        pnewval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectClientDrivesAtLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetConnectClientDrivesAtLogon(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ConnectClientPrintersAtLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::ConnectClientPrintersAtLogon(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectClientPrintersAtLogon<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetConnectClientPrintersAtLogon(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn DefaultToMainPrinter<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::DefaultToMainPrinter(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultToMainPrinter<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetDefaultToMainPrinter(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn TerminalServicesWorkDirectory<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::TerminalServicesWorkDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTerminalServicesWorkDirectory<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetTerminalServicesWorkDirectory(this, core::mem::transmute(&pnewval)).into()
            }
        }
        unsafe extern "system" fn TerminalServicesInitialProgram<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IADsTSUserEx_Impl::TerminalServicesInitialProgram(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTerminalServicesInitialProgram<Identity: IADsTSUserEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnewval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IADsTSUserEx_Impl::SetTerminalServicesInitialProgram(this, core::mem::transmute(&pnewval)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            TerminalServicesProfilePath: TerminalServicesProfilePath::<Identity, OFFSET>,
            SetTerminalServicesProfilePath: SetTerminalServicesProfilePath::<Identity, OFFSET>,
            TerminalServicesHomeDirectory: TerminalServicesHomeDirectory::<Identity, OFFSET>,
            SetTerminalServicesHomeDirectory: SetTerminalServicesHomeDirectory::<Identity, OFFSET>,
            TerminalServicesHomeDrive: TerminalServicesHomeDrive::<Identity, OFFSET>,
            SetTerminalServicesHomeDrive: SetTerminalServicesHomeDrive::<Identity, OFFSET>,
            AllowLogon: AllowLogon::<Identity, OFFSET>,
            SetAllowLogon: SetAllowLogon::<Identity, OFFSET>,
            EnableRemoteControl: EnableRemoteControl::<Identity, OFFSET>,
            SetEnableRemoteControl: SetEnableRemoteControl::<Identity, OFFSET>,
            MaxDisconnectionTime: MaxDisconnectionTime::<Identity, OFFSET>,
            SetMaxDisconnectionTime: SetMaxDisconnectionTime::<Identity, OFFSET>,
            MaxConnectionTime: MaxConnectionTime::<Identity, OFFSET>,
            SetMaxConnectionTime: SetMaxConnectionTime::<Identity, OFFSET>,
            MaxIdleTime: MaxIdleTime::<Identity, OFFSET>,
            SetMaxIdleTime: SetMaxIdleTime::<Identity, OFFSET>,
            ReconnectionAction: ReconnectionAction::<Identity, OFFSET>,
            SetReconnectionAction: SetReconnectionAction::<Identity, OFFSET>,
            BrokenConnectionAction: BrokenConnectionAction::<Identity, OFFSET>,
            SetBrokenConnectionAction: SetBrokenConnectionAction::<Identity, OFFSET>,
            ConnectClientDrivesAtLogon: ConnectClientDrivesAtLogon::<Identity, OFFSET>,
            SetConnectClientDrivesAtLogon: SetConnectClientDrivesAtLogon::<Identity, OFFSET>,
            ConnectClientPrintersAtLogon: ConnectClientPrintersAtLogon::<Identity, OFFSET>,
            SetConnectClientPrintersAtLogon: SetConnectClientPrintersAtLogon::<Identity, OFFSET>,
            DefaultToMainPrinter: DefaultToMainPrinter::<Identity, OFFSET>,
            SetDefaultToMainPrinter: SetDefaultToMainPrinter::<Identity, OFFSET>,
            TerminalServicesWorkDirectory: TerminalServicesWorkDirectory::<Identity, OFFSET>,
            SetTerminalServicesWorkDirectory: SetTerminalServicesWorkDirectory::<Identity, OFFSET>,
            TerminalServicesInitialProgram: TerminalServicesInitialProgram::<Identity, OFFSET>,
            SetTerminalServicesInitialProgram: SetTerminalServicesInitialProgram::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IADsTSUserEx as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IADsTSUserEx {}
windows_core::imp::define_interface!(IAudioDeviceEndpoint, IAudioDeviceEndpoint_Vtbl, 0xd4952f5a_a0b2_4cc4_8b82_9358488dd8ac);
windows_core::imp::interface_hierarchy!(IAudioDeviceEndpoint, windows_core::IUnknown);
impl IAudioDeviceEndpoint {
    pub unsafe fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBuffer)(windows_core::Interface::as_raw(self), maxperiod, u32latencycoefficient).ok() }
    }
    pub unsafe fn GetRTCaps(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRTCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventDrivenCapable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEventDrivenCapable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteExclusiveModeParametersToSharedMemory)(windows_core::Interface::as_raw(self), htargetprocess, hnsperiod, hnsbufferduration, u32latencycoefficient, pu32sharedmemorysize as _, phsharedmemory as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioDeviceEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
    pub GetRTCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetEventDrivenCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub WriteExclusiveModeParametersToSharedMemory: unsafe extern "system" fn(*mut core::ffi::c_void, usize, i64, i64, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
}
pub trait IAudioDeviceEndpoint_Impl: windows_core::IUnknownImpl {
    fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::Result<()>;
    fn GetRTCaps(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetEventDrivenCapable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::Result<()>;
}
impl IAudioDeviceEndpoint_Vtbl {
    pub const fn new<Identity: IAudioDeviceEndpoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBuffer<Identity: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioDeviceEndpoint_Impl::SetBuffer(this, core::mem::transmute_copy(&maxperiod), core::mem::transmute_copy(&u32latencycoefficient)).into()
            }
        }
        unsafe extern "system" fn GetRTCaps<Identity: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisrtcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioDeviceEndpoint_Impl::GetRTCaps(this) {
                    Ok(ok__) => {
                        pbisrtcapable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventDrivenCapable<Identity: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbiseventcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioDeviceEndpoint_Impl::GetEventDrivenCapable(this) {
                    Ok(ok__) => {
                        pbiseventcapable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteExclusiveModeParametersToSharedMemory<Identity: IAudioDeviceEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioDeviceEndpoint_Impl::WriteExclusiveModeParametersToSharedMemory(this, core::mem::transmute_copy(&htargetprocess), core::mem::transmute_copy(&hnsperiod), core::mem::transmute_copy(&hnsbufferduration), core::mem::transmute_copy(&u32latencycoefficient), core::mem::transmute_copy(&pu32sharedmemorysize), core::mem::transmute_copy(&phsharedmemory)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBuffer: SetBuffer::<Identity, OFFSET>,
            GetRTCaps: GetRTCaps::<Identity, OFFSET>,
            GetEventDrivenCapable: GetEventDrivenCapable::<Identity, OFFSET>,
            WriteExclusiveModeParametersToSharedMemory: WriteExclusiveModeParametersToSharedMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioDeviceEndpoint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioDeviceEndpoint {}
windows_core::imp::define_interface!(IAudioEndpoint, IAudioEndpoint_Vtbl, 0x30a99515_1527_4451_af9f_00c5f0234daf);
windows_core::imp::interface_hierarchy!(IAudioEndpoint, windows_core::IUnknown);
impl IAudioEndpoint {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFrameFormat(&self) -> windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFramesPerPacket(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFramesPerPacket)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLatency(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamFlags(&self, streamflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStreamFlags)(windows_core::Interface::as_raw(self), streamflags).ok() }
    }
    pub unsafe fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), eventhandle).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFrameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFrameFormat: usize,
    pub GetFramesPerPacket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub SetStreamFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait IAudioEndpoint_Impl: windows_core::IUnknownImpl {
    fn GetFrameFormat(&self) -> windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX>;
    fn GetFramesPerPacket(&self) -> windows_core::Result<u32>;
    fn GetLatency(&self) -> windows_core::Result<i64>;
    fn SetStreamFlags(&self, streamflags: u32) -> windows_core::Result<()>;
    fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl IAudioEndpoint_Vtbl {
    pub const fn new<Identity: IAudioEndpoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameFormat<Identity: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformat: *mut *mut super::super::Media::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpoint_Impl::GetFrameFormat(this) {
                    Ok(ok__) => {
                        ppformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFramesPerPacket<Identity: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframesperpacket: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpoint_Impl::GetFramesPerPacket(this) {
                    Ok(ok__) => {
                        pframesperpacket.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLatency<Identity: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, platency: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpoint_Impl::GetLatency(this) {
                    Ok(ok__) => {
                        platency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamFlags<Identity: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpoint_Impl::SetStreamFlags(this, core::mem::transmute_copy(&streamflags)).into()
            }
        }
        unsafe extern "system" fn SetEventHandle<Identity: IAudioEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpoint_Impl::SetEventHandle(this, core::mem::transmute_copy(&eventhandle)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameFormat: GetFrameFormat::<Identity, OFFSET>,
            GetFramesPerPacket: GetFramesPerPacket::<Identity, OFFSET>,
            GetLatency: GetLatency::<Identity, OFFSET>,
            SetStreamFlags: SetStreamFlags::<Identity, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpoint as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for IAudioEndpoint {}
windows_core::imp::define_interface!(IAudioEndpointControl, IAudioEndpointControl_Vtbl, 0xc684b72a_6df4_4774_bdf9_76b77509b653);
windows_core::imp::interface_hierarchy!(IAudioEndpointControl, windows_core::IUnknown);
impl IAudioEndpointControl {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioEndpointControl_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl IAudioEndpointControl_Vtbl {
    pub const fn new<Identity: IAudioEndpointControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointControl_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointControl_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IAudioEndpointControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointControl_Impl::Stop(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointControl {}
windows_core::imp::define_interface!(IAudioEndpointRT, IAudioEndpointRT_Vtbl, 0xdfd2005f_a6e5_4d39_a265_939ada9fbb4d);
windows_core::imp::interface_hierarchy!(IAudioEndpointRT, windows_core::IUnknown);
impl IAudioEndpointRT {
    pub unsafe fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPadding)(windows_core::Interface::as_raw(self), ppadding as _, paecurrentposition as _) }
    }
    pub unsafe fn ProcessingComplete(&self) {
        unsafe { (windows_core::Interface::vtable(self).ProcessingComplete)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPinInactive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPinInactive)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetPinActive(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPinActive)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut AE_CURRENT_POSITION),
    pub ProcessingComplete: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetPinInactive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPinActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioEndpointRT_Impl: windows_core::IUnknownImpl {
    fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION);
    fn ProcessingComplete(&self);
    fn SetPinInactive(&self) -> windows_core::Result<()>;
    fn SetPinActive(&self) -> windows_core::Result<()>;
}
impl IAudioEndpointRT_Vtbl {
    pub const fn new<Identity: IAudioEndpointRT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentPadding<Identity: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointRT_Impl::GetCurrentPadding(this, core::mem::transmute_copy(&ppadding), core::mem::transmute_copy(&paecurrentposition))
            }
        }
        unsafe extern "system" fn ProcessingComplete<Identity: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointRT_Impl::ProcessingComplete(this)
            }
        }
        unsafe extern "system" fn SetPinInactive<Identity: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointRT_Impl::SetPinInactive(this).into()
            }
        }
        unsafe extern "system" fn SetPinActive<Identity: IAudioEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointRT_Impl::SetPinActive(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentPadding: GetCurrentPadding::<Identity, OFFSET>,
            ProcessingComplete: ProcessingComplete::<Identity, OFFSET>,
            SetPinInactive: SetPinInactive::<Identity, OFFSET>,
            SetPinActive: SetPinActive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointRT as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointRT {}
windows_core::imp::define_interface!(IAudioInputEndpointRT, IAudioInputEndpointRT_Vtbl, 0x8026ab61_92b2_43c1_a1df_5c37ebd08d82);
windows_core::imp::interface_hierarchy!(IAudioInputEndpointRT, windows_core::IUnknown);
impl IAudioInputEndpointRT {
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
        unsafe { (windows_core::Interface::vtable(self).GetInputDataPointer)(windows_core::Interface::as_raw(self), pconnectionproperty as _, paetimestamp as _) }
    }
    pub unsafe fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseInputDataPointer)(windows_core::Interface::as_raw(self), u32framecount, pdatapointer) }
    }
    pub unsafe fn PulseEndpoint(&self) {
        unsafe { (windows_core::Interface::vtable(self).PulseEndpoint)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub GetInputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    GetInputDataPointer: usize,
    pub ReleaseInputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize),
    pub PulseEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioInputEndpointRT_Impl: windows_core::IUnknownImpl {
    fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION);
    fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize);
    fn PulseEndpoint(&self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioInputEndpointRT_Vtbl {
    pub const fn new<Identity: IAudioInputEndpointRT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputDataPointer<Identity: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputEndpointRT_Impl::GetInputDataPointer(this, core::mem::transmute_copy(&pconnectionproperty), core::mem::transmute_copy(&paetimestamp))
            }
        }
        unsafe extern "system" fn ReleaseInputDataPointer<Identity: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32framecount: u32, pdatapointer: usize) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputEndpointRT_Impl::ReleaseInputDataPointer(this, core::mem::transmute_copy(&u32framecount), core::mem::transmute_copy(&pdatapointer))
            }
        }
        unsafe extern "system" fn PulseEndpoint<Identity: IAudioInputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputEndpointRT_Impl::PulseEndpoint(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputDataPointer: GetInputDataPointer::<Identity, OFFSET>,
            ReleaseInputDataPointer: ReleaseInputDataPointer::<Identity, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputEndpointRT as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioInputEndpointRT {}
windows_core::imp::define_interface!(IAudioOutputEndpointRT, IAudioOutputEndpointRT_Vtbl, 0x8fa906e4_c31c_4e31_932e_19a66385e9aa);
windows_core::imp::interface_hierarchy!(IAudioOutputEndpointRT, windows_core::IUnknown);
impl IAudioOutputEndpointRT {
    pub unsafe fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
        unsafe { (windows_core::Interface::vtable(self).GetOutputDataPointer)(windows_core::Interface::as_raw(self), u32framecount, paetimestamp) }
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
        unsafe { (windows_core::Interface::vtable(self).ReleaseOutputDataPointer)(windows_core::Interface::as_raw(self), pconnectionproperty) }
    }
    pub unsafe fn PulseEndpoint(&self) {
        unsafe { (windows_core::Interface::vtable(self).PulseEndpoint)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOutputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointer: usize,
    pub PulseEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub trait IAudioOutputEndpointRT_Impl: windows_core::IUnknownImpl {
    fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize;
    fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY);
    fn PulseEndpoint(&self);
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl IAudioOutputEndpointRT_Vtbl {
    pub const fn new<Identity: IAudioOutputEndpointRT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputDataPointer<Identity: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputEndpointRT_Impl::GetOutputDataPointer(this, core::mem::transmute_copy(&u32framecount), core::mem::transmute_copy(&paetimestamp))
            }
        }
        unsafe extern "system" fn ReleaseOutputDataPointer<Identity: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputEndpointRT_Impl::ReleaseOutputDataPointer(this, core::mem::transmute_copy(&pconnectionproperty))
            }
        }
        unsafe extern "system" fn PulseEndpoint<Identity: IAudioOutputEndpointRT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputEndpointRT_Impl::PulseEndpoint(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputDataPointer: GetOutputDataPointer::<Identity, OFFSET>,
            ReleaseOutputDataPointer: ReleaseOutputDataPointer::<Identity, OFFSET>,
            PulseEndpoint: PulseEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioOutputEndpointRT as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio_Apo")]
impl windows_core::RuntimeName for IAudioOutputEndpointRT {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRemoteDesktopClient, IRemoteDesktopClient_Vtbl, 0x57d25668_625a_4905_be4e_304caa13f89c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRemoteDesktopClient {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRemoteDesktopClient, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClient {
    pub unsafe fn Connect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Reconnect(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reconnect)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn Settings(&self) -> windows_core::Result<IRemoteDesktopClientSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Actions(&self) -> windows_core::Result<IRemoteDesktopClientActions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TouchPointer(&self) -> windows_core::Result<IRemoteDesktopClientTouchPointer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TouchPointer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteSavedCredentials(&self, servername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteSavedCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servername)).ok() }
    }
    pub unsafe fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateSessionDisplaySettings)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn attachEvent<P1>(&self, eventname: &windows_core::BSTR, callback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).attachEvent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(eventname), callback.param().abi()).ok() }
    }
    pub unsafe fn detachEvent<P1>(&self, eventname: &windows_core::BSTR, callback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).detachEvent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(eventname), callback.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reconnect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TouchPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteSavedCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateSessionDisplaySettings: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub attachEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub detachEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClient_Impl: super::Com::IDispatch_Impl {
    fn Connect(&self) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn Reconnect(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn Settings(&self) -> windows_core::Result<IRemoteDesktopClientSettings>;
    fn Actions(&self) -> windows_core::Result<IRemoteDesktopClientActions>;
    fn TouchPointer(&self) -> windows_core::Result<IRemoteDesktopClientTouchPointer>;
    fn DeleteSavedCredentials(&self, servername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> windows_core::Result<()>;
    fn attachEvent(&self, eventname: &windows_core::BSTR, callback: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
    fn detachEvent(&self, eventname: &windows_core::BSTR, callback: windows_core::Ref<super::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteDesktopClient_Vtbl {
    pub const fn new<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::Connect(this).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn Reconnect<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::Reconnect(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn Settings<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, settings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClient_Impl::Settings(this) {
                    Ok(ok__) => {
                        settings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Actions<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClient_Impl::Actions(this) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TouchPointer<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, touchpointer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClient_Impl::TouchPointer(this) {
                    Ok(ok__) => {
                        touchpointer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSavedCredentials<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::DeleteSavedCredentials(this, core::mem::transmute(&servername)).into()
            }
        }
        unsafe extern "system" fn UpdateSessionDisplaySettings<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: u32, height: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::UpdateSessionDisplaySettings(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn attachEvent<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::attachEvent(this, core::mem::transmute(&eventname), core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn detachEvent<Identity: IRemoteDesktopClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventname: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClient_Impl::detachEvent(this, core::mem::transmute(&eventname), core::mem::transmute_copy(&callback)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Reconnect: Reconnect::<Identity, OFFSET>,
            Settings: Settings::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            TouchPointer: TouchPointer::<Identity, OFFSET>,
            DeleteSavedCredentials: DeleteSavedCredentials::<Identity, OFFSET>,
            UpdateSessionDisplaySettings: UpdateSessionDisplaySettings::<Identity, OFFSET>,
            attachEvent: attachEvent::<Identity, OFFSET>,
            detachEvent: detachEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteDesktopClient {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRemoteDesktopClientActions, IRemoteDesktopClientActions_Vtbl, 0x7d54bc4e_1028_45d4_8b0a_b9b6bffba176);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRemoteDesktopClientActions {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRemoteDesktopClientActions, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientActions {
    pub unsafe fn SuspendScreenUpdates(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SuspendScreenUpdates)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ResumeScreenUpdates(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResumeScreenUpdates)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExecuteRemoteAction)(windows_core::Interface::as_raw(self), remoteaction).ok() }
    }
    pub unsafe fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapshot)(windows_core::Interface::as_raw(self), snapshotencoding, snapshotformat, snapshotwidth, snapshotheight, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientActions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SuspendScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteRemoteAction: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteActionType) -> windows_core::HRESULT,
    pub GetSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, SnapshotEncodingType, SnapshotFormatType, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientActions_Impl: super::Com::IDispatch_Impl {
    fn SuspendScreenUpdates(&self) -> windows_core::Result<()>;
    fn ResumeScreenUpdates(&self) -> windows_core::Result<()>;
    fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> windows_core::Result<()>;
    fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteDesktopClientActions_Vtbl {
    pub const fn new<Identity: IRemoteDesktopClientActions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SuspendScreenUpdates<Identity: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientActions_Impl::SuspendScreenUpdates(this).into()
            }
        }
        unsafe extern "system" fn ResumeScreenUpdates<Identity: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientActions_Impl::ResumeScreenUpdates(this).into()
            }
        }
        unsafe extern "system" fn ExecuteRemoteAction<Identity: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, remoteaction: RemoteActionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientActions_Impl::ExecuteRemoteAction(this, core::mem::transmute_copy(&remoteaction)).into()
            }
        }
        unsafe extern "system" fn GetSnapshot<Identity: IRemoteDesktopClientActions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32, snapshotdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientActions_Impl::GetSnapshot(this, core::mem::transmute_copy(&snapshotencoding), core::mem::transmute_copy(&snapshotformat), core::mem::transmute_copy(&snapshotwidth), core::mem::transmute_copy(&snapshotheight)) {
                    Ok(ok__) => {
                        snapshotdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SuspendScreenUpdates: SuspendScreenUpdates::<Identity, OFFSET>,
            ResumeScreenUpdates: ResumeScreenUpdates::<Identity, OFFSET>,
            ExecuteRemoteAction: ExecuteRemoteAction::<Identity, OFFSET>,
            GetSnapshot: GetSnapshot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientActions as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteDesktopClientActions {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRemoteDesktopClientSettings, IRemoteDesktopClientSettings_Vtbl, 0x48a0f2a7_2713_431f_bbac_6f4558e7d64d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRemoteDesktopClientSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRemoteDesktopClientSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientSettings {
    pub unsafe fn ApplySettings(&self, rdpfilecontents: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplySettings)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rdpfilecontents)).ok() }
    }
    pub unsafe fn RetrieveSettings(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveSettings)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetRdpProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRdpProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetRdpProperty(&self, propertyname: &windows_core::BSTR, value: &super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRdpProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), core::mem::transmute_copy(value)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ApplySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrieveSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetRdpProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetRdpProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetRdpProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetRdpProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientSettings_Impl: super::Com::IDispatch_Impl {
    fn ApplySettings(&self, rdpfilecontents: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RetrieveSettings(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRdpProperty(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn SetRdpProperty(&self, propertyname: &windows_core::BSTR, value: &super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteDesktopClientSettings_Vtbl {
    pub const fn new<Identity: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ApplySettings<Identity: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdpfilecontents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientSettings_Impl::ApplySettings(this, core::mem::transmute(&rdpfilecontents)).into()
            }
        }
        unsafe extern "system" fn RetrieveSettings<Identity: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rdpfilecontents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientSettings_Impl::RetrieveSettings(this) {
                    Ok(ok__) => {
                        rdpfilecontents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRdpProperty<Identity: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, value: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientSettings_Impl::GetRdpProperty(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRdpProperty<Identity: IRemoteDesktopClientSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, value: super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientSettings_Impl::SetRdpProperty(this, core::mem::transmute(&propertyname), core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ApplySettings: ApplySettings::<Identity, OFFSET>,
            RetrieveSettings: RetrieveSettings::<Identity, OFFSET>,
            GetRdpProperty: GetRdpProperty::<Identity, OFFSET>,
            SetRdpProperty: SetRdpProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientSettings as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteDesktopClientSettings {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IRemoteDesktopClientTouchPointer, IRemoteDesktopClientTouchPointer_Vtbl, 0x260ec22d_8cbc_44b5_9e88_2a37f6c93ae9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IRemoteDesktopClientTouchPointer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IRemoteDesktopClientTouchPointer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IRemoteDesktopClientTouchPointer {
    pub unsafe fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled).ok() }
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventsEnabled(&self, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventsEnabled)(windows_core::Interface::as_raw(self), eventsenabled).ok() }
    }
    pub unsafe fn EventsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPointerSpeed(&self, pointerspeed: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPointerSpeed)(windows_core::Interface::as_raw(self), pointerspeed).ok() }
    }
    pub unsafe fn PointerSpeed(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteDesktopClientTouchPointer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEventsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EventsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPointerSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PointerSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRemoteDesktopClientTouchPointer_Impl: super::Com::IDispatch_Impl {
    fn SetEnabled(&self, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEventsEnabled(&self, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn EventsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPointerSpeed(&self, pointerspeed: u32) -> windows_core::Result<()>;
    fn PointerSpeed(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRemoteDesktopClientTouchPointer_Vtbl {
    pub const fn new<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEnabled<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientTouchPointer_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientTouchPointer_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventsEnabled<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsenabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientTouchPointer_Impl::SetEventsEnabled(this, core::mem::transmute_copy(&eventsenabled)).into()
            }
        }
        unsafe extern "system" fn EventsEnabled<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventsenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientTouchPointer_Impl::EventsEnabled(this) {
                    Ok(ok__) => {
                        eventsenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPointerSpeed<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerspeed: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteDesktopClientTouchPointer_Impl::SetPointerSpeed(this, core::mem::transmute_copy(&pointerspeed)).into()
            }
        }
        unsafe extern "system" fn PointerSpeed<Identity: IRemoteDesktopClientTouchPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerspeed: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRemoteDesktopClientTouchPointer_Impl::PointerSpeed(this) {
                    Ok(ok__) => {
                        pointerspeed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEventsEnabled: SetEventsEnabled::<Identity, OFFSET>,
            EventsEnabled: EventsEnabled::<Identity, OFFSET>,
            SetPointerSpeed: SetPointerSpeed::<Identity, OFFSET>,
            PointerSpeed: PointerSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteDesktopClientTouchPointer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRemoteDesktopClientTouchPointer {}
windows_core::imp::define_interface!(IRemoteSystemAdditionalInfoProvider, IRemoteSystemAdditionalInfoProvider_Vtbl, 0xeeaa3d5f_ec63_4d27_af38_e86b1d7292cb);
windows_core::imp::interface_hierarchy!(IRemoteSystemAdditionalInfoProvider, windows_core::IUnknown);
impl IRemoteSystemAdditionalInfoProvider {
    pub unsafe fn GetAdditionalInfo<T>(&self, deduplicationid: *mut windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAdditionalInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(deduplicationid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdditionalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRemoteSystemAdditionalInfoProvider_Impl: windows_core::IUnknownImpl {
    fn GetAdditionalInfo(&self, deduplicationid: *mut windows_core::HSTRING, riid: *const windows_core::GUID, mapview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub const fn new<Identity: IRemoteSystemAdditionalInfoProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdditionalInfo<Identity: IRemoteSystemAdditionalInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deduplicationid: *mut *mut core::ffi::c_void, riid: *const windows_core::GUID, mapview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRemoteSystemAdditionalInfoProvider_Impl::GetAdditionalInfo(this, core::mem::transmute_copy(&deduplicationid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&mapview)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetAdditionalInfo: GetAdditionalInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteSystemAdditionalInfoProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRemoteSystemAdditionalInfoProvider {}
windows_core::imp::define_interface!(ITSGAccountingEngine, ITSGAccountingEngine_Vtbl, 0x4ce2a0c9_e874_4f1a_86f4_06bbb9115338);
windows_core::imp::interface_hierarchy!(ITSGAccountingEngine, windows_core::IUnknown);
impl ITSGAccountingEngine {
    pub unsafe fn DoAccounting(&self, accountingdatatype: AAAccountingDataType, accountingdata: &AAAccountingData) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DoAccounting)(windows_core::Interface::as_raw(self), accountingdatatype, core::mem::transmute_copy(accountingdata)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAccountingEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoAccounting: unsafe extern "system" fn(*mut core::ffi::c_void, AAAccountingDataType, AAAccountingData) -> windows_core::HRESULT,
}
pub trait ITSGAccountingEngine_Impl: windows_core::IUnknownImpl {
    fn DoAccounting(&self, accountingdatatype: AAAccountingDataType, accountingdata: &AAAccountingData) -> windows_core::Result<()>;
}
impl ITSGAccountingEngine_Vtbl {
    pub const fn new<Identity: ITSGAccountingEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DoAccounting<Identity: ITSGAccountingEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAccountingEngine_Impl::DoAccounting(this, core::mem::transmute_copy(&accountingdatatype), core::mem::transmute(&accountingdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), DoAccounting: DoAccounting::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAccountingEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGAccountingEngine {}
windows_core::imp::define_interface!(ITSGAuthenticateUserSink, ITSGAuthenticateUserSink_Vtbl, 0x2c3e2e73_a782_47f9_8dfb_77ee1ed27a03);
windows_core::imp::interface_hierarchy!(ITSGAuthenticateUserSink, windows_core::IUnknown);
impl ITSGAuthenticateUserSink {
    pub unsafe fn OnUserAuthenticated(&self, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUserAuthenticated)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(username), core::mem::transmute_copy(userdomain), context, usertoken).ok() }
    }
    pub unsafe fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUserAuthenticationFailed)(windows_core::Interface::as_raw(self), context, genericerrorcode, specificerrorcode).ok() }
    }
    pub unsafe fn ReauthenticateUser(&self, context: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReauthenticateUser)(windows_core::Interface::as_raw(self), context).ok() }
    }
    pub unsafe fn DisconnectUser(&self, context: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectUser)(windows_core::Interface::as_raw(self), context).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticateUserSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUserAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, usize, super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub OnUserAuthenticationFailed: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ReauthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DisconnectUser: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ITSGAuthenticateUserSink_Impl: windows_core::IUnknownImpl {
    fn OnUserAuthenticated(&self, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ReauthenticateUser(&self, context: usize) -> windows_core::Result<()>;
    fn DisconnectUser(&self, context: usize) -> windows_core::Result<()>;
}
impl ITSGAuthenticateUserSink_Vtbl {
    pub const fn new<Identity: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUserAuthenticated<Identity: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::ffi::c_void, userdomain: *mut core::ffi::c_void, context: usize, usertoken: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticateUserSink_Impl::OnUserAuthenticated(this, core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute_copy(&context), core::mem::transmute_copy(&usertoken)).into()
            }
        }
        unsafe extern "system" fn OnUserAuthenticationFailed<Identity: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticateUserSink_Impl::OnUserAuthenticationFailed(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&genericerrorcode), core::mem::transmute_copy(&specificerrorcode)).into()
            }
        }
        unsafe extern "system" fn ReauthenticateUser<Identity: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticateUserSink_Impl::ReauthenticateUser(this, core::mem::transmute_copy(&context)).into()
            }
        }
        unsafe extern "system" fn DisconnectUser<Identity: ITSGAuthenticateUserSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticateUserSink_Impl::DisconnectUser(this, core::mem::transmute_copy(&context)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUserAuthenticated: OnUserAuthenticated::<Identity, OFFSET>,
            OnUserAuthenticationFailed: OnUserAuthenticationFailed::<Identity, OFFSET>,
            ReauthenticateUser: ReauthenticateUser::<Identity, OFFSET>,
            DisconnectUser: DisconnectUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthenticateUserSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGAuthenticateUserSink {}
windows_core::imp::define_interface!(ITSGAuthenticationEngine, ITSGAuthenticationEngine_Vtbl, 0x9ee3e5bf_04ab_4691_998c_d7f622321a56);
windows_core::imp::interface_hierarchy!(ITSGAuthenticationEngine, windows_core::IUnknown);
impl ITSGAuthenticationEngine {
    pub unsafe fn AuthenticateUser<P4>(&self, mainsessionid: windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: P4) -> windows_core::Result<()>
    where
        P4: windows_core::Param<ITSGAuthenticateUserSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AuthenticateUser)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), cookiedata, numcookiebytes, context, psink.param().abi()).ok() }
    }
    pub unsafe fn CancelAuthentication(&self, mainsessionid: windows_core::GUID, context: usize) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelAuthentication)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), context).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthenticationEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AuthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const u8, u32, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, usize) -> windows_core::HRESULT,
}
pub trait ITSGAuthenticationEngine_Impl: windows_core::IUnknownImpl {
    fn AuthenticateUser(&self, mainsessionid: &windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: windows_core::Ref<ITSGAuthenticateUserSink>) -> windows_core::Result<()>;
    fn CancelAuthentication(&self, mainsessionid: &windows_core::GUID, context: usize) -> windows_core::Result<()>;
}
impl ITSGAuthenticationEngine_Vtbl {
    pub const fn new<Identity: ITSGAuthenticationEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AuthenticateUser<Identity: ITSGAuthenticationEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticationEngine_Impl::AuthenticateUser(this, core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&cookiedata), core::mem::transmute_copy(&numcookiebytes), core::mem::transmute_copy(&context), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn CancelAuthentication<Identity: ITSGAuthenticationEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, context: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthenticationEngine_Impl::CancelAuthentication(this, core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&context)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AuthenticateUser: AuthenticateUser::<Identity, OFFSET>,
            CancelAuthentication: CancelAuthentication::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthenticationEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGAuthenticationEngine {}
windows_core::imp::define_interface!(ITSGAuthorizeConnectionSink, ITSGAuthorizeConnectionSink_Vtbl, 0xc27ece33_7781_4318_98ef_1cf2da7b7005);
windows_core::imp::interface_hierarchy!(ITSGAuthorizeConnectionSink, windows_core::IUnknown);
impl ITSGAuthorizeConnectionSink {
    pub unsafe fn OnConnectionAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, pbsohresponse: &[u8], idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnConnectionAuthorized)(windows_core::Interface::as_raw(self), hrin, core::mem::transmute(mainsessionid), pbsohresponse.len().try_into().unwrap(), core::mem::transmute(pbsohresponse.as_ptr()), idletimeout, sessiontimeout, sessiontimeoutaction, trustclass, policyattributes).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeConnectionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionAuthorized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::GUID, u32, *const u8, u32, u32, SESSION_TIMEOUT_ACTION_TYPE, AATrustClassID, *const u32) -> windows_core::HRESULT,
}
pub trait ITSGAuthorizeConnectionSink_Impl: windows_core::IUnknownImpl {
    fn OnConnectionAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: &windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::Result<()>;
}
impl ITSGAuthorizeConnectionSink_Vtbl {
    pub const fn new<Identity: ITSGAuthorizeConnectionSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnectionAuthorized<Identity: ITSGAuthorizeConnectionSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, cbsohresponse: u32, pbsohresponse: *const u8, idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthorizeConnectionSink_Impl::OnConnectionAuthorized(this, core::mem::transmute_copy(&hrin), core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&cbsohresponse), core::mem::transmute_copy(&pbsohresponse), core::mem::transmute_copy(&idletimeout), core::mem::transmute_copy(&sessiontimeout), core::mem::transmute_copy(&sessiontimeoutaction), core::mem::transmute_copy(&trustclass), core::mem::transmute_copy(&policyattributes)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnectionAuthorized: OnConnectionAuthorized::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthorizeConnectionSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGAuthorizeConnectionSink {}
windows_core::imp::define_interface!(ITSGAuthorizeResourceSink, ITSGAuthorizeResourceSink_Vtbl, 0xfeddfcd4_fa12_4435_ae55_7ad1a9779af7);
windows_core::imp::interface_hierarchy!(ITSGAuthorizeResourceSink, windows_core::IUnknown);
impl ITSGAuthorizeResourceSink {
    pub unsafe fn OnChannelAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, subsessionid: i32, allowedresourcenames: &[windows_core::BSTR], failedresourcenames: &[windows_core::BSTR]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnChannelAuthorized)(windows_core::Interface::as_raw(self), hrin, core::mem::transmute(mainsessionid), subsessionid, core::mem::transmute(allowedresourcenames.as_ptr()), allowedresourcenames.len().try_into().unwrap(), core::mem::transmute(failedresourcenames.as_ptr()), failedresourcenames.len().try_into().unwrap()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGAuthorizeResourceSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChannelAuthorized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::GUID, i32, *const *mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITSGAuthorizeResourceSink_Impl: windows_core::IUnknownImpl {
    fn OnChannelAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: &windows_core::GUID, subsessionid: i32, allowedresourcenames: *const windows_core::BSTR, numallowedresourcenames: u32, failedresourcenames: *const windows_core::BSTR, numfailedresourcenames: u32) -> windows_core::Result<()>;
}
impl ITSGAuthorizeResourceSink_Vtbl {
    pub const fn new<Identity: ITSGAuthorizeResourceSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChannelAuthorized<Identity: ITSGAuthorizeResourceSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, subsessionid: i32, allowedresourcenames: *const *mut core::ffi::c_void, numallowedresourcenames: u32, failedresourcenames: *const *mut core::ffi::c_void, numfailedresourcenames: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGAuthorizeResourceSink_Impl::OnChannelAuthorized(this, core::mem::transmute_copy(&hrin), core::mem::transmute(&mainsessionid), core::mem::transmute_copy(&subsessionid), core::mem::transmute_copy(&allowedresourcenames), core::mem::transmute_copy(&numallowedresourcenames), core::mem::transmute_copy(&failedresourcenames), core::mem::transmute_copy(&numfailedresourcenames)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChannelAuthorized: OnChannelAuthorized::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGAuthorizeResourceSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGAuthorizeResourceSink {}
windows_core::imp::define_interface!(ITSGPolicyEngine, ITSGPolicyEngine_Vtbl, 0x8bc24f08_6223_42f4_a5b4_8e37cd135bbd);
windows_core::imp::interface_hierarchy!(ITSGPolicyEngine, windows_core::IUnknown);
impl ITSGPolicyEngine {
    pub unsafe fn AuthorizeConnection<P10>(&self, mainsessionid: windows_core::GUID, username: &windows_core::BSTR, authtype: AAAuthSchemes, clientmachineip: &windows_core::BSTR, clientmachinename: &windows_core::BSTR, sohdata: &[u8], cookiedata: &[u8], usertoken: super::super::Foundation::HANDLE_PTR, psink: P10) -> windows_core::Result<()>
    where
        P10: windows_core::Param<ITSGAuthorizeConnectionSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AuthorizeConnection)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), core::mem::transmute_copy(username), authtype, core::mem::transmute_copy(clientmachineip), core::mem::transmute_copy(clientmachinename), core::mem::transmute(sohdata.as_ptr()), sohdata.len().try_into().unwrap(), core::mem::transmute(cookiedata.as_ptr()), cookiedata.len().try_into().unwrap(), usertoken, psink.param().abi()).ok() }
    }
    pub unsafe fn AuthorizeResource<P11>(&self, mainsessionid: windows_core::GUID, subsessionid: i32, username: &windows_core::BSTR, resourcenames: &[windows_core::BSTR], alternateresourcenames: &[windows_core::BSTR], portnumber: u32, operation: &windows_core::BSTR, cookie: &[u8], psink: P11) -> windows_core::Result<()>
    where
        P11: windows_core::Param<ITSGAuthorizeResourceSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AuthorizeResource)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), subsessionid, core::mem::transmute_copy(username), core::mem::transmute(resourcenames.as_ptr()), resourcenames.len().try_into().unwrap(), core::mem::transmute(alternateresourcenames.as_ptr()), alternateresourcenames.len().try_into().unwrap(), portnumber, core::mem::transmute_copy(operation), core::mem::transmute(cookie.as_ptr()), cookie.len().try_into().unwrap(), psink.param().abi()).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsQuarantineEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsQuarantineEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITSGPolicyEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AuthorizeConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut core::ffi::c_void, AAAuthSchemes, *mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, super::super::Foundation::HANDLE_PTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthorizeResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsQuarantineEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITSGPolicyEngine_Impl: windows_core::IUnknownImpl {
    fn AuthorizeConnection(&self, mainsessionid: &windows_core::GUID, username: &windows_core::BSTR, authtype: AAAuthSchemes, clientmachineip: &windows_core::BSTR, clientmachinename: &windows_core::BSTR, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: windows_core::Ref<ITSGAuthorizeConnectionSink>) -> windows_core::Result<()>;
    fn AuthorizeResource(&self, mainsessionid: &windows_core::GUID, subsessionid: i32, username: &windows_core::BSTR, resourcenames: *const windows_core::BSTR, numresources: u32, alternateresourcenames: *const windows_core::BSTR, numalternateresourcename: u32, portnumber: u32, operation: &windows_core::BSTR, cookie: *const u8, numbytesincookie: u32, psink: windows_core::Ref<ITSGAuthorizeResourceSink>) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn IsQuarantineEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ITSGPolicyEngine_Vtbl {
    pub const fn new<Identity: ITSGPolicyEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AuthorizeConnection<Identity: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, username: *mut core::ffi::c_void, authtype: AAAuthSchemes, clientmachineip: *mut core::ffi::c_void, clientmachinename: *mut core::ffi::c_void, sohdata: *const u8, numsohbytes: u32, cookiedata: *const u8, numcookiebytes: u32, usertoken: super::super::Foundation::HANDLE_PTR, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGPolicyEngine_Impl::AuthorizeConnection(this, core::mem::transmute(&mainsessionid), core::mem::transmute(&username), core::mem::transmute_copy(&authtype), core::mem::transmute(&clientmachineip), core::mem::transmute(&clientmachinename), core::mem::transmute_copy(&sohdata), core::mem::transmute_copy(&numsohbytes), core::mem::transmute_copy(&cookiedata), core::mem::transmute_copy(&numcookiebytes), core::mem::transmute_copy(&usertoken), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn AuthorizeResource<Identity: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mainsessionid: windows_core::GUID, subsessionid: i32, username: *mut core::ffi::c_void, resourcenames: *const *mut core::ffi::c_void, numresources: u32, alternateresourcenames: *const *mut core::ffi::c_void, numalternateresourcename: u32, portnumber: u32, operation: *mut core::ffi::c_void, cookie: *const u8, numbytesincookie: u32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGPolicyEngine_Impl::AuthorizeResource(
                    this,
                    core::mem::transmute(&mainsessionid),
                    core::mem::transmute_copy(&subsessionid),
                    core::mem::transmute(&username),
                    core::mem::transmute_copy(&resourcenames),
                    core::mem::transmute_copy(&numresources),
                    core::mem::transmute_copy(&alternateresourcenames),
                    core::mem::transmute_copy(&numalternateresourcename),
                    core::mem::transmute_copy(&portnumber),
                    core::mem::transmute(&operation),
                    core::mem::transmute_copy(&cookie),
                    core::mem::transmute_copy(&numbytesincookie),
                    core::mem::transmute_copy(&psink),
                )
                .into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITSGPolicyEngine_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn IsQuarantineEnabled<Identity: ITSGPolicyEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quarantineenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITSGPolicyEngine_Impl::IsQuarantineEnabled(this) {
                    Ok(ok__) => {
                        quarantineenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AuthorizeConnection: AuthorizeConnection::<Identity, OFFSET>,
            AuthorizeResource: AuthorizeResource::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            IsQuarantineEnabled: IsQuarantineEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITSGPolicyEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITSGPolicyEngine {}
windows_core::imp::define_interface!(ITsSbBaseNotifySink, ITsSbBaseNotifySink_Vtbl, 0x808a6537_1282_4989_9e09_f43938b71722);
windows_core::imp::interface_hierarchy!(ITsSbBaseNotifySink, windows_core::IUnknown);
impl ITsSbBaseNotifySink {
    pub unsafe fn OnError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hrerror).ok() }
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnReportStatus)(windows_core::Interface::as_raw(self), messagetype, messageid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbBaseNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, CLIENT_MESSAGE_TYPE, u32) -> windows_core::HRESULT,
}
pub trait ITsSbBaseNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::Result<()>;
}
impl ITsSbBaseNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbBaseNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnError<Identity: ITsSbBaseNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbBaseNotifySink_Impl::OnError(this, core::mem::transmute_copy(&hrerror)).into()
            }
        }
        unsafe extern "system" fn OnReportStatus<Identity: ITsSbBaseNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbBaseNotifySink_Impl::OnReportStatus(this, core::mem::transmute_copy(&messagetype), core::mem::transmute_copy(&messageid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnError: OnError::<Identity, OFFSET>,
            OnReportStatus: OnReportStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbBaseNotifySink {}
windows_core::imp::define_interface!(ITsSbClientConnection, ITsSbClientConnection_Vtbl, 0x18857499_ad61_4b1b_b7df_cbcd41fb8338);
windows_core::imp::interface_hierarchy!(ITsSbClientConnection, windows_core::IUnknown);
impl ITsSbClientConnection {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialProgram)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn LoadBalanceResult(&self) -> windows_core::Result<ITsSbLoadBalanceResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadBalanceResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FarmName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FarmName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutContext(&self, contextid: &windows_core::BSTR, context: &super::Variant::VARIANT, existingcontext: Option<*mut super::Variant::VARIANT>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PutContext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(contextid), core::mem::transmute_copy(context), existingcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetContext(&self, contextid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(contextid), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Environment(&self) -> windows_core::Result<ITsSbEnvironment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Environment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ConnectionError(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_ConnectionError)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SamUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SamUserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn ClientConnectionPropertySet(&self) -> windows_core::Result<ITsSbClientConnectionPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientConnectionPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsFirstAssignment(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstAssignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RdFarmType(&self) -> windows_core::Result<RD_FARM_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RdFarmType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserSidString(&self) -> windows_core::Result<*mut i8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserSidString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisconnectedSession(&self) -> windows_core::Result<ITsSbSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisconnectedSession)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadBalanceResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::Variant::VARIANT, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutContext: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetContext: usize,
    pub Environment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectionError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SamUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ClientConnectionPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ClientConnectionPropertySet: usize,
    pub IsFirstAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RdFarmType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RD_FARM_TYPE) -> windows_core::HRESULT,
    pub UserSidString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i8) -> windows_core::HRESULT,
    pub GetDisconnectedSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbClientConnection_Impl: windows_core::IUnknownImpl {
    fn UserName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LoadBalanceResult(&self) -> windows_core::Result<ITsSbLoadBalanceResult>;
    fn FarmName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PutContext(&self, contextid: &windows_core::BSTR, context: &super::Variant::VARIANT, existingcontext: *mut super::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetContext(&self, contextid: &windows_core::BSTR) -> windows_core::Result<super::Variant::VARIANT>;
    fn Environment(&self) -> windows_core::Result<ITsSbEnvironment>;
    fn get_ConnectionError(&self) -> windows_core::Result<()>;
    fn SamUserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClientConnectionPropertySet(&self) -> windows_core::Result<ITsSbClientConnectionPropertySet>;
    fn IsFirstAssignment(&self) -> windows_core::Result<windows_core::BOOL>;
    fn RdFarmType(&self) -> windows_core::Result<RD_FARM_TYPE>;
    fn UserSidString(&self) -> windows_core::Result<*mut i8>;
    fn GetDisconnectedSession(&self) -> windows_core::Result<ITsSbSession>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbClientConnection_Vtbl {
    pub const fn new<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UserName<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::UserName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Domain<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::Domain(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitialProgram<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::InitialProgram(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadBalanceResult<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::LoadBalanceResult(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FarmName<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::FarmName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutContext<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextid: *mut core::ffi::c_void, context: super::Variant::VARIANT, existingcontext: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbClientConnection_Impl::PutContext(this, core::mem::transmute(&contextid), core::mem::transmute(&context), core::mem::transmute_copy(&existingcontext)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextid: *mut core::ffi::c_void, context: *mut super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::GetContext(this, core::mem::transmute(&contextid)) {
                    Ok(ok__) => {
                        context.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Environment<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::Environment(this) {
                    Ok(ok__) => {
                        ppenvironment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectionError<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbClientConnection_Impl::get_ConnectionError(this).into()
            }
        }
        unsafe extern "system" fn SamUserAccount<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::SamUserAccount(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClientConnectionPropertySet<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::ClientConnectionPropertySet(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFirstAssignment<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::IsFirstAssignment(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RdFarmType<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prdfarmtype: *mut RD_FARM_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::RdFarmType(this) {
                    Ok(ok__) => {
                        prdfarmtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserSidString<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszusersidstring: *mut *mut i8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::UserSidString(this) {
                    Ok(ok__) => {
                        pszusersidstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisconnectedSession<Identity: ITsSbClientConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbClientConnection_Impl::GetDisconnectedSession(this) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UserName: UserName::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            InitialProgram: InitialProgram::<Identity, OFFSET>,
            LoadBalanceResult: LoadBalanceResult::<Identity, OFFSET>,
            FarmName: FarmName::<Identity, OFFSET>,
            PutContext: PutContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            Environment: Environment::<Identity, OFFSET>,
            get_ConnectionError: get_ConnectionError::<Identity, OFFSET>,
            SamUserAccount: SamUserAccount::<Identity, OFFSET>,
            ClientConnectionPropertySet: ClientConnectionPropertySet::<Identity, OFFSET>,
            IsFirstAssignment: IsFirstAssignment::<Identity, OFFSET>,
            RdFarmType: RdFarmType::<Identity, OFFSET>,
            UserSidString: UserSidString::<Identity, OFFSET>,
            GetDisconnectedSession: GetDisconnectedSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbClientConnection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbClientConnection {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::define_interface!(ITsSbClientConnectionPropertySet, ITsSbClientConnectionPropertySet_Vtbl, 0xe51995b0_46d6_11dd_aa21_cedc55d89593);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::ops::Deref for ITsSbClientConnectionPropertySet {
    type Target = ITsSbPropertySet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::interface_hierarchy!(ITsSbClientConnectionPropertySet, windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbClientConnectionPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbClientConnectionPropertySet_Impl: ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbClientConnectionPropertySet_Vtbl {
    pub const fn new<Identity: ITsSbClientConnectionPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbClientConnectionPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbClientConnectionPropertySet {}
windows_core::imp::define_interface!(ITsSbEnvironment, ITsSbEnvironment_Vtbl, 0x8c87f7f7_bf51_4a5c_87bf_8e94fb6e2256);
windows_core::imp::interface_hierarchy!(ITsSbEnvironment, windows_core::IUnknown);
impl ITsSbEnvironment {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ServerWeight(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerWeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnvironmentPropertySet(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnvironmentPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetEnvironmentPropertySet<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironmentPropertySet>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironmentPropertySet)(windows_core::Interface::as_raw(self), pval.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServerWeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnvironmentPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnvironmentPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetEnvironmentPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetEnvironmentPropertySet: usize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbEnvironment_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ServerWeight(&self) -> windows_core::Result<u32>;
    fn EnvironmentPropertySet(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet>;
    fn SetEnvironmentPropertySet(&self, pval: windows_core::Ref<ITsSbEnvironmentPropertySet>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironment_Vtbl {
    pub const fn new<Identity: ITsSbEnvironment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbEnvironment_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServerWeight<Identity: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbEnvironment_Impl::ServerWeight(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnvironmentPropertySet<Identity: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbEnvironment_Impl::EnvironmentPropertySet(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertySet<Identity: ITsSbEnvironment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbEnvironment_Impl::SetEnvironmentPropertySet(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            ServerWeight: ServerWeight::<Identity, OFFSET>,
            EnvironmentPropertySet: EnvironmentPropertySet::<Identity, OFFSET>,
            SetEnvironmentPropertySet: SetEnvironmentPropertySet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbEnvironment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbEnvironment {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::define_interface!(ITsSbEnvironmentPropertySet, ITsSbEnvironmentPropertySet_Vtbl, 0xd0d1bf7e_7acf_11dd_a243_e51156d89593);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::ops::Deref for ITsSbEnvironmentPropertySet {
    type Target = ITsSbPropertySet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::interface_hierarchy!(ITsSbEnvironmentPropertySet, windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbEnvironmentPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbEnvironmentPropertySet_Impl: ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbEnvironmentPropertySet_Vtbl {
    pub const fn new<Identity: ITsSbEnvironmentPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbEnvironmentPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbEnvironmentPropertySet {}
windows_core::imp::define_interface!(ITsSbFilterPluginStore, ITsSbFilterPluginStore_Vtbl, 0x85b44b0f_ed78_413f_9702_fa6d3b5ee755);
windows_core::imp::interface_hierarchy!(ITsSbFilterPluginStore, windows_core::IUnknown);
impl ITsSbFilterPluginStore {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveProperties<P0>(&self, ppropertyset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbPropertySet>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveProperties)(windows_core::Interface::as_raw(self), ppropertyset.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnumerateProperties(&self) -> windows_core::Result<ITsSbPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumerateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DeleteProperties(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbFilterPluginStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SaveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SaveProperties: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub EnumerateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    EnumerateProperties: usize,
    pub DeleteProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbFilterPluginStore_Impl: windows_core::IUnknownImpl {
    fn SaveProperties(&self, ppropertyset: windows_core::Ref<ITsSbPropertySet>) -> windows_core::Result<()>;
    fn EnumerateProperties(&self) -> windows_core::Result<ITsSbPropertySet>;
    fn DeleteProperties(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbFilterPluginStore_Vtbl {
    pub const fn new<Identity: ITsSbFilterPluginStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SaveProperties<Identity: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropertyset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbFilterPluginStore_Impl::SaveProperties(this, core::mem::transmute_copy(&ppropertyset)).into()
            }
        }
        unsafe extern "system" fn EnumerateProperties<Identity: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbFilterPluginStore_Impl::EnumerateProperties(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteProperties<Identity: ITsSbFilterPluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbFilterPluginStore_Impl::DeleteProperties(this, core::mem::transmute(&propertyname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveProperties: SaveProperties::<Identity, OFFSET>,
            EnumerateProperties: EnumerateProperties::<Identity, OFFSET>,
            DeleteProperties: DeleteProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbFilterPluginStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbFilterPluginStore {}
windows_core::imp::define_interface!(ITsSbGenericNotifySink, ITsSbGenericNotifySink_Vtbl, 0x4c4c8c4f_300b_46ad_9164_8468a7e7568c);
windows_core::imp::interface_hierarchy!(ITsSbGenericNotifySink, windows_core::IUnknown);
impl ITsSbGenericNotifySink {
    pub unsafe fn OnCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnCompleted)(windows_core::Interface::as_raw(self), status).ok() }
    }
    pub unsafe fn GetWaitTimeout(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWaitTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGenericNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetWaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
pub trait ITsSbGenericNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetWaitTimeout(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
}
impl ITsSbGenericNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbGenericNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCompleted<Identity: ITsSbGenericNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGenericNotifySink_Impl::OnCompleted(this, core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn GetWaitTimeout<Identity: ITsSbGenericNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfttimeout: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbGenericNotifySink_Impl::GetWaitTimeout(this) {
                    Ok(ok__) => {
                        pfttimeout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnCompleted: OnCompleted::<Identity, OFFSET>,
            GetWaitTimeout: GetWaitTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbGenericNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbGenericNotifySink {}
windows_core::imp::define_interface!(ITsSbGlobalStore, ITsSbGlobalStore_Vtbl, 0x9ab60f7b_bd72_4d9f_8a3a_a0ea5574e635);
windows_core::imp::interface_hierarchy!(ITsSbGlobalStore, windows_core::IUnknown);
impl ITsSbGlobalStore {
    pub unsafe fn QueryTarget(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), core::mem::transmute_copy(targetname), core::mem::transmute_copy(farmname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QuerySessionBySessionId(&self, providername: &windows_core::BSTR, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySessionBySessionId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), dwsessionid, core::mem::transmute_copy(targetname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateFarms)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), pdwcount as _, pval as _).ok() }
    }
    pub unsafe fn EnumerateTargets(&self, providername: &windows_core::BSTR, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateTargets)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), core::mem::transmute_copy(farmname), core::mem::transmute_copy(envname), pdwcount as _, pval as _).ok() }
    }
    pub unsafe fn EnumerateEnvironmentsByProvider(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateEnvironmentsByProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), pdwcount as _, ppval as _).ok() }
    }
    pub unsafe fn EnumerateSessions(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateSessions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(providername), core::mem::transmute_copy(targetname), core::mem::transmute_copy(username), core::mem::transmute_copy(userdomain), core::mem::transmute_copy(poolname), core::mem::transmute_copy(initialprogram), psessionstate, pdwcount as _, ppval as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFarmProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(farmname), core::mem::transmute_copy(propertyname), core::mem::transmute(pvarvalue)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbGlobalStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub EnumerateTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateEnvironmentsByProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const TSSESSION_STATE, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFarmProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFarmProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbGlobalStore_Impl: windows_core::IUnknownImpl {
    fn QueryTarget(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&self, providername: &windows_core::BSTR, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession>;
    fn EnumerateFarms(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn EnumerateTargets(&self, providername: &windows_core::BSTR, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>;
    fn EnumerateEnvironmentsByProvider(&self, providername: &windows_core::BSTR, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()>;
    fn EnumerateSessions(&self, providername: &windows_core::BSTR, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>;
    fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbGlobalStore_Vtbl {
    pub const fn new<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryTarget<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbGlobalStore_Impl::QueryTarget(this, core::mem::transmute(&providername), core::mem::transmute(&targetname), core::mem::transmute(&farmname)) {
                    Ok(ok__) => {
                        pptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, dwsessionid: u32, targetname: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbGlobalStore_Impl::QuerySessionBySessionId(this, core::mem::transmute(&providername), core::mem::transmute_copy(&dwsessionid), core::mem::transmute(&targetname)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateFarms<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGlobalStore_Impl::EnumerateFarms(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn EnumerateTargets<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, envname: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGlobalStore_Impl::EnumerateTargets(this, core::mem::transmute(&providername), core::mem::transmute(&farmname), core::mem::transmute(&envname), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn EnumerateEnvironmentsByProvider<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, pdwcount: *mut u32, ppval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGlobalStore_Impl::EnumerateEnvironmentsByProvider(this, core::mem::transmute(&providername), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
            }
        }
        unsafe extern "system" fn EnumerateSessions<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providername: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, username: *mut core::ffi::c_void, userdomain: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, initialprogram: *mut core::ffi::c_void, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGlobalStore_Impl::EnumerateSessions(this, core::mem::transmute(&providername), core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute(&poolname), core::mem::transmute(&initialprogram), core::mem::transmute_copy(&psessionstate), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
            }
        }
        unsafe extern "system" fn GetFarmProperty<Identity: ITsSbGlobalStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pvarvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbGlobalStore_Impl::GetFarmProperty(this, core::mem::transmute(&farmname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryTarget: QueryTarget::<Identity, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, OFFSET>,
            EnumerateEnvironmentsByProvider: EnumerateEnvironmentsByProvider::<Identity, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbGlobalStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbGlobalStore {}
windows_core::imp::define_interface!(ITsSbLoadBalanceResult, ITsSbLoadBalanceResult_Vtbl, 0x24fdb7ac_fea6_11dc_9672_9a8956d89593);
windows_core::imp::interface_hierarchy!(ITsSbLoadBalanceResult, windows_core::IUnknown);
impl ITsSbLoadBalanceResult {
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalanceResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbLoadBalanceResult_Impl: windows_core::IUnknownImpl {
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITsSbLoadBalanceResult_Vtbl {
    pub const fn new<Identity: ITsSbLoadBalanceResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TargetName<Identity: ITsSbLoadBalanceResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbLoadBalanceResult_Impl::TargetName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TargetName: TargetName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalanceResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbLoadBalanceResult {}
windows_core::imp::define_interface!(ITsSbLoadBalancing, ITsSbLoadBalancing_Vtbl, 0x24329274_9eb7_11dc_ae98_f2b456d89593);
impl core::ops::Deref for ITsSbLoadBalancing {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbLoadBalancing, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbLoadBalancing {
    pub unsafe fn GetMostSuitableTarget<P0, P1>(&self, pconnection: P0, plbsink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbClientConnection>,
        P1: windows_core::Param<ITsSbLoadBalancingNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMostSuitableTarget)(windows_core::Interface::as_raw(self), pconnection.param().abi(), plbsink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancing_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub GetMostSuitableTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbLoadBalancing_Impl: ITsSbPlugin_Impl {
    fn GetMostSuitableTarget(&self, pconnection: windows_core::Ref<ITsSbClientConnection>, plbsink: windows_core::Ref<ITsSbLoadBalancingNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbLoadBalancing_Vtbl {
    pub const fn new<Identity: ITsSbLoadBalancing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMostSuitableTarget<Identity: ITsSbLoadBalancing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, plbsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbLoadBalancing_Impl::GetMostSuitableTarget(this, core::mem::transmute_copy(&pconnection), core::mem::transmute_copy(&plbsink)).into()
            }
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), GetMostSuitableTarget: GetMostSuitableTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalancing as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbLoadBalancing {}
windows_core::imp::define_interface!(ITsSbLoadBalancingNotifySink, ITsSbLoadBalancingNotifySink_Vtbl, 0x5f8a8297_3244_4e6a_958a_27c822c1e141);
impl core::ops::Deref for ITsSbLoadBalancingNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbLoadBalancingNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbLoadBalancingNotifySink {
    pub unsafe fn OnGetMostSuitableTarget<P0>(&self, plbresult: P0, fisnewconnection: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbLoadBalanceResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnGetMostSuitableTarget)(windows_core::Interface::as_raw(self), plbresult.param().abi(), fisnewconnection.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbLoadBalancingNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnGetMostSuitableTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITsSbLoadBalancingNotifySink_Impl: ITsSbBaseNotifySink_Impl {
    fn OnGetMostSuitableTarget(&self, plbresult: windows_core::Ref<ITsSbLoadBalanceResult>, fisnewconnection: windows_core::BOOL) -> windows_core::Result<()>;
}
impl ITsSbLoadBalancingNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbLoadBalancingNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnGetMostSuitableTarget<Identity: ITsSbLoadBalancingNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbresult: *mut core::ffi::c_void, fisnewconnection: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbLoadBalancingNotifySink_Impl::OnGetMostSuitableTarget(this, core::mem::transmute_copy(&plbresult), core::mem::transmute_copy(&fisnewconnection)).into()
            }
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnGetMostSuitableTarget: OnGetMostSuitableTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbLoadBalancingNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbLoadBalancingNotifySink {}
windows_core::imp::define_interface!(ITsSbOrchestration, ITsSbOrchestration_Vtbl, 0x64fc1172_9eb7_11dc_8b00_3aba56d89593);
impl core::ops::Deref for ITsSbOrchestration {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbOrchestration, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbOrchestration {
    pub unsafe fn PrepareTargetForConnect<P0, P1>(&self, pconnection: P0, porchestrationnotifysink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbClientConnection>,
        P1: windows_core::Param<ITsSbOrchestrationNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PrepareTargetForConnect)(windows_core::Interface::as_raw(self), pconnection.param().abi(), porchestrationnotifysink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestration_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub PrepareTargetForConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbOrchestration_Impl: ITsSbPlugin_Impl {
    fn PrepareTargetForConnect(&self, pconnection: windows_core::Ref<ITsSbClientConnection>, porchestrationnotifysink: windows_core::Ref<ITsSbOrchestrationNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbOrchestration_Vtbl {
    pub const fn new<Identity: ITsSbOrchestration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareTargetForConnect<Identity: ITsSbOrchestration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, porchestrationnotifysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbOrchestration_Impl::PrepareTargetForConnect(this, core::mem::transmute_copy(&pconnection), core::mem::transmute_copy(&porchestrationnotifysink)).into()
            }
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), PrepareTargetForConnect: PrepareTargetForConnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbOrchestration as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbOrchestration {}
windows_core::imp::define_interface!(ITsSbOrchestrationNotifySink, ITsSbOrchestrationNotifySink_Vtbl, 0x36c37d61_926b_442f_bca5_118c6d50dcf2);
impl core::ops::Deref for ITsSbOrchestrationNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbOrchestrationNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbOrchestrationNotifySink {
    pub unsafe fn OnReadyToConnect<P0>(&self, ptarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReadyToConnect)(windows_core::Interface::as_raw(self), ptarget.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbOrchestrationNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnReadyToConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbOrchestrationNotifySink_Impl: ITsSbBaseNotifySink_Impl {
    fn OnReadyToConnect(&self, ptarget: windows_core::Ref<ITsSbTarget>) -> windows_core::Result<()>;
}
impl ITsSbOrchestrationNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbOrchestrationNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnReadyToConnect<Identity: ITsSbOrchestrationNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbOrchestrationNotifySink_Impl::OnReadyToConnect(this, core::mem::transmute_copy(&ptarget)).into()
            }
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnReadyToConnect: OnReadyToConnect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbOrchestrationNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbOrchestrationNotifySink {}
windows_core::imp::define_interface!(ITsSbPlacement, ITsSbPlacement_Vtbl, 0xdaadee5f_6d32_480e_9e36_ddab2329f06d);
impl core::ops::Deref for ITsSbPlacement {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbPlacement, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbPlacement {
    pub unsafe fn QueryEnvironmentForTarget<P0, P1>(&self, pconnection: P0, pplacementsink: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbClientConnection>,
        P1: windows_core::Param<ITsSbPlacementNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryEnvironmentForTarget)(windows_core::Interface::as_raw(self), pconnection.param().abi(), pplacementsink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacement_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub QueryEnvironmentForTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlacement_Impl: ITsSbPlugin_Impl {
    fn QueryEnvironmentForTarget(&self, pconnection: windows_core::Ref<ITsSbClientConnection>, pplacementsink: windows_core::Ref<ITsSbPlacementNotifySink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlacement_Vtbl {
    pub const fn new<Identity: ITsSbPlacement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryEnvironmentForTarget<Identity: ITsSbPlacement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pplacementsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPlacement_Impl::QueryEnvironmentForTarget(this, core::mem::transmute_copy(&pconnection), core::mem::transmute_copy(&pplacementsink)).into()
            }
        }
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(), QueryEnvironmentForTarget: QueryEnvironmentForTarget::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlacement as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPlacement {}
windows_core::imp::define_interface!(ITsSbPlacementNotifySink, ITsSbPlacementNotifySink_Vtbl, 0x68a0c487_2b4f_46c2_94a1_6ce685183634);
impl core::ops::Deref for ITsSbPlacementNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbPlacementNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbPlacementNotifySink {
    pub unsafe fn OnQueryEnvironmentCompleted<P0>(&self, penvironment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnQueryEnvironmentCompleted)(windows_core::Interface::as_raw(self), penvironment.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlacementNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnQueryEnvironmentCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbPlacementNotifySink_Impl: ITsSbBaseNotifySink_Impl {
    fn OnQueryEnvironmentCompleted(&self, penvironment: windows_core::Ref<ITsSbEnvironment>) -> windows_core::Result<()>;
}
impl ITsSbPlacementNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbPlacementNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnQueryEnvironmentCompleted<Identity: ITsSbPlacementNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPlacementNotifySink_Impl::OnQueryEnvironmentCompleted(this, core::mem::transmute_copy(&penvironment)).into()
            }
        }
        Self { base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(), OnQueryEnvironmentCompleted: OnQueryEnvironmentCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlacementNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbPlacementNotifySink {}
windows_core::imp::define_interface!(ITsSbPlugin, ITsSbPlugin_Vtbl, 0x48cd7406_caab_465f_a5d6_baa863b9ea4f);
windows_core::imp::interface_hierarchy!(ITsSbPlugin, windows_core::IUnknown);
impl ITsSbPlugin {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbProvider>,
        P1: windows_core::Param<ITsSbPluginNotifySink>,
        P2: windows_core::Param<ITsSbPluginPropertySet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pprovider.param().abi(), pnotifysink.param().abi(), ppropertyset.param().abi()).ok() }
    }
    pub unsafe fn Terminate(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), hr).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbPlugin_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pprovider: windows_core::Ref<ITsSbProvider>, pnotifysink: windows_core::Ref<ITsSbPluginNotifySink>, ppropertyset: windows_core::Ref<ITsSbPluginPropertySet>) -> windows_core::Result<()>;
    fn Terminate(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPlugin_Vtbl {
    pub const fn new<Identity: ITsSbPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: ITsSbPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, pnotifysink: *mut core::ffi::c_void, ppropertyset: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPlugin_Impl::Initialize(this, core::mem::transmute_copy(&pprovider), core::mem::transmute_copy(&pnotifysink), core::mem::transmute_copy(&ppropertyset)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: ITsSbPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPlugin_Impl::Terminate(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbPlugin {}
windows_core::imp::define_interface!(ITsSbPluginNotifySink, ITsSbPluginNotifySink_Vtbl, 0x44dfe30b_c3be_40f5_bf82_7a95bb795adf);
impl core::ops::Deref for ITsSbPluginNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbPluginNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbPluginNotifySink {
    pub unsafe fn OnInitialized(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnInitialized)(windows_core::Interface::as_raw(self), hr).ok() }
    }
    pub unsafe fn OnTerminated(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTerminated)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnInitialized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnTerminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbPluginNotifySink_Impl: ITsSbBaseNotifySink_Impl {
    fn OnInitialized(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnTerminated(&self) -> windows_core::Result<()>;
}
impl ITsSbPluginNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbPluginNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInitialized<Identity: ITsSbPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPluginNotifySink_Impl::OnInitialized(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn OnTerminated<Identity: ITsSbPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbPluginNotifySink_Impl::OnTerminated(this).into()
            }
        }
        Self {
            base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(),
            OnInitialized: OnInitialized::<Identity, OFFSET>,
            OnTerminated: OnTerminated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPluginNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbPluginNotifySink {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::define_interface!(ITsSbPluginPropertySet, ITsSbPluginPropertySet_Vtbl, 0x95006e34_7eff_4b6c_bb40_49a4fda7cea6);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::ops::Deref for ITsSbPluginPropertySet {
    type Target = ITsSbPropertySet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::interface_hierarchy!(ITsSbPluginPropertySet, windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPluginPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbPluginPropertySet_Impl: ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbPluginPropertySet_Vtbl {
    pub const fn new<Identity: ITsSbPluginPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPluginPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbPluginPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::define_interface!(ITsSbPropertySet, ITsSbPropertySet_Vtbl, 0x5c025171_bb1e_4baf_a212_6d5e9774b33b);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::ops::Deref for ITsSbPropertySet {
    type Target = super::Com::StructuredStorage::IPropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::interface_hierarchy!(ITsSbPropertySet, windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbPropertySet_Vtbl {
    pub base__: super::Com::StructuredStorage::IPropertyBag_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbPropertySet_Impl: super::Com::StructuredStorage::IPropertyBag_Impl {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbPropertySet_Vtbl {
    pub const fn new<Identity: ITsSbPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::StructuredStorage::IPropertyBag_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbPropertySet {}
windows_core::imp::define_interface!(ITsSbProvider, ITsSbProvider_Vtbl, 0x87a4098f_6d7b_44dd_bc17_8ce44e370d52);
windows_core::imp::interface_hierarchy!(ITsSbProvider, windows_core::IUnknown);
impl ITsSbProvider {
    pub unsafe fn CreateTargetObject(&self, targetname: &windows_core::BSTR, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(environmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLoadBalanceResultObject(&self, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbLoadBalanceResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLoadBalanceResultObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSessionObject(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32) -> windows_core::Result<ITsSbSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSessionObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(username), core::mem::transmute_copy(domain), sessionid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreatePluginPropertySet(&self) -> windows_core::Result<ITsSbPluginPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePluginPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateTargetPropertySetObject(&self) -> windows_core::Result<ITsSbTargetPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTargetPropertySetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEnvironmentObject(&self, name: &windows_core::BSTR, serverweight: u32) -> windows_core::Result<ITsSbEnvironment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEnvironmentObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), serverweight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetResourcePluginStore(&self) -> windows_core::Result<ITsSbResourcePluginStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourcePluginStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFilterPluginStore(&self) -> windows_core::Result<ITsSbFilterPluginStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilterPluginStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterForNotification<P2>(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR, ppluginnotification: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITsSbResourceNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForNotification)(windows_core::Interface::as_raw(self), notificationtype, core::mem::transmute_copy(resourcetomonitor), ppluginnotification.param().abi()).ok() }
    }
    pub unsafe fn UnRegisterForNotification(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterForNotification)(windows_core::Interface::as_raw(self), notificationtype, core::mem::transmute_copy(resourcetomonitor)).ok() }
    }
    pub unsafe fn GetInstanceOfGlobalStore(&self) -> windows_core::Result<ITsSbGlobalStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceOfGlobalStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateEnvironmentPropertySetObject(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEnvironmentPropertySetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTargetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLoadBalanceResultObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSessionObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreatePluginPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreatePluginPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateTargetPropertySetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateTargetPropertySetObject: usize,
    pub CreateEnvironmentObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourcePluginStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilterPluginStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInstanceOfGlobalStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEnvironmentPropertySetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEnvironmentPropertySetObject: usize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvider_Impl: windows_core::IUnknownImpl {
    fn CreateTargetObject(&self, targetname: &windows_core::BSTR, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn CreateLoadBalanceResultObject(&self, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbLoadBalanceResult>;
    fn CreateSessionObject(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32) -> windows_core::Result<ITsSbSession>;
    fn CreatePluginPropertySet(&self) -> windows_core::Result<ITsSbPluginPropertySet>;
    fn CreateTargetPropertySetObject(&self) -> windows_core::Result<ITsSbTargetPropertySet>;
    fn CreateEnvironmentObject(&self, name: &windows_core::BSTR, serverweight: u32) -> windows_core::Result<ITsSbEnvironment>;
    fn GetResourcePluginStore(&self) -> windows_core::Result<ITsSbResourcePluginStore>;
    fn GetFilterPluginStore(&self) -> windows_core::Result<ITsSbFilterPluginStore>;
    fn RegisterForNotification(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR, ppluginnotification: windows_core::Ref<ITsSbResourceNotification>) -> windows_core::Result<()>;
    fn UnRegisterForNotification(&self, notificationtype: u32, resourcetomonitor: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetInstanceOfGlobalStore(&self) -> windows_core::Result<ITsSbGlobalStore>;
    fn CreateEnvironmentPropertySetObject(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbProvider_Vtbl {
    pub const fn new<Identity: ITsSbProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTargetObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, environmentname: *mut core::ffi::c_void, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateTargetObject(this, core::mem::transmute(&targetname), core::mem::transmute(&environmentname)) {
                    Ok(ok__) => {
                        pptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLoadBalanceResultObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, pplbresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateLoadBalanceResultObject(this, core::mem::transmute(&targetname)) {
                    Ok(ok__) => {
                        pplbresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSessionObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, username: *mut core::ffi::c_void, domain: *mut core::ffi::c_void, sessionid: u32, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateSessionObject(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute_copy(&sessionid)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePluginPropertySet<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreatePluginPropertySet(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTargetPropertySetObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateTargetPropertySetObject(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEnvironmentObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, serverweight: u32, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateEnvironmentObject(this, core::mem::transmute(&name), core::mem::transmute_copy(&serverweight)) {
                    Ok(ok__) => {
                        ppenvironment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourcePluginStore<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::GetResourcePluginStore(this) {
                    Ok(ok__) => {
                        ppstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilterPluginStore<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::GetFilterPluginStore(this) {
                    Ok(ok__) => {
                        ppstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterForNotification<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: u32, resourcetomonitor: *mut core::ffi::c_void, ppluginnotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvider_Impl::RegisterForNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute(&resourcetomonitor), core::mem::transmute_copy(&ppluginnotification)).into()
            }
        }
        unsafe extern "system" fn UnRegisterForNotification<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: u32, resourcetomonitor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvider_Impl::UnRegisterForNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute(&resourcetomonitor)).into()
            }
        }
        unsafe extern "system" fn GetInstanceOfGlobalStore<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppglobalstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::GetInstanceOfGlobalStore(this) {
                    Ok(ok__) => {
                        ppglobalstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEnvironmentPropertySetObject<Identity: ITsSbProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbProvider_Impl::CreateEnvironmentPropertySetObject(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTargetObject: CreateTargetObject::<Identity, OFFSET>,
            CreateLoadBalanceResultObject: CreateLoadBalanceResultObject::<Identity, OFFSET>,
            CreateSessionObject: CreateSessionObject::<Identity, OFFSET>,
            CreatePluginPropertySet: CreatePluginPropertySet::<Identity, OFFSET>,
            CreateTargetPropertySetObject: CreateTargetPropertySetObject::<Identity, OFFSET>,
            CreateEnvironmentObject: CreateEnvironmentObject::<Identity, OFFSET>,
            GetResourcePluginStore: GetResourcePluginStore::<Identity, OFFSET>,
            GetFilterPluginStore: GetFilterPluginStore::<Identity, OFFSET>,
            RegisterForNotification: RegisterForNotification::<Identity, OFFSET>,
            UnRegisterForNotification: UnRegisterForNotification::<Identity, OFFSET>,
            GetInstanceOfGlobalStore: GetInstanceOfGlobalStore::<Identity, OFFSET>,
            CreateEnvironmentPropertySetObject: CreateEnvironmentPropertySetObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbProvider {}
windows_core::imp::define_interface!(ITsSbProvisioning, ITsSbProvisioning_Vtbl, 0x2f6f0dbb_9e4f_462b_9c3f_fccc3dcb6232);
impl core::ops::Deref for ITsSbProvisioning {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbProvisioning, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbProvisioning {
    pub unsafe fn CreateVirtualMachines<P2>(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateVirtualMachines)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(jobxmlstring), core::mem::transmute_copy(jobguid), psink.param().abi()).ok() }
    }
    pub unsafe fn PatchVirtualMachines<P2>(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: P2, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).PatchVirtualMachines)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(jobxmlstring), core::mem::transmute_copy(jobguid), psink.param().abi(), pvmpatchinfo).ok() }
    }
    pub unsafe fn DeleteVirtualMachines<P2>(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteVirtualMachines)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(jobxmlstring), core::mem::transmute_copy(jobguid), psink.param().abi()).ok() }
    }
    pub unsafe fn CancelJob(&self, jobguid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelJob)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(jobguid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioning_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub CreateVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PatchVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const VM_PATCH_INFO) -> windows_core::HRESULT,
    pub DeleteVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelJob: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbProvisioning_Impl: ITsSbPlugin_Impl {
    fn CreateVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: windows_core::Ref<ITsSbProvisioningPluginNotifySink>) -> windows_core::Result<()>;
    fn PatchVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: windows_core::Ref<ITsSbProvisioningPluginNotifySink>, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::Result<()>;
    fn DeleteVirtualMachines(&self, jobxmlstring: &windows_core::BSTR, jobguid: &windows_core::BSTR, psink: windows_core::Ref<ITsSbProvisioningPluginNotifySink>) -> windows_core::Result<()>;
    fn CancelJob(&self, jobguid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbProvisioning_Vtbl {
    pub const fn new<Identity: ITsSbProvisioning_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateVirtualMachines<Identity: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: *mut core::ffi::c_void, jobguid: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioning_Impl::CreateVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn PatchVirtualMachines<Identity: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: *mut core::ffi::c_void, jobguid: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioning_Impl::PatchVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), core::mem::transmute_copy(&psink), core::mem::transmute_copy(&pvmpatchinfo)).into()
            }
        }
        unsafe extern "system" fn DeleteVirtualMachines<Identity: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobxmlstring: *mut core::ffi::c_void, jobguid: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioning_Impl::DeleteVirtualMachines(this, core::mem::transmute(&jobxmlstring), core::mem::transmute(&jobguid), core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn CancelJob<Identity: ITsSbProvisioning_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, jobguid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioning_Impl::CancelJob(this, core::mem::transmute(&jobguid)).into()
            }
        }
        Self {
            base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(),
            CreateVirtualMachines: CreateVirtualMachines::<Identity, OFFSET>,
            PatchVirtualMachines: PatchVirtualMachines::<Identity, OFFSET>,
            DeleteVirtualMachines: DeleteVirtualMachines::<Identity, OFFSET>,
            CancelJob: CancelJob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvisioning as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbProvisioning {}
windows_core::imp::define_interface!(ITsSbProvisioningPluginNotifySink, ITsSbProvisioningPluginNotifySink_Vtbl, 0xaca87a8e_818b_4581_a032_49c3dfb9c701);
windows_core::imp::interface_hierarchy!(ITsSbProvisioningPluginNotifySink, windows_core::IUnknown);
impl ITsSbProvisioningPluginNotifySink {
    pub unsafe fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnJobCreated)(windows_core::Interface::as_raw(self), pvmnotifyinfo).ok() }
    }
    pub unsafe fn OnVirtualMachineStatusChanged(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnVirtualMachineStatusChanged)(windows_core::Interface::as_raw(self), pvmnotifyentry, vmnotifystatus, errorcode, core::mem::transmute_copy(errordescr)).ok() }
    }
    pub unsafe fn OnJobCompleted(&self, resultcode: windows_core::HRESULT, resultdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnJobCompleted)(windows_core::Interface::as_raw(self), resultcode, core::mem::transmute_copy(resultdescription)).ok() }
    }
    pub unsafe fn OnJobCancelled(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnJobCancelled)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockVirtualMachine)(windows_core::Interface::as_raw(self), pvmnotifyentry).ok() }
    }
    pub unsafe fn OnVirtualMachineHostStatusChanged(&self, vmhost: &windows_core::BSTR, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnVirtualMachineHostStatusChanged)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vmhost), vmhostnotifystatus, errorcode, core::mem::transmute_copy(errordescr)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbProvisioningPluginNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnJobCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_INFO) -> windows_core::HRESULT,
    pub OnVirtualMachineStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_ENTRY, VM_NOTIFY_STATUS, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnJobCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnJobCancelled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockVirtualMachine: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_ENTRY) -> windows_core::HRESULT,
    pub OnVirtualMachineHostStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, VM_HOST_NOTIFY_STATUS, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbProvisioningPluginNotifySink_Impl: windows_core::IUnknownImpl {
    fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::Result<()>;
    fn OnVirtualMachineStatusChanged(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnJobCompleted(&self, resultcode: windows_core::HRESULT, resultdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnJobCancelled(&self) -> windows_core::Result<()>;
    fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::Result<()>;
    fn OnVirtualMachineHostStatusChanged(&self, vmhost: &windows_core::BSTR, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl ITsSbProvisioningPluginNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnJobCreated<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::OnJobCreated(this, core::mem::transmute_copy(&pvmnotifyinfo)).into()
            }
        }
        unsafe extern "system" fn OnVirtualMachineStatusChanged<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::OnVirtualMachineStatusChanged(this, core::mem::transmute_copy(&pvmnotifyentry), core::mem::transmute_copy(&vmnotifystatus), core::mem::transmute_copy(&errorcode), core::mem::transmute(&errordescr)).into()
            }
        }
        unsafe extern "system" fn OnJobCompleted<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultcode: windows_core::HRESULT, resultdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::OnJobCompleted(this, core::mem::transmute_copy(&resultcode), core::mem::transmute(&resultdescription)).into()
            }
        }
        unsafe extern "system" fn OnJobCancelled<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::OnJobCancelled(this).into()
            }
        }
        unsafe extern "system" fn LockVirtualMachine<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::LockVirtualMachine(this, core::mem::transmute_copy(&pvmnotifyentry)).into()
            }
        }
        unsafe extern "system" fn OnVirtualMachineHostStatusChanged<Identity: ITsSbProvisioningPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vmhost: *mut core::ffi::c_void, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbProvisioningPluginNotifySink_Impl::OnVirtualMachineHostStatusChanged(this, core::mem::transmute(&vmhost), core::mem::transmute_copy(&vmhostnotifystatus), core::mem::transmute_copy(&errorcode), core::mem::transmute(&errordescr)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnJobCreated: OnJobCreated::<Identity, OFFSET>,
            OnVirtualMachineStatusChanged: OnVirtualMachineStatusChanged::<Identity, OFFSET>,
            OnJobCompleted: OnJobCompleted::<Identity, OFFSET>,
            OnJobCancelled: OnJobCancelled::<Identity, OFFSET>,
            LockVirtualMachine: LockVirtualMachine::<Identity, OFFSET>,
            OnVirtualMachineHostStatusChanged: OnVirtualMachineHostStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbProvisioningPluginNotifySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbProvisioningPluginNotifySink {}
windows_core::imp::define_interface!(ITsSbResourceNotification, ITsSbResourceNotification_Vtbl, 0x65d3e85a_c39b_11dc_b92d_3cd255d89593);
windows_core::imp::interface_hierarchy!(ITsSbResourceNotification, windows_core::IUnknown);
impl ITsSbResourceNotification {
    pub unsafe fn NotifySessionChange<P1>(&self, changetype: TSSESSION_STATE, psession: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITsSbSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionChange)(windows_core::Interface::as_raw(self), changetype, psession.param().abi()).ok() }
    }
    pub unsafe fn NotifyTargetChange<P1>(&self, targetchangetype: u32, ptarget: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITsSbTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyTargetChange)(windows_core::Interface::as_raw(self), targetchangetype, ptarget.param().abi()).ok() }
    }
    pub unsafe fn NotifyClientConnectionStateChange<P1>(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITsSbClientConnection>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyClientConnectionStateChange)(windows_core::Interface::as_raw(self), changetype, pconnection.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifySessionChange: unsafe extern "system" fn(*mut core::ffi::c_void, TSSESSION_STATE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyTargetChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyClientConnectionStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, CONNECTION_CHANGE_NOTIFICATION, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbResourceNotification_Impl: windows_core::IUnknownImpl {
    fn NotifySessionChange(&self, changetype: TSSESSION_STATE, psession: windows_core::Ref<ITsSbSession>) -> windows_core::Result<()>;
    fn NotifyTargetChange(&self, targetchangetype: u32, ptarget: windows_core::Ref<ITsSbTarget>) -> windows_core::Result<()>;
    fn NotifyClientConnectionStateChange(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: windows_core::Ref<ITsSbClientConnection>) -> windows_core::Result<()>;
}
impl ITsSbResourceNotification_Vtbl {
    pub const fn new<Identity: ITsSbResourceNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifySessionChange<Identity: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: TSSESSION_STATE, psession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotification_Impl::NotifySessionChange(this, core::mem::transmute_copy(&changetype), core::mem::transmute_copy(&psession)).into()
            }
        }
        unsafe extern "system" fn NotifyTargetChange<Identity: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetchangetype: u32, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotification_Impl::NotifyTargetChange(this, core::mem::transmute_copy(&targetchangetype), core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn NotifyClientConnectionStateChange<Identity: ITsSbResourceNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotification_Impl::NotifyClientConnectionStateChange(this, core::mem::transmute_copy(&changetype), core::mem::transmute_copy(&pconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifySessionChange: NotifySessionChange::<Identity, OFFSET>,
            NotifyTargetChange: NotifyTargetChange::<Identity, OFFSET>,
            NotifyClientConnectionStateChange: NotifyClientConnectionStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourceNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbResourceNotification {}
windows_core::imp::define_interface!(ITsSbResourceNotificationEx, ITsSbResourceNotificationEx_Vtbl, 0xa8a47fde_ca91_44d2_b897_3aa28a43b2b7);
windows_core::imp::interface_hierarchy!(ITsSbResourceNotificationEx, windows_core::IUnknown);
impl ITsSbResourceNotificationEx {
    pub unsafe fn NotifySessionChangeEx(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionChangeEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(username), core::mem::transmute_copy(domain), sessionid, sessionstate).ok() }
    }
    pub unsafe fn NotifyTargetChangeEx(&self, targetname: &windows_core::BSTR, targetchangetype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyTargetChangeEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), targetchangetype).ok() }
    }
    pub unsafe fn NotifyClientConnectionStateChangeEx(&self, username: &windows_core::BSTR, domain: &windows_core::BSTR, initialprogram: &windows_core::BSTR, poolname: &windows_core::BSTR, targetname: &windows_core::BSTR, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyClientConnectionStateChangeEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(username), core::mem::transmute_copy(domain), core::mem::transmute_copy(initialprogram), core::mem::transmute_copy(poolname), core::mem::transmute_copy(targetname), connectionchangetype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourceNotificationEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifySessionChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, TSSESSION_STATE) -> windows_core::HRESULT,
    pub NotifyTargetChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NotifyClientConnectionStateChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CONNECTION_CHANGE_NOTIFICATION) -> windows_core::HRESULT,
}
pub trait ITsSbResourceNotificationEx_Impl: windows_core::IUnknownImpl {
    fn NotifySessionChangeEx(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, domain: &windows_core::BSTR, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::Result<()>;
    fn NotifyTargetChangeEx(&self, targetname: &windows_core::BSTR, targetchangetype: u32) -> windows_core::Result<()>;
    fn NotifyClientConnectionStateChangeEx(&self, username: &windows_core::BSTR, domain: &windows_core::BSTR, initialprogram: &windows_core::BSTR, poolname: &windows_core::BSTR, targetname: &windows_core::BSTR, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::Result<()>;
}
impl ITsSbResourceNotificationEx_Vtbl {
    pub const fn new<Identity: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifySessionChangeEx<Identity: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, username: *mut core::ffi::c_void, domain: *mut core::ffi::c_void, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotificationEx_Impl::NotifySessionChangeEx(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&sessionstate)).into()
            }
        }
        unsafe extern "system" fn NotifyTargetChangeEx<Identity: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, targetchangetype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotificationEx_Impl::NotifyTargetChangeEx(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&targetchangetype)).into()
            }
        }
        unsafe extern "system" fn NotifyClientConnectionStateChangeEx<Identity: ITsSbResourceNotificationEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::ffi::c_void, domain: *mut core::ffi::c_void, initialprogram: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourceNotificationEx_Impl::NotifyClientConnectionStateChangeEx(this, core::mem::transmute(&username), core::mem::transmute(&domain), core::mem::transmute(&initialprogram), core::mem::transmute(&poolname), core::mem::transmute(&targetname), core::mem::transmute_copy(&connectionchangetype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifySessionChangeEx: NotifySessionChangeEx::<Identity, OFFSET>,
            NotifyTargetChangeEx: NotifyTargetChangeEx::<Identity, OFFSET>,
            NotifyClientConnectionStateChangeEx: NotifyClientConnectionStateChangeEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourceNotificationEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbResourceNotificationEx {}
windows_core::imp::define_interface!(ITsSbResourcePlugin, ITsSbResourcePlugin_Vtbl, 0xea8db42c_98ed_4535_a88b_2a164f35490f);
impl core::ops::Deref for ITsSbResourcePlugin {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbResourcePlugin, windows_core::IUnknown, ITsSbPlugin);
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbResourcePlugin_Impl: ITsSbPlugin_Impl {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbResourcePlugin_Vtbl {
    pub const fn new<Identity: ITsSbResourcePlugin_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourcePlugin as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbResourcePlugin {}
windows_core::imp::define_interface!(ITsSbResourcePluginStore, ITsSbResourcePluginStore_Vtbl, 0x5c38f65f_bcf1_4036_a6bf_9e3cccae0b63);
windows_core::imp::interface_hierarchy!(ITsSbResourcePluginStore, windows_core::IUnknown);
impl ITsSbResourcePluginStore {
    pub unsafe fn QueryTarget(&self, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(farmname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn QuerySessionBySessionId(&self, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QuerySessionBySessionId)(windows_core::Interface::as_raw(self), dwsessionid, core::mem::transmute_copy(targetname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddTargetToStore<P0>(&self, ptarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTargetToStore)(windows_core::Interface::as_raw(self), ptarget.param().abi()).ok() }
    }
    pub unsafe fn AddSessionToStore<P0>(&self, psession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSessionToStore)(windows_core::Interface::as_raw(self), psession.param().abi()).ok() }
    }
    pub unsafe fn AddEnvironmentToStore<P0>(&self, penvironment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEnvironmentToStore)(windows_core::Interface::as_raw(self), penvironment.param().abi()).ok() }
    }
    pub unsafe fn RemoveEnvironmentFromStore(&self, environmentname: &windows_core::BSTR, bignoreowner: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveEnvironmentFromStore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(environmentname), bignoreowner.into()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateFarms)(windows_core::Interface::as_raw(self), pdwcount as _, pval as _).ok() }
    }
    pub unsafe fn QueryEnvironment(&self, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbEnvironment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryEnvironment)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(environmentname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateEnvironments)(windows_core::Interface::as_raw(self), pdwcount as _, pval as _).ok() }
    }
    pub unsafe fn SaveTarget<P0>(&self, ptarget: P0, bforcewrite: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveTarget)(windows_core::Interface::as_raw(self), ptarget.param().abi(), bforcewrite.into()).ok() }
    }
    pub unsafe fn SaveEnvironment<P0>(&self, penvironment: P0, bforcewrite: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveEnvironment)(windows_core::Interface::as_raw(self), penvironment.param().abi(), bforcewrite.into()).ok() }
    }
    pub unsafe fn SaveSession<P0>(&self, psession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveSession)(windows_core::Interface::as_raw(self), psession.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTargetProperty(&self, targetname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(propertyname), core::mem::transmute(pproperty)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEnvironmentProperty(&self, environmentname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironmentProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(environmentname), core::mem::transmute_copy(propertyname), core::mem::transmute(pproperty)).ok() }
    }
    pub unsafe fn SetTargetState(&self, targetname: &windows_core::BSTR, newstate: TARGET_STATE) -> windows_core::Result<TARGET_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetTargetState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), newstate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionState<P0>(&self, sbsession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSessionState)(windows_core::Interface::as_raw(self), sbsession.param().abi()).ok() }
    }
    pub unsafe fn EnumerateTargets(&self, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateTargets)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(farmname), core::mem::transmute_copy(envname), sortbyfieldid, core::mem::transmute_copy(sortybypropname), pdwcount as _, pval as _).ok() }
    }
    pub unsafe fn EnumerateSessions(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumerateSessions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(username), core::mem::transmute_copy(userdomain), core::mem::transmute_copy(poolname), core::mem::transmute_copy(initialprogram), psessionstate, pdwcount as _, ppval as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFarmProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(farmname), core::mem::transmute_copy(propertyname), core::mem::transmute(pvarvalue)).ok() }
    }
    pub unsafe fn DeleteTarget(&self, targetname: &windows_core::BSTR, hostname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteTarget)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), core::mem::transmute_copy(hostname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTargetPropertyWithVersionCheck<P0>(&self, ptarget: P0, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetPropertyWithVersionCheck)(windows_core::Interface::as_raw(self), ptarget.param().abi(), core::mem::transmute_copy(propertyname), core::mem::transmute(pproperty)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetEnvironmentPropertyWithVersionCheck<P0>(&self, penvironment: P0, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironmentPropertyWithVersionCheck)(windows_core::Interface::as_raw(self), penvironment.param().abi(), core::mem::transmute_copy(propertyname), core::mem::transmute(pproperty)).ok() }
    }
    pub unsafe fn AcquireTargetLock(&self, targetname: &windows_core::BSTR, dwtimeout: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AcquireTargetLock)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname), dwtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReleaseTargetLock<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleaseTargetLock)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok() }
    }
    pub unsafe fn TestAndSetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR, newstate: TARGET_STATE, teststate: TARGET_STATE) -> windows_core::Result<TARGET_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TestAndSetServerState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(poolname), core::mem::transmute_copy(serverfqdn), newstate, teststate, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServerWaitingToStart(&self, poolname: &windows_core::BSTR, servername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServerWaitingToStart)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(poolname), core::mem::transmute_copy(servername)).ok() }
    }
    pub unsafe fn GetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR) -> windows_core::Result<TARGET_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServerState)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(poolname), core::mem::transmute_copy(serverfqdn), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServerDrainMode(&self, serverfqdn: &windows_core::BSTR, drainmode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetServerDrainMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(serverfqdn), drainmode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbResourcePluginStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTargetToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSessionToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEnvironmentToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEnvironmentFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub QueryEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateEnvironments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTargetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTargetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEnvironmentProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEnvironmentProperty: usize,
    pub SetTargetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TARGET_STATE, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetSessionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateTargets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, TS_SB_SORT_BY, *mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const TSSESSION_STATE, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFarmProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFarmProperty: usize,
    pub DeleteTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetTargetPropertyWithVersionCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetTargetPropertyWithVersionCheck: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetEnvironmentPropertyWithVersionCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetEnvironmentPropertyWithVersionCheck: usize,
    pub AcquireTargetLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseTargetLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TestAndSetServerState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, TARGET_STATE, TARGET_STATE, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetServerWaitingToStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServerState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetServerDrainMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbResourcePluginStore_Impl: windows_core::IUnknownImpl {
    fn QueryTarget(&self, targetname: &windows_core::BSTR, farmname: &windows_core::BSTR) -> windows_core::Result<ITsSbTarget>;
    fn QuerySessionBySessionId(&self, dwsessionid: u32, targetname: &windows_core::BSTR) -> windows_core::Result<ITsSbSession>;
    fn AddTargetToStore(&self, ptarget: windows_core::Ref<ITsSbTarget>) -> windows_core::Result<()>;
    fn AddSessionToStore(&self, psession: windows_core::Ref<ITsSbSession>) -> windows_core::Result<()>;
    fn AddEnvironmentToStore(&self, penvironment: windows_core::Ref<ITsSbEnvironment>) -> windows_core::Result<()>;
    fn RemoveEnvironmentFromStore(&self, environmentname: &windows_core::BSTR, bignoreowner: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn QueryEnvironment(&self, environmentname: &windows_core::BSTR) -> windows_core::Result<ITsSbEnvironment>;
    fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()>;
    fn SaveTarget(&self, ptarget: windows_core::Ref<ITsSbTarget>, bforcewrite: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveEnvironment(&self, penvironment: windows_core::Ref<ITsSbEnvironment>, bforcewrite: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveSession(&self, psession: windows_core::Ref<ITsSbSession>) -> windows_core::Result<()>;
    fn SetTargetProperty(&self, targetname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetEnvironmentProperty(&self, environmentname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetTargetState(&self, targetname: &windows_core::BSTR, newstate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>;
    fn SetSessionState(&self, sbsession: windows_core::Ref<ITsSbSession>) -> windows_core::Result<()>;
    fn EnumerateTargets(&self, farmname: &windows_core::BSTR, envname: &windows_core::BSTR, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: &windows_core::BSTR, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>;
    fn EnumerateSessions(&self, targetname: &windows_core::BSTR, username: &windows_core::BSTR, userdomain: &windows_core::BSTR, poolname: &windows_core::BSTR, initialprogram: &windows_core::BSTR, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>;
    fn GetFarmProperty(&self, farmname: &windows_core::BSTR, propertyname: &windows_core::BSTR, pvarvalue: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn DeleteTarget(&self, targetname: &windows_core::BSTR, hostname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetTargetPropertyWithVersionCheck(&self, ptarget: windows_core::Ref<ITsSbTarget>, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetEnvironmentPropertyWithVersionCheck(&self, penvironment: windows_core::Ref<ITsSbEnvironment>, propertyname: &windows_core::BSTR, pproperty: *const super::Variant::VARIANT) -> windows_core::Result<()>;
    fn AcquireTargetLock(&self, targetname: &windows_core::BSTR, dwtimeout: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn ReleaseTargetLock(&self, pcontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn TestAndSetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR, newstate: TARGET_STATE, teststate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>;
    fn SetServerWaitingToStart(&self, poolname: &windows_core::BSTR, servername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetServerState(&self, poolname: &windows_core::BSTR, serverfqdn: &windows_core::BSTR) -> windows_core::Result<TARGET_STATE>;
    fn SetServerDrainMode(&self, serverfqdn: &windows_core::BSTR, drainmode: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbResourcePluginStore_Vtbl {
    pub const fn new<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryTarget<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, pptarget: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::QueryTarget(this, core::mem::transmute(&targetname), core::mem::transmute(&farmname)) {
                    Ok(ok__) => {
                        pptarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QuerySessionBySessionId<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsessionid: u32, targetname: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::QuerySessionBySessionId(this, core::mem::transmute_copy(&dwsessionid), core::mem::transmute(&targetname)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTargetToStore<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::AddTargetToStore(this, core::mem::transmute_copy(&ptarget)).into()
            }
        }
        unsafe extern "system" fn AddSessionToStore<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::AddSessionToStore(this, core::mem::transmute_copy(&psession)).into()
            }
        }
        unsafe extern "system" fn AddEnvironmentToStore<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::AddEnvironmentToStore(this, core::mem::transmute_copy(&penvironment)).into()
            }
        }
        unsafe extern "system" fn RemoveEnvironmentFromStore<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: *mut core::ffi::c_void, bignoreowner: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::RemoveEnvironmentFromStore(this, core::mem::transmute(&environmentname), core::mem::transmute_copy(&bignoreowner)).into()
            }
        }
        unsafe extern "system" fn EnumerateFarms<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::EnumerateFarms(this, core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn QueryEnvironment<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: *mut core::ffi::c_void, ppenvironment: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::QueryEnvironment(this, core::mem::transmute(&environmentname)) {
                    Ok(ok__) => {
                        ppenvironment.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumerateEnvironments<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::EnumerateEnvironments(this, core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn SaveTarget<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, bforcewrite: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SaveTarget(this, core::mem::transmute_copy(&ptarget), core::mem::transmute_copy(&bforcewrite)).into()
            }
        }
        unsafe extern "system" fn SaveEnvironment<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void, bforcewrite: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SaveEnvironment(this, core::mem::transmute_copy(&penvironment), core::mem::transmute_copy(&bforcewrite)).into()
            }
        }
        unsafe extern "system" fn SaveSession<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SaveSession(this, core::mem::transmute_copy(&psession)).into()
            }
        }
        unsafe extern "system" fn SetTargetProperty<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pproperty: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetTargetProperty(this, core::mem::transmute(&targetname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn SetEnvironmentProperty<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, environmentname: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pproperty: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetEnvironmentProperty(this, core::mem::transmute(&environmentname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn SetTargetState<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, newstate: TARGET_STATE, poldstate: *mut TARGET_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::SetTargetState(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&newstate)) {
                    Ok(ok__) => {
                        poldstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionState<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sbsession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetSessionState(this, core::mem::transmute_copy(&sbsession)).into()
            }
        }
        unsafe extern "system" fn EnumerateTargets<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, envname: *mut core::ffi::c_void, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: *mut core::ffi::c_void, pdwcount: *mut u32, pval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::EnumerateTargets(this, core::mem::transmute(&farmname), core::mem::transmute(&envname), core::mem::transmute_copy(&sortbyfieldid), core::mem::transmute(&sortybypropname), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn EnumerateSessions<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, username: *mut core::ffi::c_void, userdomain: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, initialprogram: *mut core::ffi::c_void, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::EnumerateSessions(this, core::mem::transmute(&targetname), core::mem::transmute(&username), core::mem::transmute(&userdomain), core::mem::transmute(&poolname), core::mem::transmute(&initialprogram), core::mem::transmute_copy(&psessionstate), core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&ppval)).into()
            }
        }
        unsafe extern "system" fn GetFarmProperty<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, farmname: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pvarvalue: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::GetFarmProperty(this, core::mem::transmute(&farmname), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pvarvalue)).into()
            }
        }
        unsafe extern "system" fn DeleteTarget<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, hostname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::DeleteTarget(this, core::mem::transmute(&targetname), core::mem::transmute(&hostname)).into()
            }
        }
        unsafe extern "system" fn SetTargetPropertyWithVersionCheck<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pproperty: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetTargetPropertyWithVersionCheck(this, core::mem::transmute_copy(&ptarget), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn SetEnvironmentPropertyWithVersionCheck<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penvironment: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, pproperty: *const super::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetEnvironmentPropertyWithVersionCheck(this, core::mem::transmute_copy(&penvironment), core::mem::transmute(&propertyname), core::mem::transmute_copy(&pproperty)).into()
            }
        }
        unsafe extern "system" fn AcquireTargetLock<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void, dwtimeout: u32, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::AcquireTargetLock(this, core::mem::transmute(&targetname), core::mem::transmute_copy(&dwtimeout)) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseTargetLock<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::ReleaseTargetLock(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn TestAndSetServerState<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, serverfqdn: *mut core::ffi::c_void, newstate: TARGET_STATE, teststate: TARGET_STATE, pinitstate: *mut TARGET_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::TestAndSetServerState(this, core::mem::transmute(&poolname), core::mem::transmute(&serverfqdn), core::mem::transmute_copy(&newstate), core::mem::transmute_copy(&teststate)) {
                    Ok(ok__) => {
                        pinitstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerWaitingToStart<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, servername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetServerWaitingToStart(this, core::mem::transmute(&poolname), core::mem::transmute(&servername)).into()
            }
        }
        unsafe extern "system" fn GetServerState<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poolname: *mut core::ffi::c_void, serverfqdn: *mut core::ffi::c_void, pstate: *mut TARGET_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbResourcePluginStore_Impl::GetServerState(this, core::mem::transmute(&poolname), core::mem::transmute(&serverfqdn)) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerDrainMode<Identity: ITsSbResourcePluginStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serverfqdn: *mut core::ffi::c_void, drainmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbResourcePluginStore_Impl::SetServerDrainMode(this, core::mem::transmute(&serverfqdn), core::mem::transmute_copy(&drainmode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryTarget: QueryTarget::<Identity, OFFSET>,
            QuerySessionBySessionId: QuerySessionBySessionId::<Identity, OFFSET>,
            AddTargetToStore: AddTargetToStore::<Identity, OFFSET>,
            AddSessionToStore: AddSessionToStore::<Identity, OFFSET>,
            AddEnvironmentToStore: AddEnvironmentToStore::<Identity, OFFSET>,
            RemoveEnvironmentFromStore: RemoveEnvironmentFromStore::<Identity, OFFSET>,
            EnumerateFarms: EnumerateFarms::<Identity, OFFSET>,
            QueryEnvironment: QueryEnvironment::<Identity, OFFSET>,
            EnumerateEnvironments: EnumerateEnvironments::<Identity, OFFSET>,
            SaveTarget: SaveTarget::<Identity, OFFSET>,
            SaveEnvironment: SaveEnvironment::<Identity, OFFSET>,
            SaveSession: SaveSession::<Identity, OFFSET>,
            SetTargetProperty: SetTargetProperty::<Identity, OFFSET>,
            SetEnvironmentProperty: SetEnvironmentProperty::<Identity, OFFSET>,
            SetTargetState: SetTargetState::<Identity, OFFSET>,
            SetSessionState: SetSessionState::<Identity, OFFSET>,
            EnumerateTargets: EnumerateTargets::<Identity, OFFSET>,
            EnumerateSessions: EnumerateSessions::<Identity, OFFSET>,
            GetFarmProperty: GetFarmProperty::<Identity, OFFSET>,
            DeleteTarget: DeleteTarget::<Identity, OFFSET>,
            SetTargetPropertyWithVersionCheck: SetTargetPropertyWithVersionCheck::<Identity, OFFSET>,
            SetEnvironmentPropertyWithVersionCheck: SetEnvironmentPropertyWithVersionCheck::<Identity, OFFSET>,
            AcquireTargetLock: AcquireTargetLock::<Identity, OFFSET>,
            ReleaseTargetLock: ReleaseTargetLock::<Identity, OFFSET>,
            TestAndSetServerState: TestAndSetServerState::<Identity, OFFSET>,
            SetServerWaitingToStart: SetServerWaitingToStart::<Identity, OFFSET>,
            GetServerState: GetServerState::<Identity, OFFSET>,
            SetServerDrainMode: SetServerDrainMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbResourcePluginStore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbResourcePluginStore {}
windows_core::imp::define_interface!(ITsSbServiceNotification, ITsSbServiceNotification_Vtbl, 0x86cb68ae_86e0_4f57_8a64_bb7406bc5550);
windows_core::imp::interface_hierarchy!(ITsSbServiceNotification, windows_core::IUnknown);
impl ITsSbServiceNotification {
    pub unsafe fn NotifyServiceFailure(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyServiceFailure)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NotifyServiceSuccess(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyServiceSuccess)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbServiceNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyServiceFailure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyServiceSuccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITsSbServiceNotification_Impl: windows_core::IUnknownImpl {
    fn NotifyServiceFailure(&self) -> windows_core::Result<()>;
    fn NotifyServiceSuccess(&self) -> windows_core::Result<()>;
}
impl ITsSbServiceNotification_Vtbl {
    pub const fn new<Identity: ITsSbServiceNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyServiceFailure<Identity: ITsSbServiceNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbServiceNotification_Impl::NotifyServiceFailure(this).into()
            }
        }
        unsafe extern "system" fn NotifyServiceSuccess<Identity: ITsSbServiceNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbServiceNotification_Impl::NotifyServiceSuccess(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyServiceFailure: NotifyServiceFailure::<Identity, OFFSET>,
            NotifyServiceSuccess: NotifyServiceSuccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbServiceNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbServiceNotification {}
windows_core::imp::define_interface!(ITsSbSession, ITsSbSession_Vtbl, 0xd453aac7_b1d8_4c5e_ba34_9afb4c8c5510);
windows_core::imp::interface_hierarchy!(ITsSbSession, windows_core::IUnknown);
impl ITsSbSession {
    pub unsafe fn SessionId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetName(&self, targetname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(targetname)).ok() }
    }
    pub unsafe fn Username(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Username)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<TSSESSION_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetState(&self, state: TSSESSION_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok() }
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreateTime(&self, time: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCreateTime)(windows_core::Interface::as_raw(self), core::mem::transmute(time)).ok() }
    }
    pub unsafe fn DisconnectTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisconnectTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisconnectTime(&self, time: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDisconnectTime)(windows_core::Interface::as_raw(self), core::mem::transmute(time)).ok() }
    }
    pub unsafe fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialProgram)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetInitialProgram(&self, application: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInitialProgram)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(application)).ok() }
    }
    pub unsafe fn ClientDisplay(&self) -> windows_core::Result<CLIENT_DISPLAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientDisplay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientDisplay(&self, pclientdisplay: CLIENT_DISPLAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClientDisplay)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientdisplay)).ok() }
    }
    pub unsafe fn ProtocolType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProtocolType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetProtocolType(&self, val: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProtocolType)(windows_core::Interface::as_raw(self), val).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Username: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TSSESSION_STATE) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, TSSESSION_STATE) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetCreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub DisconnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetDisconnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub InitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLIENT_DISPLAY) -> windows_core::HRESULT,
    pub SetClientDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, CLIENT_DISPLAY) -> windows_core::HRESULT,
    pub ProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ITsSbSession_Impl: windows_core::IUnknownImpl {
    fn SessionId(&self) -> windows_core::Result<u32>;
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetName(&self, targetname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Username(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<TSSESSION_STATE>;
    fn SetState(&self, state: TSSESSION_STATE) -> windows_core::Result<()>;
    fn CreateTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetCreateTime(&self, time: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn DisconnectTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn SetDisconnectTime(&self, time: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInitialProgram(&self, application: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientDisplay(&self) -> windows_core::Result<CLIENT_DISPLAY>;
    fn SetClientDisplay(&self, pclientdisplay: &CLIENT_DISPLAY) -> windows_core::Result<()>;
    fn ProtocolType(&self) -> windows_core::Result<u32>;
    fn SetProtocolType(&self, val: u32) -> windows_core::Result<()>;
}
impl ITsSbSession_Vtbl {
    pub const fn new<Identity: ITsSbSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SessionId<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::SessionId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetName<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::TargetName(this) {
                    Ok(ok__) => {
                        targetname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetName<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetTargetName(this, core::mem::transmute(&targetname)).into()
            }
        }
        unsafe extern "system" fn Username<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::Username(this) {
                    Ok(ok__) => {
                        username.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Domain<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, domain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::Domain(this) {
                    Ok(ok__) => {
                        domain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TSSESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::State(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetState<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: TSSESSION_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn CreateTime<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::CreateTime(this) {
                    Ok(ok__) => {
                        ptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreateTime<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetCreateTime(this, core::mem::transmute(&time)).into()
            }
        }
        unsafe extern "system" fn DisconnectTime<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::DisconnectTime(this) {
                    Ok(ok__) => {
                        ptime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisconnectTime<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetDisconnectTime(this, core::mem::transmute(&time)).into()
            }
        }
        unsafe extern "system" fn InitialProgram<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, app: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::InitialProgram(this) {
                    Ok(ok__) => {
                        app.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInitialProgram<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, application: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetInitialProgram(this, core::mem::transmute(&application)).into()
            }
        }
        unsafe extern "system" fn ClientDisplay<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdisplay: *mut CLIENT_DISPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::ClientDisplay(this) {
                    Ok(ok__) => {
                        pclientdisplay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientDisplay<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdisplay: CLIENT_DISPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetClientDisplay(this, core::mem::transmute(&pclientdisplay)).into()
            }
        }
        unsafe extern "system" fn ProtocolType<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbSession_Impl::ProtocolType(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtocolType<Identity: ITsSbSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbSession_Impl::SetProtocolType(this, core::mem::transmute_copy(&val)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SessionId: SessionId::<Identity, OFFSET>,
            TargetName: TargetName::<Identity, OFFSET>,
            SetTargetName: SetTargetName::<Identity, OFFSET>,
            Username: Username::<Identity, OFFSET>,
            Domain: Domain::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            CreateTime: CreateTime::<Identity, OFFSET>,
            SetCreateTime: SetCreateTime::<Identity, OFFSET>,
            DisconnectTime: DisconnectTime::<Identity, OFFSET>,
            SetDisconnectTime: SetDisconnectTime::<Identity, OFFSET>,
            InitialProgram: InitialProgram::<Identity, OFFSET>,
            SetInitialProgram: SetInitialProgram::<Identity, OFFSET>,
            ClientDisplay: ClientDisplay::<Identity, OFFSET>,
            SetClientDisplay: SetClientDisplay::<Identity, OFFSET>,
            ProtocolType: ProtocolType::<Identity, OFFSET>,
            SetProtocolType: SetProtocolType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITsSbSession {}
windows_core::imp::define_interface!(ITsSbTarget, ITsSbTarget_Vtbl, 0x16616ecc_272d_411d_b324_126893033856);
windows_core::imp::interface_hierarchy!(ITsSbTarget, windows_core::IUnknown);
impl ITsSbTarget {
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetName(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn FarmName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FarmName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFarmName(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFarmName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn TargetFQDN(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetFQDN)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetFQDN(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetFQDN)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn TargetNetbios(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetNetbios)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTargetNetbios(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetNetbios)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn get_IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).get_IpAddresses)(windows_core::Interface::as_raw(self), sockaddr as _, numaddresses as _).ok() }
    }
    pub unsafe fn put_IpAddresses(&self, sockaddr: &[TSSD_ConnectionPoint]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_IpAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute(sockaddr.as_ptr()), sockaddr.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn TargetState(&self) -> windows_core::Result<TARGET_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTargetState(&self, state: TARGET_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetState)(windows_core::Interface::as_raw(self), state).ok() }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn TargetPropertySet(&self) -> windows_core::Result<ITsSbTargetPropertySet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetTargetPropertySet<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTargetPropertySet>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetPropertySet)(windows_core::Interface::as_raw(self), pval.param().abi()).ok() }
    }
    pub unsafe fn EnvironmentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnvironmentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEnvironmentName(&self, val: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnvironmentName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)).ok() }
    }
    pub unsafe fn NumSessions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumSessions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumPendingConnections(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumPendingConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TargetLoad(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetLoad)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFarmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetFQDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetFQDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TargetNetbios: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetNetbios: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IpAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TSSD_ConnectionPoint, *mut u32) -> windows_core::HRESULT,
    pub put_IpAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *const TSSD_ConnectionPoint, u32) -> windows_core::HRESULT,
    pub TargetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetTargetState: unsafe extern "system" fn(*mut core::ffi::c_void, TARGET_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub TargetPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    TargetPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub SetTargetPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    SetTargetPropertySet: usize,
    pub EnvironmentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEnvironmentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NumPendingConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TargetLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTarget_Impl: windows_core::IUnknownImpl {
    fn TargetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FarmName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFarmName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetFQDN(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetFQDN(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TargetNetbios(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTargetNetbios(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::Result<()>;
    fn put_IpAddresses(&self, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> windows_core::Result<()>;
    fn TargetState(&self) -> windows_core::Result<TARGET_STATE>;
    fn SetTargetState(&self, state: TARGET_STATE) -> windows_core::Result<()>;
    fn TargetPropertySet(&self) -> windows_core::Result<ITsSbTargetPropertySet>;
    fn SetTargetPropertySet(&self, pval: windows_core::Ref<ITsSbTargetPropertySet>) -> windows_core::Result<()>;
    fn EnvironmentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEnvironmentName(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NumSessions(&self) -> windows_core::Result<u32>;
    fn NumPendingConnections(&self) -> windows_core::Result<u32>;
    fn TargetLoad(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTarget_Vtbl {
    pub const fn new<Identity: ITsSbTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TargetName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetTargetName(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn FarmName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::FarmName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFarmName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetFarmName(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn TargetFQDN<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetfqdnname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetFQDN(this) {
                    Ok(ok__) => {
                        targetfqdnname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetFQDN<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetTargetFQDN(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn TargetNetbios<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetnetbiosname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetNetbios(this) {
                    Ok(ok__) => {
                        targetnetbiosname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetNetbios<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetTargetNetbios(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn get_IpAddresses<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::get_IpAddresses(this, core::mem::transmute_copy(&sockaddr), core::mem::transmute_copy(&numaddresses)).into()
            }
        }
        unsafe extern "system" fn put_IpAddresses<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sockaddr: *const TSSD_ConnectionPoint, numaddresses: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::put_IpAddresses(this, core::mem::transmute_copy(&sockaddr), core::mem::transmute_copy(&numaddresses)).into()
            }
        }
        unsafe extern "system" fn TargetState<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TARGET_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetState(this) {
                    Ok(ok__) => {
                        pstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetState<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: TARGET_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetTargetState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn TargetPropertySet<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetPropertySet(this) {
                    Ok(ok__) => {
                        pppropertyset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetPropertySet<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetTargetPropertySet(this, core::mem::transmute_copy(&pval)).into()
            }
        }
        unsafe extern "system" fn EnvironmentName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::EnvironmentName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnvironmentName<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTarget_Impl::SetEnvironmentName(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn NumSessions<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumsessions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::NumSessions(this) {
                    Ok(ok__) => {
                        pnumsessions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumPendingConnections<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpendingconnections: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::NumPendingConnections(this) {
                    Ok(ok__) => {
                        pnumpendingconnections.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TargetLoad<Identity: ITsSbTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetload: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTarget_Impl::TargetLoad(this) {
                    Ok(ok__) => {
                        ptargetload.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetName: TargetName::<Identity, OFFSET>,
            SetTargetName: SetTargetName::<Identity, OFFSET>,
            FarmName: FarmName::<Identity, OFFSET>,
            SetFarmName: SetFarmName::<Identity, OFFSET>,
            TargetFQDN: TargetFQDN::<Identity, OFFSET>,
            SetTargetFQDN: SetTargetFQDN::<Identity, OFFSET>,
            TargetNetbios: TargetNetbios::<Identity, OFFSET>,
            SetTargetNetbios: SetTargetNetbios::<Identity, OFFSET>,
            get_IpAddresses: get_IpAddresses::<Identity, OFFSET>,
            put_IpAddresses: put_IpAddresses::<Identity, OFFSET>,
            TargetState: TargetState::<Identity, OFFSET>,
            SetTargetState: SetTargetState::<Identity, OFFSET>,
            TargetPropertySet: TargetPropertySet::<Identity, OFFSET>,
            SetTargetPropertySet: SetTargetPropertySet::<Identity, OFFSET>,
            EnvironmentName: EnvironmentName::<Identity, OFFSET>,
            SetEnvironmentName: SetEnvironmentName::<Identity, OFFSET>,
            NumSessions: NumSessions::<Identity, OFFSET>,
            NumPendingConnections: NumPendingConnections::<Identity, OFFSET>,
            TargetLoad: TargetLoad::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTarget as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbTarget {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::define_interface!(ITsSbTargetPropertySet, ITsSbTargetPropertySet_Vtbl, 0xf7bda5d6_994c_4e11_a079_2763b61830ac);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl core::ops::Deref for ITsSbTargetPropertySet {
    type Target = ITsSbPropertySet;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
windows_core::imp::interface_hierarchy!(ITsSbTargetPropertySet, windows_core::IUnknown, super::Com::StructuredStorage::IPropertyBag, ITsSbPropertySet);
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTargetPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITsSbTargetPropertySet_Impl: ITsSbPropertySet_Impl {}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITsSbTargetPropertySet_Vtbl {
    pub const fn new<Identity: ITsSbTargetPropertySet_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITsSbPropertySet_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTargetPropertySet as windows_core::Interface>::IID || iid == &<super::Com::StructuredStorage::IPropertyBag as windows_core::Interface>::IID || iid == &<ITsSbPropertySet as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITsSbTargetPropertySet {}
windows_core::imp::define_interface!(ITsSbTaskInfo, ITsSbTaskInfo_Vtbl, 0x523d1083_89be_48dd_99ea_04e82ffa7265);
windows_core::imp::interface_hierarchy!(ITsSbTaskInfo, windows_core::IUnknown);
impl ITsSbTaskInfo {
    pub unsafe fn TargetId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Deadline(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Identifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Context(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Plugin(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Plugin)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<RDV_TASK_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Context: usize,
    pub Plugin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RDV_TASK_STATUS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbTaskInfo_Impl: windows_core::IUnknownImpl {
    fn TargetId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StartTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn EndTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Deadline(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn Identifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Context(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn Plugin(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<RDV_TASK_STATUS>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbTaskInfo_Vtbl {
    pub const fn new<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TargetId<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::TargetId(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartTime<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::StartTime(this) {
                    Ok(ok__) => {
                        pstarttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndTime<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pendtime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::EndTime(this) {
                    Ok(ok__) => {
                        pendtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Deadline<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeadline: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Deadline(this) {
                    Ok(ok__) => {
                        pdeadline.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Identifier<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidentifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Identifier(this) {
                    Ok(ok__) => {
                        pidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Label<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Label(this) {
                    Ok(ok__) => {
                        plabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Context<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Context(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Plugin<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplugin: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Plugin(this) {
                    Ok(ok__) => {
                        pplugin.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: ITsSbTaskInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut RDV_TASK_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITsSbTaskInfo_Impl::Status(this) {
                    Ok(ok__) => {
                        pstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetId: TargetId::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            EndTime: EndTime::<Identity, OFFSET>,
            Deadline: Deadline::<Identity, OFFSET>,
            Identifier: Identifier::<Identity, OFFSET>,
            Label: Label::<Identity, OFFSET>,
            Context: Context::<Identity, OFFSET>,
            Plugin: Plugin::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbTaskInfo {}
windows_core::imp::define_interface!(ITsSbTaskPlugin, ITsSbTaskPlugin_Vtbl, 0xfa22ef0f_8705_41be_93bc_44bdbcf1c9c4);
impl core::ops::Deref for ITsSbTaskPlugin {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbTaskPlugin, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbTaskPlugin {
    pub unsafe fn InitializeTaskPlugin<P0>(&self, pitssbtaskpluginnotifysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTaskPluginNotifySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeTaskPlugin)(windows_core::Interface::as_raw(self), pitssbtaskpluginnotifysink.param().abi()).ok() }
    }
    pub unsafe fn SetTaskQueue(&self, pszhostname: &windows_core::BSTR, pitssbtaskinfo: &[Option<ITsSbTaskInfo>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTaskQueue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pszhostname), pitssbtaskinfo.len().try_into().unwrap(), core::mem::transmute(pitssbtaskinfo.as_ptr())).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub InitializeTaskPlugin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait ITsSbTaskPlugin_Impl: ITsSbPlugin_Impl {
    fn InitializeTaskPlugin(&self, pitssbtaskpluginnotifysink: windows_core::Ref<ITsSbTaskPluginNotifySink>) -> windows_core::Result<()>;
    fn SetTaskQueue(&self, pszhostname: &windows_core::BSTR, sbtaskinfosize: u32, pitssbtaskinfo: *const Option<ITsSbTaskInfo>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTaskPlugin_Vtbl {
    pub const fn new<Identity: ITsSbTaskPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeTaskPlugin<Identity: ITsSbTaskPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitssbtaskpluginnotifysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPlugin_Impl::InitializeTaskPlugin(this, core::mem::transmute_copy(&pitssbtaskpluginnotifysink)).into()
            }
        }
        unsafe extern "system" fn SetTaskQueue<Identity: ITsSbTaskPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhostname: *mut core::ffi::c_void, sbtaskinfosize: u32, pitssbtaskinfo: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPlugin_Impl::SetTaskQueue(this, core::mem::transmute(&pszhostname), core::mem::transmute_copy(&sbtaskinfosize), core::mem::transmute_copy(&pitssbtaskinfo)).into()
            }
        }
        Self {
            base__: ITsSbPlugin_Vtbl::new::<Identity, OFFSET>(),
            InitializeTaskPlugin: InitializeTaskPlugin::<Identity, OFFSET>,
            SetTaskQueue: SetTaskQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskPlugin as windows_core::Interface>::IID || iid == &<ITsSbPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for ITsSbTaskPlugin {}
windows_core::imp::define_interface!(ITsSbTaskPluginNotifySink, ITsSbTaskPluginNotifySink_Vtbl, 0x6aaf899e_c2ec_45ee_aa37_45e60895261a);
impl core::ops::Deref for ITsSbTaskPluginNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbTaskPluginNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbTaskPluginNotifySink {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSetTaskTime(&self, sztargetname: &windows_core::BSTR, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR, sztaskplugin: &windows_core::BSTR, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnSetTaskTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sztargetname), core::mem::transmute(taskstarttime), core::mem::transmute(taskendtime), core::mem::transmute(taskdeadline), core::mem::transmute_copy(sztasklabel), core::mem::transmute_copy(sztaskidentifier), core::mem::transmute_copy(sztaskplugin), dwtaskstatus, sacontext).ok() }
    }
    pub unsafe fn OnDeleteTaskTime(&self, sztargetname: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDeleteTaskTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sztargetname), core::mem::transmute_copy(sztaskidentifier)).ok() }
    }
    pub unsafe fn OnUpdateTaskStatus(&self, sztargetname: &windows_core::BSTR, taskidentifier: &windows_core::BSTR, taskstatus: RDV_TASK_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnUpdateTaskStatus)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sztargetname), core::mem::transmute_copy(taskidentifier), taskstatus).ok() }
    }
    pub unsafe fn OnReportTasks(&self, szhostname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnReportTasks)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(szhostname)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITsSbTaskPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetTaskTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::FILETIME, super::super::Foundation::FILETIME, super::super::Foundation::FILETIME, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetTaskTime: usize,
    pub OnDeleteTaskTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUpdateTaskStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, RDV_TASK_STATUS) -> windows_core::HRESULT,
    pub OnReportTasks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITsSbTaskPluginNotifySink_Impl: ITsSbBaseNotifySink_Impl {
    fn OnSetTaskTime(&self, sztargetname: &windows_core::BSTR, taskstarttime: &super::super::Foundation::FILETIME, taskendtime: &super::super::Foundation::FILETIME, taskdeadline: &super::super::Foundation::FILETIME, sztasklabel: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR, sztaskplugin: &windows_core::BSTR, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn OnDeleteTaskTime(&self, sztargetname: &windows_core::BSTR, sztaskidentifier: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnUpdateTaskStatus(&self, sztargetname: &windows_core::BSTR, taskidentifier: &windows_core::BSTR, taskstatus: RDV_TASK_STATUS) -> windows_core::Result<()>;
    fn OnReportTasks(&self, szhostname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITsSbTaskPluginNotifySink_Vtbl {
    pub const fn new<Identity: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetTaskTime<Identity: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: *mut core::ffi::c_void, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: *mut core::ffi::c_void, sztaskidentifier: *mut core::ffi::c_void, sztaskplugin: *mut core::ffi::c_void, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPluginNotifySink_Impl::OnSetTaskTime(this, core::mem::transmute(&sztargetname), core::mem::transmute(&taskstarttime), core::mem::transmute(&taskendtime), core::mem::transmute(&taskdeadline), core::mem::transmute(&sztasklabel), core::mem::transmute(&sztaskidentifier), core::mem::transmute(&sztaskplugin), core::mem::transmute_copy(&dwtaskstatus), core::mem::transmute_copy(&sacontext)).into()
            }
        }
        unsafe extern "system" fn OnDeleteTaskTime<Identity: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: *mut core::ffi::c_void, sztaskidentifier: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPluginNotifySink_Impl::OnDeleteTaskTime(this, core::mem::transmute(&sztargetname), core::mem::transmute(&sztaskidentifier)).into()
            }
        }
        unsafe extern "system" fn OnUpdateTaskStatus<Identity: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sztargetname: *mut core::ffi::c_void, taskidentifier: *mut core::ffi::c_void, taskstatus: RDV_TASK_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPluginNotifySink_Impl::OnUpdateTaskStatus(this, core::mem::transmute(&sztargetname), core::mem::transmute(&taskidentifier), core::mem::transmute_copy(&taskstatus)).into()
            }
        }
        unsafe extern "system" fn OnReportTasks<Identity: ITsSbTaskPluginNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szhostname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITsSbTaskPluginNotifySink_Impl::OnReportTasks(this, core::mem::transmute(&szhostname)).into()
            }
        }
        Self {
            base__: ITsSbBaseNotifySink_Vtbl::new::<Identity, OFFSET>(),
            OnSetTaskTime: OnSetTaskTime::<Identity, OFFSET>,
            OnDeleteTaskTime: OnDeleteTaskTime::<Identity, OFFSET>,
            OnUpdateTaskStatus: OnUpdateTaskStatus::<Identity, OFFSET>,
            OnReportTasks: OnReportTasks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITsSbTaskPluginNotifySink as windows_core::Interface>::IID || iid == &<ITsSbBaseNotifySink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITsSbTaskPluginNotifySink {}
windows_core::imp::define_interface!(IWRdsEnhancedFastReconnectArbitrator, IWRdsEnhancedFastReconnectArbitrator_Vtbl, 0x5718ae9b_47f2_499f_b634_d8175bd51131);
windows_core::imp::interface_hierarchy!(IWRdsEnhancedFastReconnectArbitrator, windows_core::IUnknown);
impl IWRdsEnhancedFastReconnectArbitrator {
    pub unsafe fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionForEnhancedFastReconnect)(windows_core::Interface::as_raw(self), psessionidarray, dwsessioncount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSessionForEnhancedFastReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32, *mut i32) -> windows_core::HRESULT,
}
pub trait IWRdsEnhancedFastReconnectArbitrator_Impl: windows_core::IUnknownImpl {
    fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> windows_core::Result<i32>;
}
impl IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub const fn new<Identity: IWRdsEnhancedFastReconnectArbitrator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionForEnhancedFastReconnect<Identity: IWRdsEnhancedFastReconnectArbitrator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionidarray: *const i32, dwsessioncount: u32, presultsessionid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsEnhancedFastReconnectArbitrator_Impl::GetSessionForEnhancedFastReconnect(this, core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&dwsessioncount)) {
                    Ok(ok__) => {
                        presultsessionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSessionForEnhancedFastReconnect: GetSessionForEnhancedFastReconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsEnhancedFastReconnectArbitrator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsEnhancedFastReconnectArbitrator {}
windows_core::imp::define_interface!(IWRdsGraphicsChannel, IWRdsGraphicsChannel_Vtbl, 0x684b7a0b_edff_43ad_d5a2_4a8d5388f401);
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannel, windows_core::IUnknown);
impl IWRdsGraphicsChannel {
    pub unsafe fn Write<P2>(&self, cbsize: u32, pbuffer: *const u8, pcontext: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cbsize, pbuffer, pcontext.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Open<P0, P1>(&self, pchannelevents: P0, popencontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsGraphicsChannelEvents>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pchannelevents.param().abi(), popencontext.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWRdsGraphicsChannel_Impl: windows_core::IUnknownImpl {
    fn Write(&self, cbsize: u32, pbuffer: *const u8, pcontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Open(&self, pchannelevents: windows_core::Ref<IWRdsGraphicsChannelEvents>, popencontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IWRdsGraphicsChannel_Vtbl {
    pub const fn new<Identity: IWRdsGraphicsChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannel_Impl::Write(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannel_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Open<Identity: IWRdsGraphicsChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelevents: *mut core::ffi::c_void, popencontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannel_Impl::Open(this, core::mem::transmute_copy(&pchannelevents), core::mem::transmute_copy(&popencontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsGraphicsChannel {}
windows_core::imp::define_interface!(IWRdsGraphicsChannelEvents, IWRdsGraphicsChannelEvents_Vtbl, 0x67f2368c_d674_4fae_66a5_d20628a640d2);
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelEvents, windows_core::IUnknown);
impl IWRdsGraphicsChannelEvents {
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDataReceived)(windows_core::Interface::as_raw(self), cbsize, pbuffer).ok() }
    }
    pub unsafe fn OnClose(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnChannelOpened<P1>(&self, openresult: windows_core::HRESULT, popencontext: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChannelOpened)(windows_core::Interface::as_raw(self), openresult, popencontext.param().abi()).ok() }
    }
    pub unsafe fn OnDataSent<P0>(&self, pwritecontext: P0, bcancelled: bool, pbuffer: *const u8, cbbuffer: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDataSent)(windows_core::Interface::as_raw(self), pwritecontext.param().abi(), bcancelled.into(), pbuffer, cbbuffer).ok() }
    }
    pub unsafe fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMetricsUpdate)(windows_core::Interface::as_raw(self), bandwidth, rtt, lastsentbyteindex).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnChannelOpened: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *const u8, u32) -> windows_core::HRESULT,
    pub OnMetricsUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u64) -> windows_core::HRESULT,
}
pub trait IWRdsGraphicsChannelEvents_Impl: windows_core::IUnknownImpl {
    fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
    fn OnChannelOpened(&self, openresult: windows_core::HRESULT, popencontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnDataSent(&self, pwritecontext: windows_core::Ref<windows_core::IUnknown>, bcancelled: windows_core::BOOL, pbuffer: *const u8, cbbuffer: u32) -> windows_core::Result<()>;
    fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::Result<()>;
}
impl IWRdsGraphicsChannelEvents_Vtbl {
    pub const fn new<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDataReceived<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannelEvents_Impl::OnDataReceived(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn OnClose<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannelEvents_Impl::OnClose(this).into()
            }
        }
        unsafe extern "system" fn OnChannelOpened<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, openresult: windows_core::HRESULT, popencontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannelEvents_Impl::OnChannelOpened(this, core::mem::transmute_copy(&openresult), core::mem::transmute_copy(&popencontext)).into()
            }
        }
        unsafe extern "system" fn OnDataSent<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwritecontext: *mut core::ffi::c_void, bcancelled: windows_core::BOOL, pbuffer: *const u8, cbbuffer: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannelEvents_Impl::OnDataSent(this, core::mem::transmute_copy(&pwritecontext), core::mem::transmute_copy(&bcancelled), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&cbbuffer)).into()
            }
        }
        unsafe extern "system" fn OnMetricsUpdate<Identity: IWRdsGraphicsChannelEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsGraphicsChannelEvents_Impl::OnMetricsUpdate(this, core::mem::transmute_copy(&bandwidth), core::mem::transmute_copy(&rtt), core::mem::transmute_copy(&lastsentbyteindex)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
            OnChannelOpened: OnChannelOpened::<Identity, OFFSET>,
            OnDataSent: OnDataSent::<Identity, OFFSET>,
            OnMetricsUpdate: OnMetricsUpdate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsGraphicsChannelEvents {}
windows_core::imp::define_interface!(IWRdsGraphicsChannelManager, IWRdsGraphicsChannelManager_Vtbl, 0x0fd57159_e83e_476a_a8b9_4a7976e71e18);
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelManager, windows_core::IUnknown);
impl IWRdsGraphicsChannelManager {
    pub unsafe fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> windows_core::Result<IWRdsGraphicsChannel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChannel)(windows_core::Interface::as_raw(self), pszchannelname, channeltype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsGraphicsChannelManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, WRdsGraphicsChannelType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWRdsGraphicsChannelManager_Impl: windows_core::IUnknownImpl {
    fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> windows_core::Result<IWRdsGraphicsChannel>;
}
impl IWRdsGraphicsChannelManager_Vtbl {
    pub const fn new<Identity: IWRdsGraphicsChannelManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateChannel<Identity: IWRdsGraphicsChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType, ppvirtualchannel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsGraphicsChannelManager_Impl::CreateChannel(this, core::mem::transmute_copy(&pszchannelname), core::mem::transmute_copy(&channeltype)) {
                    Ok(ok__) => {
                        ppvirtualchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateChannel: CreateChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsGraphicsChannelManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsGraphicsChannelManager {}
windows_core::imp::define_interface!(IWRdsProtocolConnection, IWRdsProtocolConnection_Vtbl, 0x324ed94f_fdaf_4ff6_81a8_42abe755830b);
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnection, windows_core::IUnknown);
impl IWRdsProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWRdsProtocolLogonErrorRedirector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogonErrorRedirector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AcceptConnection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcceptConnection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClientData)(windows_core::Interface::as_raw(self), pclientdata as _).ok() }
    }
    pub unsafe fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClientMonitorData)(windows_core::Interface::as_raw(self), pnummonitors as _, pprimarymonitor as _).ok() }
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserCredentials)(windows_core::Interface::as_raw(self), pusercreds as _).ok() }
    }
    pub unsafe fn GetLicenseConnection(&self) -> windows_core::Result<IWRdsProtocolLicenseConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLicenseConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AuthenticateClientToSession)(windows_core::Interface::as_raw(self), sessionid as _).ok() }
    }
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionId)(windows_core::Interface::as_raw(self), sessionid, sessionhandle).ok() }
    }
    pub unsafe fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetInputHandles)(windows_core::Interface::as_raw(self), pkeyboardhandle as _, pmousehandle as _, pbeephandle as _).ok() }
    }
    pub unsafe fn GetVideoHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectNotify)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn IsUserAllowedToLogon<P2, P3>(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: P2, pusername: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsUserAllowedToLogon)(windows_core::Interface::as_raw(self), sessionid, usertoken, pdomainname.param().abi(), pusername.param().abi()).ok() }
    }
    pub unsafe fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: bool, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(windows_core::Interface::as_raw(self), husertoken, bsinglesessionperuserenabled.into(), psessionidarray as _, pdwsessionidentifiercount as _).ok() }
    }
    pub unsafe fn LogonNotify<P1, P2>(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LogonNotify)(windows_core::Interface::as_raw(self), hclienttoken, wszusername.param().abi(), wszdomainname.param().abi(), sessionid, pwrdsconnectionsettings as _).ok() }
    }
    pub unsafe fn PreDisconnect(&self, disconnectreason: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PreDisconnect)(windows_core::Interface::as_raw(self), disconnectreason).ok() }
    }
    pub unsafe fn DisconnectNotify(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectNotify)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProtocolStatus)(windows_core::Interface::as_raw(self), pprotocolstatus as _).ok() }
    }
    pub unsafe fn GetLastInputTime(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastInputTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetErrorInfo)(windows_core::Interface::as_raw(self), ulerror).ok() }
    }
    pub unsafe fn CreateVirtualChannel<P0>(&self, szendpointname: P0, bstatic: bool, requestedpriority: u32) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), szendpointname.param().abi(), bstatic.into(), requestedpriority, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryProperty(&self, querytype: windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(querytype), ppropertyentriesin.len().try_into().unwrap(), ppropertyentriesout.len().try_into().unwrap(), core::mem::transmute(ppropertyentriesin.as_ptr()), core::mem::transmute(ppropertyentriesout.as_ptr())).ok() }
    }
    pub unsafe fn GetShadowConnection(&self) -> windows_core::Result<IWRdsProtocolShadowConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShadowConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NotifyCommandProcessCreated(&self, sessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyCommandProcessCreated)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcceptConnection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_CLIENT_DATA) -> windows_core::HRESULT,
    pub GetClientMonitorData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetUserCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID, super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub GetInputHandles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE_PTR, *mut super::super::Foundation::HANDLE_PTR, *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub GetVideoHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub ConnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsUserAllowedToLogon: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::BOOL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub LogonNotify: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTS_SESSION_ID, *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT,
    pub PreDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::BOOL, u32, *mut usize) -> windows_core::HRESULT,
    pub QueryProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *const WTS_PROPERTY_VALUE, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyCommandProcessCreated: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolConnection_Impl: windows_core::IUnknownImpl {
    fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWRdsProtocolLogonErrorRedirector>;
    fn AcceptConnection(&self) -> windows_core::Result<()>;
    fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()>;
    fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::Result<()>;
    fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()>;
    fn GetLicenseConnection(&self) -> windows_core::Result<IWRdsProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn GetVideoHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR>;
    fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()>;
    fn IsUserAllowedToLogon(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &windows_core::PCWSTR, pusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: windows_core::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>;
    fn LogonNotify(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &windows_core::PCWSTR, wszdomainname: &windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>;
    fn PreDisconnect(&self, disconnectreason: u32) -> windows_core::Result<()>;
    fn DisconnectNotify(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()>;
    fn GetLastInputTime(&self) -> windows_core::Result<u64>;
    fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()>;
    fn CreateVirtualChannel(&self, szendpointname: &windows_core::PCSTR, bstatic: windows_core::BOOL, requestedpriority: u32) -> windows_core::Result<usize>;
    fn QueryProperty(&self, querytype: &windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetShadowConnection(&self) -> windows_core::Result<IWRdsProtocolShadowConnection>;
    fn NotifyCommandProcessCreated(&self, sessionid: u32) -> windows_core::Result<()>;
}
impl IWRdsProtocolConnection_Vtbl {
    pub const fn new<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplogonerrorredir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::GetLogonErrorRedirector(this) {
                    Ok(ok__) => {
                        pplogonerrorredir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcceptConnection<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::AcceptConnection(this).into()
            }
        }
        unsafe extern "system" fn GetClientData<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::GetClientData(this, core::mem::transmute_copy(&pclientdata)).into()
            }
        }
        unsafe extern "system" fn GetClientMonitorData<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::GetClientMonitorData(this, core::mem::transmute_copy(&pnummonitors), core::mem::transmute_copy(&pprimarymonitor)).into()
            }
        }
        unsafe extern "system" fn GetUserCredentials<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::GetUserCredentials(this, core::mem::transmute_copy(&pusercreds)).into()
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicenseconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::GetLicenseConnection(this) {
                    Ok(ok__) => {
                        pplicenseconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::AuthenticateClientToSession(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionId<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, sessionhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::NotifySessionId(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&sessionhandle)).into()
            }
        }
        unsafe extern "system" fn GetInputHandles<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::GetInputHandles(this, core::mem::transmute_copy(&pkeyboardhandle), core::mem::transmute_copy(&pmousehandle), core::mem::transmute_copy(&pbeephandle)).into()
            }
        }
        unsafe extern "system" fn GetVideoHandle<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::GetVideoHandle(this) {
                    Ok(ok__) => {
                        pvideohandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectNotify<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::ConnectNotify(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: windows_core::PCWSTR, pusername: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::IsUserAllowedToLogon(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&usertoken), core::mem::transmute(&pdomainname), core::mem::transmute(&pusername)).into()
            }
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: windows_core::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::SessionArbitrationEnumeration(this, core::mem::transmute_copy(&husertoken), core::mem::transmute_copy(&bsinglesessionperuserenabled), core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
            }
        }
        unsafe extern "system" fn LogonNotify<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: windows_core::PCWSTR, wszdomainname: windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::LogonNotify(this, core::mem::transmute_copy(&hclienttoken), core::mem::transmute(&wszusername), core::mem::transmute(&wszdomainname), core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
            }
        }
        unsafe extern "system" fn PreDisconnect<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectreason: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::PreDisconnect(this, core::mem::transmute_copy(&disconnectreason)).into()
            }
        }
        unsafe extern "system" fn DisconnectNotify<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::DisconnectNotify(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::GetProtocolStatus(this, core::mem::transmute_copy(&pprotocolstatus)).into()
            }
        }
        unsafe extern "system" fn GetLastInputTime<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastinputtime: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::GetLastInputTime(this) {
                    Ok(ok__) => {
                        plastinputtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorInfo<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulerror: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::SetErrorInfo(this, core::mem::transmute_copy(&ulerror)).into()
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szendpointname: windows_core::PCSTR, bstatic: windows_core::BOOL, requestedpriority: u32, phchannel: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::CreateVirtualChannel(this, core::mem::transmute(&szendpointname), core::mem::transmute_copy(&bstatic), core::mem::transmute_copy(&requestedpriority)) {
                    Ok(ok__) => {
                        phchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryProperty<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querytype: windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::QueryProperty(this, core::mem::transmute(&querytype), core::mem::transmute_copy(&ulnumentriesin), core::mem::transmute_copy(&ulnumentriesout), core::mem::transmute_copy(&ppropertyentriesin), core::mem::transmute_copy(&ppropertyentriesout)).into()
            }
        }
        unsafe extern "system" fn GetShadowConnection<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshadowconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnection_Impl::GetShadowConnection(this) {
                    Ok(ok__) => {
                        ppshadowconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyCommandProcessCreated<Identity: IWRdsProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnection_Impl::NotifyCommandProcessCreated(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, OFFSET>,
            GetClientData: GetClientData::<Identity, OFFSET>,
            GetClientMonitorData: GetClientMonitorData::<Identity, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, OFFSET>,
            GetInputHandles: GetInputHandles::<Identity, OFFSET>,
            GetVideoHandle: GetVideoHandle::<Identity, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, OFFSET>,
            LogonNotify: LogonNotify::<Identity, OFFSET>,
            PreDisconnect: PreDisconnect::<Identity, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
            QueryProperty: QueryProperty::<Identity, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, OFFSET>,
            NotifyCommandProcessCreated: NotifyCommandProcessCreated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolConnection {}
windows_core::imp::define_interface!(IWRdsProtocolConnectionCallback, IWRdsProtocolConnectionCallback_Vtbl, 0xf1d70332_d070_4ef1_a088_78313536c2d6);
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionCallback, windows_core::IUnknown);
impl IWRdsProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnReady)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BrokenConnection)(windows_core::Interface::as_raw(self), reason, source).ok() }
    }
    pub unsafe fn StopScreenUpdates(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopScreenUpdates)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RedrawWindow)(windows_core::Interface::as_raw(self), rect).ok() }
    }
    pub unsafe fn GetConnectionId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SMALL_RECT) -> windows_core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolConnectionCallback_Impl: windows_core::IUnknownImpl {
    fn OnReady(&self) -> windows_core::Result<()>;
    fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()>;
    fn StopScreenUpdates(&self) -> windows_core::Result<()>;
    fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()>;
    fn GetConnectionId(&self) -> windows_core::Result<u32>;
}
impl IWRdsProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnReady<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionCallback_Impl::OnReady(this).into()
            }
        }
        unsafe extern "system" fn BrokenConnection<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: u32, source: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionCallback_Impl::BrokenConnection(this, core::mem::transmute_copy(&reason), core::mem::transmute_copy(&source)).into()
            }
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionCallback_Impl::StopScreenUpdates(this).into()
            }
        }
        unsafe extern "system" fn RedrawWindow<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionCallback_Impl::RedrawWindow(this, core::mem::transmute_copy(&rect)).into()
            }
        }
        unsafe extern "system" fn GetConnectionId<Identity: IWRdsProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectionid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolConnectionCallback_Impl::GetConnectionId(this) {
                    Ok(ok__) => {
                        pconnectionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnReady: OnReady::<Identity, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolConnectionCallback {}
windows_core::imp::define_interface!(IWRdsProtocolConnectionSettings, IWRdsProtocolConnectionSettings_Vtbl, 0x83fcf5d3_f6f4_ea94_9cd2_32f280e1e510);
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionSettings, windows_core::IUnknown);
impl IWRdsProtocolConnectionSettings {
    pub unsafe fn SetConnectionSetting(&self, propertyid: windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionSetting)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyid), ppropertyentriesin).ok() }
    }
    pub unsafe fn GetConnectionSetting(&self, propertyid: windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConnectionSetting)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyid), ppropertyentriesout as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolConnectionSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConnectionSetting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetConnectionSetting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolConnectionSettings_Impl: windows_core::IUnknownImpl {
    fn SetConnectionSetting(&self, propertyid: &windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetConnectionSetting(&self, propertyid: &windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
}
impl IWRdsProtocolConnectionSettings_Vtbl {
    pub const fn new<Identity: IWRdsProtocolConnectionSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConnectionSetting<Identity: IWRdsProtocolConnectionSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionSettings_Impl::SetConnectionSetting(this, core::mem::transmute(&propertyid), core::mem::transmute_copy(&ppropertyentriesin)).into()
            }
        }
        unsafe extern "system" fn GetConnectionSetting<Identity: IWRdsProtocolConnectionSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolConnectionSettings_Impl::GetConnectionSetting(this, core::mem::transmute(&propertyid), core::mem::transmute_copy(&ppropertyentriesout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConnectionSetting: SetConnectionSetting::<Identity, OFFSET>,
            GetConnectionSetting: GetConnectionSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolConnectionSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolConnectionSettings {}
windows_core::imp::define_interface!(IWRdsProtocolLicenseConnection, IWRdsProtocolLicenseConnection_Vtbl, 0x1d6a145f_d095_4424_957a_407fae822d84);
windows_core::imp::interface_hierarchy!(IWRdsProtocolLicenseConnection, windows_core::IUnknown);
impl IWRdsProtocolLicenseConnection {
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestLicensingCapabilities)(windows_core::Interface::as_raw(self), pplicensecapabilities as _, pcblicensecapabilities as _).ok() }
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(reserve1.as_ptr()), reserve1.len().try_into().unwrap(), ppclientlicense as _, pcbclientlicense as _).ok() }
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProtocolComplete)(windows_core::Interface::as_raw(self), ulcomplete).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLicenseConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestLicensingCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_LICENSE_CAPABILITIES, *mut u32) -> windows_core::HRESULT,
    pub SendClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolLicenseConnection_Impl: windows_core::IUnknownImpl {
    fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()>;
    fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::Result<()>;
    fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()>;
    fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()>;
}
impl IWRdsProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolLicenseConnection_Impl::RequestLicensingCapabilities(this, core::mem::transmute_copy(&pplicensecapabilities), core::mem::transmute_copy(&pcblicensecapabilities)).into()
            }
        }
        unsafe extern "system" fn SendClientLicense<Identity: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolLicenseConnection_Impl::SendClientLicense(this, core::mem::transmute_copy(&pclientlicense), core::mem::transmute_copy(&cbclientlicense)).into()
            }
        }
        unsafe extern "system" fn RequestClientLicense<Identity: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolLicenseConnection_Impl::RequestClientLicense(this, core::mem::transmute_copy(&reserve1), core::mem::transmute_copy(&reserve2), core::mem::transmute_copy(&ppclientlicense), core::mem::transmute_copy(&pcbclientlicense)).into()
            }
        }
        unsafe extern "system" fn ProtocolComplete<Identity: IWRdsProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomplete: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolLicenseConnection_Impl::ProtocolComplete(this, core::mem::transmute_copy(&ulcomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolLicenseConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolLicenseConnection {}
windows_core::imp::define_interface!(IWRdsProtocolListener, IWRdsProtocolListener_Vtbl, 0xfcbc131b_c686_451d_a773_e279e230f540);
windows_core::imp::interface_hierarchy!(IWRdsProtocolListener, windows_core::IUnknown);
impl IWRdsProtocolListener {
    pub unsafe fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> windows_core::Result<WRDS_LISTENER_SETTINGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), wrdslistenersettinglevel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsProtocolListenerCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartListen)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn StopListen(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopListen)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, WRDS_LISTENER_SETTING_LEVEL, *mut WRDS_LISTENER_SETTINGS) -> windows_core::HRESULT,
    pub StartListen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolListener_Impl: windows_core::IUnknownImpl {
    fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> windows_core::Result<WRDS_LISTENER_SETTINGS>;
    fn StartListen(&self, pcallback: windows_core::Ref<IWRdsProtocolListenerCallback>) -> windows_core::Result<()>;
    fn StopListen(&self) -> windows_core::Result<()>;
}
impl IWRdsProtocolListener_Vtbl {
    pub const fn new<Identity: IWRdsProtocolListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSettings<Identity: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL, pwrdslistenersettings: *mut WRDS_LISTENER_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolListener_Impl::GetSettings(this, core::mem::transmute_copy(&wrdslistenersettinglevel)) {
                    Ok(ok__) => {
                        pwrdslistenersettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartListen<Identity: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolListener_Impl::StartListen(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn StopListen<Identity: IWRdsProtocolListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolListener_Impl::StopListen(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            StartListen: StartListen::<Identity, OFFSET>,
            StopListen: StopListen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolListener as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolListener {}
windows_core::imp::define_interface!(IWRdsProtocolListenerCallback, IWRdsProtocolListenerCallback_Vtbl, 0x3ab27e5b_4449_4dc1_b74a_91621d4fe984);
windows_core::imp::interface_hierarchy!(IWRdsProtocolListenerCallback, windows_core::IUnknown);
impl IWRdsProtocolListenerCallback {
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> windows_core::Result<IWRdsProtocolConnectionCallback>
    where
        P0: windows_core::Param<IWRdsProtocolConnection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), pconnection.param().abi(), pwrdsconnectionsettings, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WRDS_CONNECTION_SETTINGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolListenerCallback_Impl: windows_core::IUnknownImpl {
    fn OnConnected(&self, pconnection: windows_core::Ref<IWRdsProtocolConnection>, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> windows_core::Result<IWRdsProtocolConnectionCallback>;
}
impl IWRdsProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: IWRdsProtocolListenerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnected<Identity: IWRdsProtocolListenerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS, pcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolListenerCallback_Impl::OnConnected(this, core::mem::transmute_copy(&pconnection), core::mem::transmute_copy(&pwrdsconnectionsettings)) {
                    Ok(ok__) => {
                        pcallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnected: OnConnected::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolListenerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolListenerCallback {}
windows_core::imp::define_interface!(IWRdsProtocolLogonErrorRedirector, IWRdsProtocolLogonErrorRedirector_Vtbl, 0x519fe83b_142a_4120_a3d5_a405d315281a);
windows_core::imp::interface_hierarchy!(IWRdsProtocolLogonErrorRedirector, windows_core::IUnknown);
impl IWRdsProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBeginPainting)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectStatus)(windows_core::Interface::as_raw(self), pszmessage.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectMessage)(windows_core::Interface::as_raw(self), pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectLogonError<P2, P3>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P2, pszmessage: P3, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectLogonError)(windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolLogonErrorRedirector_Impl: windows_core::IUnknownImpl {
    fn OnBeginPainting(&self) -> windows_core::Result<()>;
    fn RedirectStatus(&self, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&self, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBeginPainting<Identity: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolLogonErrorRedirector_Impl::OnBeginPainting(this).into()
            }
        }
        unsafe extern "system" fn RedirectStatus<Identity: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmessage: windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolLogonErrorRedirector_Impl::RedirectStatus(this, core::mem::transmute(&pszmessage)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectMessage<Identity: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolLogonErrorRedirector_Impl::RedirectMessage(this, core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectLogonError<Identity: IWRdsProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolLogonErrorRedirector_Impl::RedirectLogonError(this, core::mem::transmute_copy(&ntsstatus), core::mem::transmute_copy(&ntssubstatus), core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Identity, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolLogonErrorRedirector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolLogonErrorRedirector {}
windows_core::imp::define_interface!(IWRdsProtocolManager, IWRdsProtocolManager_Vtbl, 0xdc796967_3abb_40cd_a446_105276b58950);
windows_core::imp::interface_hierarchy!(IWRdsProtocolManager, windows_core::IUnknown);
impl IWRdsProtocolManager {
    pub unsafe fn Initialize<P0>(&self, piwrdssettings: P0, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsProtocolSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), piwrdssettings.param().abi(), pwrdssettings).ok() }
    }
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> windows_core::Result<IWRdsProtocolListener>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), wszlistenername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyServiceStateChange)(windows_core::Interface::as_raw(self), ptsservicestatechange).ok() }
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionStateChange)(windows_core::Interface::as_raw(self), sessionid, eventid).ok() }
    }
    pub unsafe fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySettingsChange)(windows_core::Interface::as_raw(self), pwrdssettings).ok() }
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WRDS_SETTINGS) -> windows_core::HRESULT,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SERVICE_STATE) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID, u32) -> windows_core::HRESULT,
    pub NotifySettingsChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WRDS_SETTINGS) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolManager_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, piwrdssettings: windows_core::Ref<IWRdsProtocolSettings>, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>;
    fn CreateListener(&self, wszlistenername: &windows_core::PCWSTR) -> windows_core::Result<IWRdsProtocolListener>;
    fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()>;
    fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
}
impl IWRdsProtocolManager_Vtbl {
    pub const fn new<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwrdssettings: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::Initialize(this, core::mem::transmute_copy(&piwrdssettings), core::mem::transmute_copy(&pwrdssettings)).into()
            }
        }
        unsafe extern "system" fn CreateListener<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlistenername: windows_core::PCWSTR, pprotocollistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWRdsProtocolManager_Impl::CreateListener(this, core::mem::transmute(&wszlistenername)) {
                    Ok(ok__) => {
                        pprotocollistener.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::NotifyServiceStateChange(this, core::mem::transmute_copy(&ptsservicestatechange)).into()
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::NotifySessionOfServiceStart(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::NotifySessionOfServiceStop(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::NotifySessionStateChange(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&eventid)).into()
            }
        }
        unsafe extern "system" fn NotifySettingsChange<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::NotifySettingsChange(this, core::mem::transmute_copy(&pwrdssettings)).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IWRdsProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolManager_Impl::Uninitialize(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            CreateListener: CreateListener::<Identity, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, OFFSET>,
            NotifySettingsChange: NotifySettingsChange::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolManager {}
windows_core::imp::define_interface!(IWRdsProtocolSettings, IWRdsProtocolSettings_Vtbl, 0x654a5a6a_2550_47eb_b6f7_ebd637475265);
windows_core::imp::interface_hierarchy!(IWRdsProtocolSettings, windows_core::IUnknown);
impl IWRdsProtocolSettings {
    pub unsafe fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), wrdssettingtype, wrdssettinglevel, pwrdssettings as _).ok() }
    }
    pub unsafe fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).MergeSettings)(windows_core::Interface::as_raw(self), pwrdssettings, wrdsconnectionsettinglevel, pwrdsconnectionsettings as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, WRDS_SETTING_TYPE, WRDS_SETTING_LEVEL, *mut WRDS_SETTINGS) -> windows_core::HRESULT,
    pub MergeSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *const WRDS_SETTINGS, WRDS_CONNECTION_SETTING_LEVEL, *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolSettings_Impl: windows_core::IUnknownImpl {
    fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::Result<()>;
    fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>;
}
impl IWRdsProtocolSettings_Vtbl {
    pub const fn new<Identity: IWRdsProtocolSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSettings<Identity: IWRdsProtocolSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolSettings_Impl::GetSettings(this, core::mem::transmute_copy(&wrdssettingtype), core::mem::transmute_copy(&wrdssettinglevel), core::mem::transmute_copy(&pwrdssettings)).into()
            }
        }
        unsafe extern "system" fn MergeSettings<Identity: IWRdsProtocolSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolSettings_Impl::MergeSettings(this, core::mem::transmute_copy(&pwrdssettings), core::mem::transmute_copy(&wrdsconnectionsettinglevel), core::mem::transmute_copy(&pwrdsconnectionsettings)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSettings: GetSettings::<Identity, OFFSET>,
            MergeSettings: MergeSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolSettings as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolSettings {}
windows_core::imp::define_interface!(IWRdsProtocolShadowCallback, IWRdsProtocolShadowCallback_Vtbl, 0xe0667ce0_0372_40d6_adb2_a0f3322674d6);
windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowCallback, windows_core::IUnknown);
impl IWRdsProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopShadow)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InvokeTargetShadow<P0, P10>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P10) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P10: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeTargetShadow)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolShadowCallback_Impl: windows_core::IUnknownImpl {
    fn StopShadow(&self) -> windows_core::Result<()>;
    fn InvokeTargetShadow(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWRdsProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: IWRdsProtocolShadowCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StopShadow<Identity: IWRdsProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolShadowCallback_Impl::StopShadow(this).into()
            }
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: IWRdsProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolShadowCallback_Impl::InvokeTargetShadow(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StopShadow: StopShadow::<Identity, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolShadowCallback {}
windows_core::imp::define_interface!(IWRdsProtocolShadowConnection, IWRdsProtocolShadowConnection_Vtbl, 0x9ae85ce6_cade_4548_8feb_99016597f60a);
windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowConnection, windows_core::IUnknown);
impl IWRdsProtocolShadowConnection {
    pub unsafe fn Start<P0, P4>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IWRdsProtocolShadowCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.param().abi()).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DoTarget<P8>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P8) -> windows_core::Result<()>
    where
        P8: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsProtocolShadowConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u8, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWRdsProtocolShadowConnection_Impl: windows_core::IUnknownImpl {
    fn Start(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: windows_core::Ref<IWRdsProtocolShadowCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn DoTarget(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWRdsProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolShadowConnection_Impl::Start(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&hotkeyvk), core::mem::transmute_copy(&hotkeymodifiers), core::mem::transmute_copy(&pshadowcallback)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolShadowConnection_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn DoTarget<Identity: IWRdsProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsProtocolShadowConnection_Impl::DoTarget(this, core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            DoTarget: DoTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsProtocolShadowConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsProtocolShadowConnection {}
windows_core::imp::define_interface!(IWRdsWddmIddProps, IWRdsWddmIddProps_Vtbl, 0x1382df4d_a289_43d1_a184_144726f9af90);
windows_core::imp::interface_hierarchy!(IWRdsWddmIddProps, windows_core::IUnknown);
impl IWRdsWddmIddProps {
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHardwareId)(windows_core::Interface::as_raw(self), core::mem::transmute(pdisplaydriverhardwareid.as_ptr()), pdisplaydriverhardwareid.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn OnDriverLoad(&self, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDriverLoad)(windows_core::Interface::as_raw(self), sessionid, driverhandle).ok() }
    }
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDriverUnload)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn EnableWddmIdd(&self, enabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableWddmIdd)(windows_core::Interface::as_raw(self), enabled.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsWddmIddProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHardwareId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnDriverLoad: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub OnDriverUnload: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnableWddmIdd: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWRdsWddmIddProps_Impl: windows_core::IUnknownImpl {
    fn GetHardwareId(&self, pdisplaydriverhardwareid: &windows_core::PCWSTR, count: u32) -> windows_core::Result<()>;
    fn OnDriverLoad(&self, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()>;
    fn EnableWddmIdd(&self, enabled: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IWRdsWddmIddProps_Vtbl {
    pub const fn new<Identity: IWRdsWddmIddProps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHardwareId<Identity: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaydriverhardwareid: windows_core::PCWSTR, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps_Impl::GetHardwareId(this, core::mem::transmute(&pdisplaydriverhardwareid), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn OnDriverLoad<Identity: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, driverhandle: super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps_Impl::OnDriverLoad(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&driverhandle)).into()
            }
        }
        unsafe extern "system" fn OnDriverUnload<Identity: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps_Impl::OnDriverUnload(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn EnableWddmIdd<Identity: IWRdsWddmIddProps_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps_Impl::EnableWddmIdd(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHardwareId: GetHardwareId::<Identity, OFFSET>,
            OnDriverLoad: OnDriverLoad::<Identity, OFFSET>,
            OnDriverUnload: OnDriverUnload::<Identity, OFFSET>,
            EnableWddmIdd: EnableWddmIdd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsWddmIddProps {}
windows_core::imp::define_interface!(IWRdsWddmIddProps1, IWRdsWddmIddProps1_Vtbl, 0x60f71b1a_3682_4bc7_997e_4e4f02a08148);
windows_core::imp::interface_hierarchy!(IWRdsWddmIddProps1, windows_core::IUnknown);
impl IWRdsWddmIddProps1 {
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetHardwareId)(windows_core::Interface::as_raw(self), core::mem::transmute(pdisplaydriverhardwareid.as_ptr()), pdisplaydriverhardwareid.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn OnDriverLoad<P1>(&self, sessionid: u32, deviceinstance: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDriverLoad)(windows_core::Interface::as_raw(self), sessionid, deviceinstance.param().abi()).ok() }
    }
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDriverUnload)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWRdsWddmIddProps1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHardwareId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnDriverLoad: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDriverUnload: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWRdsWddmIddProps1_Impl: windows_core::IUnknownImpl {
    fn GetHardwareId(&self, pdisplaydriverhardwareid: &windows_core::PCWSTR, count: u32) -> windows_core::Result<()>;
    fn OnDriverLoad(&self, sessionid: u32, deviceinstance: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()>;
}
impl IWRdsWddmIddProps1_Vtbl {
    pub const fn new<Identity: IWRdsWddmIddProps1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHardwareId<Identity: IWRdsWddmIddProps1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisplaydriverhardwareid: windows_core::PCWSTR, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps1_Impl::GetHardwareId(this, core::mem::transmute(&pdisplaydriverhardwareid), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn OnDriverLoad<Identity: IWRdsWddmIddProps1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, deviceinstance: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps1_Impl::OnDriverLoad(this, core::mem::transmute_copy(&sessionid), core::mem::transmute(&deviceinstance)).into()
            }
        }
        unsafe extern "system" fn OnDriverUnload<Identity: IWRdsWddmIddProps1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWRdsWddmIddProps1_Impl::OnDriverUnload(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetHardwareId: GetHardwareId::<Identity, OFFSET>,
            OnDriverLoad: OnDriverLoad::<Identity, OFFSET>,
            OnDriverUnload: OnDriverUnload::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWRdsWddmIddProps1 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWRdsWddmIddProps1 {}
windows_core::imp::define_interface!(IWTSBitmapRenderService, IWTSBitmapRenderService_Vtbl, 0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderService, windows_core::IUnknown);
impl IWTSBitmapRenderService {
    pub unsafe fn GetMappedRenderer<P1>(&self, mappingid: u64, pmappedrenderercallback: P1) -> windows_core::Result<IWTSBitmapRenderer>
    where
        P1: windows_core::Param<IWTSBitmapRendererCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMappedRenderer)(windows_core::Interface::as_raw(self), mappingid, pmappedrenderercallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMappedRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSBitmapRenderService_Impl: windows_core::IUnknownImpl {
    fn GetMappedRenderer(&self, mappingid: u64, pmappedrenderercallback: windows_core::Ref<IWTSBitmapRendererCallback>) -> windows_core::Result<IWTSBitmapRenderer>;
}
impl IWTSBitmapRenderService_Vtbl {
    pub const fn new<Identity: IWTSBitmapRenderService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMappedRenderer<Identity: IWTSBitmapRenderService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mappingid: u64, pmappedrenderercallback: *mut core::ffi::c_void, ppmappedrenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSBitmapRenderService_Impl::GetMappedRenderer(this, core::mem::transmute_copy(&mappingid), core::mem::transmute_copy(&pmappedrenderercallback)) {
                    Ok(ok__) => {
                        ppmappedrenderer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMappedRenderer: GetMappedRenderer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSBitmapRenderService {}
windows_core::imp::define_interface!(IWTSBitmapRenderer, IWTSBitmapRenderer_Vtbl, 0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderer, windows_core::IUnknown);
impl IWTSBitmapRenderer {
    pub unsafe fn Render(&self, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, pimagebuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), core::mem::transmute(imageformat), dwwidth, dwheight, cbstride, pimagebuffer.len().try_into().unwrap(), core::mem::transmute(pimagebuffer.as_ptr())).ok() }
    }
    pub unsafe fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRendererStatistics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveMapping(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveMapping)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, i32, u32, *const u8) -> windows_core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSBitmapRenderer_Impl: windows_core::IUnknownImpl {
    fn Render(&self, imageformat: &windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::Result<()>;
    fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS>;
    fn RemoveMapping(&self) -> windows_core::Result<()>;
}
impl IWTSBitmapRenderer_Vtbl {
    pub const fn new<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Render<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, cbimagebuffer: u32, pimagebuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRenderer_Impl::Render(this, core::mem::transmute(&imageformat), core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight), core::mem::transmute_copy(&cbstride), core::mem::transmute_copy(&cbimagebuffer), core::mem::transmute_copy(&pimagebuffer)).into()
            }
        }
        unsafe extern "system" fn GetRendererStatistics<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatistics: *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSBitmapRenderer_Impl::GetRendererStatistics(this) {
                    Ok(ok__) => {
                        pstatistics.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveMapping<Identity: IWTSBitmapRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRenderer_Impl::RemoveMapping(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Render: Render::<Identity, OFFSET>,
            GetRendererStatistics: GetRendererStatistics::<Identity, OFFSET>,
            RemoveMapping: RemoveMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRenderer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSBitmapRenderer {}
windows_core::imp::define_interface!(IWTSBitmapRendererCallback, IWTSBitmapRendererCallback_Vtbl, 0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
windows_core::imp::interface_hierarchy!(IWTSBitmapRendererCallback, windows_core::IUnknown);
impl IWTSBitmapRendererCallback {
    pub unsafe fn OnTargetSizeChanged(&self, rcnewsize: super::super::Foundation::RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTargetSizeChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(rcnewsize)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSBitmapRendererCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTargetSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::RECT) -> windows_core::HRESULT,
}
pub trait IWTSBitmapRendererCallback_Impl: windows_core::IUnknownImpl {
    fn OnTargetSizeChanged(&self, rcnewsize: &super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl IWTSBitmapRendererCallback_Vtbl {
    pub const fn new<Identity: IWTSBitmapRendererCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTargetSizeChanged<Identity: IWTSBitmapRendererCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcnewsize: super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSBitmapRendererCallback_Impl::OnTargetSizeChanged(this, core::mem::transmute(&rcnewsize)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTargetSizeChanged: OnTargetSizeChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSBitmapRendererCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSBitmapRendererCallback {}
windows_core::imp::define_interface!(IWTSListener, IWTSListener_Vtbl, 0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
windows_core::imp::interface_hierarchy!(IWTSListener, windows_core::IUnknown);
impl IWTSListener {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<super::Com::StructuredStorage::IPropertyBag> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetConfiguration: usize,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IWTSListener_Impl: windows_core::IUnknownImpl {
    fn GetConfiguration(&self) -> windows_core::Result<super::Com::StructuredStorage::IPropertyBag>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IWTSListener_Vtbl {
    pub const fn new<Identity: IWTSListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConfiguration<Identity: IWTSListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertybag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSListener_Impl::GetConfiguration(this) {
                    Ok(ok__) => {
                        pppropertybag.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetConfiguration: GetConfiguration::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListener as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IWTSListener {}
windows_core::imp::define_interface!(IWTSListenerCallback, IWTSListenerCallback_Vtbl, 0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSListenerCallback, windows_core::IUnknown);
impl IWTSListenerCallback {
    pub unsafe fn OnNewChannelConnection<P0>(&self, pchannel: P0, data: &windows_core::BSTR, pbaccept: *mut windows_core::BOOL, ppcallback: *mut Option<IWTSVirtualChannelCallback>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSVirtualChannel>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNewChannelConnection)(windows_core::Interface::as_raw(self), pchannel.param().abi(), core::mem::transmute_copy(data), pbaccept as _, core::mem::transmute(ppcallback)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewChannelConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSListenerCallback_Impl: windows_core::IUnknownImpl {
    fn OnNewChannelConnection(&self, pchannel: windows_core::Ref<IWTSVirtualChannel>, data: &windows_core::BSTR, pbaccept: *mut windows_core::BOOL, ppcallback: windows_core::OutRef<IWTSVirtualChannelCallback>) -> windows_core::Result<()>;
}
impl IWTSListenerCallback_Vtbl {
    pub const fn new<Identity: IWTSListenerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNewChannelConnection<Identity: IWTSListenerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, data: *mut core::ffi::c_void, pbaccept: *mut windows_core::BOOL, ppcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSListenerCallback_Impl::OnNewChannelConnection(this, core::mem::transmute_copy(&pchannel), core::mem::transmute(&data), core::mem::transmute_copy(&pbaccept), core::mem::transmute_copy(&ppcallback)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewChannelConnection: OnNewChannelConnection::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSListenerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSListenerCallback {}
windows_core::imp::define_interface!(IWTSPlugin, IWTSPlugin_Vtbl, 0xa1230201_1439_4e62_a414_190d0ac3d40e);
windows_core::imp::interface_hierarchy!(IWTSPlugin, windows_core::IUnknown);
impl IWTSPlugin {
    pub unsafe fn Initialize<P0>(&self, pchannelmgr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSVirtualChannelManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pchannelmgr.param().abi()).ok() }
    }
    pub unsafe fn Connected(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Connected)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnected)(windows_core::Interface::as_raw(self), dwdisconnectcode).ok() }
    }
    pub unsafe fn Terminated(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminated)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSPlugin_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pchannelmgr: windows_core::Ref<IWTSVirtualChannelManager>) -> windows_core::Result<()>;
    fn Connected(&self) -> windows_core::Result<()>;
    fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::Result<()>;
    fn Terminated(&self) -> windows_core::Result<()>;
}
impl IWTSPlugin_Vtbl {
    pub const fn new<Identity: IWTSPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pchannelmgr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Initialize(this, core::mem::transmute_copy(&pchannelmgr)).into()
            }
        }
        unsafe extern "system" fn Connected<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Connected(this).into()
            }
        }
        unsafe extern "system" fn Disconnected<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdisconnectcode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Disconnected(this, core::mem::transmute_copy(&dwdisconnectcode)).into()
            }
        }
        unsafe extern "system" fn Terminated<Identity: IWTSPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSPlugin_Impl::Terminated(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Connected: Connected::<Identity, OFFSET>,
            Disconnected: Disconnected::<Identity, OFFSET>,
            Terminated: Terminated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSPlugin {}
windows_core::imp::define_interface!(IWTSPluginServiceProvider, IWTSPluginServiceProvider_Vtbl, 0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
windows_core::imp::interface_hierarchy!(IWTSPluginServiceProvider, windows_core::IUnknown);
impl IWTSPluginServiceProvider {
    pub unsafe fn GetService(&self, serviceid: windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), core::mem::transmute(serviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSPluginServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSPluginServiceProvider_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, serviceid: &windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl IWTSPluginServiceProvider_Vtbl {
    pub const fn new<Identity: IWTSPluginServiceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IWTSPluginServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, serviceid: windows_core::GUID, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSPluginServiceProvider_Impl::GetService(this, core::mem::transmute(&serviceid)) {
                    Ok(ok__) => {
                        ppunkobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSPluginServiceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSPluginServiceProvider {}
windows_core::imp::define_interface!(IWTSProtocolConnection, IWTSProtocolConnection_Vtbl, 0x23083765_9095_4648_98bf_ef81c914032d);
windows_core::imp::interface_hierarchy!(IWTSProtocolConnection, windows_core::IUnknown);
impl IWTSProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWTSProtocolLogonErrorRedirector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogonErrorRedirector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendPolicyData)(windows_core::Interface::as_raw(self), ppolicydata).ok() }
    }
    pub unsafe fn AcceptConnection(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AcceptConnection)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetClientData)(windows_core::Interface::as_raw(self), pclientdata as _).ok() }
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserCredentials)(windows_core::Interface::as_raw(self), pusercreds as _).ok() }
    }
    pub unsafe fn GetLicenseConnection(&self) -> windows_core::Result<IWTSProtocolLicenseConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLicenseConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AuthenticateClientToSession)(windows_core::Interface::as_raw(self), sessionid as _).ok() }
    }
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionId)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProtocolHandles)(windows_core::Interface::as_raw(self), pkeyboardhandle as _, pmousehandle as _, pbeephandle as _, pvideohandle as _).ok() }
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConnectNotify)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn IsUserAllowedToLogon<P2, P3>(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: P2, pusername: P3) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsUserAllowedToLogon)(windows_core::Interface::as_raw(self), sessionid, usertoken, pdomainname.param().abi(), pusername.param().abi()).ok() }
    }
    pub unsafe fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: bool, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(windows_core::Interface::as_raw(self), husertoken, bsinglesessionperuserenabled.into(), psessionidarray as _, pdwsessionidentifiercount as _).ok() }
    }
    pub unsafe fn LogonNotify<P1, P2>(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LogonNotify)(windows_core::Interface::as_raw(self), hclienttoken, wszusername.param().abi(), wszdomainname.param().abi(), sessionid).ok() }
    }
    pub unsafe fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserData)(windows_core::Interface::as_raw(self), ppolicydata, pclientdata as _).ok() }
    }
    pub unsafe fn DisconnectNotify(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectNotify)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProtocolStatus)(windows_core::Interface::as_raw(self), pprotocolstatus as _).ok() }
    }
    pub unsafe fn GetLastInputTime(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastInputTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetErrorInfo)(windows_core::Interface::as_raw(self), ulerror).ok() }
    }
    pub unsafe fn SendBeep(&self, frequency: u32, duration: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendBeep)(windows_core::Interface::as_raw(self), frequency, duration).ok() }
    }
    pub unsafe fn CreateVirtualChannel<P0>(&self, szendpointname: P0, bstatic: bool, requestedpriority: u32) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), szendpointname.param().abi(), bstatic.into(), requestedpriority, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryProperty(&self, querytype: windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).QueryProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(querytype), ppropertyentriesin.len().try_into().unwrap(), ppropertyentriesout.len().try_into().unwrap(), core::mem::transmute(ppropertyentriesin.as_ptr()), core::mem::transmute(ppropertyentriesout.as_ptr())).ok() }
    }
    pub unsafe fn GetShadowConnection(&self) -> windows_core::Result<IWTSProtocolShadowConnection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShadowConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogonErrorRedirector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendPolicyData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_POLICY_DATA) -> windows_core::HRESULT,
    pub AcceptConnection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_CLIENT_DATA) -> windows_core::HRESULT,
    pub GetUserCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT,
    pub GetLicenseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthenticateClientToSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub GetProtocolHandles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE_PTR, *mut super::super::Foundation::HANDLE_PTR, *mut super::super::Foundation::HANDLE_PTR, *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub ConnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsUserAllowedToLogon: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::BOOL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub LogonNotify: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_POLICY_DATA, *mut WTS_USER_DATA) -> windows_core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SendBeep: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, windows_core::BOOL, u32, *mut usize) -> windows_core::HRESULT,
    pub QueryProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *const WTS_PROPERTY_VALUE, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSProtocolConnection_Impl: windows_core::IUnknownImpl {
    fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWTSProtocolLogonErrorRedirector>;
    fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::Result<()>;
    fn AcceptConnection(&self) -> windows_core::Result<()>;
    fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()>;
    fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()>;
    fn GetLicenseConnection(&self) -> windows_core::Result<IWTSProtocolLicenseConnection>;
    fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()>;
    fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()>;
    fn IsUserAllowedToLogon(&self, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: &windows_core::PCWSTR, pusername: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SessionArbitrationEnumeration(&self, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: windows_core::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>;
    fn LogonNotify(&self, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: &windows_core::PCWSTR, wszdomainname: &windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::Result<()>;
    fn DisconnectNotify(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()>;
    fn GetLastInputTime(&self) -> windows_core::Result<u64>;
    fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()>;
    fn SendBeep(&self, frequency: u32, duration: u32) -> windows_core::Result<()>;
    fn CreateVirtualChannel(&self, szendpointname: &windows_core::PCSTR, bstatic: windows_core::BOOL, requestedpriority: u32) -> windows_core::Result<usize>;
    fn QueryProperty(&self, querytype: &windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()>;
    fn GetShadowConnection(&self) -> windows_core::Result<IWTSProtocolShadowConnection>;
}
impl IWTSProtocolConnection_Vtbl {
    pub const fn new<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogonErrorRedirector<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplogonerrorredir: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolConnection_Impl::GetLogonErrorRedirector(this) {
                    Ok(ok__) => {
                        pplogonerrorredir.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendPolicyData<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::SendPolicyData(this, core::mem::transmute_copy(&ppolicydata)).into()
            }
        }
        unsafe extern "system" fn AcceptConnection<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::AcceptConnection(this).into()
            }
        }
        unsafe extern "system" fn GetClientData<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::GetClientData(this, core::mem::transmute_copy(&pclientdata)).into()
            }
        }
        unsafe extern "system" fn GetUserCredentials<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::GetUserCredentials(this, core::mem::transmute_copy(&pusercreds)).into()
            }
        }
        unsafe extern "system" fn GetLicenseConnection<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicenseconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolConnection_Impl::GetLicenseConnection(this) {
                    Ok(ok__) => {
                        pplicenseconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AuthenticateClientToSession<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::AuthenticateClientToSession(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionId<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::NotifySessionId(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn GetProtocolHandles<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::GetProtocolHandles(this, core::mem::transmute_copy(&pkeyboardhandle), core::mem::transmute_copy(&pmousehandle), core::mem::transmute_copy(&pbeephandle), core::mem::transmute_copy(&pvideohandle)).into()
            }
        }
        unsafe extern "system" fn ConnectNotify<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::ConnectNotify(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn IsUserAllowedToLogon<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: u32, usertoken: super::super::Foundation::HANDLE_PTR, pdomainname: windows_core::PCWSTR, pusername: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::IsUserAllowedToLogon(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&usertoken), core::mem::transmute(&pdomainname), core::mem::transmute(&pusername)).into()
            }
        }
        unsafe extern "system" fn SessionArbitrationEnumeration<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, husertoken: super::super::Foundation::HANDLE_PTR, bsinglesessionperuserenabled: windows_core::BOOL, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::SessionArbitrationEnumeration(this, core::mem::transmute_copy(&husertoken), core::mem::transmute_copy(&bsinglesessionperuserenabled), core::mem::transmute_copy(&psessionidarray), core::mem::transmute_copy(&pdwsessionidentifiercount)).into()
            }
        }
        unsafe extern "system" fn LogonNotify<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hclienttoken: super::super::Foundation::HANDLE_PTR, wszusername: windows_core::PCWSTR, wszdomainname: windows_core::PCWSTR, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::LogonNotify(this, core::mem::transmute_copy(&hclienttoken), core::mem::transmute(&wszusername), core::mem::transmute(&wszdomainname), core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn GetUserData<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::GetUserData(this, core::mem::transmute_copy(&ppolicydata), core::mem::transmute_copy(&pclientdata)).into()
            }
        }
        unsafe extern "system" fn DisconnectNotify<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::DisconnectNotify(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn GetProtocolStatus<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::GetProtocolStatus(this, core::mem::transmute_copy(&pprotocolstatus)).into()
            }
        }
        unsafe extern "system" fn GetLastInputTime<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastinputtime: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolConnection_Impl::GetLastInputTime(this) {
                    Ok(ok__) => {
                        plastinputtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorInfo<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulerror: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::SetErrorInfo(this, core::mem::transmute_copy(&ulerror)).into()
            }
        }
        unsafe extern "system" fn SendBeep<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: u32, duration: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::SendBeep(this, core::mem::transmute_copy(&frequency), core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn CreateVirtualChannel<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szendpointname: windows_core::PCSTR, bstatic: windows_core::BOOL, requestedpriority: u32, phchannel: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolConnection_Impl::CreateVirtualChannel(this, core::mem::transmute(&szendpointname), core::mem::transmute_copy(&bstatic), core::mem::transmute_copy(&requestedpriority)) {
                    Ok(ok__) => {
                        phchannel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryProperty<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, querytype: windows_core::GUID, ulnumentriesin: u32, ulnumentriesout: u32, ppropertyentriesin: *const WTS_PROPERTY_VALUE, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnection_Impl::QueryProperty(this, core::mem::transmute(&querytype), core::mem::transmute_copy(&ulnumentriesin), core::mem::transmute_copy(&ulnumentriesout), core::mem::transmute_copy(&ppropertyentriesin), core::mem::transmute_copy(&ppropertyentriesout)).into()
            }
        }
        unsafe extern "system" fn GetShadowConnection<Identity: IWTSProtocolConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppshadowconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolConnection_Impl::GetShadowConnection(this) {
                    Ok(ok__) => {
                        ppshadowconnection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogonErrorRedirector: GetLogonErrorRedirector::<Identity, OFFSET>,
            SendPolicyData: SendPolicyData::<Identity, OFFSET>,
            AcceptConnection: AcceptConnection::<Identity, OFFSET>,
            GetClientData: GetClientData::<Identity, OFFSET>,
            GetUserCredentials: GetUserCredentials::<Identity, OFFSET>,
            GetLicenseConnection: GetLicenseConnection::<Identity, OFFSET>,
            AuthenticateClientToSession: AuthenticateClientToSession::<Identity, OFFSET>,
            NotifySessionId: NotifySessionId::<Identity, OFFSET>,
            GetProtocolHandles: GetProtocolHandles::<Identity, OFFSET>,
            ConnectNotify: ConnectNotify::<Identity, OFFSET>,
            IsUserAllowedToLogon: IsUserAllowedToLogon::<Identity, OFFSET>,
            SessionArbitrationEnumeration: SessionArbitrationEnumeration::<Identity, OFFSET>,
            LogonNotify: LogonNotify::<Identity, OFFSET>,
            GetUserData: GetUserData::<Identity, OFFSET>,
            DisconnectNotify: DisconnectNotify::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            GetProtocolStatus: GetProtocolStatus::<Identity, OFFSET>,
            GetLastInputTime: GetLastInputTime::<Identity, OFFSET>,
            SetErrorInfo: SetErrorInfo::<Identity, OFFSET>,
            SendBeep: SendBeep::<Identity, OFFSET>,
            CreateVirtualChannel: CreateVirtualChannel::<Identity, OFFSET>,
            QueryProperty: QueryProperty::<Identity, OFFSET>,
            GetShadowConnection: GetShadowConnection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolConnection {}
windows_core::imp::define_interface!(IWTSProtocolConnectionCallback, IWTSProtocolConnectionCallback_Vtbl, 0x23083765_75eb_41fe_b4fb_e086242afa0f);
windows_core::imp::interface_hierarchy!(IWTSProtocolConnectionCallback, windows_core::IUnknown);
impl IWTSProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnReady)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BrokenConnection)(windows_core::Interface::as_raw(self), reason, source).ok() }
    }
    pub unsafe fn StopScreenUpdates(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopScreenUpdates)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RedrawWindow)(windows_core::Interface::as_raw(self), rect).ok() }
    }
    pub unsafe fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisplayIOCtl)(windows_core::Interface::as_raw(self), displayioctl).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolConnectionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SMALL_RECT) -> windows_core::HRESULT,
    pub DisplayIOCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_DISPLAY_IOCTL) -> windows_core::HRESULT,
}
pub trait IWTSProtocolConnectionCallback_Impl: windows_core::IUnknownImpl {
    fn OnReady(&self) -> windows_core::Result<()>;
    fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()>;
    fn StopScreenUpdates(&self) -> windows_core::Result<()>;
    fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()>;
    fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::Result<()>;
}
impl IWTSProtocolConnectionCallback_Vtbl {
    pub const fn new<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnReady<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnectionCallback_Impl::OnReady(this).into()
            }
        }
        unsafe extern "system" fn BrokenConnection<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: u32, source: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnectionCallback_Impl::BrokenConnection(this, core::mem::transmute_copy(&reason), core::mem::transmute_copy(&source)).into()
            }
        }
        unsafe extern "system" fn StopScreenUpdates<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnectionCallback_Impl::StopScreenUpdates(this).into()
            }
        }
        unsafe extern "system" fn RedrawWindow<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: *const WTS_SMALL_RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnectionCallback_Impl::RedrawWindow(this, core::mem::transmute_copy(&rect)).into()
            }
        }
        unsafe extern "system" fn DisplayIOCtl<Identity: IWTSProtocolConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolConnectionCallback_Impl::DisplayIOCtl(this, core::mem::transmute_copy(&displayioctl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnReady: OnReady::<Identity, OFFSET>,
            BrokenConnection: BrokenConnection::<Identity, OFFSET>,
            StopScreenUpdates: StopScreenUpdates::<Identity, OFFSET>,
            RedrawWindow: RedrawWindow::<Identity, OFFSET>,
            DisplayIOCtl: DisplayIOCtl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolConnectionCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolConnectionCallback {}
windows_core::imp::define_interface!(IWTSProtocolLicenseConnection, IWTSProtocolLicenseConnection_Vtbl, 0x23083765_178c_4079_8e4a_fea6496a4d70);
windows_core::imp::interface_hierarchy!(IWTSProtocolLicenseConnection, windows_core::IUnknown);
impl IWTSProtocolLicenseConnection {
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestLicensingCapabilities)(windows_core::Interface::as_raw(self), pplicensecapabilities as _, pcblicensecapabilities as _).ok() }
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RequestClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(reserve1.as_ptr()), reserve1.len().try_into().unwrap(), ppclientlicense as _, pcbclientlicense as _).ok() }
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProtocolComplete)(windows_core::Interface::as_raw(self), ulcomplete).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLicenseConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestLicensingCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_LICENSE_CAPABILITIES, *mut u32) -> windows_core::HRESULT,
    pub SendClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWTSProtocolLicenseConnection_Impl: windows_core::IUnknownImpl {
    fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()>;
    fn SendClientLicense(&self, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::Result<()>;
    fn RequestClientLicense(&self, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()>;
    fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()>;
}
impl IWTSProtocolLicenseConnection_Vtbl {
    pub const fn new<Identity: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestLicensingCapabilities<Identity: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolLicenseConnection_Impl::RequestLicensingCapabilities(this, core::mem::transmute_copy(&pplicensecapabilities), core::mem::transmute_copy(&pcblicensecapabilities)).into()
            }
        }
        unsafe extern "system" fn SendClientLicense<Identity: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclientlicense: *const u8, cbclientlicense: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolLicenseConnection_Impl::SendClientLicense(this, core::mem::transmute_copy(&pclientlicense), core::mem::transmute_copy(&cbclientlicense)).into()
            }
        }
        unsafe extern "system" fn RequestClientLicense<Identity: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserve1: *const u8, reserve2: u32, ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolLicenseConnection_Impl::RequestClientLicense(this, core::mem::transmute_copy(&reserve1), core::mem::transmute_copy(&reserve2), core::mem::transmute_copy(&ppclientlicense), core::mem::transmute_copy(&pcbclientlicense)).into()
            }
        }
        unsafe extern "system" fn ProtocolComplete<Identity: IWTSProtocolLicenseConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomplete: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolLicenseConnection_Impl::ProtocolComplete(this, core::mem::transmute_copy(&ulcomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RequestLicensingCapabilities: RequestLicensingCapabilities::<Identity, OFFSET>,
            SendClientLicense: SendClientLicense::<Identity, OFFSET>,
            RequestClientLicense: RequestClientLicense::<Identity, OFFSET>,
            ProtocolComplete: ProtocolComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolLicenseConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolLicenseConnection {}
windows_core::imp::define_interface!(IWTSProtocolListener, IWTSProtocolListener_Vtbl, 0x23083765_45f0_4394_8f69_32b2bc0ef4ca);
windows_core::imp::interface_hierarchy!(IWTSProtocolListener, windows_core::IUnknown);
impl IWTSProtocolListener {
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSProtocolListenerCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartListen)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn StopListen(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopListen)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSProtocolListener_Impl: windows_core::IUnknownImpl {
    fn StartListen(&self, pcallback: windows_core::Ref<IWTSProtocolListenerCallback>) -> windows_core::Result<()>;
    fn StopListen(&self) -> windows_core::Result<()>;
}
impl IWTSProtocolListener_Vtbl {
    pub const fn new<Identity: IWTSProtocolListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartListen<Identity: IWTSProtocolListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolListener_Impl::StartListen(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn StopListen<Identity: IWTSProtocolListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolListener_Impl::StopListen(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListen: StartListen::<Identity, OFFSET>,
            StopListen: StopListen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolListener as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolListener {}
windows_core::imp::define_interface!(IWTSProtocolListenerCallback, IWTSProtocolListenerCallback_Vtbl, 0x23083765_1a2d_4de2_97de_4a35f260f0b3);
windows_core::imp::interface_hierarchy!(IWTSProtocolListenerCallback, windows_core::IUnknown);
impl IWTSProtocolListenerCallback {
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0) -> windows_core::Result<IWTSProtocolConnectionCallback>
    where
        P0: windows_core::Param<IWTSProtocolConnection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), pconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSProtocolListenerCallback_Impl: windows_core::IUnknownImpl {
    fn OnConnected(&self, pconnection: windows_core::Ref<IWTSProtocolConnection>) -> windows_core::Result<IWTSProtocolConnectionCallback>;
}
impl IWTSProtocolListenerCallback_Vtbl {
    pub const fn new<Identity: IWTSProtocolListenerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnected<Identity: IWTSProtocolListenerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnection: *mut core::ffi::c_void, pcallback: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolListenerCallback_Impl::OnConnected(this, core::mem::transmute_copy(&pconnection)) {
                    Ok(ok__) => {
                        pcallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnConnected: OnConnected::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolListenerCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolListenerCallback {}
windows_core::imp::define_interface!(IWTSProtocolLogonErrorRedirector, IWTSProtocolLogonErrorRedirector_Vtbl, 0xfd9b61a7_2916_4627_8dee_4328711ad6cb);
windows_core::imp::interface_hierarchy!(IWTSProtocolLogonErrorRedirector, windows_core::IUnknown);
impl IWTSProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBeginPainting)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectStatus)(windows_core::Interface::as_raw(self), pszmessage.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectMessage)(windows_core::Interface::as_raw(self), pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RedirectLogonError<P2, P3>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P2, pszmessage: P3, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RedirectLogonError)(windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolLogonErrorRedirector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
}
pub trait IWTSProtocolLogonErrorRedirector_Impl: windows_core::IUnknownImpl {
    fn OnBeginPainting(&self) -> windows_core::Result<()>;
    fn RedirectStatus(&self, pszmessage: &windows_core::PCWSTR) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectMessage(&self, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
    fn RedirectLogonError(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: &windows_core::PCWSTR, pszmessage: &windows_core::PCWSTR, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>;
}
impl IWTSProtocolLogonErrorRedirector_Vtbl {
    pub const fn new<Identity: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnBeginPainting<Identity: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolLogonErrorRedirector_Impl::OnBeginPainting(this).into()
            }
        }
        unsafe extern "system" fn RedirectStatus<Identity: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmessage: windows_core::PCWSTR, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolLogonErrorRedirector_Impl::RedirectStatus(this, core::mem::transmute(&pszmessage)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectMessage<Identity: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolLogonErrorRedirector_Impl::RedirectMessage(this, core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RedirectLogonError<Identity: IWTSProtocolLogonErrorRedirector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntsstatus: i32, ntssubstatus: i32, pszcaption: windows_core::PCWSTR, pszmessage: windows_core::PCWSTR, utype: u32, presponse: *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolLogonErrorRedirector_Impl::RedirectLogonError(this, core::mem::transmute_copy(&ntsstatus), core::mem::transmute_copy(&ntssubstatus), core::mem::transmute(&pszcaption), core::mem::transmute(&pszmessage), core::mem::transmute_copy(&utype)) {
                    Ok(ok__) => {
                        presponse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginPainting: OnBeginPainting::<Identity, OFFSET>,
            RedirectStatus: RedirectStatus::<Identity, OFFSET>,
            RedirectMessage: RedirectMessage::<Identity, OFFSET>,
            RedirectLogonError: RedirectLogonError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolLogonErrorRedirector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolLogonErrorRedirector {}
windows_core::imp::define_interface!(IWTSProtocolManager, IWTSProtocolManager_Vtbl, 0xf9eaf6cc_ed79_4f01_821d_1f881b9f66cc);
windows_core::imp::interface_hierarchy!(IWTSProtocolManager, windows_core::IUnknown);
impl IWTSProtocolManager {
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> windows_core::Result<IWTSProtocolListener>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), wszlistenername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyServiceStateChange)(windows_core::Interface::as_raw(self), ptsservicestatechange).ok() }
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(windows_core::Interface::as_raw(self), sessionid).ok() }
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifySessionStateChange)(windows_core::Interface::as_raw(self), sessionid, eventid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SERVICE_STATE) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID, u32) -> windows_core::HRESULT,
}
pub trait IWTSProtocolManager_Impl: windows_core::IUnknownImpl {
    fn CreateListener(&self, wszlistenername: &windows_core::PCWSTR) -> windows_core::Result<IWTSProtocolListener>;
    fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>;
    fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()>;
}
impl IWTSProtocolManager_Vtbl {
    pub const fn new<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateListener<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlistenername: windows_core::PCWSTR, pprotocollistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSProtocolManager_Impl::CreateListener(this, core::mem::transmute(&wszlistenername)) {
                    Ok(ok__) => {
                        pprotocollistener.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NotifyServiceStateChange<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolManager_Impl::NotifyServiceStateChange(this, core::mem::transmute_copy(&ptsservicestatechange)).into()
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStart<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolManager_Impl::NotifySessionOfServiceStart(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionOfServiceStop<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolManager_Impl::NotifySessionOfServiceStop(this, core::mem::transmute_copy(&sessionid)).into()
            }
        }
        unsafe extern "system" fn NotifySessionStateChange<Identity: IWTSProtocolManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolManager_Impl::NotifySessionStateChange(this, core::mem::transmute_copy(&sessionid), core::mem::transmute_copy(&eventid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateListener: CreateListener::<Identity, OFFSET>,
            NotifyServiceStateChange: NotifyServiceStateChange::<Identity, OFFSET>,
            NotifySessionOfServiceStart: NotifySessionOfServiceStart::<Identity, OFFSET>,
            NotifySessionOfServiceStop: NotifySessionOfServiceStop::<Identity, OFFSET>,
            NotifySessionStateChange: NotifySessionStateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolManager {}
windows_core::imp::define_interface!(IWTSProtocolShadowCallback, IWTSProtocolShadowCallback_Vtbl, 0x503a2504_aae5_4ab1_93e0_6d1c4bc6f71a);
windows_core::imp::interface_hierarchy!(IWTSProtocolShadowCallback, windows_core::IUnknown);
impl IWTSProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopShadow)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn InvokeTargetShadow<P0, P10>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P10) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P10: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeTargetShadow)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWTSProtocolShadowCallback_Impl: windows_core::IUnknownImpl {
    fn StopShadow(&self) -> windows_core::Result<()>;
    fn InvokeTargetShadow(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWTSProtocolShadowCallback_Vtbl {
    pub const fn new<Identity: IWTSProtocolShadowCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StopShadow<Identity: IWTSProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolShadowCallback_Impl::StopShadow(this).into()
            }
        }
        unsafe extern "system" fn InvokeTargetShadow<Identity: IWTSProtocolShadowCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolShadowCallback_Impl::InvokeTargetShadow(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StopShadow: StopShadow::<Identity, OFFSET>,
            InvokeTargetShadow: InvokeTargetShadow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolShadowCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolShadowCallback {}
windows_core::imp::define_interface!(IWTSProtocolShadowConnection, IWTSProtocolShadowConnection_Vtbl, 0xee3b0c14_37fb_456b_bab3_6d6cd51e13bf);
windows_core::imp::interface_hierarchy!(IWTSProtocolShadowConnection, windows_core::IUnknown);
impl IWTSProtocolShadowConnection {
    pub unsafe fn Start<P0, P4>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<IWTSProtocolShadowCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.param().abi()).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DoTarget<P8>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P8) -> windows_core::Result<()>
    where
        P8: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DoTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSProtocolShadowConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u8, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWTSProtocolShadowConnection_Impl: windows_core::IUnknownImpl {
    fn Start(&self, ptargetservername: &windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: windows_core::Ref<IWTSProtocolShadowCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn DoTarget(&self, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWTSProtocolShadowConnection_Vtbl {
    pub const fn new<Identity: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetservername: windows_core::PCWSTR, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolShadowConnection_Impl::Start(this, core::mem::transmute(&ptargetservername), core::mem::transmute_copy(&targetsessionid), core::mem::transmute_copy(&hotkeyvk), core::mem::transmute_copy(&hotkeymodifiers), core::mem::transmute_copy(&pshadowcallback)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolShadowConnection_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn DoTarget<Identity: IWTSProtocolShadowConnection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam1: *const u8, param1size: u32, pparam2: *const u8, param2size: u32, pparam3: *const u8, param3size: u32, pparam4: *const u8, param4size: u32, pclientname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSProtocolShadowConnection_Impl::DoTarget(this, core::mem::transmute_copy(&pparam1), core::mem::transmute_copy(&param1size), core::mem::transmute_copy(&pparam2), core::mem::transmute_copy(&param2size), core::mem::transmute_copy(&pparam3), core::mem::transmute_copy(&param3size), core::mem::transmute_copy(&pparam4), core::mem::transmute_copy(&param4size), core::mem::transmute(&pclientname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            DoTarget: DoTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSProtocolShadowConnection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSProtocolShadowConnection {}
windows_core::imp::define_interface!(IWTSSBPlugin, IWTSSBPlugin_Vtbl, 0xdc44be78_b18d_4399_b210_641bf67a002c);
windows_core::imp::interface_hierarchy!(IWTSSBPlugin, windows_core::IUnknown);
impl IWTSSBPlugin {
    pub unsafe fn Initialize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WTSSBX_MachineChangeNotification)(windows_core::Interface::as_raw(self), notificationtype, machineid, pmachineinfo).ok() }
    }
    pub unsafe fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, sessioninfo: &[WTSSBX_SESSION_INFO]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WTSSBX_SessionChangeNotification)(windows_core::Interface::as_raw(self), notificationtype, machineid, sessioninfo.len().try_into().unwrap(), core::mem::transmute(sessioninfo.as_ptr())).ok() }
    }
    pub unsafe fn WTSSBX_GetMostSuitableServer<P0, P1, P2, P3>(&self, username: P0, domainname: P1, applicationtype: P2, farmname: P3, pmachineid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WTSSBX_GetMostSuitableServer)(windows_core::Interface::as_raw(self), username.param().abi(), domainname.param().abi(), applicationtype.param().abi(), farmname.param().abi(), pmachineid as _).ok() }
    }
    pub unsafe fn Terminated(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminated)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn WTSSBX_GetUserExternalSession<P0, P1, P2>(&self, username: P0, domainname: P1, applicationtype: P2, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WTSSBX_GetUserExternalSession)(windows_core::Interface::as_raw(self), username.param().abi(), domainname.param().abi(), applicationtype.param().abi(), redirectorinternalip, psessionid as _, pmachineconnectinfo as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSSBPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub WTSSBX_MachineChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, WTSSBX_NOTIFICATION_TYPE, i32, *const WTSSBX_MACHINE_INFO) -> windows_core::HRESULT,
    pub WTSSBX_SessionChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, WTSSBX_NOTIFICATION_TYPE, i32, u32, *const WTSSBX_SESSION_INFO) -> windows_core::HRESULT,
    pub WTSSBX_GetMostSuitableServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WTSSBX_GetUserExternalSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTSSBX_IP_ADDRESS, *mut u32, *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::HRESULT,
}
pub trait IWTSSBPlugin_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<u32>;
    fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::Result<()>;
    fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> windows_core::Result<()>;
    fn WTSSBX_GetMostSuitableServer(&self, username: &windows_core::PCWSTR, domainname: &windows_core::PCWSTR, applicationtype: &windows_core::PCWSTR, farmname: &windows_core::PCWSTR, pmachineid: *mut i32) -> windows_core::Result<()>;
    fn Terminated(&self) -> windows_core::Result<()>;
    fn WTSSBX_GetUserExternalSession(&self, username: &windows_core::PCWSTR, domainname: &windows_core::PCWSTR, applicationtype: &windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::Result<()>;
}
impl IWTSSBPlugin_Vtbl {
    pub const fn new<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plugincapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSSBPlugin_Impl::Initialize(this) {
                    Ok(ok__) => {
                        plugincapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WTSSBX_MachineChangeNotification<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSSBPlugin_Impl::WTSSBX_MachineChangeNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&machineid), core::mem::transmute_copy(&pmachineinfo)).into()
            }
        }
        unsafe extern "system" fn WTSSBX_SessionChangeNotification<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, numofsessions: u32, sessioninfo: *const WTSSBX_SESSION_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSSBPlugin_Impl::WTSSBX_SessionChangeNotification(this, core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&machineid), core::mem::transmute_copy(&numofsessions), core::mem::transmute_copy(&sessioninfo)).into()
            }
        }
        unsafe extern "system" fn WTSSBX_GetMostSuitableServer<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: windows_core::PCWSTR, domainname: windows_core::PCWSTR, applicationtype: windows_core::PCWSTR, farmname: windows_core::PCWSTR, pmachineid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSSBPlugin_Impl::WTSSBX_GetMostSuitableServer(this, core::mem::transmute(&username), core::mem::transmute(&domainname), core::mem::transmute(&applicationtype), core::mem::transmute(&farmname), core::mem::transmute_copy(&pmachineid)).into()
            }
        }
        unsafe extern "system" fn Terminated<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSSBPlugin_Impl::Terminated(this).into()
            }
        }
        unsafe extern "system" fn WTSSBX_GetUserExternalSession<Identity: IWTSSBPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: windows_core::PCWSTR, domainname: windows_core::PCWSTR, applicationtype: windows_core::PCWSTR, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSSBPlugin_Impl::WTSSBX_GetUserExternalSession(this, core::mem::transmute(&username), core::mem::transmute(&domainname), core::mem::transmute(&applicationtype), core::mem::transmute_copy(&redirectorinternalip), core::mem::transmute_copy(&psessionid), core::mem::transmute_copy(&pmachineconnectinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            WTSSBX_MachineChangeNotification: WTSSBX_MachineChangeNotification::<Identity, OFFSET>,
            WTSSBX_SessionChangeNotification: WTSSBX_SessionChangeNotification::<Identity, OFFSET>,
            WTSSBX_GetMostSuitableServer: WTSSBX_GetMostSuitableServer::<Identity, OFFSET>,
            Terminated: Terminated::<Identity, OFFSET>,
            WTSSBX_GetUserExternalSession: WTSSBX_GetUserExternalSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSSBPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSSBPlugin {}
windows_core::imp::define_interface!(IWTSVirtualChannel, IWTSVirtualChannel_Vtbl, 0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannel, windows_core::IUnknown);
impl IWTSVirtualChannel {
    pub unsafe fn Write<P2>(&self, pbuffer: &[u8], preserved: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), preserved.param().abi()).ok() }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannel_Impl: windows_core::IUnknownImpl {
    fn Write(&self, cbsize: u32, pbuffer: *const u8, preserved: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IWTSVirtualChannel_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8, preserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannel_Impl::Write(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&preserved)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWTSVirtualChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannel_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Write: Write::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannel {}
windows_core::imp::define_interface!(IWTSVirtualChannelCallback, IWTSVirtualChannelCallback_Vtbl, 0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelCallback, windows_core::IUnknown);
impl IWTSVirtualChannelCallback {
    pub unsafe fn OnDataReceived(&self, pbuffer: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDataReceived)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr())).ok() }
    }
    pub unsafe fn OnClose(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannelCallback_Impl: windows_core::IUnknownImpl {
    fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()>;
    fn OnClose(&self) -> windows_core::Result<()>;
}
impl IWTSVirtualChannelCallback_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDataReceived<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32, pbuffer: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannelCallback_Impl::OnDataReceived(this, core::mem::transmute_copy(&cbsize), core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        unsafe extern "system" fn OnClose<Identity: IWTSVirtualChannelCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWTSVirtualChannelCallback_Impl::OnClose(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataReceived: OnDataReceived::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannelCallback {}
windows_core::imp::define_interface!(IWTSVirtualChannelManager, IWTSVirtualChannelManager_Vtbl, 0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelManager, windows_core::IUnknown);
impl IWTSVirtualChannelManager {
    pub unsafe fn CreateListener<P0, P2>(&self, pszchannelname: P0, uflags: u32, plistenercallback: P2) -> windows_core::Result<IWTSListener>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P2: windows_core::Param<IWTSListenerCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), pszchannelname.param().abi(), uflags, plistenercallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWTSVirtualChannelManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWTSVirtualChannelManager_Impl: windows_core::IUnknownImpl {
    fn CreateListener(&self, pszchannelname: &windows_core::PCSTR, uflags: u32, plistenercallback: windows_core::Ref<IWTSListenerCallback>) -> windows_core::Result<IWTSListener>;
}
impl IWTSVirtualChannelManager_Vtbl {
    pub const fn new<Identity: IWTSVirtualChannelManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateListener<Identity: IWTSVirtualChannelManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszchannelname: windows_core::PCSTR, uflags: u32, plistenercallback: *mut core::ffi::c_void, pplistener: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWTSVirtualChannelManager_Impl::CreateListener(this, core::mem::transmute(&pszchannelname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&plistenercallback)) {
                    Ok(ok__) => {
                        pplistener.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateListener: CreateListener::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWTSVirtualChannelManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWTSVirtualChannelManager {}
windows_core::imp::define_interface!(IWorkspace, IWorkspace_Vtbl, 0xb922bbb8_4c55_4fea_8496_beb0b44285e5);
windows_core::imp::interface_hierarchy!(IWorkspace, windows_core::IUnknown);
impl IWorkspace {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWorkspaceNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication(&self, bstrworkspaceid: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartRemoteApplication)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), psaparams).ok() }
    }
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWorkspaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWorkspaceNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub StartRemoteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StartRemoteApplication: usize,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace_Impl: windows_core::IUnknownImpl {
    fn GetWorkspaceNames(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn StartRemoteApplication(&self, bstrworkspaceid: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
    fn GetProcessId(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace_Vtbl {
    pub const fn new<Identity: IWorkspace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWorkspaceNames<Identity: IWorkspace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psawkspnames: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspace_Impl::GetWorkspaceNames(this) {
                    Ok(ok__) => {
                        psawkspnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartRemoteApplication<Identity: IWorkspace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, psaparams: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspace_Impl::StartRemoteApplication(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute_copy(&psaparams)).into()
            }
        }
        unsafe extern "system" fn GetProcessId<Identity: IWorkspace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprocessid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspace_Impl::GetProcessId(this) {
                    Ok(ok__) => {
                        pulprocessid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWorkspaceNames: GetWorkspaceNames::<Identity, OFFSET>,
            StartRemoteApplication: StartRemoteApplication::<Identity, OFFSET>,
            GetProcessId: GetProcessId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace {}
windows_core::imp::define_interface!(IWorkspace2, IWorkspace2_Vtbl, 0x96d8d7cf_783e_4286_834c_ebc0e95f783c);
impl core::ops::Deref for IWorkspace2 {
    type Target = IWorkspace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspace2, windows_core::IUnknown, IWorkspace);
impl IWorkspace2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplicationEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrrequestingappid: &windows_core::BSTR, bstrrequestingappfamilyname: &windows_core::BSTR, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartRemoteApplicationEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrrequestingappid), core::mem::transmute_copy(bstrrequestingappfamilyname), blaunchintoimmersiveclient, core::mem::transmute_copy(bstrimmersiveclientactivationcontext), psaparams).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace2_Vtbl {
    pub base__: IWorkspace_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub StartRemoteApplicationEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StartRemoteApplicationEx: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace2_Impl: IWorkspace_Impl {
    fn StartRemoteApplicationEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrrequestingappid: &windows_core::BSTR, bstrrequestingappfamilyname: &windows_core::BSTR, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: &windows_core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace2_Vtbl {
    pub const fn new<Identity: IWorkspace2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartRemoteApplicationEx<Identity: IWorkspace2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrrequestingappid: *mut core::ffi::c_void, bstrrequestingappfamilyname: *mut core::ffi::c_void, blaunchintoimmersiveclient: super::super::Foundation::VARIANT_BOOL, bstrimmersiveclientactivationcontext: *mut core::ffi::c_void, psaparams: *const super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspace2_Impl::StartRemoteApplicationEx(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrrequestingappid), core::mem::transmute(&bstrrequestingappfamilyname), core::mem::transmute_copy(&blaunchintoimmersiveclient), core::mem::transmute(&bstrimmersiveclientactivationcontext), core::mem::transmute_copy(&psaparams)).into()
            }
        }
        Self { base__: IWorkspace_Vtbl::new::<Identity, OFFSET>(), StartRemoteApplicationEx: StartRemoteApplicationEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace2 as windows_core::Interface>::IID || iid == &<IWorkspace as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace2 {}
windows_core::imp::define_interface!(IWorkspace3, IWorkspace3_Vtbl, 0x1becbe4a_d654_423b_afeb_be8d532c13c6);
impl core::ops::Deref for IWorkspace3 {
    type Target = IWorkspace2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspace3, windows_core::IUnknown, IWorkspace, IWorkspace2);
impl IWorkspace3 {
    pub unsafe fn GetClaimsToken2(&self, bstrclaimshint: &windows_core::BSTR, bstruserhint: &windows_core::BSTR, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClaimsToken2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclaimshint), core::mem::transmute_copy(bstruserhint), claimcookie, hwndcreduiparent, core::mem::transmute(rectcreduiparent), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClaimsToken(&self, bstraccesstoken: &windows_core::BSTR, ullaccesstokenexpiration: u64, bstrrefreshtoken: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClaimsToken)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstraccesstoken), ullaccesstokenexpiration, core::mem::transmute_copy(bstrrefreshtoken)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspace3_Vtbl {
    pub base__: IWorkspace2_Vtbl,
    pub GetClaimsToken2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, super::super::Foundation::RECT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClaimsToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWorkspace3_Impl: IWorkspace2_Impl {
    fn GetClaimsToken2(&self, bstrclaimshint: &windows_core::BSTR, bstruserhint: &windows_core::BSTR, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: &super::super::Foundation::RECT) -> windows_core::Result<windows_core::BSTR>;
    fn SetClaimsToken(&self, bstraccesstoken: &windows_core::BSTR, ullaccesstokenexpiration: u64, bstrrefreshtoken: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspace3_Vtbl {
    pub const fn new<Identity: IWorkspace3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClaimsToken2<Identity: IWorkspace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclaimshint: *mut core::ffi::c_void, bstruserhint: *mut core::ffi::c_void, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT, pbstraccesstoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspace3_Impl::GetClaimsToken2(this, core::mem::transmute(&bstrclaimshint), core::mem::transmute(&bstruserhint), core::mem::transmute_copy(&claimcookie), core::mem::transmute_copy(&hwndcreduiparent), core::mem::transmute(&rectcreduiparent)) {
                    Ok(ok__) => {
                        pbstraccesstoken.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClaimsToken<Identity: IWorkspace3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstraccesstoken: *mut core::ffi::c_void, ullaccesstokenexpiration: u64, bstrrefreshtoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspace3_Impl::SetClaimsToken(this, core::mem::transmute(&bstraccesstoken), core::mem::transmute_copy(&ullaccesstokenexpiration), core::mem::transmute(&bstrrefreshtoken)).into()
            }
        }
        Self {
            base__: IWorkspace2_Vtbl::new::<Identity, OFFSET>(),
            GetClaimsToken2: GetClaimsToken2::<Identity, OFFSET>,
            SetClaimsToken: SetClaimsToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspace3 as windows_core::Interface>::IID || iid == &<IWorkspace as windows_core::Interface>::IID || iid == &<IWorkspace2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWorkspace3 {}
windows_core::imp::define_interface!(IWorkspaceClientExt, IWorkspaceClientExt_Vtbl, 0x12b952f4_41ca_4f21_a829_a6d07d9a16e5);
windows_core::imp::interface_hierarchy!(IWorkspaceClientExt, windows_core::IUnknown);
impl IWorkspaceClientExt {
    pub unsafe fn GetResourceId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetResourceDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IssueDisconnect(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).IssueDisconnect)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceClientExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IssueDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWorkspaceClientExt_Impl: windows_core::IUnknownImpl {
    fn GetResourceId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetResourceDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IssueDisconnect(&self) -> windows_core::Result<()>;
}
impl IWorkspaceClientExt_Vtbl {
    pub const fn new<Identity: IWorkspaceClientExt_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResourceId<Identity: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceClientExt_Impl::GetResourceId(this) {
                    Ok(ok__) => {
                        bstrworkspaceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourceDisplayName<Identity: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspacedisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceClientExt_Impl::GetResourceDisplayName(this) {
                    Ok(ok__) => {
                        bstrworkspacedisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IssueDisconnect<Identity: IWorkspaceClientExt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceClientExt_Impl::IssueDisconnect(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceId: GetResourceId::<Identity, OFFSET>,
            GetResourceDisplayName: GetResourceDisplayName::<Identity, OFFSET>,
            IssueDisconnect: IssueDisconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceClientExt as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWorkspaceClientExt {}
windows_core::imp::define_interface!(IWorkspaceRegistration, IWorkspaceRegistration_Vtbl, 0xb922bbb8_4c55_4fea_8496_beb0b44285e6);
windows_core::imp::interface_hierarchy!(IWorkspaceRegistration, windows_core::IUnknown);
impl IWorkspaceRegistration {
    pub unsafe fn AddResource<P0>(&self, punk: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IWorkspaceClientExt>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), punk.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveResource)(windows_core::Interface::as_raw(self), dwcookieconnection).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWorkspaceRegistration_Impl: windows_core::IUnknownImpl {
    fn AddResource(&self, punk: windows_core::Ref<IWorkspaceClientExt>) -> windows_core::Result<u32>;
    fn RemoveResource(&self, dwcookieconnection: u32) -> windows_core::Result<()>;
}
impl IWorkspaceRegistration_Vtbl {
    pub const fn new<Identity: IWorkspaceRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddResource<Identity: IWorkspaceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceRegistration_Impl::AddResource(this, core::mem::transmute_copy(&punk)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveResource<Identity: IWorkspaceRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookieconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceRegistration_Impl::RemoveResource(this, core::mem::transmute_copy(&dwcookieconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddResource: AddResource::<Identity, OFFSET>,
            RemoveResource: RemoveResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWorkspaceRegistration {}
windows_core::imp::define_interface!(IWorkspaceRegistration2, IWorkspaceRegistration2_Vtbl, 0xcf59f654_39bb_44d8_94d0_4635728957e9);
impl core::ops::Deref for IWorkspaceRegistration2 {
    type Target = IWorkspaceRegistration;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspaceRegistration2, windows_core::IUnknown, IWorkspaceRegistration);
impl IWorkspaceRegistration2 {
    pub unsafe fn AddResourceEx<P0>(&self, punk: P0, bstreventloguploadaddress: &windows_core::BSTR, pdwcookie: *mut u32, correlationid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWorkspaceClientExt>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddResourceEx)(windows_core::Interface::as_raw(self), punk.param().abi(), core::mem::transmute_copy(bstreventloguploadaddress), pdwcookie as _, core::mem::transmute(correlationid)).ok() }
    }
    pub unsafe fn RemoveResourceEx(&self, dwcookieconnection: u32, correlationid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveResourceEx)(windows_core::Interface::as_raw(self), dwcookieconnection, core::mem::transmute(correlationid)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceRegistration2_Vtbl {
    pub base__: IWorkspaceRegistration_Vtbl,
    pub AddResourceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, windows_core::GUID) -> windows_core::HRESULT,
    pub RemoveResourceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IWorkspaceRegistration2_Impl: IWorkspaceRegistration_Impl {
    fn AddResourceEx(&self, punk: windows_core::Ref<IWorkspaceClientExt>, bstreventloguploadaddress: &windows_core::BSTR, pdwcookie: *mut u32, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
    fn RemoveResourceEx(&self, dwcookieconnection: u32, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl IWorkspaceRegistration2_Vtbl {
    pub const fn new<Identity: IWorkspaceRegistration2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddResourceEx<Identity: IWorkspaceRegistration2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, bstreventloguploadaddress: *mut core::ffi::c_void, pdwcookie: *mut u32, correlationid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceRegistration2_Impl::AddResourceEx(this, core::mem::transmute_copy(&punk), core::mem::transmute(&bstreventloguploadaddress), core::mem::transmute_copy(&pdwcookie), core::mem::transmute(&correlationid)).into()
            }
        }
        unsafe extern "system" fn RemoveResourceEx<Identity: IWorkspaceRegistration2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookieconnection: u32, correlationid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceRegistration2_Impl::RemoveResourceEx(this, core::mem::transmute_copy(&dwcookieconnection), core::mem::transmute(&correlationid)).into()
            }
        }
        Self {
            base__: IWorkspaceRegistration_Vtbl::new::<Identity, OFFSET>(),
            AddResourceEx: AddResourceEx::<Identity, OFFSET>,
            RemoveResourceEx: RemoveResourceEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceRegistration2 as windows_core::Interface>::IID || iid == &<IWorkspaceRegistration as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWorkspaceRegistration2 {}
windows_core::imp::define_interface!(IWorkspaceReportMessage, IWorkspaceReportMessage_Vtbl, 0xa7c06739_500f_4e8c_99a8_2bd6955899eb);
windows_core::imp::interface_hierarchy!(IWorkspaceReportMessage, windows_core::IUnknown);
impl IWorkspaceReportMessage {
    pub unsafe fn RegisterErrorLogMessage(&self, bstrmessage: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterErrorLogMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmessage)).ok() }
    }
    pub unsafe fn IsErrorMessageRegistered(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsErrorMessageRegistered)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrwkspid), dwerrortype, core::mem::transmute_copy(bstrerrormessagetype), dwerrorcode, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterErrorEvent(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterErrorEvent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrwkspid), dwerrortype, core::mem::transmute_copy(bstrerrormessagetype), dwerrorcode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceReportMessage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterErrorLogMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsErrorMessageRegistered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RegisterErrorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWorkspaceReportMessage_Impl: windows_core::IUnknownImpl {
    fn RegisterErrorLogMessage(&self, bstrmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsErrorMessageRegistered(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RegisterErrorEvent(&self, bstrwkspid: &windows_core::BSTR, dwerrortype: u32, bstrerrormessagetype: &windows_core::BSTR, dwerrorcode: u32) -> windows_core::Result<()>;
}
impl IWorkspaceReportMessage_Vtbl {
    pub const fn new<Identity: IWorkspaceReportMessage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterErrorLogMessage<Identity: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceReportMessage_Impl::RegisterErrorLogMessage(this, core::mem::transmute(&bstrmessage)).into()
            }
        }
        unsafe extern "system" fn IsErrorMessageRegistered<Identity: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwkspid: *mut core::ffi::c_void, dwerrortype: u32, bstrerrormessagetype: *mut core::ffi::c_void, dwerrorcode: u32, pferrorexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceReportMessage_Impl::IsErrorMessageRegistered(this, core::mem::transmute(&bstrwkspid), core::mem::transmute_copy(&dwerrortype), core::mem::transmute(&bstrerrormessagetype), core::mem::transmute_copy(&dwerrorcode)) {
                    Ok(ok__) => {
                        pferrorexist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterErrorEvent<Identity: IWorkspaceReportMessage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrwkspid: *mut core::ffi::c_void, dwerrortype: u32, bstrerrormessagetype: *mut core::ffi::c_void, dwerrorcode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceReportMessage_Impl::RegisterErrorEvent(this, core::mem::transmute(&bstrwkspid), core::mem::transmute_copy(&dwerrortype), core::mem::transmute(&bstrerrormessagetype), core::mem::transmute_copy(&dwerrorcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterErrorLogMessage: RegisterErrorLogMessage::<Identity, OFFSET>,
            IsErrorMessageRegistered: IsErrorMessageRegistered::<Identity, OFFSET>,
            RegisterErrorEvent: RegisterErrorEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceReportMessage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWorkspaceReportMessage {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWorkspaceResTypeRegistry, IWorkspaceResTypeRegistry_Vtbl, 0x1d428c79_6e2e_4351_a361_c0401a03a0ba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWorkspaceResTypeRegistry {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWorkspaceResTypeRegistry, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceResTypeRegistry {
    pub unsafe fn AddResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddResourceType)(windows_core::Interface::as_raw(self), fmachinewide, core::mem::transmute_copy(bstrfileextension), core::mem::transmute_copy(bstrlauncher)).ok() }
    }
    pub unsafe fn DeleteResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteResourceType)(windows_core::Interface::as_raw(self), fmachinewide, core::mem::transmute_copy(bstrfileextension)).ok() }
    }
    pub unsafe fn GetRegisteredFileExtensions(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredFileExtensions)(windows_core::Interface::as_raw(self), fmachinewide, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResourceTypeInfo(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceTypeInfo)(windows_core::Interface::as_raw(self), fmachinewide, core::mem::transmute_copy(bstrfileextension), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ModifyResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ModifyResourceType)(windows_core::Interface::as_raw(self), fmachinewide, core::mem::transmute_copy(bstrfileextension), core::mem::transmute_copy(bstrlauncher)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceResTypeRegistry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AddResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    pub GetResourceTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModifyResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceResTypeRegistry_Impl: super::Com::IDispatch_Impl {
    fn AddResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRegisteredFileExtensions(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<*mut super::Com::SAFEARRAY>;
    fn GetResourceTypeInfo(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn ModifyResourceType(&self, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: &windows_core::BSTR, bstrlauncher: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWorkspaceResTypeRegistry_Vtbl {
    pub const fn new<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddResourceType<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: *mut core::ffi::c_void, bstrlauncher: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceResTypeRegistry_Impl::AddResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension), core::mem::transmute(&bstrlauncher)).into()
            }
        }
        unsafe extern "system" fn DeleteResourceType<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceResTypeRegistry_Impl::DeleteResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension)).into()
            }
        }
        unsafe extern "system" fn GetRegisteredFileExtensions<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, psafileextensions: *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceResTypeRegistry_Impl::GetRegisteredFileExtensions(this, core::mem::transmute_copy(&fmachinewide)) {
                    Ok(ok__) => {
                        psafileextensions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResourceTypeInfo<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: *mut core::ffi::c_void, pbstrlauncher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceResTypeRegistry_Impl::GetResourceTypeInfo(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension)) {
                    Ok(ok__) => {
                        pbstrlauncher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyResourceType<Identity: IWorkspaceResTypeRegistry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmachinewide: super::super::Foundation::VARIANT_BOOL, bstrfileextension: *mut core::ffi::c_void, bstrlauncher: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceResTypeRegistry_Impl::ModifyResourceType(this, core::mem::transmute_copy(&fmachinewide), core::mem::transmute(&bstrfileextension), core::mem::transmute(&bstrlauncher)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddResourceType: AddResourceType::<Identity, OFFSET>,
            DeleteResourceType: DeleteResourceType::<Identity, OFFSET>,
            GetRegisteredFileExtensions: GetRegisteredFileExtensions::<Identity, OFFSET>,
            GetResourceTypeInfo: GetResourceTypeInfo::<Identity, OFFSET>,
            ModifyResourceType: ModifyResourceType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceResTypeRegistry as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWorkspaceResTypeRegistry {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWorkspaceScriptable, IWorkspaceScriptable_Vtbl, 0xefea49a2_dda5_429d_8f42_b23b92c4c347);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWorkspaceScriptable {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWorkspaceScriptable, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable {
    pub unsafe fn DisconnectWorkspace(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectWorkspace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid)).ok() }
    }
    pub unsafe fn StartWorkspace(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartWorkspace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags).ok() }
    }
    pub unsafe fn IsWorkspaceCredentialSpecified(&self, bstrworkspaceid: &windows_core::BSTR, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorkspaceCredentialSpecified)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), bcountunauthenticatedcredentials, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsWorkspaceSSOEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ClearWorkspaceCredential(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearWorkspaceCredential)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid)).ok() }
    }
    pub unsafe fn OnAuthenticated(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnAuthenticated)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrusername)).ok() }
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName(&self, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectWorkspaceByFriendlyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspacefriendlyname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisconnectWorkspace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartWorkspace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub IsWorkspaceCredentialSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorkspaceSSOEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClearWorkspaceCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectWorkspaceByFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable_Impl: super::Com::IDispatch_Impl {
    fn DisconnectWorkspace(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartWorkspace(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()>;
    fn IsWorkspaceCredentialSpecified(&self, bstrworkspaceid: &windows_core::BSTR, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsWorkspaceSSOEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClearWorkspaceCredential(&self, bstrworkspaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OnAuthenticated(&self, bstrworkspaceid: &windows_core::BSTR, bstrusername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisconnectWorkspaceByFriendlyName(&self, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWorkspaceScriptable_Vtbl {
    pub const fn new<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisconnectWorkspace<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable_Impl::DisconnectWorkspace(this, core::mem::transmute(&bstrworkspaceid)).into()
            }
        }
        unsafe extern "system" fn StartWorkspace<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrworkspaceparams: *mut core::ffi::c_void, ltimeout: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable_Impl::StartWorkspace(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn IsWorkspaceCredentialSpecified<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bcountunauthenticatedcredentials: super::super::Foundation::VARIANT_BOOL, pbcredexist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceScriptable_Impl::IsWorkspaceCredentialSpecified(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute_copy(&bcountunauthenticatedcredentials)) {
                    Ok(ok__) => {
                        pbcredexist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsWorkspaceSSOEnabled<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbssoenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWorkspaceScriptable_Impl::IsWorkspaceSSOEnabled(this) {
                    Ok(ok__) => {
                        pbssoenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearWorkspaceCredential<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable_Impl::ClearWorkspaceCredential(this, core::mem::transmute(&bstrworkspaceid)).into()
            }
        }
        unsafe extern "system" fn OnAuthenticated<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable_Impl::OnAuthenticated(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrusername)).into()
            }
        }
        unsafe extern "system" fn DisconnectWorkspaceByFriendlyName<Identity: IWorkspaceScriptable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspacefriendlyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable_Impl::DisconnectWorkspaceByFriendlyName(this, core::mem::transmute(&bstrworkspacefriendlyname)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisconnectWorkspace: DisconnectWorkspace::<Identity, OFFSET>,
            StartWorkspace: StartWorkspace::<Identity, OFFSET>,
            IsWorkspaceCredentialSpecified: IsWorkspaceCredentialSpecified::<Identity, OFFSET>,
            IsWorkspaceSSOEnabled: IsWorkspaceSSOEnabled::<Identity, OFFSET>,
            ClearWorkspaceCredential: ClearWorkspaceCredential::<Identity, OFFSET>,
            OnAuthenticated: OnAuthenticated::<Identity, OFFSET>,
            DisconnectWorkspaceByFriendlyName: DisconnectWorkspaceByFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWorkspaceScriptable {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWorkspaceScriptable2, IWorkspaceScriptable2_Vtbl, 0xefea49a2_dda5_429d_8f42_b33ba2c4c348);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWorkspaceScriptable2 {
    type Target = IWorkspaceScriptable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWorkspaceScriptable2, windows_core::IUnknown, super::Com::IDispatch, IWorkspaceScriptable);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2 {
    pub unsafe fn StartWorkspaceEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartWorkspaceEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrworkspacefriendlyname), core::mem::transmute_copy(bstrredirectorname), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrappcontainer), core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags).ok() }
    }
    pub unsafe fn ResourceDismissed(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResourceDismissed)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrworkspacefriendlyname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable2_Vtbl {
    pub base__: IWorkspaceScriptable_Vtbl,
    pub StartWorkspaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub ResourceDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable2_Impl: IWorkspaceScriptable_Impl {
    fn StartWorkspaceEx(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32) -> windows_core::Result<()>;
    fn ResourceDismissed(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWorkspaceScriptable2_Vtbl {
    pub const fn new<Identity: IWorkspaceScriptable2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartWorkspaceEx<Identity: IWorkspaceScriptable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrworkspacefriendlyname: *mut core::ffi::c_void, bstrredirectorname: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrappcontainer: *mut core::ffi::c_void, bstrworkspaceparams: *mut core::ffi::c_void, ltimeout: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable2_Impl::StartWorkspaceEx(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname), core::mem::transmute(&bstrredirectorname), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrappcontainer), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn ResourceDismissed<Identity: IWorkspaceScriptable2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrworkspacefriendlyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable2_Impl::ResourceDismissed(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname)).into()
            }
        }
        Self {
            base__: IWorkspaceScriptable_Vtbl::new::<Identity, OFFSET>(),
            StartWorkspaceEx: StartWorkspaceEx::<Identity, OFFSET>,
            ResourceDismissed: ResourceDismissed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWorkspaceScriptable2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWorkspaceScriptable3, IWorkspaceScriptable3_Vtbl, 0x531e6512_2cbf_4bd2_80a5_d90a71636a9a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWorkspaceScriptable3 {
    type Target = IWorkspaceScriptable2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWorkspaceScriptable3, windows_core::IUnknown, super::Com::IDispatch, IWorkspaceScriptable, IWorkspaceScriptable2);
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3 {
    pub unsafe fn StartWorkspaceEx2(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32, bstreventloguploadaddress: &windows_core::BSTR, correlationid: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartWorkspaceEx2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrworkspaceid), core::mem::transmute_copy(bstrworkspacefriendlyname), core::mem::transmute_copy(bstrredirectorname), core::mem::transmute_copy(bstrusername), core::mem::transmute_copy(bstrpassword), core::mem::transmute_copy(bstrappcontainer), core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags, core::mem::transmute_copy(bstreventloguploadaddress), core::mem::transmute(correlationid)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWorkspaceScriptable3_Vtbl {
    pub base__: IWorkspaceScriptable2_Vtbl,
    pub StartWorkspaceEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWorkspaceScriptable3_Impl: IWorkspaceScriptable2_Impl {
    fn StartWorkspaceEx2(&self, bstrworkspaceid: &windows_core::BSTR, bstrworkspacefriendlyname: &windows_core::BSTR, bstrredirectorname: &windows_core::BSTR, bstrusername: &windows_core::BSTR, bstrpassword: &windows_core::BSTR, bstrappcontainer: &windows_core::BSTR, bstrworkspaceparams: &windows_core::BSTR, ltimeout: i32, lflags: i32, bstreventloguploadaddress: &windows_core::BSTR, correlationid: &windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWorkspaceScriptable3_Vtbl {
    pub const fn new<Identity: IWorkspaceScriptable3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartWorkspaceEx2<Identity: IWorkspaceScriptable3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrworkspaceid: *mut core::ffi::c_void, bstrworkspacefriendlyname: *mut core::ffi::c_void, bstrredirectorname: *mut core::ffi::c_void, bstrusername: *mut core::ffi::c_void, bstrpassword: *mut core::ffi::c_void, bstrappcontainer: *mut core::ffi::c_void, bstrworkspaceparams: *mut core::ffi::c_void, ltimeout: i32, lflags: i32, bstreventloguploadaddress: *mut core::ffi::c_void, correlationid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWorkspaceScriptable3_Impl::StartWorkspaceEx2(this, core::mem::transmute(&bstrworkspaceid), core::mem::transmute(&bstrworkspacefriendlyname), core::mem::transmute(&bstrredirectorname), core::mem::transmute(&bstrusername), core::mem::transmute(&bstrpassword), core::mem::transmute(&bstrappcontainer), core::mem::transmute(&bstrworkspaceparams), core::mem::transmute_copy(&ltimeout), core::mem::transmute_copy(&lflags), core::mem::transmute(&bstreventloguploadaddress), core::mem::transmute(&correlationid)).into()
            }
        }
        Self { base__: IWorkspaceScriptable2_Vtbl::new::<Identity, OFFSET>(), StartWorkspaceEx2: StartWorkspaceEx2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWorkspaceScriptable3 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable as windows_core::Interface>::IID || iid == &<IWorkspaceScriptable2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWorkspaceScriptable3 {}
windows_core::imp::define_interface!(ItsPubPlugin, ItsPubPlugin_Vtbl, 0x70c04b05_f347_412b_822f_36c99c54ca45);
windows_core::imp::interface_hierarchy!(ItsPubPlugin, windows_core::IUnknown);
impl ItsPubPlugin {
    pub unsafe fn GetResourceList<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResourceList)(windows_core::Interface::as_raw(self), userid.param().abi(), pceapplistsize as _, resourcelist as _).ok() }
    }
    pub unsafe fn GetResource<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), alias.param().abi(), flags, resource as _).ok() }
    }
    pub unsafe fn GetCacheLastUpdateTime(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCacheLastUpdateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn pluginName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).pluginName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn pluginVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).pluginVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ResolveResource<P3, P4>(&self, resourcetype: *mut u32, resourcelocation: &mut [u16; 256], endpointname: &mut [u16; 256], userid: P3, alias: P4) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveResource)(windows_core::Interface::as_raw(self), resourcetype as _, core::mem::transmute(resourcelocation.as_ptr()), core::mem::transmute(endpointname.as_ptr()), userid.param().abi(), alias.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut *mut pluginResource) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut pluginResource) -> windows_core::HRESULT,
    pub GetCacheLastUpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub pluginName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub pluginVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResolveResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR, windows_core::PWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ItsPubPlugin_Impl: windows_core::IUnknownImpl {
    fn GetResourceList(&self, userid: &windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::Result<()>;
    fn GetResource(&self, alias: &windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> windows_core::Result<()>;
    fn GetCacheLastUpdateTime(&self) -> windows_core::Result<u64>;
    fn pluginName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn pluginVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ResolveResource(&self, resourcetype: *mut u32, resourcelocation: windows_core::PWSTR, endpointname: windows_core::PWSTR, userid: &windows_core::PCWSTR, alias: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ItsPubPlugin_Vtbl {
    pub const fn new<Identity: ItsPubPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResourceList<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin_Impl::GetResourceList(this, core::mem::transmute(&userid), core::mem::transmute_copy(&pceapplistsize), core::mem::transmute_copy(&resourcelist)).into()
            }
        }
        unsafe extern "system" fn GetResource<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alias: windows_core::PCWSTR, flags: i32, resource: *mut pluginResource) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin_Impl::GetResource(this, core::mem::transmute(&alias), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&resource)).into()
            }
        }
        unsafe extern "system" fn GetCacheLastUpdateTime<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastupdatetime: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ItsPubPlugin_Impl::GetCacheLastUpdateTime(this) {
                    Ok(ok__) => {
                        lastupdatetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn pluginName<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ItsPubPlugin_Impl::pluginName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn pluginVersion<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ItsPubPlugin_Impl::pluginVersion(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResolveResource<Identity: ItsPubPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcetype: *mut u32, resourcelocation: windows_core::PWSTR, endpointname: windows_core::PWSTR, userid: windows_core::PCWSTR, alias: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin_Impl::ResolveResource(this, core::mem::transmute_copy(&resourcetype), core::mem::transmute_copy(&resourcelocation), core::mem::transmute_copy(&endpointname), core::mem::transmute(&userid), core::mem::transmute(&alias)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceList: GetResourceList::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
            GetCacheLastUpdateTime: GetCacheLastUpdateTime::<Identity, OFFSET>,
            pluginName: pluginName::<Identity, OFFSET>,
            pluginVersion: pluginVersion::<Identity, OFFSET>,
            ResolveResource: ResolveResource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ItsPubPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ItsPubPlugin {}
windows_core::imp::define_interface!(ItsPubPlugin2, ItsPubPlugin2_Vtbl, 0xfa4ce418_aad7_4ec6_bad1_0a321ba465d5);
impl core::ops::Deref for ItsPubPlugin2 {
    type Target = ItsPubPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ItsPubPlugin2, windows_core::IUnknown, ItsPubPlugin);
impl ItsPubPlugin2 {
    pub unsafe fn GetResource2List<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResource2List)(windows_core::Interface::as_raw(self), userid.param().abi(), pceapplistsize as _, resourcelist as _).ok() }
    }
    pub unsafe fn GetResource2<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetResource2)(windows_core::Interface::as_raw(self), alias.param().abi(), flags, resource as _).ok() }
    }
    pub unsafe fn ResolvePersonalDesktop<P0, P1>(&self, userid: P0, poolid: P1, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: &mut [u16; 256]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolvePersonalDesktop)(windows_core::Interface::as_raw(self), userid.param().abi(), poolid.param().abi(), epdresolutiontype, ppdassignmenttype as _, core::mem::transmute(endpointname.as_ptr())).ok() }
    }
    pub unsafe fn DeletePersonalDesktopAssignment<P0, P1, P2>(&self, userid: P0, poolid: P1, endpointname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeletePersonalDesktopAssignment)(windows_core::Interface::as_raw(self), userid.param().abi(), poolid.param().abi(), endpointname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ItsPubPlugin2_Vtbl {
    pub base__: ItsPubPlugin_Vtbl,
    pub GetResource2List: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut *mut pluginResource2) -> windows_core::HRESULT,
    pub GetResource2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut pluginResource2) -> windows_core::HRESULT,
    pub ResolvePersonalDesktop: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, TSPUB_PLUGIN_PD_RESOLUTION_TYPE, *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, windows_core::PWSTR) -> windows_core::HRESULT,
    pub DeletePersonalDesktopAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait ItsPubPlugin2_Impl: ItsPubPlugin_Impl {
    fn GetResource2List(&self, userid: &windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> windows_core::Result<()>;
    fn GetResource2(&self, alias: &windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> windows_core::Result<()>;
    fn ResolvePersonalDesktop(&self, userid: &windows_core::PCWSTR, poolid: &windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn DeletePersonalDesktopAssignment(&self, userid: &windows_core::PCWSTR, poolid: &windows_core::PCWSTR, endpointname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl ItsPubPlugin2_Vtbl {
    pub const fn new<Identity: ItsPubPlugin2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResource2List<Identity: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin2_Impl::GetResource2List(this, core::mem::transmute(&userid), core::mem::transmute_copy(&pceapplistsize), core::mem::transmute_copy(&resourcelist)).into()
            }
        }
        unsafe extern "system" fn GetResource2<Identity: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alias: windows_core::PCWSTR, flags: i32, resource: *mut pluginResource2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin2_Impl::GetResource2(this, core::mem::transmute(&alias), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&resource)).into()
            }
        }
        unsafe extern "system" fn ResolvePersonalDesktop<Identity: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, poolid: windows_core::PCWSTR, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin2_Impl::ResolvePersonalDesktop(this, core::mem::transmute(&userid), core::mem::transmute(&poolid), core::mem::transmute_copy(&epdresolutiontype), core::mem::transmute_copy(&ppdassignmenttype), core::mem::transmute_copy(&endpointname)).into()
            }
        }
        unsafe extern "system" fn DeletePersonalDesktopAssignment<Identity: ItsPubPlugin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userid: windows_core::PCWSTR, poolid: windows_core::PCWSTR, endpointname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ItsPubPlugin2_Impl::DeletePersonalDesktopAssignment(this, core::mem::transmute(&userid), core::mem::transmute(&poolid), core::mem::transmute(&endpointname)).into()
            }
        }
        Self {
            base__: ItsPubPlugin_Vtbl::new::<Identity, OFFSET>(),
            GetResource2List: GetResource2List::<Identity, OFFSET>,
            GetResource2: GetResource2::<Identity, OFFSET>,
            ResolvePersonalDesktop: ResolvePersonalDesktop::<Identity, OFFSET>,
            DeletePersonalDesktopAssignment: DeletePersonalDesktopAssignment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ItsPubPlugin2 as windows_core::Interface>::IID || iid == &<ItsPubPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ItsPubPlugin2 {}
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub const KeyCombinationDown: KeyCombinationType = KeyCombinationType(4i32);
pub const KeyCombinationHome: KeyCombinationType = KeyCombinationType(0i32);
pub const KeyCombinationLeft: KeyCombinationType = KeyCombinationType(1i32);
pub const KeyCombinationRight: KeyCombinationType = KeyCombinationType(3i32);
pub const KeyCombinationScroll: KeyCombinationType = KeyCombinationType(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyCombinationType(pub i32);
pub const KeyCombinationUp: KeyCombinationType = KeyCombinationType(2i32);
pub const LOAD_BALANCING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(4i32);
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
pub const NONFARM: TARGET_TYPE = TARGET_TYPE(2i32);
pub const NOTIFY_FOR_ALL_SESSIONS: u32 = 1u32;
pub const NOTIFY_FOR_THIS_SESSION: u32 = 0u32;
pub const ORCHESTRATION_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(16i32);
pub const OWNER_MS_TS_PLUGIN: TARGET_OWNER = TARGET_OWNER(1i32);
pub const OWNER_MS_VM_PLUGIN: TARGET_OWNER = TARGET_OWNER(2i32);
pub const OWNER_UNKNOWN: TARGET_OWNER = TARGET_OWNER(0i32);
pub type PCHANNEL_INIT_EVENT_FN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, event: u32, pdata: *mut core::ffi::c_void, datalength: u32)>;
pub type PCHANNEL_OPEN_EVENT_FN = Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32)>;
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(8i32);
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PLUGIN_TYPE(pub i32);
pub const POLICY_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(1i32);
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(2i32);
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(1i32);
pub const POSITION_INVALID: AE_POSITION_FLAGS = AE_POSITION_FLAGS(0i32);
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = AE_POSITION_FLAGS(4i32);
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x0cdfd28e_d0b9_4c1f_a5eb_6d1f6c6535b9);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: windows_core::GUID = windows_core::GUID::from_u128(0xed2c3fda_338d_4d3f_81a3_e767310d908e);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: windows_core::GUID = windows_core::GUID::from_u128(0x6212d757_0043_4862_99c3_9f3059ac2a3b);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: windows_core::GUID = windows_core::GUID::from_u128(0x197c427a_0135_4b6d_9c5e_e6579a0ab625);
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(32i32);
pub type PVIRTUALCHANNELCLOSE = Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
pub type PVIRTUALCHANNELENTRY = Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> windows_core::BOOL>;
pub type PVIRTUALCHANNELINIT = Option<unsafe extern "system" fn(ppinithandle: *mut *mut core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELOPEN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, popenhandle: *mut u32, pchannelname: windows_core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELWRITE = Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut core::ffi::c_void, datalength: u32, puserdata: *mut core::ffi::c_void) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PasswordEncodingType(pub i32);
pub const PasswordEncodingUTF16BE: PasswordEncodingType = PasswordEncodingType(2i32);
pub const PasswordEncodingUTF16LE: PasswordEncodingType = PasswordEncodingType(1i32);
pub const PasswordEncodingUTF8: PasswordEncodingType = PasswordEncodingType(0i32);
pub const PnpRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PolicyAttributeType(pub i32);
pub const PortRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(4i32);
pub const PrinterRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(3i32);
pub const RDCLIENT_BITMAP_RENDER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RDV_TASK_STATUS(pub i32);
pub const RDV_TASK_STATUS_APPLYING: RDV_TASK_STATUS = RDV_TASK_STATUS(3i32);
pub const RDV_TASK_STATUS_DOWNLOADING: RDV_TASK_STATUS = RDV_TASK_STATUS(2i32);
pub const RDV_TASK_STATUS_FAILED: RDV_TASK_STATUS = RDV_TASK_STATUS(7i32);
pub const RDV_TASK_STATUS_REBOOTED: RDV_TASK_STATUS = RDV_TASK_STATUS(5i32);
pub const RDV_TASK_STATUS_REBOOTING: RDV_TASK_STATUS = RDV_TASK_STATUS(4i32);
pub const RDV_TASK_STATUS_SEARCHING: RDV_TASK_STATUS = RDV_TASK_STATUS(1i32);
pub const RDV_TASK_STATUS_SUCCESS: RDV_TASK_STATUS = RDV_TASK_STATUS(6i32);
pub const RDV_TASK_STATUS_TIMEOUT: RDV_TASK_STATUS = RDV_TASK_STATUS(8i32);
pub const RDV_TASK_STATUS_UNKNOWN: RDV_TASK_STATUS = RDV_TASK_STATUS(0i32);
pub const RD_FARM_AUTO_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(5i32);
pub const RD_FARM_AUTO_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(3i32);
pub const RD_FARM_MANUAL_PERSONAL_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(4i32);
pub const RD_FARM_MANUAL_PERSONAL_VM: RD_FARM_TYPE = RD_FARM_TYPE(2i32);
pub const RD_FARM_RDSH: RD_FARM_TYPE = RD_FARM_TYPE(0i32);
pub const RD_FARM_TEMP_VM: RD_FARM_TYPE = RD_FARM_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RD_FARM_TYPE(pub i32);
pub const RD_FARM_TYPE_UNKNOWN: RD_FARM_TYPE = RD_FARM_TYPE(-1i32);
pub const REMOTECONTROL_KBDALT_HOTKEY: u32 = 4u32;
pub const REMOTECONTROL_KBDCTRL_HOTKEY: u32 = 2u32;
pub const REMOTECONTROL_KBDSHIFT_HOTKEY: u32 = 1u32;
pub const RENDER_HINT_CLEAR: u32 = 0u32;
pub const RENDER_HINT_MAPPEDWINDOW: u32 = 2u32;
pub const RENDER_HINT_VIDEO: u32 = 1u32;
pub const RESERVED_FOR_LEGACY: u32 = 4u32;
pub const RESOURCE_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(2i32);
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
    pub primary: windows_core::BOOL,
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
pub const RemoteActionAppSwitch: RemoteActionType = RemoteActionType(4i32);
pub const RemoteActionAppbar: RemoteActionType = RemoteActionType(1i32);
pub const RemoteActionCharms: RemoteActionType = RemoteActionType(0i32);
pub const RemoteActionSnap: RemoteActionType = RemoteActionType(2i32);
pub const RemoteActionStartScreen: RemoteActionType = RemoteActionType(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemoteActionType(pub i32);
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(0i32);
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SESSION_TIMEOUT_ACTION_TYPE(pub i32);
pub const SINGLE_SESSION: u32 = 1u32;
pub const STATE_ACTIVE: TSSESSION_STATE = TSSESSION_STATE(0i32);
pub const STATE_CONNECTED: TSSESSION_STATE = TSSESSION_STATE(1i32);
pub const STATE_CONNECTQUERY: TSSESSION_STATE = TSSESSION_STATE(2i32);
pub const STATE_DISCONNECTED: TSSESSION_STATE = TSSESSION_STATE(4i32);
pub const STATE_DOWN: TSSESSION_STATE = TSSESSION_STATE(8i32);
pub const STATE_IDLE: TSSESSION_STATE = TSSESSION_STATE(5i32);
pub const STATE_INIT: TSSESSION_STATE = TSSESSION_STATE(9i32);
pub const STATE_INVALID: TSSESSION_STATE = TSSESSION_STATE(-1i32);
pub const STATE_LISTEN: TSSESSION_STATE = TSSESSION_STATE(6i32);
pub const STATE_MAX: TSSESSION_STATE = TSSESSION_STATE(10i32);
pub const STATE_RESET: TSSESSION_STATE = TSSESSION_STATE(7i32);
pub const STATE_SHADOW: TSSESSION_STATE = TSSESSION_STATE(3i32);
pub const SnapshotEncodingDataUri: SnapshotEncodingType = SnapshotEncodingType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SnapshotEncodingType(pub i32);
pub const SnapshotFormatBmp: SnapshotFormatType = SnapshotFormatType(2i32);
pub const SnapshotFormatJpeg: SnapshotFormatType = SnapshotFormatType(1i32);
pub const SnapshotFormatPng: SnapshotFormatType = SnapshotFormatType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SnapshotFormatType(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TARGET_CHANGE_TYPE(pub i32);
pub const TARGET_CHANGE_UNSPEC: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1i32);
pub const TARGET_CHECKED_OUT: TARGET_STATE = TARGET_STATE(6i32);
pub const TARGET_DOWN: TARGET_STATE = TARGET_STATE(4i32);
pub const TARGET_EXTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(2i32);
pub const TARGET_FARM_MEMBERSHIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(1024i32);
pub const TARGET_HIBERNATED: TARGET_STATE = TARGET_STATE(5i32);
pub const TARGET_IDLE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(64i32);
pub const TARGET_INITIALIZING: TARGET_STATE = TARGET_STATE(2i32);
pub const TARGET_INTERNALIP_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(4i32);
pub const TARGET_INUSE: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(256i32);
pub const TARGET_INVALID: TARGET_STATE = TARGET_STATE(8i32);
pub const TARGET_JOINED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(8i32);
pub const TARGET_MAXSTATE: TARGET_STATE = TARGET_STATE(11i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TARGET_OWNER(pub i32);
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(3i32);
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(4i32);
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = TARGET_PATCH_STATE(2i32);
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TARGET_PATCH_STATE(pub i32);
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(512i32);
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = TARGET_PATCH_STATE(0i32);
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(128i32);
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(16i32);
pub const TARGET_RUNNING: TARGET_STATE = TARGET_STATE(3i32);
pub const TARGET_STARTING: TARGET_STATE = TARGET_STATE(9i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TARGET_STATE(pub i32);
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(32i32);
pub const TARGET_STOPPED: TARGET_STATE = TARGET_STATE(7i32);
pub const TARGET_STOPPING: TARGET_STATE = TARGET_STATE(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TARGET_TYPE(pub i32);
pub const TARGET_UNKNOWN: TARGET_STATE = TARGET_STATE(1i32);
pub const TASK_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(64i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(pub i32);
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TSSB_NOTIFICATION_TYPE(pub i32);
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(4i32);
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(0i32);
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(2i32);
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(1i32);
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = TSSD_AddrV46Type(4i32);
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = TSSD_AddrV46Type(6i32);
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = TSSD_AddrV46Type(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TSSD_AddrV46Type(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TSSESSION_STATE(pub i32);
pub const TSUserExInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0x0910dd01_df8c_11d1_ae27_00c04fa35813);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TS_SB_SORT_BY(pub i32);
pub const TS_SB_SORT_BY_NAME: TS_SB_SORT_BY = TS_SB_SORT_BY(1i32);
pub const TS_SB_SORT_BY_NONE: TS_SB_SORT_BY = TS_SB_SORT_BY(0i32);
pub const TS_SB_SORT_BY_PROP: TS_SB_SORT_BY = TS_SB_SORT_BY(2i32);
pub const TS_VC_LISTENER_STATIC_CHANNEL: u32 = 1u32;
pub const UNKNOWN: TARGET_TYPE = TARGET_TYPE(0i32);
pub const UNKNOWN_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(0i32);
pub const USERNAME_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_HARDWAREID_LENGTH: u32 = 20u32;
pub const VALIDATIONINFORMATION_LICENSE_LENGTH: u32 = 16384u32;
pub const VIRTUAL_CHANNEL_VERSION_WIN2000: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VM_HOST_NOTIFY_STATUS(pub i32);
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(2i32);
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(3i32);
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(1i32);
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VM_NOTIFY_STATUS(pub i32);
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(4i32);
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(2i32);
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(3i32);
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(1i32);
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut windows_core::PWSTR,
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
    pub EventLogActivityId: windows_core::GUID,
    pub ContextSize: u32,
    pub ContextData: *mut u8,
}
impl Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRDS_CONNECTION_SETTING_LEVEL(pub i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(1i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(0i32);
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRDS_LISTENER_SETTING_LEVEL(pub i32);
pub const WRDS_LISTENER_SETTING_LEVEL_1: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(1i32);
pub const WRDS_LISTENER_SETTING_LEVEL_INVALID: WRDS_LISTENER_SETTING_LEVEL = WRDS_LISTENER_SETTING_LEVEL(0i32);
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
pub const WRDS_SERVICE_ID_GRAPHICS_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd2993f4d_02cf_4280_8c48_1624b44f8706);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRDS_SETTING_LEVEL(pub i32);
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(1i32);
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRDS_SETTING_STATUS(pub i32);
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(0i32);
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(1i32);
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(-1i32);
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRDS_SETTING_TYPE(pub i32);
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(0i32);
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(1i32);
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(3i32);
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(2i32);
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WRdsGraphicsChannelType(pub i32);
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(1i32);
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(0i32);
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(0i32);
pub const WTSApplicationName: WTS_INFO_CLASS = WTS_INFO_CLASS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTSClientAddress: WTS_INFO_CLASS = WTS_INFO_CLASS(14i32);
pub const WTSClientBuildNumber: WTS_INFO_CLASS = WTS_INFO_CLASS(9i32);
pub const WTSClientDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(11i32);
pub const WTSClientDisplay: WTS_INFO_CLASS = WTS_INFO_CLASS(15i32);
pub const WTSClientHardwareId: WTS_INFO_CLASS = WTS_INFO_CLASS(13i32);
pub const WTSClientInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(23i32);
pub const WTSClientName: WTS_INFO_CLASS = WTS_INFO_CLASS(10i32);
pub const WTSClientProductId: WTS_INFO_CLASS = WTS_INFO_CLASS(12i32);
pub const WTSClientProtocolType: WTS_INFO_CLASS = WTS_INFO_CLASS(16i32);
pub const WTSConfigInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(26i32);
pub const WTSConnectQuery: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(2i32);
pub const WTSConnectState: WTS_INFO_CLASS = WTS_INFO_CLASS(8i32);
pub const WTSConnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(1i32);
pub const WTSDisconnected: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(4i32);
pub const WTSDomainName: WTS_INFO_CLASS = WTS_INFO_CLASS(7i32);
pub const WTSDown: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(5i32);
pub const WTSIdleTime: WTS_INFO_CLASS = WTS_INFO_CLASS(17i32);
pub const WTSIncomingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(19i32);
pub const WTSIncomingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(21i32);
pub const WTSInit: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(9i32);
pub const WTSInitialProgram: WTS_INFO_CLASS = WTS_INFO_CLASS(0i32);
pub const WTSIsRemoteSession: WTS_INFO_CLASS = WTS_INFO_CLASS(29i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTSListen: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(6i32);
pub const WTSLogonTime: WTS_INFO_CLASS = WTS_INFO_CLASS(18i32);
pub const WTSOEMId: WTS_INFO_CLASS = WTS_INFO_CLASS(3i32);
pub const WTSOutgoingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(20i32);
pub const WTSOutgoingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(22i32);
pub const WTSReset: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_ADDRESS_FAMILY(pub i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(1i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(2i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(3i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(4i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_MACHINE_DRAIN(pub i32);
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(1i32);
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(2i32);
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_MACHINE_SESSION_MODE(pub i32);
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(2i32);
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(1i32);
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_MACHINE_STATE(pub i32);
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(1i32);
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(2i32);
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(0i32);
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(4i32);
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(2i32);
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(1i32);
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(8i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_NOTIFICATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTSSBX_SESSION_STATE(pub i32);
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(1i32);
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(2i32);
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
pub const WTSSessionAddressV4: WTS_INFO_CLASS = WTS_INFO_CLASS(28i32);
pub const WTSSessionId: WTS_INFO_CLASS = WTS_INFO_CLASS(4i32);
pub const WTSSessionInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(24i32);
pub const WTSSessionInfoEx: WTS_INFO_CLASS = WTS_INFO_CLASS(25i32);
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(3i32);
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = WTS_TYPE_CLASS(0i32);
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(1i32);
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTSUserConfigBrokenTimeoutSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(10i32);
pub const WTSUserConfigInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(0i32);
pub const WTSUserConfigModemCallbackPhoneNumber: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(13i32);
pub const WTSUserConfigModemCallbackSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(12i32);
pub const WTSUserConfigReconnectSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(11i32);
pub const WTSUserConfigShadowingSettings: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(14i32);
pub const WTSUserConfigSourceSAM: WTS_CONFIG_SOURCE = WTS_CONFIG_SOURCE(0i32);
pub const WTSUserConfigTerminalServerHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(16i32);
pub const WTSUserConfigTerminalServerHomeDirDrive: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(17i32);
pub const WTSUserConfigTerminalServerProfilePath: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(15i32);
pub const WTSUserConfigTimeoutSettingsConnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(4i32);
pub const WTSUserConfigTimeoutSettingsDisconnections: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(5i32);
pub const WTSUserConfigTimeoutSettingsIdle: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(6i32);
pub const WTSUserConfigUser: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(19i32);
pub const WTSUserConfigWorkingDirectory: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(1i32);
pub const WTSUserConfigfAllowLogonTerminalServer: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(3i32);
pub const WTSUserConfigfDeviceClientDefaultPrinter: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(9i32);
pub const WTSUserConfigfDeviceClientDrives: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(7i32);
pub const WTSUserConfigfDeviceClientPrinters: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(8i32);
pub const WTSUserConfigfInheritInitialProgram: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(2i32);
pub const WTSUserConfigfTerminalServerRemoteHomeDir: WTS_CONFIG_CLASS = WTS_CONFIG_CLASS(18i32);
pub const WTSUserName: WTS_INFO_CLASS = WTS_INFO_CLASS(5i32);
pub const WTSValidationInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(27i32);
pub const WTSVirtualClientData: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(0i32);
pub const WTSVirtualFileHandle: WTS_VIRTUAL_CLASS = WTS_VIRTUAL_CLASS(1i32);
pub const WTSWinStationName: WTS_INFO_CLASS = WTS_INFO_CLASS(6i32);
pub const WTSWorkingDirectory: WTS_INFO_CLASS = WTS_INFO_CLASS(2i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_CERT_TYPE(pub i32);
pub const WTS_CERT_TYPE_INVALID: WTS_CERT_TYPE = WTS_CERT_TYPE(0i32);
pub const WTS_CERT_TYPE_PROPRIETORY: WTS_CERT_TYPE = WTS_CERT_TYPE(1i32);
pub const WTS_CERT_TYPE_X509: WTS_CERT_TYPE = WTS_CERT_TYPE(2i32);
pub const WTS_CHANNEL_OPTION_DYNAMIC: u32 = 1u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_NO_COMPRESS: u32 = 8u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_HIGH: u32 = 4u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_LOW: u32 = 0u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_MED: u32 = 2u32;
pub const WTS_CHANNEL_OPTION_DYNAMIC_PRI_REAL: u32 = 6u32;
pub const WTS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WTS_CLIENTNAME_LENGTH: u32 = 20u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fInheritAutoLogon: windows_core::BOOL,
    pub fPromptForPassword: bool,
    pub fUsingSavedCreds: bool,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: bool,
    pub fInheritInitialProgram: windows_core::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: bool,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_CONFIG_CLASS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_CONFIG_SOURCE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_CONNECTSTATE_CLASS(pub i32);
pub const WTS_CURRENT_SERVER: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
pub const WTS_CURRENT_SERVER_HANDLE: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
pub const WTS_CURRENT_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("");
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTS_DRAIN_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(1i32);
pub const WTS_DRAIN_NOT_IN_DRAIN: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(2i32);
pub const WTS_DRAIN_STATE_NONE: WTS_RCM_DRAIN_STATE = WTS_RCM_DRAIN_STATE(0i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_INFO_CLASS(pub i32);
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: windows_core::BOOL,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(pub i32);
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(3i32);
pub const WTS_LOGON_ERR_HANDLED_DONT_SHOW_START_OVER: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(4i32);
pub const WTS_LOGON_ERR_HANDLED_SHOW: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(2i32);
pub const WTS_LOGON_ERR_INVALID: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(0i32);
pub const WTS_LOGON_ERR_NOT_HANDLED: WTS_LOGON_ERROR_REDIRECTOR_RESPONSE = WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(1i32);
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PWSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFO_EXA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PSTR,
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
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROCESS_INFO_EXW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PWSTR,
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
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_core::PCWSTR = windows_core::w!("DefaultConfig");
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
    pub guidVal: windows_core::GUID,
}
impl Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pbVal: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pstrVal: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const WTS_QUERY_ALLOWED_INITIAL_APP: windows_core::GUID = windows_core::GUID::from_u128(0xc77d1b30_5be1_4c6b_a0e1_bd6d2e5c9fcc);
pub const WTS_QUERY_AUDIOENUM_DLL: windows_core::GUID = windows_core::GUID::from_u128(0x9bf4fa97_c883_4c2a_80ab_5a39c9af00db);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x8b8e0fe7_0804_4a0e_b279_8660b1df0049);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x41869ad0_6332_4dc8_95d5_db749e2f1d94);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_RCM_DRAIN_STATE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_RCM_SERVICE_STATE(pub i32);
pub const WTS_SECURITY_ALL_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(983999u32);
pub const WTS_SECURITY_CONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(256u32);
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(72u32);
pub const WTS_SECURITY_CURRENT_USER_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(590u32);
pub const WTS_SECURITY_DISCONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_SECURITY_FLAGS(pub u32);
impl WTS_SECURITY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WTS_SECURITY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WTS_SECURITY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WTS_SECURITY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WTS_SECURITY_GUEST_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(32u32);
pub const WTS_SECURITY_LOGOFF: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(64u32);
pub const WTS_SECURITY_LOGON: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(32u32);
pub const WTS_SECURITY_MESSAGE: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(128u32);
pub const WTS_SECURITY_QUERY_INFORMATION: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(1u32);
pub const WTS_SECURITY_REMOTE_CONTROL: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(16u32);
pub const WTS_SECURITY_RESET: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(4u32);
pub const WTS_SECURITY_SET_INFORMATION: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(2u32);
pub const WTS_SECURITY_USER_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(329u32);
pub const WTS_SECURITY_VIRTUAL_CHANNELS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(8u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: windows_core::PWSTR,
}
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(0i32);
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(2i32);
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: windows_core::GUID,
    pub SessionId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFO_1A {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_core::PSTR,
    pub pHostName: windows_core::PSTR,
    pub pUserName: windows_core::PSTR,
    pub pDomainName: windows_core::PSTR,
    pub pFarmName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WTS_SESSION_INFO_1W {
    pub ExecEnvId: u32,
    pub State: WTS_CONNECTSTATE_CLASS,
    pub SessionId: u32,
    pub pSessionName: windows_core::PWSTR,
    pub pHostName: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pDomainName: windows_core::PWSTR,
    pub pFarmName: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_TYPE_CLASS(pub i32);
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WTS_VIRTUAL_CLASS(pub i32);
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
pub const WTS_WSD_LOGOFF: u32 = 1u32;
pub const WTS_WSD_POWEROFF: u32 = 8u32;
pub const WTS_WSD_REBOOT: u32 = 4u32;
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
pub const Workspace: windows_core::GUID = windows_core::GUID::from_u128(0x4f1dfca6_3aad_48e1_8406_4bc21a501d7c);
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(_ITSWkspEvents, _ITSWkspEvents_Vtbl, 0xb922bbb8_4c55_4fea_8496_beb0b44285e9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for _ITSWkspEvents {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(_ITSWkspEvents, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ITSWkspEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ITSWkspEvents_Impl: super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ITSWkspEvents_Vtbl {
    pub const fn new<Identity: _ITSWkspEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ITSWkspEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _ITSWkspEvents {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct pluginResource {
    pub alias: [u16; 256],
    pub name: [u16; 256],
    pub resourceFileContents: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: windows_core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl Default for pluginResource2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
