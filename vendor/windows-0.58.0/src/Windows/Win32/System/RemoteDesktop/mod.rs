#[inline]
pub unsafe fn ProcessIdToSessionId(dwprocessid: u32, psessionid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn ProcessIdToSessionId(dwprocessid : u32, psessionid : *mut u32) -> super::super::Foundation:: BOOL);
    ProcessIdToSessionId(dwprocessid, psessionid).ok()
}
#[inline]
pub unsafe fn WTSCloseServer<P0>(hserver: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSCloseServer(hserver : super::super::Foundation:: HANDLE));
    WTSCloseServer(hserver.param().abi())
}
#[inline]
pub unsafe fn WTSConnectSessionA<P0, P1>(logonid: u32, targetlogonid: u32, ppassword: P0, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSConnectSessionA(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCSTR, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSConnectSessionA(logonid, targetlogonid, ppassword.param().abi(), bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSConnectSessionW<P0, P1>(logonid: u32, targetlogonid: u32, ppassword: P0, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSConnectSessionW(logonid : u32, targetlogonid : u32, ppassword : windows_core::PCWSTR, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSConnectSessionW(logonid, targetlogonid, ppassword.param().abi(), bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSCreateListenerA<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *const WTSLISTENERCONFIGA, flag: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSCreateListenerA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *const WTSLISTENERCONFIGA, flag : u32) -> super::super::Foundation:: BOOL);
    WTSCreateListenerA(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), pbuffer, flag).ok()
}
#[inline]
pub unsafe fn WTSCreateListenerW<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *const WTSLISTENERCONFIGW, flag: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSCreateListenerW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *const WTSLISTENERCONFIGW, flag : u32) -> super::super::Foundation:: BOOL);
    WTSCreateListenerW(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), pbuffer, flag).ok()
}
#[inline]
pub unsafe fn WTSDisconnectSession<P0, P1>(hserver: P0, sessionid: u32, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSDisconnectSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSDisconnectSession(hserver.param().abi(), sessionid, bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSEnableChildSessions<P0>(benable: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnableChildSessions(benable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSEnableChildSessions(benable.param().abi())
}
#[inline]
pub unsafe fn WTSEnumerateListenersA<P0>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut *mut i8>, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut i8, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateListenersA(hserver.param().abi(), preserved, reserved, core::mem::transmute(plisteners.unwrap_or(std::ptr::null_mut())), pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateListenersW<P0>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plisteners: Option<*mut *mut u16>, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateListenersW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plisteners : *mut *mut u16, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateListenersW(hserver.param().abi(), preserved, reserved, core::mem::transmute(plisteners.unwrap_or(std::ptr::null_mut())), pcount).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSEnumerateProcessesA<P0>(hserver: P0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOA, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesA(hserver.param().abi(), reserved, version, ppprocessinfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateProcessesExA<P0>(hserver: P0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PSTR, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PSTR, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesExA(hserver.param().abi(), plevel, sessionid, ppprocessinfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateProcessesExW<P0>(hserver: P0, plevel: *mut u32, sessionid: u32, ppprocessinfo: *mut windows_core::PWSTR, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, sessionid : u32, ppprocessinfo : *mut windows_core::PWSTR, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesExW(hserver.param().abi(), plevel, sessionid, ppprocessinfo, pcount).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSEnumerateProcessesW<P0>(hserver: P0, reserved: u32, version: u32, ppprocessinfo: *mut *mut WTS_PROCESS_INFOW, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateProcessesW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppprocessinfo : *mut *mut WTS_PROCESS_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateProcessesW(hserver.param().abi(), reserved, version, ppprocessinfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateServersA<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOA, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateServersA(pdomainname : windows_core::PCSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateServersA(pdomainname.param().abi(), reserved, version, ppserverinfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateServersW<P0>(pdomainname: P0, reserved: u32, version: u32, ppserverinfo: *mut *mut WTS_SERVER_INFOW, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateServersW(pdomainname : windows_core::PCWSTR, reserved : u32, version : u32, ppserverinfo : *mut *mut WTS_SERVER_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateServersW(pdomainname.param().abi(), reserved, version, ppserverinfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateSessionsA<P0>(hserver: P0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOA, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsA(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOA, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsA(hserver.param().abi(), reserved, version, ppsessioninfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateSessionsExA<P0>(hserver: P0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1A, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExA(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1A, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsExA(hserver.param().abi(), plevel, filter, ppsessioninfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateSessionsExW<P0>(hserver: P0, plevel: *mut u32, filter: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFO_1W, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsExW(hserver : super::super::Foundation:: HANDLE, plevel : *mut u32, filter : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFO_1W, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsExW(hserver.param().abi(), plevel, filter, ppsessioninfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSEnumerateSessionsW<P0>(hserver: P0, reserved: u32, version: u32, ppsessioninfo: *mut *mut WTS_SESSION_INFOW, pcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSEnumerateSessionsW(hserver : super::super::Foundation:: HANDLE, reserved : u32, version : u32, ppsessioninfo : *mut *mut WTS_SESSION_INFOW, pcount : *mut u32) -> super::super::Foundation:: BOOL);
    WTSEnumerateSessionsW(hserver.param().abi(), reserved, version, ppsessioninfo, pcount).ok()
}
#[inline]
pub unsafe fn WTSFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemory(pmemory : *mut core::ffi::c_void));
    WTSFreeMemory(pmemory)
}
#[inline]
pub unsafe fn WTSFreeMemoryExA(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::Result<()> {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExA(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> super::super::Foundation:: BOOL);
    WTSFreeMemoryExA(wtstypeclass, pmemory, numberofentries).ok()
}
#[inline]
pub unsafe fn WTSFreeMemoryExW(wtstypeclass: WTS_TYPE_CLASS, pmemory: *const core::ffi::c_void, numberofentries: u32) -> windows_core::Result<()> {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSFreeMemoryExW(wtstypeclass : WTS_TYPE_CLASS, pmemory : *const core::ffi::c_void, numberofentries : u32) -> super::super::Foundation:: BOOL);
    WTSFreeMemoryExW(wtstypeclass, pmemory, numberofentries).ok()
}
#[inline]
pub unsafe fn WTSGetActiveConsoleSessionId() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> u32);
    WTSGetActiveConsoleSessionId()
}
#[inline]
pub unsafe fn WTSGetChildSessionId(psessionid: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSGetChildSessionId(psessionid : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetChildSessionId(psessionid)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSGetListenerSecurityA<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetListenerSecurityA(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSGetListenerSecurityW<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSGetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> super::super::Foundation:: BOOL);
    WTSGetListenerSecurityW(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor, nlength, lpnlengthneeded).ok()
}
#[inline]
pub unsafe fn WTSIsChildSessionsEnabled(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSIsChildSessionsEnabled(pbenabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSIsChildSessionsEnabled(pbenabled)
}
#[inline]
pub unsafe fn WTSLogoffSession<P0, P1>(hserver: P0, sessionid: u32, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSLogoffSession(hserver : super::super::Foundation:: HANDLE, sessionid : u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSLogoffSession(hserver.param().abi(), sessionid, bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSOpenServerA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerA(pservername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerA(pservername.param().abi())
}
#[inline]
pub unsafe fn WTSOpenServerExA<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerExA(pservername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerExA(pservername.param().abi())
}
#[inline]
pub unsafe fn WTSOpenServerExW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerExW(pservername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerExW(pservername.param().abi())
}
#[inline]
pub unsafe fn WTSOpenServerW<P0>(pservername: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSOpenServerW(pservername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    WTSOpenServerW(pservername.param().abi())
}
#[inline]
pub unsafe fn WTSQueryListenerConfigA<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *mut WTSLISTENERCONFIGA) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, pbuffer : *mut WTSLISTENERCONFIGA) -> super::super::Foundation:: BOOL);
    WTSQueryListenerConfigA(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), pbuffer).ok()
}
#[inline]
pub unsafe fn WTSQueryListenerConfigW<P0, P1>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, pbuffer: *mut WTSLISTENERCONFIGW) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryListenerConfigW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, pbuffer : *mut WTSLISTENERCONFIGW) -> super::super::Foundation:: BOOL);
    WTSQueryListenerConfigW(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), pbuffer).ok()
}
#[inline]
pub unsafe fn WTSQuerySessionInformationA<P0>(hserver: P0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQuerySessionInformationA(hserver.param().abi(), sessionid, wtsinfoclass, ppbuffer, pbytesreturned).ok()
}
#[inline]
pub unsafe fn WTSQuerySessionInformationW<P0>(hserver: P0, sessionid: u32, wtsinfoclass: WTS_INFO_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQuerySessionInformationW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, wtsinfoclass : WTS_INFO_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQuerySessionInformationW(hserver.param().abi(), sessionid, wtsinfoclass, ppbuffer, pbytesreturned).ok()
}
#[inline]
pub unsafe fn WTSQueryUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQueryUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer, pbytesreturned).ok()
}
#[inline]
pub unsafe fn WTSQueryUserConfigW<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, ppbuffer: *mut windows_core::PWSTR, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, ppbuffer : *mut windows_core::PWSTR, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSQueryUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, ppbuffer, pbytesreturned).ok()
}
#[inline]
pub unsafe fn WTSQueryUserToken(sessionid: u32, phtoken: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSQueryUserToken(sessionid : u32, phtoken : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSQueryUserToken(sessionid, phtoken).ok()
}
#[inline]
pub unsafe fn WTSRegisterSessionNotification<P0>(hwnd: P0, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : super::super::Foundation:: HWND, dwflags : u32) -> super::super::Foundation:: BOOL);
    WTSRegisterSessionNotification(hwnd.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn WTSRegisterSessionNotificationEx<P0, P1>(hserver: P0, hwnd: P1, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND, dwflags : u32) -> super::super::Foundation:: BOOL);
    WTSRegisterSessionNotificationEx(hserver.param().abi(), hwnd.param().abi(), dwflags).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn WTSSendMessageA<P0, P1>(hserver: P0, sessionid: u32, ptitle: &[u8], pmessage: &[u8], style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSendMessageA(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_core::PCSTR, titlelength : u32, pmessage : windows_core::PCSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSSendMessageA(hserver.param().abi(), sessionid, core::mem::transmute(ptitle.as_ptr()), ptitle.len().try_into().unwrap(), core::mem::transmute(pmessage.as_ptr()), pmessage.len().try_into().unwrap(), style, timeout, presponse, bwait.param().abi()).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn WTSSendMessageW<P0, P1, P2, P3>(hserver: P0, sessionid: u32, ptitle: P1, titlelength: u32, pmessage: P2, messagelength: u32, style: super::super::UI::WindowsAndMessaging::MESSAGEBOX_STYLE, timeout: u32, presponse: *mut super::super::UI::WindowsAndMessaging::MESSAGEBOX_RESULT, bwait: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSendMessageW(hserver : super::super::Foundation:: HANDLE, sessionid : u32, ptitle : windows_core::PCWSTR, titlelength : u32, pmessage : windows_core::PCWSTR, messagelength : u32, style : super::super::UI::WindowsAndMessaging:: MESSAGEBOX_STYLE, timeout : u32, presponse : *mut super::super::UI::WindowsAndMessaging:: MESSAGEBOX_RESULT, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WTSSendMessageW(hserver.param().abi(), sessionid, ptitle.param().abi(), titlelength, pmessage.param().abi(), messagelength, style, timeout, presponse, bwait.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSSetListenerSecurityA<P0, P1, P2>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityA(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: BOOL);
    WTSSetListenerSecurityA(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn WTSSetListenerSecurityW<P0, P1, P2>(hserver: P0, preserved: *const core::ffi::c_void, reserved: u32, plistenername: P1, securityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSetListenerSecurityW(hserver : super::super::Foundation:: HANDLE, preserved : *const core::ffi::c_void, reserved : u32, plistenername : windows_core::PCWSTR, securityinformation : super::super::Security:: OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR) -> super::super::Foundation:: BOOL);
    WTSSetListenerSecurityW(hserver.param().abi(), preserved, reserved, plistenername.param().abi(), securityinformation, psecuritydescriptor.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSSetRenderHint<P0>(prenderhintid: *mut u64, hwndowner: P0, renderhinttype: u32, phintdata: Option<&[u8]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSetRenderHint(prenderhintid : *mut u64, hwndowner : super::super::Foundation:: HWND, renderhinttype : u32, cbhintdatalength : u32, phintdata : *const u8) -> windows_core::HRESULT);
    WTSSetRenderHint(prenderhintid, hwndowner.param().abi(), renderhinttype, phintdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(phintdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn WTSSetUserConfigA<P0, P1>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSetUserConfigA(pservername : windows_core::PCSTR, pusername : windows_core::PCSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCSTR, datalength : u32) -> super::super::Foundation:: BOOL);
    WTSSetUserConfigA(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn WTSSetUserConfigW<P0, P1, P2>(pservername: P0, pusername: P1, wtsconfigclass: WTS_CONFIG_CLASS, pbuffer: P2, datalength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSSetUserConfigW(pservername : windows_core::PCWSTR, pusername : windows_core::PCWSTR, wtsconfigclass : WTS_CONFIG_CLASS, pbuffer : windows_core::PCWSTR, datalength : u32) -> super::super::Foundation:: BOOL);
    WTSSetUserConfigW(pservername.param().abi(), pusername.param().abi(), wtsconfigclass, pbuffer.param().abi(), datalength).ok()
}
#[inline]
pub unsafe fn WTSShutdownSystem<P0>(hserver: P0, shutdownflag: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSShutdownSystem(hserver : super::super::Foundation:: HANDLE, shutdownflag : u32) -> super::super::Foundation:: BOOL);
    WTSShutdownSystem(hserver.param().abi(), shutdownflag).ok()
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionA<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionA(ptargetservername : windows_core::PCSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> super::super::Foundation:: BOOL);
    WTSStartRemoteControlSessionA(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok()
}
#[inline]
pub unsafe fn WTSStartRemoteControlSessionW<P0>(ptargetservername: P0, targetlogonid: u32, hotkeyvk: u8, hotkeymodifiers: u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSStartRemoteControlSessionW(ptargetservername : windows_core::PCWSTR, targetlogonid : u32, hotkeyvk : u8, hotkeymodifiers : u16) -> super::super::Foundation:: BOOL);
    WTSStartRemoteControlSessionW(ptargetservername.param().abi(), targetlogonid, hotkeyvk, hotkeymodifiers).ok()
}
#[inline]
pub unsafe fn WTSStopRemoteControlSession(logonid: u32) -> windows_core::Result<()> {
    windows_targets::link!("wtsapi32.dll" "system" fn WTSStopRemoteControlSession(logonid : u32) -> super::super::Foundation:: BOOL);
    WTSStopRemoteControlSession(logonid).ok()
}
#[inline]
pub unsafe fn WTSTerminateProcess<P0>(hserver: P0, processid: u32, exitcode: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSTerminateProcess(hserver : super::super::Foundation:: HANDLE, processid : u32, exitcode : u32) -> super::super::Foundation:: BOOL);
    WTSTerminateProcess(hserver.param().abi(), processid, exitcode).ok()
}
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    WTSUnRegisterSessionNotification(hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSUnRegisterSessionNotificationEx<P0, P1>(hserver: P0, hwnd: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotificationEx(hserver : super::super::Foundation:: HANDLE, hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    WTSUnRegisterSessionNotificationEx(hserver.param().abi(), hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelClose<P0>(hchannelhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelClose(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelClose(hchannelhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelOpen<P0, P1>(hserver: P0, sessionid: u32, pvirtualname: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpen(hserver : super::super::Foundation:: HANDLE, sessionid : u32, pvirtualname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = WTSVirtualChannelOpen(hserver.param().abi(), sessionid, pvirtualname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn WTSVirtualChannelOpenEx<P0>(sessionid: u32, pvirtualname: P0, flags: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelOpenEx(sessionid : u32, pvirtualname : windows_core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
    let result__ = WTSVirtualChannelOpenEx(sessionid, pvirtualname.param().abi(), flags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn WTSVirtualChannelPurgeInput<P0>(hchannelhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeInput(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelPurgeInput(hchannelhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelPurgeOutput<P0>(hchannelhandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelPurgeOutput(hchannelhandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelPurgeOutput(hchannelhandle.param().abi()).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelQuery<P0>(hchannelhandle: P0, param1: WTS_VIRTUAL_CLASS, ppbuffer: *mut *mut core::ffi::c_void, pbytesreturned: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelQuery(hchannelhandle : super::super::Foundation:: HANDLE, param1 : WTS_VIRTUAL_CLASS, ppbuffer : *mut *mut core::ffi::c_void, pbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelQuery(hchannelhandle.param().abi(), param1, ppbuffer, pbytesreturned).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelRead<P0>(hchannelhandle: P0, timeout: u32, buffer: &mut [u8], pbytesread: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelRead(hchannelhandle : super::super::Foundation:: HANDLE, timeout : u32, buffer : windows_core::PSTR, buffersize : u32, pbytesread : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelRead(hchannelhandle.param().abi(), timeout, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), pbytesread).ok()
}
#[inline]
pub unsafe fn WTSVirtualChannelWrite<P0>(hchannelhandle: P0, buffer: &[u8], pbyteswritten: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSVirtualChannelWrite(hchannelhandle : super::super::Foundation:: HANDLE, buffer : windows_core::PCSTR, length : u32, pbyteswritten : *mut u32) -> super::super::Foundation:: BOOL);
    WTSVirtualChannelWrite(hchannelhandle.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), pbyteswritten).ok()
}
#[inline]
pub unsafe fn WTSWaitSystemEvent<P0>(hserver: P0, eventmask: u32, peventflags: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("wtsapi32.dll" "system" fn WTSWaitSystemEvent(hserver : super::super::Foundation:: HANDLE, eventmask : u32, peventflags : *mut u32) -> super::super::Foundation:: BOOL);
    WTSWaitSystemEvent(hserver.param().abi(), eventmask, peventflags).ok()
}
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
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalServicesProfilePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTerminalServicesProfilePath<P0>(&self, pnewval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTerminalServicesProfilePath)(windows_core::Interface::as_raw(self), pnewval.param().abi()).ok()
    }
    pub unsafe fn TerminalServicesHomeDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalServicesHomeDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTerminalServicesHomeDirectory<P0>(&self, pnewval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTerminalServicesHomeDirectory)(windows_core::Interface::as_raw(self), pnewval.param().abi()).ok()
    }
    pub unsafe fn TerminalServicesHomeDrive(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalServicesHomeDrive)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTerminalServicesHomeDrive<P0>(&self, pnewval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTerminalServicesHomeDrive)(windows_core::Interface::as_raw(self), pnewval.param().abi()).ok()
    }
    pub unsafe fn AllowLogon(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowLogon(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAllowLogon)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn EnableRemoteControl(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnableRemoteControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnableRemoteControl(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEnableRemoteControl)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxDisconnectionTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxDisconnectionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxDisconnectionTime(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxDisconnectionTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxConnectionTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxConnectionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxConnectionTime(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxConnectionTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn MaxIdleTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxIdleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxIdleTime(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxIdleTime)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ReconnectionAction(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReconnectionAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReconnectionAction(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReconnectionAction)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn BrokenConnectionAction(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BrokenConnectionAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBrokenConnectionAction(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBrokenConnectionAction)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ConnectClientDrivesAtLogon(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectClientDrivesAtLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConnectClientDrivesAtLogon(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectClientDrivesAtLogon)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn ConnectClientPrintersAtLogon(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectClientPrintersAtLogon)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetConnectClientPrintersAtLogon(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectClientPrintersAtLogon)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn DefaultToMainPrinter(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultToMainPrinter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefaultToMainPrinter(&self, newval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultToMainPrinter)(windows_core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn TerminalServicesWorkDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalServicesWorkDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTerminalServicesWorkDirectory<P0>(&self, pnewval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTerminalServicesWorkDirectory)(windows_core::Interface::as_raw(self), pnewval.param().abi()).ok()
    }
    pub unsafe fn TerminalServicesInitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TerminalServicesInitialProgram)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTerminalServicesInitialProgram<P0>(&self, pnewval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTerminalServicesInitialProgram)(windows_core::Interface::as_raw(self), pnewval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsTSUserEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TerminalServicesProfilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTerminalServicesProfilePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TerminalServicesHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTerminalServicesHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TerminalServicesHomeDrive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTerminalServicesHomeDrive: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub TerminalServicesWorkDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTerminalServicesWorkDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TerminalServicesInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTerminalServicesInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioDeviceEndpoint, IAudioDeviceEndpoint_Vtbl, 0xd4952f5a_a0b2_4cc4_8b82_9358488dd8ac);
impl core::ops::Deref for IAudioDeviceEndpoint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioDeviceEndpoint, windows_core::IUnknown);
impl IAudioDeviceEndpoint {
    pub unsafe fn SetBuffer(&self, maxperiod: i64, u32latencycoefficient: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBuffer)(windows_core::Interface::as_raw(self), maxperiod, u32latencycoefficient).ok()
    }
    pub unsafe fn GetRTCaps(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRTCaps)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetEventDrivenCapable(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEventDrivenCapable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WriteExclusiveModeParametersToSharedMemory(&self, htargetprocess: usize, hnsperiod: i64, hnsbufferduration: i64, u32latencycoefficient: u32, pu32sharedmemorysize: *mut u32, phsharedmemory: *mut usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WriteExclusiveModeParametersToSharedMemory)(windows_core::Interface::as_raw(self), htargetprocess, hnsperiod, hnsbufferduration, u32latencycoefficient, pu32sharedmemorysize, phsharedmemory).ok()
    }
}
#[repr(C)]
pub struct IAudioDeviceEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
    pub GetRTCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetEventDrivenCapable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub WriteExclusiveModeParametersToSharedMemory: unsafe extern "system" fn(*mut core::ffi::c_void, usize, i64, i64, u32, *mut u32, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpoint, IAudioEndpoint_Vtbl, 0x30a99515_1527_4451_af9f_00c5f0234daf);
impl core::ops::Deref for IAudioEndpoint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpoint, windows_core::IUnknown);
impl IAudioEndpoint {
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFrameFormat(&self) -> windows_core::Result<*mut super::super::Media::Audio::WAVEFORMATEX> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFramesPerPacket(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFramesPerPacket)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetLatency(&self) -> windows_core::Result<i64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStreamFlags(&self, streamflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStreamFlags)(windows_core::Interface::as_raw(self), streamflags).ok()
    }
    pub unsafe fn SetEventHandle<P0>(&self, eventhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE>,
    {
        (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), eventhandle.param().abi()).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IAudioEndpointControl, IAudioEndpointControl_Vtbl, 0xc684b72a_6df4_4774_bdf9_76b77509b653);
impl core::ops::Deref for IAudioEndpointControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointControl, windows_core::IUnknown);
impl IAudioEndpointControl {
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAudioEndpointControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioEndpointRT, IAudioEndpointRT_Vtbl, 0xdfd2005f_a6e5_4d39_a265_939ada9fbb4d);
impl core::ops::Deref for IAudioEndpointRT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointRT, windows_core::IUnknown);
impl IAudioEndpointRT {
    pub unsafe fn GetCurrentPadding(&self, ppadding: *mut i64, paecurrentposition: *mut AE_CURRENT_POSITION) {
        (windows_core::Interface::vtable(self).GetCurrentPadding)(windows_core::Interface::as_raw(self), ppadding, paecurrentposition)
    }
    pub unsafe fn ProcessingComplete(&self) {
        (windows_core::Interface::vtable(self).ProcessingComplete)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetPinInactive(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPinInactive)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPinActive(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPinActive)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAudioEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut AE_CURRENT_POSITION),
    pub ProcessingComplete: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SetPinInactive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPinActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioInputEndpointRT, IAudioInputEndpointRT_Vtbl, 0x8026ab61_92b2_43c1_a1df_5c37ebd08d82);
impl core::ops::Deref for IAudioInputEndpointRT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioInputEndpointRT, windows_core::IUnknown);
impl IAudioInputEndpointRT {
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn GetInputDataPointer(&self, pconnectionproperty: *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, paetimestamp: *mut AE_CURRENT_POSITION) {
        (windows_core::Interface::vtable(self).GetInputDataPointer)(windows_core::Interface::as_raw(self), pconnectionproperty, paetimestamp)
    }
    pub unsafe fn ReleaseInputDataPointer(&self, u32framecount: u32, pdatapointer: usize) {
        (windows_core::Interface::vtable(self).ReleaseInputDataPointer)(windows_core::Interface::as_raw(self), u32framecount, pdatapointer)
    }
    pub unsafe fn PulseEndpoint(&self) {
        (windows_core::Interface::vtable(self).PulseEndpoint)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IAudioInputEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub GetInputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY, *mut AE_CURRENT_POSITION),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    GetInputDataPointer: usize,
    pub ReleaseInputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize),
    pub PulseEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IAudioOutputEndpointRT, IAudioOutputEndpointRT_Vtbl, 0x8fa906e4_c31c_4e31_932e_19a66385e9aa);
impl core::ops::Deref for IAudioOutputEndpointRT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioOutputEndpointRT, windows_core::IUnknown);
impl IAudioOutputEndpointRT {
    pub unsafe fn GetOutputDataPointer(&self, u32framecount: u32, paetimestamp: *const AE_CURRENT_POSITION) -> usize {
        (windows_core::Interface::vtable(self).GetOutputDataPointer)(windows_core::Interface::as_raw(self), u32framecount, paetimestamp)
    }
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub unsafe fn ReleaseOutputDataPointer(&self, pconnectionproperty: *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY) {
        (windows_core::Interface::vtable(self).ReleaseOutputDataPointer)(windows_core::Interface::as_raw(self), pconnectionproperty)
    }
    pub unsafe fn PulseEndpoint(&self) {
        (windows_core::Interface::vtable(self).PulseEndpoint)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IAudioOutputEndpointRT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOutputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const AE_CURRENT_POSITION) -> usize,
    #[cfg(feature = "Win32_Media_Audio_Apo")]
    pub ReleaseOutputDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Media::Audio::Apo::APO_CONNECTION_PROPERTY),
    #[cfg(not(feature = "Win32_Media_Audio_Apo"))]
    ReleaseOutputDataPointer: usize,
    pub PulseEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void),
}
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
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reconnect(&self, width: u32, height: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reconnect)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> windows_core::Result<IRemoteDesktopClientSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Actions(&self) -> windows_core::Result<IRemoteDesktopClientActions> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TouchPointer(&self) -> windows_core::Result<IRemoteDesktopClientTouchPointer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TouchPointer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteSavedCredentials<P0>(&self, servername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteSavedCredentials)(windows_core::Interface::as_raw(self), servername.param().abi()).ok()
    }
    pub unsafe fn UpdateSessionDisplaySettings(&self, width: u32, height: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UpdateSessionDisplaySettings)(windows_core::Interface::as_raw(self), width, height).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn attachEvent<P0, P1>(&self, eventname: P0, callback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).attachEvent)(windows_core::Interface::as_raw(self), eventname.param().abi(), callback.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn detachEvent<P0, P1>(&self, eventname: P0, callback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).detachEvent)(windows_core::Interface::as_raw(self), eventname.param().abi(), callback.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reconnect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub TouchPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    TouchPointer: usize,
    pub DeleteSavedCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UpdateSessionDisplaySettings: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub attachEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    attachEvent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub detachEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    detachEvent: usize,
}
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
        (windows_core::Interface::vtable(self).SuspendScreenUpdates)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeScreenUpdates(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResumeScreenUpdates)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExecuteRemoteAction(&self, remoteaction: RemoteActionType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExecuteRemoteAction)(windows_core::Interface::as_raw(self), remoteaction).ok()
    }
    pub unsafe fn GetSnapshot(&self, snapshotencoding: SnapshotEncodingType, snapshotformat: SnapshotFormatType, snapshotwidth: u32, snapshotheight: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSnapshot)(windows_core::Interface::as_raw(self), snapshotencoding, snapshotformat, snapshotwidth, snapshotheight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientActions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SuspendScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResumeScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteRemoteAction: unsafe extern "system" fn(*mut core::ffi::c_void, RemoteActionType) -> windows_core::HRESULT,
    pub GetSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, SnapshotEncodingType, SnapshotFormatType, u32, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn ApplySettings<P0>(&self, rdpfilecontents: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ApplySettings)(windows_core::Interface::as_raw(self), rdpfilecontents.param().abi()).ok()
    }
    pub unsafe fn RetrieveSettings(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RetrieveSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRdpProperty<P0>(&self, propertyname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRdpProperty)(windows_core::Interface::as_raw(self), propertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRdpProperty<P0, P1>(&self, propertyname: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetRdpProperty)(windows_core::Interface::as_raw(self), propertyname.param().abi(), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ApplySettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RetrieveSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRdpProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetRdpProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
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
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventsEnabled<P0>(&self, eventsenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEventsEnabled)(windows_core::Interface::as_raw(self), eventsenabled.param().abi()).ok()
    }
    pub unsafe fn EventsEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPointerSpeed(&self, pointerspeed: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPointerSpeed)(windows_core::Interface::as_raw(self), pointerspeed).ok()
    }
    pub unsafe fn PointerSpeed(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PointerSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IRemoteDesktopClientTouchPointer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEventsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EventsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPointerSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PointerSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteSystemAdditionalInfoProvider, IRemoteSystemAdditionalInfoProvider_Vtbl, 0xeeaa3d5f_ec63_4d27_af38_e86b1d7292cb);
impl core::ops::Deref for IRemoteSystemAdditionalInfoProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRemoteSystemAdditionalInfoProvider, windows_core::IUnknown);
impl IRemoteSystemAdditionalInfoProvider {
    pub unsafe fn GetAdditionalInfo<T>(&self, deduplicationid: *mut windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetAdditionalInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(deduplicationid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRemoteSystemAdditionalInfoProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdditionalInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGAccountingEngine, ITSGAccountingEngine_Vtbl, 0x4ce2a0c9_e874_4f1a_86f4_06bbb9115338);
impl core::ops::Deref for ITSGAccountingEngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGAccountingEngine, windows_core::IUnknown);
impl ITSGAccountingEngine {
    pub unsafe fn DoAccounting(&self, accountingdatatype: AAAccountingDataType, accountingdata: AAAccountingData) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoAccounting)(windows_core::Interface::as_raw(self), accountingdatatype, core::mem::transmute(accountingdata)).ok()
    }
}
#[repr(C)]
pub struct ITSGAccountingEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoAccounting: unsafe extern "system" fn(*mut core::ffi::c_void, AAAccountingDataType, AAAccountingData) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGAuthenticateUserSink, ITSGAuthenticateUserSink_Vtbl, 0x2c3e2e73_a782_47f9_8dfb_77ee1ed27a03);
impl core::ops::Deref for ITSGAuthenticateUserSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGAuthenticateUserSink, windows_core::IUnknown);
impl ITSGAuthenticateUserSink {
    pub unsafe fn OnUserAuthenticated<P0, P1, P2>(&self, username: P0, userdomain: P1, context: usize, usertoken: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
    {
        (windows_core::Interface::vtable(self).OnUserAuthenticated)(windows_core::Interface::as_raw(self), username.param().abi(), userdomain.param().abi(), context, usertoken.param().abi()).ok()
    }
    pub unsafe fn OnUserAuthenticationFailed(&self, context: usize, genericerrorcode: windows_core::HRESULT, specificerrorcode: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUserAuthenticationFailed)(windows_core::Interface::as_raw(self), context, genericerrorcode, specificerrorcode).ok()
    }
    pub unsafe fn ReauthenticateUser(&self, context: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReauthenticateUser)(windows_core::Interface::as_raw(self), context).ok()
    }
    pub unsafe fn DisconnectUser(&self, context: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectUser)(windows_core::Interface::as_raw(self), context).ok()
    }
}
#[repr(C)]
pub struct ITSGAuthenticateUserSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUserAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, usize, super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub OnUserAuthenticationFailed: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::HRESULT, windows_core::HRESULT) -> windows_core::HRESULT,
    pub ReauthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub DisconnectUser: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGAuthenticationEngine, ITSGAuthenticationEngine_Vtbl, 0x9ee3e5bf_04ab_4691_998c_d7f622321a56);
impl core::ops::Deref for ITSGAuthenticationEngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGAuthenticationEngine, windows_core::IUnknown);
impl ITSGAuthenticationEngine {
    pub unsafe fn AuthenticateUser<P0>(&self, mainsessionid: windows_core::GUID, cookiedata: *const u8, numcookiebytes: u32, context: usize, psink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITSGAuthenticateUserSink>,
    {
        (windows_core::Interface::vtable(self).AuthenticateUser)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), cookiedata, numcookiebytes, context, psink.param().abi()).ok()
    }
    pub unsafe fn CancelAuthentication(&self, mainsessionid: windows_core::GUID, context: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelAuthentication)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), context).ok()
    }
}
#[repr(C)]
pub struct ITSGAuthenticationEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AuthenticateUser: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const u8, u32, usize, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelAuthentication: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGAuthorizeConnectionSink, ITSGAuthorizeConnectionSink_Vtbl, 0xc27ece33_7781_4318_98ef_1cf2da7b7005);
impl core::ops::Deref for ITSGAuthorizeConnectionSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGAuthorizeConnectionSink, windows_core::IUnknown);
impl ITSGAuthorizeConnectionSink {
    pub unsafe fn OnConnectionAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, pbsohresponse: &[u8], idletimeout: u32, sessiontimeout: u32, sessiontimeoutaction: SESSION_TIMEOUT_ACTION_TYPE, trustclass: AATrustClassID, policyattributes: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnConnectionAuthorized)(windows_core::Interface::as_raw(self), hrin, core::mem::transmute(mainsessionid), pbsohresponse.len().try_into().unwrap(), core::mem::transmute(pbsohresponse.as_ptr()), idletimeout, sessiontimeout, sessiontimeoutaction, trustclass, policyattributes).ok()
    }
}
#[repr(C)]
pub struct ITSGAuthorizeConnectionSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnectionAuthorized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::GUID, u32, *const u8, u32, u32, SESSION_TIMEOUT_ACTION_TYPE, AATrustClassID, *const u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGAuthorizeResourceSink, ITSGAuthorizeResourceSink_Vtbl, 0xfeddfcd4_fa12_4435_ae55_7ad1a9779af7);
impl core::ops::Deref for ITSGAuthorizeResourceSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGAuthorizeResourceSink, windows_core::IUnknown);
impl ITSGAuthorizeResourceSink {
    pub unsafe fn OnChannelAuthorized(&self, hrin: windows_core::HRESULT, mainsessionid: windows_core::GUID, subsessionid: i32, allowedresourcenames: &[windows_core::BSTR], failedresourcenames: &[windows_core::BSTR]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChannelAuthorized)(windows_core::Interface::as_raw(self), hrin, core::mem::transmute(mainsessionid), subsessionid, core::mem::transmute(allowedresourcenames.as_ptr()), allowedresourcenames.len().try_into().unwrap(), core::mem::transmute(failedresourcenames.as_ptr()), failedresourcenames.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct ITSGAuthorizeResourceSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChannelAuthorized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::GUID, i32, *const core::mem::MaybeUninit<windows_core::BSTR>, u32, *const core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITSGPolicyEngine, ITSGPolicyEngine_Vtbl, 0x8bc24f08_6223_42f4_a5b4_8e37cd135bbd);
impl core::ops::Deref for ITSGPolicyEngine {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITSGPolicyEngine, windows_core::IUnknown);
impl ITSGPolicyEngine {
    pub unsafe fn AuthorizeConnection<P0, P1, P2, P3, P4>(&self, mainsessionid: windows_core::GUID, username: P0, authtype: AAAuthSchemes, clientmachineip: P1, clientmachinename: P2, sohdata: &[u8], cookiedata: &[u8], usertoken: P3, psink: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P4: windows_core::Param<ITSGAuthorizeConnectionSink>,
    {
        (windows_core::Interface::vtable(self).AuthorizeConnection)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), username.param().abi(), authtype, clientmachineip.param().abi(), clientmachinename.param().abi(), core::mem::transmute(sohdata.as_ptr()), sohdata.len().try_into().unwrap(), core::mem::transmute(cookiedata.as_ptr()), cookiedata.len().try_into().unwrap(), usertoken.param().abi(), psink.param().abi()).ok()
    }
    pub unsafe fn AuthorizeResource<P0, P1, P2>(&self, mainsessionid: windows_core::GUID, subsessionid: i32, username: P0, resourcenames: &[windows_core::BSTR], alternateresourcenames: &[windows_core::BSTR], portnumber: u32, operation: P1, cookie: &[u8], psink: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<ITSGAuthorizeResourceSink>,
    {
        (windows_core::Interface::vtable(self).AuthorizeResource)(windows_core::Interface::as_raw(self), core::mem::transmute(mainsessionid), subsessionid, username.param().abi(), core::mem::transmute(resourcenames.as_ptr()), resourcenames.len().try_into().unwrap(), core::mem::transmute(alternateresourcenames.as_ptr()), alternateresourcenames.len().try_into().unwrap(), portnumber, operation.param().abi(), core::mem::transmute(cookie.as_ptr()), cookie.len().try_into().unwrap(), psink.param().abi()).ok()
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsQuarantineEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsQuarantineEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITSGPolicyEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AuthorizeConnection: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, AAAuthSchemes, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const u8, u32, *const u8, u32, super::super::Foundation::HANDLE_PTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthorizeResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, i32, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::BSTR>, u32, *const core::mem::MaybeUninit<windows_core::BSTR>, u32, u32, core::mem::MaybeUninit<windows_core::BSTR>, *const u8, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsQuarantineEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbBaseNotifySink, ITsSbBaseNotifySink_Vtbl, 0x808a6537_1282_4989_9e09_f43938b71722);
impl core::ops::Deref for ITsSbBaseNotifySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbBaseNotifySink, windows_core::IUnknown);
impl ITsSbBaseNotifySink {
    pub unsafe fn OnError(&self, hrerror: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnReportStatus)(windows_core::Interface::as_raw(self), messagetype, messageid).ok()
    }
}
#[repr(C)]
pub struct ITsSbBaseNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnReportStatus: unsafe extern "system" fn(*mut core::ffi::c_void, CLIENT_MESSAGE_TYPE, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbClientConnection, ITsSbClientConnection_Vtbl, 0x18857499_ad61_4b1b_b7df_cbcd41fb8338);
impl core::ops::Deref for ITsSbClientConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbClientConnection, windows_core::IUnknown);
impl ITsSbClientConnection {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialProgram)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LoadBalanceResult(&self) -> windows_core::Result<ITsSbLoadBalanceResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadBalanceResult)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FarmName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FarmName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutContext<P0, P1>(&self, contextid: P0, context: P1, existingcontext: Option<*mut windows_core::VARIANT>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).PutContext)(windows_core::Interface::as_raw(self), contextid.param().abi(), context.param().abi(), core::mem::transmute(existingcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetContext<P0>(&self, contextid: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), contextid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Environment(&self) -> windows_core::Result<ITsSbEnvironment> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Environment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn get_ConnectionError(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).get_ConnectionError)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SamUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SamUserAccount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn ClientConnectionPropertySet(&self) -> windows_core::Result<ITsSbClientConnectionPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientConnectionPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsFirstAssignment(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFirstAssignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RdFarmType(&self) -> windows_core::Result<RD_FARM_TYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RdFarmType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UserSidString(&self) -> windows_core::Result<*mut i8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserSidString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDisconnectedSession(&self) -> windows_core::Result<ITsSbSession> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisconnectedSession)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITsSbClientConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LoadBalanceResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PutContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Environment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectionError: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SamUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub ClientConnectionPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    ClientConnectionPropertySet: usize,
    pub IsFirstAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RdFarmType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RD_FARM_TYPE) -> windows_core::HRESULT,
    pub UserSidString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut i8) -> windows_core::HRESULT,
    pub GetDisconnectedSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
impl ITsSbClientConnectionPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbClientConnectionPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
windows_core::imp::define_interface!(ITsSbEnvironment, ITsSbEnvironment_Vtbl, 0x8c87f7f7_bf51_4a5c_87bf_8e94fb6e2256);
impl core::ops::Deref for ITsSbEnvironment {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbEnvironment, windows_core::IUnknown);
impl ITsSbEnvironment {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServerWeight(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerWeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnvironmentPropertySet(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnvironmentPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetEnvironmentPropertySet<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironmentPropertySet>,
    {
        (windows_core::Interface::vtable(self).SetEnvironmentPropertySet)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbEnvironment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
impl ITsSbEnvironmentPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbEnvironmentPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
windows_core::imp::define_interface!(ITsSbFilterPluginStore, ITsSbFilterPluginStore_Vtbl, 0x85b44b0f_ed78_413f_9702_fa6d3b5ee755);
impl core::ops::Deref for ITsSbFilterPluginStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbFilterPluginStore, windows_core::IUnknown);
impl ITsSbFilterPluginStore {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SaveProperties<P0>(&self, ppropertyset: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbPropertySet>,
    {
        (windows_core::Interface::vtable(self).SaveProperties)(windows_core::Interface::as_raw(self), ppropertyset.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn EnumerateProperties(&self) -> windows_core::Result<ITsSbPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumerateProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteProperties<P0>(&self, propertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteProperties)(windows_core::Interface::as_raw(self), propertyname.param().abi()).ok()
    }
}
#[repr(C)]
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
    pub DeleteProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbGenericNotifySink, ITsSbGenericNotifySink_Vtbl, 0x4c4c8c4f_300b_46ad_9164_8468a7e7568c);
impl core::ops::Deref for ITsSbGenericNotifySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbGenericNotifySink, windows_core::IUnknown);
impl ITsSbGenericNotifySink {
    pub unsafe fn OnCompleted(&self, status: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnCompleted)(windows_core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn GetWaitTimeout(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWaitTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITsSbGenericNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetWaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbGlobalStore, ITsSbGlobalStore_Vtbl, 0x9ab60f7b_bd72_4d9f_8a3a_a0ea5574e635);
impl core::ops::Deref for ITsSbGlobalStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbGlobalStore, windows_core::IUnknown);
impl ITsSbGlobalStore {
    pub unsafe fn QueryTarget<P0, P1, P2>(&self, providername: P0, targetname: P1, farmname: P2) -> windows_core::Result<ITsSbTarget>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryTarget)(windows_core::Interface::as_raw(self), providername.param().abi(), targetname.param().abi(), farmname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuerySessionBySessionId<P0, P1>(&self, providername: P0, dwsessionid: u32, targetname: P1) -> windows_core::Result<ITsSbSession>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySessionBySessionId)(windows_core::Interface::as_raw(self), providername.param().abi(), dwsessionid, targetname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms<P0>(&self, providername: P0, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateFarms)(windows_core::Interface::as_raw(self), providername.param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateTargets<P0, P1, P2>(&self, providername: P0, farmname: P1, envname: P2, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateTargets)(windows_core::Interface::as_raw(self), providername.param().abi(), farmname.param().abi(), envname.param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateEnvironmentsByProvider<P0>(&self, providername: P0, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateEnvironmentsByProvider)(windows_core::Interface::as_raw(self), providername.param().abi(), pdwcount, ppval).ok()
    }
    pub unsafe fn EnumerateSessions<P0, P1, P2, P3, P4, P5>(&self, providername: P0, targetname: P1, username: P2, userdomain: P3, poolname: P4, initialprogram: P5, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateSessions)(windows_core::Interface::as_raw(self), providername.param().abi(), targetname.param().abi(), username.param().abi(), userdomain.param().abi(), poolname.param().abi(), initialprogram.param().abi(), psessionstate, pdwcount, ppval).ok()
    }
    pub unsafe fn GetFarmProperty<P0, P1>(&self, farmname: P0, propertyname: P1, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetFarmProperty)(windows_core::Interface::as_raw(self), farmname.param().abi(), propertyname.param().abi(), core::mem::transmute(pvarvalue)).ok()
    }
}
#[repr(C)]
pub struct ITsSbGlobalStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub EnumerateTargets: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut Option<ITsSbTarget>) -> windows_core::HRESULT,
    pub EnumerateEnvironmentsByProvider: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut Option<ITsSbEnvironment>) -> windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const TSSESSION_STATE, *mut u32, *mut *mut Option<ITsSbSession>) -> windows_core::HRESULT,
    pub GetFarmProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbLoadBalanceResult, ITsSbLoadBalanceResult_Vtbl, 0x24fdb7ac_fea6_11dc_9672_9a8956d89593);
impl core::ops::Deref for ITsSbLoadBalanceResult {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbLoadBalanceResult, windows_core::IUnknown);
impl ITsSbLoadBalanceResult {
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITsSbLoadBalanceResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).GetMostSuitableTarget)(windows_core::Interface::as_raw(self), pconnection.param().abi(), plbsink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbLoadBalancing_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub GetMostSuitableTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbLoadBalancingNotifySink, ITsSbLoadBalancingNotifySink_Vtbl, 0x5f8a8297_3244_4e6a_958a_27c822c1e141);
impl core::ops::Deref for ITsSbLoadBalancingNotifySink {
    type Target = ITsSbBaseNotifySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbLoadBalancingNotifySink, windows_core::IUnknown, ITsSbBaseNotifySink);
impl ITsSbLoadBalancingNotifySink {
    pub unsafe fn OnGetMostSuitableTarget<P0, P1>(&self, plbresult: P0, fisnewconnection: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbLoadBalanceResult>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnGetMostSuitableTarget)(windows_core::Interface::as_raw(self), plbresult.param().abi(), fisnewconnection.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbLoadBalancingNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnGetMostSuitableTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).PrepareTargetForConnect)(windows_core::Interface::as_raw(self), pconnection.param().abi(), porchestrationnotifysink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbOrchestration_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub PrepareTargetForConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).OnReadyToConnect)(windows_core::Interface::as_raw(self), ptarget.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbOrchestrationNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnReadyToConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).QueryEnvironmentForTarget)(windows_core::Interface::as_raw(self), pconnection.param().abi(), pplacementsink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbPlacement_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub QueryEnvironmentForTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).OnQueryEnvironmentCompleted)(windows_core::Interface::as_raw(self), penvironment.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbPlacementNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnQueryEnvironmentCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbPlugin, ITsSbPlugin_Vtbl, 0x48cd7406_caab_465f_a5d6_baa863b9ea4f);
impl core::ops::Deref for ITsSbPlugin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbPlugin, windows_core::IUnknown);
impl ITsSbPlugin {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbProvider>,
        P1: windows_core::Param<ITsSbPluginNotifySink>,
        P2: windows_core::Param<ITsSbPluginPropertySet>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pprovider.param().abi(), pnotifysink.param().abi(), ppropertyset.param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), hr).ok()
    }
}
#[repr(C)]
pub struct ITsSbPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).OnInitialized)(windows_core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn OnTerminated(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTerminated)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITsSbPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    pub OnInitialized: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnTerminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
impl ITsSbPluginPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbPluginPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
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
impl ITsSbPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbPropertySet_Vtbl {
    pub base__: super::Com::StructuredStorage::IPropertyBag_Vtbl,
}
windows_core::imp::define_interface!(ITsSbProvider, ITsSbProvider_Vtbl, 0x87a4098f_6d7b_44dd_bc17_8ce44e370d52);
impl core::ops::Deref for ITsSbProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbProvider, windows_core::IUnknown);
impl ITsSbProvider {
    pub unsafe fn CreateTargetObject<P0, P1>(&self, targetname: P0, environmentname: P1) -> windows_core::Result<ITsSbTarget>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTargetObject)(windows_core::Interface::as_raw(self), targetname.param().abi(), environmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateLoadBalanceResultObject<P0>(&self, targetname: P0) -> windows_core::Result<ITsSbLoadBalanceResult>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateLoadBalanceResultObject)(windows_core::Interface::as_raw(self), targetname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateSessionObject<P0, P1, P2>(&self, targetname: P0, username: P1, domain: P2, sessionid: u32) -> windows_core::Result<ITsSbSession>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateSessionObject)(windows_core::Interface::as_raw(self), targetname.param().abi(), username.param().abi(), domain.param().abi(), sessionid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreatePluginPropertySet(&self) -> windows_core::Result<ITsSbPluginPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreatePluginPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateTargetPropertySetObject(&self) -> windows_core::Result<ITsSbTargetPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTargetPropertySetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEnvironmentObject<P0>(&self, name: P0, serverweight: u32) -> windows_core::Result<ITsSbEnvironment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEnvironmentObject)(windows_core::Interface::as_raw(self), name.param().abi(), serverweight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResourcePluginStore(&self) -> windows_core::Result<ITsSbResourcePluginStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourcePluginStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFilterPluginStore(&self) -> windows_core::Result<ITsSbFilterPluginStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFilterPluginStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterForNotification<P0, P1>(&self, notificationtype: u32, resourcetomonitor: P0, ppluginnotification: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<ITsSbResourceNotification>,
    {
        (windows_core::Interface::vtable(self).RegisterForNotification)(windows_core::Interface::as_raw(self), notificationtype, resourcetomonitor.param().abi(), ppluginnotification.param().abi()).ok()
    }
    pub unsafe fn UnRegisterForNotification<P0>(&self, notificationtype: u32, resourcetomonitor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UnRegisterForNotification)(windows_core::Interface::as_raw(self), notificationtype, resourcetomonitor.param().abi()).ok()
    }
    pub unsafe fn GetInstanceOfGlobalStore(&self) -> windows_core::Result<ITsSbGlobalStore> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstanceOfGlobalStore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateEnvironmentPropertySetObject(&self) -> windows_core::Result<ITsSbEnvironmentPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateEnvironmentPropertySetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITsSbProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTargetObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateLoadBalanceResultObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSessionObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreatePluginPropertySet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreatePluginPropertySet: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateTargetPropertySetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateTargetPropertySetObject: usize,
    pub CreateEnvironmentObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourcePluginStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilterPluginStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterForNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetInstanceOfGlobalStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEnvironmentPropertySetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEnvironmentPropertySetObject: usize,
}
windows_core::imp::define_interface!(ITsSbProvisioning, ITsSbProvisioning_Vtbl, 0x2f6f0dbb_9e4f_462b_9c3f_fccc3dcb6232);
impl core::ops::Deref for ITsSbProvisioning {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbProvisioning, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbProvisioning {
    pub unsafe fn CreateVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        (windows_core::Interface::vtable(self).CreateVirtualMachines)(windows_core::Interface::as_raw(self), jobxmlstring.param().abi(), jobguid.param().abi(), psink.param().abi()).ok()
    }
    pub unsafe fn PatchVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2, pvmpatchinfo: *const VM_PATCH_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        (windows_core::Interface::vtable(self).PatchVirtualMachines)(windows_core::Interface::as_raw(self), jobxmlstring.param().abi(), jobguid.param().abi(), psink.param().abi(), pvmpatchinfo).ok()
    }
    pub unsafe fn DeleteVirtualMachines<P0, P1, P2>(&self, jobxmlstring: P0, jobguid: P1, psink: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<ITsSbProvisioningPluginNotifySink>,
    {
        (windows_core::Interface::vtable(self).DeleteVirtualMachines)(windows_core::Interface::as_raw(self), jobxmlstring.param().abi(), jobguid.param().abi(), psink.param().abi()).ok()
    }
    pub unsafe fn CancelJob<P0>(&self, jobguid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).CancelJob)(windows_core::Interface::as_raw(self), jobguid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbProvisioning_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub CreateVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PatchVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, *const VM_PATCH_INFO) -> windows_core::HRESULT,
    pub DeleteVirtualMachines: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelJob: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbProvisioningPluginNotifySink, ITsSbProvisioningPluginNotifySink_Vtbl, 0xaca87a8e_818b_4581_a032_49c3dfb9c701);
impl core::ops::Deref for ITsSbProvisioningPluginNotifySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbProvisioningPluginNotifySink, windows_core::IUnknown);
impl ITsSbProvisioningPluginNotifySink {
    pub unsafe fn OnJobCreated(&self, pvmnotifyinfo: *const VM_NOTIFY_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnJobCreated)(windows_core::Interface::as_raw(self), pvmnotifyinfo).ok()
    }
    pub unsafe fn OnVirtualMachineStatusChanged<P0>(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY, vmnotifystatus: VM_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnVirtualMachineStatusChanged)(windows_core::Interface::as_raw(self), pvmnotifyentry, vmnotifystatus, errorcode, errordescr.param().abi()).ok()
    }
    pub unsafe fn OnJobCompleted<P0>(&self, resultcode: windows_core::HRESULT, resultdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnJobCompleted)(windows_core::Interface::as_raw(self), resultcode, resultdescription.param().abi()).ok()
    }
    pub unsafe fn OnJobCancelled(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnJobCancelled)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockVirtualMachine(&self, pvmnotifyentry: *const VM_NOTIFY_ENTRY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockVirtualMachine)(windows_core::Interface::as_raw(self), pvmnotifyentry).ok()
    }
    pub unsafe fn OnVirtualMachineHostStatusChanged<P0, P1>(&self, vmhost: P0, vmhostnotifystatus: VM_HOST_NOTIFY_STATUS, errorcode: windows_core::HRESULT, errordescr: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnVirtualMachineHostStatusChanged)(windows_core::Interface::as_raw(self), vmhost.param().abi(), vmhostnotifystatus, errorcode, errordescr.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbProvisioningPluginNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnJobCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_INFO) -> windows_core::HRESULT,
    pub OnVirtualMachineStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_ENTRY, VM_NOTIFY_STATUS, windows_core::HRESULT, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OnJobCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OnJobCancelled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockVirtualMachine: unsafe extern "system" fn(*mut core::ffi::c_void, *const VM_NOTIFY_ENTRY) -> windows_core::HRESULT,
    pub OnVirtualMachineHostStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, VM_HOST_NOTIFY_STATUS, windows_core::HRESULT, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbResourceNotification, ITsSbResourceNotification_Vtbl, 0x65d3e85a_c39b_11dc_b92d_3cd255d89593);
impl core::ops::Deref for ITsSbResourceNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbResourceNotification, windows_core::IUnknown);
impl ITsSbResourceNotification {
    pub unsafe fn NotifySessionChange<P0>(&self, changetype: TSSESSION_STATE, psession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        (windows_core::Interface::vtable(self).NotifySessionChange)(windows_core::Interface::as_raw(self), changetype, psession.param().abi()).ok()
    }
    pub unsafe fn NotifyTargetChange<P0>(&self, targetchangetype: u32, ptarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        (windows_core::Interface::vtable(self).NotifyTargetChange)(windows_core::Interface::as_raw(self), targetchangetype, ptarget.param().abi()).ok()
    }
    pub unsafe fn NotifyClientConnectionStateChange<P0>(&self, changetype: CONNECTION_CHANGE_NOTIFICATION, pconnection: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbClientConnection>,
    {
        (windows_core::Interface::vtable(self).NotifyClientConnectionStateChange)(windows_core::Interface::as_raw(self), changetype, pconnection.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbResourceNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifySessionChange: unsafe extern "system" fn(*mut core::ffi::c_void, TSSESSION_STATE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyTargetChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyClientConnectionStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, CONNECTION_CHANGE_NOTIFICATION, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbResourceNotificationEx, ITsSbResourceNotificationEx_Vtbl, 0xa8a47fde_ca91_44d2_b897_3aa28a43b2b7);
impl core::ops::Deref for ITsSbResourceNotificationEx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbResourceNotificationEx, windows_core::IUnknown);
impl ITsSbResourceNotificationEx {
    pub unsafe fn NotifySessionChangeEx<P0, P1, P2>(&self, targetname: P0, username: P1, domain: P2, sessionid: u32, sessionstate: TSSESSION_STATE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NotifySessionChangeEx)(windows_core::Interface::as_raw(self), targetname.param().abi(), username.param().abi(), domain.param().abi(), sessionid, sessionstate).ok()
    }
    pub unsafe fn NotifyTargetChangeEx<P0>(&self, targetname: P0, targetchangetype: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NotifyTargetChangeEx)(windows_core::Interface::as_raw(self), targetname.param().abi(), targetchangetype).ok()
    }
    pub unsafe fn NotifyClientConnectionStateChangeEx<P0, P1, P2, P3, P4>(&self, username: P0, domain: P1, initialprogram: P2, poolname: P3, targetname: P4, connectionchangetype: CONNECTION_CHANGE_NOTIFICATION) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).NotifyClientConnectionStateChangeEx)(windows_core::Interface::as_raw(self), username.param().abi(), domain.param().abi(), initialprogram.param().abi(), poolname.param().abi(), targetname.param().abi(), connectionchangetype).ok()
    }
}
#[repr(C)]
pub struct ITsSbResourceNotificationEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifySessionChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, TSSESSION_STATE) -> windows_core::HRESULT,
    pub NotifyTargetChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub NotifyClientConnectionStateChangeEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, CONNECTION_CHANGE_NOTIFICATION) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbResourcePlugin, ITsSbResourcePlugin_Vtbl, 0xea8db42c_98ed_4535_a88b_2a164f35490f);
impl core::ops::Deref for ITsSbResourcePlugin {
    type Target = ITsSbPlugin;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbResourcePlugin, windows_core::IUnknown, ITsSbPlugin);
impl ITsSbResourcePlugin {}
#[repr(C)]
pub struct ITsSbResourcePlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
}
windows_core::imp::define_interface!(ITsSbResourcePluginStore, ITsSbResourcePluginStore_Vtbl, 0x5c38f65f_bcf1_4036_a6bf_9e3cccae0b63);
impl core::ops::Deref for ITsSbResourcePluginStore {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbResourcePluginStore, windows_core::IUnknown);
impl ITsSbResourcePluginStore {
    pub unsafe fn QueryTarget<P0, P1>(&self, targetname: P0, farmname: P1) -> windows_core::Result<ITsSbTarget>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryTarget)(windows_core::Interface::as_raw(self), targetname.param().abi(), farmname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QuerySessionBySessionId<P0>(&self, dwsessionid: u32, targetname: P0) -> windows_core::Result<ITsSbSession>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QuerySessionBySessionId)(windows_core::Interface::as_raw(self), dwsessionid, targetname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddTargetToStore<P0>(&self, ptarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
    {
        (windows_core::Interface::vtable(self).AddTargetToStore)(windows_core::Interface::as_raw(self), ptarget.param().abi()).ok()
    }
    pub unsafe fn AddSessionToStore<P0>(&self, psession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        (windows_core::Interface::vtable(self).AddSessionToStore)(windows_core::Interface::as_raw(self), psession.param().abi()).ok()
    }
    pub unsafe fn AddEnvironmentToStore<P0>(&self, penvironment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
    {
        (windows_core::Interface::vtable(self).AddEnvironmentToStore)(windows_core::Interface::as_raw(self), penvironment.param().abi()).ok()
    }
    pub unsafe fn RemoveEnvironmentFromStore<P0, P1>(&self, environmentname: P0, bignoreowner: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).RemoveEnvironmentFromStore)(windows_core::Interface::as_raw(self), environmentname.param().abi(), bignoreowner.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateFarms(&self, pdwcount: *mut u32, pval: *mut *mut super::Com::SAFEARRAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateFarms)(windows_core::Interface::as_raw(self), pdwcount, pval).ok()
    }
    pub unsafe fn QueryEnvironment<P0>(&self, environmentname: P0) -> windows_core::Result<ITsSbEnvironment>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryEnvironment)(windows_core::Interface::as_raw(self), environmentname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumerateEnvironments(&self, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbEnvironment>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumerateEnvironments)(windows_core::Interface::as_raw(self), pdwcount, pval).ok()
    }
    pub unsafe fn SaveTarget<P0, P1>(&self, ptarget: P0, bforcewrite: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SaveTarget)(windows_core::Interface::as_raw(self), ptarget.param().abi(), bforcewrite.param().abi()).ok()
    }
    pub unsafe fn SaveEnvironment<P0, P1>(&self, penvironment: P0, bforcewrite: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SaveEnvironment)(windows_core::Interface::as_raw(self), penvironment.param().abi(), bforcewrite.param().abi()).ok()
    }
    pub unsafe fn SaveSession<P0>(&self, psession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        (windows_core::Interface::vtable(self).SaveSession)(windows_core::Interface::as_raw(self), psession.param().abi()).ok()
    }
    pub unsafe fn SetTargetProperty<P0, P1>(&self, targetname: P0, propertyname: P1, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetProperty)(windows_core::Interface::as_raw(self), targetname.param().abi(), propertyname.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn SetEnvironmentProperty<P0, P1>(&self, environmentname: P0, propertyname: P1, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEnvironmentProperty)(windows_core::Interface::as_raw(self), environmentname.param().abi(), propertyname.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn SetTargetState<P0>(&self, targetname: P0, newstate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetTargetState)(windows_core::Interface::as_raw(self), targetname.param().abi(), newstate, &mut result__).map(|| result__)
    }
    pub unsafe fn SetSessionState<P0>(&self, sbsession: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbSession>,
    {
        (windows_core::Interface::vtable(self).SetSessionState)(windows_core::Interface::as_raw(self), sbsession.param().abi()).ok()
    }
    pub unsafe fn EnumerateTargets<P0, P1, P2>(&self, farmname: P0, envname: P1, sortbyfieldid: TS_SB_SORT_BY, sortybypropname: P2, pdwcount: *mut u32, pval: *mut *mut Option<ITsSbTarget>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateTargets)(windows_core::Interface::as_raw(self), farmname.param().abi(), envname.param().abi(), sortbyfieldid, sortybypropname.param().abi(), pdwcount, pval).ok()
    }
    pub unsafe fn EnumerateSessions<P0, P1, P2, P3, P4>(&self, targetname: P0, username: P1, userdomain: P2, poolname: P3, initialprogram: P4, psessionstate: *const TSSESSION_STATE, pdwcount: *mut u32, ppval: *mut *mut Option<ITsSbSession>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).EnumerateSessions)(windows_core::Interface::as_raw(self), targetname.param().abi(), username.param().abi(), userdomain.param().abi(), poolname.param().abi(), initialprogram.param().abi(), psessionstate, pdwcount, ppval).ok()
    }
    pub unsafe fn GetFarmProperty<P0, P1>(&self, farmname: P0, propertyname: P1, pvarvalue: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).GetFarmProperty)(windows_core::Interface::as_raw(self), farmname.param().abi(), propertyname.param().abi(), core::mem::transmute(pvarvalue)).ok()
    }
    pub unsafe fn DeleteTarget<P0, P1>(&self, targetname: P0, hostname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteTarget)(windows_core::Interface::as_raw(self), targetname.param().abi(), hostname.param().abi()).ok()
    }
    pub unsafe fn SetTargetPropertyWithVersionCheck<P0, P1>(&self, ptarget: P0, propertyname: P1, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTarget>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetPropertyWithVersionCheck)(windows_core::Interface::as_raw(self), ptarget.param().abi(), propertyname.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn SetEnvironmentPropertyWithVersionCheck<P0, P1>(&self, penvironment: P0, propertyname: P1, pproperty: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbEnvironment>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEnvironmentPropertyWithVersionCheck)(windows_core::Interface::as_raw(self), penvironment.param().abi(), propertyname.param().abi(), core::mem::transmute(pproperty)).ok()
    }
    pub unsafe fn AcquireTargetLock<P0>(&self, targetname: P0, dwtimeout: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AcquireTargetLock)(windows_core::Interface::as_raw(self), targetname.param().abi(), dwtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ReleaseTargetLock<P0>(&self, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ReleaseTargetLock)(windows_core::Interface::as_raw(self), pcontext.param().abi()).ok()
    }
    pub unsafe fn TestAndSetServerState<P0, P1>(&self, poolname: P0, serverfqdn: P1, newstate: TARGET_STATE, teststate: TARGET_STATE) -> windows_core::Result<TARGET_STATE>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TestAndSetServerState)(windows_core::Interface::as_raw(self), poolname.param().abi(), serverfqdn.param().abi(), newstate, teststate, &mut result__).map(|| result__)
    }
    pub unsafe fn SetServerWaitingToStart<P0, P1>(&self, poolname: P0, servername: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServerWaitingToStart)(windows_core::Interface::as_raw(self), poolname.param().abi(), servername.param().abi()).ok()
    }
    pub unsafe fn GetServerState<P0, P1>(&self, poolname: P0, serverfqdn: P1) -> windows_core::Result<TARGET_STATE>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetServerState)(windows_core::Interface::as_raw(self), poolname.param().abi(), serverfqdn.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetServerDrainMode<P0>(&self, serverfqdn: P0, drainmode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServerDrainMode)(windows_core::Interface::as_raw(self), serverfqdn.param().abi(), drainmode).ok()
    }
}
#[repr(C)]
pub struct ITsSbResourcePluginStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QuerySessionBySessionId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTargetToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSessionToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEnvironmentToStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEnvironmentFromStore: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateFarms: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateFarms: usize,
    pub QueryEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateEnvironments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Option<ITsSbEnvironment>) -> windows_core::HRESULT,
    pub SaveTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SaveEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SaveSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTargetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetEnvironmentProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetTargetState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, TARGET_STATE, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetSessionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumerateTargets: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, TS_SB_SORT_BY, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut *mut Option<ITsSbTarget>) -> windows_core::HRESULT,
    pub EnumerateSessions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const TSSESSION_STATE, *mut u32, *mut *mut Option<ITsSbSession>) -> windows_core::HRESULT,
    pub GetFarmProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeleteTarget: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetPropertyWithVersionCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetEnvironmentPropertyWithVersionCheck: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AcquireTargetLock: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseTargetLock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TestAndSetServerState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, TARGET_STATE, TARGET_STATE, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetServerWaitingToStart: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetServerState: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut TARGET_STATE) -> windows_core::HRESULT,
    pub SetServerDrainMode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbServiceNotification, ITsSbServiceNotification_Vtbl, 0x86cb68ae_86e0_4f57_8a64_bb7406bc5550);
impl core::ops::Deref for ITsSbServiceNotification {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbServiceNotification, windows_core::IUnknown);
impl ITsSbServiceNotification {
    pub unsafe fn NotifyServiceFailure(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyServiceFailure)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyServiceSuccess(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyServiceSuccess)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ITsSbServiceNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyServiceFailure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyServiceSuccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbSession, ITsSbSession_Vtbl, 0xd453aac7_b1d8_4c5e_ba34_9afb4c8c5510);
impl core::ops::Deref for ITsSbSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbSession, windows_core::IUnknown);
impl ITsSbSession {
    pub unsafe fn SessionId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetName<P0>(&self, targetname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetName)(windows_core::Interface::as_raw(self), targetname.param().abi()).ok()
    }
    pub unsafe fn Username(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Username)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Domain(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn State(&self) -> windows_core::Result<TSSESSION_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetState(&self, state: TSSESSION_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn CreateTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCreateTime(&self, time: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCreateTime)(windows_core::Interface::as_raw(self), core::mem::transmute(time)).ok()
    }
    pub unsafe fn DisconnectTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisconnectTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDisconnectTime(&self, time: super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisconnectTime)(windows_core::Interface::as_raw(self), core::mem::transmute(time)).ok()
    }
    pub unsafe fn InitialProgram(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InitialProgram)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInitialProgram<P0>(&self, application: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInitialProgram)(windows_core::Interface::as_raw(self), application.param().abi()).ok()
    }
    pub unsafe fn ClientDisplay(&self) -> windows_core::Result<CLIENT_DISPLAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientDisplay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetClientDisplay(&self, pclientdisplay: CLIENT_DISPLAY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetClientDisplay)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientdisplay)).ok()
    }
    pub unsafe fn ProtocolType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProtocolType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetProtocolType(&self, val: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetProtocolType)(windows_core::Interface::as_raw(self), val).ok()
    }
}
#[repr(C)]
pub struct ITsSbSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Username: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TSSESSION_STATE) -> windows_core::HRESULT,
    pub SetState: unsafe extern "system" fn(*mut core::ffi::c_void, TSSESSION_STATE) -> windows_core::HRESULT,
    pub CreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetCreateTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub DisconnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub SetDisconnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub InitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInitialProgram: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ClientDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CLIENT_DISPLAY) -> windows_core::HRESULT,
    pub SetClientDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, CLIENT_DISPLAY) -> windows_core::HRESULT,
    pub ProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITsSbTarget, ITsSbTarget_Vtbl, 0x16616ecc_272d_411d_b324_126893033856);
impl core::ops::Deref for ITsSbTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbTarget, windows_core::IUnknown);
impl ITsSbTarget {
    pub unsafe fn TargetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetName<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetName)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn FarmName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FarmName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFarmName<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFarmName)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn TargetFQDN(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetFQDN)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetFQDN<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetFQDN)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn TargetNetbios(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetNetbios)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTargetNetbios<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTargetNetbios)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn get_IpAddresses(&self, sockaddr: *mut TSSD_ConnectionPoint, numaddresses: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).get_IpAddresses)(windows_core::Interface::as_raw(self), sockaddr, numaddresses).ok()
    }
    pub unsafe fn put_IpAddresses(&self, sockaddr: &[TSSD_ConnectionPoint]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).put_IpAddresses)(windows_core::Interface::as_raw(self), core::mem::transmute(sockaddr.as_ptr()), sockaddr.len().try_into().unwrap()).ok()
    }
    pub unsafe fn TargetState(&self) -> windows_core::Result<TARGET_STATE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTargetState(&self, state: TARGET_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTargetState)(windows_core::Interface::as_raw(self), state).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn TargetPropertySet(&self) -> windows_core::Result<ITsSbTargetPropertySet> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetPropertySet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn SetTargetPropertySet<P0>(&self, pval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITsSbTargetPropertySet>,
    {
        (windows_core::Interface::vtable(self).SetTargetPropertySet)(windows_core::Interface::as_raw(self), pval.param().abi()).ok()
    }
    pub unsafe fn EnvironmentName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnvironmentName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEnvironmentName<P0>(&self, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEnvironmentName)(windows_core::Interface::as_raw(self), val.param().abi()).ok()
    }
    pub unsafe fn NumSessions(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumSessions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn NumPendingConnections(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumPendingConnections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TargetLoad(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetLoad)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITsSbTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FarmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFarmName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TargetFQDN: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetFQDN: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TargetNetbios: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTargetNetbios: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
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
    pub EnvironmentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEnvironmentName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NumSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NumPendingConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub TargetLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
impl ITsSbTargetPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[repr(C)]
pub struct ITsSbTargetPropertySet_Vtbl {
    pub base__: ITsSbPropertySet_Vtbl,
}
windows_core::imp::define_interface!(ITsSbTaskInfo, ITsSbTaskInfo_Vtbl, 0x523d1083_89be_48dd_99ea_04e82ffa7265);
impl core::ops::Deref for ITsSbTaskInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITsSbTaskInfo, windows_core::IUnknown);
impl ITsSbTaskInfo {
    pub unsafe fn TargetId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TargetId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EndTime(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Deadline(&self) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Identifier(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Identifier)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Label(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Context(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Plugin(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Plugin)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Status(&self) -> windows_core::Result<RDV_TASK_STATUS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITsSbTaskInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TargetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub Identifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Context: usize,
    pub Plugin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RDV_TASK_STATUS) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).InitializeTaskPlugin)(windows_core::Interface::as_raw(self), pitssbtaskpluginnotifysink.param().abi()).ok()
    }
    pub unsafe fn SetTaskQueue<P0>(&self, pszhostname: P0, pitssbtaskinfo: &[Option<ITsSbTaskInfo>]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTaskQueue)(windows_core::Interface::as_raw(self), pszhostname.param().abi(), pitssbtaskinfo.len().try_into().unwrap(), core::mem::transmute(pitssbtaskinfo.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ITsSbTaskPlugin_Vtbl {
    pub base__: ITsSbPlugin_Vtbl,
    pub InitializeTaskPlugin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskQueue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
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
    pub unsafe fn OnSetTaskTime<P0, P1, P2, P3>(&self, sztargetname: P0, taskstarttime: super::super::Foundation::FILETIME, taskendtime: super::super::Foundation::FILETIME, taskdeadline: super::super::Foundation::FILETIME, sztasklabel: P1, sztaskidentifier: P2, sztaskplugin: P3, dwtaskstatus: u32, sacontext: *const super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnSetTaskTime)(windows_core::Interface::as_raw(self), sztargetname.param().abi(), core::mem::transmute(taskstarttime), core::mem::transmute(taskendtime), core::mem::transmute(taskdeadline), sztasklabel.param().abi(), sztaskidentifier.param().abi(), sztaskplugin.param().abi(), dwtaskstatus, sacontext).ok()
    }
    pub unsafe fn OnDeleteTaskTime<P0, P1>(&self, sztargetname: P0, sztaskidentifier: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnDeleteTaskTime)(windows_core::Interface::as_raw(self), sztargetname.param().abi(), sztaskidentifier.param().abi()).ok()
    }
    pub unsafe fn OnUpdateTaskStatus<P0, P1>(&self, sztargetname: P0, taskidentifier: P1, taskstatus: RDV_TASK_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnUpdateTaskStatus)(windows_core::Interface::as_raw(self), sztargetname.param().abi(), taskidentifier.param().abi(), taskstatus).ok()
    }
    pub unsafe fn OnReportTasks<P0>(&self, szhostname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnReportTasks)(windows_core::Interface::as_raw(self), szhostname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITsSbTaskPluginNotifySink_Vtbl {
    pub base__: ITsSbBaseNotifySink_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSetTaskTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::FILETIME, super::super::Foundation::FILETIME, super::super::Foundation::FILETIME, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSetTaskTime: usize,
    pub OnDeleteTaskTime: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OnUpdateTaskStatus: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, RDV_TASK_STATUS) -> windows_core::HRESULT,
    pub OnReportTasks: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsEnhancedFastReconnectArbitrator, IWRdsEnhancedFastReconnectArbitrator_Vtbl, 0x5718ae9b_47f2_499f_b634_d8175bd51131);
impl core::ops::Deref for IWRdsEnhancedFastReconnectArbitrator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsEnhancedFastReconnectArbitrator, windows_core::IUnknown);
impl IWRdsEnhancedFastReconnectArbitrator {
    pub unsafe fn GetSessionForEnhancedFastReconnect(&self, psessionidarray: *const i32, dwsessioncount: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSessionForEnhancedFastReconnect)(windows_core::Interface::as_raw(self), psessionidarray, dwsessioncount, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWRdsEnhancedFastReconnectArbitrator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSessionForEnhancedFastReconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsGraphicsChannel, IWRdsGraphicsChannel_Vtbl, 0x684b7a0b_edff_43ad_d5a2_4a8d5388f401);
impl core::ops::Deref for IWRdsGraphicsChannel {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannel, windows_core::IUnknown);
impl IWRdsGraphicsChannel {
    pub unsafe fn Write<P0>(&self, cbsize: u32, pbuffer: *const u8, pcontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), cbsize, pbuffer, pcontext.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0, P1>(&self, pchannelevents: P0, popencontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsGraphicsChannelEvents>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), pchannelevents.param().abi(), popencontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWRdsGraphicsChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsGraphicsChannelEvents, IWRdsGraphicsChannelEvents_Vtbl, 0x67f2368c_d674_4fae_66a5_d20628a640d2);
impl core::ops::Deref for IWRdsGraphicsChannelEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelEvents, windows_core::IUnknown);
impl IWRdsGraphicsChannelEvents {
    pub unsafe fn OnDataReceived(&self, cbsize: u32, pbuffer: *const u8) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDataReceived)(windows_core::Interface::as_raw(self), cbsize, pbuffer).ok()
    }
    pub unsafe fn OnClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnChannelOpened<P0>(&self, openresult: windows_core::HRESULT, popencontext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChannelOpened)(windows_core::Interface::as_raw(self), openresult, popencontext.param().abi()).ok()
    }
    pub unsafe fn OnDataSent<P0, P1>(&self, pwritecontext: P0, bcancelled: P1, pbuffer: *const u8, cbbuffer: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnDataSent)(windows_core::Interface::as_raw(self), pwritecontext.param().abi(), bcancelled.param().abi(), pbuffer, cbbuffer).ok()
    }
    pub unsafe fn OnMetricsUpdate(&self, bandwidth: u32, rtt: u32, lastsentbyteindex: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMetricsUpdate)(windows_core::Interface::as_raw(self), bandwidth, rtt, lastsentbyteindex).ok()
    }
}
#[repr(C)]
pub struct IWRdsGraphicsChannelEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnChannelOpened: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDataSent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *const u8, u32) -> windows_core::HRESULT,
    pub OnMetricsUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsGraphicsChannelManager, IWRdsGraphicsChannelManager_Vtbl, 0x0fd57159_e83e_476a_a8b9_4a7976e71e18);
impl core::ops::Deref for IWRdsGraphicsChannelManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsGraphicsChannelManager, windows_core::IUnknown);
impl IWRdsGraphicsChannelManager {
    pub unsafe fn CreateChannel(&self, pszchannelname: *const u8, channeltype: WRdsGraphicsChannelType) -> windows_core::Result<IWRdsGraphicsChannel> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateChannel)(windows_core::Interface::as_raw(self), pszchannelname, channeltype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWRdsGraphicsChannelManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateChannel: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, WRdsGraphicsChannelType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolConnection, IWRdsProtocolConnection_Vtbl, 0x324ed94f_fdaf_4ff6_81a8_42abe755830b);
impl core::ops::Deref for IWRdsProtocolConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnection, windows_core::IUnknown);
impl IWRdsProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWRdsProtocolLogonErrorRedirector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLogonErrorRedirector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AcceptConnection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcceptConnection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClientData)(windows_core::Interface::as_raw(self), pclientdata).ok()
    }
    pub unsafe fn GetClientMonitorData(&self, pnummonitors: *mut u32, pprimarymonitor: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClientMonitorData)(windows_core::Interface::as_raw(self), pnummonitors, pprimarymonitor).ok()
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserCredentials)(windows_core::Interface::as_raw(self), pusercreds).ok()
    }
    pub unsafe fn GetLicenseConnection(&self) -> windows_core::Result<IWRdsProtocolLicenseConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLicenseConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AuthenticateClientToSession)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionId<P0>(&self, sessionid: *const WTS_SESSION_ID, sessionhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
    {
        (windows_core::Interface::vtable(self).NotifySessionId)(windows_core::Interface::as_raw(self), sessionid, sessionhandle.param().abi()).ok()
    }
    pub unsafe fn GetInputHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInputHandles)(windows_core::Interface::as_raw(self), pkeyboardhandle, pmousehandle, pbeephandle).ok()
    }
    pub unsafe fn GetVideoHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVideoHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectNotify)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn IsUserAllowedToLogon<P0, P1, P2>(&self, sessionid: u32, usertoken: P0, pdomainname: P1, pusername: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsUserAllowedToLogon)(windows_core::Interface::as_raw(self), sessionid, usertoken.param().abi(), pdomainname.param().abi(), pusername.param().abi()).ok()
    }
    pub unsafe fn SessionArbitrationEnumeration<P0, P1>(&self, husertoken: P0, bsinglesessionperuserenabled: P1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(windows_core::Interface::as_raw(self), husertoken.param().abi(), bsinglesessionperuserenabled.param().abi(), psessionidarray, pdwsessionidentifiercount).ok()
    }
    pub unsafe fn LogonNotify<P0, P1, P2>(&self, hclienttoken: P0, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LogonNotify)(windows_core::Interface::as_raw(self), hclienttoken.param().abi(), wszusername.param().abi(), wszdomainname.param().abi(), sessionid, pwrdsconnectionsettings).ok()
    }
    pub unsafe fn PreDisconnect(&self, disconnectreason: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreDisconnect)(windows_core::Interface::as_raw(self), disconnectreason).ok()
    }
    pub unsafe fn DisconnectNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectNotify)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProtocolStatus)(windows_core::Interface::as_raw(self), pprotocolstatus).ok()
    }
    pub unsafe fn GetLastInputTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastInputTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetErrorInfo)(windows_core::Interface::as_raw(self), ulerror).ok()
    }
    pub unsafe fn CreateVirtualChannel<P0, P1>(&self, szendpointname: P0, bstatic: P1, requestedpriority: u32) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), szendpointname.param().abi(), bstatic.param().abi(), requestedpriority, &mut result__).map(|| result__)
    }
    pub unsafe fn QueryProperty(&self, querytype: windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(querytype), ppropertyentriesin.len().try_into().unwrap(), ppropertyentriesout.len().try_into().unwrap(), core::mem::transmute(ppropertyentriesin.as_ptr()), core::mem::transmute(ppropertyentriesout.as_ptr())).ok()
    }
    pub unsafe fn GetShadowConnection(&self) -> windows_core::Result<IWRdsProtocolShadowConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetShadowConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NotifyCommandProcessCreated(&self, sessionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyCommandProcessCreated)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
}
#[repr(C)]
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
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, super::super::Foundation::BOOL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub LogonNotify: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTS_SESSION_ID, *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT,
    pub PreDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, super::super::Foundation::BOOL, u32, *mut usize) -> windows_core::HRESULT,
    pub QueryProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *const WTS_PROPERTY_VALUE, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyCommandProcessCreated: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolConnectionCallback, IWRdsProtocolConnectionCallback_Vtbl, 0xf1d70332_d070_4ef1_a088_78313536c2d6);
impl core::ops::Deref for IWRdsProtocolConnectionCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionCallback, windows_core::IUnknown);
impl IWRdsProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnReady)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BrokenConnection)(windows_core::Interface::as_raw(self), reason, source).ok()
    }
    pub unsafe fn StopScreenUpdates(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopScreenUpdates)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RedrawWindow)(windows_core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn GetConnectionId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWRdsProtocolConnectionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SMALL_RECT) -> windows_core::HRESULT,
    pub GetConnectionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolConnectionSettings, IWRdsProtocolConnectionSettings_Vtbl, 0x83fcf5d3_f6f4_ea94_9cd2_32f280e1e510);
impl core::ops::Deref for IWRdsProtocolConnectionSettings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolConnectionSettings, windows_core::IUnknown);
impl IWRdsProtocolConnectionSettings {
    pub unsafe fn SetConnectionSetting(&self, propertyid: windows_core::GUID, ppropertyentriesin: *const WTS_PROPERTY_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetConnectionSetting)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyid), ppropertyentriesin).ok()
    }
    pub unsafe fn GetConnectionSetting(&self, propertyid: windows_core::GUID, ppropertyentriesout: *mut WTS_PROPERTY_VALUE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConnectionSetting)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyid), ppropertyentriesout).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolConnectionSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConnectionSetting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *const WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetConnectionSetting: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolLicenseConnection, IWRdsProtocolLicenseConnection_Vtbl, 0x1d6a145f_d095_4424_957a_407fae822d84);
impl core::ops::Deref for IWRdsProtocolLicenseConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolLicenseConnection, windows_core::IUnknown);
impl IWRdsProtocolLicenseConnection {
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestLicensingCapabilities)(windows_core::Interface::as_raw(self), pplicensecapabilities, pcblicensecapabilities).ok()
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(reserve1.as_ptr()), reserve1.len().try_into().unwrap(), ppclientlicense, pcbclientlicense).ok()
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProtocolComplete)(windows_core::Interface::as_raw(self), ulcomplete).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolLicenseConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestLicensingCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_LICENSE_CAPABILITIES, *mut u32) -> windows_core::HRESULT,
    pub SendClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolListener, IWRdsProtocolListener_Vtbl, 0xfcbc131b_c686_451d_a773_e279e230f540);
impl core::ops::Deref for IWRdsProtocolListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolListener, windows_core::IUnknown);
impl IWRdsProtocolListener {
    pub unsafe fn GetSettings(&self, wrdslistenersettinglevel: WRDS_LISTENER_SETTING_LEVEL) -> windows_core::Result<WRDS_LISTENER_SETTINGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), wrdslistenersettinglevel, &mut result__).map(|| result__)
    }
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsProtocolListenerCallback>,
    {
        (windows_core::Interface::vtable(self).StartListen)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn StopListen(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopListen)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, WRDS_LISTENER_SETTING_LEVEL, *mut WRDS_LISTENER_SETTINGS) -> windows_core::HRESULT,
    pub StartListen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolListenerCallback, IWRdsProtocolListenerCallback_Vtbl, 0x3ab27e5b_4449_4dc1_b74a_91621d4fe984);
impl core::ops::Deref for IWRdsProtocolListenerCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolListenerCallback, windows_core::IUnknown);
impl IWRdsProtocolListenerCallback {
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0, pwrdsconnectionsettings: *const WRDS_CONNECTION_SETTINGS) -> windows_core::Result<IWRdsProtocolConnectionCallback>
    where
        P0: windows_core::Param<IWRdsProtocolConnection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), pconnection.param().abi(), pwrdsconnectionsettings, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWRdsProtocolListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WRDS_CONNECTION_SETTINGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolLogonErrorRedirector, IWRdsProtocolLogonErrorRedirector_Vtbl, 0x519fe83b_142a_4120_a3d5_a405d315281a);
impl core::ops::Deref for IWRdsProtocolLogonErrorRedirector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolLogonErrorRedirector, windows_core::IUnknown);
impl IWRdsProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBeginPainting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectStatus)(windows_core::Interface::as_raw(self), pszmessage.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectMessage)(windows_core::Interface::as_raw(self), pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
    }
    pub unsafe fn RedirectLogonError<P0, P1>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectLogonError)(windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWRdsProtocolLogonErrorRedirector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolManager, IWRdsProtocolManager_Vtbl, 0xdc796967_3abb_40cd_a446_105276b58950);
impl core::ops::Deref for IWRdsProtocolManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolManager, windows_core::IUnknown);
impl IWRdsProtocolManager {
    pub unsafe fn Initialize<P0>(&self, piwrdssettings: P0, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWRdsProtocolSettings>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), piwrdssettings.param().abi(), pwrdssettings).ok()
    }
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> windows_core::Result<IWRdsProtocolListener>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), wszlistenername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyServiceStateChange)(windows_core::Interface::as_raw(self), ptsservicestatechange).ok()
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionStateChange)(windows_core::Interface::as_raw(self), sessionid, eventid).ok()
    }
    pub unsafe fn NotifySettingsChange(&self, pwrdssettings: *const WRDS_SETTINGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySettingsChange)(windows_core::Interface::as_raw(self), pwrdssettings).ok()
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
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
windows_core::imp::define_interface!(IWRdsProtocolSettings, IWRdsProtocolSettings_Vtbl, 0x654a5a6a_2550_47eb_b6f7_ebd637475265);
impl core::ops::Deref for IWRdsProtocolSettings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolSettings, windows_core::IUnknown);
impl IWRdsProtocolSettings {
    pub unsafe fn GetSettings(&self, wrdssettingtype: WRDS_SETTING_TYPE, wrdssettinglevel: WRDS_SETTING_LEVEL, pwrdssettings: *mut WRDS_SETTINGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSettings)(windows_core::Interface::as_raw(self), wrdssettingtype, wrdssettinglevel, pwrdssettings).ok()
    }
    pub unsafe fn MergeSettings(&self, pwrdssettings: *const WRDS_SETTINGS, wrdsconnectionsettinglevel: WRDS_CONNECTION_SETTING_LEVEL, pwrdsconnectionsettings: *mut WRDS_CONNECTION_SETTINGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).MergeSettings)(windows_core::Interface::as_raw(self), pwrdssettings, wrdsconnectionsettinglevel, pwrdsconnectionsettings).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolSettings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSettings: unsafe extern "system" fn(*mut core::ffi::c_void, WRDS_SETTING_TYPE, WRDS_SETTING_LEVEL, *mut WRDS_SETTINGS) -> windows_core::HRESULT,
    pub MergeSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *const WRDS_SETTINGS, WRDS_CONNECTION_SETTING_LEVEL, *mut WRDS_CONNECTION_SETTINGS) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolShadowCallback, IWRdsProtocolShadowCallback_Vtbl, 0xe0667ce0_0372_40d6_adb2_a0f3322674d6);
impl core::ops::Deref for IWRdsProtocolShadowCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowCallback, windows_core::IUnknown);
impl IWRdsProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopShadow)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvokeTargetShadow<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InvokeTargetShadow)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolShadowCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsProtocolShadowConnection, IWRdsProtocolShadowConnection_Vtbl, 0x9ae85ce6_cade_4548_8feb_99016597f60a);
impl core::ops::Deref for IWRdsProtocolShadowConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsProtocolShadowConnection, windows_core::IUnknown);
impl IWRdsProtocolShadowConnection {
    pub unsafe fn Start<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWRdsProtocolShadowCallback>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoTarget<P0>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DoTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWRdsProtocolShadowConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u8, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsWddmIddProps, IWRdsWddmIddProps_Vtbl, 0x1382df4d_a289_43d1_a184_144726f9af90);
impl core::ops::Deref for IWRdsWddmIddProps {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsWddmIddProps, windows_core::IUnknown);
impl IWRdsWddmIddProps {
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHardwareId)(windows_core::Interface::as_raw(self), core::mem::transmute(pdisplaydriverhardwareid.as_ptr()), pdisplaydriverhardwareid.len().try_into().unwrap()).ok()
    }
    pub unsafe fn OnDriverLoad<P0>(&self, sessionid: u32, driverhandle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
    {
        (windows_core::Interface::vtable(self).OnDriverLoad)(windows_core::Interface::as_raw(self), sessionid, driverhandle.param().abi()).ok()
    }
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDriverUnload)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn EnableWddmIdd<P0>(&self, enabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableWddmIdd)(windows_core::Interface::as_raw(self), enabled.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWRdsWddmIddProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHardwareId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnDriverLoad: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub OnDriverUnload: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnableWddmIdd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWRdsWddmIddProps1, IWRdsWddmIddProps1_Vtbl, 0x60f71b1a_3682_4bc7_997e_4e4f02a08148);
impl core::ops::Deref for IWRdsWddmIddProps1 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWRdsWddmIddProps1, windows_core::IUnknown);
impl IWRdsWddmIddProps1 {
    pub unsafe fn GetHardwareId(&self, pdisplaydriverhardwareid: &[u16]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHardwareId)(windows_core::Interface::as_raw(self), core::mem::transmute(pdisplaydriverhardwareid.as_ptr()), pdisplaydriverhardwareid.len().try_into().unwrap()).ok()
    }
    pub unsafe fn OnDriverLoad<P0>(&self, sessionid: u32, deviceinstance: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnDriverLoad)(windows_core::Interface::as_raw(self), sessionid, deviceinstance.param().abi()).ok()
    }
    pub unsafe fn OnDriverUnload(&self, sessionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDriverUnload)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
}
#[repr(C)]
pub struct IWRdsWddmIddProps1_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHardwareId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnDriverLoad: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDriverUnload: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSBitmapRenderService, IWTSBitmapRenderService_Vtbl, 0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
impl core::ops::Deref for IWTSBitmapRenderService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderService, windows_core::IUnknown);
impl IWTSBitmapRenderService {
    pub unsafe fn GetMappedRenderer<P0>(&self, mappingid: u64, pmappedrenderercallback: P0) -> windows_core::Result<IWTSBitmapRenderer>
    where
        P0: windows_core::Param<IWTSBitmapRendererCallback>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMappedRenderer)(windows_core::Interface::as_raw(self), mappingid, pmappedrenderercallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWTSBitmapRenderService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMappedRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSBitmapRenderer, IWTSBitmapRenderer_Vtbl, 0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
impl core::ops::Deref for IWTSBitmapRenderer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSBitmapRenderer, windows_core::IUnknown);
impl IWTSBitmapRenderer {
    pub unsafe fn Render(&self, imageformat: windows_core::GUID, dwwidth: u32, dwheight: u32, cbstride: i32, pimagebuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), core::mem::transmute(imageformat), dwwidth, dwheight, cbstride, pimagebuffer.len().try_into().unwrap(), core::mem::transmute(pimagebuffer.as_ptr())).ok()
    }
    pub unsafe fn GetRendererStatistics(&self) -> windows_core::Result<BITMAP_RENDERER_STATISTICS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRendererStatistics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveMapping(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveMapping)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWTSBitmapRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, i32, u32, *const u8) -> windows_core::HRESULT,
    pub GetRendererStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BITMAP_RENDERER_STATISTICS) -> windows_core::HRESULT,
    pub RemoveMapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSBitmapRendererCallback, IWTSBitmapRendererCallback_Vtbl, 0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
impl core::ops::Deref for IWTSBitmapRendererCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSBitmapRendererCallback, windows_core::IUnknown);
impl IWTSBitmapRendererCallback {
    pub unsafe fn OnTargetSizeChanged(&self, rcnewsize: super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnTargetSizeChanged)(windows_core::Interface::as_raw(self), core::mem::transmute(rcnewsize)).ok()
    }
}
#[repr(C)]
pub struct IWTSBitmapRendererCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTargetSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSListener, IWTSListener_Vtbl, 0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
impl core::ops::Deref for IWTSListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSListener, windows_core::IUnknown);
impl IWTSListener {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<super::Com::StructuredStorage::IPropertyBag> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWTSListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetConfiguration: usize,
}
windows_core::imp::define_interface!(IWTSListenerCallback, IWTSListenerCallback_Vtbl, 0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
impl core::ops::Deref for IWTSListenerCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSListenerCallback, windows_core::IUnknown);
impl IWTSListenerCallback {
    pub unsafe fn OnNewChannelConnection<P0, P1>(&self, pchannel: P0, data: P1, pbaccept: *mut super::super::Foundation::BOOL, ppcallback: *mut Option<IWTSVirtualChannelCallback>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSVirtualChannel>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnNewChannelConnection)(windows_core::Interface::as_raw(self), pchannel.param().abi(), data.param().abi(), pbaccept, core::mem::transmute(ppcallback)).ok()
    }
}
#[repr(C)]
pub struct IWTSListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNewChannelConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSPlugin, IWTSPlugin_Vtbl, 0xa1230201_1439_4e62_a414_190d0ac3d40e);
impl core::ops::Deref for IWTSPlugin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSPlugin, windows_core::IUnknown);
impl IWTSPlugin {
    pub unsafe fn Initialize<P0>(&self, pchannelmgr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSVirtualChannelManager>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pchannelmgr.param().abi()).ok()
    }
    pub unsafe fn Connected(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Connected)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Disconnected(&self, dwdisconnectcode: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Disconnected)(windows_core::Interface::as_raw(self), dwdisconnectcode).ok()
    }
    pub unsafe fn Terminated(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminated)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWTSPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Connected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnected: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSPluginServiceProvider, IWTSPluginServiceProvider_Vtbl, 0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
impl core::ops::Deref for IWTSPluginServiceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSPluginServiceProvider, windows_core::IUnknown);
impl IWTSPluginServiceProvider {
    pub unsafe fn GetService(&self, serviceid: windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), core::mem::transmute(serviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWTSPluginServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolConnection, IWTSProtocolConnection_Vtbl, 0x23083765_9095_4648_98bf_ef81c914032d);
impl core::ops::Deref for IWTSProtocolConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolConnection, windows_core::IUnknown);
impl IWTSProtocolConnection {
    pub unsafe fn GetLogonErrorRedirector(&self) -> windows_core::Result<IWTSProtocolLogonErrorRedirector> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLogonErrorRedirector)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SendPolicyData(&self, ppolicydata: *const WTS_POLICY_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendPolicyData)(windows_core::Interface::as_raw(self), ppolicydata).ok()
    }
    pub unsafe fn AcceptConnection(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcceptConnection)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetClientData(&self, pclientdata: *mut WTS_CLIENT_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClientData)(windows_core::Interface::as_raw(self), pclientdata).ok()
    }
    pub unsafe fn GetUserCredentials(&self, pusercreds: *mut WTS_USER_CREDENTIAL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserCredentials)(windows_core::Interface::as_raw(self), pusercreds).ok()
    }
    pub unsafe fn GetLicenseConnection(&self) -> windows_core::Result<IWTSProtocolLicenseConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLicenseConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AuthenticateClientToSession(&self, sessionid: *mut WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AuthenticateClientToSession)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionId(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionId)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn GetProtocolHandles(&self, pkeyboardhandle: *mut super::super::Foundation::HANDLE_PTR, pmousehandle: *mut super::super::Foundation::HANDLE_PTR, pbeephandle: *mut super::super::Foundation::HANDLE_PTR, pvideohandle: *mut super::super::Foundation::HANDLE_PTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProtocolHandles)(windows_core::Interface::as_raw(self), pkeyboardhandle, pmousehandle, pbeephandle, pvideohandle).ok()
    }
    pub unsafe fn ConnectNotify(&self, sessionid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConnectNotify)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn IsUserAllowedToLogon<P0, P1, P2>(&self, sessionid: u32, usertoken: P0, pdomainname: P1, pusername: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsUserAllowedToLogon)(windows_core::Interface::as_raw(self), sessionid, usertoken.param().abi(), pdomainname.param().abi(), pusername.param().abi()).ok()
    }
    pub unsafe fn SessionArbitrationEnumeration<P0, P1>(&self, husertoken: P0, bsinglesessionperuserenabled: P1, psessionidarray: *mut u32, pdwsessionidentifiercount: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SessionArbitrationEnumeration)(windows_core::Interface::as_raw(self), husertoken.param().abi(), bsinglesessionperuserenabled.param().abi(), psessionidarray, pdwsessionidentifiercount).ok()
    }
    pub unsafe fn LogonNotify<P0, P1, P2>(&self, hclienttoken: P0, wszusername: P1, wszdomainname: P2, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HANDLE_PTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LogonNotify)(windows_core::Interface::as_raw(self), hclienttoken.param().abi(), wszusername.param().abi(), wszdomainname.param().abi(), sessionid).ok()
    }
    pub unsafe fn GetUserData(&self, ppolicydata: *const WTS_POLICY_DATA, pclientdata: *mut WTS_USER_DATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserData)(windows_core::Interface::as_raw(self), ppolicydata, pclientdata).ok()
    }
    pub unsafe fn DisconnectNotify(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectNotify)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProtocolStatus(&self, pprotocolstatus: *mut WTS_PROTOCOL_STATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProtocolStatus)(windows_core::Interface::as_raw(self), pprotocolstatus).ok()
    }
    pub unsafe fn GetLastInputTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastInputTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetErrorInfo(&self, ulerror: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetErrorInfo)(windows_core::Interface::as_raw(self), ulerror).ok()
    }
    pub unsafe fn SendBeep(&self, frequency: u32, duration: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendBeep)(windows_core::Interface::as_raw(self), frequency, duration).ok()
    }
    pub unsafe fn CreateVirtualChannel<P0, P1>(&self, szendpointname: P0, bstatic: P1, requestedpriority: u32) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateVirtualChannel)(windows_core::Interface::as_raw(self), szendpointname.param().abi(), bstatic.param().abi(), requestedpriority, &mut result__).map(|| result__)
    }
    pub unsafe fn QueryProperty(&self, querytype: windows_core::GUID, ppropertyentriesin: &[WTS_PROPERTY_VALUE], ppropertyentriesout: &mut [WTS_PROPERTY_VALUE]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryProperty)(windows_core::Interface::as_raw(self), core::mem::transmute(querytype), ppropertyentriesin.len().try_into().unwrap(), ppropertyentriesout.len().try_into().unwrap(), core::mem::transmute(ppropertyentriesin.as_ptr()), core::mem::transmute(ppropertyentriesout.as_ptr())).ok()
    }
    pub unsafe fn GetShadowConnection(&self) -> windows_core::Result<IWTSProtocolShadowConnection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetShadowConnection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
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
    pub SessionArbitrationEnumeration: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, super::super::Foundation::BOOL, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub LogonNotify: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE_PTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_POLICY_DATA, *mut WTS_USER_DATA) -> windows_core::HRESULT,
    pub DisconnectNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProtocolStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_PROTOCOL_STATUS) -> windows_core::HRESULT,
    pub GetLastInputTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SendBeep: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CreateVirtualChannel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, super::super::Foundation::BOOL, u32, *mut usize) -> windows_core::HRESULT,
    pub QueryProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32, u32, *const WTS_PROPERTY_VALUE, *mut WTS_PROPERTY_VALUE) -> windows_core::HRESULT,
    pub GetShadowConnection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolConnectionCallback, IWTSProtocolConnectionCallback_Vtbl, 0x23083765_75eb_41fe_b4fb_e086242afa0f);
impl core::ops::Deref for IWTSProtocolConnectionCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolConnectionCallback, windows_core::IUnknown);
impl IWTSProtocolConnectionCallback {
    pub unsafe fn OnReady(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnReady)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BrokenConnection(&self, reason: u32, source: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BrokenConnection)(windows_core::Interface::as_raw(self), reason, source).ok()
    }
    pub unsafe fn StopScreenUpdates(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopScreenUpdates)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedrawWindow(&self, rect: *const WTS_SMALL_RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RedrawWindow)(windows_core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn DisplayIOCtl(&self, displayioctl: *const WTS_DISPLAY_IOCTL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisplayIOCtl)(windows_core::Interface::as_raw(self), displayioctl).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolConnectionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnReady: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrokenConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub StopScreenUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedrawWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SMALL_RECT) -> windows_core::HRESULT,
    pub DisplayIOCtl: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_DISPLAY_IOCTL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolLicenseConnection, IWTSProtocolLicenseConnection_Vtbl, 0x23083765_178c_4079_8e4a_fea6496a4d70);
impl core::ops::Deref for IWTSProtocolLicenseConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolLicenseConnection, windows_core::IUnknown);
impl IWTSProtocolLicenseConnection {
    pub unsafe fn RequestLicensingCapabilities(&self, pplicensecapabilities: *mut WTS_LICENSE_CAPABILITIES, pcblicensecapabilities: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestLicensingCapabilities)(windows_core::Interface::as_raw(self), pplicensecapabilities, pcblicensecapabilities).ok()
    }
    pub unsafe fn SendClientLicense(&self, pclientlicense: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(pclientlicense.as_ptr()), pclientlicense.len().try_into().unwrap()).ok()
    }
    pub unsafe fn RequestClientLicense(&self, reserve1: &[u8], ppclientlicense: *mut u8, pcbclientlicense: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestClientLicense)(windows_core::Interface::as_raw(self), core::mem::transmute(reserve1.as_ptr()), reserve1.len().try_into().unwrap(), ppclientlicense, pcbclientlicense).ok()
    }
    pub unsafe fn ProtocolComplete(&self, ulcomplete: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProtocolComplete)(windows_core::Interface::as_raw(self), ulcomplete).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolLicenseConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RequestLicensingCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WTS_LICENSE_CAPABILITIES, *mut u32) -> windows_core::HRESULT,
    pub SendClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub RequestClientLicense: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub ProtocolComplete: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolListener, IWTSProtocolListener_Vtbl, 0x23083765_45f0_4394_8f69_32b2bc0ef4ca);
impl core::ops::Deref for IWTSProtocolListener {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolListener, windows_core::IUnknown);
impl IWTSProtocolListener {
    pub unsafe fn StartListen<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWTSProtocolListenerCallback>,
    {
        (windows_core::Interface::vtable(self).StartListen)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok()
    }
    pub unsafe fn StopListen(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopListen)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolListener_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopListen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolListenerCallback, IWTSProtocolListenerCallback_Vtbl, 0x23083765_1a2d_4de2_97de_4a35f260f0b3);
impl core::ops::Deref for IWTSProtocolListenerCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolListenerCallback, windows_core::IUnknown);
impl IWTSProtocolListenerCallback {
    pub unsafe fn OnConnected<P0>(&self, pconnection: P0) -> windows_core::Result<IWTSProtocolConnectionCallback>
    where
        P0: windows_core::Param<IWTSProtocolConnection>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), pconnection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWTSProtocolListenerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolLogonErrorRedirector, IWTSProtocolLogonErrorRedirector_Vtbl, 0xfd9b61a7_2916_4627_8dee_4328711ad6cb);
impl core::ops::Deref for IWTSProtocolLogonErrorRedirector {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolLogonErrorRedirector, windows_core::IUnknown);
impl IWTSProtocolLogonErrorRedirector {
    pub unsafe fn OnBeginPainting(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBeginPainting)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RedirectStatus<P0>(&self, pszmessage: P0) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectStatus)(windows_core::Interface::as_raw(self), pszmessage.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RedirectMessage<P0, P1>(&self, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectMessage)(windows_core::Interface::as_raw(self), pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
    }
    pub unsafe fn RedirectLogonError<P0, P1>(&self, ntsstatus: i32, ntssubstatus: i32, pszcaption: P0, pszmessage: P1, utype: u32) -> windows_core::Result<WTS_LOGON_ERROR_REDIRECTOR_RESPONSE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectLogonError)(windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, pszcaption.param().abi(), pszmessage.param().abi(), utype, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWTSProtocolLogonErrorRedirector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBeginPainting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedirectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
    pub RedirectLogonError: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut WTS_LOGON_ERROR_REDIRECTOR_RESPONSE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolManager, IWTSProtocolManager_Vtbl, 0xf9eaf6cc_ed79_4f01_821d_1f881b9f66cc);
impl core::ops::Deref for IWTSProtocolManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolManager, windows_core::IUnknown);
impl IWTSProtocolManager {
    pub unsafe fn CreateListener<P0>(&self, wszlistenername: P0) -> windows_core::Result<IWTSProtocolListener>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), wszlistenername.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NotifyServiceStateChange(&self, ptsservicestatechange: *const WTS_SERVICE_STATE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyServiceStateChange)(windows_core::Interface::as_raw(self), ptsservicestatechange).ok()
    }
    pub unsafe fn NotifySessionOfServiceStart(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionOfServiceStart)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionOfServiceStop(&self, sessionid: *const WTS_SESSION_ID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionOfServiceStop)(windows_core::Interface::as_raw(self), sessionid).ok()
    }
    pub unsafe fn NotifySessionStateChange(&self, sessionid: *const WTS_SESSION_ID, eventid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifySessionStateChange)(windows_core::Interface::as_raw(self), sessionid, eventid).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyServiceStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SERVICE_STATE) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStart: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionOfServiceStop: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID) -> windows_core::HRESULT,
    pub NotifySessionStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const WTS_SESSION_ID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolShadowCallback, IWTSProtocolShadowCallback_Vtbl, 0x503a2504_aae5_4ab1_93e0_6d1c4bc6f71a);
impl core::ops::Deref for IWTSProtocolShadowCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolShadowCallback, windows_core::IUnknown);
impl IWTSProtocolShadowCallback {
    pub unsafe fn StopShadow(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).StopShadow)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvokeTargetShadow<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).InvokeTargetShadow)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolShadowCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StopShadow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokeTargetShadow: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSProtocolShadowConnection, IWTSProtocolShadowConnection_Vtbl, 0xee3b0c14_37fb_456b_bab3_6d6cd51e13bf);
impl core::ops::Deref for IWTSProtocolShadowConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSProtocolShadowConnection, windows_core::IUnknown);
impl IWTSProtocolShadowConnection {
    pub unsafe fn Start<P0, P1>(&self, ptargetservername: P0, targetsessionid: u32, hotkeyvk: u8, hotkeymodifiers: u16, pshadowcallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IWTSProtocolShadowCallback>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ptargetservername.param().abi(), targetsessionid, hotkeyvk, hotkeymodifiers, pshadowcallback.param().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DoTarget<P0>(&self, pparam1: &[u8], pparam2: &[u8], pparam3: &[u8], pparam4: &[u8], pclientname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DoTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(pparam1.as_ptr()), pparam1.len().try_into().unwrap(), core::mem::transmute(pparam2.as_ptr()), pparam2.len().try_into().unwrap(), core::mem::transmute(pparam3.as_ptr()), pparam3.len().try_into().unwrap(), core::mem::transmute(pparam4.as_ptr()), pparam4.len().try_into().unwrap(), pclientname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWTSProtocolShadowConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u8, u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *const u8, u32, *const u8, u32, *const u8, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSSBPlugin, IWTSSBPlugin_Vtbl, 0xdc44be78_b18d_4399_b210_641bf67a002c);
impl core::ops::Deref for IWTSSBPlugin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSSBPlugin, windows_core::IUnknown);
impl IWTSSBPlugin {
    pub unsafe fn Initialize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn WTSSBX_MachineChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, pmachineinfo: *const WTSSBX_MACHINE_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WTSSBX_MachineChangeNotification)(windows_core::Interface::as_raw(self), notificationtype, machineid, pmachineinfo).ok()
    }
    pub unsafe fn WTSSBX_SessionChangeNotification(&self, notificationtype: WTSSBX_NOTIFICATION_TYPE, machineid: i32, sessioninfo: &[WTSSBX_SESSION_INFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WTSSBX_SessionChangeNotification)(windows_core::Interface::as_raw(self), notificationtype, machineid, sessioninfo.len().try_into().unwrap(), core::mem::transmute(sessioninfo.as_ptr())).ok()
    }
    pub unsafe fn WTSSBX_GetMostSuitableServer<P0, P1, P2, P3>(&self, username: P0, domainname: P1, applicationtype: P2, farmname: P3, pmachineid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WTSSBX_GetMostSuitableServer)(windows_core::Interface::as_raw(self), username.param().abi(), domainname.param().abi(), applicationtype.param().abi(), farmname.param().abi(), pmachineid).ok()
    }
    pub unsafe fn Terminated(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminated)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WTSSBX_GetUserExternalSession<P0, P1, P2>(&self, username: P0, domainname: P1, applicationtype: P2, redirectorinternalip: *const WTSSBX_IP_ADDRESS, psessionid: *mut u32, pmachineconnectinfo: *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WTSSBX_GetUserExternalSession)(windows_core::Interface::as_raw(self), username.param().abi(), domainname.param().abi(), applicationtype.param().abi(), redirectorinternalip, psessionid, pmachineconnectinfo).ok()
    }
}
#[repr(C)]
pub struct IWTSSBPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub WTSSBX_MachineChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, WTSSBX_NOTIFICATION_TYPE, i32, *const WTSSBX_MACHINE_INFO) -> windows_core::HRESULT,
    pub WTSSBX_SessionChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, WTSSBX_NOTIFICATION_TYPE, i32, u32, *const WTSSBX_SESSION_INFO) -> windows_core::HRESULT,
    pub WTSSBX_GetMostSuitableServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub Terminated: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WTSSBX_GetUserExternalSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const WTSSBX_IP_ADDRESS, *mut u32, *mut WTSSBX_MACHINE_CONNECT_INFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSVirtualChannel, IWTSVirtualChannel_Vtbl, 0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
impl core::ops::Deref for IWTSVirtualChannel {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSVirtualChannel, windows_core::IUnknown);
impl IWTSVirtualChannel {
    pub unsafe fn Write<P0>(&self, pbuffer: &[u8], preserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), preserved.param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWTSVirtualChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSVirtualChannelCallback, IWTSVirtualChannelCallback_Vtbl, 0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
impl core::ops::Deref for IWTSVirtualChannelCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelCallback, windows_core::IUnknown);
impl IWTSVirtualChannelCallback {
    pub unsafe fn OnDataReceived(&self, pbuffer: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDataReceived)(windows_core::Interface::as_raw(self), pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr())).ok()
    }
    pub unsafe fn OnClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWTSVirtualChannelCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWTSVirtualChannelManager, IWTSVirtualChannelManager_Vtbl, 0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
impl core::ops::Deref for IWTSVirtualChannelManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWTSVirtualChannelManager, windows_core::IUnknown);
impl IWTSVirtualChannelManager {
    pub unsafe fn CreateListener<P0, P1>(&self, pszchannelname: P0, uflags: u32, plistenercallback: P1) -> windows_core::Result<IWTSListener>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<IWTSListenerCallback>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateListener)(windows_core::Interface::as_raw(self), pszchannelname.param().abi(), uflags, plistenercallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWTSVirtualChannelManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateListener: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkspace, IWorkspace_Vtbl, 0xb922bbb8_4c55_4fea_8496_beb0b44285e5);
impl core::ops::Deref for IWorkspace {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspace, windows_core::IUnknown);
impl IWorkspace {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWorkspaceNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication<P0>(&self, bstrworkspaceid: P0, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartRemoteApplication)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWorkspace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWorkspaceNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWorkspaceNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub StartRemoteApplication: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StartRemoteApplication: usize,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
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
    pub unsafe fn StartRemoteApplicationEx<P0, P1, P2, P3, P4>(&self, bstrworkspaceid: P0, bstrrequestingappid: P1, bstrrequestingappfamilyname: P2, blaunchintoimmersiveclient: P3, bstrimmersiveclientactivationcontext: P4, psaparams: *const super::Com::SAFEARRAY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P4: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartRemoteApplicationEx)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrrequestingappid.param().abi(), bstrrequestingappfamilyname.param().abi(), blaunchintoimmersiveclient.param().abi(), bstrimmersiveclientactivationcontext.param().abi(), psaparams).ok()
    }
}
#[repr(C)]
pub struct IWorkspace2_Vtbl {
    pub base__: IWorkspace_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub StartRemoteApplicationEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, *const super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    StartRemoteApplicationEx: usize,
}
windows_core::imp::define_interface!(IWorkspace3, IWorkspace3_Vtbl, 0x1becbe4a_d654_423b_afeb_be8d532c13c6);
impl core::ops::Deref for IWorkspace3 {
    type Target = IWorkspace2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspace3, windows_core::IUnknown, IWorkspace, IWorkspace2);
impl IWorkspace3 {
    pub unsafe fn GetClaimsToken2<P0, P1>(&self, bstrclaimshint: P0, bstruserhint: P1, claimcookie: u32, hwndcreduiparent: u32, rectcreduiparent: super::super::Foundation::RECT) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClaimsToken2)(windows_core::Interface::as_raw(self), bstrclaimshint.param().abi(), bstruserhint.param().abi(), claimcookie, hwndcreduiparent, core::mem::transmute(rectcreduiparent), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClaimsToken<P0, P1>(&self, bstraccesstoken: P0, ullaccesstokenexpiration: u64, bstrrefreshtoken: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClaimsToken)(windows_core::Interface::as_raw(self), bstraccesstoken.param().abi(), ullaccesstokenexpiration, bstrrefreshtoken.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWorkspace3_Vtbl {
    pub base__: IWorkspace2_Vtbl,
    pub GetClaimsToken2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, u32, super::super::Foundation::RECT, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClaimsToken: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u64, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkspaceClientExt, IWorkspaceClientExt_Vtbl, 0x12b952f4_41ca_4f21_a829_a6d07d9a16e5);
impl core::ops::Deref for IWorkspaceClientExt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspaceClientExt, windows_core::IUnknown);
impl IWorkspaceClientExt {
    pub unsafe fn GetResourceId(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceId)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetResourceDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceDisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IssueDisconnect(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IssueDisconnect)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWorkspaceClientExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetResourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IssueDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkspaceRegistration, IWorkspaceRegistration_Vtbl, 0xb922bbb8_4c55_4fea_8496_beb0b44285e6);
impl core::ops::Deref for IWorkspaceRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspaceRegistration, windows_core::IUnknown);
impl IWorkspaceRegistration {
    pub unsafe fn AddResource<P0>(&self, punk: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IWorkspaceClientExt>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddResource)(windows_core::Interface::as_raw(self), punk.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveResource)(windows_core::Interface::as_raw(self), dwcookieconnection).ok()
    }
}
#[repr(C)]
pub struct IWorkspaceRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkspaceRegistration2, IWorkspaceRegistration2_Vtbl, 0xcf59f654_39bb_44d8_94d0_4635728957e9);
impl core::ops::Deref for IWorkspaceRegistration2 {
    type Target = IWorkspaceRegistration;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspaceRegistration2, windows_core::IUnknown, IWorkspaceRegistration);
impl IWorkspaceRegistration2 {
    pub unsafe fn AddResourceEx<P0, P1>(&self, punk: P0, bstreventloguploadaddress: P1, pdwcookie: *mut u32, correlationid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWorkspaceClientExt>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddResourceEx)(windows_core::Interface::as_raw(self), punk.param().abi(), bstreventloguploadaddress.param().abi(), pdwcookie, core::mem::transmute(correlationid)).ok()
    }
    pub unsafe fn RemoveResourceEx(&self, dwcookieconnection: u32, correlationid: windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveResourceEx)(windows_core::Interface::as_raw(self), dwcookieconnection, core::mem::transmute(correlationid)).ok()
    }
}
#[repr(C)]
pub struct IWorkspaceRegistration2_Vtbl {
    pub base__: IWorkspaceRegistration_Vtbl,
    pub AddResourceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, windows_core::GUID) -> windows_core::HRESULT,
    pub RemoveResourceEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWorkspaceReportMessage, IWorkspaceReportMessage_Vtbl, 0xa7c06739_500f_4e8c_99a8_2bd6955899eb);
impl core::ops::Deref for IWorkspaceReportMessage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWorkspaceReportMessage, windows_core::IUnknown);
impl IWorkspaceReportMessage {
    pub unsafe fn RegisterErrorLogMessage<P0>(&self, bstrmessage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterErrorLogMessage)(windows_core::Interface::as_raw(self), bstrmessage.param().abi()).ok()
    }
    pub unsafe fn IsErrorMessageRegistered<P0, P1>(&self, bstrwkspid: P0, dwerrortype: u32, bstrerrormessagetype: P1, dwerrorcode: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsErrorMessageRegistered)(windows_core::Interface::as_raw(self), bstrwkspid.param().abi(), dwerrortype, bstrerrormessagetype.param().abi(), dwerrorcode, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterErrorEvent<P0, P1>(&self, bstrwkspid: P0, dwerrortype: u32, bstrerrormessagetype: P1, dwerrorcode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterErrorEvent)(windows_core::Interface::as_raw(self), bstrwkspid.param().abi(), dwerrortype, bstrerrormessagetype.param().abi(), dwerrorcode).ok()
    }
}
#[repr(C)]
pub struct IWorkspaceReportMessage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterErrorLogMessage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsErrorMessageRegistered: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RegisterErrorEvent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
}
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
    pub unsafe fn AddResourceType<P0, P1, P2>(&self, fmachinewide: P0, bstrfileextension: P1, bstrlauncher: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddResourceType)(windows_core::Interface::as_raw(self), fmachinewide.param().abi(), bstrfileextension.param().abi(), bstrlauncher.param().abi()).ok()
    }
    pub unsafe fn DeleteResourceType<P0, P1>(&self, fmachinewide: P0, bstrfileextension: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteResourceType)(windows_core::Interface::as_raw(self), fmachinewide.param().abi(), bstrfileextension.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRegisteredFileExtensions<P0>(&self, fmachinewide: P0) -> windows_core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRegisteredFileExtensions)(windows_core::Interface::as_raw(self), fmachinewide.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetResourceTypeInfo<P0, P1>(&self, fmachinewide: P0, bstrfileextension: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetResourceTypeInfo)(windows_core::Interface::as_raw(self), fmachinewide.param().abi(), bstrfileextension.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ModifyResourceType<P0, P1, P2>(&self, fmachinewide: P0, bstrfileextension: P1, bstrlauncher: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ModifyResourceType)(windows_core::Interface::as_raw(self), fmachinewide.param().abi(), bstrfileextension.param().abi(), bstrlauncher.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceResTypeRegistry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AddResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DeleteResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRegisteredFileExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut super::Com::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRegisteredFileExtensions: usize,
    pub GetResourceTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ModifyResourceType: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn DisconnectWorkspace<P0>(&self, bstrworkspaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisconnectWorkspace)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi()).ok()
    }
    pub unsafe fn StartWorkspace<P0, P1, P2, P3>(&self, bstrworkspaceid: P0, bstrusername: P1, bstrpassword: P2, bstrworkspaceparams: P3, ltimeout: i32, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartWorkspace)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrusername.param().abi(), bstrpassword.param().abi(), bstrworkspaceparams.param().abi(), ltimeout, lflags).ok()
    }
    pub unsafe fn IsWorkspaceCredentialSpecified<P0, P1>(&self, bstrworkspaceid: P0, bcountunauthenticatedcredentials: P1) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorkspaceCredentialSpecified)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bcountunauthenticatedcredentials.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorkspaceSSOEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClearWorkspaceCredential<P0>(&self, bstrworkspaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ClearWorkspaceCredential)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi()).ok()
    }
    pub unsafe fn OnAuthenticated<P0, P1>(&self, bstrworkspaceid: P0, bstrusername: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).OnAuthenticated)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrusername.param().abi()).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName<P0>(&self, bstrworkspacefriendlyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DisconnectWorkspaceByFriendlyName)(windows_core::Interface::as_raw(self), bstrworkspacefriendlyname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisconnectWorkspace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartWorkspace: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32) -> windows_core::HRESULT,
    pub IsWorkspaceCredentialSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsWorkspaceSSOEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClearWorkspaceCredential: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OnAuthenticated: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisconnectWorkspaceByFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn StartWorkspaceEx<P0, P1, P2, P3, P4, P5, P6>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1, bstrredirectorname: P2, bstrusername: P3, bstrpassword: P4, bstrappcontainer: P5, bstrworkspaceparams: P6, ltimeout: i32, lflags: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartWorkspaceEx)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrworkspacefriendlyname.param().abi(), bstrredirectorname.param().abi(), bstrusername.param().abi(), bstrpassword.param().abi(), bstrappcontainer.param().abi(), bstrworkspaceparams.param().abi(), ltimeout, lflags).ok()
    }
    pub unsafe fn ResourceDismissed<P0, P1>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ResourceDismissed)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrworkspacefriendlyname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable2_Vtbl {
    pub base__: IWorkspaceScriptable_Vtbl,
    pub StartWorkspaceEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32) -> windows_core::HRESULT,
    pub ResourceDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
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
    pub unsafe fn StartWorkspaceEx2<P0, P1, P2, P3, P4, P5, P6, P7>(&self, bstrworkspaceid: P0, bstrworkspacefriendlyname: P1, bstrredirectorname: P2, bstrusername: P3, bstrpassword: P4, bstrappcontainer: P5, bstrworkspaceparams: P6, ltimeout: i32, lflags: i32, bstreventloguploadaddress: P7, correlationid: windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
        P4: windows_core::Param<windows_core::BSTR>,
        P5: windows_core::Param<windows_core::BSTR>,
        P6: windows_core::Param<windows_core::BSTR>,
        P7: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).StartWorkspaceEx2)(windows_core::Interface::as_raw(self), bstrworkspaceid.param().abi(), bstrworkspacefriendlyname.param().abi(), bstrredirectorname.param().abi(), bstrusername.param().abi(), bstrpassword.param().abi(), bstrappcontainer.param().abi(), bstrworkspaceparams.param().abi(), ltimeout, lflags, bstreventloguploadaddress.param().abi(), core::mem::transmute(correlationid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWorkspaceScriptable3_Vtbl {
    pub base__: IWorkspaceScriptable2_Vtbl,
    pub StartWorkspaceEx2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, core::mem::MaybeUninit<windows_core::BSTR>, windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ItsPubPlugin, ItsPubPlugin_Vtbl, 0x70c04b05_f347_412b_822f_36c99c54ca45);
impl core::ops::Deref for ItsPubPlugin {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ItsPubPlugin, windows_core::IUnknown);
impl ItsPubPlugin {
    pub unsafe fn GetResourceList<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetResourceList)(windows_core::Interface::as_raw(self), userid.param().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetResource)(windows_core::Interface::as_raw(self), alias.param().abi(), flags, resource).ok()
    }
    pub unsafe fn GetCacheLastUpdateTime(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCacheLastUpdateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn pluginName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).pluginName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn pluginVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).pluginVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ResolveResource<P0, P1>(&self, resourcetype: *mut u32, resourcelocation: &mut [u16; 256], endpointname: &mut [u16; 256], userid: P0, alias: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ResolveResource)(windows_core::Interface::as_raw(self), resourcetype, core::mem::transmute(resourcelocation.as_ptr()), core::mem::transmute(endpointname.as_ptr()), userid.param().abi(), alias.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ItsPubPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetResourceList: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut *mut pluginResource) -> windows_core::HRESULT,
    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut pluginResource) -> windows_core::HRESULT,
    pub GetCacheLastUpdateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub pluginName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub pluginVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ResolveResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR, windows_core::PWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
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
        (windows_core::Interface::vtable(self).GetResource2List)(windows_core::Interface::as_raw(self), userid.param().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource2<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetResource2)(windows_core::Interface::as_raw(self), alias.param().abi(), flags, resource).ok()
    }
    pub unsafe fn ResolvePersonalDesktop<P0, P1>(&self, userid: P0, poolid: P1, epdresolutiontype: TSPUB_PLUGIN_PD_RESOLUTION_TYPE, ppdassignmenttype: *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, endpointname: &mut [u16; 256]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ResolvePersonalDesktop)(windows_core::Interface::as_raw(self), userid.param().abi(), poolid.param().abi(), epdresolutiontype, ppdassignmenttype, core::mem::transmute(endpointname.as_ptr())).ok()
    }
    pub unsafe fn DeletePersonalDesktopAssignment<P0, P1, P2>(&self, userid: P0, poolid: P1, endpointname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeletePersonalDesktopAssignment)(windows_core::Interface::as_raw(self), userid.param().abi(), poolid.param().abi(), endpointname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ItsPubPlugin2_Vtbl {
    pub base__: ItsPubPlugin_Vtbl,
    pub GetResource2List: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut i32, *mut *mut pluginResource2) -> windows_core::HRESULT,
    pub GetResource2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, *mut pluginResource2) -> windows_core::HRESULT,
    pub ResolvePersonalDesktop: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, TSPUB_PLUGIN_PD_RESOLUTION_TYPE, *mut TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE, windows_core::PWSTR) -> windows_core::HRESULT,
    pub DeletePersonalDesktopAssignment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
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
impl _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct _ITSWkspEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
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
pub const AllowOnlySDRServers: PolicyAttributeType = PolicyAttributeType(7i32);
pub const CHANNEL_BUFFER_SIZE: u32 = 65535u32;
pub const CHANNEL_CHUNK_LENGTH: u32 = 1600u32;
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
pub const CLIENT_MESSAGE_CONNECTION_ERROR: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(2i32);
pub const CLIENT_MESSAGE_CONNECTION_INVALID: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(0i32);
pub const CLIENT_MESSAGE_CONNECTION_STATUS: CLIENT_MESSAGE_TYPE = CLIENT_MESSAGE_TYPE(1i32);
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
pub const KEEP_EXISTING_SESSIONS: u32 = 8u32;
pub const KeyCombinationDown: KeyCombinationType = KeyCombinationType(4i32);
pub const KeyCombinationHome: KeyCombinationType = KeyCombinationType(0i32);
pub const KeyCombinationLeft: KeyCombinationType = KeyCombinationType(1i32);
pub const KeyCombinationRight: KeyCombinationType = KeyCombinationType(3i32);
pub const KeyCombinationScroll: KeyCombinationType = KeyCombinationType(5i32);
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
pub const PLACEMENT_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(8i32);
pub const PLUGIN_CAPABILITY_EXTERNAL_REDIRECTION: u32 = 1u32;
pub const POLICY_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(1i32);
pub const POSITION_CONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(2i32);
pub const POSITION_DISCONTINUOUS: AE_POSITION_FLAGS = AE_POSITION_FLAGS(1i32);
pub const POSITION_INVALID: AE_POSITION_FLAGS = AE_POSITION_FLAGS(0i32);
pub const POSITION_QPC_ERROR: AE_POSITION_FLAGS = AE_POSITION_FLAGS(4i32);
pub const PRODUCTINFO_COMPANYNAME_LENGTH: u32 = 256u32;
pub const PRODUCTINFO_PRODUCTID_LENGTH: u32 = 4u32;
pub const PROPERTY_DYNAMIC_TIME_ZONE_INFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x0cdfd28e_d0b9_4c1f_a5eb_6d1f6c6535b9);
pub const PROPERTY_TYPE_ENABLE_UNIVERSAL_APPS_FOR_CUSTOM_SHELL: windows_core::GUID = windows_core::GUID::from_u128(0xed2c3fda_338d_4d3f_81a3_e767310d908e);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT: windows_core::GUID = windows_core::GUID::from_u128(0x6212d757_0043_4862_99c3_9f3059ac2a3b);
pub const PROPERTY_TYPE_GET_FAST_RECONNECT_USER_SID: windows_core::GUID = windows_core::GUID::from_u128(0x197c427a_0135_4b6d_9c5e_e6579a0ab625);
pub const PROVISIONING_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(32i32);
pub const PasswordEncodingUTF16BE: PasswordEncodingType = PasswordEncodingType(2i32);
pub const PasswordEncodingUTF16LE: PasswordEncodingType = PasswordEncodingType(1i32);
pub const PasswordEncodingUTF8: PasswordEncodingType = PasswordEncodingType(0i32);
pub const PnpRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(6i32);
pub const PortRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(4i32);
pub const PrinterRedirectionDisabled: PolicyAttributeType = PolicyAttributeType(3i32);
pub const RDCLIENT_BITMAP_RENDER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
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
pub const RFX_GFX_MSG_PREFIX: u32 = 48u32;
pub const RFX_GFX_MSG_PREFIX_MASK: u32 = 48u32;
pub const RFX_RDP_MSG_PREFIX: u32 = 0u32;
pub const RemoteActionAppSwitch: RemoteActionType = RemoteActionType(4i32);
pub const RemoteActionAppbar: RemoteActionType = RemoteActionType(1i32);
pub const RemoteActionCharms: RemoteActionType = RemoteActionType(0i32);
pub const RemoteActionSnap: RemoteActionType = RemoteActionType(2i32);
pub const RemoteActionStartScreen: RemoteActionType = RemoteActionType(3i32);
pub const SB_SYNCH_CONFLICT_MAX_WRITE_ATTEMPTS: u32 = 100u32;
pub const SESSION_TIMEOUT_ACTION_DISCONNECT: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(0i32);
pub const SESSION_TIMEOUT_ACTION_SILENT_REAUTH: SESSION_TIMEOUT_ACTION_TYPE = SESSION_TIMEOUT_ACTION_TYPE(1i32);
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
pub const SnapshotFormatBmp: SnapshotFormatType = SnapshotFormatType(2i32);
pub const SnapshotFormatJpeg: SnapshotFormatType = SnapshotFormatType(1i32);
pub const SnapshotFormatPng: SnapshotFormatType = SnapshotFormatType(0i32);
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
pub const TARGET_PATCH_COMPLETED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(3i32);
pub const TARGET_PATCH_FAILED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(4i32);
pub const TARGET_PATCH_IN_PROGRESS: TARGET_PATCH_STATE = TARGET_PATCH_STATE(2i32);
pub const TARGET_PATCH_NOT_STARTED: TARGET_PATCH_STATE = TARGET_PATCH_STATE(1i32);
pub const TARGET_PATCH_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(512i32);
pub const TARGET_PATCH_UNKNOWN: TARGET_PATCH_STATE = TARGET_PATCH_STATE(0i32);
pub const TARGET_PENDING: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(128i32);
pub const TARGET_REMOVED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(16i32);
pub const TARGET_RUNNING: TARGET_STATE = TARGET_STATE(3i32);
pub const TARGET_STARTING: TARGET_STATE = TARGET_STATE(9i32);
pub const TARGET_STATE_CHANGED: TARGET_CHANGE_TYPE = TARGET_CHANGE_TYPE(32i32);
pub const TARGET_STOPPED: TARGET_STATE = TARGET_STATE(7i32);
pub const TARGET_STOPPING: TARGET_STATE = TARGET_STATE(10i32);
pub const TARGET_UNKNOWN: TARGET_STATE = TARGET_STATE(1i32);
pub const TASK_PLUGIN: PLUGIN_TYPE = PLUGIN_TYPE(64i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_ASSIGNMENT_NEW: TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE = TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(0i32);
pub const TSPUB_PLUGIN_PD_QUERY_EXISTING: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(1i32);
pub const TSPUB_PLUGIN_PD_QUERY_OR_CREATE: TSPUB_PLUGIN_PD_RESOLUTION_TYPE = TSPUB_PLUGIN_PD_RESOLUTION_TYPE(0i32);
pub const TSSB_NOTIFY_CONNECTION_REQUEST_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(4i32);
pub const TSSB_NOTIFY_INVALID: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(0i32);
pub const TSSB_NOTIFY_SESSION_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(2i32);
pub const TSSB_NOTIFY_TARGET_CHANGE: TSSB_NOTIFICATION_TYPE = TSSB_NOTIFICATION_TYPE(1i32);
pub const TSSD_ADDR_IPv4: TSSD_AddrV46Type = TSSD_AddrV46Type(4i32);
pub const TSSD_ADDR_IPv6: TSSD_AddrV46Type = TSSD_AddrV46Type(6i32);
pub const TSSD_ADDR_UNDEFINED: TSSD_AddrV46Type = TSSD_AddrV46Type(0i32);
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
pub const VM_HOST_STATUS_INIT_COMPLETE: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(2i32);
pub const VM_HOST_STATUS_INIT_FAILED: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(3i32);
pub const VM_HOST_STATUS_INIT_IN_PROGRESS: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(1i32);
pub const VM_HOST_STATUS_INIT_PENDING: VM_HOST_NOTIFY_STATUS = VM_HOST_NOTIFY_STATUS(0i32);
pub const VM_NOTIFY_STATUS_CANCELED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(4i32);
pub const VM_NOTIFY_STATUS_COMPLETE: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(2i32);
pub const VM_NOTIFY_STATUS_FAILED: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(3i32);
pub const VM_NOTIFY_STATUS_IN_PROGRESS: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(1i32);
pub const VM_NOTIFY_STATUS_PENDING: VM_NOTIFY_STATUS = VM_NOTIFY_STATUS(0i32);
pub const WINSTATIONNAME_LENGTH: u32 = 32u32;
pub const WKS_FLAG_CLEAR_CREDS_ON_LAST_RESOURCE: u32 = 1u32;
pub const WKS_FLAG_CREDS_AUTHENTICATED: u32 = 4u32;
pub const WKS_FLAG_PASSWORD_ENCRYPTED: u32 = 2u32;
pub const WRDS_CLIENTADDRESS_LENGTH: u32 = 30u32;
pub const WRDS_CLIENTNAME_LENGTH: u32 = 20u32;
pub const WRDS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WRDS_CONNECTION_SETTING_LEVEL_1: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(1i32);
pub const WRDS_CONNECTION_SETTING_LEVEL_INVALID: WRDS_CONNECTION_SETTING_LEVEL = WRDS_CONNECTION_SETTING_LEVEL(0i32);
pub const WRDS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WRDS_DIRECTORY_LENGTH: u32 = 256u32;
pub const WRDS_DOMAIN_LENGTH: u32 = 255u32;
pub const WRDS_DRIVER_NAME_LENGTH: u32 = 8u32;
pub const WRDS_IMEFILENAME_LENGTH: u32 = 32u32;
pub const WRDS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WRDS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WRDS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WRDS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WRDS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
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
pub const WRDS_SETTING_LEVEL_1: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(1i32);
pub const WRDS_SETTING_LEVEL_INVALID: WRDS_SETTING_LEVEL = WRDS_SETTING_LEVEL(0i32);
pub const WRDS_SETTING_STATUS_DISABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(0i32);
pub const WRDS_SETTING_STATUS_ENABLED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(1i32);
pub const WRDS_SETTING_STATUS_NOTAPPLICABLE: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(-1i32);
pub const WRDS_SETTING_STATUS_NOTCONFIGURED: WRDS_SETTING_STATUS = WRDS_SETTING_STATUS(2i32);
pub const WRDS_SETTING_TYPE_INVALID: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(0i32);
pub const WRDS_SETTING_TYPE_MACHINE: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(1i32);
pub const WRDS_SETTING_TYPE_SAM: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(3i32);
pub const WRDS_SETTING_TYPE_USER: WRDS_SETTING_TYPE = WRDS_SETTING_TYPE(2i32);
pub const WRDS_USERNAME_LENGTH: u32 = 255u32;
pub const WRDS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WRDS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WRDS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WRDS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WRdsGraphicsChannelType_BestEffortDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(1i32);
pub const WRdsGraphicsChannelType_GuaranteedDelivery: WRdsGraphicsChannelType = WRdsGraphicsChannelType(0i32);
pub const WRdsGraphicsChannels_LossyChannelMaxMessageSize: u32 = 988u32;
pub const WTSActive: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(0i32);
pub const WTSApplicationName: WTS_INFO_CLASS = WTS_INFO_CLASS(1i32);
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
pub const WTSIdle: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(5i32);
pub const WTSIdleTime: WTS_INFO_CLASS = WTS_INFO_CLASS(17i32);
pub const WTSIncomingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(19i32);
pub const WTSIncomingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(21i32);
pub const WTSInit: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(9i32);
pub const WTSInitialProgram: WTS_INFO_CLASS = WTS_INFO_CLASS(0i32);
pub const WTSIsRemoteSession: WTS_INFO_CLASS = WTS_INFO_CLASS(29i32);
pub const WTSListen: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(6i32);
pub const WTSLogonTime: WTS_INFO_CLASS = WTS_INFO_CLASS(18i32);
pub const WTSOEMId: WTS_INFO_CLASS = WTS_INFO_CLASS(3i32);
pub const WTSOutgoingBytes: WTS_INFO_CLASS = WTS_INFO_CLASS(20i32);
pub const WTSOutgoingFrames: WTS_INFO_CLASS = WTS_INFO_CLASS(22i32);
pub const WTSReset: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(7i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(1i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_INET6: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(2i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_IPX: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(3i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_NETBIOS: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(4i32);
pub const WTSSBX_ADDRESS_FAMILY_AF_UNSPEC: WTSSBX_ADDRESS_FAMILY = WTSSBX_ADDRESS_FAMILY(0i32);
pub const WTSSBX_MACHINE_DRAIN_OFF: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(1i32);
pub const WTSSBX_MACHINE_DRAIN_ON: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(2i32);
pub const WTSSBX_MACHINE_DRAIN_UNSPEC: WTSSBX_MACHINE_DRAIN = WTSSBX_MACHINE_DRAIN(0i32);
pub const WTSSBX_MACHINE_SESSION_MODE_MULTIPLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(2i32);
pub const WTSSBX_MACHINE_SESSION_MODE_SINGLE: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(1i32);
pub const WTSSBX_MACHINE_SESSION_MODE_UNSPEC: WTSSBX_MACHINE_SESSION_MODE = WTSSBX_MACHINE_SESSION_MODE(0i32);
pub const WTSSBX_MACHINE_STATE_READY: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(1i32);
pub const WTSSBX_MACHINE_STATE_SYNCHRONIZING: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(2i32);
pub const WTSSBX_MACHINE_STATE_UNSPEC: WTSSBX_MACHINE_STATE = WTSSBX_MACHINE_STATE(0i32);
pub const WTSSBX_NOTIFICATION_ADDED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(4i32);
pub const WTSSBX_NOTIFICATION_CHANGED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(2i32);
pub const WTSSBX_NOTIFICATION_REMOVED: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(1i32);
pub const WTSSBX_NOTIFICATION_RESYNC: WTSSBX_NOTIFICATION_TYPE = WTSSBX_NOTIFICATION_TYPE(8i32);
pub const WTSSBX_SESSION_STATE_ACTIVE: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(1i32);
pub const WTSSBX_SESSION_STATE_DISCONNECTED: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(2i32);
pub const WTSSBX_SESSION_STATE_UNSPEC: WTSSBX_SESSION_STATE = WTSSBX_SESSION_STATE(0i32);
pub const WTSSessionAddressV4: WTS_INFO_CLASS = WTS_INFO_CLASS(28i32);
pub const WTSSessionId: WTS_INFO_CLASS = WTS_INFO_CLASS(4i32);
pub const WTSSessionInfo: WTS_INFO_CLASS = WTS_INFO_CLASS(24i32);
pub const WTSSessionInfoEx: WTS_INFO_CLASS = WTS_INFO_CLASS(25i32);
pub const WTSShadow: WTS_CONNECTSTATE_CLASS = WTS_CONNECTSTATE_CLASS(3i32);
pub const WTSTypeProcessInfoLevel0: WTS_TYPE_CLASS = WTS_TYPE_CLASS(0i32);
pub const WTSTypeProcessInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(1i32);
pub const WTSTypeSessionInfoLevel1: WTS_TYPE_CLASS = WTS_TYPE_CLASS(2i32);
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
pub const WTS_CLIENT_PRODUCT_ID_LENGTH: u32 = 32u32;
pub const WTS_COMMENT_LENGTH: u32 = 60u32;
pub const WTS_CURRENT_SERVER: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
pub const WTS_CURRENT_SERVER_HANDLE: super::super::Foundation::HANDLE = super::super::Foundation::HANDLE(0i32 as _);
pub const WTS_CURRENT_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("");
pub const WTS_CURRENT_SESSION: u32 = 4294967295u32;
pub const WTS_DEVICE_NAME_LENGTH: u32 = 19u32;
pub const WTS_DIRECTORY_LENGTH: u32 = 256u32;
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
pub const WTS_INITIALPROGRAM_LENGTH: u32 = 256u32;
pub const WTS_KEY_EXCHANGE_ALG_DH: u32 = 2u32;
pub const WTS_KEY_EXCHANGE_ALG_RSA: u32 = 1u32;
pub const WTS_LICENSE_PREAMBLE_VERSION: u32 = 3u32;
pub const WTS_LICENSE_PROTOCOL_VERSION: u32 = 65536u32;
pub const WTS_LISTENER_CREATE: u32 = 1u32;
pub const WTS_LISTENER_NAME_LENGTH: u32 = 32u32;
pub const WTS_LISTENER_UPDATE: u32 = 16u32;
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
pub const WTS_PROCESS_INFO_LEVEL_0: u32 = 0u32;
pub const WTS_PROCESS_INFO_LEVEL_1: u32 = 1u32;
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_core::PCWSTR = windows_core::w!("DefaultConfig");
pub const WTS_PROTOCOL_NAME_LENGTH: u32 = 8u32;
pub const WTS_PROTOCOL_TYPE_CONSOLE: u32 = 0u32;
pub const WTS_PROTOCOL_TYPE_ICA: u32 = 1u32;
pub const WTS_PROTOCOL_TYPE_RDP: u32 = 2u32;
pub const WTS_QUERY_ALLOWED_INITIAL_APP: windows_core::GUID = windows_core::GUID::from_u128(0xc77d1b30_5be1_4c6b_a0e1_bd6d2e5c9fcc);
pub const WTS_QUERY_AUDIOENUM_DLL: windows_core::GUID = windows_core::GUID::from_u128(0x9bf4fa97_c883_4c2a_80ab_5a39c9af00db);
pub const WTS_QUERY_LOGON_SCREEN_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x8b8e0fe7_0804_4a0e_b279_8660b1df0049);
pub const WTS_QUERY_MF_FORMAT_SUPPORT: windows_core::GUID = windows_core::GUID::from_u128(0x41869ad0_6332_4dc8_95d5_db749e2f1d94);
pub const WTS_SECURITY_ALL_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(983999u32);
pub const WTS_SECURITY_CONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(256u32);
pub const WTS_SECURITY_CURRENT_GUEST_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(72u32);
pub const WTS_SECURITY_CURRENT_USER_ACCESS: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(590u32);
pub const WTS_SECURITY_DISCONNECT: WTS_SECURITY_FLAGS = WTS_SECURITY_FLAGS(512u32);
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
pub const WTS_SERVICE_NONE: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(0i32);
pub const WTS_SERVICE_START: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(1i32);
pub const WTS_SERVICE_STOP: WTS_RCM_SERVICE_STATE = WTS_RCM_SERVICE_STATE(2i32);
pub const WTS_SESSIONSTATE_LOCK: u32 = 0u32;
pub const WTS_SESSIONSTATE_UNKNOWN: u32 = 4294967295u32;
pub const WTS_SESSIONSTATE_UNLOCK: u32 = 1u32;
pub const WTS_USERNAME_LENGTH: u32 = 255u32;
pub const WTS_VALUE_TYPE_BINARY: u32 = 3u32;
pub const WTS_VALUE_TYPE_GUID: u32 = 4u32;
pub const WTS_VALUE_TYPE_STRING: u32 = 2u32;
pub const WTS_VALUE_TYPE_ULONG: u32 = 1u32;
pub const WTS_WSD_FASTREBOOT: u32 = 16u32;
pub const WTS_WSD_LOGOFF: u32 = 1u32;
pub const WTS_WSD_POWEROFF: u32 = 8u32;
pub const WTS_WSD_REBOOT: u32 = 4u32;
pub const WTS_WSD_SHUTDOWN: u32 = 2u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AAAccountingDataType(pub i32);
impl windows_core::TypeKind for AAAccountingDataType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AAAccountingDataType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AAAccountingDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AAAuthSchemes(pub i32);
impl windows_core::TypeKind for AAAuthSchemes {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AAAuthSchemes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AAAuthSchemes").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AATrustClassID(pub i32);
impl windows_core::TypeKind for AATrustClassID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AATrustClassID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AATrustClassID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AE_POSITION_FLAGS(pub i32);
impl windows_core::TypeKind for AE_POSITION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AE_POSITION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AE_POSITION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLIENT_MESSAGE_TYPE(pub i32);
impl windows_core::TypeKind for CLIENT_MESSAGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLIENT_MESSAGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLIENT_MESSAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CONNECTION_CHANGE_NOTIFICATION(pub i32);
impl windows_core::TypeKind for CONNECTION_CHANGE_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CONNECTION_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CONNECTION_CHANGE_NOTIFICATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KeyCombinationType(pub i32);
impl windows_core::TypeKind for KeyCombinationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KeyCombinationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KeyCombinationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PLUGIN_TYPE(pub i32);
impl windows_core::TypeKind for PLUGIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PLUGIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PLUGIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PasswordEncodingType(pub i32);
impl windows_core::TypeKind for PasswordEncodingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PasswordEncodingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PasswordEncodingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PolicyAttributeType(pub i32);
impl windows_core::TypeKind for PolicyAttributeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PolicyAttributeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PolicyAttributeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RDV_TASK_STATUS(pub i32);
impl windows_core::TypeKind for RDV_TASK_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RDV_TASK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RDV_TASK_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RD_FARM_TYPE(pub i32);
impl windows_core::TypeKind for RD_FARM_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RD_FARM_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RD_FARM_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RemoteActionType(pub i32);
impl windows_core::TypeKind for RemoteActionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RemoteActionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RemoteActionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SESSION_TIMEOUT_ACTION_TYPE(pub i32);
impl windows_core::TypeKind for SESSION_TIMEOUT_ACTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SESSION_TIMEOUT_ACTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SESSION_TIMEOUT_ACTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SnapshotEncodingType(pub i32);
impl windows_core::TypeKind for SnapshotEncodingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SnapshotEncodingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SnapshotEncodingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SnapshotFormatType(pub i32);
impl windows_core::TypeKind for SnapshotFormatType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SnapshotFormatType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SnapshotFormatType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for TARGET_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_OWNER(pub i32);
impl windows_core::TypeKind for TARGET_OWNER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_OWNER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_OWNER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_PATCH_STATE(pub i32);
impl windows_core::TypeKind for TARGET_PATCH_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_PATCH_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_PATCH_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_STATE(pub i32);
impl windows_core::TypeKind for TARGET_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TARGET_TYPE(pub i32);
impl windows_core::TypeKind for TARGET_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TARGET_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TARGET_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE(pub i32);
impl windows_core::TypeKind for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TSPUB_PLUGIN_PD_RESOLUTION_TYPE(pub i32);
impl windows_core::TypeKind for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_RESOLUTION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TSSB_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for TSSB_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TSSB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TSSB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TSSD_AddrV46Type(pub i32);
impl windows_core::TypeKind for TSSD_AddrV46Type {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TSSD_AddrV46Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TSSD_AddrV46Type").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TSSESSION_STATE(pub i32);
impl windows_core::TypeKind for TSSESSION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TSSESSION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TSSESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TS_SB_SORT_BY(pub i32);
impl windows_core::TypeKind for TS_SB_SORT_BY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TS_SB_SORT_BY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TS_SB_SORT_BY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VM_HOST_NOTIFY_STATUS(pub i32);
impl windows_core::TypeKind for VM_HOST_NOTIFY_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VM_HOST_NOTIFY_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VM_HOST_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VM_NOTIFY_STATUS(pub i32);
impl windows_core::TypeKind for VM_NOTIFY_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VM_NOTIFY_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VM_NOTIFY_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRDS_CONNECTION_SETTING_LEVEL(pub i32);
impl windows_core::TypeKind for WRDS_CONNECTION_SETTING_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRDS_CONNECTION_SETTING_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRDS_CONNECTION_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRDS_LISTENER_SETTING_LEVEL(pub i32);
impl windows_core::TypeKind for WRDS_LISTENER_SETTING_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRDS_LISTENER_SETTING_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRDS_LISTENER_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRDS_SETTING_LEVEL(pub i32);
impl windows_core::TypeKind for WRDS_SETTING_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRDS_SETTING_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRDS_SETTING_STATUS(pub i32);
impl windows_core::TypeKind for WRDS_SETTING_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRDS_SETTING_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRDS_SETTING_TYPE(pub i32);
impl windows_core::TypeKind for WRDS_SETTING_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRDS_SETTING_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WRdsGraphicsChannelType(pub i32);
impl windows_core::TypeKind for WRdsGraphicsChannelType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WRdsGraphicsChannelType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WRdsGraphicsChannelType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_ADDRESS_FAMILY(pub i32);
impl windows_core::TypeKind for WTSSBX_ADDRESS_FAMILY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_ADDRESS_FAMILY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_ADDRESS_FAMILY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_MACHINE_DRAIN(pub i32);
impl windows_core::TypeKind for WTSSBX_MACHINE_DRAIN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_MACHINE_DRAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_DRAIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_MACHINE_SESSION_MODE(pub i32);
impl windows_core::TypeKind for WTSSBX_MACHINE_SESSION_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_MACHINE_SESSION_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_SESSION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_MACHINE_STATE(pub i32);
impl windows_core::TypeKind for WTSSBX_MACHINE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_MACHINE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_NOTIFICATION_TYPE(pub i32);
impl windows_core::TypeKind for WTSSBX_NOTIFICATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTSSBX_SESSION_STATE(pub i32);
impl windows_core::TypeKind for WTSSBX_SESSION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTSSBX_SESSION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTSSBX_SESSION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_CERT_TYPE(pub i32);
impl windows_core::TypeKind for WTS_CERT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_CERT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_CERT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_CONFIG_CLASS(pub i32);
impl windows_core::TypeKind for WTS_CONFIG_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_CONFIG_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_CONFIG_SOURCE(pub i32);
impl windows_core::TypeKind for WTS_CONFIG_SOURCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_CONFIG_SOURCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_SOURCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_CONNECTSTATE_CLASS(pub i32);
impl windows_core::TypeKind for WTS_CONNECTSTATE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_CONNECTSTATE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_CONNECTSTATE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_INFO_CLASS(pub i32);
impl windows_core::TypeKind for WTS_INFO_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_INFO_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_LOGON_ERROR_REDIRECTOR_RESPONSE(pub i32);
impl windows_core::TypeKind for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_LOGON_ERROR_REDIRECTOR_RESPONSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_RCM_DRAIN_STATE(pub i32);
impl windows_core::TypeKind for WTS_RCM_DRAIN_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_RCM_DRAIN_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_RCM_DRAIN_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_RCM_SERVICE_STATE(pub i32);
impl windows_core::TypeKind for WTS_RCM_SERVICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_RCM_SERVICE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_RCM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_SECURITY_FLAGS(pub u32);
impl windows_core::TypeKind for WTS_SECURITY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_SECURITY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_SECURITY_FLAGS").field(&self.0).finish()
    }
}
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
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_TYPE_CLASS(pub i32);
impl windows_core::TypeKind for WTS_TYPE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_TYPE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_TYPE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WTS_VIRTUAL_CLASS(pub i32);
impl windows_core::TypeKind for WTS_VIRTUAL_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WTS_VIRTUAL_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WTS_VIRTUAL_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
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
impl Clone for AAAccountingData {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for AAAccountingData {
    type TypeKind = windows_core::CopyType;
}
impl Default for AAAccountingData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADsTSUserEx: windows_core::GUID = windows_core::GUID::from_u128(0xe2e9cae6_1e7b_4b8e_babd_e9bf6292ac29);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AE_CURRENT_POSITION {
    pub u64DevicePosition: u64,
    pub u64StreamPosition: u64,
    pub u64PaddingFrames: u64,
    pub hnsQPCPosition: i64,
    pub f32FramesPerSecond: f32,
    pub Flag: AE_POSITION_FLAGS,
}
impl windows_core::TypeKind for AE_CURRENT_POSITION {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: u32,
    pub dwFramesDropped: u32,
}
impl windows_core::TypeKind for BITMAP_RENDERER_STATISTICS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BITMAP_RENDERER_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CHANNEL_DEF {
    pub name: [i8; 8],
    pub options: u32,
}
impl windows_core::TypeKind for CHANNEL_DEF {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CHANNEL_ENTRY_POINTS {
    pub cbSize: u32,
    pub protocolVersion: u32,
    pub pVirtualChannelInit: PVIRTUALCHANNELINIT,
    pub pVirtualChannelOpen: PVIRTUALCHANNELOPEN,
    pub pVirtualChannelClose: PVIRTUALCHANNELCLOSE,
    pub pVirtualChannelWrite: PVIRTUALCHANNELWRITE,
}
impl windows_core::TypeKind for CHANNEL_ENTRY_POINTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANNEL_ENTRY_POINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CHANNEL_PDU_HEADER {
    pub length: u32,
    pub flags: u32,
}
impl windows_core::TypeKind for CHANNEL_PDU_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for CHANNEL_PDU_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl windows_core::TypeKind for CLIENT_DISPLAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRODUCT_INFOA {
    pub CompanyName: [i8; 256],
    pub ProductID: [i8; 4],
}
impl windows_core::TypeKind for PRODUCT_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRODUCT_INFOW {
    pub CompanyName: [u16; 256],
    pub ProductID: [u16; 4],
}
impl windows_core::TypeKind for PRODUCT_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MONITOR_INFO {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub physicalWidth: u32,
    pub physicalHeight: u32,
    pub orientation: u32,
    pub primary: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for RFX_GFX_MONITOR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MONITOR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl windows_core::TypeKind for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
impl windows_core::TypeKind for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    pub channelHdr: RFX_GFX_MSG_HEADER,
}
impl windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
    pub ulBpp: u32,
    pub Reserved: u32,
}
impl windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub ulWidth: u32,
    pub ulHeight: u32,
}
impl windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub RedrawRect: RFX_GFX_RECT,
}
impl windows_core::TypeKind for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_DISCONNECT_NOTIFY {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub DisconnectReason: u32,
}
impl windows_core::TypeKind for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_HEADER {
    pub uMSGType: u16,
    pub cbSize: u16,
}
impl windows_core::TypeKind for RFX_GFX_MSG_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RFX_GFX_MSG_RDP_DATA {
    pub channelHdr: RFX_GFX_MSG_HEADER,
    pub rdpData: [u8; 1],
}
impl windows_core::TypeKind for RFX_GFX_MSG_RDP_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFX_GFX_RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl windows_core::TypeKind for RFX_GFX_RECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for RFX_GFX_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TSSD_ConnectionPoint {
    pub ServerAddressB: [u8; 16],
    pub AddressType: TSSD_AddrV46Type,
    pub PortNumber: u16,
    pub AddressScope: u32,
}
impl windows_core::TypeKind for TSSD_ConnectionPoint {
    type TypeKind = windows_core::CopyType;
}
impl Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TSUserExInterfaces: windows_core::GUID = windows_core::GUID::from_u128(0x0910dd01_df8c_11d1_ae27_00c04fa35813);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VM_NOTIFY_ENTRY {
    pub VmName: [u16; 128],
    pub VmHost: [u16; 128],
}
impl windows_core::TypeKind for VM_NOTIFY_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VM_NOTIFY_INFO {
    pub dwNumEntries: u32,
    pub ppVmEntries: *mut *mut VM_NOTIFY_ENTRY,
}
impl windows_core::TypeKind for VM_NOTIFY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VM_PATCH_INFO {
    pub dwNumEntries: u32,
    pub pVmNames: *mut windows_core::PWSTR,
}
impl windows_core::TypeKind for VM_PATCH_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_CONNECTION_SETTING {
    pub WRdsConnectionSettings1: WRDS_CONNECTION_SETTINGS_1,
}
impl windows_core::TypeKind for WRDS_CONNECTION_SETTING {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WRDS_CONNECTION_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WRDS_CONNECTION_SETTINGS_1 {
    pub fInheritInitialProgram: super::super::Foundation::BOOLEAN,
    pub fInheritColorDepth: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOLEAN,
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fResetBroken: super::super::Foundation::BOOLEAN,
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
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
impl windows_core::TypeKind for WRDS_CONNECTION_SETTINGS_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_LISTENER_SETTING {
    pub WRdsListenerSettings1: WRDS_LISTENER_SETTINGS_1,
}
impl windows_core::TypeKind for WRDS_LISTENER_SETTING {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WRDS_LISTENER_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WRDS_LISTENER_SETTINGS_1 {
    pub MaxProtocolListenerConnectionCount: u32,
    pub SecurityDescriptorSize: u32,
    pub pSecurityDescriptor: *mut u8,
}
impl windows_core::TypeKind for WRDS_LISTENER_SETTINGS_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WRDS_SETTING {
    pub WRdsSettings1: WRDS_SETTINGS_1,
}
impl windows_core::TypeKind for WRDS_SETTING {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WRDS_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    pub WRdsKeepAliveStartValue: super::super::Foundation::BOOLEAN,
    pub WRdsKeepAliveIntervalValue: u32,
}
impl windows_core::TypeKind for WRDS_SETTINGS_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WRDS_SETTINGS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSCLIENTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSCLIENTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSCONFIGINFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSCONFIGINFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSINFOA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTSINFOEXA {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTSINFOEXW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSINFOEX_LEVEL1_A {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSINFOEX_LEVEL1_W {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTSINFOEX_LEVEL_A {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTSINFOEX_LEVEL_W {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSINFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSLISTENERCONFIGA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSLISTENERCONFIGW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTSSBX_IP_ADDRESS {
    pub AddressFamily: WTSSBX_ADDRESS_FAMILY,
    pub Address: [u8; 16],
    pub PortNumber: u16,
    pub dwScope: u32,
}
impl windows_core::TypeKind for WTSSBX_IP_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTSSBX_MACHINE_CONNECT_INFO {
    pub wczMachineFQDN: [u16; 257],
    pub wczMachineNetBiosName: [u16; 17],
    pub dwNumOfIPAddr: u32,
    pub IPaddr: [WTSSBX_IP_ADDRESS; 12],
}
impl windows_core::TypeKind for WTSSBX_MACHINE_CONNECT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSSBX_MACHINE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTSSBX_SESSION_INFO {
    pub wszUserName: [u16; 105],
    pub wszDomainName: [u16; 257],
    pub ApplicationType: [u16; 257],
    pub dwSessionId: u32,
    pub CreateTime: super::super::Foundation::FILETIME,
    pub DisconnectTime: super::super::Foundation::FILETIME,
    pub SessionState: WTSSBX_SESSION_STATE,
}
impl windows_core::TypeKind for WTSSBX_SESSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTSSESSION_NOTIFICATION {
    pub cbSize: u32,
    pub dwSessionId: u32,
}
impl windows_core::TypeKind for WTSSESSION_NOTIFICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSSESSION_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSUSERCONFIGA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTSUSERCONFIGW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_CACHE_STATS {
    pub Specific: u32,
    pub Data: WTS_CACHE_STATS_UN,
    pub ProtocolType: u16,
    pub Length: u16,
}
impl windows_core::TypeKind for WTS_CACHE_STATS {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTS_CACHE_STATS_UN {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_CLIENT_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl windows_core::TypeKind for WTS_CLIENT_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_CLIENT_DATA {
    pub fDisableCtrlAltDel: super::super::Foundation::BOOLEAN,
    pub fDoubleClickDetect: super::super::Foundation::BOOLEAN,
    pub fEnableWindowsKey: super::super::Foundation::BOOLEAN,
    pub fHideTitleBar: super::super::Foundation::BOOLEAN,
    pub fInheritAutoLogon: super::super::Foundation::BOOL,
    pub fPromptForPassword: super::super::Foundation::BOOLEAN,
    pub fUsingSavedCreds: super::super::Foundation::BOOLEAN,
    pub Domain: [u16; 256],
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub fPasswordIsScPin: super::super::Foundation::BOOLEAN,
    pub fInheritInitialProgram: super::super::Foundation::BOOL,
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub fMaximizeShell: super::super::Foundation::BOOLEAN,
    pub EncryptionLevel: u8,
    pub PerformanceFlags: u32,
    pub ProtocolName: [u16; 9],
    pub ProtocolType: u16,
    pub fInheritColorDepth: super::super::Foundation::BOOL,
    pub HRes: u16,
    pub VRes: u16,
    pub ColorDepth: u16,
    pub DisplayDriverName: [u16; 9],
    pub DisplayDeviceName: [u16; 20],
    pub fMouse: super::super::Foundation::BOOLEAN,
    pub KeyboardLayout: u32,
    pub KeyboardType: u32,
    pub KeyboardSubType: u32,
    pub KeyboardFunctionKey: u32,
    pub imeFileName: [u16; 33],
    pub ActiveInputLocale: u32,
    pub fNoAudioPlayback: super::super::Foundation::BOOLEAN,
    pub fRemoteConsoleAudio: super::super::Foundation::BOOLEAN,
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
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNP: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for WTS_CLIENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_CLIENT_DISPLAY {
    pub HorizontalResolution: u32,
    pub VerticalResolution: u32,
    pub ColorDepth: u32,
}
impl windows_core::TypeKind for WTS_CLIENT_DISPLAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_DISPLAY_IOCTL {
    pub pDisplayIOCtlData: [u8; 256],
    pub cbDisplayIOCtlData: u32,
}
impl windows_core::TypeKind for WTS_DISPLAY_IOCTL {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_LICENSE_CAPABILITIES {
    pub KeyExchangeAlg: u32,
    pub ProtocolVer: u32,
    pub fAuthenticateServer: super::super::Foundation::BOOL,
    pub CertType: WTS_CERT_TYPE,
    pub cbClientName: u32,
    pub rgbClientName: [u8; 42],
}
impl windows_core::TypeKind for WTS_LICENSE_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_POLICY_DATA {
    pub fDisableEncryption: super::super::Foundation::BOOLEAN,
    pub fDisableAutoReconnect: super::super::Foundation::BOOLEAN,
    pub ColorDepth: u32,
    pub MinEncryptionLevel: u8,
    pub fDisableCpm: super::super::Foundation::BOOLEAN,
    pub fDisableCdm: super::super::Foundation::BOOLEAN,
    pub fDisableCcm: super::super::Foundation::BOOLEAN,
    pub fDisableLPT: super::super::Foundation::BOOLEAN,
    pub fDisableClip: super::super::Foundation::BOOLEAN,
    pub fDisablePNPRedir: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for WTS_POLICY_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_POLICY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_PROCESS_INFOA {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for WTS_PROCESS_INFOA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_PROCESS_INFOW {
    pub SessionId: u32,
    pub ProcessId: u32,
    pub pProcessName: windows_core::PWSTR,
    pub pUserSid: super::super::Security::PSID,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for WTS_PROCESS_INFOW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for WTS_PROCESS_INFO_EXA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for WTS_PROCESS_INFO_EXW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_PROPERTY_VALUE {
    pub Type: u16,
    pub u: WTS_PROPERTY_VALUE_0,
}
impl windows_core::TypeKind for WTS_PROPERTY_VALUE {
    type TypeKind = windows_core::CopyType;
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
    pub strVal: WTS_PROPERTY_VALUE_0_1,
    pub bVal: WTS_PROPERTY_VALUE_0_0,
    pub guidVal: windows_core::GUID,
}
impl windows_core::TypeKind for WTS_PROPERTY_VALUE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROPERTY_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_PROPERTY_VALUE_0_0 {
    pub size: u32,
    pub pbVal: windows_core::PSTR,
}
impl windows_core::TypeKind for WTS_PROPERTY_VALUE_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROPERTY_VALUE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_PROPERTY_VALUE_0_1 {
    pub size: u32,
    pub pstrVal: windows_core::PWSTR,
}
impl windows_core::TypeKind for WTS_PROPERTY_VALUE_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROPERTY_VALUE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_PROTOCOL_CACHE {
    pub CacheReads: u32,
    pub CacheHits: u32,
}
impl windows_core::TypeKind for WTS_PROTOCOL_CACHE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROTOCOL_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTS_PROTOCOL_COUNTERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
impl windows_core::TypeKind for WTS_PROTOCOL_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SERVER_INFOA {
    pub pServerName: windows_core::PSTR,
}
impl windows_core::TypeKind for WTS_SERVER_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SERVER_INFOW {
    pub pServerName: windows_core::PWSTR,
}
impl windows_core::TypeKind for WTS_SERVER_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SERVICE_STATE {
    pub RcmServiceState: WTS_RCM_SERVICE_STATE,
    pub RcmDrainState: WTS_RCM_DRAIN_STATE,
}
impl windows_core::TypeKind for WTS_SERVICE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SERVICE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SESSION_ADDRESS {
    pub AddressFamily: u32,
    pub Address: [u8; 20],
}
impl windows_core::TypeKind for WTS_SESSION_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SESSION_ID {
    pub SessionUniqueGuid: windows_core::GUID,
    pub SessionId: u32,
}
impl windows_core::TypeKind for WTS_SESSION_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SESSION_INFOA {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl windows_core::TypeKind for WTS_SESSION_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SESSION_INFOW {
    pub SessionId: u32,
    pub pWinStationName: windows_core::PWSTR,
    pub State: WTS_CONNECTSTATE_CLASS,
}
impl windows_core::TypeKind for WTS_SESSION_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTS_SESSION_INFO_1A {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTS_SESSION_INFO_1W {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SMALL_RECT {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl windows_core::TypeKind for WTS_SMALL_RECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SMALL_RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WTS_SOCKADDR {
    pub sin_family: u16,
    pub u: WTS_SOCKADDR_0,
}
impl windows_core::TypeKind for WTS_SOCKADDR {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for WTS_SOCKADDR_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SOCKADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SOCKADDR_0_0 {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl windows_core::TypeKind for WTS_SOCKADDR_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SOCKADDR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_SOCKADDR_0_1 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl windows_core::TypeKind for WTS_SOCKADDR_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SOCKADDR_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for WTS_SYSTEMTIME {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_SYSTEMTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: WTS_SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: WTS_SYSTEMTIME,
    pub DaylightBias: i32,
}
impl windows_core::TypeKind for WTS_TIME_ZONE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_USER_CREDENTIAL {
    pub UserName: [u16; 256],
    pub Password: [u16; 256],
    pub Domain: [u16; 256],
}
impl windows_core::TypeKind for WTS_USER_CREDENTIAL {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_USER_DATA {
    pub WorkDirectory: [u16; 257],
    pub InitialProgram: [u16; 257],
    pub UserTimeZone: WTS_TIME_ZONE_INFORMATION,
}
impl windows_core::TypeKind for WTS_USER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_VALIDATION_INFORMATIONA {
    pub ProductInfo: PRODUCT_INFOA,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl windows_core::TypeKind for WTS_VALIDATION_INFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WTS_VALIDATION_INFORMATIONW {
    pub ProductInfo: PRODUCT_INFOW,
    pub License: [u8; 16384],
    pub LicenseLength: u32,
    pub HardwareID: [u8; 20],
    pub HardwareIDLength: u32,
}
impl windows_core::TypeKind for WTS_VALIDATION_INFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Workspace: windows_core::GUID = windows_core::GUID::from_u128(0x4f1dfca6_3aad_48e1_8406_4bc21a501d7c);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for pluginResource {
    type TypeKind = windows_core::CopyType;
}
impl Default for pluginResource {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct pluginResource2 {
    pub resourceV1: pluginResource,
    pub pceFileAssocListSize: u32,
    pub fileAssocList: *mut pluginResource2FileAssociation,
    pub securityDescriptor: windows_core::PWSTR,
    pub pceFolderListSize: u32,
    pub folderList: *mut *mut u16,
}
impl windows_core::TypeKind for pluginResource2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for pluginResource2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct pluginResource2FileAssociation {
    pub extName: [u16; 256],
    pub primaryHandler: u8,
    pub pceIconSize: u32,
    pub iconContents: *mut u8,
}
impl windows_core::TypeKind for pluginResource2FileAssociation {
    type TypeKind = windows_core::CopyType;
}
impl Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCHANNEL_INIT_EVENT_FN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, event: u32, pdata: *mut core::ffi::c_void, datalength: u32)>;
pub type PCHANNEL_OPEN_EVENT_FN = Option<unsafe extern "system" fn(openhandle: u32, event: u32, pdata: *mut core::ffi::c_void, datalength: u32, totallength: u32, dataflags: u32)>;
pub type PVIRTUALCHANNELCLOSE = Option<unsafe extern "system" fn(openhandle: u32) -> u32>;
pub type PVIRTUALCHANNELENTRY = Option<unsafe extern "system" fn(pentrypoints: *mut CHANNEL_ENTRY_POINTS) -> super::super::Foundation::BOOL>;
pub type PVIRTUALCHANNELINIT = Option<unsafe extern "system" fn(ppinithandle: *mut *mut core::ffi::c_void, pchannel: *mut CHANNEL_DEF, channelcount: i32, versionrequested: u32, pchanneliniteventproc: PCHANNEL_INIT_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELOPEN = Option<unsafe extern "system" fn(pinithandle: *mut core::ffi::c_void, popenhandle: *mut u32, pchannelname: windows_core::PCSTR, pchannelopeneventproc: PCHANNEL_OPEN_EVENT_FN) -> u32>;
pub type PVIRTUALCHANNELWRITE = Option<unsafe extern "system" fn(openhandle: u32, pdata: *mut core::ffi::c_void, datalength: u32, puserdata: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
