#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn ADsBuildEnumerator<P0>(padscontainer: P0) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>
where
    P0: windows_core::Param<IADsContainer>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsBuildEnumerator(padscontainer : * mut core::ffi::c_void, ppenumvariant : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ADsBuildEnumerator(padscontainer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn ADsBuildVarArrayInt(lpdwobjecttypes: *mut u32, dwobjecttypes: u32, pvar: *mut windows_core::VARIANT) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn ADsBuildVarArrayInt(lpdwobjecttypes : *mut u32, dwobjecttypes : u32, pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    ADsBuildVarArrayInt(lpdwobjecttypes, dwobjecttypes, core::mem::transmute(pvar)).ok()
}
#[inline]
pub unsafe fn ADsBuildVarArrayStr(lpppathnames: &[windows_core::PCWSTR], pvar: *mut windows_core::VARIANT) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn ADsBuildVarArrayStr(lpppathnames : *const windows_core::PCWSTR, dwpathnames : u32, pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    ADsBuildVarArrayStr(core::mem::transmute(lpppathnames.as_ptr()), lpppathnames.len().try_into().unwrap(), core::mem::transmute(pvar)).ok()
}
#[inline]
pub unsafe fn ADsDecodeBinaryData<P0>(szsrcdata: P0, ppbdestdata: *mut *mut u8, pdwdestlen: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsDecodeBinaryData(szsrcdata : windows_core::PCWSTR, ppbdestdata : *mut *mut u8, pdwdestlen : *mut u32) -> windows_core::HRESULT);
    ADsDecodeBinaryData(szsrcdata.param().abi(), ppbdestdata, pdwdestlen).ok()
}
#[inline]
pub unsafe fn ADsEncodeBinaryData(pbsrcdata: *mut u8, dwsrclen: u32, ppszdestdata: *mut windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn ADsEncodeBinaryData(pbsrcdata : *mut u8, dwsrclen : u32, ppszdestdata : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    ADsEncodeBinaryData(pbsrcdata, dwsrclen, ppszdestdata).ok()
}
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn ADsEnumerateNext<P0>(penumvariant: P0, celements: u32, pvar: *mut windows_core::VARIANT, pcelementsfetched: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Ole::IEnumVARIANT>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsEnumerateNext(penumvariant : * mut core::ffi::c_void, celements : u32, pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >, pcelementsfetched : *mut u32) -> windows_core::HRESULT);
    ADsEnumerateNext(penumvariant.param().abi(), celements, core::mem::transmute(pvar), pcelementsfetched).ok()
}
#[cfg(feature = "Win32_System_Ole")]
#[inline]
pub unsafe fn ADsFreeEnumerator<P0>(penumvariant: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Ole::IEnumVARIANT>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsFreeEnumerator(penumvariant : * mut core::ffi::c_void) -> windows_core::HRESULT);
    ADsFreeEnumerator(penumvariant.param().abi()).ok()
}
#[inline]
pub unsafe fn ADsGetLastError(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn ADsGetLastError(lperror : *mut u32, lperrorbuf : windows_core::PWSTR, dwerrorbuflen : u32, lpnamebuf : windows_core::PWSTR, dwnamebuflen : u32) -> windows_core::HRESULT);
    ADsGetLastError(lperror, core::mem::transmute(lperrorbuf.as_ptr()), lperrorbuf.len().try_into().unwrap(), core::mem::transmute(lpnamebuf.as_ptr()), lpnamebuf.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn ADsGetObject<P0>(lpszpathname: P0, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsGetObject(lpszpathname : windows_core::PCWSTR, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ADsGetObject(lpszpathname.param().abi(), riid, ppobject).ok()
}
#[inline]
pub unsafe fn ADsOpenObject<P0, P1, P2>(lpszpathname: P0, lpszusername: P1, lpszpassword: P2, dwreserved: ADS_AUTHENTICATION_ENUM, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsOpenObject(lpszpathname : windows_core::PCWSTR, lpszusername : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwreserved : ADS_AUTHENTICATION_ENUM, riid : *const windows_core::GUID, ppobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    ADsOpenObject(lpszpathname.param().abi(), lpszusername.param().abi(), lpszpassword.param().abi(), dwreserved, riid, ppobject).ok()
}
#[inline]
pub unsafe fn ADsPropCheckIfWritable<P0>(pwzattr: P0, pwritableattrs: *const ADS_ATTR_INFO) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropCheckIfWritable(pwzattr : windows_core::PCWSTR, pwritableattrs : *const ADS_ATTR_INFO) -> super::super::Foundation:: BOOL);
    ADsPropCheckIfWritable(pwzattr.param().abi(), pwritableattrs)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ADsPropCreateNotifyObj<P0, P1>(pappthddataobj: P0, pwzadsobjname: P1, phnotifyobj: *mut super::super::Foundation::HWND) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IDataObject>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropCreateNotifyObj(pappthddataobj : * mut core::ffi::c_void, pwzadsobjname : windows_core::PCWSTR, phnotifyobj : *mut super::super::Foundation:: HWND) -> windows_core::HRESULT);
    ADsPropCreateNotifyObj(pappthddataobj.param().abi(), pwzadsobjname.param().abi(), phnotifyobj).ok()
}
#[inline]
pub unsafe fn ADsPropGetInitInfo<P0>(hnotifyobj: P0, pinitparams: *mut ADSPROPINITPARAMS) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropGetInitInfo(hnotifyobj : super::super::Foundation:: HWND, pinitparams : *mut ADSPROPINITPARAMS) -> super::super::Foundation:: BOOL);
    ADsPropGetInitInfo(hnotifyobj.param().abi(), pinitparams)
}
#[inline]
pub unsafe fn ADsPropSendErrorMessage<P0>(hnotifyobj: P0, perror: *mut ADSPROPERROR) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropSendErrorMessage(hnotifyobj : super::super::Foundation:: HWND, perror : *mut ADSPROPERROR) -> super::super::Foundation:: BOOL);
    ADsPropSendErrorMessage(hnotifyobj.param().abi(), perror)
}
#[inline]
pub unsafe fn ADsPropSetHwnd<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropSetHwnd(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ADsPropSetHwnd(hnotifyobj.param().abi(), hpage.param().abi())
}
#[inline]
pub unsafe fn ADsPropSetHwndWithTitle<P0, P1>(hnotifyobj: P0, hpage: P1, ptztitle: *const i8) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropSetHwndWithTitle(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND, ptztitle : *const i8) -> super::super::Foundation:: BOOL);
    ADsPropSetHwndWithTitle(hnotifyobj.param().abi(), hpage.param().abi(), ptztitle)
}
#[inline]
pub unsafe fn ADsPropShowErrorDialog<P0, P1>(hnotifyobj: P0, hpage: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("dsprop.dll" "system" fn ADsPropShowErrorDialog(hnotifyobj : super::super::Foundation:: HWND, hpage : super::super::Foundation:: HWND) -> super::super::Foundation:: BOOL);
    ADsPropShowErrorDialog(hnotifyobj.param().abi(), hpage.param().abi())
}
#[inline]
pub unsafe fn ADsSetLastError<P0, P1>(dwerr: u32, pszerror: P0, pszprovider: P1)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn ADsSetLastError(dwerr : u32, pszerror : windows_core::PCWSTR, pszprovider : windows_core::PCWSTR));
    ADsSetLastError(dwerr, pszerror.param().abi(), pszprovider.param().abi())
}
#[inline]
pub unsafe fn AdsFreeAdsValues(padsvalues: *mut ADSVALUE, dwnumvalues: u32) {
    windows_targets::link!("activeds.dll" "system" fn AdsFreeAdsValues(padsvalues : *mut ADSVALUE, dwnumvalues : u32));
    AdsFreeAdsValues(padsvalues, dwnumvalues)
}
#[inline]
pub unsafe fn AdsTypeToPropVariant(padsvalues: *mut ADSVALUE, dwnumvalues: u32, pvariant: *mut windows_core::VARIANT) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn AdsTypeToPropVariant(padsvalues : *mut ADSVALUE, dwnumvalues : u32, pvariant : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    AdsTypeToPropVariant(padsvalues, dwnumvalues, core::mem::transmute(pvariant)).ok()
}
#[inline]
pub unsafe fn AllocADsMem(cb: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("activeds.dll" "system" fn AllocADsMem(cb : u32) -> *mut core::ffi::c_void);
    AllocADsMem(cb)
}
#[inline]
pub unsafe fn AllocADsStr<P0>(pstr: P0) -> windows_core::PWSTR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn AllocADsStr(pstr : windows_core::PCWSTR) -> windows_core::PWSTR);
    AllocADsStr(pstr.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn BinarySDToSecurityDescriptor<P0, P1, P2, P3>(psecuritydescriptor: P0, pvarsec: *mut windows_core::VARIANT, pszservername: P1, username: P2, password: P3, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn BinarySDToSecurityDescriptor(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pvarsec : *mut core::mem::MaybeUninit < windows_core::VARIANT >, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    BinarySDToSecurityDescriptor(psecuritydescriptor.param().abi(), core::mem::transmute(pvarsec), pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn DsAddSidHistoryA<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: Option<*const core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsAddSidHistoryA(hds : super::super::Foundation:: HANDLE, flags : u32, srcdomain : windows_core::PCSTR, srcprincipal : windows_core::PCSTR, srcdomaincontroller : windows_core::PCSTR, srcdomaincreds : *const core::ffi::c_void, dstdomain : windows_core::PCSTR, dstprincipal : windows_core::PCSTR) -> u32);
    DsAddSidHistoryA(hds.param().abi(), flags, srcdomain.param().abi(), srcprincipal.param().abi(), srcdomaincontroller.param().abi(), core::mem::transmute(srcdomaincreds.unwrap_or(std::ptr::null())), dstdomain.param().abi(), dstprincipal.param().abi())
}
#[inline]
pub unsafe fn DsAddSidHistoryW<P0, P1, P2, P3, P4, P5>(hds: P0, flags: u32, srcdomain: P1, srcprincipal: P2, srcdomaincontroller: P3, srcdomaincreds: Option<*const core::ffi::c_void>, dstdomain: P4, dstprincipal: P5) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsAddSidHistoryW(hds : super::super::Foundation:: HANDLE, flags : u32, srcdomain : windows_core::PCWSTR, srcprincipal : windows_core::PCWSTR, srcdomaincontroller : windows_core::PCWSTR, srcdomaincreds : *const core::ffi::c_void, dstdomain : windows_core::PCWSTR, dstprincipal : windows_core::PCWSTR) -> u32);
    DsAddSidHistoryW(hds.param().abi(), flags, srcdomain.param().abi(), srcprincipal.param().abi(), srcdomaincontroller.param().abi(), core::mem::transmute(srcdomaincreds.unwrap_or(std::ptr::null())), dstdomain.param().abi(), dstprincipal.param().abi())
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesA(computername : windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PSTR) -> u32);
    DsAddressToSiteNamesA(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExA<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PSTR, subnetnames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExA(computername : windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PSTR, subnetnames : *mut *mut windows_core::PSTR) -> u32);
    DsAddressToSiteNamesExA(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PWSTR, subnetnames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExW(computername : windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PWSTR, subnetnames : *mut *mut windows_core::PWSTR) -> u32);
    DsAddressToSiteNamesExW(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames, subnetnames)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsAddressToSiteNamesW<P0>(computername: P0, socketaddresses: &[super::WinSock::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsAddressToSiteNamesW(computername : windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::WinSock:: SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PWSTR) -> u32);
    DsAddressToSiteNamesW(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames)
}
#[inline]
pub unsafe fn DsBindA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindByInstanceA<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: Option<*const windows_core::GUID>, dnsdomainname: P2, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindByInstanceA(servername : windows_core::PCSTR, annotation : windows_core::PCSTR, instanceguid : *const windows_core::GUID, dnsdomainname : windows_core::PCSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindByInstanceA(servername.param().abi(), annotation.param().abi(), core::mem::transmute(instanceguid.unwrap_or(std::ptr::null())), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), bindflags, phds)
}
#[inline]
pub unsafe fn DsBindByInstanceW<P0, P1, P2, P3>(servername: P0, annotation: P1, instanceguid: Option<*const windows_core::GUID>, dnsdomainname: P2, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P3, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindByInstanceW(servername : windows_core::PCWSTR, annotation : windows_core::PCWSTR, instanceguid : *const windows_core::GUID, dnsdomainname : windows_core::PCWSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCWSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindByInstanceW(servername.param().abi(), annotation.param().abi(), core::mem::transmute(instanceguid.unwrap_or(std::ptr::null())), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), bindflags, phds)
}
#[inline]
pub unsafe fn DsBindToISTGA<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindToISTGA(sitename : windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindToISTGA(sitename.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindToISTGW<P0>(sitename: P0, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindToISTGW(sitename : windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindToISTGW(sitename.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindWithCredA<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithCredA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : *const core::ffi::c_void, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithCredA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), phds)
}
#[inline]
pub unsafe fn DsBindWithCredW<P0, P1>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithCredW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : *const core::ffi::c_void, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithCredW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), phds)
}
#[inline]
pub unsafe fn DsBindWithSpnA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindWithSpnExA<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnExA(domaincontrollername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnExA(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), bindflags, phds)
}
#[inline]
pub unsafe fn DsBindWithSpnExW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P2, bindflags: u32, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnExW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCWSTR, bindflags : u32, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnExW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), bindflags, phds)
}
#[inline]
pub unsafe fn DsBindWithSpnW<P0, P1, P2>(domaincontrollername: P0, dnsdomainname: P1, authidentity: Option<*const core::ffi::c_void>, serviceprincipalname: P2, phds: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindWithSpnW(domaincontrollername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, authidentity : *const core::ffi::c_void, serviceprincipalname : windows_core::PCWSTR, phds : *mut super::super::Foundation:: HANDLE) -> u32);
    DsBindWithSpnW(domaincontrollername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(authidentity.unwrap_or(std::ptr::null())), serviceprincipalname.param().abi(), phds)
}
#[inline]
pub unsafe fn DsBindingSetTimeout<P0>(hds: P0, ctimeoutsecs: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsBindingSetTimeout(hds : super::super::Foundation:: HANDLE, ctimeoutsecs : u32) -> u32);
    DsBindingSetTimeout(hds.param().abi(), ctimeoutsecs)
}
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn DsBrowseForContainerA(pinfo: *mut DSBROWSEINFOA) -> i32 {
    windows_targets::link!("dsuiext.dll" "system" fn DsBrowseForContainerA(pinfo : *mut DSBROWSEINFOA) -> i32);
    DsBrowseForContainerA(pinfo)
}
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn DsBrowseForContainerW(pinfo: *mut DSBROWSEINFOW) -> i32 {
    windows_targets::link!("dsuiext.dll" "system" fn DsBrowseForContainerW(pinfo : *mut DSBROWSEINFOW) -> i32);
    DsBrowseForContainerW(pinfo)
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerA<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerA(serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, pcspnlength : *mut u32, pszspn : windows_core::PSTR) -> u32);
    DsClientMakeSpnForTargetServerA(serviceclass.param().abi(), servicename.param().abi(), pcspnlength, core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsClientMakeSpnForTargetServerW<P0, P1>(serviceclass: P0, servicename: P1, pcspnlength: *mut u32, pszspn: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsClientMakeSpnForTargetServerW(serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : windows_core::PWSTR) -> u32);
    DsClientMakeSpnForTargetServerW(serviceclass.param().abi(), servicename.param().abi(), pcspnlength, core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsCrackNamesA<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[windows_core::PCSTR], ppresult: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsCrackNamesA(hds : super::super::Foundation:: HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_core::PCSTR, ppresult : *mut *mut DS_NAME_RESULTA) -> u32);
    DsCrackNamesA(hds.param().abi(), flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[inline]
pub unsafe fn DsCrackNamesW<P0>(hds: P0, flags: DS_NAME_FLAGS, formatoffered: DS_NAME_FORMAT, formatdesired: DS_NAME_FORMAT, rpnames: &[windows_core::PCWSTR], ppresult: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsCrackNamesW(hds : super::super::Foundation:: HANDLE, flags : DS_NAME_FLAGS, formatoffered : DS_NAME_FORMAT, formatdesired : DS_NAME_FORMAT, cnames : u32, rpnames : *const windows_core::PCWSTR, ppresult : *mut *mut DS_NAME_RESULTW) -> u32);
    DsCrackNamesW(hds.param().abi(), flags, formatoffered, formatdesired, rpnames.len().try_into().unwrap(), core::mem::transmute(rpnames.as_ptr()), ppresult)
}
#[inline]
pub unsafe fn DsCrackSpn2A(pszspn: &[u8], pcserviceclass: Option<*mut u32>, serviceclass: windows_core::PSTR, pcservicename: Option<*mut u32>, servicename: windows_core::PSTR, pcinstancename: Option<*mut u32>, instancename: windows_core::PSTR, pinstanceport: Option<*mut u16>) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn2A(pszspn : windows_core::PCSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : windows_core::PSTR, pcservicename : *mut u32, servicename : windows_core::PSTR, pcinstancename : *mut u32, instancename : windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpn2A(core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), core::mem::transmute(pcserviceclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(serviceclass), core::mem::transmute(pcservicename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(servicename), core::mem::transmute(pcinstancename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(instancename), core::mem::transmute(pinstanceport.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpn2W(pszspn: &[u16], pcserviceclass: Option<*mut u32>, serviceclass: windows_core::PWSTR, pcservicename: Option<*mut u32>, servicename: windows_core::PWSTR, pcinstancename: Option<*mut u32>, instancename: windows_core::PWSTR, pinstanceport: Option<*mut u16>) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn2W(pszspn : windows_core::PCWSTR, cspn : u32, pcserviceclass : *mut u32, serviceclass : windows_core::PWSTR, pcservicename : *mut u32, servicename : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpn2W(core::mem::transmute(pszspn.as_ptr()), pszspn.len().try_into().unwrap(), core::mem::transmute(pcserviceclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(serviceclass), core::mem::transmute(pcservicename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(servicename), core::mem::transmute(pcinstancename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(instancename), core::mem::transmute(pinstanceport.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpn3W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: windows_core::PWSTR, pcinstancename: *mut u32, instancename: windows_core::PWSTR, pportnumber: *mut u16, pcdomainname: *mut u32, domainname: windows_core::PWSTR, pcrealmname: *mut u32, realmname: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn3W(pszspn : windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pportnumber : *mut u16, pcdomainname : *mut u32, domainname : windows_core::PWSTR, pcrealmname : *mut u32, realmname : windows_core::PWSTR) -> u32);
    DsCrackSpn3W(pszspn.param().abi(), cspn, pchostname, core::mem::transmute(hostname), pcinstancename, core::mem::transmute(instancename), pportnumber, pcdomainname, core::mem::transmute(domainname), pcrealmname, core::mem::transmute(realmname))
}
#[inline]
pub unsafe fn DsCrackSpn4W<P0>(pszspn: P0, cspn: u32, pchostname: *mut u32, hostname: windows_core::PWSTR, pcinstancename: *mut u32, instancename: windows_core::PWSTR, pcportname: *mut u32, portname: windows_core::PWSTR, pcdomainname: *mut u32, domainname: windows_core::PWSTR, pcrealmname: *mut u32, realmname: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpn4W(pszspn : windows_core::PCWSTR, cspn : u32, pchostname : *mut u32, hostname : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pcportname : *mut u32, portname : windows_core::PWSTR, pcdomainname : *mut u32, domainname : windows_core::PWSTR, pcrealmname : *mut u32, realmname : windows_core::PWSTR) -> u32);
    DsCrackSpn4W(pszspn.param().abi(), cspn, pchostname, core::mem::transmute(hostname), pcinstancename, core::mem::transmute(instancename), pcportname, core::mem::transmute(portname), pcdomainname, core::mem::transmute(domainname), pcrealmname, core::mem::transmute(realmname))
}
#[inline]
pub unsafe fn DsCrackSpnA<P0>(pszspn: P0, pcserviceclass: Option<*mut u32>, serviceclass: windows_core::PSTR, pcservicename: Option<*mut u32>, servicename: windows_core::PSTR, pcinstancename: Option<*mut u32>, instancename: windows_core::PSTR, pinstanceport: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpnA(pszspn : windows_core::PCSTR, pcserviceclass : *mut u32, serviceclass : windows_core::PSTR, pcservicename : *mut u32, servicename : windows_core::PSTR, pcinstancename : *mut u32, instancename : windows_core::PSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpnA(pszspn.param().abi(), core::mem::transmute(pcserviceclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(serviceclass), core::mem::transmute(pcservicename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(servicename), core::mem::transmute(pcinstancename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(instancename), core::mem::transmute(pinstanceport.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackSpnW<P0>(pszspn: P0, pcserviceclass: Option<*mut u32>, serviceclass: windows_core::PWSTR, pcservicename: Option<*mut u32>, servicename: windows_core::PWSTR, pcinstancename: Option<*mut u32>, instancename: windows_core::PWSTR, pinstanceport: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsCrackSpnW(pszspn : windows_core::PCWSTR, pcserviceclass : *mut u32, serviceclass : windows_core::PWSTR, pcservicename : *mut u32, servicename : windows_core::PWSTR, pcinstancename : *mut u32, instancename : windows_core::PWSTR, pinstanceport : *mut u16) -> u32);
    DsCrackSpnW(pszspn.param().abi(), core::mem::transmute(pcserviceclass.unwrap_or(std::ptr::null_mut())), core::mem::transmute(serviceclass), core::mem::transmute(pcservicename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(servicename), core::mem::transmute(pcinstancename.unwrap_or(std::ptr::null_mut())), core::mem::transmute(instancename), core::mem::transmute(pinstanceport.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnA(pszrdn: &[u8], pguid: Option<*mut windows_core::GUID>, pedsmanglefor: Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("dsparse.dll" "system" fn DsCrackUnquotedMangledRdnA(pszrdn : windows_core::PCSTR, cchrdn : u32, pguid : *mut windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsCrackUnquotedMangledRdnA(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), core::mem::transmute(pguid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pedsmanglefor.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsCrackUnquotedMangledRdnW(pszrdn: &[u16], pguid: Option<*mut windows_core::GUID>, pedsmanglefor: Option<*mut DS_MANGLE_FOR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("dsparse.dll" "system" fn DsCrackUnquotedMangledRdnW(pszrdn : windows_core::PCWSTR, cchrdn : u32, pguid : *mut windows_core::GUID, pedsmanglefor : *mut DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsCrackUnquotedMangledRdnW(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), core::mem::transmute(pguid.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pedsmanglefor.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsA<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: Option<*const windows_core::GUID>, dsaguid: Option<*const windows_core::GUID>, dnshostname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsA(servername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, domainguid : *const windows_core::GUID, dsaguid : *const windows_core::GUID, dnshostname : windows_core::PCSTR) -> u32);
    DsDeregisterDnsHostRecordsA(servername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), core::mem::transmute(dsaguid.unwrap_or(std::ptr::null())), dnshostname.param().abi())
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsW<P0, P1, P2>(servername: P0, dnsdomainname: P1, domainguid: Option<*const windows_core::GUID>, dsaguid: Option<*const windows_core::GUID>, dnshostname: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsW(servername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, domainguid : *const windows_core::GUID, dsaguid : *const windows_core::GUID, dnshostname : windows_core::PCWSTR) -> u32);
    DsDeregisterDnsHostRecordsW(servername.param().abi(), dnsdomainname.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), core::mem::transmute(dsaguid.unwrap_or(std::ptr::null())), dnshostname.param().abi())
}
#[inline]
pub unsafe fn DsEnumerateDomainTrustsA<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsA(servername : windows_core::PCSTR, flags : u32, domains : *mut *mut DS_DOMAIN_TRUSTSA, domaincount : *mut u32) -> u32);
    DsEnumerateDomainTrustsA(servername.param().abi(), flags, domains, domaincount)
}
#[inline]
pub unsafe fn DsEnumerateDomainTrustsW<P0>(servername: P0, flags: u32, domains: *mut *mut DS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsW(servername : windows_core::PCWSTR, flags : u32, domains : *mut *mut DS_DOMAIN_TRUSTSW, domaincount : *mut u32) -> u32);
    DsEnumerateDomainTrustsW(servername.param().abi(), flags, domains, domaincount)
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoA(infolevel: u32, pinfo: &[u8]) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoA(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
    DsFreeDomainControllerInfoA(infolevel, pinfo.len().try_into().unwrap(), core::mem::transmute(pinfo.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeDomainControllerInfoW(infolevel: u32, pinfo: &[u8]) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeDomainControllerInfoW(infolevel : u32, cinfo : u32, pinfo : *const core::ffi::c_void));
    DsFreeDomainControllerInfoW(infolevel, pinfo.len().try_into().unwrap(), core::mem::transmute(pinfo.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeNameResultA(presult: *const DS_NAME_RESULTA) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeNameResultA(presult : *const DS_NAME_RESULTA));
    DsFreeNameResultA(presult)
}
#[inline]
pub unsafe fn DsFreeNameResultW(presult: *const DS_NAME_RESULTW) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeNameResultW(presult : *const DS_NAME_RESULTW));
    DsFreeNameResultW(presult)
}
#[inline]
pub unsafe fn DsFreePasswordCredentials(authidentity: *const core::ffi::c_void) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreePasswordCredentials(authidentity : *const core::ffi::c_void));
    DsFreePasswordCredentials(authidentity)
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapA(pguidmap: *const DS_SCHEMA_GUID_MAPA) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapA(pguidmap : *const DS_SCHEMA_GUID_MAPA));
    DsFreeSchemaGuidMapA(pguidmap)
}
#[inline]
pub unsafe fn DsFreeSchemaGuidMapW(pguidmap: *const DS_SCHEMA_GUID_MAPW) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSchemaGuidMapW(pguidmap : *const DS_SCHEMA_GUID_MAPW));
    DsFreeSchemaGuidMapW(pguidmap)
}
#[inline]
pub unsafe fn DsFreeSpnArrayA(rpszspn: &mut [windows_core::PSTR]) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayA(cspn : u32, rpszspn : *mut windows_core::PSTR));
    DsFreeSpnArrayA(rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr()))
}
#[inline]
pub unsafe fn DsFreeSpnArrayW(rpszspn: &mut [windows_core::PWSTR]) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsFreeSpnArrayW(cspn : u32, rpszspn : *mut windows_core::PWSTR));
    DsFreeSpnArrayW(rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr()))
}
#[inline]
pub unsafe fn DsGetDcCloseW<P0>(getdccontexthandle: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcCloseW(getdccontexthandle : super::super::Foundation:: HANDLE));
    DsGetDcCloseW(getdccontexthandle.param().abi())
}
#[inline]
pub unsafe fn DsGetDcNameA<P0, P1, P2>(computername: P0, domainname: P1, domainguid: Option<*const windows_core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcNameA(computername : windows_core::PCSTR, domainname : windows_core::PCSTR, domainguid : *const windows_core::GUID, sitename : windows_core::PCSTR, flags : u32, domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOA) -> u32);
    DsGetDcNameA(computername.param().abi(), domainname.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), sitename.param().abi(), flags, domaincontrollerinfo)
}
#[inline]
pub unsafe fn DsGetDcNameW<P0, P1, P2>(computername: P0, domainname: P1, domainguid: Option<*const windows_core::GUID>, sitename: P2, flags: u32, domaincontrollerinfo: *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcNameW(computername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, domainguid : *const windows_core::GUID, sitename : windows_core::PCWSTR, flags : u32, domaincontrollerinfo : *mut *mut DOMAIN_CONTROLLER_INFOW) -> u32);
    DsGetDcNameW(computername.param().abi(), domainname.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), sitename.param().abi(), flags, domaincontrollerinfo)
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsGetDcNextA<P0>(getdccontexthandle: P0, sockaddresscount: Option<*mut u32>, sockaddresses: Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: Option<*mut windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcNextA(getdccontexthandle : super::super::Foundation:: HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS, dnshostname : *mut windows_core::PSTR) -> u32);
    DsGetDcNextA(getdccontexthandle.param().abi(), core::mem::transmute(sockaddresscount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sockaddresses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(dnshostname.unwrap_or(std::ptr::null_mut())))
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn DsGetDcNextW<P0>(getdccontexthandle: P0, sockaddresscount: Option<*mut u32>, sockaddresses: Option<*mut *mut super::WinSock::SOCKET_ADDRESS>, dnshostname: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcNextW(getdccontexthandle : super::super::Foundation:: HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut *mut super::WinSock:: SOCKET_ADDRESS, dnshostname : *mut windows_core::PWSTR) -> u32);
    DsGetDcNextW(getdccontexthandle.param().abi(), core::mem::transmute(sockaddresscount.unwrap_or(std::ptr::null_mut())), core::mem::transmute(sockaddresses.unwrap_or(std::ptr::null_mut())), core::mem::transmute(dnshostname.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsGetDcOpenA<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: Option<*const windows_core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcOpenA(dnsname : windows_core::PCSTR, optionflags : u32, sitename : windows_core::PCSTR, domainguid : *const windows_core::GUID, dnsforestname : windows_core::PCSTR, dcflags : u32, retgetdccontext : *mut super::super::Foundation:: HANDLE) -> u32);
    DsGetDcOpenA(dnsname.param().abi(), optionflags, sitename.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), dnsforestname.param().abi(), dcflags, retgetdccontext)
}
#[inline]
pub unsafe fn DsGetDcOpenW<P0, P1, P2>(dnsname: P0, optionflags: u32, sitename: P1, domainguid: Option<*const windows_core::GUID>, dnsforestname: P2, dcflags: u32, retgetdccontext: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcOpenW(dnsname : windows_core::PCWSTR, optionflags : u32, sitename : windows_core::PCWSTR, domainguid : *const windows_core::GUID, dnsforestname : windows_core::PCWSTR, dcflags : u32, retgetdccontext : *mut super::super::Foundation:: HANDLE) -> u32);
    DsGetDcOpenW(dnsname.param().abi(), optionflags, sitename.param().abi(), core::mem::transmute(domainguid.unwrap_or(std::ptr::null())), dnsforestname.param().abi(), dcflags, retgetdccontext)
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageA<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageA(servername : windows_core::PCSTR, entrycount : *mut u32, sitenames : *mut *mut windows_core::PSTR) -> u32);
    DsGetDcSiteCoverageA(servername.param().abi(), entrycount, sitenames)
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageW<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageW(servername : windows_core::PCWSTR, entrycount : *mut u32, sitenames : *mut *mut windows_core::PWSTR) -> u32);
    DsGetDcSiteCoverageW(servername.param().abi(), entrycount, sitenames)
}
#[inline]
pub unsafe fn DsGetDomainControllerInfoA<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoA(hds : super::super::Foundation:: HANDLE, domainname : windows_core::PCSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    DsGetDomainControllerInfoA(hds.param().abi(), domainname.param().abi(), infolevel, pcout, ppinfo)
}
#[inline]
pub unsafe fn DsGetDomainControllerInfoW<P0, P1>(hds: P0, domainname: P1, infolevel: u32, pcout: *mut u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsGetDomainControllerInfoW(hds : super::super::Foundation:: HANDLE, domainname : windows_core::PCWSTR, infolevel : u32, pcout : *mut u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    DsGetDomainControllerInfoW(hds.param().abi(), domainname.param().abi(), infolevel, pcout, ppinfo)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn DsGetForestTrustInformationW<P0, P1>(servername: P0, trusteddomainname: P1, flags: u32, foresttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetForestTrustInformationW(servername : windows_core::PCWSTR, trusteddomainname : windows_core::PCWSTR, flags : u32, foresttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION) -> u32);
    DsGetForestTrustInformationW(servername.param().abi(), trusteddomainname.param().abi(), flags, foresttrustinfo)
}
#[inline]
pub unsafe fn DsGetFriendlyClassName<P0>(pszobjectclass: P0, pszbuffer: &mut [u16]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsuiext.dll" "system" fn DsGetFriendlyClassName(pszobjectclass : windows_core::PCWSTR, pszbuffer : windows_core::PWSTR, cchbuffer : u32) -> windows_core::HRESULT);
    DsGetFriendlyClassName(pszobjectclass.param().abi(), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn DsGetIcon<P0>(dwflags: u32, pszobjectclass: P0, cximage: i32, cyimage: i32) -> super::super::UI::WindowsAndMessaging::HICON
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsuiext.dll" "system" fn DsGetIcon(dwflags : u32, pszobjectclass : windows_core::PCWSTR, cximage : i32, cyimage : i32) -> super::super::UI::WindowsAndMessaging:: HICON);
    DsGetIcon(dwflags, pszobjectclass.param().abi(), cximage, cyimage)
}
#[inline]
pub unsafe fn DsGetRdnW(ppdn: *mut windows_core::PWSTR, pcdn: *mut u32, ppkey: *mut windows_core::PWSTR, pckey: *mut u32, ppval: *mut windows_core::PWSTR, pcval: *mut u32) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsGetRdnW(ppdn : *mut windows_core::PWSTR, pcdn : *mut u32, ppkey : *mut windows_core::PWSTR, pckey : *mut u32, ppval : *mut windows_core::PWSTR, pcval : *mut u32) -> u32);
    DsGetRdnW(ppdn, pcdn, ppkey, pckey, ppval, pcval)
}
#[inline]
pub unsafe fn DsGetSiteNameA<P0>(computername: P0, sitename: *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetSiteNameA(computername : windows_core::PCSTR, sitename : *mut windows_core::PSTR) -> u32);
    DsGetSiteNameA(computername.param().abi(), sitename)
}
#[inline]
pub unsafe fn DsGetSiteNameW<P0>(computername: P0, sitename: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsGetSiteNameW(computername : windows_core::PCWSTR, sitename : *mut windows_core::PWSTR) -> u32);
    DsGetSiteNameW(computername.param().abi(), sitename)
}
#[inline]
pub unsafe fn DsGetSpnA<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: Option<*const windows_core::PCSTR>, pinstanceports: Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsGetSpnA(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_core::PCSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_core::PSTR) -> u32);
    DsGetSpnA(servicetype, serviceclass.param().abi(), servicename.param().abi(), instanceport, cinstancenames, core::mem::transmute(pinstancenames.unwrap_or(std::ptr::null())), core::mem::transmute(pinstanceports.unwrap_or(std::ptr::null())), pcspn, prpszspn)
}
#[inline]
pub unsafe fn DsGetSpnW<P0, P1>(servicetype: DS_SPN_NAME_TYPE, serviceclass: P0, servicename: P1, instanceport: u16, cinstancenames: u16, pinstancenames: Option<*const windows_core::PCWSTR>, pinstanceports: Option<*const u16>, pcspn: *mut u32, prpszspn: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsGetSpnW(servicetype : DS_SPN_NAME_TYPE, serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, instanceport : u16, cinstancenames : u16, pinstancenames : *const windows_core::PCWSTR, pinstanceports : *const u16, pcspn : *mut u32, prpszspn : *mut *mut windows_core::PWSTR) -> u32);
    DsGetSpnW(servicetype, serviceclass.param().abi(), servicename.param().abi(), instanceport, cinstancenames, core::mem::transmute(pinstancenames.unwrap_or(std::ptr::null())), core::mem::transmute(pinstanceports.unwrap_or(std::ptr::null())), pcspn, prpszspn)
}
#[inline]
pub unsafe fn DsInheritSecurityIdentityA<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityA(hds : super::super::Foundation:: HANDLE, flags : u32, srcprincipal : windows_core::PCSTR, dstprincipal : windows_core::PCSTR) -> u32);
    DsInheritSecurityIdentityA(hds.param().abi(), flags, srcprincipal.param().abi(), dstprincipal.param().abi())
}
#[inline]
pub unsafe fn DsInheritSecurityIdentityW<P0, P1, P2>(hds: P0, flags: u32, srcprincipal: P1, dstprincipal: P2) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsInheritSecurityIdentityW(hds : super::super::Foundation:: HANDLE, flags : u32, srcprincipal : windows_core::PCWSTR, dstprincipal : windows_core::PCWSTR) -> u32);
    DsInheritSecurityIdentityW(hds.param().abi(), flags, srcprincipal.param().abi(), dstprincipal.param().abi())
}
#[inline]
pub unsafe fn DsIsMangledDnA<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsIsMangledDnA(pszdn : windows_core::PCSTR, edsmanglefor : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledDnA(pszdn.param().abi(), edsmanglefor)
}
#[inline]
pub unsafe fn DsIsMangledDnW<P0>(pszdn: P0, edsmanglefor: DS_MANGLE_FOR) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsIsMangledDnW(pszdn : windows_core::PCWSTR, edsmanglefor : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledDnW(pszdn.param().abi(), edsmanglefor)
}
#[inline]
pub unsafe fn DsIsMangledRdnValueA(pszrdn: &[u8], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    windows_targets::link!("dsparse.dll" "system" fn DsIsMangledRdnValueA(pszrdn : windows_core::PCSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledRdnValueA(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired)
}
#[inline]
pub unsafe fn DsIsMangledRdnValueW(pszrdn: &[u16], edsmanglefordesired: DS_MANGLE_FOR) -> super::super::Foundation::BOOL {
    windows_targets::link!("dsparse.dll" "system" fn DsIsMangledRdnValueW(pszrdn : windows_core::PCWSTR, crdn : u32, edsmanglefordesired : DS_MANGLE_FOR) -> super::super::Foundation:: BOOL);
    DsIsMangledRdnValueW(core::mem::transmute(pszrdn.as_ptr()), pszrdn.len().try_into().unwrap(), edsmanglefordesired)
}
#[inline]
pub unsafe fn DsListDomainsInSiteA<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteA(hds : super::super::Foundation:: HANDLE, site : windows_core::PCSTR, ppdomains : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListDomainsInSiteA(hds.param().abi(), site.param().abi(), ppdomains)
}
#[inline]
pub unsafe fn DsListDomainsInSiteW<P0, P1>(hds: P0, site: P1, ppdomains: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListDomainsInSiteW(hds : super::super::Foundation:: HANDLE, site : windows_core::PCWSTR, ppdomains : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListDomainsInSiteW(hds.param().abi(), site.param().abi(), ppdomains)
}
#[inline]
pub unsafe fn DsListInfoForServerA<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListInfoForServerA(hds : super::super::Foundation:: HANDLE, server : windows_core::PCSTR, ppinfo : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListInfoForServerA(hds.param().abi(), server.param().abi(), ppinfo)
}
#[inline]
pub unsafe fn DsListInfoForServerW<P0, P1>(hds: P0, server: P1, ppinfo: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListInfoForServerW(hds : super::super::Foundation:: HANDLE, server : windows_core::PCWSTR, ppinfo : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListInfoForServerW(hds.param().abi(), server.param().abi(), ppinfo)
}
#[inline]
pub unsafe fn DsListRolesA<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListRolesA(hds : super::super::Foundation:: HANDLE, pproles : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListRolesA(hds.param().abi(), pproles)
}
#[inline]
pub unsafe fn DsListRolesW<P0>(hds: P0, pproles: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListRolesW(hds : super::super::Foundation:: HANDLE, pproles : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListRolesW(hds.param().abi(), pproles)
}
#[inline]
pub unsafe fn DsListServersForDomainInSiteA<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteA(hds : super::super::Foundation:: HANDLE, domain : windows_core::PCSTR, site : windows_core::PCSTR, ppservers : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListServersForDomainInSiteA(hds.param().abi(), domain.param().abi(), site.param().abi(), ppservers)
}
#[inline]
pub unsafe fn DsListServersForDomainInSiteW<P0, P1, P2>(hds: P0, domain: P1, site: P2, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListServersForDomainInSiteW(hds : super::super::Foundation:: HANDLE, domain : windows_core::PCWSTR, site : windows_core::PCWSTR, ppservers : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListServersForDomainInSiteW(hds.param().abi(), domain.param().abi(), site.param().abi(), ppservers)
}
#[inline]
pub unsafe fn DsListServersInSiteA<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListServersInSiteA(hds : super::super::Foundation:: HANDLE, site : windows_core::PCSTR, ppservers : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListServersInSiteA(hds.param().abi(), site.param().abi(), ppservers)
}
#[inline]
pub unsafe fn DsListServersInSiteW<P0, P1>(hds: P0, site: P1, ppservers: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListServersInSiteW(hds : super::super::Foundation:: HANDLE, site : windows_core::PCWSTR, ppservers : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListServersInSiteW(hds.param().abi(), site.param().abi(), ppservers)
}
#[inline]
pub unsafe fn DsListSitesA<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListSitesA(hds : super::super::Foundation:: HANDLE, ppsites : *mut *mut DS_NAME_RESULTA) -> u32);
    DsListSitesA(hds.param().abi(), ppsites)
}
#[inline]
pub unsafe fn DsListSitesW<P0>(hds: P0, ppsites: *mut *mut DS_NAME_RESULTW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsListSitesW(hds : super::super::Foundation:: HANDLE, ppsites : *mut *mut DS_NAME_RESULTW) -> u32);
    DsListSitesW(hds.param().abi(), ppsites)
}
#[inline]
pub unsafe fn DsMakePasswordCredentialsA<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsA(user : windows_core::PCSTR, domain : windows_core::PCSTR, password : windows_core::PCSTR, pauthidentity : *mut *mut core::ffi::c_void) -> u32);
    DsMakePasswordCredentialsA(user.param().abi(), domain.param().abi(), password.param().abi(), pauthidentity)
}
#[inline]
pub unsafe fn DsMakePasswordCredentialsW<P0, P1, P2>(user: P0, domain: P1, password: P2, pauthidentity: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsMakePasswordCredentialsW(user : windows_core::PCWSTR, domain : windows_core::PCWSTR, password : windows_core::PCWSTR, pauthidentity : *mut *mut core::ffi::c_void) -> u32);
    DsMakePasswordCredentialsW(user.param().abi(), domain.param().abi(), password.param().abi(), pauthidentity)
}
#[inline]
pub unsafe fn DsMakeSpnA<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsMakeSpnA(serviceclass : windows_core::PCSTR, servicename : windows_core::PCSTR, instancename : windows_core::PCSTR, instanceport : u16, referrer : windows_core::PCSTR, pcspnlength : *mut u32, pszspn : windows_core::PSTR) -> u32);
    DsMakeSpnA(serviceclass.param().abi(), servicename.param().abi(), instancename.param().abi(), instanceport, referrer.param().abi(), pcspnlength, core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsMakeSpnW<P0, P1, P2, P3>(serviceclass: P0, servicename: P1, instancename: P2, instanceport: u16, referrer: P3, pcspnlength: *mut u32, pszspn: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("dsparse.dll" "system" fn DsMakeSpnW(serviceclass : windows_core::PCWSTR, servicename : windows_core::PCWSTR, instancename : windows_core::PCWSTR, instanceport : u16, referrer : windows_core::PCWSTR, pcspnlength : *mut u32, pszspn : windows_core::PWSTR) -> u32);
    DsMakeSpnW(serviceclass.param().abi(), servicename.param().abi(), instancename.param().abi(), instanceport, referrer.param().abi(), pcspnlength, core::mem::transmute(pszspn))
}
#[inline]
pub unsafe fn DsMapSchemaGuidsA<P0>(hds: P0, rguids: &[windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPA) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsA(hds : super::super::Foundation:: HANDLE, cguids : u32, rguids : *const windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPA) -> u32);
    DsMapSchemaGuidsA(hds.param().abi(), rguids.len().try_into().unwrap(), core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[inline]
pub unsafe fn DsMapSchemaGuidsW<P0>(hds: P0, rguids: &[windows_core::GUID], ppguidmap: *mut *mut DS_SCHEMA_GUID_MAPW) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsMapSchemaGuidsW(hds : super::super::Foundation:: HANDLE, cguids : u32, rguids : *const windows_core::GUID, ppguidmap : *mut *mut DS_SCHEMA_GUID_MAPW) -> u32);
    DsMapSchemaGuidsW(hds.param().abi(), rguids.len().try_into().unwrap(), core::mem::transmute(rguids.as_ptr()), ppguidmap)
}
#[cfg(feature = "Win32_Security_Authentication_Identity")]
#[inline]
pub unsafe fn DsMergeForestTrustInformationW<P0>(domainname: P0, newforesttrustinfo: *const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: Option<*const super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION>, mergedforesttrustinfo: *mut *mut super::super::Security::Authentication::Identity::LSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsMergeForestTrustInformationW(domainname : windows_core::PCWSTR, newforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo : *const super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo : *mut *mut super::super::Security::Authentication::Identity:: LSA_FOREST_TRUST_INFORMATION) -> u32);
    DsMergeForestTrustInformationW(domainname.param().abi(), newforesttrustinfo, core::mem::transmute(oldforesttrustinfo.unwrap_or(std::ptr::null())), mergedforesttrustinfo)
}
#[inline]
pub unsafe fn DsQuerySitesByCostA<P0, P1>(hds: P0, pszfromsite: P1, rgsztosites: &[windows_core::PCSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostA(hds : super::super::Foundation:: HANDLE, pszfromsite : windows_core::PCSTR, rgsztosites : *const windows_core::PCSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut *mut DS_SITE_COST_INFO) -> u32);
    DsQuerySitesByCostA(hds.param().abi(), pszfromsite.param().abi(), core::mem::transmute(rgsztosites.as_ptr()), rgsztosites.len().try_into().unwrap(), dwflags, prgsiteinfo)
}
#[inline]
pub unsafe fn DsQuerySitesByCostW<P0, P1>(hds: P0, pwszfromsite: P1, rgwsztosites: &[windows_core::PCWSTR], dwflags: u32, prgsiteinfo: *mut *mut DS_SITE_COST_INFO) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesByCostW(hds : super::super::Foundation:: HANDLE, pwszfromsite : windows_core::PCWSTR, rgwsztosites : *const windows_core::PCWSTR, ctosites : u32, dwflags : u32, prgsiteinfo : *mut *mut DS_SITE_COST_INFO) -> u32);
    DsQuerySitesByCostW(hds.param().abi(), pwszfromsite.param().abi(), core::mem::transmute(rgwsztosites.as_ptr()), rgwsztosites.len().try_into().unwrap(), dwflags, prgsiteinfo)
}
#[inline]
pub unsafe fn DsQuerySitesFree(rgsiteinfo: *const DS_SITE_COST_INFO) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsQuerySitesFree(rgsiteinfo : *const DS_SITE_COST_INFO));
    DsQuerySitesFree(rgsiteinfo)
}
#[inline]
pub unsafe fn DsQuoteRdnValueA(psunquotedrdnvalue: &[u8], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: windows_core::PSTR) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsQuoteRdnValueA(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : windows_core::PCSTR, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : windows_core::PSTR) -> u32);
    DsQuoteRdnValueA(psunquotedrdnvalue.len().try_into().unwrap(), core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, core::mem::transmute(psquotedrdnvalue))
}
#[inline]
pub unsafe fn DsQuoteRdnValueW(psunquotedrdnvalue: &[u16], pcquotedrdnvaluelength: *mut u32, psquotedrdnvalue: windows_core::PWSTR) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsQuoteRdnValueW(cunquotedrdnvaluelength : u32, psunquotedrdnvalue : windows_core::PCWSTR, pcquotedrdnvaluelength : *mut u32, psquotedrdnvalue : windows_core::PWSTR) -> u32);
    DsQuoteRdnValueW(psunquotedrdnvalue.len().try_into().unwrap(), core::mem::transmute(psunquotedrdnvalue.as_ptr()), pcquotedrdnvaluelength, core::mem::transmute(psquotedrdnvalue))
}
#[inline]
pub unsafe fn DsRemoveDsDomainA<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainA(hds : super::super::Foundation:: HANDLE, domaindn : windows_core::PCSTR) -> u32);
    DsRemoveDsDomainA(hds.param().abi(), domaindn.param().abi())
}
#[inline]
pub unsafe fn DsRemoveDsDomainW<P0, P1>(hds: P0, domaindn: P1) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsDomainW(hds : super::super::Foundation:: HANDLE, domaindn : windows_core::PCWSTR) -> u32);
    DsRemoveDsDomainW(hds.param().abi(), domaindn.param().abi())
}
#[inline]
pub unsafe fn DsRemoveDsServerA<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsServerA(hds : super::super::Foundation:: HANDLE, serverdn : windows_core::PCSTR, domaindn : windows_core::PCSTR, flastdcindomain : *mut super::super::Foundation:: BOOL, fcommit : super::super::Foundation:: BOOL) -> u32);
    DsRemoveDsServerA(hds.param().abi(), serverdn.param().abi(), domaindn.param().abi(), core::mem::transmute(flastdcindomain.unwrap_or(std::ptr::null_mut())), fcommit.param().abi())
}
#[inline]
pub unsafe fn DsRemoveDsServerW<P0, P1, P2, P3>(hds: P0, serverdn: P1, domaindn: P2, flastdcindomain: Option<*mut super::super::Foundation::BOOL>, fcommit: P3) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsRemoveDsServerW(hds : super::super::Foundation:: HANDLE, serverdn : windows_core::PCWSTR, domaindn : windows_core::PCWSTR, flastdcindomain : *mut super::super::Foundation:: BOOL, fcommit : super::super::Foundation:: BOOL) -> u32);
    DsRemoveDsServerW(hds.param().abi(), serverdn.param().abi(), domaindn.param().abi(), core::mem::transmute(flastdcindomain.unwrap_or(std::ptr::null_mut())), fcommit.param().abi())
}
#[inline]
pub unsafe fn DsReplicaAddA<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaAddA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, sourcedsadn : windows_core::PCSTR, transportdn : windows_core::PCSTR, sourcedsaaddress : windows_core::PCSTR, pschedule : *const SCHEDULE, options : u32) -> u32);
    DsReplicaAddA(hds.param().abi(), namecontext.param().abi(), sourcedsadn.param().abi(), transportdn.param().abi(), sourcedsaaddress.param().abi(), core::mem::transmute(pschedule.unwrap_or(std::ptr::null())), options)
}
#[inline]
pub unsafe fn DsReplicaAddW<P0, P1, P2, P3, P4>(hds: P0, namecontext: P1, sourcedsadn: P2, transportdn: P3, sourcedsaaddress: P4, pschedule: Option<*const SCHEDULE>, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaAddW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, sourcedsadn : windows_core::PCWSTR, transportdn : windows_core::PCWSTR, sourcedsaaddress : windows_core::PCWSTR, pschedule : *const SCHEDULE, options : u32) -> u32);
    DsReplicaAddW(hds.param().abi(), namecontext.param().abi(), sourcedsadn.param().abi(), transportdn.param().abi(), sourcedsaaddress.param().abi(), core::mem::transmute(pschedule.unwrap_or(std::ptr::null())), options)
}
#[inline]
pub unsafe fn DsReplicaConsistencyCheck<P0>(hds: P0, taskid: DS_KCC_TASKID, dwflags: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaConsistencyCheck(hds : super::super::Foundation:: HANDLE, taskid : DS_KCC_TASKID, dwflags : u32) -> u32);
    DsReplicaConsistencyCheck(hds.param().abi(), taskid, dwflags)
}
#[inline]
pub unsafe fn DsReplicaDelA<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaDelA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, dsasrc : windows_core::PCSTR, options : u32) -> u32);
    DsReplicaDelA(hds.param().abi(), namecontext.param().abi(), dsasrc.param().abi(), options)
}
#[inline]
pub unsafe fn DsReplicaDelW<P0, P1, P2>(hds: P0, namecontext: P1, dsasrc: P2, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaDelW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, dsasrc : windows_core::PCWSTR, options : u32) -> u32);
    DsReplicaDelW(hds.param().abi(), namecontext.param().abi(), dsasrc.param().abi(), options)
}
#[inline]
pub unsafe fn DsReplicaFreeInfo(infotype: DS_REPL_INFO_TYPE, pinfo: *const core::ffi::c_void) {
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaFreeInfo(infotype : DS_REPL_INFO_TYPE, pinfo : *const core::ffi::c_void));
    DsReplicaFreeInfo(infotype, pinfo)
}
#[inline]
pub unsafe fn DsReplicaGetInfo2W<P0, P1, P2, P3>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: Option<*const windows_core::GUID>, pszattributename: P2, pszvalue: P3, dwflags: u32, dwenumerationcontext: u32, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaGetInfo2W(hds : super::super::Foundation:: HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_core::PCWSTR, puuidforsourcedsaobjguid : *const windows_core::GUID, pszattributename : windows_core::PCWSTR, pszvalue : windows_core::PCWSTR, dwflags : u32, dwenumerationcontext : u32, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    DsReplicaGetInfo2W(hds.param().abi(), infotype, pszobject.param().abi(), core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(std::ptr::null())), pszattributename.param().abi(), pszvalue.param().abi(), dwflags, dwenumerationcontext, ppinfo)
}
#[inline]
pub unsafe fn DsReplicaGetInfoW<P0, P1>(hds: P0, infotype: DS_REPL_INFO_TYPE, pszobject: P1, puuidforsourcedsaobjguid: Option<*const windows_core::GUID>, ppinfo: *mut *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaGetInfoW(hds : super::super::Foundation:: HANDLE, infotype : DS_REPL_INFO_TYPE, pszobject : windows_core::PCWSTR, puuidforsourcedsaobjguid : *const windows_core::GUID, ppinfo : *mut *mut core::ffi::c_void) -> u32);
    DsReplicaGetInfoW(hds.param().abi(), infotype, pszobject.param().abi(), core::mem::transmute(puuidforsourcedsaobjguid.unwrap_or(std::ptr::null())), ppinfo)
}
#[inline]
pub unsafe fn DsReplicaModifyA<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: Option<*const windows_core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaModifyA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, puuidsourcedsa : *const windows_core::GUID, transportdn : windows_core::PCSTR, sourcedsaaddress : windows_core::PCSTR, pschedule : *const SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    DsReplicaModifyA(hds.param().abi(), namecontext.param().abi(), core::mem::transmute(puuidsourcedsa.unwrap_or(std::ptr::null())), transportdn.param().abi(), sourcedsaaddress.param().abi(), core::mem::transmute(pschedule.unwrap_or(std::ptr::null())), replicaflags, modifyfields, options)
}
#[inline]
pub unsafe fn DsReplicaModifyW<P0, P1, P2, P3>(hds: P0, namecontext: P1, puuidsourcedsa: Option<*const windows_core::GUID>, transportdn: P2, sourcedsaaddress: P3, pschedule: Option<*const SCHEDULE>, replicaflags: u32, modifyfields: u32, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaModifyW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, puuidsourcedsa : *const windows_core::GUID, transportdn : windows_core::PCWSTR, sourcedsaaddress : windows_core::PCWSTR, pschedule : *const SCHEDULE, replicaflags : u32, modifyfields : u32, options : u32) -> u32);
    DsReplicaModifyW(hds.param().abi(), namecontext.param().abi(), core::mem::transmute(puuidsourcedsa.unwrap_or(std::ptr::null())), transportdn.param().abi(), sourcedsaaddress.param().abi(), core::mem::transmute(pschedule.unwrap_or(std::ptr::null())), replicaflags, modifyfields, options)
}
#[inline]
pub unsafe fn DsReplicaSyncA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const windows_core::GUID, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, puuiddsasrc : *const windows_core::GUID, options : u32) -> u32);
    DsReplicaSyncA(hds.param().abi(), namecontext.param().abi(), puuiddsasrc, options)
}
#[inline]
pub unsafe fn DsReplicaSyncAllA<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: Option<*const core::ffi::c_void>, perrors: Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOA>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllA(hds : super::super::Foundation:: HANDLE, psznamecontext : windows_core::PCSTR, ulflags : u32, pfncallback : isize, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOA) -> u32);
    DsReplicaSyncAllA(hds.param().abi(), psznamecontext.param().abi(), ulflags, pfncallback, core::mem::transmute(pcallbackdata.unwrap_or(std::ptr::null())), core::mem::transmute(perrors.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsReplicaSyncAllW<P0, P1>(hds: P0, psznamecontext: P1, ulflags: u32, pfncallback: isize, pcallbackdata: Option<*const core::ffi::c_void>, perrors: Option<*mut *mut *mut DS_REPSYNCALL_ERRINFOW>) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncAllW(hds : super::super::Foundation:: HANDLE, psznamecontext : windows_core::PCWSTR, ulflags : u32, pfncallback : isize, pcallbackdata : *const core::ffi::c_void, perrors : *mut *mut *mut DS_REPSYNCALL_ERRINFOW) -> u32);
    DsReplicaSyncAllW(hds.param().abi(), psznamecontext.param().abi(), ulflags, pfncallback, core::mem::transmute(pcallbackdata.unwrap_or(std::ptr::null())), core::mem::transmute(perrors.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn DsReplicaSyncW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const windows_core::GUID, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaSyncW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, puuiddsasrc : *const windows_core::GUID, options : u32) -> u32);
    DsReplicaSyncW(hds.param().abi(), namecontext.param().abi(), puuiddsasrc, options)
}
#[inline]
pub unsafe fn DsReplicaUpdateRefsA<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const windows_core::GUID, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, dsadest : windows_core::PCSTR, puuiddsadest : *const windows_core::GUID, options : u32) -> u32);
    DsReplicaUpdateRefsA(hds.param().abi(), namecontext.param().abi(), dsadest.param().abi(), puuiddsadest, options)
}
#[inline]
pub unsafe fn DsReplicaUpdateRefsW<P0, P1, P2>(hds: P0, namecontext: P1, dsadest: P2, puuiddsadest: *const windows_core::GUID, options: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaUpdateRefsW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, dsadest : windows_core::PCWSTR, puuiddsadest : *const windows_core::GUID, options : u32) -> u32);
    DsReplicaUpdateRefsW(hds.param().abi(), namecontext.param().abi(), dsadest.param().abi(), puuiddsadest, options)
}
#[inline]
pub unsafe fn DsReplicaVerifyObjectsA<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const windows_core::GUID, uloptions: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsA(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCSTR, puuiddsasrc : *const windows_core::GUID, uloptions : u32) -> u32);
    DsReplicaVerifyObjectsA(hds.param().abi(), namecontext.param().abi(), puuiddsasrc, uloptions)
}
#[inline]
pub unsafe fn DsReplicaVerifyObjectsW<P0, P1>(hds: P0, namecontext: P1, puuiddsasrc: *const windows_core::GUID, uloptions: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsReplicaVerifyObjectsW(hds : super::super::Foundation:: HANDLE, namecontext : windows_core::PCWSTR, puuiddsasrc : *const windows_core::GUID, uloptions : u32) -> u32);
    DsReplicaVerifyObjectsW(hds.param().abi(), namecontext.param().abi(), puuiddsasrc, uloptions)
}
#[inline]
pub unsafe fn DsRoleFreeMemory(buffer: *mut core::ffi::c_void) {
    windows_targets::link!("netapi32.dll" "system" fn DsRoleFreeMemory(buffer : *mut core::ffi::c_void));
    DsRoleFreeMemory(buffer)
}
#[inline]
pub unsafe fn DsRoleGetPrimaryDomainInformation<P0>(lpserver: P0, infolevel: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer: *mut *mut u8) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsRoleGetPrimaryDomainInformation(lpserver : windows_core::PCWSTR, infolevel : DSROLE_PRIMARY_DOMAIN_INFO_LEVEL, buffer : *mut *mut u8) -> u32);
    DsRoleGetPrimaryDomainInformation(lpserver.param().abi(), infolevel, buffer)
}
#[inline]
pub unsafe fn DsServerRegisterSpnA<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnA(operation : DS_SPN_WRITE_OP, serviceclass : windows_core::PCSTR, userobjectdn : windows_core::PCSTR) -> u32);
    DsServerRegisterSpnA(operation, serviceclass.param().abi(), userobjectdn.param().abi())
}
#[inline]
pub unsafe fn DsServerRegisterSpnW<P0, P1>(operation: DS_SPN_WRITE_OP, serviceclass: P0, userobjectdn: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsServerRegisterSpnW(operation : DS_SPN_WRITE_OP, serviceclass : windows_core::PCWSTR, userobjectdn : windows_core::PCWSTR) -> u32);
    DsServerRegisterSpnW(operation, serviceclass.param().abi(), userobjectdn.param().abi())
}
#[inline]
pub unsafe fn DsUnBindA(phds: *const super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ntdsapi.dll" "system" fn DsUnBindA(phds : *const super::super::Foundation:: HANDLE) -> u32);
    DsUnBindA(phds)
}
#[inline]
pub unsafe fn DsUnBindW(phds: *const super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("ntdsapi.dll" "system" fn DsUnBindW(phds : *const super::super::Foundation:: HANDLE) -> u32);
    DsUnBindW(phds)
}
#[inline]
pub unsafe fn DsUnquoteRdnValueA(psquotedrdnvalue: &[u8], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: windows_core::PSTR) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsUnquoteRdnValueA(cquotedrdnvaluelength : u32, psquotedrdnvalue : windows_core::PCSTR, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : windows_core::PSTR) -> u32);
    DsUnquoteRdnValueA(psquotedrdnvalue.len().try_into().unwrap(), core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, core::mem::transmute(psunquotedrdnvalue))
}
#[inline]
pub unsafe fn DsUnquoteRdnValueW(psquotedrdnvalue: &[u16], pcunquotedrdnvaluelength: *mut u32, psunquotedrdnvalue: windows_core::PWSTR) -> u32 {
    windows_targets::link!("dsparse.dll" "system" fn DsUnquoteRdnValueW(cquotedrdnvaluelength : u32, psquotedrdnvalue : windows_core::PCWSTR, pcunquotedrdnvaluelength : *mut u32, psunquotedrdnvalue : windows_core::PWSTR) -> u32);
    DsUnquoteRdnValueW(psquotedrdnvalue.len().try_into().unwrap(), core::mem::transmute(psquotedrdnvalue.as_ptr()), pcunquotedrdnvaluelength, core::mem::transmute(psunquotedrdnvalue))
}
#[inline]
pub unsafe fn DsValidateSubnetNameA<P0>(subnetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsValidateSubnetNameA(subnetname : windows_core::PCSTR) -> u32);
    DsValidateSubnetNameA(subnetname.param().abi())
}
#[inline]
pub unsafe fn DsValidateSubnetNameW<P0>(subnetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("netapi32.dll" "system" fn DsValidateSubnetNameW(subnetname : windows_core::PCWSTR) -> u32);
    DsValidateSubnetNameW(subnetname.param().abi())
}
#[inline]
pub unsafe fn DsWriteAccountSpnA<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[windows_core::PCSTR]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnA(hds : super::super::Foundation:: HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_core::PCSTR, cspn : u32, rpszspn : *const windows_core::PCSTR) -> u32);
    DsWriteAccountSpnA(hds.param().abi(), operation, pszaccount.param().abi(), rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr()))
}
#[inline]
pub unsafe fn DsWriteAccountSpnW<P0, P1>(hds: P0, operation: DS_SPN_WRITE_OP, pszaccount: P1, rpszspn: &[windows_core::PCWSTR]) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ntdsapi.dll" "system" fn DsWriteAccountSpnW(hds : super::super::Foundation:: HANDLE, operation : DS_SPN_WRITE_OP, pszaccount : windows_core::PCWSTR, cspn : u32, rpszspn : *const windows_core::PCWSTR) -> u32);
    DsWriteAccountSpnW(hds.param().abi(), operation, pszaccount.param().abi(), rpszspn.len().try_into().unwrap(), core::mem::transmute(rpszspn.as_ptr()))
}
#[inline]
pub unsafe fn FreeADsMem(pmem: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("activeds.dll" "system" fn FreeADsMem(pmem : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FreeADsMem(pmem)
}
#[inline]
pub unsafe fn FreeADsStr<P0>(pstr: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn FreeADsStr(pstr : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    FreeADsStr(pstr.param().abi())
}
#[inline]
pub unsafe fn PropVariantToAdsType(pvariant: *mut windows_core::VARIANT, dwnumvariant: u32, ppadsvalues: *mut *mut ADSVALUE, pdwnumvalues: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("activeds.dll" "system" fn PropVariantToAdsType(pvariant : *mut core::mem::MaybeUninit < windows_core::VARIANT >, dwnumvariant : u32, ppadsvalues : *mut *mut ADSVALUE, pdwnumvalues : *mut u32) -> windows_core::HRESULT);
    PropVariantToAdsType(core::mem::transmute(pvariant), dwnumvariant, ppadsvalues, pdwnumvalues).ok()
}
#[inline]
pub unsafe fn ReallocADsMem(poldmem: *mut core::ffi::c_void, cbold: u32, cbnew: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("activeds.dll" "system" fn ReallocADsMem(poldmem : *mut core::ffi::c_void, cbold : u32, cbnew : u32) -> *mut core::ffi::c_void);
    ReallocADsMem(poldmem, cbold, cbnew)
}
#[inline]
pub unsafe fn ReallocADsStr<P0>(ppstr: *mut windows_core::PWSTR, pstr: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn ReallocADsStr(ppstr : *mut windows_core::PWSTR, pstr : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    ReallocADsStr(ppstr, pstr.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn SecurityDescriptorToBinarySD<P0, P1, P2, P3>(vvarsecdes: P0, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, pdwsdlength: *mut u32, pszservername: P1, username: P2, password: P3, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::VARIANT>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("activeds.dll" "system" fn SecurityDescriptorToBinarySD(vvarsecdes : core::mem::MaybeUninit < windows_core::VARIANT >, ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR, pdwsdlength : *mut u32, pszservername : windows_core::PCWSTR, username : windows_core::PCWSTR, password : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    SecurityDescriptorToBinarySD(vvarsecdes.param().abi(), ppsecuritydescriptor, pdwsdlength, pszservername.param().abi(), username.param().abi(), password.param().abi(), dwflags).ok()
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADs, IADs_Vtbl, 0xfd8256d0_fd15_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADs {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADs, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADs {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Class(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Class)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GUID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GUID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ADsPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Schema(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Schema)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetInfo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInfo)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Get<P0>(&self, bstrname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Put<P0, P1>(&self, bstrname: P0, vprop: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Put)(windows_core::Interface::as_raw(self), bstrname.param().abi(), vprop.param().abi()).ok()
    }
    pub unsafe fn GetEx<P0>(&self, bstrname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutEx<P0, P1>(&self, lncontrolcode: i32, bstrname: P0, vprop: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).PutEx)(windows_core::Interface::as_raw(self), lncontrolcode, bstrname.param().abi(), vprop.param().abi()).ok()
    }
    pub unsafe fn GetInfoEx<P0>(&self, vproperties: P0, lnreserved: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).GetInfoEx)(windows_core::Interface::as_raw(self), vproperties.param().abi(), lnreserved).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADs_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Class: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ADsPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Schema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Put: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsADSystemInfo, IADsADSystemInfo_Vtbl, 0x5bb11929_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsADSystemInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsADSystemInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsADSystemInfo {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SiteName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SiteName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DomainShortName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainShortName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DomainDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainDNSName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ForestDNSName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForestDNSName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PDCRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PDCRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SchemaRoleOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SchemaRoleOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsNativeMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsNativeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAnyDCName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAnyDCName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDCSiteName<P0>(&self, szserver: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDCSiteName)(windows_core::Interface::as_raw(self), szserver.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RefreshSchemaCache(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RefreshSchemaCache)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTrees(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTrees)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsADSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SiteName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DomainShortName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DomainDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ForestDNSName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PDCRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SchemaRoleOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsNativeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetAnyDCName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDCSiteName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RefreshSchemaCache: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsAccessControlEntry, IADsAccessControlEntry_Vtbl, 0xb4f3a14c_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsAccessControlEntry {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsAccessControlEntry, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlEntry {
    pub unsafe fn AccessMask(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccessMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccessMask(&self, lnaccessmask: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAccessMask)(windows_core::Interface::as_raw(self), lnaccessmask).ok()
    }
    pub unsafe fn AceType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAceType(&self, lnacetype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAceType)(windows_core::Interface::as_raw(self), lnacetype).ok()
    }
    pub unsafe fn AceFlags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AceFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAceFlags(&self, lnaceflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAceFlags)(windows_core::Interface::as_raw(self), lnaceflags).ok()
    }
    pub unsafe fn Flags(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Flags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFlags(&self, lnflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), lnflags).ok()
    }
    pub unsafe fn ObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetObjectType<P0>(&self, bstrobjecttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetObjectType)(windows_core::Interface::as_raw(self), bstrobjecttype.param().abi()).ok()
    }
    pub unsafe fn InheritedObjectType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InheritedObjectType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInheritedObjectType<P0>(&self, bstrinheritedobjecttype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInheritedObjectType)(windows_core::Interface::as_raw(self), bstrinheritedobjecttype.param().abi()).ok()
    }
    pub unsafe fn Trustee(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Trustee)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTrustee<P0>(&self, bstrtrustee: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTrustee)(windows_core::Interface::as_raw(self), bstrtrustee.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsAccessControlEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAccessMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Flags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInheritedObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Trustee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTrustee: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsAccessControlList, IADsAccessControlList_Vtbl, 0xb7ee91cc_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsAccessControlList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsAccessControlList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsAccessControlList {
    pub unsafe fn AclRevision(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AclRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAclRevision(&self, lnaclrevision: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAclRevision)(windows_core::Interface::as_raw(self), lnaclrevision).ok()
    }
    pub unsafe fn AceCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAceCount(&self, lnacecount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAceCount)(windows_core::Interface::as_raw(self), lnacecount).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).AddAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RemoveAce<P0>(&self, paccesscontrolentry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).RemoveAce)(windows_core::Interface::as_raw(self), paccesscontrolentry.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAccessList(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyAccessList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsAccessControlList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAclRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAceCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RemoveAce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RemoveAce: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAccessList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAccessList: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsAcl, IADsAcl_Vtbl, 0x8452d3ab_0869_11d1_a377_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsAcl {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsAcl, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsAcl {
    pub unsafe fn ProtectedAttrName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProtectedAttrName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProtectedAttrName<P0>(&self, bstrprotectedattrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProtectedAttrName)(windows_core::Interface::as_raw(self), bstrprotectedattrname.param().abi()).ok()
    }
    pub unsafe fn SubjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubjectName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubjectName<P0>(&self, bstrsubjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubjectName)(windows_core::Interface::as_raw(self), bstrsubjectname.param().abi()).ok()
    }
    pub unsafe fn Privileges(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Privileges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPrivileges(&self, lnprivileges: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPrivileges)(windows_core::Interface::as_raw(self), lnprivileges).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsAcl_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProtectedAttrName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubjectName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Privileges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPrivileges: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyAcl: usize,
}
windows_core::imp::define_interface!(IADsAggregatee, IADsAggregatee_Vtbl, 0x1346ce8c_9039_11d0_8528_00c04fd8d503);
impl core::ops::Deref for IADsAggregatee {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IADsAggregatee, windows_core::IUnknown);
impl IADsAggregatee {
    pub unsafe fn ConnectAsAggregatee<P0>(&self, pouterunknown: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ConnectAsAggregatee)(windows_core::Interface::as_raw(self), pouterunknown.param().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregatee(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectAsAggregatee)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RelinquishInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RelinquishInterface)(windows_core::Interface::as_raw(self), riid).ok()
    }
    pub unsafe fn RestoreInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RestoreInterface)(windows_core::Interface::as_raw(self), riid).ok()
    }
}
#[repr(C)]
pub struct IADsAggregatee_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsAggregatee: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelinquishInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RestoreInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IADsAggregator, IADsAggregator_Vtbl, 0x52db5fb0_941f_11d0_8529_00c04fd8d503);
impl core::ops::Deref for IADsAggregator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IADsAggregator, windows_core::IUnknown);
impl IADsAggregator {
    pub unsafe fn ConnectAsAggregator<P0>(&self, paggregatee: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ConnectAsAggregator)(windows_core::Interface::as_raw(self), paggregatee.param().abi()).ok()
    }
    pub unsafe fn DisconnectAsAggregator(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisconnectAsAggregator)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IADsAggregator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ConnectAsAggregator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectAsAggregator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsBackLink, IADsBackLink_Vtbl, 0xfd1302bd_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsBackLink {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsBackLink, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsBackLink {
    pub unsafe fn RemoteID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RemoteID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRemoteID(&self, lnremoteid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRemoteID)(windows_core::Interface::as_raw(self), lnremoteid).ok()
    }
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), bstrobjectname.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsBackLink_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub RemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRemoteID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsCaseIgnoreList, IADsCaseIgnoreList_Vtbl, 0x7b66b533_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsCaseIgnoreList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsCaseIgnoreList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsCaseIgnoreList {
    pub unsafe fn CaseIgnoreList(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CaseIgnoreList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCaseIgnoreList<P0>(&self, vcaseignorelist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetCaseIgnoreList)(windows_core::Interface::as_raw(self), vcaseignorelist.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsCaseIgnoreList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetCaseIgnoreList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsClass, IADsClass_Vtbl, 0xc8f93dd0_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsClass {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsClass, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsClass {
    pub unsafe fn PrimaryInterface(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrimaryInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCLSID<P0>(&self, bstrclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCLSID)(windows_core::Interface::as_raw(self), bstrclsid.param().abi()).ok()
    }
    pub unsafe fn OID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOID<P0>(&self, bstroid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOID)(windows_core::Interface::as_raw(self), bstroid.param().abi()).ok()
    }
    pub unsafe fn Abstract(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Abstract)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAbstract<P0>(&self, fabstract: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAbstract)(windows_core::Interface::as_raw(self), fabstract.param().abi()).ok()
    }
    pub unsafe fn Auxiliary(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Auxiliary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAuxiliary<P0>(&self, fauxiliary: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAuxiliary)(windows_core::Interface::as_raw(self), fauxiliary.param().abi()).ok()
    }
    pub unsafe fn MandatoryProperties(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MandatoryProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMandatoryProperties<P0>(&self, vmandatoryproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetMandatoryProperties)(windows_core::Interface::as_raw(self), vmandatoryproperties.param().abi()).ok()
    }
    pub unsafe fn OptionalProperties(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OptionalProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOptionalProperties<P0>(&self, voptionalproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOptionalProperties)(windows_core::Interface::as_raw(self), voptionalproperties.param().abi()).ok()
    }
    pub unsafe fn NamingProperties(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NamingProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNamingProperties<P0>(&self, vnamingproperties: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetNamingProperties)(windows_core::Interface::as_raw(self), vnamingproperties.param().abi()).ok()
    }
    pub unsafe fn DerivedFrom(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDerivedFrom<P0>(&self, vderivedfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDerivedFrom)(windows_core::Interface::as_raw(self), vderivedfrom.param().abi()).ok()
    }
    pub unsafe fn AuxDerivedFrom(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AuxDerivedFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAuxDerivedFrom<P0>(&self, vauxderivedfrom: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetAuxDerivedFrom)(windows_core::Interface::as_raw(self), vauxderivedfrom.param().abi()).ok()
    }
    pub unsafe fn PossibleSuperiors(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PossibleSuperiors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPossibleSuperiors<P0>(&self, vpossiblesuperiors: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPossibleSuperiors)(windows_core::Interface::as_raw(self), vpossiblesuperiors.param().abi()).ok()
    }
    pub unsafe fn Containment(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Containment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetContainment<P0>(&self, vcontainment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetContainment)(windows_core::Interface::as_raw(self), vcontainment.param().abi()).ok()
    }
    pub unsafe fn Container(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetContainer<P0>(&self, fcontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetContainer)(windows_core::Interface::as_raw(self), fcontainer.param().abi()).ok()
    }
    pub unsafe fn HelpFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HelpFileName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHelpFileName<P0>(&self, bstrhelpfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHelpFileName)(windows_core::Interface::as_raw(self), bstrhelpfilename.param().abi()).ok()
    }
    pub unsafe fn HelpFileContext(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HelpFileContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHelpFileContext(&self, lnhelpfilecontext: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpFileContext)(windows_core::Interface::as_raw(self), lnhelpfilecontext).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsClass_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrimaryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Abstract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAbstract: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Auxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAuxiliary: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetMandatoryProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub OptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetOptionalProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetNamingProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetAuxDerivedFrom: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPossibleSuperiors: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Containment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetContainment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub HelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHelpFileContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsCollection, IADsCollection_Vtbl, 0x72b945e0_253b_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsCollection {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsCollection, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0, P1>(&self, bstrname: P0, vitem: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), bstrname.param().abi(), vitem.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstritemtoberemoved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), bstritemtoberemoved.param().abi()).ok()
    }
    pub unsafe fn GetObject<P0>(&self, bstrname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), bstrname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsComputer, IADsComputer_Vtbl, 0xefe3cc70_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsComputer {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsComputer, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputer {
    pub unsafe fn ComputerID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputerID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Site(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Site)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocation<P0>(&self, bstrlocation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocation)(windows_core::Interface::as_raw(self), bstrlocation.param().abi()).ok()
    }
    pub unsafe fn PrimaryUser(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrimaryUser)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrimaryUser<P0>(&self, bstrprimaryuser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrimaryUser)(windows_core::Interface::as_raw(self), bstrprimaryuser.param().abi()).ok()
    }
    pub unsafe fn Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Owner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwner<P0>(&self, bstrowner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), bstrowner.param().abi()).ok()
    }
    pub unsafe fn Division(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Division)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDivision<P0>(&self, bstrdivision: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDivision)(windows_core::Interface::as_raw(self), bstrdivision.param().abi()).ok()
    }
    pub unsafe fn Department(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Department)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDepartment<P0>(&self, bstrdepartment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDepartment)(windows_core::Interface::as_raw(self), bstrdepartment.param().abi()).ok()
    }
    pub unsafe fn Role(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Role)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRole<P0>(&self, bstrrole: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetRole)(windows_core::Interface::as_raw(self), bstrrole.param().abi()).ok()
    }
    pub unsafe fn OperatingSystem(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OperatingSystem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOperatingSystem<P0>(&self, bstroperatingsystem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOperatingSystem)(windows_core::Interface::as_raw(self), bstroperatingsystem.param().abi()).ok()
    }
    pub unsafe fn OperatingSystemVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OperatingSystemVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOperatingSystemVersion<P0>(&self, bstroperatingsystemversion: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOperatingSystemVersion)(windows_core::Interface::as_raw(self), bstroperatingsystemversion.param().abi()).ok()
    }
    pub unsafe fn Model(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Model)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetModel<P0>(&self, bstrmodel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetModel)(windows_core::Interface::as_raw(self), bstrmodel.param().abi()).ok()
    }
    pub unsafe fn Processor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Processor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProcessor<P0>(&self, bstrprocessor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProcessor)(windows_core::Interface::as_raw(self), bstrprocessor.param().abi()).ok()
    }
    pub unsafe fn ProcessorCount(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ProcessorCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProcessorCount<P0>(&self, bstrprocessorcount: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProcessorCount)(windows_core::Interface::as_raw(self), bstrprocessorcount.param().abi()).ok()
    }
    pub unsafe fn MemorySize(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MemorySize)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMemorySize<P0>(&self, bstrmemorysize: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMemorySize)(windows_core::Interface::as_raw(self), bstrmemorysize.param().abi()).ok()
    }
    pub unsafe fn StorageCapacity(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StorageCapacity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStorageCapacity<P0>(&self, bstrstoragecapacity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStorageCapacity)(windows_core::Interface::as_raw(self), bstrstoragecapacity.param().abi()).ok()
    }
    pub unsafe fn NetAddresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNetAddresses<P0>(&self, vnetaddresses: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetNetAddresses)(windows_core::Interface::as_raw(self), vnetaddresses.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsComputer_Vtbl {
    pub base__: IADs_Vtbl,
    pub ComputerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Site: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPrimaryUser: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOperatingSystem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOperatingSystemVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Processor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProcessorCount: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMemorySize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStorageCapacity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetNetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsComputerOperations, IADsComputerOperations_Vtbl, 0xef497680_1d9f_11cf_b1f3_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsComputerOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsComputerOperations, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsComputerOperations {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Shutdown<P0>(&self, breboot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self), breboot.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsComputerOperations_Vtbl {
    pub base__: IADs_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsContainer, IADsContainer_Vtbl, 0x001677d0_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsContainer {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsContainer, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsContainer {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Filter(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFilter<P0>(&self, var: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), var.param().abi()).ok()
    }
    pub unsafe fn Hints(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHints<P0>(&self, vhints: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetHints)(windows_core::Interface::as_raw(self), vhints.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject<P0, P1>(&self, classname: P0, relativename: P1) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), classname.param().abi(), relativename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create<P0, P1>(&self, classname: P0, relativename: P1) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), classname.param().abi(), relativename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Delete<P0, P1>(&self, bstrclassname: P0, bstrrelativename: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self), bstrclassname.param().abi(), bstrrelativename.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyHere<P0, P1>(&self, sourcename: P0, newname: P1) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyHere)(windows_core::Interface::as_raw(self), sourcename.param().abi(), newname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoveHere<P0, P1>(&self, sourcename: P0, newname: P1) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoveHere)(windows_core::Interface::as_raw(self), sourcename.param().abi(), newname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsContainer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Hints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetHints: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyHere: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyHere: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MoveHere: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoveHere: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsDNWithBinary, IADsDNWithBinary_Vtbl, 0x7e99c0a2_f935_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsDNWithBinary {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsDNWithBinary, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithBinary {
    pub unsafe fn BinaryValue(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BinaryValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBinaryValue<P0>(&self, vbinaryvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetBinaryValue)(windows_core::Interface::as_raw(self), vbinaryvalue.param().abi()).ok()
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), bstrdnstring.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsDNWithBinary_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub BinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetBinaryValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsDNWithString, IADsDNWithString_Vtbl, 0x370df02e_f934_11d2_ba96_00c04fb6d0d1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsDNWithString {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsDNWithString, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsDNWithString {
    pub unsafe fn StringValue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StringValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStringValue<P0>(&self, bstrstringvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), bstrstringvalue.param().abi()).ok()
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), bstrdnstring.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsDNWithString_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub StringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsDeleteOps, IADsDeleteOps_Vtbl, 0xb2bd0902_8878_11d1_8c21_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsDeleteOps {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsDeleteOps, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsDeleteOps {
    pub unsafe fn DeleteObject(&self, lnflags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteObject)(windows_core::Interface::as_raw(self), lnflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsDeleteOps_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DeleteObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsDomain, IADsDomain_Vtbl, 0x00e4c220_fd16_11ce_abc4_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsDomain {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsDomain, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsDomain {
    pub unsafe fn IsWorkgroup(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsWorkgroup)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinPasswordLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinPasswordLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinPasswordLength(&self, lnminpasswordlength: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinPasswordLength)(windows_core::Interface::as_raw(self), lnminpasswordlength).ok()
    }
    pub unsafe fn MinPasswordAge(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinPasswordAge(&self, lnminpasswordage: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinPasswordAge)(windows_core::Interface::as_raw(self), lnminpasswordage).ok()
    }
    pub unsafe fn MaxPasswordAge(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxPasswordAge)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxPasswordAge(&self, lnmaxpasswordage: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxPasswordAge)(windows_core::Interface::as_raw(self), lnmaxpasswordage).ok()
    }
    pub unsafe fn MaxBadPasswordsAllowed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxBadPasswordsAllowed(&self, lnmaxbadpasswordsallowed: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxBadPasswordsAllowed)(windows_core::Interface::as_raw(self), lnmaxbadpasswordsallowed).ok()
    }
    pub unsafe fn PasswordHistoryLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordHistoryLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPasswordHistoryLength(&self, lnpasswordhistorylength: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPasswordHistoryLength)(windows_core::Interface::as_raw(self), lnpasswordhistorylength).ok()
    }
    pub unsafe fn PasswordAttributes(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordAttributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPasswordAttributes(&self, lnpasswordattributes: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPasswordAttributes)(windows_core::Interface::as_raw(self), lnpasswordattributes).ok()
    }
    pub unsafe fn AutoUnlockInterval(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoUnlockInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoUnlockInterval(&self, lnautounlockinterval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAutoUnlockInterval)(windows_core::Interface::as_raw(self), lnautounlockinterval).ok()
    }
    pub unsafe fn LockoutObservationInterval(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LockoutObservationInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLockoutObservationInterval(&self, lnlockoutobservationinterval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLockoutObservationInterval)(windows_core::Interface::as_raw(self), lnlockoutobservationinterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsDomain_Vtbl {
    pub base__: IADs_Vtbl,
    pub IsWorkgroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub MinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinPasswordLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxPasswordAge: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxBadPasswordsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordHistoryLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub AutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAutoUnlockInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLockoutObservationInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsEmail, IADsEmail_Vtbl, 0x97af011a_478e_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsEmail {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsEmail, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsEmail {
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lntype).ok()
    }
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAddress<P0>(&self, bstraddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), bstraddress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsEmail_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IADsExtension, IADsExtension_Vtbl, 0x3d35553c_d2b0_11d1_b17b_0000f87593a0);
impl core::ops::Deref for IADsExtension {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IADsExtension, windows_core::IUnknown);
impl IADsExtension {
    pub unsafe fn Operate<P0, P1, P2>(&self, dwcode: u32, vardata1: P0, vardata2: P1, vardata3: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).Operate)(windows_core::Interface::as_raw(self), dwcode, vardata1.param().abi(), vardata2.param().abi(), vardata3.param().abi()).ok()
    }
    pub unsafe fn PrivateGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrivateGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateInvoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PrivateInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), pexcepinfo, puargerr).ok()
    }
}
#[repr(C)]
pub struct IADsExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Operate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PrivateGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const u16, u32, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrivateInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, u32, u16, *const super::super::System::Com::DISPPARAMS, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::System::Com::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrivateInvoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsFaxNumber, IADsFaxNumber_Vtbl, 0xa910dea9_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsFaxNumber {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsFaxNumber, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsFaxNumber {
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), bstrtelephonenumber.param().abi()).ok()
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetParameters<P0>(&self, vparameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), vparameters.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsFaxNumber_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsFileService, IADsFileService_Vtbl, 0xa89d1900_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsFileService {
    type Target = IADsService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsFileService, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsService);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileService {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsFileService_Vtbl {
    pub base__: IADsService_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsFileServiceOperations, IADsFileServiceOperations_Vtbl, 0xa02ded10_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsFileServiceOperations {
    type Target = IADsServiceOperations;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsFileServiceOperations, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs, IADsServiceOperations);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileServiceOperations {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sessions(&self) -> windows_core::Result<IADsCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Sessions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Resources(&self) -> windows_core::Result<IADsCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsFileServiceOperations_Vtbl {
    pub base__: IADsServiceOperations_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Sessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sessions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Resources: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsFileShare, IADsFileShare_Vtbl, 0xeb6dcaf0_4b83_11cf_a995_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsFileShare {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsFileShare, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsFileShare {
    pub unsafe fn CurrentUserCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostComputer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHostComputer<P0>(&self, bstrhostcomputer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHostComputer)(windows_core::Interface::as_raw(self), bstrhostcomputer.param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), bstrpath.param().abi()).ok()
    }
    pub unsafe fn MaxUserCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxUserCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxUserCount(&self, lnmaxusercount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxUserCount)(windows_core::Interface::as_raw(self), lnmaxusercount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsFileShare_Vtbl {
    pub base__: IADs_Vtbl,
    pub CurrentUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxUserCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsGroup, IADsGroup_Vtbl, 0x27636b00_410f_11cf_b1ff_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsGroup {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsGroup, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsGroup {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> windows_core::Result<IADsMembers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsMember<P0>(&self, bstrmember: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsMember)(windows_core::Interface::as_raw(self), bstrmember.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Add<P0>(&self, bstrnewitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), bstrnewitem.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, bstritemtoberemoved: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), bstritemtoberemoved.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsGroup_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    pub IsMember: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsHold, IADsHold_Vtbl, 0xb3eb3b37_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsHold {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsHold, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsHold {
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), bstrobjectname.param().abi()).ok()
    }
    pub unsafe fn Amount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Amount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAmount(&self, lnamount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAmount)(windows_core::Interface::as_raw(self), lnamount).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsHold_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Amount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAmount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsLargeInteger, IADsLargeInteger_Vtbl, 0x9068270b_0939_11d1_8be1_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsLargeInteger {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsLargeInteger, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsLargeInteger {
    pub unsafe fn HighPart(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HighPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHighPart(&self, lnhighpart: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHighPart)(windows_core::Interface::as_raw(self), lnhighpart).ok()
    }
    pub unsafe fn LowPart(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LowPart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLowPart(&self, lnlowpart: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLowPart)(windows_core::Interface::as_raw(self), lnlowpart).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsLargeInteger_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub HighPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHighPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LowPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLowPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsLocality, IADsLocality_Vtbl, 0xa05e03a2_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsLocality {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsLocality, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsLocality {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), bstrlocalityname.param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), bstrpostaladdress.param().abi()).ok()
    }
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSeeAlso<P0>(&self, vseealso: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), vseealso.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsLocality_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsMembers, IADsMembers_Vtbl, 0x451a0030_72ec_11cf_b03b_00aa006e0975);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsMembers {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsMembers, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsMembers {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Filter(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFilter<P0>(&self, pvfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), pvfilter.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsMembers_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsNameTranslate, IADsNameTranslate_Vtbl, 0xb1b272a3_3625_11d1_a3a4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsNameTranslate {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsNameTranslate, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsNameTranslate {
    pub unsafe fn SetChaseReferral(&self, lnchasereferral: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetChaseReferral)(windows_core::Interface::as_raw(self), lnchasereferral).ok()
    }
    pub unsafe fn Init<P0>(&self, lnsettype: i32, bstradspath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), lnsettype, bstradspath.param().abi()).ok()
    }
    pub unsafe fn InitEx<P0, P1, P2, P3>(&self, lnsettype: i32, bstradspath: P0, bstruserid: P1, bstrdomain: P2, bstrpassword: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
        P3: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).InitEx)(windows_core::Interface::as_raw(self), lnsettype, bstradspath.param().abi(), bstruserid.param().abi(), bstrdomain.param().abi(), bstrpassword.param().abi()).ok()
    }
    pub unsafe fn Set<P0>(&self, lnsettype: i32, bstradspath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), lnsettype, bstradspath.param().abi()).ok()
    }
    pub unsafe fn Get(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Get)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEx<P0>(&self, lnformattype: i32, pvar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetEx)(windows_core::Interface::as_raw(self), lnformattype, pvar.param().abi()).ok()
    }
    pub unsafe fn GetEx(&self, lnformattype: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEx)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsNameTranslate_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetChaseReferral: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub InitEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Get: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsNamespaces, IADsNamespaces_Vtbl, 0x28b96ba0_b330_11cf_a9ad_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsNamespaces {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsNamespaces, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsNamespaces {
    pub unsafe fn DefaultContainer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultContainer<P0>(&self, bstrdefaultcontainer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDefaultContainer)(windows_core::Interface::as_raw(self), bstrdefaultcontainer.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsNamespaces_Vtbl {
    pub base__: IADs_Vtbl,
    pub DefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDefaultContainer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsNetAddress, IADsNetAddress_Vtbl, 0xb21a50a9_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsNetAddress {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsNetAddress, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsNetAddress {
    pub unsafe fn AddressType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddressType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAddressType(&self, lnaddresstype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAddressType)(windows_core::Interface::as_raw(self), lnaddresstype).ok()
    }
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAddress<P0>(&self, vaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), vaddress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsNetAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AddressType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetAddressType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsO, IADsO_Vtbl, 0xa1cd2dc6_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsO {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsO, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsO {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), bstrlocalityname.param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), bstrpostaladdress.param().abi()).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), bstrtelephonenumber.param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), bstrfaxnumber.param().abi()).ok()
    }
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSeeAlso<P0>(&self, vseealso: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), vseealso.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsO_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsOU, IADsOU_Vtbl, 0xa2f733b8_effe_11cf_8abc_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsOU {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsOU, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsOU {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn LocalityName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LocalityName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocalityName<P0>(&self, bstrlocalityname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocalityName)(windows_core::Interface::as_raw(self), bstrlocalityname.param().abi()).ok()
    }
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalAddress<P0>(&self, bstrpostaladdress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), bstrpostaladdress.param().abi()).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, bstrtelephonenumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), bstrtelephonenumber.param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFaxNumber<P0>(&self, bstrfaxnumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), bstrfaxnumber.param().abi()).ok()
    }
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSeeAlso<P0>(&self, vseealso: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), vseealso.param().abi()).ok()
    }
    pub unsafe fn BusinessCategory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BusinessCategory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBusinessCategory<P0>(&self, bstrbusinesscategory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBusinessCategory)(windows_core::Interface::as_raw(self), bstrbusinesscategory.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsOU_Vtbl {
    pub base__: IADs_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocalityName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub BusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBusinessCategory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsObjectOptions, IADsObjectOptions_Vtbl, 0x46f14fda_232b_11d1_a808_00c04fd8d5a8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsObjectOptions {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsObjectOptions, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsObjectOptions {
    pub unsafe fn GetOption(&self, lnoption: i32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOption)(windows_core::Interface::as_raw(self), lnoption, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOption<P0>(&self, lnoption: i32, vvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), lnoption, vvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsObjectOptions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsOctetList, IADsOctetList_Vtbl, 0x7b28b80f_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsOctetList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsOctetList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsOctetList {
    pub unsafe fn OctetList(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OctetList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOctetList<P0>(&self, voctetlist: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOctetList)(windows_core::Interface::as_raw(self), voctetlist.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsOctetList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub OctetList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetOctetList: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsOpenDSObject, IADsOpenDSObject_Vtbl, 0xddf2891e_0f9c_11d0_8ad4_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsOpenDSObject {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsOpenDSObject, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsOpenDSObject {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenDSObject<P0, P1, P2>(&self, lpszdnname: P0, lpszusername: P1, lpszpassword: P2, lnreserved: i32) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenDSObject)(windows_core::Interface::as_raw(self), lpszdnname.param().abi(), lpszusername.param().abi(), lpszpassword.param().abi(), lnreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsOpenDSObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenDSObject: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPath, IADsPath_Vtbl, 0xb287fcd5_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPath {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPath, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPath {
    pub unsafe fn Type(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetType(&self, lntype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), lntype).ok()
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVolumeName<P0>(&self, bstrvolumename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVolumeName)(windows_core::Interface::as_raw(self), bstrvolumename.param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), bstrpath.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPath_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPathname, IADsPathname_Vtbl, 0xd592aed4_f420_11d0_a36e_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPathname {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPathname, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPathname {
    pub unsafe fn Set<P0>(&self, bstradspath: P0, lnsettype: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), bstradspath.param().abi(), lnsettype).ok()
    }
    pub unsafe fn SetDisplayType(&self, lndisplaytype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDisplayType)(windows_core::Interface::as_raw(self), lndisplaytype).ok()
    }
    pub unsafe fn Retrieve(&self, lnformattype: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Retrieve)(windows_core::Interface::as_raw(self), lnformattype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNumElements(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumElements)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetElement(&self, lnelementindex: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), lnelementindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddLeafElement<P0>(&self, bstrleafelement: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).AddLeafElement)(windows_core::Interface::as_raw(self), bstrleafelement.param().abi()).ok()
    }
    pub unsafe fn RemoveLeafElement(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveLeafElement)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyPath(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetEscapedElement<P0>(&self, lnreserved: i32, bstrinstr: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetEscapedElement)(windows_core::Interface::as_raw(self), lnreserved, bstrinstr.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EscapedMode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EscapedMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEscapedMode(&self, lnescapedmode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEscapedMode)(windows_core::Interface::as_raw(self), lnescapedmode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPathname_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub SetDisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNumElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AddLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveLeafElement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyPath: usize,
    pub GetEscapedElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEscapedMode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPostalAddress, IADsPostalAddress_Vtbl, 0x7adecf29_4680_11d1_a3b4_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPostalAddress {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPostalAddress, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPostalAddress {
    pub unsafe fn PostalAddress(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalAddress<P0>(&self, vpostaladdress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPostalAddress)(windows_core::Interface::as_raw(self), vpostaladdress.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPostalAddress_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPostalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPrintJob, IADsPrintJob_Vtbl, 0x32fb6780_1ed0_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPrintJob {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPrintJob, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJob {
    pub unsafe fn HostPrintQueue(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostPrintQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TimeSubmitted(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TimeSubmitted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalPages(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUntilTime)(windows_core::Interface::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn Notify(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNotify<P0>(&self, bstrnotify: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNotify)(windows_core::Interface::as_raw(self), bstrnotify.param().abi()).ok()
    }
    pub unsafe fn NotifyPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotifyPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNotifyPath<P0>(&self, bstrnotifypath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNotifyPath)(windows_core::Interface::as_raw(self), bstrnotifypath.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPrintJob_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostPrintQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TimeSubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TotalPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNotify: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNotifyPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPrintJobOperations, IADsPrintJobOperations_Vtbl, 0x9a52db30_1ecf_11cf_a988_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPrintJobOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPrintJobOperations, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintJobOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TimeElapsed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TimeElapsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PagesPrinted(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PagesPrinted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Position(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPosition(&self, lnposition: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), lnposition).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPrintJobOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TimeElapsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PagesPrinted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPrintQueue, IADsPrintQueue_Vtbl, 0xb15160d0_1226_11cf_a985_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPrintQueue {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPrintQueue, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueue {
    pub unsafe fn PrinterPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrinterPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrinterPath<P0>(&self, bstrprinterpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrinterPath)(windows_core::Interface::as_raw(self), bstrprinterpath.param().abi()).ok()
    }
    pub unsafe fn Model(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Model)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetModel<P0>(&self, bstrmodel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetModel)(windows_core::Interface::as_raw(self), bstrmodel.param().abi()).ok()
    }
    pub unsafe fn Datatype(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Datatype)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDatatype<P0>(&self, bstrdatatype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDatatype)(windows_core::Interface::as_raw(self), bstrdatatype.param().abi()).ok()
    }
    pub unsafe fn PrintProcessor(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrintProcessor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintProcessor<P0>(&self, bstrprintprocessor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrintProcessor)(windows_core::Interface::as_raw(self), bstrprintprocessor.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Location)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLocation<P0>(&self, bstrlocation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLocation)(windows_core::Interface::as_raw(self), bstrlocation.param().abi()).ok()
    }
    pub unsafe fn StartTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartTime(&self, dastarttime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), dastarttime).ok()
    }
    pub unsafe fn UntilTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UntilTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUntilTime(&self, dauntiltime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUntilTime)(windows_core::Interface::as_raw(self), dauntiltime).ok()
    }
    pub unsafe fn DefaultJobPriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DefaultJobPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefaultJobPriority(&self, lndefaultjobpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDefaultJobPriority)(windows_core::Interface::as_raw(self), lndefaultjobpriority).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, lnpriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), lnpriority).ok()
    }
    pub unsafe fn BannerPage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BannerPage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetBannerPage<P0>(&self, bstrbannerpage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetBannerPage)(windows_core::Interface::as_raw(self), bstrbannerpage.param().abi()).ok()
    }
    pub unsafe fn PrintDevices(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrintDevices)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintDevices<P0>(&self, vprintdevices: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPrintDevices)(windows_core::Interface::as_raw(self), vprintdevices.param().abi()).ok()
    }
    pub unsafe fn NetAddresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NetAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNetAddresses<P0>(&self, vnetaddresses: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetNetAddresses)(windows_core::Interface::as_raw(self), vnetaddresses.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPrintQueue_Vtbl {
    pub base__: IADs_Vtbl,
    pub PrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPrinterPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Datatype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDatatype: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPrintProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub UntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUntilTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetDefaultJobPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetBannerPage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPrintDevices: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub NetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetNetAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPrintQueueOperations, IADsPrintQueueOperations_Vtbl, 0x124be5c0_156e_11cf_a986_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPrintQueueOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPrintQueueOperations, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsPrintQueueOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrintJobs(&self) -> windows_core::Result<IADsCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrintJobs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Purge(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Purge)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPrintQueueOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PrintJobs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PrintJobs: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Purge: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsProperty, IADsProperty_Vtbl, 0xc8f93dd3_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsProperty {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsProperty, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsProperty {
    pub unsafe fn OID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOID<P0>(&self, bstroid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOID)(windows_core::Interface::as_raw(self), bstroid.param().abi()).ok()
    }
    pub unsafe fn Syntax(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Syntax)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSyntax<P0>(&self, bstrsyntax: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSyntax)(windows_core::Interface::as_raw(self), bstrsyntax.param().abi()).ok()
    }
    pub unsafe fn MaxRange(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxRange(&self, lnmaxrange: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxRange)(windows_core::Interface::as_raw(self), lnmaxrange).ok()
    }
    pub unsafe fn MinRange(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMinRange(&self, lnminrange: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMinRange)(windows_core::Interface::as_raw(self), lnminrange).ok()
    }
    pub unsafe fn MultiValued(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MultiValued)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMultiValued<P0>(&self, fmultivalued: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetMultiValued)(windows_core::Interface::as_raw(self), fmultivalued.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers(&self) -> windows_core::Result<IADsCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Qualifiers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsProperty_Vtbl {
    pub base__: IADs_Vtbl,
    pub OID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Syntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MinRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMinRange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetMultiValued: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Qualifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Qualifiers: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPropertyEntry, IADsPropertyEntry_Vtbl, 0x05792c8e_941f_11d0_8529_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPropertyEntry {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPropertyEntry, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyEntry {
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, bstrname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), bstrname.param().abi()).ok()
    }
    pub unsafe fn ADsType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetADsType)(windows_core::Interface::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn ControlCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ControlCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetControlCode(&self, lncontrolcode: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControlCode)(windows_core::Interface::as_raw(self), lncontrolcode).ok()
    }
    pub unsafe fn Values(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Values)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetValues<P0>(&self, vvalues: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetValues)(windows_core::Interface::as_raw(self), vvalues.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPropertyEntry_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetControlCode: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetValues: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPropertyList, IADsPropertyList_Vtbl, 0xc6f602b6_8f69_11d0_8528_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPropertyList {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPropertyList, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyList {
    pub unsafe fn PropertyCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PropertyCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Next(&self, pvariant: *mut windows_core::VARIANT) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), core::mem::transmute(pvariant))
    }
    pub unsafe fn Skip(&self, celements: i32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celements)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Item<P0>(&self, varindex: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), varindex.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPropertyItem<P0>(&self, bstrname: P0, lnadstype: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPropertyItem)(windows_core::Interface::as_raw(self), bstrname.param().abi(), lnadstype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PutPropertyItem<P0>(&self, vardata: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).PutPropertyItem)(windows_core::Interface::as_raw(self), vardata.param().abi()).ok()
    }
    pub unsafe fn ResetPropertyItem<P0>(&self, varentry: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).ResetPropertyItem)(windows_core::Interface::as_raw(self), varentry.param().abi()).ok()
    }
    pub unsafe fn PurgePropertyList(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PurgePropertyList)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPropertyList_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub PropertyCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ResetPropertyItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PurgePropertyList: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPropertyValue, IADsPropertyValue_Vtbl, 0x79fa9ad0_a97c_11d0_8534_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPropertyValue {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue {
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ADsType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADsType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetADsType(&self, lnadstype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetADsType)(windows_core::Interface::as_raw(self), lnadstype).ok()
    }
    pub unsafe fn DNString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DNString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDNString<P0>(&self, bstrdnstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDNString)(windows_core::Interface::as_raw(self), bstrdnstring.param().abi()).ok()
    }
    pub unsafe fn CaseExactString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CaseExactString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCaseExactString<P0>(&self, bstrcaseexactstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCaseExactString)(windows_core::Interface::as_raw(self), bstrcaseexactstring.param().abi()).ok()
    }
    pub unsafe fn CaseIgnoreString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CaseIgnoreString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCaseIgnoreString<P0>(&self, bstrcaseignorestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCaseIgnoreString)(windows_core::Interface::as_raw(self), bstrcaseignorestring.param().abi()).ok()
    }
    pub unsafe fn PrintableString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PrintableString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPrintableString<P0>(&self, bstrprintablestring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPrintableString)(windows_core::Interface::as_raw(self), bstrprintablestring.param().abi()).ok()
    }
    pub unsafe fn NumericString(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NumericString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNumericString<P0>(&self, bstrnumericstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNumericString)(windows_core::Interface::as_raw(self), bstrnumericstring.param().abi()).ok()
    }
    pub unsafe fn Boolean(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Boolean)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBoolean(&self, lnboolean: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBoolean)(windows_core::Interface::as_raw(self), lnboolean).ok()
    }
    pub unsafe fn Integer(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Integer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInteger(&self, lninteger: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInteger)(windows_core::Interface::as_raw(self), lninteger).ok()
    }
    pub unsafe fn OctetString(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OctetString)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOctetString<P0>(&self, voctetstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOctetString)(windows_core::Interface::as_raw(self), voctetstring.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, psecuritydescriptor: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), psecuritydescriptor.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LargeInteger(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LargeInteger)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetLargeInteger<P0>(&self, plargeinteger: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetLargeInteger)(windows_core::Interface::as_raw(self), plargeinteger.param().abi()).ok()
    }
    pub unsafe fn UTCTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UTCTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUTCTime(&self, dautctime: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUTCTime)(windows_core::Interface::as_raw(self), dautctime).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPropertyValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ADsType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetADsType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DNString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDNString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCaseExactString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCaseIgnoreString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPrintableString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NumericString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNumericString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Boolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Integer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInteger: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OctetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetOctetString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LargeInteger: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetLargeInteger: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetLargeInteger: usize,
    pub UTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetUTCTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsPropertyValue2, IADsPropertyValue2_Vtbl, 0x306e831c_5bc7_11d1_a3b8_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsPropertyValue2 {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsPropertyValue2, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsPropertyValue2 {
    pub unsafe fn GetObjectProperty(&self, lnadstype: *mut i32, pvprop: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectProperty)(windows_core::Interface::as_raw(self), lnadstype, core::mem::transmute(pvprop)).ok()
    }
    pub unsafe fn PutObjectProperty<P0>(&self, lnadstype: i32, vprop: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).PutObjectProperty)(windows_core::Interface::as_raw(self), lnadstype, vprop.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsPropertyValue2_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutObjectProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsReplicaPointer, IADsReplicaPointer_Vtbl, 0xf60fb803_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsReplicaPointer {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsReplicaPointer, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsReplicaPointer {
    pub unsafe fn ServerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServerName<P0>(&self, bstrservername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServerName)(windows_core::Interface::as_raw(self), bstrservername.param().abi()).ok()
    }
    pub unsafe fn ReplicaType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReplicaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReplicaType(&self, lnreplicatype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReplicaType)(windows_core::Interface::as_raw(self), lnreplicatype).ok()
    }
    pub unsafe fn ReplicaNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReplicaNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetReplicaNumber(&self, lnreplicanumber: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetReplicaNumber)(windows_core::Interface::as_raw(self), lnreplicanumber).ok()
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCount(&self, lncount: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCount)(windows_core::Interface::as_raw(self), lncount).ok()
    }
    pub unsafe fn ReplicaAddressHints(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReplicaAddressHints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetReplicaAddressHints<P0>(&self, vreplicaaddresshints: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetReplicaAddressHints)(windows_core::Interface::as_raw(self), vreplicaaddresshints.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsReplicaPointer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ServerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServerName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetReplicaType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetReplicaNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetReplicaAddressHints: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsResource, IADsResource_Vtbl, 0x34a05b20_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsResource {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsResource, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsResource {
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LockCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsResource_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsSecurityDescriptor, IADsSecurityDescriptor_Vtbl, 0xb8c787ca_9bdd_11d0_852c_00c04fd8d503);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsSecurityDescriptor {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsSecurityDescriptor, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityDescriptor {
    pub unsafe fn Revision(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Revision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRevision(&self, lnrevision: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRevision)(windows_core::Interface::as_raw(self), lnrevision).ok()
    }
    pub unsafe fn Control(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Control)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetControl(&self, lncontrol: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetControl)(windows_core::Interface::as_raw(self), lncontrol).ok()
    }
    pub unsafe fn Owner(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Owner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwner<P0>(&self, bstrowner: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwner)(windows_core::Interface::as_raw(self), bstrowner.param().abi()).ok()
    }
    pub unsafe fn OwnerDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOwnerDefaulted<P0>(&self, fownerdefaulted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOwnerDefaulted)(windows_core::Interface::as_raw(self), fownerdefaulted.param().abi()).ok()
    }
    pub unsafe fn Group(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Group)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetGroup<P0>(&self, bstrgroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetGroup)(windows_core::Interface::as_raw(self), bstrgroup.param().abi()).ok()
    }
    pub unsafe fn GroupDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GroupDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGroupDefaulted<P0>(&self, fgroupdefaulted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetGroupDefaulted)(windows_core::Interface::as_raw(self), fgroupdefaulted.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DiscretionaryAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DiscretionaryAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetDiscretionaryAcl<P0>(&self, pdiscretionaryacl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetDiscretionaryAcl)(windows_core::Interface::as_raw(self), pdiscretionaryacl.param().abi()).ok()
    }
    pub unsafe fn DaclDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDaclDefaulted<P0>(&self, fdacldefaulted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetDaclDefaulted)(windows_core::Interface::as_raw(self), fdacldefaulted.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SystemAcl(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SystemAcl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSystemAcl<P0>(&self, psystemacl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetSystemAcl)(windows_core::Interface::as_raw(self), psystemacl.param().abi()).ok()
    }
    pub unsafe fn SaclDefaulted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaclDefaulted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSaclDefaulted<P0>(&self, fsacldefaulted: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetSaclDefaulted)(windows_core::Interface::as_raw(self), fsacldefaulted.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopySecurityDescriptor(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopySecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsSecurityDescriptor_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Revision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Control: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Owner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwner: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOwnerDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetGroupDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DiscretionaryAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetDiscretionaryAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetDiscretionaryAcl: usize,
    pub DaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetDaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SystemAcl: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSystemAcl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSystemAcl: usize,
    pub SaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetSaclDefaulted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CopySecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopySecurityDescriptor: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsSecurityUtility, IADsSecurityUtility_Vtbl, 0xa63251b2_5f21_474b_ab52_4a8efad10895);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsSecurityUtility {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsSecurityUtility, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsSecurityUtility {
    pub unsafe fn GetSecurityDescriptor<P0>(&self, varpath: P0, lpathformat: i32, lformat: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), varpath.param().abi(), lpathformat, lformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSecurityDescriptor<P0, P1>(&self, varpath: P0, lpathformat: i32, vardata: P1, ldataformat: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), varpath.param().abi(), lpathformat, vardata.param().abi(), ldataformat).ok()
    }
    pub unsafe fn ConvertSecurityDescriptor<P0>(&self, varsd: P0, ldataformat: i32, loutformat: i32) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConvertSecurityDescriptor)(windows_core::Interface::as_raw(self), varsd.param().abi(), ldataformat, loutformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SecurityMask(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecurityMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSecurityMask(&self, lnsecuritymask: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSecurityMask)(windows_core::Interface::as_raw(self), lnsecuritymask).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsSecurityUtility_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32, core::mem::MaybeUninit<windows_core::VARIANT>, i32) -> windows_core::HRESULT,
    pub ConvertSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, i32, i32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSecurityMask: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsService, IADsService_Vtbl, 0x68af66e0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsService {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsService, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsService {
    pub unsafe fn HostComputer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HostComputer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHostComputer<P0>(&self, bstrhostcomputer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHostComputer)(windows_core::Interface::as_raw(self), bstrhostcomputer.param().abi()).ok()
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDisplayName<P0>(&self, bstrdisplayname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), bstrdisplayname.param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetVersion<P0>(&self, bstrversion: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), bstrversion.param().abi()).ok()
    }
    pub unsafe fn ServiceType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetServiceType(&self, lnservicetype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetServiceType)(windows_core::Interface::as_raw(self), lnservicetype).ok()
    }
    pub unsafe fn StartType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStartType(&self, lnstarttype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStartType)(windows_core::Interface::as_raw(self), lnstarttype).ok()
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPath<P0>(&self, bstrpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), bstrpath.param().abi()).ok()
    }
    pub unsafe fn StartupParameters(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).StartupParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetStartupParameters<P0>(&self, bstrstartupparameters: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetStartupParameters)(windows_core::Interface::as_raw(self), bstrstartupparameters.param().abi()).ok()
    }
    pub unsafe fn ErrorControl(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ErrorControl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetErrorControl(&self, lnerrorcontrol: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetErrorControl)(windows_core::Interface::as_raw(self), lnerrorcontrol).ok()
    }
    pub unsafe fn LoadOrderGroup(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadOrderGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLoadOrderGroup<P0>(&self, bstrloadordergroup: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLoadOrderGroup)(windows_core::Interface::as_raw(self), bstrloadordergroup.param().abi()).ok()
    }
    pub unsafe fn ServiceAccountName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceAccountName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceAccountName<P0>(&self, bstrserviceaccountname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceAccountName)(windows_core::Interface::as_raw(self), bstrserviceaccountname.param().abi()).ok()
    }
    pub unsafe fn ServiceAccountPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceAccountPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceAccountPath<P0>(&self, bstrserviceaccountpath: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceAccountPath)(windows_core::Interface::as_raw(self), bstrserviceaccountpath.param().abi()).ok()
    }
    pub unsafe fn Dependencies(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Dependencies)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDependencies<P0>(&self, vdependencies: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetDependencies)(windows_core::Interface::as_raw(self), vdependencies.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsService_Vtbl {
    pub base__: IADs_Vtbl,
    pub HostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHostComputer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub StartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetStartupParameters: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetErrorControl: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLoadOrderGroup: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceAccountName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceAccountPath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Dependencies: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetDependencies: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsServiceOperations, IADsServiceOperations_Vtbl, 0x5d7b33f0_31ca_11cf_a98a_00aa006bc149);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsServiceOperations {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsServiceOperations, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsServiceOperations {
    pub unsafe fn Status(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Start(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, bstrnewpassword: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), bstrnewpassword.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsServiceOperations_Vtbl {
    pub base__: IADs_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsSession, IADsSession_Vtbl, 0x398b7da0_4aab_11cf_ae2c_00aa006ebfb9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsSession {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsSession, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsSession {
    pub unsafe fn User(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).User)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UserPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Computer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Computer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ComputerPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputerPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConnectTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ConnectTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IdleTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IdleTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsSession_Vtbl {
    pub base__: IADs_Vtbl,
    pub User: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UserPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Computer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ComputerPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ConnectTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IdleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsSyntax, IADsSyntax_Vtbl, 0xc8f93dd2_4ae0_11cf_9e73_00aa004a5691);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsSyntax {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsSyntax, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsSyntax {
    pub unsafe fn OleAutoDataType(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OleAutoDataType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOleAutoDataType(&self, lnoleautodatatype: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetOleAutoDataType)(windows_core::Interface::as_raw(self), lnoleautodatatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsSyntax_Vtbl {
    pub base__: IADs_Vtbl,
    pub OleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetOleAutoDataType: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsTimestamp, IADsTimestamp_Vtbl, 0xb2f5a901_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsTimestamp {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsTimestamp, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsTimestamp {
    pub unsafe fn WholeSeconds(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WholeSeconds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWholeSeconds(&self, lnwholeseconds: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWholeSeconds)(windows_core::Interface::as_raw(self), lnwholeseconds).ok()
    }
    pub unsafe fn EventID(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEventID(&self, lneventid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventID)(windows_core::Interface::as_raw(self), lneventid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsTimestamp_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetWholeSeconds: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EventID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEventID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsTypedName, IADsTypedName_Vtbl, 0xb371a349_4080_11d1_a3ac_00c04fb950dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsTypedName {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsTypedName, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsTypedName {
    pub unsafe fn ObjectName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ObjectName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetObjectName<P0>(&self, bstrobjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetObjectName)(windows_core::Interface::as_raw(self), bstrobjectname.param().abi()).ok()
    }
    pub unsafe fn Level(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLevel(&self, lnlevel: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), lnlevel).ok()
    }
    pub unsafe fn Interval(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Interval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetInterval(&self, lninterval: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInterval)(windows_core::Interface::as_raw(self), lninterval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsTypedName_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetObjectName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsUser, IADsUser_Vtbl, 0x3e37e320_17e2_11cf_abc4_02608c9e7553);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsUser {
    type Target = IADs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsUser, windows_core::IUnknown, super::super::System::Com::IDispatch, IADs);
#[cfg(feature = "Win32_System_Com")]
impl IADsUser {
    pub unsafe fn BadLoginAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BadLoginAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BadLoginCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BadLoginCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastLogin(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastLogoff(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastLogoff)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LastFailedLogin(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastFailedLogin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PasswordLastChanged(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordLastChanged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn Division(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Division)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDivision<P0>(&self, bstrdivision: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDivision)(windows_core::Interface::as_raw(self), bstrdivision.param().abi()).ok()
    }
    pub unsafe fn Department(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Department)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDepartment<P0>(&self, bstrdepartment: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDepartment)(windows_core::Interface::as_raw(self), bstrdepartment.param().abi()).ok()
    }
    pub unsafe fn EmployeeID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EmployeeID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEmployeeID<P0>(&self, bstremployeeid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEmployeeID)(windows_core::Interface::as_raw(self), bstremployeeid.param().abi()).ok()
    }
    pub unsafe fn FullName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FullName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFullName<P0>(&self, bstrfullname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFullName)(windows_core::Interface::as_raw(self), bstrfullname.param().abi()).ok()
    }
    pub unsafe fn FirstName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FirstName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFirstName<P0>(&self, bstrfirstname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFirstName)(windows_core::Interface::as_raw(self), bstrfirstname.param().abi()).ok()
    }
    pub unsafe fn LastName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLastName<P0>(&self, bstrlastname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLastName)(windows_core::Interface::as_raw(self), bstrlastname.param().abi()).ok()
    }
    pub unsafe fn OtherName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OtherName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOtherName<P0>(&self, bstrothername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOtherName)(windows_core::Interface::as_raw(self), bstrothername.param().abi()).ok()
    }
    pub unsafe fn NamePrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NamePrefix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNamePrefix<P0>(&self, bstrnameprefix: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNamePrefix)(windows_core::Interface::as_raw(self), bstrnameprefix.param().abi()).ok()
    }
    pub unsafe fn NameSuffix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NameSuffix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNameSuffix<P0>(&self, bstrnamesuffix: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetNameSuffix)(windows_core::Interface::as_raw(self), bstrnamesuffix.param().abi()).ok()
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTitle<P0>(&self, bstrtitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), bstrtitle.param().abi()).ok()
    }
    pub unsafe fn Manager(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Manager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetManager<P0>(&self, bstrmanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetManager)(windows_core::Interface::as_raw(self), bstrmanager.param().abi()).ok()
    }
    pub unsafe fn TelephoneHome(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneHome)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneHome<P0>(&self, vtelephonehome: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneHome)(windows_core::Interface::as_raw(self), vtelephonehome.param().abi()).ok()
    }
    pub unsafe fn TelephoneMobile(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneMobile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneMobile<P0>(&self, vtelephonemobile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneMobile)(windows_core::Interface::as_raw(self), vtelephonemobile.param().abi()).ok()
    }
    pub unsafe fn TelephoneNumber(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephoneNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephoneNumber<P0>(&self, vtelephonenumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetTelephoneNumber)(windows_core::Interface::as_raw(self), vtelephonenumber.param().abi()).ok()
    }
    pub unsafe fn TelephonePager(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TelephonePager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTelephonePager<P0>(&self, vtelephonepager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetTelephonePager)(windows_core::Interface::as_raw(self), vtelephonepager.param().abi()).ok()
    }
    pub unsafe fn FaxNumber(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FaxNumber)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFaxNumber<P0>(&self, vfaxnumber: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetFaxNumber)(windows_core::Interface::as_raw(self), vfaxnumber.param().abi()).ok()
    }
    pub unsafe fn OfficeLocations(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OfficeLocations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOfficeLocations<P0>(&self, vofficelocations: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOfficeLocations)(windows_core::Interface::as_raw(self), vofficelocations.param().abi()).ok()
    }
    pub unsafe fn PostalAddresses(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalAddresses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalAddresses<P0>(&self, vpostaladdresses: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPostalAddresses)(windows_core::Interface::as_raw(self), vpostaladdresses.param().abi()).ok()
    }
    pub unsafe fn PostalCodes(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PostalCodes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPostalCodes<P0>(&self, vpostalcodes: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPostalCodes)(windows_core::Interface::as_raw(self), vpostalcodes.param().abi()).ok()
    }
    pub unsafe fn SeeAlso(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SeeAlso)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSeeAlso<P0>(&self, vseealso: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetSeeAlso)(windows_core::Interface::as_raw(self), vseealso.param().abi()).ok()
    }
    pub unsafe fn AccountDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccountDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccountDisabled<P0>(&self, faccountdisabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAccountDisabled)(windows_core::Interface::as_raw(self), faccountdisabled.param().abi()).ok()
    }
    pub unsafe fn AccountExpirationDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AccountExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAccountExpirationDate(&self, daaccountexpirationdate: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAccountExpirationDate)(windows_core::Interface::as_raw(self), daaccountexpirationdate).ok()
    }
    pub unsafe fn GraceLoginsAllowed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GraceLoginsAllowed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGraceLoginsAllowed(&self, lngraceloginsallowed: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGraceLoginsAllowed)(windows_core::Interface::as_raw(self), lngraceloginsallowed).ok()
    }
    pub unsafe fn GraceLoginsRemaining(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GraceLoginsRemaining)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetGraceLoginsRemaining(&self, lngraceloginsremaining: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGraceLoginsRemaining)(windows_core::Interface::as_raw(self), lngraceloginsremaining).ok()
    }
    pub unsafe fn IsAccountLocked(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAccountLocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsAccountLocked<P0>(&self, fisaccountlocked: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsAccountLocked)(windows_core::Interface::as_raw(self), fisaccountlocked.param().abi()).ok()
    }
    pub unsafe fn LoginHours(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoginHours)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLoginHours<P0>(&self, vloginhours: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetLoginHours)(windows_core::Interface::as_raw(self), vloginhours.param().abi()).ok()
    }
    pub unsafe fn LoginWorkstations(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoginWorkstations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLoginWorkstations<P0>(&self, vloginworkstations: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetLoginWorkstations)(windows_core::Interface::as_raw(self), vloginworkstations.param().abi()).ok()
    }
    pub unsafe fn MaxLogins(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxLogins)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxLogins(&self, lnmaxlogins: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxLogins)(windows_core::Interface::as_raw(self), lnmaxlogins).ok()
    }
    pub unsafe fn MaxStorage(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxStorage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetMaxStorage(&self, lnmaxstorage: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMaxStorage)(windows_core::Interface::as_raw(self), lnmaxstorage).ok()
    }
    pub unsafe fn PasswordExpirationDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPasswordExpirationDate(&self, dapasswordexpirationdate: f64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPasswordExpirationDate)(windows_core::Interface::as_raw(self), dapasswordexpirationdate).ok()
    }
    pub unsafe fn PasswordMinimumLength(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordMinimumLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPasswordMinimumLength(&self, lnpasswordminimumlength: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPasswordMinimumLength)(windows_core::Interface::as_raw(self), lnpasswordminimumlength).ok()
    }
    pub unsafe fn PasswordRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PasswordRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPasswordRequired<P0>(&self, fpasswordrequired: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPasswordRequired)(windows_core::Interface::as_raw(self), fpasswordrequired.param().abi()).ok()
    }
    pub unsafe fn RequireUniquePassword(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequireUniquePassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRequireUniquePassword<P0>(&self, frequireuniquepassword: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetRequireUniquePassword)(windows_core::Interface::as_raw(self), frequireuniquepassword.param().abi()).ok()
    }
    pub unsafe fn EmailAddress(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EmailAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEmailAddress<P0>(&self, bstremailaddress: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEmailAddress)(windows_core::Interface::as_raw(self), bstremailaddress.param().abi()).ok()
    }
    pub unsafe fn HomeDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HomeDirectory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHomeDirectory<P0>(&self, bstrhomedirectory: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHomeDirectory)(windows_core::Interface::as_raw(self), bstrhomedirectory.param().abi()).ok()
    }
    pub unsafe fn Languages(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Languages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLanguages<P0>(&self, vlanguages: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetLanguages)(windows_core::Interface::as_raw(self), vlanguages.param().abi()).ok()
    }
    pub unsafe fn Profile(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Profile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetProfile<P0>(&self, bstrprofile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetProfile)(windows_core::Interface::as_raw(self), bstrprofile.param().abi()).ok()
    }
    pub unsafe fn LoginScript(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoginScript)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetLoginScript<P0>(&self, bstrloginscript: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetLoginScript)(windows_core::Interface::as_raw(self), bstrloginscript.param().abi()).ok()
    }
    pub unsafe fn Picture(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Picture)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPicture<P0>(&self, vpicture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetPicture)(windows_core::Interface::as_raw(self), vpicture.param().abi()).ok()
    }
    pub unsafe fn HomePage(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HomePage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHomePage<P0>(&self, bstrhomepage: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetHomePage)(windows_core::Interface::as_raw(self), bstrhomepage.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Groups(&self) -> windows_core::Result<IADsMembers> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPassword<P0>(&self, newpassword: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), newpassword.param().abi()).ok()
    }
    pub unsafe fn ChangePassword<P0, P1>(&self, bstroldpassword: P0, bstrnewpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangePassword)(windows_core::Interface::as_raw(self), bstroldpassword.param().abi(), bstrnewpassword.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsUser_Vtbl {
    pub base__: IADs_Vtbl,
    pub BadLoginAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub BadLoginCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastLogoff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LastFailedLogin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub PasswordLastChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Division: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDivision: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Department: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDepartment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEmployeeID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFullName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FirstName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFirstName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LastName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLastName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OtherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOtherName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNamePrefix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub NameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetNameSuffix: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Manager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetManager: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetTelephoneHome: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetTelephoneMobile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetTelephoneNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub TelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetTelephonePager: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub FaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetFaxNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub OfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetOfficeLocations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPostalAddresses: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPostalCodes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetSeeAlso: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub AccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAccountDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub AccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAccountExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGraceLoginsAllowed: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetGraceLoginsRemaining: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsAccountLocked: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub LoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetLoginHours: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub LoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetLoginWorkstations: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub MaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxLogins: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetMaxStorage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPasswordExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub PasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetPasswordMinimumLength: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub PasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetPasswordRequired: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetRequireUniquePassword: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHomeDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub Profile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub LoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetLoginScript: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Picture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetPicture: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub HomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Groups: usize,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangePassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IADsWinNTSystemInfo, IADsWinNTSystemInfo_Vtbl, 0x6c6d65dc_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IADsWinNTSystemInfo {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IADsWinNTSystemInfo, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IADsWinNTSystemInfo {
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ComputerName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComputerName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DomainName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DomainName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PDC(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PDC)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IADsWinNTSystemInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ComputerName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommonQuery, ICommonQuery_Vtbl, 0xab50dec0_6f1d_11d0_a1c4_00aa00c16e65);
impl core::ops::Deref for ICommonQuery {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICommonQuery, windows_core::IUnknown);
impl ICommonQuery {
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn OpenQueryWindow<P0>(&self, hwndparent: P0, pquerywnd: *mut OPENQUERYWINDOW, ppdataobject: *mut Option<super::super::System::Com::IDataObject>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).OpenQueryWindow)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), pquerywnd, core::mem::transmute(ppdataobject)).ok()
    }
}
#[repr(C)]
pub struct ICommonQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub OpenQueryWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut OPENQUERYWINDOW, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    OpenQueryWindow: usize,
}
windows_core::imp::define_interface!(IDirectoryObject, IDirectoryObject_Vtbl, 0xe798de2c_22e4_11d0_84fe_00c04fd8d503);
impl core::ops::Deref for IDirectoryObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectoryObject, windows_core::IUnknown);
impl IDirectoryObject {
    pub unsafe fn GetObjectInformation(&self) -> windows_core::Result<*mut ADS_OBJECT_INFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObjectAttributes(&self, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32, ppattributeentries: *mut *mut ADS_ATTR_INFO, pdwnumattributesreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectAttributes)(windows_core::Interface::as_raw(self), pattributenames, dwnumberattributes, ppattributeentries, pdwnumattributesreturned).ok()
    }
    pub unsafe fn SetObjectAttributes(&self, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetObjectAttributes)(windows_core::Interface::as_raw(self), pattributeentries, dwnumattributes, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDSObject<P0>(&self, pszrdnname: P0, pattributeentries: *const ADS_ATTR_INFO, dwnumattributes: u32) -> windows_core::Result<super::super::System::Com::IDispatch>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateDSObject)(windows_core::Interface::as_raw(self), pszrdnname.param().abi(), pattributeentries, dwnumattributes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeleteDSObject<P0>(&self, pszrdnname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteDSObject)(windows_core::Interface::as_raw(self), pszrdnname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectoryObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ADS_OBJECT_INFO) -> windows_core::HRESULT,
    pub GetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut *mut ADS_ATTR_INFO, *mut u32) -> windows_core::HRESULT,
    pub SetObjectAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_ATTR_INFO, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_ATTR_INFO, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDSObject: usize,
    pub DeleteDSObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectorySchemaMgmt, IDirectorySchemaMgmt_Vtbl, 0x75db3b9c_a4d8_11d0_a79c_00c04fd8d5a8);
impl core::ops::Deref for IDirectorySchemaMgmt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectorySchemaMgmt, windows_core::IUnknown);
impl IDirectorySchemaMgmt {
    pub unsafe fn EnumAttributes(&self, ppszattrnames: *const windows_core::PCWSTR, dwnumattributes: u32, ppattrdefinition: *const *const ADS_ATTR_DEF, pdwnumattributes: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumAttributes)(windows_core::Interface::as_raw(self), ppszattrnames, dwnumattributes, ppattrdefinition, pdwnumattributes).ok()
    }
    pub unsafe fn CreateAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition).ok()
    }
    pub unsafe fn WriteAttributeDefinition<P0>(&self, pszattributename: P0, pattributedefinition: *const ADS_ATTR_DEF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi(), pattributedefinition).ok()
    }
    pub unsafe fn DeleteAttributeDefinition<P0>(&self, pszattributename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteAttributeDefinition)(windows_core::Interface::as_raw(self), pszattributename.param().abi()).ok()
    }
    pub unsafe fn EnumClasses(&self, ppszclassnames: *const windows_core::PCWSTR, dwnumclasses: u32, ppclassdefinition: *const *const ADS_CLASS_DEF, pdwnumclasses: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumClasses)(windows_core::Interface::as_raw(self), ppszclassnames, dwnumclasses, ppclassdefinition, pdwnumclasses).ok()
    }
    pub unsafe fn WriteClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition).ok()
    }
    pub unsafe fn CreateClassDefinition<P0>(&self, pszclassname: P0, pclassdefinition: *const ADS_CLASS_DEF) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi(), pclassdefinition).ok()
    }
    pub unsafe fn DeleteClassDefinition<P0>(&self, pszclassname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteClassDefinition)(windows_core::Interface::as_raw(self), pszclassname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectorySchemaMgmt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *const *const ADS_ATTR_DEF, *const u32) -> windows_core::HRESULT,
    pub CreateAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_ATTR_DEF) -> windows_core::HRESULT,
    pub WriteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_ATTR_DEF) -> windows_core::HRESULT,
    pub DeleteAttributeDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnumClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *const *const ADS_CLASS_DEF, *const u32) -> windows_core::HRESULT,
    pub WriteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_CLASS_DEF) -> windows_core::HRESULT,
    pub CreateClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const ADS_CLASS_DEF) -> windows_core::HRESULT,
    pub DeleteClassDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDirectorySearch, IDirectorySearch_Vtbl, 0x109ba8ec_92f0_11d0_a790_00c04fd8d5a8);
impl core::ops::Deref for IDirectorySearch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectorySearch, windows_core::IUnknown);
impl IDirectorySearch {
    pub unsafe fn SetSearchPreference(&self, psearchprefs: *const ADS_SEARCHPREF_INFO, dwnumprefs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSearchPreference)(windows_core::Interface::as_raw(self), psearchprefs, dwnumprefs).ok()
    }
    pub unsafe fn ExecuteSearch<P0>(&self, pszsearchfilter: P0, pattributenames: *const windows_core::PCWSTR, dwnumberattributes: u32) -> windows_core::Result<ADS_SEARCH_HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecuteSearch)(windows_core::Interface::as_raw(self), pszsearchfilter.param().abi(), pattributenames, dwnumberattributes, &mut result__).map(|| result__)
    }
    pub unsafe fn AbandonSearch<P0>(&self, phsearchresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).AbandonSearch)(windows_core::Interface::as_raw(self), phsearchresult.param().abi()).ok()
    }
    pub unsafe fn GetFirstRow<P0>(&self, hsearchresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetFirstRow)(windows_core::Interface::as_raw(self), hsearchresult.param().abi())
    }
    pub unsafe fn GetNextRow<P0>(&self, hsearchresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetNextRow)(windows_core::Interface::as_raw(self), hsearchresult.param().abi())
    }
    pub unsafe fn GetPreviousRow<P0>(&self, hsearchresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetPreviousRow)(windows_core::Interface::as_raw(self), hsearchresult.param().abi())
    }
    pub unsafe fn GetNextColumnName<P0>(&self, hsearchhandle: P0, ppszcolumnname: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).GetNextColumnName)(windows_core::Interface::as_raw(self), hsearchhandle.param().abi(), ppszcolumnname)
    }
    pub unsafe fn GetColumn<P0, P1>(&self, hsearchresult: P0, szcolumnname: P1, psearchcolumn: *mut ADS_SEARCH_COLUMN) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetColumn)(windows_core::Interface::as_raw(self), hsearchresult.param().abi(), szcolumnname.param().abi(), psearchcolumn).ok()
    }
    pub unsafe fn FreeColumn(&self, psearchcolumn: *const ADS_SEARCH_COLUMN) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeColumn)(windows_core::Interface::as_raw(self), psearchcolumn).ok()
    }
    pub unsafe fn CloseSearchHandle<P0>(&self, hsearchresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ADS_SEARCH_HANDLE>,
    {
        (windows_core::Interface::vtable(self).CloseSearchHandle)(windows_core::Interface::as_raw(self), hsearchresult.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDirectorySearch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSearchPreference: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_SEARCHPREF_INFO, u32) -> windows_core::HRESULT,
    pub ExecuteSearch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::PCWSTR, u32, *mut ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub AbandonSearch: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetFirstRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetNextRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetPreviousRow: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
    pub GetNextColumnName: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE, windows_core::PCWSTR, *mut ADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    pub FreeColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *const ADS_SEARCH_COLUMN) -> windows_core::HRESULT,
    pub CloseSearchHandle: unsafe extern "system" fn(*mut core::ffi::c_void, ADS_SEARCH_HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsAdminCreateObj, IDsAdminCreateObj_Vtbl, 0x53554a38_f902_11d2_82b9_00c04f68928b);
impl core::ops::Deref for IDsAdminCreateObj {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsAdminCreateObj, windows_core::IUnknown);
impl IDsAdminCreateObj {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IADsContainer>,
        P1: windows_core::Param<IADs>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), padscontainerobj.param().abi(), padscopysource.param().abi(), lpszclassname.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModal<P0>(&self, hwndparent: P0) -> windows_core::Result<IADs>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateModal)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDsAdminCreateObj_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateModal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateModal: usize,
}
windows_core::imp::define_interface!(IDsAdminNewObj, IDsAdminNewObj_Vtbl, 0xf2573587_e6fc_11d2_82af_00c04f68928b);
impl core::ops::Deref for IDsAdminNewObj {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsAdminNewObj, windows_core::IUnknown);
impl IDsAdminNewObj {
    pub unsafe fn SetButtons<P0>(&self, ncurrindex: u32, bvalid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetButtons)(windows_core::Interface::as_raw(self), ncurrindex, bvalid.param().abi()).ok()
    }
    pub unsafe fn GetPageCounts(&self, pntotal: *mut i32, pnstartindex: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPageCounts)(windows_core::Interface::as_raw(self), pntotal, pnstartindex).ok()
    }
}
#[repr(C)]
pub struct IDsAdminNewObj_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetButtons: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetPageCounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsAdminNewObjExt, IDsAdminNewObjExt_Vtbl, 0x6088eae2_e7bf_11d2_82af_00c04f68928b);
impl core::ops::Deref for IDsAdminNewObjExt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsAdminNewObjExt, windows_core::IUnknown);
impl IDsAdminNewObjExt {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn Initialize<P0, P1, P2, P3>(&self, padscontainerobj: P0, padscopysource: P1, lpszclassname: P2, pdsadminnewobj: P3, pdispinfo: *mut DSA_NEWOBJ_DISPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IADsContainer>,
        P1: windows_core::Param<IADs>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IDsAdminNewObj>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), padscontainerobj.param().abi(), padscopysource.param().abi(), lpszclassname.param().abi(), pdsadminnewobj.param().abi(), pdispinfo).ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPages<P0>(&self, lpfnaddpage: super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, lparam: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).AddPages)(windows_core::Interface::as_raw(self), lpfnaddpage, lparam.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetObject<P0>(&self, padsobj: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IADs>,
    {
        (windows_core::Interface::vtable(self).SetObject)(windows_core::Interface::as_raw(self), padsobj.param().abi()).ok()
    }
    pub unsafe fn WriteData<P0>(&self, hwnd: P0, ucontext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).WriteData)(windows_core::Interface::as_raw(self), hwnd.param().abi(), ucontext).ok()
    }
    pub unsafe fn OnError<P0>(&self, hwnd: P0, hr: windows_core::HRESULT, ucontext: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hwnd.param().abi(), hr, ucontext).ok()
    }
    pub unsafe fn GetSummaryInfo(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSummaryInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrtext)).ok()
    }
}
#[repr(C)]
pub struct IDsAdminNewObjExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut DSA_NEWOBJ_DISPINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging")))]
    Initialize: usize,
    #[cfg(feature = "Win32_UI_Controls")]
    pub AddPages: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Controls::LPFNSVADDPROPSHEETPAGE, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    AddPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetObject: usize,
    pub WriteData: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub GetSummaryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsAdminNewObjPrimarySite, IDsAdminNewObjPrimarySite_Vtbl, 0xbe2b487e_f904_11d2_82b9_00c04f68928b);
impl core::ops::Deref for IDsAdminNewObjPrimarySite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsAdminNewObjPrimarySite, windows_core::IUnknown);
impl IDsAdminNewObjPrimarySite {
    pub unsafe fn CreateNew<P0>(&self, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CreateNew)(windows_core::Interface::as_raw(self), pszname.param().abi()).ok()
    }
    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDsAdminNewObjPrimarySite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateNew: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsAdminNotifyHandler, IDsAdminNotifyHandler_Vtbl, 0xe4a2b8b3_5a18_11d2_97c1_00a0c9a06d2d);
impl core::ops::Deref for IDsAdminNotifyHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsAdminNotifyHandler, windows_core::IUnknown);
impl IDsAdminNotifyHandler {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pextrainfo: P0, pueventflags: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pextrainfo.param().abi(), pueventflags).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Begin<P0, P1>(&self, uevent: u32, parg1: P0, parg2: P1, puflags: *mut u32, pbstr: *mut windows_core::BSTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDataObject>,
        P1: windows_core::Param<super::super::System::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self), uevent, parg1.param().abi(), parg2.param().abi(), puflags, core::mem::transmute(pbstr)).ok()
    }
    pub unsafe fn Notify(&self, nitem: u32, uflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Notify)(windows_core::Interface::as_raw(self), nitem, uflags).ok()
    }
    pub unsafe fn End(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDsAdminNotifyHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Begin: usize,
    pub Notify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsBrowseDomainTree, IDsBrowseDomainTree_Vtbl, 0x7cabcf1e_78f5_11d2_960c_00c04fa31a86);
impl core::ops::Deref for IDsBrowseDomainTree {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsBrowseDomainTree, windows_core::IUnknown);
impl IDsBrowseDomainTree {
    pub unsafe fn BrowseTo<P0>(&self, hwndparent: P0, ppsztargetpath: *mut windows_core::PWSTR, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).BrowseTo)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), ppsztargetpath, dwflags).ok()
    }
    pub unsafe fn GetDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDomains)(windows_core::Interface::as_raw(self), ppdomaintree, dwflags).ok()
    }
    pub unsafe fn FreeDomains(&self, ppdomaintree: *mut *mut DOMAIN_TREE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeDomains)(windows_core::Interface::as_raw(self), ppdomaintree).ok()
    }
    pub unsafe fn FlushCachedDomains(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FlushCachedDomains)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetComputer<P0, P1, P2>(&self, pszcomputername: P0, pszusername: P1, pszpassword: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetComputer)(windows_core::Interface::as_raw(self), pszcomputername.param().abi(), pszusername.param().abi(), pszpassword.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDsBrowseDomainTree_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BrowseTo: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetDomains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DOMAIN_TREE, u32) -> windows_core::HRESULT,
    pub FreeDomains: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut DOMAIN_TREE) -> windows_core::HRESULT,
    pub FlushCachedDomains: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComputer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDsDisplaySpecifier, IDsDisplaySpecifier_Vtbl, 0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
impl core::ops::Deref for IDsDisplaySpecifier {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsDisplaySpecifier, windows_core::IUnknown);
impl IDsDisplaySpecifier {
    pub unsafe fn SetServer<P0, P1, P2>(&self, pszserver: P0, pszusername: P1, pszpassword: P2, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), pszserver.param().abi(), pszusername.param().abi(), pszpassword.param().abi(), dwflags).ok()
    }
    pub unsafe fn SetLanguageID(&self, langid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLanguageID)(windows_core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn GetDisplaySpecifier<P0>(&self, pszobjectclass: P0, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetDisplaySpecifier)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), riid, ppv).ok()
    }
    pub unsafe fn GetIconLocation<P0>(&self, pszobjectclass: P0, dwflags: u32, pszbuffer: &mut [u16], presid: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetIconLocation)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), dwflags, core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), presid).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon<P0>(&self, pszobjectclass: P0, dwflags: u32, cxicon: i32, cyicon: i32) -> super::super::UI::WindowsAndMessaging::HICON
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), dwflags, cxicon, cyicon)
    }
    pub unsafe fn GetFriendlyClassName<P0>(&self, pszobjectclass: P0, pszbuffer: &mut [u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetFriendlyClassName)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFriendlyAttributeName<P0, P1>(&self, pszobjectclass: P0, pszattributename: P1, pszbuffer: &mut [u16]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetFriendlyAttributeName)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), pszattributename.param().abi(), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap()).ok()
    }
    pub unsafe fn IsClassContainer<P0, P1>(&self, pszobjectclass: P0, pszadspath: P1, dwflags: u32) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsClassContainer)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), pszadspath.param().abi(), dwflags)
    }
    pub unsafe fn GetClassCreationInfo<P0>(&self, pszobjectclass: P0, ppdscci: *mut *mut DSCLASSCREATIONINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetClassCreationInfo)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), ppdscci).ok()
    }
    pub unsafe fn EnumClassAttributes<P0, P1>(&self, pszobjectclass: P0, pcbenum: LPDSENUMATTRIBUTES, lparam: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).EnumClassAttributes)(windows_core::Interface::as_raw(self), pszobjectclass.param().abi(), pcbenum, lparam.param().abi()).ok()
    }
    pub unsafe fn GetAttributeADsType<P0>(&self, pszattributename: P0) -> ADSTYPE
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetAttributeADsType)(windows_core::Interface::as_raw(self), pszattributename.param().abi())
    }
}
#[repr(C)]
pub struct IDsDisplaySpecifier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetDisplaySpecifier: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIconLocation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, i32, i32) -> super::super::UI::WindowsAndMessaging::HICON,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetFriendlyClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub GetFriendlyAttributeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub IsClassContainer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> super::super::Foundation::BOOL,
    pub GetClassCreationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut DSCLASSCREATIONINFO) -> windows_core::HRESULT,
    pub EnumClassAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, LPDSENUMATTRIBUTES, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    pub GetAttributeADsType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> ADSTYPE,
}
windows_core::imp::define_interface!(IDsObjectPicker, IDsObjectPicker_Vtbl, 0x0c87e64e_3b7a_11d2_b9e0_00c04fd8dbf7);
impl core::ops::Deref for IDsObjectPicker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsObjectPicker, windows_core::IUnknown);
impl IDsObjectPicker {
    pub unsafe fn Initialize(&self, pinitinfo: *mut DSOP_INIT_INFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pinitinfo).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InvokeDialog<P0>(&self, hwndparent: P0) -> windows_core::Result<super::super::System::Com::IDataObject>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InvokeDialog)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDsObjectPicker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DSOP_INIT_INFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InvokeDialog: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InvokeDialog: usize,
}
windows_core::imp::define_interface!(IDsObjectPickerCredentials, IDsObjectPickerCredentials_Vtbl, 0xe2d3ec9b_d041_445a_8f16_4748de8fb1cf);
impl core::ops::Deref for IDsObjectPickerCredentials {
    type Target = IDsObjectPicker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDsObjectPickerCredentials, windows_core::IUnknown, IDsObjectPicker);
impl IDsObjectPickerCredentials {
    pub unsafe fn SetCredentials<P0, P1>(&self, szusername: P0, szpassword: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), szusername.param().abi(), szpassword.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDsObjectPickerCredentials_Vtbl {
    pub base__: IDsObjectPicker_Vtbl,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPersistQuery, IPersistQuery_Vtbl, 0x1a3114b8_a62e_11d0_a6c5_00a0c906af45);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPersistQuery {
    type Target = super::super::System::Com::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPersistQuery, windows_core::IUnknown, super::super::System::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl IPersistQuery {
    pub unsafe fn WriteString<P0, P1, P2>(&self, psection: P0, pvaluename: P1, pvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteString)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), pvalue.param().abi()).ok()
    }
    pub unsafe fn ReadString<P0, P1>(&self, psection: P0, pvaluename: P1, pbuffer: windows_core::PWSTR, cchbuffer: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReadString)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), core::mem::transmute(pbuffer), cchbuffer).ok()
    }
    pub unsafe fn WriteInt<P0, P1>(&self, psection: P0, pvaluename: P1, value: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteInt)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), value).ok()
    }
    pub unsafe fn ReadInt<P0, P1>(&self, psection: P0, pvaluename: P1, pvalue: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReadInt)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), pvalue).ok()
    }
    pub unsafe fn WriteStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).WriteStruct)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn ReadStruct<P0, P1>(&self, psection: P0, pvaluename: P1, pstruct: *mut core::ffi::c_void, cbstruct: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReadStruct)(windows_core::Interface::as_raw(self), psection.param().abi(), pvaluename.param().abi(), pstruct, cbstruct).ok()
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistQuery_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    pub WriteString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PWSTR, i32) -> windows_core::HRESULT,
    pub WriteInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub ReadInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    pub WriteStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReadStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrivateDispatch, IPrivateDispatch_Vtbl, 0x86ab4bbe_65f6_11d1_8c13_00c04fd8d503);
impl core::ops::Deref for IPrivateDispatch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrivateDispatch, windows_core::IUnknown);
impl IPrivateDispatch {
    pub unsafe fn ADSIInitializeDispatchManager(&self, dwextensionid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ADSIInitializeDispatchManager)(windows_core::Interface::as_raw(self), dwextensionid).ok()
    }
    pub unsafe fn ADSIGetTypeInfoCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADSIGetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ADSIGetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<super::super::System::Com::ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADSIGetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ADSIGetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const *const u16, cnames: u32, lcid: u32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ADSIGetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ADSIInvoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut super::super::System::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ADSIInvoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult), pexcepinfo, puargerr).ok()
    }
}
#[repr(C)]
pub struct IPrivateDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ADSIInitializeDispatchManager: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ADSIGetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ADSIGetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ADSIGetTypeInfo: usize,
    pub ADSIGetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const *const u16, u32, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ADSIInvoke: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, u32, u16, *const super::super::System::Com::DISPPARAMS, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::super::System::Com::EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ADSIInvoke: usize,
}
windows_core::imp::define_interface!(IPrivateUnknown, IPrivateUnknown_Vtbl, 0x89126bab_6ead_11d1_8c18_00c04fd8d503);
impl core::ops::Deref for IPrivateUnknown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrivateUnknown, windows_core::IUnknown);
impl IPrivateUnknown {
    pub unsafe fn ADSIInitializeObject<P0, P1>(&self, lpszusername: P0, lpszpassword: P1, lnreserved: i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ADSIInitializeObject)(windows_core::Interface::as_raw(self), lpszusername.param().abi(), lpszpassword.param().abi(), lnreserved).ok()
    }
    pub unsafe fn ADSIReleaseObject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ADSIReleaseObject)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPrivateUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ADSIInitializeObject: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32) -> windows_core::HRESULT,
    pub ADSIReleaseObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IQueryForm, IQueryForm_Vtbl, 0x8cfcee30_39bd_11d0_b8d1_00a024ab2dbb);
impl core::ops::Deref for IQueryForm {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQueryForm, windows_core::IUnknown);
impl IQueryForm {
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0>(&self, hkform: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Registry::HKEY>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), hkform.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn AddForms<P0>(&self, paddformsproc: LPCQADDFORMSPROC, lparam: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).AddForms)(windows_core::Interface::as_raw(self), paddformsproc, lparam.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn AddPages<P0>(&self, paddpagesproc: LPCQADDPAGESPROC, lparam: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).AddPages)(windows_core::Interface::as_raw(self), paddpagesproc, lparam.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IQueryForm_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Registry")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    Initialize: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub AddForms: unsafe extern "system" fn(*mut core::ffi::c_void, LPCQADDFORMSPROC, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    AddForms: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub AddPages: unsafe extern "system" fn(*mut core::ffi::c_void, LPCQADDPAGESPROC, super::super::Foundation::LPARAM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    AddPages: usize,
}
pub const ACTRL_DS_CONTROL_ACCESS: u32 = 256u32;
pub const ACTRL_DS_CREATE_CHILD: u32 = 1u32;
pub const ACTRL_DS_DELETE_CHILD: u32 = 2u32;
pub const ACTRL_DS_DELETE_TREE: u32 = 64u32;
pub const ACTRL_DS_LIST: u32 = 4u32;
pub const ACTRL_DS_LIST_OBJECT: u32 = 128u32;
pub const ACTRL_DS_OPEN: u32 = 0u32;
pub const ACTRL_DS_READ_PROP: u32 = 16u32;
pub const ACTRL_DS_SELF: u32 = 8u32;
pub const ACTRL_DS_WRITE_PROP: u32 = 32u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_MUTUAL_AUTH_REQUIRED: u32 = 2u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE: u32 = 1u32;
pub const ADAM_REPL_AUTHENTICATION_MODE_NEGOTIATE_PASS_THROUGH: u32 = 0u32;
pub const ADAM_SCP_FSMO_NAMING_STRING: windows_core::PCSTR = windows_core::s!("naming");
pub const ADAM_SCP_FSMO_NAMING_STRING_W: windows_core::PCWSTR = windows_core::w!("naming");
pub const ADAM_SCP_FSMO_SCHEMA_STRING: windows_core::PCSTR = windows_core::s!("schema");
pub const ADAM_SCP_FSMO_SCHEMA_STRING_W: windows_core::PCWSTR = windows_core::w!("schema");
pub const ADAM_SCP_FSMO_STRING: windows_core::PCSTR = windows_core::s!("fsmo:");
pub const ADAM_SCP_FSMO_STRING_W: windows_core::PCWSTR = windows_core::w!("fsmo:");
pub const ADAM_SCP_INSTANCE_NAME_STRING: windows_core::PCSTR = windows_core::s!("instance:");
pub const ADAM_SCP_INSTANCE_NAME_STRING_W: windows_core::PCWSTR = windows_core::w!("instance:");
pub const ADAM_SCP_PARTITION_STRING: windows_core::PCSTR = windows_core::s!("partition:");
pub const ADAM_SCP_PARTITION_STRING_W: windows_core::PCWSTR = windows_core::w!("partition:");
pub const ADAM_SCP_SITE_NAME_STRING: windows_core::PCSTR = windows_core::s!("site:");
pub const ADAM_SCP_SITE_NAME_STRING_W: windows_core::PCWSTR = windows_core::w!("site:");
pub const ADSIPROP_ADSIFLAG: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(12i32);
pub const ADSIPROP_ASYNCHRONOUS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(0i32);
pub const ADSIPROP_ATTRIBTYPES_ONLY: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(4i32);
pub const ADSIPROP_CACHE_RESULTS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(11i32);
pub const ADSIPROP_CHASE_REFERRALS: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(9i32);
pub const ADSIPROP_DEREF_ALIASES: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(1i32);
pub const ADSIPROP_PAGED_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(8i32);
pub const ADSIPROP_PAGESIZE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(7i32);
pub const ADSIPROP_SEARCH_SCOPE: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(5i32);
pub const ADSIPROP_SIZE_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(2i32);
pub const ADSIPROP_SORT_ON: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(10i32);
pub const ADSIPROP_TIMEOUT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(6i32);
pub const ADSIPROP_TIME_LIMIT: ADS_PREFERENCES_ENUM = ADS_PREFERENCES_ENUM(3i32);
pub const ADSI_DIALECT_LDAP: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(0i32);
pub const ADSI_DIALECT_SQL: ADSI_DIALECT_ENUM = ADSI_DIALECT_ENUM(1i32);
pub const ADSTYPE_BACKLINK: ADSTYPE = ADSTYPE(18i32);
pub const ADSTYPE_BOOLEAN: ADSTYPE = ADSTYPE(6i32);
pub const ADSTYPE_CASEIGNORE_LIST: ADSTYPE = ADSTYPE(13i32);
pub const ADSTYPE_CASE_EXACT_STRING: ADSTYPE = ADSTYPE(2i32);
pub const ADSTYPE_CASE_IGNORE_STRING: ADSTYPE = ADSTYPE(3i32);
pub const ADSTYPE_DN_STRING: ADSTYPE = ADSTYPE(1i32);
pub const ADSTYPE_DN_WITH_BINARY: ADSTYPE = ADSTYPE(27i32);
pub const ADSTYPE_DN_WITH_STRING: ADSTYPE = ADSTYPE(28i32);
pub const ADSTYPE_EMAIL: ADSTYPE = ADSTYPE(24i32);
pub const ADSTYPE_FAXNUMBER: ADSTYPE = ADSTYPE(23i32);
pub const ADSTYPE_HOLD: ADSTYPE = ADSTYPE(20i32);
pub const ADSTYPE_INTEGER: ADSTYPE = ADSTYPE(7i32);
pub const ADSTYPE_INVALID: ADSTYPE = ADSTYPE(0i32);
pub const ADSTYPE_LARGE_INTEGER: ADSTYPE = ADSTYPE(10i32);
pub const ADSTYPE_NETADDRESS: ADSTYPE = ADSTYPE(21i32);
pub const ADSTYPE_NT_SECURITY_DESCRIPTOR: ADSTYPE = ADSTYPE(25i32);
pub const ADSTYPE_NUMERIC_STRING: ADSTYPE = ADSTYPE(5i32);
pub const ADSTYPE_OBJECT_CLASS: ADSTYPE = ADSTYPE(12i32);
pub const ADSTYPE_OCTET_LIST: ADSTYPE = ADSTYPE(14i32);
pub const ADSTYPE_OCTET_STRING: ADSTYPE = ADSTYPE(8i32);
pub const ADSTYPE_PATH: ADSTYPE = ADSTYPE(15i32);
pub const ADSTYPE_POSTALADDRESS: ADSTYPE = ADSTYPE(16i32);
pub const ADSTYPE_PRINTABLE_STRING: ADSTYPE = ADSTYPE(4i32);
pub const ADSTYPE_PROV_SPECIFIC: ADSTYPE = ADSTYPE(11i32);
pub const ADSTYPE_REPLICAPOINTER: ADSTYPE = ADSTYPE(22i32);
pub const ADSTYPE_TIMESTAMP: ADSTYPE = ADSTYPE(17i32);
pub const ADSTYPE_TYPEDNAME: ADSTYPE = ADSTYPE(19i32);
pub const ADSTYPE_UNKNOWN: ADSTYPE = ADSTYPE(26i32);
pub const ADSTYPE_UTC_TIME: ADSTYPE = ADSTYPE(9i32);
pub const ADS_ACEFLAG_FAILED_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(128i32);
pub const ADS_ACEFLAG_INHERITED_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(16i32);
pub const ADS_ACEFLAG_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(2i32);
pub const ADS_ACEFLAG_INHERIT_ONLY_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(8i32);
pub const ADS_ACEFLAG_NO_PROPAGATE_INHERIT_ACE: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(4i32);
pub const ADS_ACEFLAG_SUCCESSFUL_ACCESS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(64i32);
pub const ADS_ACEFLAG_VALID_INHERIT_FLAGS: ADS_ACEFLAG_ENUM = ADS_ACEFLAG_ENUM(31i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(0i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(9i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(11i32);
pub const ADS_ACETYPE_ACCESS_ALLOWED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(5i32);
pub const ADS_ACETYPE_ACCESS_DENIED: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(1i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(10i32);
pub const ADS_ACETYPE_ACCESS_DENIED_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(12i32);
pub const ADS_ACETYPE_ACCESS_DENIED_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(6i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(14i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(16i32);
pub const ADS_ACETYPE_SYSTEM_ALARM_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(8i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(2i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(13i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_CALLBACK_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(15i32);
pub const ADS_ACETYPE_SYSTEM_AUDIT_OBJECT: ADS_ACETYPE_ENUM = ADS_ACETYPE_ENUM(7i32);
pub const ADS_ATTR_APPEND: u32 = 3u32;
pub const ADS_ATTR_CLEAR: u32 = 1u32;
pub const ADS_ATTR_DELETE: u32 = 4u32;
pub const ADS_ATTR_UPDATE: u32 = 2u32;
pub const ADS_AUTH_RESERVED: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2147483648u32);
pub const ADS_CHASE_REFERRALS_ALWAYS: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(96i32);
pub const ADS_CHASE_REFERRALS_EXTERNAL: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(64i32);
pub const ADS_CHASE_REFERRALS_NEVER: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(0i32);
pub const ADS_CHASE_REFERRALS_SUBORDINATE: ADS_CHASE_REFERRALS_ENUM = ADS_CHASE_REFERRALS_ENUM(32i32);
pub const ADS_DEREF_ALWAYS: ADS_DEREFENUM = ADS_DEREFENUM(3i32);
pub const ADS_DEREF_FINDING: ADS_DEREFENUM = ADS_DEREFENUM(2i32);
pub const ADS_DEREF_NEVER: ADS_DEREFENUM = ADS_DEREFENUM(0i32);
pub const ADS_DEREF_SEARCHING: ADS_DEREFENUM = ADS_DEREFENUM(1i32);
pub const ADS_DISPLAY_FULL: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(1i32);
pub const ADS_DISPLAY_VALUE_ONLY: ADS_DISPLAY_ENUM = ADS_DISPLAY_ENUM(2i32);
pub const ADS_ESCAPEDMODE_DEFAULT: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(1i32);
pub const ADS_ESCAPEDMODE_OFF: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(3i32);
pub const ADS_ESCAPEDMODE_OFF_EX: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(4i32);
pub const ADS_ESCAPEDMODE_ON: ADS_ESCAPE_MODE_ENUM = ADS_ESCAPE_MODE_ENUM(2i32);
pub const ADS_EXT_INITCREDENTIALS: u32 = 1u32;
pub const ADS_EXT_INITIALIZE_COMPLETE: u32 = 2u32;
pub const ADS_EXT_MAXEXTDISPID: u32 = 16777215u32;
pub const ADS_EXT_MINEXTDISPID: u32 = 1u32;
pub const ADS_FAST_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(32u32);
pub const ADS_FLAG_INHERITED_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(2i32);
pub const ADS_FLAG_OBJECT_TYPE_PRESENT: ADS_FLAGTYPE_ENUM = ADS_FLAGTYPE_ENUM(1i32);
pub const ADS_FORMAT_LEAF: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(11i32);
pub const ADS_FORMAT_PROVIDER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(10i32);
pub const ADS_FORMAT_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(9i32);
pub const ADS_FORMAT_WINDOWS: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(1i32);
pub const ADS_FORMAT_WINDOWS_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(3i32);
pub const ADS_FORMAT_WINDOWS_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(2i32);
pub const ADS_FORMAT_WINDOWS_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(4i32);
pub const ADS_FORMAT_X500: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(5i32);
pub const ADS_FORMAT_X500_DN: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(7i32);
pub const ADS_FORMAT_X500_NO_SERVER: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(6i32);
pub const ADS_FORMAT_X500_PARENT: ADS_FORMAT_ENUM = ADS_FORMAT_ENUM(8i32);
pub const ADS_GROUP_TYPE_DOMAIN_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_GLOBAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(2i32);
pub const ADS_GROUP_TYPE_LOCAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(4i32);
pub const ADS_GROUP_TYPE_SECURITY_ENABLED: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(-2147483648i32);
pub const ADS_GROUP_TYPE_UNIVERSAL_GROUP: ADS_GROUP_TYPE_ENUM = ADS_GROUP_TYPE_ENUM(8i32);
pub const ADS_NAME_INITTYPE_DOMAIN: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(1i32);
pub const ADS_NAME_INITTYPE_GC: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(3i32);
pub const ADS_NAME_INITTYPE_SERVER: ADS_NAME_INITTYPE_ENUM = ADS_NAME_INITTYPE_ENUM(2i32);
pub const ADS_NAME_TYPE_1779: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(1i32);
pub const ADS_NAME_TYPE_CANONICAL: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(2i32);
pub const ADS_NAME_TYPE_CANONICAL_EX: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(10i32);
pub const ADS_NAME_TYPE_DISPLAY: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(4i32);
pub const ADS_NAME_TYPE_DOMAIN_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(5i32);
pub const ADS_NAME_TYPE_ENTERPRISE_SIMPLE: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(6i32);
pub const ADS_NAME_TYPE_GUID: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(7i32);
pub const ADS_NAME_TYPE_NT4: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(3i32);
pub const ADS_NAME_TYPE_SERVICE_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(11i32);
pub const ADS_NAME_TYPE_SID_OR_SID_HISTORY_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(12i32);
pub const ADS_NAME_TYPE_UNKNOWN: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(8i32);
pub const ADS_NAME_TYPE_USER_PRINCIPAL_NAME: ADS_NAME_TYPE_ENUM = ADS_NAME_TYPE_ENUM(9i32);
pub const ADS_NO_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(16u32);
pub const ADS_NO_REFERRAL_CHASING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1024u32);
pub const ADS_OPTION_ACCUMULATIVE_MODIFICATION: ADS_OPTION_ENUM = ADS_OPTION_ENUM(8i32);
pub const ADS_OPTION_MUTUAL_AUTH_STATUS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(4i32);
pub const ADS_OPTION_PAGE_SIZE: ADS_OPTION_ENUM = ADS_OPTION_ENUM(2i32);
pub const ADS_OPTION_PASSWORD_METHOD: ADS_OPTION_ENUM = ADS_OPTION_ENUM(7i32);
pub const ADS_OPTION_PASSWORD_PORTNUMBER: ADS_OPTION_ENUM = ADS_OPTION_ENUM(6i32);
pub const ADS_OPTION_QUOTA: ADS_OPTION_ENUM = ADS_OPTION_ENUM(5i32);
pub const ADS_OPTION_REFERRALS: ADS_OPTION_ENUM = ADS_OPTION_ENUM(1i32);
pub const ADS_OPTION_SECURITY_MASK: ADS_OPTION_ENUM = ADS_OPTION_ENUM(3i32);
pub const ADS_OPTION_SERVERNAME: ADS_OPTION_ENUM = ADS_OPTION_ENUM(0i32);
pub const ADS_OPTION_SKIP_SID_LOOKUP: ADS_OPTION_ENUM = ADS_OPTION_ENUM(9i32);
pub const ADS_PASSWORD_ENCODE_CLEAR: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(1i32);
pub const ADS_PASSWORD_ENCODE_REQUIRE_SSL: ADS_PASSWORD_ENCODING_ENUM = ADS_PASSWORD_ENCODING_ENUM(0i32);
pub const ADS_PATH_FILE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(1i32);
pub const ADS_PATH_FILESHARE: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(2i32);
pub const ADS_PATH_REGISTRY: ADS_PATHTYPE_ENUM = ADS_PATHTYPE_ENUM(3i32);
pub const ADS_PROMPT_CREDENTIALS: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(8u32);
pub const ADS_PROPERTY_APPEND: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(3i32);
pub const ADS_PROPERTY_CLEAR: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(1i32);
pub const ADS_PROPERTY_DELETE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(4i32);
pub const ADS_PROPERTY_UPDATE: ADS_PROPERTY_OPERATION_ENUM = ADS_PROPERTY_OPERATION_ENUM(2i32);
pub const ADS_READONLY_SERVER: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(4u32);
pub const ADS_RIGHT_ACCESS_SYSTEM_SECURITY: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16777216i32);
pub const ADS_RIGHT_ACTRL_DS_LIST: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(4i32);
pub const ADS_RIGHT_DELETE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(65536i32);
pub const ADS_RIGHT_DS_CONTROL_ACCESS: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(256i32);
pub const ADS_RIGHT_DS_CREATE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1i32);
pub const ADS_RIGHT_DS_DELETE_CHILD: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(2i32);
pub const ADS_RIGHT_DS_DELETE_TREE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(64i32);
pub const ADS_RIGHT_DS_LIST_OBJECT: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(128i32);
pub const ADS_RIGHT_DS_READ_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(16i32);
pub const ADS_RIGHT_DS_SELF: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(8i32);
pub const ADS_RIGHT_DS_WRITE_PROP: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(32i32);
pub const ADS_RIGHT_GENERIC_ALL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(268435456i32);
pub const ADS_RIGHT_GENERIC_EXECUTE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(536870912i32);
pub const ADS_RIGHT_GENERIC_READ: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(-2147483648i32);
pub const ADS_RIGHT_GENERIC_WRITE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1073741824i32);
pub const ADS_RIGHT_READ_CONTROL: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(131072i32);
pub const ADS_RIGHT_SYNCHRONIZE: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(1048576i32);
pub const ADS_RIGHT_WRITE_DAC: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(262144i32);
pub const ADS_RIGHT_WRITE_OWNER: ADS_RIGHTS_ENUM = ADS_RIGHTS_ENUM(524288i32);
pub const ADS_SCOPE_BASE: ADS_SCOPEENUM = ADS_SCOPEENUM(0i32);
pub const ADS_SCOPE_ONELEVEL: ADS_SCOPEENUM = ADS_SCOPEENUM(1i32);
pub const ADS_SCOPE_SUBTREE: ADS_SCOPEENUM = ADS_SCOPEENUM(2i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1024i32);
pub const ADS_SD_CONTROL_SE_DACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(256i32);
pub const ADS_SD_CONTROL_SE_DACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8i32);
pub const ADS_SD_CONTROL_SE_DACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4i32);
pub const ADS_SD_CONTROL_SE_DACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(4096i32);
pub const ADS_SD_CONTROL_SE_GROUP_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2i32);
pub const ADS_SD_CONTROL_SE_OWNER_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(1i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERITED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(2048i32);
pub const ADS_SD_CONTROL_SE_SACL_AUTO_INHERIT_REQ: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(512i32);
pub const ADS_SD_CONTROL_SE_SACL_DEFAULTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32i32);
pub const ADS_SD_CONTROL_SE_SACL_PRESENT: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(16i32);
pub const ADS_SD_CONTROL_SE_SACL_PROTECTED: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(8192i32);
pub const ADS_SD_CONTROL_SE_SELF_RELATIVE: ADS_SD_CONTROL_ENUM = ADS_SD_CONTROL_ENUM(32768i32);
pub const ADS_SD_FORMAT_HEXSTRING: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(3i32);
pub const ADS_SD_FORMAT_IID: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(1i32);
pub const ADS_SD_FORMAT_RAW: ADS_SD_FORMAT_ENUM = ADS_SD_FORMAT_ENUM(2i32);
pub const ADS_SD_REVISION_DS: ADS_SD_REVISION_ENUM = ADS_SD_REVISION_ENUM(4i32);
pub const ADS_SEARCHPREF_ASYNCHRONOUS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(0i32);
pub const ADS_SEARCHPREF_ATTRIBTYPES_ONLY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(4i32);
pub const ADS_SEARCHPREF_ATTRIBUTE_QUERY: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(15i32);
pub const ADS_SEARCHPREF_CACHE_RESULTS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(11i32);
pub const ADS_SEARCHPREF_CHASE_REFERRALS: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(9i32);
pub const ADS_SEARCHPREF_DEREF_ALIASES: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(1i32);
pub const ADS_SEARCHPREF_DIRSYNC: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(12i32);
pub const ADS_SEARCHPREF_DIRSYNC_FLAG: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(17i32);
pub const ADS_SEARCHPREF_EXTENDED_DN: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(18i32);
pub const ADS_SEARCHPREF_PAGED_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(8i32);
pub const ADS_SEARCHPREF_PAGESIZE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(7i32);
pub const ADS_SEARCHPREF_SEARCH_SCOPE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(5i32);
pub const ADS_SEARCHPREF_SECURITY_MASK: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(16i32);
pub const ADS_SEARCHPREF_SIZE_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(2i32);
pub const ADS_SEARCHPREF_SORT_ON: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(10i32);
pub const ADS_SEARCHPREF_TIMEOUT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(6i32);
pub const ADS_SEARCHPREF_TIME_LIMIT: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(3i32);
pub const ADS_SEARCHPREF_TOMBSTONE: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(13i32);
pub const ADS_SEARCHPREF_VLV: ADS_SEARCHPREF_ENUM = ADS_SEARCHPREF_ENUM(14i32);
pub const ADS_SECURE_AUTHENTICATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(1u32);
pub const ADS_SECURITY_INFO_DACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(4i32);
pub const ADS_SECURITY_INFO_GROUP: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(2i32);
pub const ADS_SECURITY_INFO_OWNER: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(1i32);
pub const ADS_SECURITY_INFO_SACL: ADS_SECURITY_INFO_ENUM = ADS_SECURITY_INFO_ENUM(8i32);
pub const ADS_SERVER_BIND: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(512u32);
pub const ADS_SETTYPE_DN: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(4i32);
pub const ADS_SETTYPE_FULL: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(1i32);
pub const ADS_SETTYPE_PROVIDER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(2i32);
pub const ADS_SETTYPE_SERVER: ADS_SETTYPE_ENUM = ADS_SETTYPE_ENUM(3i32);
pub const ADS_STATUS_INVALID_SEARCHPREF: ADS_STATUSENUM = ADS_STATUSENUM(1i32);
pub const ADS_STATUS_INVALID_SEARCHPREFVALUE: ADS_STATUSENUM = ADS_STATUSENUM(2i32);
pub const ADS_STATUS_S_OK: ADS_STATUSENUM = ADS_STATUSENUM(0i32);
pub const ADS_SYSTEMFLAG_ATTR_IS_CONSTRUCTED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(4i32);
pub const ADS_SYSTEMFLAG_ATTR_NOT_REPLICATED: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_LIMITED_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(268435456i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(536870912i32);
pub const ADS_SYSTEMFLAG_CONFIG_ALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1073741824i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_DOMAIN: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(2i32);
pub const ADS_SYSTEMFLAG_CR_NTDS_NC: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(1i32);
pub const ADS_SYSTEMFLAG_DISALLOW_DELETE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(-2147483648i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_MOVE: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(67108864i32);
pub const ADS_SYSTEMFLAG_DOMAIN_DISALLOW_RENAME: ADS_SYSTEMFLAG_ENUM = ADS_SYSTEMFLAG_ENUM(134217728i32);
pub const ADS_UF_ACCOUNTDISABLE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2i32);
pub const ADS_UF_DONT_EXPIRE_PASSWD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(65536i32);
pub const ADS_UF_DONT_REQUIRE_PREAUTH: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4194304i32);
pub const ADS_UF_ENCRYPTED_TEXT_PASSWORD_ALLOWED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(128i32);
pub const ADS_UF_HOMEDIR_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8i32);
pub const ADS_UF_INTERDOMAIN_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2048i32);
pub const ADS_UF_LOCKOUT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16i32);
pub const ADS_UF_MNS_LOGON_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(131072i32);
pub const ADS_UF_NORMAL_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(512i32);
pub const ADS_UF_NOT_DELEGATED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1048576i32);
pub const ADS_UF_PASSWD_CANT_CHANGE: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(64i32);
pub const ADS_UF_PASSWD_NOTREQD: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(32i32);
pub const ADS_UF_PASSWORD_EXPIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8388608i32);
pub const ADS_UF_SCRIPT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(1i32);
pub const ADS_UF_SERVER_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(8192i32);
pub const ADS_UF_SMARTCARD_REQUIRED: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(262144i32);
pub const ADS_UF_TEMP_DUPLICATE_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(256i32);
pub const ADS_UF_TRUSTED_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(524288i32);
pub const ADS_UF_TRUSTED_TO_AUTHENTICATE_FOR_DELEGATION: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(16777216i32);
pub const ADS_UF_USE_DES_KEY_ONLY: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(2097152i32);
pub const ADS_UF_WORKSTATION_TRUST_ACCOUNT: ADS_USER_FLAG_ENUM = ADS_USER_FLAG_ENUM(4096i32);
pub const ADS_USE_DELEGATION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(256u32);
pub const ADS_USE_ENCRYPTION: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const ADS_USE_SEALING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(128u32);
pub const ADS_USE_SIGNING: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(64u32);
pub const ADS_USE_SSL: ADS_AUTHENTICATION_ENUM = ADS_AUTHENTICATION_ENUM(2u32);
pub const CFSTR_DSDISPLAYSPECOPTIONS: windows_core::PCWSTR = windows_core::w!("DsDisplaySpecOptions");
pub const CFSTR_DSOBJECTNAMES: windows_core::PCWSTR = windows_core::w!("DsObjectNames");
pub const CFSTR_DSOP_DS_SELECTION_LIST: windows_core::PCWSTR = windows_core::w!("CFSTR_DSOP_DS_SELECTION_LIST");
pub const CFSTR_DSPROPERTYPAGEINFO: windows_core::PCWSTR = windows_core::w!("DsPropPageInfo");
pub const CFSTR_DSQUERYPARAMS: windows_core::PCWSTR = windows_core::w!("DsQueryParameters");
pub const CFSTR_DSQUERYSCOPE: windows_core::PCWSTR = windows_core::w!("DsQueryScope");
pub const CFSTR_DS_DISPLAY_SPEC_OPTIONS: windows_core::PCWSTR = windows_core::w!("DsDisplaySpecOptions");
pub const CLSID_CommonQuery: windows_core::GUID = windows_core::GUID::from_u128(0x83bc5ec0_6f2a_11d0_a1c4_00aa00c16e65);
pub const CLSID_DsAdminCreateObj: windows_core::GUID = windows_core::GUID::from_u128(0xe301a009_f901_11d2_82b9_00c04f68928b);
pub const CLSID_DsDisplaySpecifier: windows_core::GUID = windows_core::GUID::from_u128(0x1ab4a8c0_6a0b_11d2_ad49_00c04fa31a86);
pub const CLSID_DsDomainTreeBrowser: windows_core::GUID = windows_core::GUID::from_u128(0x1698790a_e2b4_11d0_b0b1_00c04fd8dca6);
pub const CLSID_DsFindAdvanced: windows_core::GUID = windows_core::GUID::from_u128(0x83ee3fe3_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindComputer: windows_core::GUID = windows_core::GUID::from_u128(0x16006700_87ad_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindContainer: windows_core::GUID = windows_core::GUID::from_u128(0xc1b3cbf2_886a_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindDomainController: windows_core::GUID = windows_core::GUID::from_u128(0x538c7b7e_d25e_11d0_9742_00a0c906af45);
pub const CLSID_DsFindFrsMembers: windows_core::GUID = windows_core::GUID::from_u128(0x94ce4b18_b3d3_11d1_b9b4_00c04fd8d5b0);
pub const CLSID_DsFindObjects: windows_core::GUID = windows_core::GUID::from_u128(0x83ee3fe1_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindPeople: windows_core::GUID = windows_core::GUID::from_u128(0x83ee3fe2_57d9_11d0_b932_00a024ab2dbb);
pub const CLSID_DsFindPrinter: windows_core::GUID = windows_core::GUID::from_u128(0xb577f070_7ee2_11d0_913f_00aa00c16e65);
pub const CLSID_DsFindVolume: windows_core::GUID = windows_core::GUID::from_u128(0xc1b3cbf1_886a_11d0_9140_00aa00c16e65);
pub const CLSID_DsFindWriteableDomainController: windows_core::GUID = windows_core::GUID::from_u128(0x7cbef079_aa84_444b_bc70_68e41283eabc);
pub const CLSID_DsFolderProperties: windows_core::GUID = windows_core::GUID::from_u128(0x9e51e0d0_6e0f_11d2_9601_00c04fa31a86);
pub const CLSID_DsObjectPicker: windows_core::GUID = windows_core::GUID::from_u128(0x17d6ccd8_3b7b_11d2_b9e0_00c04fd8dbf7);
pub const CLSID_DsPropertyPages: windows_core::GUID = windows_core::GUID::from_u128(0x0d45d530_764b_11d0_a1ca_00aa00c16e65);
pub const CLSID_DsQuery: windows_core::GUID = windows_core::GUID::from_u128(0x8a23e65e_31c2_11d0_891c_00a024ab2dbb);
pub const CLSID_MicrosoftDS: windows_core::GUID = windows_core::GUID::from_u128(0xfe1290f0_cfbd_11cf_a330_00aa00c16e65);
pub const CQFF_ISOPTIONAL: u32 = 2u32;
pub const CQFF_NOGLOBALPAGES: u32 = 1u32;
pub const CQPM_CLEARFORM: u32 = 6u32;
pub const CQPM_ENABLE: u32 = 3u32;
pub const CQPM_GETPARAMETERS: u32 = 5u32;
pub const CQPM_HANDLERSPECIFIC: u32 = 268435456u32;
pub const CQPM_HELP: u32 = 8u32;
pub const CQPM_INITIALIZE: u32 = 1u32;
pub const CQPM_PERSIST: u32 = 7u32;
pub const CQPM_RELEASE: u32 = 2u32;
pub const CQPM_SETDEFAULTPARAMETERS: u32 = 9u32;
pub const DBDTF_RETURNEXTERNAL: u32 = 4u32;
pub const DBDTF_RETURNFQDN: u32 = 1u32;
pub const DBDTF_RETURNINBOUND: u32 = 8u32;
pub const DBDTF_RETURNINOUTBOUND: u32 = 16u32;
pub const DBDTF_RETURNMIXEDDOMAINS: u32 = 2u32;
pub const DSA_NEWOBJ_CTX_CLEANUP: u32 = 4u32;
pub const DSA_NEWOBJ_CTX_COMMIT: u32 = 2u32;
pub const DSA_NEWOBJ_CTX_POSTCOMMIT: u32 = 3u32;
pub const DSA_NEWOBJ_CTX_PRECOMMIT: u32 = 1u32;
pub const DSA_NOTIFY_DEL: u32 = 1u32;
pub const DSA_NOTIFY_FLAG_ADDITIONAL_DATA: u32 = 2u32;
pub const DSA_NOTIFY_FLAG_FORCE_ADDITIONAL_DATA: u32 = 1u32;
pub const DSA_NOTIFY_MOV: u32 = 4u32;
pub const DSA_NOTIFY_PROP: u32 = 8u32;
pub const DSA_NOTIFY_REN: u32 = 2u32;
pub const DSBF_DISPLAYNAME: u32 = 4u32;
pub const DSBF_ICONLOCATION: u32 = 2u32;
pub const DSBF_STATE: u32 = 1u32;
pub const DSBID_BANNER: u32 = 256u32;
pub const DSBID_CONTAINERLIST: u32 = 257u32;
pub const DSBI_CHECKBOXES: u32 = 256u32;
pub const DSBI_DONTSIGNSEAL: u32 = 33554432u32;
pub const DSBI_ENTIREDIRECTORY: u32 = 589824u32;
pub const DSBI_EXPANDONOPEN: u32 = 262144u32;
pub const DSBI_HASCREDENTIALS: u32 = 2097152u32;
pub const DSBI_IGNORETREATASLEAF: u32 = 4194304u32;
pub const DSBI_INCLUDEHIDDEN: u32 = 131072u32;
pub const DSBI_NOBUTTONS: u32 = 1u32;
pub const DSBI_NOLINES: u32 = 2u32;
pub const DSBI_NOLINESATROOT: u32 = 4u32;
pub const DSBI_NOROOT: u32 = 65536u32;
pub const DSBI_RETURNOBJECTCLASS: u32 = 16777216u32;
pub const DSBI_RETURN_FORMAT: u32 = 1048576u32;
pub const DSBI_SIMPLEAUTHENTICATE: u32 = 8388608u32;
pub const DSBM_CHANGEIMAGESTATE: u32 = 102u32;
pub const DSBM_CONTEXTMENU: u32 = 104u32;
pub const DSBM_HELP: u32 = 103u32;
pub const DSBM_QUERYINSERT: u32 = 100u32;
pub const DSBM_QUERYINSERTA: u32 = 101u32;
pub const DSBM_QUERYINSERTW: u32 = 100u32;
pub const DSBS_CHECKED: u32 = 1u32;
pub const DSBS_HIDDEN: u32 = 2u32;
pub const DSBS_ROOT: u32 = 4u32;
pub const DSB_MAX_DISPLAYNAME_CHARS: u32 = 64u32;
pub const DSCCIF_HASWIZARDDIALOG: u32 = 1u32;
pub const DSCCIF_HASWIZARDPRIMARYPAGE: u32 = 2u32;
pub const DSDSOF_DONTSIGNSEAL: u32 = 4u32;
pub const DSDSOF_DSAVAILABLE: u32 = 1073741824u32;
pub const DSDSOF_HASUSERANDSERVERINFO: u32 = 1u32;
pub const DSDSOF_SIMPLEAUTHENTICATE: u32 = 2u32;
pub const DSECAF_NOTLISTED: u32 = 1u32;
pub const DSGIF_DEFAULTISCONTAINER: u32 = 32u32;
pub const DSGIF_GETDEFAULTICON: u32 = 16u32;
pub const DSGIF_ISDISABLED: u32 = 2u32;
pub const DSGIF_ISMASK: u32 = 15u32;
pub const DSGIF_ISNORMAL: u32 = 0u32;
pub const DSGIF_ISOPEN: u32 = 1u32;
pub const DSICCF_IGNORETREATASLEAF: u32 = 1u32;
pub const DSOBJECT_ISCONTAINER: u32 = 1u32;
pub const DSOBJECT_READONLYPAGES: u32 = 2147483648u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_APP_PACKAGES: u32 = 2281701376u32;
pub const DSOP_DOWNLEVEL_FILTER_ALL_WELLKNOWN_SIDS: u32 = 2147614720u32;
pub const DSOP_DOWNLEVEL_FILTER_ANONYMOUS: u32 = 2147483712u32;
pub const DSOP_DOWNLEVEL_FILTER_AUTHENTICATED_USER: u32 = 2147483680u32;
pub const DSOP_DOWNLEVEL_FILTER_BATCH: u32 = 2147483776u32;
pub const DSOP_DOWNLEVEL_FILTER_COMPUTERS: u32 = 2147483656u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_GROUP: u32 = 2147484160u32;
pub const DSOP_DOWNLEVEL_FILTER_CREATOR_OWNER: u32 = 2147483904u32;
pub const DSOP_DOWNLEVEL_FILTER_DIALUP: u32 = 2147484672u32;
pub const DSOP_DOWNLEVEL_FILTER_EXCLUDE_BUILTIN_GROUPS: u32 = 2147516416u32;
pub const DSOP_DOWNLEVEL_FILTER_GLOBAL_GROUPS: u32 = 2147483652u32;
pub const DSOP_DOWNLEVEL_FILTER_IIS_APP_POOL: u32 = 2214592512u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERACTIVE: u32 = 2147485696u32;
pub const DSOP_DOWNLEVEL_FILTER_INTERNET_USER: u32 = 2149580800u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_ACCOUNTS: u32 = 2415919104u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_GROUPS: u32 = 2147483650u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_LOGON: u32 = 2164260864u32;
pub const DSOP_DOWNLEVEL_FILTER_LOCAL_SERVICE: u32 = 2147745792u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK: u32 = 2147487744u32;
pub const DSOP_DOWNLEVEL_FILTER_NETWORK_SERVICE: u32 = 2148007936u32;
pub const DSOP_DOWNLEVEL_FILTER_OWNER_RIGHTS: u32 = 2151677952u32;
pub const DSOP_DOWNLEVEL_FILTER_REMOTE_LOGON: u32 = 2148532224u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICE: u32 = 2147491840u32;
pub const DSOP_DOWNLEVEL_FILTER_SERVICES: u32 = 2155872256u32;
pub const DSOP_DOWNLEVEL_FILTER_SYSTEM: u32 = 2147500032u32;
pub const DSOP_DOWNLEVEL_FILTER_TERMINAL_SERVER: u32 = 2147549184u32;
pub const DSOP_DOWNLEVEL_FILTER_THIS_ORG_CERT: u32 = 2181038080u32;
pub const DSOP_DOWNLEVEL_FILTER_USERS: u32 = 2147483649u32;
pub const DSOP_DOWNLEVEL_FILTER_WORLD: u32 = 2147483664u32;
pub const DSOP_FILTER_BUILTIN_GROUPS: u32 = 4u32;
pub const DSOP_FILTER_COMPUTERS: u32 = 2048u32;
pub const DSOP_FILTER_CONTACTS: u32 = 1024u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_DL: u32 = 256u32;
pub const DSOP_FILTER_DOMAIN_LOCAL_GROUPS_SE: u32 = 512u32;
pub const DSOP_FILTER_GLOBAL_GROUPS_DL: u32 = 64u32;
pub const DSOP_FILTER_GLOBAL_GROUPS_SE: u32 = 128u32;
pub const DSOP_FILTER_INCLUDE_ADVANCED_VIEW: u32 = 1u32;
pub const DSOP_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 8192u32;
pub const DSOP_FILTER_SERVICE_ACCOUNTS: u32 = 4096u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_DL: u32 = 16u32;
pub const DSOP_FILTER_UNIVERSAL_GROUPS_SE: u32 = 32u32;
pub const DSOP_FILTER_USERS: u32 = 2u32;
pub const DSOP_FILTER_WELL_KNOWN_PRINCIPALS: u32 = 8u32;
pub const DSOP_FLAG_MULTISELECT: u32 = 1u32;
pub const DSOP_FLAG_SKIP_TARGET_COMPUTER_DC_CHECK: u32 = 2u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_COMPUTERS: u32 = 256u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_CONTACTS: u32 = 512u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_GROUPS: u32 = 128u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_PASSWORDSETTINGS_OBJECTS: u32 = 2048u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_SERVICE_ACCOUNTS: u32 = 1024u32;
pub const DSOP_SCOPE_FLAG_DEFAULT_FILTER_USERS: u32 = 64u32;
pub const DSOP_SCOPE_FLAG_STARTING_SCOPE: u32 = 1u32;
pub const DSOP_SCOPE_FLAG_WANT_DOWNLEVEL_BUILTIN_PATH: u32 = 32u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_GC: u32 = 8u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_LDAP: u32 = 4u32;
pub const DSOP_SCOPE_FLAG_WANT_PROVIDER_WINNT: u32 = 2u32;
pub const DSOP_SCOPE_FLAG_WANT_SID_PATH: u32 = 16u32;
pub const DSOP_SCOPE_TYPE_DOWNLEVEL_JOINED_DOMAIN: u32 = 4u32;
pub const DSOP_SCOPE_TYPE_ENTERPRISE_DOMAIN: u32 = 8u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_DOWNLEVEL_DOMAIN: u32 = 64u32;
pub const DSOP_SCOPE_TYPE_EXTERNAL_UPLEVEL_DOMAIN: u32 = 32u32;
pub const DSOP_SCOPE_TYPE_GLOBAL_CATALOG: u32 = 16u32;
pub const DSOP_SCOPE_TYPE_TARGET_COMPUTER: u32 = 1u32;
pub const DSOP_SCOPE_TYPE_UPLEVEL_JOINED_DOMAIN: u32 = 2u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_DOWNLEVEL_SCOPE: u32 = 512u32;
pub const DSOP_SCOPE_TYPE_USER_ENTERED_UPLEVEL_SCOPE: u32 = 256u32;
pub const DSOP_SCOPE_TYPE_WORKGROUP: u32 = 128u32;
pub const DSPROP_ATTRCHANGED_MSG: windows_core::PCWSTR = windows_core::w!("DsPropAttrChanged");
pub const DSPROVIDER_ADVANCED: u32 = 16u32;
pub const DSPROVIDER_AD_LDS: u32 = 32u32;
pub const DSPROVIDER_UNUSED_0: u32 = 1u32;
pub const DSPROVIDER_UNUSED_1: u32 = 2u32;
pub const DSPROVIDER_UNUSED_2: u32 = 4u32;
pub const DSPROVIDER_UNUSED_3: u32 = 8u32;
pub const DSQPF_ENABLEADMINFEATURES: u32 = 8u32;
pub const DSQPF_ENABLEADVANCEDFEATURES: u32 = 16u32;
pub const DSQPF_HASCREDENTIALS: u32 = 32u32;
pub const DSQPF_NOCHOOSECOLUMNS: u32 = 64u32;
pub const DSQPF_NOSAVE: u32 = 1u32;
pub const DSQPF_SAVELOCATION: u32 = 2u32;
pub const DSQPF_SHOWHIDDENOBJECTS: u32 = 4u32;
pub const DSQPM_GETCLASSLIST: u32 = 268435456u32;
pub const DSQPM_HELPTOPICS: u32 = 268435457u32;
pub const DSROLE_PRIMARY_DOMAIN_GUID_PRESENT: u32 = 16777216u32;
pub const DSROLE_PRIMARY_DS_MIXED_MODE: u32 = 2u32;
pub const DSROLE_PRIMARY_DS_READONLY: u32 = 8u32;
pub const DSROLE_PRIMARY_DS_RUNNING: u32 = 1u32;
pub const DSROLE_UPGRADE_IN_PROGRESS: u32 = 4u32;
pub const DSSSF_DONTSIGNSEAL: u32 = 2u32;
pub const DSSSF_DSAVAILABLE: u32 = 2147483648u32;
pub const DSSSF_SIMPLEAUTHENTICATE: u32 = 1u32;
pub const DS_AVOID_SELF: u32 = 16384u32;
pub const DS_BACKGROUND_ONLY: u32 = 256u32;
pub const DS_BEHAVIOR_LONGHORN: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2000: u32 = 0u32;
pub const DS_BEHAVIOR_WIN2003: u32 = 2u32;
pub const DS_BEHAVIOR_WIN2003_WITH_MIXED_DOMAINS: u32 = 1u32;
pub const DS_BEHAVIOR_WIN2008: u32 = 3u32;
pub const DS_BEHAVIOR_WIN2008R2: u32 = 4u32;
pub const DS_BEHAVIOR_WIN2012: u32 = 5u32;
pub const DS_BEHAVIOR_WIN2012R2: u32 = 6u32;
pub const DS_BEHAVIOR_WIN2016: u32 = 7u32;
pub const DS_BEHAVIOR_WIN7: u32 = 4u32;
pub const DS_BEHAVIOR_WIN8: u32 = 5u32;
pub const DS_BEHAVIOR_WINBLUE: u32 = 6u32;
pub const DS_BEHAVIOR_WINTHRESHOLD: u32 = 7u32;
pub const DS_CANONICAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(7i32);
pub const DS_CANONICAL_NAME_EX: DS_NAME_FORMAT = DS_NAME_FORMAT(9i32);
pub const DS_CLOSEST_FLAG: u32 = 128u32;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608u32;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288u32;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152u32;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304u32;
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32u32;
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16u32;
pub const DS_DISPLAY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(3i32);
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912u32;
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824u32;
pub const DS_DNS_DOMAIN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(12i32);
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648u32;
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32u32;
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2u32;
pub const DS_DOMAIN_IN_FOREST: u32 = 1u32;
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16u32;
pub const DS_DOMAIN_PRIMARY: u32 = 8u32;
pub const DS_DOMAIN_TREE_ROOT: u32 = 4u32;
pub const DS_DS_10_FLAG: u32 = 65536u32;
pub const DS_DS_8_FLAG: u32 = 16384u32;
pub const DS_DS_9_FLAG: u32 = 32768u32;
pub const DS_DS_FLAG: u32 = 16u32;
pub const DS_EXIST_ADVISORY_MODE: u32 = 1u32;
pub const DS_FORCE_REDISCOVERY: u32 = 1u32;
pub const DS_FQDN_1779_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(1i32);
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096u32;
pub const DS_GC_FLAG: u32 = 4u32;
pub const DS_GC_SERVER_REQUIRED: u32 = 64u32;
pub const DS_GFTI_UPDATE_TDO: u32 = 1u32;
pub const DS_GFTI_VALID_FLAGS: u32 = 1u32;
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512u32;
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192u32;
pub const DS_INSTANCETYPE_IS_NC_HEAD: u32 = 1u32;
pub const DS_INSTANCETYPE_NC_COMING: u32 = 16u32;
pub const DS_INSTANCETYPE_NC_GOING: u32 = 32u32;
pub const DS_INSTANCETYPE_NC_IS_WRITEABLE: u32 = 4u32;
pub const DS_IP_REQUIRED: u32 = 512u32;
pub const DS_IS_DNS_NAME: u32 = 131072u32;
pub const DS_IS_FLAT_NAME: u32 = 65536u32;
pub const DS_KCC_FLAG_ASYNC_OP: u32 = 1u32;
pub const DS_KCC_FLAG_DAMPED: u32 = 2u32;
pub const DS_KCC_TASKID_UPDATE_TOPOLOGY: DS_KCC_TASKID = DS_KCC_TASKID(0i32);
pub const DS_KDC_FLAG: u32 = 32u32;
pub const DS_KDC_REQUIRED: u32 = 1024u32;
pub const DS_KEY_LIST_FLAG: u32 = 131072u32;
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216u32;
pub const DS_LDAP_FLAG: u32 = 8u32;
pub const DS_LIST_ACCOUNT_OBJECT_FOR_SERVER: u32 = 2u32;
pub const DS_LIST_DNS_HOST_NAME_FOR_SERVER: u32 = 1u32;
pub const DS_LIST_DSA_OBJECT_FOR_SERVER: u32 = 0u32;
pub const DS_MANGLE_OBJECT_RDN_FOR_DELETION: DS_MANGLE_FOR = DS_MANGLE_FOR(1i32);
pub const DS_MANGLE_OBJECT_RDN_FOR_NAME_CONFLICT: DS_MANGLE_FOR = DS_MANGLE_FOR(2i32);
pub const DS_MANGLE_UNKNOWN: DS_MANGLE_FOR = DS_MANGLE_FOR(0i32);
pub const DS_NAME_ERROR_DOMAIN_ONLY: DS_NAME_ERROR = DS_NAME_ERROR(5i32);
pub const DS_NAME_ERROR_NOT_FOUND: DS_NAME_ERROR = DS_NAME_ERROR(2i32);
pub const DS_NAME_ERROR_NOT_UNIQUE: DS_NAME_ERROR = DS_NAME_ERROR(3i32);
pub const DS_NAME_ERROR_NO_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(4i32);
pub const DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: DS_NAME_ERROR = DS_NAME_ERROR(6i32);
pub const DS_NAME_ERROR_RESOLVING: DS_NAME_ERROR = DS_NAME_ERROR(1i32);
pub const DS_NAME_ERROR_TRUST_REFERRAL: DS_NAME_ERROR = DS_NAME_ERROR(7i32);
pub const DS_NAME_FLAG_EVAL_AT_DC: DS_NAME_FLAGS = DS_NAME_FLAGS(2i32);
pub const DS_NAME_FLAG_GCVERIFY: DS_NAME_FLAGS = DS_NAME_FLAGS(4i32);
pub const DS_NAME_FLAG_SYNTACTICAL_ONLY: DS_NAME_FLAGS = DS_NAME_FLAGS(1i32);
pub const DS_NAME_FLAG_TRUST_REFERRAL: DS_NAME_FLAGS = DS_NAME_FLAGS(8i32);
pub const DS_NAME_NO_ERROR: DS_NAME_ERROR = DS_NAME_ERROR(0i32);
pub const DS_NAME_NO_FLAGS: DS_NAME_FLAGS = DS_NAME_FLAGS(0i32);
pub const DS_NDNC_FLAG: u32 = 1024u32;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2u32;
pub const DS_NT4_ACCOUNT_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(2i32);
pub const DS_ONLY_DO_SITE_NAME: u32 = 1u32;
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768u32;
pub const DS_PDC_FLAG: u32 = 1u32;
pub const DS_PDC_REQUIRED: u32 = 128u32;
pub const DS_PING_FLAGS: u32 = 1048575u32;
pub const DS_PROP_ADMIN_PREFIX: windows_core::PCWSTR = windows_core::w!("admin");
pub const DS_PROP_SHELL_PREFIX: windows_core::PCWSTR = windows_core::w!("shell");
pub const DS_REPADD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPADD_ASYNCHRONOUS_REPLICA: u32 = 32u32;
pub const DS_REPADD_CRITICAL: u32 = 2048u32;
pub const DS_REPADD_DISABLE_NOTIFICATION: u32 = 64u32;
pub const DS_REPADD_DISABLE_PERIODIC: u32 = 128u32;
pub const DS_REPADD_INITIAL: u32 = 4u32;
pub const DS_REPADD_INTERSITE_MESSAGING: u32 = 16u32;
pub const DS_REPADD_NEVER_NOTIFY: u32 = 512u32;
pub const DS_REPADD_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPADD_PERIODIC: u32 = 8u32;
pub const DS_REPADD_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPADD_TWO_WAY: u32 = 1024u32;
pub const DS_REPADD_USE_COMPRESSION: u32 = 256u32;
pub const DS_REPADD_WRITEABLE: u32 = 2u32;
pub const DS_REPDEL_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPDEL_IGNORE_ERRORS: u32 = 8u32;
pub const DS_REPDEL_INTERSITE_MESSAGING: u32 = 4u32;
pub const DS_REPDEL_LOCAL_ONLY: u32 = 16u32;
pub const DS_REPDEL_NO_SOURCE: u32 = 32u32;
pub const DS_REPDEL_REF_OK: u32 = 64u32;
pub const DS_REPDEL_WRITEABLE: u32 = 2u32;
pub const DS_REPL_INFO_CURSORS_2_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(7i32);
pub const DS_REPL_INFO_CURSORS_3_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(8i32);
pub const DS_REPL_INFO_CURSORS_FOR_NC: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(1i32);
pub const DS_REPL_INFO_FLAG_IMPROVE_LINKED_ATTRS: u32 = 1u32;
pub const DS_REPL_INFO_KCC_DSA_CONNECT_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(3i32);
pub const DS_REPL_INFO_KCC_DSA_LINK_FAILURES: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(4i32);
pub const DS_REPL_INFO_METADATA_2_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(10i32);
pub const DS_REPL_INFO_METADATA_2_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(9i32);
pub const DS_REPL_INFO_METADATA_EXT_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(11i32);
pub const DS_REPL_INFO_METADATA_FOR_ATTR_VALUE: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(6i32);
pub const DS_REPL_INFO_METADATA_FOR_OBJ: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(2i32);
pub const DS_REPL_INFO_NEIGHBORS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(0i32);
pub const DS_REPL_INFO_PENDING_OPS: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(5i32);
pub const DS_REPL_INFO_TYPE_MAX: DS_REPL_INFO_TYPE = DS_REPL_INFO_TYPE(12i32);
pub const DS_REPL_NBR_COMPRESS_CHANGES: u32 = 268435456u32;
pub const DS_REPL_NBR_DISABLE_SCHEDULED_SYNC: u32 = 134217728u32;
pub const DS_REPL_NBR_DO_SCHEDULED_SYNCS: u32 = 64u32;
pub const DS_REPL_NBR_FULL_SYNC_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPL_NBR_FULL_SYNC_NEXT_PACKET: u32 = 131072u32;
pub const DS_REPL_NBR_GCSPN: u32 = 1048576u32;
pub const DS_REPL_NBR_IGNORE_CHANGE_NOTIFICATIONS: u32 = 67108864u32;
pub const DS_REPL_NBR_NEVER_SYNCED: u32 = 2097152u32;
pub const DS_REPL_NBR_NONGC_RO_REPLICA: u32 = 1024u32;
pub const DS_REPL_NBR_NO_CHANGE_NOTIFICATIONS: u32 = 536870912u32;
pub const DS_REPL_NBR_PARTIAL_ATTRIBUTE_SET: u32 = 1073741824u32;
pub const DS_REPL_NBR_PREEMPTED: u32 = 16777216u32;
pub const DS_REPL_NBR_RETURN_OBJECT_PARENTS: u32 = 2048u32;
pub const DS_REPL_NBR_SELECT_SECRETS: u32 = 4096u32;
pub const DS_REPL_NBR_SYNC_ON_STARTUP: u32 = 32u32;
pub const DS_REPL_NBR_TWO_WAY_SYNC: u32 = 512u32;
pub const DS_REPL_NBR_USE_ASYNC_INTERSITE_TRANSPORT: u32 = 128u32;
pub const DS_REPL_NBR_WRITEABLE: u32 = 16u32;
pub const DS_REPL_OP_TYPE_ADD: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(1i32);
pub const DS_REPL_OP_TYPE_DELETE: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(2i32);
pub const DS_REPL_OP_TYPE_MODIFY: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(3i32);
pub const DS_REPL_OP_TYPE_SYNC: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(0i32);
pub const DS_REPL_OP_TYPE_UPDATE_REFS: DS_REPL_OP_TYPE = DS_REPL_OP_TYPE(4i32);
pub const DS_REPMOD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPMOD_UPDATE_ADDRESS: u32 = 2u32;
pub const DS_REPMOD_UPDATE_FLAGS: u32 = 1u32;
pub const DS_REPMOD_UPDATE_INSTANCE: u32 = 2u32;
pub const DS_REPMOD_UPDATE_RESULT: u32 = 8u32;
pub const DS_REPMOD_UPDATE_SCHEDULE: u32 = 4u32;
pub const DS_REPMOD_UPDATE_TRANSPORT: u32 = 16u32;
pub const DS_REPMOD_WRITEABLE: u32 = 2u32;
pub const DS_REPSYNCALL_ABORT_IF_SERVER_UNAVAILABLE: u32 = 1u32;
pub const DS_REPSYNCALL_CROSS_SITE_BOUNDARIES: u32 = 64u32;
pub const DS_REPSYNCALL_DO_NOT_SYNC: u32 = 8u32;
pub const DS_REPSYNCALL_EVENT_ERROR: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(0i32);
pub const DS_REPSYNCALL_EVENT_FINISHED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(3i32);
pub const DS_REPSYNCALL_EVENT_SYNC_COMPLETED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(2i32);
pub const DS_REPSYNCALL_EVENT_SYNC_STARTED: DS_REPSYNCALL_EVENT = DS_REPSYNCALL_EVENT(1i32);
pub const DS_REPSYNCALL_ID_SERVERS_BY_DN: u32 = 4u32;
pub const DS_REPSYNCALL_NO_OPTIONS: u32 = 0u32;
pub const DS_REPSYNCALL_PUSH_CHANGES_OUTWARD: u32 = 32u32;
pub const DS_REPSYNCALL_SERVER_UNREACHABLE: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(2i32);
pub const DS_REPSYNCALL_SKIP_INITIAL_CHECK: u32 = 16u32;
pub const DS_REPSYNCALL_SYNC_ADJACENT_SERVERS_ONLY: u32 = 2u32;
pub const DS_REPSYNCALL_WIN32_ERROR_CONTACTING_SERVER: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(0i32);
pub const DS_REPSYNCALL_WIN32_ERROR_REPLICATING: DS_REPSYNCALL_ERROR = DS_REPSYNCALL_ERROR(1i32);
pub const DS_REPSYNC_ABANDONED: u32 = 32768u32;
pub const DS_REPSYNC_ADD_REFERENCE: u32 = 512u32;
pub const DS_REPSYNC_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPSYNC_ASYNCHRONOUS_REPLICA: u32 = 1048576u32;
pub const DS_REPSYNC_CRITICAL: u32 = 2097152u32;
pub const DS_REPSYNC_FORCE: u32 = 256u32;
pub const DS_REPSYNC_FULL: u32 = 32u32;
pub const DS_REPSYNC_FULL_IN_PROGRESS: u32 = 4194304u32;
pub const DS_REPSYNC_INITIAL: u32 = 8192u32;
pub const DS_REPSYNC_INITIAL_IN_PROGRESS: u32 = 65536u32;
pub const DS_REPSYNC_INTERSITE_MESSAGING: u32 = 8u32;
pub const DS_REPSYNC_NEVER_COMPLETED: u32 = 1024u32;
pub const DS_REPSYNC_NEVER_NOTIFY: u32 = 4096u32;
pub const DS_REPSYNC_NONGC_RO_REPLICA: u32 = 16777216u32;
pub const DS_REPSYNC_NOTIFICATION: u32 = 524288u32;
pub const DS_REPSYNC_NO_DISCARD: u32 = 128u32;
pub const DS_REPSYNC_PARTIAL_ATTRIBUTE_SET: u32 = 131072u32;
pub const DS_REPSYNC_PERIODIC: u32 = 4u32;
pub const DS_REPSYNC_PREEMPTED: u32 = 8388608u32;
pub const DS_REPSYNC_REQUEUE: u32 = 262144u32;
pub const DS_REPSYNC_SELECT_SECRETS: u32 = 32768u32;
pub const DS_REPSYNC_TWO_WAY: u32 = 2048u32;
pub const DS_REPSYNC_URGENT: u32 = 64u32;
pub const DS_REPSYNC_USE_COMPRESSION: u32 = 16384u32;
pub const DS_REPSYNC_WRITEABLE: u32 = 2u32;
pub const DS_REPUPD_ADD_REFERENCE: u32 = 4u32;
pub const DS_REPUPD_ASYNCHRONOUS_OPERATION: u32 = 1u32;
pub const DS_REPUPD_DELETE_REFERENCE: u32 = 8u32;
pub const DS_REPUPD_REFERENCE_GCSPN: u32 = 16u32;
pub const DS_REPUPD_WRITEABLE: u32 = 2u32;
pub const DS_RETURN_DNS_NAME: u32 = 1073741824u32;
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648u32;
pub const DS_ROLE_DOMAIN_OWNER: u32 = 1u32;
pub const DS_ROLE_INFRASTRUCTURE_OWNER: u32 = 4u32;
pub const DS_ROLE_PDC_OWNER: u32 = 2u32;
pub const DS_ROLE_RID_OWNER: u32 = 3u32;
pub const DS_ROLE_SCHEMA_OWNER: u32 = 0u32;
pub const DS_SCHEMA_GUID_ATTR: u32 = 1u32;
pub const DS_SCHEMA_GUID_ATTR_SET: u32 = 2u32;
pub const DS_SCHEMA_GUID_CLASS: u32 = 3u32;
pub const DS_SCHEMA_GUID_CONTROL_RIGHT: u32 = 4u32;
pub const DS_SCHEMA_GUID_NOT_FOUND: u32 = 0u32;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048u32;
pub const DS_SERVICE_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(10i32);
pub const DS_SID_OR_SID_HISTORY_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(11i32);
pub const DS_SPN_ADD_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(0i32);
pub const DS_SPN_DELETE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(2i32);
pub const DS_SPN_DNS_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(0i32);
pub const DS_SPN_DN_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(1i32);
pub const DS_SPN_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(3i32);
pub const DS_SPN_NB_DOMAIN: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(4i32);
pub const DS_SPN_NB_HOST: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(2i32);
pub const DS_SPN_REPLACE_SPN_OP: DS_SPN_WRITE_OP = DS_SPN_WRITE_OP(1i32);
pub const DS_SPN_SERVICE: DS_SPN_NAME_TYPE = DS_SPN_NAME_TYPE(5i32);
pub const DS_SYNCED_EVENT_NAME: windows_core::PCSTR = windows_core::s!("NTDSInitialSyncsCompleted");
pub const DS_SYNCED_EVENT_NAME_W: windows_core::PCWSTR = windows_core::w!("NTDSInitialSyncsCompleted");
pub const DS_TIMESERV_FLAG: u32 = 64u32;
pub const DS_TIMESERV_REQUIRED: u32 = 2048u32;
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144u32;
pub const DS_UNIQUE_ID_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(6i32);
pub const DS_UNKNOWN_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(0i32);
pub const DS_USER_PRINCIPAL_NAME: DS_NAME_FORMAT = DS_NAME_FORMAT(8i32);
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576u32;
pub const DS_WRITABLE_FLAG: u32 = 256u32;
pub const DS_WRITABLE_REQUIRED: u32 = 4096u32;
pub const DS_WS_FLAG: u32 = 8192u32;
pub const DsRoleOperationActive: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(1i32);
pub const DsRoleOperationIdle: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(0i32);
pub const DsRoleOperationNeedReboot: DSROLE_OPERATION_STATE = DSROLE_OPERATION_STATE(2i32);
pub const DsRoleOperationState: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(3i32);
pub const DsRolePrimaryDomainInfoBasic: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(1i32);
pub const DsRoleServerBackup: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(2i32);
pub const DsRoleServerPrimary: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(1i32);
pub const DsRoleServerUnknown: DSROLE_SERVER_STATE = DSROLE_SERVER_STATE(0i32);
pub const DsRoleUpgradeStatus: DSROLE_PRIMARY_DOMAIN_INFO_LEVEL = DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(2i32);
pub const DsRole_RoleBackupDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(4i32);
pub const DsRole_RoleMemberServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(3i32);
pub const DsRole_RoleMemberWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(1i32);
pub const DsRole_RolePrimaryDomainController: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(5i32);
pub const DsRole_RoleStandaloneServer: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(2i32);
pub const DsRole_RoleStandaloneWorkstation: DSROLE_MACHINE_ROLE = DSROLE_MACHINE_ROLE(0i32);
pub const FACILITY_BACKUP: u32 = 2047u32;
pub const FACILITY_NTDSB: u32 = 2048u32;
pub const FACILITY_SYSTEM: u32 = 0u32;
pub const FLAG_DISABLABLE_OPTIONAL_FEATURE: u32 = 4u32;
pub const FLAG_DOMAIN_OPTIONAL_FEATURE: u32 = 2u32;
pub const FLAG_FOREST_OPTIONAL_FEATURE: u32 = 1u32;
pub const FLAG_SERVER_OPTIONAL_FEATURE: u32 = 8u32;
pub const FRSCONN_MAX_PRIORITY: u32 = 8u32;
pub const FRSCONN_PRIORITY_MASK: u32 = 1879048192u32;
pub const GUID_COMPUTRS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_COMPUTRS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("aa312825768811d1aded00c04fd8d5cd");
pub const GUID_DELETED_OBJECTS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DELETED_OBJECTS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("18e2ea80684f11d2b9aa00c04f79f805");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_DOMAIN_CONTROLLERS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("a361b2ffffd211d1aa4b00c04fd7d83a");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_FOREIGNSECURITYPRINCIPALS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("22b70c67d56e4efb91e9300fca3dc1aa");
pub const GUID_INFRASTRUCTURE_CONTAINER_A: windows_core::PCSTR = windows_core::s!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_INFRASTRUCTURE_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("2fbac1870ade11d297c400c04fd8d5cd");
pub const GUID_KEYS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("683A24E2E8164BD3AF86AC3C2CF3F981");
pub const GUID_LOSTANDFOUND_CONTAINER_A: windows_core::PCSTR = windows_core::s!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_LOSTANDFOUND_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("ab8153b7768811d1aded00c04fd8d5cd");
pub const GUID_MANAGED_SERVICE_ACCOUNTS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("1EB93889E40C45DF9F0C64D23BBB6237");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_A: windows_core::PCSTR = windows_core::s!("f4be92a4c777485e878e9421d53087db");
pub const GUID_MICROSOFT_PROGRAM_DATA_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("f4be92a4c777485e878e9421d53087db");
pub const GUID_NTDS_QUOTAS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_NTDS_QUOTAS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("6227f0af1fc2410d8e3bb10615bb5b0f");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_A: windows_core::PCSTR = windows_core::s!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PRIVILEGED_ACCESS_MANAGEMENT_OPTIONAL_FEATURE_W: windows_core::PCWSTR = windows_core::w!("73e843ece8cc4046b4ab07ffe4ab5bcd");
pub const GUID_PROGRAM_DATA_CONTAINER_A: windows_core::PCSTR = windows_core::s!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_PROGRAM_DATA_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("09460c08ae1e4a4ea0f64aee7daa1e5a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_A: windows_core::PCSTR = windows_core::s!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_RECYCLE_BIN_OPTIONAL_FEATURE_W: windows_core::PCWSTR = windows_core::w!("d8dc6d76d0ac5e44f3b9a7f9b6744f2a");
pub const GUID_SYSTEMS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_SYSTEMS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("ab1d30f3768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_A: windows_core::PCSTR = windows_core::s!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const GUID_USERS_CONTAINER_W: windows_core::PCWSTR = windows_core::w!("a9d1ca15768811d1aded00c04fd8d5cd");
pub const NTDSAPI_BIND_ALLOW_DELEGATION: u32 = 1u32;
pub const NTDSAPI_BIND_FIND_BINDING: u32 = 2u32;
pub const NTDSAPI_BIND_FORCE_KERBEROS: u32 = 4u32;
pub const NTDSCONN_KCC_GC_TOPOLOGY: u32 = 1u32;
pub const NTDSCONN_KCC_INTERSITE_GC_TOPOLOGY: u32 = 32u32;
pub const NTDSCONN_KCC_INTERSITE_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_KCC_MINIMIZE_HOPS_TOPOLOGY: u32 = 4u32;
pub const NTDSCONN_KCC_NO_REASON: u32 = 0u32;
pub const NTDSCONN_KCC_OSCILLATING_CONNECTION_TOPOLOGY: u32 = 16u32;
pub const NTDSCONN_KCC_REDUNDANT_SERVER_TOPOLOGY: u32 = 512u32;
pub const NTDSCONN_KCC_RING_TOPOLOGY: u32 = 2u32;
pub const NTDSCONN_KCC_SERVER_FAILOVER_TOPOLOGY: u32 = 128u32;
pub const NTDSCONN_KCC_SITE_FAILOVER_TOPOLOGY: u32 = 256u32;
pub const NTDSCONN_KCC_STALE_SERVERS_TOPOLOGY: u32 = 8u32;
pub const NTDSCONN_OPT_DISABLE_INTERSITE_COMPRESSION: u32 = 16u32;
pub const NTDSCONN_OPT_IGNORE_SCHEDULE_MASK: u32 = 2147483648u32;
pub const NTDSCONN_OPT_IS_GENERATED: u32 = 1u32;
pub const NTDSCONN_OPT_OVERRIDE_NOTIFY_DEFAULT: u32 = 4u32;
pub const NTDSCONN_OPT_RODC_TOPOLOGY: u32 = 64u32;
pub const NTDSCONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSCONN_OPT_USER_OWNED_SCHEDULE: u32 = 32u32;
pub const NTDSCONN_OPT_USE_NOTIFY: u32 = 8u32;
pub const NTDSDSA_OPT_BLOCK_RPC: u32 = 64u32;
pub const NTDSDSA_OPT_DISABLE_INBOUND_REPL: u32 = 2u32;
pub const NTDSDSA_OPT_DISABLE_NTDSCONN_XLATE: u32 = 8u32;
pub const NTDSDSA_OPT_DISABLE_OUTBOUND_REPL: u32 = 4u32;
pub const NTDSDSA_OPT_DISABLE_SPN_REGISTRATION: u32 = 16u32;
pub const NTDSDSA_OPT_GENERATE_OWN_TOPO: u32 = 32u32;
pub const NTDSDSA_OPT_IS_GC: u32 = 1u32;
pub const NTDSSETTINGS_DEFAULT_SERVER_REDUNDANCY: u32 = 2u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_W2K_ELECTION: u32 = 128u32;
pub const NTDSSETTINGS_OPT_FORCE_KCC_WHISTLER_BEHAVIOR: u32 = 64u32;
pub const NTDSSETTINGS_OPT_IS_AUTO_TOPOLOGY_DISABLED: u32 = 1u32;
pub const NTDSSETTINGS_OPT_IS_GROUP_CACHING_ENABLED: u32 = 32u32;
pub const NTDSSETTINGS_OPT_IS_INTER_SITE_AUTO_TOPOLOGY_DISABLED: u32 = 16u32;
pub const NTDSSETTINGS_OPT_IS_RAND_BH_SELECTION_DISABLED: u32 = 256u32;
pub const NTDSSETTINGS_OPT_IS_REDUNDANT_SERVER_TOPOLOGY_ENABLED: u32 = 1024u32;
pub const NTDSSETTINGS_OPT_IS_SCHEDULE_HASHING_ENABLED: u32 = 512u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_CLEANUP_DISABLED: u32 = 2u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_DETECT_STALE_DISABLED: u32 = 8u32;
pub const NTDSSETTINGS_OPT_IS_TOPL_MIN_HOPS_DISABLED: u32 = 4u32;
pub const NTDSSETTINGS_OPT_W2K3_BRIDGES_REQUIRED: u32 = 4096u32;
pub const NTDSSETTINGS_OPT_W2K3_IGNORE_SCHEDULES: u32 = 2048u32;
pub const NTDSSITECONN_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITECONN_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITECONN_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSSITELINK_OPT_DISABLE_COMPRESSION: u32 = 4u32;
pub const NTDSSITELINK_OPT_TWOWAY_SYNC: u32 = 2u32;
pub const NTDSSITELINK_OPT_USE_NOTIFY: u32 = 1u32;
pub const NTDSTRANSPORT_OPT_BRIDGES_REQUIRED: u32 = 2u32;
pub const NTDSTRANSPORT_OPT_IGNORE_SCHEDULES: u32 = 1u32;
pub const OQWF_DEFAULTFORM: u32 = 2u32;
pub const OQWF_HIDEMENUS: u32 = 1024u32;
pub const OQWF_HIDESEARCHUI: u32 = 2048u32;
pub const OQWF_ISSUEONOPEN: u32 = 64u32;
pub const OQWF_LOADQUERY: u32 = 8u32;
pub const OQWF_OKCANCEL: u32 = 1u32;
pub const OQWF_PARAMISPROPERTYBAG: u32 = 2147483648u32;
pub const OQWF_REMOVEFORMS: u32 = 32u32;
pub const OQWF_REMOVESCOPES: u32 = 16u32;
pub const OQWF_SAVEQUERYONOK: u32 = 512u32;
pub const OQWF_SHOWOPTIONAL: u32 = 128u32;
pub const OQWF_SINGLESELECT: u32 = 4u32;
pub const QUERYFORM_CHANGESFORMLIST: u64 = 1u64;
pub const QUERYFORM_CHANGESOPTFORMLIST: u64 = 2u64;
pub const SCHEDULE_BANDWIDTH: u32 = 1u32;
pub const SCHEDULE_INTERVAL: u32 = 0u32;
pub const SCHEDULE_PRIORITY: u32 = 2u32;
pub const WM_ADSPROP_NOTIFY_APPLY: u32 = 2128u32;
pub const WM_ADSPROP_NOTIFY_CHANGE: u32 = 2127u32;
pub const WM_ADSPROP_NOTIFY_ERROR: u32 = 2134u32;
pub const WM_ADSPROP_NOTIFY_EXIT: u32 = 2131u32;
pub const WM_ADSPROP_NOTIFY_FOREGROUND: u32 = 2130u32;
pub const WM_ADSPROP_NOTIFY_PAGEHWND: u32 = 2126u32;
pub const WM_ADSPROP_NOTIFY_PAGEINIT: u32 = 2125u32;
pub const WM_ADSPROP_NOTIFY_SETFOCUS: u32 = 2129u32;
pub const hrAccessDenied: windows_core::HRESULT = windows_core::HRESULT(0xC8000773_u32 as _);
pub const hrAfterInitialization: windows_core::HRESULT = windows_core::HRESULT(0xC800073A_u32 as _);
pub const hrAlreadyInitialized: windows_core::HRESULT = windows_core::HRESULT(0xC8000406_u32 as _);
pub const hrAlreadyOpen: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0005_u32 as _);
pub const hrAlreadyPrepared: windows_core::HRESULT = windows_core::HRESULT(0xC8000647_u32 as _);
pub const hrBFInUse: windows_core::HRESULT = windows_core::HRESULT(0xC80000CA_u32 as _);
pub const hrBFNotSynchronous: windows_core::HRESULT = windows_core::HRESULT(0x880000C8_u32 as _);
pub const hrBFPageNotFound: windows_core::HRESULT = windows_core::HRESULT(0x880000C9_u32 as _);
pub const hrBackupDirectoryNotEmpty: windows_core::HRESULT = windows_core::HRESULT(0xC80001F8_u32 as _);
pub const hrBackupInProgress: windows_core::HRESULT = windows_core::HRESULT(0xC80001F9_u32 as _);
pub const hrBackupNotAllowedYet: windows_core::HRESULT = windows_core::HRESULT(0xC800020B_u32 as _);
pub const hrBadBackupDatabaseSize: windows_core::HRESULT = windows_core::HRESULT(0xC8000231_u32 as _);
pub const hrBadCheckpointSignature: windows_core::HRESULT = windows_core::HRESULT(0xC8000214_u32 as _);
pub const hrBadColumnId: windows_core::HRESULT = windows_core::HRESULT(0xC80005ED_u32 as _);
pub const hrBadDbSignature: windows_core::HRESULT = windows_core::HRESULT(0xC8000213_u32 as _);
pub const hrBadItagSequence: windows_core::HRESULT = windows_core::HRESULT(0xC80005EE_u32 as _);
pub const hrBadLogSignature: windows_core::HRESULT = windows_core::HRESULT(0xC8000212_u32 as _);
pub const hrBadLogVersion: windows_core::HRESULT = windows_core::HRESULT(0xC8000202_u32 as _);
pub const hrBufferTooSmall: windows_core::HRESULT = windows_core::HRESULT(0xC800040E_u32 as _);
pub const hrBufferTruncated: windows_core::HRESULT = windows_core::HRESULT(0x880003EE_u32 as _);
pub const hrCannotBeTagged: windows_core::HRESULT = windows_core::HRESULT(0xC80005F1_u32 as _);
pub const hrCannotRename: windows_core::HRESULT = windows_core::HRESULT(0xC800051A_u32 as _);
pub const hrCheckpointCorrupt: windows_core::HRESULT = windows_core::HRESULT(0xC8000215_u32 as _);
pub const hrCircularLogging: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000B_u32 as _);
pub const hrColumn2ndSysMaint: windows_core::HRESULT = windows_core::HRESULT(0xC80005E6_u32 as _);
pub const hrColumnCannotIndex: windows_core::HRESULT = windows_core::HRESULT(0xC80005E9_u32 as _);
pub const hrColumnDoesNotFit: windows_core::HRESULT = windows_core::HRESULT(0xC80005DF_u32 as _);
pub const hrColumnDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC80005E4_u32 as _);
pub const hrColumnInUse: windows_core::HRESULT = windows_core::HRESULT(0xC8000416_u32 as _);
pub const hrColumnIndexed: windows_core::HRESULT = windows_core::HRESULT(0xC80005E1_u32 as _);
pub const hrColumnLong: windows_core::HRESULT = windows_core::HRESULT(0xC80005DD_u32 as _);
pub const hrColumnMaxTruncated: windows_core::HRESULT = windows_core::HRESULT(0x880005E8_u32 as _);
pub const hrColumnNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC80005E3_u32 as _);
pub const hrColumnNotUpdatable: windows_core::HRESULT = windows_core::HRESULT(0xC8000418_u32 as _);
pub const hrColumnNull: windows_core::HRESULT = windows_core::HRESULT(0x880003EC_u32 as _);
pub const hrColumnSetNull: windows_core::HRESULT = windows_core::HRESULT(0x8800042C_u32 as _);
pub const hrColumnTooBig: windows_core::HRESULT = windows_core::HRESULT(0xC80005E2_u32 as _);
pub const hrCommunicationError: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000D_u32 as _);
pub const hrConsistentTimeMismatch: windows_core::HRESULT = windows_core::HRESULT(0xC8000227_u32 as _);
pub const hrContainerNotEmpty: windows_core::HRESULT = windows_core::HRESULT(0xC8000413_u32 as _);
pub const hrContentsExpired: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0011_u32 as _);
pub const hrCouldNotConnect: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0007_u32 as _);
pub const hrCreateIndexFailed: windows_core::HRESULT = windows_core::HRESULT(0x88000581_u32 as _);
pub const hrCurrencyStackOutOfMemory: windows_core::HRESULT = windows_core::HRESULT(0xC800042E_u32 as _);
pub const hrDatabaseAttached: windows_core::HRESULT = windows_core::HRESULT(0x880003EF_u32 as _);
pub const hrDatabaseCorrupted: windows_core::HRESULT = windows_core::HRESULT(0xC80004B6_u32 as _);
pub const hrDatabaseDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC80004B1_u32 as _);
pub const hrDatabaseInUse: windows_core::HRESULT = windows_core::HRESULT(0xC80004B2_u32 as _);
pub const hrDatabaseInconsistent: windows_core::HRESULT = windows_core::HRESULT(0xC8000226_u32 as _);
pub const hrDatabaseInvalidName: windows_core::HRESULT = windows_core::HRESULT(0xC80004B4_u32 as _);
pub const hrDatabaseInvalidPages: windows_core::HRESULT = windows_core::HRESULT(0xC80004B5_u32 as _);
pub const hrDatabaseLocked: windows_core::HRESULT = windows_core::HRESULT(0xC80004B7_u32 as _);
pub const hrDatabaseNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC80004B3_u32 as _);
pub const hrDeleteBackupFileFail: windows_core::HRESULT = windows_core::HRESULT(0xC800020C_u32 as _);
pub const hrDensityInvalid: windows_core::HRESULT = windows_core::HRESULT(0xC800051B_u32 as _);
pub const hrDiskFull: windows_core::HRESULT = windows_core::HRESULT(0xC8000710_u32 as _);
pub const hrDiskIO: windows_core::HRESULT = windows_core::HRESULT(0xC80003FE_u32 as _);
pub const hrError: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0002_u32 as _);
pub const hrExistingLogFileHasBadSignature: windows_core::HRESULT = windows_core::HRESULT(0x8800022E_u32 as _);
pub const hrExistingLogFileIsNotContiguous: windows_core::HRESULT = windows_core::HRESULT(0x8800022F_u32 as _);
pub const hrFLDKeyTooBig: windows_core::HRESULT = windows_core::HRESULT(0x88000190_u32 as _);
pub const hrFLDNullKey: windows_core::HRESULT = windows_core::HRESULT(0x88000192_u32 as _);
pub const hrFLDTooManySegments: windows_core::HRESULT = windows_core::HRESULT(0xC8000191_u32 as _);
pub const hrFeatureNotAvailable: windows_core::HRESULT = windows_core::HRESULT(0xC80003E9_u32 as _);
pub const hrFileAccessDenied: windows_core::HRESULT = windows_core::HRESULT(0xC8000408_u32 as _);
pub const hrFileClose: windows_core::HRESULT = windows_core::HRESULT(0xC8000066_u32 as _);
pub const hrFileNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC8000713_u32 as _);
pub const hrFileOpenReadOnly: windows_core::HRESULT = windows_core::HRESULT(0x88000715_u32 as _);
pub const hrFullBackupNotTaken: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000E_u32 as _);
pub const hrGivenLogFileHasBadSignature: windows_core::HRESULT = windows_core::HRESULT(0xC800022B_u32 as _);
pub const hrGivenLogFileIsNotContiguous: windows_core::HRESULT = windows_core::HRESULT(0xC800022C_u32 as _);
pub const hrIllegalOperation: windows_core::HRESULT = windows_core::HRESULT(0xC8000520_u32 as _);
pub const hrInTransaction: windows_core::HRESULT = windows_core::HRESULT(0xC8000454_u32 as _);
pub const hrIncrementalBackupDisabled: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0009_u32 as _);
pub const hrIndexCantBuild: windows_core::HRESULT = windows_core::HRESULT(0xC8000579_u32 as _);
pub const hrIndexDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC800057B_u32 as _);
pub const hrIndexHasClustered: windows_core::HRESULT = windows_core::HRESULT(0xC8000580_u32 as _);
pub const hrIndexHasPrimary: windows_core::HRESULT = windows_core::HRESULT(0xC800057A_u32 as _);
pub const hrIndexInUse: windows_core::HRESULT = windows_core::HRESULT(0xC800041B_u32 as _);
pub const hrIndexInvalidDef: windows_core::HRESULT = windows_core::HRESULT(0xC800057E_u32 as _);
pub const hrIndexMustStay: windows_core::HRESULT = windows_core::HRESULT(0xC800057D_u32 as _);
pub const hrIndexNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC800057C_u32 as _);
pub const hrInvalidBackup: windows_core::HRESULT = windows_core::HRESULT(0xC800020E_u32 as _);
pub const hrInvalidBackupSequence: windows_core::HRESULT = windows_core::HRESULT(0xC8000209_u32 as _);
pub const hrInvalidBookmark: windows_core::HRESULT = windows_core::HRESULT(0xC8000415_u32 as _);
pub const hrInvalidBufferSize: windows_core::HRESULT = windows_core::HRESULT(0xC8000417_u32 as _);
pub const hrInvalidCodePage: windows_core::HRESULT = windows_core::HRESULT(0xC8000427_u32 as _);
pub const hrInvalidColumnType: windows_core::HRESULT = windows_core::HRESULT(0xC80005E7_u32 as _);
pub const hrInvalidCountry: windows_core::HRESULT = windows_core::HRESULT(0xC8000425_u32 as _);
pub const hrInvalidDatabase: windows_core::HRESULT = windows_core::HRESULT(0xC8000404_u32 as _);
pub const hrInvalidDatabaseId: windows_core::HRESULT = windows_core::HRESULT(0xC80003F2_u32 as _);
pub const hrInvalidFilename: windows_core::HRESULT = windows_core::HRESULT(0xC8000414_u32 as _);
pub const hrInvalidHandle: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0003_u32 as _);
pub const hrInvalidLanguageId: windows_core::HRESULT = windows_core::HRESULT(0xC8000426_u32 as _);
pub const hrInvalidLogSequence: windows_core::HRESULT = windows_core::HRESULT(0xC8000203_u32 as _);
pub const hrInvalidName: windows_core::HRESULT = windows_core::HRESULT(0xC80003EA_u32 as _);
pub const hrInvalidObject: windows_core::HRESULT = windows_core::HRESULT(0xC8000524_u32 as _);
pub const hrInvalidOnSort: windows_core::HRESULT = windows_core::HRESULT(0xC80006A6_u32 as _);
pub const hrInvalidOperation: windows_core::HRESULT = windows_core::HRESULT(0xC8000772_u32 as _);
pub const hrInvalidParam: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0001_u32 as _);
pub const hrInvalidParameter: windows_core::HRESULT = windows_core::HRESULT(0xC80003EB_u32 as _);
pub const hrInvalidPath: windows_core::HRESULT = windows_core::HRESULT(0xC80003FF_u32 as _);
pub const hrInvalidRecips: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0006_u32 as _);
pub const hrInvalidSesid: windows_core::HRESULT = windows_core::HRESULT(0xC8000450_u32 as _);
pub const hrInvalidTableId: windows_core::HRESULT = windows_core::HRESULT(0xC800051E_u32 as _);
pub const hrKeyChanged: windows_core::HRESULT = windows_core::HRESULT(0x88000652_u32 as _);
pub const hrKeyDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC8000645_u32 as _);
pub const hrKeyIsMade: windows_core::HRESULT = windows_core::HRESULT(0xC80005EC_u32 as _);
pub const hrKeyNotMade: windows_core::HRESULT = windows_core::HRESULT(0xC8000648_u32 as _);
pub const hrLogBufferTooSmall: windows_core::HRESULT = windows_core::HRESULT(0xC8000205_u32 as _);
pub const hrLogCorrupted: windows_core::HRESULT = windows_core::HRESULT(0xC800073C_u32 as _);
pub const hrLogDiskFull: windows_core::HRESULT = windows_core::HRESULT(0xC8000211_u32 as _);
pub const hrLogFileCorrupt: windows_core::HRESULT = windows_core::HRESULT(0xC80001F5_u32 as _);
pub const hrLogFileNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000A_u32 as _);
pub const hrLogSequenceEnd: windows_core::HRESULT = windows_core::HRESULT(0xC8000207_u32 as _);
pub const hrLogWriteFail: windows_core::HRESULT = windows_core::HRESULT(0xC80001FE_u32 as _);
pub const hrLoggingDisabled: windows_core::HRESULT = windows_core::HRESULT(0xC8000204_u32 as _);
pub const hrMakeBackupDirectoryFail: windows_core::HRESULT = windows_core::HRESULT(0xC800020D_u32 as _);
pub const hrMissingExpiryToken: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000F_u32 as _);
pub const hrMissingFullBackup: windows_core::HRESULT = windows_core::HRESULT(0xC8000230_u32 as _);
pub const hrMissingLogFile: windows_core::HRESULT = windows_core::HRESULT(0xC8000210_u32 as _);
pub const hrMissingPreviousLogFile: windows_core::HRESULT = windows_core::HRESULT(0xC80001FD_u32 as _);
pub const hrMissingRestoreLogFiles: windows_core::HRESULT = windows_core::HRESULT(0xC800022D_u32 as _);
pub const hrNoBackup: windows_core::HRESULT = windows_core::HRESULT(0xC8000208_u32 as _);
pub const hrNoBackupDirectory: windows_core::HRESULT = windows_core::HRESULT(0xC80001F7_u32 as _);
pub const hrNoCurrentIndex: windows_core::HRESULT = windows_core::HRESULT(0xC80005EB_u32 as _);
pub const hrNoCurrentRecord: windows_core::HRESULT = windows_core::HRESULT(0xC8000643_u32 as _);
pub const hrNoFullRestore: windows_core::HRESULT = windows_core::HRESULT(0xC7FF000C_u32 as _);
pub const hrNoIdleActivity: windows_core::HRESULT = windows_core::HRESULT(0x88000422_u32 as _);
pub const hrNoWriteLock: windows_core::HRESULT = windows_core::HRESULT(0x8800042B_u32 as _);
pub const hrNone: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const hrNotInTransaction: windows_core::HRESULT = windows_core::HRESULT(0xC800041E_u32 as _);
pub const hrNotInitialized: windows_core::HRESULT = windows_core::HRESULT(0xC8000405_u32 as _);
pub const hrNullInvalid: windows_core::HRESULT = windows_core::HRESULT(0xC80005E0_u32 as _);
pub const hrNullKeyDisallowed: windows_core::HRESULT = windows_core::HRESULT(0xC800041D_u32 as _);
pub const hrNyi: windows_core::HRESULT = windows_core::HRESULT(0xC0000001_u32 as _);
pub const hrObjectDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC8000522_u32 as _);
pub const hrObjectNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC8000519_u32 as _);
pub const hrOutOfBuffers: windows_core::HRESULT = windows_core::HRESULT(0xC80003F6_u32 as _);
pub const hrOutOfCursors: windows_core::HRESULT = windows_core::HRESULT(0xC80003F5_u32 as _);
pub const hrOutOfDatabaseSpace: windows_core::HRESULT = windows_core::HRESULT(0xC80003F4_u32 as _);
pub const hrOutOfFileHandles: windows_core::HRESULT = windows_core::HRESULT(0xC80003FC_u32 as _);
pub const hrOutOfMemory: windows_core::HRESULT = windows_core::HRESULT(0xC80003F3_u32 as _);
pub const hrOutOfSessions: windows_core::HRESULT = windows_core::HRESULT(0xC800044D_u32 as _);
pub const hrOutOfThreads: windows_core::HRESULT = windows_core::HRESULT(0xC8000067_u32 as _);
pub const hrPMRecDeleted: windows_core::HRESULT = windows_core::HRESULT(0xC800012E_u32 as _);
pub const hrPatchFileMismatch: windows_core::HRESULT = windows_core::HRESULT(0xC8000228_u32 as _);
pub const hrPermissionDenied: windows_core::HRESULT = windows_core::HRESULT(0xC8000711_u32 as _);
pub const hrReadVerifyFailure: windows_core::HRESULT = windows_core::HRESULT(0xC80003FA_u32 as _);
pub const hrRecordClusteredChanged: windows_core::HRESULT = windows_core::HRESULT(0xC8000644_u32 as _);
pub const hrRecordDeleted: windows_core::HRESULT = windows_core::HRESULT(0xC80003F9_u32 as _);
pub const hrRecordNotFound: windows_core::HRESULT = windows_core::HRESULT(0xC8000641_u32 as _);
pub const hrRecordTooBig: windows_core::HRESULT = windows_core::HRESULT(0xC8000402_u32 as _);
pub const hrRecoveredWithErrors: windows_core::HRESULT = windows_core::HRESULT(0xC800020F_u32 as _);
pub const hrRemainingVersions: windows_core::HRESULT = windows_core::HRESULT(0x88000141_u32 as _);
pub const hrRestoreInProgress: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0004_u32 as _);
pub const hrRestoreLogTooHigh: windows_core::HRESULT = windows_core::HRESULT(0xC800022A_u32 as _);
pub const hrRestoreLogTooLow: windows_core::HRESULT = windows_core::HRESULT(0xC8000229_u32 as _);
pub const hrRestoreMapExists: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0008_u32 as _);
pub const hrSeekNotEqual: windows_core::HRESULT = windows_core::HRESULT(0x8800040F_u32 as _);
pub const hrSessionWriteConflict: windows_core::HRESULT = windows_core::HRESULT(0xC8000453_u32 as _);
pub const hrTableDuplicate: windows_core::HRESULT = windows_core::HRESULT(0xC8000517_u32 as _);
pub const hrTableEmpty: windows_core::HRESULT = windows_core::HRESULT(0x88000515_u32 as _);
pub const hrTableInUse: windows_core::HRESULT = windows_core::HRESULT(0xC8000518_u32 as _);
pub const hrTableLocked: windows_core::HRESULT = windows_core::HRESULT(0xC8000516_u32 as _);
pub const hrTableNotEmpty: windows_core::HRESULT = windows_core::HRESULT(0xC800051C_u32 as _);
pub const hrTaggedNotNULL: windows_core::HRESULT = windows_core::HRESULT(0xC80005EA_u32 as _);
pub const hrTempFileOpenError: windows_core::HRESULT = windows_core::HRESULT(0xC800070B_u32 as _);
pub const hrTermInProgress: windows_core::HRESULT = windows_core::HRESULT(0xC80003E8_u32 as _);
pub const hrTooManyActiveUsers: windows_core::HRESULT = windows_core::HRESULT(0xC8000423_u32 as _);
pub const hrTooManyAttachedDatabases: windows_core::HRESULT = windows_core::HRESULT(0xC800070D_u32 as _);
pub const hrTooManyColumns: windows_core::HRESULT = windows_core::HRESULT(0xC8000410_u32 as _);
pub const hrTooManyIO: windows_core::HRESULT = windows_core::HRESULT(0xC8000069_u32 as _);
pub const hrTooManyIndexes: windows_core::HRESULT = windows_core::HRESULT(0xC80003F7_u32 as _);
pub const hrTooManyKeys: windows_core::HRESULT = windows_core::HRESULT(0xC80003F8_u32 as _);
pub const hrTooManyOpenDatabases: windows_core::HRESULT = windows_core::HRESULT(0xC8000403_u32 as _);
pub const hrTooManyOpenIndexes: windows_core::HRESULT = windows_core::HRESULT(0xC8000582_u32 as _);
pub const hrTooManyOpenTables: windows_core::HRESULT = windows_core::HRESULT(0xC800051F_u32 as _);
pub const hrTooManySorts: windows_core::HRESULT = windows_core::HRESULT(0xC80006A5_u32 as _);
pub const hrTransTooDeep: windows_core::HRESULT = windows_core::HRESULT(0xC800044F_u32 as _);
pub const hrUnknownExpiryTokenFormat: windows_core::HRESULT = windows_core::HRESULT(0xC7FF0010_u32 as _);
pub const hrUpdateNotPrepared: windows_core::HRESULT = windows_core::HRESULT(0xC8000649_u32 as _);
pub const hrVersionStoreOutOfMemory: windows_core::HRESULT = windows_core::HRESULT(0xC800042D_u32 as _);
pub const hrWriteConflict: windows_core::HRESULT = windows_core::HRESULT(0xC800044E_u32 as _);
pub const hrerrDataHasChanged: windows_core::HRESULT = windows_core::HRESULT(0xC800064B_u32 as _);
pub const hrwrnDataHasChanged: windows_core::HRESULT = windows_core::HRESULT(0x8800064A_u32 as _);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADSI_DIALECT_ENUM(pub i32);
impl windows_core::TypeKind for ADSI_DIALECT_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADSI_DIALECT_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADSI_DIALECT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADSTYPE(pub i32);
impl windows_core::TypeKind for ADSTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADSTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADSTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_ACEFLAG_ENUM(pub i32);
impl windows_core::TypeKind for ADS_ACEFLAG_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_ACEFLAG_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_ACEFLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_ACETYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_ACETYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_ACETYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_ACETYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_AUTHENTICATION_ENUM(pub u32);
impl windows_core::TypeKind for ADS_AUTHENTICATION_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_AUTHENTICATION_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_AUTHENTICATION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_CHASE_REFERRALS_ENUM(pub i32);
impl windows_core::TypeKind for ADS_CHASE_REFERRALS_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_CHASE_REFERRALS_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_CHASE_REFERRALS_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_DEREFENUM(pub i32);
impl windows_core::TypeKind for ADS_DEREFENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_DEREFENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_DEREFENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_DISPLAY_ENUM(pub i32);
impl windows_core::TypeKind for ADS_DISPLAY_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_DISPLAY_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_DISPLAY_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_ESCAPE_MODE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_ESCAPE_MODE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_ESCAPE_MODE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_ESCAPE_MODE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_FLAGTYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_FLAGTYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_FLAGTYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_FLAGTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_FORMAT_ENUM(pub i32);
impl windows_core::TypeKind for ADS_FORMAT_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_FORMAT_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_FORMAT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_GROUP_TYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_GROUP_TYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_GROUP_TYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_GROUP_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_NAME_INITTYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_NAME_INITTYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_NAME_INITTYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_NAME_INITTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_NAME_TYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_NAME_TYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_NAME_TYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_NAME_TYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_OPTION_ENUM(pub i32);
impl windows_core::TypeKind for ADS_OPTION_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_OPTION_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_OPTION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_PASSWORD_ENCODING_ENUM(pub i32);
impl windows_core::TypeKind for ADS_PASSWORD_ENCODING_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_PASSWORD_ENCODING_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_PASSWORD_ENCODING_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_PATHTYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_PATHTYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_PATHTYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_PATHTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_PREFERENCES_ENUM(pub i32);
impl windows_core::TypeKind for ADS_PREFERENCES_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_PREFERENCES_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_PREFERENCES_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_PROPERTY_OPERATION_ENUM(pub i32);
impl windows_core::TypeKind for ADS_PROPERTY_OPERATION_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_PROPERTY_OPERATION_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_PROPERTY_OPERATION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_RIGHTS_ENUM(pub i32);
impl windows_core::TypeKind for ADS_RIGHTS_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_RIGHTS_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_RIGHTS_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SCOPEENUM(pub i32);
impl windows_core::TypeKind for ADS_SCOPEENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SCOPEENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SCOPEENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SD_CONTROL_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SD_CONTROL_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SD_CONTROL_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SD_CONTROL_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SD_FORMAT_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SD_FORMAT_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SD_FORMAT_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SD_FORMAT_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SD_REVISION_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SD_REVISION_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SD_REVISION_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SD_REVISION_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SEARCHPREF_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SEARCHPREF_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SEARCHPREF_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SEARCHPREF_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SECURITY_INFO_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SECURITY_INFO_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SECURITY_INFO_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SECURITY_INFO_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SETTYPE_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SETTYPE_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SETTYPE_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SETTYPE_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_STATUSENUM(pub i32);
impl windows_core::TypeKind for ADS_STATUSENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_STATUSENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_STATUSENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_SYSTEMFLAG_ENUM(pub i32);
impl windows_core::TypeKind for ADS_SYSTEMFLAG_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_SYSTEMFLAG_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_SYSTEMFLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADS_USER_FLAG_ENUM(pub i32);
impl windows_core::TypeKind for ADS_USER_FLAG_ENUM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADS_USER_FLAG_ENUM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADS_USER_FLAG_ENUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSROLE_MACHINE_ROLE(pub i32);
impl windows_core::TypeKind for DSROLE_MACHINE_ROLE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSROLE_MACHINE_ROLE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSROLE_MACHINE_ROLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSROLE_OPERATION_STATE(pub i32);
impl windows_core::TypeKind for DSROLE_OPERATION_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSROLE_OPERATION_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSROLE_OPERATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_LEVEL(pub i32);
impl windows_core::TypeKind for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSROLE_PRIMARY_DOMAIN_INFO_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSROLE_PRIMARY_DOMAIN_INFO_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DSROLE_SERVER_STATE(pub i32);
impl windows_core::TypeKind for DSROLE_SERVER_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DSROLE_SERVER_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DSROLE_SERVER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_KCC_TASKID(pub i32);
impl windows_core::TypeKind for DS_KCC_TASKID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_KCC_TASKID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_KCC_TASKID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_MANGLE_FOR(pub i32);
impl windows_core::TypeKind for DS_MANGLE_FOR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_MANGLE_FOR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_MANGLE_FOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_NAME_ERROR(pub i32);
impl windows_core::TypeKind for DS_NAME_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_NAME_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_NAME_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_NAME_FLAGS(pub i32);
impl windows_core::TypeKind for DS_NAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_NAME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_NAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_NAME_FORMAT(pub i32);
impl windows_core::TypeKind for DS_NAME_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_NAME_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_NAME_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_REPL_INFO_TYPE(pub i32);
impl windows_core::TypeKind for DS_REPL_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_REPL_INFO_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_REPL_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_REPL_OP_TYPE(pub i32);
impl windows_core::TypeKind for DS_REPL_OP_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_REPL_OP_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_REPL_OP_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_REPSYNCALL_ERROR(pub i32);
impl windows_core::TypeKind for DS_REPSYNCALL_ERROR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_REPSYNCALL_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_ERROR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_REPSYNCALL_EVENT(pub i32);
impl windows_core::TypeKind for DS_REPSYNCALL_EVENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_REPSYNCALL_EVENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_REPSYNCALL_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_SPN_NAME_TYPE(pub i32);
impl windows_core::TypeKind for DS_SPN_NAME_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_SPN_NAME_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_SPN_NAME_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DS_SPN_WRITE_OP(pub i32);
impl windows_core::TypeKind for DS_SPN_WRITE_OP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DS_SPN_WRITE_OP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DS_SPN_WRITE_OP").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADSPROPERROR {
    pub hwndPage: super::super::Foundation::HWND,
    pub pszPageTitle: windows_core::PWSTR,
    pub pszObjPath: windows_core::PWSTR,
    pub pszObjClass: windows_core::PWSTR,
    pub hr: windows_core::HRESULT,
    pub pszError: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADSPROPERROR {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADSPROPERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct ADSPROPINITPARAMS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hr: windows_core::HRESULT,
    pub pDsObj: core::mem::ManuallyDrop<Option<IDirectoryObject>>,
    pub pwzCN: windows_core::PWSTR,
    pub pWritableAttrs: *mut ADS_ATTR_INFO,
}
impl Clone for ADSPROPINITPARAMS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for ADSPROPINITPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADSPROPINITPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADSVALUE {
    pub dwType: ADSTYPE,
    pub Anonymous: ADSVALUE_0,
}
impl windows_core::TypeKind for ADSVALUE {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADSVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ADSVALUE_0 {
    pub DNString: *mut u16,
    pub CaseExactString: *mut u16,
    pub CaseIgnoreString: *mut u16,
    pub PrintableString: *mut u16,
    pub NumericString: *mut u16,
    pub Boolean: u32,
    pub Integer: u32,
    pub OctetString: ADS_OCTET_STRING,
    pub UTCTime: super::super::Foundation::SYSTEMTIME,
    pub LargeInteger: i64,
    pub ClassName: *mut u16,
    pub ProviderSpecific: ADS_PROV_SPECIFIC,
    pub pCaseIgnoreList: *mut ADS_CASEIGNORE_LIST,
    pub pOctetList: *mut ADS_OCTET_LIST,
    pub pPath: *mut ADS_PATH,
    pub pPostalAddress: *mut ADS_POSTALADDRESS,
    pub Timestamp: ADS_TIMESTAMP,
    pub BackLink: ADS_BACKLINK,
    pub pTypedName: *mut ADS_TYPEDNAME,
    pub Hold: ADS_HOLD,
    pub pNetAddress: *mut ADS_NETADDRESS,
    pub pReplicaPointer: *mut ADS_REPLICAPOINTER,
    pub pFaxNumber: *mut ADS_FAXNUMBER,
    pub Email: ADS_EMAIL,
    pub SecurityDescriptor: ADS_NT_SECURITY_DESCRIPTOR,
    pub pDNWithBinary: *mut ADS_DN_WITH_BINARY,
    pub pDNWithString: *mut ADS_DN_WITH_STRING,
}
impl windows_core::TypeKind for ADSVALUE_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADSVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_ATTR_DEF {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub dwMinRange: u32,
    pub dwMaxRange: u32,
    pub fMultiValued: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for ADS_ATTR_DEF {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_ATTR_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_ATTR_INFO {
    pub pszAttrName: windows_core::PWSTR,
    pub dwControlCode: u32,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
}
impl windows_core::TypeKind for ADS_ATTR_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_ATTR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_BACKLINK {
    pub RemoteID: u32,
    pub ObjectName: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_BACKLINK {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_BACKLINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_CASEIGNORE_LIST {
    pub Next: *mut ADS_CASEIGNORE_LIST,
    pub String: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_CASEIGNORE_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_CASEIGNORE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_CLASS_DEF {
    pub pszClassName: windows_core::PWSTR,
    pub dwMandatoryAttrs: u32,
    pub ppszMandatoryAttrs: *mut windows_core::PWSTR,
    pub optionalAttrs: u32,
    pub ppszOptionalAttrs: *mut *mut windows_core::PWSTR,
    pub dwNamingAttrs: u32,
    pub ppszNamingAttrs: *mut *mut windows_core::PWSTR,
    pub dwSuperClasses: u32,
    pub ppszSuperClasses: *mut *mut windows_core::PWSTR,
    pub fIsContainer: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for ADS_CLASS_DEF {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_CLASS_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_DN_WITH_BINARY {
    pub dwLength: u32,
    pub lpBinaryValue: *mut u8,
    pub pszDNString: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_DN_WITH_BINARY {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_DN_WITH_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_DN_WITH_STRING {
    pub pszStringValue: windows_core::PWSTR,
    pub pszDNString: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_DN_WITH_STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_DN_WITH_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_EMAIL {
    pub Address: windows_core::PWSTR,
    pub Type: u32,
}
impl windows_core::TypeKind for ADS_EMAIL {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_EMAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_FAXNUMBER {
    pub TelephoneNumber: windows_core::PWSTR,
    pub NumberOfBits: u32,
    pub Parameters: *mut u8,
}
impl windows_core::TypeKind for ADS_FAXNUMBER {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_FAXNUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_HOLD {
    pub ObjectName: windows_core::PWSTR,
    pub Amount: u32,
}
impl windows_core::TypeKind for ADS_HOLD {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_HOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_NETADDRESS {
    pub AddressType: u32,
    pub AddressLength: u32,
    pub Address: *mut u8,
}
impl windows_core::TypeKind for ADS_NETADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_NETADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_NT_SECURITY_DESCRIPTOR {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl windows_core::TypeKind for ADS_NT_SECURITY_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_NT_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_OBJECT_INFO {
    pub pszRDN: windows_core::PWSTR,
    pub pszObjectDN: windows_core::PWSTR,
    pub pszParentDN: windows_core::PWSTR,
    pub pszSchemaDN: windows_core::PWSTR,
    pub pszClassName: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_OBJECT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_OBJECT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_OCTET_LIST {
    pub Next: *mut ADS_OCTET_LIST,
    pub Length: u32,
    pub Data: *mut u8,
}
impl windows_core::TypeKind for ADS_OCTET_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_OCTET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl windows_core::TypeKind for ADS_OCTET_STRING {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_OCTET_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_PATH {
    pub Type: u32,
    pub VolumeName: windows_core::PWSTR,
    pub Path: windows_core::PWSTR,
}
impl windows_core::TypeKind for ADS_PATH {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_POSTALADDRESS {
    pub PostalAddress: [windows_core::PWSTR; 6],
}
impl windows_core::TypeKind for ADS_POSTALADDRESS {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_POSTALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_PROV_SPECIFIC {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl windows_core::TypeKind for ADS_PROV_SPECIFIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_PROV_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_REPLICAPOINTER {
    pub ServerName: windows_core::PWSTR,
    pub ReplicaType: u32,
    pub ReplicaNumber: u32,
    pub Count: u32,
    pub ReplicaAddressHints: *mut ADS_NETADDRESS,
}
impl windows_core::TypeKind for ADS_REPLICAPOINTER {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_REPLICAPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADS_SEARCHPREF_INFO {
    pub dwSearchPref: ADS_SEARCHPREF_ENUM,
    pub vValue: ADSVALUE,
    pub dwStatus: ADS_STATUSENUM,
}
impl windows_core::TypeKind for ADS_SEARCHPREF_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_SEARCHPREF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_SEARCH_COLUMN {
    pub pszAttrName: windows_core::PWSTR,
    pub dwADsType: ADSTYPE,
    pub pADsValues: *mut ADSVALUE,
    pub dwNumValues: u32,
    pub hReserved: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for ADS_SEARCH_COLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_SEARCH_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ADS_SEARCH_HANDLE(pub isize);
impl ADS_SEARCH_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for ADS_SEARCH_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ADS_SEARCH_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_SORTKEY {
    pub pszAttrType: windows_core::PWSTR,
    pub pszReserved: windows_core::PWSTR,
    pub fReverseorder: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for ADS_SORTKEY {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_SORTKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_TIMESTAMP {
    pub WholeSeconds: u32,
    pub EventID: u32,
}
impl windows_core::TypeKind for ADS_TIMESTAMP {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_TYPEDNAME {
    pub ObjectName: windows_core::PWSTR,
    pub Level: u32,
    pub Interval: u32,
}
impl windows_core::TypeKind for ADS_TYPEDNAME {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_TYPEDNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADS_VLV {
    pub dwBeforeCount: u32,
    pub dwAfterCount: u32,
    pub dwOffset: u32,
    pub dwContentCount: u32,
    pub pszTarget: windows_core::PWSTR,
    pub dwContextIDLength: u32,
    pub lpContextID: *mut u8,
}
impl windows_core::TypeKind for ADS_VLV {
    type TypeKind = windows_core::CopyType;
}
impl Default for ADS_VLV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADSystemInfo: windows_core::GUID = windows_core::GUID::from_u128(0x50b6327f_afd1_11d2_9cb9_0000f87a369e);
pub const ADsSecurityUtility: windows_core::GUID = windows_core::GUID::from_u128(0xf270c64a_ffb8_4ae4_85fe_3a75e5347966);
pub const AccessControlEntry: windows_core::GUID = windows_core::GUID::from_u128(0xb75ac000_9bdd_11d0_852c_00c04fd8d503);
pub const AccessControlList: windows_core::GUID = windows_core::GUID::from_u128(0xb85ea052_9bdd_11d0_852c_00c04fd8d503);
pub const BackLink: windows_core::GUID = windows_core::GUID::from_u128(0xfcbf906f_4080_11d1_a3ac_00c04fb950dc);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CQFORM {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsid: windows_core::GUID,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pszTitle: windows_core::PCWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for CQFORM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CQFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug)]
pub struct CQPAGE {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pPageProc: LPCQPAGEPROC,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub idPageName: i32,
    pub idPageTemplate: i32,
    pub pDlgProc: super::super::UI::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for CQPAGE {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CQPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CaseIgnoreList: windows_core::GUID = windows_core::GUID::from_u128(0x15f88a55_4680_11d1_a3b4_00c04fb950dc);
pub const DNWithBinary: windows_core::GUID = windows_core::GUID::from_u128(0x7e99c0a3_f935_11d2_ba96_00c04fb6d0d1);
pub const DNWithString: windows_core::GUID = windows_core::GUID::from_u128(0x334857cc_f934_11d2_ba96_00c04fb6d0d1);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOMAINDESC {
    pub pszName: windows_core::PWSTR,
    pub pszPath: windows_core::PWSTR,
    pub pszNCName: windows_core::PWSTR,
    pub pszTrustParent: windows_core::PWSTR,
    pub pszObjectClass: windows_core::PWSTR,
    pub ulFlags: u32,
    pub fDownLevel: super::super::Foundation::BOOL,
    pub pdChildList: *mut DOMAINDESC,
    pub pdNextSibling: *mut DOMAINDESC,
}
impl windows_core::TypeKind for DOMAINDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOMAINDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: windows_core::PSTR,
    pub DomainControllerAddress: windows_core::PSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_core::GUID,
    pub DomainName: windows_core::PSTR,
    pub DnsForestName: windows_core::PSTR,
    pub Flags: u32,
    pub DcSiteName: windows_core::PSTR,
    pub ClientSiteName: windows_core::PSTR,
}
impl windows_core::TypeKind for DOMAIN_CONTROLLER_INFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOMAIN_CONTROLLER_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: windows_core::PWSTR,
    pub DomainControllerAddress: windows_core::PWSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_core::GUID,
    pub DomainName: windows_core::PWSTR,
    pub DnsForestName: windows_core::PWSTR,
    pub Flags: u32,
    pub DcSiteName: windows_core::PWSTR,
    pub ClientSiteName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DOMAIN_CONTROLLER_INFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOMAIN_CONTROLLER_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DOMAIN_TREE {
    pub dsSize: u32,
    pub dwCount: u32,
    pub aDomains: [DOMAINDESC; 1],
}
impl windows_core::TypeKind for DOMAIN_TREE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DOMAIN_TREE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSA_NEWOBJ_DISPINFO {
    pub dwSize: u32,
    pub hObjClassIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub lpszWizTitle: windows_core::PWSTR,
    pub lpszContDisplayName: windows_core::PWSTR,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for DSA_NEWOBJ_DISPINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for DSA_NEWOBJ_DISPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBITEMA {
    pub cbStruct: u32,
    pub pszADsPath: windows_core::PCWSTR,
    pub pszClass: windows_core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [i8; 64],
    pub szIconLocation: [i8; 260],
    pub iIconResID: i32,
}
impl windows_core::TypeKind for DSBITEMA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSBITEMW {
    pub cbStruct: u32,
    pub pszADsPath: windows_core::PCWSTR,
    pub pszClass: windows_core::PCWSTR,
    pub dwMask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szDisplayName: [u16; 64],
    pub szIconLocation: [u16; 260],
    pub iIconResID: i32,
}
impl windows_core::TypeKind for DSBITEMW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSBITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell")]
#[derive(Clone, Copy, Debug)]
pub struct DSBROWSEINFOA {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: windows_core::PCSTR,
    pub pszTitle: windows_core::PCSTR,
    pub pszRoot: windows_core::PCWSTR,
    pub pszPath: windows_core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: windows_core::PCWSTR,
    pub pPassword: windows_core::PCWSTR,
    pub pszObjectClass: windows_core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(feature = "Win32_UI_Shell")]
impl windows_core::TypeKind for DSBROWSEINFOA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell")]
impl Default for DSBROWSEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell")]
#[derive(Clone, Copy, Debug)]
pub struct DSBROWSEINFOW {
    pub cbStruct: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pszCaption: windows_core::PCWSTR,
    pub pszTitle: windows_core::PCWSTR,
    pub pszRoot: windows_core::PCWSTR,
    pub pszPath: windows_core::PWSTR,
    pub cchPath: u32,
    pub dwFlags: u32,
    pub pfnCallback: super::super::UI::Shell::BFFCALLBACK,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwReturnFormat: u32,
    pub pUserName: windows_core::PCWSTR,
    pub pPassword: windows_core::PCWSTR,
    pub pszObjectClass: windows_core::PWSTR,
    pub cchObjectClass: u32,
}
#[cfg(feature = "Win32_UI_Shell")]
impl windows_core::TypeKind for DSBROWSEINFOW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Shell")]
impl Default for DSBROWSEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCLASSCREATIONINFO {
    pub dwFlags: u32,
    pub clsidWizardDialog: windows_core::GUID,
    pub clsidWizardPrimaryPage: windows_core::GUID,
    pub cWizardExtensions: u32,
    pub aWizardExtensions: [windows_core::GUID; 1],
}
impl windows_core::TypeKind for DSCLASSCREATIONINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCLASSCREATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSCOLUMN {
    pub dwFlags: u32,
    pub fmt: i32,
    pub cx: i32,
    pub idsName: i32,
    pub offsetProperty: i32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for DSCOLUMN {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSCOLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSDISPLAYSPECOPTIONS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub offsetAttribPrefix: u32,
    pub offsetUserName: u32,
    pub offsetPassword: u32,
    pub offsetServer: u32,
    pub offsetServerConfigPath: u32,
}
impl windows_core::TypeKind for DSDISPLAYSPECOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSDISPLAYSPECOPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOBJECT {
    pub dwFlags: u32,
    pub dwProviderFlags: u32,
    pub offsetName: u32,
    pub offsetClass: u32,
}
impl windows_core::TypeKind for DSOBJECT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOBJECTNAMES {
    pub clsidNamespace: windows_core::GUID,
    pub cItems: u32,
    pub aObjects: [DSOBJECT; 1],
}
impl windows_core::TypeKind for DSOBJECTNAMES {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOBJECTNAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOP_FILTER_FLAGS {
    pub Uplevel: DSOP_UPLEVEL_FILTER_FLAGS,
    pub flDownlevel: u32,
}
impl windows_core::TypeKind for DSOP_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOP_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOP_INIT_INFO {
    pub cbSize: u32,
    pub pwzTargetComputer: windows_core::PCWSTR,
    pub cDsScopeInfos: u32,
    pub aDsScopeInfos: *mut DSOP_SCOPE_INIT_INFO,
    pub flOptions: u32,
    pub cAttributesToFetch: u32,
    pub apwzAttributeNames: *const windows_core::PCWSTR,
}
impl windows_core::TypeKind for DSOP_INIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOP_INIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOP_SCOPE_INIT_INFO {
    pub cbSize: u32,
    pub flType: u32,
    pub flScope: u32,
    pub FilterFlags: DSOP_FILTER_FLAGS,
    pub pwzDcName: windows_core::PCWSTR,
    pub pwzADsPath: windows_core::PCWSTR,
    pub hr: windows_core::HRESULT,
}
impl windows_core::TypeKind for DSOP_SCOPE_INIT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOP_SCOPE_INIT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSOP_UPLEVEL_FILTER_FLAGS {
    pub flBothModes: u32,
    pub flMixedModeOnly: u32,
    pub flNativeModeOnly: u32,
}
impl windows_core::TypeKind for DSOP_UPLEVEL_FILTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSOP_UPLEVEL_FILTER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSPROPERTYPAGEINFO {
    pub offsetString: u32,
}
impl windows_core::TypeKind for DSPROPERTYPAGEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSPROPERTYPAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSQUERYCLASSLIST {
    pub cbStruct: u32,
    pub cClasses: i32,
    pub offsetClass: [u32; 1],
}
impl windows_core::TypeKind for DSQUERYCLASSLIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSQUERYCLASSLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSQUERYINITPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub pDefaultScope: windows_core::PWSTR,
    pub pDefaultSaveLocation: windows_core::PWSTR,
    pub pUserName: windows_core::PWSTR,
    pub pPassword: windows_core::PWSTR,
    pub pServer: windows_core::PWSTR,
}
impl windows_core::TypeKind for DSQUERYINITPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSQUERYINITPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSQUERYPARAMS {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub offsetQuery: i32,
    pub iColumns: i32,
    pub dwReserved: u32,
    pub aColumns: [DSCOLUMN; 1],
}
impl windows_core::TypeKind for DSQUERYPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSQUERYPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSROLE_OPERATION_STATE_INFO {
    pub OperationState: DSROLE_OPERATION_STATE,
}
impl windows_core::TypeKind for DSROLE_OPERATION_STATE_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSROLE_OPERATION_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    pub MachineRole: DSROLE_MACHINE_ROLE,
    pub Flags: u32,
    pub DomainNameFlat: windows_core::PWSTR,
    pub DomainNameDns: windows_core::PWSTR,
    pub DomainForestName: windows_core::PWSTR,
    pub DomainGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSROLE_PRIMARY_DOMAIN_INFO_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DSROLE_UPGRADE_STATUS_INFO {
    pub OperationState: u32,
    pub PreviousServerState: DSROLE_SERVER_STATE,
}
impl windows_core::TypeKind for DSROLE_UPGRADE_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DSROLE_UPGRADE_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_1A {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_1A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_1W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_1W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_1W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub SiteObjectName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub NtdsDsaObjectName: windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_2A {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_2W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub SiteObjectName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub NtdsDsaObjectName: windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_2W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3A {
    pub NetbiosName: windows_core::PSTR,
    pub DnsHostName: windows_core::PSTR,
    pub SiteName: windows_core::PSTR,
    pub SiteObjectName: windows_core::PSTR,
    pub ComputerObjectName: windows_core::PSTR,
    pub ServerObjectName: windows_core::PSTR,
    pub NtdsDsaObjectName: windows_core::PSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_3A {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_CONTROLLER_INFO_3W {
    pub NetbiosName: windows_core::PWSTR,
    pub DnsHostName: windows_core::PWSTR,
    pub SiteName: windows_core::PWSTR,
    pub SiteObjectName: windows_core::PWSTR,
    pub ComputerObjectName: windows_core::PWSTR,
    pub ServerObjectName: windows_core::PWSTR,
    pub NtdsDsaObjectName: windows_core::PWSTR,
    pub fIsPdc: super::super::Foundation::BOOL,
    pub fDsEnabled: super::super::Foundation::BOOL,
    pub fIsGc: super::super::Foundation::BOOL,
    pub fIsRodc: super::super::Foundation::BOOL,
    pub SiteObjectGuid: windows_core::GUID,
    pub ComputerObjectGuid: windows_core::GUID,
    pub ServerObjectGuid: windows_core::GUID,
    pub NtdsDsaObjectGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_CONTROLLER_INFO_3W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_CONTROLLER_INFO_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: windows_core::PSTR,
    pub DnsDomainName: windows_core::PSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_TRUSTSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_TRUSTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: windows_core::PWSTR,
    pub DnsDomainName: windows_core::PWSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::super::Foundation::PSID,
    pub DomainGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_DOMAIN_TRUSTSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_DOMAIN_TRUSTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_NAME_RESULTA {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMA,
}
impl windows_core::TypeKind for DS_NAME_RESULTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_NAME_RESULTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_NAME_RESULTW {
    pub cItems: u32,
    pub rItems: *mut DS_NAME_RESULT_ITEMW,
}
impl windows_core::TypeKind for DS_NAME_RESULTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_NAME_RESULTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_NAME_RESULT_ITEMA {
    pub status: u32,
    pub pDomain: windows_core::PSTR,
    pub pName: windows_core::PSTR,
}
impl windows_core::TypeKind for DS_NAME_RESULT_ITEMA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_NAME_RESULT_ITEMA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_NAME_RESULT_ITEMW {
    pub status: u32,
    pub pDomain: windows_core::PWSTR,
    pub pName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_NAME_RESULT_ITEMW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_NAME_RESULT_ITEMW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA {
    pub pszAttributeName: windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
impl windows_core::TypeKind for DS_REPL_ATTR_META_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA_2 {
    pub pszAttributeName: windows_core::PWSTR,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_REPL_ATTR_META_DATA_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
impl windows_core::TypeKind for DS_REPL_ATTR_META_DATA_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA; 1],
}
impl windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_2; 1],
}
impl windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_ATTR_VALUE_META_DATA_EXT {
    pub cNumEntries: u32,
    pub dwEnumerationContext: u32,
    pub rgMetaData: [DS_REPL_VALUE_META_DATA_EXT; 1],
}
impl windows_core::TypeKind for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_ATTR_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSOR {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: i64,
}
impl windows_core::TypeKind for DS_REPL_CURSOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSORS {
    pub cNumCursors: u32,
    pub dwReserved: u32,
    pub rgCursor: [DS_REPL_CURSOR; 1],
}
impl windows_core::TypeKind for DS_REPL_CURSORS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSORS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSORS_2 {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_2; 1],
}
impl windows_core::TypeKind for DS_REPL_CURSORS_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSORS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSORS_3W {
    pub cNumCursors: u32,
    pub dwEnumerationContext: u32,
    pub rgCursor: [DS_REPL_CURSOR_3W; 1],
}
impl windows_core::TypeKind for DS_REPL_CURSORS_3W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSORS_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSOR_2 {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for DS_REPL_CURSOR_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSOR_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSOR_3W {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub pszSourceDsaDN: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_REPL_CURSOR_3W {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSOR_3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_CURSOR_BLOB {
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub oszSourceDsaDN: u32,
}
impl windows_core::TypeKind for DS_REPL_CURSOR_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_CURSOR_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILURESW {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgDsaFailure: [DS_REPL_KCC_DSA_FAILUREW; 1],
}
impl windows_core::TypeKind for DS_REPL_KCC_DSA_FAILURESW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_KCC_DSA_FAILURESW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILUREW {
    pub pszDsaDN: windows_core::PWSTR,
    pub uuidDsaObjGuid: windows_core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
impl windows_core::TypeKind for DS_REPL_KCC_DSA_FAILUREW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_KCC_DSA_FAILUREW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_KCC_DSA_FAILUREW_BLOB {
    pub oszDsaDN: u32,
    pub uuidDsaObjGuid: windows_core::GUID,
    pub ftimeFirstFailure: super::super::Foundation::FILETIME,
    pub cNumFailures: u32,
    pub dwLastResult: u32,
}
impl windows_core::TypeKind for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_KCC_DSA_FAILUREW_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_NEIGHBORSW {
    pub cNumNeighbors: u32,
    pub dwReserved: u32,
    pub rgNeighbor: [DS_REPL_NEIGHBORW; 1],
}
impl windows_core::TypeKind for DS_REPL_NEIGHBORSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_NEIGHBORSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_NEIGHBORW {
    pub pszNamingContext: windows_core::PWSTR,
    pub pszSourceDsaDN: windows_core::PWSTR,
    pub pszSourceDsaAddress: windows_core::PWSTR,
    pub pszAsyncIntersiteTransportDN: windows_core::PWSTR,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidSourceDsaObjGuid: windows_core::GUID,
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
impl windows_core::TypeKind for DS_REPL_NEIGHBORW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_NEIGHBORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_NEIGHBORW_BLOB {
    pub oszNamingContext: u32,
    pub oszSourceDsaDN: u32,
    pub oszSourceDsaAddress: u32,
    pub oszAsyncIntersiteTransportDN: u32,
    pub dwReplicaFlags: u32,
    pub dwReserved: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidSourceDsaObjGuid: windows_core::GUID,
    pub uuidSourceDsaInvocationID: windows_core::GUID,
    pub uuidAsyncIntersiteTransportObjGuid: windows_core::GUID,
    pub usnLastObjChangeSynced: i64,
    pub usnAttributeFilter: i64,
    pub ftimeLastSyncSuccess: super::super::Foundation::FILETIME,
    pub ftimeLastSyncAttempt: super::super::Foundation::FILETIME,
    pub dwLastSyncResult: u32,
    pub cNumConsecutiveSyncFailures: u32,
}
impl windows_core::TypeKind for DS_REPL_NEIGHBORW_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_NEIGHBORW_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_OBJ_META_DATA {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA; 1],
}
impl windows_core::TypeKind for DS_REPL_OBJ_META_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_OBJ_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_OBJ_META_DATA_2 {
    pub cNumEntries: u32,
    pub dwReserved: u32,
    pub rgMetaData: [DS_REPL_ATTR_META_DATA_2; 1],
}
impl windows_core::TypeKind for DS_REPL_OBJ_META_DATA_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_OBJ_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_OPW {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub pszNamingContext: windows_core::PWSTR,
    pub pszDsaDN: windows_core::PWSTR,
    pub pszDsaAddress: windows_core::PWSTR,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidDsaObjGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_REPL_OPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_OPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_OPW_BLOB {
    pub ftimeEnqueued: super::super::Foundation::FILETIME,
    pub ulSerialNumber: u32,
    pub ulPriority: u32,
    pub OpType: DS_REPL_OP_TYPE,
    pub ulOptions: u32,
    pub oszNamingContext: u32,
    pub oszDsaDN: u32,
    pub oszDsaAddress: u32,
    pub uuidNamingContextObjGuid: windows_core::GUID,
    pub uuidDsaObjGuid: windows_core::GUID,
}
impl windows_core::TypeKind for DS_REPL_OPW_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_OPW_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_PENDING_OPSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub rgPendingOp: [DS_REPL_OPW; 1],
}
impl windows_core::TypeKind for DS_REPL_PENDING_OPSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_PENDING_OPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_QUEUE_STATISTICSW {
    pub ftimeCurrentOpStarted: super::super::Foundation::FILETIME,
    pub cNumPendingOps: u32,
    pub ftimeOldestSync: super::super::Foundation::FILETIME,
    pub ftimeOldestAdd: super::super::Foundation::FILETIME,
    pub ftimeOldestMod: super::super::Foundation::FILETIME,
    pub ftimeOldestDel: super::super::Foundation::FILETIME,
    pub ftimeOldestUpdRefs: super::super::Foundation::FILETIME,
}
impl windows_core::TypeKind for DS_REPL_QUEUE_STATISTICSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_QUEUE_STATISTICSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
}
impl windows_core::TypeKind for DS_REPL_VALUE_META_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_VALUE_META_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_2 {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_REPL_VALUE_META_DATA_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_VALUE_META_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_BLOB {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
}
impl windows_core::TypeKind for DS_REPL_VALUE_META_DATA_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_VALUE_META_DATA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_BLOB_EXT {
    pub oszAttributeName: u32,
    pub oszObjectDn: u32,
    pub cbData: u32,
    pub obData: u32,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub oszLastOriginatingDsaDN: u32,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
impl windows_core::TypeKind for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_VALUE_META_DATA_BLOB_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPL_VALUE_META_DATA_EXT {
    pub pszAttributeName: windows_core::PWSTR,
    pub pszObjectDn: windows_core::PWSTR,
    pub cbData: u32,
    pub pbData: *mut u8,
    pub ftimeDeleted: super::super::Foundation::FILETIME,
    pub ftimeCreated: super::super::Foundation::FILETIME,
    pub dwVersion: u32,
    pub ftimeLastOriginatingChange: super::super::Foundation::FILETIME,
    pub uuidLastOriginatingDsaInvocationID: windows_core::GUID,
    pub usnOriginatingChange: i64,
    pub usnLocalChange: i64,
    pub pszLastOriginatingDsaDN: windows_core::PWSTR,
    pub dwUserIdentifier: u32,
    pub dwPriorLinkState: u32,
    pub dwCurrentLinkState: u32,
}
impl windows_core::TypeKind for DS_REPL_VALUE_META_DATA_EXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPL_VALUE_META_DATA_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_ERRINFOA {
    pub pszSvrId: windows_core::PSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_core::PSTR,
}
impl windows_core::TypeKind for DS_REPSYNCALL_ERRINFOA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_ERRINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_ERRINFOW {
    pub pszSvrId: windows_core::PWSTR,
    pub error: DS_REPSYNCALL_ERROR,
    pub dwWin32Err: u32,
    pub pszSrcId: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_REPSYNCALL_ERRINFOW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_ERRINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_SYNCA {
    pub pszSrcId: windows_core::PSTR,
    pub pszDstId: windows_core::PSTR,
    pub pszNC: windows_core::PSTR,
    pub pguidSrc: *mut windows_core::GUID,
    pub pguidDst: *mut windows_core::GUID,
}
impl windows_core::TypeKind for DS_REPSYNCALL_SYNCA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_SYNCA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_SYNCW {
    pub pszSrcId: windows_core::PWSTR,
    pub pszDstId: windows_core::PWSTR,
    pub pszNC: windows_core::PWSTR,
    pub pguidSrc: *mut windows_core::GUID,
    pub pguidDst: *mut windows_core::GUID,
}
impl windows_core::TypeKind for DS_REPSYNCALL_SYNCW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_SYNCW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_UPDATEA {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOA,
    pub pSync: *mut DS_REPSYNCALL_SYNCA,
}
impl windows_core::TypeKind for DS_REPSYNCALL_UPDATEA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_UPDATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_REPSYNCALL_UPDATEW {
    pub event: DS_REPSYNCALL_EVENT,
    pub pErrInfo: *mut DS_REPSYNCALL_ERRINFOW,
    pub pSync: *mut DS_REPSYNCALL_SYNCW,
}
impl windows_core::TypeKind for DS_REPSYNCALL_UPDATEW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_REPSYNCALL_UPDATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_SCHEMA_GUID_MAPA {
    pub guid: windows_core::GUID,
    pub guidType: u32,
    pub pName: windows_core::PSTR,
}
impl windows_core::TypeKind for DS_SCHEMA_GUID_MAPA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_SCHEMA_GUID_MAPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_SCHEMA_GUID_MAPW {
    pub guid: windows_core::GUID,
    pub guidType: u32,
    pub pName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DS_SCHEMA_GUID_MAPW {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_SCHEMA_GUID_MAPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_SELECTION {
    pub pwzName: windows_core::PWSTR,
    pub pwzADsPath: windows_core::PWSTR,
    pub pwzClass: windows_core::PWSTR,
    pub pwzUPN: windows_core::PWSTR,
    pub pvarFetchedAttributes: *mut windows_core::VARIANT,
    pub flScopeType: u32,
}
impl windows_core::TypeKind for DS_SELECTION {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_SELECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_SELECTION_LIST {
    pub cItems: u32,
    pub cFetchedAttributes: u32,
    pub aDsSelection: [DS_SELECTION; 1],
}
impl windows_core::TypeKind for DS_SELECTION_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_SELECTION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DS_SITE_COST_INFO {
    pub errorCode: u32,
    pub cost: u32,
}
impl windows_core::TypeKind for DS_SITE_COST_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DS_SITE_COST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const Email: windows_core::GUID = windows_core::GUID::from_u128(0x8f92a857_478e_11d1_a3b4_00c04fb950dc);
pub const FaxNumber: windows_core::GUID = windows_core::GUID::from_u128(0xa5062215_4681_11d1_a3b4_00c04fb950dc);
pub const Hold: windows_core::GUID = windows_core::GUID::from_u128(0xb3ad3e13_4080_11d1_a3ac_00c04fb950dc);
pub const LargeInteger: windows_core::GUID = windows_core::GUID::from_u128(0x927971f5_0939_11d1_8be1_00c04fd8d503);
pub const NameTranslate: windows_core::GUID = windows_core::GUID::from_u128(0x274fae1f_3626_11d1_a3a4_00c04fb950dc);
pub const NetAddress: windows_core::GUID = windows_core::GUID::from_u128(0xb0b71247_4080_11d1_a3ac_00c04fb950dc);
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub struct OPENQUERYWINDOW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub clsidHandler: windows_core::GUID,
    pub pHandlerParameters: *mut core::ffi::c_void,
    pub clsidDefaultForm: windows_core::GUID,
    pub pPersistQuery: core::mem::ManuallyDrop<Option<IPersistQuery>>,
    pub Anonymous: OPENQUERYWINDOW_0,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Clone for OPENQUERYWINDOW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::TypeKind for OPENQUERYWINDOW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OPENQUERYWINDOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub union OPENQUERYWINDOW_0 {
    pub pFormParameters: *mut core::ffi::c_void,
    pub ppbFormParameters: core::mem::ManuallyDrop<Option<super::super::System::Com::StructuredStorage::IPropertyBag>>,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Clone for OPENQUERYWINDOW_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::TypeKind for OPENQUERYWINDOW_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OPENQUERYWINDOW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OctetList: windows_core::GUID = windows_core::GUID::from_u128(0x1241400f_4680_11d1_a3b4_00c04fb950dc);
pub const Path: windows_core::GUID = windows_core::GUID::from_u128(0xb2538919_4080_11d1_a3ac_00c04fb950dc);
pub const Pathname: windows_core::GUID = windows_core::GUID::from_u128(0x080d0d78_f421_11d0_a36e_00c04fb950dc);
pub const PostalAddress: windows_core::GUID = windows_core::GUID::from_u128(0x0a75afcd_4680_11d1_a3b4_00c04fb950dc);
pub const PropertyEntry: windows_core::GUID = windows_core::GUID::from_u128(0x72d3edc2_a4c4_11d0_8533_00c04fd8d503);
pub const PropertyValue: windows_core::GUID = windows_core::GUID::from_u128(0x7b9e38b0_a97c_11d0_8534_00c04fd8d503);
pub const ReplicaPointer: windows_core::GUID = windows_core::GUID::from_u128(0xf5d1badf_4080_11d1_a3ac_00c04fb950dc);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHEDULE {
    pub Size: u32,
    pub Bandwidth: u32,
    pub NumberOfSchedules: u32,
    pub Schedules: [SCHEDULE_HEADER; 1],
}
impl windows_core::TypeKind for SCHEDULE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCHEDULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SCHEDULE_HEADER {
    pub Type: u32,
    pub Offset: u32,
}
impl windows_core::TypeKind for SCHEDULE_HEADER {
    type TypeKind = windows_core::CopyType;
}
impl Default for SCHEDULE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SecurityDescriptor: windows_core::GUID = windows_core::GUID::from_u128(0xb958f73c_9bdd_11d0_852c_00c04fd8d503);
pub const Timestamp: windows_core::GUID = windows_core::GUID::from_u128(0xb2bed2eb_4080_11d1_a3ac_00c04fb950dc);
pub const TypedName: windows_core::GUID = windows_core::GUID::from_u128(0xb33143cb_4080_11d1_a3ac_00c04fb950dc);
pub const WinNTSystemInfo: windows_core::GUID = windows_core::GUID::from_u128(0x66182ec4_afd1_11d2_9cb9_0000f87a369e);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type LPCQADDFORMSPROC = Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pform: *mut CQFORM) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type LPCQADDPAGESPROC = Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, clsidform: *const windows_core::GUID, ppage: *mut CQPAGE) -> windows_core::HRESULT>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type LPCQPAGEPROC = Option<unsafe extern "system" fn(ppage: *mut CQPAGE, hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT>;
pub type LPDSENUMATTRIBUTES = Option<unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, pszattributename: windows_core::PCWSTR, pszdisplayname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
