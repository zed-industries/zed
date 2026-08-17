#[inline]
pub unsafe fn DrtClose(hdrt: *const core::ffi::c_void) {
    windows_targets::link!("drt.dll" "system" fn DrtClose(hdrt : *const core::ffi::c_void));
    DrtClose(hdrt)
}
#[inline]
pub unsafe fn DrtContinueSearch(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtContinueSearch(hsearchcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DrtContinueSearch(hsearchcontext).ok()
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn DrtCreateDerivedKey(plocalcert: *const super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::Result<DRT_DATA> {
    windows_targets::link!("drtprov.dll" "system" fn DrtCreateDerivedKey(plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, pkey : *mut DRT_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtCreateDerivedKey(plocalcert, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn DrtCreateDerivedKeySecurityProvider(prootcert: *const super::super::Security::Cryptography::CERT_CONTEXT, plocalcert: Option<*const super::super::Security::Cryptography::CERT_CONTEXT>) -> windows_core::Result<*mut DRT_SECURITY_PROVIDER> {
    windows_targets::link!("drtprov.dll" "system" fn DrtCreateDerivedKeySecurityProvider(prootcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, plocalcert : *const super::super::Security::Cryptography:: CERT_CONTEXT, ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtCreateDerivedKeySecurityProvider(prootcert, core::mem::transmute(plocalcert.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtCreateDnsBootstrapResolver<P0>(port: u16, pwszaddress: P0) -> windows_core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("drtprov.dll" "system" fn DrtCreateDnsBootstrapResolver(port : u16, pwszaddress : windows_core::PCWSTR, ppmodule : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtCreateDnsBootstrapResolver(port, pwszaddress.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtCreateIpv6UdpTransport(scope: DRT_SCOPE, dwscopeid: u32, dwlocalitythreshold: u32, pwport: *mut u16, phtransport: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("drttransport.dll" "system" fn DrtCreateIpv6UdpTransport(scope : DRT_SCOPE, dwscopeid : u32, dwlocalitythreshold : u32, pwport : *mut u16, phtransport : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DrtCreateIpv6UdpTransport(scope, dwscopeid, dwlocalitythreshold, pwport, phtransport).ok()
}
#[inline]
pub unsafe fn DrtCreateNullSecurityProvider() -> windows_core::Result<*mut DRT_SECURITY_PROVIDER> {
    windows_targets::link!("drtprov.dll" "system" fn DrtCreateNullSecurityProvider(ppsecurityprovider : *mut *mut DRT_SECURITY_PROVIDER) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtCreateNullSecurityProvider(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtCreatePnrpBootstrapResolver<P0, P1, P2, P3>(fpublish: P0, pwzpeername: P1, pwzcloudname: P2, pwzpublishingidentity: P3) -> windows_core::Result<*mut DRT_BOOTSTRAP_PROVIDER>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("drtprov.dll" "system" fn DrtCreatePnrpBootstrapResolver(fpublish : super::super::Foundation:: BOOL, pwzpeername : windows_core::PCWSTR, pwzcloudname : windows_core::PCWSTR, pwzpublishingidentity : windows_core::PCWSTR, ppresolver : *mut *mut DRT_BOOTSTRAP_PROVIDER) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtCreatePnrpBootstrapResolver(fpublish.param().abi(), pwzpeername.param().abi(), pwzcloudname.param().abi(), pwzpublishingidentity.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    windows_targets::link!("drtprov.dll" "system" fn DrtDeleteDerivedKeySecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
    DrtDeleteDerivedKeySecurityProvider(psecurityprovider)
}
#[inline]
pub unsafe fn DrtDeleteDnsBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    windows_targets::link!("drtprov.dll" "system" fn DrtDeleteDnsBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
    DrtDeleteDnsBootstrapResolver(presolver)
}
#[inline]
pub unsafe fn DrtDeleteIpv6UdpTransport(htransport: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("drttransport.dll" "system" fn DrtDeleteIpv6UdpTransport(htransport : *const core::ffi::c_void) -> windows_core::HRESULT);
    DrtDeleteIpv6UdpTransport(htransport).ok()
}
#[inline]
pub unsafe fn DrtDeleteNullSecurityProvider(psecurityprovider: *const DRT_SECURITY_PROVIDER) {
    windows_targets::link!("drtprov.dll" "system" fn DrtDeleteNullSecurityProvider(psecurityprovider : *const DRT_SECURITY_PROVIDER));
    DrtDeleteNullSecurityProvider(psecurityprovider)
}
#[inline]
pub unsafe fn DrtDeletePnrpBootstrapResolver(presolver: *const DRT_BOOTSTRAP_PROVIDER) {
    windows_targets::link!("drtprov.dll" "system" fn DrtDeletePnrpBootstrapResolver(presolver : *const DRT_BOOTSTRAP_PROVIDER));
    DrtDeletePnrpBootstrapResolver(presolver)
}
#[inline]
pub unsafe fn DrtEndSearch(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtEndSearch(hsearchcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DrtEndSearch(hsearchcontext).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetEventData(hdrt: *const core::ffi::c_void, uleventdatalen: u32, peventdata: *mut DRT_EVENT_DATA) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtGetEventData(hdrt : *const core::ffi::c_void, uleventdatalen : u32, peventdata : *mut DRT_EVENT_DATA) -> windows_core::HRESULT);
    DrtGetEventData(hdrt, uleventdatalen, peventdata).ok()
}
#[inline]
pub unsafe fn DrtGetEventDataSize(hdrt: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("drt.dll" "system" fn DrtGetEventDataSize(hdrt : *const core::ffi::c_void, puleventdatalen : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtGetEventDataSize(hdrt, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtGetInstanceName(hdrt: *const core::ffi::c_void, ulcbinstancenamesize: u32, pwzdrtinstancename: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtGetInstanceName(hdrt : *const core::ffi::c_void, ulcbinstancenamesize : u32, pwzdrtinstancename : windows_core::PWSTR) -> windows_core::HRESULT);
    DrtGetInstanceName(hdrt, ulcbinstancenamesize, core::mem::transmute(pwzdrtinstancename)).ok()
}
#[inline]
pub unsafe fn DrtGetInstanceNameSize(hdrt: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("drt.dll" "system" fn DrtGetInstanceNameSize(hdrt : *const core::ffi::c_void, pulcbinstancenamesize : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtGetInstanceNameSize(hdrt, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DrtGetSearchPath(hsearchcontext: *const core::ffi::c_void, ulsearchpathsize: u32, psearchpath: *mut DRT_ADDRESS_LIST) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtGetSearchPath(hsearchcontext : *const core::ffi::c_void, ulsearchpathsize : u32, psearchpath : *mut DRT_ADDRESS_LIST) -> windows_core::HRESULT);
    DrtGetSearchPath(hsearchcontext, ulsearchpathsize, psearchpath).ok()
}
#[inline]
pub unsafe fn DrtGetSearchPathSize(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("drt.dll" "system" fn DrtGetSearchPathSize(hsearchcontext : *const core::ffi::c_void, pulsearchpathsize : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtGetSearchPathSize(hsearchcontext, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtGetSearchResult(hsearchcontext: *const core::ffi::c_void, ulsearchresultsize: u32, psearchresult: *mut DRT_SEARCH_RESULT) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtGetSearchResult(hsearchcontext : *const core::ffi::c_void, ulsearchresultsize : u32, psearchresult : *mut DRT_SEARCH_RESULT) -> windows_core::HRESULT);
    DrtGetSearchResult(hsearchcontext, ulsearchresultsize, psearchresult).ok()
}
#[inline]
pub unsafe fn DrtGetSearchResultSize(hsearchcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("drt.dll" "system" fn DrtGetSearchResultSize(hsearchcontext : *const core::ffi::c_void, pulsearchresultsize : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DrtGetSearchResultSize(hsearchcontext, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DrtOpen<P0>(psettings: *const DRT_SETTINGS, hevent: P0, pvcontext: Option<*const core::ffi::c_void>, phdrt: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("drt.dll" "system" fn DrtOpen(psettings : *const DRT_SETTINGS, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, phdrt : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DrtOpen(psettings, hevent.param().abi(), core::mem::transmute(pvcontext.unwrap_or(std::ptr::null())), phdrt).ok()
}
#[inline]
pub unsafe fn DrtRegisterKey(hdrt: *const core::ffi::c_void, pregistration: *const DRT_REGISTRATION, pvkeycontext: Option<*const core::ffi::c_void>, phkeyregistration: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtRegisterKey(hdrt : *const core::ffi::c_void, pregistration : *const DRT_REGISTRATION, pvkeycontext : *const core::ffi::c_void, phkeyregistration : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DrtRegisterKey(hdrt, pregistration, core::mem::transmute(pvkeycontext.unwrap_or(std::ptr::null())), phkeyregistration).ok()
}
#[inline]
pub unsafe fn DrtStartSearch<P0>(hdrt: *const core::ffi::c_void, pkey: *const DRT_DATA, pinfo: Option<*const DRT_SEARCH_INFO>, timeout: u32, hevent: P0, pvcontext: Option<*const core::ffi::c_void>, hsearchcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("drt.dll" "system" fn DrtStartSearch(hdrt : *const core::ffi::c_void, pkey : *const DRT_DATA, pinfo : *const DRT_SEARCH_INFO, timeout : u32, hevent : super::super::Foundation:: HANDLE, pvcontext : *const core::ffi::c_void, hsearchcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    DrtStartSearch(hdrt, pkey, core::mem::transmute(pinfo.unwrap_or(std::ptr::null())), timeout, hevent.param().abi(), core::mem::transmute(pvcontext.unwrap_or(std::ptr::null())), hsearchcontext).ok()
}
#[inline]
pub unsafe fn DrtUnregisterKey(hkeyregistration: *const core::ffi::c_void) {
    windows_targets::link!("drt.dll" "system" fn DrtUnregisterKey(hkeyregistration : *const core::ffi::c_void));
    DrtUnregisterKey(hkeyregistration)
}
#[inline]
pub unsafe fn DrtUpdateKey(hkeyregistration: *const core::ffi::c_void, pappdata: *const DRT_DATA) -> windows_core::Result<()> {
    windows_targets::link!("drt.dll" "system" fn DrtUpdateKey(hkeyregistration : *const core::ffi::c_void, pappdata : *const DRT_DATA) -> windows_core::HRESULT);
    DrtUpdateKey(hkeyregistration, pappdata).ok()
}
#[inline]
pub unsafe fn PeerCollabAddContact<P0>(pwzcontactdata: P0, ppcontact: Option<*mut *mut PEER_CONTACT>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabAddContact(pwzcontactdata : windows_core::PCWSTR, ppcontact : *mut *mut PEER_CONTACT) -> windows_core::HRESULT);
    PeerCollabAddContact(pwzcontactdata.param().abi(), core::mem::transmute(ppcontact.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabAsyncInviteContact<P0>(pccontact: Option<*const PEER_CONTACT>, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: P0, phinvitation: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabAsyncInviteContact(pccontact : *const PEER_CONTACT, pcendpoint : *const PEER_ENDPOINT, pcinvitation : *const PEER_INVITATION, hevent : super::super::Foundation:: HANDLE, phinvitation : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    PeerCollabAsyncInviteContact(core::mem::transmute(pccontact.unwrap_or(std::ptr::null())), pcendpoint, pcinvitation, hevent.param().abi(), core::mem::transmute(phinvitation.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabAsyncInviteEndpoint<P0>(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION, hevent: P0, phinvitation: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabAsyncInviteEndpoint(pcendpoint : *const PEER_ENDPOINT, pcinvitation : *const PEER_INVITATION, hevent : super::super::Foundation:: HANDLE, phinvitation : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    PeerCollabAsyncInviteEndpoint(pcendpoint, pcinvitation, hevent.param().abi(), core::mem::transmute(phinvitation.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn PeerCollabCancelInvitation<P0>(hinvitation: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabCancelInvitation(hinvitation : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    PeerCollabCancelInvitation(hinvitation.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerCollabCloseHandle<P0>(hinvitation: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabCloseHandle(hinvitation : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    PeerCollabCloseHandle(hinvitation.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerCollabDeleteContact<P0>(pwzpeername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabDeleteContact(pwzpeername : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerCollabDeleteContact(pwzpeername.param().abi()).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabDeleteEndpointData(pcendpoint: *const PEER_ENDPOINT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabDeleteEndpointData(pcendpoint : *const PEER_ENDPOINT) -> windows_core::HRESULT);
    PeerCollabDeleteEndpointData(pcendpoint).ok()
}
#[inline]
pub unsafe fn PeerCollabDeleteObject(pobjectid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabDeleteObject(pobjectid : *const windows_core::GUID) -> windows_core::HRESULT);
    PeerCollabDeleteObject(pobjectid).ok()
}
#[inline]
pub unsafe fn PeerCollabEnumApplicationRegistrationInfo(registrationtype: PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumApplicationRegistrationInfo(registrationtype : PEER_APPLICATION_REGISTRATION_TYPE, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumApplicationRegistrationInfo(registrationtype, phpeerenum).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabEnumApplications(pcendpoint: Option<*const PEER_ENDPOINT>, papplicationid: Option<*const windows_core::GUID>, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumApplications(pcendpoint : *const PEER_ENDPOINT, papplicationid : *const windows_core::GUID, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumApplications(core::mem::transmute(pcendpoint.unwrap_or(std::ptr::null())), core::mem::transmute(papplicationid.unwrap_or(std::ptr::null())), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerCollabEnumContacts(phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumContacts(phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumContacts(phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerCollabEnumEndpoints(pccontact: Option<*const PEER_CONTACT>, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumEndpoints(pccontact : *const PEER_CONTACT, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumEndpoints(core::mem::transmute(pccontact.unwrap_or(std::ptr::null())), phpeerenum).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabEnumObjects(pcendpoint: Option<*const PEER_ENDPOINT>, pobjectid: Option<*const windows_core::GUID>, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumObjects(pcendpoint : *const PEER_ENDPOINT, pobjectid : *const windows_core::GUID, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumObjects(core::mem::transmute(pcendpoint.unwrap_or(std::ptr::null())), core::mem::transmute(pobjectid.unwrap_or(std::ptr::null())), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerCollabEnumPeopleNearMe(phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabEnumPeopleNearMe(phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabEnumPeopleNearMe(phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerCollabExportContact<P0>(pwzpeername: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabExportContact(pwzpeername : windows_core::PCWSTR, ppwzcontactdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabExportContact(pwzpeername.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabGetAppLaunchInfo() -> windows_core::Result<*mut PEER_APP_LAUNCH_INFO> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetAppLaunchInfo(pplaunchinfo : *mut *mut PEER_APP_LAUNCH_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetAppLaunchInfo(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabGetApplicationRegistrationInfo(papplicationid: *const windows_core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> windows_core::Result<*mut PEER_APPLICATION_REGISTRATION_INFO> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetApplicationRegistrationInfo(papplicationid : *const windows_core::GUID, registrationtype : PEER_APPLICATION_REGISTRATION_TYPE, ppapplication : *mut *mut PEER_APPLICATION_REGISTRATION_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetApplicationRegistrationInfo(papplicationid, registrationtype, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabGetContact<P0>(pwzpeername: P0) -> windows_core::Result<*mut PEER_CONTACT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetContact(pwzpeername : windows_core::PCWSTR, ppcontact : *mut *mut PEER_CONTACT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetContact(pwzpeername.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabGetEndpointName() -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetEndpointName(ppwzendpointname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetEndpointName(&mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabGetEventData(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_COLLAB_EVENT_DATA> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetEventData(hpeerevent : *const core::ffi::c_void, ppeventdata : *mut *mut PEER_COLLAB_EVENT_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetEventData(hpeerevent, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabGetInvitationResponse<P0>(hinvitation: P0) -> windows_core::Result<*mut PEER_INVITATION_RESPONSE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetInvitationResponse(hinvitation : super::super::Foundation:: HANDLE, ppinvitationresponse : *mut *mut PEER_INVITATION_RESPONSE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetInvitationResponse(hinvitation.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabGetPresenceInfo(pcendpoint: Option<*const PEER_ENDPOINT>) -> windows_core::Result<*mut PEER_PRESENCE_INFO> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetPresenceInfo(pcendpoint : *const PEER_ENDPOINT, pppresenceinfo : *mut *mut PEER_PRESENCE_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetPresenceInfo(core::mem::transmute(pcendpoint.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabGetSigninOptions() -> windows_core::Result<u32> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabGetSigninOptions(pdwsigninoptions : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabGetSigninOptions(&mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabInviteContact(pccontact: Option<*const PEER_CONTACT>, pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> windows_core::Result<*mut PEER_INVITATION_RESPONSE> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabInviteContact(pccontact : *const PEER_CONTACT, pcendpoint : *const PEER_ENDPOINT, pcinvitation : *const PEER_INVITATION, ppresponse : *mut *mut PEER_INVITATION_RESPONSE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabInviteContact(core::mem::transmute(pccontact.unwrap_or(std::ptr::null())), pcendpoint, pcinvitation, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabInviteEndpoint(pcendpoint: *const PEER_ENDPOINT, pcinvitation: *const PEER_INVITATION) -> windows_core::Result<*mut PEER_INVITATION_RESPONSE> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabInviteEndpoint(pcendpoint : *const PEER_ENDPOINT, pcinvitation : *const PEER_INVITATION, ppresponse : *mut *mut PEER_INVITATION_RESPONSE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabInviteEndpoint(pcendpoint, pcinvitation, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerCollabParseContact<P0>(pwzcontactdata: P0) -> windows_core::Result<*mut PEER_CONTACT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabParseContact(pwzcontactdata : windows_core::PCWSTR, ppcontact : *mut *mut PEER_CONTACT) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabParseContact(pwzcontactdata.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabQueryContactData(pcendpoint: Option<*const PEER_ENDPOINT>) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabQueryContactData(pcendpoint : *const PEER_ENDPOINT, ppwzcontactdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCollabQueryContactData(core::mem::transmute(pcendpoint.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabRefreshEndpointData(pcendpoint: *const PEER_ENDPOINT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabRefreshEndpointData(pcendpoint : *const PEER_ENDPOINT) -> windows_core::HRESULT);
    PeerCollabRefreshEndpointData(pcendpoint).ok()
}
#[inline]
pub unsafe fn PeerCollabRegisterApplication(pcapplication: *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabRegisterApplication(pcapplication : *const PEER_APPLICATION_REGISTRATION_INFO, registrationtype : PEER_APPLICATION_REGISTRATION_TYPE) -> windows_core::HRESULT);
    PeerCollabRegisterApplication(pcapplication, registrationtype).ok()
}
#[inline]
pub unsafe fn PeerCollabRegisterEvent<P0>(hevent: P0, peventregistrations: &[PEER_COLLAB_EVENT_REGISTRATION], phpeerevent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabRegisterEvent(hevent : super::super::Foundation:: HANDLE, ceventregistration : u32, peventregistrations : *const PEER_COLLAB_EVENT_REGISTRATION, phpeerevent : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabRegisterEvent(hevent.param().abi(), peventregistrations.len().try_into().unwrap(), core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[inline]
pub unsafe fn PeerCollabSetEndpointName<P0>(pwzendpointname: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSetEndpointName(pwzendpointname : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerCollabSetEndpointName(pwzendpointname.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerCollabSetObject(pcobject: *const PEER_OBJECT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSetObject(pcobject : *const PEER_OBJECT) -> windows_core::HRESULT);
    PeerCollabSetObject(pcobject).ok()
}
#[inline]
pub unsafe fn PeerCollabSetPresenceInfo(pcpresenceinfo: *const PEER_PRESENCE_INFO) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSetPresenceInfo(pcpresenceinfo : *const PEER_PRESENCE_INFO) -> windows_core::HRESULT);
    PeerCollabSetPresenceInfo(pcpresenceinfo).ok()
}
#[inline]
pub unsafe fn PeerCollabShutdown() -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabShutdown() -> windows_core::HRESULT);
    PeerCollabShutdown().ok()
}
#[inline]
pub unsafe fn PeerCollabSignin<P0>(hwndparent: P0, dwsigninoptions: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSignin(hwndparent : super::super::Foundation:: HWND, dwsigninoptions : u32) -> windows_core::HRESULT);
    PeerCollabSignin(hwndparent.param().abi(), dwsigninoptions).ok()
}
#[inline]
pub unsafe fn PeerCollabSignout(dwsigninoptions: u32) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSignout(dwsigninoptions : u32) -> windows_core::HRESULT);
    PeerCollabSignout(dwsigninoptions).ok()
}
#[inline]
pub unsafe fn PeerCollabStartup(wversionrequested: u16) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabStartup(wversionrequested : u16) -> windows_core::HRESULT);
    PeerCollabStartup(wversionrequested).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabSubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabSubscribeEndpointData(pcendpoint : *const PEER_ENDPOINT) -> windows_core::HRESULT);
    PeerCollabSubscribeEndpointData(pcendpoint).ok()
}
#[inline]
pub unsafe fn PeerCollabUnregisterApplication(papplicationid: *const windows_core::GUID, registrationtype: PEER_APPLICATION_REGISTRATION_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabUnregisterApplication(papplicationid : *const windows_core::GUID, registrationtype : PEER_APPLICATION_REGISTRATION_TYPE) -> windows_core::HRESULT);
    PeerCollabUnregisterApplication(papplicationid, registrationtype).ok()
}
#[inline]
pub unsafe fn PeerCollabUnregisterEvent(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabUnregisterEvent(hpeerevent : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerCollabUnregisterEvent(hpeerevent).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerCollabUnsubscribeEndpointData(pcendpoint: *const PEER_ENDPOINT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabUnsubscribeEndpointData(pcendpoint : *const PEER_ENDPOINT) -> windows_core::HRESULT);
    PeerCollabUnsubscribeEndpointData(pcendpoint).ok()
}
#[inline]
pub unsafe fn PeerCollabUpdateContact(pcontact: *const PEER_CONTACT) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerCollabUpdateContact(pcontact : *const PEER_CONTACT) -> windows_core::HRESULT);
    PeerCollabUpdateContact(pcontact).ok()
}
#[inline]
pub unsafe fn PeerCreatePeerName<P0, P1>(pwzidentity: P0, pwzclassifier: P1) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerCreatePeerName(pwzidentity : windows_core::PCWSTR, pwzclassifier : windows_core::PCWSTR, ppwzpeername : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerCreatePeerName(pwzidentity.param().abi(), pwzclassifier.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientAddContentInformation(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientAddContentInformation(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientAddContentInformation(hpeerdist, hcontenthandle, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientAddData(hpeerdist: isize, hcontenthandle: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientAddData(hpeerdist : isize, hcontenthandle : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientAddData(hpeerdist, hcontenthandle, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientBlockRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientBlockRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientBlockRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientCancelAsyncOperation(hpeerdist: isize, hcontenthandle: isize, poverlapped: Option<*const super::super::System::IO::OVERLAPPED>) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCancelAsyncOperation(hpeerdist : isize, hcontenthandle : isize, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientCancelAsyncOperation(hpeerdist, hcontenthandle, core::mem::transmute(poverlapped.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn PeerDistClientCloseContent(hpeerdist: isize, hcontenthandle: isize) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCloseContent(hpeerdist : isize, hcontenthandle : isize) -> u32);
    PeerDistClientCloseContent(hpeerdist, hcontenthandle)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientCompleteContentInformation(hpeerdist: isize, hcontenthandle: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientCompleteContentInformation(hpeerdist : isize, hcontenthandle : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientCompleteContentInformation(hpeerdist, hcontenthandle, lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientFlushContent<P0>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientFlushContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientFlushContent(hpeerdist, pcontenttag, hcompletionport.param().abi(), ulcompletionkey, lpoverlapped)
}
#[inline]
pub unsafe fn PeerDistClientGetInformationByHandle(hpeerdist: isize, hcontenthandle: isize, peerdistclientinfoclass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize: u32, lpinformation: *mut core::ffi::c_void) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientGetInformationByHandle(hpeerdist : isize, hcontenthandle : isize, peerdistclientinfoclass : PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS, dwbuffersize : u32, lpinformation : *mut core::ffi::c_void) -> u32);
    PeerDistClientGetInformationByHandle(hpeerdist, hcontenthandle, peerdistclientinfoclass, dwbuffersize, lpinformation)
}
#[inline]
pub unsafe fn PeerDistClientOpenContent<P0>(hpeerdist: isize, pcontenttag: *const PEERDIST_CONTENT_TAG, hcompletionport: P0, ulcompletionkey: usize, phcontenthandle: *mut isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientOpenContent(hpeerdist : isize, pcontenttag : *const PEERDIST_CONTENT_TAG, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontenthandle : *mut isize) -> u32);
    PeerDistClientOpenContent(hpeerdist, pcontenttag, hcompletionport.param().abi(), ulcompletionkey, phcontenthandle)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistClientStreamRead(hpeerdist: isize, hcontenthandle: isize, pbuffer: Option<&mut [u8]>, dwtimeoutinmilliseconds: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistClientStreamRead(hpeerdist : isize, hcontenthandle : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, dwtimeoutinmilliseconds : u32, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistClientStreamRead(hpeerdist, hcontenthandle, pbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dwtimeoutinmilliseconds, lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistGetOverlappedResult<P0>(lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistGetOverlappedResult(lpoverlapped : *const super::super::System::IO:: OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    PeerDistGetOverlappedResult(lpoverlapped, lpnumberofbytestransferred, bwait.param().abi())
}
#[inline]
pub unsafe fn PeerDistGetStatus(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistGetStatus(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
    PeerDistGetStatus(hpeerdist, ppeerdiststatus)
}
#[inline]
pub unsafe fn PeerDistGetStatusEx(hpeerdist: isize, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistGetStatusEx(hpeerdist : isize, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
    PeerDistGetStatusEx(hpeerdist, ppeerdiststatus)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotification<P0>(hpeerdist: isize, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotification(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS) -> u32);
    PeerDistRegisterForStatusChangeNotification(hpeerdist, hcompletionport.param().abi(), ulcompletionkey, lpoverlapped, ppeerdiststatus)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistRegisterForStatusChangeNotificationEx<P0>(hpeerdist: isize, hcompletionport: P0, ulcompletionkey: usize, lpoverlapped: *const super::super::System::IO::OVERLAPPED, ppeerdiststatus: *mut PEERDIST_STATUS_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistRegisterForStatusChangeNotificationEx(hpeerdist : isize, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED, ppeerdiststatus : *mut PEERDIST_STATUS_INFO) -> u32);
    PeerDistRegisterForStatusChangeNotificationEx(hpeerdist, hcompletionport.param().abi(), ulcompletionkey, lpoverlapped, ppeerdiststatus)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerCancelAsyncOperation(hpeerdist: isize, pcontentidentifier: &[u8], poverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCancelAsyncOperation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, poverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistServerCancelAsyncOperation(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), poverlapped)
}
#[inline]
pub unsafe fn PeerDistServerCloseContentInformation(hpeerdist: isize, hcontentinfo: isize) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCloseContentInformation(hpeerdist : isize, hcontentinfo : isize) -> u32);
    PeerDistServerCloseContentInformation(hpeerdist, hcontentinfo)
}
#[inline]
pub unsafe fn PeerDistServerCloseStreamHandle(hpeerdist: isize, hstream: isize) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerCloseStreamHandle(hpeerdist : isize, hstream : isize) -> u32);
    PeerDistServerCloseStreamHandle(hpeerdist, hstream)
}
#[inline]
pub unsafe fn PeerDistServerOpenContentInformation<P0>(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, hcompletionport: P0, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformation(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
    PeerDistServerOpenContentInformation(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, hcompletionport.param().abi(), ulcompletionkey, phcontentinfo)
}
#[inline]
pub unsafe fn PeerDistServerOpenContentInformationEx<P0>(hpeerdist: isize, pcontentidentifier: &[u8], ullcontentoffset: u64, cbcontentlength: u64, pretrievaloptions: *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport: P0, ulcompletionkey: usize, phcontentinfo: *mut isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerOpenContentInformationEx(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, ullcontentoffset : u64, cbcontentlength : u64, pretrievaloptions : *const PEERDIST_RETRIEVAL_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phcontentinfo : *mut isize) -> u32);
    PeerDistServerOpenContentInformationEx(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), ullcontentoffset, cbcontentlength, pretrievaloptions, hcompletionport.param().abi(), ulcompletionkey, phcontentinfo)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerPublishAddToStream(hpeerdist: isize, hstream: isize, pbuffer: &[u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishAddToStream(hpeerdist : isize, hstream : isize, cbnumberofbytes : u32, pbuffer : *const u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistServerPublishAddToStream(hpeerdist, hstream, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerPublishCompleteStream(hpeerdist: isize, hstream: isize, lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishCompleteStream(hpeerdist : isize, hstream : isize, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistServerPublishCompleteStream(hpeerdist, hstream, lpoverlapped)
}
#[inline]
pub unsafe fn PeerDistServerPublishStream<P0>(hpeerdist: isize, pcontentidentifier: &[u8], cbcontentlength: u64, ppublishoptions: Option<*const PEERDIST_PUBLICATION_OPTIONS>, hcompletionport: P0, ulcompletionkey: usize, phstream: *mut isize) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerPublishStream(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8, cbcontentlength : u64, ppublishoptions : *const PEERDIST_PUBLICATION_OPTIONS, hcompletionport : super::super::Foundation:: HANDLE, ulcompletionkey : usize, phstream : *mut isize) -> u32);
    PeerDistServerPublishStream(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()), cbcontentlength, core::mem::transmute(ppublishoptions.unwrap_or(std::ptr::null())), hcompletionport.param().abi(), ulcompletionkey, phstream)
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn PeerDistServerRetrieveContentInformation(hpeerdist: isize, hcontentinfo: isize, pbuffer: &mut [u8], lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerRetrieveContentInformation(hpeerdist : isize, hcontentinfo : isize, cbmaxnumberofbytes : u32, pbuffer : *mut u8, lpoverlapped : *const super::super::System::IO:: OVERLAPPED) -> u32);
    PeerDistServerRetrieveContentInformation(hpeerdist, hcontentinfo, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), lpoverlapped)
}
#[inline]
pub unsafe fn PeerDistServerUnpublish(hpeerdist: isize, pcontentidentifier: &[u8]) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistServerUnpublish(hpeerdist : isize, cbcontentidentifier : u32, pcontentidentifier : *const u8) -> u32);
    PeerDistServerUnpublish(hpeerdist, pcontentidentifier.len().try_into().unwrap(), core::mem::transmute(pcontentidentifier.as_ptr()))
}
#[inline]
pub unsafe fn PeerDistShutdown(hpeerdist: isize) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistShutdown(hpeerdist : isize) -> u32);
    PeerDistShutdown(hpeerdist)
}
#[inline]
pub unsafe fn PeerDistStartup(dwversionrequested: u32, phpeerdist: *mut isize, pdwsupportedversion: Option<*mut u32>) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistStartup(dwversionrequested : u32, phpeerdist : *mut isize, pdwsupportedversion : *mut u32) -> u32);
    PeerDistStartup(dwversionrequested, phpeerdist, core::mem::transmute(pdwsupportedversion.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn PeerDistUnregisterForStatusChangeNotification(hpeerdist: isize) -> u32 {
    windows_targets::link!("peerdist.dll" "system" fn PeerDistUnregisterForStatusChangeNotification(hpeerdist : isize) -> u32);
    PeerDistUnregisterForStatusChangeNotification(hpeerdist)
}
#[inline]
pub unsafe fn PeerEndEnumeration(hpeerenum: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerEndEnumeration(hpeerenum : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerEndEnumeration(hpeerenum).ok()
}
#[inline]
pub unsafe fn PeerEnumGroups<P0>(pwzidentity: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerEnumGroups(pwzidentity : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerEnumGroups(pwzidentity.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerEnumIdentities(phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerEnumIdentities(phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerEnumIdentities(phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerFreeData(pvdata: Option<*const core::ffi::c_void>) {
    windows_targets::link!("p2p.dll" "system" fn PeerFreeData(pvdata : *const core::ffi::c_void));
    PeerFreeData(core::mem::transmute(pvdata.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn PeerGetItemCount(hpeerenum: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("p2p.dll" "system" fn PeerGetItemCount(hpeerenum : *const core::ffi::c_void, pcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGetItemCount(hpeerenum, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGetNextItem(hpeerenum: *const core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGetNextItem(hpeerenum : *const core::ffi::c_void, pcount : *mut u32, pppvitems : *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGetNextItem(hpeerenum, pcount, pppvitems).ok()
}
#[inline]
pub unsafe fn PeerGraphAddRecord(hgraph: *const core::ffi::c_void, precord: *const PEER_RECORD) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphAddRecord(hgraph : *const core::ffi::c_void, precord : *const PEER_RECORD, precordid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphAddRecord(hgraph, precord, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphClose(hgraph: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphClose(hgraph : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphClose(hgraph).ok()
}
#[inline]
pub unsafe fn PeerGraphCloseDirectConnection(hgraph: *const core::ffi::c_void, ullconnectionid: u64) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphCloseDirectConnection(hgraph : *const core::ffi::c_void, ullconnectionid : u64) -> windows_core::HRESULT);
    PeerGraphCloseDirectConnection(hgraph, ullconnectionid).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphConnect<P0>(hgraph: *const core::ffi::c_void, pwzpeerid: P0, paddress: *const PEER_ADDRESS) -> windows_core::Result<u64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphConnect(hgraph : *const core::ffi::c_void, pwzpeerid : windows_core::PCWSTR, paddress : *const PEER_ADDRESS, pullconnectionid : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphConnect(hgraph, pwzpeerid.param().abi(), paddress, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphCreate<P0>(pgraphproperties: *const PEER_GRAPH_PROPERTIES, pwzdatabasename: P0, psecurityinterface: Option<*const PEER_SECURITY_INTERFACE>, phgraph: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphCreate(pgraphproperties : *const PEER_GRAPH_PROPERTIES, pwzdatabasename : windows_core::PCWSTR, psecurityinterface : *const PEER_SECURITY_INTERFACE, phgraph : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphCreate(pgraphproperties, pwzdatabasename.param().abi(), core::mem::transmute(psecurityinterface.unwrap_or(std::ptr::null())), phgraph).ok()
}
#[inline]
pub unsafe fn PeerGraphDelete<P0, P1, P2>(pwzgraphid: P0, pwzpeerid: P1, pwzdatabasename: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphDelete(pwzgraphid : windows_core::PCWSTR, pwzpeerid : windows_core::PCWSTR, pwzdatabasename : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGraphDelete(pwzgraphid.param().abi(), pwzpeerid.param().abi(), pwzdatabasename.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphDeleteRecord<P0>(hgraph: *const core::ffi::c_void, precordid: *const windows_core::GUID, flocal: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphDeleteRecord(hgraph : *const core::ffi::c_void, precordid : *const windows_core::GUID, flocal : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    PeerGraphDeleteRecord(hgraph, precordid, flocal.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphEndEnumeration(hpeerenum: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphEndEnumeration(hpeerenum : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphEndEnumeration(hpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGraphEnumConnections(hgraph: *const core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphEnumConnections(hgraph : *const core::ffi::c_void, dwflags : u32, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphEnumConnections(hgraph, dwflags, phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGraphEnumNodes<P0>(hgraph: *const core::ffi::c_void, pwzpeerid: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphEnumNodes(hgraph : *const core::ffi::c_void, pwzpeerid : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphEnumNodes(hgraph, pwzpeerid.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGraphEnumRecords<P0>(hgraph: *const core::ffi::c_void, precordtype: Option<*const windows_core::GUID>, pwzpeerid: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphEnumRecords(hgraph : *const core::ffi::c_void, precordtype : *const windows_core::GUID, pwzpeerid : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphEnumRecords(hgraph, core::mem::transmute(precordtype.unwrap_or(std::ptr::null())), pwzpeerid.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGraphExportDatabase<P0>(hgraph: *const core::ffi::c_void, pwzfilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphExportDatabase(hgraph : *const core::ffi::c_void, pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGraphExportDatabase(hgraph, pwzfilepath.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphFreeData(pvdata: *const core::ffi::c_void) {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphFreeData(pvdata : *const core::ffi::c_void));
    PeerGraphFreeData(pvdata)
}
#[inline]
pub unsafe fn PeerGraphGetEventData(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_GRAPH_EVENT_DATA> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetEventData(hpeerevent : *const core::ffi::c_void, ppeventdata : *mut *mut PEER_GRAPH_EVENT_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetEventData(hpeerevent, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphGetItemCount(hpeerenum: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetItemCount(hpeerenum : *const core::ffi::c_void, pcount : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetItemCount(hpeerenum, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphGetNextItem(hpeerenum: *const core::ffi::c_void, pcount: *mut u32, pppvitems: *mut *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetNextItem(hpeerenum : *const core::ffi::c_void, pcount : *mut u32, pppvitems : *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphGetNextItem(hpeerenum, pcount, pppvitems).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphGetNodeInfo(hgraph: *const core::ffi::c_void, ullnodeid: u64) -> windows_core::Result<*mut PEER_NODE_INFO> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetNodeInfo(hgraph : *const core::ffi::c_void, ullnodeid : u64, ppnodeinfo : *mut *mut PEER_NODE_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetNodeInfo(hgraph, ullnodeid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphGetProperties(hgraph: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_GRAPH_PROPERTIES> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetProperties(hgraph : *const core::ffi::c_void, ppgraphproperties : *mut *mut PEER_GRAPH_PROPERTIES) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetProperties(hgraph, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphGetRecord(hgraph: *const core::ffi::c_void, precordid: *const windows_core::GUID) -> windows_core::Result<*mut PEER_RECORD> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetRecord(hgraph : *const core::ffi::c_void, precordid : *const windows_core::GUID, pprecord : *mut *mut PEER_RECORD) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetRecord(hgraph, precordid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphGetStatus(hgraph: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphGetStatus(hgraph : *const core::ffi::c_void, pdwstatus : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphGetStatus(hgraph, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphImportDatabase<P0>(hgraph: *const core::ffi::c_void, pwzfilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphImportDatabase(hgraph : *const core::ffi::c_void, pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGraphImportDatabase(hgraph, pwzfilepath.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphListen(hgraph: *const core::ffi::c_void, dwscope: u32, dwscopeid: u32, wport: u16) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphListen(hgraph : *const core::ffi::c_void, dwscope : u32, dwscopeid : u32, wport : u16) -> windows_core::HRESULT);
    PeerGraphListen(hgraph, dwscope, dwscopeid, wport).ok()
}
#[inline]
pub unsafe fn PeerGraphOpen<P0, P1, P2>(pwzgraphid: P0, pwzpeerid: P1, pwzdatabasename: P2, psecurityinterface: Option<*const PEER_SECURITY_INTERFACE>, precordtypesyncprecedence: Option<&[windows_core::GUID]>, phgraph: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphOpen(pwzgraphid : windows_core::PCWSTR, pwzpeerid : windows_core::PCWSTR, pwzdatabasename : windows_core::PCWSTR, psecurityinterface : *const PEER_SECURITY_INTERFACE, crecordtypesyncprecedence : u32, precordtypesyncprecedence : *const windows_core::GUID, phgraph : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphOpen(pwzgraphid.param().abi(), pwzpeerid.param().abi(), pwzdatabasename.param().abi(), core::mem::transmute(psecurityinterface.unwrap_or(std::ptr::null())), precordtypesyncprecedence.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(precordtypesyncprecedence.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), phgraph).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGraphOpenDirectConnection<P0>(hgraph: *const core::ffi::c_void, pwzpeerid: P0, paddress: *const PEER_ADDRESS) -> windows_core::Result<u64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphOpenDirectConnection(hgraph : *const core::ffi::c_void, pwzpeerid : windows_core::PCWSTR, paddress : *const PEER_ADDRESS, pullconnectionid : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphOpenDirectConnection(hgraph, pwzpeerid.param().abi(), paddress, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphPeerTimeToUniversalTime(hgraph: *const core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphPeerTimeToUniversalTime(hgraph : *const core::ffi::c_void, pftpeertime : *const super::super::Foundation:: FILETIME, pftuniversaltime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphPeerTimeToUniversalTime(hgraph, pftpeertime, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphRegisterEvent<P0>(hgraph: *const core::ffi::c_void, hevent: P0, peventregistrations: &[PEER_GRAPH_EVENT_REGISTRATION], phpeerevent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphRegisterEvent(hgraph : *const core::ffi::c_void, hevent : super::super::Foundation:: HANDLE, ceventregistrations : u32, peventregistrations : *const PEER_GRAPH_EVENT_REGISTRATION, phpeerevent : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphRegisterEvent(hgraph, hevent.param().abi(), peventregistrations.len().try_into().unwrap(), core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[inline]
pub unsafe fn PeerGraphSearchRecords<P0>(hgraph: *const core::ffi::c_void, pwzcriteria: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphSearchRecords(hgraph : *const core::ffi::c_void, pwzcriteria : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphSearchRecords(hgraph, pwzcriteria.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGraphSendData(hgraph: *const core::ffi::c_void, ullconnectionid: u64, ptype: *const windows_core::GUID, cbdata: u32, pvdata: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphSendData(hgraph : *const core::ffi::c_void, ullconnectionid : u64, ptype : *const windows_core::GUID, cbdata : u32, pvdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphSendData(hgraph, ullconnectionid, ptype, cbdata, pvdata).ok()
}
#[inline]
pub unsafe fn PeerGraphSetNodeAttributes<P0>(hgraph: *const core::ffi::c_void, pwzattributes: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphSetNodeAttributes(hgraph : *const core::ffi::c_void, pwzattributes : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGraphSetNodeAttributes(hgraph, pwzattributes.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphSetPresence<P0>(hgraph: *const core::ffi::c_void, fpresent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphSetPresence(hgraph : *const core::ffi::c_void, fpresent : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    PeerGraphSetPresence(hgraph, fpresent.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGraphSetProperties(hgraph: *const core::ffi::c_void, pgraphproperties: *const PEER_GRAPH_PROPERTIES) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphSetProperties(hgraph : *const core::ffi::c_void, pgraphproperties : *const PEER_GRAPH_PROPERTIES) -> windows_core::HRESULT);
    PeerGraphSetProperties(hgraph, pgraphproperties).ok()
}
#[inline]
pub unsafe fn PeerGraphShutdown() -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphShutdown() -> windows_core::HRESULT);
    PeerGraphShutdown().ok()
}
#[inline]
pub unsafe fn PeerGraphStartup(wversionrequested: u16) -> windows_core::Result<PEER_VERSION_DATA> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphStartup(wversionrequested : u16, pversiondata : *mut PEER_VERSION_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphStartup(wversionrequested, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphUniversalTimeToPeerTime(hgraph: *const core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphUniversalTimeToPeerTime(hgraph : *const core::ffi::c_void, pftuniversaltime : *const super::super::Foundation:: FILETIME, pftpeertime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGraphUniversalTimeToPeerTime(hgraph, pftuniversaltime, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGraphUnregisterEvent(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphUnregisterEvent(hpeerevent : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGraphUnregisterEvent(hpeerevent).ok()
}
#[inline]
pub unsafe fn PeerGraphUpdateRecord(hgraph: *const core::ffi::c_void, precord: *const PEER_RECORD) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphUpdateRecord(hgraph : *const core::ffi::c_void, precord : *const PEER_RECORD) -> windows_core::HRESULT);
    PeerGraphUpdateRecord(hgraph, precord).ok()
}
#[inline]
pub unsafe fn PeerGraphValidateDeferredRecords(hgraph: *const core::ffi::c_void, precordids: &[windows_core::GUID]) -> windows_core::Result<()> {
    windows_targets::link!("p2pgraph.dll" "system" fn PeerGraphValidateDeferredRecords(hgraph : *const core::ffi::c_void, crecordids : u32, precordids : *const windows_core::GUID) -> windows_core::HRESULT);
    PeerGraphValidateDeferredRecords(hgraph, precordids.len().try_into().unwrap(), core::mem::transmute(precordids.as_ptr())).ok()
}
#[inline]
pub unsafe fn PeerGroupAddRecord(hgroup: *const core::ffi::c_void, precord: *const PEER_RECORD) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupAddRecord(hgroup : *const core::ffi::c_void, precord : *const PEER_RECORD, precordid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupAddRecord(hgroup, precord, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupClose(hgroup: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupClose(hgroup : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupClose(hgroup).ok()
}
#[inline]
pub unsafe fn PeerGroupCloseDirectConnection(hgroup: *const core::ffi::c_void, ullconnectionid: u64) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupCloseDirectConnection(hgroup : *const core::ffi::c_void, ullconnectionid : u64) -> windows_core::HRESULT);
    PeerGroupCloseDirectConnection(hgroup, ullconnectionid).ok()
}
#[inline]
pub unsafe fn PeerGroupConnect(hgroup: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupConnect(hgroup : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupConnect(hgroup).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGroupConnectByAddress(hgroup: *const core::ffi::c_void, paddresses: &[PEER_ADDRESS]) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupConnectByAddress(hgroup : *const core::ffi::c_void, caddresses : u32, paddresses : *const PEER_ADDRESS) -> windows_core::HRESULT);
    PeerGroupConnectByAddress(hgroup, paddresses.len().try_into().unwrap(), core::mem::transmute(paddresses.as_ptr())).ok()
}
#[inline]
pub unsafe fn PeerGroupCreate(pproperties: *const PEER_GROUP_PROPERTIES, phgroup: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupCreate(pproperties : *const PEER_GROUP_PROPERTIES, phgroup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupCreate(pproperties, phgroup).ok()
}
#[inline]
pub unsafe fn PeerGroupCreateInvitation<P0>(hgroup: *const core::ffi::c_void, pwzidentityinfo: P0, pftexpiration: Option<*const super::super::Foundation::FILETIME>, proles: Option<&[windows_core::GUID]>) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupCreateInvitation(hgroup : *const core::ffi::c_void, pwzidentityinfo : windows_core::PCWSTR, pftexpiration : *const super::super::Foundation:: FILETIME, croles : u32, proles : *const windows_core::GUID, ppwzinvitation : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupCreateInvitation(hgroup, pwzidentityinfo.param().abi(), core::mem::transmute(pftexpiration.unwrap_or(std::ptr::null())), proles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(proles.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupCreatePasswordInvitation(hgroup: *const core::ffi::c_void) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupCreatePasswordInvitation(hgroup : *const core::ffi::c_void, ppwzinvitation : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupCreatePasswordInvitation(hgroup, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupDelete<P0, P1>(pwzidentity: P0, pwzgrouppeername: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupDelete(pwzidentity : windows_core::PCWSTR, pwzgrouppeername : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGroupDelete(pwzidentity.param().abi(), pwzgrouppeername.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGroupDeleteRecord(hgroup: *const core::ffi::c_void, precordid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupDeleteRecord(hgroup : *const core::ffi::c_void, precordid : *const windows_core::GUID) -> windows_core::HRESULT);
    PeerGroupDeleteRecord(hgroup, precordid).ok()
}
#[inline]
pub unsafe fn PeerGroupEnumConnections(hgroup: *const core::ffi::c_void, dwflags: u32, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupEnumConnections(hgroup : *const core::ffi::c_void, dwflags : u32, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupEnumConnections(hgroup, dwflags, phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGroupEnumMembers<P0>(hgroup: *const core::ffi::c_void, dwflags: u32, pwzidentity: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupEnumMembers(hgroup : *const core::ffi::c_void, dwflags : u32, pwzidentity : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupEnumMembers(hgroup, dwflags, pwzidentity.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGroupEnumRecords(hgroup: *const core::ffi::c_void, precordtype: Option<*const windows_core::GUID>, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupEnumRecords(hgroup : *const core::ffi::c_void, precordtype : *const windows_core::GUID, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupEnumRecords(hgroup, core::mem::transmute(precordtype.unwrap_or(std::ptr::null())), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGroupExportConfig<P0>(hgroup: *const core::ffi::c_void, pwzpassword: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupExportConfig(hgroup : *const core::ffi::c_void, pwzpassword : windows_core::PCWSTR, ppwzxml : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupExportConfig(hgroup, pwzpassword.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupExportDatabase<P0>(hgroup: *const core::ffi::c_void, pwzfilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupExportDatabase(hgroup : *const core::ffi::c_void, pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGroupExportDatabase(hgroup, pwzfilepath.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerGroupGetEventData(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_GROUP_EVENT_DATA> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupGetEventData(hpeerevent : *const core::ffi::c_void, ppeventdata : *mut *mut PEER_GROUP_EVENT_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupGetEventData(hpeerevent, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupGetProperties(hgroup: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_GROUP_PROPERTIES> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupGetProperties(hgroup : *const core::ffi::c_void, ppproperties : *mut *mut PEER_GROUP_PROPERTIES) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupGetProperties(hgroup, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupGetRecord(hgroup: *const core::ffi::c_void, precordid: *const windows_core::GUID) -> windows_core::Result<*mut PEER_RECORD> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupGetRecord(hgroup : *const core::ffi::c_void, precordid : *const windows_core::GUID, pprecord : *mut *mut PEER_RECORD) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupGetRecord(hgroup, precordid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupGetStatus(hgroup: *const core::ffi::c_void) -> windows_core::Result<u32> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupGetStatus(hgroup : *const core::ffi::c_void, pdwstatus : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupGetStatus(hgroup, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupImportConfig<P0, P1, P2>(pwzxml: P0, pwzpassword: P1, foverwrite: P2, ppwzidentity: *mut windows_core::PWSTR, ppwzgroup: *mut windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupImportConfig(pwzxml : windows_core::PCWSTR, pwzpassword : windows_core::PCWSTR, foverwrite : super::super::Foundation:: BOOL, ppwzidentity : *mut windows_core::PWSTR, ppwzgroup : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    PeerGroupImportConfig(pwzxml.param().abi(), pwzpassword.param().abi(), foverwrite.param().abi(), ppwzidentity, ppwzgroup).ok()
}
#[inline]
pub unsafe fn PeerGroupImportDatabase<P0>(hgroup: *const core::ffi::c_void, pwzfilepath: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupImportDatabase(hgroup : *const core::ffi::c_void, pwzfilepath : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerGroupImportDatabase(hgroup, pwzfilepath.param().abi()).ok()
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn PeerGroupIssueCredentials<P0>(hgroup: *const core::ffi::c_void, pwzsubjectidentity: P0, pcredentialinfo: Option<*const PEER_CREDENTIAL_INFO>, dwflags: u32, ppwzinvitation: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupIssueCredentials(hgroup : *const core::ffi::c_void, pwzsubjectidentity : windows_core::PCWSTR, pcredentialinfo : *const PEER_CREDENTIAL_INFO, dwflags : u32, ppwzinvitation : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    PeerGroupIssueCredentials(hgroup, pwzsubjectidentity.param().abi(), core::mem::transmute(pcredentialinfo.unwrap_or(std::ptr::null())), dwflags, core::mem::transmute(ppwzinvitation.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn PeerGroupJoin<P0, P1, P2>(pwzidentity: P0, pwzinvitation: P1, pwzcloud: P2, phgroup: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupJoin(pwzidentity : windows_core::PCWSTR, pwzinvitation : windows_core::PCWSTR, pwzcloud : windows_core::PCWSTR, phgroup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupJoin(pwzidentity.param().abi(), pwzinvitation.param().abi(), pwzcloud.param().abi(), phgroup).ok()
}
#[inline]
pub unsafe fn PeerGroupOpen<P0, P1, P2>(pwzidentity: P0, pwzgrouppeername: P1, pwzcloud: P2, phgroup: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupOpen(pwzidentity : windows_core::PCWSTR, pwzgrouppeername : windows_core::PCWSTR, pwzcloud : windows_core::PCWSTR, phgroup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupOpen(pwzidentity.param().abi(), pwzgrouppeername.param().abi(), pwzcloud.param().abi(), phgroup).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerGroupOpenDirectConnection<P0>(hgroup: *const core::ffi::c_void, pwzidentity: P0, paddress: *const PEER_ADDRESS) -> windows_core::Result<u64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupOpenDirectConnection(hgroup : *const core::ffi::c_void, pwzidentity : windows_core::PCWSTR, paddress : *const PEER_ADDRESS, pullconnectionid : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupOpenDirectConnection(hgroup, pwzidentity.param().abi(), paddress, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Security_Cryptography")]
#[inline]
pub unsafe fn PeerGroupParseInvitation<P0>(pwzinvitation: P0) -> windows_core::Result<*mut PEER_INVITATION_INFO>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupParseInvitation(pwzinvitation : windows_core::PCWSTR, ppinvitationinfo : *mut *mut PEER_INVITATION_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupParseInvitation(pwzinvitation.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupPasswordJoin<P0, P1, P2, P3>(pwzidentity: P0, pwzinvitation: P1, pwzpassword: P2, pwzcloud: P3, phgroup: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupPasswordJoin(pwzidentity : windows_core::PCWSTR, pwzinvitation : windows_core::PCWSTR, pwzpassword : windows_core::PCWSTR, pwzcloud : windows_core::PCWSTR, phgroup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupPasswordJoin(pwzidentity.param().abi(), pwzinvitation.param().abi(), pwzpassword.param().abi(), pwzcloud.param().abi(), phgroup).ok()
}
#[inline]
pub unsafe fn PeerGroupPeerTimeToUniversalTime(hgroup: *const core::ffi::c_void, pftpeertime: *const super::super::Foundation::FILETIME) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupPeerTimeToUniversalTime(hgroup : *const core::ffi::c_void, pftpeertime : *const super::super::Foundation:: FILETIME, pftuniversaltime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupPeerTimeToUniversalTime(hgroup, pftpeertime, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupRegisterEvent<P0>(hgroup: *const core::ffi::c_void, hevent: P0, peventregistrations: &[PEER_GROUP_EVENT_REGISTRATION], phpeerevent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupRegisterEvent(hgroup : *const core::ffi::c_void, hevent : super::super::Foundation:: HANDLE, ceventregistration : u32, peventregistrations : *const PEER_GROUP_EVENT_REGISTRATION, phpeerevent : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupRegisterEvent(hgroup, hevent.param().abi(), peventregistrations.len().try_into().unwrap(), core::mem::transmute(peventregistrations.as_ptr()), phpeerevent).ok()
}
#[inline]
pub unsafe fn PeerGroupResumePasswordAuthentication(hgroup: *const core::ffi::c_void, hpeereventhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupResumePasswordAuthentication(hgroup : *const core::ffi::c_void, hpeereventhandle : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupResumePasswordAuthentication(hgroup, hpeereventhandle).ok()
}
#[inline]
pub unsafe fn PeerGroupSearchRecords<P0>(hgroup: *const core::ffi::c_void, pwzcriteria: P0, phpeerenum: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerGroupSearchRecords(hgroup : *const core::ffi::c_void, pwzcriteria : windows_core::PCWSTR, phpeerenum : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupSearchRecords(hgroup, pwzcriteria.param().abi(), phpeerenum).ok()
}
#[inline]
pub unsafe fn PeerGroupSendData(hgroup: *const core::ffi::c_void, ullconnectionid: u64, ptype: *const windows_core::GUID, cbdata: u32, pvdata: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupSendData(hgroup : *const core::ffi::c_void, ullconnectionid : u64, ptype : *const windows_core::GUID, cbdata : u32, pvdata : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupSendData(hgroup, ullconnectionid, ptype, cbdata, pvdata).ok()
}
#[inline]
pub unsafe fn PeerGroupSetProperties(hgroup: *const core::ffi::c_void, pproperties: *const PEER_GROUP_PROPERTIES) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupSetProperties(hgroup : *const core::ffi::c_void, pproperties : *const PEER_GROUP_PROPERTIES) -> windows_core::HRESULT);
    PeerGroupSetProperties(hgroup, pproperties).ok()
}
#[inline]
pub unsafe fn PeerGroupShutdown() -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupShutdown() -> windows_core::HRESULT);
    PeerGroupShutdown().ok()
}
#[inline]
pub unsafe fn PeerGroupStartup(wversionrequested: u16) -> windows_core::Result<PEER_VERSION_DATA> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupStartup(wversionrequested : u16, pversiondata : *mut PEER_VERSION_DATA) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupStartup(wversionrequested, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupUniversalTimeToPeerTime(hgroup: *const core::ffi::c_void, pftuniversaltime: *const super::super::Foundation::FILETIME) -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupUniversalTimeToPeerTime(hgroup : *const core::ffi::c_void, pftuniversaltime : *const super::super::Foundation:: FILETIME, pftpeertime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerGroupUniversalTimeToPeerTime(hgroup, pftuniversaltime, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerGroupUnregisterEvent(hpeerevent: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupUnregisterEvent(hpeerevent : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerGroupUnregisterEvent(hpeerevent).ok()
}
#[inline]
pub unsafe fn PeerGroupUpdateRecord(hgroup: *const core::ffi::c_void, precord: *const PEER_RECORD) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerGroupUpdateRecord(hgroup : *const core::ffi::c_void, precord : *const PEER_RECORD) -> windows_core::HRESULT);
    PeerGroupUpdateRecord(hgroup, precord).ok()
}
#[inline]
pub unsafe fn PeerHostNameToPeerName<P0>(pwzhostname: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerHostNameToPeerName(pwzhostname : windows_core::PCWSTR, ppwzpeername : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerHostNameToPeerName(pwzhostname.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityCreate<P0, P1>(pwzclassifier: P0, pwzfriendlyname: P1, hcryptprov: usize) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityCreate(pwzclassifier : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR, hcryptprov : usize, ppwzidentity : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityCreate(pwzclassifier.param().abi(), pwzfriendlyname.param().abi(), hcryptprov, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityDelete<P0>(pwzidentity: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityDelete(pwzidentity : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerIdentityDelete(pwzidentity.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerIdentityExport<P0, P1>(pwzidentity: P0, pwzpassword: P1) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityExport(pwzidentity : windows_core::PCWSTR, pwzpassword : windows_core::PCWSTR, ppwzexportxml : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityExport(pwzidentity.param().abi(), pwzpassword.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityGetCryptKey<P0>(pwzidentity: P0) -> windows_core::Result<usize>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityGetCryptKey(pwzidentity : windows_core::PCWSTR, phcryptprov : *mut usize) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityGetCryptKey(pwzidentity.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityGetDefault() -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityGetDefault(ppwzpeername : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityGetDefault(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityGetFriendlyName<P0>(pwzidentity: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityGetFriendlyName(pwzidentity : windows_core::PCWSTR, ppwzfriendlyname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityGetFriendlyName(pwzidentity.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityGetXML<P0>(pwzidentity: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityGetXML(pwzidentity : windows_core::PCWSTR, ppwzidentityxml : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityGetXML(pwzidentity.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentityImport<P0, P1>(pwzimportxml: P0, pwzpassword: P1) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentityImport(pwzimportxml : windows_core::PCWSTR, pwzpassword : windows_core::PCWSTR, ppwzidentity : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerIdentityImport(pwzimportxml.param().abi(), pwzpassword.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerIdentitySetFriendlyName<P0, P1>(pwzidentity: P0, pwzfriendlyname: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerIdentitySetFriendlyName(pwzidentity : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR) -> windows_core::HRESULT);
    PeerIdentitySetFriendlyName(pwzidentity.param().abi(), pwzfriendlyname.param().abi()).ok()
}
#[inline]
pub unsafe fn PeerNameToPeerHostName<P0>(pwzpeername: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerNameToPeerHostName(pwzpeername : windows_core::PCWSTR, ppwzhostname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerNameToPeerHostName(pwzpeername.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn PeerPnrpEndResolve(hresolve: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpEndResolve(hresolve : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerPnrpEndResolve(hresolve).ok()
}
#[inline]
pub unsafe fn PeerPnrpGetCloudInfo(pcnumclouds: *mut u32, ppcloudinfo: *mut *mut PEER_PNRP_CLOUD_INFO) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpGetCloudInfo(pcnumclouds : *mut u32, ppcloudinfo : *mut *mut PEER_PNRP_CLOUD_INFO) -> windows_core::HRESULT);
    PeerPnrpGetCloudInfo(pcnumclouds, ppcloudinfo).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpGetEndpoint(hresolve: *const core::ffi::c_void) -> windows_core::Result<*mut PEER_PNRP_ENDPOINT_INFO> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpGetEndpoint(hresolve : *const core::ffi::c_void, ppendpoint : *mut *mut PEER_PNRP_ENDPOINT_INFO) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    PeerPnrpGetEndpoint(hresolve, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpRegister<P0>(pcwzpeername: P0, pregistrationinfo: Option<*const PEER_PNRP_REGISTRATION_INFO>, phregistration: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpRegister(pcwzpeername : windows_core::PCWSTR, pregistrationinfo : *const PEER_PNRP_REGISTRATION_INFO, phregistration : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerPnrpRegister(pcwzpeername.param().abi(), core::mem::transmute(pregistrationinfo.unwrap_or(std::ptr::null())), phregistration).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpResolve<P0, P1>(pcwzpeername: P0, pcwzcloudname: P1, pcendpoints: *mut u32, ppendpoints: *mut *mut PEER_PNRP_ENDPOINT_INFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpResolve(pcwzpeername : windows_core::PCWSTR, pcwzcloudname : windows_core::PCWSTR, pcendpoints : *mut u32, ppendpoints : *mut *mut PEER_PNRP_ENDPOINT_INFO) -> windows_core::HRESULT);
    PeerPnrpResolve(pcwzpeername.param().abi(), pcwzcloudname.param().abi(), pcendpoints, ppendpoints).ok()
}
#[inline]
pub unsafe fn PeerPnrpShutdown() -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpShutdown() -> windows_core::HRESULT);
    PeerPnrpShutdown().ok()
}
#[inline]
pub unsafe fn PeerPnrpStartResolve<P0, P1, P2>(pcwzpeername: P0, pcwzcloudname: P1, cmaxendpoints: u32, hevent: P2, phresolve: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpStartResolve(pcwzpeername : windows_core::PCWSTR, pcwzcloudname : windows_core::PCWSTR, cmaxendpoints : u32, hevent : super::super::Foundation:: HANDLE, phresolve : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    PeerPnrpStartResolve(pcwzpeername.param().abi(), pcwzcloudname.param().abi(), cmaxendpoints, hevent.param().abi(), phresolve).ok()
}
#[inline]
pub unsafe fn PeerPnrpStartup(wversionrequested: u16) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpStartup(wversionrequested : u16) -> windows_core::HRESULT);
    PeerPnrpStartup(wversionrequested).ok()
}
#[inline]
pub unsafe fn PeerPnrpUnregister(hregistration: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpUnregister(hregistration : *const core::ffi::c_void) -> windows_core::HRESULT);
    PeerPnrpUnregister(hregistration).ok()
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn PeerPnrpUpdateRegistration(hregistration: *const core::ffi::c_void, pregistrationinfo: *const PEER_PNRP_REGISTRATION_INFO) -> windows_core::Result<()> {
    windows_targets::link!("p2p.dll" "system" fn PeerPnrpUpdateRegistration(hregistration : *const core::ffi::c_void, pregistrationinfo : *const PEER_PNRP_REGISTRATION_INFO) -> windows_core::HRESULT);
    PeerPnrpUpdateRegistration(hregistration, pregistrationinfo).ok()
}
pub const DRT_ACTIVE: DRT_STATUS = DRT_STATUS(0i32);
pub const DRT_ADDRESS_FLAG_ACCEPTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(1i32);
pub const DRT_ADDRESS_FLAG_BAD_VALIDATE_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(32i32);
pub const DRT_ADDRESS_FLAG_INQUIRE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(128i32);
pub const DRT_ADDRESS_FLAG_LOOP: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(8i32);
pub const DRT_ADDRESS_FLAG_REJECTED: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(2i32);
pub const DRT_ADDRESS_FLAG_SUSPECT_UNREGISTERED_ID: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(64i32);
pub const DRT_ADDRESS_FLAG_TOO_BUSY: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(16i32);
pub const DRT_ADDRESS_FLAG_UNREACHABLE: DRT_ADDRESS_FLAGS = DRT_ADDRESS_FLAGS(4i32);
pub const DRT_ALONE: DRT_STATUS = DRT_STATUS(1i32);
pub const DRT_EVENT_LEAFSET_KEY_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(1i32);
pub const DRT_EVENT_REGISTRATION_STATE_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(2i32);
pub const DRT_EVENT_STATUS_CHANGED: DRT_EVENT_TYPE = DRT_EVENT_TYPE(0i32);
pub const DRT_E_BOOTSTRAPPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200E_u32 as _);
pub const DRT_E_BOOTSTRAPPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200F_u32 as _);
pub const DRT_E_CAPABILITY_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8062210F_u32 as _);
pub const DRT_E_DUPLICATE_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80622009_u32 as _);
pub const DRT_E_FAULTED: windows_core::HRESULT = windows_core::HRESULT(0x8062210A_u32 as _);
pub const DRT_E_INSUFFICIENT_BUFFER: windows_core::HRESULT = windows_core::HRESULT(0x8062210C_u32 as _);
pub const DRT_E_INVALID_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0x80622005_u32 as _);
pub const DRT_E_INVALID_BOOTSTRAP_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622004_u32 as _);
pub const DRT_E_INVALID_CERT_CHAIN: windows_core::HRESULT = windows_core::HRESULT(0x80621004_u32 as _);
pub const DRT_E_INVALID_INSTANCE_PREFIX: windows_core::HRESULT = windows_core::HRESULT(0x8062210D_u32 as _);
pub const DRT_E_INVALID_KEY: windows_core::HRESULT = windows_core::HRESULT(0x80621009_u32 as _);
pub const DRT_E_INVALID_KEY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0x80621002_u32 as _);
pub const DRT_E_INVALID_MAX_ADDRESSES: windows_core::HRESULT = windows_core::HRESULT(0x80621007_u32 as _);
pub const DRT_E_INVALID_MAX_ENDPOINTS: windows_core::HRESULT = windows_core::HRESULT(0x80621011_u32 as _);
pub const DRT_E_INVALID_MESSAGE: windows_core::HRESULT = windows_core::HRESULT(0x80621005_u32 as _);
pub const DRT_E_INVALID_PORT: windows_core::HRESULT = windows_core::HRESULT(0x80622000_u32 as _);
pub const DRT_E_INVALID_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80622006_u32 as _);
pub const DRT_E_INVALID_SEARCH_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80622109_u32 as _);
pub const DRT_E_INVALID_SEARCH_RANGE: windows_core::HRESULT = windows_core::HRESULT(0x80621012_u32 as _);
pub const DRT_E_INVALID_SECURITY_MODE: windows_core::HRESULT = windows_core::HRESULT(0x8062210E_u32 as _);
pub const DRT_E_INVALID_SECURITY_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622002_u32 as _);
pub const DRT_E_INVALID_SETTINGS: windows_core::HRESULT = windows_core::HRESULT(0x80622108_u32 as _);
pub const DRT_E_INVALID_TRANSPORT_PROVIDER: windows_core::HRESULT = windows_core::HRESULT(0x80622001_u32 as _);
pub const DRT_E_NO_ADDRESSES_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80622008_u32 as _);
pub const DRT_E_NO_MORE: windows_core::HRESULT = windows_core::HRESULT(0x80621006_u32 as _);
pub const DRT_E_SEARCH_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80621008_u32 as _);
pub const DRT_E_SECURITYPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200C_u32 as _);
pub const DRT_E_SECURITYPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200D_u32 as _);
pub const DRT_E_STILL_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80622003_u32 as _);
pub const DRT_E_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80621001_u32 as _);
pub const DRT_E_TRANSPORTPROVIDER_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x8062200A_u32 as _);
pub const DRT_E_TRANSPORTPROVIDER_NOT_ATTACHED: windows_core::HRESULT = windows_core::HRESULT(0x8062200B_u32 as _);
pub const DRT_E_TRANSPORT_ALREADY_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80622101_u32 as _);
pub const DRT_E_TRANSPORT_ALREADY_EXISTS_FOR_SCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80622107_u32 as _);
pub const DRT_E_TRANSPORT_EXECUTING_CALLBACK: windows_core::HRESULT = windows_core::HRESULT(0x80622106_u32 as _);
pub const DRT_E_TRANSPORT_INVALID_ARGUMENT: windows_core::HRESULT = windows_core::HRESULT(0x80622104_u32 as _);
pub const DRT_E_TRANSPORT_NOT_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x80622102_u32 as _);
pub const DRT_E_TRANSPORT_NO_DEST_ADDRESSES: windows_core::HRESULT = windows_core::HRESULT(0x80622105_u32 as _);
pub const DRT_E_TRANSPORT_SHUTTING_DOWN: windows_core::HRESULT = windows_core::HRESULT(0x80622007_u32 as _);
pub const DRT_E_TRANSPORT_STILL_BOUND: windows_core::HRESULT = windows_core::HRESULT(0x8062210B_u32 as _);
pub const DRT_E_TRANSPORT_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80622103_u32 as _);
pub const DRT_FAULTED: DRT_STATUS = DRT_STATUS(20i32);
pub const DRT_GLOBAL_SCOPE: DRT_SCOPE = DRT_SCOPE(1i32);
pub const DRT_LEAFSET_KEY_ADDED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(0i32);
pub const DRT_LEAFSET_KEY_DELETED: DRT_LEAFSET_KEY_CHANGE_TYPE = DRT_LEAFSET_KEY_CHANGE_TYPE(1i32);
pub const DRT_LINK_LOCAL_ISATAP_SCOPEID: u32 = 4294967295u32;
pub const DRT_LINK_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(3i32);
pub const DRT_MATCH_EXACT: DRT_MATCH_TYPE = DRT_MATCH_TYPE(0i32);
pub const DRT_MATCH_INTERMEDIATE: DRT_MATCH_TYPE = DRT_MATCH_TYPE(2i32);
pub const DRT_MATCH_NEAR: DRT_MATCH_TYPE = DRT_MATCH_TYPE(1i32);
pub const DRT_MAX_INSTANCE_PREFIX_LEN: u32 = 128u32;
pub const DRT_MAX_PAYLOAD_SIZE: u32 = 5120u32;
pub const DRT_MAX_ROUTING_ADDRESSES: u32 = 20u32;
pub const DRT_MIN_ROUTING_ADDRESSES: u32 = 1u32;
pub const DRT_NO_NETWORK: DRT_STATUS = DRT_STATUS(10i32);
pub const DRT_PAYLOAD_REVOKED: u32 = 1u32;
pub const DRT_REGISTRATION_STATE_UNRESOLVEABLE: DRT_REGISTRATION_STATE = DRT_REGISTRATION_STATE(1i32);
pub const DRT_SECURE_CONFIDENTIALPAYLOAD: DRT_SECURITY_MODE = DRT_SECURITY_MODE(2i32);
pub const DRT_SECURE_MEMBERSHIP: DRT_SECURITY_MODE = DRT_SECURITY_MODE(1i32);
pub const DRT_SECURE_RESOLVE: DRT_SECURITY_MODE = DRT_SECURITY_MODE(0i32);
pub const DRT_SITE_LOCAL_SCOPE: DRT_SCOPE = DRT_SCOPE(2i32);
pub const DRT_S_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x621010_u32 as _);
pub const FACILITY_DRT: u32 = 98u32;
pub const MaximumPeerDistClientInfoByHandlesClass: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(1i32);
pub const NS_PNRPCLOUD: u32 = 39u32;
pub const NS_PNRPNAME: u32 = 38u32;
pub const NS_PROVIDER_PNRPCLOUD: windows_core::GUID = windows_core::GUID::from_u128(0x03fe89ce_766d_4976_b9c1_bb9bc42c7b4d);
pub const NS_PROVIDER_PNRPNAME: windows_core::GUID = windows_core::GUID::from_u128(0x03fe89cd_766d_4976_b9c1_bb9bc42c7b4d);
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION: i32 = 2i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_1: i32 = 1i32;
pub const PEERDIST_PUBLICATION_OPTIONS_VERSION_2: i32 = 2i32;
pub const PEERDIST_READ_TIMEOUT_DEFAULT: u32 = 4294967294u32;
pub const PEERDIST_READ_TIMEOUT_LOCAL_CACHE_ONLY: u32 = 0u32;
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_1: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(1u32);
pub const PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_2: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE = PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(2u32);
pub const PEERDIST_STATUS_AVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(2i32);
pub const PEERDIST_STATUS_DISABLED: PEERDIST_STATUS = PEERDIST_STATUS(0i32);
pub const PEERDIST_STATUS_UNAVAILABLE: PEERDIST_STATUS = PEERDIST_STATUS(1i32);
pub const PEER_APPLICATION_ALL_USERS: PEER_APPLICATION_REGISTRATION_TYPE = PEER_APPLICATION_REGISTRATION_TYPE(1i32);
pub const PEER_APPLICATION_CURRENT_USER: PEER_APPLICATION_REGISTRATION_TYPE = PEER_APPLICATION_REGISTRATION_TYPE(0i32);
pub const PEER_CHANGE_ADDED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(0i32);
pub const PEER_CHANGE_DELETED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(1i32);
pub const PEER_CHANGE_UPDATED: PEER_CHANGE_TYPE = PEER_CHANGE_TYPE(2i32);
pub const PEER_COLLAB_OBJECTID_USER_PICTURE: windows_core::GUID = windows_core::GUID::from_u128(0xdd15f41f_fc4e_4922_b035_4c06a754d01d);
pub const PEER_CONNECTED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(1i32);
pub const PEER_CONNECTION_DIRECT: PEER_CONNECTION_FLAGS = PEER_CONNECTION_FLAGS(2i32);
pub const PEER_CONNECTION_FAILED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(3i32);
pub const PEER_CONNECTION_NEIGHBOR: PEER_CONNECTION_FLAGS = PEER_CONNECTION_FLAGS(1i32);
pub const PEER_DEFER_EXPIRATION: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(4i32);
pub const PEER_DISABLE_PRESENCE: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(2i32);
pub const PEER_DISCONNECTED: PEER_CONNECTION_STATUS = PEER_CONNECTION_STATUS(2i32);
pub const PEER_EVENT_ENDPOINT_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(4i32);
pub const PEER_EVENT_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(2i32);
pub const PEER_EVENT_ENDPOINT_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(5i32);
pub const PEER_EVENT_ENDPOINT_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(3i32);
pub const PEER_EVENT_MY_APPLICATION_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(8i32);
pub const PEER_EVENT_MY_ENDPOINT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(6i32);
pub const PEER_EVENT_MY_OBJECT_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(9i32);
pub const PEER_EVENT_MY_PRESENCE_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(7i32);
pub const PEER_EVENT_PEOPLE_NEAR_ME_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(10i32);
pub const PEER_EVENT_REQUEST_STATUS_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(11i32);
pub const PEER_EVENT_WATCHLIST_CHANGED: PEER_COLLAB_EVENT_TYPE = PEER_COLLAB_EVENT_TYPE(1i32);
pub const PEER_E_ALREADY_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0x800700B7_u32 as _);
pub const PEER_E_CLIENT_INVALID_COMPARTMENT_ID: windows_core::HRESULT = windows_core::HRESULT(0x80072CF2_u32 as _);
pub const PEER_E_CLOUD_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80072CEE_u32 as _);
pub const PEER_E_CLOUD_IS_DEAD: windows_core::HRESULT = windows_core::HRESULT(0x80072CF5_u32 as _);
pub const PEER_E_CLOUD_IS_SEARCH_ONLY: windows_core::HRESULT = windows_core::HRESULT(0x80072CF1_u32 as _);
pub const PEER_E_CLOUD_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80072CED_u32 as _);
pub const PEER_E_DISK_FULL: windows_core::HRESULT = windows_core::HRESULT(0x80070070_u32 as _);
pub const PEER_E_DUPLICATE_PEER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80072CF4_u32 as _);
pub const PEER_E_INVALID_IDENTITY: windows_core::HRESULT = windows_core::HRESULT(0x80072CEF_u32 as _);
pub const PEER_E_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070490_u32 as _);
pub const PEER_E_TOO_MUCH_LOAD: windows_core::HRESULT = windows_core::HRESULT(0x80072CF0_u32 as _);
pub const PEER_GRAPH_EVENT_CONNECTION_REQUIRED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(7i32);
pub const PEER_GRAPH_EVENT_DIRECT_CONNECTION: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(4i32);
pub const PEER_GRAPH_EVENT_INCOMING_DATA: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(6i32);
pub const PEER_GRAPH_EVENT_NEIGHBOR_CONNECTION: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(5i32);
pub const PEER_GRAPH_EVENT_NODE_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(8i32);
pub const PEER_GRAPH_EVENT_PROPERTY_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(2i32);
pub const PEER_GRAPH_EVENT_RECORD_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(3i32);
pub const PEER_GRAPH_EVENT_STATUS_CHANGED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(1i32);
pub const PEER_GRAPH_EVENT_SYNCHRONIZED: PEER_GRAPH_EVENT_TYPE = PEER_GRAPH_EVENT_TYPE(9i32);
pub const PEER_GRAPH_PROPERTY_DEFER_EXPIRATION: PEER_GRAPH_PROPERTY_FLAGS = PEER_GRAPH_PROPERTY_FLAGS(2i32);
pub const PEER_GRAPH_PROPERTY_HEARTBEATS: PEER_GRAPH_PROPERTY_FLAGS = PEER_GRAPH_PROPERTY_FLAGS(1i32);
pub const PEER_GRAPH_SCOPE_ANY: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(0i32);
pub const PEER_GRAPH_SCOPE_GLOBAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(1i32);
pub const PEER_GRAPH_SCOPE_LINKLOCAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(3i32);
pub const PEER_GRAPH_SCOPE_LOOPBACK: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(4i32);
pub const PEER_GRAPH_SCOPE_SITELOCAL: PEER_GRAPH_SCOPE = PEER_GRAPH_SCOPE(2i32);
pub const PEER_GRAPH_STATUS_HAS_CONNECTIONS: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(2i32);
pub const PEER_GRAPH_STATUS_LISTENING: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(1i32);
pub const PEER_GRAPH_STATUS_SYNCHRONIZED: PEER_GRAPH_STATUS_FLAGS = PEER_GRAPH_STATUS_FLAGS(4i32);
pub const PEER_GROUP_EVENT_AUTHENTICATION_FAILED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(11i32);
pub const PEER_GROUP_EVENT_CONNECTION_FAILED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(10i32);
pub const PEER_GROUP_EVENT_DIRECT_CONNECTION: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(4i32);
pub const PEER_GROUP_EVENT_INCOMING_DATA: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(6i32);
pub const PEER_GROUP_EVENT_MEMBER_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(8i32);
pub const PEER_GROUP_EVENT_NEIGHBOR_CONNECTION: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(5i32);
pub const PEER_GROUP_EVENT_PROPERTY_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(2i32);
pub const PEER_GROUP_EVENT_RECORD_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(3i32);
pub const PEER_GROUP_EVENT_STATUS_CHANGED: PEER_GROUP_EVENT_TYPE = PEER_GROUP_EVENT_TYPE(1i32);
pub const PEER_GROUP_GMC_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = PEER_GROUP_AUTHENTICATION_SCHEME(1i32);
pub const PEER_GROUP_PASSWORD_AUTHENTICATION: PEER_GROUP_AUTHENTICATION_SCHEME = PEER_GROUP_AUTHENTICATION_SCHEME(2i32);
pub const PEER_GROUP_ROLE_ADMIN: windows_core::GUID = windows_core::GUID::from_u128(0x04387127_aa56_450a_8ce5_4f565c6790f4);
pub const PEER_GROUP_ROLE_INVITING_MEMBER: windows_core::GUID = windows_core::GUID::from_u128(0x4370fd89_dc18_4cfb_8dbf_9853a8a9f905);
pub const PEER_GROUP_ROLE_MEMBER: windows_core::GUID = windows_core::GUID::from_u128(0xf12dc4c7_0857_4ca0_93fc_b1bb19a3d8c2);
pub const PEER_GROUP_STATUS_HAS_CONNECTIONS: PEER_GROUP_STATUS = PEER_GROUP_STATUS(2i32);
pub const PEER_GROUP_STATUS_LISTENING: PEER_GROUP_STATUS = PEER_GROUP_STATUS(1i32);
pub const PEER_GROUP_STORE_CREDENTIALS: PEER_GROUP_ISSUE_CREDENTIAL_FLAGS = PEER_GROUP_ISSUE_CREDENTIAL_FLAGS(1i32);
pub const PEER_INVITATION_RESPONSE_ACCEPTED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(1i32);
pub const PEER_INVITATION_RESPONSE_DECLINED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(0i32);
pub const PEER_INVITATION_RESPONSE_ERROR: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(3i32);
pub const PEER_INVITATION_RESPONSE_EXPIRED: PEER_INVITATION_RESPONSE_TYPE = PEER_INVITATION_RESPONSE_TYPE(2i32);
pub const PEER_MEMBER_CONNECTED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(1i32);
pub const PEER_MEMBER_DATA_OPTIONAL: PEER_GROUP_PROPERTY_FLAGS = PEER_GROUP_PROPERTY_FLAGS(1i32);
pub const PEER_MEMBER_DISCONNECTED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(2i32);
pub const PEER_MEMBER_JOINED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(4i32);
pub const PEER_MEMBER_LEFT: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(5i32);
pub const PEER_MEMBER_PRESENT: PEER_MEMBER_FLAGS = PEER_MEMBER_FLAGS(1i32);
pub const PEER_MEMBER_UPDATED: PEER_MEMBER_CHANGE_TYPE = PEER_MEMBER_CHANGE_TYPE(3i32);
pub const PEER_NODE_CHANGE_CONNECTED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(1i32);
pub const PEER_NODE_CHANGE_DISCONNECTED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(2i32);
pub const PEER_NODE_CHANGE_UPDATED: PEER_NODE_CHANGE_TYPE = PEER_NODE_CHANGE_TYPE(3i32);
pub const PEER_PNRP_ALL_LINK_CLOUDS: windows_core::PCWSTR = windows_core::w!("PEER_PNRP_ALL_LINKS");
pub const PEER_PRESENCE_AWAY: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(2i32);
pub const PEER_PRESENCE_BE_RIGHT_BACK: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(3i32);
pub const PEER_PRESENCE_BUSY: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(5i32);
pub const PEER_PRESENCE_IDLE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(4i32);
pub const PEER_PRESENCE_OFFLINE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(0i32);
pub const PEER_PRESENCE_ONLINE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(7i32);
pub const PEER_PRESENCE_ON_THE_PHONE: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(6i32);
pub const PEER_PRESENCE_OUT_TO_LUNCH: PEER_PRESENCE_STATUS = PEER_PRESENCE_STATUS(1i32);
pub const PEER_PUBLICATION_SCOPE_ALL: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(3i32);
pub const PEER_PUBLICATION_SCOPE_INTERNET: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(2i32);
pub const PEER_PUBLICATION_SCOPE_NEAR_ME: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(1i32);
pub const PEER_PUBLICATION_SCOPE_NONE: PEER_PUBLICATION_SCOPE = PEER_PUBLICATION_SCOPE(0i32);
pub const PEER_RECORD_ADDED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(1i32);
pub const PEER_RECORD_DELETED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(3i32);
pub const PEER_RECORD_EXPIRED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(4i32);
pub const PEER_RECORD_FLAG_AUTOREFRESH: PEER_RECORD_FLAGS = PEER_RECORD_FLAGS(1i32);
pub const PEER_RECORD_FLAG_DELETED: PEER_RECORD_FLAGS = PEER_RECORD_FLAGS(2i32);
pub const PEER_RECORD_UPDATED: PEER_RECORD_CHANGE_TYPE = PEER_RECORD_CHANGE_TYPE(2i32);
pub const PEER_SIGNIN_ALL: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(3i32);
pub const PEER_SIGNIN_INTERNET: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(2i32);
pub const PEER_SIGNIN_NEAR_ME: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(1i32);
pub const PEER_SIGNIN_NONE: PEER_SIGNIN_FLAGS = PEER_SIGNIN_FLAGS(0i32);
pub const PEER_WATCH_ALLOWED: PEER_WATCH_PERMISSION = PEER_WATCH_PERMISSION(1i32);
pub const PEER_WATCH_BLOCKED: PEER_WATCH_PERMISSION = PEER_WATCH_PERMISSION(0i32);
pub const PNRPINFO_HINT: u32 = 1u32;
pub const PNRP_CLOUD_FULL_PARTICIPANT: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(4i32);
pub const PNRP_CLOUD_NAME_LOCAL: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(1i32);
pub const PNRP_CLOUD_NO_FLAGS: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(0i32);
pub const PNRP_CLOUD_RESOLVE_ONLY: PNRP_CLOUD_FLAGS = PNRP_CLOUD_FLAGS(2i32);
pub const PNRP_CLOUD_STATE_ACTIVE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(2i32);
pub const PNRP_CLOUD_STATE_ALONE: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(6i32);
pub const PNRP_CLOUD_STATE_DEAD: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(3i32);
pub const PNRP_CLOUD_STATE_DISABLED: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(4i32);
pub const PNRP_CLOUD_STATE_NO_NET: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(5i32);
pub const PNRP_CLOUD_STATE_SYNCHRONISING: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(1i32);
pub const PNRP_CLOUD_STATE_VIRTUAL: PNRP_CLOUD_STATE = PNRP_CLOUD_STATE(0i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_BINARY: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(1i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_NONE: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(0i32);
pub const PNRP_EXTENDED_PAYLOAD_TYPE_STRING: PNRP_EXTENDED_PAYLOAD_TYPE = PNRP_EXTENDED_PAYLOAD_TYPE(2i32);
pub const PNRP_GLOBAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(1i32);
pub const PNRP_LINK_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(3i32);
pub const PNRP_MAX_ENDPOINT_ADDRESSES: u32 = 10u32;
pub const PNRP_MAX_EXTENDED_PAYLOAD_BYTES: u32 = 4096u32;
pub const PNRP_REGISTERED_ID_STATE_OK: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(1i32);
pub const PNRP_REGISTERED_ID_STATE_PROBLEM: PNRP_REGISTERED_ID_STATE = PNRP_REGISTERED_ID_STATE(2i32);
pub const PNRP_RESOLVE_CRITERIA_ANY_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(5i32);
pub const PNRP_RESOLVE_CRITERIA_DEFAULT: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(0i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(4i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(6i32);
pub const PNRP_RESOLVE_CRITERIA_NEAREST_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(2i32);
pub const PNRP_RESOLVE_CRITERIA_NON_CURRENT_PROCESS_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(3i32);
pub const PNRP_RESOLVE_CRITERIA_REMOTE_PEER_NAME: PNRP_RESOLVE_CRITERIA = PNRP_RESOLVE_CRITERIA(1i32);
pub const PNRP_SCOPE_ANY: PNRP_SCOPE = PNRP_SCOPE(0i32);
pub const PNRP_SITE_LOCAL_SCOPE: PNRP_SCOPE = PNRP_SCOPE(2i32);
pub const PeerDistClientBasicInfo: PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS = PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(0i32);
pub const SVCID_PNRPCLOUD: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce6_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V1: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce5_00c0_4fbf_bad6_18139385a49a);
pub const SVCID_PNRPNAME_V2: windows_core::GUID = windows_core::GUID::from_u128(0xc2239ce7_00c0_4fbf_bad6_18139385a49a);
pub const WSA_PNRP_CLIENT_INVALID_COMPARTMENT_ID: u32 = 11506u32;
pub const WSA_PNRP_CLOUD_DISABLED: u32 = 11502u32;
pub const WSA_PNRP_CLOUD_IS_DEAD: u32 = 11509u32;
pub const WSA_PNRP_CLOUD_IS_SEARCH_ONLY: u32 = 11505u32;
pub const WSA_PNRP_CLOUD_NOT_FOUND: u32 = 11501u32;
pub const WSA_PNRP_DUPLICATE_PEER_NAME: u32 = 11508u32;
pub const WSA_PNRP_ERROR_BASE: u32 = 11500u32;
pub const WSA_PNRP_INVALID_IDENTITY: u32 = 11503u32;
pub const WSA_PNRP_TOO_MUCH_LOAD: u32 = 11504u32;
pub const WSZ_SCOPE_GLOBAL: windows_core::PCWSTR = windows_core::w!("GLOBAL");
pub const WSZ_SCOPE_LINKLOCAL: windows_core::PCWSTR = windows_core::w!("LINKLOCAL");
pub const WSZ_SCOPE_SITELOCAL: windows_core::PCWSTR = windows_core::w!("SITELOCAL");
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_ADDRESS_FLAGS(pub i32);
impl windows_core::TypeKind for DRT_ADDRESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for DRT_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_LEAFSET_KEY_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for DRT_LEAFSET_KEY_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_LEAFSET_KEY_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_MATCH_TYPE(pub i32);
impl windows_core::TypeKind for DRT_MATCH_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_MATCH_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_MATCH_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_REGISTRATION_STATE(pub i32);
impl windows_core::TypeKind for DRT_REGISTRATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_REGISTRATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_REGISTRATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_SCOPE(pub i32);
impl windows_core::TypeKind for DRT_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_SECURITY_MODE(pub i32);
impl windows_core::TypeKind for DRT_SECURITY_MODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_SECURITY_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_SECURITY_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRT_STATUS(pub i32);
impl windows_core::TypeKind for DRT_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRT_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS(pub i32);
impl windows_core::TypeKind for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE(pub u32);
impl windows_core::TypeKind for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEERDIST_STATUS(pub i32);
impl windows_core::TypeKind for PEERDIST_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEERDIST_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEERDIST_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_APPLICATION_REGISTRATION_TYPE(pub i32);
impl windows_core::TypeKind for PEER_APPLICATION_REGISTRATION_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_APPLICATION_REGISTRATION_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_APPLICATION_REGISTRATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for PEER_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_COLLAB_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for PEER_COLLAB_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_COLLAB_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_COLLAB_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_CONNECTION_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_CONNECTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_CONNECTION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_CONNECTION_STATUS(pub i32);
impl windows_core::TypeKind for PEER_CONNECTION_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_CONNECTION_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GRAPH_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for PEER_GRAPH_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GRAPH_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GRAPH_PROPERTY_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_GRAPH_PROPERTY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GRAPH_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GRAPH_SCOPE(pub i32);
impl windows_core::TypeKind for PEER_GRAPH_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GRAPH_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GRAPH_STATUS_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_GRAPH_STATUS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GRAPH_STATUS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_STATUS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GROUP_AUTHENTICATION_SCHEME(pub i32);
impl windows_core::TypeKind for PEER_GROUP_AUTHENTICATION_SCHEME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GROUP_AUTHENTICATION_SCHEME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GROUP_EVENT_TYPE(pub i32);
impl windows_core::TypeKind for PEER_GROUP_EVENT_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GROUP_ISSUE_CREDENTIAL_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GROUP_ISSUE_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GROUP_PROPERTY_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_GROUP_PROPERTY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GROUP_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GROUP_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_GROUP_STATUS(pub i32);
impl windows_core::TypeKind for PEER_GROUP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_GROUP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_GROUP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_INVITATION_RESPONSE_TYPE(pub i32);
impl windows_core::TypeKind for PEER_INVITATION_RESPONSE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_INVITATION_RESPONSE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_INVITATION_RESPONSE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_MEMBER_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for PEER_MEMBER_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_MEMBER_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_MEMBER_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_MEMBER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_MEMBER_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_NODE_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for PEER_NODE_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_NODE_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_NODE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_PRESENCE_STATUS(pub i32);
impl windows_core::TypeKind for PEER_PRESENCE_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_PRESENCE_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_PRESENCE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_PUBLICATION_SCOPE(pub i32);
impl windows_core::TypeKind for PEER_PUBLICATION_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_PUBLICATION_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_PUBLICATION_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_RECORD_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for PEER_RECORD_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_RECORD_CHANGE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_RECORD_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_RECORD_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_RECORD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_RECORD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_RECORD_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_SIGNIN_FLAGS(pub i32);
impl windows_core::TypeKind for PEER_SIGNIN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_SIGNIN_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_SIGNIN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PEER_WATCH_PERMISSION(pub i32);
impl windows_core::TypeKind for PEER_WATCH_PERMISSION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PEER_WATCH_PERMISSION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PEER_WATCH_PERMISSION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_CLOUD_FLAGS(pub i32);
impl windows_core::TypeKind for PNRP_CLOUD_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_CLOUD_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_CLOUD_STATE(pub i32);
impl windows_core::TypeKind for PNRP_CLOUD_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_CLOUD_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_EXTENDED_PAYLOAD_TYPE(pub i32);
impl windows_core::TypeKind for PNRP_EXTENDED_PAYLOAD_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_EXTENDED_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_REGISTERED_ID_STATE(pub i32);
impl windows_core::TypeKind for PNRP_REGISTERED_ID_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_REGISTERED_ID_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_REGISTERED_ID_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_RESOLVE_CRITERIA(pub i32);
impl windows_core::TypeKind for PNRP_RESOLVE_CRITERIA {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_RESOLVE_CRITERIA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_RESOLVE_CRITERIA").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PNRP_SCOPE(pub i32);
impl windows_core::TypeKind for PNRP_SCOPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PNRP_SCOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PNRP_SCOPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_ADDRESS {
    pub socketAddress: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub flags: u32,
    pub nearness: i32,
    pub latency: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_ADDRESS_LIST {
    pub AddressCount: u32,
    pub AddressList: [DRT_ADDRESS; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_ADDRESS_LIST {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_BOOTSTRAP_PROVIDER {
    pub pvContext: *mut core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub InitResolve: isize,
    pub IssueResolve: isize,
    pub EndResolve: isize,
    pub Register: isize,
    pub Unregister: isize,
}
impl windows_core::TypeKind for DRT_BOOTSTRAP_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_BOOTSTRAP_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_DATA {
    pub cb: u32,
    pub pb: *mut u8,
}
impl windows_core::TypeKind for DRT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct DRT_EVENT_DATA {
    pub r#type: DRT_EVENT_TYPE,
    pub hr: windows_core::HRESULT,
    pub pvContext: *mut core::ffi::c_void,
    pub Anonymous: DRT_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union DRT_EVENT_DATA_0 {
    pub leafsetKeyChange: DRT_EVENT_DATA_0_0,
    pub registrationStateChange: DRT_EVENT_DATA_0_1,
    pub statusChange: DRT_EVENT_DATA_0_2,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_EVENT_DATA_0_0 {
    pub change: DRT_LEAFSET_KEY_CHANGE_TYPE,
    pub localKey: DRT_DATA,
    pub remoteKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_EVENT_DATA_0_1 {
    pub state: DRT_REGISTRATION_STATE,
    pub localKey: DRT_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_EVENT_DATA_0_2 {
    pub status: DRT_STATUS,
    pub bootstrapAddresses: DRT_EVENT_DATA_0_2_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_EVENT_DATA_0_2_0 {
    pub cntAddress: u32,
    pub pAddresses: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for DRT_EVENT_DATA_0_2_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for DRT_EVENT_DATA_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_REGISTRATION {
    pub key: DRT_DATA,
    pub appData: DRT_DATA,
}
impl windows_core::TypeKind for DRT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_SEARCH_INFO {
    pub dwSize: u32,
    pub fIterative: super::super::Foundation::BOOL,
    pub fAllowCurrentInstanceMatch: super::super::Foundation::BOOL,
    pub fAnyMatchInRange: super::super::Foundation::BOOL,
    pub cMaxEndpoints: u32,
    pub pMaximumKey: *mut DRT_DATA,
    pub pMinimumKey: *mut DRT_DATA,
}
impl windows_core::TypeKind for DRT_SEARCH_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_SEARCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_SEARCH_RESULT {
    pub dwSize: u32,
    pub r#type: DRT_MATCH_TYPE,
    pub pvContext: *mut core::ffi::c_void,
    pub registration: DRT_REGISTRATION,
}
impl windows_core::TypeKind for DRT_SEARCH_RESULT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_SEARCH_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_SECURITY_PROVIDER {
    pub pvContext: *mut core::ffi::c_void,
    pub Attach: isize,
    pub Detach: isize,
    pub RegisterKey: isize,
    pub UnregisterKey: isize,
    pub ValidateAndUnpackPayload: isize,
    pub SecureAndPackPayload: isize,
    pub FreeData: isize,
    pub EncryptData: isize,
    pub DecryptData: isize,
    pub GetSerializedCredential: isize,
    pub ValidateRemoteCredential: isize,
    pub SignData: isize,
    pub VerifyData: isize,
}
impl windows_core::TypeKind for DRT_SECURITY_PROVIDER {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_SECURITY_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRT_SETTINGS {
    pub dwSize: u32,
    pub cbKey: u32,
    pub bProtocolMajorVersion: u8,
    pub bProtocolMinorVersion: u8,
    pub ulMaxRoutingAddresses: u32,
    pub pwzDrtInstancePrefix: windows_core::PWSTR,
    pub hTransport: *mut core::ffi::c_void,
    pub pSecurityProvider: *mut DRT_SECURITY_PROVIDER,
    pub pBootstrapProvider: *mut DRT_BOOTSTRAP_PROVIDER,
    pub eSecurityMode: DRT_SECURITY_MODE,
}
impl windows_core::TypeKind for DRT_SETTINGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRT_SETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEERDIST_CLIENT_BASIC_INFO {
    pub fFlashCrowd: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for PEERDIST_CLIENT_BASIC_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEERDIST_CLIENT_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEERDIST_CONTENT_TAG {
    pub Data: [u8; 16],
}
impl windows_core::TypeKind for PEERDIST_CONTENT_TAG {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEERDIST_CONTENT_TAG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEERDIST_PUBLICATION_OPTIONS {
    pub dwVersion: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for PEERDIST_PUBLICATION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEERDIST_PUBLICATION_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEERDIST_RETRIEVAL_OPTIONS {
    pub cbSize: u32,
    pub dwContentInfoMinVersion: u32,
    pub dwContentInfoMaxVersion: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for PEERDIST_RETRIEVAL_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEERDIST_RETRIEVAL_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEERDIST_STATUS_INFO {
    pub cbSize: u32,
    pub status: PEERDIST_STATUS,
    pub dwMinVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
    pub dwMaxVer: PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE,
}
impl windows_core::TypeKind for PEERDIST_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEERDIST_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PEER_ADDRESS {
    pub dwSize: u32,
    pub sin6: super::super::Networking::WinSock::SOCKADDR_IN6,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_ADDRESS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_APPLICATION {
    pub id: windows_core::GUID,
    pub data: PEER_DATA,
    pub pwzDescription: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_APPLICATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_APPLICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_APPLICATION_REGISTRATION_INFO {
    pub application: PEER_APPLICATION,
    pub pwzApplicationToLaunch: windows_core::PWSTR,
    pub pwzApplicationArguments: windows_core::PWSTR,
    pub dwPublicationScope: u32,
}
impl windows_core::TypeKind for PEER_APPLICATION_REGISTRATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_APPLICATION_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_APP_LAUNCH_INFO {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub pInvitation: *mut PEER_INVITATION,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_APP_LAUNCH_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_APP_LAUNCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PEER_COLLAB_EVENT_DATA {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub Anonymous: PEER_COLLAB_EVENT_DATA_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_COLLAB_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_COLLAB_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub union PEER_COLLAB_EVENT_DATA_0 {
    pub watchListChangedData: PEER_EVENT_WATCHLIST_CHANGED_DATA,
    pub presenceChangedData: PEER_EVENT_PRESENCE_CHANGED_DATA,
    pub applicationChangedData: PEER_EVENT_APPLICATION_CHANGED_DATA,
    pub objectChangedData: PEER_EVENT_OBJECT_CHANGED_DATA,
    pub endpointChangedData: PEER_EVENT_ENDPOINT_CHANGED_DATA,
    pub peopleNearMeChangedData: PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA,
    pub requestStatusChangedData: PEER_EVENT_REQUEST_STATUS_CHANGED_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_COLLAB_EVENT_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_COLLAB_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_COLLAB_EVENT_REGISTRATION {
    pub eventType: PEER_COLLAB_EVENT_TYPE,
    pub pInstance: *mut windows_core::GUID,
}
impl windows_core::TypeKind for PEER_COLLAB_EVENT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_COLLAB_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PEER_CONNECTION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub pwzPeerId: windows_core::PWSTR,
    pub address: PEER_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_CONNECTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_CONTACT {
    pub pwzPeerName: windows_core::PWSTR,
    pub pwzNickName: windows_core::PWSTR,
    pub pwzDisplayName: windows_core::PWSTR,
    pub pwzEmailAddress: windows_core::PWSTR,
    pub fWatch: super::super::Foundation::BOOL,
    pub WatcherPermissions: PEER_WATCH_PERMISSION,
    pub credentials: PEER_DATA,
}
impl windows_core::TypeKind for PEER_CONTACT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_CONTACT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_CREDENTIAL_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzFriendlyName: windows_core::PWSTR,
    pub pPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub pwzIssuerPeerName: windows_core::PWSTR,
    pub pwzIssuerFriendlyName: windows_core::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut windows_core::GUID,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for PEER_CREDENTIAL_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for PEER_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_DATA {
    pub cbData: u32,
    pub pbData: *mut u8,
}
impl windows_core::TypeKind for PEER_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PEER_ENDPOINT {
    pub address: PEER_ADDRESS,
    pub pwzEndpointName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_ENDPOINT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_APPLICATION_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pApplication: *mut PEER_APPLICATION,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_APPLICATION_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_CONNECTION_CHANGE_DATA {
    pub dwSize: u32,
    pub status: PEER_CONNECTION_STATUS,
    pub ullConnectionId: u64,
    pub ullNodeId: u64,
    pub ullNextConnectionId: u64,
    pub hrConnectionFailedReason: windows_core::HRESULT,
}
impl windows_core::TypeKind for PEER_EVENT_CONNECTION_CHANGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_ENDPOINT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_INCOMING_DATA {
    pub dwSize: u32,
    pub ullConnectionId: u64,
    pub r#type: windows_core::GUID,
    pub data: PEER_DATA,
}
impl windows_core::TypeKind for PEER_EVENT_INCOMING_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_INCOMING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_MEMBER_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_MEMBER_CHANGE_TYPE,
    pub pwzIdentity: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_EVENT_MEMBER_CHANGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_NODE_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_NODE_CHANGE_TYPE,
    pub ullNodeId: u64,
    pub pwzPeerId: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_EVENT_NODE_CHANGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_NODE_CHANGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_OBJECT_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pObject: *mut PEER_OBJECT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_OBJECT_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    pub changeType: PEER_CHANGE_TYPE,
    pub pPeopleNearMe: *mut PEER_PEOPLE_NEAR_ME,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_PRESENCE_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub changeType: PEER_CHANGE_TYPE,
    pub pPresenceInfo: *mut PEER_PRESENCE_INFO,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_PRESENCE_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_RECORD_CHANGE_DATA {
    pub dwSize: u32,
    pub changeType: PEER_RECORD_CHANGE_TYPE,
    pub recordId: windows_core::GUID,
    pub recordType: windows_core::GUID,
}
impl windows_core::TypeKind for PEER_EVENT_RECORD_CHANGE_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_RECORD_CHANGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    pub pEndpoint: *mut PEER_ENDPOINT,
    pub hrChange: windows_core::HRESULT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_SYNCHRONIZED_DATA {
    pub dwSize: u32,
    pub recordType: windows_core::GUID,
}
impl windows_core::TypeKind for PEER_EVENT_SYNCHRONIZED_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_SYNCHRONIZED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_EVENT_WATCHLIST_CHANGED_DATA {
    pub pContact: *mut PEER_CONTACT,
    pub changeType: PEER_CHANGE_TYPE,
}
impl windows_core::TypeKind for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEER_GRAPH_EVENT_DATA {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub Anonymous: PEER_GRAPH_EVENT_DATA_0,
}
impl windows_core::TypeKind for PEER_GRAPH_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GRAPH_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PEER_GRAPH_EVENT_DATA_0 {
    pub dwStatus: PEER_GRAPH_STATUS_FLAGS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub nodeChangeData: PEER_EVENT_NODE_CHANGE_DATA,
    pub synchronizedData: PEER_EVENT_SYNCHRONIZED_DATA,
}
impl windows_core::TypeKind for PEER_GRAPH_EVENT_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GRAPH_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_GRAPH_EVENT_REGISTRATION {
    pub eventType: PEER_GRAPH_EVENT_TYPE,
    pub pType: *mut windows_core::GUID,
}
impl windows_core::TypeKind for PEER_GRAPH_EVENT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GRAPH_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_GRAPH_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwScope: u32,
    pub dwMaxRecordSize: u32,
    pub pwzGraphId: windows_core::PWSTR,
    pub pwzCreatorId: windows_core::PWSTR,
    pub pwzFriendlyName: windows_core::PWSTR,
    pub pwzComment: windows_core::PWSTR,
    pub ulPresenceLifetime: u32,
    pub cPresenceMax: u32,
}
impl windows_core::TypeKind for PEER_GRAPH_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GRAPH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PEER_GROUP_EVENT_DATA {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub Anonymous: PEER_GROUP_EVENT_DATA_0,
}
impl windows_core::TypeKind for PEER_GROUP_EVENT_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GROUP_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PEER_GROUP_EVENT_DATA_0 {
    pub dwStatus: PEER_GROUP_STATUS,
    pub incomingData: PEER_EVENT_INCOMING_DATA,
    pub recordChangeData: PEER_EVENT_RECORD_CHANGE_DATA,
    pub connectionChangeData: PEER_EVENT_CONNECTION_CHANGE_DATA,
    pub memberChangeData: PEER_EVENT_MEMBER_CHANGE_DATA,
    pub hrConnectionFailedReason: windows_core::HRESULT,
}
impl windows_core::TypeKind for PEER_GROUP_EVENT_DATA_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GROUP_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_GROUP_EVENT_REGISTRATION {
    pub eventType: PEER_GROUP_EVENT_TYPE,
    pub pType: *mut windows_core::GUID,
}
impl windows_core::TypeKind for PEER_GROUP_EVENT_REGISTRATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GROUP_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_GROUP_PROPERTIES {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloud: windows_core::PWSTR,
    pub pwzClassifier: windows_core::PWSTR,
    pub pwzGroupPeerName: windows_core::PWSTR,
    pub pwzCreatorPeerName: windows_core::PWSTR,
    pub pwzFriendlyName: windows_core::PWSTR,
    pub pwzComment: windows_core::PWSTR,
    pub ulMemberDataLifetime: u32,
    pub ulPresenceLifetime: u32,
    pub dwAuthenticationSchemes: u32,
    pub pwzGroupPassword: windows_core::PWSTR,
    pub groupPasswordRole: windows_core::GUID,
}
impl windows_core::TypeKind for PEER_GROUP_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_GROUP_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_INVITATION {
    pub applicationId: windows_core::GUID,
    pub applicationData: PEER_DATA,
    pub pwzMessage: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_INVITATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_INVITATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_INVITATION_INFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzCloudName: windows_core::PWSTR,
    pub dwScope: u32,
    pub dwCloudFlags: u32,
    pub pwzGroupPeerName: windows_core::PWSTR,
    pub pwzIssuerPeerName: windows_core::PWSTR,
    pub pwzSubjectPeerName: windows_core::PWSTR,
    pub pwzGroupFriendlyName: windows_core::PWSTR,
    pub pwzIssuerFriendlyName: windows_core::PWSTR,
    pub pwzSubjectFriendlyName: windows_core::PWSTR,
    pub ftValidityStart: super::super::Foundation::FILETIME,
    pub ftValidityEnd: super::super::Foundation::FILETIME,
    pub cRoles: u32,
    pub pRoles: *mut windows_core::GUID,
    pub cClassifiers: u32,
    pub ppwzClassifiers: *mut windows_core::PWSTR,
    pub pSubjectPublicKey: *mut super::super::Security::Cryptography::CERT_PUBLIC_KEY_INFO,
    pub authScheme: PEER_GROUP_AUTHENTICATION_SCHEME,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::TypeKind for PEER_INVITATION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for PEER_INVITATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_INVITATION_RESPONSE {
    pub action: PEER_INVITATION_RESPONSE_TYPE,
    pub pwzMessage: windows_core::PWSTR,
    pub hrExtendedInfo: windows_core::HRESULT,
}
impl windows_core::TypeKind for PEER_INVITATION_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_INVITATION_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_MEMBER {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub pwzIdentity: windows_core::PWSTR,
    pub pwzAttributes: windows_core::PWSTR,
    pub ullNodeId: u64,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pCredentialInfo: *mut PEER_CREDENTIAL_INFO,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl windows_core::TypeKind for PEER_MEMBER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl Default for PEER_MEMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_NAME_PAIR {
    pub dwSize: u32,
    pub pwzPeerName: windows_core::PWSTR,
    pub pwzFriendlyName: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_NAME_PAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_NAME_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_NODE_INFO {
    pub dwSize: u32,
    pub ullNodeId: u64,
    pub pwzPeerId: windows_core::PWSTR,
    pub cAddresses: u32,
    pub pAddresses: *mut PEER_ADDRESS,
    pub pwzAttributes: windows_core::PWSTR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_NODE_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_NODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_OBJECT {
    pub id: windows_core::GUID,
    pub data: PEER_DATA,
    pub dwPublicationScope: u32,
}
impl windows_core::TypeKind for PEER_OBJECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy)]
pub struct PEER_PEOPLE_NEAR_ME {
    pub pwzNickName: windows_core::PWSTR,
    pub endpoint: PEER_ENDPOINT,
    pub id: windows_core::GUID,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_PEOPLE_NEAR_ME {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_PEOPLE_NEAR_ME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_PNRP_CLOUD_INFO {
    pub pwzCloudName: windows_core::PWSTR,
    pub dwScope: PNRP_SCOPE,
    pub dwScopeId: u32,
}
impl windows_core::TypeKind for PEER_PNRP_CLOUD_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_PNRP_CLOUD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_PNRP_ENDPOINT_INFO {
    pub pwzPeerName: windows_core::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub pwzComment: windows_core::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_PNRP_ENDPOINT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_PNRP_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_PNRP_REGISTRATION_INFO {
    pub pwzCloudName: windows_core::PWSTR,
    pub pwzPublishingIdentity: windows_core::PWSTR,
    pub cAddresses: u32,
    pub ppAddresses: *mut *mut super::super::Networking::WinSock::SOCKADDR,
    pub wPort: u16,
    pub pwzComment: windows_core::PWSTR,
    pub payload: PEER_DATA,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PEER_PNRP_REGISTRATION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PEER_PNRP_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_PRESENCE_INFO {
    pub status: PEER_PRESENCE_STATUS,
    pub pwzDescriptiveText: windows_core::PWSTR,
}
impl windows_core::TypeKind for PEER_PRESENCE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_PRESENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_RECORD {
    pub dwSize: u32,
    pub r#type: windows_core::GUID,
    pub id: windows_core::GUID,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pwzCreatorId: windows_core::PWSTR,
    pub pwzModifiedById: windows_core::PWSTR,
    pub pwzAttributes: windows_core::PWSTR,
    pub ftCreation: super::super::Foundation::FILETIME,
    pub ftExpiration: super::super::Foundation::FILETIME,
    pub ftLastModified: super::super::Foundation::FILETIME,
    pub securityData: PEER_DATA,
    pub data: PEER_DATA,
}
impl windows_core::TypeKind for PEER_RECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PEER_SECURITY_INTERFACE {
    pub dwSize: u32,
    pub pwzSspFilename: windows_core::PWSTR,
    pub pwzPackageName: windows_core::PWSTR,
    pub cbSecurityInfo: u32,
    pub pbSecurityInfo: *mut u8,
    pub pvContext: *mut core::ffi::c_void,
    pub pfnValidateRecord: PFNPEER_VALIDATE_RECORD,
    pub pfnSecureRecord: PFNPEER_SECURE_RECORD,
    pub pfnFreeSecurityData: PFNPEER_FREE_SECURITY_DATA,
    pub pfnAuthFailed: PFNPEER_ON_PASSWORD_AUTH_FAILED,
}
impl windows_core::TypeKind for PEER_SECURITY_INTERFACE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_SECURITY_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEER_VERSION_DATA {
    pub wVersion: u16,
    pub wHighestVersion: u16,
}
impl windows_core::TypeKind for PEER_VERSION_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PEER_VERSION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PNRPCLOUDINFO {
    pub dwSize: u32,
    pub Cloud: PNRP_CLOUD_ID,
    pub enCloudState: PNRP_CLOUD_STATE,
    pub enCloudFlags: PNRP_CLOUD_FLAGS,
}
impl windows_core::TypeKind for PNRPCLOUDINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PNRPCLOUDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PNRPINFO_V1 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::TypeKind for PNRPINFO_V1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for PNRPINFO_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub struct PNRPINFO_V2 {
    pub dwSize: u32,
    pub lpwszIdentity: windows_core::PWSTR,
    pub nMaxResolve: u32,
    pub dwTimeout: u32,
    pub dwLifetime: u32,
    pub enResolveCriteria: PNRP_RESOLVE_CRITERIA,
    pub dwFlags: u32,
    pub saHint: super::super::Networking::WinSock::SOCKET_ADDRESS,
    pub enNameState: PNRP_REGISTERED_ID_STATE,
    pub enExtendedPayloadType: PNRP_EXTENDED_PAYLOAD_TYPE,
    pub Anonymous: PNRPINFO_V2_0,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for PNRPINFO_V2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl Default for PNRPINFO_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
#[derive(Clone, Copy)]
pub union PNRPINFO_V2_0 {
    pub blobPayload: super::super::System::Com::BLOB,
    pub pwszPayload: windows_core::PWSTR,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for PNRPINFO_V2_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl Default for PNRPINFO_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PNRP_CLOUD_ID {
    pub AddressFamily: i32,
    pub Scope: PNRP_SCOPE,
    pub ScopeId: u32,
}
impl windows_core::TypeKind for PNRP_CLOUD_ID {
    type TypeKind = windows_core::CopyType;
}
impl Default for PNRP_CLOUD_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub type DRT_BOOTSTRAP_RESOLVE_CALLBACK = Option<unsafe extern "system" fn(hr: windows_core::HRESULT, pvcontext: *mut core::ffi::c_void, paddresses: *mut super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, ffatalerror: super::super::Foundation::BOOL)>;
pub type PFNPEER_FREE_SECURITY_DATA = Option<unsafe extern "system" fn(hgraph: *const core::ffi::c_void, pvcontext: *const core::ffi::c_void, psecuritydata: *const PEER_DATA) -> windows_core::HRESULT>;
pub type PFNPEER_ON_PASSWORD_AUTH_FAILED = Option<unsafe extern "system" fn(hgraph: *const core::ffi::c_void, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFNPEER_SECURE_RECORD = Option<unsafe extern "system" fn(hgraph: *const core::ffi::c_void, pvcontext: *const core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE, ppsecuritydata: *mut *mut PEER_DATA) -> windows_core::HRESULT>;
pub type PFNPEER_VALIDATE_RECORD = Option<unsafe extern "system" fn(hgraph: *const core::ffi::c_void, pvcontext: *const core::ffi::c_void, precord: *const PEER_RECORD, changetype: PEER_RECORD_CHANGE_TYPE) -> windows_core::HRESULT>;
