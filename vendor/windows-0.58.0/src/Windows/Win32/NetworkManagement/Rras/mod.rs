#[inline]
pub unsafe fn MgmAddGroupMembershipEntry<P0>(hprotocol: P0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmAddGroupMembershipEntry(hprotocol : super::super::Foundation:: HANDLE, dwsourceaddr : u32, dwsourcemask : u32, dwgroupaddr : u32, dwgroupmask : u32, dwifindex : u32, dwifnexthopipaddr : u32, dwflags : u32) -> u32);
    MgmAddGroupMembershipEntry(hprotocol.param().abi(), dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags)
}
#[inline]
pub unsafe fn MgmDeRegisterMProtocol<P0>(hprotocol: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmDeRegisterMProtocol(hprotocol : super::super::Foundation:: HANDLE) -> u32);
    MgmDeRegisterMProtocol(hprotocol.param().abi())
}
#[inline]
pub unsafe fn MgmDeleteGroupMembershipEntry<P0>(hprotocol: P0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmDeleteGroupMembershipEntry(hprotocol : super::super::Foundation:: HANDLE, dwsourceaddr : u32, dwsourcemask : u32, dwgroupaddr : u32, dwgroupmask : u32, dwifindex : u32, dwifnexthopipaddr : u32, dwflags : u32) -> u32);
    MgmDeleteGroupMembershipEntry(hprotocol.param().abi(), dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags)
}
#[inline]
pub unsafe fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetFirstMfe(pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    MgmGetFirstMfe(pdwbuffersize, pbbuffer, pdwnumentries)
}
#[inline]
pub unsafe fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetFirstMfeStats(pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32, dwflags : u32) -> u32);
    MgmGetFirstMfeStats(pdwbuffersize, pbbuffer, pdwnumentries, dwflags)
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetMfe(pimm : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8) -> u32);
    MgmGetMfe(pimm, pdwbuffersize, pbbuffer)
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetMfeStats(pimm : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, dwflags : u32) -> u32);
    MgmGetMfeStats(pimm, pdwbuffersize, pbbuffer, dwflags)
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetNextMfe(pimmstart : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    MgmGetNextMfe(pimmstart, pdwbuffersize, pbbuffer, pdwnumentries)
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetNextMfeStats(pimmstart : *mut super::IpHelper:: MIB_IPMCAST_MFE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32, dwflags : u32) -> u32);
    MgmGetNextMfeStats(pimmstart, pdwbuffersize, pbbuffer, pdwnumentries, dwflags)
}
#[inline]
pub unsafe fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmGetProtocolOnInterface(dwifindex : u32, dwifnexthopaddr : u32, pdwifprotocolid : *mut u32, pdwifcomponentid : *mut u32) -> u32);
    MgmGetProtocolOnInterface(dwifindex, dwifnexthopaddr, pdwifprotocolid, pdwifcomponentid)
}
#[inline]
pub unsafe fn MgmGroupEnumerationEnd<P0>(henum: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmGroupEnumerationEnd(henum : super::super::Foundation:: HANDLE) -> u32);
    MgmGroupEnumerationEnd(henum.param().abi())
}
#[inline]
pub unsafe fn MgmGroupEnumerationGetNext<P0>(henum: P0, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmGroupEnumerationGetNext(henum : super::super::Foundation:: HANDLE, pdwbuffersize : *mut u32, pbbuffer : *mut u8, pdwnumentries : *mut u32) -> u32);
    MgmGroupEnumerationGetNext(henum.param().abi(), pdwbuffersize, pbbuffer, pdwnumentries)
}
#[inline]
pub unsafe fn MgmGroupEnumerationStart<P0>(hprotocol: P0, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmGroupEnumerationStart(hprotocol : super::super::Foundation:: HANDLE, metenumtype : MGM_ENUM_TYPES, phenumhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    MgmGroupEnumerationStart(hprotocol.param().abi(), metenumtype, phenumhandle)
}
#[inline]
pub unsafe fn MgmRegisterMProtocol(prpiinfo: *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn MgmRegisterMProtocol(prpiinfo : *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid : u32, dwcomponentid : u32, phprotocol : *mut super::super::Foundation:: HANDLE) -> u32);
    MgmRegisterMProtocol(prpiinfo, dwprotocolid, dwcomponentid, phprotocol)
}
#[inline]
pub unsafe fn MgmReleaseInterfaceOwnership<P0>(hprotocol: P0, dwifindex: u32, dwifnexthopaddr: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmReleaseInterfaceOwnership(hprotocol : super::super::Foundation:: HANDLE, dwifindex : u32, dwifnexthopaddr : u32) -> u32);
    MgmReleaseInterfaceOwnership(hprotocol.param().abi(), dwifindex, dwifnexthopaddr)
}
#[inline]
pub unsafe fn MgmTakeInterfaceOwnership<P0>(hprotocol: P0, dwifindex: u32, dwifnexthopaddr: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn MgmTakeInterfaceOwnership(hprotocol : super::super::Foundation:: HANDLE, dwifindex : u32, dwifnexthopaddr : u32) -> u32);
    MgmTakeInterfaceOwnership(hprotocol.param().abi(), dwifindex, dwifnexthopaddr)
}
#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    MprAdminBufferFree(pbuffer)
}
#[inline]
pub unsafe fn MprAdminConnectionClearStats<P0>(hrasserver: isize, hrasconnection: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionClearStats(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE) -> u32);
    MprAdminConnectionClearStats(hrasserver, hrasconnection.param().abi())
}
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionEnum(hrasserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    MprAdminConnectionEnum(hrasserver, dwlevel, lplpbbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionEnumEx(hrasserver : isize, pobjectheader : *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, pprasconn : *mut *mut RAS_CONNECTION_EX, lpdwresumehandle : *const u32) -> u32);
    MprAdminConnectionEnumEx(hrasserver, pobjectheader, dwpreferedmaxlen, lpdwentriesread, lpdwtotalentries, pprasconn, lpdwresumehandle)
}
#[inline]
pub unsafe fn MprAdminConnectionGetInfo<P0>(hrasserver: isize, dwlevel: u32, hrasconnection: P0, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfo(hrasserver : isize, dwlevel : u32, hrasconnection : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8) -> u32);
    MprAdminConnectionGetInfo(hrasserver, dwlevel, hrasconnection.param().abi(), lplpbbuffer)
}
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx<P0>(hrasserver: isize, hrasconnection: P0, prasconnection: *mut RAS_CONNECTION_EX) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionGetInfoEx(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE, prasconnection : *mut RAS_CONNECTION_EX) -> u32);
    MprAdminConnectionGetInfoEx(hrasserver, hrasconnection.param().abi(), prasconnection)
}
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine<P0, P1, P2>(hrasserver: P0, hrasconnection: P1, fisipaddress: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminConnectionRemoveQuarantine(hrasserver : super::super::Foundation:: HANDLE, hrasconnection : super::super::Foundation:: HANDLE, fisipaddress : super::super::Foundation:: BOOL) -> u32);
    MprAdminConnectionRemoveQuarantine(hrasserver.param().abi(), hrasconnection.param().abi(), fisipaddress.param().abi())
}
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification<P0>(hmprserver: isize, heventnotification: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminDeregisterConnectionNotification(hmprserver : isize, heventnotification : super::super::Foundation:: HANDLE) -> u32);
    MprAdminDeregisterConnectionNotification(hmprserver, heventnotification.param().abi())
}
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminDeviceEnum(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, lpdwtotalentries : *mut u32) -> u32);
    MprAdminDeviceEnum(hmprserver, dwlevel, lplpbbuffer, lpdwtotalentries)
}
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<P0, P1, P2>(pszdomain: P0, pszmachine: P1, benable: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminEstablishDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, benable : super::super::Foundation:: BOOL) -> u32);
    MprAdminEstablishDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), benable.param().abi())
}
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut windows_core::PWSTR) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminGetErrorString(dwerror : u32, lplpwserrorstring : *mut windows_core::PWSTR) -> u32);
    MprAdminGetErrorString(dwerror, lplpwserrorstring)
}
#[inline]
pub unsafe fn MprAdminGetPDCServer<P0, P1>(lpszdomain: P0, lpszserver: P1, lpszpdcserver: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminGetPDCServer(lpszdomain : windows_core::PCWSTR, lpszserver : windows_core::PCWSTR, lpszpdcserver : windows_core::PWSTR) -> u32);
    MprAdminGetPDCServer(lpszdomain.param().abi(), lpszserver.param().abi(), core::mem::transmute(lpszpdcserver))
}
#[inline]
pub unsafe fn MprAdminInterfaceConnect<P0, P1, P2>(hmprserver: isize, hinterface: P0, hevent: P1, fsynchronous: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceConnect(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, hevent : super::super::Foundation:: HANDLE, fsynchronous : super::super::Foundation:: BOOL) -> u32);
    MprAdminInterfaceConnect(hmprserver, hinterface.param().abi(), hevent.param().abi(), fsynchronous.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceCreate(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8, phinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    MprAdminInterfaceCreate(hmprserver, dwlevel, lpbbuffer, phinterface)
}
#[inline]
pub unsafe fn MprAdminInterfaceDelete<P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceDelete(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    MprAdminInterfaceDelete(hmprserver, hinterface.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo<P0>(hmprserver: isize, hinterface: P0, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwindex : u32, dwlevel : u32, lplpbuffer : *mut *mut u8) -> u32);
    MprAdminInterfaceDeviceGetInfo(hmprserver, hinterface.param().abi(), dwindex, dwlevel, lplpbuffer)
}
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo<P0>(hmprserver: isize, hinterface: P0, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceDeviceSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwindex : u32, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminInterfaceDeviceSetInfo(hmprserver, hinterface.param().abi(), dwindex, dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect<P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceDisconnect(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    MprAdminInterfaceDisconnect(hmprserver, hinterface.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceEnum(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    MprAdminInterfaceEnum(hmprserver, dwlevel, lplpbbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<P0, P1>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: windows_core::PWSTR, lpwspassword: windows_core::PWSTR, lpwsdomainname: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PWSTR, lpwspassword : windows_core::PWSTR, lpwsdomainname : windows_core::PWSTR) -> u32);
    MprAdminInterfaceGetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), core::mem::transmute(lpwsusername), core::mem::transmute(lpwspassword), core::mem::transmute(lpwsdomainname))
}
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx<P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCredentialsEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    MprAdminInterfaceGetCredentialsEx(hmprserver, hinterface.param().abi(), dwlevel, lplpbbuffer)
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx<P0>(hmprserver: isize, hinterface: P0, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceGetCustomInfoEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    MprAdminInterfaceGetCustomInfoEx(hmprserver, hinterface.param().abi(), pcustominfo)
}
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<P0, P1>(hmprserver: isize, lpwsinterfacename: P0, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceGetHandle(hmprserver : isize, lpwsinterfacename : windows_core::PCWSTR, phinterface : *mut super::super::Foundation:: HANDLE, fincludeclientinterfaces : super::super::Foundation:: BOOL) -> u32);
    MprAdminInterfaceGetHandle(hmprserver, lpwsinterfacename.param().abi(), phinterface, fincludeclientinterfaces.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo<P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *const *const u8) -> u32);
    MprAdminInterfaceGetInfo(hmprserver, hinterface.param().abi(), dwlevel, lplpbbuffer)
}
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult<P0>(hmprserver: isize, hinterface: P0, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceQueryUpdateResult(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwprotocolid : u32, lpdwupdateresult : *mut u32) -> u32);
    MprAdminInterfaceQueryUpdateResult(hmprserver, hinterface.param().abi(), dwprotocolid, lpdwupdateresult)
}
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<P0, P1, P2, P3, P4>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: P2, lpwsdomainname: P3, lpwspassword: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentials(lpwsserver : windows_core::PCWSTR, lpwsinterfacename : windows_core::PCWSTR, lpwsusername : windows_core::PCWSTR, lpwsdomainname : windows_core::PCWSTR, lpwspassword : windows_core::PCWSTR) -> u32);
    MprAdminInterfaceSetCredentials(lpwsserver.param().abi(), lpwsinterfacename.param().abi(), lpwsusername.param().abi(), lpwsdomainname.param().abi(), lpwspassword.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx<P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCredentialsEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminInterfaceSetCredentialsEx(hmprserver, hinterface.param().abi(), dwlevel, lpbbuffer)
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx<P0>(hmprserver: isize, hinterface: P0, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceSetCustomInfoEx(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    MprAdminInterfaceSetCustomInfoEx(hmprserver, hinterface.param().abi(), pcustominfo)
}
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo<P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminInterfaceSetInfo(hmprserver, hinterface.param().abi(), dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd<P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportAdd(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    MprAdminInterfaceTransportAdd(hmprserver, hinterface.param().abi(), dwtransportid, pinterfaceinfo, dwinterfaceinfosize)
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo<P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportGetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, ppinterfaceinfo : *mut *mut u8, lpdwinterfaceinfosize : *mut u32) -> u32);
    MprAdminInterfaceTransportGetInfo(hmprserver, hinterface.param().abi(), dwtransportid, ppinterfaceinfo, core::mem::transmute(lpdwinterfaceinfosize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove<P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportRemove(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32) -> u32);
    MprAdminInterfaceTransportRemove(hmprserver, hinterface.param().abi(), dwtransportid)
}
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo<P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceTransportSetInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    MprAdminInterfaceTransportSetInfo(hmprserver, hinterface.param().abi(), dwtransportid, pinterfaceinfo, dwinterfaceinfosize)
}
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo<P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE) -> u32);
    MprAdminInterfaceUpdatePhonebookInfo(hmprserver, hinterface.param().abi())
}
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes<P0, P1>(hmprserver: isize, hinterface: P0, dwprotocolid: u32, hevent: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminInterfaceUpdateRoutes(hmprserver : isize, hinterface : super::super::Foundation:: HANDLE, dwprotocolid : u32, hevent : super::super::Foundation:: HANDLE) -> u32);
    MprAdminInterfaceUpdateRoutes(hmprserver, hinterface.param().abi(), dwprotocolid, hevent.param().abi())
}
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<P0, P1>(pszdomain: P0, pszmachine: P1, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminIsDomainRasServer(pszdomain : windows_core::PCWSTR, pszmachine : windows_core::PCWSTR, pbisrasserver : *mut super::super::Foundation:: BOOL) -> u32);
    MprAdminIsDomainRasServer(pszdomain.param().abi(), pszmachine.param().abi(), pbisrasserver)
}
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<P0>(lpwsservername: P0, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminIsServiceInitialized(lpwsservername : windows_core::PCWSTR, fisserviceinitialized : *const super::super::Foundation:: BOOL) -> u32);
    MprAdminIsServiceInitialized(lpwsservername.param().abi(), fisserviceinitialized)
}
#[inline]
pub unsafe fn MprAdminIsServiceRunning<P0>(lpwsservername: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminIsServiceRunning(lpwsservername : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    MprAdminIsServiceRunning(lpwsservername.param().abi())
}
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    MprAdminMIBBufferFree(pbuffer)
}
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntryCreate(hmibserver : isize, dwpid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    MprAdminMIBEntryCreate(hmibserver, dwpid, dwroutingpid, lpentry, dwentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntryDelete(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    MprAdminMIBEntryDelete(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntryGet(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    MprAdminMIBEntryGet(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry, lpoutentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetFirst(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    MprAdminMIBEntryGetFirst(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry, lpoutentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntryGetNext(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpinentry : *const core::ffi::c_void, dwinentrysize : u32, lplpoutentry : *mut *mut core::ffi::c_void, lpoutentrysize : *mut u32) -> u32);
    MprAdminMIBEntryGetNext(hmibserver, dwprotocolid, dwroutingpid, lpinentry, dwinentrysize, lplpoutentry, lpoutentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const core::ffi::c_void, dwentrysize: u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBEntrySet(hmibserver : isize, dwprotocolid : u32, dwroutingpid : u32, lpentry : *const core::ffi::c_void, dwentrysize : u32) -> u32);
    MprAdminMIBEntrySet(hmibserver, dwprotocolid, dwroutingpid, lpentry, dwentrysize)
}
#[inline]
pub unsafe fn MprAdminMIBServerConnect<P0>(lpwsservername: P0, phmibserver: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBServerConnect(lpwsservername : windows_core::PCWSTR, phmibserver : *mut isize) -> u32);
    MprAdminMIBServerConnect(lpwsservername.param().abi(), phmibserver)
}
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: isize) {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminMIBServerDisconnect(hmibserver : isize));
    MprAdminMIBServerDisconnect(hmibserver)
}
#[inline]
pub unsafe fn MprAdminPortClearStats<P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminPortClearStats(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    MprAdminPortClearStats(hrasserver, hport.param().abi())
}
#[inline]
pub unsafe fn MprAdminPortDisconnect<P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminPortDisconnect(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    MprAdminPortDisconnect(hrasserver, hport.param().abi())
}
#[inline]
pub unsafe fn MprAdminPortEnum<P0>(hrasserver: isize, dwlevel: u32, hrasconnection: P0, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*const u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminPortEnum(hrasserver : isize, dwlevel : u32, hrasconnection : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *const u32) -> u32);
    MprAdminPortEnum(hrasserver, dwlevel, hrasconnection.param().abi(), lplpbbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn MprAdminPortGetInfo<P0>(hrasserver: isize, dwlevel: u32, hport: P0, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminPortGetInfo(hrasserver : isize, dwlevel : u32, hport : super::super::Foundation:: HANDLE, lplpbbuffer : *mut *mut u8) -> u32);
    MprAdminPortGetInfo(hrasserver, dwlevel, hport.param().abi(), lplpbbuffer)
}
#[inline]
pub unsafe fn MprAdminPortReset<P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminPortReset(hrasserver : isize, hport : super::super::Foundation:: HANDLE) -> u32);
    MprAdminPortReset(hrasserver, hport.param().abi())
}
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification<P0>(hmprserver: isize, heventnotification: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminRegisterConnectionNotification(hmprserver : isize, heventnotification : super::super::Foundation:: HANDLE) -> u32);
    MprAdminRegisterConnectionNotification(hmprserver, heventnotification.param().abi())
}
#[inline]
pub unsafe fn MprAdminSendUserMessage<P0, P1>(hmprserver: isize, hconnection: P0, lpwszmessage: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminSendUserMessage(hmprserver : isize, hconnection : super::super::Foundation:: HANDLE, lpwszmessage : windows_core::PCWSTR) -> u32);
    MprAdminSendUserMessage(hmprserver, hconnection.param().abi(), lpwszmessage.param().abi())
}
#[inline]
pub unsafe fn MprAdminServerConnect<P0>(lpwsservername: P0, phmprserver: *mut isize) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerConnect(lpwsservername : windows_core::PCWSTR, phmprserver : *mut isize) -> u32);
    MprAdminServerConnect(lpwsservername.param().abi(), phmprserver)
}
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: isize) {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerDisconnect(hmprserver : isize));
    MprAdminServerDisconnect(hmprserver)
}
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerGetCredentials(hmprserver : isize, dwlevel : u32, lplpbbuffer : *const *const u8) -> u32);
    MprAdminServerGetCredentials(hmprserver, dwlevel, lplpbbuffer)
}
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerGetInfo(hmprserver : isize, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    MprAdminServerGetInfo(hmprserver, dwlevel, lplpbbuffer)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerGetInfoEx(hmprserver : isize, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    MprAdminServerGetInfoEx(hmprserver, pserverinfo)
}
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerSetCredentials(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminServerSetCredentials(hmprserver, dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerSetInfo(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminServerSetInfo(hmprserver, dwlevel, lpbbuffer)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminServerSetInfoEx(hmprserver : isize, pserverinfo : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    MprAdminServerSetInfoEx(hmprserver, pserverinfo)
}
#[inline]
pub unsafe fn MprAdminTransportCreate<P0, P1>(hmprserver: isize, dwtransportid: u32, lpwstransportname: P0, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: u32, lpwsdllpath: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminTransportCreate(hmprserver : isize, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    MprAdminTransportCreate(hmprserver, dwtransportid, lpwstransportname.param().abi(), pglobalinfo, dwglobalinfosize, core::mem::transmute(pclientinterfaceinfo.unwrap_or(std::ptr::null())), dwclientinterfaceinfosize, lpwsdllpath.param().abi())
}
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: Option<*mut *mut u8>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut *mut u8>, lpdwclientinterfaceinfosize: Option<*mut u32>) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminTransportGetInfo(hmprserver : isize, dwtransportid : u32, ppglobalinfo : *mut *mut u8, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut *mut u8, lpdwclientinterfaceinfosize : *mut u32) -> u32);
    MprAdminTransportGetInfo(hmprserver, dwtransportid, core::mem::transmute(ppglobalinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdwglobalinfosize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppclientinterfaceinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdwclientinterfaceinfosize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: Option<*const u8>, dwglobalinfosize: u32, pclientinterfaceinfo: Option<*const u8>, dwclientinterfaceinfosize: u32) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprAdminTransportSetInfo(hmprserver : isize, dwtransportid : u32, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32) -> u32);
    MprAdminTransportSetInfo(hmprserver, dwtransportid, core::mem::transmute(pglobalinfo.unwrap_or(std::ptr::null())), dwglobalinfosize, core::mem::transmute(pclientinterfaceinfo.unwrap_or(std::ptr::null())), dwclientinterfaceinfosize)
}
#[inline]
pub unsafe fn MprAdminUpdateConnection<P0>(hrasserver: isize, hrasconnection: P0, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminUpdateConnection(hrasserver : isize, hrasconnection : super::super::Foundation:: HANDLE, prasupdateconnection : *const RAS_UPDATE_CONNECTION) -> u32);
    MprAdminUpdateConnection(hrasserver, hrasconnection.param().abi(), prasupdateconnection)
}
#[inline]
pub unsafe fn MprAdminUserGetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminUserGetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *mut u8) -> u32);
    MprAdminUserGetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprAdminUserSetInfo<P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprAdminUserSetInfo(lpszserver : windows_core::PCWSTR, lpszuser : windows_core::PCWSTR, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprAdminUserSetInfo(lpszserver.param().abi(), lpszuser.param().abi(), dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprConfigBufferFree(pbuffer : *const core::ffi::c_void) -> u32);
    MprConfigBufferFree(pbuffer)
}
#[inline]
pub unsafe fn MprConfigFilterGetInfo<P0>(hmprconfig: P0, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigFilterGetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *mut u8) -> u32);
    MprConfigFilterGetInfo(hmprconfig.param().abi(), dwlevel, dwtransportid, lpbuffer)
}
#[inline]
pub unsafe fn MprConfigFilterSetInfo<P0>(hmprconfig: P0, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigFilterSetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, dwtransportid : u32, lpbuffer : *const u8) -> u32);
    MprConfigFilterSetInfo(hmprconfig.param().abi(), dwlevel, dwtransportid, lpbuffer)
}
#[inline]
pub unsafe fn MprConfigGetFriendlyName<P0, P1>(hmprconfig: P0, pszguidname: P1, pszbuffer: windows_core::PWSTR, dwbuffersize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigGetFriendlyName(hmprconfig : super::super::Foundation:: HANDLE, pszguidname : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, dwbuffersize : u32) -> u32);
    MprConfigGetFriendlyName(hmprconfig.param().abi(), pszguidname.param().abi(), core::mem::transmute(pszbuffer), dwbuffersize)
}
#[inline]
pub unsafe fn MprConfigGetGuidName<P0, P1>(hmprconfig: P0, pszfriendlyname: P1, pszbuffer: windows_core::PWSTR, dwbuffersize: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigGetGuidName(hmprconfig : super::super::Foundation:: HANDLE, pszfriendlyname : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, dwbuffersize : u32) -> u32);
    MprConfigGetGuidName(hmprconfig.param().abi(), pszfriendlyname.param().abi(), core::mem::transmute(pszbuffer), dwbuffersize)
}
#[inline]
pub unsafe fn MprConfigInterfaceCreate<P0>(hmprconfig: P0, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceCreate(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8, phrouterinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceCreate(hmprconfig.param().abi(), dwlevel, lpbbuffer, phrouterinterface)
}
#[inline]
pub unsafe fn MprConfigInterfaceDelete<P0, P1>(hmprconfig: P0, hrouterinterface: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceDelete(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceDelete(hmprconfig.param().abi(), hrouterinterface.param().abi())
}
#[inline]
pub unsafe fn MprConfigInterfaceEnum<P0>(hmprconfig: P0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceEnum(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    MprConfigInterfaceEnum(hmprconfig.param().abi(), dwlevel, lplpbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null_mut())))
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx<P0, P1>(hmprconfig: P0, hrouterinterface: P1, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceGetCustomInfoEx(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, pcustominfo : *mut MPR_IF_CUSTOMINFOEX2) -> u32);
    MprConfigInterfaceGetCustomInfoEx(hmprconfig.param().abi(), hrouterinterface.param().abi(), pcustominfo)
}
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<P0, P1>(hmprconfig: P0, lpwsinterfacename: P1, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceGetHandle(hmprconfig : super::super::Foundation:: HANDLE, lpwsinterfacename : windows_core::PCWSTR, phrouterinterface : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceGetHandle(hmprconfig.param().abi(), lpwsinterfacename.param().abi(), phrouterinterface)
}
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo<P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, lpdwbuffersize : *mut u32) -> u32);
    MprConfigInterfaceGetInfo(hmprconfig.param().abi(), hrouterinterface.param().abi(), dwlevel, lplpbuffer, lpdwbuffersize)
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx<P0, P1>(hmprconfig: P0, hrouterinterface: P1, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceSetCustomInfoEx(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, pcustominfo : *const MPR_IF_CUSTOMINFOEX2) -> u32);
    MprConfigInterfaceSetCustomInfoEx(hmprconfig.param().abi(), hrouterinterface.param().abi(), pcustominfo)
}
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo<P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprConfigInterfaceSetInfo(hmprconfig.param().abi(), hrouterinterface.param().abi(), dwlevel, lpbbuffer)
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, dwtransportid: u32, lpwstransportname: P2, pinterfaceinfo: &[u8], phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportAdd(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32, phrouteriftransport : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceTransportAdd(hmprconfig.param().abi(), hrouterinterface.param().abi(), dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pinterfaceinfo.as_ptr()), pinterfaceinfo.len().try_into().unwrap(), phrouteriftransport)
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum<P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportEnum(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    MprConfigInterfaceTransportEnum(hmprconfig.param().abi(), hrouterinterface.param().abi(), dwlevel, lplpbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle<P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetHandle(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, dwtransportid : u32, phrouteriftransport : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceTransportGetHandle(hmprconfig.param().abi(), hrouterinterface.param().abi(), dwtransportid, phrouteriftransport)
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo<P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2, ppinterfaceinfo: Option<*mut *mut u8>, lpdwinterfaceinfosize: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE, ppinterfaceinfo : *mut *mut u8, lpdwinterfaceinfosize : *mut u32) -> u32);
    MprConfigInterfaceTransportGetInfo(hmprconfig.param().abi(), hrouterinterface.param().abi(), hrouteriftransport.param().abi(), core::mem::transmute(ppinterfaceinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdwinterfaceinfosize.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove<P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportRemove(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE) -> u32);
    MprConfigInterfaceTransportRemove(hmprconfig.param().abi(), hrouterinterface.param().abi(), hrouteriftransport.param().abi())
}
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo<P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2, pinterfaceinfo: Option<&[u8]>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigInterfaceTransportSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hrouterinterface : super::super::Foundation:: HANDLE, hrouteriftransport : super::super::Foundation:: HANDLE, pinterfaceinfo : *const u8, dwinterfaceinfosize : u32) -> u32);
    MprConfigInterfaceTransportSetInfo(hmprconfig.param().abi(), hrouterinterface.param().abi(), hrouteriftransport.param().abi(), core::mem::transmute(pinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn MprConfigServerBackup<P0, P1>(hmprconfig: P0, lpwspath: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerBackup(hmprconfig : super::super::Foundation:: HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    MprConfigServerBackup(hmprconfig.param().abi(), lpwspath.param().abi())
}
#[inline]
pub unsafe fn MprConfigServerConnect<P0>(lpwsservername: P0, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerConnect(lpwsservername : windows_core::PCWSTR, phmprconfig : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigServerConnect(lpwsservername.param().abi(), phmprconfig)
}
#[inline]
pub unsafe fn MprConfigServerDisconnect<P0>(hmprconfig: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerDisconnect(hmprconfig : super::super::Foundation:: HANDLE));
    MprConfigServerDisconnect(hmprconfig.param().abi())
}
#[inline]
pub unsafe fn MprConfigServerGetInfo<P0>(hmprconfig: P0, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerGetInfo(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbbuffer : *mut *mut u8) -> u32);
    MprConfigServerGetInfo(hmprconfig.param().abi(), dwlevel, lplpbbuffer)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx<P0>(hmprconfig: P0, pserverinfo: *mut MPR_SERVER_EX1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerGetInfoEx(hmprconfig : super::super::Foundation:: HANDLE, pserverinfo : *mut MPR_SERVER_EX1) -> u32);
    MprConfigServerGetInfoEx(hmprconfig.param().abi(), pserverinfo)
}
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerInstall(dwlevel : u32, pbuffer : *const core::ffi::c_void) -> u32);
    MprConfigServerInstall(dwlevel, pbuffer)
}
#[inline]
pub unsafe fn MprConfigServerRefresh<P0>(hmprconfig: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerRefresh(hmprconfig : super::super::Foundation:: HANDLE) -> u32);
    MprConfigServerRefresh(hmprconfig.param().abi())
}
#[inline]
pub unsafe fn MprConfigServerRestore<P0, P1>(hmprconfig: P0, lpwspath: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerRestore(hmprconfig : super::super::Foundation:: HANDLE, lpwspath : windows_core::PCWSTR) -> u32);
    MprConfigServerRestore(hmprconfig.param().abi(), lpwspath.param().abi())
}
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerSetInfo(hmprserver : isize, dwlevel : u32, lpbbuffer : *const u8) -> u32);
    MprConfigServerSetInfo(hmprserver, dwlevel, lpbbuffer)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx<P0>(hmprconfig: P0, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigServerSetInfoEx(hmprconfig : super::super::Foundation:: HANDLE, psetserverconfig : *const MPR_SERVER_SET_CONFIG_EX1) -> u32);
    MprConfigServerSetInfoEx(hmprconfig.param().abi(), psetserverconfig)
}
#[inline]
pub unsafe fn MprConfigTransportCreate<P0, P1, P2>(hmprconfig: P0, dwtransportid: u32, lpwstransportname: P1, pglobalinfo: &[u8], pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P2, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportCreate(hmprconfig : super::super::Foundation:: HANDLE, dwtransportid : u32, lpwstransportname : windows_core::PCWSTR, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR, phroutertransport : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigTransportCreate(hmprconfig.param().abi(), dwtransportid, lpwstransportname.param().abi(), core::mem::transmute(pglobalinfo.as_ptr()), pglobalinfo.len().try_into().unwrap(), core::mem::transmute(pclientinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi(), phroutertransport)
}
#[inline]
pub unsafe fn MprConfigTransportDelete<P0, P1>(hmprconfig: P0, hroutertransport: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportDelete(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE) -> u32);
    MprConfigTransportDelete(hmprconfig.param().abi(), hroutertransport.param().abi())
}
#[inline]
pub unsafe fn MprConfigTransportEnum<P0>(hmprconfig: P0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportEnum(hmprconfig : super::super::Foundation:: HANDLE, dwlevel : u32, lplpbuffer : *mut *mut u8, dwprefmaxlen : u32, lpdwentriesread : *mut u32, lpdwtotalentries : *mut u32, lpdwresumehandle : *mut u32) -> u32);
    MprConfigTransportEnum(hmprconfig.param().abi(), dwlevel, lplpbuffer, dwprefmaxlen, lpdwentriesread, lpdwtotalentries, core::mem::transmute(lpdwresumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprConfigTransportGetHandle<P0>(hmprconfig: P0, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportGetHandle(hmprconfig : super::super::Foundation:: HANDLE, dwtransportid : u32, phroutertransport : *mut super::super::Foundation:: HANDLE) -> u32);
    MprConfigTransportGetHandle(hmprconfig.param().abi(), dwtransportid, phroutertransport)
}
#[inline]
pub unsafe fn MprConfigTransportGetInfo<P0, P1>(hmprconfig: P0, hroutertransport: P1, ppglobalinfo: Option<*mut *mut u8>, lpdwglobalinfosize: Option<*mut u32>, ppclientinterfaceinfo: Option<*mut *mut u8>, lpdwclientinterfaceinfosize: Option<*mut u32>, lplpwsdllpath: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportGetInfo(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE, ppglobalinfo : *mut *mut u8, lpdwglobalinfosize : *mut u32, ppclientinterfaceinfo : *mut *mut u8, lpdwclientinterfaceinfosize : *mut u32, lplpwsdllpath : *mut windows_core::PWSTR) -> u32);
    MprConfigTransportGetInfo(hmprconfig.param().abi(), hroutertransport.param().abi(), core::mem::transmute(ppglobalinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdwglobalinfosize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppclientinterfaceinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lpdwclientinterfaceinfosize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpwsdllpath.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn MprConfigTransportSetInfo<P0, P1, P2>(hmprconfig: P0, hroutertransport: P1, pglobalinfo: Option<&[u8]>, pclientinterfaceinfo: Option<&[u8]>, lpwsdllpath: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mprapi.dll" "system" fn MprConfigTransportSetInfo(hmprconfig : super::super::Foundation:: HANDLE, hroutertransport : super::super::Foundation:: HANDLE, pglobalinfo : *const u8, dwglobalinfosize : u32, pclientinterfaceinfo : *const u8, dwclientinterfaceinfosize : u32, lpwsdllpath : windows_core::PCWSTR) -> u32);
    MprConfigTransportSetInfo(hmprconfig.param().abi(), hroutertransport.param().abi(), core::mem::transmute(pglobalinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pglobalinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pclientinterfaceinfo.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pclientinterfaceinfo.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpwsdllpath.param().abi())
}
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoBlockAdd(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoBlockAdd(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader)
}
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: *const core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoBlockFind(lpheader : *const core::ffi::c_void, dwinfotype : u32, lpdwitemsize : *mut u32, lpdwitemcount : *mut u32, lplpitemdata : *mut *mut u8) -> u32);
    MprInfoBlockFind(lpheader, dwinfotype, lpdwitemsize, lpdwitemcount, lplpitemdata)
}
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoBlockQuerySize(lpheader : *const core::ffi::c_void) -> u32);
    MprInfoBlockQuerySize(lpheader)
}
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: *const core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoBlockRemove(lpheader : *const core::ffi::c_void, dwinfotype : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoBlockRemove(lpheader, dwinfotype, lplpnewheader)
}
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: *const core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoBlockSet(lpheader : *const core::ffi::c_void, dwinfotype : u32, dwitemsize : u32, dwitemcount : u32, lpitemdata : *const u8, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoBlockSet(lpheader, dwinfotype, dwitemsize, dwitemcount, lpitemdata, lplpnewheader)
}
#[inline]
pub unsafe fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoCreate(dwversion : u32, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoCreate(dwversion, lplpnewheader)
}
#[inline]
pub unsafe fn MprInfoDelete(lpheader: *const core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoDelete(lpheader : *const core::ffi::c_void) -> u32);
    MprInfoDelete(lpheader)
}
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoDuplicate(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoDuplicate(lpheader, lplpnewheader)
}
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: *const core::ffi::c_void, lplpnewheader: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("mprapi.dll" "system" fn MprInfoRemoveAll(lpheader : *const core::ffi::c_void, lplpnewheader : *mut *mut core::ffi::c_void) -> u32);
    MprInfoRemoveAll(lpheader, lplpnewheader)
}
#[inline]
pub unsafe fn RasClearConnectionStatistics<P0>(hrasconn: P0) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasClearConnectionStatistics(hrasconn : HRASCONN) -> u32);
    RasClearConnectionStatistics(hrasconn.param().abi())
}
#[inline]
pub unsafe fn RasClearLinkStatistics<P0>(hrasconn: P0, dwsubentry: u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasClearLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32) -> u32);
    RasClearLinkStatistics(hrasconn.param().abi(), dwsubentry)
}
#[inline]
pub unsafe fn RasConnectionNotificationA<P0, P1>(param0: P0, param1: P1, param2: u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasConnectionNotificationA(param0 : HRASCONN, param1 : super::super::Foundation:: HANDLE, param2 : u32) -> u32);
    RasConnectionNotificationA(param0.param().abi(), param1.param().abi(), param2)
}
#[inline]
pub unsafe fn RasConnectionNotificationW<P0, P1>(param0: P0, param1: P1, param2: u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasConnectionNotificationW(param0 : HRASCONN, param1 : super::super::Foundation:: HANDLE, param2 : u32) -> u32);
    RasConnectionNotificationW(param0.param().abi(), param1.param().abi(), param2)
}
#[inline]
pub unsafe fn RasCreatePhonebookEntryA<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryA(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCSTR) -> u32);
    RasCreatePhonebookEntryA(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RasCreatePhonebookEntryW<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasCreatePhonebookEntryW(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCWSTR) -> u32);
    RasCreatePhonebookEntryW(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RasDeleteEntryA<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDeleteEntryA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR) -> u32);
    RasDeleteEntryA(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RasDeleteEntryW<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDeleteEntryW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR) -> u32);
    RasDeleteEntryW(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RasDeleteSubEntryA<P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDeleteSubEntryA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, dwsubentryid : u32) -> u32);
    RasDeleteSubEntryA(pszphonebook.param().abi(), pszentry.param().abi(), dwsubentryid)
}
#[inline]
pub unsafe fn RasDeleteSubEntryW<P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDeleteSubEntryW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, dwsubentryid : u32) -> u32);
    RasDeleteSubEntryW(pszphonebook.param().abi(), pszentry.param().abi(), dwsubentryid)
}
#[inline]
pub unsafe fn RasDialA<P0>(param0: Option<*const RASDIALEXTENSIONS>, param1: P0, param2: *const RASDIALPARAMSA, param3: u32, param4: Option<*const core::ffi::c_void>, param5: *mut HRASCONN) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDialA(param0 : *const RASDIALEXTENSIONS, param1 : windows_core::PCSTR, param2 : *const RASDIALPARAMSA, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
    RasDialA(core::mem::transmute(param0.unwrap_or(std::ptr::null())), param1.param().abi(), param2, param3, core::mem::transmute(param4.unwrap_or(std::ptr::null())), param5)
}
#[inline]
pub unsafe fn RasDialDlgA<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasDialDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpszphonenumber : windows_core::PCSTR, lpinfo : *mut RASDIALDLG) -> super::super::Foundation:: BOOL);
    RasDialDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasDialDlgW<P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasDialDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpszphonenumber : windows_core::PCWSTR, lpinfo : *mut RASDIALDLG) -> super::super::Foundation:: BOOL);
    RasDialDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpszphonenumber.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasDialW<P0>(param0: Option<*const RASDIALEXTENSIONS>, param1: P0, param2: *const RASDIALPARAMSW, param3: u32, param4: Option<*const core::ffi::c_void>, param5: *mut HRASCONN) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasDialW(param0 : *const RASDIALEXTENSIONS, param1 : windows_core::PCWSTR, param2 : *const RASDIALPARAMSW, param3 : u32, param4 : *const core::ffi::c_void, param5 : *mut HRASCONN) -> u32);
    RasDialW(core::mem::transmute(param0.unwrap_or(std::ptr::null())), param1.param().abi(), param2, param3, core::mem::transmute(param4.unwrap_or(std::ptr::null())), param5)
}
#[inline]
pub unsafe fn RasEditPhonebookEntryA<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryA(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR) -> u32);
    RasEditPhonebookEntryA(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn RasEditPhonebookEntryW<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasEditPhonebookEntryW(param0 : super::super::Foundation:: HWND, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR) -> u32);
    RasEditPhonebookEntryW(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn RasEntryDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasEntryDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut RASENTRYDLGA) -> super::super::Foundation:: BOOL);
    RasEntryDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasEntryDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasEntryDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut RASENTRYDLGW) -> super::super::Foundation:: BOOL);
    RasEntryDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasEnumAutodialAddressesA(lpprasautodialaddresses: Option<*mut windows_core::PSTR>, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesA(lpprasautodialaddresses : *mut windows_core::PSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
    RasEnumAutodialAddressesA(core::mem::transmute(lpprasautodialaddresses.unwrap_or(std::ptr::null_mut())), lpdwcbrasautodialaddresses, lpdwcrasautodialaddresses)
}
#[inline]
pub unsafe fn RasEnumAutodialAddressesW(lpprasautodialaddresses: Option<*mut windows_core::PWSTR>, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumAutodialAddressesW(lpprasautodialaddresses : *mut windows_core::PWSTR, lpdwcbrasautodialaddresses : *mut u32, lpdwcrasautodialaddresses : *mut u32) -> u32);
    RasEnumAutodialAddressesW(core::mem::transmute(lpprasautodialaddresses.unwrap_or(std::ptr::null_mut())), lpdwcbrasautodialaddresses, lpdwcrasautodialaddresses)
}
#[inline]
pub unsafe fn RasEnumConnectionsA(param0: Option<*mut RASCONNA>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumConnectionsA(param0 : *mut RASCONNA, param1 : *mut u32, param2 : *mut u32) -> u32);
    RasEnumConnectionsA(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1, param2)
}
#[inline]
pub unsafe fn RasEnumConnectionsW(param0: Option<*mut RASCONNW>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumConnectionsW(param0 : *mut RASCONNW, param1 : *mut u32, param2 : *mut u32) -> u32);
    RasEnumConnectionsW(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1, param2)
}
#[inline]
pub unsafe fn RasEnumDevicesA(param0: Option<*mut RASDEVINFOA>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumDevicesA(param0 : *mut RASDEVINFOA, param1 : *mut u32, param2 : *mut u32) -> u32);
    RasEnumDevicesA(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1, param2)
}
#[inline]
pub unsafe fn RasEnumDevicesW(param0: Option<*mut RASDEVINFOW>, param1: *mut u32, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumDevicesW(param0 : *mut RASDEVINFOW, param1 : *mut u32, param2 : *mut u32) -> u32);
    RasEnumDevicesW(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1, param2)
}
#[inline]
pub unsafe fn RasEnumEntriesA<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYNAMEA>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumEntriesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASENTRYNAMEA, param3 : *mut u32, param4 : *mut u32) -> u32);
    RasEnumEntriesA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, param4)
}
#[inline]
pub unsafe fn RasEnumEntriesW<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYNAMEW>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasEnumEntriesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASENTRYNAMEW, param3 : *mut u32, param4 : *mut u32) -> u32);
    RasEnumEntriesW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, param4)
}
#[inline]
pub unsafe fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA) {
    windows_targets::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityA(praseapuseridentity : *const RASEAPUSERIDENTITYA));
    RasFreeEapUserIdentityA(praseapuseridentity)
}
#[inline]
pub unsafe fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW) {
    windows_targets::link!("rasapi32.dll" "system" fn RasFreeEapUserIdentityW(praseapuseridentity : *const RASEAPUSERIDENTITYW));
    RasFreeEapUserIdentityW(praseapuseridentity)
}
#[inline]
pub unsafe fn RasGetAutodialAddressA<P0>(param0: P0, param1: Option<*const u32>, param2: Option<*mut RASAUTODIALENTRYA>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialAddressA(param0 : windows_core::PCSTR, param1 : *const u32, param2 : *mut RASAUTODIALENTRYA, param3 : *mut u32, param4 : *mut u32) -> u32);
    RasGetAutodialAddressA(param0.param().abi(), core::mem::transmute(param1.unwrap_or(std::ptr::null())), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, param4)
}
#[inline]
pub unsafe fn RasGetAutodialAddressW<P0>(param0: P0, param1: Option<*const u32>, param2: Option<*mut RASAUTODIALENTRYW>, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialAddressW(param0 : windows_core::PCWSTR, param1 : *const u32, param2 : *mut RASAUTODIALENTRYW, param3 : *mut u32, param4 : *mut u32) -> u32);
    RasGetAutodialAddressW(param0.param().abi(), core::mem::transmute(param1.unwrap_or(std::ptr::null())), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, param4)
}
#[inline]
pub unsafe fn RasGetAutodialEnableA(param0: u32, param1: *mut super::super::Foundation::BOOL) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialEnableA(param0 : u32, param1 : *mut super::super::Foundation:: BOOL) -> u32);
    RasGetAutodialEnableA(param0, param1)
}
#[inline]
pub unsafe fn RasGetAutodialEnableW(param0: u32, param1: *mut super::super::Foundation::BOOL) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialEnableW(param0 : u32, param1 : *mut super::super::Foundation:: BOOL) -> u32);
    RasGetAutodialEnableW(param0, param1)
}
#[inline]
pub unsafe fn RasGetAutodialParamA(param0: u32, param1: *mut core::ffi::c_void, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialParamA(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
    RasGetAutodialParamA(param0, param1, param2)
}
#[inline]
pub unsafe fn RasGetAutodialParamW(param0: u32, param1: *mut core::ffi::c_void, param2: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetAutodialParamW(param0 : u32, param1 : *mut core::ffi::c_void, param2 : *mut u32) -> u32);
    RasGetAutodialParamW(param0, param1, param2)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusA<P0>(param0: P0, param1: *mut RASCONNSTATUSA) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetConnectStatusA(param0 : HRASCONN, param1 : *mut RASCONNSTATUSA) -> u32);
    RasGetConnectStatusA(param0.param().abi(), param1)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusW<P0>(param0: P0, param1: *mut RASCONNSTATUSW) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetConnectStatusW(param0 : HRASCONN, param1 : *mut RASCONNSTATUSW) -> u32);
    RasGetConnectStatusW(param0.param().abi(), param1)
}
#[inline]
pub unsafe fn RasGetConnectionStatistics<P0>(hrasconn: P0, lpstatistics: *mut RAS_STATS) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetConnectionStatistics(hrasconn : HRASCONN, lpstatistics : *mut RAS_STATS) -> u32);
    RasGetConnectionStatistics(hrasconn.param().abi(), lpstatistics)
}
#[inline]
pub unsafe fn RasGetCountryInfoA(param0: Option<*mut RASCTRYINFO>, param1: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCountryInfoA(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
    RasGetCountryInfoA(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1)
}
#[inline]
pub unsafe fn RasGetCountryInfoW(param0: Option<*mut RASCTRYINFO>, param1: *mut u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCountryInfoW(param0 : *mut RASCTRYINFO, param1 : *mut u32) -> u32);
    RasGetCountryInfoW(core::mem::transmute(param0.unwrap_or(std::ptr::null_mut())), param1)
}
#[inline]
pub unsafe fn RasGetCredentialsA<P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCredentialsA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASCREDENTIALSA) -> u32);
    RasGetCredentialsA(param0.param().abi(), param1.param().abi(), param2)
}
#[inline]
pub unsafe fn RasGetCredentialsW<P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCredentialsW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASCREDENTIALSW) -> u32);
    RasGetCredentialsW(param0.param().abi(), param1.param().abi(), param2)
}
#[inline]
pub unsafe fn RasGetCustomAuthDataA<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: Option<*mut u8>, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
    RasGetCustomAuthDataA(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.unwrap_or(std::ptr::null_mut())), pdwsizeofcustomauthdata)
}
#[inline]
pub unsafe fn RasGetCustomAuthDataW<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: Option<*mut u8>, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetCustomAuthDataW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbcustomauthdata : *mut u8, pdwsizeofcustomauthdata : *mut u32) -> u32);
    RasGetCustomAuthDataW(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.unwrap_or(std::ptr::null_mut())), pdwsizeofcustomauthdata)
}
#[inline]
pub unsafe fn RasGetEapUserDataA<P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: Option<*mut u8>, pdwsizeofeapdata: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEapUserDataA(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
    RasGetEapUserDataA(htoken.param().abi(), pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbeapdata.unwrap_or(std::ptr::null_mut())), pdwsizeofeapdata)
}
#[inline]
pub unsafe fn RasGetEapUserDataW<P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: Option<*mut u8>, pdwsizeofeapdata: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEapUserDataW(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbeapdata : *mut u8, pdwsizeofeapdata : *mut u32) -> u32);
    RasGetEapUserDataW(htoken.param().abi(), pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbeapdata.unwrap_or(std::ptr::null_mut())), pdwsizeofeapdata)
}
#[inline]
pub unsafe fn RasGetEapUserIdentityA<P0, P1, P2>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: P2, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, ppraseapuseridentity : *mut *mut RASEAPUSERIDENTITYA) -> u32);
    RasGetEapUserIdentityA(pszphonebook.param().abi(), pszentry.param().abi(), dwflags, hwnd.param().abi(), ppraseapuseridentity)
}
#[inline]
pub unsafe fn RasGetEapUserIdentityW<P0, P1, P2>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: P2, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEapUserIdentityW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, dwflags : u32, hwnd : super::super::Foundation:: HWND, ppraseapuseridentity : *mut *mut RASEAPUSERIDENTITYW) -> u32);
    RasGetEapUserIdentityW(pszphonebook.param().abi(), pszentry.param().abi(), dwflags, hwnd.param().abi(), ppraseapuseridentity)
}
#[inline]
pub unsafe fn RasGetEntryDialParamsA<P0>(param0: P0, param1: *mut RASDIALPARAMSA, param2: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsA(param0 : windows_core::PCSTR, param1 : *mut RASDIALPARAMSA, param2 : *mut super::super::Foundation:: BOOL) -> u32);
    RasGetEntryDialParamsA(param0.param().abi(), param1, param2)
}
#[inline]
pub unsafe fn RasGetEntryDialParamsW<P0>(param0: P0, param1: *mut RASDIALPARAMSW, param2: *mut super::super::Foundation::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEntryDialParamsW(param0 : windows_core::PCWSTR, param1 : *mut RASDIALPARAMSW, param2 : *mut super::super::Foundation:: BOOL) -> u32);
    RasGetEntryDialParamsW(param0.param().abi(), param1, param2)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYA>, param3: *mut u32, param4: Option<*mut u8>, param5: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *mut RASENTRYA, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
    RasGetEntryPropertiesA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, core::mem::transmute(param4.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param5.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: Option<*mut RASENTRYW>, param3: *mut u32, param4: Option<*mut u8>, param5: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *mut RASENTRYW, param3 : *mut u32, param4 : *mut u8, param5 : *mut u32) -> u32);
    RasGetEntryPropertiesW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.unwrap_or(std::ptr::null_mut())), param3, core::mem::transmute(param4.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param5.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RasGetErrorStringA(resourceid: u32, lpszstring: &mut [u8]) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetErrorStringA(resourceid : u32, lpszstring : windows_core::PSTR, inbufsize : u32) -> u32);
    RasGetErrorStringA(resourceid, core::mem::transmute(lpszstring.as_ptr()), lpszstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RasGetErrorStringW(resourceid: u32, lpszstring: &mut [u16]) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetErrorStringW(resourceid : u32, lpszstring : windows_core::PWSTR, inbufsize : u32) -> u32);
    RasGetErrorStringW(resourceid, core::mem::transmute(lpszstring.as_ptr()), lpszstring.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RasGetLinkStatistics<P0>(hrasconn: P0, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetLinkStatistics(hrasconn : HRASCONN, dwsubentry : u32, lpstatistics : *mut RAS_STATS) -> u32);
    RasGetLinkStatistics(hrasconn.param().abi(), dwsubentry, lpstatistics)
}
#[inline]
pub unsafe fn RasGetPCscf(lpszpcscf: windows_core::PWSTR) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasGetPCscf(lpszpcscf : windows_core::PWSTR) -> u32);
    RasGetPCscf(core::mem::transmute(lpszpcscf))
}
#[inline]
pub unsafe fn RasGetProjectionInfoA<P0>(param0: P0, param1: RASPROJECTION, param2: *mut core::ffi::c_void, param3: *mut u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetProjectionInfoA(param0 : HRASCONN, param1 : RASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
    RasGetProjectionInfoA(param0.param().abi(), param1, param2, param3)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetProjectionInfoEx<P0>(hrasconn: P0, prasprojection: Option<*mut RAS_PROJECTION_INFO>, lpdwsize: *mut u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetProjectionInfoEx(hrasconn : HRASCONN, prasprojection : *mut RAS_PROJECTION_INFO, lpdwsize : *mut u32) -> u32);
    RasGetProjectionInfoEx(hrasconn.param().abi(), core::mem::transmute(prasprojection.unwrap_or(std::ptr::null_mut())), lpdwsize)
}
#[inline]
pub unsafe fn RasGetProjectionInfoW<P0>(param0: P0, param1: RASPROJECTION, param2: *mut core::ffi::c_void, param3: *mut u32) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetProjectionInfoW(param0 : HRASCONN, param1 : RASPROJECTION, param2 : *mut core::ffi::c_void, param3 : *mut u32) -> u32);
    RasGetProjectionInfoW(param0.param().abi(), param1, param2, param3)
}
#[inline]
pub unsafe fn RasGetSubEntryHandleA<P0>(param0: P0, param1: u32, param2: *mut HRASCONN) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleA(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
    RasGetSubEntryHandleA(param0.param().abi(), param1, param2)
}
#[inline]
pub unsafe fn RasGetSubEntryHandleW<P0>(param0: P0, param1: u32, param2: *mut HRASCONN) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetSubEntryHandleW(param0 : HRASCONN, param1 : u32, param2 : *mut HRASCONN) -> u32);
    RasGetSubEntryHandleW(param0.param().abi(), param1, param2)
}
#[inline]
pub unsafe fn RasGetSubEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: Option<*mut RASSUBENTRYA>, param4: Option<*mut u32>, param5: Option<*mut u8>, param6: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : u32, param3 : *mut RASSUBENTRYA, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
    RasGetSubEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, core::mem::transmute(param3.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param4.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param5.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param6.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RasGetSubEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: Option<*mut RASSUBENTRYW>, param4: Option<*mut u32>, param5: Option<*mut u8>, param6: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasGetSubEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : u32, param3 : *mut RASSUBENTRYW, param4 : *mut u32, param5 : *mut u8, param6 : *mut u32) -> u32);
    RasGetSubEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, core::mem::transmute(param3.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param4.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param5.unwrap_or(std::ptr::null_mut())), core::mem::transmute(param6.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RasHangUpA<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasHangUpA(param0 : HRASCONN) -> u32);
    RasHangUpA(param0.param().abi())
}
#[inline]
pub unsafe fn RasHangUpW<P0>(param0: P0) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasHangUpW(param0 : HRASCONN) -> u32);
    RasHangUpW(param0.param().abi())
}
#[inline]
pub unsafe fn RasInvokeEapUI<P0, P1>(param0: P0, param1: u32, param2: *const RASDIALEXTENSIONS, param3: P1) -> u32
where
    P0: windows_core::Param<HRASCONN>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasInvokeEapUI(param0 : HRASCONN, param1 : u32, param2 : *const RASDIALEXTENSIONS, param3 : super::super::Foundation:: HWND) -> u32);
    RasInvokeEapUI(param0.param().abi(), param1, param2, param3.param().abi())
}
#[inline]
pub unsafe fn RasPhonebookDlgA<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGA) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasPhonebookDlgA(lpszphonebook : windows_core::PCSTR, lpszentry : windows_core::PCSTR, lpinfo : *mut RASPBDLGA) -> super::super::Foundation:: BOOL);
    RasPhonebookDlgA(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasPhonebookDlgW<P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGW) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasdlg.dll" "system" fn RasPhonebookDlgW(lpszphonebook : windows_core::PCWSTR, lpszentry : windows_core::PCWSTR, lpinfo : *mut RASPBDLGW) -> super::super::Foundation:: BOOL);
    RasPhonebookDlgW(lpszphonebook.param().abi(), lpszentry.param().abi(), lpinfo)
}
#[inline]
pub unsafe fn RasRenameEntryA<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasRenameEntryA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR) -> u32);
    RasRenameEntryA(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn RasRenameEntryW<P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasRenameEntryW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR) -> u32);
    RasRenameEntryW(param0.param().abi(), param1.param().abi(), param2.param().abi())
}
#[inline]
pub unsafe fn RasSetAutodialAddressA<P0>(param0: P0, param1: u32, param2: Option<*const RASAUTODIALENTRYA>, param3: u32, param4: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialAddressA(param0 : windows_core::PCSTR, param1 : u32, param2 : *const RASAUTODIALENTRYA, param3 : u32, param4 : u32) -> u32);
    RasSetAutodialAddressA(param0.param().abi(), param1, core::mem::transmute(param2.unwrap_or(std::ptr::null())), param3, param4)
}
#[inline]
pub unsafe fn RasSetAutodialAddressW<P0>(param0: P0, param1: u32, param2: Option<*const RASAUTODIALENTRYW>, param3: u32, param4: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialAddressW(param0 : windows_core::PCWSTR, param1 : u32, param2 : *const RASAUTODIALENTRYW, param3 : u32, param4 : u32) -> u32);
    RasSetAutodialAddressW(param0.param().abi(), param1, core::mem::transmute(param2.unwrap_or(std::ptr::null())), param3, param4)
}
#[inline]
pub unsafe fn RasSetAutodialEnableA<P0>(param0: u32, param1: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialEnableA(param0 : u32, param1 : super::super::Foundation:: BOOL) -> u32);
    RasSetAutodialEnableA(param0, param1.param().abi())
}
#[inline]
pub unsafe fn RasSetAutodialEnableW<P0>(param0: u32, param1: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialEnableW(param0 : u32, param1 : super::super::Foundation:: BOOL) -> u32);
    RasSetAutodialEnableW(param0, param1.param().abi())
}
#[inline]
pub unsafe fn RasSetAutodialParamA(param0: u32, param1: *const core::ffi::c_void, param2: u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialParamA(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
    RasSetAutodialParamA(param0, param1, param2)
}
#[inline]
pub unsafe fn RasSetAutodialParamW(param0: u32, param1: *const core::ffi::c_void, param2: u32) -> u32 {
    windows_targets::link!("rasapi32.dll" "system" fn RasSetAutodialParamW(param0 : u32, param1 : *const core::ffi::c_void, param2 : u32) -> u32);
    RasSetAutodialParamW(param0, param1, param2)
}
#[inline]
pub unsafe fn RasSetCredentialsA<P0, P1, P2>(param0: P0, param1: P1, param2: *const RASCREDENTIALSA, param3: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetCredentialsA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *const RASCREDENTIALSA, param3 : super::super::Foundation:: BOOL) -> u32);
    RasSetCredentialsA(param0.param().abi(), param1.param().abi(), param2, param3.param().abi())
}
#[inline]
pub unsafe fn RasSetCredentialsW<P0, P1, P2>(param0: P0, param1: P1, param2: *const RASCREDENTIALSW, param3: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetCredentialsW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *const RASCREDENTIALSW, param3 : super::super::Foundation:: BOOL) -> u32);
    RasSetCredentialsW(param0.param().abi(), param1.param().abi(), param2, param3.param().abi())
}
#[inline]
pub unsafe fn RasSetCustomAuthDataA<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: &[u8]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataA(pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
    RasSetCustomAuthDataA(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.as_ptr()), pbcustomauthdata.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RasSetCustomAuthDataW<P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: &[u8]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetCustomAuthDataW(pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbcustomauthdata : *const u8, dwsizeofcustomauthdata : u32) -> u32);
    RasSetCustomAuthDataW(pszphonebook.param().abi(), pszentry.param().abi(), core::mem::transmute(pbcustomauthdata.as_ptr()), pbcustomauthdata.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RasSetEapUserDataA<P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEapUserDataA(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCSTR, pszentry : windows_core::PCSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
    RasSetEapUserDataA(htoken.param().abi(), pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata, dwsizeofeapdata)
}
#[inline]
pub unsafe fn RasSetEapUserDataW<P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEapUserDataW(htoken : super::super::Foundation:: HANDLE, pszphonebook : windows_core::PCWSTR, pszentry : windows_core::PCWSTR, pbeapdata : *const u8, dwsizeofeapdata : u32) -> u32);
    RasSetEapUserDataW(htoken.param().abi(), pszphonebook.param().abi(), pszentry.param().abi(), pbeapdata, dwsizeofeapdata)
}
#[inline]
pub unsafe fn RasSetEntryDialParamsA<P0, P1>(param0: P0, param1: *const RASDIALPARAMSA, param2: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsA(param0 : windows_core::PCSTR, param1 : *const RASDIALPARAMSA, param2 : super::super::Foundation:: BOOL) -> u32);
    RasSetEntryDialParamsA(param0.param().abi(), param1, param2.param().abi())
}
#[inline]
pub unsafe fn RasSetEntryDialParamsW<P0, P1>(param0: P0, param1: *const RASDIALPARAMSW, param2: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEntryDialParamsW(param0 : windows_core::PCWSTR, param1 : *const RASDIALPARAMSW, param2 : super::super::Foundation:: BOOL) -> u32);
    RasSetEntryDialParamsW(param0.param().abi(), param1, param2.param().abi())
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasSetEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYA, param3: u32, param4: Option<*const u8>, param5: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : *const RASENTRYA, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
    RasSetEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, param3, core::mem::transmute(param4.unwrap_or(std::ptr::null())), param5)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasSetEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYW, param3: u32, param4: Option<*const u8>, param5: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : *const RASENTRYW, param3 : u32, param4 : *const u8, param5 : u32) -> u32);
    RasSetEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, param3, core::mem::transmute(param4.unwrap_or(std::ptr::null())), param5)
}
#[inline]
pub unsafe fn RasSetSubEntryPropertiesA<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: Option<*const u8>, param6: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR, param2 : u32, param3 : *const RASSUBENTRYA, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
    RasSetSubEntryPropertiesA(param0.param().abi(), param1.param().abi(), param2, param3, param4, core::mem::transmute(param5.unwrap_or(std::ptr::null())), param6)
}
#[inline]
pub unsafe fn RasSetSubEntryPropertiesW<P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: Option<*const u8>, param6: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasSetSubEntryPropertiesW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR, param2 : u32, param3 : *const RASSUBENTRYW, param4 : u32, param5 : *const u8, param6 : u32) -> u32);
    RasSetSubEntryPropertiesW(param0.param().abi(), param1.param().abi(), param2, param3, param4, core::mem::transmute(param5.unwrap_or(std::ptr::null())), param6)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasUpdateConnection<P0>(hrasconn: P0, lprasupdateconn: *const RASUPDATECONN) -> u32
where
    P0: windows_core::Param<HRASCONN>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasUpdateConnection(hrasconn : HRASCONN, lprasupdateconn : *const RASUPDATECONN) -> u32);
    RasUpdateConnection(hrasconn.param().abi(), lprasupdateconn)
}
#[inline]
pub unsafe fn RasValidateEntryNameA<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasValidateEntryNameA(param0 : windows_core::PCSTR, param1 : windows_core::PCSTR) -> u32);
    RasValidateEntryNameA(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RasValidateEntryNameW<P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rasapi32.dll" "system" fn RasValidateEntryNameW(param0 : windows_core::PCWSTR, param1 : windows_core::PCWSTR) -> u32);
    RasValidateEntryNameW(param0.param().abi(), param1.param().abi())
}
#[inline]
pub unsafe fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmAddNextHop(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO, nexthophandle : *mut isize, changeflags : *mut u32) -> u32);
    RtmAddNextHop(rtmreghandle, nexthopinfo, nexthophandle, changeflags)
}
#[inline]
pub unsafe fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmAddRouteToDest(rtmreghandle : isize, routehandle : *mut isize, destaddress : *mut RTM_NET_ADDRESS, routeinfo : *mut RTM_ROUTE_INFO, timetolive : u32, routelisthandle : isize, notifytype : u32, notifyhandle : isize, changeflags : *mut u32) -> u32);
    RtmAddRouteToDest(rtmreghandle, routehandle, destaddress, routeinfo, timetolive, routelisthandle, notifytype, notifyhandle, changeflags)
}
#[inline]
pub unsafe fn RtmBlockMethods<P0>(rtmreghandle: isize, targethandle: P0, targettype: u8, blockingflag: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmBlockMethods(rtmreghandle : isize, targethandle : super::super::Foundation:: HANDLE, targettype : u8, blockingflag : u32) -> u32);
    RtmBlockMethods(rtmreghandle, targethandle.param().abi(), targettype, blockingflag)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress : *mut RTM_NET_ADDRESS, address : super::super::Networking::WinSock:: IN6_ADDR, dwlength : u32, dwaddresssize : u32) -> u32);
    RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress, core::mem::transmute(address), dwlength, dwaddresssize)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress : *mut RTM_NET_ADDRESS, paddress : *mut super::super::Networking::WinSock:: IN6_ADDR, plength : *mut u32, dwaddresssize : u32) -> u32);
    RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress, paddress, plength, dwaddresssize)
}
#[inline]
pub unsafe fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmCreateDestEnum(rtmreghandle : isize, targetviews : u32, enumflags : u32, netaddress : *mut RTM_NET_ADDRESS, protocolid : u32, rtmenumhandle : *mut isize) -> u32);
    RtmCreateDestEnum(rtmreghandle, targetviews, enumflags, netaddress, protocolid, rtmenumhandle)
}
#[inline]
pub unsafe fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmCreateNextHopEnum(rtmreghandle : isize, enumflags : u32, netaddress : *mut RTM_NET_ADDRESS, rtmenumhandle : *mut isize) -> u32);
    RtmCreateNextHopEnum(rtmreghandle, enumflags, netaddress, rtmenumhandle)
}
#[inline]
pub unsafe fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmCreateRouteEnum(rtmreghandle : isize, desthandle : isize, targetviews : u32, enumflags : u32, startdest : *mut RTM_NET_ADDRESS, matchingflags : u32, criteriaroute : *mut RTM_ROUTE_INFO, criteriainterface : u32, rtmenumhandle : *mut isize) -> u32);
    RtmCreateRouteEnum(rtmreghandle, desthandle, targetviews, enumflags, startdest, matchingflags, criteriaroute, criteriainterface, rtmenumhandle)
}
#[inline]
pub unsafe fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmCreateRouteList(rtmreghandle : isize, routelisthandle : *mut isize) -> u32);
    RtmCreateRouteList(rtmreghandle, routelisthandle)
}
#[inline]
pub unsafe fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmCreateRouteListEnum(rtmreghandle : isize, routelisthandle : isize, rtmenumhandle : *mut isize) -> u32);
    RtmCreateRouteListEnum(rtmreghandle, routelisthandle, rtmenumhandle)
}
#[inline]
pub unsafe fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeleteEnumHandle(rtmreghandle : isize, enumhandle : isize) -> u32);
    RtmDeleteEnumHandle(rtmreghandle, enumhandle)
}
#[inline]
pub unsafe fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeleteNextHop(rtmreghandle : isize, nexthophandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    RtmDeleteNextHop(rtmreghandle, nexthophandle, nexthopinfo)
}
#[inline]
pub unsafe fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeleteRouteList(rtmreghandle : isize, routelisthandle : isize) -> u32);
    RtmDeleteRouteList(rtmreghandle, routelisthandle)
}
#[inline]
pub unsafe fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeleteRouteToDest(rtmreghandle : isize, routehandle : isize, changeflags : *mut u32) -> u32);
    RtmDeleteRouteToDest(rtmreghandle, routehandle, changeflags)
}
#[inline]
pub unsafe fn RtmDeregisterEntity(rtmreghandle: isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeregisterEntity(rtmreghandle : isize) -> u32);
    RtmDeregisterEntity(rtmreghandle)
}
#[inline]
pub unsafe fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmDeregisterFromChangeNotification(rtmreghandle : isize, notifyhandle : isize) -> u32);
    RtmDeregisterFromChangeNotification(rtmreghandle, notifyhandle)
}
#[inline]
pub unsafe fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmFindNextHop(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO, nexthophandle : *mut isize, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    RtmFindNextHop(rtmreghandle, nexthopinfo, nexthophandle, nexthoppointer)
}
#[inline]
pub unsafe fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetChangeStatus(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, changestatus : *mut super::super::Foundation:: BOOL) -> u32);
    RtmGetChangeStatus(rtmreghandle, notifyhandle, desthandle, changestatus)
}
#[inline]
pub unsafe fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : *mut u32, changeddests : *mut RTM_DEST_INFO) -> u32);
    RtmGetChangedDests(rtmreghandle, notifyhandle, numdests, changeddests)
}
#[inline]
pub unsafe fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetDestInfo(rtmreghandle : isize, desthandle : isize, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    RtmGetDestInfo(rtmreghandle, desthandle, protocolid, targetviews, destinfo)
}
#[inline]
pub unsafe fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetEntityInfo(rtmreghandle : isize, entityhandle : isize, entityinfo : *mut RTM_ENTITY_INFO) -> u32);
    RtmGetEntityInfo(rtmreghandle, entityhandle, entityinfo)
}
#[inline]
pub unsafe fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut RTM_ENTITY_EXPORT_METHOD) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetEntityMethods(rtmreghandle : isize, entityhandle : isize, nummethods : *mut u32, exptmethods : *mut RTM_ENTITY_EXPORT_METHOD) -> u32);
    RtmGetEntityMethods(rtmreghandle, entityhandle, nummethods, exptmethods)
}
#[inline]
pub unsafe fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetEnumDests(rtmreghandle : isize, enumhandle : isize, numdests : *mut u32, destinfos : *mut RTM_DEST_INFO) -> u32);
    RtmGetEnumDests(rtmreghandle, enumhandle, numdests, destinfos)
}
#[inline]
pub unsafe fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetEnumNextHops(rtmreghandle : isize, enumhandle : isize, numnexthops : *mut u32, nexthophandles : *mut isize) -> u32);
    RtmGetEnumNextHops(rtmreghandle, enumhandle, numnexthops, nexthophandles)
}
#[inline]
pub unsafe fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetEnumRoutes(rtmreghandle : isize, enumhandle : isize, numroutes : *mut u32, routehandles : *mut isize) -> u32);
    RtmGetEnumRoutes(rtmreghandle, enumhandle, numroutes, routehandles)
}
#[inline]
pub unsafe fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetExactMatchDestination(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    RtmGetExactMatchDestination(rtmreghandle, destaddress, protocolid, targetviews, destinfo)
}
#[inline]
pub unsafe fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetExactMatchRoute(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, matchingflags : u32, routeinfo : *mut RTM_ROUTE_INFO, interfaceindex : u32, targetviews : u32, routehandle : *mut isize) -> u32);
    RtmGetExactMatchRoute(rtmreghandle, destaddress, matchingflags, routeinfo, interfaceindex, targetviews, routehandle)
}
#[inline]
pub unsafe fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetLessSpecificDestination(rtmreghandle : isize, desthandle : isize, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    RtmGetLessSpecificDestination(rtmreghandle, desthandle, protocolid, targetviews, destinfo)
}
#[inline]
pub unsafe fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetListEnumRoutes(rtmreghandle : isize, enumhandle : isize, numroutes : *mut u32, routehandles : *mut isize) -> u32);
    RtmGetListEnumRoutes(rtmreghandle, enumhandle, numroutes, routehandles)
}
#[inline]
pub unsafe fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetMostSpecificDestination(rtmreghandle : isize, destaddress : *mut RTM_NET_ADDRESS, protocolid : u32, targetviews : u32, destinfo : *mut RTM_DEST_INFO) -> u32);
    RtmGetMostSpecificDestination(rtmreghandle, destaddress, protocolid, targetviews, destinfo)
}
#[inline]
pub unsafe fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetNextHopInfo(rtmreghandle : isize, nexthophandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    RtmGetNextHopInfo(rtmreghandle, nexthophandle, nexthopinfo)
}
#[inline]
pub unsafe fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetNextHopPointer(rtmreghandle : isize, nexthophandle : isize, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    RtmGetNextHopPointer(rtmreghandle, nexthophandle, nexthoppointer)
}
#[inline]
pub unsafe fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetOpaqueInformationPointer(rtmreghandle : isize, desthandle : isize, opaqueinfopointer : *mut *mut core::ffi::c_void) -> u32);
    RtmGetOpaqueInformationPointer(rtmreghandle, desthandle, opaqueinfopointer)
}
#[inline]
pub unsafe fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetRegisteredEntities(rtmreghandle : isize, numentities : *mut u32, entityhandles : *mut isize, entityinfos : *mut RTM_ENTITY_INFO) -> u32);
    RtmGetRegisteredEntities(rtmreghandle, numentities, entityhandles, entityinfos)
}
#[inline]
pub unsafe fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetRouteInfo(rtmreghandle : isize, routehandle : isize, routeinfo : *mut RTM_ROUTE_INFO, destaddress : *mut RTM_NET_ADDRESS) -> u32);
    RtmGetRouteInfo(rtmreghandle, routehandle, routeinfo, destaddress)
}
#[inline]
pub unsafe fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmGetRoutePointer(rtmreghandle : isize, routehandle : isize, routepointer : *mut *mut RTM_ROUTE_INFO) -> u32);
    RtmGetRoutePointer(rtmreghandle, routehandle, routepointer)
}
#[inline]
pub unsafe fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmHoldDestination(rtmreghandle : isize, desthandle : isize, targetviews : u32, holdtime : u32) -> u32);
    RtmHoldDestination(rtmreghandle, desthandle, targetviews, holdtime)
}
#[inline]
pub unsafe fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmIgnoreChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : u32, changeddests : *mut isize) -> u32);
    RtmIgnoreChangedDests(rtmreghandle, notifyhandle, numdests, changeddests)
}
#[inline]
pub unsafe fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmInsertInRouteList(rtmreghandle : isize, routelisthandle : isize, numroutes : u32, routehandles : *mut isize) -> u32);
    RtmInsertInRouteList(rtmreghandle, routelisthandle, numroutes, routehandles)
}
#[inline]
pub unsafe fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmInvokeMethod(rtmreghandle : isize, entityhandle : isize, input : *mut RTM_ENTITY_METHOD_INPUT, outputsize : *mut u32, output : *mut RTM_ENTITY_METHOD_OUTPUT) -> u32);
    RtmInvokeMethod(rtmreghandle, entityhandle, input, outputsize, output)
}
#[inline]
pub unsafe fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmIsBestRoute(rtmreghandle : isize, routehandle : isize, bestinviews : *mut u32) -> u32);
    RtmIsBestRoute(rtmreghandle, routehandle, bestinviews)
}
#[inline]
pub unsafe fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmIsMarkedForChangeNotification(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, destmarked : *mut super::super::Foundation:: BOOL) -> u32);
    RtmIsMarkedForChangeNotification(rtmreghandle, notifyhandle, desthandle, destmarked)
}
#[inline]
pub unsafe fn RtmLockDestination<P0, P1>(rtmreghandle: isize, desthandle: isize, exclusive: P0, lockdest: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmLockDestination(rtmreghandle : isize, desthandle : isize, exclusive : super::super::Foundation:: BOOL, lockdest : super::super::Foundation:: BOOL) -> u32);
    RtmLockDestination(rtmreghandle, desthandle, exclusive.param().abi(), lockdest.param().abi())
}
#[inline]
pub unsafe fn RtmLockNextHop<P0, P1>(rtmreghandle: isize, nexthophandle: isize, exclusive: P0, locknexthop: P1, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmLockNextHop(rtmreghandle : isize, nexthophandle : isize, exclusive : super::super::Foundation:: BOOL, locknexthop : super::super::Foundation:: BOOL, nexthoppointer : *mut *mut RTM_NEXTHOP_INFO) -> u32);
    RtmLockNextHop(rtmreghandle, nexthophandle, exclusive.param().abi(), locknexthop.param().abi(), nexthoppointer)
}
#[inline]
pub unsafe fn RtmLockRoute<P0, P1>(rtmreghandle: isize, routehandle: isize, exclusive: P0, lockroute: P1, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmLockRoute(rtmreghandle : isize, routehandle : isize, exclusive : super::super::Foundation:: BOOL, lockroute : super::super::Foundation:: BOOL, routepointer : *mut *mut RTM_ROUTE_INFO) -> u32);
    RtmLockRoute(rtmreghandle, routehandle, exclusive.param().abi(), lockroute.param().abi(), routepointer)
}
#[inline]
pub unsafe fn RtmMarkDestForChangeNotification<P0>(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmMarkDestForChangeNotification(rtmreghandle : isize, notifyhandle : isize, desthandle : isize, markdest : super::super::Foundation:: BOOL) -> u32);
    RtmMarkDestForChangeNotification(rtmreghandle, notifyhandle, desthandle, markdest.param().abi())
}
#[inline]
pub unsafe fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReferenceHandles(rtmreghandle : isize, numhandles : u32, rtmhandles : *mut super::super::Foundation:: HANDLE) -> u32);
    RtmReferenceHandles(rtmreghandle, numhandles, rtmhandles)
}
#[inline]
pub unsafe fn RtmRegisterEntity<P0>(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut RTM_ENTITY_EXPORT_METHODS, eventcallback: RTM_EVENT_CALLBACK, reserveopaquepointer: P0, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtm.dll" "system" fn RtmRegisterEntity(rtmentityinfo : *mut RTM_ENTITY_INFO, exportmethods : *mut RTM_ENTITY_EXPORT_METHODS, eventcallback : RTM_EVENT_CALLBACK, reserveopaquepointer : super::super::Foundation:: BOOL, rtmregprofile : *mut RTM_REGN_PROFILE, rtmreghandle : *mut isize) -> u32);
    RtmRegisterEntity(rtmentityinfo, exportmethods, eventcallback, reserveopaquepointer.param().abi(), rtmregprofile, rtmreghandle)
}
#[inline]
pub unsafe fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut core::ffi::c_void, notifyhandle: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmRegisterForChangeNotification(rtmreghandle : isize, targetviews : u32, notifyflags : u32, notifycontext : *mut core::ffi::c_void, notifyhandle : *mut isize) -> u32);
    RtmRegisterForChangeNotification(rtmreghandle, targetviews, notifyflags, notifycontext, notifyhandle)
}
#[inline]
pub unsafe fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseChangedDests(rtmreghandle : isize, notifyhandle : isize, numdests : u32, changeddests : *mut RTM_DEST_INFO) -> u32);
    RtmReleaseChangedDests(rtmreghandle, notifyhandle, numdests, changeddests)
}
#[inline]
pub unsafe fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseDestInfo(rtmreghandle : isize, destinfo : *mut RTM_DEST_INFO) -> u32);
    RtmReleaseDestInfo(rtmreghandle, destinfo)
}
#[inline]
pub unsafe fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseDests(rtmreghandle : isize, numdests : u32, destinfos : *mut RTM_DEST_INFO) -> u32);
    RtmReleaseDests(rtmreghandle, numdests, destinfos)
}
#[inline]
pub unsafe fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseEntities(rtmreghandle : isize, numentities : u32, entityhandles : *mut isize) -> u32);
    RtmReleaseEntities(rtmreghandle, numentities, entityhandles)
}
#[inline]
pub unsafe fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseEntityInfo(rtmreghandle : isize, entityinfo : *mut RTM_ENTITY_INFO) -> u32);
    RtmReleaseEntityInfo(rtmreghandle, entityinfo)
}
#[inline]
pub unsafe fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseNextHopInfo(rtmreghandle : isize, nexthopinfo : *mut RTM_NEXTHOP_INFO) -> u32);
    RtmReleaseNextHopInfo(rtmreghandle, nexthopinfo)
}
#[inline]
pub unsafe fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseNextHops(rtmreghandle : isize, numnexthops : u32, nexthophandles : *mut isize) -> u32);
    RtmReleaseNextHops(rtmreghandle, numnexthops, nexthophandles)
}
#[inline]
pub unsafe fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseRouteInfo(rtmreghandle : isize, routeinfo : *mut RTM_ROUTE_INFO) -> u32);
    RtmReleaseRouteInfo(rtmreghandle, routeinfo)
}
#[inline]
pub unsafe fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmReleaseRoutes(rtmreghandle : isize, numroutes : u32, routehandles : *mut isize) -> u32);
    RtmReleaseRoutes(rtmreghandle, numroutes, routehandles)
}
#[inline]
pub unsafe fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    windows_targets::link!("rtm.dll" "system" fn RtmUpdateAndUnlockRoute(rtmreghandle : isize, routehandle : isize, timetolive : u32, routelisthandle : isize, notifytype : u32, notifyhandle : isize, changeflags : *mut u32) -> u32);
    RtmUpdateAndUnlockRoute(rtmreghandle, routehandle, timetolive, routelisthandle, notifytype, notifyhandle, changeflags)
}
pub const ALLOW_NO_AUTH: u32 = 1u32;
pub const ALL_SOURCES: MGM_ENUM_TYPES = MGM_ENUM_TYPES(1i32);
pub const ANY_SOURCE: MGM_ENUM_TYPES = MGM_ENUM_TYPES(0i32);
pub const ATADDRESSLEN: u32 = 32u32;
pub const DO_NOT_ALLOW_NO_AUTH: u32 = 0u32;
pub const ERROR_ACCESSING_TCPCFGDLL: u32 = 727u32;
pub const ERROR_ACCT_DISABLED: u32 = 647u32;
pub const ERROR_ACCT_EXPIRED: u32 = 708u32;
pub const ERROR_ACTION_REQUIRED: u32 = 877u32;
pub const ERROR_ALLOCATING_MEMORY: u32 = 664u32;
pub const ERROR_ALREADY_DISCONNECTING: u32 = 617u32;
pub const ERROR_ASYNC_REQUEST_PENDING: u32 = 616u32;
pub const ERROR_AUTHENTICATION_FAILURE: u32 = 691u32;
pub const ERROR_AUTH_INTERNAL: u32 = 645u32;
pub const ERROR_AUTOMATIC_VPN_FAILED: u32 = 800u32;
pub const ERROR_BAD_ADDRESS_SPECIFIED: u32 = 769u32;
pub const ERROR_BAD_CALLBACK_NUMBER: u32 = 704u32;
pub const ERROR_BAD_PHONE_NUMBER: u32 = 749u32;
pub const ERROR_BAD_STRING: u32 = 637u32;
pub const ERROR_BAD_USAGE_IN_INI_FILE: u32 = 669u32;
pub const ERROR_BIPLEX_PORT_NOT_AVAILABLE: u32 = 712u32;
pub const ERROR_BLOCKED: u32 = 775u32;
pub const ERROR_BROADBAND_ACTIVE: u32 = 813u32;
pub const ERROR_BROADBAND_NO_NIC: u32 = 814u32;
pub const ERROR_BROADBAND_TIMEOUT: u32 = 815u32;
pub const ERROR_BUFFER_INVALID: u32 = 610u32;
pub const ERROR_BUFFER_TOO_SMALL: u32 = 603u32;
pub const ERROR_BUNDLE_NOT_FOUND: u32 = 754u32;
pub const ERROR_CANNOT_DELETE: u32 = 817u32;
pub const ERROR_CANNOT_DO_CUSTOMDIAL: u32 = 755u32;
pub const ERROR_CANNOT_FIND_PHONEBOOK_ENTRY: u32 = 623u32;
pub const ERROR_CANNOT_GET_LANA: u32 = 639u32;
pub const ERROR_CANNOT_INITIATE_MOBIKE_UPDATE: u32 = 844u32;
pub const ERROR_CANNOT_LOAD_PHONEBOOK: u32 = 622u32;
pub const ERROR_CANNOT_LOAD_STRING: u32 = 626u32;
pub const ERROR_CANNOT_OPEN_PHONEBOOK: u32 = 621u32;
pub const ERROR_CANNOT_PROJECT_CLIENT: u32 = 634u32;
pub const ERROR_CANNOT_SET_PORT_INFO: u32 = 605u32;
pub const ERROR_CANNOT_SHARE_CONNECTION: u32 = 763u32;
pub const ERROR_CANNOT_USE_LOGON_CREDENTIALS: u32 = 739u32;
pub const ERROR_CANNOT_WRITE_PHONEBOOK: u32 = 624u32;
pub const ERROR_CERT_FOR_ENCRYPTION_NOT_FOUND: u32 = 781u32;
pub const ERROR_CHANGING_PASSWORD: u32 = 709u32;
pub const ERROR_CMD_TOO_LONG: u32 = 700u32;
pub const ERROR_CONGESTION: u32 = 771u32;
pub const ERROR_CONNECTING_DEVICE_NOT_FOUND: u32 = 797u32;
pub const ERROR_CONNECTION_ALREADY_SHARED: u32 = 758u32;
pub const ERROR_CONNECTION_REJECT: u32 = 770u32;
pub const ERROR_CORRUPT_PHONEBOOK: u32 = 625u32;
pub const ERROR_DCB_NOT_FOUND: u32 = 694u32;
pub const ERROR_DEFAULTOFF_MACRO_NOT_FOUND: u32 = 656u32;
pub const ERROR_DEVICENAME_NOT_FOUND: u32 = 659u32;
pub const ERROR_DEVICENAME_TOO_LONG: u32 = 658u32;
pub const ERROR_DEVICETYPE_DOES_NOT_EXIST: u32 = 609u32;
pub const ERROR_DEVICE_COMPLIANCE: u32 = 875u32;
pub const ERROR_DEVICE_DOES_NOT_EXIST: u32 = 608u32;
pub const ERROR_DEVICE_NOT_READY: u32 = 666u32;
pub const ERROR_DIAL_ALREADY_IN_PROGRESS: u32 = 756u32;
pub const ERROR_DISCONNECTION: u32 = 628u32;
pub const ERROR_DNSNAME_NOT_RESOLVABLE: u32 = 868u32;
pub const ERROR_DONOTDISTURB: u32 = 776u32;
pub const ERROR_EAPTLS_CACHE_CREDENTIALS_INVALID: u32 = 826u32;
pub const ERROR_EAPTLS_PASSWD_INVALID: u32 = 869u32;
pub const ERROR_EAPTLS_SCARD_CACHE_CREDENTIALS_INVALID: u32 = 847u32;
pub const ERROR_EAP_METHOD_DOES_NOT_SUPPORT_SSO: u32 = 851u32;
pub const ERROR_EAP_METHOD_NOT_INSTALLED: u32 = 850u32;
pub const ERROR_EAP_METHOD_OPERATION_NOT_SUPPORTED: u32 = 852u32;
pub const ERROR_EAP_SERVER_CERT_EXPIRED: u32 = 858u32;
pub const ERROR_EAP_SERVER_CERT_INVALID: u32 = 857u32;
pub const ERROR_EAP_SERVER_CERT_OTHER_ERROR: u32 = 860u32;
pub const ERROR_EAP_SERVER_CERT_REVOKED: u32 = 859u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_INVALID: u32 = 865u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 866u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NOT_FOUND: u32 = 864u32;
pub const ERROR_EAP_USER_CERT_EXPIRED: u32 = 854u32;
pub const ERROR_EAP_USER_CERT_INVALID: u32 = 853u32;
pub const ERROR_EAP_USER_CERT_OTHER_ERROR: u32 = 856u32;
pub const ERROR_EAP_USER_CERT_REVOKED: u32 = 855u32;
pub const ERROR_EAP_USER_ROOT_CERT_EXPIRED: u32 = 863u32;
pub const ERROR_EAP_USER_ROOT_CERT_INVALID: u32 = 862u32;
pub const ERROR_EAP_USER_ROOT_CERT_NOT_FOUND: u32 = 861u32;
pub const ERROR_EMPTY_INI_FILE: u32 = 690u32;
pub const ERROR_EVENT_INVALID: u32 = 607u32;
pub const ERROR_FAILED_CP_REQUIRED: u32 = 841u32;
pub const ERROR_FAILED_TO_ENCRYPT: u32 = 768u32;
pub const ERROR_FAST_USER_SWITCH: u32 = 831u32;
pub const ERROR_FEATURE_DEPRECATED: u32 = 816u32;
pub const ERROR_FILE_COULD_NOT_BE_OPENED: u32 = 657u32;
pub const ERROR_FROM_DEVICE: u32 = 651u32;
pub const ERROR_HANGUP_FAILED: u32 = 753u32;
pub const ERROR_HARDWARE_FAILURE: u32 = 630u32;
pub const ERROR_HIBERNATION: u32 = 832u32;
pub const ERROR_IDLE_TIMEOUT: u32 = 828u32;
pub const ERROR_IKEV2_PSK_INTERFACE_ALREADY_EXISTS: u32 = 870u32;
pub const ERROR_INCOMPATIBLE: u32 = 772u32;
pub const ERROR_INTERACTIVE_MODE: u32 = 703u32;
pub const ERROR_INTERNAL_ADDRESS_FAILURE: u32 = 840u32;
pub const ERROR_INVALID_AUTH_STATE: u32 = 705u32;
pub const ERROR_INVALID_CALLBACK_NUMBER: u32 = 751u32;
pub const ERROR_INVALID_COMPRESSION_SPECIFIED: u32 = 613u32;
pub const ERROR_INVALID_DESTINATION_IP: u32 = 871u32;
pub const ERROR_INVALID_FUNCTION_FOR_ENTRY: u32 = 780u32;
pub const ERROR_INVALID_INTERFACE_CONFIG: u32 = 872u32;
pub const ERROR_INVALID_MSCHAPV2_CONFIG: u32 = 805u32;
pub const ERROR_INVALID_PEAP_COOKIE_ATTRIBUTES: u32 = 849u32;
pub const ERROR_INVALID_PEAP_COOKIE_CONFIG: u32 = 803u32;
pub const ERROR_INVALID_PEAP_COOKIE_USER: u32 = 804u32;
pub const ERROR_INVALID_PORT_HANDLE: u32 = 601u32;
pub const ERROR_INVALID_PREFERENCES: u32 = 846u32;
pub const ERROR_INVALID_SERVER_CERT: u32 = 835u32;
pub const ERROR_INVALID_SIZE: u32 = 632u32;
pub const ERROR_INVALID_SMM: u32 = 745u32;
pub const ERROR_INVALID_TUNNELID: u32 = 837u32;
pub const ERROR_INVALID_VPNSTRATEGY: u32 = 825u32;
pub const ERROR_IN_COMMAND: u32 = 681u32;
pub const ERROR_IPSEC_SERVICE_STOPPED: u32 = 827u32;
pub const ERROR_IPXCP_DIALOUT_ALREADY_ACTIVE: u32 = 726u32;
pub const ERROR_IPXCP_NET_NUMBER_CONFLICT: u32 = 744u32;
pub const ERROR_IPXCP_NO_DIALIN_CONFIGURED: u32 = 725u32;
pub const ERROR_IPXCP_NO_DIALOUT_CONFIGURED: u32 = 724u32;
pub const ERROR_IP_CONFIGURATION: u32 = 716u32;
pub const ERROR_KEY_NOT_FOUND: u32 = 627u32;
pub const ERROR_LINE_BUSY: u32 = 676u32;
pub const ERROR_LINK_FAILURE: u32 = 829u32;
pub const ERROR_MACRO_NOT_DEFINED: u32 = 654u32;
pub const ERROR_MACRO_NOT_FOUND: u32 = 653u32;
pub const ERROR_MESSAGE_MACRO_NOT_FOUND: u32 = 655u32;
pub const ERROR_MOBIKE_DISABLED: u32 = 843u32;
pub const ERROR_NAME_EXISTS_ON_NET: u32 = 642u32;
pub const ERROR_NETBIOS_ERROR: u32 = 640u32;
pub const ERROR_NOT_BINARY_MACRO: u32 = 693u32;
pub const ERROR_NOT_NAP_CAPABLE: u32 = 836u32;
pub const ERROR_NO_ACTIVE_ISDN_LINES: u32 = 713u32;
pub const ERROR_NO_ANSWER: u32 = 678u32;
pub const ERROR_NO_CARRIER: u32 = 679u32;
pub const ERROR_NO_CERTIFICATE: u32 = 766u32;
pub const ERROR_NO_COMMAND_FOUND: u32 = 661u32;
pub const ERROR_NO_CONNECTION: u32 = 668u32;
pub const ERROR_NO_DIALIN_PERMISSION: u32 = 649u32;
pub const ERROR_NO_DIALTONE: u32 = 680u32;
pub const ERROR_NO_DIFF_USER_AT_LOGON: u32 = 784u32;
pub const ERROR_NO_EAPTLS_CERTIFICATE: u32 = 798u32;
pub const ERROR_NO_ENDPOINTS: u32 = 620u32;
pub const ERROR_NO_IP_ADDRESSES: u32 = 717u32;
pub const ERROR_NO_IP_RAS_ADAPTER: u32 = 728u32;
pub const ERROR_NO_ISDN_CHANNELS_AVAILABLE: u32 = 714u32;
pub const ERROR_NO_LOCAL_ENCRYPTION: u32 = 741u32;
pub const ERROR_NO_MAC_FOR_PORT: u32 = 747u32;
pub const ERROR_NO_REG_CERT_AT_LOGON: u32 = 785u32;
pub const ERROR_NO_REMOTE_ENCRYPTION: u32 = 742u32;
pub const ERROR_NO_RESPONSES: u32 = 660u32;
pub const ERROR_NO_SMART_CARD_READER: u32 = 764u32;
pub const ERROR_NUMBERCHANGED: u32 = 773u32;
pub const ERROR_OAKLEY_ATTRIB_FAIL: u32 = 788u32;
pub const ERROR_OAKLEY_AUTH_FAIL: u32 = 787u32;
pub const ERROR_OAKLEY_ERROR: u32 = 793u32;
pub const ERROR_OAKLEY_GENERAL_PROCESSING: u32 = 789u32;
pub const ERROR_OAKLEY_NO_CERT: u32 = 786u32;
pub const ERROR_OAKLEY_NO_PEER_CERT: u32 = 790u32;
pub const ERROR_OAKLEY_NO_POLICY: u32 = 791u32;
pub const ERROR_OAKLEY_TIMED_OUT: u32 = 792u32;
pub const ERROR_OUTOFORDER: u32 = 777u32;
pub const ERROR_OUT_OF_BUFFERS: u32 = 614u32;
pub const ERROR_OVERRUN: u32 = 710u32;
pub const ERROR_PARTIAL_RESPONSE_LOOPING: u32 = 697u32;
pub const ERROR_PASSWD_EXPIRED: u32 = 648u32;
pub const ERROR_PEAP_CRYPTOBINDING_INVALID: u32 = 823u32;
pub const ERROR_PEAP_CRYPTOBINDING_NOTRECEIVED: u32 = 824u32;
pub const ERROR_PEAP_IDENTITY_MISMATCH: u32 = 867u32;
pub const ERROR_PEAP_SERVER_REJECTED_CLIENT_TLV: u32 = 845u32;
pub const ERROR_PHONE_NUMBER_TOO_LONG: u32 = 723u32;
pub const ERROR_PLUGIN_NOT_INSTALLED: u32 = 876u32;
pub const ERROR_PORT_ALREADY_OPEN: u32 = 602u32;
pub const ERROR_PORT_DISCONNECTED: u32 = 619u32;
pub const ERROR_PORT_NOT_AVAILABLE: u32 = 633u32;
pub const ERROR_PORT_NOT_CONFIGURED: u32 = 665u32;
pub const ERROR_PORT_NOT_CONNECTED: u32 = 606u32;
pub const ERROR_PORT_NOT_FOUND: u32 = 615u32;
pub const ERROR_PORT_NOT_OPEN: u32 = 618u32;
pub const ERROR_PORT_OR_DEVICE: u32 = 692u32;
pub const ERROR_PPP_CP_REJECTED: u32 = 733u32;
pub const ERROR_PPP_INVALID_PACKET: u32 = 722u32;
pub const ERROR_PPP_LCP_TERMINATED: u32 = 734u32;
pub const ERROR_PPP_LOOPBACK_DETECTED: u32 = 737u32;
pub const ERROR_PPP_NCP_TERMINATED: u32 = 736u32;
pub const ERROR_PPP_NOT_CONVERGING: u32 = 732u32;
pub const ERROR_PPP_NO_ADDRESS_ASSIGNED: u32 = 738u32;
pub const ERROR_PPP_NO_PROTOCOLS_CONFIGURED: u32 = 720u32;
pub const ERROR_PPP_NO_RESPONSE: u32 = 721u32;
pub const ERROR_PPP_REMOTE_TERMINATED: u32 = 719u32;
pub const ERROR_PPP_REQUIRED_ADDRESS_REJECTED: u32 = 735u32;
pub const ERROR_PPP_TIMEOUT: u32 = 718u32;
pub const ERROR_PROJECTION_NOT_COMPLETE: u32 = 730u32;
pub const ERROR_PROTOCOL_ENGINE_DISABLED: u32 = 839u32;
pub const ERROR_PROTOCOL_NOT_CONFIGURED: u32 = 731u32;
pub const ERROR_RASAUTO_CANNOT_INITIALIZE: u32 = 757u32;
pub const ERROR_RASMAN_CANNOT_INITIALIZE: u32 = 711u32;
pub const ERROR_RASMAN_SERVICE_STOPPED: u32 = 834u32;
pub const ERROR_RASQEC_CONN_DOESNOTEXIST: u32 = 821u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_CONNECTED: u32 = 820u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_ENABLED: u32 = 819u32;
pub const ERROR_RASQEC_RESOURCE_CREATION_FAILED: u32 = 818u32;
pub const ERROR_RASQEC_TIMEOUT: u32 = 822u32;
pub const ERROR_READING_DEFAULTOFF: u32 = 689u32;
pub const ERROR_READING_DEVICENAME: u32 = 672u32;
pub const ERROR_READING_DEVICETYPE: u32 = 671u32;
pub const ERROR_READING_INI_FILE: u32 = 667u32;
pub const ERROR_READING_MAXCARRIERBPS: u32 = 675u32;
pub const ERROR_READING_MAXCONNECTBPS: u32 = 674u32;
pub const ERROR_READING_SCARD: u32 = 802u32;
pub const ERROR_READING_SECTIONNAME: u32 = 670u32;
pub const ERROR_READING_USAGE: u32 = 673u32;
pub const ERROR_RECV_BUF_FULL: u32 = 699u32;
pub const ERROR_REMOTE_DISCONNECTION: u32 = 629u32;
pub const ERROR_REMOTE_REQUIRES_ENCRYPTION: u32 = 743u32;
pub const ERROR_REQUEST_TIMEOUT: u32 = 638u32;
pub const ERROR_RESTRICTED_LOGON_HOURS: u32 = 646u32;
pub const ERROR_ROUTE_NOT_ALLOCATED: u32 = 612u32;
pub const ERROR_ROUTE_NOT_AVAILABLE: u32 = 611u32;
pub const ERROR_SCRIPT_SYNTAX: u32 = 752u32;
pub const ERROR_SERVER_GENERAL_NET_FAILURE: u32 = 643u32;
pub const ERROR_SERVER_NOT_RESPONDING: u32 = 650u32;
pub const ERROR_SERVER_OUT_OF_RESOURCES: u32 = 641u32;
pub const ERROR_SERVER_POLICY: u32 = 812u32;
pub const ERROR_SHARE_CONNECTION_FAILED: u32 = 761u32;
pub const ERROR_SHARING_ADDRESS_EXISTS: u32 = 765u32;
pub const ERROR_SHARING_CHANGE_FAILED: u32 = 759u32;
pub const ERROR_SHARING_HOST_ADDRESS_CONFLICT: u32 = 799u32;
pub const ERROR_SHARING_MULTIPLE_ADDRESSES: u32 = 767u32;
pub const ERROR_SHARING_NO_PRIVATE_LAN: u32 = 783u32;
pub const ERROR_SHARING_PRIVATE_INSTALL: u32 = 762u32;
pub const ERROR_SHARING_ROUTER_INSTALL: u32 = 760u32;
pub const ERROR_SHARING_RRAS_CONFLICT: u32 = 782u32;
pub const ERROR_SLIP_REQUIRES_IP: u32 = 729u32;
pub const ERROR_SMART_CARD_REQUIRED: u32 = 779u32;
pub const ERROR_SMM_TIMEOUT: u32 = 748u32;
pub const ERROR_SMM_UNINITIALIZED: u32 = 746u32;
pub const ERROR_SSO_CERT_MISSING: u32 = 874u32;
pub const ERROR_SSTP_COOKIE_SET_FAILURE: u32 = 848u32;
pub const ERROR_STATE_MACHINES_ALREADY_STARTED: u32 = 696u32;
pub const ERROR_STATE_MACHINES_NOT_STARTED: u32 = 695u32;
pub const ERROR_SYSTEM_SUSPENDED: u32 = 833u32;
pub const ERROR_TAPI_CONFIGURATION: u32 = 740u32;
pub const ERROR_TEMPFAILURE: u32 = 774u32;
pub const ERROR_TOO_MANY_LINE_ERRORS: u32 = 715u32;
pub const ERROR_TS_UNACCEPTABLE: u32 = 842u32;
pub const ERROR_UNABLE_TO_AUTHENTICATE_SERVER: u32 = 778u32;
pub const ERROR_UNEXPECTED_RESPONSE: u32 = 702u32;
pub const ERROR_UNKNOWN: u32 = 635u32;
pub const ERROR_UNKNOWN_DEVICE_TYPE: u32 = 663u32;
pub const ERROR_UNKNOWN_FRAMED_PROTOCOL: u32 = 794u32;
pub const ERROR_UNKNOWN_RESPONSE_KEY: u32 = 698u32;
pub const ERROR_UNKNOWN_SERVICE_TYPE: u32 = 796u32;
pub const ERROR_UNRECOGNIZED_RESPONSE: u32 = 652u32;
pub const ERROR_UNSUPPORTED_BPS: u32 = 701u32;
pub const ERROR_UPDATECONNECTION_REQUEST_IN_PROCESS: u32 = 838u32;
pub const ERROR_USER_DISCONNECTION: u32 = 631u32;
pub const ERROR_USER_LOGOFF: u32 = 830u32;
pub const ERROR_VALIDATING_SERVER_CERT: u32 = 801u32;
pub const ERROR_VOICE_ANSWER: u32 = 677u32;
pub const ERROR_VPN_BAD_CERT: u32 = 810u32;
pub const ERROR_VPN_BAD_PSK: u32 = 811u32;
pub const ERROR_VPN_DISCONNECT: u32 = 807u32;
pub const ERROR_VPN_GRE_BLOCKED: u32 = 806u32;
pub const ERROR_VPN_PLUGIN_GENERIC: u32 = 873u32;
pub const ERROR_VPN_REFUSED: u32 = 808u32;
pub const ERROR_VPN_TIMEOUT: u32 = 809u32;
pub const ERROR_WRITING_DEFAULTOFF: u32 = 688u32;
pub const ERROR_WRITING_DEVICENAME: u32 = 684u32;
pub const ERROR_WRITING_DEVICETYPE: u32 = 683u32;
pub const ERROR_WRITING_INITBPS: u32 = 706u32;
pub const ERROR_WRITING_MAXCARRIERBPS: u32 = 686u32;
pub const ERROR_WRITING_MAXCONNECTBPS: u32 = 685u32;
pub const ERROR_WRITING_SECTIONNAME: u32 = 682u32;
pub const ERROR_WRITING_USAGE: u32 = 687u32;
pub const ERROR_WRONG_DEVICE_ATTACHED: u32 = 636u32;
pub const ERROR_WRONG_INFO_SPECIFIED: u32 = 604u32;
pub const ERROR_WRONG_KEY_SPECIFIED: u32 = 662u32;
pub const ERROR_WRONG_MODULE: u32 = 750u32;
pub const ERROR_WRONG_TUNNEL_TYPE: u32 = 795u32;
pub const ERROR_X25_DIAGNOSTIC: u32 = 707u32;
pub const ET_None: u32 = 0u32;
pub const ET_Optional: u32 = 3u32;
pub const ET_Require: u32 = 1u32;
pub const ET_RequireMax: u32 = 2u32;
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_DN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(9i32);
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_GN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(10i32);
pub const IKEV2_ID_PAYLOAD_TYPE_FQDN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(2i32);
pub const IKEV2_ID_PAYLOAD_TYPE_ID_IPV6_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(5i32);
pub const IKEV2_ID_PAYLOAD_TYPE_INVALID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(0i32);
pub const IKEV2_ID_PAYLOAD_TYPE_IPV4_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(1i32);
pub const IKEV2_ID_PAYLOAD_TYPE_KEY_ID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(11i32);
pub const IKEV2_ID_PAYLOAD_TYPE_MAX: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(12i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED1: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(4i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED2: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(6i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED3: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(7i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED4: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(8i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RFC822_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(3i32);
pub const IPADDRESSLEN: u32 = 15u32;
pub const IPV6_ADDRESS_LEN_IN_BYTES: u32 = 16u32;
pub const IPXADDRESSLEN: u32 = 22u32;
pub const MAXIPADRESSLEN: u32 = 64u32;
pub const MAX_SSTP_HASH_SIZE: u32 = 32u32;
pub const METHOD_BGP4_AS_PATH: u32 = 1u32;
pub const METHOD_BGP4_NEXTHOP_ATTR: u32 = 8u32;
pub const METHOD_BGP4_PA_ORIGIN: u32 = 4u32;
pub const METHOD_BGP4_PEER_ID: u32 = 2u32;
pub const METHOD_RIP2_NEIGHBOUR_ADDR: u32 = 1u32;
pub const METHOD_RIP2_OUTBOUND_INTF: u32 = 2u32;
pub const METHOD_RIP2_ROUTE_TAG: u32 = 4u32;
pub const METHOD_RIP2_ROUTE_TIMESTAMP: u32 = 8u32;
pub const METHOD_TYPE_ALL_METHODS: u32 = 4294967295u32;
pub const MGM_FORWARD_STATE_FLAG: u32 = 2u32;
pub const MGM_JOIN_STATE_FLAG: u32 = 1u32;
pub const MGM_MFE_STATS_0: u32 = 1u32;
pub const MGM_MFE_STATS_1: u32 = 2u32;
pub const MPRAPI_ADMIN_DLL_VERSION_1: u32 = 1u32;
pub const MPRAPI_ADMIN_DLL_VERSION_2: u32 = 2u32;
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_CERT: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_EAP: u32 = 2u32;
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: u32 = 2u32;
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: u32 = 5u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: u32 = 5u32;
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(4i32);
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(6i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(2i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(3i32);
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(1i32);
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(5i32);
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: u32 = 1u32;
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: u32 = 16u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: u32 = 8u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: u32 = 2u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: u32 = 4u32;
pub const MPRDM_DialAll: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(1u32);
pub const MPRDM_DialAsNeeded: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(2u32);
pub const MPRDM_DialFirst: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(0u32);
pub const MPRDT_Atm: windows_core::PCWSTR = windows_core::w!("ATM");
pub const MPRDT_FrameRelay: windows_core::PCWSTR = windows_core::w!("FRAMERELAY");
pub const MPRDT_Generic: windows_core::PCWSTR = windows_core::w!("GENERIC");
pub const MPRDT_Irda: windows_core::PCWSTR = windows_core::w!("IRDA");
pub const MPRDT_Isdn: windows_core::PCWSTR = windows_core::w!("isdn");
pub const MPRDT_Modem: windows_core::PCWSTR = windows_core::w!("modem");
pub const MPRDT_Pad: windows_core::PCWSTR = windows_core::w!("pad");
pub const MPRDT_Parallel: windows_core::PCWSTR = windows_core::w!("PARALLEL");
pub const MPRDT_SW56: windows_core::PCWSTR = windows_core::w!("SW56");
pub const MPRDT_Serial: windows_core::PCWSTR = windows_core::w!("SERIAL");
pub const MPRDT_Sonet: windows_core::PCWSTR = windows_core::w!("SONET");
pub const MPRDT_Vpn: windows_core::PCWSTR = windows_core::w!("vpn");
pub const MPRDT_X25: windows_core::PCWSTR = windows_core::w!("x25");
pub const MPRET_Direct: u32 = 3u32;
pub const MPRET_Phone: u32 = 1u32;
pub const MPRET_Vpn: u32 = 2u32;
pub const MPRIDS_Disabled: u32 = 4294967295u32;
pub const MPRIDS_UseGlobalValue: u32 = 0u32;
pub const MPRIO_DisableLcpExtensions: u32 = 32u32;
pub const MPRIO_IpHeaderCompression: u32 = 8u32;
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648u32;
pub const MPRIO_NetworkLogon: u32 = 8192u32;
pub const MPRIO_PromoteAlternates: u32 = 32768u32;
pub const MPRIO_RemoteDefaultGateway: u32 = 16u32;
pub const MPRIO_RequireCHAP: u32 = 134217728u32;
pub const MPRIO_RequireDataEncryption: u32 = 4096u32;
pub const MPRIO_RequireEAP: u32 = 131072u32;
pub const MPRIO_RequireEncryptedPw: u32 = 1024u32;
pub const MPRIO_RequireMachineCertificates: u32 = 16777216u32;
pub const MPRIO_RequireMsCHAP: u32 = 268435456u32;
pub const MPRIO_RequireMsCHAP2: u32 = 536870912u32;
pub const MPRIO_RequireMsEncryptedPw: u32 = 2048u32;
pub const MPRIO_RequirePAP: u32 = 262144u32;
pub const MPRIO_RequireSPAP: u32 = 524288u32;
pub const MPRIO_SecureLocalFiles: u32 = 65536u32;
pub const MPRIO_SharedPhoneNumbers: u32 = 8388608u32;
pub const MPRIO_SpecificIpAddr: u32 = 2u32;
pub const MPRIO_SpecificNameServers: u32 = 4u32;
pub const MPRIO_SwCompression: u32 = 512u32;
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: u32 = 33554432u32;
pub const MPRIO_UsePreSharedKeyForIkev2Responder: u32 = 67108864u32;
pub const MPRNP_Ip: u32 = 4u32;
pub const MPRNP_Ipv6: u32 = 8u32;
pub const MPRNP_Ipx: u32 = 2u32;
pub const MPR_ENABLE_RAS_ON_DEVICE: u32 = 1u32;
pub const MPR_ENABLE_ROUTING_ON_DEVICE: u32 = 2u32;
pub const MPR_ET_None: MPR_ET = MPR_ET(0u32);
pub const MPR_ET_Optional: MPR_ET = MPR_ET(3u32);
pub const MPR_ET_Require: MPR_ET = MPR_ET(1u32);
pub const MPR_ET_RequireMax: MPR_ET = MPR_ET(2u32);
pub const MPR_INTERFACE_ADMIN_DISABLED: u32 = 2u32;
pub const MPR_INTERFACE_CONNECTION_FAILURE: u32 = 4u32;
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: u32 = 16u32;
pub const MPR_INTERFACE_NO_DEVICE: u32 = 64u32;
pub const MPR_INTERFACE_NO_MEDIA_SENSE: u32 = 32u32;
pub const MPR_INTERFACE_OUT_OF_RESOURCES: u32 = 1u32;
pub const MPR_INTERFACE_SERVICE_PAUSED: u32 = 8u32;
pub const MPR_MaxAreaCode: u32 = 10u32;
pub const MPR_MaxCallbackNumber: u32 = 128u32;
pub const MPR_MaxDeviceName: u32 = 128u32;
pub const MPR_MaxDeviceType: u32 = 16u32;
pub const MPR_MaxEntryName: u32 = 256u32;
pub const MPR_MaxFacilities: u32 = 200u32;
pub const MPR_MaxIpAddress: u32 = 15u32;
pub const MPR_MaxIpxAddress: u32 = 21u32;
pub const MPR_MaxPadType: u32 = 32u32;
pub const MPR_MaxPhoneNumber: u32 = 128u32;
pub const MPR_MaxUserData: u32 = 200u32;
pub const MPR_MaxX25Address: u32 = 200u32;
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(7i32);
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(8i32);
pub const MPR_VS_Default: MPR_VS = MPR_VS(0u32);
pub const MPR_VS_Ikev2First: u32 = 8u32;
pub const MPR_VS_Ikev2Only: u32 = 7u32;
pub const MPR_VS_L2tpFirst: MPR_VS = MPR_VS(4u32);
pub const MPR_VS_L2tpOnly: MPR_VS = MPR_VS(3u32);
pub const MPR_VS_PptpFirst: MPR_VS = MPR_VS(2u32);
pub const MPR_VS_PptpOnly: MPR_VS = MPR_VS(1u32);
pub const PENDING: u32 = 600u32;
pub const PID_ATALK: u32 = 41u32;
pub const PID_IP: u32 = 33u32;
pub const PID_IPV6: u32 = 87u32;
pub const PID_IPX: u32 = 43u32;
pub const PID_NBF: u32 = 63u32;
pub const PPP_CCP_COMPRESSION: u32 = 1u32;
pub const PPP_CCP_ENCRYPTION128BIT: u32 = 64u32;
pub const PPP_CCP_ENCRYPTION40BIT: u32 = 32u32;
pub const PPP_CCP_ENCRYPTION40BITOLD: u32 = 16u32;
pub const PPP_CCP_ENCRYPTION56BIT: u32 = 128u32;
pub const PPP_CCP_HISTORYLESS: u32 = 16777216u32;
pub const PPP_IPCP_VJ: u32 = 1u32;
pub const PPP_LCP_3_DES: u32 = 32u32;
pub const PPP_LCP_ACFC: u32 = 4u32;
pub const PPP_LCP_AES_128: u32 = 64u32;
pub const PPP_LCP_AES_192: u32 = 256u32;
pub const PPP_LCP_AES_256: u32 = 128u32;
pub const PPP_LCP_CHAP: PPP_LCP = PPP_LCP(49699u32);
pub const PPP_LCP_CHAP_MD5: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(5u32);
pub const PPP_LCP_CHAP_MS: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(128u32);
pub const PPP_LCP_CHAP_MSV2: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(129u32);
pub const PPP_LCP_DES_56: u32 = 16u32;
pub const PPP_LCP_EAP: PPP_LCP = PPP_LCP(49703u32);
pub const PPP_LCP_GCM_AES_128: u32 = 512u32;
pub const PPP_LCP_GCM_AES_192: u32 = 1024u32;
pub const PPP_LCP_GCM_AES_256: u32 = 2048u32;
pub const PPP_LCP_MULTILINK_FRAMING: u32 = 1u32;
pub const PPP_LCP_PAP: PPP_LCP = PPP_LCP(49187u32);
pub const PPP_LCP_PFC: u32 = 2u32;
pub const PPP_LCP_SPAP: PPP_LCP = PPP_LCP(49191u32);
pub const PPP_LCP_SSHF: u32 = 8u32;
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(2i32);
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(1i32);
pub const RASADFLG_PositionDlg: u32 = 1u32;
pub const RASADP_ConnectionQueryTimeout: u32 = 4u32;
pub const RASADP_DisableConnectionQuery: u32 = 0u32;
pub const RASADP_FailedConnectionTimeout: u32 = 3u32;
pub const RASADP_LoginSessionDisable: u32 = 1u32;
pub const RASADP_SavedAddressesLimit: u32 = 2u32;
pub const RASAPIVERSION_500: RASAPIVERSION = RASAPIVERSION(1i32);
pub const RASAPIVERSION_501: RASAPIVERSION = RASAPIVERSION(2i32);
pub const RASAPIVERSION_600: RASAPIVERSION = RASAPIVERSION(3i32);
pub const RASAPIVERSION_601: RASAPIVERSION = RASAPIVERSION(4i32);
pub const RASBASE: u32 = 600u32;
pub const RASBASEEND: u32 = 877u32;
pub const RASCCPCA_MPPC: u32 = 6u32;
pub const RASCCPCA_STAC: u32 = 5u32;
pub const RASCCPO_Compression: u32 = 1u32;
pub const RASCCPO_Encryption128bit: u32 = 64u32;
pub const RASCCPO_Encryption40bit: u32 = 32u32;
pub const RASCCPO_Encryption56bit: u32 = 16u32;
pub const RASCCPO_HistoryLess: u32 = 2u32;
pub const RASCF_AllUsers: u32 = 1u32;
pub const RASCF_GlobalCreds: u32 = 2u32;
pub const RASCF_OwnerKnown: u32 = 4u32;
pub const RASCF_OwnerMatch: u32 = 8u32;
pub const RASCM_DDMPreSharedKey: u32 = 64u32;
pub const RASCM_DefaultCreds: u32 = 8u32;
pub const RASCM_Domain: u32 = 4u32;
pub const RASCM_Password: u32 = 2u32;
pub const RASCM_PreSharedKey: u32 = 16u32;
pub const RASCM_ServerPreSharedKey: u32 = 32u32;
pub const RASCM_UserName: u32 = 1u32;
pub const RASCN_BandwidthAdded: u32 = 4u32;
pub const RASCN_BandwidthRemoved: u32 = 8u32;
pub const RASCN_Connection: u32 = 1u32;
pub const RASCN_Disconnection: u32 = 2u32;
pub const RASCN_Dormant: u32 = 16u32;
pub const RASCN_EPDGPacketArrival: u32 = 64u32;
pub const RASCN_ReConnection: u32 = 32u32;
pub const RASCSS_DONE: u32 = 8192u32;
pub const RASCSS_Dormant: RASCONNSUBSTATE = RASCONNSUBSTATE(1i32);
pub const RASCSS_None: RASCONNSUBSTATE = RASCONNSUBSTATE(0i32);
pub const RASCSS_Reconnected: RASCONNSUBSTATE = RASCONNSUBSTATE(8192i32);
pub const RASCSS_Reconnecting: RASCONNSUBSTATE = RASCONNSUBSTATE(2i32);
pub const RASCS_AllDevicesConnected: RASCONNSTATE = RASCONNSTATE(4i32);
pub const RASCS_ApplySettings: RASCONNSTATE = RASCONNSTATE(24i32);
pub const RASCS_AuthAck: RASCONNSTATE = RASCONNSTATE(12i32);
pub const RASCS_AuthCallback: RASCONNSTATE = RASCONNSTATE(8i32);
pub const RASCS_AuthChangePassword: RASCONNSTATE = RASCONNSTATE(9i32);
pub const RASCS_AuthLinkSpeed: RASCONNSTATE = RASCONNSTATE(11i32);
pub const RASCS_AuthNotify: RASCONNSTATE = RASCONNSTATE(6i32);
pub const RASCS_AuthProject: RASCONNSTATE = RASCONNSTATE(10i32);
pub const RASCS_AuthRetry: RASCONNSTATE = RASCONNSTATE(7i32);
pub const RASCS_Authenticate: RASCONNSTATE = RASCONNSTATE(5i32);
pub const RASCS_Authenticated: RASCONNSTATE = RASCONNSTATE(14i32);
pub const RASCS_CallbackComplete: RASCONNSTATE = RASCONNSTATE(20i32);
pub const RASCS_CallbackSetByCaller: RASCONNSTATE = RASCONNSTATE(4098i32);
pub const RASCS_ConnectDevice: RASCONNSTATE = RASCONNSTATE(2i32);
pub const RASCS_Connected: RASCONNSTATE = RASCONNSTATE(8192i32);
pub const RASCS_DONE: u32 = 8192u32;
pub const RASCS_DeviceConnected: RASCONNSTATE = RASCONNSTATE(3i32);
pub const RASCS_Disconnected: RASCONNSTATE = RASCONNSTATE(8193i32);
pub const RASCS_Interactive: RASCONNSTATE = RASCONNSTATE(4096i32);
pub const RASCS_InvokeEapUI: RASCONNSTATE = RASCONNSTATE(4100i32);
pub const RASCS_LogonNetwork: RASCONNSTATE = RASCONNSTATE(21i32);
pub const RASCS_OpenPort: RASCONNSTATE = RASCONNSTATE(0i32);
pub const RASCS_PAUSED: u32 = 4096u32;
pub const RASCS_PasswordExpired: RASCONNSTATE = RASCONNSTATE(4099i32);
pub const RASCS_PortOpened: RASCONNSTATE = RASCONNSTATE(1i32);
pub const RASCS_PrepareForCallback: RASCONNSTATE = RASCONNSTATE(15i32);
pub const RASCS_Projected: RASCONNSTATE = RASCONNSTATE(18i32);
pub const RASCS_ReAuthenticate: RASCONNSTATE = RASCONNSTATE(13i32);
pub const RASCS_RetryAuthentication: RASCONNSTATE = RASCONNSTATE(4097i32);
pub const RASCS_StartAuthentication: RASCONNSTATE = RASCONNSTATE(19i32);
pub const RASCS_SubEntryConnected: RASCONNSTATE = RASCONNSTATE(22i32);
pub const RASCS_SubEntryDisconnected: RASCONNSTATE = RASCONNSTATE(23i32);
pub const RASCS_WaitForCallback: RASCONNSTATE = RASCONNSTATE(17i32);
pub const RASCS_WaitForModemReset: RASCONNSTATE = RASCONNSTATE(16i32);
pub const RASDDFLAG_AoacRedial: u32 = 4u32;
pub const RASDDFLAG_LinkFailure: u32 = 2147483648u32;
pub const RASDDFLAG_NoPrompt: u32 = 2u32;
pub const RASDDFLAG_PositionDlg: u32 = 1u32;
pub const RASDIALEVENT: windows_core::PCSTR = windows_core::s!("RasDialEvent");
pub const RASDT_Atm: windows_core::PCWSTR = windows_core::w!("ATM");
pub const RASDT_FrameRelay: windows_core::PCWSTR = windows_core::w!("FRAMERELAY");
pub const RASDT_Generic: windows_core::PCWSTR = windows_core::w!("GENERIC");
pub const RASDT_Irda: windows_core::PCWSTR = windows_core::w!("IRDA");
pub const RASDT_Isdn: windows_core::PCWSTR = windows_core::w!("isdn");
pub const RASDT_Modem: windows_core::PCWSTR = windows_core::w!("modem");
pub const RASDT_PPPoE: windows_core::PCWSTR = windows_core::w!("PPPoE");
pub const RASDT_Pad: windows_core::PCWSTR = windows_core::w!("pad");
pub const RASDT_Parallel: windows_core::PCWSTR = windows_core::w!("PARALLEL");
pub const RASDT_SW56: windows_core::PCWSTR = windows_core::w!("SW56");
pub const RASDT_Serial: windows_core::PCWSTR = windows_core::w!("SERIAL");
pub const RASDT_Sonet: windows_core::PCWSTR = windows_core::w!("SONET");
pub const RASDT_Vpn: windows_core::PCWSTR = windows_core::w!("vpn");
pub const RASDT_X25: windows_core::PCWSTR = windows_core::w!("x25");
pub const RASEAPF_Logon: u32 = 4u32;
pub const RASEAPF_NonInteractive: u32 = 2u32;
pub const RASEAPF_Preview: u32 = 8u32;
pub const RASEDFLAG_CloneEntry: u32 = 4u32;
pub const RASEDFLAG_IncomingConnection: u32 = 1024u32;
pub const RASEDFLAG_InternetEntry: u32 = 256u32;
pub const RASEDFLAG_NAT: u32 = 512u32;
pub const RASEDFLAG_NewBroadbandEntry: u32 = 128u32;
pub const RASEDFLAG_NewDirectEntry: u32 = 64u32;
pub const RASEDFLAG_NewEntry: u32 = 2u32;
pub const RASEDFLAG_NewPhoneEntry: u32 = 16u32;
pub const RASEDFLAG_NewTunnelEntry: u32 = 32u32;
pub const RASEDFLAG_NoRename: u32 = 8u32;
pub const RASEDFLAG_PositionDlg: u32 = 1u32;
pub const RASEDFLAG_ShellOwned: u32 = 1073741824u32;
pub const RASEDM_DialAll: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(1u32);
pub const RASEDM_DialAsNeeded: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(2u32);
pub const RASEO2_AuthTypeIsOtp: u32 = 268435456u32;
pub const RASEO2_AutoTriggerCapable: u32 = 67108864u32;
pub const RASEO2_CacheCredentials: u32 = 33554432u32;
pub const RASEO2_DisableClassBasedStaticRoute: u32 = 524288u32;
pub const RASEO2_DisableIKENameEkuCheck: u32 = 262144u32;
pub const RASEO2_DisableMobility: u32 = 2097152u32;
pub const RASEO2_DisableNbtOverIP: u32 = 64u32;
pub const RASEO2_DontNegotiateMultilink: u32 = 4u32;
pub const RASEO2_DontUseRasCredentials: u32 = 8u32;
pub const RASEO2_IPv4ExplicitMetric: u32 = 65536u32;
pub const RASEO2_IPv6ExplicitMetric: u32 = 131072u32;
pub const RASEO2_IPv6RemoteDefaultGateway: u32 = 8192u32;
pub const RASEO2_IPv6SpecificNameServers: u32 = 4096u32;
pub const RASEO2_Internet: u32 = 32u32;
pub const RASEO2_IsAlwaysOn: u32 = 536870912u32;
pub const RASEO2_IsPrivateNetwork: u32 = 1073741824u32;
pub const RASEO2_IsThirdPartyProfile: u32 = 134217728u32;
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648u32;
pub const RASEO2_ReconnectIfDropped: u32 = 256u32;
pub const RASEO2_RegisterIpWithDNS: u32 = 16384u32;
pub const RASEO2_RequireMachineCertificates: u32 = 4194304u32;
pub const RASEO2_SecureClientForMSNet: u32 = 2u32;
pub const RASEO2_SecureFileAndPrint: u32 = 1u32;
pub const RASEO2_SecureRoutingCompartment: u32 = 1024u32;
pub const RASEO2_SharePhoneNumbers: u32 = 512u32;
pub const RASEO2_SpecificIPv6Addr: u32 = 1048576u32;
pub const RASEO2_UseDNSSuffixForRegistration: u32 = 32768u32;
pub const RASEO2_UseGlobalDeviceSettings: u32 = 128u32;
pub const RASEO2_UsePreSharedKey: u32 = 16u32;
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: u32 = 8388608u32;
pub const RASEO2_UsePreSharedKeyForIkev2Responder: u32 = 16777216u32;
pub const RASEO2_UseTypicalSettings: u32 = 2048u32;
pub const RASEO_Custom: u32 = 1048576u32;
pub const RASEO_CustomScript: u32 = 2147483648u32;
pub const RASEO_DisableLcpExtensions: u32 = 32u32;
pub const RASEO_IpHeaderCompression: u32 = 8u32;
pub const RASEO_ModemLights: u32 = 256u32;
pub const RASEO_NetworkLogon: u32 = 8192u32;
pub const RASEO_PreviewDomain: u32 = 33554432u32;
pub const RASEO_PreviewPhoneNumber: u32 = 2097152u32;
pub const RASEO_PreviewUserPw: u32 = 16777216u32;
pub const RASEO_PromoteAlternates: u32 = 32768u32;
pub const RASEO_RemoteDefaultGateway: u32 = 16u32;
pub const RASEO_RequireCHAP: u32 = 134217728u32;
pub const RASEO_RequireDataEncryption: u32 = 4096u32;
pub const RASEO_RequireEAP: u32 = 131072u32;
pub const RASEO_RequireEncryptedPw: u32 = 1024u32;
pub const RASEO_RequireMsCHAP: u32 = 268435456u32;
pub const RASEO_RequireMsCHAP2: u32 = 536870912u32;
pub const RASEO_RequireMsEncryptedPw: u32 = 2048u32;
pub const RASEO_RequirePAP: u32 = 262144u32;
pub const RASEO_RequireSPAP: u32 = 524288u32;
pub const RASEO_RequireW95MSCHAP: u32 = 1073741824u32;
pub const RASEO_SecureLocalFiles: u32 = 65536u32;
pub const RASEO_SharedPhoneNumbers: u32 = 8388608u32;
pub const RASEO_ShowDialingProgress: u32 = 67108864u32;
pub const RASEO_SpecificIpAddr: u32 = 2u32;
pub const RASEO_SpecificNameServers: u32 = 4u32;
pub const RASEO_SwCompression: u32 = 512u32;
pub const RASEO_TerminalAfterDial: u32 = 128u32;
pub const RASEO_TerminalBeforeDial: u32 = 64u32;
pub const RASEO_UseCountryAndAreaCodes: u32 = 1u32;
pub const RASEO_UseLogonCredentials: u32 = 16384u32;
pub const RASET_Broadband: u32 = 5u32;
pub const RASET_Direct: u32 = 3u32;
pub const RASET_Internet: u32 = 4u32;
pub const RASET_Phone: u32 = 1u32;
pub const RASET_Vpn: u32 = 2u32;
pub const RASFP_Ppp: u32 = 1u32;
pub const RASFP_Ras: u32 = 4u32;
pub const RASFP_Slip: u32 = 2u32;
pub const RASIDS_Disabled: u32 = 4294967295u32;
pub const RASIDS_UseGlobalValue: u32 = 0u32;
pub const RASIKEv2_AUTH_EAP: u32 = 2u32;
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: u32 = 1u32;
pub const RASIKEv2_AUTH_PSK: u32 = 3u32;
pub const RASIKEv2_FLAGS_BEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(2u32);
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(1u32);
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(4u32);
pub const RASIPO_VJ: u32 = 1u32;
pub const RASLCPAD_CHAP_MD5: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(5u32);
pub const RASLCPAD_CHAP_MS: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(128u32);
pub const RASLCPAD_CHAP_MSV2: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(129u32);
pub const RASLCPAP_CHAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49699u32);
pub const RASLCPAP_EAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49703u32);
pub const RASLCPAP_PAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49187u32);
pub const RASLCPAP_SPAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49191u32);
pub const RASLCPO_3_DES: u32 = 16u32;
pub const RASLCPO_ACFC: u32 = 2u32;
pub const RASLCPO_AES_128: u32 = 32u32;
pub const RASLCPO_AES_192: u32 = 128u32;
pub const RASLCPO_AES_256: u32 = 64u32;
pub const RASLCPO_DES_56: u32 = 8u32;
pub const RASLCPO_GCM_AES_128: u32 = 256u32;
pub const RASLCPO_GCM_AES_192: u32 = 512u32;
pub const RASLCPO_GCM_AES_256: u32 = 1024u32;
pub const RASLCPO_PFC: u32 = 1u32;
pub const RASLCPO_SSHF: u32 = 4u32;
pub const RASNAP_ProbationTime: u32 = 1u32;
pub const RASNOUSER_SmartCard: u32 = 1u32;
pub const RASNP_Ip: u32 = 4u32;
pub const RASNP_Ipv6: u32 = 8u32;
pub const RASNP_Ipx: u32 = 2u32;
pub const RASNP_NetBEUI: u32 = 1u32;
pub const RASPBDEVENT_AddEntry: u32 = 1u32;
pub const RASPBDEVENT_DialEntry: u32 = 4u32;
pub const RASPBDEVENT_EditEntry: u32 = 2u32;
pub const RASPBDEVENT_EditGlobals: u32 = 5u32;
pub const RASPBDEVENT_NoUser: u32 = 6u32;
pub const RASPBDEVENT_NoUserEdit: u32 = 7u32;
pub const RASPBDEVENT_RemoveEntry: u32 = 3u32;
pub const RASPBDFLAG_ForceCloseOnDial: u32 = 2u32;
pub const RASPBDFLAG_NoUser: u32 = 16u32;
pub const RASPBDFLAG_PositionDlg: u32 = 1u32;
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648u32;
pub const RASPRIV2_DialinPolicy: u32 = 1u32;
pub const RASPRIV_AdminSetCallback: u32 = 2u32;
pub const RASPRIV_CallerSetCallback: u32 = 4u32;
pub const RASPRIV_DialinPrivilege: u32 = 8u32;
pub const RASPRIV_NoCallback: u32 = 1u32;
pub const RASP_Amb: RASPROJECTION = RASPROJECTION(65536i32);
pub const RASP_PppCcp: RASPROJECTION = RASPROJECTION(33021i32);
pub const RASP_PppIp: RASPROJECTION = RASPROJECTION(32801i32);
pub const RASP_PppIpv6: RASPROJECTION = RASPROJECTION(32855i32);
pub const RASP_PppIpx: RASPROJECTION = RASPROJECTION(32811i32);
pub const RASP_PppLcp: RASPROJECTION = RASPROJECTION(49185i32);
pub const RASP_PppNbf: RASPROJECTION = RASPROJECTION(32831i32);
pub const RASTUNNELENDPOINT_IPv4: u32 = 1u32;
pub const RASTUNNELENDPOINT_IPv6: u32 = 2u32;
pub const RASTUNNELENDPOINT_UNKNOWN: u32 = 0u32;
pub const RAS_FLAGS_ARAP_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_DORMANT: RAS_FLAGS = RAS_FLAGS(32u32);
pub const RAS_FLAGS_IKEV2_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_MESSENGER_PRESENT: RAS_FLAGS = RAS_FLAGS(2u32);
pub const RAS_FLAGS_PPP_CONNECTION: RAS_FLAGS = RAS_FLAGS(1u32);
pub const RAS_FLAGS_QUARANTINE_PRESENT: RAS_FLAGS = RAS_FLAGS(8u32);
pub const RAS_FLAGS_RAS_CONNECTION: u32 = 4u32;
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(1i32);
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(0i32);
pub const RAS_MaxAreaCode: u32 = 10u32;
pub const RAS_MaxCallbackNumber: u32 = 128u32;
pub const RAS_MaxDeviceName: u32 = 128u32;
pub const RAS_MaxDeviceType: u32 = 16u32;
pub const RAS_MaxDnsSuffix: u32 = 256u32;
pub const RAS_MaxEntryName: u32 = 256u32;
pub const RAS_MaxFacilities: u32 = 200u32;
pub const RAS_MaxIDSize: u32 = 256u32;
pub const RAS_MaxIpAddress: u32 = 15u32;
pub const RAS_MaxIpxAddress: u32 = 21u32;
pub const RAS_MaxPadType: u32 = 32u32;
pub const RAS_MaxPhoneNumber: u32 = 128u32;
pub const RAS_MaxReplyMessage: u32 = 1024u32;
pub const RAS_MaxUserData: u32 = 200u32;
pub const RAS_MaxX25Address: u32 = 200u32;
pub const RAS_PORT_AUTHENTICATED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(5i32);
pub const RAS_PORT_AUTHENTICATING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(4i32);
pub const RAS_PORT_CALLING_BACK: RAS_PORT_CONDITION = RAS_PORT_CONDITION(2i32);
pub const RAS_PORT_DISCONNECTED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(1i32);
pub const RAS_PORT_INITIALIZING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(6i32);
pub const RAS_PORT_LISTENING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(3i32);
pub const RAS_PORT_NON_OPERATIONAL: RAS_PORT_CONDITION = RAS_PORT_CONDITION(0i32);
pub const RAS_QUAR_STATE_NORMAL: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(0i32);
pub const RAS_QUAR_STATE_NOT_CAPABLE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(3i32);
pub const RAS_QUAR_STATE_PROBATION: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(2i32);
pub const RAS_QUAR_STATE_QUARANTINE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(1i32);
pub const RCD_AllUsers: u32 = 1u32;
pub const RCD_Eap: u32 = 2u32;
pub const RCD_Logon: u32 = 4u32;
pub const RCD_SingleUser: u32 = 0u32;
pub const RDEOPT_CustomDial: u32 = 4096u32;
pub const RDEOPT_DisableConnectedUI: u32 = 64u32;
pub const RDEOPT_DisableReconnect: u32 = 256u32;
pub const RDEOPT_DisableReconnectUI: u32 = 128u32;
pub const RDEOPT_EapInfoCryptInCapable: u32 = 32768u32;
pub const RDEOPT_IgnoreModemSpeaker: u32 = 4u32;
pub const RDEOPT_IgnoreSoftwareCompression: u32 = 16u32;
pub const RDEOPT_InvokeAutoTriggerCredentialUI: u32 = 16384u32;
pub const RDEOPT_NoUser: u32 = 512u32;
pub const RDEOPT_PauseOnScript: u32 = 1024u32;
pub const RDEOPT_PausedStates: u32 = 2u32;
pub const RDEOPT_Router: u32 = 2048u32;
pub const RDEOPT_SetModemSpeaker: u32 = 8u32;
pub const RDEOPT_SetSoftwareCompression: u32 = 32u32;
pub const RDEOPT_UseCustomScripting: u32 = 8192u32;
pub const RDEOPT_UsePrefixSuffix: u32 = 1u32;
pub const REN_AllUsers: u32 = 1u32;
pub const REN_User: u32 = 0u32;
pub const ROUTER_IF_STATE_CONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(3i32);
pub const ROUTER_IF_STATE_CONNECTING: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(2i32);
pub const ROUTER_IF_STATE_DISCONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(1i32);
pub const ROUTER_IF_STATE_UNREACHABLE: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(0i32);
pub const ROUTER_IF_TYPE_CLIENT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(0i32);
pub const ROUTER_IF_TYPE_DEDICATED: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(3i32);
pub const ROUTER_IF_TYPE_DIALOUT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(7i32);
pub const ROUTER_IF_TYPE_FULL_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(2i32);
pub const ROUTER_IF_TYPE_HOME_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(1i32);
pub const ROUTER_IF_TYPE_INTERNAL: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(4i32);
pub const ROUTER_IF_TYPE_LOOPBACK: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(5i32);
pub const ROUTER_IF_TYPE_MAX: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(8i32);
pub const ROUTER_IF_TYPE_TUNNEL1: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(6i32);
pub const RRAS_SERVICE_NAME: windows_core::PCWSTR = windows_core::w!("RemoteAccess");
pub const RTM_BLOCK_METHODS: u32 = 1u32;
pub const RTM_CHANGE_NOTIFICATION: RTM_EVENT_TYPE = RTM_EVENT_TYPE(3i32);
pub const RTM_CHANGE_TYPE_ALL: u32 = 1u32;
pub const RTM_CHANGE_TYPE_BEST: u32 = 2u32;
pub const RTM_CHANGE_TYPE_FORWARDING: u32 = 4u32;
pub const RTM_DEST_FLAG_DONT_FORWARD: u32 = 4u32;
pub const RTM_DEST_FLAG_FWD_ENGIN_ADD: u32 = 2u32;
pub const RTM_DEST_FLAG_NATURAL_NET: u32 = 1u32;
pub const RTM_ENTITY_DEREGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(1i32);
pub const RTM_ENTITY_REGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(0i32);
pub const RTM_ENUM_ALL_DESTS: u32 = 0u32;
pub const RTM_ENUM_ALL_ROUTES: u32 = 0u32;
pub const RTM_ENUM_NEXT: u32 = 1u32;
pub const RTM_ENUM_OWN_DESTS: u32 = 16777216u32;
pub const RTM_ENUM_OWN_ROUTES: u32 = 65536u32;
pub const RTM_ENUM_RANGE: u32 = 2u32;
pub const RTM_ENUM_START: u32 = 0u32;
pub const RTM_MATCH_FULL: u32 = 65535u32;
pub const RTM_MATCH_INTERFACE: u32 = 16u32;
pub const RTM_MATCH_NEIGHBOUR: u32 = 2u32;
pub const RTM_MATCH_NEXTHOP: u32 = 8u32;
pub const RTM_MATCH_NONE: u32 = 0u32;
pub const RTM_MATCH_OWNER: u32 = 1u32;
pub const RTM_MATCH_PREF: u32 = 4u32;
pub const RTM_MAX_ADDRESS_SIZE: u32 = 16u32;
pub const RTM_MAX_VIEWS: u32 = 32u32;
pub const RTM_NEXTHOP_CHANGE_NEW: u32 = 1u32;
pub const RTM_NEXTHOP_FLAGS_DOWN: u32 = 2u32;
pub const RTM_NEXTHOP_FLAGS_REMOTE: u32 = 1u32;
pub const RTM_NEXTHOP_STATE_CREATED: u32 = 0u32;
pub const RTM_NEXTHOP_STATE_DELETED: u32 = 1u32;
pub const RTM_NOTIFY_ONLY_MARKED_DESTS: u32 = 65536u32;
pub const RTM_NUM_CHANGE_TYPES: u32 = 3u32;
pub const RTM_RESUME_METHODS: u32 = 0u32;
pub const RTM_ROUTE_CHANGE_BEST: u32 = 65536u32;
pub const RTM_ROUTE_CHANGE_FIRST: u32 = 1u32;
pub const RTM_ROUTE_CHANGE_NEW: u32 = 2u32;
pub const RTM_ROUTE_EXPIRED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(2i32);
pub const RTM_ROUTE_FLAGS_BLACKHOLE: u32 = 2u32;
pub const RTM_ROUTE_FLAGS_DISCARD: u32 = 4u32;
pub const RTM_ROUTE_FLAGS_INACTIVE: u32 = 8u32;
pub const RTM_ROUTE_FLAGS_LIMITED_BC: u32 = 1024u32;
pub const RTM_ROUTE_FLAGS_LOCAL: u32 = 16u32;
pub const RTM_ROUTE_FLAGS_LOCAL_MCAST: u32 = 512u32;
pub const RTM_ROUTE_FLAGS_LOOPBACK: u32 = 128u32;
pub const RTM_ROUTE_FLAGS_MARTIAN: u32 = 1u32;
pub const RTM_ROUTE_FLAGS_MCAST: u32 = 256u32;
pub const RTM_ROUTE_FLAGS_MYSELF: u32 = 64u32;
pub const RTM_ROUTE_FLAGS_ONES_NETBC: u32 = 16384u32;
pub const RTM_ROUTE_FLAGS_ONES_SUBNETBC: u32 = 32768u32;
pub const RTM_ROUTE_FLAGS_REMOTE: u32 = 32u32;
pub const RTM_ROUTE_FLAGS_ZEROS_NETBC: u32 = 4096u32;
pub const RTM_ROUTE_FLAGS_ZEROS_SUBNETBC: u32 = 8192u32;
pub const RTM_ROUTE_STATE_CREATED: u32 = 0u32;
pub const RTM_ROUTE_STATE_DELETED: u32 = 2u32;
pub const RTM_ROUTE_STATE_DELETING: u32 = 1u32;
pub const RTM_VIEW_ID_MCAST: u32 = 1u32;
pub const RTM_VIEW_ID_UCAST: u32 = 0u32;
pub const RTM_VIEW_MASK_ALL: u32 = 4294967295u32;
pub const RTM_VIEW_MASK_ANY: u32 = 0u32;
pub const RTM_VIEW_MASK_MCAST: u32 = 2u32;
pub const RTM_VIEW_MASK_NONE: u32 = 0u32;
pub const RTM_VIEW_MASK_SIZE: u32 = 32u32;
pub const RTM_VIEW_MASK_UCAST: u32 = 1u32;
pub const SECURITYMSG_ERROR: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(3u32);
pub const SECURITYMSG_FAILURE: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(2u32);
pub const SECURITYMSG_SUCCESS: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(1u32);
pub const VS_Default: u32 = 0u32;
pub const VS_GREOnly: u32 = 9u32;
pub const VS_Ikev2First: u32 = 8u32;
pub const VS_Ikev2Only: u32 = 7u32;
pub const VS_Ikev2Sstp: u32 = 14u32;
pub const VS_L2tpFirst: u32 = 4u32;
pub const VS_L2tpOnly: u32 = 3u32;
pub const VS_L2tpSstp: u32 = 13u32;
pub const VS_PptpFirst: u32 = 2u32;
pub const VS_PptpOnly: u32 = 1u32;
pub const VS_PptpSstp: u32 = 12u32;
pub const VS_ProtocolList: u32 = 15u32;
pub const VS_SstpFirst: u32 = 6u32;
pub const VS_SstpOnly: u32 = 5u32;
pub const WARNING_MSG_ALIAS_NOT_ADDED: u32 = 644u32;
pub const WM_RASDIALEVENT: u32 = 52429u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IKEV2_ID_PAYLOAD_TYPE(pub i32);
impl windows_core::TypeKind for IKEV2_ID_PAYLOAD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IKEV2_ID_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IKEV2_ID_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MGM_ENUM_TYPES(pub i32);
impl windows_core::TypeKind for MGM_ENUM_TYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MGM_ENUM_TYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MGM_ENUM_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MPRAPI_OBJECT_TYPE(pub i32);
impl windows_core::TypeKind for MPRAPI_OBJECT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MPRAPI_OBJECT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MPRAPI_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MPR_ET(pub u32);
impl windows_core::TypeKind for MPR_ET {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MPR_ET {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MPR_ET").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MPR_INTERFACE_DIAL_MODE(pub u32);
impl windows_core::TypeKind for MPR_INTERFACE_DIAL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MPR_INTERFACE_DIAL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MPR_INTERFACE_DIAL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MPR_VPN_TS_TYPE(pub i32);
impl windows_core::TypeKind for MPR_VPN_TS_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MPR_VPN_TS_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MPR_VPN_TS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MPR_VS(pub u32);
impl windows_core::TypeKind for MPR_VS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MPR_VS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MPR_VS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PPP_LCP(pub u32);
impl windows_core::TypeKind for PPP_LCP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PPP_LCP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PPP_LCP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PPP_LCP_INFO_AUTH_DATA(pub u32);
impl windows_core::TypeKind for PPP_LCP_INFO_AUTH_DATA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PPP_LCP_INFO_AUTH_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PPP_LCP_INFO_AUTH_DATA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASAPIVERSION(pub i32);
impl windows_core::TypeKind for RASAPIVERSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASAPIVERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASAPIVERSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASCONNSTATE(pub i32);
impl windows_core::TypeKind for RASCONNSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASCONNSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASCONNSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASCONNSUBSTATE(pub i32);
impl windows_core::TypeKind for RASCONNSUBSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASCONNSUBSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASCONNSUBSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASENTRY_DIAL_MODE(pub u32);
impl windows_core::TypeKind for RASENTRY_DIAL_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASENTRY_DIAL_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASENTRY_DIAL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASIKEV_PROJECTION_INFO_FLAGS(pub u32);
impl windows_core::TypeKind for RASIKEV_PROJECTION_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASIKEV_PROJECTION_INFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASIKEV_PROJECTION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl RASIKEV_PROJECTION_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(pub u32);
impl windows_core::TypeKind for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(pub u32);
impl windows_core::TypeKind for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASPROJECTION(pub i32);
impl windows_core::TypeKind for RASPROJECTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASPROJECTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASPROJECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASPROJECTION_INFO_TYPE(pub i32);
impl windows_core::TypeKind for RASPROJECTION_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASPROJECTION_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASPROJECTION_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAS_FLAGS(pub u32);
impl windows_core::TypeKind for RAS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAS_HARDWARE_CONDITION(pub i32);
impl windows_core::TypeKind for RAS_HARDWARE_CONDITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAS_HARDWARE_CONDITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAS_HARDWARE_CONDITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAS_PORT_CONDITION(pub i32);
impl windows_core::TypeKind for RAS_PORT_CONDITION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAS_PORT_CONDITION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAS_PORT_CONDITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RAS_QUARANTINE_STATE(pub i32);
impl windows_core::TypeKind for RAS_QUARANTINE_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RAS_QUARANTINE_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RAS_QUARANTINE_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ROUTER_CONNECTION_STATE(pub i32);
impl windows_core::TypeKind for ROUTER_CONNECTION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ROUTER_CONNECTION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ROUTER_CONNECTION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ROUTER_INTERFACE_TYPE(pub i32);
impl windows_core::TypeKind for ROUTER_INTERFACE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ROUTER_INTERFACE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ROUTER_INTERFACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RTM_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for RTM_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RTM_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RTM_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SECURITY_MESSAGE_MSG_ID(pub u32);
impl windows_core::TypeKind for SECURITY_MESSAGE_MSG_ID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SECURITY_MESSAGE_MSG_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SECURITY_MESSAGE_MSG_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTH_VALIDATION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub hRasConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub AuthInfoSize: u32,
    pub AuthInfo: [u8; 1],
}
impl windows_core::TypeKind for AUTH_VALIDATION_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl windows_core::TypeKind for GRE_CONFIG_PARAMS0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GRE_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRASCONN(pub *mut core::ffi::c_void);
impl HRASCONN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for HRASCONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HRASCONN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for IKEV2_CONFIG_PARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl windows_core::TypeKind for IKEV2_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl windows_core::TypeKind for IKEV2_PROJECTION_INFO2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS3 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS4 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwMmSaLifeTime: u32,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl windows_core::TypeKind for L2TP_CONFIG_PARAMS0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for L2TP_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
impl windows_core::TypeKind for L2TP_CONFIG_PARAMS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for L2TP_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
impl windows_core::TypeKind for L2TP_TUNNEL_CONFIG_PARAMS1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwMmSaLifeTime: u32,
}
impl windows_core::TypeKind for L2TP_TUNNEL_CONFIG_PARAMS2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MGM_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwIfNextHopAddr: u32,
    pub bIGMP: super::super::Foundation::BOOL,
    pub bIsEnabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for MGM_IF_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for MGM_IF_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug)]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: u8,
    pub lpfnMprAdminGetIpAddressForUser: PMPRADMINGETIPADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpAddress: PMPRADMINRELEASEIPADRESS,
    pub lpfnMprAdminGetIpv6AddressForUser: PMPRADMINGETIPV6ADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpV6AddressForUser: PMPRADMINRELEASEIPV6ADDRESSFORUSER,
    pub lpfnRasAdminAcceptNewLink: PMPRADMINACCEPTNEWLINK,
    pub lpfnRasAdminLinkHangupNotification: PMPRADMINLINKHANGUPNOTIFICATION,
    pub lpfnRasAdminTerminateDll: PMPRADMINTERMINATEDLL,
    pub lpfnRasAdminAcceptNewConnectionEx: PMPRADMINACCEPTNEWCONNECTIONEX,
    pub lpfnRasAdminAcceptEndpointChangeEx: PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX,
    pub lpfnRasAdminAcceptReauthenticationEx: PMPRADMINACCEPTREAUTHENTICATIONEX,
    pub lpfnRasAdminConnectionHangupNotificationEx: PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX,
    pub lpfnRASValidatePreAuthenticatedConnectionEx: PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for MPRAPI_ADMIN_DLL_CALLBACKS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: u8,
    pub r#type: u8,
    pub size: u16,
}
impl windows_core::TypeKind for MPRAPI_OBJECT_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPRAPI_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_CERT_EKU {
    pub dwSize: u32,
    pub IsEKUOID: super::super::Foundation::BOOL,
    pub pwszEKU: windows_core::PWSTR,
}
impl windows_core::TypeKind for MPR_CERT_EKU {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_CERT_EKU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl windows_core::TypeKind for MPR_CREDENTIALSEX_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_CREDENTIALSEX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl windows_core::TypeKind for MPR_CREDENTIALSEX_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_CREDENTIALSEX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl windows_core::TypeKind for MPR_DEVICE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
}
impl windows_core::TypeKind for MPR_DEVICE_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_FILTER_0 {
    pub fEnable: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for MPR_FILTER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_FILTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: u32,
    pub hIfTransport: super::super::Foundation::HANDLE,
    pub wszIfTransportName: [u16; 41],
}
impl windows_core::TypeKind for MPR_IFTRANSPORT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_IF_CUSTOMINFOEX0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_IF_CUSTOMINFOEX0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_IF_CUSTOMINFOEX1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_IF_CUSTOMINFOEX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl windows_core::TypeKind for MPR_IF_CUSTOMINFOEX2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl Default for MPR_IF_CUSTOMINFOEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_0 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
}
impl windows_core::TypeKind for MPR_INTERFACE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub lpwsDialoutHoursRestriction: windows_core::PWSTR,
}
impl windows_core::TypeKind for MPR_INTERFACE_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_INTERFACE_2 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: MPR_VS,
}
impl windows_core::TypeKind for MPR_INTERFACE_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MPR_INTERFACE_3 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: windows_core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: windows_core::GUID,
    pub dwVpnStrategy: MPR_VS,
    pub AddressCount: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addr: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for MPR_INTERFACE_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [u16; 257],
    pub Guid: windows_core::GUID,
}
impl windows_core::TypeKind for MPR_IPINIP_INTERFACE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: super::super::Foundation::BOOL,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
}
impl windows_core::TypeKind for MPR_SERVER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
}
impl windows_core::TypeKind for MPR_SERVER_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_SERVER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
    pub dwNumSstpPorts: u32,
    pub dwSstpPortFlags: u32,
}
impl windows_core::TypeKind for MPR_SERVER_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_SERVER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_SERVER_EX0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_SERVER_EX0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_SERVER_EX1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_SERVER_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_SERVER_SET_CONFIG_EX0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_SERVER_SET_CONFIG_EX0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for MPR_SERVER_SET_CONFIG_EX1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for MPR_SERVER_SET_CONFIG_EX1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: u32,
    pub hTransport: super::super::Foundation::HANDLE,
    pub wszTransportName: [u16; 41],
}
impl windows_core::TypeKind for MPR_TRANSPORT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct MPR_VPN_TRAFFIC_SELECTOR {
    pub r#type: MPR_VPN_TS_TYPE,
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub tsPayloadId: u16,
    pub addrStart: VPN_TS_IP_ADDRESS,
    pub addrEnd: VPN_TS_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for MPR_VPN_TRAFFIC_SELECTOR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_VPN_TRAFFIC_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: u32,
    pub numTsr: u32,
    pub tsI: *mut MPR_VPN_TRAFFIC_SELECTOR,
    pub tsR: *mut MPR_VPN_TRAFFIC_SELECTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for MPR_VPN_TRAFFIC_SELECTORS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for MPR_VPN_TRAFFIC_SELECTORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_ATCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 33],
}
impl windows_core::TypeKind for PPP_ATCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_CCP_INFO {
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwRemoteOptions: u32,
}
impl windows_core::TypeKind for PPP_CCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_CCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
impl windows_core::TypeKind for PPP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl windows_core::TypeKind for PPP_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl windows_core::TypeKind for PPP_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
}
impl windows_core::TypeKind for PPP_IPCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPCP_INFO2 {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
}
impl windows_core::TypeKind for PPP_IPCP_INFO2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPV6_CP_INFO {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
}
impl windows_core::TypeKind for PPP_IPV6_CP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_IPXCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 23],
}
impl windows_core::TypeKind for PPP_IPXCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_LCP_INFO {
    pub dwError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwTerminateReason: u32,
    pub dwRemoteTerminateReason: u32,
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
}
impl windows_core::TypeKind for PPP_LCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_LCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_NBFCP_INFO {
    pub dwError: u32,
    pub wszWksta: [u16; 17],
}
impl windows_core::TypeKind for PPP_NBFCP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl windows_core::TypeKind for PPP_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPP_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl windows_core::TypeKind for PPP_PROJECTION_INFO2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl windows_core::TypeKind for PPTP_CONFIG_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PPTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO_0,
}
impl windows_core::TypeKind for PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
impl windows_core::TypeKind for PROJECTION_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO2_0,
}
impl windows_core::TypeKind for PROJECTION_INFO2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
impl windows_core::TypeKind for PROJECTION_INFO2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
impl windows_core::TypeKind for RASADPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASADPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASAMBA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [i8; 17],
    pub bLana: u8,
}
impl windows_core::TypeKind for RASAMBA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASAMBA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASAMBW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [u16; 17],
    pub bLana: u8,
}
impl windows_core::TypeKind for RASAMBW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASAMBW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASAUTODIALENTRYA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [i8; 257],
}
impl windows_core::TypeKind for RASAUTODIALENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASAUTODIALENTRYW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [u16; 257],
}
impl windows_core::TypeKind for RASAUTODIALENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASCOMMSETTINGS {
    pub dwSize: u32,
    pub bParity: u8,
    pub bStop: u8,
    pub bByteSize: u8,
    pub bAlign: u8,
}
impl windows_core::TypeKind for RASCOMMSETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCOMMSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASCONNA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [i8; 257],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhonebook: [i8; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASCONNA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASCONNA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASCONNSTATUSA {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szPhoneNumber: [i8; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASCONNSTATUSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASCONNSTATUSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASCONNSTATUSW {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhoneNumber: [u16; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASCONNSTATUSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASCONNSTATUSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASCONNW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: windows_core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: windows_core::GUID,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASCONNW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASCONNW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASCREDENTIALSA {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl windows_core::TypeKind for RASCREDENTIALSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCREDENTIALSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASCREDENTIALSW {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl windows_core::TypeKind for RASCREDENTIALSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCREDENTIALSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASCTRYINFO {
    pub dwSize: u32,
    pub dwCountryID: u32,
    pub dwNextCountryID: u32,
    pub dwCountryCode: u32,
    pub dwCountryNameOffset: u32,
}
impl windows_core::TypeKind for RASCTRYINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCTRYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
impl windows_core::TypeKind for RASCUSTOMSCRIPTEXTENSIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCUSTOMSCRIPTEXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASDEVINFOA {
    pub dwSize: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
}
impl windows_core::TypeKind for RASDEVINFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASDEVINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASDEVINFOW {
    pub dwSize: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl windows_core::TypeKind for RASDEVINFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASDEVINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASDEVSPECIFICINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASDEVSPECIFICINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASDIALDLG {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwSubEntry: u32,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
impl windows_core::TypeKind for RASDIALDLG {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASDIALDLG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASDIALEXTENSIONS {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub reserved: usize,
    pub reserved1: usize,
    pub RasEapInfo: RASEAPINFO,
    pub fSkipPppAuth: super::super::Foundation::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
impl windows_core::TypeKind for RASDIALEXTENSIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASDIALPARAMSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub szPhoneNumber: [i8; 129],
    pub szCallbackNumber: [i8; 129],
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PSTR,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASDIALPARAMSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASDIALPARAMSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASDIALPARAMSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct RASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
impl windows_core::TypeKind for RASEAPINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASEAPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASEAPUSERIDENTITYA {
    pub szUserName: [i8; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl windows_core::TypeKind for RASEAPUSERIDENTITYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASEAPUSERIDENTITYW {
    pub szUserName: [u16; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl windows_core::TypeKind for RASEAPUSERIDENTITYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASENTRYA {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [i8; 11],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [i8; 260],
    pub szAutodialDll: [i8; 260],
    pub szAutodialFunc: [i8; 260],
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szX25PadType: [i8; 33],
    pub szX25Address: [i8; 201],
    pub szX25Facilities: [i8; 201],
    pub szX25UserData: [i8; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_core::GUID,
    pub szCustomDialDll: [i8; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [i8; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [i8; 260],
    pub szPrerequisiteEntry: [i8; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [i8; 257],
    pub szIDr: [i8; 257],
    pub fIsImsConfig: super::super::Foundation::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASENTRYA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASENTRYDLGA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [i8; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASENTRYDLGA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASENTRYDLGW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASENTRYDLGW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASENTRYNAMEA {
    pub dwSize: u32,
    pub szEntryName: [i8; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [i8; 261],
}
impl windows_core::TypeKind for RASENTRYNAMEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASENTRYNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASENTRYNAMEW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [u16; 261],
}
impl windows_core::TypeKind for RASENTRYNAMEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASENTRYNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASENTRYW {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [u16; 11],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [u16; 260],
    pub szAutodialDll: [u16; 260],
    pub szAutodialFunc: [u16; 260],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: windows_core::GUID,
    pub szCustomDialDll: [u16; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [u16; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [u16; 260],
    pub szPrerequisiteEntry: [u16; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [u16; 257],
    pub szIDr: [u16; 257],
    pub fIsImsConfig: super::super::Foundation::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASENTRYW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6ServerAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: RASIKEV_PROJECTION_INFO_FLAGS,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut super::super::Networking::WinSock::IN_ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASIKEV2_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6ServerAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: RASIKEV_PROJECTION_INFO_FLAGS,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut super::super::Networking::WinSock::IN_ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASIKEV2_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASIPADDR {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
impl windows_core::TypeKind for RASIPADDR {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASIPADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASIPXW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [u16; 22],
}
impl windows_core::TypeKind for RASIPXW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASIPXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASNOUSERA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [i8; 257],
    pub szPassword: [i8; 257],
    pub szDomain: [i8; 16],
}
impl windows_core::TypeKind for RASNOUSERA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASNOUSERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASNOUSERW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl windows_core::TypeKind for RASNOUSERW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASNOUSERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASPBDLGA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASPBDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASPBDLGA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASPBDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for RASPBDLGW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RASPBDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for RASPBDLGW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(target_arch = "x86")]
impl Default for RASPBDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPCCP {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwServerCompressionAlgorithm: u32,
    pub dwServerOptions: u32,
}
impl windows_core::TypeKind for RASPPPCCP {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPCCP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPIPA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [i8; 16],
    pub szServerIpAddress: [i8; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl windows_core::TypeKind for RASPPPIPA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPIPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPIPV6 {
    pub dwSize: u32,
    pub dwError: u32,
    pub bLocalInterfaceIdentifier: [u8; 8],
    pub bPeerInterfaceIdentifier: [u8; 8],
    pub bLocalCompressionProtocol: [u8; 2],
    pub bPeerCompressionProtocol: [u8; 2],
}
impl windows_core::TypeKind for RASPPPIPV6 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPIPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPIPW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [u16; 16],
    pub szServerIpAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl windows_core::TypeKind for RASPPPIPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPIPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPIPXA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [i8; 22],
}
impl windows_core::TypeKind for RASPPPIPXA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPIPXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPLCPA {
    pub dwSize: u32,
    pub fBundled: super::super::Foundation::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [i8; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl windows_core::TypeKind for RASPPPLCPA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPLCPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPLCPW {
    pub dwSize: u32,
    pub fBundled: super::super::Foundation::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [u16; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl windows_core::TypeKind for RASPPPLCPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPLCPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPNBFA {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [i8; 17],
    pub szWorkstationName: [i8; 17],
    pub bLana: u8,
}
impl windows_core::TypeKind for RASPPPNBFA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPNBFA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASPPPNBFW {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [u16; 17],
    pub szWorkstationName: [u16; 17],
    pub bLana: u8,
}
impl windows_core::TypeKind for RASPPPNBFW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASPPPNBFW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASPPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv4Options: u32,
    pub dwIPv4ServerOptions: u32,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bServerInterfaceIdentifier: [u8; 8],
    pub fBundled: super::super::Foundation::BOOL,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwServerAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwServerAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwEapTypeId: u32,
    pub dwServerEapTypeId: u32,
    pub dwLcpOptions: u32,
    pub dwLcpServerOptions: u32,
    pub dwCcpError: u32,
    pub dwCcpCompressionAlgorithm: u32,
    pub dwCcpServerCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwCcpServerOptions: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASPPP_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASSUBENTRYA {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [i8; 17],
    pub szDeviceName: [i8; 129],
    pub szLocalPhoneNumber: [i8; 129],
    pub dwAlternateOffset: u32,
}
impl windows_core::TypeKind for RASSUBENTRYA {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASSUBENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASSUBENTRYW {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
}
impl windows_core::TypeKind for RASSUBENTRYW {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASSUBENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASTUNNELENDPOINT {
    pub dwType: u32,
    pub Anonymous: RASTUNNELENDPOINT_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASTUNNELENDPOINT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union RASTUNNELENDPOINT_0 {
    pub ipv4: super::super::Networking::WinSock::IN_ADDR,
    pub ipv6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASTUNNELENDPOINT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RASUPDATECONN {
    pub version: RASAPIVERSION,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwIfIndex: u32,
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RASUPDATECONN {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RASUPDATECONN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_0 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
}
impl windows_core::TypeKind for RAS_CONNECTION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_1 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub PppInfo: PPP_INFO,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
impl windows_core::TypeKind for RAS_CONNECTION_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo2: PPP_INFO_2,
}
impl windows_core::TypeKind for RAS_CONNECTION_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: windows_core::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for RAS_CONNECTION_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub connectionStartTime: super::super::Foundation::FILETIME,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO2,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwDeviceType: u32,
}
impl windows_core::TypeKind for RAS_CONNECTION_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RAS_CONNECTION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: windows_core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for RAS_CONNECTION_EX {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_PORT_0 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwPortCondition: RAS_PORT_CONDITION,
    pub dwTotalNumberOfCalls: u32,
    pub dwConnectDuration: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
}
impl windows_core::TypeKind for RAS_PORT_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_PORT_1 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
impl windows_core::TypeKind for RAS_PORT_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_PORT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_PORT_2 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwConn_State: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwCrcErr: u32,
    pub dwSerialOverRunErrs: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwTotalErrors: u32,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub ullFramesXmited: u64,
    pub ullFramesRcved: u64,
    pub ullBytesTxUncompressed: u64,
    pub ullBytesTxCompressed: u64,
    pub ullBytesRcvUncompressed: u64,
    pub ullBytesRcvCompressed: u64,
}
impl windows_core::TypeKind for RAS_PORT_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RAS_PROJECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for RAS_PROJECTION_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_SECURITY_INFO {
    pub LastError: u32,
    pub BytesReceived: u32,
    pub DeviceName: [i8; 129],
}
impl windows_core::TypeKind for RAS_SECURITY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_SECURITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_STATS {
    pub dwSize: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwBps: u32,
    pub dwConnectDuration: u32,
}
impl windows_core::TypeKind for RAS_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: u32,
    pub wszLocalEndpointAddress: [u16; 65],
    pub wszRemoteEndpointAddress: [u16; 65],
}
impl windows_core::TypeKind for RAS_UPDATE_CONNECTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_USER_0 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
}
impl windows_core::TypeKind for RAS_USER_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RAS_USER_1 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
    pub bfPrivilege2: u8,
}
impl windows_core::TypeKind for RAS_USER_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: u32,
    pub dwEncryptionMethod: u32,
    pub dwCipherTransformConstant: u32,
    pub dwAuthTransformConstant: u32,
    pub dwPfsGroup: u32,
    pub dwDhGroup: u32,
}
impl windows_core::TypeKind for ROUTER_CUSTOM_IKEv2_POLICY0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
    pub dwMmSaLifeTime: u32,
    pub vpnTrafficSelectors: MPR_VPN_TRAFFIC_SELECTORS,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl windows_core::TypeKind for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ROUTING_PROTOCOL_CONFIG {
    pub dwCallbackFlags: u32,
    pub pfnRpfCallback: PMGM_RPF_CALLBACK,
    pub pfnCreationAlertCallback: PMGM_CREATION_ALERT_CALLBACK,
    pub pfnPruneAlertCallback: PMGM_PRUNE_ALERT_CALLBACK,
    pub pfnJoinAlertCallback: PMGM_JOIN_ALERT_CALLBACK,
    pub pfnWrongIfCallback: PMGM_WRONG_IF_CALLBACK,
    pub pfnLocalJoinCallback: PMGM_LOCAL_JOIN_CALLBACK,
    pub pfnLocalLeaveCallback: PMGM_LOCAL_LEAVE_CALLBACK,
    pub pfnDisableIgmpCallback: PMGM_DISABLE_IGMP_CALLBACK,
    pub pfnEnableIgmpCallback: PMGM_ENABLE_IGMP_CALLBACK,
}
impl windows_core::TypeKind for ROUTING_PROTOCOL_CONFIG {
    type TypeKind = windows_core::CopyType;
}
impl Default for ROUTING_PROTOCOL_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_DEST_INFO {
    pub DestHandle: isize,
    pub DestAddress: RTM_NET_ADDRESS,
    pub LastChanged: super::super::Foundation::FILETIME,
    pub BelongsToViews: u32,
    pub NumberOfViews: u32,
    pub ViewInfo: [RTM_DEST_INFO_0; 1],
}
impl windows_core::TypeKind for RTM_DEST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_DEST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_DEST_INFO_0 {
    pub ViewId: i32,
    pub NumRoutes: u32,
    pub Route: isize,
    pub Owner: isize,
    pub DestFlags: u32,
    pub HoldRoute: isize,
}
impl windows_core::TypeKind for RTM_DEST_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_DEST_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RTM_ENTITY_EXPORT_METHODS {
    pub NumMethods: u32,
    pub Methods: [RTM_ENTITY_EXPORT_METHOD; 1],
}
impl windows_core::TypeKind for RTM_ENTITY_EXPORT_METHODS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_EXPORT_METHODS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTM_ENTITY_ID {
    pub Anonymous: RTM_ENTITY_ID_0,
}
impl windows_core::TypeKind for RTM_ENTITY_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTM_ENTITY_ID_0 {
    pub Anonymous: RTM_ENTITY_ID_0_0,
    pub EntityId: u64,
}
impl windows_core::TypeKind for RTM_ENTITY_ID_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_ENTITY_ID_0_0 {
    pub EntityProtocolId: u32,
    pub EntityInstanceId: u32,
}
impl windows_core::TypeKind for RTM_ENTITY_ID_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_ID_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTM_ENTITY_INFO {
    pub RtmInstanceId: u16,
    pub AddressFamily: u16,
    pub EntityId: RTM_ENTITY_ID,
}
impl windows_core::TypeKind for RTM_ENTITY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_ENTITY_METHOD_INPUT {
    pub MethodType: u32,
    pub InputSize: u32,
    pub InputData: [u8; 1],
}
impl windows_core::TypeKind for RTM_ENTITY_METHOD_INPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_METHOD_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_ENTITY_METHOD_OUTPUT {
    pub MethodType: u32,
    pub MethodStatus: u32,
    pub OutputSize: u32,
    pub OutputData: [u8; 1],
}
impl windows_core::TypeKind for RTM_ENTITY_METHOD_OUTPUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ENTITY_METHOD_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_NET_ADDRESS {
    pub AddressFamily: u16,
    pub NumBits: u16,
    pub AddrBits: [u8; 16],
}
impl windows_core::TypeKind for RTM_NET_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_NET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_NEXTHOP_INFO {
    pub NextHopAddress: RTM_NET_ADDRESS,
    pub NextHopOwner: isize,
    pub InterfaceIndex: u32,
    pub State: u16,
    pub Flags: u16,
    pub EntitySpecificInfo: *mut core::ffi::c_void,
    pub RemoteNextHop: isize,
}
impl windows_core::TypeKind for RTM_NEXTHOP_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_NEXTHOP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_NEXTHOP_LIST {
    pub NumNextHops: u16,
    pub NextHops: [isize; 1],
}
impl windows_core::TypeKind for RTM_NEXTHOP_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_NEXTHOP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_PREF_INFO {
    pub Metric: u32,
    pub Preference: u32,
}
impl windows_core::TypeKind for RTM_PREF_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_PREF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_REGN_PROFILE {
    pub MaxNextHopsInRoute: u32,
    pub MaxHandlesInEnum: u32,
    pub ViewsSupported: u32,
    pub NumberOfViews: u32,
}
impl windows_core::TypeKind for RTM_REGN_PROFILE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_REGN_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTM_ROUTE_INFO {
    pub DestHandle: isize,
    pub RouteOwner: isize,
    pub Neighbour: isize,
    pub State: u8,
    pub Flags1: u8,
    pub Flags: u16,
    pub PrefInfo: RTM_PREF_INFO,
    pub BelongsToViews: u32,
    pub EntitySpecificInfo: *mut core::ffi::c_void,
    pub NextHopsList: RTM_NEXTHOP_LIST,
}
impl windows_core::TypeKind for RTM_ROUTE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTM_ROUTE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_MESSAGE {
    pub dwMsgId: SECURITY_MESSAGE_MSG_ID,
    pub hPort: isize,
    pub dwError: u32,
    pub UserName: [i8; 257],
    pub Domain: [i8; 16],
}
impl windows_core::TypeKind for SECURITY_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SECURITY_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOURCE_GROUP_ENTRY {
    pub dwSourceAddr: u32,
    pub dwSourceMask: u32,
    pub dwGroupAddr: u32,
    pub dwGroupMask: u32,
}
impl windows_core::TypeKind for SOURCE_GROUP_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOURCE_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSTP_CERT_INFO {
    pub isDefault: super::super::Foundation::BOOL,
    pub certBlob: super::super::Security::Cryptography::CRYPT_INTEGER_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SSTP_CERT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SSTP_CERT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub isUseHttps: super::super::Foundation::BOOL,
    pub certAlgorithm: u32,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for SSTP_CONFIG_PARAMS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for SSTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: u16,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for VPN_TS_IP_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for VPN_TS_IP_ADDRESS_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ORASADFUNC = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: windows_core::PCSTR, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL>;
pub type PFNRASFREEBUFFER = Option<unsafe extern "system" fn(pbufer: *mut u8) -> u32>;
pub type PFNRASGETBUFFER = Option<unsafe extern "system" fn(ppbuffer: *mut *mut u8, pdwsize: *mut u32) -> u32>;
pub type PFNRASRECEIVEBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32, dwtimeout: u32, hevent: super::super::Foundation::HANDLE) -> u32>;
pub type PFNRASRETRIEVEBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32) -> u32>;
pub type PFNRASSENDBUFFER = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, dwsize: u32) -> u32>;
pub type PFNRASSETCOMMSETTINGS = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, prascommsettings: *mut RASCOMMSETTINGS, pvreserved: *mut core::ffi::c_void) -> u32>;
pub type PMGM_CREATION_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwinifindex: u32, dwinifnexthopaddr: u32, dwifcount: u32, pmieoutiflist: *mut MGM_IF_ENTRY) -> u32>;
pub type PMGM_DISABLE_IGMP_CALLBACK = Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_ENABLE_IGMP_CALLBACK = Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_JOIN_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, bmemberupdate: super::super::Foundation::BOOL) -> u32>;
pub type PMGM_LOCAL_JOIN_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_LOCAL_LEAVE_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
pub type PMGM_PRUNE_ALERT_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32, bmemberdelete: super::super::Foundation::BOOL, pdwtimeout: *mut u32) -> u32>;
pub type PMGM_RPF_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, pdwinifindex: *mut u32, pdwinifnexthopaddr: *mut u32, pdwupstreamnbr: *mut u32, dwhdrsize: u32, pbpackethdr: *mut u8, pbroute: *mut u8) -> u32>;
pub type PMGM_WRONG_IF_CALLBACK = Option<unsafe extern "system" fn(dwsourceaddr: u32, dwgroupaddr: u32, dwifindex: u32, dwifnexthopaddr: u32, dwhdrsize: u32, pbpackethdr: *mut u8) -> u32>;
pub type PMPRADMINACCEPTNEWCONNECTION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTNEWCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTNEWLINK = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTREAUTHENTICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: RAS_CONNECTION_3)>;
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX)>;
pub type PMPRADMINGETIPADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut u32, param3: *mut super::super::Foundation::BOOL) -> u32>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR, param3: *mut super::super::Foundation::BOOL) -> u32>;
pub type PMPRADMINLINKHANGUPNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1)>;
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = Option<unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> u32>;
pub type PMPRADMINRELEASEIPADRESS = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut u32)>;
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR)>;
pub type PMPRADMINTERMINATEDLL = Option<unsafe extern "system" fn() -> u32>;
pub type RASADFUNCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: windows_core::PCSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL>;
pub type RASADFUNCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: windows_core::PCWSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL>;
pub type RASDIALFUNC = Option<unsafe extern "system" fn(param0: u32, param1: RASCONNSTATE, param2: u32)>;
pub type RASDIALFUNC1 = Option<unsafe extern "system" fn(param0: HRASCONN, param1: u32, param2: RASCONNSTATE, param3: u32, param4: u32)>;
pub type RASDIALFUNC2 = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: HRASCONN, param3: u32, param4: RASCONNSTATE, param5: u32, param6: u32) -> u32>;
pub type RASPBDLGFUNCA = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCSTR, param3: *mut core::ffi::c_void)>;
pub type RASPBDLGFUNCW = Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: windows_core::PCWSTR, param3: *mut core::ffi::c_void)>;
pub type RASSECURITYPROC = Option<unsafe extern "system" fn() -> u32>;
pub type RTM_ENTITY_EXPORT_METHOD = Option<unsafe extern "system" fn(callerhandle: isize, calleehandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, output: *mut RTM_ENTITY_METHOD_OUTPUT)>;
pub type RTM_EVENT_CALLBACK = Option<unsafe extern "system" fn(rtmreghandle: isize, eventtype: RTM_EVENT_TYPE, context1: *mut core::ffi::c_void, context2: *mut core::ffi::c_void) -> u32>;
pub type RasCustomDeleteEntryNotifyFn = Option<unsafe extern "system" fn(lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, dwflags: u32) -> u32>;
pub type RasCustomDialDlgFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, dwflags: u32, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpszphonenumber: windows_core::PCWSTR, lpinfo: *mut RASDIALDLG, pvinfo: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type RasCustomDialFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lprasdialextensions: *mut RASDIALEXTENSIONS, lpszphonebook: windows_core::PCWSTR, lprasdialparams: *mut RASDIALPARAMSA, dwnotifiertype: u32, lpvnotifier: *mut core::ffi::c_void, lphrasconn: *mut HRASCONN, dwflags: u32) -> u32>;
pub type RasCustomEntryDlgFn = Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lpszphonebook: windows_core::PCWSTR, lpszentry: windows_core::PCWSTR, lpinfo: *mut RASENTRYDLGA, dwflags: u32) -> super::super::Foundation::BOOL>;
pub type RasCustomHangUpFn = Option<unsafe extern "system" fn(hrasconn: HRASCONN) -> u32>;
pub type RasCustomScriptExecuteFn = Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, lpszphonebook: windows_core::PCWSTR, lpszentryname: windows_core::PCWSTR, pfnrasgetbuffer: PFNRASGETBUFFER, pfnrasfreebuffer: PFNRASFREEBUFFER, pfnrassendbuffer: PFNRASSENDBUFFER, pfnrasreceivebuffer: PFNRASRECEIVEBUFFER, pfnrasretrievebuffer: PFNRASRETRIEVEBUFFER, hwnd: super::super::Foundation::HWND, prasdialparams: *mut RASDIALPARAMSA, pvreserved: *mut core::ffi::c_void) -> u32>;
