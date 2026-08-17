#[inline]
pub unsafe fn PxeAsyncRecvDone(hclientrequest: super::super::Foundation::HANDLE, action: u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeAsyncRecvDone(hclientrequest : super::super::Foundation:: HANDLE, action : u32) -> u32);
    unsafe { PxeAsyncRecvDone(hclientrequest, action) }
}
#[inline]
pub unsafe fn PxeDhcpAppendOption(preplypacket: *mut core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpAppendOption(preplypacket : *mut core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32, boption : u8, boptionlen : u8, pvalue : *const core::ffi::c_void) -> u32);
    unsafe { PxeDhcpAppendOption(preplypacket as _, umaxreplypacketlen, pureplypacketlen as _, boption, boptionlen, pvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpAppendOptionRaw(preplypacket: *mut core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpAppendOptionRaw(preplypacket : *mut core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32, ubufferlen : u16, pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { PxeDhcpAppendOptionRaw(preplypacket as _, umaxreplypacketlen, pureplypacketlen as _, ubufferlen, pbuffer) }
}
#[inline]
pub unsafe fn PxeDhcpGetOptionValue(ppacket: *const core::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: Option<*mut u8>, ppoptionvalue: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpGetOptionValue(ppacket : *const core::ffi::c_void, upacketlen : u32, uinstance : u32, boption : u8, pboptionlen : *mut u8, ppoptionvalue : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PxeDhcpGetOptionValue(ppacket, upacketlen, uinstance, boption, pboptionlen.unwrap_or(core::mem::zeroed()) as _, ppoptionvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpGetVendorOptionValue(ppacket: *const core::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: Option<*mut u8>, ppoptionvalue: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpGetVendorOptionValue(ppacket : *const core::ffi::c_void, upacketlen : u32, boption : u8, uinstance : u32, pboptionlen : *mut u8, ppoptionvalue : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PxeDhcpGetVendorOptionValue(ppacket, upacketlen, boption, uinstance, pboptionlen.unwrap_or(core::mem::zeroed()) as _, ppoptionvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpInitialize(precvpacket: *const core::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpInitialize(precvpacket : *const core::ffi::c_void, urecvpacketlen : u32, preplypacket : *mut core::ffi::c_void, umaxreplypacketlen : u32, pureplypacketlen : *mut u32) -> u32);
    unsafe { PxeDhcpInitialize(precvpacket, urecvpacketlen, preplypacket as _, umaxreplypacketlen, pureplypacketlen as _) }
}
#[inline]
pub unsafe fn PxeDhcpIsValid(ppacket: *const core::ffi::c_void, upacketlen: u32, brequestpacket: bool, pbpxeoptionpresent: Option<*mut windows_core::BOOL>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpIsValid(ppacket : *const core::ffi::c_void, upacketlen : u32, brequestpacket : windows_core::BOOL, pbpxeoptionpresent : *mut windows_core::BOOL) -> u32);
    unsafe { PxeDhcpIsValid(ppacket, upacketlen, brequestpacket.into(), pbpxeoptionpresent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOption(preply: *mut core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6AppendOption(preply : *mut core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32, woptiontype : u16, cboption : u16, poption : *const core::ffi::c_void) -> u32);
    unsafe { PxeDhcpv6AppendOption(preply as _, cbreply, pcbreplyused as _, woptiontype, cboption, poption) }
}
#[inline]
pub unsafe fn PxeDhcpv6AppendOptionRaw(preply: *mut core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6AppendOptionRaw(preply : *mut core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32, cbbuffer : u16, pbuffer : *const core::ffi::c_void) -> u32);
    unsafe { PxeDhcpv6AppendOptionRaw(preply as _, cbreply, pcbreplyused as _, cbbuffer, pbuffer) }
}
#[inline]
pub unsafe fn PxeDhcpv6CreateRelayRepl(prelaymessages: &[PXE_DHCPV6_NESTED_RELAY_MESSAGE], pinnerpacket: &[u8], preplybuffer: *mut core::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6CreateRelayRepl(prelaymessages : *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages : u32, pinnerpacket : *const u8, cbinnerpacket : u32, preplybuffer : *mut core::ffi::c_void, cbreplybuffer : u32, pcbreplybuffer : *mut u32) -> u32);
    unsafe { PxeDhcpv6CreateRelayRepl(core::mem::transmute(prelaymessages.as_ptr()), prelaymessages.len().try_into().unwrap(), core::mem::transmute(pinnerpacket.as_ptr()), pinnerpacket.len().try_into().unwrap(), preplybuffer as _, cbreplybuffer, pcbreplybuffer as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6GetOptionValue(ppacket: *const core::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: Option<*mut u16>, ppoptionvalue: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6GetOptionValue(ppacket : *const core::ffi::c_void, upacketlen : u32, uinstance : u32, woption : u16, pwoptionlen : *mut u16, ppoptionvalue : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PxeDhcpv6GetOptionValue(ppacket, upacketlen, uinstance, woption, pwoptionlen.unwrap_or(core::mem::zeroed()) as _, ppoptionvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6GetVendorOptionValue(ppacket: *const core::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: Option<*mut u16>, ppoptionvalue: Option<*mut *mut core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6GetVendorOptionValue(ppacket : *const core::ffi::c_void, upacketlen : u32, dwenterprisenumber : u32, woption : u16, uinstance : u32, pwoptionlen : *mut u16, ppoptionvalue : *mut *mut core::ffi::c_void) -> u32);
    unsafe { PxeDhcpv6GetVendorOptionValue(ppacket, upacketlen, dwenterprisenumber, woption, uinstance, pwoptionlen.unwrap_or(core::mem::zeroed()) as _, ppoptionvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6Initialize(prequest: *const core::ffi::c_void, cbrequest: u32, preply: *mut core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6Initialize(prequest : *const core::ffi::c_void, cbrequest : u32, preply : *mut core::ffi::c_void, cbreply : u32, pcbreplyused : *mut u32) -> u32);
    unsafe { PxeDhcpv6Initialize(prequest, cbrequest, preply as _, cbreply, pcbreplyused as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6IsValid(ppacket: *const core::ffi::c_void, upacketlen: u32, brequestpacket: bool, pbpxeoptionpresent: Option<*mut windows_core::BOOL>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6IsValid(ppacket : *const core::ffi::c_void, upacketlen : u32, brequestpacket : windows_core::BOOL, pbpxeoptionpresent : *mut windows_core::BOOL) -> u32);
    unsafe { PxeDhcpv6IsValid(ppacket, upacketlen, brequestpacket.into(), pbpxeoptionpresent.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const core::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: &mut [PXE_DHCPV6_NESTED_RELAY_MESSAGE], pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeDhcpv6ParseRelayForw(prelayforwpacket : *const core::ffi::c_void, urelayforwpacketlen : u32, prelaymessages : *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages : u32, pnrelaymessages : *mut u32, ppinnerpacket : *mut *mut u8, pcbinnerpacket : *mut u32) -> u32);
    unsafe { PxeDhcpv6ParseRelayForw(prelayforwpacket, urelayforwpacketlen, core::mem::transmute(prelaymessages.as_ptr()), prelaymessages.len().try_into().unwrap(), pnrelaymessages as _, ppinnerpacket as _, pcbinnerpacket as _) }
}
#[inline]
pub unsafe fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut core::ffi::c_void, ubufferlen: u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeGetServerInfo(uinfotype : u32, pbuffer : *mut core::ffi::c_void, ubufferlen : u32) -> u32);
    unsafe { PxeGetServerInfo(uinfotype, pbuffer as _, ubufferlen) }
}
#[inline]
pub unsafe fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut core::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeGetServerInfoEx(uinfotype : u32, pbuffer : *mut core::ffi::c_void, ubufferlen : u32, pubufferused : *mut u32) -> u32);
    unsafe { PxeGetServerInfoEx(uinfotype, pbuffer as _, ubufferlen, pubufferused as _) }
}
#[inline]
pub unsafe fn PxePacketAllocate(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, usize: u32) -> *mut core::ffi::c_void {
    windows_core::link!("wdspxe.dll" "system" fn PxePacketAllocate(hprovider : super::super::Foundation:: HANDLE, hclientrequest : super::super::Foundation:: HANDLE, usize : u32) -> *mut core::ffi::c_void);
    unsafe { PxePacketAllocate(hprovider, hclientrequest, usize) }
}
#[inline]
pub unsafe fn PxePacketFree(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, ppacket: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxePacketFree(hprovider : super::super::Foundation:: HANDLE, hclientrequest : super::super::Foundation:: HANDLE, ppacket : *const core::ffi::c_void) -> u32);
    unsafe { PxePacketFree(hprovider, hclientrequest, ppacket) }
}
#[inline]
pub unsafe fn PxeProviderEnumClose(henum: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderEnumClose(henum : super::super::Foundation:: HANDLE) -> u32);
    unsafe { PxeProviderEnumClose(henum) }
}
#[inline]
pub unsafe fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderEnumFirst(phenum : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { PxeProviderEnumFirst(phenum as _) }
}
#[inline]
pub unsafe fn PxeProviderEnumNext(henum: super::super::Foundation::HANDLE, ppprovider: *mut *mut PXE_PROVIDER) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderEnumNext(henum : super::super::Foundation:: HANDLE, ppprovider : *mut *mut PXE_PROVIDER) -> u32);
    unsafe { PxeProviderEnumNext(henum, ppprovider as _) }
}
#[inline]
pub unsafe fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderFreeInfo(pprovider : *const PXE_PROVIDER) -> u32);
    unsafe { PxeProviderFreeInfo(pprovider) }
}
#[inline]
pub unsafe fn PxeProviderQueryIndex<P0>(pszprovidername: P0, puindex: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderQueryIndex(pszprovidername : windows_core::PCWSTR, puindex : *mut u32) -> u32);
    unsafe { PxeProviderQueryIndex(pszprovidername.param().abi(), puindex as _) }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn PxeProviderRegister<P0, P1>(pszprovidername: P0, pszmodulepath: P1, index: u32, biscritical: bool, phproviderkey: Option<*mut super::Registry::HKEY>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderRegister(pszprovidername : windows_core::PCWSTR, pszmodulepath : windows_core::PCWSTR, index : u32, biscritical : windows_core::BOOL, phproviderkey : *mut super::Registry:: HKEY) -> u32);
    unsafe { PxeProviderRegister(pszprovidername.param().abi(), pszmodulepath.param().abi(), index, biscritical.into(), phproviderkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeProviderSetAttribute(hprovider: super::super::Foundation::HANDLE, attribute: u32, pparameterbuffer: *const core::ffi::c_void, uparamlen: u32) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderSetAttribute(hprovider : super::super::Foundation:: HANDLE, attribute : u32, pparameterbuffer : *const core::ffi::c_void, uparamlen : u32) -> u32);
    unsafe { PxeProviderSetAttribute(hprovider, attribute, pparameterbuffer, uparamlen) }
}
#[inline]
pub unsafe fn PxeProviderUnRegister<P0>(pszprovidername: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdspxe.dll" "system" fn PxeProviderUnRegister(pszprovidername : windows_core::PCWSTR) -> u32);
    unsafe { PxeProviderUnRegister(pszprovidername.param().abi()) }
}
#[inline]
pub unsafe fn PxeRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbacktype: u32, pcallbackfunction: *const core::ffi::c_void, pcontext: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeRegisterCallback(hprovider : super::super::Foundation:: HANDLE, callbacktype : u32, pcallbackfunction : *const core::ffi::c_void, pcontext : *const core::ffi::c_void) -> u32);
    unsafe { PxeRegisterCallback(hprovider, callbacktype, pcallbackfunction, pcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PxeSendReply(hclientrequest: super::super::Foundation::HANDLE, ppacket: *const core::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32 {
    windows_core::link!("wdspxe.dll" "system" fn PxeSendReply(hclientrequest : super::super::Foundation:: HANDLE, ppacket : *const core::ffi::c_void, upacketlen : u32, paddress : *const PXE_ADDRESS) -> u32);
    unsafe { PxeSendReply(hclientrequest, ppacket, upacketlen, paddress) }
}
#[inline]
pub unsafe fn PxeTrace<P2>(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdspxe.dll" "C" fn PxeTrace(hprovider : super::super::Foundation:: HANDLE, severity : u32, pszformat : windows_core::PCWSTR) -> u32);
    unsafe { PxeTrace(hprovider, severity, pszformat.param().abi()) }
}
#[inline]
pub unsafe fn PxeTraceV<P2>(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: P2, params: *const i8) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdspxe.dll" "system" fn PxeTraceV(hprovider : super::super::Foundation:: HANDLE, severity : u32, pszformat : windows_core::PCWSTR, params : *const i8) -> u32);
    unsafe { PxeTraceV(hprovider, severity, pszformat.param().abi(), params) }
}
#[inline]
pub unsafe fn WdsBpAddOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpAddOption(hhandle : super::super::Foundation:: HANDLE, uoption : u32, uvaluelen : u32, pvalue : *const core::ffi::c_void) -> u32);
    unsafe { WdsBpAddOption(hhandle, uoption, uvaluelen, pvalue) }
}
#[inline]
pub unsafe fn WdsBpCloseHandle(hhandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpCloseHandle(hhandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsBpCloseHandle(hhandle) }
}
#[inline]
pub unsafe fn WdsBpGetOptionBuffer(hhandle: super::super::Foundation::HANDLE, ubufferlen: u32, pbuffer: *mut core::ffi::c_void, pubytes: *mut u32) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpGetOptionBuffer(hhandle : super::super::Foundation:: HANDLE, ubufferlen : u32, pbuffer : *mut core::ffi::c_void, pubytes : *mut u32) -> u32);
    unsafe { WdsBpGetOptionBuffer(hhandle, ubufferlen, pbuffer as _, pubytes as _) }
}
#[inline]
pub unsafe fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpInitialize(bpackettype : u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsBpInitialize(bpackettype, phhandle as _) }
}
#[inline]
pub unsafe fn WdsBpParseInitialize(ppacket: *const core::ffi::c_void, upacketlen: u32, pbpackettype: Option<*mut u8>, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpParseInitialize(ppacket : *const core::ffi::c_void, upacketlen : u32, pbpackettype : *mut u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsBpParseInitialize(ppacket, upacketlen, pbpackettype.unwrap_or(core::mem::zeroed()) as _, phhandle as _) }
}
#[inline]
pub unsafe fn WdsBpParseInitializev6(ppacket: *const core::ffi::c_void, upacketlen: u32, pbpackettype: Option<*mut u8>, phhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpParseInitializev6(ppacket : *const core::ffi::c_void, upacketlen : u32, pbpackettype : *mut u8, phhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsBpParseInitializev6(ppacket, upacketlen, pbpackettype.unwrap_or(core::mem::zeroed()) as _, phhandle as _) }
}
#[inline]
pub unsafe fn WdsBpQueryOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *mut core::ffi::c_void, pubytes: Option<*mut u32>) -> u32 {
    windows_core::link!("wdsbp.dll" "system" fn WdsBpQueryOption(hhandle : super::super::Foundation:: HANDLE, uoption : u32, uvaluelen : u32, pvalue : *mut core::ffi::c_void, pubytes : *mut u32) -> u32);
    unsafe { WdsBpQueryOption(hhandle, uoption, uvaluelen, pvalue as _, pubytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WdsCliAuthorizeSession(hsession: super::super::Foundation::HANDLE, pcred: Option<*const WDS_CLI_CRED>) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliAuthorizeSession(hsession : super::super::Foundation:: HANDLE, pcred : *const WDS_CLI_CRED) -> windows_core::HRESULT);
    unsafe { WdsCliAuthorizeSession(hsession as _, pcred.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn WdsCliCancelTransfer(htransfer: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliCancelTransfer(htransfer : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WdsCliCancelTransfer(htransfer).ok() }
}
#[inline]
pub unsafe fn WdsCliClose(handle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliClose(handle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WdsCliClose(handle).ok() }
}
#[inline]
pub unsafe fn WdsCliCreateSession<P0>(pwszserver: P0, pcred: Option<*const WDS_CLI_CRED>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliCreateSession(pwszserver : windows_core::PCWSTR, pcred : *const WDS_CLI_CRED, phsession : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliCreateSession(pwszserver.param().abi(), pcred.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliFindFirstImage(hsession: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliFindFirstImage(hsession : super::super::Foundation:: HANDLE, phfindhandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliFindFirstImage(hsession, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliFindNextImage(handle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliFindNextImage(handle : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WdsCliFindNextImage(handle).ok() }
}
#[inline]
pub unsafe fn WdsCliFreeStringArray(ppwszarray: Option<&mut [windows_core::PWSTR]>) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliFreeStringArray(ppwszarray : *mut windows_core::PWSTR, ulcount : u32) -> windows_core::HRESULT);
    unsafe { WdsCliFreeStringArray(core::mem::transmute(ppwszarray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), ppwszarray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn WdsCliGetDriverQueryXml<P0>(pwszwindirpath: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetDriverQueryXml(pwszwindirpath : windows_core::PCWSTR, ppwszdriverquery : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetDriverQueryXml(pwszwindirpath.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetEnumerationFlags(handle: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetEnumerationFlags(handle : super::super::Foundation:: HANDLE, pdwflags : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetEnumerationFlags(handle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageArchitecture(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<CPU_ARCHITECTURE> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageArchitecture(hifh : super::super::Foundation:: HANDLE, pdwvalue : *mut CPU_ARCHITECTURE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageArchitecture(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageDescription(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageDescription(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageDescription(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageFiles(hifh: super::super::Foundation::HANDLE, pppwszfiles: *mut *mut windows_core::PWSTR, pdwcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageFiles(hifh : super::super::Foundation:: HANDLE, pppwszfiles : *mut *mut windows_core::PWSTR, pdwcount : *mut u32) -> windows_core::HRESULT);
    unsafe { WdsCliGetImageFiles(hifh, pppwszfiles as _, pdwcount as _).ok() }
}
#[inline]
pub unsafe fn WdsCliGetImageGroup(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageGroup(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageGroup(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageHalName(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHalName(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageHalName(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageHandleFromFindHandle(findhandle: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHandleFromFindHandle(findhandle : super::super::Foundation:: HANDLE, phimagehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageHandleFromFindHandle(findhandle, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageHandleFromTransferHandle(htransfer: super::super::Foundation::HANDLE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageHandleFromTransferHandle(htransfer : super::super::Foundation:: HANDLE, phimagehandle : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageHandleFromTransferHandle(htransfer, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageIndex(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageIndex(hifh : super::super::Foundation:: HANDLE, pdwvalue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageIndex(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageLanguage(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLanguage(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageLanguage(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageLanguages(hifh: super::super::Foundation::HANDLE, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLanguages(hifh : super::super::Foundation:: HANDLE, pppszvalues : *mut *mut *mut i8, pdwnumvalues : *mut u32) -> windows_core::HRESULT);
    unsafe { WdsCliGetImageLanguages(hifh, pppszvalues as _, pdwnumvalues as _).ok() }
}
#[inline]
pub unsafe fn WdsCliGetImageLastModifiedTime(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<*mut super::super::Foundation::SYSTEMTIME> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageLastModifiedTime(hifh : super::super::Foundation:: HANDLE, ppsystimevalue : *mut *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageLastModifiedTime(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageName(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageName(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageName(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageNamespace(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageNamespace(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageNamespace(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageParameter(hifh: super::super::Foundation::HANDLE, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut core::ffi::c_void, uresponselen: u32) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageParameter(hifh : super::super::Foundation:: HANDLE, paramtype : WDS_CLI_IMAGE_PARAM_TYPE, presponse : *mut core::ffi::c_void, uresponselen : u32) -> windows_core::HRESULT);
    unsafe { WdsCliGetImageParameter(hifh, paramtype, presponse as _, uresponselen).ok() }
}
#[inline]
pub unsafe fn WdsCliGetImagePath(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImagePath(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImagePath(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageSize(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<u64> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageSize(hifh : super::super::Foundation:: HANDLE, pullvalue : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageSize(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageType(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<WDS_CLI_IMAGE_TYPE> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageType(hifh : super::super::Foundation:: HANDLE, pimagetype : *mut WDS_CLI_IMAGE_TYPE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageType(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetImageVersion(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetImageVersion(hifh : super::super::Foundation:: HANDLE, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetImageVersion(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliGetTransferSize(hifh: super::super::Foundation::HANDLE) -> windows_core::Result<u64> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliGetTransferSize(hifh : super::super::Foundation:: HANDLE, pullvalue : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliGetTransferSize(hifh, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliInitializeLog<P2, P3>(hsession: super::super::Foundation::HANDLE, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: P2, pwszclientaddress: P3) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliInitializeLog(hsession : super::super::Foundation:: HANDLE, ulclientarchitecture : CPU_ARCHITECTURE, pwszclientid : windows_core::PCWSTR, pwszclientaddress : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WdsCliInitializeLog(hsession, ulclientarchitecture, pwszclientid.param().abi(), pwszclientaddress.param().abi()).ok() }
}
#[inline]
pub unsafe fn WdsCliLog(hsession: super::super::Foundation::HANDLE, ulloglevel: u32, ulmessagecode: u32) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "C" fn WdsCliLog(hsession : super::super::Foundation:: HANDLE, ulloglevel : u32, ulmessagecode : u32) -> windows_core::HRESULT);
    unsafe { WdsCliLog(hsession, ulloglevel, ulmessagecode).ok() }
}
#[inline]
pub unsafe fn WdsCliObtainDriverPackages(himage: super::super::Foundation::HANDLE, ppwszservername: *mut windows_core::PWSTR, pppwszdriverpackages: *mut *mut windows_core::PWSTR, pulcount: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliObtainDriverPackages(himage : super::super::Foundation:: HANDLE, ppwszservername : *mut windows_core::PWSTR, pppwszdriverpackages : *mut *mut windows_core::PWSTR, pulcount : *mut u32) -> windows_core::HRESULT);
    unsafe { WdsCliObtainDriverPackages(himage, ppwszservername as _, pppwszdriverpackages as _, pulcount as _).ok() }
}
#[inline]
pub unsafe fn WdsCliObtainDriverPackagesEx<P1>(hsession: super::super::Foundation::HANDLE, pwszmachineinfo: P1, ppwszservername: *mut windows_core::PWSTR, pppwszdriverpackages: *mut *mut windows_core::PWSTR, pulcount: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliObtainDriverPackagesEx(hsession : super::super::Foundation:: HANDLE, pwszmachineinfo : windows_core::PCWSTR, ppwszservername : *mut windows_core::PWSTR, pppwszdriverpackages : *mut *mut windows_core::PWSTR, pulcount : *mut u32) -> windows_core::HRESULT);
    unsafe { WdsCliObtainDriverPackagesEx(hsession, pwszmachineinfo.param().abi(), ppwszservername as _, pppwszdriverpackages as _, pulcount as _).ok() }
}
#[inline]
pub unsafe fn WdsCliRegisterTrace(pfn: PFN_WdsCliTraceFunction) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliRegisterTrace(pfn : PFN_WdsCliTraceFunction) -> windows_core::HRESULT);
    unsafe { WdsCliRegisterTrace(pfn).ok() }
}
#[inline]
pub unsafe fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32) {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliSetTransferBufferSize(ulsizeinbytes : u32));
    unsafe { WdsCliSetTransferBufferSize(ulsizeinbytes) }
}
#[inline]
pub unsafe fn WdsCliTransferFile<P0, P1, P2, P3>(pwszserver: P0, pwsznamespace: P1, pwszremotefilepath: P2, pwszlocalfilepath: P3, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliTransferFile(pwszserver : windows_core::PCWSTR, pwsznamespace : windows_core::PCWSTR, pwszremotefilepath : windows_core::PCWSTR, pwszlocalfilepath : windows_core::PCWSTR, dwflags : u32, dwreserved : u32, pfnwdsclicallback : PFN_WdsCliCallback, pvuserdata : *const core::ffi::c_void, phtransfer : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliTransferFile(pwszserver.param().abi(), pwsznamespace.param().abi(), pwszremotefilepath.param().abi(), pwszlocalfilepath.param().abi(), dwflags, dwreserved, pfnwdsclicallback, pvuserdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliTransferImage<P1>(himage: super::super::Foundation::HANDLE, pwszlocalpath: P1, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: Option<*const core::ffi::c_void>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliTransferImage(himage : super::super::Foundation:: HANDLE, pwszlocalpath : windows_core::PCWSTR, dwflags : u32, dwreserved : u32, pfnwdsclicallback : PFN_WdsCliCallback, pvuserdata : *const core::ffi::c_void, phtransfer : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WdsCliTransferImage(himage, pwszlocalpath.param().abi(), dwflags, dwreserved, pfnwdsclicallback, pvuserdata.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WdsCliWaitForTransfer(htransfer: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("wdsclientapi.dll" "system" fn WdsCliWaitForTransfer(htransfer : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { WdsCliWaitForTransfer(htransfer).ok() }
}
#[inline]
pub unsafe fn WdsTransportClientAddRefBuffer(pvbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientAddRefBuffer(pvbuffer : *const core::ffi::c_void) -> u32);
    unsafe { WdsTransportClientAddRefBuffer(pvbuffer) }
}
#[inline]
pub unsafe fn WdsTransportClientCancelSession(hsessionkey: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientCancelSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsTransportClientCancelSession(hsessionkey) }
}
#[inline]
pub unsafe fn WdsTransportClientCancelSessionEx(hsessionkey: super::super::Foundation::HANDLE, dwerrorcode: u32) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientCancelSessionEx(hsessionkey : super::super::Foundation:: HANDLE, dwerrorcode : u32) -> u32);
    unsafe { WdsTransportClientCancelSessionEx(hsessionkey, dwerrorcode) }
}
#[inline]
pub unsafe fn WdsTransportClientCloseSession(hsessionkey: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientCloseSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsTransportClientCloseSession(hsessionkey) }
}
#[inline]
pub unsafe fn WdsTransportClientCompleteReceive(hsessionkey: super::super::Foundation::HANDLE, ulsize: u32, pulloffset: *const u64) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientCompleteReceive(hsessionkey : super::super::Foundation:: HANDLE, ulsize : u32, pulloffset : *const u64) -> u32);
    unsafe { WdsTransportClientCompleteReceive(hsessionkey, ulsize, pulloffset) }
}
#[inline]
pub unsafe fn WdsTransportClientInitialize() -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientInitialize() -> u32);
    unsafe { WdsTransportClientInitialize() }
}
#[inline]
pub unsafe fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const core::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientInitializeSession(psessionrequest : *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata : *const core::ffi::c_void, hsessionkey : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsTransportClientInitializeSession(psessionrequest, pcallerdata, hsessionkey as _) }
}
#[inline]
pub unsafe fn WdsTransportClientQueryStatus(hsessionkey: super::super::Foundation::HANDLE, pustatus: *mut u32, puerrorcode: *mut u32) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientQueryStatus(hsessionkey : super::super::Foundation:: HANDLE, pustatus : *mut u32, puerrorcode : *mut u32) -> u32);
    unsafe { WdsTransportClientQueryStatus(hsessionkey, pustatus as _, puerrorcode as _) }
}
#[inline]
pub unsafe fn WdsTransportClientRegisterCallback(hsessionkey: super::super::Foundation::HANDLE, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientRegisterCallback(hsessionkey : super::super::Foundation:: HANDLE, callbackid : TRANSPORTCLIENT_CALLBACK_ID, pfncallback : *const core::ffi::c_void) -> u32);
    unsafe { WdsTransportClientRegisterCallback(hsessionkey, callbackid, pfncallback) }
}
#[inline]
pub unsafe fn WdsTransportClientReleaseBuffer(pvbuffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientReleaseBuffer(pvbuffer : *const core::ffi::c_void) -> u32);
    unsafe { WdsTransportClientReleaseBuffer(pvbuffer) }
}
#[inline]
pub unsafe fn WdsTransportClientShutdown() -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientShutdown() -> u32);
    unsafe { WdsTransportClientShutdown() }
}
#[inline]
pub unsafe fn WdsTransportClientStartSession(hsessionkey: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientStartSession(hsessionkey : super::super::Foundation:: HANDLE) -> u32);
    unsafe { WdsTransportClientStartSession(hsessionkey) }
}
#[inline]
pub unsafe fn WdsTransportClientWaitForCompletion(hsessionkey: super::super::Foundation::HANDLE, utimeout: u32) -> u32 {
    windows_core::link!("wdstptc.dll" "system" fn WdsTransportClientWaitForCompletion(hsessionkey : super::super::Foundation:: HANDLE, utimeout : u32) -> u32);
    unsafe { WdsTransportClientWaitForCompletion(hsessionkey, utimeout) }
}
#[inline]
pub unsafe fn WdsTransportServerAllocateBuffer(hprovider: super::super::Foundation::HANDLE, ulbuffersize: u32) -> *mut core::ffi::c_void {
    windows_core::link!("wdsmc.dll" "system" fn WdsTransportServerAllocateBuffer(hprovider : super::super::Foundation:: HANDLE, ulbuffersize : u32) -> *mut core::ffi::c_void);
    unsafe { WdsTransportServerAllocateBuffer(hprovider, ulbuffersize) }
}
#[inline]
pub unsafe fn WdsTransportServerCompleteRead(hprovider: super::super::Foundation::HANDLE, ulbytesread: u32, pvuserdata: *const core::ffi::c_void, hreadresult: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_core::link!("wdsmc.dll" "system" fn WdsTransportServerCompleteRead(hprovider : super::super::Foundation:: HANDLE, ulbytesread : u32, pvuserdata : *const core::ffi::c_void, hreadresult : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { WdsTransportServerCompleteRead(hprovider, ulbytesread, pvuserdata, hreadresult).ok() }
}
#[inline]
pub unsafe fn WdsTransportServerFreeBuffer(hprovider: super::super::Foundation::HANDLE, pvbuffer: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wdsmc.dll" "system" fn WdsTransportServerFreeBuffer(hprovider : super::super::Foundation:: HANDLE, pvbuffer : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WdsTransportServerFreeBuffer(hprovider, pvbuffer).ok() }
}
#[inline]
pub unsafe fn WdsTransportServerRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_core::link!("wdsmc.dll" "system" fn WdsTransportServerRegisterCallback(hprovider : super::super::Foundation:: HANDLE, callbackid : TRANSPORTPROVIDER_CALLBACK_ID, pfncallback : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WdsTransportServerRegisterCallback(hprovider, callbackid, pfncallback).ok() }
}
#[inline]
pub unsafe fn WdsTransportServerTrace<P2>(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsmc.dll" "C" fn WdsTransportServerTrace(hprovider : super::super::Foundation:: HANDLE, severity : u32, pwszformat : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { WdsTransportServerTrace(hprovider, severity, pwszformat.param().abi()).ok() }
}
#[inline]
pub unsafe fn WdsTransportServerTraceV<P2>(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: P2, params: *const i8) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wdsmc.dll" "system" fn WdsTransportServerTraceV(hprovider : super::super::Foundation:: HANDLE, severity : u32, pwszformat : windows_core::PCWSTR, params : *const i8) -> windows_core::HRESULT);
    unsafe { WdsTransportServerTraceV(hprovider, severity, pwszformat.param().abi(), params).ok() }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CPU_ARCHITECTURE(pub u32);
pub const CPU_ARCHITECTURE_AMD64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(9u32);
pub const CPU_ARCHITECTURE_IA64: CPU_ARCHITECTURE = CPU_ARCHITECTURE(6u32);
pub const CPU_ARCHITECTURE_INTEL: CPU_ARCHITECTURE = CPU_ARCHITECTURE(0u32);
pub const EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG: windows_core::HRESULT = windows_core::HRESULT(0xC1210254_u32 as _);
pub const EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210258_u32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210250_u32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0xC121025B_u32 as _);
pub const EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC1210253_u32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210252_u32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_MISSING: windows_core::HRESULT = windows_core::HRESULT(0xC1210251_u32 as _);
pub const EVT_WDSMCS_E_CP_MEMORY_LEAK: windows_core::HRESULT = windows_core::HRESULT(0xC1210256_u32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210259_u32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210257_u32 as _);
pub const EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210255_u32 as _);
pub const EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR: windows_core::HRESULT = windows_core::HRESULT(0xC1210202_u32 as _);
pub const EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR: windows_core::HRESULT = windows_core::HRESULT(0xC1210203_u32 as _);
pub const EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG: windows_core::HRESULT = windows_core::HRESULT(0xC1210301_u32 as _);
pub const EVT_WDSMCS_E_NSREG_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0xC1210303_u32 as _);
pub const EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC1210302_u32 as _);
pub const EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST: windows_core::HRESULT = windows_core::HRESULT(0xC1210300_u32 as _);
pub const EVT_WDSMCS_E_PARAMETERS_READ_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC1210201_u32 as _);
pub const EVT_WDSMCS_S_PARAMETERS_READ: windows_core::HRESULT = windows_core::HRESULT(0x41210200_u32 as _);
pub const EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL: windows_core::HRESULT = windows_core::HRESULT(0x8121025A_u32 as _);
pub const FACILITY_WDSMCCLIENT: u32 = 290u32;
pub const FACILITY_WDSMCSERVER: u32 = 289u32;
pub const FACILITY_WDSTPTMGMT: u32 = 272u32;
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportCacheable, IWdsTransportCacheable_Vtbl, 0x46ad894b_0bab_47dc_84b2_7b553f1d8f80);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportCacheable {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportCacheable, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCacheable {
    pub unsafe fn Dirty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dirty)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Discard(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Discard)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCacheable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Dirty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Discard: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportCacheable_Impl: super::Com::IDispatch_Impl {
    fn Dirty(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Discard(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportCacheable_Vtbl {
    pub const fn new<Identity: IWdsTransportCacheable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Dirty<Identity: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdirty: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportCacheable_Impl::Dirty(this) {
                    Ok(ok__) => {
                        pbdirty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Discard<Identity: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportCacheable_Impl::Discard(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportCacheable_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IWdsTransportCacheable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportCacheable_Impl::Commit(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Dirty: Dirty::<Identity, OFFSET>,
            Discard: Discard::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportCacheable as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportCacheable {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportClient, IWdsTransportClient_Vtbl, 0xb5dbc93a_cabe_46ca_837f_3e44e93c6545);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportClient {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportClient, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportClient {
    pub unsafe fn Session(&self) -> windows_core::Result<IWdsTransportSession> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Session)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn MacAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MacAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PercentCompletion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PercentCompletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn JoinDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).JoinDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CpuUtilization(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CpuUtilization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MemoryUtilization(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MemoryUtilization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NetworkUtilization(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkUtilization)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UserIdentity(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserIdentity)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), disconnectiontype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Session: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MacAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PercentCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub JoinDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CpuUtilization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MemoryUtilization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NetworkUtilization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UserIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportClient_Impl: super::Com::IDispatch_Impl {
    fn Session(&self) -> windows_core::Result<IWdsTransportSession>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MacAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PercentCompletion(&self) -> windows_core::Result<u32>;
    fn JoinDuration(&self) -> windows_core::Result<u32>;
    fn CpuUtilization(&self) -> windows_core::Result<u32>;
    fn MemoryUtilization(&self) -> windows_core::Result<u32>;
    fn NetworkUtilization(&self) -> windows_core::Result<u32>;
    fn UserIdentity(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Disconnect(&self, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportClient_Vtbl {
    pub const fn new<Identity: IWdsTransportClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Session<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::Session(this) {
                    Ok(ok__) => {
                        ppwdstransportsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::Id(this) {
                    Ok(ok__) => {
                        pulid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::Name(this) {
                    Ok(ok__) => {
                        pbszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MacAddress<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszmacaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::MacAddress(this) {
                    Ok(ok__) => {
                        pbszmacaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IpAddress<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszipaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::IpAddress(this) {
                    Ok(ok__) => {
                        pbszipaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PercentCompletion<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulpercentcompletion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::PercentCompletion(this) {
                    Ok(ok__) => {
                        pulpercentcompletion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn JoinDuration<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puljoinduration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::JoinDuration(this) {
                    Ok(ok__) => {
                        puljoinduration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CpuUtilization<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcpuutilization: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::CpuUtilization(this) {
                    Ok(ok__) => {
                        pulcpuutilization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MemoryUtilization<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmemoryutilization: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::MemoryUtilization(this) {
                    Ok(ok__) => {
                        pulmemoryutilization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetworkUtilization<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulnetworkutilization: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::NetworkUtilization(this) {
                    Ok(ok__) => {
                        pulnetworkutilization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UserIdentity<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszuseridentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportClient_Impl::UserIdentity(this) {
                    Ok(ok__) => {
                        pbszuseridentity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IWdsTransportClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportClient_Impl::Disconnect(this, core::mem::transmute_copy(&disconnectiontype)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Session: Session::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            MacAddress: MacAddress::<Identity, OFFSET>,
            IpAddress: IpAddress::<Identity, OFFSET>,
            PercentCompletion: PercentCompletion::<Identity, OFFSET>,
            JoinDuration: JoinDuration::<Identity, OFFSET>,
            CpuUtilization: CpuUtilization::<Identity, OFFSET>,
            MemoryUtilization: MemoryUtilization::<Identity, OFFSET>,
            NetworkUtilization: NetworkUtilization::<Identity, OFFSET>,
            UserIdentity: UserIdentity::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportClient {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportCollection, IWdsTransportCollection_Vtbl, 0xb8ba4b1a_2ff4_43ab_996c_b2b10a91a6eb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Item(&self, ulindex: u32) -> windows_core::Result<super::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportCollection_Impl: super::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn get_Item(&self, ulindex: u32) -> windows_core::Result<super::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportCollection_Vtbl {
    pub const fn new<Identity: IWdsTransportCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pulcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportCollection_Impl::get_Item(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IWdsTransportCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportCollection as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportCollection {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportConfigurationManager, IWdsTransportConfigurationManager_Vtbl, 0x84cc4779_42dd_4792_891e_1321d6d74b44);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportConfigurationManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportConfigurationManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager {
    pub unsafe fn ServicePolicy(&self) -> windows_core::Result<IWdsTransportServicePolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServicePolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DiagnosticsPolicy(&self) -> windows_core::Result<IWdsTransportDiagnosticsPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiagnosticsPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_WdsTransportServicesRunning(&self, brealtimestatus: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WdsTransportServicesRunning)(windows_core::Interface::as_raw(self), brealtimestatus, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableWdsTransportServices(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableWdsTransportServices)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DisableWdsTransportServices(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableWdsTransportServices)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StartWdsTransportServices(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartWdsTransportServices)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn StopWdsTransportServices(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopWdsTransportServices)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RestartWdsTransportServices(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RestartWdsTransportServices)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyWdsTransportServices)(windows_core::Interface::as_raw(self), servicenotification).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ServicePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiagnosticsPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_WdsTransportServicesRunning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestartWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyWdsTransportServices: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportConfigurationManager_Impl: super::Com::IDispatch_Impl {
    fn ServicePolicy(&self) -> windows_core::Result<IWdsTransportServicePolicy>;
    fn DiagnosticsPolicy(&self) -> windows_core::Result<IWdsTransportDiagnosticsPolicy>;
    fn get_WdsTransportServicesRunning(&self, brealtimestatus: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EnableWdsTransportServices(&self) -> windows_core::Result<()>;
    fn DisableWdsTransportServices(&self) -> windows_core::Result<()>;
    fn StartWdsTransportServices(&self) -> windows_core::Result<()>;
    fn StopWdsTransportServices(&self) -> windows_core::Result<()>;
    fn RestartWdsTransportServices(&self) -> windows_core::Result<()>;
    fn NotifyWdsTransportServices(&self, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportConfigurationManager_Vtbl {
    pub const fn new<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ServicePolicy<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportservicepolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportConfigurationManager_Impl::ServicePolicy(this) {
                    Ok(ok__) => {
                        ppwdstransportservicepolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DiagnosticsPolicy<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportdiagnosticspolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportConfigurationManager_Impl::DiagnosticsPolicy(this) {
                    Ok(ok__) => {
                        ppwdstransportdiagnosticspolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WdsTransportServicesRunning<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brealtimestatus: super::super::Foundation::VARIANT_BOOL, pbservicesrunning: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportConfigurationManager_Impl::get_WdsTransportServicesRunning(this, core::mem::transmute_copy(&brealtimestatus)) {
                    Ok(ok__) => {
                        pbservicesrunning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::EnableWdsTransportServices(this).into()
            }
        }
        unsafe extern "system" fn DisableWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::DisableWdsTransportServices(this).into()
            }
        }
        unsafe extern "system" fn StartWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::StartWdsTransportServices(this).into()
            }
        }
        unsafe extern "system" fn StopWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::StopWdsTransportServices(this).into()
            }
        }
        unsafe extern "system" fn RestartWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::RestartWdsTransportServices(this).into()
            }
        }
        unsafe extern "system" fn NotifyWdsTransportServices<Identity: IWdsTransportConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicenotification: WDSTRANSPORT_SERVICE_NOTIFICATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportConfigurationManager_Impl::NotifyWdsTransportServices(this, core::mem::transmute_copy(&servicenotification)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ServicePolicy: ServicePolicy::<Identity, OFFSET>,
            DiagnosticsPolicy: DiagnosticsPolicy::<Identity, OFFSET>,
            get_WdsTransportServicesRunning: get_WdsTransportServicesRunning::<Identity, OFFSET>,
            EnableWdsTransportServices: EnableWdsTransportServices::<Identity, OFFSET>,
            DisableWdsTransportServices: DisableWdsTransportServices::<Identity, OFFSET>,
            StartWdsTransportServices: StartWdsTransportServices::<Identity, OFFSET>,
            StopWdsTransportServices: StopWdsTransportServices::<Identity, OFFSET>,
            RestartWdsTransportServices: RestartWdsTransportServices::<Identity, OFFSET>,
            NotifyWdsTransportServices: NotifyWdsTransportServices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportConfigurationManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportConfigurationManager2, IWdsTransportConfigurationManager2_Vtbl, 0xd0d85caf_a153_4f1d_a9dd_96f431c50717);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportConfigurationManager2 {
    type Target = IWdsTransportConfigurationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportConfigurationManager2, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportConfigurationManager);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportConfigurationManager2 {
    pub unsafe fn MulticastSessionPolicy(&self) -> windows_core::Result<IWdsTransportMulticastSessionPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MulticastSessionPolicy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportConfigurationManager2_Vtbl {
    pub base__: IWdsTransportConfigurationManager_Vtbl,
    pub MulticastSessionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportConfigurationManager2_Impl: IWdsTransportConfigurationManager_Impl {
    fn MulticastSessionPolicy(&self) -> windows_core::Result<IWdsTransportMulticastSessionPolicy>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportConfigurationManager2_Vtbl {
    pub const fn new<Identity: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MulticastSessionPolicy<Identity: IWdsTransportConfigurationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportmulticastsessionpolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportConfigurationManager2_Impl::MulticastSessionPolicy(this) {
                    Ok(ok__) => {
                        ppwdstransportmulticastsessionpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWdsTransportConfigurationManager_Vtbl::new::<Identity, OFFSET>(), MulticastSessionPolicy: MulticastSessionPolicy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportConfigurationManager2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportConfigurationManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportConfigurationManager2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportContent, IWdsTransportContent_Vtbl, 0xd405d711_0296_4ab4_a860_ac7d32e65798);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportContent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportContent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContent {
    pub unsafe fn Namespace(&self) -> windows_core::Result<IWdsTransportNamespace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Namespace)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RetrieveSessions(&self) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveSessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Namespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrieveSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportContent_Impl: super::Com::IDispatch_Impl {
    fn Namespace(&self) -> windows_core::Result<IWdsTransportNamespace>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RetrieveSessions(&self) -> windows_core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportContent_Vtbl {
    pub const fn new<Identity: IWdsTransportContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Namespace<Identity: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContent_Impl::Namespace(this) {
                    Ok(ok__) => {
                        ppwdstransportnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContent_Impl::Id(this) {
                    Ok(ok__) => {
                        pulid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContent_Impl::Name(this) {
                    Ok(ok__) => {
                        pbszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RetrieveSessions<Identity: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsessions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContent_Impl::RetrieveSessions(this) {
                    Ok(ok__) => {
                        ppwdstransportsessions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Terminate<Identity: IWdsTransportContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportContent_Impl::Terminate(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Namespace: Namespace::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            RetrieveSessions: RetrieveSessions::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportContent as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportContent {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportContentProvider, IWdsTransportContentProvider_Vtbl, 0xb9489f24_f219_4acf_aad7_265c7c08a6ae);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportContentProvider {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportContentProvider, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportContentProvider {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn InitializationRoutine(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializationRoutine)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportContentProvider_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializationRoutine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportContentProvider_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn InitializationRoutine(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportContentProvider_Vtbl {
    pub const fn new<Identity: IWdsTransportContentProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContentProvider_Impl::Name(this) {
                    Ok(ok__) => {
                        pbszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContentProvider_Impl::Description(this) {
                    Ok(ok__) => {
                        pbszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilePath<Identity: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfilepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContentProvider_Impl::FilePath(this) {
                    Ok(ok__) => {
                        pbszfilepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializationRoutine<Identity: IWdsTransportContentProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszinitializationroutine: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportContentProvider_Impl::InitializationRoutine(this) {
                    Ok(ok__) => {
                        pbszinitializationroutine.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            FilePath: FilePath::<Identity, OFFSET>,
            InitializationRoutine: InitializationRoutine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportContentProvider as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportContentProvider {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportDiagnosticsPolicy, IWdsTransportDiagnosticsPolicy_Vtbl, 0x13b33efc_7856_4f61_9a59_8de67b6b87b6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportDiagnosticsPolicy {
    type Target = IWdsTransportCacheable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportDiagnosticsPolicy, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportDiagnosticsPolicy {
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benabled).ok() }
    }
    pub unsafe fn Components(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Components)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetComponents(&self, ulcomponents: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetComponents)(windows_core::Interface::as_raw(self), ulcomponents).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportDiagnosticsPolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Components: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportDiagnosticsPolicy_Impl: IWdsTransportCacheable_Impl {
    fn Enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Components(&self) -> windows_core::Result<u32>;
    fn SetComponents(&self, ulcomponents: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportDiagnosticsPolicy_Vtbl {
    pub const fn new<Identity: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enabled<Identity: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportDiagnosticsPolicy_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportDiagnosticsPolicy_Impl::SetEnabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn Components<Identity: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulcomponents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportDiagnosticsPolicy_Impl::Components(this) {
                    Ok(ok__) => {
                        pulcomponents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetComponents<Identity: IWdsTransportDiagnosticsPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcomponents: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportDiagnosticsPolicy_Impl::SetComponents(this, core::mem::transmute_copy(&ulcomponents)).into()
            }
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Components: Components::<Identity, OFFSET>,
            SetComponents: SetComponents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportDiagnosticsPolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportDiagnosticsPolicy {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportManager, IWdsTransportManager_Vtbl, 0x5b0d35f5_1b13_4afd_b878_6526dc340b5d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportManager {
    pub unsafe fn GetWdsTransportServer(&self, bszservername: &windows_core::BSTR) -> windows_core::Result<IWdsTransportServer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWdsTransportServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszservername), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetWdsTransportServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportManager_Impl: super::Com::IDispatch_Impl {
    fn GetWdsTransportServer(&self, bszservername: &windows_core::BSTR) -> windows_core::Result<IWdsTransportServer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportManager_Vtbl {
    pub const fn new<Identity: IWdsTransportManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWdsTransportServer<Identity: IWdsTransportManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszservername: *mut core::ffi::c_void, ppwdstransportserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportManager_Impl::GetWdsTransportServer(this, core::mem::transmute(&bszservername)) {
                    Ok(ok__) => {
                        ppwdstransportserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetWdsTransportServer: GetWdsTransportServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportMulticastSessionPolicy, IWdsTransportMulticastSessionPolicy_Vtbl, 0x4e5753cf_68ec_4504_a951_4a003266606b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportMulticastSessionPolicy {
    type Target = IWdsTransportCacheable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportMulticastSessionPolicy, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportMulticastSessionPolicy {
    pub unsafe fn SlowClientHandling(&self) -> windows_core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SlowClientHandling)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSlowClientHandling)(windows_core::Interface::as_raw(self), slowclienthandling).ok() }
    }
    pub unsafe fn AutoDisconnectThreshold(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoDisconnectThreshold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoDisconnectThreshold)(windows_core::Interface::as_raw(self), ulthreshold).ok() }
    }
    pub unsafe fn MultistreamStreamCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultistreamStreamCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMultistreamStreamCount)(windows_core::Interface::as_raw(self), ulstreamcount).ok() }
    }
    pub unsafe fn SlowClientFallback(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SlowClientFallback)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSlowClientFallback(&self, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSlowClientFallback)(windows_core::Interface::as_raw(self), bclientfallback).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportMulticastSessionPolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    pub SlowClientHandling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT,
    pub SetSlowClientHandling: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT,
    pub AutoDisconnectThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAutoDisconnectThreshold: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MultistreamStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMultistreamStreamCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SlowClientFallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSlowClientFallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportMulticastSessionPolicy_Impl: IWdsTransportCacheable_Impl {
    fn SlowClientHandling(&self) -> windows_core::Result<WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE>;
    fn SetSlowClientHandling(&self, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::Result<()>;
    fn AutoDisconnectThreshold(&self) -> windows_core::Result<u32>;
    fn SetAutoDisconnectThreshold(&self, ulthreshold: u32) -> windows_core::Result<()>;
    fn MultistreamStreamCount(&self) -> windows_core::Result<u32>;
    fn SetMultistreamStreamCount(&self, ulstreamcount: u32) -> windows_core::Result<()>;
    fn SlowClientFallback(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetSlowClientFallback(&self, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportMulticastSessionPolicy_Vtbl {
    pub const fn new<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SlowClientHandling<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pslowclienthandling: *mut WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportMulticastSessionPolicy_Impl::SlowClientHandling(this) {
                    Ok(ok__) => {
                        pslowclienthandling.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSlowClientHandling<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, slowclienthandling: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportMulticastSessionPolicy_Impl::SetSlowClientHandling(this, core::mem::transmute_copy(&slowclienthandling)).into()
            }
        }
        unsafe extern "system" fn AutoDisconnectThreshold<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulthreshold: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportMulticastSessionPolicy_Impl::AutoDisconnectThreshold(this) {
                    Ok(ok__) => {
                        pulthreshold.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoDisconnectThreshold<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulthreshold: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportMulticastSessionPolicy_Impl::SetAutoDisconnectThreshold(this, core::mem::transmute_copy(&ulthreshold)).into()
            }
        }
        unsafe extern "system" fn MultistreamStreamCount<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstreamcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportMulticastSessionPolicy_Impl::MultistreamStreamCount(this) {
                    Ok(ok__) => {
                        pulstreamcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMultistreamStreamCount<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstreamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportMulticastSessionPolicy_Impl::SetMultistreamStreamCount(this, core::mem::transmute_copy(&ulstreamcount)).into()
            }
        }
        unsafe extern "system" fn SlowClientFallback<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbclientfallback: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportMulticastSessionPolicy_Impl::SlowClientFallback(this) {
                    Ok(ok__) => {
                        pbclientfallback.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSlowClientFallback<Identity: IWdsTransportMulticastSessionPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bclientfallback: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportMulticastSessionPolicy_Impl::SetSlowClientFallback(this, core::mem::transmute_copy(&bclientfallback)).into()
            }
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, OFFSET>(),
            SlowClientHandling: SlowClientHandling::<Identity, OFFSET>,
            SetSlowClientHandling: SetSlowClientHandling::<Identity, OFFSET>,
            AutoDisconnectThreshold: AutoDisconnectThreshold::<Identity, OFFSET>,
            SetAutoDisconnectThreshold: SetAutoDisconnectThreshold::<Identity, OFFSET>,
            MultistreamStreamCount: MultistreamStreamCount::<Identity, OFFSET>,
            SetMultistreamStreamCount: SetMultistreamStreamCount::<Identity, OFFSET>,
            SlowClientFallback: SlowClientFallback::<Identity, OFFSET>,
            SetSlowClientFallback: SetSlowClientFallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportMulticastSessionPolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportMulticastSessionPolicy {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespace, IWdsTransportNamespace_Vtbl, 0xfa561f57_fbef_4ed3_b056_127cb1b33b84);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespace {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespace, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespace {
    pub unsafe fn Type(&self) -> windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszname)).ok() }
    }
    pub unsafe fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFriendlyName(&self, bszfriendlyname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszfriendlyname)).ok() }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bszdescription: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszdescription)).ok() }
    }
    pub unsafe fn ContentProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentProvider)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetContentProvider(&self, bszcontentprovider: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetContentProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszcontentprovider)).ok() }
    }
    pub unsafe fn Configuration(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Configuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetConfiguration(&self, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszconfiguration)).ok() }
    }
    pub unsafe fn Registered(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Registered)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Tombstoned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tombstoned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TombstoneTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TombstoneTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TransmissionStarted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransmissionStarted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Register(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Deregister(&self, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Deregister)(windows_core::Interface::as_raw(self), bterminatesessions).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IWdsTransportNamespace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn RetrieveContents(&self) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveContents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WDSTRANSPORT_NAMESPACE_TYPE) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContentProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Registered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Tombstoned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub TombstoneTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TransmissionStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Deregister: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrieveContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespace_Impl: super::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<WDSTRANSPORT_NAMESPACE_TYPE>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFriendlyName(&self, bszfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bszdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ContentProvider(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContentProvider(&self, bszcontentprovider: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Configuration(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetConfiguration(&self, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Registered(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Tombstoned(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TombstoneTime(&self) -> windows_core::Result<f64>;
    fn TransmissionStarted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Register(&self) -> windows_core::Result<()>;
    fn Deregister(&self, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IWdsTransportNamespace>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn RetrieveContents(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespace_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut WDSTRANSPORT_NAMESPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Type(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Id(this) {
                    Ok(ok__) => {
                        pulid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Name(this) {
                    Ok(ok__) => {
                        pbszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::SetName(this, core::mem::transmute(&bszname)).into()
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        pbszfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszfriendlyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::SetFriendlyName(this, core::mem::transmute(&bszfriendlyname)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Description(this) {
                    Ok(ok__) => {
                        pbszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszdescription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::SetDescription(this, core::mem::transmute(&bszdescription)).into()
            }
        }
        unsafe extern "system" fn ContentProvider<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszcontentprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::ContentProvider(this) {
                    Ok(ok__) => {
                        pbszcontentprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentProvider<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszcontentprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::SetContentProvider(this, core::mem::transmute(&bszcontentprovider)).into()
            }
        }
        unsafe extern "system" fn Configuration<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Configuration(this) {
                    Ok(ok__) => {
                        pbszconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConfiguration<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszconfiguration: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::SetConfiguration(this, core::mem::transmute(&bszconfiguration)).into()
            }
        }
        unsafe extern "system" fn Registered<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbregistered: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Registered(this) {
                    Ok(ok__) => {
                        pbregistered.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Tombstoned<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtombstoned: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Tombstoned(this) {
                    Ok(ok__) => {
                        pbtombstoned.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TombstoneTime<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptombstonetime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::TombstoneTime(this) {
                    Ok(ok__) => {
                        ptombstonetime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransmissionStarted<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbtransmissionstarted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::TransmissionStarted(this) {
                    Ok(ok__) => {
                        pbtransmissionstarted.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Register<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::Register(this).into()
            }
        }
        unsafe extern "system" fn Deregister<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bterminatesessions: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::Deregister(this, core::mem::transmute_copy(&bterminatesessions)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespaceclone: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppwdstransportnamespaceclone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Refresh<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespace_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn RetrieveContents<Identity: IWdsTransportNamespace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportcontents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespace_Impl::RetrieveContents(this) {
                    Ok(ok__) => {
                        ppwdstransportcontents.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            ContentProvider: ContentProvider::<Identity, OFFSET>,
            SetContentProvider: SetContentProvider::<Identity, OFFSET>,
            Configuration: Configuration::<Identity, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            Registered: Registered::<Identity, OFFSET>,
            Tombstoned: Tombstoned::<Identity, OFFSET>,
            TombstoneTime: TombstoneTime::<Identity, OFFSET>,
            TransmissionStarted: TransmissionStarted::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Deregister: Deregister::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            RetrieveContents: RetrieveContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespace {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespaceAutoCast, IWdsTransportNamespaceAutoCast_Vtbl, 0xad931a72_c4bd_4c41_8fbc_59c9c748df9e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespaceAutoCast {
    type Target = IWdsTransportNamespace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceAutoCast, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceAutoCast_Vtbl {
    pub base__: IWdsTransportNamespace_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceAutoCast_Impl: IWdsTransportNamespace_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespaceAutoCast_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespaceAutoCast_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceAutoCast as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespaceAutoCast {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespaceManager, IWdsTransportNamespaceManager_Vtbl, 0x3e22d9f6_3777_4d98_83e1_f98696717ba3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespaceManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceManager {
    pub unsafe fn CreateNamespace(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: &windows_core::BSTR, bszcontentprovider: &windows_core::BSTR, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNamespace)(windows_core::Interface::as_raw(self), namespacetype, core::mem::transmute_copy(bsznamespacename), core::mem::transmute_copy(bszcontentprovider), core::mem::transmute_copy(bszconfiguration), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RetrieveNamespace(&self, bsznamespacename: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveNamespace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bsznamespacename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RetrieveNamespaces(&self, bszcontentprovider: &windows_core::BSTR, bsznamespacename: &windows_core::BSTR, bincludetombstones: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveNamespaces)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszcontentprovider), core::mem::transmute_copy(bsznamespacename), bincludetombstones, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CreateNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_NAMESPACE_TYPE, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrieveNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetrieveNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceManager_Impl: super::Com::IDispatch_Impl {
    fn CreateNamespace(&self, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: &windows_core::BSTR, bszcontentprovider: &windows_core::BSTR, bszconfiguration: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespace(&self, bsznamespacename: &windows_core::BSTR) -> windows_core::Result<IWdsTransportNamespace>;
    fn RetrieveNamespaces(&self, bszcontentprovider: &windows_core::BSTR, bsznamespacename: &windows_core::BSTR, bincludetombstones: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespaceManager_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateNamespace<Identity: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespacetype: WDSTRANSPORT_NAMESPACE_TYPE, bsznamespacename: *mut core::ffi::c_void, bszcontentprovider: *mut core::ffi::c_void, bszconfiguration: *mut core::ffi::c_void, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespaceManager_Impl::CreateNamespace(this, core::mem::transmute_copy(&namespacetype), core::mem::transmute(&bsznamespacename), core::mem::transmute(&bszcontentprovider), core::mem::transmute(&bszconfiguration)) {
                    Ok(ok__) => {
                        ppwdstransportnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RetrieveNamespace<Identity: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsznamespacename: *mut core::ffi::c_void, ppwdstransportnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespaceManager_Impl::RetrieveNamespace(this, core::mem::transmute(&bsznamespacename)) {
                    Ok(ok__) => {
                        ppwdstransportnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RetrieveNamespaces<Identity: IWdsTransportNamespaceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszcontentprovider: *mut core::ffi::c_void, bsznamespacename: *mut core::ffi::c_void, bincludetombstones: super::super::Foundation::VARIANT_BOOL, ppwdstransportnamespaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespaceManager_Impl::RetrieveNamespaces(this, core::mem::transmute(&bszcontentprovider), core::mem::transmute(&bsznamespacename), core::mem::transmute_copy(&bincludetombstones)) {
                    Ok(ok__) => {
                        ppwdstransportnamespaces.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateNamespace: CreateNamespace::<Identity, OFFSET>,
            RetrieveNamespace: RetrieveNamespace::<Identity, OFFSET>,
            RetrieveNamespaces: RetrieveNamespaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespaceManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespaceScheduledCast, IWdsTransportNamespaceScheduledCast_Vtbl, 0x3840cecf_d76c_416e_a4cc_31c741d2874b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespaceScheduledCast {
    type Target = IWdsTransportNamespace;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCast, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCast {
    pub unsafe fn StartTransmission(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartTransmission)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCast_Vtbl {
    pub base__: IWdsTransportNamespace_Vtbl,
    pub StartTransmission: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCast_Impl: IWdsTransportNamespace_Impl {
    fn StartTransmission(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespaceScheduledCast_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartTransmission<Identity: IWdsTransportNamespaceScheduledCast_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespaceScheduledCast_Impl::StartTransmission(this).into()
            }
        }
        Self { base__: IWdsTransportNamespace_Vtbl::new::<Identity, OFFSET>(), StartTransmission: StartTransmission::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCast {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespaceScheduledCastAutoStart, IWdsTransportNamespaceScheduledCastAutoStart_Vtbl, 0xd606af3d_ea9c_4219_961e_7491d618d9b9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespaceScheduledCastAutoStart {
    type Target = IWdsTransportNamespaceScheduledCast;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCastAutoStart, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace, IWdsTransportNamespaceScheduledCast);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportNamespaceScheduledCastAutoStart {
    pub unsafe fn MinimumClients(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumClients)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumClients(&self, ulminimumclients: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumClients)(windows_core::Interface::as_raw(self), ulminimumclients).ok() }
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartTime(&self, starttime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), starttime).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub base__: IWdsTransportNamespaceScheduledCast_Vtbl,
    pub MinimumClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinimumClients: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCastAutoStart_Impl: IWdsTransportNamespaceScheduledCast_Impl {
    fn MinimumClients(&self) -> windows_core::Result<u32>;
    fn SetMinimumClients(&self, ulminimumclients: u32) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<f64>;
    fn SetStartTime(&self, starttime: f64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespaceScheduledCastAutoStart_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MinimumClients<Identity: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulminimumclients: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespaceScheduledCastAutoStart_Impl::MinimumClients(this) {
                    Ok(ok__) => {
                        pulminimumclients.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumClients<Identity: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulminimumclients: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespaceScheduledCastAutoStart_Impl::SetMinimumClients(this, core::mem::transmute_copy(&ulminimumclients)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstarttime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportNamespaceScheduledCastAutoStart_Impl::StartTime(this) {
                    Ok(ok__) => {
                        pstarttime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: IWdsTransportNamespaceScheduledCastAutoStart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportNamespaceScheduledCastAutoStart_Impl::SetStartTime(this, core::mem::transmute_copy(&starttime)).into()
            }
        }
        Self {
            base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, OFFSET>(),
            MinimumClients: MinimumClients::<Identity, OFFSET>,
            SetMinimumClients: SetMinimumClients::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastAutoStart as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCastAutoStart {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportNamespaceScheduledCastManualStart, IWdsTransportNamespaceScheduledCastManualStart_Vtbl, 0x013e6e4c_e6a7_4fb5_b7ff_d9f5da805c31);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportNamespaceScheduledCastManualStart {
    type Target = IWdsTransportNamespaceScheduledCast;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportNamespaceScheduledCastManualStart, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportNamespace, IWdsTransportNamespaceScheduledCast);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub base__: IWdsTransportNamespaceScheduledCast_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportNamespaceScheduledCastManualStart_Impl: IWdsTransportNamespaceScheduledCast_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportNamespaceScheduledCastManualStart_Vtbl {
    pub const fn new<Identity: IWdsTransportNamespaceScheduledCastManualStart_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IWdsTransportNamespaceScheduledCast_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportNamespaceScheduledCastManualStart as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportNamespace as windows_core::Interface>::IID || iid == &<IWdsTransportNamespaceScheduledCast as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportNamespaceScheduledCastManualStart {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportServer, IWdsTransportServer_Vtbl, 0x09ccd093_830d_4344_a30a_73ae8e8fca90);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportServer {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportServer, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetupManager(&self) -> windows_core::Result<IWdsTransportSetupManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetupManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ConfigurationManager(&self) -> windows_core::Result<IWdsTransportConfigurationManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConfigurationManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NamespaceManager(&self) -> windows_core::Result<IWdsTransportNamespaceManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisconnectClient)(windows_core::Interface::as_raw(self), ulclientid, disconnectiontype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetupManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ConfigurationManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectClient: unsafe extern "system" fn(*mut core::ffi::c_void, u32, WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServer_Impl: super::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetupManager(&self) -> windows_core::Result<IWdsTransportSetupManager>;
    fn ConfigurationManager(&self) -> windows_core::Result<IWdsTransportConfigurationManager>;
    fn NamespaceManager(&self) -> windows_core::Result<IWdsTransportNamespaceManager>;
    fn DisconnectClient(&self, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportServer_Vtbl {
    pub const fn new<Identity: IWdsTransportServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServer_Impl::Name(this) {
                    Ok(ok__) => {
                        pbszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetupManager<Identity: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportsetupmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServer_Impl::SetupManager(this) {
                    Ok(ok__) => {
                        ppwdstransportsetupmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConfigurationManager<Identity: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportconfigurationmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServer_Impl::ConfigurationManager(this) {
                    Ok(ok__) => {
                        ppwdstransportconfigurationmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NamespaceManager<Identity: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportnamespacemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServer_Impl::NamespaceManager(this) {
                    Ok(ok__) => {
                        ppwdstransportnamespacemanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisconnectClient<Identity: IWdsTransportServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulclientid: u32, disconnectiontype: WDSTRANSPORT_DISCONNECT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServer_Impl::DisconnectClient(this, core::mem::transmute_copy(&ulclientid), core::mem::transmute_copy(&disconnectiontype)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetupManager: SetupManager::<Identity, OFFSET>,
            ConfigurationManager: ConfigurationManager::<Identity, OFFSET>,
            NamespaceManager: NamespaceManager::<Identity, OFFSET>,
            DisconnectClient: DisconnectClient::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServer as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportServer {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportServer2, IWdsTransportServer2_Vtbl, 0x256e999f_6df4_4538_81b9_857b9ab8fb47);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportServer2 {
    type Target = IWdsTransportServer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportServer2, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportServer);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServer2 {
    pub unsafe fn TftpManager(&self) -> windows_core::Result<IWdsTransportTftpManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TftpManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServer2_Vtbl {
    pub base__: IWdsTransportServer_Vtbl,
    pub TftpManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServer2_Impl: IWdsTransportServer_Impl {
    fn TftpManager(&self) -> windows_core::Result<IWdsTransportTftpManager>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportServer2_Vtbl {
    pub const fn new<Identity: IWdsTransportServer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TftpManager<Identity: IWdsTransportServer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransporttftpmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServer2_Impl::TftpManager(this) {
                    Ok(ok__) => {
                        ppwdstransporttftpmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWdsTransportServer_Vtbl::new::<Identity, OFFSET>(), TftpManager: TftpManager::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServer2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportServer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportServer2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportServicePolicy, IWdsTransportServicePolicy_Vtbl, 0xb9468578_9f2b_48cc_b27a_a60799c2750c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportServicePolicy {
    type Target = IWdsTransportCacheable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportServicePolicy, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy {
    pub unsafe fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IpAddressSource)(windows_core::Interface::as_raw(self), addresstype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_IpAddressSource)(windows_core::Interface::as_raw(self), addresstype, sourcetype).ok() }
    }
    pub unsafe fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartIpAddress)(windows_core::Interface::as_raw(self), addresstype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_StartIpAddress)(windows_core::Interface::as_raw(self), addresstype, core::mem::transmute_copy(bszstartipaddress)).ok() }
    }
    pub unsafe fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EndIpAddress)(windows_core::Interface::as_raw(self), addresstype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).put_EndIpAddress)(windows_core::Interface::as_raw(self), addresstype, core::mem::transmute_copy(bszendipaddress)).ok() }
    }
    pub unsafe fn StartPort(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartPort(&self, ulstartport: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartPort)(windows_core::Interface::as_raw(self), ulstartport).ok() }
    }
    pub unsafe fn EndPort(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEndPort(&self, ulendport: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEndPort)(windows_core::Interface::as_raw(self), ulendport).ok() }
    }
    pub unsafe fn NetworkProfile(&self) -> windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkProfile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkProfile)(windows_core::Interface::as_raw(self), profiletype).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy_Vtbl {
    pub base__: IWdsTransportCacheable_Vtbl,
    pub get_IpAddressSource: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT,
    pub put_IpAddressSource: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT,
    pub get_StartIpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_StartIpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EndIpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_EndIpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_IP_ADDRESS_TYPE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetStartPort: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EndPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetEndPort: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NetworkProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT,
    pub SetNetworkProfile: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServicePolicy_Impl: IWdsTransportCacheable_Impl {
    fn get_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE>;
    fn put_IpAddressSource(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::Result<()>;
    fn get_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn put_StartIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE) -> windows_core::Result<windows_core::BSTR>;
    fn put_EndIpAddress(&self, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StartPort(&self) -> windows_core::Result<u32>;
    fn SetStartPort(&self, ulstartport: u32) -> windows_core::Result<()>;
    fn EndPort(&self) -> windows_core::Result<u32>;
    fn SetEndPort(&self, ulendport: u32) -> windows_core::Result<()>;
    fn NetworkProfile(&self) -> windows_core::Result<WDSTRANSPORT_NETWORK_PROFILE_TYPE>;
    fn SetNetworkProfile(&self, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportServicePolicy_Vtbl {
    pub const fn new<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IpAddressSource<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, psourcetype: *mut WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::get_IpAddressSource(this, core::mem::transmute_copy(&addresstype)) {
                    Ok(ok__) => {
                        psourcetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IpAddressSource<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, sourcetype: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::put_IpAddressSource(this, core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&sourcetype)).into()
            }
        }
        unsafe extern "system" fn get_StartIpAddress<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszstartipaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::get_StartIpAddress(this, core::mem::transmute_copy(&addresstype)) {
                    Ok(ok__) => {
                        pbszstartipaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartIpAddress<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszstartipaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::put_StartIpAddress(this, core::mem::transmute_copy(&addresstype), core::mem::transmute(&bszstartipaddress)).into()
            }
        }
        unsafe extern "system" fn get_EndIpAddress<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, pbszendipaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::get_EndIpAddress(this, core::mem::transmute_copy(&addresstype)) {
                    Ok(ok__) => {
                        pbszendipaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EndIpAddress<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, addresstype: WDSTRANSPORT_IP_ADDRESS_TYPE, bszendipaddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::put_EndIpAddress(this, core::mem::transmute_copy(&addresstype), core::mem::transmute(&bszendipaddress)).into()
            }
        }
        unsafe extern "system" fn StartPort<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulstartport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::StartPort(this) {
                    Ok(ok__) => {
                        pulstartport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartPort<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartport: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::SetStartPort(this, core::mem::transmute_copy(&ulstartport)).into()
            }
        }
        unsafe extern "system" fn EndPort<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulendport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::EndPort(this) {
                    Ok(ok__) => {
                        pulendport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndPort<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulendport: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::SetEndPort(this, core::mem::transmute_copy(&ulendport)).into()
            }
        }
        unsafe extern "system" fn NetworkProfile<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofiletype: *mut WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy_Impl::NetworkProfile(this) {
                    Ok(ok__) => {
                        pprofiletype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkProfile<Identity: IWdsTransportServicePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profiletype: WDSTRANSPORT_NETWORK_PROFILE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy_Impl::SetNetworkProfile(this, core::mem::transmute_copy(&profiletype)).into()
            }
        }
        Self {
            base__: IWdsTransportCacheable_Vtbl::new::<Identity, OFFSET>(),
            get_IpAddressSource: get_IpAddressSource::<Identity, OFFSET>,
            put_IpAddressSource: put_IpAddressSource::<Identity, OFFSET>,
            get_StartIpAddress: get_StartIpAddress::<Identity, OFFSET>,
            put_StartIpAddress: put_StartIpAddress::<Identity, OFFSET>,
            get_EndIpAddress: get_EndIpAddress::<Identity, OFFSET>,
            put_EndIpAddress: put_EndIpAddress::<Identity, OFFSET>,
            StartPort: StartPort::<Identity, OFFSET>,
            SetStartPort: SetStartPort::<Identity, OFFSET>,
            EndPort: EndPort::<Identity, OFFSET>,
            SetEndPort: SetEndPort::<Identity, OFFSET>,
            NetworkProfile: NetworkProfile::<Identity, OFFSET>,
            SetNetworkProfile: SetNetworkProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportServicePolicy {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportServicePolicy2, IWdsTransportServicePolicy2_Vtbl, 0x65c19e5c_aa7e_4b91_8944_91e0e5572797);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportServicePolicy2 {
    type Target = IWdsTransportServicePolicy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportServicePolicy2, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportCacheable, IWdsTransportServicePolicy);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportServicePolicy2 {
    pub unsafe fn UdpPortPolicy(&self) -> windows_core::Result<WDSTRANSPORT_UDP_PORT_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UdpPortPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUdpPortPolicy)(windows_core::Interface::as_raw(self), udpportpolicy).ok() }
    }
    pub unsafe fn TftpMaximumBlockSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TftpMaximumBlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTftpMaximumBlockSize)(windows_core::Interface::as_raw(self), ultftpmaximumblocksize).ok() }
    }
    pub unsafe fn EnableTftpVariableWindowExtension(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnableTftpVariableWindowExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnableTftpVariableWindowExtension(&self, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnableTftpVariableWindowExtension)(windows_core::Interface::as_raw(self), benabletftpvariablewindowextension).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportServicePolicy2_Vtbl {
    pub base__: IWdsTransportServicePolicy_Vtbl,
    pub UdpPortPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT,
    pub SetUdpPortPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT,
    pub TftpMaximumBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTftpMaximumBlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnableTftpVariableWindowExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetEnableTftpVariableWindowExtension: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportServicePolicy2_Impl: IWdsTransportServicePolicy_Impl {
    fn UdpPortPolicy(&self) -> windows_core::Result<WDSTRANSPORT_UDP_PORT_POLICY>;
    fn SetUdpPortPolicy(&self, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::Result<()>;
    fn TftpMaximumBlockSize(&self) -> windows_core::Result<u32>;
    fn SetTftpMaximumBlockSize(&self, ultftpmaximumblocksize: u32) -> windows_core::Result<()>;
    fn EnableTftpVariableWindowExtension(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetEnableTftpVariableWindowExtension(&self, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportServicePolicy2_Vtbl {
    pub const fn new<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UdpPortPolicy<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pudpportpolicy: *mut WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy2_Impl::UdpPortPolicy(this) {
                    Ok(ok__) => {
                        pudpportpolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUdpPortPolicy<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, udpportpolicy: WDSTRANSPORT_UDP_PORT_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy2_Impl::SetUdpPortPolicy(this, core::mem::transmute_copy(&udpportpolicy)).into()
            }
        }
        unsafe extern "system" fn TftpMaximumBlockSize<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultftpmaximumblocksize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy2_Impl::TftpMaximumBlockSize(this) {
                    Ok(ok__) => {
                        pultftpmaximumblocksize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTftpMaximumBlockSize<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultftpmaximumblocksize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy2_Impl::SetTftpMaximumBlockSize(this, core::mem::transmute_copy(&ultftpmaximumblocksize)).into()
            }
        }
        unsafe extern "system" fn EnableTftpVariableWindowExtension<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabletftpvariablewindowextension: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportServicePolicy2_Impl::EnableTftpVariableWindowExtension(this) {
                    Ok(ok__) => {
                        pbenabletftpvariablewindowextension.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnableTftpVariableWindowExtension<Identity: IWdsTransportServicePolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabletftpvariablewindowextension: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportServicePolicy2_Impl::SetEnableTftpVariableWindowExtension(this, core::mem::transmute_copy(&benabletftpvariablewindowextension)).into()
            }
        }
        Self {
            base__: IWdsTransportServicePolicy_Vtbl::new::<Identity, OFFSET>(),
            UdpPortPolicy: UdpPortPolicy::<Identity, OFFSET>,
            SetUdpPortPolicy: SetUdpPortPolicy::<Identity, OFFSET>,
            TftpMaximumBlockSize: TftpMaximumBlockSize::<Identity, OFFSET>,
            SetTftpMaximumBlockSize: SetTftpMaximumBlockSize::<Identity, OFFSET>,
            EnableTftpVariableWindowExtension: EnableTftpVariableWindowExtension::<Identity, OFFSET>,
            SetEnableTftpVariableWindowExtension: SetEnableTftpVariableWindowExtension::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportServicePolicy2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportCacheable as windows_core::Interface>::IID || iid == &<IWdsTransportServicePolicy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportServicePolicy2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportSession, IWdsTransportSession_Vtbl, 0xf4efea88_65b1_4f30_a4b9_2793987796fb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportSession {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportSession, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSession {
    pub unsafe fn Content(&self) -> windows_core::Result<IWdsTransportContent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NetworkInterfaceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkInterfaceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn NetworkInterfaceAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkInterfaceAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn TransferRate(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransferRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MasterClientId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MasterClientId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RetrieveClients(&self) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveClients)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Terminate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub NetworkInterfaceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NetworkInterfaceAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MasterClientId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RetrieveClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSession_Impl: super::Com::IDispatch_Impl {
    fn Content(&self) -> windows_core::Result<IWdsTransportContent>;
    fn Id(&self) -> windows_core::Result<u32>;
    fn NetworkInterfaceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn NetworkInterfaceAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn TransferRate(&self) -> windows_core::Result<u32>;
    fn MasterClientId(&self) -> windows_core::Result<u32>;
    fn RetrieveClients(&self) -> windows_core::Result<IWdsTransportCollection>;
    fn Terminate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportSession_Vtbl {
    pub const fn new<Identity: IWdsTransportSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Content<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::Content(this) {
                    Ok(ok__) => {
                        ppwdstransportcontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::Id(this) {
                    Ok(ok__) => {
                        pulid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetworkInterfaceName<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsznetworkinterfacename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::NetworkInterfaceName(this) {
                    Ok(ok__) => {
                        pbsznetworkinterfacename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetworkInterfaceAddress<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsznetworkinterfaceaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::NetworkInterfaceAddress(this) {
                    Ok(ok__) => {
                        pbsznetworkinterfaceaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TransferRate<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultransferrate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::TransferRate(this) {
                    Ok(ok__) => {
                        pultransferrate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MasterClientId<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmasterclientid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::MasterClientId(this) {
                    Ok(ok__) => {
                        pulmasterclientid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RetrieveClients<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransportclients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSession_Impl::RetrieveClients(this) {
                    Ok(ok__) => {
                        ppwdstransportclients.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Terminate<Identity: IWdsTransportSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportSession_Impl::Terminate(this).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Content: Content::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            NetworkInterfaceName: NetworkInterfaceName::<Identity, OFFSET>,
            NetworkInterfaceAddress: NetworkInterfaceAddress::<Identity, OFFSET>,
            TransferRate: TransferRate::<Identity, OFFSET>,
            MasterClientId: MasterClientId::<Identity, OFFSET>,
            RetrieveClients: RetrieveClients::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSession as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportSession {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportSetupManager, IWdsTransportSetupManager_Vtbl, 0xf7238425_efa8_40a4_aef9_c98d969c0b75);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportSetupManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportSetupManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager {
    pub unsafe fn Version(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InstalledFeatures(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstalledFeatures)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Protocols(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Protocols)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterContentProvider(&self, bszname: &windows_core::BSTR, bszdescription: &windows_core::BSTR, bszfilepath: &windows_core::BSTR, bszinitializationroutine: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RegisterContentProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszname), core::mem::transmute_copy(bszdescription), core::mem::transmute_copy(bszfilepath), core::mem::transmute_copy(bszinitializationroutine)).ok() }
    }
    pub unsafe fn DeregisterContentProvider(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeregisterContentProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bszname)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InstalledFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Protocols: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RegisterContentProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeregisterContentProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSetupManager_Impl: super::Com::IDispatch_Impl {
    fn Version(&self) -> windows_core::Result<u64>;
    fn InstalledFeatures(&self) -> windows_core::Result<u32>;
    fn Protocols(&self) -> windows_core::Result<u32>;
    fn RegisterContentProvider(&self, bszname: &windows_core::BSTR, bszdescription: &windows_core::BSTR, bszfilepath: &windows_core::BSTR, bszinitializationroutine: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeregisterContentProvider(&self, bszname: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportSetupManager_Vtbl {
    pub const fn new<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Version<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSetupManager_Impl::Version(this) {
                    Ok(ok__) => {
                        pullversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InstalledFeatures<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulinstalledfeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSetupManager_Impl::InstalledFeatures(this) {
                    Ok(ok__) => {
                        pulinstalledfeatures.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Protocols<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulprotocols: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSetupManager_Impl::Protocols(this) {
                    Ok(ok__) => {
                        pulprotocols.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterContentProvider<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: *mut core::ffi::c_void, bszdescription: *mut core::ffi::c_void, bszfilepath: *mut core::ffi::c_void, bszinitializationroutine: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportSetupManager_Impl::RegisterContentProvider(this, core::mem::transmute(&bszname), core::mem::transmute(&bszdescription), core::mem::transmute(&bszfilepath), core::mem::transmute(&bszinitializationroutine)).into()
            }
        }
        unsafe extern "system" fn DeregisterContentProvider<Identity: IWdsTransportSetupManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bszname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWdsTransportSetupManager_Impl::DeregisterContentProvider(this, core::mem::transmute(&bszname)).into()
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Version: Version::<Identity, OFFSET>,
            InstalledFeatures: InstalledFeatures::<Identity, OFFSET>,
            Protocols: Protocols::<Identity, OFFSET>,
            RegisterContentProvider: RegisterContentProvider::<Identity, OFFSET>,
            DeregisterContentProvider: DeregisterContentProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportSetupManager {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportSetupManager2, IWdsTransportSetupManager2_Vtbl, 0x02be79da_7e9e_4366_8b6e_2aa9a91be47f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportSetupManager2 {
    type Target = IWdsTransportSetupManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportSetupManager2, windows_core::IUnknown, super::Com::IDispatch, IWdsTransportSetupManager);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportSetupManager2 {
    pub unsafe fn TftpCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TftpCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ContentProviders(&self) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportSetupManager2_Vtbl {
    pub base__: IWdsTransportSetupManager_Vtbl,
    pub TftpCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ContentProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportSetupManager2_Impl: IWdsTransportSetupManager_Impl {
    fn TftpCapabilities(&self) -> windows_core::Result<u32>;
    fn ContentProviders(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportSetupManager2_Vtbl {
    pub const fn new<Identity: IWdsTransportSetupManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TftpCapabilities<Identity: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultftpcapabilities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSetupManager2_Impl::TftpCapabilities(this) {
                    Ok(ok__) => {
                        pultftpcapabilities.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentProviders<Identity: IWdsTransportSetupManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprovidercollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportSetupManager2_Impl::ContentProviders(this) {
                    Ok(ok__) => {
                        ppprovidercollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWdsTransportSetupManager_Vtbl::new::<Identity, OFFSET>(),
            TftpCapabilities: TftpCapabilities::<Identity, OFFSET>,
            ContentProviders: ContentProviders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportSetupManager2 as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWdsTransportSetupManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportSetupManager2 {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportTftpClient, IWdsTransportTftpClient_Vtbl, 0xb022d3ae_884d_4d85_b146_53320e76ef62);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportTftpClient {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportTftpClient, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpClient {
    pub unsafe fn FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IpAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Timeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFileOffset(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFileOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileSize(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BlockSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WindowSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpClient_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IpAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentFileOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub FileSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub WindowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportTftpClient_Impl: super::Com::IDispatch_Impl {
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IpAddress(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Timeout(&self) -> windows_core::Result<u32>;
    fn CurrentFileOffset(&self) -> windows_core::Result<u64>;
    fn FileSize(&self) -> windows_core::Result<u64>;
    fn BlockSize(&self) -> windows_core::Result<u32>;
    fn WindowSize(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportTftpClient_Vtbl {
    pub const fn new<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileName<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::FileName(this) {
                    Ok(ok__) => {
                        pbszfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IpAddress<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbszipaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::IpAddress(this) {
                    Ok(ok__) => {
                        pbszipaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timeout<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultimeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::Timeout(this) {
                    Ok(ok__) => {
                        pultimeout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFileOffset<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul64currentoffset: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::CurrentFileOffset(this) {
                    Ok(ok__) => {
                        pul64currentoffset.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileSize<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul64filesize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::FileSize(this) {
                    Ok(ok__) => {
                        pul64filesize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BlockSize<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulblocksize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::BlockSize(this) {
                    Ok(ok__) => {
                        pulblocksize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WindowSize<Identity: IWdsTransportTftpClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulwindowsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpClient_Impl::WindowSize(this) {
                    Ok(ok__) => {
                        pulwindowsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FileName: FileName::<Identity, OFFSET>,
            IpAddress: IpAddress::<Identity, OFFSET>,
            Timeout: Timeout::<Identity, OFFSET>,
            CurrentFileOffset: CurrentFileOffset::<Identity, OFFSET>,
            FileSize: FileSize::<Identity, OFFSET>,
            BlockSize: BlockSize::<Identity, OFFSET>,
            WindowSize: WindowSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportTftpClient as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportTftpClient {}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWdsTransportTftpManager, IWdsTransportTftpManager_Vtbl, 0x1327a7c8_ae8a_4fb3_8150_136227c37e9a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWdsTransportTftpManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWdsTransportTftpManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWdsTransportTftpManager {
    pub unsafe fn RetrieveTftpClients(&self) -> windows_core::Result<IWdsTransportCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrieveTftpClients)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWdsTransportTftpManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RetrieveTftpClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWdsTransportTftpManager_Impl: super::Com::IDispatch_Impl {
    fn RetrieveTftpClients(&self) -> windows_core::Result<IWdsTransportCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWdsTransportTftpManager_Vtbl {
    pub const fn new<Identity: IWdsTransportTftpManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RetrieveTftpClients<Identity: IWdsTransportTftpManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwdstransporttftpclients: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWdsTransportTftpManager_Impl::RetrieveTftpClients(this) {
                    Ok(ok__) => {
                        ppwdstransporttftpclients.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), RetrieveTftpClients: RetrieveTftpClients::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWdsTransportTftpManager as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWdsTransportTftpManager {}
pub const MC_SERVER_CURRENT_VERSION: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFN_WDS_CLI_CALLBACK_MESSAGE_ID(pub u32);
pub type PFN_WdsCliCallback = Option<unsafe extern "system" fn(dwmessageid: PFN_WDS_CLI_CALLBACK_MESSAGE_ID, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pvuserdata: *const core::ffi::c_void)>;
pub type PFN_WdsCliTraceFunction = Option<unsafe extern "system" fn(pwszformat: windows_core::PCWSTR, params: *const i8)>;
pub type PFN_WdsTransportClientReceiveContents = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, pcontents: *const core::ffi::c_void, ulsize: u32, pullcontentoffset: *const u64)>;
pub type PFN_WdsTransportClientReceiveMetadata = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, pmetadata: *const core::ffi::c_void, ulsize: u32)>;
pub type PFN_WdsTransportClientSessionComplete = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, dwerror: u32)>;
pub type PFN_WdsTransportClientSessionNegotiate = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, pinfo: *const TRANSPORTCLIENT_SESSION_INFO, hnegotiatekey: super::super::Foundation::HANDLE)>;
pub type PFN_WdsTransportClientSessionStart = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, ullfilesize: *const u64)>;
pub type PFN_WdsTransportClientSessionStartEx = Option<unsafe extern "system" fn(hsessionkey: super::super::Foundation::HANDLE, pcallerdata: *const core::ffi::c_void, info: *const TRANSPORTCLIENT_SESSION_INFO)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PXE_ADDRESS {
    pub uFlags: u32,
    pub Anonymous: PXE_ADDRESS_0,
    pub uAddrLen: u32,
    pub uPort: u16,
}
impl Default for PXE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PXE_ADDRESS_0 {
    pub bAddress: [u8; 16],
    pub uIpAddress: u32,
}
impl Default for PXE_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PXE_ADDR_BROADCAST: u32 = 1u32;
pub const PXE_ADDR_USE_ADDR: u32 = 4u32;
pub const PXE_ADDR_USE_DHCP_RULES: u32 = 8u32;
pub const PXE_ADDR_USE_PORT: u32 = 2u32;
pub const PXE_BA_CUSTOM: u32 = 2u32;
pub const PXE_BA_IGNORE: u32 = 3u32;
pub const PXE_BA_NBP: u32 = 1u32;
pub const PXE_BA_REJECTED: u32 = 4u32;
pub const PXE_CALLBACK_MAX: u32 = 3u32;
pub const PXE_CALLBACK_RECV_REQUEST: u32 = 0u32;
pub const PXE_CALLBACK_SERVICE_CONTROL: u32 = 2u32;
pub const PXE_CALLBACK_SHUTDOWN: u32 = 1u32;
pub const PXE_DHCPV6_CLIENT_PORT: u32 = 546u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCPV6_MESSAGE {
    pub MessageType: u8,
    pub TransactionIDByte1: u8,
    pub TransactionIDByte2: u8,
    pub TransactionIDByte3: u8,
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl Default for PXE_DHCPV6_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCPV6_MESSAGE_HEADER {
    pub MessageType: u8,
    pub Message: [u8; 1],
}
impl Default for PXE_DHCPV6_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    pub pRelayMessage: *mut PXE_DHCPV6_RELAY_MESSAGE,
    pub cbRelayMessage: u32,
    pub pInterfaceIdOption: *mut core::ffi::c_void,
    pub cbInterfaceIdOption: u16,
}
impl Default for PXE_DHCPV6_NESTED_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCPV6_OPTION {
    pub OptionCode: u16,
    pub DataLength: u16,
    pub Data: [u8; 1],
}
impl Default for PXE_DHCPV6_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT: u32 = 32u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCPV6_RELAY_MESSAGE {
    pub MessageType: u8,
    pub HopCount: u8,
    pub LinkAddress: [u8; 16],
    pub PeerAddress: [u8; 16],
    pub Options: [PXE_DHCPV6_OPTION; 1],
}
impl Default for PXE_DHCPV6_RELAY_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PXE_DHCPV6_SERVER_PORT: u32 = 547u32;
pub const PXE_DHCP_CLIENT_PORT: u32 = 68u32;
pub const PXE_DHCP_FILE_SIZE: u32 = 128u32;
pub const PXE_DHCP_HWAADR_SIZE: u32 = 16u32;
pub const PXE_DHCP_MAGIC_COOKIE_SIZE: u32 = 4u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCP_MESSAGE {
    pub Operation: u8,
    pub HardwareAddressType: u8,
    pub HardwareAddressLength: u8,
    pub HopCount: u8,
    pub TransactionID: u32,
    pub SecondsSinceBoot: u16,
    pub Reserved: u16,
    pub ClientIpAddress: u32,
    pub YourIpAddress: u32,
    pub BootstrapServerAddress: u32,
    pub RelayAgentIpAddress: u32,
    pub HardwareAddress: [u8; 16],
    pub HostName: [u8; 64],
    pub BootFileName: [u8; 128],
    pub Anonymous: PXE_DHCP_MESSAGE_0,
    pub Option: PXE_DHCP_OPTION,
}
impl Default for PXE_DHCP_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union PXE_DHCP_MESSAGE_0 {
    pub bMagicCookie: [u8; 4],
    pub uMagicCookie: u32,
}
impl Default for PXE_DHCP_MESSAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PXE_DHCP_OPTION {
    pub OptionType: u8,
    pub OptionLength: u8,
    pub OptionValue: [u8; 1],
}
impl Default for PXE_DHCP_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PXE_DHCP_SERVER_PORT: u32 = 67u32;
pub const PXE_DHCP_SERVER_SIZE: u32 = 64u32;
pub const PXE_GSI_SERVER_DUID: u32 = 2u32;
pub const PXE_GSI_TRACE_ENABLED: u32 = 1u32;
pub const PXE_MAX_ADDRESS: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PXE_PROVIDER {
    pub uSizeOfStruct: u32,
    pub pwszName: windows_core::PWSTR,
    pub pwszFilePath: windows_core::PWSTR,
    pub bIsCritical: windows_core::BOOL,
    pub uIndex: u32,
}
pub const PXE_PROV_ATTR_FILTER: u32 = 0u32;
pub const PXE_PROV_ATTR_FILTER_IPV6: u32 = 1u32;
pub const PXE_PROV_ATTR_IPV6_CAPABLE: u32 = 2u32;
pub const PXE_PROV_FILTER_ALL: u32 = 0u32;
pub const PXE_PROV_FILTER_DHCP_ONLY: u32 = 1u32;
pub const PXE_PROV_FILTER_PXE_ONLY: u32 = 2u32;
pub const PXE_REG_INDEX_BOTTOM: u32 = 4294967295u32;
pub const PXE_REG_INDEX_TOP: u32 = 0u32;
pub const PXE_SERVER_PORT: u32 = 4011u32;
pub const PXE_TRACE_ERROR: u32 = 524288u32;
pub const PXE_TRACE_FATAL: u32 = 1048576u32;
pub const PXE_TRACE_INFO: u32 = 131072u32;
pub const PXE_TRACE_VERBOSE: u32 = 65536u32;
pub const PXE_TRACE_WARNING: u32 = 262144u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORTCLIENT_CALLBACK_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSPORTCLIENT_SESSION_INFO {
    pub ulStructureLength: u32,
    pub ullFileSize: u64,
    pub ulBlockSize: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORTPROVIDER_CALLBACK_ID(pub i32);
pub const TRANSPORTPROVIDER_CURRENT_VERSION: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_ABORT: u32 = 5u32;
pub const WDSBP_OPTVAL_ACTION_APPROVAL: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_REFERRAL: u32 = 3u32;
pub const WDSBP_OPTVAL_NBP_VER_7: u32 = 1792u32;
pub const WDSBP_OPTVAL_NBP_VER_8: u32 = 2048u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT: u32 = 2u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTIN: u32 = 1u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTOUT: u32 = 3u32;
pub const WDSBP_OPT_TYPE_BYTE: u32 = 1u32;
pub const WDSBP_OPT_TYPE_IP4: u32 = 6u32;
pub const WDSBP_OPT_TYPE_IP6: u32 = 7u32;
pub const WDSBP_OPT_TYPE_NONE: u32 = 0u32;
pub const WDSBP_OPT_TYPE_STR: u32 = 5u32;
pub const WDSBP_OPT_TYPE_ULONG: u32 = 3u32;
pub const WDSBP_OPT_TYPE_USHORT: u32 = 2u32;
pub const WDSBP_OPT_TYPE_WSTR: u32 = 4u32;
pub const WDSBP_PK_TYPE_BCD: u32 = 4u32;
pub const WDSBP_PK_TYPE_DHCP: u32 = 1u32;
pub const WDSBP_PK_TYPE_DHCPV6: u32 = 8u32;
pub const WDSBP_PK_TYPE_WDSNBP: u32 = 2u32;
pub const WDSMCCLIENT_CATEGORY: windows_core::HRESULT = windows_core::HRESULT(0x2_u32 as _);
pub const WDSMCSERVER_CATEGORY: windows_core::HRESULT = windows_core::HRESULT(0x1_u32 as _);
pub const WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC1210110_u32 as _);
pub const WDSMCS_E_CLIENT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC1210104_u32 as _);
pub const WDSMCS_E_CONTENT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC1210103_u32 as _);
pub const WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC1210106_u32 as _);
pub const WDSMCS_E_INCOMPATIBLE_VERSION: windows_core::HRESULT = windows_core::HRESULT(0xC1210102_u32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC1210107_u32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_STARTED: windows_core::HRESULT = windows_core::HRESULT(0xC1210109_u32 as _);
pub const WDSMCS_E_NAMESPACE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC1210105_u32 as _);
pub const WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1210108_u32 as _);
pub const WDSMCS_E_NS_START_FAILED_NO_CLIENTS: windows_core::HRESULT = windows_core::HRESULT(0xC121010A_u32 as _);
pub const WDSMCS_E_PACKET_HAS_SECURITY: windows_core::HRESULT = windows_core::HRESULT(0xC121010E_u32 as _);
pub const WDSMCS_E_PACKET_NOT_CHECKSUMED: windows_core::HRESULT = windows_core::HRESULT(0xC121010F_u32 as _);
pub const WDSMCS_E_PACKET_NOT_HASHED: windows_core::HRESULT = windows_core::HRESULT(0xC121010C_u32 as _);
pub const WDSMCS_E_PACKET_NOT_SIGNED: windows_core::HRESULT = windows_core::HRESULT(0xC121010D_u32 as _);
pub const WDSMCS_E_REQCALLBACKS_NOT_REG: windows_core::HRESULT = windows_core::HRESULT(0xC1210101_u32 as _);
pub const WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1210100_u32 as _);
pub const WDSMCS_E_START_TIME_IN_PAST: windows_core::HRESULT = windows_core::HRESULT(0xC121010B_u32 as _);
pub const WDSTPC_E_ALREADY_COMPLETED: windows_core::HRESULT = windows_core::HRESULT(0xC1220301_u32 as _);
pub const WDSTPC_E_ALREADY_IN_LOWEST_SESSION: windows_core::HRESULT = windows_core::HRESULT(0xC122030A_u32 as _);
pub const WDSTPC_E_ALREADY_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1220302_u32 as _);
pub const WDSTPC_E_CALLBACKS_NOT_REG: windows_core::HRESULT = windows_core::HRESULT(0xC1220300_u32 as _);
pub const WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC122030B_u32 as _);
pub const WDSTPC_E_KICKED_FAIL: windows_core::HRESULT = windows_core::HRESULT(0xC1220307_u32 as _);
pub const WDSTPC_E_KICKED_FALLBACK: windows_core::HRESULT = windows_core::HRESULT(0xC1220306_u32 as _);
pub const WDSTPC_E_KICKED_POLICY_NOT_MET: windows_core::HRESULT = windows_core::HRESULT(0xC1220305_u32 as _);
pub const WDSTPC_E_KICKED_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0xC1220308_u32 as _);
pub const WDSTPC_E_MULTISTREAM_NOT_ENABLED: windows_core::HRESULT = windows_core::HRESULT(0xC1220309_u32 as _);
pub const WDSTPC_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0xC1220304_u32 as _);
pub const WDSTPC_E_NO_IP4_INTERFACE: windows_core::HRESULT = windows_core::HRESULT(0xC122030C_u32 as _);
pub const WDSTPC_E_UNKNOWN_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC1220303_u32 as _);
pub const WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE: windows_core::HRESULT = windows_core::HRESULT(0xC122030D_u32 as _);
pub const WDSTPTMGMT_CATEGORY: windows_core::HRESULT = windows_core::HRESULT(0x1_u32 as _);
pub const WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0xC110010F_u32 as _);
pub const WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0xC1100109_u32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC1100103_u32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC1100104_u32 as _);
pub const WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD: windows_core::HRESULT = windows_core::HRESULT(0xC110011C_u32 as _);
pub const WDSTPTMGMT_E_INVALID_CLASS: windows_core::HRESULT = windows_core::HRESULT(0xC1100102_u32 as _);
pub const WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC1100105_u32 as _);
pub const WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS: windows_core::HRESULT = windows_core::HRESULT(0xC110010E_u32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1100117_u32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1100118_u32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE: windows_core::HRESULT = windows_core::HRESULT(0xC110011A_u32 as _);
pub const WDSTPTMGMT_E_INVALID_IP_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0xC1100116_u32 as _);
pub const WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT: windows_core::HRESULT = windows_core::HRESULT(0xC110011B_u32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC110010B_u32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_NAME: windows_core::HRESULT = windows_core::HRESULT(0xC110010A_u32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS: windows_core::HRESULT = windows_core::HRESULT(0xC1100112_u32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME: windows_core::HRESULT = windows_core::HRESULT(0xC110010D_u32 as _);
pub const WDSTPTMGMT_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0xC1100101_u32 as _);
pub const WDSTPTMGMT_E_INVALID_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0xC1100100_u32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE: windows_core::HRESULT = windows_core::HRESULT(0xC1100110_u32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE: windows_core::HRESULT = windows_core::HRESULT(0xC1100111_u32 as _);
pub const WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC110011E_u32 as _);
pub const WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE: windows_core::HRESULT = windows_core::HRESULT(0xC1100123_u32 as _);
pub const WDSTPTMGMT_E_IPV6_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC1100119_u32 as _);
pub const WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC110011D_u32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC1100107_u32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER: windows_core::HRESULT = windows_core::HRESULT(0xC1100114_u32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0xC1100108_u32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_READ_ONLY: windows_core::HRESULT = windows_core::HRESULT(0xC110010C_u32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER: windows_core::HRESULT = windows_core::HRESULT(0xC1100115_u32 as _);
pub const WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC110011F_u32 as _);
pub const WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC1100121_u32 as _);
pub const WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC1100122_u32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0xC1100106_u32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC1100113_u32 as _);
pub const WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC1100120_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_DISCONNECT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_FEATURE_FLAGS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_IP_ADDRESS_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_NAMESPACE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_NETWORK_PROFILE_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_PROTOCOL_FLAGS(pub i32);
pub const WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_SERVICE_NOTIFICATION(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_TFTP_CAPABILITY(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDSTRANSPORT_UDP_PORT_POLICY(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WDS_CLI_CRED {
    pub pwszUserName: windows_core::PCWSTR,
    pub pwszDomain: windows_core::PCWSTR,
    pub pwszPassword: windows_core::PCWSTR,
}
pub const WDS_CLI_FIRMWARE_BIOS: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(1i32);
pub const WDS_CLI_FIRMWARE_EFI: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDS_CLI_FIRMWARE_TYPE(pub i32);
pub const WDS_CLI_FIRMWARE_UNKNOWN: WDS_CLI_FIRMWARE_TYPE = WDS_CLI_FIRMWARE_TYPE(0i32);
pub const WDS_CLI_IMAGE_PARAM_SPARSE_FILE: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(1i32);
pub const WDS_CLI_IMAGE_PARAM_SUPPORTED_FIRMWARES: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDS_CLI_IMAGE_PARAM_TYPE(pub i32);
pub const WDS_CLI_IMAGE_PARAM_UNKNOWN: WDS_CLI_IMAGE_PARAM_TYPE = WDS_CLI_IMAGE_PARAM_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDS_CLI_IMAGE_TYPE(pub i32);
pub const WDS_CLI_IMAGE_TYPE_UNKNOWN: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(0i32);
pub const WDS_CLI_IMAGE_TYPE_VHD: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(2i32);
pub const WDS_CLI_IMAGE_TYPE_VHDX: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(3i32);
pub const WDS_CLI_IMAGE_TYPE_WIM: WDS_CLI_IMAGE_TYPE = WDS_CLI_IMAGE_TYPE(1i32);
pub const WDS_CLI_MSG_COMPLETE: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(1u32);
pub const WDS_CLI_MSG_PROGRESS: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(2u32);
pub const WDS_CLI_MSG_START: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(0u32);
pub const WDS_CLI_MSG_TEXT: PFN_WDS_CLI_CALLBACK_MESSAGE_ID = PFN_WDS_CLI_CALLBACK_MESSAGE_ID(3u32);
pub const WDS_CLI_NO_SPARSE_FILE: u32 = 2u32;
pub const WDS_CLI_TRANSFER_ASYNCHRONOUS: u32 = 1u32;
pub const WDS_LOG_LEVEL_DISABLED: i32 = 0i32;
pub const WDS_LOG_LEVEL_ERROR: i32 = 1i32;
pub const WDS_LOG_LEVEL_INFO: i32 = 3i32;
pub const WDS_LOG_LEVEL_WARNING: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED: i32 = 6i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2: i32 = 16i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED: i32 = 5i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2: i32 = 15i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR: i32 = 12i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2: i32 = 17i32;
pub const WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE: i32 = 18i32;
pub const WDS_LOG_TYPE_CLIENT_ERROR: i32 = 1i32;
pub const WDS_LOG_TYPE_CLIENT_FINISHED: i32 = 3i32;
pub const WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE: i32 = 7i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED: i32 = 4i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2: i32 = 22i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3: i32 = 23i32;
pub const WDS_LOG_TYPE_CLIENT_MAX_CODE: i32 = 24i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END: i32 = 20i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE: i32 = 21i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START: i32 = 19i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END: i32 = 14i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START: i32 = 13i32;
pub const WDS_LOG_TYPE_CLIENT_STARTED: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE: i32 = 11i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_END: i32 = 10i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_START: i32 = 9i32;
pub const WDS_LOG_TYPE_CLIENT_UNATTEND_MODE: i32 = 8i32;
pub const WDS_MC_TRACE_ERROR: u32 = 524288u32;
pub const WDS_MC_TRACE_FATAL: u32 = 1048576u32;
pub const WDS_MC_TRACE_INFO: u32 = 131072u32;
pub const WDS_MC_TRACE_VERBOSE: u32 = 65536u32;
pub const WDS_MC_TRACE_WARNING: u32 = 262144u32;
pub const WDS_TRANSPORTCLIENT_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(1u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WDS_TRANSPORTCLIENT_CALLBACKS {
    pub SessionStart: PFN_WdsTransportClientSessionStart,
    pub SessionStartEx: PFN_WdsTransportClientSessionStartEx,
    pub ReceiveContents: PFN_WdsTransportClientReceiveContents,
    pub ReceiveMetadata: PFN_WdsTransportClientReceiveMetadata,
    pub SessionComplete: PFN_WdsTransportClientSessionComplete,
    pub SessionNegotiate: PFN_WdsTransportClientSessionNegotiate,
}
pub const WDS_TRANSPORTCLIENT_CURRENT_API_VERSION: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_MAX_CALLBACKS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(6i32);
pub const WDS_TRANSPORTCLIENT_NO_AUTH: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL = WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(2u32);
pub const WDS_TRANSPORTCLIENT_NO_CACHE: u32 = 0u32;
pub const WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_RECEIVE_CONTENTS: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTCLIENT_RECEIVE_METADATA: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WDS_TRANSPORTCLIENT_REQUEST {
    pub ulLength: u32,
    pub ulApiVersion: u32,
    pub ulAuthLevel: WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL,
    pub pwszServer: windows_core::PCWSTR,
    pub pwszNamespace: windows_core::PCWSTR,
    pub pwszObjectName: windows_core::PCWSTR,
    pub ulCacheSize: u32,
    pub ulProtocol: u32,
    pub pvProtocolData: *mut core::ffi::c_void,
    pub ulProtocolDataLength: u32,
}
impl Default for WDS_TRANSPORTCLIENT_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(pub u32);
pub const WDS_TRANSPORTCLIENT_SESSION_COMPLETE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTCLIENT_SESSION_NEGOTIATE: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTCLIENT_SESSION_START: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTCLIENT_SESSION_STARTEX: TRANSPORTCLIENT_CALLBACK_ID = TRANSPORTCLIENT_CALLBACK_ID(4i32);
pub const WDS_TRANSPORTCLIENT_STATUS_FAILURE: u32 = 3u32;
pub const WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_STATUS_SUCCESS: u32 = 2u32;
pub const WDS_TRANSPORTPROVIDER_CLOSE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(6i32);
pub const WDS_TRANSPORTPROVIDER_CLOSE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(7i32);
pub const WDS_TRANSPORTPROVIDER_COMPARE_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(1i32);
pub const WDS_TRANSPORTPROVIDER_CREATE_INSTANCE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(0i32);
pub const WDS_TRANSPORTPROVIDER_DUMP_STATE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(9i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_METADATA: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(11i32);
pub const WDS_TRANSPORTPROVIDER_GET_CONTENT_SIZE: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(4i32);
#[repr(C)]
#[cfg(feature = "Win32_System_Registry")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WDS_TRANSPORTPROVIDER_INIT_PARAMS {
    pub ulLength: u32,
    pub ulMcServerVersion: u32,
    pub hRegistryKey: super::Registry::HKEY,
    pub hProvider: super::super::Foundation::HANDLE,
}
pub const WDS_TRANSPORTPROVIDER_MAX_CALLBACKS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(12i32);
pub const WDS_TRANSPORTPROVIDER_OPEN_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(2i32);
pub const WDS_TRANSPORTPROVIDER_READ_CONTENT: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(5i32);
pub const WDS_TRANSPORTPROVIDER_REFRESH_SETTINGS: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(10i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WDS_TRANSPORTPROVIDER_SETTINGS {
    pub ulLength: u32,
    pub ulProviderVersion: u32,
}
pub const WDS_TRANSPORTPROVIDER_SHUTDOWN: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(8i32);
pub const WDS_TRANSPORTPROVIDER_USER_ACCESS_CHECK: TRANSPORTPROVIDER_CALLBACK_ID = TRANSPORTPROVIDER_CALLBACK_ID(3i32);
pub const WdsCliFlagEnumFilterFirmware: i32 = 2i32;
pub const WdsCliFlagEnumFilterVersion: i32 = 1i32;
pub const WdsTptDiagnosticsComponentImageServer: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(4i32);
pub const WdsTptDiagnosticsComponentMulticast: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(8i32);
pub const WdsTptDiagnosticsComponentPxe: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(1i32);
pub const WdsTptDiagnosticsComponentTftp: WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS = WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(2i32);
pub const WdsTptDisconnectAbort: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(2i32);
pub const WdsTptDisconnectFallback: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(1i32);
pub const WdsTptDisconnectUnknown: WDSTRANSPORT_DISCONNECT_TYPE = WDSTRANSPORT_DISCONNECT_TYPE(0i32);
pub const WdsTptFeatureAdminPack: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(1i32);
pub const WdsTptFeatureDeploymentServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(4i32);
pub const WdsTptFeatureTransportServer: WDSTRANSPORT_FEATURE_FLAGS = WDSTRANSPORT_FEATURE_FLAGS(2i32);
pub const WdsTptIpAddressIpv4: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(1i32);
pub const WdsTptIpAddressIpv6: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(2i32);
pub const WdsTptIpAddressSourceDhcp: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(1i32);
pub const WdsTptIpAddressSourceRange: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(2i32);
pub const WdsTptIpAddressSourceUnknown: WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE = WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(0i32);
pub const WdsTptIpAddressUnknown: WDSTRANSPORT_IP_ADDRESS_TYPE = WDSTRANSPORT_IP_ADDRESS_TYPE(0i32);
pub const WdsTptNamespaceTypeAutoCast: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(1i32);
pub const WdsTptNamespaceTypeScheduledCastAutoStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(3i32);
pub const WdsTptNamespaceTypeScheduledCastManualStart: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(2i32);
pub const WdsTptNamespaceTypeUnknown: WDSTRANSPORT_NAMESPACE_TYPE = WDSTRANSPORT_NAMESPACE_TYPE(0i32);
pub const WdsTptNetworkProfile100Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(3i32);
pub const WdsTptNetworkProfile10Mbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(2i32);
pub const WdsTptNetworkProfile1Gbps: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(4i32);
pub const WdsTptNetworkProfileCustom: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(1i32);
pub const WdsTptNetworkProfileUnknown: WDSTRANSPORT_NETWORK_PROFILE_TYPE = WDSTRANSPORT_NETWORK_PROFILE_TYPE(0i32);
pub const WdsTptProtocolMulticast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(2i32);
pub const WdsTptProtocolUnicast: WDSTRANSPORT_PROTOCOL_FLAGS = WDSTRANSPORT_PROTOCOL_FLAGS(1i32);
pub const WdsTptServiceNotifyReadSettings: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(1i32);
pub const WdsTptServiceNotifyUnknown: WDSTRANSPORT_SERVICE_NOTIFICATION = WDSTRANSPORT_SERVICE_NOTIFICATION(0i32);
pub const WdsTptSlowClientHandlingAutoDisconnect: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(2i32);
pub const WdsTptSlowClientHandlingMultistream: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(3i32);
pub const WdsTptSlowClientHandlingNone: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(1i32);
pub const WdsTptSlowClientHandlingUnknown: WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE = WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(0i32);
pub const WdsTptTftpCapMaximumBlockSize: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(1i32);
pub const WdsTptTftpCapVariableWindow: WDSTRANSPORT_TFTP_CAPABILITY = WDSTRANSPORT_TFTP_CAPABILITY(2i32);
pub const WdsTptUdpPortPolicyDynamic: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(0i32);
pub const WdsTptUdpPortPolicyFixed: WDSTRANSPORT_UDP_PORT_POLICY = WDSTRANSPORT_UDP_PORT_POLICY(1i32);
pub const WdsTransportCacheable: windows_core::GUID = windows_core::GUID::from_u128(0x70590b16_f146_46bd_bd9d_4aaa90084bf5);
pub const WdsTransportClient: windows_core::GUID = windows_core::GUID::from_u128(0x66d2c5e9_0ff6_49ec_9733_dafb1e01df1c);
pub const WdsTransportCollection: windows_core::GUID = windows_core::GUID::from_u128(0xc7f18b09_391e_436e_b10b_c3ef46f2c34f);
pub const WdsTransportConfigurationManager: windows_core::GUID = windows_core::GUID::from_u128(0x8743f674_904c_47ca_8512_35fe98f6b0ac);
pub const WdsTransportContent: windows_core::GUID = windows_core::GUID::from_u128(0x0a891fe7_4a3f_4c65_b6f2_1467619679ea);
pub const WdsTransportContentProvider: windows_core::GUID = windows_core::GUID::from_u128(0xe0be741f_5a75_4eb9_8a2d_5e189b45f327);
pub const WdsTransportDiagnosticsPolicy: windows_core::GUID = windows_core::GUID::from_u128(0xeb3333e1_a7ad_46f5_80d6_6b740204e509);
pub const WdsTransportManager: windows_core::GUID = windows_core::GUID::from_u128(0xf21523f6_837c_4a58_af99_8a7e27f8ff59);
pub const WdsTransportMulticastSessionPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x3c6bc3f4_6418_472a_b6f1_52d457195437);
pub const WdsTransportNamespace: windows_core::GUID = windows_core::GUID::from_u128(0xd8385768_0732_4ec1_95ea_16da581908a1);
pub const WdsTransportNamespaceAutoCast: windows_core::GUID = windows_core::GUID::from_u128(0xb091f5a8_6a99_478d_b23b_09e8fee04574);
pub const WdsTransportNamespaceManager: windows_core::GUID = windows_core::GUID::from_u128(0xf08cdb63_85de_4a28_a1a9_5ca3e7efda73);
pub const WdsTransportNamespaceScheduledCast: windows_core::GUID = windows_core::GUID::from_u128(0xbadc1897_7025_44eb_9108_fb61c4055792);
pub const WdsTransportNamespaceScheduledCastAutoStart: windows_core::GUID = windows_core::GUID::from_u128(0xa1107052_122c_4b81_9b7c_386e6855383f);
pub const WdsTransportNamespaceScheduledCastManualStart: windows_core::GUID = windows_core::GUID::from_u128(0xd3e1a2aa_caac_460e_b98a_47f9f318a1fa);
pub const WdsTransportServer: windows_core::GUID = windows_core::GUID::from_u128(0xea19b643_4adf_4413_942c_14f379118760);
pub const WdsTransportServicePolicy: windows_core::GUID = windows_core::GUID::from_u128(0x65aceadc_2f0b_4f43_9f4d_811865d8cead);
pub const WdsTransportSession: windows_core::GUID = windows_core::GUID::from_u128(0x749ac4e0_67bc_4743_bfe5_cacb1f26f57f);
pub const WdsTransportSetupManager: windows_core::GUID = windows_core::GUID::from_u128(0xc7beeaad_9f04_4923_9f0c_fbf52bc7590f);
pub const WdsTransportTftpClient: windows_core::GUID = windows_core::GUID::from_u128(0x50343925_7c5c_4c8c_96c4_ad9fa5005fba);
pub const WdsTransportTftpManager: windows_core::GUID = windows_core::GUID::from_u128(0xc8e9dca2_3241_4e4d_b806_bc74019dfeda);
