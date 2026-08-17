#[inline]
pub unsafe fn GetNetScheduleAccountInformation<P0>(pwszservername: P0, wszaccount: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mstask.dll" "system" fn GetNetScheduleAccountInformation(pwszservername : windows_core::PCWSTR, ccaccount : u32, wszaccount : windows_core::PWSTR) -> windows_core::HRESULT);
    GetNetScheduleAccountInformation(pwszservername.param().abi(), wszaccount.len().try_into().unwrap(), core::mem::transmute(wszaccount.as_ptr())).ok()
}
#[inline]
pub unsafe fn I_NetLogonControl2<P0>(servername: P0, functioncode: u32, querylevel: u32, data: *const u8, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn I_NetLogonControl2(servername : windows_core::PCWSTR, functioncode : u32, querylevel : u32, data : *const u8, buffer : *mut *mut u8) -> u32);
    I_NetLogonControl2(servername.param().abi(), functioncode, querylevel, data, buffer)
}
#[inline]
pub unsafe fn LogErrorA(dwmessageid: u32, plpwssubstrings: &[windows_core::PCSTR], dwerrorcode: u32) {
    windows_targets::link!("rtutils.dll" "system" fn LogErrorA(dwmessageid : u32, cnumberofsubstrings : u32, plpwssubstrings : *const windows_core::PCSTR, dwerrorcode : u32));
    LogErrorA(dwmessageid, plpwssubstrings.len().try_into().unwrap(), core::mem::transmute(plpwssubstrings.as_ptr()), dwerrorcode)
}
#[inline]
pub unsafe fn LogErrorW(dwmessageid: u32, plpwssubstrings: &[windows_core::PCWSTR], dwerrorcode: u32) {
    windows_targets::link!("rtutils.dll" "system" fn LogErrorW(dwmessageid : u32, cnumberofsubstrings : u32, plpwssubstrings : *const windows_core::PCWSTR, dwerrorcode : u32));
    LogErrorW(dwmessageid, plpwssubstrings.len().try_into().unwrap(), core::mem::transmute(plpwssubstrings.as_ptr()), dwerrorcode)
}
#[inline]
pub unsafe fn LogEventA(weventtype: u32, dwmessageid: u32, plpwssubstrings: &[windows_core::PCSTR]) {
    windows_targets::link!("rtutils.dll" "system" fn LogEventA(weventtype : u32, dwmessageid : u32, cnumberofsubstrings : u32, plpwssubstrings : *const windows_core::PCSTR));
    LogEventA(weventtype, dwmessageid, plpwssubstrings.len().try_into().unwrap(), core::mem::transmute(plpwssubstrings.as_ptr()))
}
#[inline]
pub unsafe fn LogEventW(weventtype: u32, dwmessageid: u32, plpwssubstrings: &[windows_core::PCWSTR]) {
    windows_targets::link!("rtutils.dll" "system" fn LogEventW(weventtype : u32, dwmessageid : u32, cnumberofsubstrings : u32, plpwssubstrings : *const windows_core::PCWSTR));
    LogEventW(weventtype, dwmessageid, plpwssubstrings.len().try_into().unwrap(), core::mem::transmute(plpwssubstrings.as_ptr()))
}
#[inline]
pub unsafe fn MprSetupProtocolEnum(dwtransportid: u32, lplpbuffer: *mut *mut u8, lpdwentriesread: *mut u32) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn MprSetupProtocolEnum(dwtransportid : u32, lplpbuffer : *mut *mut u8, lpdwentriesread : *mut u32) -> u32);
    MprSetupProtocolEnum(dwtransportid, lplpbuffer, lpdwentriesread)
}
#[inline]
pub unsafe fn MprSetupProtocolFree(lpbuffer: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn MprSetupProtocolFree(lpbuffer : *mut core::ffi::c_void) -> u32);
    MprSetupProtocolFree(lpbuffer)
}
#[inline]
pub unsafe fn NetAccessAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetAccessAdd(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetAccessDel<P0, P1>(servername: P0, resource: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessDel(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR) -> u32);
    NetAccessDel(servername.param().abi(), resource.param().abi())
}
#[inline]
pub unsafe fn NetAccessEnum<P0, P1>(servername: P0, basepath: P1, recursive: u32, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessEnum(servername : windows_core::PCWSTR, basepath : windows_core::PCWSTR, recursive : u32, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetAccessEnum(servername.param().abi(), basepath.param().abi(), recursive, level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetAccessGetInfo<P0, P1>(servername: P0, resource: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessGetInfo(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetAccessGetInfo(servername.param().abi(), resource.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetAccessGetUserPerms<P0, P1, P2>(servername: P0, ugname: P1, resource: P2, perms: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessGetUserPerms(servername : windows_core::PCWSTR, ugname : windows_core::PCWSTR, resource : windows_core::PCWSTR, perms : *mut u32) -> u32);
    NetAccessGetUserPerms(servername.param().abi(), ugname.param().abi(), resource.param().abi(), perms)
}
#[inline]
pub unsafe fn NetAccessSetInfo<P0, P1>(servername: P0, resource: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAccessSetInfo(servername : windows_core::PCWSTR, resource : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetAccessSetInfo(servername.param().abi(), resource.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetAddAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAddAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    NetAddAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetAddServiceAccount<P0, P1, P2>(servername: P0, accountname: P1, password: P2, flags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAddServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, password : windows_core::PCWSTR, flags : u32) -> super::super::Foundation:: NTSTATUS);
    NetAddServiceAccount(servername.param().abi(), accountname.param().abi(), password.param().abi(), flags)
}
#[inline]
pub unsafe fn NetAlertRaise<P0>(alerttype: P0, buffer: *const core::ffi::c_void, buffersize: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAlertRaise(alerttype : windows_core::PCWSTR, buffer : *const core::ffi::c_void, buffersize : u32) -> u32);
    NetAlertRaise(alerttype.param().abi(), buffer, buffersize)
}
#[inline]
pub unsafe fn NetAlertRaiseEx<P0, P1>(alerttype: P0, variableinfo: *const core::ffi::c_void, variableinfosize: u32, servicename: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAlertRaiseEx(alerttype : windows_core::PCWSTR, variableinfo : *const core::ffi::c_void, variableinfosize : u32, servicename : windows_core::PCWSTR) -> u32);
    NetAlertRaiseEx(alerttype.param().abi(), variableinfo, variableinfosize, servicename.param().abi())
}
#[inline]
pub unsafe fn NetApiBufferAllocate(bytecount: u32, buffer: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetApiBufferAllocate(bytecount : u32, buffer : *mut *mut core::ffi::c_void) -> u32);
    NetApiBufferAllocate(bytecount, buffer)
}
#[inline]
pub unsafe fn NetApiBufferFree(buffer: Option<*const core::ffi::c_void>) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetApiBufferFree(buffer : *const core::ffi::c_void) -> u32);
    NetApiBufferFree(core::mem::transmute(buffer.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn NetApiBufferReallocate(oldbuffer: Option<*const core::ffi::c_void>, newbytecount: u32, newbuffer: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetApiBufferReallocate(oldbuffer : *const core::ffi::c_void, newbytecount : u32, newbuffer : *mut *mut core::ffi::c_void) -> u32);
    NetApiBufferReallocate(core::mem::transmute(oldbuffer.unwrap_or(std::ptr::null())), newbytecount, newbuffer)
}
#[inline]
pub unsafe fn NetApiBufferSize(buffer: *const core::ffi::c_void, bytecount: *mut u32) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetApiBufferSize(buffer : *const core::ffi::c_void, bytecount : *mut u32) -> u32);
    NetApiBufferSize(buffer, bytecount)
}
#[inline]
pub unsafe fn NetAuditClear<P0, P1, P2>(server: P0, backupfile: P1, service: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAuditClear(server : windows_core::PCWSTR, backupfile : windows_core::PCWSTR, service : windows_core::PCWSTR) -> u32);
    NetAuditClear(server.param().abi(), backupfile.param().abi(), service.param().abi())
}
#[inline]
pub unsafe fn NetAuditRead<P0, P1>(server: P0, service: P1, auditloghandle: *mut HLOG, offset: u32, reserved1: *mut u32, reserved2: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxlen: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAuditRead(server : windows_core::PCWSTR, service : windows_core::PCWSTR, auditloghandle : *mut HLOG, offset : u32, reserved1 : *mut u32, reserved2 : u32, offsetflag : u32, bufptr : *mut *mut u8, prefmaxlen : u32, bytesread : *mut u32, totalavailable : *mut u32) -> u32);
    NetAuditRead(server.param().abi(), service.param().abi(), auditloghandle, offset, reserved1, reserved2, offsetflag, bufptr, prefmaxlen, bytesread, totalavailable)
}
#[inline]
pub unsafe fn NetAuditWrite<P0>(r#type: u32, buf: *mut u8, numbytes: u32, service: P0, reserved: *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetAuditWrite(r#type : u32, buf : *mut u8, numbytes : u32, service : windows_core::PCWSTR, reserved : *mut u8) -> u32);
    NetAuditWrite(r#type, buf, numbytes, service.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetConfigGet<P0, P1, P2>(server: P0, component: P1, parameter: P2, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetConfigGet(server : windows_core::PCWSTR, component : windows_core::PCWSTR, parameter : windows_core::PCWSTR, bufptr : *mut *mut u8) -> u32);
    NetConfigGet(server.param().abi(), component.param().abi(), parameter.param().abi(), bufptr)
}
#[inline]
pub unsafe fn NetConfigGetAll<P0, P1>(server: P0, component: P1, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetConfigGetAll(server : windows_core::PCWSTR, component : windows_core::PCWSTR, bufptr : *mut *mut u8) -> u32);
    NetConfigGetAll(server.param().abi(), component.param().abi(), bufptr)
}
#[inline]
pub unsafe fn NetConfigSet<P0, P1, P2>(server: P0, reserved1: P1, component: P2, level: u32, reserved2: u32, buf: *mut u8, reserved3: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetConfigSet(server : windows_core::PCWSTR, reserved1 : windows_core::PCWSTR, component : windows_core::PCWSTR, level : u32, reserved2 : u32, buf : *mut u8, reserved3 : u32) -> u32);
    NetConfigSet(server.param().abi(), reserved1.param().abi(), component.param().abi(), level, reserved2, buf, reserved3)
}
#[inline]
pub unsafe fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: Option<*mut *mut u8>, pdwpackagebindatasize: Option<*mut u32>, pppackagetextdata: Option<*mut windows_core::PWSTR>) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetCreateProvisioningPackage(pprovisioningparams : *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata : *mut *mut u8, pdwpackagebindatasize : *mut u32, pppackagetextdata : *mut windows_core::PWSTR) -> u32);
    NetCreateProvisioningPackage(pprovisioningparams, core::mem::transmute(pppackagebindata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwpackagebindatasize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pppackagetextdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetEnumerateComputerNames<P0>(server: P0, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetEnumerateComputerNames(server : windows_core::PCWSTR, nametype : NET_COMPUTER_NAME_TYPE, reserved : u32, entrycount : *mut u32, computernames : *mut *mut windows_core::PWSTR) -> u32);
    NetEnumerateComputerNames(server.param().abi(), nametype, reserved, entrycount, computernames)
}
#[inline]
pub unsafe fn NetEnumerateServiceAccounts<P0>(servername: P0, flags: u32, accountscount: *mut u32, accounts: *mut *mut *mut u16) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetEnumerateServiceAccounts(servername : windows_core::PCWSTR, flags : u32, accountscount : *mut u32, accounts : *mut *mut *mut u16) -> super::super::Foundation:: NTSTATUS);
    NetEnumerateServiceAccounts(servername.param().abi(), flags, accountscount, accounts)
}
#[inline]
pub unsafe fn NetErrorLogClear<P0, P1>(uncservername: P0, backupfile: P1, reserved: Option<*const u8>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetErrorLogClear(uncservername : windows_core::PCWSTR, backupfile : windows_core::PCWSTR, reserved : *const u8) -> u32);
    NetErrorLogClear(uncservername.param().abi(), backupfile.param().abi(), core::mem::transmute(reserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn NetErrorLogRead<P0, P1>(uncservername: P0, reserved1: P1, errorloghandle: *const HLOG, offset: u32, reserved2: Option<*const u32>, reserved3: u32, offsetflag: u32, bufptr: *mut *mut u8, prefmaxsize: u32, bytesread: *mut u32, totalavailable: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetErrorLogRead(uncservername : windows_core::PCWSTR, reserved1 : windows_core::PCWSTR, errorloghandle : *const HLOG, offset : u32, reserved2 : *const u32, reserved3 : u32, offsetflag : u32, bufptr : *mut *mut u8, prefmaxsize : u32, bytesread : *mut u32, totalavailable : *mut u32) -> u32);
    NetErrorLogRead(uncservername.param().abi(), reserved1.param().abi(), errorloghandle, offset, core::mem::transmute(reserved2.unwrap_or(std::ptr::null())), reserved3, offsetflag, bufptr, prefmaxsize, bytesread, totalavailable)
}
#[inline]
pub unsafe fn NetErrorLogWrite<P0>(reserved1: Option<*const u8>, code: u32, component: P0, buffer: *const u8, numbytes: u32, msgbuf: *const u8, strcount: u32, reserved2: Option<*const u8>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetErrorLogWrite(reserved1 : *const u8, code : u32, component : windows_core::PCWSTR, buffer : *const u8, numbytes : u32, msgbuf : *const u8, strcount : u32, reserved2 : *const u8) -> u32);
    NetErrorLogWrite(core::mem::transmute(reserved1.unwrap_or(std::ptr::null())), code, component.param().abi(), buffer, numbytes, msgbuf, strcount, core::mem::transmute(reserved2.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn NetFreeAadJoinInformation(pjoininfo: Option<*const DSREG_JOIN_INFO>) {
    windows_targets::link!("netapi32.dll" "system" fn NetFreeAadJoinInformation(pjoininfo : *const DSREG_JOIN_INFO));
    NetFreeAadJoinInformation(core::mem::transmute(pjoininfo.unwrap_or(std::ptr::null())))
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn NetGetAadJoinInformation<P0>(pcsztenantid: P0) -> windows_core::Result<*mut DSREG_JOIN_INFO>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetAadJoinInformation(pcsztenantid : windows_core::PCWSTR, ppjoininfo : *mut *mut DSREG_JOIN_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    NetGetAadJoinInformation(pcsztenantid.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn NetGetAnyDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetAnyDCName(servername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, buffer : *mut *mut u8) -> u32);
    NetGetAnyDCName(servername.param().abi(), domainname.param().abi(), buffer)
}
#[inline]
pub unsafe fn NetGetDCName<P0, P1>(servername: P0, domainname: P1, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetDCName(servername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, buffer : *mut *mut u8) -> u32);
    NetGetDCName(servername.param().abi(), domainname.param().abi(), buffer)
}
#[inline]
pub unsafe fn NetGetDisplayInformationIndex<P0, P1>(servername: P0, level: u32, prefix: P1, index: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetDisplayInformationIndex(servername : windows_core::PCWSTR, level : u32, prefix : windows_core::PCWSTR, index : *mut u32) -> u32);
    NetGetDisplayInformationIndex(servername.param().abi(), level, prefix.param().abi(), index)
}
#[inline]
pub unsafe fn NetGetJoinInformation<P0>(lpserver: P0, lpnamebuffer: *mut windows_core::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetJoinInformation(lpserver : windows_core::PCWSTR, lpnamebuffer : *mut windows_core::PWSTR, buffertype : *mut NETSETUP_JOIN_STATUS) -> u32);
    NetGetJoinInformation(lpserver.param().abi(), lpnamebuffer, buffertype)
}
#[inline]
pub unsafe fn NetGetJoinableOUs<P0, P1, P2, P3>(lpserver: P0, lpdomain: P1, lpaccount: P2, lppassword: P3, oucount: *mut u32, ous: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGetJoinableOUs(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, oucount : *mut u32, ous : *mut *mut windows_core::PWSTR) -> u32);
    NetGetJoinableOUs(lpserver.param().abi(), lpdomain.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), oucount, ous)
}
#[inline]
pub unsafe fn NetGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetGroupAdd(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetGroupAddUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupAddUser(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    NetGroupAddUser(servername.param().abi(), groupname.param().abi(), username.param().abi())
}
#[inline]
pub unsafe fn NetGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupDel(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR) -> u32);
    NetGroupDel(servername.param().abi(), groupname.param().abi())
}
#[inline]
pub unsafe fn NetGroupDelUser<P0, P1, P2>(servername: P0, groupname: P1, username: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupDelUser(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    NetGroupDelUser(servername.param().abi(), groupname.param().abi(), username.param().abi())
}
#[inline]
pub unsafe fn NetGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut usize) -> u32);
    NetGroupEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupGetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetGroupGetInfo(servername.param().abi(), groupname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetGroupGetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupGetUsers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    NetGroupGetUsers(servername.param().abi(), groupname.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupSetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetGroupSetInfo(servername.param().abi(), groupname.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetGroupSetUsers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetGroupSetUsers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    NetGroupSetUsers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries)
}
#[inline]
pub unsafe fn NetIsServiceAccount<P0, P1>(servername: P0, accountname: P1, isservice: *mut super::super::Foundation::BOOL) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetIsServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, isservice : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: NTSTATUS);
    NetIsServiceAccount(servername.param().abi(), accountname.param().abi(), isservice)
}
#[inline]
pub unsafe fn NetJoinDomain<P0, P1, P2, P3, P4>(lpserver: P0, lpdomain: P1, lpmachineaccountou: P2, lpaccount: P3, lppassword: P4, fjoinoptions: NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetJoinDomain(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, fjoinoptions : NET_JOIN_DOMAIN_JOIN_OPTIONS) -> u32);
    NetJoinDomain(lpserver.param().abi(), lpdomain.param().abi(), lpmachineaccountou.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), fjoinoptions)
}
#[inline]
pub unsafe fn NetLocalGroupAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetLocalGroupAdd(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetLocalGroupAddMember<P0, P1, P2>(servername: P0, groupname: P1, membersid: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::PSID>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupAddMember(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, membersid : super::super::Foundation:: PSID) -> u32);
    NetLocalGroupAddMember(servername.param().abi(), groupname.param().abi(), membersid.param().abi())
}
#[inline]
pub unsafe fn NetLocalGroupAddMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupAddMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    NetLocalGroupAddMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries)
}
#[inline]
pub unsafe fn NetLocalGroupDel<P0, P1>(servername: P0, groupname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupDel(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR) -> u32);
    NetLocalGroupDel(servername.param().abi(), groupname.param().abi())
}
#[inline]
pub unsafe fn NetLocalGroupDelMember<P0, P1, P2>(servername: P0, groupname: P1, membersid: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::PSID>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupDelMember(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, membersid : super::super::Foundation:: PSID) -> u32);
    NetLocalGroupDelMember(servername.param().abi(), groupname.param().abi(), membersid.param().abi())
}
#[inline]
pub unsafe fn NetLocalGroupDelMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupDelMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    NetLocalGroupDelMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries)
}
#[inline]
pub unsafe fn NetLocalGroupEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    NetLocalGroupEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetLocalGroupGetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupGetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetLocalGroupGetInfo(servername.param().abi(), groupname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetLocalGroupGetMembers<P0, P1>(servername: P0, localgroupname: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: Option<*mut usize>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupGetMembers(servername : windows_core::PCWSTR, localgroupname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut usize) -> u32);
    NetLocalGroupGetMembers(servername.param().abi(), localgroupname.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetLocalGroupSetInfo<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupSetInfo(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetLocalGroupSetInfo(servername.param().abi(), groupname.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetLocalGroupSetMembers<P0, P1>(servername: P0, groupname: P1, level: u32, buf: *const u8, totalentries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetLocalGroupSetMembers(servername : windows_core::PCWSTR, groupname : windows_core::PCWSTR, level : u32, buf : *const u8, totalentries : u32) -> u32);
    NetLocalGroupSetMembers(servername.param().abi(), groupname.param().abi(), level, buf, totalentries)
}
#[inline]
pub unsafe fn NetMessageBufferSend<P0, P1, P2>(servername: P0, msgname: P1, fromname: P2, buf: *const u8, buflen: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetMessageBufferSend(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, fromname : windows_core::PCWSTR, buf : *const u8, buflen : u32) -> u32);
    NetMessageBufferSend(servername.param().abi(), msgname.param().abi(), fromname.param().abi(), buf, buflen)
}
#[inline]
pub unsafe fn NetMessageNameAdd<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetMessageNameAdd(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> u32);
    NetMessageNameAdd(servername.param().abi(), msgname.param().abi())
}
#[inline]
pub unsafe fn NetMessageNameDel<P0, P1>(servername: P0, msgname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetMessageNameDel(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR) -> u32);
    NetMessageNameDel(servername.param().abi(), msgname.param().abi())
}
#[inline]
pub unsafe fn NetMessageNameEnum<P0>(servername: P0, level: u32, bufptr: *const *const u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetMessageNameEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *const *const u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetMessageNameEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resume_handle)
}
#[inline]
pub unsafe fn NetMessageNameGetInfo<P0, P1>(servername: P0, msgname: P1, level: u32, bufptr: *const *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetMessageNameGetInfo(servername : windows_core::PCWSTR, msgname : windows_core::PCWSTR, level : u32, bufptr : *const *const u8) -> u32);
    NetMessageNameGetInfo(servername.param().abi(), msgname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetProvisionComputerAccount<P0, P1, P2, P3>(lpdomain: P0, lpmachinename: P1, lpmachineaccountou: P2, lpdcname: P3, dwoptions: NETSETUP_PROVISION, pprovisionbindata: Option<*mut *mut u8>, pdwprovisionbindatasize: Option<*mut u32>, pprovisiontextdata: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetProvisionComputerAccount(lpdomain : windows_core::PCWSTR, lpmachinename : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpdcname : windows_core::PCWSTR, dwoptions : NETSETUP_PROVISION, pprovisionbindata : *mut *mut u8, pdwprovisionbindatasize : *mut u32, pprovisiontextdata : *mut windows_core::PWSTR) -> u32);
    NetProvisionComputerAccount(lpdomain.param().abi(), lpmachinename.param().abi(), lpmachineaccountou.param().abi(), lpdcname.param().abi(), dwoptions, core::mem::transmute(pprovisionbindata.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwprovisionbindatasize.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pprovisiontextdata.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetQueryDisplayInformation<P0>(servername: P0, level: u32, index: u32, entriesrequested: u32, preferredmaximumlength: u32, returnedentrycount: *mut u32, sortedbuffer: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetQueryDisplayInformation(servername : windows_core::PCWSTR, level : u32, index : u32, entriesrequested : u32, preferredmaximumlength : u32, returnedentrycount : *mut u32, sortedbuffer : *mut *mut core::ffi::c_void) -> u32);
    NetQueryDisplayInformation(servername.param().abi(), level, index, entriesrequested, preferredmaximumlength, returnedentrycount, sortedbuffer)
}
#[inline]
pub unsafe fn NetQueryServiceAccount<P0, P1>(servername: P0, accountname: P1, infolevel: u32, buffer: *mut *mut u8) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetQueryServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, infolevel : u32, buffer : *mut *mut u8) -> super::super::Foundation:: NTSTATUS);
    NetQueryServiceAccount(servername.param().abi(), accountname.param().abi(), infolevel, buffer)
}
#[inline]
pub unsafe fn NetRemoteComputerSupports<P0>(uncservername: P0, optionswanted: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRemoteComputerSupports(uncservername : windows_core::PCWSTR, optionswanted : NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS, optionssupported : *mut u32) -> u32);
    NetRemoteComputerSupports(uncservername.param().abi(), optionswanted, optionssupported)
}
#[inline]
pub unsafe fn NetRemoteTOD<P0>(uncservername: P0, bufferptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRemoteTOD(uncservername : windows_core::PCWSTR, bufferptr : *mut *mut u8) -> u32);
    NetRemoteTOD(uncservername.param().abi(), bufferptr)
}
#[inline]
pub unsafe fn NetRemoveAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRemoveAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    NetRemoveAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetRemoveServiceAccount<P0, P1>(servername: P0, accountname: P1, flags: u32) -> super::super::Foundation::NTSTATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRemoveServiceAccount(servername : windows_core::PCWSTR, accountname : windows_core::PCWSTR, flags : u32) -> super::super::Foundation:: NTSTATUS);
    NetRemoveServiceAccount(servername.param().abi(), accountname.param().abi(), flags)
}
#[inline]
pub unsafe fn NetRenameMachineInDomain<P0, P1, P2, P3>(lpserver: P0, lpnewmachinename: P1, lpaccount: P2, lppassword: P3, frenameoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRenameMachineInDomain(lpserver : windows_core::PCWSTR, lpnewmachinename : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, frenameoptions : u32) -> u32);
    NetRenameMachineInDomain(lpserver.param().abi(), lpnewmachinename.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), frenameoptions)
}
#[inline]
pub unsafe fn NetReplExportDirAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetReplExportDirAdd(servername.param().abi(), level, buf, parm_err)
}
#[inline]
pub unsafe fn NetReplExportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    NetReplExportDirDel(servername.param().abi(), dirname.param().abi())
}
#[inline]
pub unsafe fn NetReplExportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetReplExportDirEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resumehandle)
}
#[inline]
pub unsafe fn NetReplExportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetReplExportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetReplExportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    NetReplExportDirLock(servername.param().abi(), dirname.param().abi())
}
#[inline]
pub unsafe fn NetReplExportDirSetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirSetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetReplExportDirSetInfo(servername.param().abi(), dirname.param().abi(), level, buf, parm_err)
}
#[inline]
pub unsafe fn NetReplExportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplExportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : u32) -> u32);
    NetReplExportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce)
}
#[inline]
pub unsafe fn NetReplGetInfo<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplGetInfo(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetReplGetInfo(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetReplImportDirAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetReplImportDirAdd(servername.param().abi(), level, buf, parm_err)
}
#[inline]
pub unsafe fn NetReplImportDirDel<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirDel(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    NetReplImportDirDel(servername.param().abi(), dirname.param().abi())
}
#[inline]
pub unsafe fn NetReplImportDirEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetReplImportDirEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, resumehandle)
}
#[inline]
pub unsafe fn NetReplImportDirGetInfo<P0, P1>(servername: P0, dirname: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirGetInfo(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetReplImportDirGetInfo(servername.param().abi(), dirname.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetReplImportDirLock<P0, P1>(servername: P0, dirname: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirLock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR) -> u32);
    NetReplImportDirLock(servername.param().abi(), dirname.param().abi())
}
#[inline]
pub unsafe fn NetReplImportDirUnlock<P0, P1>(servername: P0, dirname: P1, unlockforce: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplImportDirUnlock(servername : windows_core::PCWSTR, dirname : windows_core::PCWSTR, unlockforce : u32) -> u32);
    NetReplImportDirUnlock(servername.param().abi(), dirname.param().abi(), unlockforce)
}
#[inline]
pub unsafe fn NetReplSetInfo<P0>(servername: P0, level: u32, buf: *const u8, parm_err: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetReplSetInfo(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetReplSetInfo(servername.param().abi(), level, buf, parm_err)
}
#[inline]
pub unsafe fn NetRequestOfflineDomainJoin<P0>(pprovisionbindata: &[u8], dwoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRequestOfflineDomainJoin(pprovisionbindata : *const u8, cbprovisionbindatasize : u32, dwoptions : NET_REQUEST_PROVISION_OPTIONS, lpwindowspath : windows_core::PCWSTR) -> u32);
    NetRequestOfflineDomainJoin(core::mem::transmute(pprovisionbindata.as_ptr()), pprovisionbindata.len().try_into().unwrap(), dwoptions, lpwindowspath.param().abi())
}
#[inline]
pub unsafe fn NetRequestProvisioningPackageInstall<P0>(ppackagebindata: &[u8], dwprovisionoptions: NET_REQUEST_PROVISION_OPTIONS, lpwindowspath: P0, pvreserved: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetRequestProvisioningPackageInstall(ppackagebindata : *const u8, dwpackagebindatasize : u32, dwprovisionoptions : NET_REQUEST_PROVISION_OPTIONS, lpwindowspath : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void) -> u32);
    NetRequestProvisioningPackageInstall(core::mem::transmute(ppackagebindata.as_ptr()), ppackagebindata.len().try_into().unwrap(), dwprovisionoptions, lpwindowspath.param().abi(), core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn NetScheduleJobAdd<P0>(servername: P0, buffer: *mut u8, jobid: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetScheduleJobAdd(servername : windows_core::PCWSTR, buffer : *mut u8, jobid : *mut u32) -> u32);
    NetScheduleJobAdd(servername.param().abi(), buffer, jobid)
}
#[inline]
pub unsafe fn NetScheduleJobDel<P0>(servername: P0, minjobid: u32, maxjobid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetScheduleJobDel(servername : windows_core::PCWSTR, minjobid : u32, maxjobid : u32) -> u32);
    NetScheduleJobDel(servername.param().abi(), minjobid, maxjobid)
}
#[inline]
pub unsafe fn NetScheduleJobEnum<P0>(servername: P0, pointertobuffer: *mut *mut u8, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetScheduleJobEnum(servername : windows_core::PCWSTR, pointertobuffer : *mut *mut u8, prefferedmaximumlength : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetScheduleJobEnum(servername.param().abi(), pointertobuffer, prefferedmaximumlength, entriesread, totalentries, resumehandle)
}
#[inline]
pub unsafe fn NetScheduleJobGetInfo<P0>(servername: P0, jobid: u32, pointertobuffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetScheduleJobGetInfo(servername : windows_core::PCWSTR, jobid : u32, pointertobuffer : *mut *mut u8) -> u32);
    NetScheduleJobGetInfo(servername.param().abi(), jobid, pointertobuffer)
}
#[inline]
pub unsafe fn NetServerComputerNameAdd<P0, P1, P2>(servername: P0, emulateddomainname: P1, emulatedservername: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerComputerNameAdd(servername : windows_core::PCWSTR, emulateddomainname : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> u32);
    NetServerComputerNameAdd(servername.param().abi(), emulateddomainname.param().abi(), emulatedservername.param().abi())
}
#[inline]
pub unsafe fn NetServerComputerNameDel<P0, P1>(servername: P0, emulatedservername: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerComputerNameDel(servername : windows_core::PCWSTR, emulatedservername : windows_core::PCWSTR) -> u32);
    NetServerComputerNameDel(servername.param().abi(), emulatedservername.param().abi())
}
#[inline]
pub unsafe fn NetServerDiskEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerDiskEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetServerDiskEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetServerEnum<P0, P1>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, servertype: NET_SERVER_TYPE, domain: P1, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, servertype : NET_SERVER_TYPE, domain : windows_core::PCWSTR, resume_handle : *mut u32) -> u32);
    NetServerEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, servertype, domain.param().abi(), core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetServerGetInfo<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerGetInfo(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetServerGetInfo(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetServerSetInfo<P0>(servername: P0, level: u32, buf: *const u8, parmerror: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerSetInfo(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parmerror : *mut u32) -> u32);
    NetServerSetInfo(servername.param().abi(), level, buf, core::mem::transmute(parmerror.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetServerTransportAdd<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerTransportAdd(servername : windows_core::PCWSTR, level : u32, bufptr : *const u8) -> u32);
    NetServerTransportAdd(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetServerTransportAddEx<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerTransportAddEx(servername : windows_core::PCWSTR, level : u32, bufptr : *const u8) -> u32);
    NetServerTransportAddEx(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetServerTransportDel<P0>(servername: P0, level: u32, bufptr: *const u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerTransportDel(servername : windows_core::PCWSTR, level : u32, bufptr : *const u8) -> u32);
    NetServerTransportDel(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetServerTransportEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServerTransportEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetServerTransportEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetServiceControl<P0, P1>(servername: P0, service: P1, opcode: u32, arg: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServiceControl(servername : windows_core::PCWSTR, service : windows_core::PCWSTR, opcode : u32, arg : u32, bufptr : *mut *mut u8) -> u32);
    NetServiceControl(servername.param().abi(), service.param().abi(), opcode, arg, bufptr)
}
#[inline]
pub unsafe fn NetServiceEnum<P0>(servername: P0, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServiceEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetServiceEnum(servername.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetServiceGetInfo<P0, P1>(servername: P0, service: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServiceGetInfo(servername : windows_core::PCWSTR, service : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetServiceGetInfo(servername.param().abi(), service.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetServiceInstall<P0, P1>(servername: P0, service: P1, argv: &[windows_core::PCWSTR], bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetServiceInstall(servername : windows_core::PCWSTR, service : windows_core::PCWSTR, argc : u32, argv : *const windows_core::PCWSTR, bufptr : *mut *mut u8) -> u32);
    NetServiceInstall(servername.param().abi(), service.param().abi(), argv.len().try_into().unwrap(), core::mem::transmute(argv.as_ptr()), bufptr)
}
#[inline]
pub unsafe fn NetSetPrimaryComputerName<P0, P1, P2, P3>(server: P0, primaryname: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetSetPrimaryComputerName(server : windows_core::PCWSTR, primaryname : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    NetSetPrimaryComputerName(server.param().abi(), primaryname.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved)
}
#[inline]
pub unsafe fn NetUnjoinDomain<P0, P1, P2>(lpserver: P0, lpaccount: P1, lppassword: P2, funjoinoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUnjoinDomain(lpserver : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, funjoinoptions : u32) -> u32);
    NetUnjoinDomain(lpserver.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), funjoinoptions)
}
#[inline]
pub unsafe fn NetUseAdd(servername: Option<*const i8>, levelflags: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetUseAdd(servername : *const i8, levelflags : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetUseAdd(core::mem::transmute(servername.unwrap_or(std::ptr::null())), levelflags, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUseDel<P0, P1>(uncservername: P0, usename: P1, forcelevelflags: FORCE_LEVEL_FLAGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUseDel(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, forcelevelflags : FORCE_LEVEL_FLAGS) -> u32);
    NetUseDel(uncservername.param().abi(), usename.param().abi(), forcelevelflags)
}
#[inline]
pub unsafe fn NetUseEnum<P0>(uncservername: P0, levelflags: u32, bufptr: Option<*mut *mut u8>, preferedmaximumsize: u32, entriesread: Option<*mut u32>, totalentries: *mut u32, resumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUseEnum(uncservername : windows_core::PCWSTR, levelflags : u32, bufptr : *mut *mut u8, preferedmaximumsize : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetUseEnum(uncservername.param().abi(), levelflags, core::mem::transmute(bufptr.unwrap_or(std::ptr::null_mut())), preferedmaximumsize, core::mem::transmute(entriesread.unwrap_or(std::ptr::null_mut())), totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUseGetInfo<P0, P1>(uncservername: P0, usename: P1, levelflags: u32, bufptr: Option<*mut *mut u8>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUseGetInfo(uncservername : windows_core::PCWSTR, usename : windows_core::PCWSTR, levelflags : u32, bufptr : *mut *mut u8) -> u32);
    NetUseGetInfo(uncservername.param().abi(), usename.param().abi(), levelflags, core::mem::transmute(bufptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUserAdd<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserAdd(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetUserAdd(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUserChangePassword<P0, P1, P2, P3>(domainname: P0, username: P1, oldpassword: P2, newpassword: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserChangePassword(domainname : windows_core::PCWSTR, username : windows_core::PCWSTR, oldpassword : windows_core::PCWSTR, newpassword : windows_core::PCWSTR) -> u32);
    NetUserChangePassword(domainname.param().abi(), username.param().abi(), oldpassword.param().abi(), newpassword.param().abi())
}
#[inline]
pub unsafe fn NetUserDel<P0, P1>(servername: P0, username: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserDel(servername : windows_core::PCWSTR, username : windows_core::PCWSTR) -> u32);
    NetUserDel(servername.param().abi(), username.param().abi())
}
#[inline]
pub unsafe fn NetUserEnum<P0>(servername: P0, level: u32, filter: NET_USER_ENUM_FILTER_FLAGS, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserEnum(servername : windows_core::PCWSTR, level : u32, filter : NET_USER_ENUM_FILTER_FLAGS, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetUserEnum(servername.param().abi(), level, filter, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUserGetGroups<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserGetGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32) -> u32);
    NetUserGetGroups(servername.param().abi(), username.param().abi(), level, bufptr, prefmaxlen, entriesread, totalentries)
}
#[inline]
pub unsafe fn NetUserGetInfo<P0, P1>(servername: P0, username: P1, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserGetInfo(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetUserGetInfo(servername.param().abi(), username.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetUserGetLocalGroups<P0, P1>(servername: P0, username: P1, level: u32, flags: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserGetLocalGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, flags : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32) -> u32);
    NetUserGetLocalGroups(servername.param().abi(), username.param().abi(), level, flags, bufptr, prefmaxlen, entriesread, totalentries)
}
#[inline]
pub unsafe fn NetUserModalsGet<P0>(servername: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserModalsGet(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetUserModalsGet(servername.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetUserModalsSet<P0>(servername: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserModalsSet(servername : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetUserModalsSet(servername.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetUserSetGroups<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, num_entries: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserSetGroups(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, buf : *const u8, num_entries : u32) -> u32);
    NetUserSetGroups(servername.param().abi(), username.param().abi(), level, buf, num_entries)
}
#[inline]
pub unsafe fn NetUserSetInfo<P0, P1>(servername: P0, username: P1, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetUserSetInfo(servername : windows_core::PCWSTR, username : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetUserSetInfo(servername.param().abi(), username.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetValidateName<P0, P1, P2, P3>(lpserver: P0, lpname: P1, lpaccount: P2, lppassword: P3, nametype: NETSETUP_NAME_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetValidateName(lpserver : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, nametype : NETSETUP_NAME_TYPE) -> u32);
    NetValidateName(lpserver.param().abi(), lpname.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), nametype)
}
#[inline]
pub unsafe fn NetValidatePasswordPolicy<P0>(servername: P0, qualifier: *mut core::ffi::c_void, validationtype: NET_VALIDATE_PASSWORD_TYPE, inputarg: *mut core::ffi::c_void, outputarg: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetValidatePasswordPolicy(servername : windows_core::PCWSTR, qualifier : *mut core::ffi::c_void, validationtype : NET_VALIDATE_PASSWORD_TYPE, inputarg : *mut core::ffi::c_void, outputarg : *mut *mut core::ffi::c_void) -> u32);
    NetValidatePasswordPolicy(servername.param().abi(), qualifier, validationtype, inputarg, outputarg)
}
#[inline]
pub unsafe fn NetValidatePasswordPolicyFree(outputarg: *mut *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetValidatePasswordPolicyFree(outputarg : *mut *mut core::ffi::c_void) -> u32);
    NetValidatePasswordPolicyFree(outputarg)
}
#[inline]
pub unsafe fn NetWkstaGetInfo<P0>(servername: P0, level: u32, bufptr: Option<*mut *mut u8>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaGetInfo(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetWkstaGetInfo(servername.param().abi(), level, core::mem::transmute(bufptr.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetWkstaSetInfo<P0>(servername: P0, level: u32, buffer: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaSetInfo(servername : windows_core::PCWSTR, level : u32, buffer : *const u8, parm_err : *mut u32) -> u32);
    NetWkstaSetInfo(servername.param().abi(), level, buffer, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetWkstaTransportAdd(servername: Option<*const i8>, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaTransportAdd(servername : *const i8, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetWkstaTransportAdd(core::mem::transmute(servername.unwrap_or(std::ptr::null())), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetWkstaTransportDel<P0, P1>(servername: P0, transportname: P1, ucond: FORCE_LEVEL_FLAGS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaTransportDel(servername : windows_core::PCWSTR, transportname : windows_core::PCWSTR, ucond : FORCE_LEVEL_FLAGS) -> u32);
    NetWkstaTransportDel(servername.param().abi(), transportname.param().abi(), ucond)
}
#[inline]
pub unsafe fn NetWkstaTransportEnum(servername: Option<*const i8>, level: u32, bufptr: *mut *mut u8, prefmaxlen: u32, entriesread: *mut u32, totalentries: *mut u32, resume_handle: Option<*mut u32>) -> u32 {
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaTransportEnum(servername : *const i8, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resume_handle : *mut u32) -> u32);
    NetWkstaTransportEnum(core::mem::transmute(servername.unwrap_or(std::ptr::null())), level, bufptr, prefmaxlen, entriesread, totalentries, core::mem::transmute(resume_handle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetWkstaUserEnum<P0>(servername: P0, level: u32, bufptr: Option<*mut *mut u8>, prefmaxlen: u32, entriesread: Option<*mut u32>, totalentries: *mut u32, resumehandle: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaUserEnum(servername : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8, prefmaxlen : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    NetWkstaUserEnum(servername.param().abi(), level, core::mem::transmute(bufptr.unwrap_or(std::ptr::null_mut())), prefmaxlen, core::mem::transmute(entriesread.unwrap_or(std::ptr::null_mut())), totalentries, core::mem::transmute(resumehandle.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NetWkstaUserGetInfo<P0>(reserved: P0, level: u32, bufptr: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaUserGetInfo(reserved : windows_core::PCWSTR, level : u32, bufptr : *mut *mut u8) -> u32);
    NetWkstaUserGetInfo(reserved.param().abi(), level, bufptr)
}
#[inline]
pub unsafe fn NetWkstaUserSetInfo<P0>(reserved: P0, level: u32, buf: *const u8, parm_err: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn NetWkstaUserSetInfo(reserved : windows_core::PCWSTR, level : u32, buf : *const u8, parm_err : *mut u32) -> u32);
    NetWkstaUserSetInfo(reserved.param().abi(), level, buf, core::mem::transmute(parm_err.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn RouterAssert<P0, P1, P2>(pszfailedassertion: P0, pszfilename: P1, dwlinenumber: u32, pszmessage: P2)
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterAssert(pszfailedassertion : windows_core::PCSTR, pszfilename : windows_core::PCSTR, dwlinenumber : u32, pszmessage : windows_core::PCSTR));
    RouterAssert(pszfailedassertion.param().abi(), pszfilename.param().abi(), dwlinenumber, pszmessage.param().abi())
}
#[inline]
pub unsafe fn RouterGetErrorStringA(dwerrorcode: u32, lplpszerrorstring: *mut windows_core::PSTR) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn RouterGetErrorStringA(dwerrorcode : u32, lplpszerrorstring : *mut windows_core::PSTR) -> u32);
    RouterGetErrorStringA(dwerrorcode, lplpszerrorstring)
}
#[inline]
pub unsafe fn RouterGetErrorStringW(dwerrorcode: u32, lplpwszerrorstring: *mut windows_core::PWSTR) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn RouterGetErrorStringW(dwerrorcode : u32, lplpwszerrorstring : *mut windows_core::PWSTR) -> u32);
    RouterGetErrorStringW(dwerrorcode, lplpwszerrorstring)
}
#[inline]
pub unsafe fn RouterLogDeregisterA<P0>(hloghandle: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogDeregisterA(hloghandle : super::super::Foundation:: HANDLE));
    RouterLogDeregisterA(hloghandle.param().abi())
}
#[inline]
pub unsafe fn RouterLogDeregisterW<P0>(hloghandle: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogDeregisterW(hloghandle : super::super::Foundation:: HANDLE));
    RouterLogDeregisterW(hloghandle.param().abi())
}
#[inline]
pub unsafe fn RouterLogEventA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: Option<&[windows_core::PCSTR]>, dwerrorcode: u32)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventA(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCSTR, dwerrorcode : u32));
    RouterLogEventA(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plpszsubstringarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwerrorcode)
}
#[inline]
pub unsafe fn RouterLogEventDataA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: Option<&[windows_core::PCSTR]>, dwdatabytes: u32, lpdatabytes: *mut u8)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventDataA(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCSTR, dwdatabytes : u32, lpdatabytes : *mut u8));
    RouterLogEventDataA(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plpszsubstringarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwdatabytes, lpdatabytes)
}
#[inline]
pub unsafe fn RouterLogEventDataW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: Option<&[windows_core::PCWSTR]>, dwdatabytes: u32, lpdatabytes: *mut u8)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventDataW(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCWSTR, dwdatabytes : u32, lpdatabytes : *mut u8));
    RouterLogEventDataW(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plpszsubstringarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwdatabytes, lpdatabytes)
}
#[inline]
pub unsafe fn RouterLogEventExA<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn RouterLogEventExA(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwerrorcode : u32, dwmessageid : u32, ptszformat : windows_core::PCSTR));
    RouterLogEventExA(hloghandle.param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.param().abi())
}
#[inline]
pub unsafe fn RouterLogEventExW<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn RouterLogEventExW(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwerrorcode : u32, dwmessageid : u32, ptszformat : windows_core::PCWSTR));
    RouterLogEventExW(hloghandle.param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.param().abi())
}
#[inline]
pub unsafe fn RouterLogEventStringA<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: &[windows_core::PCSTR], dwerrorcode: u32, dwerrorindex: u32)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventStringA(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCSTR, dwerrorcode : u32, dwerrorindex : u32));
    RouterLogEventStringA(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.len().try_into().unwrap(), core::mem::transmute(plpszsubstringarray.as_ptr()), dwerrorcode, dwerrorindex)
}
#[inline]
pub unsafe fn RouterLogEventStringW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: &[windows_core::PCWSTR], dwerrorcode: u32, dwerrorindex: u32)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventStringW(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCWSTR, dwerrorcode : u32, dwerrorindex : u32));
    RouterLogEventStringW(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.len().try_into().unwrap(), core::mem::transmute(plpszsubstringarray.as_ptr()), dwerrorcode, dwerrorindex)
}
#[inline]
pub unsafe fn RouterLogEventValistExA<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1, arglist: *mut i8)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventValistExA(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwerrorcode : u32, dwmessageid : u32, ptszformat : windows_core::PCSTR, arglist : *mut i8));
    RouterLogEventValistExA(hloghandle.param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.param().abi(), arglist)
}
#[inline]
pub unsafe fn RouterLogEventValistExW<P0, P1>(hloghandle: P0, dweventtype: u32, dwerrorcode: u32, dwmessageid: u32, ptszformat: P1, arglist: *mut i8)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventValistExW(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwerrorcode : u32, dwmessageid : u32, ptszformat : windows_core::PCWSTR, arglist : *mut i8));
    RouterLogEventValistExW(hloghandle.param().abi(), dweventtype, dwerrorcode, dwmessageid, ptszformat.param().abi(), arglist)
}
#[inline]
pub unsafe fn RouterLogEventW<P0>(hloghandle: P0, dweventtype: u32, dwmessageid: u32, plpszsubstringarray: Option<&[windows_core::PCWSTR]>, dwerrorcode: u32)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogEventW(hloghandle : super::super::Foundation:: HANDLE, dweventtype : u32, dwmessageid : u32, dwsubstringcount : u32, plpszsubstringarray : *const windows_core::PCWSTR, dwerrorcode : u32));
    RouterLogEventW(hloghandle.param().abi(), dweventtype, dwmessageid, plpszsubstringarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(plpszsubstringarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwerrorcode)
}
#[inline]
pub unsafe fn RouterLogRegisterA<P0>(lpszsource: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogRegisterA(lpszsource : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    RouterLogRegisterA(lpszsource.param().abi())
}
#[inline]
pub unsafe fn RouterLogRegisterW<P0>(lpszsource: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn RouterLogRegisterW(lpszsource : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    RouterLogRegisterW(lpszsource.param().abi())
}
#[inline]
pub unsafe fn SetNetScheduleAccountInformation<P0, P1, P2>(pwszservername: P0, pwszaccount: P1, pwszpassword: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mstask.dll" "system" fn SetNetScheduleAccountInformation(pwszservername : windows_core::PCWSTR, pwszaccount : windows_core::PCWSTR, pwszpassword : windows_core::PCWSTR) -> windows_core::HRESULT);
    SetNetScheduleAccountInformation(pwszservername.param().abi(), pwszaccount.param().abi(), pwszpassword.param().abi()).ok()
}
#[inline]
pub unsafe fn TraceDeregisterA(dwtraceid: u32) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceDeregisterA(dwtraceid : u32) -> u32);
    TraceDeregisterA(dwtraceid)
}
#[inline]
pub unsafe fn TraceDeregisterExA(dwtraceid: u32, dwflags: u32) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceDeregisterExA(dwtraceid : u32, dwflags : u32) -> u32);
    TraceDeregisterExA(dwtraceid, dwflags)
}
#[inline]
pub unsafe fn TraceDeregisterExW(dwtraceid: u32, dwflags: u32) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceDeregisterExW(dwtraceid : u32, dwflags : u32) -> u32);
    TraceDeregisterExW(dwtraceid, dwflags)
}
#[inline]
pub unsafe fn TraceDeregisterW(dwtraceid: u32) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceDeregisterW(dwtraceid : u32) -> u32);
    TraceDeregisterW(dwtraceid)
}
#[inline]
pub unsafe fn TraceDumpExA<P0, P1>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: P0, lpszprefix: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceDumpExA(dwtraceid : u32, dwflags : u32, lpbbytes : *mut u8, dwbytecount : u32, dwgroupsize : u32, baddressprefix : super::super::Foundation:: BOOL, lpszprefix : windows_core::PCSTR) -> u32);
    TraceDumpExA(dwtraceid, dwflags, lpbbytes, dwbytecount, dwgroupsize, baddressprefix.param().abi(), lpszprefix.param().abi())
}
#[inline]
pub unsafe fn TraceDumpExW<P0, P1>(dwtraceid: u32, dwflags: u32, lpbbytes: *mut u8, dwbytecount: u32, dwgroupsize: u32, baddressprefix: P0, lpszprefix: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceDumpExW(dwtraceid : u32, dwflags : u32, lpbbytes : *mut u8, dwbytecount : u32, dwgroupsize : u32, baddressprefix : super::super::Foundation:: BOOL, lpszprefix : windows_core::PCWSTR) -> u32);
    TraceDumpExW(dwtraceid, dwflags, lpbbytes, dwbytecount, dwgroupsize, baddressprefix.param().abi(), lpszprefix.param().abi())
}
#[inline]
pub unsafe fn TraceGetConsoleA(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceGetConsoleA(dwtraceid : u32, lphconsole : *mut super::super::Foundation:: HANDLE) -> u32);
    TraceGetConsoleA(dwtraceid, lphconsole)
}
#[inline]
pub unsafe fn TraceGetConsoleW(dwtraceid: u32, lphconsole: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("rtutils.dll" "system" fn TraceGetConsoleW(dwtraceid : u32, lphconsole : *mut super::super::Foundation:: HANDLE) -> u32);
    TraceGetConsoleW(dwtraceid, lphconsole)
}
#[inline]
pub unsafe fn TracePrintfA<P0>(dwtraceid: u32, lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn TracePrintfA(dwtraceid : u32, lpszformat : windows_core::PCSTR) -> u32);
    TracePrintfA(dwtraceid, lpszformat.param().abi())
}
#[inline]
pub unsafe fn TracePrintfExA<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn TracePrintfExA(dwtraceid : u32, dwflags : u32, lpszformat : windows_core::PCSTR) -> u32);
    TracePrintfExA(dwtraceid, dwflags, lpszformat.param().abi())
}
#[inline]
pub unsafe fn TracePrintfExW<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn TracePrintfExW(dwtraceid : u32, dwflags : u32, lpszformat : windows_core::PCWSTR) -> u32);
    TracePrintfExW(dwtraceid, dwflags, lpszformat.param().abi())
}
#[inline]
pub unsafe fn TracePrintfW<P0>(dwtraceid: u32, lpszformat: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "cdecl" fn TracePrintfW(dwtraceid : u32, lpszformat : windows_core::PCWSTR) -> u32);
    TracePrintfW(dwtraceid, lpszformat.param().abi())
}
#[inline]
pub unsafe fn TracePutsExA<P0>(dwtraceid: u32, dwflags: u32, lpszstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TracePutsExA(dwtraceid : u32, dwflags : u32, lpszstring : windows_core::PCSTR) -> u32);
    TracePutsExA(dwtraceid, dwflags, lpszstring.param().abi())
}
#[inline]
pub unsafe fn TracePutsExW<P0>(dwtraceid: u32, dwflags: u32, lpszstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TracePutsExW(dwtraceid : u32, dwflags : u32, lpszstring : windows_core::PCWSTR) -> u32);
    TracePutsExW(dwtraceid, dwflags, lpszstring.param().abi())
}
#[inline]
pub unsafe fn TraceRegisterExA<P0>(lpszcallername: P0, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceRegisterExA(lpszcallername : windows_core::PCSTR, dwflags : u32) -> u32);
    TraceRegisterExA(lpszcallername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn TraceRegisterExW<P0>(lpszcallername: P0, dwflags: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceRegisterExW(lpszcallername : windows_core::PCWSTR, dwflags : u32) -> u32);
    TraceRegisterExW(lpszcallername.param().abi(), dwflags)
}
#[inline]
pub unsafe fn TraceVprintfExA<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0, arglist: *mut i8) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceVprintfExA(dwtraceid : u32, dwflags : u32, lpszformat : windows_core::PCSTR, arglist : *mut i8) -> u32);
    TraceVprintfExA(dwtraceid, dwflags, lpszformat.param().abi(), arglist)
}
#[inline]
pub unsafe fn TraceVprintfExW<P0>(dwtraceid: u32, dwflags: u32, lpszformat: P0, arglist: *mut i8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtutils.dll" "system" fn TraceVprintfExW(dwtraceid : u32, dwflags : u32, lpszformat : windows_core::PCWSTR, arglist : *mut i8) -> u32);
    TraceVprintfExW(dwtraceid, dwflags, lpszformat.param().abi(), arglist)
}
windows_core::imp::define_interface!(IEnumNetCfgBindingInterface, IEnumNetCfgBindingInterface_Vtbl, 0xc0e8ae90_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for IEnumNetCfgBindingInterface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetCfgBindingInterface, windows_core::IUnknown);
impl IEnumNetCfgBindingInterface {
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetCfgBindingInterface>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumNetCfgBindingInterface>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IEnumNetCfgBindingInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetCfgBindingPath, IEnumNetCfgBindingPath_Vtbl, 0xc0e8ae91_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for IEnumNetCfgBindingPath {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetCfgBindingPath, windows_core::IUnknown);
impl IEnumNetCfgBindingPath {
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetCfgBindingPath>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumNetCfgBindingPath>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IEnumNetCfgBindingPath_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumNetCfgComponent, IEnumNetCfgComponent_Vtbl, 0xc0e8ae92_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for IEnumNetCfgComponent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumNetCfgComponent, windows_core::IUnknown);
impl IEnumNetCfgComponent {
    pub unsafe fn Next(&self, rgelt: &mut [Option<INetCfgComponent>], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: Option<*const Option<IEnumNetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IEnumNetCfgComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfg, INetCfg_Vtbl, 0xc0e8ae93_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfg {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfg, windows_core::IUnknown);
impl INetCfg {
    pub unsafe fn Initialize(&self, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(pvreserved.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn Uninitialize(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumComponents(&self, pguidclass: *const windows_core::GUID, ppenumcomponent: Option<*mut Option<IEnumNetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumComponents)(windows_core::Interface::as_raw(self), pguidclass, core::mem::transmute(ppenumcomponent.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FindComponent<P0>(&self, pszwinfid: P0, pcomponent: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindComponent)(windows_core::Interface::as_raw(self), pszwinfid.param().abi(), core::mem::transmute(pcomponent.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn QueryNetCfgClass(&self, pguidclass: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryNetCfgClass)(windows_core::Interface::as_raw(self), pguidclass, riid, core::mem::transmute(ppvobject.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfg_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindComponent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub QueryNetCfgClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgBindingInterface, INetCfgBindingInterface_Vtbl, 0xc0e8ae94_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgBindingInterface {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgBindingInterface, windows_core::IUnknown);
impl INetCfgBindingInterface {
    pub unsafe fn GetName(&self, ppszwinterfacename: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwinterfacename.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetUpperComponent(&self, ppnccitem: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUpperComponent)(windows_core::Interface::as_raw(self), core::mem::transmute(ppnccitem.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLowerComponent(&self, ppnccitem: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLowerComponent)(windows_core::Interface::as_raw(self), core::mem::transmute(ppnccitem.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfgBindingInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetUpperComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLowerComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgBindingPath, INetCfgBindingPath_Vtbl, 0xc0e8ae96_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgBindingPath {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgBindingPath, windows_core::IUnknown);
impl INetCfgBindingPath {
    pub unsafe fn IsSamePathAs<P0>(&self, ppath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).IsSamePathAs)(windows_core::Interface::as_raw(self), ppath.param().abi()).ok()
    }
    pub unsafe fn IsSubPathOf<P0>(&self, ppath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).IsSubPathOf)(windows_core::Interface::as_raw(self), ppath.param().abi()).ok()
    }
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    pub unsafe fn GetPathToken(&self, ppszwpathtoken: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPathToken)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwpathtoken.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetOwner(&self, ppcomponent: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOwner)(windows_core::Interface::as_raw(self), core::mem::transmute(ppcomponent.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDepth(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumBindingInterfaces(&self, ppenuminterface: Option<*mut Option<IEnumNetCfgBindingInterface>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumBindingInterfaces)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenuminterface.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfgBindingPath_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsSamePathAs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSubPathOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPathToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumBindingInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgClass, INetCfgClass_Vtbl, 0xc0e8ae97_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgClass {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgClass, windows_core::IUnknown);
impl INetCfgClass {
    pub unsafe fn FindComponent<P0>(&self, pszwinfid: P0, ppnccitem: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FindComponent)(windows_core::Interface::as_raw(self), pszwinfid.param().abi(), core::mem::transmute(ppnccitem.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumComponents(&self, ppenumcomponent: Option<*mut Option<IEnumNetCfgComponent>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumComponents)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenumcomponent.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfgClass_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindComponent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumComponents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgClassSetup, INetCfgClassSetup_Vtbl, 0xc0e8ae9d_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgClassSetup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgClassSetup, windows_core::IUnknown);
impl INetCfgClassSetup {
    pub unsafe fn SelectAndInstall<P0>(&self, hwndparent: P0, pobotoken: Option<*const OBO_TOKEN>, ppnccitem: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SelectAndInstall)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), core::mem::transmute(pobotoken.unwrap_or(std::ptr::null())), core::mem::transmute(ppnccitem.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Install<P0, P1, P2>(&self, pszwinfid: P0, pobotoken: Option<*const OBO_TOKEN>, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: P1, pszwanswersections: P2, ppnccitem: Option<*mut Option<INetCfgComponent>>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), pszwinfid.param().abi(), core::mem::transmute(pobotoken.unwrap_or(std::ptr::null())), dwsetupflags, dwupgradefrombuildno, pszwanswerfile.param().abi(), pszwanswersections.param().abi(), core::mem::transmute(ppnccitem.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn DeInstall<P0>(&self, pcomponent: P0, pobotoken: Option<*const OBO_TOKEN>, pmszwrefs: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).DeInstall)(windows_core::Interface::as_raw(self), pcomponent.param().abi(), core::mem::transmute(pobotoken.unwrap_or(std::ptr::null())), core::mem::transmute(pmszwrefs.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfgClassSetup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SelectAndInstall: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const OBO_TOKEN, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const OBO_TOKEN, u32, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const OBO_TOKEN, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgClassSetup2, INetCfgClassSetup2_Vtbl, 0xc0e8aea0_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgClassSetup2 {
    type Target = INetCfgClassSetup;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgClassSetup2, windows_core::IUnknown, INetCfgClassSetup);
impl INetCfgClassSetup2 {
    pub unsafe fn UpdateNonEnumeratedComponent<P0>(&self, picomp: P0, dwsetupflags: u32, dwupgradefrombuildno: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).UpdateNonEnumeratedComponent)(windows_core::Interface::as_raw(self), picomp.param().abi(), dwsetupflags, dwupgradefrombuildno).ok()
    }
}
#[repr(C)]
pub struct INetCfgClassSetup2_Vtbl {
    pub base__: INetCfgClassSetup_Vtbl,
    pub UpdateNonEnumeratedComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponent, INetCfgComponent_Vtbl, 0xc0e8ae99_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgComponent {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponent, windows_core::IUnknown);
impl INetCfgComponent {
    pub unsafe fn GetDisplayName(&self, ppszwdisplayname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwdisplayname.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszwdisplayname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), pszwdisplayname.param().abi()).ok()
    }
    pub unsafe fn GetHelpText(&self, pszwhelptext: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHelpText)(windows_core::Interface::as_raw(self), core::mem::transmute(pszwhelptext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetId(&self, ppszwid: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInstanceGuid(&self, pguid: Option<*mut windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInstanceGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(pguid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetPnpDevNodeId(&self, ppszwdevnodeid: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPnpDevNodeId)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwdevnodeid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetClassGuid(&self, pguid: Option<*mut windows_core::GUID>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetClassGuid)(windows_core::Interface::as_raw(self), core::mem::transmute(pguid.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetBindName(&self, ppszwbindname: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindName)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwbindname.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDeviceStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenParamKey(&self, phkey: Option<*mut super::super::System::Registry::HKEY>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenParamKey)(windows_core::Interface::as_raw(self), core::mem::transmute(phkey.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn RaisePropertyUi<P0, P1>(&self, hwndparent: P0, dwflags: u32, punkcontext: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RaisePropertyUi)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), dwflags, punkcontext.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInstanceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPnpDevNodeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetClassGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetBindName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenParamKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenParamKey: usize,
    pub RaisePropertyUi: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentBindings, INetCfgComponentBindings_Vtbl, 0xc0e8ae9e_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgComponentBindings {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentBindings, windows_core::IUnknown);
impl INetCfgComponentBindings {
    pub unsafe fn BindTo<P0>(&self, pnccitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).BindTo)(windows_core::Interface::as_raw(self), pnccitem.param().abi()).ok()
    }
    pub unsafe fn UnbindFrom<P0>(&self, pnccitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).UnbindFrom)(windows_core::Interface::as_raw(self), pnccitem.param().abi()).ok()
    }
    pub unsafe fn SupportsBindingInterface<P0>(&self, dwflags: u32, pszwinterfacename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SupportsBindingInterface)(windows_core::Interface::as_raw(self), dwflags, pszwinterfacename.param().abi()).ok()
    }
    pub unsafe fn IsBoundTo<P0>(&self, pnccitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).IsBoundTo)(windows_core::Interface::as_raw(self), pnccitem.param().abi()).ok()
    }
    pub unsafe fn IsBindableTo<P0>(&self, pnccitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).IsBindableTo)(windows_core::Interface::as_raw(self), pnccitem.param().abi()).ok()
    }
    pub unsafe fn EnumBindingPaths(&self, dwflags: u32, ppienum: Option<*mut Option<IEnumNetCfgBindingPath>>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumBindingPaths)(windows_core::Interface::as_raw(self), dwflags, core::mem::transmute(ppienum.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn MoveBefore<P0, P1>(&self, pncbitemsrc: P0, pncbitemdest: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
        P1: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).MoveBefore)(windows_core::Interface::as_raw(self), pncbitemsrc.param().abi(), pncbitemdest.param().abi()).ok()
    }
    pub unsafe fn MoveAfter<P0, P1>(&self, pncbitemsrc: P0, pncbitemdest: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
        P1: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).MoveAfter)(windows_core::Interface::as_raw(self), pncbitemsrc.param().abi(), pncbitemdest.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentBindings_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportsBindingInterface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsBoundTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsBindableTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumBindingPaths: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentControl, INetCfgComponentControl_Vtbl, 0x932238df_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentControl, windows_core::IUnknown);
impl INetCfgComponentControl {
    pub unsafe fn Initialize<P0, P1, P2>(&self, picomp: P0, pinetcfg: P1, finstalling: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
        P1: windows_core::Param<INetCfg>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), picomp.param().abi(), pinetcfg.param().abi(), finstalling.param().abi()).ok()
    }
    pub unsafe fn ApplyRegistryChanges(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyRegistryChanges)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ApplyPnpChanges<P0>(&self, picallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgPnpReconfigCallback>,
    {
        (windows_core::Interface::vtable(self).ApplyPnpChanges)(windows_core::Interface::as_raw(self), picallback.param().abi()).ok()
    }
    pub unsafe fn CancelChanges(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelChanges)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ApplyRegistryChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApplyPnpChanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentNotifyBinding, INetCfgComponentNotifyBinding_Vtbl, 0x932238e1_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentNotifyBinding {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentNotifyBinding, windows_core::IUnknown);
impl INetCfgComponentNotifyBinding {
    pub unsafe fn QueryBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).QueryBindingPath)(windows_core::Interface::as_raw(self), dwchangeflag, pipath.param().abi()).ok()
    }
    pub unsafe fn NotifyBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).NotifyBindingPath)(windows_core::Interface::as_raw(self), dwchangeflag, pipath.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentNotifyBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryBindingPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyBindingPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentNotifyGlobal, INetCfgComponentNotifyGlobal_Vtbl, 0x932238e2_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentNotifyGlobal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentNotifyGlobal, windows_core::IUnknown);
impl INetCfgComponentNotifyGlobal {
    pub unsafe fn GetSupportedNotifications(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSupportedNotifications)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SysQueryBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).SysQueryBindingPath)(windows_core::Interface::as_raw(self), dwchangeflag, pipath.param().abi()).ok()
    }
    pub unsafe fn SysNotifyBindingPath<P0>(&self, dwchangeflag: u32, pipath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgBindingPath>,
    {
        (windows_core::Interface::vtable(self).SysNotifyBindingPath)(windows_core::Interface::as_raw(self), dwchangeflag, pipath.param().abi()).ok()
    }
    pub unsafe fn SysNotifyComponent<P0>(&self, dwchangeflag: u32, picomp: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).SysNotifyComponent)(windows_core::Interface::as_raw(self), dwchangeflag, picomp.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentNotifyGlobal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSupportedNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SysQueryBindingPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SysNotifyBindingPath: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SysNotifyComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentPropertyUi, INetCfgComponentPropertyUi_Vtbl, 0x932238e0_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentPropertyUi {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentPropertyUi, windows_core::IUnknown);
impl INetCfgComponentPropertyUi {
    pub unsafe fn QueryPropertyUi<P0>(&self, punkreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).QueryPropertyUi)(windows_core::Interface::as_raw(self), punkreserved.param().abi()).ok()
    }
    pub unsafe fn SetContext<P0>(&self, punkreserved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), punkreserved.param().abi()).ok()
    }
    pub unsafe fn MergePropPages<P0>(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: P0, pszstartpage: Option<*const windows_core::PCWSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).MergePropPages)(windows_core::Interface::as_raw(self), pdwdefpages, pahpspprivate, pcpages, hwndparent.param().abi(), core::mem::transmute(pszstartpage.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn ValidateProperties<P0>(&self, hwndsheet: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).ValidateProperties)(windows_core::Interface::as_raw(self), hwndsheet.param().abi()).ok()
    }
    pub unsafe fn ApplyProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplyProperties)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CancelProperties(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelProperties)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentPropertyUi_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryPropertyUi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MergePropPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8, *mut u32, super::super::Foundation::HWND, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ValidateProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ApplyProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentSetup, INetCfgComponentSetup_Vtbl, 0x932238e3_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentSetup {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentSetup, windows_core::IUnknown);
impl INetCfgComponentSetup {
    pub unsafe fn Install(&self, dwsetupflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), dwsetupflags).ok()
    }
    pub unsafe fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Upgrade)(windows_core::Interface::as_raw(self), dwsetupflags, dwupgradefombuildno).ok()
    }
    pub unsafe fn ReadAnswerFile<P0, P1>(&self, pszwanswerfile: P0, pszwanswersections: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReadAnswerFile)(windows_core::Interface::as_raw(self), pszwanswerfile.param().abi(), pszwanswersections.param().abi()).ok()
    }
    pub unsafe fn Removing(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Removing)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentSetup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Upgrade: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReadAnswerFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Removing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentSysPrep, INetCfgComponentSysPrep_Vtbl, 0xc0e8ae9a_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgComponentSysPrep {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentSysPrep, windows_core::IUnknown);
impl INetCfgComponentSysPrep {
    pub unsafe fn SaveAdapterParameters<P0, P1>(&self, pncsp: P0, pszwanswersections: P1, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgSysPrep>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveAdapterParameters)(windows_core::Interface::as_raw(self), pncsp.param().abi(), pszwanswersections.param().abi(), padapterinstanceguid).ok()
    }
    pub unsafe fn RestoreAdapterParameters<P0, P1>(&self, pszwanswerfile: P0, pszwanswersection: P1, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RestoreAdapterParameters)(windows_core::Interface::as_raw(self), pszwanswerfile.param().abi(), pszwanswersection.param().abi(), padapterinstanceguid).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentSysPrep_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SaveAdapterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreAdapterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgComponentUpperEdge, INetCfgComponentUpperEdge_Vtbl, 0x932238e4_bea1_11d0_9298_00c04fc99dcf);
impl core::ops::Deref for INetCfgComponentUpperEdge {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgComponentUpperEdge, windows_core::IUnknown);
impl INetCfgComponentUpperEdge {
    pub unsafe fn GetInterfaceIdsForAdapter<P0>(&self, padapter: P0, pdwnuminterfaces: *mut u32, ppguidinterfaceids: Option<*mut *mut windows_core::GUID>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).GetInterfaceIdsForAdapter)(windows_core::Interface::as_raw(self), padapter.param().abi(), pdwnuminterfaces, core::mem::transmute(ppguidinterfaceids.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddInterfacesToAdapter<P0>(&self, padapter: P0, dwnuminterfaces: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).AddInterfacesToAdapter)(windows_core::Interface::as_raw(self), padapter.param().abi(), dwnuminterfaces).ok()
    }
    pub unsafe fn RemoveInterfacesFromAdapter<P0>(&self, padapter: P0, pguidinterfaceids: &[windows_core::GUID]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<INetCfgComponent>,
    {
        (windows_core::Interface::vtable(self).RemoveInterfacesFromAdapter)(windows_core::Interface::as_raw(self), padapter.param().abi(), pguidinterfaceids.len().try_into().unwrap(), core::mem::transmute(pguidinterfaceids.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct INetCfgComponentUpperEdge_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInterfaceIdsForAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub AddInterfacesToAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveInterfacesFromAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgLock, INetCfgLock_Vtbl, 0xc0e8ae9f_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgLock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgLock, windows_core::IUnknown);
impl INetCfgLock {
    pub unsafe fn AcquireWriteLock<P0>(&self, cmstimeout: u32, pszwclientdescription: P0, ppszwclientdescription: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AcquireWriteLock)(windows_core::Interface::as_raw(self), cmstimeout, pszwclientdescription.param().abi(), core::mem::transmute(ppszwclientdescription.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseWriteLock(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseWriteLock)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsWriteLocked(&self, ppszwclientdescription: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsWriteLocked)(windows_core::Interface::as_raw(self), core::mem::transmute(ppszwclientdescription.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct INetCfgLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AcquireWriteLock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ReleaseWriteLock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsWriteLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgPnpReconfigCallback, INetCfgPnpReconfigCallback_Vtbl, 0x8d84bd35_e227_11d2_b700_00a0c98a6a85);
impl core::ops::Deref for INetCfgPnpReconfigCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgPnpReconfigCallback, windows_core::IUnknown);
impl INetCfgPnpReconfigCallback {
    pub unsafe fn SendPnpReconfig<P0, P1>(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: P0, pszwlower: P1, pvdata: *const core::ffi::c_void, dwsizeofdata: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SendPnpReconfig)(windows_core::Interface::as_raw(self), layer, pszwupper.param().abi(), pszwlower.param().abi(), pvdata, dwsizeofdata).ok()
    }
}
#[repr(C)]
pub struct INetCfgPnpReconfigCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SendPnpReconfig: unsafe extern "system" fn(*mut core::ffi::c_void, NCPNP_RECONFIG_LAYER, windows_core::PCWSTR, windows_core::PCWSTR, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetCfgSysPrep, INetCfgSysPrep_Vtbl, 0xc0e8ae98_306e_11d1_aacf_00805fc1270e);
impl core::ops::Deref for INetCfgSysPrep {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetCfgSysPrep, windows_core::IUnknown);
impl INetCfgSysPrep {
    pub unsafe fn HrSetupSetFirstDword<P0, P1>(&self, pwszsection: P0, pwszkey: P1, dwvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).HrSetupSetFirstDword)(windows_core::Interface::as_raw(self), pwszsection.param().abi(), pwszkey.param().abi(), dwvalue).ok()
    }
    pub unsafe fn HrSetupSetFirstString<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, pwszvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).HrSetupSetFirstString)(windows_core::Interface::as_raw(self), pwszsection.param().abi(), pwszkey.param().abi(), pwszvalue.param().abi()).ok()
    }
    pub unsafe fn HrSetupSetFirstStringAsBool<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, fvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).HrSetupSetFirstStringAsBool)(windows_core::Interface::as_raw(self), pwszsection.param().abi(), pwszkey.param().abi(), fvalue.param().abi()).ok()
    }
    pub unsafe fn HrSetupSetFirstMultiSzField<P0, P1, P2>(&self, pwszsection: P0, pwszkey: P1, pmszvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).HrSetupSetFirstMultiSzField)(windows_core::Interface::as_raw(self), pwszsection.param().abi(), pwszkey.param().abi(), pmszvalue.param().abi()).ok()
    }
}
#[repr(C)]
pub struct INetCfgSysPrep_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HrSetupSetFirstDword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub HrSetupSetFirstString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub HrSetupSetFirstStringAsBool: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub HrSetupSetFirstMultiSzField: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetLanConnectionUiInfo, INetLanConnectionUiInfo_Vtbl, 0xc08956a6_1cd3_11d1_b1c5_00805fc1270e);
impl core::ops::Deref for INetLanConnectionUiInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetLanConnectionUiInfo, windows_core::IUnknown);
impl INetLanConnectionUiInfo {
    pub unsafe fn GetDeviceGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct INetLanConnectionUiInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INetRasConnectionIpUiInfo, INetRasConnectionIpUiInfo_Vtbl, 0xfaedcf58_31fe_11d1_aad2_00805fc1270e);
impl core::ops::Deref for INetRasConnectionIpUiInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INetRasConnectionIpUiInfo, windows_core::IUnknown);
impl INetRasConnectionIpUiInfo {
    pub unsafe fn GetUiInfo(&self, pinfo: *mut RASCON_IPUI) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUiInfo)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct INetRasConnectionIpUiInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUiInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RASCON_IPUI) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvisioningDomain, IProvisioningDomain_Vtbl, 0xc96fbd50_24dd_11d8_89fb_00904b2ea9c6);
impl core::ops::Deref for IProvisioningDomain {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvisioningDomain, windows_core::IUnknown);
impl IProvisioningDomain {
    pub unsafe fn Add<P0>(&self, pszwpathtofolder: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pszwpathtofolder.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn Query<P0, P1, P2>(&self, pszwdomain: P0, pszwlanguage: P1, pszwxpathquery: P2) -> windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pszwdomain.param().abi(), pszwlanguage.param().abi(), pszwxpathquery.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IProvisioningDomain_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    Query: usize,
}
windows_core::imp::define_interface!(IProvisioningProfileWireless, IProvisioningProfileWireless_Vtbl, 0xc96fbd51_24dd_11d8_89fb_00904b2ea9c6);
impl core::ops::Deref for IProvisioningProfileWireless {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvisioningProfileWireless, windows_core::IUnknown);
impl IProvisioningProfileWireless {
    pub unsafe fn CreateProfile<P0, P1>(&self, bstrxmlwirelessconfigprofile: P0, bstrxmlconnectionconfigprofile: P1, padapterinstanceguid: *const windows_core::GUID) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateProfile)(windows_core::Interface::as_raw(self), bstrxmlwirelessconfigprofile.param().abi(), bstrxmlconnectionconfigprofile.param().abi(), padapterinstanceguid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProvisioningProfileWireless_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProfile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
}
pub const AA_AUDIT_ALL: u32 = 1u32;
pub const AA_A_ACL: u32 = 32768u32;
pub const AA_A_CREATE: u32 = 8192u32;
pub const AA_A_DELETE: u32 = 16384u32;
pub const AA_A_OPEN: u32 = 4096u32;
pub const AA_A_OWNER: u32 = 4u32;
pub const AA_A_WRITE: u32 = 8192u32;
pub const AA_CLOSE: u32 = 8u32;
pub const AA_F_ACL: u32 = 2048u32;
pub const AA_F_CREATE: u32 = 512u32;
pub const AA_F_DELETE: u32 = 1024u32;
pub const AA_F_OPEN: u32 = 256u32;
pub const AA_F_WRITE: u32 = 512u32;
pub const AA_S_ACL: u32 = 128u32;
pub const AA_S_CREATE: u32 = 32u32;
pub const AA_S_DELETE: u32 = 64u32;
pub const AA_S_OPEN: u32 = 16u32;
pub const AA_S_WRITE: u32 = 32u32;
pub const ACCESS_ACCESS_LIST_PARMNUM: u32 = 4u32;
pub const ACCESS_ATTR_PARMNUM: u32 = 2u32;
pub const ACCESS_AUDIT: u32 = 1u32;
pub const ACCESS_COUNT_PARMNUM: u32 = 3u32;
pub const ACCESS_FAIL_ACL: u32 = 2048u32;
pub const ACCESS_FAIL_DELETE: u32 = 1024u32;
pub const ACCESS_FAIL_MASK: u32 = 3840u32;
pub const ACCESS_FAIL_OPEN: u32 = 256u32;
pub const ACCESS_FAIL_SHIFT: u32 = 4u32;
pub const ACCESS_FAIL_WRITE: u32 = 512u32;
pub const ACCESS_GROUP: u32 = 32768u32;
pub const ACCESS_LETTERS: windows_core::PCSTR = windows_core::s!("RWCXDAP         ");
pub const ACCESS_NONE: u32 = 0u32;
pub const ACCESS_RESOURCE_NAME_PARMNUM: u32 = 1u32;
pub const ACCESS_SUCCESS_ACL: u32 = 128u32;
pub const ACCESS_SUCCESS_DELETE: u32 = 64u32;
pub const ACCESS_SUCCESS_MASK: u32 = 240u32;
pub const ACCESS_SUCCESS_OPEN: u32 = 16u32;
pub const ACCESS_SUCCESS_WRITE: u32 = 32u32;
pub const ACTION_ADMINUNLOCK: u32 = 1u32;
pub const ACTION_LOCKOUT: u32 = 0u32;
pub const AE_ACCLIMITEXCD: u32 = 17u32;
pub const AE_ACCRESTRICT: u32 = 4u32;
pub const AE_ACLMOD: u32 = 12u32;
pub const AE_ACLMODFAIL: u32 = 19u32;
pub const AE_ADD: u32 = 2u32;
pub const AE_ADMIN: u32 = 2u32;
pub const AE_ADMINDIS: u32 = 3u32;
pub const AE_ADMINPRIVREQD: u32 = 2u32;
pub const AE_ADMIN_CLOSE: u32 = 2u32;
pub const AE_AUTODIS: u32 = 2u32;
pub const AE_BADPW: u32 = 1u32;
pub const AE_CLOSEFILE: u32 = 9u32;
pub const AE_CONNREJ: u32 = 6u32;
pub const AE_CONNSTART: u32 = 4u32;
pub const AE_CONNSTOP: u32 = 5u32;
pub const AE_DELETE: u32 = 1u32;
pub const AE_ERROR: u32 = 1u32;
pub const AE_GENERAL: u32 = 0u32;
pub const AE_GENERIC_TYPE: u32 = 21u32;
pub const AE_GUEST: u32 = 0u32;
pub const AE_LIM_DELETED: u32 = 5u32;
pub const AE_LIM_DISABLED: u32 = 4u32;
pub const AE_LIM_EXPIRED: u32 = 2u32;
pub const AE_LIM_INVAL_WKSTA: u32 = 3u32;
pub const AE_LIM_LOGONHOURS: u32 = 1u32;
pub const AE_LIM_UNKNOWN: u32 = 0u32;
pub const AE_LOCKOUT: u32 = 20u32;
pub const AE_MOD: u32 = 0u32;
pub const AE_NETLOGDENIED: u32 = 16u32;
pub const AE_NETLOGOFF: u32 = 15u32;
pub const AE_NETLOGON: u32 = 14u32;
pub const AE_NOACCESSPERM: u32 = 3u32;
pub const AE_NORMAL: u32 = 0u32;
pub const AE_NORMAL_CLOSE: u32 = 0u32;
pub const AE_RESACCESS: u32 = 7u32;
pub const AE_RESACCESS2: u32 = 18u32;
pub const AE_RESACCESSREJ: u32 = 8u32;
pub const AE_SERVICESTAT: u32 = 11u32;
pub const AE_SESSDIS: u32 = 1u32;
pub const AE_SESSLOGOFF: u32 = 2u32;
pub const AE_SESSLOGON: u32 = 1u32;
pub const AE_SESSPWERR: u32 = 3u32;
pub const AE_SES_CLOSE: u32 = 1u32;
pub const AE_SRVCONT: u32 = 2u32;
pub const AE_SRVPAUSED: u32 = 1u32;
pub const AE_SRVSTART: u32 = 0u32;
pub const AE_SRVSTATUS: u32 = 0u32;
pub const AE_SRVSTOP: u32 = 3u32;
pub const AE_UASMOD: u32 = 13u32;
pub const AE_UAS_GROUP: u32 = 1u32;
pub const AE_UAS_MODALS: u32 = 2u32;
pub const AE_UAS_USER: u32 = 0u32;
pub const AE_UNSHARE: u32 = 2u32;
pub const AE_USER: u32 = 1u32;
pub const AE_USERLIMIT: u32 = 0u32;
pub const AF_OP_ACCOUNTS: AF_OP = AF_OP(8u32);
pub const AF_OP_COMM: AF_OP = AF_OP(2u32);
pub const AF_OP_PRINT: AF_OP = AF_OP(1u32);
pub const AF_OP_SERVER: AF_OP = AF_OP(4u32);
pub const ALERTER_MAILSLOT: windows_core::PCWSTR = windows_core::w!("\\\\.\\MAILSLOT\\Alerter");
pub const ALERTSZ: u32 = 128u32;
pub const ALERT_ADMIN_EVENT: windows_core::PCWSTR = windows_core::w!("ADMIN");
pub const ALERT_ERRORLOG_EVENT: windows_core::PCWSTR = windows_core::w!("ERRORLOG");
pub const ALERT_MESSAGE_EVENT: windows_core::PCWSTR = windows_core::w!("MESSAGE");
pub const ALERT_PRINT_EVENT: windows_core::PCWSTR = windows_core::w!("PRINTING");
pub const ALERT_USER_EVENT: windows_core::PCWSTR = windows_core::w!("USER");
pub const ALIGN_SHIFT: u32 = 7u32;
pub const ALIGN_SIZE: u32 = 8u32;
pub const ALLOCATE_RESPONSE: u32 = 2u32;
pub const BACKUP_MSG_FILENAME: windows_core::PCWSTR = windows_core::w!("BAK.MSG");
pub const CLTYPE_LEN: u32 = 12u32;
pub const CNLEN: u32 = 15u32;
pub const COULD_NOT_VERIFY_VOLUMES: i32 = -1073727512i32;
pub const CREATE_BYPASS_CSC: u32 = 2u32;
pub const CREATE_CRED_RESET: u32 = 4u32;
pub const CREATE_GLOBAL_MAPPING: u32 = 256u32;
pub const CREATE_NO_CONNECT: u32 = 1u32;
pub const CREATE_PERSIST_MAPPING: u32 = 32u32;
pub const CREATE_REQUIRE_CONNECTION_INTEGRITY: u32 = 8u32;
pub const CREATE_REQUIRE_CONNECTION_PRIVACY: u32 = 16u32;
pub const CREATE_WRITE_THROUGH_SEMANTICS: u32 = 64u32;
pub const CRYPT_KEY_LEN: u32 = 7u32;
pub const CRYPT_TXT_LEN: u32 = 8u32;
pub const DEF_MAX_BADPW: u32 = 0u32;
pub const DEF_MAX_PWHIST: u32 = 8u32;
pub const DEF_MIN_PWLEN: u32 = 6u32;
pub const DEF_PWUNIQUENESS: u32 = 5u32;
pub const DEVLEN: u32 = 80u32;
pub const DFS_CONNECTION_FAILURE: i32 = 1073756226i32;
pub const DFS_ERROR_ACTIVEDIRECTORY_OFFLINE: i32 = -1073727301i32;
pub const DFS_ERROR_CLUSTERINFO_FAILED: i32 = -1073727307i32;
pub const DFS_ERROR_COMPUTERINFO_FAILED: i32 = -1073727308i32;
pub const DFS_ERROR_CREATEEVENT_FAILED: i32 = -1073727309i32;
pub const DFS_ERROR_CREATE_REPARSEPOINT_FAILURE: i32 = -1073727321i32;
pub const DFS_ERROR_CREATE_REPARSEPOINT_SUCCESS: i32 = 1073756370i32;
pub const DFS_ERROR_CROSS_FOREST_TRUST_INFO_FAILED: i32 = -1073727274i32;
pub const DFS_ERROR_DCINFO_FAILED: i32 = -1073727306i32;
pub const DFS_ERROR_DSCONNECT_FAILED: i32 = -2147469122i32;
pub const DFS_ERROR_DUPLICATE_LINK: i32 = -1073727277i32;
pub const DFS_ERROR_HANDLENAMESPACE_FAILED: i32 = -1073727304i32;
pub const DFS_ERROR_LINKS_OVERLAP: i32 = -1073727280i32;
pub const DFS_ERROR_LINK_OVERLAP: i32 = -1073727279i32;
pub const DFS_ERROR_MUTLIPLE_ROOTS_NOT_SUPPORTED: i32 = -1073727289i32;
pub const DFS_ERROR_NO_DFS_DATA: i32 = -1073727294i32;
pub const DFS_ERROR_ON_ROOT: i32 = -2147469114i32;
pub const DFS_ERROR_OVERLAPPING_DIRECTORIES: i32 = -1073727319i32;
pub const DFS_ERROR_PREFIXTABLE_FAILED: i32 = -1073727305i32;
pub const DFS_ERROR_REFLECTIONENGINE_FAILED: i32 = -1073727302i32;
pub const DFS_ERROR_REGISTERSTORE_FAILED: i32 = -1073727303i32;
pub const DFS_ERROR_REMOVE_LINK_FAILED: i32 = -1073727284i32;
pub const DFS_ERROR_RESYNCHRONIZE_FAILED: i32 = -1073727285i32;
pub const DFS_ERROR_ROOTSYNCINIT_FAILED: i32 = -1073727310i32;
pub const DFS_ERROR_SECURITYINIT_FAILED: i32 = -1073727313i32;
pub const DFS_ERROR_SITECACHEINIT_FAILED: i32 = -1073727311i32;
pub const DFS_ERROR_SITESUPPOR_FAILED: i32 = -1073727300i32;
pub const DFS_ERROR_TARGET_LIST_INCORRECT: i32 = -1073727281i32;
pub const DFS_ERROR_THREADINIT_FAILED: i32 = -1073727312i32;
pub const DFS_ERROR_TOO_MANY_ERRORS: i32 = -1073727315i32;
pub const DFS_ERROR_TRUSTED_DOMAIN_INFO_FAILED: i32 = -1073727276i32;
pub const DFS_ERROR_UNSUPPORTED_FILESYSTEM: i32 = -1073727320i32;
pub const DFS_ERROR_WINSOCKINIT_FAILED: i32 = -1073727314i32;
pub const DFS_INFO_ACTIVEDIRECTORY_ONLINE: i32 = 1073756332i32;
pub const DFS_INFO_CROSS_FOREST_TRUST_INFO_SUCCESS: i32 = 1073756375i32;
pub const DFS_INFO_DOMAIN_REFERRAL_MIN_OVERFLOW: i32 = 1073756361i32;
pub const DFS_INFO_DS_RECONNECTED: i32 = 1073756353i32;
pub const DFS_INFO_FINISH_BUILDING_NAMESPACE: i32 = 1073756357i32;
pub const DFS_INFO_FINISH_INIT: i32 = 1073756355i32;
pub const DFS_INFO_RECONNECT_DATA: i32 = 1073756356i32;
pub const DFS_INFO_TRUSTED_DOMAIN_INFO_SUCCESS: i32 = 1073756373i32;
pub const DFS_INIT_SUCCESS: i32 = 1073756376i32;
pub const DFS_MAX_DNR_ATTEMPTS: i32 = 1073756229i32;
pub const DFS_OPEN_FAILURE: i32 = 1073756231i32;
pub const DFS_REFERRAL_FAILURE: i32 = 1073756227i32;
pub const DFS_REFERRAL_REQUEST: i32 = 1073756142i32;
pub const DFS_REFERRAL_SUCCESS: i32 = 1073756228i32;
pub const DFS_ROOT_SHARE_ACQUIRE_FAILED: i32 = -2147469095i32;
pub const DFS_ROOT_SHARE_ACQUIRE_SUCCESS: i32 = 1073756378i32;
pub const DFS_SPECIAL_REFERRAL_FAILURE: i32 = 1073756230i32;
pub const DFS_WARN_DOMAIN_REFERRAL_OVERFLOW: i32 = -2147469112i32;
pub const DFS_WARN_INCOMPLETE_MOVE: i32 = -2147469110i32;
pub const DFS_WARN_METADATA_LINK_INFO_INVALID: i32 = -2147469106i32;
pub const DFS_WARN_METADATA_LINK_TYPE_INCORRECT: i32 = -2147469107i32;
pub const DNLEN: u32 = 15u32;
pub const DPP_ADVANCED: DEFAULT_PAGES = DEFAULT_PAGES(1i32);
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(1i32);
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(0i32);
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = DSREG_JOIN_TYPE(2i32);
pub const EBP_ABOVE: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(1i32);
pub const EBP_BELOW: ENUM_BINDING_PATHS_FLAGS = ENUM_BINDING_PATHS_FLAGS(2i32);
pub const ENCRYPTED_PWLEN: u32 = 16u32;
pub const ERRLOG2_BASE: u32 = 5700u32;
pub const ERRLOG_BASE: u32 = 3100u32;
pub const EVENT_BAD_ACCOUNT_NAME: i32 = -1073734816i32;
pub const EVENT_BAD_SERVICE_STATE: i32 = -1073734808i32;
pub const EVENT_BOOT_SYSTEM_DRIVERS_FAILED: i32 = -1073734798i32;
pub const EVENT_BOWSER_CANT_READ_REGISTRY: i32 = 1073749853i32;
pub const EVENT_BOWSER_ELECTION_RECEIVED: i32 = 8012i32;
pub const EVENT_BOWSER_ELECTION_SENT_FIND_MASTER_FAILED: i32 = 1073749838i32;
pub const EVENT_BOWSER_ELECTION_SENT_GETBLIST_FAILED: i32 = 1073749837i32;
pub const EVENT_BOWSER_GETBROWSERLIST_THRESHOLD_EXCEEDED: i32 = 1073749855i32;
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM: i32 = -2147475642i32;
pub const EVENT_BOWSER_ILLEGAL_DATAGRAM_THRESHOLD: i32 = -1073733808i32;
pub const EVENT_BOWSER_MAILSLOT_DATAGRAM_THRESHOLD_EXCEEDED: i32 = 1073749854i32;
pub const EVENT_BOWSER_NAME_CONVERSION_FAILED: i32 = -1073733814i32;
pub const EVENT_BOWSER_NON_MASTER_MASTER_ANNOUNCE: i32 = -2147475643i32;
pub const EVENT_BOWSER_NON_PDC_WON_ELECTION: i32 = 1073749852i32;
pub const EVENT_BOWSER_OLD_BACKUP_FOUND: i32 = 1073749848i32;
pub const EVENT_BOWSER_OTHER_MASTER_ON_NET: i32 = -1073733821i32;
pub const EVENT_BOWSER_PDC_LOST_ELECTION: i32 = 1073749851i32;
pub const EVENT_BOWSER_PROMOTED_WHILE_ALREADY_MASTER: i32 = -2147475644i32;
pub const EVENT_BRIDGE_ADAPTER_BIND_FAILED: i32 = -1073727120i32;
pub const EVENT_BRIDGE_ADAPTER_FILTER_FAILED: i32 = -1073727122i32;
pub const EVENT_BRIDGE_ADAPTER_LINK_SPEED_QUERY_FAILED: i32 = -1073727124i32;
pub const EVENT_BRIDGE_ADAPTER_MAC_ADDR_QUERY_FAILED: i32 = -1073727123i32;
pub const EVENT_BRIDGE_ADAPTER_NAME_QUERY_FAILED: i32 = -1073727121i32;
pub const EVENT_BRIDGE_BUFFER_POOL_CREATION_FAILED: i32 = -1073727214i32;
pub const EVENT_BRIDGE_DEVICE_CREATION_FAILED: i32 = -1073727221i32;
pub const EVENT_BRIDGE_ETHERNET_NOT_OFFERED: i32 = -1073727218i32;
pub const EVENT_BRIDGE_INIT_MALLOC_FAILED: i32 = -1073727213i32;
pub const EVENT_BRIDGE_MINIPORT_INIT_FAILED: i32 = -1073727219i32;
pub const EVENT_BRIDGE_MINIPORT_REGISTER_FAILED: i32 = -1073727222i32;
pub const EVENT_BRIDGE_MINIPROT_DEVNAME_MISSING: i32 = -1073727223i32;
pub const EVENT_BRIDGE_NO_BRIDGE_MAC_ADDR: i32 = -1073727220i32;
pub const EVENT_BRIDGE_PACKET_POOL_CREATION_FAILED: i32 = -1073727215i32;
pub const EVENT_BRIDGE_PROTOCOL_REGISTER_FAILED: i32 = -1073727224i32;
pub const EVENT_BRIDGE_THREAD_CREATION_FAILED: i32 = -1073727217i32;
pub const EVENT_BRIDGE_THREAD_REF_FAILED: i32 = -1073727216i32;
pub const EVENT_BROWSER_BACKUP_STOPPED: i32 = -1073733792i32;
pub const EVENT_BROWSER_DEPENDANT_SERVICE_FAILED: i32 = -1073733807i32;
pub const EVENT_BROWSER_DOMAIN_LIST_FAILED: i32 = -2147475626i32;
pub const EVENT_BROWSER_DOMAIN_LIST_RETRIEVED: i32 = 8026i32;
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STARTED: i32 = 1073749839i32;
pub const EVENT_BROWSER_ELECTION_SENT_LANMAN_NT_STOPPED: i32 = 1073749857i32;
pub const EVENT_BROWSER_ELECTION_SENT_ROLE_CHANGED: i32 = 1073749859i32;
pub const EVENT_BROWSER_GETBLIST_RECEIVED_NOT_MASTER: i32 = -1073733790i32;
pub const EVENT_BROWSER_ILLEGAL_CONFIG: i32 = -2147475625i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED: i32 = -1073733815i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_NO_MASTER: i32 = -1073733804i32;
pub const EVENT_BROWSER_MASTER_PROMOTION_FAILED_STOPPING: i32 = -1073733805i32;
pub const EVENT_BROWSER_NOT_STARTED_IPX_CONFIG_MISMATCH: i32 = -1073733788i32;
pub const EVENT_BROWSER_OTHERDOMAIN_ADD_FAILED: i32 = -1073733813i32;
pub const EVENT_BROWSER_ROLE_CHANGE_FAILED: i32 = -1073733816i32;
pub const EVENT_BROWSER_SERVER_LIST_FAILED: i32 = -2147475627i32;
pub const EVENT_BROWSER_SERVER_LIST_RETRIEVED: i32 = 8025i32;
pub const EVENT_BROWSER_STATUS_BITS_UPDATE_FAILED: i32 = -1073733817i32;
pub const EVENT_CALL_TO_FUNCTION_FAILED: i32 = -1073734819i32;
pub const EVENT_CALL_TO_FUNCTION_FAILED_II: i32 = -1073734818i32;
pub const EVENT_CIRCULAR_DEPENDENCY_AUTO: i32 = -1073734806i32;
pub const EVENT_CIRCULAR_DEPENDENCY_DEMAND: i32 = -1073734807i32;
pub const EVENT_COMMAND_NOT_INTERACTIVE: i32 = -1073733924i32;
pub const EVENT_COMMAND_START_FAILED: i32 = -1073733923i32;
pub const EVENT_CONNECTION_TIMEOUT: i32 = -1073734815i32;
pub const EVENT_ComputerNameChange: i32 = -2147477637i32;
pub const EVENT_DAV_REDIR_DELAYED_WRITE_FAILED: i32 = -2147468848i32;
pub const EVENT_DCOM_ASSERTION_FAILURE: i32 = -1073731812i32;
pub const EVENT_DCOM_COMPLUS_DISABLED: i32 = -1073731810i32;
pub const EVENT_DCOM_INVALID_ENDPOINT_DATA: i32 = -1073731811i32;
pub const EVENT_DEPEND_ON_LATER_GROUP: i32 = -1073734804i32;
pub const EVENT_DEPEND_ON_LATER_SERVICE: i32 = -1073734805i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472466i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472454i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER: i32 = -2147472463i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472451i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472465i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472453i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472464i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472452i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472467i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472455i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472468i32;
pub const EVENT_DNSAPI_DEREGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472456i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_NOTSUPP: i32 = -2147472460i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_OTHER: i32 = -2147472457i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_REFUSED: i32 = -2147472459i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SECURITY: i32 = -2147472458i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_SERVERFAIL: i32 = -2147472461i32;
pub const EVENT_DNSAPI_PTR_DEREGISTRATION_FAILED_TIMEOUT: i32 = -2147472462i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472490i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_OTHER: i32 = -2147472487i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_REFUSED: i32 = -2147472489i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SECURITY: i32 = -2147472488i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472491i32;
pub const EVENT_DNSAPI_PTR_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472492i32;
pub const EVENT_DNSAPI_REGISTERED_ADAPTER: i32 = 1073753024i32;
pub const EVENT_DNSAPI_REGISTERED_ADAPTER_PRIMARY_DN: i32 = 1073753026i32;
pub const EVENT_DNSAPI_REGISTERED_PTR: i32 = 1073753025i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP: i32 = -2147472496i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_NOTSUPP_PRIMARY_DN: i32 = -2147472484i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER: i32 = -2147472493i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_OTHER_PRIMARY_DN: i32 = -2147472481i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED: i32 = -2147472495i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_REFUSED_PRIMARY_DN: i32 = -2147472483i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY: i32 = -2147472494i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SECURITY_PRIMARY_DN: i32 = -2147472482i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL: i32 = -2147472497i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_SERVERFAIL_PRIMARY_DN: i32 = -2147472485i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT: i32 = -2147472498i32;
pub const EVENT_DNSAPI_REGISTRATION_FAILED_TIMEOUT_PRIMARY_DN: i32 = -2147472486i32;
pub const EVENT_DNSDomainNameChange: i32 = -2147477636i32;
pub const EVENT_DNS_CACHE_NETWORK_PERF_WARNING: i32 = -2147472598i32;
pub const EVENT_DNS_CACHE_START_FAILURE_LOW_MEMORY: i32 = -1073730817i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_CONTROL: i32 = -1073730822i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DLL: i32 = -1073730824i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_DONE_EVENT: i32 = -1073730821i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_ENTRY: i32 = -1073730823i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_RPC: i32 = -1073730820i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_SHUTDOWN_NOTIFY: i32 = -1073730819i32;
pub const EVENT_DNS_CACHE_START_FAILURE_NO_UPDATE: i32 = -1073730818i32;
pub const EVENT_DNS_CACHE_UNABLE_TO_REACH_SERVER_WARNING: i32 = -2147472597i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725118i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_KEYNAME_TOO_LONG: i32 = -1073725120i32;
pub const EVENT_EQOS_ERROR_MACHINE_POLICY_REFERESH: i32 = -1073725124i32;
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_ROOT_KEY: i32 = -1073725122i32;
pub const EVENT_EQOS_ERROR_OPENING_MACHINE_POLICY_SUBKEY: i32 = -1073725116i32;
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_ROOT_KEY: i32 = -1073725121i32;
pub const EVENT_EQOS_ERROR_OPENING_USER_POLICY_SUBKEY: i32 = -1073725115i32;
pub const EVENT_EQOS_ERROR_PROCESSING_MACHINE_POLICY_FIELD: i32 = -1073725114i32;
pub const EVENT_EQOS_ERROR_PROCESSING_USER_POLICY_FIELD: i32 = -1073725113i32;
pub const EVENT_EQOS_ERROR_SETTING_APP_MARKING: i32 = -1073725111i32;
pub const EVENT_EQOS_ERROR_SETTING_TCP_AUTOTUNING: i32 = -1073725112i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_SIZE_ZERO: i32 = -1073725117i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_KEYNAME_TOO_LONG: i32 = -1073725119i32;
pub const EVENT_EQOS_ERROR_USER_POLICY_REFERESH: i32 = -1073725123i32;
pub const EVENT_EQOS_INFO_APP_MARKING_ALLOWED: i32 = 1073758335i32;
pub const EVENT_EQOS_INFO_APP_MARKING_IGNORED: i32 = 1073758334i32;
pub const EVENT_EQOS_INFO_APP_MARKING_NOT_CONFIGURED: i32 = 1073758333i32;
pub const EVENT_EQOS_INFO_LOCAL_SETTING_DONT_USE_NLA: i32 = 1073758336i32;
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_NO_CHANGE: i32 = 1073758324i32;
pub const EVENT_EQOS_INFO_MACHINE_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758325i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_HIGHLY_RESTRICTED: i32 = 1073758330i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NORMAL: i32 = 1073758332i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_NOT_CONFIGURED: i32 = 1073758328i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_OFF: i32 = 1073758329i32;
pub const EVENT_EQOS_INFO_TCP_AUTOTUNING_RESTRICTED: i32 = 1073758331i32;
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_NO_CHANGE: i32 = 1073758326i32;
pub const EVENT_EQOS_INFO_USER_POLICY_REFRESH_WITH_CHANGE: i32 = 1073758327i32;
pub const EVENT_EQOS_URL_QOS_APPLICATION_CONFLICT: i32 = 1073758337i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_CONFLICT: i32 = -2147467040i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467038i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467044i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_QUOTA_EXCEEDED: i32 = -2147467042i32;
pub const EVENT_EQOS_WARNING_MACHINE_POLICY_VERSION: i32 = -2147467046i32;
pub const EVENT_EQOS_WARNING_TEST_1: i32 = -2147467048i32;
pub const EVENT_EQOS_WARNING_TEST_2: i32 = -2147467047i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_CONFLICT: i32 = -2147467039i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_NO_FULLPATH_APPNAME: i32 = -2147467037i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_PROFILE_NOT_SPECIFIED: i32 = -2147467043i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_QUOTA_EXCEEDED: i32 = -2147467041i32;
pub const EVENT_EQOS_WARNING_USER_POLICY_VERSION: i32 = -2147467045i32;
pub const EVENT_EventLogProductInfo: i32 = -2147477639i32;
pub const EVENT_EventlogAbnormalShutdown: i32 = -2147477640i32;
pub const EVENT_EventlogStarted: i32 = -2147477643i32;
pub const EVENT_EventlogStopped: i32 = -2147477642i32;
pub const EVENT_EventlogUptime: i32 = -2147477635i32;
pub const EVENT_FIRST_LOGON_FAILED: i32 = -1073734811i32;
pub const EVENT_FIRST_LOGON_FAILED_II: i32 = -1073734786i32;
pub const EVENT_FRS_ACCESS_CHECKS_DISABLED: i32 = -2147470131i32;
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_UNKNOWN: i32 = -1073728305i32;
pub const EVENT_FRS_ACCESS_CHECKS_FAILED_USER: i32 = -2147470130i32;
pub const EVENT_FRS_ASSERT: i32 = -1073728318i32;
pub const EVENT_FRS_BAD_REG_DATA: i32 = -2147470101i32;
pub const EVENT_FRS_CANNOT_COMMUNICATE: i32 = -1073728314i32;
pub const EVENT_FRS_CANNOT_CREATE_UUID: i32 = -1073728300i32;
pub const EVENT_FRS_CANNOT_START_BACKUP_RESTORE_IN_PROGRESS: i32 = -1073728303i32;
pub const EVENT_FRS_CANT_OPEN_PREINSTALL: i32 = -1073728273i32;
pub const EVENT_FRS_CANT_OPEN_STAGE: i32 = -1073728274i32;
pub const EVENT_FRS_DATABASE_SPACE: i32 = -1073728313i32;
pub const EVENT_FRS_DISK_WRITE_CACHE_ENABLED: i32 = -2147470136i32;
pub const EVENT_FRS_DS_POLL_ERROR_SUMMARY: i32 = -2147470086i32;
pub const EVENT_FRS_DUPLICATE_IN_CXTION: i32 = -1073728266i32;
pub const EVENT_FRS_DUPLICATE_IN_CXTION_SYSVOL: i32 = -1073728267i32;
pub const EVENT_FRS_ERROR: i32 = -1073728324i32;
pub const EVENT_FRS_ERROR_REPLICA_SET_DELETED: i32 = -2147470088i32;
pub const EVENT_FRS_HUGE_FILE: i32 = -2147470125i32;
pub const EVENT_FRS_IN_ERROR_STATE: i32 = -1073728269i32;
pub const EVENT_FRS_JET_1414: i32 = -1073728311i32;
pub const EVENT_FRS_JOIN_FAIL_TIME_SKEW: i32 = -1073728276i32;
pub const EVENT_FRS_LONG_JOIN: i32 = -2147470140i32;
pub const EVENT_FRS_LONG_JOIN_DONE: i32 = -2147470139i32;
pub const EVENT_FRS_MOVED_PREEXISTING: i32 = -2147470128i32;
pub const EVENT_FRS_NO_DNS_ATTRIBUTE: i32 = -2147470123i32;
pub const EVENT_FRS_NO_SID: i32 = -1073728298i32;
pub const EVENT_FRS_OVERLAPS_LOGGING: i32 = -1073728283i32;
pub const EVENT_FRS_OVERLAPS_OTHER_STAGE: i32 = -1073728279i32;
pub const EVENT_FRS_OVERLAPS_ROOT: i32 = -1073728280i32;
pub const EVENT_FRS_OVERLAPS_STAGE: i32 = -1073728281i32;
pub const EVENT_FRS_OVERLAPS_WORKING: i32 = -1073728282i32;
pub const EVENT_FRS_PREPARE_ROOT_FAILED: i32 = -1073728278i32;
pub const EVENT_FRS_REPLICA_IN_JRNL_WRAP_ERROR: i32 = -1073728263i32;
pub const EVENT_FRS_REPLICA_NO_ROOT_CHANGE: i32 = -1073728268i32;
pub const EVENT_FRS_REPLICA_SET_CREATE_FAIL: i32 = -1073728272i32;
pub const EVENT_FRS_REPLICA_SET_CREATE_OK: i32 = 1073755377i32;
pub const EVENT_FRS_REPLICA_SET_CXTIONS: i32 = 1073755378i32;
pub const EVENT_FRS_RMTCO_TIME_SKEW: i32 = -1073728275i32;
pub const EVENT_FRS_ROOT_HAS_MOVED: i32 = -1073728265i32;
pub const EVENT_FRS_ROOT_NOT_VALID: i32 = -1073728285i32;
pub const EVENT_FRS_STAGE_NOT_VALID: i32 = -1073728284i32;
pub const EVENT_FRS_STAGING_AREA_FULL: i32 = -2147470126i32;
pub const EVENT_FRS_STARTING: i32 = 1073755325i32;
pub const EVENT_FRS_STOPPED: i32 = 1073755327i32;
pub const EVENT_FRS_STOPPED_ASSERT: i32 = -1073728319i32;
pub const EVENT_FRS_STOPPED_FORCE: i32 = -1073728320i32;
pub const EVENT_FRS_STOPPING: i32 = 1073755326i32;
pub const EVENT_FRS_SYSVOL_NOT_READY: i32 = -2147470134i32;
pub const EVENT_FRS_SYSVOL_NOT_READY_PRIMARY: i32 = -2147470133i32;
pub const EVENT_FRS_SYSVOL_READY: i32 = 1073755340i32;
pub const EVENT_FRS_VOLUME_NOT_SUPPORTED: i32 = -1073728317i32;
pub const EVENT_INVALID_DRIVER_DEPENDENCY: i32 = -1073734809i32;
pub const EVENT_IPX_CREATE_DEVICE: i32 = -1073732318i32;
pub const EVENT_IPX_ILLEGAL_CONFIG: i32 = -2147474145i32;
pub const EVENT_IPX_INTERNAL_NET_INVALID: i32 = -1073732320i32;
pub const EVENT_IPX_NEW_DEFAULT_TYPE: i32 = 1073751325i32;
pub const EVENT_IPX_NO_ADAPTERS: i32 = -1073732317i32;
pub const EVENT_IPX_NO_FRAME_TYPES: i32 = -1073732319i32;
pub const EVENT_IPX_SAP_ANNOUNCE: i32 = -2147474146i32;
pub const EVENT_NBT_BAD_BACKUP_WINS_ADDR: i32 = -2147479344i32;
pub const EVENT_NBT_BAD_PRIMARY_WINS_ADDR: i32 = -2147479343i32;
pub const EVENT_NBT_CREATE_ADDRESS: i32 = -1073737517i32;
pub const EVENT_NBT_CREATE_CONNECTION: i32 = -1073737516i32;
pub const EVENT_NBT_CREATE_DEVICE: i32 = -1073737513i32;
pub const EVENT_NBT_CREATE_DRIVER: i32 = -1073737524i32;
pub const EVENT_NBT_DUPLICATE_NAME: i32 = -1073737505i32;
pub const EVENT_NBT_DUPLICATE_NAME_ERROR: i32 = -1073737503i32;
pub const EVENT_NBT_NAME_RELEASE: i32 = -1073737504i32;
pub const EVENT_NBT_NAME_SERVER_ADDRS: i32 = -1073737518i32;
pub const EVENT_NBT_NON_OS_INIT: i32 = -1073737515i32;
pub const EVENT_NBT_NO_BACKUP_WINS: i32 = -2147479346i32;
pub const EVENT_NBT_NO_DEVICES: i32 = -2147479336i32;
pub const EVENT_NBT_NO_RESOURCES: i32 = -1073737502i32;
pub const EVENT_NBT_NO_WINS: i32 = -2147479345i32;
pub const EVENT_NBT_OPEN_REG_LINKAGE: i32 = -1073737511i32;
pub const EVENT_NBT_OPEN_REG_NAMESERVER: i32 = -2147479332i32;
pub const EVENT_NBT_OPEN_REG_PARAMS: i32 = -1073737523i32;
pub const EVENT_NBT_READ_BIND: i32 = -1073737510i32;
pub const EVENT_NBT_READ_EXPORT: i32 = -1073737509i32;
pub const EVENT_NBT_TIMERS: i32 = -1073737514i32;
pub const EVENT_NDIS_ADAPTER_CHECK_ERROR: i32 = -1073736793i32;
pub const EVENT_NDIS_ADAPTER_DISABLED: i32 = -2147478634i32;
pub const EVENT_NDIS_ADAPTER_NOT_FOUND: i32 = -1073736821i32;
pub const EVENT_NDIS_BAD_IO_BASE_ADDRESS: i32 = -1073736812i32;
pub const EVENT_NDIS_BAD_VERSION: i32 = -1073736818i32;
pub const EVENT_NDIS_CABLE_DISCONNECTED_ERROR: i32 = -2147478615i32;
pub const EVENT_NDIS_DMA_CONFLICT: i32 = -2147478629i32;
pub const EVENT_NDIS_DRIVER_FAILURE: i32 = -1073736819i32;
pub const EVENT_NDIS_HARDWARE_FAILURE: i32 = -1073736822i32;
pub const EVENT_NDIS_INTERRUPT_CONFLICT: i32 = -2147478630i32;
pub const EVENT_NDIS_INTERRUPT_CONNECT: i32 = -1073736820i32;
pub const EVENT_NDIS_INVALID_DOWNLOAD_FILE_ERROR: i32 = -1073736804i32;
pub const EVENT_NDIS_INVALID_VALUE_FROM_ADAPTER: i32 = -1073736814i32;
pub const EVENT_NDIS_IO_PORT_CONFLICT: i32 = -2147478633i32;
pub const EVENT_NDIS_LOBE_FAILUE_ERROR: i32 = -2147478621i32;
pub const EVENT_NDIS_MAXFRAMESIZE_ERROR: i32 = -2147478625i32;
pub const EVENT_NDIS_MAXINTERNALBUFS_ERROR: i32 = -2147478624i32;
pub const EVENT_NDIS_MAXMULTICAST_ERROR: i32 = -2147478623i32;
pub const EVENT_NDIS_MAXRECEIVES_ERROR: i32 = -2147478627i32;
pub const EVENT_NDIS_MAXTRANSMITS_ERROR: i32 = -2147478626i32;
pub const EVENT_NDIS_MEMORY_CONFLICT: i32 = -2147478631i32;
pub const EVENT_NDIS_MISSING_CONFIGURATION_PARAMETER: i32 = -1073736813i32;
pub const EVENT_NDIS_NETWORK_ADDRESS: i32 = -1073736816i32;
pub const EVENT_NDIS_OUT_OF_RESOURCE: i32 = -1073736823i32;
pub const EVENT_NDIS_PORT_OR_DMA_CONFLICT: i32 = -2147478632i32;
pub const EVENT_NDIS_PRODUCTID_ERROR: i32 = -2147478622i32;
pub const EVENT_NDIS_RECEIVE_SPACE_SMALL: i32 = 1073746837i32;
pub const EVENT_NDIS_REMOVE_RECEIVED_ERROR: i32 = -2147478619i32;
pub const EVENT_NDIS_RESET_FAILURE_CORRECTION: i32 = -2147478614i32;
pub const EVENT_NDIS_RESET_FAILURE_ERROR: i32 = -2147478616i32;
pub const EVENT_NDIS_RESOURCE_CONFLICT: i32 = -1073736824i32;
pub const EVENT_NDIS_SIGNAL_LOSS_ERROR: i32 = -2147478620i32;
pub const EVENT_NDIS_TIMEOUT: i32 = -2147478641i32;
pub const EVENT_NDIS_TOKEN_RING_CORRECTION: i32 = 1073746854i32;
pub const EVENT_NDIS_UNSUPPORTED_CONFIGURATION: i32 = -1073736815i32;
pub const EVENT_PS_ADMISSIONCONTROL_OVERFLOW: i32 = -2147469537i32;
pub const EVENT_PS_BAD_BESTEFFORT_LIMIT: i32 = -2147469548i32;
pub const EVENT_PS_BINDING_FAILED: i32 = -1073727720i32;
pub const EVENT_PS_GPC_REGISTER_FAILED: i32 = -1073727824i32;
pub const EVENT_PS_INIT_DEVICE_FAILED: i32 = -1073727717i32;
pub const EVENT_PS_MISSING_ADAPTER_REGISTRY_DATA: i32 = -1073727719i32;
pub const EVENT_PS_NETWORK_ADDRESS_FAIL: i32 = -1073727712i32;
pub const EVENT_PS_NO_RESOURCES_FOR_INIT: i32 = -1073727823i32;
pub const EVENT_PS_QUERY_OID_GEN_LINK_SPEED: i32 = -1073727721i32;
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_FRAME_SIZE: i32 = -1073727723i32;
pub const EVENT_PS_QUERY_OID_GEN_MAXIMUM_TOTAL_SIZE: i32 = -1073727722i32;
pub const EVENT_PS_REGISTER_ADDRESS_FAMILY_FAILED: i32 = -1073727718i32;
pub const EVENT_PS_REGISTER_MINIPORT_FAILED: i32 = -1073727821i32;
pub const EVENT_PS_REGISTER_PROTOCOL_FAILED: i32 = -1073727822i32;
pub const EVENT_PS_RESOURCE_POOL: i32 = -1073727714i32;
pub const EVENT_PS_WAN_LIMITED_BESTEFFORT: i32 = -2147469539i32;
pub const EVENT_PS_WMI_INSTANCE_NAME_FAILED: i32 = -1073727716i32;
pub const EVENT_RDR_AT_THREAD_MAX: i32 = -2147480622i32;
pub const EVENT_RDR_CANT_BIND_TRANSPORT: i32 = -2147480616i32;
pub const EVENT_RDR_CANT_BUILD_SMB_HEADER: i32 = -2147480613i32;
pub const EVENT_RDR_CANT_CREATE_DEVICE: i32 = -2147480646i32;
pub const EVENT_RDR_CANT_CREATE_THREAD: i32 = -2147480645i32;
pub const EVENT_RDR_CANT_GET_SECURITY_CONTEXT: i32 = -2147480614i32;
pub const EVENT_RDR_CANT_READ_REGISTRY: i32 = -2147480621i32;
pub const EVENT_RDR_CANT_REGISTER_ADDRESS: i32 = -2147480615i32;
pub const EVENT_RDR_CANT_SET_THREAD: i32 = -2147480644i32;
pub const EVENT_RDR_CLOSE_BEHIND: i32 = -2147480637i32;
pub const EVENT_RDR_CONNECTION: i32 = -2147480629i32;
pub const EVENT_RDR_CONNECTION_REFERENCE: i32 = -2147480633i32;
pub const EVENT_RDR_CONTEXTS: i32 = -2147480624i32;
pub const EVENT_RDR_DELAYED_SET_ATTRIBUTES_FAILED: i32 = -2147480618i32;
pub const EVENT_RDR_DELETEONCLOSE_FAILED: i32 = -2147480617i32;
pub const EVENT_RDR_DISPOSITION: i32 = -2147480625i32;
pub const EVENT_RDR_ENCRYPT: i32 = -2147480630i32;
pub const EVENT_RDR_FAILED_UNLOCK: i32 = -2147480639i32;
pub const EVENT_RDR_INVALID_LOCK_REPLY: i32 = -2147480641i32;
pub const EVENT_RDR_INVALID_OPLOCK: i32 = -2147480634i32;
pub const EVENT_RDR_INVALID_REPLY: i32 = -2147480643i32;
pub const EVENT_RDR_INVALID_SMB: i32 = -2147480642i32;
pub const EVENT_RDR_MAXCMDS: i32 = -2147480627i32;
pub const EVENT_RDR_OPLOCK_SMB: i32 = -2147480626i32;
pub const EVENT_RDR_PRIMARY_TRANSPORT_CONNECT_FAILED: i32 = -2147480619i32;
pub const EVENT_RDR_RESOURCE_SHORTAGE: i32 = -2147480647i32;
pub const EVENT_RDR_SECURITY_SIGNATURE_MISMATCH: i32 = -2147480612i32;
pub const EVENT_RDR_SERVER_REFERENCE: i32 = -2147480632i32;
pub const EVENT_RDR_SMB_REFERENCE: i32 = -2147480631i32;
pub const EVENT_RDR_TIMEOUT: i32 = -2147480635i32;
pub const EVENT_RDR_TIMEZONE_BIAS_TOO_LARGE: i32 = -2147480620i32;
pub const EVENT_RDR_UNEXPECTED_ERROR: i32 = -2147480636i32;
pub const EVENT_RDR_WRITE_BEHIND_FLUSH_FAILED: i32 = -2147480623i32;
pub const EVENT_READFILE_TIMEOUT: i32 = -1073734814i32;
pub const EVENT_REVERTED_TO_LASTKNOWNGOOD: i32 = -1073734817i32;
pub const EVENT_RPCSS_ACTIVATION_ERROR: i32 = -1073731817i32;
pub const EVENT_RPCSS_CREATEDEBUGGERPROCESS_FAILURE: i32 = -1073731794i32;
pub const EVENT_RPCSS_CREATEPROCESS_FAILURE: i32 = -1073731824i32;
pub const EVENT_RPCSS_DEFAULT_LAUNCH_ACCESS_DENIED: i32 = -1073731821i32;
pub const EVENT_RPCSS_LAUNCH_ACCESS_DENIED: i32 = -1073731822i32;
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR: i32 = -1073731818i32;
pub const EVENT_RPCSS_REMOTE_SIDE_ERROR_WITH_FILE: i32 = -1073731816i32;
pub const EVENT_RPCSS_REMOTE_SIDE_UNAVAILABLE: i32 = -1073731815i32;
pub const EVENT_RPCSS_RUNAS_CANT_LOGIN: i32 = -1073731820i32;
pub const EVENT_RPCSS_RUNAS_CREATEPROCESS_FAILURE: i32 = -1073731823i32;
pub const EVENT_RPCSS_SERVER_NOT_RESPONDING: i32 = -1073731813i32;
pub const EVENT_RPCSS_SERVER_START_TIMEOUT: i32 = -1073731814i32;
pub const EVENT_RPCSS_START_SERVICE_FAILURE: i32 = -1073731819i32;
pub const EVENT_RPCSS_STOP_SERVICE_FAILURE: i32 = -1073731795i32;
pub const EVENT_RUNNING_LASTKNOWNGOOD: i32 = -1073734797i32;
pub const EVENT_SCOPE_LABEL_TOO_LONG: i32 = -2147479331i32;
pub const EVENT_SCOPE_TOO_LONG: i32 = -2147479330i32;
pub const EVENT_SECOND_LOGON_FAILED: i32 = -1073734810i32;
pub const EVENT_SERVICE_CONFIG_BACKOUT_FAILED: i32 = -1073734787i32;
pub const EVENT_SERVICE_CONTROL_SUCCESS: i32 = 1073748859i32;
pub const EVENT_SERVICE_CRASH: i32 = -1073734793i32;
pub const EVENT_SERVICE_CRASH_NO_ACTION: i32 = -1073734790i32;
pub const EVENT_SERVICE_DIFFERENT_PID_CONNECTED: i32 = -2147476609i32;
pub const EVENT_SERVICE_EXIT_FAILED: i32 = -1073734801i32;
pub const EVENT_SERVICE_EXIT_FAILED_SPECIFIC: i32 = -1073734800i32;
pub const EVENT_SERVICE_LOGON_TYPE_NOT_GRANTED: i32 = -1073734783i32;
pub const EVENT_SERVICE_NOT_INTERACTIVE: i32 = -1073734794i32;
pub const EVENT_SERVICE_RECOVERY_FAILED: i32 = -1073734792i32;
pub const EVENT_SERVICE_SCESRV_FAILED: i32 = -1073734791i32;
pub const EVENT_SERVICE_SHUTDOWN_FAILED: i32 = -1073734781i32;
pub const EVENT_SERVICE_START_AT_BOOT_FAILED: i32 = -1073734799i32;
pub const EVENT_SERVICE_START_FAILED: i32 = -1073734824i32;
pub const EVENT_SERVICE_START_FAILED_GROUP: i32 = -1073734822i32;
pub const EVENT_SERVICE_START_FAILED_II: i32 = -1073734823i32;
pub const EVENT_SERVICE_START_FAILED_NONE: i32 = -1073734821i32;
pub const EVENT_SERVICE_START_HUNG: i32 = -1073734802i32;
pub const EVENT_SERVICE_START_TYPE_CHANGED: i32 = 1073748864i32;
pub const EVENT_SERVICE_STATUS_SUCCESS: i32 = 1073748860i32;
pub const EVENT_SERVICE_STOP_SUCCESS_WITH_REASON: i32 = 1073748866i32;
pub const EVENT_SEVERE_SERVICE_FAILED: i32 = -1073734803i32;
pub const EVENT_SRV_CANT_BIND_DUP_NAME: i32 = -1073739319i32;
pub const EVENT_SRV_CANT_BIND_TO_TRANSPORT: i32 = -2147481144i32;
pub const EVENT_SRV_CANT_CHANGE_DOMAIN_NAME: i32 = -2147481136i32;
pub const EVENT_SRV_CANT_CREATE_DEVICE: i32 = -1073739822i32;
pub const EVENT_SRV_CANT_CREATE_PROCESS: i32 = -1073739821i32;
pub const EVENT_SRV_CANT_CREATE_THREAD: i32 = -1073739820i32;
pub const EVENT_SRV_CANT_GROW_TABLE: i32 = -2147481639i32;
pub const EVENT_SRV_CANT_LOAD_DRIVER: i32 = -2147481140i32;
pub const EVENT_SRV_CANT_MAP_ERROR: i32 = -2147481138i32;
pub const EVENT_SRV_CANT_OPEN_NPFS: i32 = -1073739817i32;
pub const EVENT_SRV_CANT_RECREATE_SHARE: i32 = -2147481137i32;
pub const EVENT_SRV_CANT_START_SCAVENGER: i32 = -1073739814i32;
pub const EVENT_SRV_CANT_UNLOAD_DRIVER: i32 = -2147481139i32;
pub const EVENT_SRV_DISK_FULL: i32 = -2147481635i32;
pub const EVENT_SRV_DOS_ATTACK_DETECTED: i32 = -2147481623i32;
pub const EVENT_SRV_INVALID_REGISTRY_VALUE: i32 = -2147481142i32;
pub const EVENT_SRV_INVALID_REQUEST: i32 = -1073739818i32;
pub const EVENT_SRV_INVALID_SD: i32 = -2147481141i32;
pub const EVENT_SRV_IRP_STACK_SIZE: i32 = -1073739813i32;
pub const EVENT_SRV_KEY_NOT_CREATED: i32 = -1073739322i32;
pub const EVENT_SRV_KEY_NOT_FOUND: i32 = -1073739323i32;
pub const EVENT_SRV_NETWORK_ERROR: i32 = -2147481636i32;
pub const EVENT_SRV_NONPAGED_POOL_LIMIT: i32 = -1073739807i32;
pub const EVENT_SRV_NO_BLOCKING_IO: i32 = -2147481624i32;
pub const EVENT_SRV_NO_FREE_CONNECTIONS: i32 = -2147481626i32;
pub const EVENT_SRV_NO_FREE_RAW_WORK_ITEM: i32 = -2147481625i32;
pub const EVENT_SRV_NO_NONPAGED_POOL: i32 = -1073739805i32;
pub const EVENT_SRV_NO_PAGED_POOL: i32 = -1073739804i32;
pub const EVENT_SRV_NO_TRANSPORTS_BOUND: i32 = -1073739321i32;
pub const EVENT_SRV_NO_VIRTUAL_MEMORY: i32 = -1073739808i32;
pub const EVENT_SRV_NO_WORK_ITEM: i32 = -2147481627i32;
pub const EVENT_SRV_OUT_OF_WORK_ITEM_DOS: i32 = -2147481621i32;
pub const EVENT_SRV_PAGED_POOL_LIMIT: i32 = -1073739806i32;
pub const EVENT_SRV_RESOURCE_SHORTAGE: i32 = -1073739823i32;
pub const EVENT_SRV_SERVICE_FAILED: i32 = -1073739824i32;
pub const EVENT_SRV_TOO_MANY_DOS: i32 = -2147481622i32;
pub const EVENT_SRV_TXF_INIT_FAILED: i32 = -2147481135i32;
pub const EVENT_SRV_UNEXPECTED_DISC: i32 = -1073739819i32;
pub const EVENT_STREAMS_ALLOCB_FAILURE: i32 = -2147479647i32;
pub const EVENT_STREAMS_ALLOCB_FAILURE_CNT: i32 = -2147479646i32;
pub const EVENT_STREAMS_ESBALLOC_FAILURE: i32 = -2147479645i32;
pub const EVENT_STREAMS_ESBALLOC_FAILURE_CNT: i32 = -2147479644i32;
pub const EVENT_STREAMS_STRLOG: i32 = -1073737824i32;
pub const EVENT_TAKE_OWNERSHIP: i32 = -1073734796i32;
pub const EVENT_TCPIP6_STARTED: i32 = 1073744924i32;
pub const EVENT_TCPIP_ADAPTER_REG_FAILURE: i32 = -1073737633i32;
pub const EVENT_TCPIP_ADDRESS_CONFLICT1: i32 = -1073737626i32;
pub const EVENT_TCPIP_ADDRESS_CONFLICT2: i32 = -1073737625i32;
pub const EVENT_TCPIP_AUTOCONFIGURED_ADDRESS_LIMIT_REACHED: i32 = -2147479444i32;
pub const EVENT_TCPIP_AUTOCONFIGURED_ROUTE_LIMIT_REACHED: i32 = -2147479443i32;
pub const EVENT_TCPIP_CREATE_DEVICE_FAILED: i32 = -1073737724i32;
pub const EVENT_TCPIP_DHCP_INIT_FAILED: i32 = -2147479458i32;
pub const EVENT_TCPIP_INTERFACE_BIND_FAILURE: i32 = -1073737617i32;
pub const EVENT_TCPIP_INVALID_ADDRESS: i32 = -1073737637i32;
pub const EVENT_TCPIP_INVALID_DEFAULT_GATEWAY: i32 = -2147479456i32;
pub const EVENT_TCPIP_INVALID_MASK: i32 = -1073737636i32;
pub const EVENT_TCPIP_IPV4_UNINSTALLED: i32 = 1073746027i32;
pub const EVENT_TCPIP_IP_INIT_FAILED: i32 = -1073737628i32;
pub const EVENT_TCPIP_MEDIA_CONNECT: i32 = 1073746025i32;
pub const EVENT_TCPIP_MEDIA_DISCONNECT: i32 = 1073746026i32;
pub const EVENT_TCPIP_NO_ADAPTER_RESOURCES: i32 = -1073737635i32;
pub const EVENT_TCPIP_NO_ADDRESS_LIST: i32 = -1073737631i32;
pub const EVENT_TCPIP_NO_BINDINGS: i32 = -1073737629i32;
pub const EVENT_TCPIP_NO_MASK: i32 = -1073737638i32;
pub const EVENT_TCPIP_NO_MASK_LIST: i32 = -1073737630i32;
pub const EVENT_TCPIP_NO_RESOURCES_FOR_INIT: i32 = -1073737723i32;
pub const EVENT_TCPIP_NTE_CONTEXT_LIST_FAILURE: i32 = -1073737624i32;
pub const EVENT_TCPIP_OUT_OF_ORDER_FRAGMENTS_EXCEEDED: i32 = -2147479442i32;
pub const EVENT_TCPIP_PCF_CLEAR_FILTER_FAILURE: i32 = -1073737530i32;
pub const EVENT_TCPIP_PCF_MISSING_CAPABILITY: i32 = -2147479357i32;
pub const EVENT_TCPIP_PCF_MULTICAST_OID_ISSUE: i32 = -2147479358i32;
pub const EVENT_TCPIP_PCF_NO_ARP_FILTER: i32 = -2147479355i32;
pub const EVENT_TCPIP_PCF_SET_FILTER_FAILURE: i32 = -2147479356i32;
pub const EVENT_TCPIP_TCP_CONNECTIONS_PERF_IMPACTED: i32 = -2147479418i32;
pub const EVENT_TCPIP_TCP_CONNECT_LIMIT_REACHED: i32 = -2147479422i32;
pub const EVENT_TCPIP_TCP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479417i32;
pub const EVENT_TCPIP_TCP_INIT_FAILED: i32 = -1073737599i32;
pub const EVENT_TCPIP_TCP_MPP_ATTACKS_DETECTED: i32 = -2147479419i32;
pub const EVENT_TCPIP_TCP_TIME_WAIT_COLLISION: i32 = -2147479421i32;
pub const EVENT_TCPIP_TCP_WSD_WS_RESTRICTED: i32 = -2147479420i32;
pub const EVENT_TCPIP_TOO_MANY_GATEWAYS: i32 = -2147479451i32;
pub const EVENT_TCPIP_TOO_MANY_NETS: i32 = -1073737639i32;
pub const EVENT_TCPIP_UDP_GLOBAL_EPHEMERAL_PORT_SPACE_EXHAUSTED: i32 = -2147479382i32;
pub const EVENT_TCPIP_UDP_LIMIT_REACHED: i32 = -2147479383i32;
pub const EVENT_TRANSACT_INVALID: i32 = -1073734812i32;
pub const EVENT_TRANSACT_TIMEOUT: i32 = -1073734813i32;
pub const EVENT_TRANSPORT_ADAPTER_NOT_FOUND: i32 = -1073732818i32;
pub const EVENT_TRANSPORT_BAD_PROTOCOL: i32 = 1073750835i32;
pub const EVENT_TRANSPORT_BINDING_FAILED: i32 = -1073732819i32;
pub const EVENT_TRANSPORT_QUERY_OID_FAILED: i32 = -1073732816i32;
pub const EVENT_TRANSPORT_REGISTER_FAILED: i32 = -1073732820i32;
pub const EVENT_TRANSPORT_RESOURCE_LIMIT: i32 = -2147474646i32;
pub const EVENT_TRANSPORT_RESOURCE_POOL: i32 = -2147474647i32;
pub const EVENT_TRANSPORT_RESOURCE_SPECIFIC: i32 = -2147474645i32;
pub const EVENT_TRANSPORT_SET_OID_FAILED: i32 = -1073732817i32;
pub const EVENT_TRANSPORT_TOO_MANY_LINKS: i32 = 1073750834i32;
pub const EVENT_TRANSPORT_TRANSFER_DATA: i32 = 1073750833i32;
pub const EVENT_TRK_INTERNAL_ERROR: i32 = -1073729324i32;
pub const EVENT_TRK_SERVICE_CORRUPT_LOG: i32 = -1073729321i32;
pub const EVENT_TRK_SERVICE_DUPLICATE_VOLIDS: i32 = 1073754331i32;
pub const EVENT_TRK_SERVICE_MOVE_QUOTA_EXCEEDED: i32 = -2147471140i32;
pub const EVENT_TRK_SERVICE_START_FAILURE: i32 = -1073729322i32;
pub const EVENT_TRK_SERVICE_START_SUCCESS: i32 = 1073754325i32;
pub const EVENT_TRK_SERVICE_VOLUME_CLAIM: i32 = 1073754330i32;
pub const EVENT_TRK_SERVICE_VOLUME_CREATE: i32 = 1073754329i32;
pub const EVENT_TRK_SERVICE_VOL_QUOTA_EXCEEDED: i32 = -2147471144i32;
pub const EVENT_UP_DRIVER_ON_MP: i32 = -1073735724i32;
pub const EVENT_WEBCLIENT_CLOSE_DELETE_FAILED: i32 = -2147468746i32;
pub const EVENT_WEBCLIENT_CLOSE_PROPPATCH_FAILED: i32 = -2147468745i32;
pub const EVENT_WEBCLIENT_CLOSE_PUT_FAILED: i32 = -2147468747i32;
pub const EVENT_WEBCLIENT_SETINFO_PROPPATCH_FAILED: i32 = -2147468744i32;
pub const EVENT_WINNAT_SESSION_LIMIT_REACHED: i32 = -2147466648i32;
pub const EVENT_WINSOCK_CLOSESOCKET_STUCK: i32 = -2147467646i32;
pub const EVENT_WINSOCK_TDI_FILTER_DETECTED: i32 = -2147467647i32;
pub const EVENT_WSK_OWNINGTHREAD_PARAMETER_IGNORED: i32 = -1073725824i32;
pub const EVLEN: u32 = 16u32;
pub const EXTRA_EXIT_POINT: i32 = -1073727524i32;
pub const EXTRA_EXIT_POINT_DELETED: i32 = -1073727520i32;
pub const EXTRA_EXIT_POINT_NOT_DELETED: i32 = -1073727519i32;
pub const EXTRA_VOLUME: i32 = -1073727521i32;
pub const EXTRA_VOLUME_DELETED: i32 = -1073727514i32;
pub const EXTRA_VOLUME_NOT_DELETED: i32 = -1073727513i32;
pub const FILTER_INTERDOMAIN_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(8u32);
pub const FILTER_NORMAL_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(2u32);
pub const FILTER_SERVER_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(32u32);
pub const FILTER_TEMP_DUPLICATE_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(1u32);
pub const FILTER_WORKSTATION_TRUST_ACCOUNT: NET_USER_ENUM_FILTER_FLAGS = NET_USER_ENUM_FILTER_FLAGS(16u32);
pub const GNLEN: u32 = 256u32;
pub const GROUPIDMASK: u32 = 32768u32;
pub const GROUP_ALL_PARMNUM: u32 = 0u32;
pub const GROUP_ATTRIBUTES_PARMNUM: u32 = 3u32;
pub const GROUP_COMMENT_PARMNUM: u32 = 2u32;
pub const GROUP_NAME_PARMNUM: u32 = 1u32;
pub const GROUP_SPECIALGRP_ADMINS: windows_core::PCWSTR = windows_core::w!("ADMINS");
pub const GROUP_SPECIALGRP_GUESTS: windows_core::PCWSTR = windows_core::w!("GUESTS");
pub const GROUP_SPECIALGRP_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const GROUP_SPECIALGRP_USERS: windows_core::PCWSTR = windows_core::w!("USERS");
pub const HARDWARE_ADDRESS_LENGTH: u32 = 6u32;
pub const HELP_MSG_FILENAME: windows_core::PCWSTR = windows_core::w!("NETH");
pub const INTERFACE_INFO_REVISION_1: u32 = 1u32;
pub const INVALID_TRACEID: u32 = 4294967295u32;
pub const IPX_PROTOCOL_BASE: u32 = 131071u32;
pub const IPX_PROTOCOL_RIP: u32 = 131072u32;
pub const IR_PROMISCUOUS: u32 = 0u32;
pub const IR_PROMISCUOUS_MULTICAST: u32 = 1u32;
pub const JOB_ADD_CURRENT_DATE: u32 = 8u32;
pub const JOB_EXEC_ERROR: u32 = 2u32;
pub const JOB_NONINTERACTIVE: u32 = 16u32;
pub const JOB_RUNS_TODAY: u32 = 4u32;
pub const JOB_RUN_PERIODICALLY: u32 = 1u32;
pub const KNOWLEDGE_INCONSISTENCY_DETECTED: i32 = -1073727511i32;
pub const LG_INCLUDE_INDIRECT: u32 = 1u32;
pub const LM20_CNLEN: u32 = 15u32;
pub const LM20_DEVLEN: u32 = 8u32;
pub const LM20_DNLEN: u32 = 15u32;
pub const LM20_GNLEN: u32 = 20u32;
pub const LM20_MAXCOMMENTSZ: u32 = 48u32;
pub const LM20_NNLEN: u32 = 12u32;
pub const LM20_PATHLEN: u32 = 256u32;
pub const LM20_PWLEN: u32 = 14u32;
pub const LM20_QNLEN: u32 = 12u32;
pub const LM20_SERVICE_ACTIVE: u32 = 0u32;
pub const LM20_SERVICE_CONTINUE_PENDING: u32 = 4u32;
pub const LM20_SERVICE_PAUSED: u32 = 12u32;
pub const LM20_SERVICE_PAUSE_PENDING: u32 = 8u32;
pub const LM20_SNLEN: u32 = 15u32;
pub const LM20_STXTLEN: u32 = 63u32;
pub const LM20_UNCLEN: u32 = 17u32;
pub const LM20_UNLEN: u32 = 20u32;
pub const LM_REDIR_FAILURE: i32 = 1073756225i32;
pub const LOCALGROUP_COMMENT_PARMNUM: u32 = 2u32;
pub const LOCALGROUP_NAME_PARMNUM: u32 = 1u32;
pub const LOGFLAGS_BACKWARD: u32 = 1u32;
pub const LOGFLAGS_FORWARD: u32 = 0u32;
pub const LOGFLAGS_SEEK: u32 = 2u32;
pub const LOWER_GET_HINT_MASK: u32 = 65280u32;
pub const LOWER_HINT_MASK: u32 = 255u32;
pub const MACHINE_UNJOINED: i32 = -1073727507i32;
pub const MAJOR_VERSION_MASK: u32 = 15u32;
pub const MAXCOMMENTSZ: u32 = 256u32;
pub const MAXPERMENTRIES: u32 = 64u32;
pub const MAX_LANMAN_MESSAGE_ID: u32 = 5899u32;
pub const MAX_NERR: u32 = 2999u32;
pub const MAX_PASSWD_LEN: u32 = 256u32;
pub const MAX_PREFERRED_LENGTH: u32 = 4294967295u32;
pub const MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
pub const MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
pub const MESSAGE_FILENAME: windows_core::PCWSTR = windows_core::w!("NETMSG");
pub const MFE_BOUNDARY_REACHED: u32 = 6u32;
pub const MFE_IIF: u32 = 8u32;
pub const MFE_NOT_FORWARDING: u32 = 2u32;
pub const MFE_NOT_LAST_HOP: u32 = 10u32;
pub const MFE_NO_ERROR: u32 = 0u32;
pub const MFE_NO_MULTICAST: u32 = 7u32;
pub const MFE_NO_ROUTE: u32 = 9u32;
pub const MFE_NO_SPACE: u32 = 13u32;
pub const MFE_OIF_PRUNED: u32 = 5u32;
pub const MFE_OLD_ROUTER: u32 = 11u32;
pub const MFE_PROHIBITED: u32 = 12u32;
pub const MFE_PRUNED_UPSTREAM: u32 = 4u32;
pub const MFE_REACHED_CORE: u32 = 1u32;
pub const MFE_WRONG_IF: u32 = 3u32;
pub const MIN_LANMAN_MESSAGE_ID: u32 = 2100u32;
pub const MISSING_EXIT_POINT: i32 = -1073727523i32;
pub const MISSING_EXIT_POINT_CREATED: i32 = -1073727518i32;
pub const MISSING_EXIT_POINT_NOT_CREATED: i32 = -1073727517i32;
pub const MISSING_VOLUME: i32 = -1073727522i32;
pub const MISSING_VOLUME_CREATED: i32 = -1073727516i32;
pub const MISSING_VOLUME_NOT_CREATED: i32 = -1073727515i32;
pub const MODALS_DOMAIN_ID_PARMNUM: u32 = 9u32;
pub const MODALS_DOMAIN_NAME_PARMNUM: u32 = 8u32;
pub const MODALS_FORCE_LOGOFF_PARMNUM: u32 = 4u32;
pub const MODALS_LOCKOUT_DURATION_PARMNUM: u32 = 10u32;
pub const MODALS_LOCKOUT_OBSERVATION_WINDOW_PARMNUM: u32 = 11u32;
pub const MODALS_LOCKOUT_THRESHOLD_PARMNUM: u32 = 12u32;
pub const MODALS_MAX_PASSWD_AGE_PARMNUM: u32 = 2u32;
pub const MODALS_MIN_PASSWD_AGE_PARMNUM: u32 = 3u32;
pub const MODALS_MIN_PASSWD_LEN_PARMNUM: u32 = 1u32;
pub const MODALS_PASSWD_HIST_LEN_PARMNUM: u32 = 5u32;
pub const MODALS_PRIMARY_PARMNUM: u32 = 7u32;
pub const MODALS_ROLE_PARMNUM: u32 = 6u32;
pub const MRINFO_DISABLED_FLAG: u32 = 32u32;
pub const MRINFO_DOWN_FLAG: u32 = 16u32;
pub const MRINFO_LEAF_FLAG: u32 = 128u32;
pub const MRINFO_PIM_FLAG: u32 = 4u32;
pub const MRINFO_QUERIER_FLAG: u32 = 64u32;
pub const MRINFO_TUNNEL_FLAG: u32 = 1u32;
pub const MSGNAME_FORWARDED_FROM: u32 = 16u32;
pub const MSGNAME_FORWARDED_TO: u32 = 4u32;
pub const MSGNAME_NOT_FORWARDED: u32 = 0u32;
pub const MS_ROUTER_VERSION: u32 = 1536u32;
pub const MsaInfoCanInstall: MSA_INFO_STATE = MSA_INFO_STATE(4i32);
pub const MsaInfoCannotInstall: MSA_INFO_STATE = MSA_INFO_STATE(3i32);
pub const MsaInfoInstalled: MSA_INFO_STATE = MSA_INFO_STATE(5i32);
pub const MsaInfoLevel0: MSA_INFO_LEVEL = MSA_INFO_LEVEL(0i32);
pub const MsaInfoLevelMax: MSA_INFO_LEVEL = MSA_INFO_LEVEL(1i32);
pub const MsaInfoNotExist: MSA_INFO_STATE = MSA_INFO_STATE(1i32);
pub const MsaInfoNotService: MSA_INFO_STATE = MSA_INFO_STATE(2i32);
pub const NCF_DONTEXPOSELOWER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4096i32);
pub const NCF_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1024i32);
pub const NCF_FIXED_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(131072i32);
pub const NCF_HAS_UI: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(128i32);
pub const NCF_HIDDEN: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8i32);
pub const NCF_HIDE_BINDING: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(8192i32);
pub const NCF_LOWER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(1i32);
pub const NCF_LW_FILTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(262144i32);
pub const NCF_MULTIPORT_INSTANCED_ADAPTER: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(64i32);
pub const NCF_NDIS_PROTOCOL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16384i32);
pub const NCF_NOT_USER_REMOVABLE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(32i32);
pub const NCF_NO_SERVICE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(16i32);
pub const NCF_PHYSICAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(4i32);
pub const NCF_SINGLE_INSTANCE: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(256i32);
pub const NCF_SOFTWARE_ENUMERATED: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(2i32);
pub const NCF_UPPER: SUPPORTS_BINDING_INTERFACE_FLAGS = SUPPORTS_BINDING_INTERFACE_FLAGS(2i32);
pub const NCF_VIRTUAL: COMPONENT_CHARACTERISTICS = COMPONENT_CHARACTERISTICS(1i32);
pub const NCN_ADD: BIND_FLAGS1 = BIND_FLAGS1(1i32);
pub const NCN_BINDING_PATH: BIND_FLAGS1 = BIND_FLAGS1(256i32);
pub const NCN_DISABLE: BIND_FLAGS1 = BIND_FLAGS1(32i32);
pub const NCN_ENABLE: BIND_FLAGS1 = BIND_FLAGS1(16i32);
pub const NCN_NET: BIND_FLAGS1 = BIND_FLAGS1(65536i32);
pub const NCN_NETCLIENT: BIND_FLAGS1 = BIND_FLAGS1(262144i32);
pub const NCN_NETSERVICE: BIND_FLAGS1 = BIND_FLAGS1(524288i32);
pub const NCN_NETTRANS: BIND_FLAGS1 = BIND_FLAGS1(131072i32);
pub const NCN_PROPERTYCHANGE: BIND_FLAGS1 = BIND_FLAGS1(512i32);
pub const NCN_REMOVE: BIND_FLAGS1 = BIND_FLAGS1(2i32);
pub const NCN_UPDATE: BIND_FLAGS1 = BIND_FLAGS1(4i32);
pub const NCRL_NDIS: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(1i32);
pub const NCRL_TDI: NCPNP_RECONFIG_LAYER = NCPNP_RECONFIG_LAYER(2i32);
pub const NCRP_QUERY_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(1i32);
pub const NCRP_SHOW_PROPERTY_UI: NCRP_FLAGS = NCRP_FLAGS(2i32);
pub const NELOG_AT_Exec_Err: u32 = 3178u32;
pub const NELOG_AT_cannot_read: u32 = 3174u32;
pub const NELOG_AT_cannot_write: u32 = 3129u32;
pub const NELOG_AT_sched_err: u32 = 3175u32;
pub const NELOG_AT_schedule_file_created: u32 = 3176u32;
pub const NELOG_Access_File_Bad: u32 = 3122u32;
pub const NELOG_Build_Name: u32 = 3170u32;
pub const NELOG_Cant_Make_Msg_File: u32 = 3130u32;
pub const NELOG_DiskFT: u32 = 3221u32;
pub const NELOG_DriverNotLoaded: u32 = 5727u32;
pub const NELOG_Entries_Lost: u32 = 3114u32;
pub const NELOG_Error_in_DLL: u32 = 3256u32;
pub const NELOG_Exec_Netservr_NoMem: u32 = 3131u32;
pub const NELOG_FT_ErrLog_Too_Large: u32 = 3258u32;
pub const NELOG_FT_Update_In_Progress: u32 = 3259u32;
pub const NELOG_FailedToGetComputerName: u32 = 5726u32;
pub const NELOG_FailedToRegisterSC: u32 = 5724u32;
pub const NELOG_FailedToSetServiceStatus: u32 = 5725u32;
pub const NELOG_File_Changed: u32 = 3253u32;
pub const NELOG_Files_Dont_Fit: u32 = 3254u32;
pub const NELOG_HardErr_From_Server: u32 = 3182u32;
pub const NELOG_HotFix: u32 = 3181u32;
pub const NELOG_Init_Chardev_Err: u32 = 3124u32;
pub const NELOG_Init_Exec_Fail: u32 = 3105u32;
pub const NELOG_Init_OpenCreate_Err: u32 = 3110u32;
pub const NELOG_Init_Seg_Overflow: u32 = 3120u32;
pub const NELOG_Internal_Error: u32 = 3100u32;
pub const NELOG_Invalid_Config_File: u32 = 3252u32;
pub const NELOG_Invalid_Config_Line: u32 = 3251u32;
pub const NELOG_Ioctl_Error: u32 = 3108u32;
pub const NELOG_Joined_Domain: u32 = 3260u32;
pub const NELOG_Joined_Workgroup: u32 = 3261u32;
pub const NELOG_Lazy_Write_Err: u32 = 3180u32;
pub const NELOG_LocalSecFail1: u32 = 3183u32;
pub const NELOG_LocalSecFail2: u32 = 3184u32;
pub const NELOG_LocalSecFail3: u32 = 3185u32;
pub const NELOG_LocalSecGeneralFail: u32 = 3186u32;
pub const NELOG_Mail_Slt_Err: u32 = 3173u32;
pub const NELOG_Mailslot_err: u32 = 3127u32;
pub const NELOG_Message_Send: u32 = 3172u32;
pub const NELOG_Missing_Parameter: u32 = 3250u32;
pub const NELOG_Msg_Log_Err: u32 = 3150u32;
pub const NELOG_Msg_Sem_Shutdown: u32 = 3141u32;
pub const NELOG_Msg_Shutdown: u32 = 3140u32;
pub const NELOG_Msg_Unexpected_SMB_Type: u32 = 3152u32;
pub const NELOG_Name_Expansion: u32 = 3171u32;
pub const NELOG_Ncb_Error: u32 = 3106u32;
pub const NELOG_Ncb_TooManyErr: u32 = 3126u32;
pub const NELOG_NetBios: u32 = 3111u32;
pub const NELOG_NetLogonFailedToInitializeAuthzRm: u32 = 5821u32;
pub const NELOG_NetLogonFailedToInitializeRPCSD: u32 = 5822u32;
pub const NELOG_NetWkSta_Internal_Error: u32 = 3190u32;
pub const NELOG_NetWkSta_NCB_Err: u32 = 3195u32;
pub const NELOG_NetWkSta_No_Resource: u32 = 3191u32;
pub const NELOG_NetWkSta_Reset_Err: u32 = 3197u32;
pub const NELOG_NetWkSta_SMB_Err: u32 = 3192u32;
pub const NELOG_NetWkSta_Stuck_VC_Err: u32 = 3194u32;
pub const NELOG_NetWkSta_Too_Many: u32 = 3198u32;
pub const NELOG_NetWkSta_VC_Err: u32 = 3193u32;
pub const NELOG_NetWkSta_Write_Behind_Err: u32 = 3196u32;
pub const NELOG_Net_Not_Started: u32 = 3107u32;
pub const NELOG_NetlogonAddNameFailure: u32 = 5741u32;
pub const NELOG_NetlogonAuthDCFail: u32 = 3210u32;
pub const NELOG_NetlogonAuthDomainDowngraded: u32 = 5791u32;
pub const NELOG_NetlogonAuthNoDomainController: u32 = 5719u32;
pub const NELOG_NetlogonAuthNoTrustLsaSecret: u32 = 5720u32;
pub const NELOG_NetlogonAuthNoTrustSamAccount: u32 = 5721u32;
pub const NELOG_NetlogonAuthNoUplevelDomainController: u32 = 5790u32;
pub const NELOG_NetlogonBadSiteName: u32 = 5779u32;
pub const NELOG_NetlogonBadSubnetName: u32 = 5780u32;
pub const NELOG_NetlogonBrowserDriver: u32 = 5740u32;
pub const NELOG_NetlogonChangeLogCorrupt: u32 = 5705u32;
pub const NELOG_NetlogonDcOldSiteCovered: u32 = 5794u32;
pub const NELOG_NetlogonDcSiteCovered: u32 = 5784u32;
pub const NELOG_NetlogonDcSiteNotCovered: u32 = 5785u32;
pub const NELOG_NetlogonDcSiteNotCoveredAuto: u32 = 5795u32;
pub const NELOG_NetlogonDnsDeregAborted: u32 = 5808u32;
pub const NELOG_NetlogonDnsHostNameLowerCasingFailed: u32 = 5825u32;
pub const NELOG_NetlogonDownLevelLogoffFailed: u32 = 5708u32;
pub const NELOG_NetlogonDownLevelLogonFailed: u32 = 5707u32;
pub const NELOG_NetlogonDuplicateMachineAccounts: u32 = 5738u32;
pub const NELOG_NetlogonDynamicDnsDeregisterFailure: u32 = 5775u32;
pub const NELOG_NetlogonDynamicDnsFailure: u32 = 5782u32;
pub const NELOG_NetlogonDynamicDnsRegisterFailure: u32 = 5774u32;
pub const NELOG_NetlogonDynamicDnsServerFailure: u32 = 5781u32;
pub const NELOG_NetlogonFailedAccountDelta: u32 = 5735u32;
pub const NELOG_NetlogonFailedDnsHostNameUpdate: u32 = 5789u32;
pub const NELOG_NetlogonFailedDomainDelta: u32 = 5729u32;
pub const NELOG_NetlogonFailedFileCreate: u32 = 5776u32;
pub const NELOG_NetlogonFailedGlobalGroupDelta: u32 = 5730u32;
pub const NELOG_NetlogonFailedLocalGroupDelta: u32 = 5731u32;
pub const NELOG_NetlogonFailedPolicyDelta: u32 = 5733u32;
pub const NELOG_NetlogonFailedPrimary: u32 = 3223u32;
pub const NELOG_NetlogonFailedSecretDelta: u32 = 5736u32;
pub const NELOG_NetlogonFailedSpnUpdate: u32 = 5788u32;
pub const NELOG_NetlogonFailedToAddAuthzRpcInterface: u32 = 5820u32;
pub const NELOG_NetlogonFailedToAddRpcInterface: u32 = 5702u32;
pub const NELOG_NetlogonFailedToCreateShare: u32 = 5706u32;
pub const NELOG_NetlogonFailedToReadMailslot: u32 = 5703u32;
pub const NELOG_NetlogonFailedToRegisterSC: u32 = 5704u32;
pub const NELOG_NetlogonFailedToUpdateTrustList: u32 = 5701u32;
pub const NELOG_NetlogonFailedTrustedDomainDelta: u32 = 5734u32;
pub const NELOG_NetlogonFailedUserDelta: u32 = 5732u32;
pub const NELOG_NetlogonFullSyncCallFailed: u32 = 5714u32;
pub const NELOG_NetlogonFullSyncCallSuccess: u32 = 5713u32;
pub const NELOG_NetlogonFullSyncFailed: u32 = 5718u32;
pub const NELOG_NetlogonFullSyncSuccess: u32 = 5717u32;
pub const NELOG_NetlogonGcOldSiteCovered: u32 = 5796u32;
pub const NELOG_NetlogonGcSiteCovered: u32 = 5786u32;
pub const NELOG_NetlogonGcSiteNotCovered: u32 = 5787u32;
pub const NELOG_NetlogonGcSiteNotCoveredAuto: u32 = 5797u32;
pub const NELOG_NetlogonGetSubnetToSite: u32 = 5777u32;
pub const NELOG_NetlogonInvalidDwordParameterValue: u32 = 5804u32;
pub const NELOG_NetlogonInvalidGenericParameterValue: u32 = 5803u32;
pub const NELOG_NetlogonLanmanBdcsNotAllowed: u32 = 5772u32;
pub const NELOG_NetlogonMachinePasswdSetSucceeded: u32 = 5823u32;
pub const NELOG_NetlogonMsaPasswdSetSucceeded: u32 = 5824u32;
pub const NELOG_NetlogonNTLogoffFailed: u32 = 5710u32;
pub const NELOG_NetlogonNTLogonFailed: u32 = 5709u32;
pub const NELOG_NetlogonNdncOldSiteCovered: u32 = 5798u32;
pub const NELOG_NetlogonNdncSiteCovered: u32 = 5792u32;
pub const NELOG_NetlogonNdncSiteNotCovered: u32 = 5793u32;
pub const NELOG_NetlogonNdncSiteNotCoveredAuto: u32 = 5799u32;
pub const NELOG_NetlogonNoAddressToSiteMapping: u32 = 5802u32;
pub const NELOG_NetlogonNoDynamicDns: u32 = 5773u32;
pub const NELOG_NetlogonNoDynamicDnsManual: u32 = 5806u32;
pub const NELOG_NetlogonNoSiteForClient: u32 = 5778u32;
pub const NELOG_NetlogonNoSiteForClients: u32 = 5807u32;
pub const NELOG_NetlogonPartialSiteMappingForClients: u32 = 5810u32;
pub const NELOG_NetlogonPartialSyncCallFailed: u32 = 5712u32;
pub const NELOG_NetlogonPartialSyncCallSuccess: u32 = 5711u32;
pub const NELOG_NetlogonPartialSyncFailed: u32 = 5716u32;
pub const NELOG_NetlogonPartialSyncSuccess: u32 = 5715u32;
pub const NELOG_NetlogonPasswdSetFailed: u32 = 3224u32;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsDeregister: u32 = 5814u32;
pub const NELOG_NetlogonRejectedRemoteDynamicDnsRegister: u32 = 5813u32;
pub const NELOG_NetlogonRemoteDynamicDnsDeregisterFailure: u32 = 5812u32;
pub const NELOG_NetlogonRemoteDynamicDnsRegisterFailure: u32 = 5811u32;
pub const NELOG_NetlogonRemoteDynamicDnsUpdateRequestFailure: u32 = 5815u32;
pub const NELOG_NetlogonRequireSignOrSealError: u32 = 3227u32;
pub const NELOG_NetlogonRpcCallCancelled: u32 = 5783u32;
pub const NELOG_NetlogonRpcPortRequestFailure: u32 = 5809u32;
pub const NELOG_NetlogonSSIInitError: u32 = 5700u32;
pub const NELOG_NetlogonServerAuthFailed: u32 = 5722u32;
pub const NELOG_NetlogonServerAuthFailedNoAccount: u32 = 5805u32;
pub const NELOG_NetlogonServerAuthNoTrustSamAccount: u32 = 5723u32;
pub const NELOG_NetlogonSessionTypeWrong: u32 = 5770u32;
pub const NELOG_NetlogonSpnCrackNamesFailure: u32 = 5801u32;
pub const NELOG_NetlogonSpnMultipleSamAccountNames: u32 = 5800u32;
pub const NELOG_NetlogonSyncError: u32 = 3226u32;
pub const NELOG_NetlogonSystemError: u32 = 5737u32;
pub const NELOG_NetlogonTooManyGlobalGroups: u32 = 5739u32;
pub const NELOG_NetlogonTrackingError: u32 = 3225u32;
pub const NELOG_NetlogonUserValidationReqInitialTimeOut: u32 = 5816u32;
pub const NELOG_NetlogonUserValidationReqRecurringTimeOut: u32 = 5817u32;
pub const NELOG_NetlogonUserValidationReqWaitInitialWarning: u32 = 5818u32;
pub const NELOG_NetlogonUserValidationReqWaitRecurringWarning: u32 = 5819u32;
pub const NELOG_NoTranportLoaded: u32 = 5728u32;
pub const NELOG_OEM_Code: u32 = 3299u32;
pub const NELOG_ReleaseMem_Alert: u32 = 3128u32;
pub const NELOG_Remote_API: u32 = 3125u32;
pub const NELOG_ReplAccessDenied: u32 = 3222u32;
pub const NELOG_ReplBadExport: u32 = 3219u32;
pub const NELOG_ReplBadImport: u32 = 3218u32;
pub const NELOG_ReplBadMsg: u32 = 3215u32;
pub const NELOG_ReplCannotMasterDir: u32 = 3207u32;
pub const NELOG_ReplLogonFailed: u32 = 3211u32;
pub const NELOG_ReplLostMaster: u32 = 3209u32;
pub const NELOG_ReplMaxFiles: u32 = 3213u32;
pub const NELOG_ReplMaxTreeDepth: u32 = 3214u32;
pub const NELOG_ReplNetErr: u32 = 3212u32;
pub const NELOG_ReplSignalFileErr: u32 = 3220u32;
pub const NELOG_ReplSysErr: u32 = 3216u32;
pub const NELOG_ReplUpdateError: u32 = 3208u32;
pub const NELOG_ReplUserCurDir: u32 = 3206u32;
pub const NELOG_ReplUserLoged: u32 = 3217u32;
pub const NELOG_Resource_Shortage: u32 = 3101u32;
pub const NELOG_RplAdapterResource: u32 = 5756u32;
pub const NELOG_RplBackupDatabase: u32 = 5765u32;
pub const NELOG_RplCheckConfigs: u32 = 5760u32;
pub const NELOG_RplCheckSecurity: u32 = 5764u32;
pub const NELOG_RplCreateProfiles: u32 = 5761u32;
pub const NELOG_RplFileCopy: u32 = 5757u32;
pub const NELOG_RplFileDelete: u32 = 5758u32;
pub const NELOG_RplFilePerms: u32 = 5759u32;
pub const NELOG_RplInitDatabase: u32 = 5766u32;
pub const NELOG_RplInitRestoredDatabase: u32 = 5769u32;
pub const NELOG_RplMessages: u32 = 5742u32;
pub const NELOG_RplRegistry: u32 = 5762u32;
pub const NELOG_RplReplaceRPLDISK: u32 = 5763u32;
pub const NELOG_RplRestoreDatabaseFailure: u32 = 5767u32;
pub const NELOG_RplRestoreDatabaseSuccess: u32 = 5768u32;
pub const NELOG_RplSystem: u32 = 5744u32;
pub const NELOG_RplUpgradeDBTo40: u32 = 5771u32;
pub const NELOG_RplWkstaBbcFile: u32 = 5751u32;
pub const NELOG_RplWkstaFileChecksum: u32 = 5749u32;
pub const NELOG_RplWkstaFileLineCount: u32 = 5750u32;
pub const NELOG_RplWkstaFileOpen: u32 = 5746u32;
pub const NELOG_RplWkstaFileRead: u32 = 5747u32;
pub const NELOG_RplWkstaFileSize: u32 = 5752u32;
pub const NELOG_RplWkstaInternal: u32 = 5753u32;
pub const NELOG_RplWkstaMemory: u32 = 5748u32;
pub const NELOG_RplWkstaNetwork: u32 = 5755u32;
pub const NELOG_RplWkstaTimeout: u32 = 5745u32;
pub const NELOG_RplWkstaWrongVersion: u32 = 5754u32;
pub const NELOG_RplXnsBoot: u32 = 5743u32;
pub const NELOG_SMB_Illegal: u32 = 3112u32;
pub const NELOG_Server_Lock_Failure: u32 = 3132u32;
pub const NELOG_Service_Fail: u32 = 3113u32;
pub const NELOG_Srv_Close_Failure: u32 = 3205u32;
pub const NELOG_Srv_No_Mem_Grow: u32 = 3121u32;
pub const NELOG_Srv_Thread_Failure: u32 = 3204u32;
pub const NELOG_Srvnet_NB_Open: u32 = 3177u32;
pub const NELOG_Srvnet_Not_Started: u32 = 3123u32;
pub const NELOG_System_Error: u32 = 3257u32;
pub const NELOG_System_Semaphore: u32 = 3109u32;
pub const NELOG_UPS_CannotOpenDriver: u32 = 3233u32;
pub const NELOG_UPS_CmdFileConfig: u32 = 3235u32;
pub const NELOG_UPS_CmdFileError: u32 = 3232u32;
pub const NELOG_UPS_CmdFileExec: u32 = 3236u32;
pub const NELOG_UPS_PowerBack: u32 = 3234u32;
pub const NELOG_UPS_PowerOut: u32 = 3230u32;
pub const NELOG_UPS_Shutdown: u32 = 3231u32;
pub const NELOG_Unable_To_Lock_Segment: u32 = 3102u32;
pub const NELOG_Unable_To_Unlock_Segment: u32 = 3103u32;
pub const NELOG_Uninstall_Service: u32 = 3104u32;
pub const NELOG_VIO_POPUP_ERR: u32 = 3151u32;
pub const NELOG_Wksta_Bad_Mailslot_SMB: u32 = 3165u32;
pub const NELOG_Wksta_BiosThreadFailure: u32 = 3162u32;
pub const NELOG_Wksta_Compname: u32 = 3161u32;
pub const NELOG_Wksta_HostTab_Full: u32 = 3164u32;
pub const NELOG_Wksta_Infoseg: u32 = 3160u32;
pub const NELOG_Wksta_IniSeg: u32 = 3163u32;
pub const NELOG_Wksta_SSIRelogon: u32 = 3167u32;
pub const NELOG_Wksta_UASInit: u32 = 3166u32;
pub const NELOG_Wrong_DLL_Version: u32 = 3255u32;
pub const NERR_ACFFileIOFail: u32 = 2229u32;
pub const NERR_ACFNoParent: u32 = 2232u32;
pub const NERR_ACFNoRoom: u32 = 2228u32;
pub const NERR_ACFNotFound: u32 = 2219u32;
pub const NERR_ACFNotLoaded: u32 = 2227u32;
pub const NERR_ACFTooManyLists: u32 = 2230u32;
pub const NERR_AccountExpired: u32 = 2239u32;
pub const NERR_AccountLockedOut: u32 = 2702u32;
pub const NERR_AccountReuseBlockedByPolicy: u32 = 2732u32;
pub const NERR_AccountUndefined: u32 = 2238u32;
pub const NERR_AcctLimitExceeded: u32 = 2434u32;
pub const NERR_ActiveConns: u32 = 2402u32;
pub const NERR_AddForwarded: u32 = 2275u32;
pub const NERR_AlertExists: u32 = 2430u32;
pub const NERR_AlreadyCloudDomainJoined: u32 = 2700u32;
pub const NERR_AlreadyExists: u32 = 2276u32;
pub const NERR_AlreadyForwarded: u32 = 2274u32;
pub const NERR_AlreadyLoggedOn: u32 = 2200u32;
pub const NERR_BASE: u32 = 2100u32;
pub const NERR_BadAsgType: u32 = 2251u32;
pub const NERR_BadComponent: u32 = 2356u32;
pub const NERR_BadControlRecv: u32 = 2193u32;
pub const NERR_BadDest: u32 = 2382u32;
pub const NERR_BadDev: u32 = 2341u32;
pub const NERR_BadDevString: u32 = 2340u32;
pub const NERR_BadDomainJoinInfo: u32 = 2712u32;
pub const NERR_BadDosFunction: u32 = 2502u32;
pub const NERR_BadDosRetCode: u32 = 2500u32;
pub const NERR_BadEventName: u32 = 2143u32;
pub const NERR_BadFileCheckSum: u32 = 2504u32;
pub const NERR_BadOfflineJoinInfo: u32 = 2710u32;
pub const NERR_BadPassword: u32 = 2203u32;
pub const NERR_BadPasswordCore: u32 = 2403u32;
pub const NERR_BadQueueDevString: u32 = 2334u32;
pub const NERR_BadQueuePriority: u32 = 2335u32;
pub const NERR_BadReceive: u32 = 2282u32;
pub const NERR_BadRecipient: u32 = 2433u32;
pub const NERR_BadServiceName: u32 = 2185u32;
pub const NERR_BadServiceProgName: u32 = 2188u32;
pub const NERR_BadSource: u32 = 2381u32;
pub const NERR_BadTransactConfig: u32 = 2141u32;
pub const NERR_BadUasConfig: u32 = 2450u32;
pub const NERR_BadUsername: u32 = 2202u32;
pub const NERR_BrowserConfiguredToNotRun: u32 = 2550u32;
pub const NERR_BrowserNotStarted: u32 = 2139u32;
pub const NERR_BrowserTableIncomplete: u32 = 2319u32;
pub const NERR_BufTooSmall: u32 = 2123u32;
pub const NERR_CallingRplSrvr: u32 = 2515u32;
pub const NERR_CanNotGrowSegment: u32 = 2233u32;
pub const NERR_CanNotGrowUASFile: u32 = 2456u32;
pub const NERR_CannotUnjoinAadDomain: u32 = 2727u32;
pub const NERR_CannotUpdateAadHostName: u32 = 2728u32;
pub const NERR_CantConnectRplSrvr: u32 = 2513u32;
pub const NERR_CantCreateJoinInfo: u32 = 2711u32;
pub const NERR_CantLoadOfflineHive: u32 = 2717u32;
pub const NERR_CantOpenImageFile: u32 = 2514u32;
pub const NERR_CantType: u32 = 2357u32;
pub const NERR_CantVerifyHostname: u32 = 2716u32;
pub const NERR_CfgCompNotFound: u32 = 2146u32;
pub const NERR_CfgParamNotFound: u32 = 2147u32;
pub const NERR_ClientNameNotFound: u32 = 2312u32;
pub const NERR_CommDevInUse: u32 = 2343u32;
pub const NERR_ComputerAccountNotFound: u32 = 2697u32;
pub const NERR_ConnectionInsecure: u32 = 2718u32;
pub const NERR_DCNotFound: u32 = 2453u32;
pub const NERR_DS8DCNotFound: u32 = 2722u32;
pub const NERR_DS8DCRequired: u32 = 2720u32;
pub const NERR_DS9DCNotFound: u32 = 2725u32;
pub const NERR_DataTypeInvalid: u32 = 2167u32;
pub const NERR_DatabaseUpToDate: u32 = 2248u32;
pub const NERR_DefaultJoinRequired: u32 = 2694u32;
pub const NERR_DelComputerName: u32 = 2278u32;
pub const NERR_DeleteLater: u32 = 2298u32;
pub const NERR_DestExists: u32 = 2153u32;
pub const NERR_DestIdle: u32 = 2158u32;
pub const NERR_DestInvalidOp: u32 = 2159u32;
pub const NERR_DestInvalidState: u32 = 2162u32;
pub const NERR_DestNoRoom: u32 = 2157u32;
pub const NERR_DestNotFound: u32 = 2152u32;
pub const NERR_DevInUse: u32 = 2404u32;
pub const NERR_DevInvalidOpCode: u32 = 2331u32;
pub const NERR_DevNotFound: u32 = 2332u32;
pub const NERR_DevNotOpen: u32 = 2333u32;
pub const NERR_DevNotRedirected: u32 = 2107u32;
pub const NERR_DeviceIsShared: u32 = 2252u32;
pub const NERR_DeviceNotShared: u32 = 2311u32;
pub const NERR_DeviceShareConflict: u32 = 2318u32;
pub const NERR_DfsAlreadyShared: u32 = 2664u32;
pub const NERR_DfsBadRenamePath: u32 = 2671u32;
pub const NERR_DfsCantCreateJunctionPoint: u32 = 2669u32;
pub const NERR_DfsCantRemoveDfsRoot: u32 = 2682u32;
pub const NERR_DfsCantRemoveLastServerShare: u32 = 2677u32;
pub const NERR_DfsChildOrParentInDfs: u32 = 2683u32;
pub const NERR_DfsCyclicalName: u32 = 2674u32;
pub const NERR_DfsDataIsIdentical: u32 = 2681u32;
pub const NERR_DfsDuplicateService: u32 = 2676u32;
pub const NERR_DfsInconsistent: u32 = 2679u32;
pub const NERR_DfsInternalCorruption: u32 = 2660u32;
pub const NERR_DfsInternalError: u32 = 2690u32;
pub const NERR_DfsLeafVolume: u32 = 2667u32;
pub const NERR_DfsNoSuchServer: u32 = 2673u32;
pub const NERR_DfsNoSuchShare: u32 = 2665u32;
pub const NERR_DfsNoSuchVolume: u32 = 2662u32;
pub const NERR_DfsNotALeafVolume: u32 = 2666u32;
pub const NERR_DfsNotSupportedInServerDfs: u32 = 2675u32;
pub const NERR_DfsServerNotDfsAware: u32 = 2670u32;
pub const NERR_DfsServerUpgraded: u32 = 2680u32;
pub const NERR_DfsVolumeAlreadyExists: u32 = 2663u32;
pub const NERR_DfsVolumeDataCorrupt: u32 = 2661u32;
pub const NERR_DfsVolumeHasMultipleServers: u32 = 2668u32;
pub const NERR_DfsVolumeIsInterDfs: u32 = 2678u32;
pub const NERR_DfsVolumeIsOffline: u32 = 2672u32;
pub const NERR_DifferentServers: u32 = 2383u32;
pub const NERR_DriverNotFound: u32 = 2166u32;
pub const NERR_DupNameReboot: u32 = 2144u32;
pub const NERR_DuplicateHostName: u32 = 2729u32;
pub const NERR_DuplicateName: u32 = 2297u32;
pub const NERR_DuplicateShare: u32 = 2118u32;
pub const NERR_ErrCommRunSrv: u32 = 2389u32;
pub const NERR_ErrorExecingGhost: u32 = 2391u32;
pub const NERR_ExecFailure: u32 = 2315u32;
pub const NERR_FileIdNotFound: u32 = 2314u32;
pub const NERR_GroupExists: u32 = 2223u32;
pub const NERR_GroupNotFound: u32 = 2220u32;
pub const NERR_GrpMsgProcessor: u32 = 2280u32;
pub const NERR_HostNameTooLong: u32 = 2730u32;
pub const NERR_ImageParamErr: u32 = 2508u32;
pub const NERR_InUseBySpooler: u32 = 2342u32;
pub const NERR_IncompleteDel: u32 = 2299u32;
pub const NERR_InternalError: u32 = 2140u32;
pub const NERR_InvalidAPI: u32 = 2142u32;
pub const NERR_InvalidComputer: u32 = 2351u32;
pub const NERR_InvalidDatabase: u32 = 2247u32;
pub const NERR_InvalidDevice: u32 = 2294u32;
pub const NERR_InvalidLana: u32 = 2400u32;
pub const NERR_InvalidLogSeek: u32 = 2440u32;
pub const NERR_InvalidLogonHours: u32 = 2241u32;
pub const NERR_InvalidMachineNameForJoin: u32 = 2724u32;
pub const NERR_InvalidMaxUsers: u32 = 2122u32;
pub const NERR_InvalidUASOp: u32 = 2451u32;
pub const NERR_InvalidWorkgroupName: u32 = 2695u32;
pub const NERR_InvalidWorkstation: u32 = 2240u32;
pub const NERR_IsDfsShare: u32 = 2321u32;
pub const NERR_ItemNotFound: u32 = 2115u32;
pub const NERR_JobInvalidState: u32 = 2164u32;
pub const NERR_JobNoRoom: u32 = 2156u32;
pub const NERR_JobNotFound: u32 = 2151u32;
pub const NERR_JoinPerformedMustRestart: u32 = 2713u32;
pub const NERR_LDAPCapableDCRequired: u32 = 2721u32;
pub const NERR_LanmanIniError: u32 = 2131u32;
pub const NERR_LastAdmin: u32 = 2452u32;
pub const NERR_LineTooLong: u32 = 2149u32;
pub const NERR_LocalDrive: u32 = 2405u32;
pub const NERR_LocalForward: u32 = 2279u32;
pub const NERR_LogFileChanged: u32 = 2378u32;
pub const NERR_LogFileCorrupt: u32 = 2379u32;
pub const NERR_LogOverflow: u32 = 2377u32;
pub const NERR_LogonDomainExists: u32 = 2216u32;
pub const NERR_LogonNoUserPath: u32 = 2211u32;
pub const NERR_LogonScriptError: u32 = 2212u32;
pub const NERR_LogonServerConflict: u32 = 2210u32;
pub const NERR_LogonServerNotFound: u32 = 2215u32;
pub const NERR_LogonTrackingError: u32 = 2454u32;
pub const NERR_LogonsPaused: u32 = 2209u32;
pub const NERR_MaxLenExceeded: u32 = 2354u32;
pub const NERR_MsgAlreadyStarted: u32 = 2271u32;
pub const NERR_MsgInitFailed: u32 = 2272u32;
pub const NERR_MsgNotStarted: u32 = 2284u32;
pub const NERR_MultipleNets: u32 = 2300u32;
pub const NERR_NameInUse: u32 = 2283u32;
pub const NERR_NameNotForwarded: u32 = 2288u32;
pub const NERR_NameNotFound: u32 = 2273u32;
pub const NERR_NameUsesIncompatibleCodePage: u32 = 2696u32;
pub const NERR_NetNameNotFound: u32 = 2310u32;
pub const NERR_NetNotStarted: u32 = 2102u32;
pub const NERR_NetlogonNotStarted: u32 = 2455u32;
pub const NERR_NetworkError: u32 = 2136u32;
pub const NERR_NoAlternateServers: u32 = 2467u32;
pub const NERR_NoCommDevs: u32 = 2337u32;
pub const NERR_NoComputerName: u32 = 2270u32;
pub const NERR_NoForwardName: u32 = 2286u32;
pub const NERR_NoJoinPending: u32 = 2714u32;
pub const NERR_NoNetworkResource: u32 = 2105u32;
pub const NERR_NoOfflineJoinInfo: u32 = 2709u32;
pub const NERR_NoRoom: u32 = 2119u32;
pub const NERR_NoRplBootSystem: u32 = 2505u32;
pub const NERR_NoSuchAlert: u32 = 2432u32;
pub const NERR_NoSuchConnection: u32 = 2462u32;
pub const NERR_NoSuchServer: u32 = 2460u32;
pub const NERR_NoSuchSession: u32 = 2461u32;
pub const NERR_NonDosFloppyUsed: u32 = 2510u32;
pub const NERR_NonValidatedLogon: u32 = 2217u32;
pub const NERR_NotInCache: u32 = 2235u32;
pub const NERR_NotInDispatchTbl: u32 = 2192u32;
pub const NERR_NotLocalDomain: u32 = 2320u32;
pub const NERR_NotLocalName: u32 = 2285u32;
pub const NERR_NotLoggedOn: u32 = 2201u32;
pub const NERR_NotPrimary: u32 = 2226u32;
pub const NERR_OpenFiles: u32 = 2401u32;
pub const NERR_PasswordCantChange: u32 = 2243u32;
pub const NERR_PasswordExpired: u32 = 2242u32;
pub const NERR_PasswordFilterError: u32 = 2705u32;
pub const NERR_PasswordHistConflict: u32 = 2244u32;
pub const NERR_PasswordMismatch: u32 = 2458u32;
pub const NERR_PasswordMustChange: u32 = 2701u32;
pub const NERR_PasswordNotComplexEnough: u32 = 2704u32;
pub const NERR_PasswordTooLong: u32 = 2703u32;
pub const NERR_PasswordTooRecent: u32 = 2246u32;
pub const NERR_PasswordTooShort: u32 = 2245u32;
pub const NERR_PausedRemote: u32 = 2281u32;
pub const NERR_PersonalSku: u32 = 2698u32;
pub const NERR_PlainTextSecretsRequired: u32 = 2726u32;
pub const NERR_ProcNoRespond: u32 = 2160u32;
pub const NERR_ProcNotFound: u32 = 2168u32;
pub const NERR_ProfileCleanup: u32 = 2372u32;
pub const NERR_ProfileFileTooBig: u32 = 2370u32;
pub const NERR_ProfileLoadErr: u32 = 2374u32;
pub const NERR_ProfileOffset: u32 = 2371u32;
pub const NERR_ProfileSaveErr: u32 = 2375u32;
pub const NERR_ProfileUnknownCmd: u32 = 2373u32;
pub const NERR_ProgNeedsExtraMem: u32 = 2501u32;
pub const NERR_ProvisioningBlobUnsupported: u32 = 2719u32;
pub const NERR_QExists: u32 = 2154u32;
pub const NERR_QInvalidState: u32 = 2163u32;
pub const NERR_QNoRoom: u32 = 2155u32;
pub const NERR_QNotFound: u32 = 2150u32;
pub const NERR_QueueNotFound: u32 = 2338u32;
pub const NERR_RPL_CONNECTED: u32 = 2519u32;
pub const NERR_RedirectedPath: u32 = 2117u32;
pub const NERR_RemoteBootFailed: u32 = 2503u32;
pub const NERR_RemoteErr: u32 = 2127u32;
pub const NERR_RemoteFull: u32 = 2287u32;
pub const NERR_RemoteOnly: u32 = 2106u32;
pub const NERR_ResourceExists: u32 = 2225u32;
pub const NERR_ResourceNotFound: u32 = 2222u32;
pub const NERR_RplAdapterInfoCorrupted: u32 = 2625u32;
pub const NERR_RplAdapterNameUnavailable: u32 = 2633u32;
pub const NERR_RplAdapterNotFound: u32 = 2637u32;
pub const NERR_RplBackupDatabase: u32 = 2636u32;
pub const NERR_RplBadDatabase: u32 = 2612u32;
pub const NERR_RplBadRegistry: u32 = 2611u32;
pub const NERR_RplBootInUse: u32 = 2635u32;
pub const NERR_RplBootInfoCorrupted: u32 = 2628u32;
pub const NERR_RplBootNameUnavailable: u32 = 2640u32;
pub const NERR_RplBootNotFound: u32 = 2631u32;
pub const NERR_RplBootRestart: u32 = 2511u32;
pub const NERR_RplBootServiceTerm: u32 = 2517u32;
pub const NERR_RplBootStartFailed: u32 = 2518u32;
pub const NERR_RplCannotEnum: u32 = 2615u32;
pub const NERR_RplConfigInfoCorrupted: u32 = 2623u32;
pub const NERR_RplConfigNameUnavailable: u32 = 2641u32;
pub const NERR_RplConfigNotEmpty: u32 = 2634u32;
pub const NERR_RplConfigNotFound: u32 = 2624u32;
pub const NERR_RplIncompatibleProfile: u32 = 2632u32;
pub const NERR_RplInternal: u32 = 2626u32;
pub const NERR_RplLoadrDiskErr: u32 = 2507u32;
pub const NERR_RplLoadrNetBiosErr: u32 = 2506u32;
pub const NERR_RplNeedsRPLUSERAcct: u32 = 2630u32;
pub const NERR_RplNoAdaptersStarted: u32 = 2610u32;
pub const NERR_RplNotRplServer: u32 = 2614u32;
pub const NERR_RplProfileInfoCorrupted: u32 = 2619u32;
pub const NERR_RplProfileNameUnavailable: u32 = 2621u32;
pub const NERR_RplProfileNotEmpty: u32 = 2622u32;
pub const NERR_RplProfileNotFound: u32 = 2620u32;
pub const NERR_RplRplfilesShare: u32 = 2613u32;
pub const NERR_RplSrvrCallFailed: u32 = 2512u32;
pub const NERR_RplVendorInfoCorrupted: u32 = 2627u32;
pub const NERR_RplVendorNameUnavailable: u32 = 2639u32;
pub const NERR_RplVendorNotFound: u32 = 2638u32;
pub const NERR_RplWkstaInfoCorrupted: u32 = 2616u32;
pub const NERR_RplWkstaNameUnavailable: u32 = 2618u32;
pub const NERR_RplWkstaNeedsUserAcct: u32 = 2629u32;
pub const NERR_RplWkstaNotFound: u32 = 2617u32;
pub const NERR_RunSrvPaused: u32 = 2385u32;
pub const NERR_SameAsComputerName: u32 = 2253u32;
pub const NERR_ServerNotStarted: u32 = 2114u32;
pub const NERR_ServiceCtlBusy: u32 = 2187u32;
pub const NERR_ServiceCtlNotValid: u32 = 2191u32;
pub const NERR_ServiceCtlTimeout: u32 = 2186u32;
pub const NERR_ServiceEntryLocked: u32 = 2183u32;
pub const NERR_ServiceInstalled: u32 = 2182u32;
pub const NERR_ServiceKillProc: u32 = 2190u32;
pub const NERR_ServiceNotCtrl: u32 = 2189u32;
pub const NERR_ServiceNotInstalled: u32 = 2184u32;
pub const NERR_ServiceNotStarting: u32 = 2194u32;
pub const NERR_ServiceTableFull: u32 = 2181u32;
pub const NERR_ServiceTableLocked: u32 = 2180u32;
pub const NERR_SetupAlreadyJoined: u32 = 2691u32;
pub const NERR_SetupCheckDNSConfig: u32 = 2699u32;
pub const NERR_SetupDomainController: u32 = 2693u32;
pub const NERR_SetupNotJoined: u32 = 2692u32;
pub const NERR_ShareMem: u32 = 2104u32;
pub const NERR_ShareNotFound: u32 = 2392u32;
pub const NERR_SourceIsDir: u32 = 2380u32;
pub const NERR_SpeGroupOp: u32 = 2234u32;
pub const NERR_SpoolNoMemory: u32 = 2165u32;
pub const NERR_SpoolerNotLoaded: u32 = 2161u32;
pub const NERR_StandaloneLogon: u32 = 2214u32;
pub const NERR_StartingRplBoot: u32 = 2516u32;
pub const NERR_Success: u32 = 0u32;
pub const NERR_SyncRequired: u32 = 2249u32;
pub const NERR_TargetVersionUnsupported: u32 = 2723u32;
pub const NERR_TimeDiffAtDC: u32 = 2457u32;
pub const NERR_TmpFile: u32 = 2316u32;
pub const NERR_TooManyAlerts: u32 = 2431u32;
pub const NERR_TooManyConnections: u32 = 2465u32;
pub const NERR_TooManyEntries: u32 = 2362u32;
pub const NERR_TooManyFiles: u32 = 2466u32;
pub const NERR_TooManyHostNames: u32 = 2731u32;
pub const NERR_TooManyImageParams: u32 = 2509u32;
pub const NERR_TooManyItems: u32 = 2121u32;
pub const NERR_TooManyNames: u32 = 2277u32;
pub const NERR_TooManyServers: u32 = 2463u32;
pub const NERR_TooManySessions: u32 = 2464u32;
pub const NERR_TooMuchData: u32 = 2317u32;
pub const NERR_TruncatedBroadcast: u32 = 2289u32;
pub const NERR_TryDownLevel: u32 = 2470u32;
pub const NERR_UPSDriverNotStarted: u32 = 2480u32;
pub const NERR_UPSInvalidCommPort: u32 = 2482u32;
pub const NERR_UPSInvalidConfig: u32 = 2481u32;
pub const NERR_UPSShutdownFailed: u32 = 2484u32;
pub const NERR_UPSSignalAsserted: u32 = 2483u32;
pub const NERR_UnableToAddName_F: u32 = 2205u32;
pub const NERR_UnableToAddName_W: u32 = 2204u32;
pub const NERR_UnableToDelName_F: u32 = 2207u32;
pub const NERR_UnableToDelName_W: u32 = 2206u32;
pub const NERR_UnknownDevDir: u32 = 2116u32;
pub const NERR_UnknownServer: u32 = 2103u32;
pub const NERR_UseNotFound: u32 = 2250u32;
pub const NERR_UserExists: u32 = 2224u32;
pub const NERR_UserInGroup: u32 = 2236u32;
pub const NERR_UserLogon: u32 = 2231u32;
pub const NERR_UserNotFound: u32 = 2221u32;
pub const NERR_UserNotInGroup: u32 = 2237u32;
pub const NERR_ValuesNotSet: u32 = 2715u32;
pub const NERR_WkstaInconsistentState: u32 = 2137u32;
pub const NERR_WkstaNotStarted: u32 = 2138u32;
pub const NERR_WriteFault: u32 = 2295u32;
pub const NETBIOS_NAME_LEN: u32 = 16u32;
pub const NETCFG_CLIENT_CID_MS_MSClient: windows_core::PCWSTR = windows_core::w!("ms_msclient");
pub const NETCFG_E_ACTIVE_RAS_CONNECTIONS: windows_core::HRESULT = windows_core::HRESULT(0x8004A026_u32 as _);
pub const NETCFG_E_ADAPTER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8004A027_u32 as _);
pub const NETCFG_E_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004A020_u32 as _);
pub const NETCFG_E_COMPONENT_REMOVED_PENDING_REBOOT: windows_core::HRESULT = windows_core::HRESULT(0x8004A028_u32 as _);
pub const NETCFG_E_DUPLICATE_INSTANCEID: windows_core::HRESULT = windows_core::HRESULT(0x8004A02B_u32 as _);
pub const NETCFG_E_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8004A022_u32 as _);
pub const NETCFG_E_MAX_FILTER_LIMIT: windows_core::HRESULT = windows_core::HRESULT(0x8004A029_u32 as _);
pub const NETCFG_E_NEED_REBOOT: windows_core::HRESULT = windows_core::HRESULT(0x8004A025_u32 as _);
pub const NETCFG_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8004A021_u32 as _);
pub const NETCFG_E_NO_WRITE_LOCK: windows_core::HRESULT = windows_core::HRESULT(0x8004A024_u32 as _);
pub const NETCFG_E_VMSWITCH_ACTIVE_OVER_ADAPTER: windows_core::HRESULT = windows_core::HRESULT(0x8004A02A_u32 as _);
pub const NETCFG_SERVICE_CID_MS_NETBIOS: windows_core::PCWSTR = windows_core::w!("ms_netbios");
pub const NETCFG_SERVICE_CID_MS_PSCHED: windows_core::PCWSTR = windows_core::w!("ms_pschedpc");
pub const NETCFG_SERVICE_CID_MS_SERVER: windows_core::PCWSTR = windows_core::w!("ms_server");
pub const NETCFG_SERVICE_CID_MS_WLBS: windows_core::PCWSTR = windows_core::w!("ms_wlbs");
pub const NETCFG_S_CAUSED_SETUP_CHANGE: windows_core::HRESULT = windows_core::HRESULT(0x4A024_u32 as _);
pub const NETCFG_S_COMMIT_NOW: windows_core::HRESULT = windows_core::HRESULT(0x4A025_u32 as _);
pub const NETCFG_S_DISABLE_QUERY: windows_core::HRESULT = windows_core::HRESULT(0x4A022_u32 as _);
pub const NETCFG_S_REBOOT: windows_core::HRESULT = windows_core::HRESULT(0x4A020_u32 as _);
pub const NETCFG_S_STILL_REFERENCED: windows_core::HRESULT = windows_core::HRESULT(0x4A023_u32 as _);
pub const NETCFG_TRANS_CID_MS_APPLETALK: windows_core::PCWSTR = windows_core::w!("ms_appletalk");
pub const NETCFG_TRANS_CID_MS_NETBEUI: windows_core::PCWSTR = windows_core::w!("ms_netbeui");
pub const NETCFG_TRANS_CID_MS_NETMON: windows_core::PCWSTR = windows_core::w!("ms_netmon");
pub const NETCFG_TRANS_CID_MS_NWIPX: windows_core::PCWSTR = windows_core::w!("ms_nwipx");
pub const NETCFG_TRANS_CID_MS_NWSPX: windows_core::PCWSTR = windows_core::w!("ms_nwspx");
pub const NETCFG_TRANS_CID_MS_TCPIP: windows_core::PCWSTR = windows_core::w!("ms_tcpip");
pub const NETLOGON_CONTROL_BACKUP_CHANGE_LOG: u32 = 65532u32;
pub const NETLOGON_CONTROL_BREAKPOINT: u32 = 65535u32;
pub const NETLOGON_CONTROL_CHANGE_PASSWORD: u32 = 9u32;
pub const NETLOGON_CONTROL_FIND_USER: u32 = 8u32;
pub const NETLOGON_CONTROL_FORCE_DNS_REG: u32 = 11u32;
pub const NETLOGON_CONTROL_PDC_REPLICATE: u32 = 4u32;
pub const NETLOGON_CONTROL_QUERY: u32 = 1u32;
pub const NETLOGON_CONTROL_QUERY_DNS_REG: u32 = 12u32;
pub const NETLOGON_CONTROL_QUERY_ENC_TYPES: u32 = 13u32;
pub const NETLOGON_CONTROL_REDISCOVER: u32 = 5u32;
pub const NETLOGON_CONTROL_REPLICATE: u32 = 2u32;
pub const NETLOGON_CONTROL_SET_DBFLAG: u32 = 65534u32;
pub const NETLOGON_CONTROL_SYNCHRONIZE: u32 = 3u32;
pub const NETLOGON_CONTROL_TC_QUERY: u32 = 6u32;
pub const NETLOGON_CONTROL_TC_VERIFY: u32 = 10u32;
pub const NETLOGON_CONTROL_TRANSPORT_NOTIFY: u32 = 7u32;
pub const NETLOGON_CONTROL_TRUNCATE_LOG: u32 = 65533u32;
pub const NETLOGON_CONTROL_UNLOAD_NETLOGON_DLL: u32 = 65531u32;
pub const NETLOGON_DNS_UPDATE_FAILURE: u32 = 64u32;
pub const NETLOGON_FULL_SYNC_REPLICATION: u32 = 4u32;
pub const NETLOGON_HAS_IP: u32 = 16u32;
pub const NETLOGON_HAS_TIMESERV: u32 = 32u32;
pub const NETLOGON_REDO_NEEDED: u32 = 8u32;
pub const NETLOGON_REPLICATION_IN_PROGRESS: u32 = 2u32;
pub const NETLOGON_REPLICATION_NEEDED: u32 = 1u32;
pub const NETLOGON_VERIFY_STATUS_RETURNED: u32 = 128u32;
pub const NETLOG_NetlogonNonWindowsSupportsSecureRpc: u32 = 5826u32;
pub const NETLOG_NetlogonRc4Allowed: u32 = 5840u32;
pub const NETLOG_NetlogonRc4Denied: u32 = 5841u32;
pub const NETLOG_NetlogonRpcBacklogLimitFailure: u32 = 5837u32;
pub const NETLOG_NetlogonRpcBacklogLimitSet: u32 = 5836u32;
pub const NETLOG_NetlogonRpcSigningClient: u32 = 5838u32;
pub const NETLOG_NetlogonRpcSigningTrust: u32 = 5839u32;
pub const NETLOG_NetlogonUnsecureRpcClient: u32 = 5827u32;
pub const NETLOG_NetlogonUnsecureRpcMachineAllowedBySsdl: u32 = 5830u32;
pub const NETLOG_NetlogonUnsecureRpcTrust: u32 = 5828u32;
pub const NETLOG_NetlogonUnsecureRpcTrustAllowedBySsdl: u32 = 5831u32;
pub const NETLOG_NetlogonUnsecuredRpcMachineTemporarilyAllowed: u32 = 5829u32;
pub const NETLOG_PassThruFilterError_Request_AdminOverride: u32 = 5834u32;
pub const NETLOG_PassThruFilterError_Request_Blocked: u32 = 5835u32;
pub const NETLOG_PassThruFilterError_Summary_AdminOverride: u32 = 5832u32;
pub const NETLOG_PassThruFilterError_Summary_Blocked: u32 = 5833u32;
pub const NETMAN_VARTYPE_HARDWARE_ADDRESS: u32 = 1u32;
pub const NETMAN_VARTYPE_STRING: u32 = 2u32;
pub const NETMAN_VARTYPE_ULONG: u32 = 0u32;
pub const NETSETUP_ACCT_CREATE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2u32);
pub const NETSETUP_ACCT_DELETE: u32 = 4u32;
pub const NETSETUP_ALT_SAMACCOUNTNAME: u32 = 131072u32;
pub const NETSETUP_AMBIGUOUS_DC: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(4096u32);
pub const NETSETUP_DEFER_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(256u32);
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: u32 = 4096u32;
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32u32);
pub const NETSETUP_DONT_CONTROL_SERVICES: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16384u32);
pub const NETSETUP_FORCE_SPN_SET: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(65536u32);
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(268435456u32);
pub const NETSETUP_INSTALL_INVOCATION: u32 = 262144u32;
pub const NETSETUP_JOIN_DC_ACCOUNT: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(512u32);
pub const NETSETUP_JOIN_DOMAIN: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1u32);
pub const NETSETUP_JOIN_READONLY: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(2048u32);
pub const NETSETUP_JOIN_UNSECURE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(64u32);
pub const NETSETUP_JOIN_WITH_NEW_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(1024u32);
pub const NETSETUP_MACHINE_PWD_PASSED: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(128u32);
pub const NETSETUP_NO_ACCT_REUSE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(131072u32);
pub const NETSETUP_NO_NETLOGON_CACHE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(8192u32);
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: u32 = 2u32;
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: u32 = 1u32;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648u32;
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: NETSETUP_PROVISION = NETSETUP_PROVISION(1u32);
pub const NETSETUP_PROVISION_ONLINE_CALLER: NET_REQUEST_PROVISION_OPTIONS = NET_REQUEST_PROVISION_OPTIONS(1073741824u32);
pub const NETSETUP_PROVISION_PERSISTENTSITE: u32 = 32u32;
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: NETSETUP_PROVISION = NETSETUP_PROVISION(2u32);
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: NETSETUP_PROVISION = NETSETUP_PROVISION(16u32);
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: NETSETUP_PROVISION = NETSETUP_PROVISION(8u32);
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: NETSETUP_PROVISION = NETSETUP_PROVISION(4u32);
pub const NETSETUP_SET_MACHINE_NAME: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(32768u32);
pub const NETSETUP_WIN9X_UPGRADE: NET_JOIN_DOMAIN_JOIN_OPTIONS = NET_JOIN_DOMAIN_JOIN_OPTIONS(16u32);
pub const NET_DFS_ENUM: i32 = 1073756324i32;
pub const NET_DFS_ENUMEX: i32 = 1073756325i32;
pub const NET_IGNORE_UNSUPPORTED_FLAGS: u32 = 1u32;
pub const NET_VALIDATE_BAD_PASSWORD_COUNT: u32 = 8u32;
pub const NET_VALIDATE_BAD_PASSWORD_TIME: u32 = 2u32;
pub const NET_VALIDATE_LOCKOUT_TIME: u32 = 4u32;
pub const NET_VALIDATE_PASSWORD_HISTORY: u32 = 32u32;
pub const NET_VALIDATE_PASSWORD_HISTORY_LENGTH: u32 = 16u32;
pub const NET_VALIDATE_PASSWORD_LAST_SET: u32 = 1u32;
pub const NON_VALIDATED_LOGON: u32 = 3u32;
pub const NOT_A_DFS_PATH: i32 = 1073756224i32;
pub const NO_PERMISSION_REQUIRED: u32 = 1u32;
pub const NSF_COMPONENT_UPDATE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(512i32);
pub const NSF_POSTSYSINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(2i32);
pub const NSF_PRIMARYINSTALL: NETWORK_INSTALL_TIME = NETWORK_INSTALL_TIME(1i32);
pub const NSF_WIN16_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(16i32);
pub const NSF_WIN95_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(32i32);
pub const NSF_WINNT_SBS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(256i32);
pub const NSF_WINNT_SVR_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(128i32);
pub const NSF_WINNT_WKS_UPGRADE: NETWORK_UPGRADE_TYPE = NETWORK_UPGRADE_TYPE(64i32);
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_CONN: i32 = -1073728292i32;
pub const NTFRSPRF_COLLECT_RPC_BINDING_ERROR_SET: i32 = -1073728293i32;
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_CONN: i32 = -1073728290i32;
pub const NTFRSPRF_COLLECT_RPC_CALL_ERROR_SET: i32 = -1073728291i32;
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_CONN: i32 = -1073728296i32;
pub const NTFRSPRF_OPEN_RPC_BINDING_ERROR_SET: i32 = -1073728297i32;
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_CONN: i32 = -1073728294i32;
pub const NTFRSPRF_OPEN_RPC_CALL_ERROR_SET: i32 = -1073728295i32;
pub const NTFRSPRF_REGISTRY_ERROR_CONN: i32 = -1073728286i32;
pub const NTFRSPRF_REGISTRY_ERROR_SET: i32 = -1073728287i32;
pub const NTFRSPRF_VIRTUALALLOC_ERROR_CONN: i32 = -1073728288i32;
pub const NTFRSPRF_VIRTUALALLOC_ERROR_SET: i32 = -1073728289i32;
pub const NULL_USERSETINFO_PASSWD: windows_core::PCSTR = windows_core::s!("              ");
pub const NWSAP_DISPLAY_NAME: windows_core::PCWSTR = windows_core::w!("NW Sap Agent");
pub const NWSAP_EVENT_BADWANFILTER_VALUE: i32 = -1073733302i32;
pub const NWSAP_EVENT_BIND_FAILED: i32 = -1073733320i32;
pub const NWSAP_EVENT_CARDLISTEVENT_FAIL: i32 = -1073733301i32;
pub const NWSAP_EVENT_CARDMALLOC_FAILED: i32 = -1073733316i32;
pub const NWSAP_EVENT_CREATELPCEVENT_ERROR: i32 = -1073733305i32;
pub const NWSAP_EVENT_CREATELPCPORT_ERROR: i32 = -1073733306i32;
pub const NWSAP_EVENT_GETSOCKNAME_FAILED: i32 = -1073733319i32;
pub const NWSAP_EVENT_HASHTABLE_MALLOC_FAILED: i32 = -1073733308i32;
pub const NWSAP_EVENT_INVALID_FILTERNAME: i32 = -2147475123i32;
pub const NWSAP_EVENT_KEY_NOT_FOUND: i32 = -1073733324i32;
pub const NWSAP_EVENT_LPCHANDLEMEMORY_ERROR: i32 = -1073733303i32;
pub const NWSAP_EVENT_LPCLISTENMEMORY_ERROR: i32 = -1073733304i32;
pub const NWSAP_EVENT_NOCARDS: i32 = -1073733315i32;
pub const NWSAP_EVENT_OPTBCASTINADDR_FAILED: i32 = -1073733317i32;
pub const NWSAP_EVENT_OPTEXTENDEDADDR_FAILED: i32 = -1073733318i32;
pub const NWSAP_EVENT_OPTMAXADAPTERNUM_ERROR: i32 = -1073733293i32;
pub const NWSAP_EVENT_RECVSEM_FAIL: i32 = -1073733313i32;
pub const NWSAP_EVENT_SDMDEVENT_FAIL: i32 = -1073733300i32;
pub const NWSAP_EVENT_SENDEVENT_FAIL: i32 = -1073733312i32;
pub const NWSAP_EVENT_SETOPTBCAST_FAILED: i32 = -1073733321i32;
pub const NWSAP_EVENT_SOCKET_FAILED: i32 = -1073733322i32;
pub const NWSAP_EVENT_STARTLPCWORKER_ERROR: i32 = -1073733307i32;
pub const NWSAP_EVENT_STARTRECEIVE_ERROR: i32 = -1073733311i32;
pub const NWSAP_EVENT_STARTWANCHECK_ERROR: i32 = -1073733294i32;
pub const NWSAP_EVENT_STARTWANWORKER_ERROR: i32 = -1073733295i32;
pub const NWSAP_EVENT_STARTWORKER_ERROR: i32 = -1073733310i32;
pub const NWSAP_EVENT_TABLE_MALLOC_FAILED: i32 = -1073733309i32;
pub const NWSAP_EVENT_THREADEVENT_FAIL: i32 = -1073733314i32;
pub const NWSAP_EVENT_WANBIND_FAILED: i32 = -1073733296i32;
pub const NWSAP_EVENT_WANEVENT_ERROR: i32 = -1073733291i32;
pub const NWSAP_EVENT_WANHANDLEMEMORY_ERROR: i32 = -1073733292i32;
pub const NWSAP_EVENT_WANSEM_FAIL: i32 = -1073733298i32;
pub const NWSAP_EVENT_WANSOCKET_FAILED: i32 = -1073733297i32;
pub const NWSAP_EVENT_WSASTARTUP_FAILED: i32 = -1073733323i32;
pub const NetAllComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(2i32);
pub const NetAlternateComputerNames: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(1i32);
pub const NetComputerNameTypeMax: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(3i32);
pub const NetPrimaryComputerName: NET_COMPUTER_NAME_TYPE = NET_COMPUTER_NAME_TYPE(0i32);
pub const NetSetupDnsMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(5i32);
pub const NetSetupDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(3i32);
pub const NetSetupDomainName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(3i32);
pub const NetSetupMachine: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(1i32);
pub const NetSetupNonExistentDomain: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(4i32);
pub const NetSetupUnjoined: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(1i32);
pub const NetSetupUnknown: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(0i32);
pub const NetSetupUnknownStatus: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(0i32);
pub const NetSetupWorkgroup: NETSETUP_NAME_TYPE = NETSETUP_NAME_TYPE(2i32);
pub const NetSetupWorkgroupName: NETSETUP_JOIN_STATUS = NETSETUP_JOIN_STATUS(2i32);
pub const NetValidateAuthentication: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(1i32);
pub const NetValidatePasswordChange: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(2i32);
pub const NetValidatePasswordReset: NET_VALIDATE_PASSWORD_TYPE = NET_VALIDATE_PASSWORD_TYPE(3i32);
pub const OBO_COMPONENT: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(2i32);
pub const OBO_SOFTWARE: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(3i32);
pub const OBO_USER: OBO_TOKEN_TYPE = OBO_TOKEN_TYPE(1i32);
pub const OS2MSG_FILENAME: windows_core::PCWSTR = windows_core::w!("BASE");
pub const PARMNUM_ALL: u32 = 0u32;
pub const PARMNUM_BASE_INFOLEVEL: u32 = 1000u32;
pub const PARM_ERROR_NONE: u32 = 0u32;
pub const PARM_ERROR_UNKNOWN: u32 = 4294967295u32;
pub const PASSWORD_EXPIRED: u32 = 2u32;
pub const PATHLEN: u32 = 256u32;
pub const PLATFORM_ID_DOS: u32 = 300u32;
pub const PLATFORM_ID_NT: u32 = 500u32;
pub const PLATFORM_ID_OS2: u32 = 400u32;
pub const PLATFORM_ID_OSF: u32 = 600u32;
pub const PLATFORM_ID_VMS: u32 = 700u32;
pub const PREFIX_MISMATCH: i32 = -1073727510i32;
pub const PREFIX_MISMATCH_FIXED: i32 = -1073727509i32;
pub const PREFIX_MISMATCH_NOT_FIXED: i32 = -1073727508i32;
pub const PRJOB_COMPLETE: u32 = 4u32;
pub const PRJOB_DELETED: u32 = 32768u32;
pub const PRJOB_DESTNOPAPER: u32 = 256u32;
pub const PRJOB_DESTOFFLINE: u32 = 32u32;
pub const PRJOB_DESTPAUSED: u32 = 64u32;
pub const PRJOB_DEVSTATUS: u32 = 508u32;
pub const PRJOB_ERROR: u32 = 16u32;
pub const PRJOB_INTERV: u32 = 8u32;
pub const PRJOB_NOTIFY: u32 = 128u32;
pub const PRJOB_QSTATUS: u32 = 3u32;
pub const PRJOB_QS_PAUSED: u32 = 1u32;
pub const PRJOB_QS_PRINTING: u32 = 3u32;
pub const PRJOB_QS_QUEUED: u32 = 0u32;
pub const PRJOB_QS_SPOOLING: u32 = 2u32;
pub const PROTO_IPV6_DHCP: u32 = 999u32;
pub const PROTO_IP_ALG: u32 = 10010u32;
pub const PROTO_IP_BGMP: u32 = 11u32;
pub const PROTO_IP_BOOTP: u32 = 9999u32;
pub const PROTO_IP_DHCP_ALLOCATOR: u32 = 10004u32;
pub const PROTO_IP_DIFFSERV: u32 = 10008u32;
pub const PROTO_IP_DNS_PROXY: u32 = 10003u32;
pub const PROTO_IP_DTP: u32 = 10013u32;
pub const PROTO_IP_FTP: u32 = 10012u32;
pub const PROTO_IP_H323: u32 = 10011u32;
pub const PROTO_IP_IGMP: u32 = 10u32;
pub const PROTO_IP_MGM: u32 = 10009u32;
pub const PROTO_IP_MSDP: u32 = 9u32;
pub const PROTO_IP_NAT: u32 = 10005u32;
pub const PROTO_IP_VRRP: u32 = 112u32;
pub const PROTO_TYPE_MCAST: u32 = 1u32;
pub const PROTO_TYPE_MS0: u32 = 2u32;
pub const PROTO_TYPE_MS1: u32 = 3u32;
pub const PROTO_TYPE_UCAST: u32 = 0u32;
pub const PROTO_VENDOR_MS0: u32 = 0u32;
pub const PROTO_VENDOR_MS1: u32 = 311u32;
pub const PROTO_VENDOR_MS2: u32 = 16383u32;
pub const PWLEN: u32 = 256u32;
pub const QNLEN: u32 = 80u32;
pub const RCUIF_DEMAND_DIAL: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(2i32);
pub const RCUIF_DISABLE_CLASS_BASED_ROUTE: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(32768i32);
pub const RCUIF_ENABLE_NBT: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(1024i32);
pub const RCUIF_NOT_ADMIN: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(4i32);
pub const RCUIF_USE_DISABLE_REGISTER_DNS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(256i32);
pub const RCUIF_USE_HEADER_COMPRESSION: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(128i32);
pub const RCUIF_USE_IPv4_EXPLICIT_METRIC: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(64i32);
pub const RCUIF_USE_IPv4_NAME_SERVERS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(16i32);
pub const RCUIF_USE_IPv4_REMOTE_GATEWAY: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(32i32);
pub const RCUIF_USE_IPv4_STATICADDRESS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(8i32);
pub const RCUIF_USE_IPv6_EXPLICIT_METRIC: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(16384i32);
pub const RCUIF_USE_IPv6_NAME_SERVERS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(4096i32);
pub const RCUIF_USE_IPv6_REMOTE_GATEWAY: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(8192i32);
pub const RCUIF_USE_IPv6_STATICADDRESS: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(2048i32);
pub const RCUIF_USE_PRIVATE_DNS_SUFFIX: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(512i32);
pub const RCUIF_VPN: RASCON_UIINFO_FLAGS = RASCON_UIINFO_FLAGS(1i32);
pub const REGISTER_PROTOCOL_ENTRY_POINT_STRING: windows_core::PCSTR = windows_core::s!("RegisterProtocol");
pub const REPL_EXPORT_EXTENT_INFOLEVEL: u32 = 1001u32;
pub const REPL_EXPORT_INTEGRITY_INFOLEVEL: u32 = 1000u32;
pub const REPL_EXTENT_FILE: u32 = 1u32;
pub const REPL_EXTENT_TREE: u32 = 2u32;
pub const REPL_GUARDTIME_INFOLEVEL: u32 = 1002u32;
pub const REPL_INTEGRITY_FILE: u32 = 1u32;
pub const REPL_INTEGRITY_TREE: u32 = 2u32;
pub const REPL_INTERVAL_INFOLEVEL: u32 = 1000u32;
pub const REPL_PULSE_INFOLEVEL: u32 = 1001u32;
pub const REPL_RANDOM_INFOLEVEL: u32 = 1003u32;
pub const REPL_ROLE_BOTH: u32 = 3u32;
pub const REPL_ROLE_EXPORT: u32 = 1u32;
pub const REPL_ROLE_IMPORT: u32 = 2u32;
pub const REPL_STATE_NEVER_REPLICATED: u32 = 3u32;
pub const REPL_STATE_NO_MASTER: u32 = 1u32;
pub const REPL_STATE_NO_SYNC: u32 = 2u32;
pub const REPL_STATE_OK: u32 = 0u32;
pub const REPL_UNLOCK_FORCE: u32 = 1u32;
pub const REPL_UNLOCK_NOFORCE: u32 = 0u32;
pub const RF_ADD_ALL_INTERFACES: u32 = 16u32;
pub const RF_DEMAND_UPDATE_ROUTES: u32 = 4u32;
pub const RF_MULTICAST: u32 = 32u32;
pub const RF_POWER: u32 = 64u32;
pub const RF_ROUTING: u32 = 1u32;
pub const RF_ROUTINGV6: u32 = 2u32;
pub const RIS_INTERFACE_ADDRESS_CHANGE: u32 = 0u32;
pub const RIS_INTERFACE_DISABLED: u32 = 2u32;
pub const RIS_INTERFACE_ENABLED: u32 = 1u32;
pub const RIS_INTERFACE_MEDIA_ABSENT: u32 = 4u32;
pub const RIS_INTERFACE_MEDIA_PRESENT: u32 = 3u32;
pub const ROUTING_DOMAIN_INFO_REVISION_1: u32 = 1u32;
pub const RTR_INFO_BLOCK_VERSION: u32 = 1u32;
pub const RTUTILS_MAX_PROTOCOL_DLL_LEN: u32 = 48u32;
pub const RTUTILS_MAX_PROTOCOL_NAME_LEN: u32 = 40u32;
pub const SERVCE_LM20_W32TIME: windows_core::PCWSTR = windows_core::w!("w32time");
pub const SERVER_DISPLAY_NAME: windows_core::PCWSTR = windows_core::w!("Server");
pub const SERVICE2_BASE: u32 = 5600u32;
pub const SERVICE_ACCOUNT_FLAG_ADD_AGAINST_RODC: i32 = 2i32;
pub const SERVICE_ACCOUNT_FLAG_LINK_TO_HOST_ONLY: i32 = 1i32;
pub const SERVICE_ACCOUNT_FLAG_REMOVE_OFFLINE: i32 = 2i32;
pub const SERVICE_ACCOUNT_FLAG_UNLINK_FROM_HOST_ONLY: i32 = 1i32;
pub const SERVICE_ACCOUNT_PASSWORD: windows_core::PCWSTR = windows_core::w!("_SA_{262E99C9-6160-4871-ACEC-4E61736B6F21}");
pub const SERVICE_ACCOUNT_SECRET_PREFIX: windows_core::PCWSTR = windows_core::w!("_SC_{262E99C9-6160-4871-ACEC-4E61736B6F21}_");
pub const SERVICE_ADWS: windows_core::PCWSTR = windows_core::w!("ADWS");
pub const SERVICE_AFP: windows_core::PCWSTR = windows_core::w!("AFP");
pub const SERVICE_ALERTER: windows_core::PCWSTR = windows_core::w!("ALERTER");
pub const SERVICE_BASE: u32 = 3050u32;
pub const SERVICE_BROWSER: windows_core::PCWSTR = windows_core::w!("BROWSER");
pub const SERVICE_CCP_CHKPT_NUM: u32 = 255u32;
pub const SERVICE_CCP_NO_HINT: u32 = 0u32;
pub const SERVICE_CCP_QUERY_HINT: u32 = 65536u32;
pub const SERVICE_CCP_WAIT_TIME: u32 = 65280u32;
pub const SERVICE_CTRL_CONTINUE: u32 = 2u32;
pub const SERVICE_CTRL_INTERROGATE: u32 = 0u32;
pub const SERVICE_CTRL_PAUSE: u32 = 1u32;
pub const SERVICE_CTRL_REDIR_COMM: u32 = 4u32;
pub const SERVICE_CTRL_REDIR_DISK: u32 = 1u32;
pub const SERVICE_CTRL_REDIR_PRINT: u32 = 2u32;
pub const SERVICE_CTRL_UNINSTALL: u32 = 3u32;
pub const SERVICE_DHCP: windows_core::PCWSTR = windows_core::w!("DHCP");
pub const SERVICE_DNS_CACHE: windows_core::PCWSTR = windows_core::w!("DnsCache");
pub const SERVICE_DOS_ENCRYPTION: windows_core::PCWSTR = windows_core::w!("ENCRYPT");
pub const SERVICE_DSROLE: windows_core::PCWSTR = windows_core::w!("DsRoleSvc");
pub const SERVICE_INSTALLED: u32 = 3u32;
pub const SERVICE_INSTALL_PENDING: u32 = 1u32;
pub const SERVICE_INSTALL_STATE: u32 = 3u32;
pub const SERVICE_IP_CHKPT_NUM: u32 = 255u32;
pub const SERVICE_IP_NO_HINT: u32 = 0u32;
pub const SERVICE_IP_QUERY_HINT: u32 = 65536u32;
pub const SERVICE_IP_WAITTIME_SHIFT: u32 = 8u32;
pub const SERVICE_IP_WAIT_TIME: u32 = 65280u32;
pub const SERVICE_ISMSERV: windows_core::PCWSTR = windows_core::w!("IsmServ");
pub const SERVICE_KDC: windows_core::PCWSTR = windows_core::w!("kdc");
pub const SERVICE_LM20_AFP: windows_core::PCWSTR = windows_core::w!("AFP");
pub const SERVICE_LM20_ALERTER: windows_core::PCWSTR = windows_core::w!("ALERTER");
pub const SERVICE_LM20_BROWSER: windows_core::PCWSTR = windows_core::w!("BROWSER");
pub const SERVICE_LM20_DHCP: windows_core::PCWSTR = windows_core::w!("DHCP");
pub const SERVICE_LM20_DSROLE: windows_core::PCWSTR = windows_core::w!("DsRoleSvc");
pub const SERVICE_LM20_ISMSERV: windows_core::PCWSTR = windows_core::w!("IsmServ");
pub const SERVICE_LM20_KDC: windows_core::PCWSTR = windows_core::w!("kdc");
pub const SERVICE_LM20_LMHOSTS: windows_core::PCWSTR = windows_core::w!("LMHOSTS");
pub const SERVICE_LM20_MESSENGER: windows_core::PCWSTR = windows_core::w!("MESSENGER");
pub const SERVICE_LM20_NBT: windows_core::PCWSTR = windows_core::w!("NBT");
pub const SERVICE_LM20_NETLOGON: windows_core::PCWSTR = windows_core::w!("NETLOGON");
pub const SERVICE_LM20_NETPOPUP: windows_core::PCWSTR = windows_core::w!("NETPOPUP");
pub const SERVICE_LM20_NETRUN: windows_core::PCWSTR = windows_core::w!("NETRUN");
pub const SERVICE_LM20_NTDS: windows_core::PCWSTR = windows_core::w!("NTDS");
pub const SERVICE_LM20_NTFRS: windows_core::PCWSTR = windows_core::w!("NtFrs");
pub const SERVICE_LM20_NWSAP: windows_core::PCWSTR = windows_core::w!("NwSapAgent");
pub const SERVICE_LM20_REPL: windows_core::PCWSTR = windows_core::w!("REPLICATOR");
pub const SERVICE_LM20_RIPL: windows_core::PCWSTR = windows_core::w!("REMOTEBOOT");
pub const SERVICE_LM20_RPCLOCATOR: windows_core::PCWSTR = windows_core::w!("RPCLOCATOR");
pub const SERVICE_LM20_SCHEDULE: windows_core::PCWSTR = windows_core::w!("Schedule");
pub const SERVICE_LM20_SERVER: windows_core::PCWSTR = windows_core::w!("SERVER");
pub const SERVICE_LM20_SPOOLER: windows_core::PCWSTR = windows_core::w!("SPOOLER");
pub const SERVICE_LM20_SQLSERVER: windows_core::PCWSTR = windows_core::w!("SQLSERVER");
pub const SERVICE_LM20_TCPIP: windows_core::PCWSTR = windows_core::w!("TCPIP");
pub const SERVICE_LM20_TELNET: windows_core::PCWSTR = windows_core::w!("Telnet");
pub const SERVICE_LM20_TIMESOURCE: windows_core::PCWSTR = windows_core::w!("TIMESOURCE");
pub const SERVICE_LM20_TRKSVR: windows_core::PCWSTR = windows_core::w!("TrkSvr");
pub const SERVICE_LM20_TRKWKS: windows_core::PCWSTR = windows_core::w!("TrkWks");
pub const SERVICE_LM20_UPS: windows_core::PCWSTR = windows_core::w!("UPS");
pub const SERVICE_LM20_WORKSTATION: windows_core::PCWSTR = windows_core::w!("WORKSTATION");
pub const SERVICE_LM20_XACTSRV: windows_core::PCWSTR = windows_core::w!("XACTSRV");
pub const SERVICE_LMHOSTS: windows_core::PCWSTR = windows_core::w!("LMHOSTS");
pub const SERVICE_MAXTIME: u32 = 255u32;
pub const SERVICE_MESSENGER: windows_core::PCWSTR = windows_core::w!("MESSENGER");
pub const SERVICE_NBT: windows_core::PCWSTR = windows_core::w!("NBT");
pub const SERVICE_NETLOGON: windows_core::PCWSTR = windows_core::w!("NETLOGON");
pub const SERVICE_NETPOPUP: windows_core::PCWSTR = windows_core::w!("NETPOPUP");
pub const SERVICE_NETRUN: windows_core::PCWSTR = windows_core::w!("NETRUN");
pub const SERVICE_NOT_PAUSABLE: u32 = 0u32;
pub const SERVICE_NOT_UNINSTALLABLE: u32 = 0u32;
pub const SERVICE_NTDS: windows_core::PCWSTR = windows_core::w!("NTDS");
pub const SERVICE_NTFRS: windows_core::PCWSTR = windows_core::w!("NtFrs");
pub const SERVICE_NTIP_WAITTIME_SHIFT: u32 = 12u32;
pub const SERVICE_NTLMSSP: windows_core::PCWSTR = windows_core::w!("NtLmSsp");
pub const SERVICE_NT_MAXTIME: u32 = 65535u32;
pub const SERVICE_NWCS: windows_core::PCWSTR = windows_core::w!("NWCWorkstation");
pub const SERVICE_NWSAP: windows_core::PCWSTR = windows_core::w!("NwSapAgent");
pub const SERVICE_PAUSABLE: u32 = 32u32;
pub const SERVICE_PAUSE_STATE: u32 = 12u32;
pub const SERVICE_REDIR_COMM_PAUSED: u32 = 1024u32;
pub const SERVICE_REDIR_DISK_PAUSED: u32 = 256u32;
pub const SERVICE_REDIR_PAUSED: u32 = 1792u32;
pub const SERVICE_REDIR_PRINT_PAUSED: u32 = 512u32;
pub const SERVICE_REPL: windows_core::PCWSTR = windows_core::w!("REPLICATOR");
pub const SERVICE_RESRV_MASK: u32 = 131071u32;
pub const SERVICE_RIPL: windows_core::PCWSTR = windows_core::w!("REMOTEBOOT");
pub const SERVICE_RPCLOCATOR: windows_core::PCWSTR = windows_core::w!("RPCLOCATOR");
pub const SERVICE_SCHEDULE: windows_core::PCWSTR = windows_core::w!("Schedule");
pub const SERVICE_SERVER: windows_core::PCWSTR = windows_core::w!("LanmanServer");
pub const SERVICE_SPOOLER: windows_core::PCWSTR = windows_core::w!("SPOOLER");
pub const SERVICE_SQLSERVER: windows_core::PCWSTR = windows_core::w!("SQLSERVER");
pub const SERVICE_TCPIP: windows_core::PCWSTR = windows_core::w!("TCPIP");
pub const SERVICE_TELNET: windows_core::PCWSTR = windows_core::w!("Telnet");
pub const SERVICE_TIMESOURCE: windows_core::PCWSTR = windows_core::w!("TIMESOURCE");
pub const SERVICE_TRKSVR: windows_core::PCWSTR = windows_core::w!("TrkSvr");
pub const SERVICE_TRKWKS: windows_core::PCWSTR = windows_core::w!("TrkWks");
pub const SERVICE_UIC_AMBIGPARM: u32 = 3058u32;
pub const SERVICE_UIC_BADPARMVAL: u32 = 3051u32;
pub const SERVICE_UIC_CONFIG: u32 = 3055u32;
pub const SERVICE_UIC_CONFLPARM: u32 = 3063u32;
pub const SERVICE_UIC_DUPPARM: u32 = 3059u32;
pub const SERVICE_UIC_EXEC: u32 = 3061u32;
pub const SERVICE_UIC_FILE: u32 = 3064u32;
pub const SERVICE_UIC_INTERNAL: u32 = 3057u32;
pub const SERVICE_UIC_KILL: u32 = 3060u32;
pub const SERVICE_UIC_MISSPARM: u32 = 3052u32;
pub const SERVICE_UIC_M_ADDPAK: u32 = 3090u32;
pub const SERVICE_UIC_M_ANNOUNCE: u32 = 3083u32;
pub const SERVICE_UIC_M_DATABASE_ERROR: u32 = 5602u32;
pub const SERVICE_UIC_M_DISK: u32 = 3071u32;
pub const SERVICE_UIC_M_ERRLOG: u32 = 3088u32;
pub const SERVICE_UIC_M_FILES: u32 = 3079u32;
pub const SERVICE_UIC_M_FILE_UW: u32 = 3089u32;
pub const SERVICE_UIC_M_LANGROUP: u32 = 3081u32;
pub const SERVICE_UIC_M_LANROOT: u32 = 3075u32;
pub const SERVICE_UIC_M_LAZY: u32 = 3091u32;
pub const SERVICE_UIC_M_LOGS: u32 = 3080u32;
pub const SERVICE_UIC_M_LSA_MACHINE_ACCT: u32 = 5601u32;
pub const SERVICE_UIC_M_MEMORY: u32 = 3070u32;
pub const SERVICE_UIC_M_MSGNAME: u32 = 3082u32;
pub const SERVICE_UIC_M_NETLOGON_AUTH: u32 = 3098u32;
pub const SERVICE_UIC_M_NETLOGON_DC_CFLCT: u32 = 3097u32;
pub const SERVICE_UIC_M_NETLOGON_MPATH: u32 = 5600u32;
pub const SERVICE_UIC_M_NETLOGON_NO_DC: u32 = 3096u32;
pub const SERVICE_UIC_M_NULL: u32 = 0u32;
pub const SERVICE_UIC_M_PROCESSES: u32 = 3073u32;
pub const SERVICE_UIC_M_REDIR: u32 = 3076u32;
pub const SERVICE_UIC_M_SECURITY: u32 = 3074u32;
pub const SERVICE_UIC_M_SEC_FILE_ERR: u32 = 3078u32;
pub const SERVICE_UIC_M_SERVER: u32 = 3077u32;
pub const SERVICE_UIC_M_SERVER_SEC_ERR: u32 = 3085u32;
pub const SERVICE_UIC_M_THREADS: u32 = 3072u32;
pub const SERVICE_UIC_M_UAS: u32 = 3084u32;
pub const SERVICE_UIC_M_UAS_INVALID_ROLE: u32 = 3095u32;
pub const SERVICE_UIC_M_UAS_MACHINE_ACCT: u32 = 3092u32;
pub const SERVICE_UIC_M_UAS_PROLOG: u32 = 3099u32;
pub const SERVICE_UIC_M_UAS_SERVERS_NMEMB: u32 = 3093u32;
pub const SERVICE_UIC_M_UAS_SERVERS_NOGRP: u32 = 3094u32;
pub const SERVICE_UIC_M_WKSTA: u32 = 3087u32;
pub const SERVICE_UIC_NORMAL: u32 = 0u32;
pub const SERVICE_UIC_RESOURCE: u32 = 3054u32;
pub const SERVICE_UIC_SUBSERV: u32 = 3062u32;
pub const SERVICE_UIC_SYSTEM: u32 = 3056u32;
pub const SERVICE_UIC_UNKPARM: u32 = 3053u32;
pub const SERVICE_UNINSTALLABLE: u32 = 16u32;
pub const SERVICE_UNINSTALLED: u32 = 0u32;
pub const SERVICE_UNINSTALL_PENDING: u32 = 2u32;
pub const SERVICE_UPS: windows_core::PCWSTR = windows_core::w!("UPS");
pub const SERVICE_W32TIME: windows_core::PCWSTR = windows_core::w!("w32time");
pub const SERVICE_WORKSTATION: windows_core::PCWSTR = windows_core::w!("LanmanWorkstation");
pub const SERVICE_XACTSRV: windows_core::PCWSTR = windows_core::w!("XACTSRV");
pub const SESSION_CRYPT_KLEN: u32 = 21u32;
pub const SESSION_PWLEN: u32 = 24u32;
pub const SHPWLEN: u32 = 8u32;
pub const SNLEN: u32 = 80u32;
pub const SRV_HASH_GENERATION_ACTIVE: u32 = 2u32;
pub const SRV_SUPPORT_HASH_GENERATION: u32 = 1u32;
pub const STXTLEN: u32 = 256u32;
pub const SUPPORTS_ANY: i32 = -1i32;
pub const SUPPORTS_LOCAL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(32u32);
pub const SUPPORTS_REMOTE_ADMIN_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(2u32);
pub const SUPPORTS_RPC: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(4u32);
pub const SUPPORTS_SAM_PROTOCOL: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(8u32);
pub const SUPPORTS_UNICODE: NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS = NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(16u32);
pub const SVAUD_BADNETLOGON: u32 = 384u32;
pub const SVAUD_BADSESSLOGON: u32 = 24u32;
pub const SVAUD_BADUSE: u32 = 6144u32;
pub const SVAUD_GOODNETLOGON: u32 = 96u32;
pub const SVAUD_GOODSESSLOGON: u32 = 6u32;
pub const SVAUD_GOODUSE: u32 = 1536u32;
pub const SVAUD_LOGONLIM: u32 = 65536u32;
pub const SVAUD_PERMISSIONS: u32 = 16384u32;
pub const SVAUD_RESOURCE: u32 = 32768u32;
pub const SVAUD_SERVICE: u32 = 1u32;
pub const SVAUD_USERLIST: u32 = 8192u32;
pub const SVI1_NUM_ELEMENTS: u32 = 5u32;
pub const SVI2_NUM_ELEMENTS: u32 = 40u32;
pub const SVI3_NUM_ELEMENTS: u32 = 44u32;
pub const SVTI2_CLUSTER_DNN_NAME: u32 = 16u32;
pub const SVTI2_CLUSTER_NAME: u32 = 8u32;
pub const SVTI2_REMAP_PIPE_NAMES: u32 = 2u32;
pub const SVTI2_RESERVED1: u32 = 4096u32;
pub const SVTI2_RESERVED2: u32 = 8192u32;
pub const SVTI2_RESERVED3: u32 = 16384u32;
pub const SVTI2_SCOPED_NAME: u32 = 4u32;
pub const SVTI2_UNICODE_TRANSPORT_ADDRESS: u32 = 32u32;
pub const SV_ACCEPTDOWNLEVELAPIS_PARMNUM: u32 = 517u32;
pub const SV_ACCESSALERT_PARMNUM: u32 = 40u32;
pub const SV_ACTIVELOCKS_PARMNUM: u32 = 419u32;
pub const SV_ALERTSCHEDULE_PARMNUM: u32 = 547u32;
pub const SV_ALERTSCHED_PARMNUM: u32 = 37u32;
pub const SV_ALERTS_PARMNUM: u32 = 11u32;
pub const SV_ALIST_MTIME_PARMNUM: u32 = 403u32;
pub const SV_ANNDELTA_PARMNUM: u32 = 18u32;
pub const SV_ANNOUNCE_PARMNUM: u32 = 17u32;
pub const SV_AUTOSHARESERVER_PARMNUM: u32 = 592u32;
pub const SV_AUTOSHAREWKS_PARMNUM: u32 = 591u32;
pub const SV_BALANCECOUNT_PARMNUM: u32 = 577u32;
pub const SV_CACHEDDIRECTORYLIMIT_PARMNUM: u32 = 587u32;
pub const SV_CACHEDOPENLIMIT_PARMNUM: u32 = 571u32;
pub const SV_CHDEVJOBS_PARMNUM: u32 = 411u32;
pub const SV_CHDEVQ_PARMNUM: u32 = 410u32;
pub const SV_COMMENT_PARMNUM: u32 = 5u32;
pub const SV_CONNECTIONLESSAUTODISC_PARMNUM: u32 = 562u32;
pub const SV_CONNECTIONNOSESSIONSTIMEOUT_PARMNUM: u32 = 596u32;
pub const SV_CONNECTIONS_PARMNUM: u32 = 412u32;
pub const SV_CRITICALTHREADS_PARMNUM: u32 = 572u32;
pub const SV_DISABLEDOS_PARMNUM: u32 = 600u32;
pub const SV_DISABLESTRICTNAMECHECKING_PARMNUM: u32 = 602u32;
pub const SV_DISC_PARMNUM: u32 = 10u32;
pub const SV_DISKALERT_PARMNUM: u32 = 41u32;
pub const SV_DISKSPACETHRESHOLD_PARMNUM: u32 = 550u32;
pub const SV_DOMAIN_PARMNUM: u32 = 519u32;
pub const SV_ENABLEAUTHENTICATEUSERSHARING_PARMNUM: u32 = 603u32;
pub const SV_ENABLECOMPRESSION_PARMNUM: u32 = 590u32;
pub const SV_ENABLEFCBOPENS_PARMNUM: u32 = 538u32;
pub const SV_ENABLEFORCEDLOGOFF_PARMNUM: u32 = 515u32;
pub const SV_ENABLEOPLOCKFORCECLOSE_PARMNUM: u32 = 537u32;
pub const SV_ENABLEOPLOCKS_PARMNUM: u32 = 536u32;
pub const SV_ENABLERAW_PARMNUM: u32 = 539u32;
pub const SV_ENABLESECURITYSIGNATURE_PARMNUM: u32 = 593u32;
pub const SV_ENABLESHAREDNETDRIVES_PARMNUM: u32 = 540u32;
pub const SV_ENABLESOFTCOMPAT_PARMNUM: u32 = 514u32;
pub const SV_ENABLEW9XSECURITYSIGNATURE_PARMNUM: u32 = 598u32;
pub const SV_ENABLEWFW311DIRECTIPX_PARMNUM: u32 = 574u32;
pub const SV_ENFORCEKERBEROSREAUTHENTICATION_PARMNUM: u32 = 599u32;
pub const SV_ERRORALERT_PARMNUM: u32 = 38u32;
pub const SV_ERRORTHRESHOLD_PARMNUM: u32 = 548u32;
pub const SV_GLIST_MTIME_PARMNUM: u32 = 402u32;
pub const SV_GUESTACC_PARMNUM: u32 = 408u32;
pub const SV_HIDDEN: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(1i32);
pub const SV_HIDDEN_PARMNUM: u32 = 16u32;
pub const SV_IDLETHREADTIMEOUT_PARMNUM: u32 = 597u32;
pub const SV_INITCONNTABLE_PARMNUM: u32 = 544u32;
pub const SV_INITFILETABLE_PARMNUM: u32 = 545u32;
pub const SV_INITSEARCHTABLE_PARMNUM: u32 = 546u32;
pub const SV_INITSESSTABLE_PARMNUM: u32 = 543u32;
pub const SV_INITWORKITEMS_PARMNUM: u32 = 505u32;
pub const SV_IRPSTACKSIZE_PARMNUM: u32 = 508u32;
pub const SV_LANMASK_PARMNUM: u32 = 407u32;
pub const SV_LINKINFOVALIDTIME_PARMNUM: u32 = 554u32;
pub const SV_LMANNOUNCE_PARMNUM: u32 = 518u32;
pub const SV_LOCKVIOLATIONDELAY_PARMNUM: u32 = 569u32;
pub const SV_LOCKVIOLATIONOFFSET_PARMNUM: u32 = 568u32;
pub const SV_LOCKVIOLATIONRETRIES_PARMNUM: u32 = 567u32;
pub const SV_LOGONALERT_PARMNUM: u32 = 39u32;
pub const SV_LOWDISKSPACEMINIMUM_PARMNUM: u32 = 601u32;
pub const SV_MAXAUDITSZ_PARMNUM: u32 = 43u32;
pub const SV_MAXCOPYLENGTH_PARMNUM: u32 = 588u32;
pub const SV_MAXCOPYREADLEN_PARMNUM: u32 = 520u32;
pub const SV_MAXCOPYWRITELEN_PARMNUM: u32 = 521u32;
pub const SV_MAXFREECONNECTIONS_PARMNUM: u32 = 542u32;
pub const SV_MAXFREELFCBS_PARMNUM: u32 = 581u32;
pub const SV_MAXFREEMFCBS_PARMNUM: u32 = 580u32;
pub const SV_MAXFREEPAGEDPOOLCHUNKS_PARMNUM: u32 = 582u32;
pub const SV_MAXFREERFCBS_PARMNUM: u32 = 579u32;
pub const SV_MAXGLOBALOPENSEARCH_PARMNUM: u32 = 565u32;
pub const SV_MAXKEEPCOMPLSEARCH_PARMNUM: u32 = 525u32;
pub const SV_MAXKEEPSEARCH_PARMNUM: u32 = 523u32;
pub const SV_MAXLINKDELAY_PARMNUM: u32 = 552u32;
pub const SV_MAXMPXCT_PARMNUM: u32 = 533u32;
pub const SV_MAXNONPAGEDMEMORYUSAGE_PARMNUM: u32 = 512u32;
pub const SV_MAXPAGEDMEMORYUSAGE_PARMNUM: u32 = 513u32;
pub const SV_MAXPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 584u32;
pub const SV_MAXRAWBUFLEN_PARMNUM: u32 = 509u32;
pub const SV_MAXRAWWORKITEMS_PARMNUM: u32 = 557u32;
pub const SV_MAXTHREADSPERQUEUE_PARMNUM: u32 = 586u32;
pub const SV_MAXWORKITEMIDLETIME_PARMNUM: u32 = 556u32;
pub const SV_MAXWORKITEMS_PARMNUM: u32 = 506u32;
pub const SV_MAX_CMD_LEN: u32 = 256u32;
pub const SV_MAX_SRV_HEUR_LEN: u32 = 32u32;
pub const SV_MDLREADSWITCHOVER_PARMNUM: u32 = 570u32;
pub const SV_MINCLIENTBUFFERSIZE_PARMNUM: u32 = 595u32;
pub const SV_MINFREECONNECTIONS_PARMNUM: u32 = 541u32;
pub const SV_MINFREEWORKITEMS_PARMNUM: u32 = 530u32;
pub const SV_MINKEEPCOMPLSEARCH_PARMNUM: u32 = 524u32;
pub const SV_MINKEEPSEARCH_PARMNUM: u32 = 522u32;
pub const SV_MINLINKTHROUGHPUT_PARMNUM: u32 = 553u32;
pub const SV_MINPAGEDPOOLCHUNKSIZE_PARMNUM: u32 = 583u32;
pub const SV_MINRCVQUEUE_PARMNUM: u32 = 529u32;
pub const SV_NAME_PARMNUM: u32 = 102u32;
pub const SV_NETIOALERT_PARMNUM: u32 = 42u32;
pub const SV_NETWORKERRORTHRESHOLD_PARMNUM: u32 = 549u32;
pub const SV_NODISC: i32 = -1i32;
pub const SV_NUMADMIN_PARMNUM: u32 = 406u32;
pub const SV_NUMBIGBUF_PARMNUM: u32 = 422u32;
pub const SV_NUMBLOCKTHREADS_PARMNUM: u32 = 527u32;
pub const SV_NUMFILETASKS_PARMNUM: u32 = 423u32;
pub const SV_NUMREQBUF_PARMNUM: u32 = 420u32;
pub const SV_OPENFILES_PARMNUM: u32 = 414u32;
pub const SV_OPENSEARCH_PARMNUM: u32 = 503u32;
pub const SV_OPLOCKBREAKRESPONSEWAIT_PARMNUM: u32 = 535u32;
pub const SV_OPLOCKBREAKWAIT_PARMNUM: u32 = 534u32;
pub const SV_OTHERQUEUEAFFINITY_PARMNUM: u32 = 575u32;
pub const SV_PLATFORM_ID_NT: u32 = 500u32;
pub const SV_PLATFORM_ID_OS2: u32 = 400u32;
pub const SV_PLATFORM_ID_PARMNUM: u32 = 101u32;
pub const SV_PREFERREDAFFINITY_PARMNUM: u32 = 578u32;
pub const SV_PRODUCTTYPE_PARMNUM: u32 = 560u32;
pub const SV_QUEUESAMPLESECS_PARMNUM: u32 = 576u32;
pub const SV_RAWWORKITEMS_PARMNUM: u32 = 507u32;
pub const SV_REMOVEDUPLICATESEARCHES_PARMNUM: u32 = 566u32;
pub const SV_REQUIRESECURITYSIGNATURE_PARMNUM: u32 = 594u32;
pub const SV_RESTRICTNULLSESSACCESS_PARMNUM: u32 = 573u32;
pub const SV_SCAVQOSINFOUPDATETIME_PARMNUM: u32 = 555u32;
pub const SV_SCAVTIMEOUT_PARMNUM: u32 = 528u32;
pub const SV_SECURITY_PARMNUM: u32 = 405u32;
pub const SV_SENDSFROMPREFERREDPROCESSOR_PARMNUM: u32 = 585u32;
pub const SV_SERVERSIZE_PARMNUM: u32 = 561u32;
pub const SV_SESSCONNS_PARMNUM: u32 = 511u32;
pub const SV_SESSOPENS_PARMNUM: u32 = 501u32;
pub const SV_SESSREQS_PARMNUM: u32 = 417u32;
pub const SV_SESSUSERS_PARMNUM: u32 = 510u32;
pub const SV_SESSVCS_PARMNUM: u32 = 502u32;
pub const SV_SHARESECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(0u32);
pub const SV_SHARES_PARMNUM: u32 = 413u32;
pub const SV_SHARINGVIOLATIONDELAY_PARMNUM: u32 = 564u32;
pub const SV_SHARINGVIOLATIONRETRIES_PARMNUM: u32 = 563u32;
pub const SV_SIZREQBUF_PARMNUM: u32 = 504u32;
pub const SV_SRVHEURISTICS_PARMNUM: u32 = 431u32;
pub const SV_THREADCOUNTADD_PARMNUM: u32 = 526u32;
pub const SV_THREADPRIORITY_PARMNUM: u32 = 532u32;
pub const SV_TIMESOURCE_PARMNUM: u32 = 516u32;
pub const SV_TYPE_AFP: NET_SERVER_TYPE = NET_SERVER_TYPE(64u32);
pub const SV_TYPE_ALL: NET_SERVER_TYPE = NET_SERVER_TYPE(4294967295u32);
pub const SV_TYPE_ALTERNATE_XPORT: NET_SERVER_TYPE = NET_SERVER_TYPE(536870912u32);
pub const SV_TYPE_BACKUP_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(131072u32);
pub const SV_TYPE_CLUSTER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(16777216u32);
pub const SV_TYPE_CLUSTER_VS_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(67108864u32);
pub const SV_TYPE_DCE: NET_SERVER_TYPE = NET_SERVER_TYPE(268435456u32);
pub const SV_TYPE_DFS: NET_SERVER_TYPE = NET_SERVER_TYPE(8388608u32);
pub const SV_TYPE_DIALIN_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(1024u32);
pub const SV_TYPE_DOMAIN_BAKCTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(16u32);
pub const SV_TYPE_DOMAIN_CTRL: NET_SERVER_TYPE = NET_SERVER_TYPE(8u32);
pub const SV_TYPE_DOMAIN_ENUM: NET_SERVER_TYPE = NET_SERVER_TYPE(2147483648u32);
pub const SV_TYPE_DOMAIN_MASTER: NET_SERVER_TYPE = NET_SERVER_TYPE(524288u32);
pub const SV_TYPE_DOMAIN_MEMBER: NET_SERVER_TYPE = NET_SERVER_TYPE(256u32);
pub const SV_TYPE_LOCAL_LIST_ONLY: NET_SERVER_TYPE = NET_SERVER_TYPE(1073741824u32);
pub const SV_TYPE_MASTER_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(262144u32);
pub const SV_TYPE_NOVELL: NET_SERVER_TYPE = NET_SERVER_TYPE(128u32);
pub const SV_TYPE_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(4096u32);
pub const SV_TYPE_PARMNUM: u32 = 105u32;
pub const SV_TYPE_POTENTIAL_BROWSER: NET_SERVER_TYPE = NET_SERVER_TYPE(65536u32);
pub const SV_TYPE_PRINTQ_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(512u32);
pub const SV_TYPE_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2u32);
pub const SV_TYPE_SERVER_MFPN: NET_SERVER_TYPE = NET_SERVER_TYPE(16384u32);
pub const SV_TYPE_SERVER_NT: NET_SERVER_TYPE = NET_SERVER_TYPE(32768u32);
pub const SV_TYPE_SERVER_OSF: NET_SERVER_TYPE = NET_SERVER_TYPE(1048576u32);
pub const SV_TYPE_SERVER_UNIX: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
pub const SV_TYPE_SERVER_VMS: NET_SERVER_TYPE = NET_SERVER_TYPE(2097152u32);
pub const SV_TYPE_SQLSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(4u32);
pub const SV_TYPE_TERMINALSERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(33554432u32);
pub const SV_TYPE_TIME_SOURCE: NET_SERVER_TYPE = NET_SERVER_TYPE(32u32);
pub const SV_TYPE_WFW: NET_SERVER_TYPE = NET_SERVER_TYPE(8192u32);
pub const SV_TYPE_WINDOWS: NET_SERVER_TYPE = NET_SERVER_TYPE(4194304u32);
pub const SV_TYPE_WORKSTATION: NET_SERVER_TYPE = NET_SERVER_TYPE(1u32);
pub const SV_TYPE_XENIX_SERVER: NET_SERVER_TYPE = NET_SERVER_TYPE(2048u32);
pub const SV_ULIST_MTIME_PARMNUM: u32 = 401u32;
pub const SV_USERPATH_PARMNUM: u32 = 112u32;
pub const SV_USERSECURITY: SERVER_INFO_SECURITY = SERVER_INFO_SECURITY(1u32);
pub const SV_USERS_PARMNUM: u32 = 107u32;
pub const SV_USERS_PER_LICENSE: u32 = 5u32;
pub const SV_VERSION_MAJOR_PARMNUM: u32 = 103u32;
pub const SV_VERSION_MINOR_PARMNUM: u32 = 104u32;
pub const SV_VISIBLE: SERVER_INFO_HIDDEN = SERVER_INFO_HIDDEN(0i32);
pub const SV_XACTMEMSIZE_PARMNUM: u32 = 531u32;
pub const SW_AUTOPROF_LOAD_MASK: u32 = 1u32;
pub const SW_AUTOPROF_SAVE_MASK: u32 = 2u32;
pub const ServiceAccountPasswordGUID: windows_core::GUID = windows_core::GUID::from_u128(0x262e99c9_6160_4871_acec_4e61736b6f21);
pub const TITLE_SC_MESSAGE_BOX: i32 = -1073734795i32;
pub const TRACE_NO_STDINFO: u32 = 1u32;
pub const TRACE_NO_SYNCH: u32 = 4u32;
pub const TRACE_USE_CONSOLE: u32 = 2u32;
pub const TRACE_USE_DATE: u32 = 8u32;
pub const TRACE_USE_FILE: u32 = 1u32;
pub const TRACE_USE_MASK: u32 = 2u32;
pub const TRACE_USE_MSEC: u32 = 4u32;
pub const TRANSPORT_NAME_PARMNUM: u32 = 202u32;
pub const TRANSPORT_QUALITYOFSERVICE_PARMNUM: u32 = 201u32;
pub const UAS_ROLE_BACKUP: USER_MODALS_ROLES = USER_MODALS_ROLES(2u32);
pub const UAS_ROLE_MEMBER: USER_MODALS_ROLES = USER_MODALS_ROLES(1u32);
pub const UAS_ROLE_PRIMARY: USER_MODALS_ROLES = USER_MODALS_ROLES(3u32);
pub const UAS_ROLE_STANDALONE: USER_MODALS_ROLES = USER_MODALS_ROLES(0u32);
pub const UF_ACCOUNTDISABLE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2u32);
pub const UF_DONT_EXPIRE_PASSWD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(65536u32);
pub const UF_DONT_REQUIRE_PREAUTH: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(4194304u32);
pub const UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(128u32);
pub const UF_HOMEDIR_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8u32);
pub const UF_INTERDOMAIN_TRUST_ACCOUNT: u32 = 2048u32;
pub const UF_LOCKOUT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16u32);
pub const UF_MNS_LOGON_ACCOUNT: u32 = 131072u32;
pub const UF_NORMAL_ACCOUNT: u32 = 512u32;
pub const UF_NOT_DELEGATED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1048576u32);
pub const UF_NO_AUTH_DATA_REQUIRED: u32 = 33554432u32;
pub const UF_PARTIAL_SECRETS_ACCOUNT: u32 = 67108864u32;
pub const UF_PASSWD_CANT_CHANGE: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(64u32);
pub const UF_PASSWD_NOTREQD: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(32u32);
pub const UF_PASSWORD_EXPIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(8388608u32);
pub const UF_SCRIPT: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(1u32);
pub const UF_SERVER_TRUST_ACCOUNT: u32 = 8192u32;
pub const UF_SMARTCARD_REQUIRED: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(262144u32);
pub const UF_TEMP_DUPLICATE_ACCOUNT: u32 = 256u32;
pub const UF_TRUSTED_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(524288u32);
pub const UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(16777216u32);
pub const UF_USE_AES_KEYS: u32 = 134217728u32;
pub const UF_USE_DES_KEY_ONLY: USER_ACCOUNT_FLAGS = USER_ACCOUNT_FLAGS(2097152u32);
pub const UF_WORKSTATION_TRUST_ACCOUNT: u32 = 4096u32;
pub const UNCLEN: u32 = 17u32;
pub const UNITS_PER_DAY: u32 = 24u32;
pub const UNLEN: u32 = 256u32;
pub const UPPER_GET_HINT_MASK: u32 = 267386880u32;
pub const UPPER_HINT_MASK: u32 = 65280u32;
pub const USER_ACCT_EXPIRES_PARMNUM: u32 = 17u32;
pub const USER_AUTH_FLAGS_PARMNUM: u32 = 10u32;
pub const USER_CODE_PAGE_PARMNUM: u32 = 25u32;
pub const USER_COMMENT_PARMNUM: u32 = 7u32;
pub const USER_COUNTRY_CODE_PARMNUM: u32 = 24u32;
pub const USER_FLAGS_PARMNUM: u32 = 8u32;
pub const USER_FULL_NAME_PARMNUM: u32 = 11u32;
pub const USER_HOME_DIR_DRIVE_PARMNUM: u32 = 53u32;
pub const USER_HOME_DIR_PARMNUM: u32 = 6u32;
pub const USER_LAST_LOGOFF_PARMNUM: u32 = 16u32;
pub const USER_LAST_LOGON_PARMNUM: u32 = 15u32;
pub const USER_LOGON_HOURS_PARMNUM: u32 = 20u32;
pub const USER_LOGON_SERVER_PARMNUM: u32 = 23u32;
pub const USER_MAX_STORAGE_PARMNUM: u32 = 18u32;
pub const USER_NAME_PARMNUM: u32 = 1u32;
pub const USER_NUM_LOGONS_PARMNUM: u32 = 22u32;
pub const USER_PAD_PW_COUNT_PARMNUM: u32 = 21u32;
pub const USER_PARMS_PARMNUM: u32 = 13u32;
pub const USER_PASSWORD_AGE_PARMNUM: u32 = 4u32;
pub const USER_PASSWORD_PARMNUM: u32 = 3u32;
pub const USER_PRIMARY_GROUP_PARMNUM: u32 = 51u32;
pub const USER_PRIV_ADMIN: USER_PRIV = USER_PRIV(2u32);
pub const USER_PRIV_GUEST: USER_PRIV = USER_PRIV(0u32);
pub const USER_PRIV_MASK: u32 = 3u32;
pub const USER_PRIV_PARMNUM: u32 = 5u32;
pub const USER_PRIV_USER: USER_PRIV = USER_PRIV(1u32);
pub const USER_PROFILE: u32 = 52u32;
pub const USER_PROFILE_PARMNUM: u32 = 52u32;
pub const USER_SCRIPT_PATH_PARMNUM: u32 = 9u32;
pub const USER_UNITS_PER_WEEK_PARMNUM: u32 = 19u32;
pub const USER_USR_COMMENT_PARMNUM: u32 = 12u32;
pub const USER_WORKSTATIONS_PARMNUM: u32 = 14u32;
pub const USE_ASGTYPE_PARMNUM: u32 = 4u32;
pub const USE_AUTHIDENTITY_PARMNUM: u32 = 8u32;
pub const USE_CHARDEV: u32 = 2u32;
pub const USE_CONN: u32 = 4u32;
pub const USE_DEFAULT_CREDENTIALS: u32 = 4u32;
pub const USE_DISCONN: u32 = 2u32;
pub const USE_DISKDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(0u32);
pub const USE_DOMAINNAME_PARMNUM: u32 = 6u32;
pub const USE_FLAGS_PARMNUM: u32 = 7u32;
pub const USE_FLAG_GLOBAL_MAPPING: u32 = 65536u32;
pub const USE_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(1u32);
pub const USE_IPC: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(3u32);
pub const USE_LOCAL_PARMNUM: u32 = 1u32;
pub const USE_LOTS_OF_FORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(2u32);
pub const USE_NETERR: u32 = 3u32;
pub const USE_NOFORCE: FORCE_LEVEL_FLAGS = FORCE_LEVEL_FLAGS(0u32);
pub const USE_OK: u32 = 0u32;
pub const USE_OPTIONS_PARMNUM: u32 = 10u32;
pub const USE_PASSWORD_PARMNUM: u32 = 3u32;
pub const USE_PAUSED: u32 = 1u32;
pub const USE_RECONN: u32 = 5u32;
pub const USE_REMOTE_PARMNUM: u32 = 2u32;
pub const USE_SD_PARMNUM: u32 = 9u32;
pub const USE_SESSLOST: u32 = 2u32;
pub const USE_SPECIFIC_TRANSPORT: u32 = 2147483648u32;
pub const USE_SPOOLDEV: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(1u32);
pub const USE_USERNAME_PARMNUM: u32 = 5u32;
pub const USE_WILDCARD: USE_INFO_ASG_TYPE = USE_INFO_ASG_TYPE(4294967295u32);
pub const UseTransportType_None: TRANSPORT_TYPE = TRANSPORT_TYPE(0i32);
pub const UseTransportType_Quic: TRANSPORT_TYPE = TRANSPORT_TYPE(2i32);
pub const UseTransportType_Wsk: TRANSPORT_TYPE = TRANSPORT_TYPE(1i32);
pub const VALIDATED_LOGON: u32 = 0u32;
pub const VALID_LOGOFF: u32 = 1u32;
pub const WKSTA_BUFFERNAMEDPIPES_PARMNUM: u32 = 51u32;
pub const WKSTA_BUFFERREADONLYFILES_PARMNUM: u32 = 59u32;
pub const WKSTA_BUFFILESWITHDENYWRITE_PARMNUM: u32 = 58u32;
pub const WKSTA_CACHEFILETIMEOUT_PARMNUM: u32 = 47u32;
pub const WKSTA_CHARCOUNT_PARMNUM: u32 = 12u32;
pub const WKSTA_CHARTIME_PARMNUM: u32 = 11u32;
pub const WKSTA_CHARWAIT_PARMNUM: u32 = 10u32;
pub const WKSTA_COMPUTERNAME_PARMNUM: u32 = 1u32;
pub const WKSTA_DORMANTFILELIMIT_PARMNUM: u32 = 46u32;
pub const WKSTA_ERRLOGSZ_PARMNUM: u32 = 27u32;
pub const WKSTA_FORCECORECREATEMODE_PARMNUM: u32 = 60u32;
pub const WKSTA_KEEPCONN_PARMNUM: u32 = 13u32;
pub const WKSTA_KEEPSEARCH_PARMNUM: u32 = 14u32;
pub const WKSTA_LANGROUP_PARMNUM: u32 = 2u32;
pub const WKSTA_LANROOT_PARMNUM: u32 = 7u32;
pub const WKSTA_LOCKINCREMENT_PARMNUM: u32 = 42u32;
pub const WKSTA_LOCKMAXIMUM_PARMNUM: u32 = 43u32;
pub const WKSTA_LOCKQUOTA_PARMNUM: u32 = 41u32;
pub const WKSTA_LOGGED_ON_USERS_PARMNUM: u32 = 6u32;
pub const WKSTA_LOGON_DOMAIN_PARMNUM: u32 = 8u32;
pub const WKSTA_LOGON_SERVER_PARMNUM: u32 = 9u32;
pub const WKSTA_MAILSLOTS_PARMNUM: u32 = 30u32;
pub const WKSTA_MAXCMDS_PARMNUM: u32 = 15u32;
pub const WKSTA_MAXTHREADS_PARMNUM: u32 = 33u32;
pub const WKSTA_MAXWRKCACHE_PARMNUM: u32 = 17u32;
pub const WKSTA_NUMALERTS_PARMNUM: u32 = 20u32;
pub const WKSTA_NUMCHARBUF_PARMNUM: u32 = 22u32;
pub const WKSTA_NUMDGRAMBUF_PARMNUM: u32 = 31u32;
pub const WKSTA_NUMSERVICES_PARMNUM: u32 = 21u32;
pub const WKSTA_NUMWORKBUF_PARMNUM: u32 = 16u32;
pub const WKSTA_OTH_DOMAINS_PARMNUM: u32 = 101u32;
pub const WKSTA_PIPEINCREMENT_PARMNUM: u32 = 44u32;
pub const WKSTA_PIPEMAXIMUM_PARMNUM: u32 = 45u32;
pub const WKSTA_PLATFORM_ID_PARMNUM: u32 = 100u32;
pub const WKSTA_PRINTBUFTIME_PARMNUM: u32 = 28u32;
pub const WKSTA_READAHEADTHRUPUT_PARMNUM: u32 = 62u32;
pub const WKSTA_SESSTIMEOUT_PARMNUM: u32 = 18u32;
pub const WKSTA_SIZCHARBUF_PARMNUM: u32 = 23u32;
pub const WKSTA_SIZERROR_PARMNUM: u32 = 19u32;
pub const WKSTA_SIZWORKBUF_PARMNUM: u32 = 29u32;
pub const WKSTA_USE512BYTESMAXTRANSFER_PARMNUM: u32 = 61u32;
pub const WKSTA_USECLOSEBEHIND_PARMNUM: u32 = 50u32;
pub const WKSTA_USEENCRYPTION_PARMNUM: u32 = 57u32;
pub const WKSTA_USELOCKANDREADANDUNLOCK_PARMNUM: u32 = 52u32;
pub const WKSTA_USEOPPORTUNISTICLOCKING_PARMNUM: u32 = 48u32;
pub const WKSTA_USERAWREAD_PARMNUM: u32 = 54u32;
pub const WKSTA_USERAWWRITE_PARMNUM: u32 = 55u32;
pub const WKSTA_USEUNLOCKBEHIND_PARMNUM: u32 = 49u32;
pub const WKSTA_USEWRITERAWWITHDATA_PARMNUM: u32 = 56u32;
pub const WKSTA_UTILIZENTCACHING_PARMNUM: u32 = 53u32;
pub const WKSTA_VER_MAJOR_PARMNUM: u32 = 4u32;
pub const WKSTA_VER_MINOR_PARMNUM: u32 = 5u32;
pub const WKSTA_WRKHEURISTICS_PARMNUM: u32 = 32u32;
pub const WORKSTATION_DISPLAY_NAME: windows_core::PCWSTR = windows_core::w!("Workstation");
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_SCHEMA: u32 = 34u32;
pub const WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_XML: u32 = 33u32;
pub const WZC_PROFILE_API_ERROR_INTERNAL: u32 = 36u32;
pub const WZC_PROFILE_API_ERROR_NOT_SUPPORTED: u32 = 32u32;
pub const WZC_PROFILE_API_ERROR_XML_VALIDATION_FAILED: u32 = 35u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED: u32 = 20u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED_KEY_REQUIRED: u32 = 21u32;
pub const WZC_PROFILE_CONFIG_ERROR_1X_NOT_ENABLED_KEY_PROVIDED: u32 = 22u32;
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_NOT_APPLICABLE: u32 = 24u32;
pub const WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_REQUIRED: u32 = 23u32;
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_AUTH_FOR_CONNECTION_TYPE: u32 = 15u32;
pub const WZC_PROFILE_CONFIG_ERROR_INVALID_ENCRYPTION_FOR_AUTHMODE: u32 = 16u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_NOT_APPLICABLE: u32 = 19u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_REQUIRED: u32 = 18u32;
pub const WZC_PROFILE_CONFIG_ERROR_KEY_REQUIRED: u32 = 17u32;
pub const WZC_PROFILE_CONFIG_ERROR_WPA_ENCRYPTION_NOT_SUPPORTED: u32 = 26u32;
pub const WZC_PROFILE_CONFIG_ERROR_WPA_NOT_SUPPORTED: u32 = 25u32;
pub const WZC_PROFILE_SET_ERROR_DUPLICATE_NETWORK: u32 = 27u32;
pub const WZC_PROFILE_SET_ERROR_MEMORY_ALLOCATION: u32 = 28u32;
pub const WZC_PROFILE_SET_ERROR_READING_1X_CONFIG: u32 = 29u32;
pub const WZC_PROFILE_SET_ERROR_WRITING_1X_CONFIG: u32 = 30u32;
pub const WZC_PROFILE_SET_ERROR_WRITING_WZC_CFG: u32 = 31u32;
pub const WZC_PROFILE_SUCCESS: u32 = 0u32;
pub const WZC_PROFILE_XML_ERROR_1X_ENABLED: u32 = 10u32;
pub const WZC_PROFILE_XML_ERROR_AUTHENTICATION: u32 = 7u32;
pub const WZC_PROFILE_XML_ERROR_BAD_KEY_INDEX: u32 = 12u32;
pub const WZC_PROFILE_XML_ERROR_BAD_NETWORK_KEY: u32 = 14u32;
pub const WZC_PROFILE_XML_ERROR_BAD_SSID: u32 = 5u32;
pub const WZC_PROFILE_XML_ERROR_BAD_VERSION: u32 = 2u32;
pub const WZC_PROFILE_XML_ERROR_CONNECTION_TYPE: u32 = 6u32;
pub const WZC_PROFILE_XML_ERROR_EAP_METHOD: u32 = 11u32;
pub const WZC_PROFILE_XML_ERROR_ENCRYPTION: u32 = 8u32;
pub const WZC_PROFILE_XML_ERROR_KEY_INDEX_RANGE: u32 = 13u32;
pub const WZC_PROFILE_XML_ERROR_KEY_PROVIDED_AUTOMATICALLY: u32 = 9u32;
pub const WZC_PROFILE_XML_ERROR_NO_VERSION: u32 = 1u32;
pub const WZC_PROFILE_XML_ERROR_SSID_NOT_FOUND: u32 = 4u32;
pub const WZC_PROFILE_XML_ERROR_UNSUPPORTED_VERSION: u32 = 3u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AF_OP(pub u32);
impl windows_core::TypeKind for AF_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AF_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AF_OP").field(&self.0).finish()
    }
}
impl AF_OP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AF_OP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AF_OP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AF_OP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AF_OP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AF_OP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BIND_FLAGS1(pub i32);
impl windows_core::TypeKind for BIND_FLAGS1 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BIND_FLAGS1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BIND_FLAGS1").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMPONENT_CHARACTERISTICS(pub i32);
impl windows_core::TypeKind for COMPONENT_CHARACTERISTICS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMPONENT_CHARACTERISTICS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMPONENT_CHARACTERISTICS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DEFAULT_PAGES(pub i32);
impl windows_core::TypeKind for DEFAULT_PAGES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DEFAULT_PAGES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DEFAULT_PAGES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSREG_JOIN_TYPE(pub i32);
impl windows_core::TypeKind for DSREG_JOIN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSREG_JOIN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSREG_JOIN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_BINDING_PATHS_FLAGS(pub i32);
impl windows_core::TypeKind for ENUM_BINDING_PATHS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_BINDING_PATHS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_BINDING_PATHS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FORCE_LEVEL_FLAGS(pub u32);
impl windows_core::TypeKind for FORCE_LEVEL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FORCE_LEVEL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FORCE_LEVEL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSA_INFO_LEVEL(pub i32);
impl windows_core::TypeKind for MSA_INFO_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSA_INFO_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSA_INFO_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSA_INFO_STATE(pub i32);
impl windows_core::TypeKind for MSA_INFO_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSA_INFO_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSA_INFO_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NCPNP_RECONFIG_LAYER(pub i32);
impl windows_core::TypeKind for NCPNP_RECONFIG_LAYER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NCPNP_RECONFIG_LAYER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NCPNP_RECONFIG_LAYER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NCRP_FLAGS(pub i32);
impl windows_core::TypeKind for NCRP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NCRP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NCRP_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETSETUP_JOIN_STATUS(pub i32);
impl windows_core::TypeKind for NETSETUP_JOIN_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETSETUP_JOIN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETSETUP_JOIN_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETSETUP_NAME_TYPE(pub i32);
impl windows_core::TypeKind for NETSETUP_NAME_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETSETUP_NAME_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETSETUP_NAME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETSETUP_PROVISION(pub u32);
impl windows_core::TypeKind for NETSETUP_PROVISION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETSETUP_PROVISION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETSETUP_PROVISION").field(&self.0).finish()
    }
}
impl NETSETUP_PROVISION {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NETSETUP_PROVISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NETSETUP_PROVISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NETSETUP_PROVISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NETSETUP_PROVISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NETSETUP_PROVISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETWORK_INSTALL_TIME(pub i32);
impl windows_core::TypeKind for NETWORK_INSTALL_TIME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETWORK_INSTALL_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETWORK_INSTALL_TIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NETWORK_UPGRADE_TYPE(pub i32);
impl windows_core::TypeKind for NETWORK_UPGRADE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NETWORK_UPGRADE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NETWORK_UPGRADE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_COMPUTER_NAME_TYPE(pub i32);
impl windows_core::TypeKind for NET_COMPUTER_NAME_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_COMPUTER_NAME_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_COMPUTER_NAME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_JOIN_DOMAIN_JOIN_OPTIONS(pub u32);
impl windows_core::TypeKind for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_JOIN_DOMAIN_JOIN_OPTIONS").field(&self.0).finish()
    }
}
impl NET_JOIN_DOMAIN_JOIN_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_JOIN_DOMAIN_JOIN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS(pub u32);
impl windows_core::TypeKind for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_REMOTE_COMPUTER_SUPPORTS_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_REQUEST_PROVISION_OPTIONS(pub u32);
impl windows_core::TypeKind for NET_REQUEST_PROVISION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_REQUEST_PROVISION_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_REQUEST_PROVISION_OPTIONS").field(&self.0).finish()
    }
}
impl NET_REQUEST_PROVISION_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_REQUEST_PROVISION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_REQUEST_PROVISION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_SERVER_TYPE(pub u32);
impl windows_core::TypeKind for NET_SERVER_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_SERVER_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_SERVER_TYPE").field(&self.0).finish()
    }
}
impl NET_SERVER_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_SERVER_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_SERVER_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_SERVER_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_SERVER_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_SERVER_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_USER_ENUM_FILTER_FLAGS(pub u32);
impl windows_core::TypeKind for NET_USER_ENUM_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_USER_ENUM_FILTER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_USER_ENUM_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl NET_USER_ENUM_FILTER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NET_USER_ENUM_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NET_USER_ENUM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NET_VALIDATE_PASSWORD_TYPE(pub i32);
impl windows_core::TypeKind for NET_VALIDATE_PASSWORD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NET_VALIDATE_PASSWORD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NET_VALIDATE_PASSWORD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OBO_TOKEN_TYPE(pub i32);
impl windows_core::TypeKind for OBO_TOKEN_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OBO_TOKEN_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OBO_TOKEN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RASCON_UIINFO_FLAGS(pub i32);
impl windows_core::TypeKind for RASCON_UIINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RASCON_UIINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RASCON_UIINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVER_INFO_HIDDEN(pub i32);
impl windows_core::TypeKind for SERVER_INFO_HIDDEN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVER_INFO_HIDDEN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVER_INFO_HIDDEN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVER_INFO_SECURITY(pub u32);
impl windows_core::TypeKind for SERVER_INFO_SECURITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVER_INFO_SECURITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVER_INFO_SECURITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SUPPORTS_BINDING_INTERFACE_FLAGS(pub i32);
impl windows_core::TypeKind for SUPPORTS_BINDING_INTERFACE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SUPPORTS_BINDING_INTERFACE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SUPPORTS_BINDING_INTERFACE_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TRANSPORT_TYPE(pub i32);
impl windows_core::TypeKind for TRANSPORT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TRANSPORT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TRANSPORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_ACCOUNT_FLAGS(pub u32);
impl windows_core::TypeKind for USER_ACCOUNT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_ACCOUNT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_ACCOUNT_FLAGS").field(&self.0).finish()
    }
}
impl USER_ACCOUNT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for USER_ACCOUNT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for USER_ACCOUNT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for USER_ACCOUNT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_MODALS_ROLES(pub u32);
impl windows_core::TypeKind for USER_MODALS_ROLES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_MODALS_ROLES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_MODALS_ROLES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USER_PRIV(pub u32);
impl windows_core::TypeKind for USER_PRIV {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USER_PRIV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USER_PRIV").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USE_INFO_ASG_TYPE(pub u32);
impl windows_core::TypeKind for USE_INFO_ASG_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USE_INFO_ASG_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USE_INFO_ASG_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_INFO_0 {
    pub acc0_resource_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for ACCESS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_INFO_1 {
    pub acc1_resource_name: windows_core::PWSTR,
    pub acc1_attr: u32,
    pub acc1_count: u32,
}
impl windows_core::TypeKind for ACCESS_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_INFO_1002 {
    pub acc1002_attr: u32,
}
impl windows_core::TypeKind for ACCESS_INFO_1002 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ACCESS_LIST {
    pub acl_ugname: windows_core::PWSTR,
    pub acl_access: u32,
}
impl windows_core::TypeKind for ACCESS_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for ACCESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADMIN_OTHER_INFO {
    pub alrtad_errcode: u32,
    pub alrtad_numstrings: u32,
}
impl windows_core::TypeKind for ADMIN_OTHER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADMIN_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_ACCLIM {
    pub ae_al_compname: u32,
    pub ae_al_username: u32,
    pub ae_al_resname: u32,
    pub ae_al_limit: u32,
}
impl windows_core::TypeKind for AE_ACCLIM {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_ACCLIM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_ACLMOD {
    pub ae_am_compname: u32,
    pub ae_am_username: u32,
    pub ae_am_resname: u32,
    pub ae_am_action: u32,
    pub ae_am_datalen: u32,
}
impl windows_core::TypeKind for AE_ACLMOD {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_ACLMOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_CLOSEFILE {
    pub ae_cf_compname: u32,
    pub ae_cf_username: u32,
    pub ae_cf_resname: u32,
    pub ae_cf_fileid: u32,
    pub ae_cf_duration: u32,
    pub ae_cf_reason: u32,
}
impl windows_core::TypeKind for AE_CLOSEFILE {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_CLOSEFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_CONNREJ {
    pub ae_cr_compname: u32,
    pub ae_cr_username: u32,
    pub ae_cr_netname: u32,
    pub ae_cr_reason: u32,
}
impl windows_core::TypeKind for AE_CONNREJ {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_CONNREJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_CONNSTART {
    pub ae_ct_compname: u32,
    pub ae_ct_username: u32,
    pub ae_ct_netname: u32,
    pub ae_ct_connid: u32,
}
impl windows_core::TypeKind for AE_CONNSTART {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_CONNSTART {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_CONNSTOP {
    pub ae_cp_compname: u32,
    pub ae_cp_username: u32,
    pub ae_cp_netname: u32,
    pub ae_cp_connid: u32,
    pub ae_cp_reason: u32,
}
impl windows_core::TypeKind for AE_CONNSTOP {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_CONNSTOP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_GENERIC {
    pub ae_ge_msgfile: u32,
    pub ae_ge_msgnum: u32,
    pub ae_ge_params: u32,
    pub ae_ge_param1: u32,
    pub ae_ge_param2: u32,
    pub ae_ge_param3: u32,
    pub ae_ge_param4: u32,
    pub ae_ge_param5: u32,
    pub ae_ge_param6: u32,
    pub ae_ge_param7: u32,
    pub ae_ge_param8: u32,
    pub ae_ge_param9: u32,
}
impl windows_core::TypeKind for AE_GENERIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_GENERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_LOCKOUT {
    pub ae_lk_compname: u32,
    pub ae_lk_username: u32,
    pub ae_lk_action: u32,
    pub ae_lk_bad_pw_count: u32,
}
impl windows_core::TypeKind for AE_LOCKOUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_LOCKOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_NETLOGOFF {
    pub ae_nf_compname: u32,
    pub ae_nf_username: u32,
    pub ae_nf_reserved1: u32,
    pub ae_nf_reserved2: u32,
}
impl windows_core::TypeKind for AE_NETLOGOFF {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_NETLOGOFF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_NETLOGON {
    pub ae_no_compname: u32,
    pub ae_no_username: u32,
    pub ae_no_privilege: u32,
    pub ae_no_authflags: u32,
}
impl windows_core::TypeKind for AE_NETLOGON {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_NETLOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_RESACCESS {
    pub ae_ra_compname: u32,
    pub ae_ra_username: u32,
    pub ae_ra_resname: u32,
    pub ae_ra_operation: u32,
    pub ae_ra_returncode: u32,
    pub ae_ra_restype: u32,
    pub ae_ra_fileid: u32,
}
impl windows_core::TypeKind for AE_RESACCESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_RESACCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_RESACCESSREJ {
    pub ae_rr_compname: u32,
    pub ae_rr_username: u32,
    pub ae_rr_resname: u32,
    pub ae_rr_operation: u32,
}
impl windows_core::TypeKind for AE_RESACCESSREJ {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_RESACCESSREJ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_SERVICESTAT {
    pub ae_ss_compname: u32,
    pub ae_ss_username: u32,
    pub ae_ss_svcname: u32,
    pub ae_ss_status: u32,
    pub ae_ss_code: u32,
    pub ae_ss_text: u32,
    pub ae_ss_returnval: u32,
}
impl windows_core::TypeKind for AE_SERVICESTAT {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_SERVICESTAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_SESSLOGOFF {
    pub ae_sf_compname: u32,
    pub ae_sf_username: u32,
    pub ae_sf_reason: u32,
}
impl windows_core::TypeKind for AE_SESSLOGOFF {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_SESSLOGOFF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_SESSLOGON {
    pub ae_so_compname: u32,
    pub ae_so_username: u32,
    pub ae_so_privilege: u32,
}
impl windows_core::TypeKind for AE_SESSLOGON {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_SESSLOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_SESSPWERR {
    pub ae_sp_compname: u32,
    pub ae_sp_username: u32,
}
impl windows_core::TypeKind for AE_SESSPWERR {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_SESSPWERR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_SRVSTATUS {
    pub ae_sv_status: u32,
}
impl windows_core::TypeKind for AE_SRVSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_SRVSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AE_UASMOD {
    pub ae_um_compname: u32,
    pub ae_um_username: u32,
    pub ae_um_resname: u32,
    pub ae_um_rectype: u32,
    pub ae_um_action: u32,
    pub ae_um_datalen: u32,
}
impl windows_core::TypeKind for AE_UASMOD {
    type TypeKind = windows_core::CopyType;
}
impl Default for AE_UASMOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AT_ENUM {
    pub JobId: u32,
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_core::PWSTR,
}
impl windows_core::TypeKind for AT_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl Default for AT_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AT_INFO {
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_core::PWSTR,
}
impl windows_core::TypeKind for AT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for AT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUDIT_ENTRY {
    pub ae_len: u32,
    pub ae_reserved: u32,
    pub ae_time: u32,
    pub ae_type: u32,
    pub ae_data_offset: u32,
    pub ae_data_size: u32,
}
impl windows_core::TypeKind for AUDIT_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUDIT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONFIG_INFO_0 {
    pub cfgi0_key: windows_core::PWSTR,
    pub cfgi0_data: windows_core::PWSTR,
}
impl windows_core::TypeKind for CONFIG_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONFIG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: *const super::super::Security::Cryptography::CERT_CONTEXT,
    pub pszDeviceId: windows_core::PWSTR,
    pub pszIdpDomain: windows_core::PWSTR,
    pub pszTenantId: windows_core::PWSTR,
    pub pszJoinUserEmail: windows_core::PWSTR,
    pub pszTenantDisplayName: windows_core::PWSTR,
    pub pszMdmEnrollmentUrl: windows_core::PWSTR,
    pub pszMdmTermsOfUseUrl: windows_core::PWSTR,
    pub pszMdmComplianceUrl: windows_core::PWSTR,
    pub pszUserSettingSyncUrl: windows_core::PWSTR,
    pub pUserInfo: *mut DSREG_USER_INFO,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for DSREG_JOIN_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for DSREG_JOIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: windows_core::PWSTR,
    pub pszUserKeyId: windows_core::PWSTR,
    pub pszUserKeyName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DSREG_USER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSREG_USER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ERRLOG_OTHER_INFO {
    pub alrter_errcode: u32,
    pub alrter_offset: u32,
}
impl windows_core::TypeKind for ERRLOG_OTHER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ERRLOG_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ERROR_LOG {
    pub el_len: u32,
    pub el_reserved: u32,
    pub el_time: u32,
    pub el_error: u32,
    pub el_name: windows_core::PWSTR,
    pub el_text: windows_core::PWSTR,
    pub el_data: *mut u8,
    pub el_data_size: u32,
    pub el_nstrings: u32,
}
impl windows_core::TypeKind for ERROR_LOG {
    type TypeKind = windows_core::CopyType;
}
impl Default for ERROR_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLAT_STRING {
    pub MaximumLength: i16,
    pub Length: i16,
    pub Buffer: [i8; 1],
}
impl windows_core::TypeKind for FLAT_STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_0 {
    pub grpi0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for GROUP_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_1 {
    pub grpi1_name: windows_core::PWSTR,
    pub grpi1_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for GROUP_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_1002 {
    pub grpi1002_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for GROUP_INFO_1002 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_1005 {
    pub grpi1005_attributes: u32,
}
impl windows_core::TypeKind for GROUP_INFO_1005 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_2 {
    pub grpi2_name: windows_core::PWSTR,
    pub grpi2_comment: windows_core::PWSTR,
    pub grpi2_group_id: u32,
    pub grpi2_attributes: u32,
}
impl windows_core::TypeKind for GROUP_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_INFO_3 {
    pub grpi3_name: windows_core::PWSTR,
    pub grpi3_comment: windows_core::PWSTR,
    pub grpi3_group_sid: super::super::Foundation::PSID,
    pub grpi3_attributes: u32,
}
impl windows_core::TypeKind for GROUP_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_USERS_INFO_0 {
    pub grui0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for GROUP_USERS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_USERS_INFO_1 {
    pub grui1_name: windows_core::PWSTR,
    pub grui1_attributes: u32,
}
impl windows_core::TypeKind for GROUP_USERS_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for GROUP_USERS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HARDWARE_ADDRESS {
    pub Address: [u8; 6],
}
impl windows_core::TypeKind for HARDWARE_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for HARDWARE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HLOG {
    pub time: u32,
    pub last_flags: u32,
    pub offset: u32,
    pub rec_offset: u32,
}
impl windows_core::TypeKind for HLOG {
    type TypeKind = windows_core::CopyType;
}
impl Default for HLOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_0 {
    pub lgrpi0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for LOCALGROUP_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_1 {
    pub lgrpi1_name: windows_core::PWSTR,
    pub lgrpi1_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for LOCALGROUP_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_INFO_1002 {
    pub lgrpi1002_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for LOCALGROUP_INFO_1002 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_0 {
    pub lgrmi0_sid: super::super::Foundation::PSID,
}
impl windows_core::TypeKind for LOCALGROUP_MEMBERS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_MEMBERS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_1 {
    pub lgrmi1_sid: super::super::Foundation::PSID,
    pub lgrmi1_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi1_name: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for LOCALGROUP_MEMBERS_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for LOCALGROUP_MEMBERS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_2 {
    pub lgrmi2_sid: super::super::Foundation::PSID,
    pub lgrmi2_sidusage: super::super::Security::SID_NAME_USE,
    pub lgrmi2_domainandname: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for LOCALGROUP_MEMBERS_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for LOCALGROUP_MEMBERS_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_MEMBERS_INFO_3 {
    pub lgrmi3_domainandname: windows_core::PWSTR,
}
impl windows_core::TypeKind for LOCALGROUP_MEMBERS_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_MEMBERS_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOCALGROUP_USERS_INFO_0 {
    pub lgrui0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for LOCALGROUP_USERS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for LOCALGROUP_USERS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPR_PROTOCOL_0 {
    pub dwProtocolId: u32,
    pub wszProtocol: [u16; 41],
    pub wszDLLName: [u16; 49],
}
impl windows_core::TypeKind for MPR_PROTOCOL_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MPR_PROTOCOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSA_INFO_0 {
    pub State: MSA_INFO_STATE,
}
impl windows_core::TypeKind for MSA_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSA_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSG_INFO_0 {
    pub msgi0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for MSG_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSG_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSG_INFO_1 {
    pub msgi1_name: windows_core::PWSTR,
    pub msgi1_forward_flag: u32,
    pub msgi1_forward: windows_core::PWSTR,
}
impl windows_core::TypeKind for MSG_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for MSG_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETLOGON_INFO_1 {
    pub netlog1_flags: u32,
    pub netlog1_pdc_connection_status: u32,
}
impl windows_core::TypeKind for NETLOGON_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETLOGON_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETLOGON_INFO_2 {
    pub netlog2_flags: u32,
    pub netlog2_pdc_connection_status: u32,
    pub netlog2_trusted_dc_name: windows_core::PWSTR,
    pub netlog2_tc_connection_status: u32,
}
impl windows_core::TypeKind for NETLOGON_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETLOGON_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETLOGON_INFO_3 {
    pub netlog3_flags: u32,
    pub netlog3_logon_attempts: u32,
    pub netlog3_reserved1: u32,
    pub netlog3_reserved2: u32,
    pub netlog3_reserved3: u32,
    pub netlog3_reserved4: u32,
    pub netlog3_reserved5: u32,
}
impl windows_core::TypeKind for NETLOGON_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETLOGON_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETLOGON_INFO_4 {
    pub netlog4_trusted_dc_name: windows_core::PWSTR,
    pub netlog4_trusted_domain_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for NETLOGON_INFO_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETLOGON_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: u32,
    pub lpDomain: windows_core::PCWSTR,
    pub lpHostName: windows_core::PCWSTR,
    pub lpMachineAccountOU: windows_core::PCWSTR,
    pub lpDcName: windows_core::PCWSTR,
    pub dwProvisionOptions: NETSETUP_PROVISION,
    pub aCertTemplateNames: *const windows_core::PCWSTR,
    pub cCertTemplateNames: u32,
    pub aMachinePolicyNames: *const windows_core::PCWSTR,
    pub cMachinePolicyNames: u32,
    pub aMachinePolicyPaths: *const windows_core::PCWSTR,
    pub cMachinePolicyPaths: u32,
    pub lpNetbiosName: windows_core::PWSTR,
    pub lpSiteName: windows_core::PWSTR,
    pub lpPrimaryDNSDomain: windows_core::PWSTR,
}
impl windows_core::TypeKind for NETSETUP_PROVISIONING_PARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETSETUP_PROVISIONING_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NETWORK_NAME {
    pub Name: FLAT_STRING,
}
impl windows_core::TypeKind for NETWORK_NAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for NETWORK_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_DISPLAY_GROUP {
    pub grpi3_name: windows_core::PWSTR,
    pub grpi3_comment: windows_core::PWSTR,
    pub grpi3_group_id: u32,
    pub grpi3_attributes: u32,
    pub grpi3_next_index: u32,
}
impl windows_core::TypeKind for NET_DISPLAY_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_DISPLAY_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_DISPLAY_MACHINE {
    pub usri2_name: windows_core::PWSTR,
    pub usri2_comment: windows_core::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_user_id: u32,
    pub usri2_next_index: u32,
}
impl windows_core::TypeKind for NET_DISPLAY_MACHINE {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_DISPLAY_MACHINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_DISPLAY_USER {
    pub usri1_name: windows_core::PWSTR,
    pub usri1_comment: windows_core::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_full_name: windows_core::PWSTR,
    pub usri1_user_id: u32,
    pub usri1_next_index: u32,
}
impl windows_core::TypeKind for NET_DISPLAY_USER {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_DISPLAY_USER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub PasswordMatched: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_AUTHENTICATION_INPUT_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_OUTPUT_ARG {
    pub ChangedPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ValidationStatus: u32,
}
impl windows_core::TypeKind for NET_VALIDATE_OUTPUT_ARG {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_OUTPUT_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_core::PWSTR,
    pub UserAccountName: windows_core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMatch: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_PASSWORD_CHANGE_INPUT_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_HASH {
    pub Length: u32,
    pub Hash: *mut u8,
}
impl windows_core::TypeKind for NET_VALIDATE_PASSWORD_HASH {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_PASSWORD_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    pub InputPersistedFields: NET_VALIDATE_PERSISTED_FIELDS,
    pub ClearPassword: windows_core::PWSTR,
    pub UserAccountName: windows_core::PWSTR,
    pub HashedPassword: NET_VALIDATE_PASSWORD_HASH,
    pub PasswordMustChangeAtNextLogon: super::super::Foundation::BOOLEAN,
    pub ClearLockout: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_PASSWORD_RESET_INPUT_ARG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NET_VALIDATE_PERSISTED_FIELDS {
    pub PresentFields: u32,
    pub PasswordLastSet: super::super::Foundation::FILETIME,
    pub BadPasswordTime: super::super::Foundation::FILETIME,
    pub LockoutTime: super::super::Foundation::FILETIME,
    pub BadPasswordCount: u32,
    pub PasswordHistoryLength: u32,
    pub PasswordHistory: *mut NET_VALIDATE_PASSWORD_HASH,
}
impl windows_core::TypeKind for NET_VALIDATE_PERSISTED_FIELDS {
    type TypeKind = windows_core::CopyType;
}
impl Default for NET_VALIDATE_PERSISTED_FIELDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NetProvisioning: windows_core::GUID = windows_core::GUID::from_u128(0x2aa2b5fe_b846_4d07_810c_b21ee45320e3);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct OBO_TOKEN {
    pub Type: OBO_TOKEN_TYPE,
    pub pncc: core::mem::ManuallyDrop<Option<INetCfgComponent>>,
    pub pszwManufacturer: windows_core::PCWSTR,
    pub pszwProduct: windows_core::PCWSTR,
    pub pszwDisplayName: windows_core::PCWSTR,
    pub fRegistered: super::super::Foundation::BOOL,
}
impl Clone for OBO_TOKEN {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for OBO_TOKEN {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBO_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PRINT_OTHER_INFO {
    pub alrtpr_jobid: u32,
    pub alrtpr_status: u32,
    pub alrtpr_submitted: u32,
    pub alrtpr_size: u32,
}
impl windows_core::TypeKind for PRINT_OTHER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PRINT_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RASCON_IPUI {
    pub guidConnection: windows_core::GUID,
    pub fIPv6Cfg: super::super::Foundation::BOOL,
    pub dwFlags: u32,
    pub pszwIpAddr: [u16; 16],
    pub pszwDnsAddr: [u16; 16],
    pub pszwDns2Addr: [u16; 16],
    pub pszwWinsAddr: [u16; 16],
    pub pszwWins2Addr: [u16; 16],
    pub pszwDnsSuffix: [u16; 256],
    pub pszwIpv6Addr: [u16; 65],
    pub dwIpv6PrefixLength: u32,
    pub pszwIpv6DnsAddr: [u16; 65],
    pub pszwIpv6Dns2Addr: [u16; 65],
    pub dwIPv4InfMetric: u32,
    pub dwIPv6InfMetric: u32,
}
impl windows_core::TypeKind for RASCON_IPUI {
    type TypeKind = windows_core::CopyType;
}
impl Default for RASCON_IPUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_0 {
    pub rped0_dirname: windows_core::PWSTR,
}
impl windows_core::TypeKind for REPL_EDIR_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_EDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1 {
    pub rped1_dirname: windows_core::PWSTR,
    pub rped1_integrity: u32,
    pub rped1_extent: u32,
}
impl windows_core::TypeKind for REPL_EDIR_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_EDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1000 {
    pub rped1000_integrity: u32,
}
impl windows_core::TypeKind for REPL_EDIR_INFO_1000 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_EDIR_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_1001 {
    pub rped1001_extent: u32,
}
impl windows_core::TypeKind for REPL_EDIR_INFO_1001 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_EDIR_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_EDIR_INFO_2 {
    pub rped2_dirname: windows_core::PWSTR,
    pub rped2_integrity: u32,
    pub rped2_extent: u32,
    pub rped2_lockcount: u32,
    pub rped2_locktime: u32,
}
impl windows_core::TypeKind for REPL_EDIR_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_EDIR_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_IDIR_INFO_0 {
    pub rpid0_dirname: windows_core::PWSTR,
}
impl windows_core::TypeKind for REPL_IDIR_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_IDIR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_IDIR_INFO_1 {
    pub rpid1_dirname: windows_core::PWSTR,
    pub rpid1_state: u32,
    pub rpid1_mastername: windows_core::PWSTR,
    pub rpid1_last_update_time: u32,
    pub rpid1_lockcount: u32,
    pub rpid1_locktime: u32,
}
impl windows_core::TypeKind for REPL_IDIR_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_IDIR_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_INFO_0 {
    pub rp0_role: u32,
    pub rp0_exportpath: windows_core::PWSTR,
    pub rp0_exportlist: windows_core::PWSTR,
    pub rp0_importpath: windows_core::PWSTR,
    pub rp0_importlist: windows_core::PWSTR,
    pub rp0_logonusername: windows_core::PWSTR,
    pub rp0_interval: u32,
    pub rp0_pulse: u32,
    pub rp0_guardtime: u32,
    pub rp0_random: u32,
}
impl windows_core::TypeKind for REPL_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_INFO_1000 {
    pub rp1000_interval: u32,
}
impl windows_core::TypeKind for REPL_INFO_1000 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_INFO_1000 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_INFO_1001 {
    pub rp1001_pulse: u32,
}
impl windows_core::TypeKind for REPL_INFO_1001 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_INFO_1002 {
    pub rp1002_guardtime: u32,
}
impl windows_core::TypeKind for REPL_INFO_1002 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REPL_INFO_1003 {
    pub rp1003_random: u32,
}
impl windows_core::TypeKind for REPL_INFO_1003 {
    type TypeKind = windows_core::CopyType;
}
impl Default for REPL_INFO_1003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTR_INFO_BLOCK_HEADER {
    pub Version: u32,
    pub Size: u32,
    pub TocEntriesCount: u32,
    pub TocEntry: [RTR_TOC_ENTRY; 1],
}
impl windows_core::TypeKind for RTR_INFO_BLOCK_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTR_INFO_BLOCK_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTR_TOC_ENTRY {
    pub InfoType: u32,
    pub InfoSize: u32,
    pub Count: u32,
    pub Offset: u32,
}
impl windows_core::TypeKind for RTR_TOC_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for RTR_TOC_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_100 {
    pub sv100_platform_id: u32,
    pub sv100_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_100 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1005 {
    pub sv1005_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_1005 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_101 {
    pub sv101_platform_id: u32,
    pub sv101_name: windows_core::PWSTR,
    pub sv101_version_major: u32,
    pub sv101_version_minor: u32,
    pub sv101_type: NET_SERVER_TYPE,
    pub sv101_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_101 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1010 {
    pub sv1010_disc: i32,
}
impl windows_core::TypeKind for SERVER_INFO_1010 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1010 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1016 {
    pub sv1016_hidden: SERVER_INFO_HIDDEN,
}
impl windows_core::TypeKind for SERVER_INFO_1016 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1016 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1017 {
    pub sv1017_announce: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1017 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1017 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1018 {
    pub sv1018_anndelta: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1018 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1018 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_102 {
    pub sv102_platform_id: u32,
    pub sv102_name: windows_core::PWSTR,
    pub sv102_version_major: u32,
    pub sv102_version_minor: u32,
    pub sv102_type: NET_SERVER_TYPE,
    pub sv102_comment: windows_core::PWSTR,
    pub sv102_users: u32,
    pub sv102_disc: i32,
    pub sv102_hidden: SERVER_INFO_HIDDEN,
    pub sv102_announce: u32,
    pub sv102_anndelta: u32,
    pub sv102_licenses: u32,
    pub sv102_userpath: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_102 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_103 {
    pub sv103_platform_id: u32,
    pub sv103_name: windows_core::PWSTR,
    pub sv103_version_major: u32,
    pub sv103_version_minor: u32,
    pub sv103_type: u32,
    pub sv103_comment: windows_core::PWSTR,
    pub sv103_users: u32,
    pub sv103_disc: i32,
    pub sv103_hidden: super::super::Foundation::BOOL,
    pub sv103_announce: u32,
    pub sv103_anndelta: u32,
    pub sv103_licenses: u32,
    pub sv103_userpath: windows_core::PWSTR,
    pub sv103_capabilities: u32,
}
impl windows_core::TypeKind for SERVER_INFO_103 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_103 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1107 {
    pub sv1107_users: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1107 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1107 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1501 {
    pub sv1501_sessopens: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1501 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1501 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1502 {
    pub sv1502_sessvcs: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1502 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1503 {
    pub sv1503_opensearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1503 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1506 {
    pub sv1506_maxworkitems: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1506 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1506 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1509 {
    pub sv1509_maxrawbuflen: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1509 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1509 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1510 {
    pub sv1510_sessusers: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1510 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1510 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1511 {
    pub sv1511_sessconns: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1511 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1511 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1512 {
    pub sv1512_maxnonpagedmemoryusage: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1512 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1512 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1513 {
    pub sv1513_maxpagedmemoryusage: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1513 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1513 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1514 {
    pub sv1514_enablesoftcompat: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1514 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1514 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1515 {
    pub sv1515_enableforcedlogoff: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1515 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1515 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1516 {
    pub sv1516_timesource: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1516 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1516 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1518 {
    pub sv1518_lmannounce: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1518 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1518 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1520 {
    pub sv1520_maxcopyreadlen: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1520 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1520 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1521 {
    pub sv1521_maxcopywritelen: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1521 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1521 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1522 {
    pub sv1522_minkeepsearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1522 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1522 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1523 {
    pub sv1523_maxkeepsearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1523 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1523 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1524 {
    pub sv1524_minkeepcomplsearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1524 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1524 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1525 {
    pub sv1525_maxkeepcomplsearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1525 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1525 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1528 {
    pub sv1528_scavtimeout: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1528 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1528 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1529 {
    pub sv1529_minrcvqueue: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1529 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1529 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1530 {
    pub sv1530_minfreeworkitems: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1530 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1530 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1533 {
    pub sv1533_maxmpxct: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1533 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1533 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1534 {
    pub sv1534_oplockbreakwait: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1534 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1534 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1535 {
    pub sv1535_oplockbreakresponsewait: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1535 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1535 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1536 {
    pub sv1536_enableoplocks: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1536 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1536 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1537 {
    pub sv1537_enableoplockforceclose: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1537 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1537 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1538 {
    pub sv1538_enablefcbopens: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1538 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1538 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1539 {
    pub sv1539_enableraw: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1539 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1539 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1540 {
    pub sv1540_enablesharednetdrives: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1540 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1540 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1541 {
    pub sv1541_minfreeconnections: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1541 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1541 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1542 {
    pub sv1542_maxfreeconnections: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1542 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1542 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1543 {
    pub sv1543_initsesstable: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1543 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1543 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1544 {
    pub sv1544_initconntable: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1544 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1544 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1545 {
    pub sv1545_initfiletable: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1545 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1545 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1546 {
    pub sv1546_initsearchtable: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1546 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1546 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1547 {
    pub sv1547_alertschedule: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1547 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1547 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1548 {
    pub sv1548_errorthreshold: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1548 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1548 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1549 {
    pub sv1549_networkerrorthreshold: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1549 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1549 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1550 {
    pub sv1550_diskspacethreshold: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1550 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1550 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1552 {
    pub sv1552_maxlinkdelay: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1552 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1552 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1553 {
    pub sv1553_minlinkthroughput: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1553 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1553 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1554 {
    pub sv1554_linkinfovalidtime: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1554 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1554 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1555 {
    pub sv1555_scavqosinfoupdatetime: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1555 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1555 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1556 {
    pub sv1556_maxworkitemidletime: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1556 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1556 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1557 {
    pub sv1557_maxrawworkitems: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1557 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1557 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1560 {
    pub sv1560_producttype: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1560 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1560 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1561 {
    pub sv1561_serversize: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1561 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1561 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1562 {
    pub sv1562_connectionlessautodisc: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1562 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1562 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1563 {
    pub sv1563_sharingviolationretries: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1563 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1563 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1564 {
    pub sv1564_sharingviolationdelay: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1564 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1564 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1565 {
    pub sv1565_maxglobalopensearch: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1565 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1565 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1566 {
    pub sv1566_removeduplicatesearches: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1566 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1566 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1567 {
    pub sv1567_lockviolationretries: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1567 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1567 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1568 {
    pub sv1568_lockviolationoffset: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1568 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1568 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1569 {
    pub sv1569_lockviolationdelay: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1569 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1569 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1570 {
    pub sv1570_mdlreadswitchover: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1570 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1570 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1571 {
    pub sv1571_cachedopenlimit: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1571 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1571 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1572 {
    pub sv1572_criticalthreads: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1572 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1572 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1573 {
    pub sv1573_restrictnullsessaccess: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1573 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1573 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1574 {
    pub sv1574_enablewfw311directipx: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1574 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1574 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1575 {
    pub sv1575_otherqueueaffinity: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1575 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1575 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1576 {
    pub sv1576_queuesamplesecs: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1576 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1576 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1577 {
    pub sv1577_balancecount: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1577 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1577 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1578 {
    pub sv1578_preferredaffinity: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1578 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1578 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1579 {
    pub sv1579_maxfreerfcbs: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1579 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1579 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1580 {
    pub sv1580_maxfreemfcbs: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1580 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1580 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1581 {
    pub sv1581_maxfreemlcbs: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1581 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1581 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1582 {
    pub sv1582_maxfreepagedpoolchunks: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1582 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1582 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1583 {
    pub sv1583_minpagedpoolchunksize: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1583 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1583 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1584 {
    pub sv1584_maxpagedpoolchunksize: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1584 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1584 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1585 {
    pub sv1585_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1585 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1585 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1586 {
    pub sv1586_maxthreadsperqueue: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1586 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1586 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1587 {
    pub sv1587_cacheddirectorylimit: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1587 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1587 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1588 {
    pub sv1588_maxcopylength: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1588 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1588 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1590 {
    pub sv1590_enablecompression: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1590 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1590 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1591 {
    pub sv1591_autosharewks: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1591 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1591 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1592 {
    pub sv1592_autosharewks: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1592 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1592 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1593 {
    pub sv1593_enablesecuritysignature: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1593 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1593 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1594 {
    pub sv1594_requiresecuritysignature: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1594 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1594 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1595 {
    pub sv1595_minclientbuffersize: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1595 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1595 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1596 {
    pub sv1596_ConnectionNoSessionsTimeout: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1596 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1596 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1597 {
    pub sv1597_IdleThreadTimeOut: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1597 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1597 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1598 {
    pub sv1598_enableW9xsecuritysignature: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1598 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1599 {
    pub sv1598_enforcekerberosreauthentication: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SERVER_INFO_1599 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1600 {
    pub sv1598_disabledos: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for SERVER_INFO_1600 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1601 {
    pub sv1598_lowdiskspaceminimum: u32,
}
impl windows_core::TypeKind for SERVER_INFO_1601 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1601 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_1602 {
    pub sv_1598_disablestrictnamechecking: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_1602 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_1602 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_402 {
    pub sv402_ulist_mtime: u32,
    pub sv402_glist_mtime: u32,
    pub sv402_alist_mtime: u32,
    pub sv402_alerts: windows_core::PWSTR,
    pub sv402_security: SERVER_INFO_SECURITY,
    pub sv402_numadmin: u32,
    pub sv402_lanmask: u32,
    pub sv402_guestacct: windows_core::PWSTR,
    pub sv402_chdevs: u32,
    pub sv402_chdevq: u32,
    pub sv402_chdevjobs: u32,
    pub sv402_connections: u32,
    pub sv402_shares: u32,
    pub sv402_openfiles: u32,
    pub sv402_sessopens: u32,
    pub sv402_sessvcs: u32,
    pub sv402_sessreqs: u32,
    pub sv402_opensearch: u32,
    pub sv402_activelocks: u32,
    pub sv402_numreqbuf: u32,
    pub sv402_sizreqbuf: u32,
    pub sv402_numbigbuf: u32,
    pub sv402_numfiletasks: u32,
    pub sv402_alertsched: u32,
    pub sv402_erroralert: u32,
    pub sv402_logonalert: u32,
    pub sv402_accessalert: u32,
    pub sv402_diskalert: u32,
    pub sv402_netioalert: u32,
    pub sv402_maxauditsz: u32,
    pub sv402_srvheuristics: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_402 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_402 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_403 {
    pub sv403_ulist_mtime: u32,
    pub sv403_glist_mtime: u32,
    pub sv403_alist_mtime: u32,
    pub sv403_alerts: windows_core::PWSTR,
    pub sv403_security: SERVER_INFO_SECURITY,
    pub sv403_numadmin: u32,
    pub sv403_lanmask: u32,
    pub sv403_guestacct: windows_core::PWSTR,
    pub sv403_chdevs: u32,
    pub sv403_chdevq: u32,
    pub sv403_chdevjobs: u32,
    pub sv403_connections: u32,
    pub sv403_shares: u32,
    pub sv403_openfiles: u32,
    pub sv403_sessopens: u32,
    pub sv403_sessvcs: u32,
    pub sv403_sessreqs: u32,
    pub sv403_opensearch: u32,
    pub sv403_activelocks: u32,
    pub sv403_numreqbuf: u32,
    pub sv403_sizreqbuf: u32,
    pub sv403_numbigbuf: u32,
    pub sv403_numfiletasks: u32,
    pub sv403_alertsched: u32,
    pub sv403_erroralert: u32,
    pub sv403_logonalert: u32,
    pub sv403_accessalert: u32,
    pub sv403_diskalert: u32,
    pub sv403_netioalert: u32,
    pub sv403_maxauditsz: u32,
    pub sv403_srvheuristics: windows_core::PWSTR,
    pub sv403_auditedevents: u32,
    pub sv403_autoprofile: u32,
    pub sv403_autopath: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_INFO_403 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_403 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_502 {
    pub sv502_sessopens: u32,
    pub sv502_sessvcs: u32,
    pub sv502_opensearch: u32,
    pub sv502_sizreqbuf: u32,
    pub sv502_initworkitems: u32,
    pub sv502_maxworkitems: u32,
    pub sv502_rawworkitems: u32,
    pub sv502_irpstacksize: u32,
    pub sv502_maxrawbuflen: u32,
    pub sv502_sessusers: u32,
    pub sv502_sessconns: u32,
    pub sv502_maxpagedmemoryusage: u32,
    pub sv502_maxnonpagedmemoryusage: u32,
    pub sv502_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv502_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv502_timesource: super::super::Foundation::BOOL,
    pub sv502_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv502_lmannounce: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_502 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_503 {
    pub sv503_sessopens: u32,
    pub sv503_sessvcs: u32,
    pub sv503_opensearch: u32,
    pub sv503_sizreqbuf: u32,
    pub sv503_initworkitems: u32,
    pub sv503_maxworkitems: u32,
    pub sv503_rawworkitems: u32,
    pub sv503_irpstacksize: u32,
    pub sv503_maxrawbuflen: u32,
    pub sv503_sessusers: u32,
    pub sv503_sessconns: u32,
    pub sv503_maxpagedmemoryusage: u32,
    pub sv503_maxnonpagedmemoryusage: u32,
    pub sv503_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv503_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv503_timesource: super::super::Foundation::BOOL,
    pub sv503_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv503_lmannounce: super::super::Foundation::BOOL,
    pub sv503_domain: windows_core::PWSTR,
    pub sv503_maxcopyreadlen: u32,
    pub sv503_maxcopywritelen: u32,
    pub sv503_minkeepsearch: u32,
    pub sv503_maxkeepsearch: u32,
    pub sv503_minkeepcomplsearch: u32,
    pub sv503_maxkeepcomplsearch: u32,
    pub sv503_threadcountadd: u32,
    pub sv503_numblockthreads: u32,
    pub sv503_scavtimeout: u32,
    pub sv503_minrcvqueue: u32,
    pub sv503_minfreeworkitems: u32,
    pub sv503_xactmemsize: u32,
    pub sv503_threadpriority: u32,
    pub sv503_maxmpxct: u32,
    pub sv503_oplockbreakwait: u32,
    pub sv503_oplockbreakresponsewait: u32,
    pub sv503_enableoplocks: super::super::Foundation::BOOL,
    pub sv503_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv503_enablefcbopens: super::super::Foundation::BOOL,
    pub sv503_enableraw: super::super::Foundation::BOOL,
    pub sv503_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv503_minfreeconnections: u32,
    pub sv503_maxfreeconnections: u32,
}
impl windows_core::TypeKind for SERVER_INFO_503 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_503 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_598 {
    pub sv598_maxrawworkitems: u32,
    pub sv598_maxthreadsperqueue: u32,
    pub sv598_producttype: u32,
    pub sv598_serversize: u32,
    pub sv598_connectionlessautodisc: u32,
    pub sv598_sharingviolationretries: u32,
    pub sv598_sharingviolationdelay: u32,
    pub sv598_maxglobalopensearch: u32,
    pub sv598_removeduplicatesearches: u32,
    pub sv598_lockviolationoffset: u32,
    pub sv598_lockviolationdelay: u32,
    pub sv598_mdlreadswitchover: u32,
    pub sv598_cachedopenlimit: u32,
    pub sv598_otherqueueaffinity: u32,
    pub sv598_restrictnullsessaccess: super::super::Foundation::BOOL,
    pub sv598_enablewfw311directipx: super::super::Foundation::BOOL,
    pub sv598_queuesamplesecs: u32,
    pub sv598_balancecount: u32,
    pub sv598_preferredaffinity: u32,
    pub sv598_maxfreerfcbs: u32,
    pub sv598_maxfreemfcbs: u32,
    pub sv598_maxfreelfcbs: u32,
    pub sv598_maxfreepagedpoolchunks: u32,
    pub sv598_minpagedpoolchunksize: u32,
    pub sv598_maxpagedpoolchunksize: u32,
    pub sv598_sendsfrompreferredprocessor: super::super::Foundation::BOOL,
    pub sv598_cacheddirectorylimit: u32,
    pub sv598_maxcopylength: u32,
    pub sv598_enablecompression: super::super::Foundation::BOOL,
    pub sv598_autosharewks: super::super::Foundation::BOOL,
    pub sv598_autoshareserver: super::super::Foundation::BOOL,
    pub sv598_enablesecuritysignature: super::super::Foundation::BOOL,
    pub sv598_requiresecuritysignature: super::super::Foundation::BOOL,
    pub sv598_minclientbuffersize: u32,
    pub sv598_serverguid: windows_core::GUID,
    pub sv598_ConnectionNoSessionsTimeout: u32,
    pub sv598_IdleThreadTimeOut: u32,
    pub sv598_enableW9xsecuritysignature: super::super::Foundation::BOOL,
    pub sv598_enforcekerberosreauthentication: super::super::Foundation::BOOL,
    pub sv598_disabledos: super::super::Foundation::BOOL,
    pub sv598_lowdiskspaceminimum: u32,
    pub sv598_disablestrictnamechecking: super::super::Foundation::BOOL,
    pub sv598_enableauthenticateusersharing: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for SERVER_INFO_598 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_598 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_INFO_599 {
    pub sv599_sessopens: u32,
    pub sv599_sessvcs: u32,
    pub sv599_opensearch: u32,
    pub sv599_sizreqbuf: u32,
    pub sv599_initworkitems: u32,
    pub sv599_maxworkitems: u32,
    pub sv599_rawworkitems: u32,
    pub sv599_irpstacksize: u32,
    pub sv599_maxrawbuflen: u32,
    pub sv599_sessusers: u32,
    pub sv599_sessconns: u32,
    pub sv599_maxpagedmemoryusage: u32,
    pub sv599_maxnonpagedmemoryusage: u32,
    pub sv599_enablesoftcompat: super::super::Foundation::BOOL,
    pub sv599_enableforcedlogoff: super::super::Foundation::BOOL,
    pub sv599_timesource: super::super::Foundation::BOOL,
    pub sv599_acceptdownlevelapis: super::super::Foundation::BOOL,
    pub sv599_lmannounce: super::super::Foundation::BOOL,
    pub sv599_domain: windows_core::PWSTR,
    pub sv599_maxcopyreadlen: u32,
    pub sv599_maxcopywritelen: u32,
    pub sv599_minkeepsearch: u32,
    pub sv599_maxkeepsearch: u32,
    pub sv599_minkeepcomplsearch: u32,
    pub sv599_maxkeepcomplsearch: u32,
    pub sv599_threadcountadd: u32,
    pub sv599_numblockthreads: u32,
    pub sv599_scavtimeout: u32,
    pub sv599_minrcvqueue: u32,
    pub sv599_minfreeworkitems: u32,
    pub sv599_xactmemsize: u32,
    pub sv599_threadpriority: u32,
    pub sv599_maxmpxct: u32,
    pub sv599_oplockbreakwait: u32,
    pub sv599_oplockbreakresponsewait: u32,
    pub sv599_enableoplocks: super::super::Foundation::BOOL,
    pub sv599_enableoplockforceclose: super::super::Foundation::BOOL,
    pub sv599_enablefcbopens: super::super::Foundation::BOOL,
    pub sv599_enableraw: super::super::Foundation::BOOL,
    pub sv599_enablesharednetdrives: super::super::Foundation::BOOL,
    pub sv599_minfreeconnections: u32,
    pub sv599_maxfreeconnections: u32,
    pub sv599_initsesstable: u32,
    pub sv599_initconntable: u32,
    pub sv599_initfiletable: u32,
    pub sv599_initsearchtable: u32,
    pub sv599_alertschedule: u32,
    pub sv599_errorthreshold: u32,
    pub sv599_networkerrorthreshold: u32,
    pub sv599_diskspacethreshold: u32,
    pub sv599_reserved: u32,
    pub sv599_maxlinkdelay: u32,
    pub sv599_minlinkthroughput: u32,
    pub sv599_linkinfovalidtime: u32,
    pub sv599_scavqosinfoupdatetime: u32,
    pub sv599_maxworkitemidletime: u32,
}
impl windows_core::TypeKind for SERVER_INFO_599 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_INFO_599 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_0 {
    pub svti0_numberofvcs: u32,
    pub svti0_transportname: windows_core::PWSTR,
    pub svti0_transportaddress: *mut u8,
    pub svti0_transportaddresslength: u32,
    pub svti0_networkaddress: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_TRANSPORT_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_1 {
    pub svti1_numberofvcs: u32,
    pub svti1_transportname: windows_core::PWSTR,
    pub svti1_transportaddress: *mut u8,
    pub svti1_transportaddresslength: u32,
    pub svti1_networkaddress: windows_core::PWSTR,
    pub svti1_domain: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVER_TRANSPORT_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_TRANSPORT_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_2 {
    pub svti2_numberofvcs: u32,
    pub svti2_transportname: windows_core::PWSTR,
    pub svti2_transportaddress: *mut u8,
    pub svti2_transportaddresslength: u32,
    pub svti2_networkaddress: windows_core::PWSTR,
    pub svti2_domain: windows_core::PWSTR,
    pub svti2_flags: u32,
}
impl windows_core::TypeKind for SERVER_TRANSPORT_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_TRANSPORT_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVER_TRANSPORT_INFO_3 {
    pub svti3_numberofvcs: u32,
    pub svti3_transportname: windows_core::PWSTR,
    pub svti3_transportaddress: *mut u8,
    pub svti3_transportaddresslength: u32,
    pub svti3_networkaddress: windows_core::PWSTR,
    pub svti3_domain: windows_core::PWSTR,
    pub svti3_flags: u32,
    pub svti3_passwordlength: u32,
    pub svti3_password: [u8; 256],
}
impl windows_core::TypeKind for SERVER_TRANSPORT_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVER_TRANSPORT_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_INFO_0 {
    pub svci0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVICE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_INFO_1 {
    pub svci1_name: windows_core::PWSTR,
    pub svci1_status: u32,
    pub svci1_code: u32,
    pub svci1_pid: u32,
}
impl windows_core::TypeKind for SERVICE_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERVICE_INFO_2 {
    pub svci2_name: windows_core::PWSTR,
    pub svci2_status: u32,
    pub svci2_code: u32,
    pub svci2_pid: u32,
    pub svci2_text: windows_core::PWSTR,
    pub svci2_specific_error: u32,
    pub svci2_display_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for SERVICE_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for SERVICE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SMB_COMPRESSION_INFO {
    pub Switch: super::super::Foundation::BOOLEAN,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
impl windows_core::TypeKind for SMB_COMPRESSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SMB_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SMB_TREE_CONNECT_PARAMETERS {
    pub EABufferOffset: u32,
    pub EABufferLen: u32,
    pub CreateOptions: u32,
    pub TreeConnectAttributes: u32,
}
impl windows_core::TypeKind for SMB_TREE_CONNECT_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SMB_TREE_CONNECT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl windows_core::TypeKind for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for SMB_USE_OPTION_COMPRESSION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STD_ALERT {
    pub alrt_timestamp: u32,
    pub alrt_eventname: [u16; 17],
    pub alrt_servicename: [u16; 81],
}
impl windows_core::TypeKind for STD_ALERT {
    type TypeKind = windows_core::CopyType;
}
impl Default for STD_ALERT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIME_OF_DAY_INFO {
    pub tod_elapsedt: u32,
    pub tod_msecs: u32,
    pub tod_hours: u32,
    pub tod_mins: u32,
    pub tod_secs: u32,
    pub tod_hunds: u32,
    pub tod_timezone: i32,
    pub tod_tinterval: u32,
    pub tod_day: u32,
    pub tod_month: u32,
    pub tod_year: u32,
    pub tod_weekday: u32,
}
impl windows_core::TypeKind for TIME_OF_DAY_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for TIME_OF_DAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSPORT_INFO {
    pub Type: TRANSPORT_TYPE,
    pub SkipCertificateCheck: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for TRANSPORT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for TRANSPORT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_0 {
    pub usri0_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1 {
    pub usri1_name: windows_core::PWSTR,
    pub usri1_password: windows_core::PWSTR,
    pub usri1_password_age: u32,
    pub usri1_priv: USER_PRIV,
    pub usri1_home_dir: windows_core::PWSTR,
    pub usri1_comment: windows_core::PWSTR,
    pub usri1_flags: USER_ACCOUNT_FLAGS,
    pub usri1_script_path: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_10 {
    pub usri10_name: windows_core::PWSTR,
    pub usri10_comment: windows_core::PWSTR,
    pub usri10_usr_comment: windows_core::PWSTR,
    pub usri10_full_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_10 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1003 {
    pub usri1003_password: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1003 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1005 {
    pub usri1005_priv: USER_PRIV,
}
impl windows_core::TypeKind for USER_INFO_1005 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1006 {
    pub usri1006_home_dir: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1006 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1006 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1007 {
    pub usri1007_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1007 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1007 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1008 {
    pub usri1008_flags: USER_ACCOUNT_FLAGS,
}
impl windows_core::TypeKind for USER_INFO_1008 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1008 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1009 {
    pub usri1009_script_path: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1009 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1009 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1010 {
    pub usri1010_auth_flags: AF_OP,
}
impl windows_core::TypeKind for USER_INFO_1010 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1010 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1011 {
    pub usri1011_full_name: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1011 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1011 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1012 {
    pub usri1012_usr_comment: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1012 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1012 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1013 {
    pub usri1013_parms: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1013 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1013 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1014 {
    pub usri1014_workstations: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1014 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1014 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1017 {
    pub usri1017_acct_expires: u32,
}
impl windows_core::TypeKind for USER_INFO_1017 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1017 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1018 {
    pub usri1018_max_storage: u32,
}
impl windows_core::TypeKind for USER_INFO_1018 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1018 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1020 {
    pub usri1020_units_per_week: u32,
    pub usri1020_logon_hours: *mut u8,
}
impl windows_core::TypeKind for USER_INFO_1020 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1020 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1023 {
    pub usri1023_logon_server: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1023 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1023 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1024 {
    pub usri1024_country_code: u32,
}
impl windows_core::TypeKind for USER_INFO_1024 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1024 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1025 {
    pub usri1025_code_page: u32,
}
impl windows_core::TypeKind for USER_INFO_1025 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1025 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1051 {
    pub usri1051_primary_group_id: u32,
}
impl windows_core::TypeKind for USER_INFO_1051 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1051 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1052 {
    pub usri1052_profile: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1052 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1052 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_1053 {
    pub usri1053_home_dir_drive: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_INFO_1053 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_1053 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_11 {
    pub usri11_name: windows_core::PWSTR,
    pub usri11_comment: windows_core::PWSTR,
    pub usri11_usr_comment: windows_core::PWSTR,
    pub usri11_full_name: windows_core::PWSTR,
    pub usri11_priv: USER_PRIV,
    pub usri11_auth_flags: AF_OP,
    pub usri11_password_age: u32,
    pub usri11_home_dir: windows_core::PWSTR,
    pub usri11_parms: windows_core::PWSTR,
    pub usri11_last_logon: u32,
    pub usri11_last_logoff: u32,
    pub usri11_bad_pw_count: u32,
    pub usri11_num_logons: u32,
    pub usri11_logon_server: windows_core::PWSTR,
    pub usri11_country_code: u32,
    pub usri11_workstations: windows_core::PWSTR,
    pub usri11_max_storage: u32,
    pub usri11_units_per_week: u32,
    pub usri11_logon_hours: *mut u8,
    pub usri11_code_page: u32,
}
impl windows_core::TypeKind for USER_INFO_11 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_2 {
    pub usri2_name: windows_core::PWSTR,
    pub usri2_password: windows_core::PWSTR,
    pub usri2_password_age: u32,
    pub usri2_priv: USER_PRIV,
    pub usri2_home_dir: windows_core::PWSTR,
    pub usri2_comment: windows_core::PWSTR,
    pub usri2_flags: USER_ACCOUNT_FLAGS,
    pub usri2_script_path: windows_core::PWSTR,
    pub usri2_auth_flags: AF_OP,
    pub usri2_full_name: windows_core::PWSTR,
    pub usri2_usr_comment: windows_core::PWSTR,
    pub usri2_parms: windows_core::PWSTR,
    pub usri2_workstations: windows_core::PWSTR,
    pub usri2_last_logon: u32,
    pub usri2_last_logoff: u32,
    pub usri2_acct_expires: u32,
    pub usri2_max_storage: u32,
    pub usri2_units_per_week: u32,
    pub usri2_logon_hours: *mut u8,
    pub usri2_bad_pw_count: u32,
    pub usri2_num_logons: u32,
    pub usri2_logon_server: windows_core::PWSTR,
    pub usri2_country_code: u32,
    pub usri2_code_page: u32,
}
impl windows_core::TypeKind for USER_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_20 {
    pub usri20_name: windows_core::PWSTR,
    pub usri20_full_name: windows_core::PWSTR,
    pub usri20_comment: windows_core::PWSTR,
    pub usri20_flags: USER_ACCOUNT_FLAGS,
    pub usri20_user_id: u32,
}
impl windows_core::TypeKind for USER_INFO_20 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_21 {
    pub usri21_password: [u8; 16],
}
impl windows_core::TypeKind for USER_INFO_21 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_22 {
    pub usri22_name: windows_core::PWSTR,
    pub usri22_password: [u8; 16],
    pub usri22_password_age: u32,
    pub usri22_priv: USER_PRIV,
    pub usri22_home_dir: windows_core::PWSTR,
    pub usri22_comment: windows_core::PWSTR,
    pub usri22_flags: USER_ACCOUNT_FLAGS,
    pub usri22_script_path: windows_core::PWSTR,
    pub usri22_auth_flags: AF_OP,
    pub usri22_full_name: windows_core::PWSTR,
    pub usri22_usr_comment: windows_core::PWSTR,
    pub usri22_parms: windows_core::PWSTR,
    pub usri22_workstations: windows_core::PWSTR,
    pub usri22_last_logon: u32,
    pub usri22_last_logoff: u32,
    pub usri22_acct_expires: u32,
    pub usri22_max_storage: u32,
    pub usri22_units_per_week: u32,
    pub usri22_logon_hours: *mut u8,
    pub usri22_bad_pw_count: u32,
    pub usri22_num_logons: u32,
    pub usri22_logon_server: windows_core::PWSTR,
    pub usri22_country_code: u32,
    pub usri22_code_page: u32,
}
impl windows_core::TypeKind for USER_INFO_22 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_23 {
    pub usri23_name: windows_core::PWSTR,
    pub usri23_full_name: windows_core::PWSTR,
    pub usri23_comment: windows_core::PWSTR,
    pub usri23_flags: USER_ACCOUNT_FLAGS,
    pub usri23_user_sid: super::super::Foundation::PSID,
}
impl windows_core::TypeKind for USER_INFO_23 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_24 {
    pub usri24_internet_identity: super::super::Foundation::BOOL,
    pub usri24_flags: u32,
    pub usri24_internet_provider_name: windows_core::PWSTR,
    pub usri24_internet_principal_name: windows_core::PWSTR,
    pub usri24_user_sid: super::super::Foundation::PSID,
}
impl windows_core::TypeKind for USER_INFO_24 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_3 {
    pub usri3_name: windows_core::PWSTR,
    pub usri3_password: windows_core::PWSTR,
    pub usri3_password_age: u32,
    pub usri3_priv: USER_PRIV,
    pub usri3_home_dir: windows_core::PWSTR,
    pub usri3_comment: windows_core::PWSTR,
    pub usri3_flags: USER_ACCOUNT_FLAGS,
    pub usri3_script_path: windows_core::PWSTR,
    pub usri3_auth_flags: AF_OP,
    pub usri3_full_name: windows_core::PWSTR,
    pub usri3_usr_comment: windows_core::PWSTR,
    pub usri3_parms: windows_core::PWSTR,
    pub usri3_workstations: windows_core::PWSTR,
    pub usri3_last_logon: u32,
    pub usri3_last_logoff: u32,
    pub usri3_acct_expires: u32,
    pub usri3_max_storage: u32,
    pub usri3_units_per_week: u32,
    pub usri3_logon_hours: *mut u8,
    pub usri3_bad_pw_count: u32,
    pub usri3_num_logons: u32,
    pub usri3_logon_server: windows_core::PWSTR,
    pub usri3_country_code: u32,
    pub usri3_code_page: u32,
    pub usri3_user_id: u32,
    pub usri3_primary_group_id: u32,
    pub usri3_profile: windows_core::PWSTR,
    pub usri3_home_dir_drive: windows_core::PWSTR,
    pub usri3_password_expired: u32,
}
impl windows_core::TypeKind for USER_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_INFO_4 {
    pub usri4_name: windows_core::PWSTR,
    pub usri4_password: windows_core::PWSTR,
    pub usri4_password_age: u32,
    pub usri4_priv: USER_PRIV,
    pub usri4_home_dir: windows_core::PWSTR,
    pub usri4_comment: windows_core::PWSTR,
    pub usri4_flags: USER_ACCOUNT_FLAGS,
    pub usri4_script_path: windows_core::PWSTR,
    pub usri4_auth_flags: AF_OP,
    pub usri4_full_name: windows_core::PWSTR,
    pub usri4_usr_comment: windows_core::PWSTR,
    pub usri4_parms: windows_core::PWSTR,
    pub usri4_workstations: windows_core::PWSTR,
    pub usri4_last_logon: u32,
    pub usri4_last_logoff: u32,
    pub usri4_acct_expires: u32,
    pub usri4_max_storage: u32,
    pub usri4_units_per_week: u32,
    pub usri4_logon_hours: *mut u8,
    pub usri4_bad_pw_count: u32,
    pub usri4_num_logons: u32,
    pub usri4_logon_server: windows_core::PWSTR,
    pub usri4_country_code: u32,
    pub usri4_code_page: u32,
    pub usri4_user_sid: super::super::Foundation::PSID,
    pub usri4_primary_group_id: u32,
    pub usri4_profile: windows_core::PWSTR,
    pub usri4_home_dir_drive: windows_core::PWSTR,
    pub usri4_password_expired: u32,
}
impl windows_core::TypeKind for USER_INFO_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_0 {
    pub usrmod0_min_passwd_len: u32,
    pub usrmod0_max_passwd_age: u32,
    pub usrmod0_min_passwd_age: u32,
    pub usrmod0_force_logoff: u32,
    pub usrmod0_password_hist_len: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1 {
    pub usrmod1_role: u32,
    pub usrmod1_primary: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1001 {
    pub usrmod1001_min_passwd_len: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1001 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1001 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1002 {
    pub usrmod1002_max_passwd_age: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1002 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1002 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1003 {
    pub usrmod1003_min_passwd_age: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1003 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1003 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1004 {
    pub usrmod1004_force_logoff: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1004 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1004 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1005 {
    pub usrmod1005_password_hist_len: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1005 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1005 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1006 {
    pub usrmod1006_role: USER_MODALS_ROLES,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1006 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1006 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_1007 {
    pub usrmod1007_primary: windows_core::PWSTR,
}
impl windows_core::TypeKind for USER_MODALS_INFO_1007 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_1007 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_2 {
    pub usrmod2_domain_name: windows_core::PWSTR,
    pub usrmod2_domain_id: super::super::Foundation::PSID,
}
impl windows_core::TypeKind for USER_MODALS_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_MODALS_INFO_3 {
    pub usrmod3_lockout_duration: u32,
    pub usrmod3_lockout_observation_window: u32,
    pub usrmod3_lockout_threshold: u32,
}
impl windows_core::TypeKind for USER_MODALS_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_MODALS_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USER_OTHER_INFO {
    pub alrtus_errcode: u32,
    pub alrtus_numstrings: u32,
}
impl windows_core::TypeKind for USER_OTHER_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for USER_OTHER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_0 {
    pub ui0_local: windows_core::PWSTR,
    pub ui0_remote: windows_core::PWSTR,
}
impl windows_core::TypeKind for USE_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_1 {
    pub ui1_local: windows_core::PWSTR,
    pub ui1_remote: windows_core::PWSTR,
    pub ui1_password: windows_core::PWSTR,
    pub ui1_status: u32,
    pub ui1_asg_type: USE_INFO_ASG_TYPE,
    pub ui1_refcount: u32,
    pub ui1_usecount: u32,
}
impl windows_core::TypeKind for USE_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_2 {
    pub ui2_local: windows_core::PWSTR,
    pub ui2_remote: windows_core::PWSTR,
    pub ui2_password: windows_core::PWSTR,
    pub ui2_status: u32,
    pub ui2_asg_type: USE_INFO_ASG_TYPE,
    pub ui2_refcount: u32,
    pub ui2_usecount: u32,
    pub ui2_username: windows_core::PWSTR,
    pub ui2_domainname: windows_core::PWSTR,
}
impl windows_core::TypeKind for USE_INFO_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_3 {
    pub ui3_ui2: USE_INFO_2,
    pub ui3_flags: u32,
}
impl windows_core::TypeKind for USE_INFO_3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_4 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
}
impl windows_core::TypeKind for USE_INFO_4 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_INFO_5 {
    pub ui4_ui3: USE_INFO_3,
    pub ui4_auth_identity_length: u32,
    pub ui4_auth_identity: *mut u8,
    pub ui5_security_descriptor_length: u32,
    pub ui5_security_descriptor: *mut u8,
    pub ui5_use_options_length: u32,
    pub ui5_use_options: *mut u8,
}
impl windows_core::TypeKind for USE_INFO_5 {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_INFO_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl windows_core::TypeKind for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_OPTION_DEFERRED_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_OPTION_GENERIC {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl windows_core::TypeKind for USE_OPTION_GENERIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_OPTION_GENERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_OPTION_PROPERTIES {
    pub Tag: u32,
    pub pInfo: *mut core::ffi::c_void,
    pub Length: usize,
}
impl windows_core::TypeKind for USE_OPTION_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_OPTION_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct USE_OPTION_TRANSPORT_PARAMETERS {
    pub Tag: u32,
    pub Length: u16,
    pub Reserved: u16,
}
impl windows_core::TypeKind for USE_OPTION_TRANSPORT_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
impl Default for USE_OPTION_TRANSPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_100 {
    pub wki100_platform_id: u32,
    pub wki100_computername: windows_core::PWSTR,
    pub wki100_langroup: windows_core::PWSTR,
    pub wki100_ver_major: u32,
    pub wki100_ver_minor: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_100 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_100 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_101 {
    pub wki101_platform_id: u32,
    pub wki101_computername: windows_core::PWSTR,
    pub wki101_langroup: windows_core::PWSTR,
    pub wki101_ver_major: u32,
    pub wki101_ver_minor: u32,
    pub wki101_lanroot: windows_core::PWSTR,
}
impl windows_core::TypeKind for WKSTA_INFO_101 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1010 {
    pub wki1010_char_wait: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1010 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1010 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1011 {
    pub wki1011_collection_time: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1011 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1011 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1012 {
    pub wki1012_maximum_collection_count: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1012 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1012 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1013 {
    pub wki1013_keep_conn: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1013 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1013 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1018 {
    pub wki1018_sess_timeout: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1018 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1018 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_102 {
    pub wki102_platform_id: u32,
    pub wki102_computername: windows_core::PWSTR,
    pub wki102_langroup: windows_core::PWSTR,
    pub wki102_ver_major: u32,
    pub wki102_ver_minor: u32,
    pub wki102_lanroot: windows_core::PWSTR,
    pub wki102_logged_on_users: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_102 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_102 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1023 {
    pub wki1023_siz_char_buf: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1023 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1023 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1027 {
    pub wki1027_errlog_sz: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1027 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1027 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1028 {
    pub wki1028_print_buf_time: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1028 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1028 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1032 {
    pub wki1032_wrk_heuristics: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1032 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1032 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1033 {
    pub wki1033_max_threads: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1033 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1033 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1041 {
    pub wki1041_lock_quota: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1041 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1041 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1042 {
    pub wki1042_lock_increment: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1042 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1042 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1043 {
    pub wki1043_lock_maximum: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1043 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1043 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1044 {
    pub wki1044_pipe_increment: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1044 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1044 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1045 {
    pub wki1045_pipe_maximum: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1045 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1045 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1046 {
    pub wki1046_dormant_file_limit: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1046 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1046 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1047 {
    pub wki1047_cache_file_timeout: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1047 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1047 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1048 {
    pub wki1048_use_opportunistic_locking: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1048 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1048 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1049 {
    pub wki1049_use_unlock_behind: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1049 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1049 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1050 {
    pub wki1050_use_close_behind: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1050 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1050 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1051 {
    pub wki1051_buf_named_pipes: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1051 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1051 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1052 {
    pub wki1052_use_lock_read_unlock: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1052 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1052 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1053 {
    pub wki1053_utilize_nt_caching: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1053 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1053 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1054 {
    pub wki1054_use_raw_read: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1054 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1054 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1055 {
    pub wki1055_use_raw_write: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1055 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1055 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1056 {
    pub wki1056_use_write_raw_data: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1056 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1056 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1057 {
    pub wki1057_use_encryption: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1057 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1057 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1058 {
    pub wki1058_buf_files_deny_write: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1058 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1058 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1059 {
    pub wki1059_buf_read_only_files: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1059 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1059 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1060 {
    pub wki1060_force_core_create_mode: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1060 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1060 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1061 {
    pub wki1061_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_1061 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1061 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_1062 {
    pub wki1062_read_ahead_throughput: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_1062 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_1062 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_302 {
    pub wki302_char_wait: u32,
    pub wki302_collection_time: u32,
    pub wki302_maximum_collection_count: u32,
    pub wki302_keep_conn: u32,
    pub wki302_keep_search: u32,
    pub wki302_max_cmds: u32,
    pub wki302_num_work_buf: u32,
    pub wki302_siz_work_buf: u32,
    pub wki302_max_wrk_cache: u32,
    pub wki302_sess_timeout: u32,
    pub wki302_siz_error: u32,
    pub wki302_num_alerts: u32,
    pub wki302_num_services: u32,
    pub wki302_errlog_sz: u32,
    pub wki302_print_buf_time: u32,
    pub wki302_num_char_buf: u32,
    pub wki302_siz_char_buf: u32,
    pub wki302_wrk_heuristics: windows_core::PWSTR,
    pub wki302_mailslots: u32,
    pub wki302_num_dgram_buf: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_302 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_302 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_402 {
    pub wki402_char_wait: u32,
    pub wki402_collection_time: u32,
    pub wki402_maximum_collection_count: u32,
    pub wki402_keep_conn: u32,
    pub wki402_keep_search: u32,
    pub wki402_max_cmds: u32,
    pub wki402_num_work_buf: u32,
    pub wki402_siz_work_buf: u32,
    pub wki402_max_wrk_cache: u32,
    pub wki402_sess_timeout: u32,
    pub wki402_siz_error: u32,
    pub wki402_num_alerts: u32,
    pub wki402_num_services: u32,
    pub wki402_errlog_sz: u32,
    pub wki402_print_buf_time: u32,
    pub wki402_num_char_buf: u32,
    pub wki402_siz_char_buf: u32,
    pub wki402_wrk_heuristics: windows_core::PWSTR,
    pub wki402_mailslots: u32,
    pub wki402_num_dgram_buf: u32,
    pub wki402_max_threads: u32,
}
impl windows_core::TypeKind for WKSTA_INFO_402 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_402 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_INFO_502 {
    pub wki502_char_wait: u32,
    pub wki502_collection_time: u32,
    pub wki502_maximum_collection_count: u32,
    pub wki502_keep_conn: u32,
    pub wki502_max_cmds: u32,
    pub wki502_sess_timeout: u32,
    pub wki502_siz_char_buf: u32,
    pub wki502_max_threads: u32,
    pub wki502_lock_quota: u32,
    pub wki502_lock_increment: u32,
    pub wki502_lock_maximum: u32,
    pub wki502_pipe_increment: u32,
    pub wki502_pipe_maximum: u32,
    pub wki502_cache_file_timeout: u32,
    pub wki502_dormant_file_limit: u32,
    pub wki502_read_ahead_throughput: u32,
    pub wki502_num_mailslot_buffers: u32,
    pub wki502_num_srv_announce_buffers: u32,
    pub wki502_max_illegal_datagram_events: u32,
    pub wki502_illegal_datagram_event_reset_frequency: u32,
    pub wki502_log_election_packets: super::super::Foundation::BOOL,
    pub wki502_use_opportunistic_locking: super::super::Foundation::BOOL,
    pub wki502_use_unlock_behind: super::super::Foundation::BOOL,
    pub wki502_use_close_behind: super::super::Foundation::BOOL,
    pub wki502_buf_named_pipes: super::super::Foundation::BOOL,
    pub wki502_use_lock_read_unlock: super::super::Foundation::BOOL,
    pub wki502_utilize_nt_caching: super::super::Foundation::BOOL,
    pub wki502_use_raw_read: super::super::Foundation::BOOL,
    pub wki502_use_raw_write: super::super::Foundation::BOOL,
    pub wki502_use_write_raw_data: super::super::Foundation::BOOL,
    pub wki502_use_encryption: super::super::Foundation::BOOL,
    pub wki502_buf_files_deny_write: super::super::Foundation::BOOL,
    pub wki502_buf_read_only_files: super::super::Foundation::BOOL,
    pub wki502_force_core_create_mode: super::super::Foundation::BOOL,
    pub wki502_use_512_byte_max_transfer: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_INFO_502 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_INFO_502 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_TRANSPORT_INFO_0 {
    pub wkti0_quality_of_service: u32,
    pub wkti0_number_of_vcs: u32,
    pub wkti0_transport_name: windows_core::PWSTR,
    pub wkti0_transport_address: windows_core::PWSTR,
    pub wkti0_wan_ish: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for WKSTA_TRANSPORT_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_TRANSPORT_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_USER_INFO_0 {
    pub wkui0_username: windows_core::PWSTR,
}
impl windows_core::TypeKind for WKSTA_USER_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_USER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_USER_INFO_1 {
    pub wkui1_username: windows_core::PWSTR,
    pub wkui1_logon_domain: windows_core::PWSTR,
    pub wkui1_oth_domains: windows_core::PWSTR,
    pub wkui1_logon_server: windows_core::PWSTR,
}
impl windows_core::TypeKind for WKSTA_USER_INFO_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_USER_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WKSTA_USER_INFO_1101 {
    pub wkui1101_oth_domains: windows_core::PWSTR,
}
impl windows_core::TypeKind for WKSTA_USER_INFO_1101 {
    type TypeKind = windows_core::CopyType;
}
impl Default for WKSTA_USER_INFO_1101 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WORKERFUNCTION = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
