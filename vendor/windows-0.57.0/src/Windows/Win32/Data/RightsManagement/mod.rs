#[inline]
pub unsafe fn DRMAcquireAdvisories<P0, P1>(hlicensestorage: u32, wszlicense: P0, wszurl: P1, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : u32, wszlicense : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireAdvisories(hlicensestorage, wszlicense.param().abi(), wszurl.param().abi(), pvcontext).ok()
}
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<P0>(hclient: u32, uflags: u32, pvreserved: *mut core::ffi::c_void, pwsztemplateids: Option<&[windows_core::PCWSTR]>, wszurl: P0, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : u32, uflags : u32, pvreserved : *mut core::ffi::c_void, ctemplates : u32, pwsztemplateids : *const windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireIssuanceLicenseTemplate(hclient, uflags, pvreserved, pwsztemplateids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pwsztemplateids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), wszurl.param().abi(), pvcontext).ok()
}
#[inline]
pub unsafe fn DRMAcquireLicense<P0, P1, P2, P3>(hsession: u32, uflags: u32, wszgroupidentitycredential: P0, wszrequestedrights: P1, wszcustomdata: P2, wszurl: P3, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : u32, uflags : u32, wszgroupidentitycredential : windows_core::PCWSTR, wszrequestedrights : windows_core::PCWSTR, wszcustomdata : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireLicense(hsession, uflags, wszgroupidentitycredential.param().abi(), wszrequestedrights.param().abi(), wszcustomdata.param().abi(), wszurl.param().abi(), pvcontext).ok()
}
#[inline]
pub unsafe fn DRMActivate<P0>(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut core::ffi::c_void, hparentwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMActivate(hclient : u32, uflags : u32, ulangid : u32, pactservinfo : *mut DRM_ACTSERV_INFO, pvcontext : *mut core::ffi::c_void, hparentwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DRMActivate(hclient, uflags, ulangid, pactservinfo, pvcontext, hparentwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMAddLicense<P0>(hlicensestorage: u32, uflags: u32, wszlicense: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : u32, uflags : u32, wszlicense : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMAddLicense(hlicensestorage, uflags, wszlicense.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : u32, hright : u32, huser : u32) -> windows_core::HRESULT);
    DRMAddRightWithUser(hissuancelicense, hright, huser).ok()
}
#[inline]
pub unsafe fn DRMAttest<P0>(henablingprincipal: u32, wszdata: P0, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : u32, wszdata : windows_core::PCWSTR, etype : DRMATTESTTYPE, pcattestedblob : *mut u32, wszattestedblob : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMAttest(henablingprincipal, wszdata.param().abi(), etype, pcattestedblob, core::mem::transmute(wszattestedblob)).ok()
}
#[inline]
pub unsafe fn DRMCheckSecurity(henv: u32, clevel: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : u32, clevel : u32) -> windows_core::HRESULT);
    DRMCheckSecurity(henv, clevel).ok()
}
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : u32) -> windows_core::HRESULT);
    DRMClearAllRights(hissuancelicense).ok()
}
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : u32) -> windows_core::HRESULT);
    DRMCloseEnvironmentHandle(henv).ok()
}
#[inline]
pub unsafe fn DRMCloseHandle(handle: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : u32) -> windows_core::HRESULT);
    DRMCloseHandle(handle).ok()
}
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : u32) -> windows_core::HRESULT);
    DRMClosePubHandle(hpub).ok()
}
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : u32) -> windows_core::HRESULT);
    DRMCloseQueryHandle(hquery).ok()
}
#[inline]
pub unsafe fn DRMCloseSession(hsession: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : u32) -> windows_core::HRESULT);
    DRMCloseSession(hsession).ok()
}
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[windows_core::PCWSTR], pcchain: *mut u32, wszchain: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : u32, rgwszcertificates : *const windows_core::PCWSTR, pcchain : *mut u32, wszchain : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMConstructCertificateChain(rgwszcertificates.len().try_into().unwrap(), core::mem::transmute(rgwszcertificates.as_ptr()), pcchain, core::mem::transmute(wszchain)).ok()
}
#[inline]
pub unsafe fn DRMCreateBoundLicense<P0>(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: P0, phboundlicense: *mut u32, pherrorlog: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateBoundLicense(henv : u32, pparams : *mut DRMBOUNDLICENSEPARAMS, wszlicensechain : windows_core::PCWSTR, phboundlicense : *mut u32, pherrorlog : *mut u32) -> windows_core::HRESULT);
    DRMCreateBoundLicense(henv, pparams, wszlicensechain.param().abi(), phboundlicense, pherrorlog).ok()
}
#[inline]
pub unsafe fn DRMCreateClientSession<P0, P1>(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: P0, wszgroupid: P1, phclient: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : DRMCALLBACK, ucallbackversion : u32, wszgroupidprovidertype : windows_core::PCWSTR, wszgroupid : windows_core::PCWSTR, phclient : *mut u32) -> windows_core::HRESULT);
    DRMCreateClientSession(pfncallback, ucallbackversion, wszgroupidprovidertype.param().abi(), wszgroupid.param().abi(), phclient).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<P0, P1>(hboundlicense: u32, wszright: P0, hauxlib: u32, wszauxplug: P1, phdecryptor: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : u32, wszright : windows_core::PCWSTR, hauxlib : u32, wszauxplug : windows_core::PCWSTR, phdecryptor : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingBitsDecryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), phdecryptor).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<P0, P1>(hboundlicense: u32, wszright: P0, hauxlib: u32, wszauxplug: P1, phencryptor: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : u32, wszright : windows_core::PCWSTR, hauxlib : u32, wszauxplug : windows_core::PCWSTR, phencryptor : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingBitsEncryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), phencryptor).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal<P0, P1>(henv: u32, hlibrary: u32, wszobject: P0, pidprincipal: *mut DRMID, wszcredentials: P1, phenablingprincipal: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingPrincipal(henv : u32, hlibrary : u32, wszobject : windows_core::PCWSTR, pidprincipal : *mut DRMID, wszcredentials : windows_core::PCWSTR, phenablingprincipal : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingPrincipal(henv, hlibrary, wszobject.param().abi(), pidprincipal, wszcredentials.param().abi(), phenablingprincipal).ok()
}
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<P0, P1, P2>(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: P0, wszreferralinfourl: P1, howner: u32, wszissuancelicense: P2, hboundlicense: u32, phissuancelicense: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::super::Foundation:: SYSTEMTIME, psttimeuntil : *mut super::super::Foundation:: SYSTEMTIME, wszreferralinfoname : windows_core::PCWSTR, wszreferralinfourl : windows_core::PCWSTR, howner : u32, wszissuancelicense : windows_core::PCWSTR, hboundlicense : u32, phissuancelicense : *mut u32) -> windows_core::HRESULT);
    DRMCreateIssuanceLicense(psttimefrom, psttimeuntil, wszreferralinfoname.param().abi(), wszreferralinfourl.param().abi(), howner, wszissuancelicense.param().abi(), hboundlicense, phissuancelicense).ok()
}
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<P0>(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: P0, phlicensestorage: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : u32, hdefaultlibrary : u32, hclient : u32, uflags : u32, wszissuancelicense : windows_core::PCWSTR, phlicensestorage : *mut u32) -> windows_core::HRESULT);
    DRMCreateLicenseStorageSession(henv, hdefaultlibrary, hclient, uflags, wszissuancelicense.param().abi(), phlicensestorage).ok()
}
#[inline]
pub unsafe fn DRMCreateRight<P0>(wszrightname: P0, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: Option<*const windows_core::PCWSTR>, pwszextendedinfovalue: Option<*const windows_core::PCWSTR>, phright: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_core::PCWSTR, pstfrom : *mut super::super::Foundation:: SYSTEMTIME, pstuntil : *mut super::super::Foundation:: SYSTEMTIME, cextendedinfo : u32, pwszextendedinfoname : *const windows_core::PCWSTR, pwszextendedinfovalue : *const windows_core::PCWSTR, phright : *mut u32) -> windows_core::HRESULT);
    DRMCreateRight(wszrightname.param().abi(), pstfrom, pstuntil, cextendedinfo, core::mem::transmute(pwszextendedinfoname.unwrap_or(std::ptr::null())), core::mem::transmute(pwszextendedinfovalue.unwrap_or(std::ptr::null())), phright).ok()
}
#[inline]
pub unsafe fn DRMCreateUser<P0, P1, P2>(wszusername: P0, wszuserid: P1, wszuseridtype: P2, phuser: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_core::PCWSTR, wszuserid : windows_core::PCWSTR, wszuseridtype : windows_core::PCWSTR, phuser : *mut u32) -> windows_core::HRESULT);
    DRMCreateUser(wszusername.param().abi(), wszuserid.param().abi(), wszuseridtype.param().abi(), phuser).ok()
}
#[inline]
pub unsafe fn DRMDecode<P0, P1>(wszalgid: P0, wszencodedstring: P1, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_core::PCWSTR, wszencodedstring : windows_core::PCWSTR, pudecodeddatalen : *mut u32, pbdecodeddata : *mut u8) -> windows_core::HRESULT);
    DRMDecode(wszalgid.param().abi(), wszencodedstring.param().abi(), pudecodeddatalen, pbdecodeddata).ok()
}
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<P0>(wszchain: P0, iwhich: u32, pccert: *mut u32, wszcert: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_core::PCWSTR, iwhich : u32, pccert : *mut u32, wszcert : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMDeconstructCertificateChain(wszchain.param().abi(), iwhich, pccert, core::mem::transmute(wszcert)).ok()
}
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : u32, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    DRMDecrypt(hcryptoprovider, iposition, cnuminbytes, pbindata, pcnumoutbytes, pboutdata).ok()
}
#[inline]
pub unsafe fn DRMDeleteLicense<P0>(hsession: u32, wszlicenseid: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : u32, wszlicenseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMDeleteLicense(hsession, wszlicenseid.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : u32, phcopy : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateEnvironmentHandle(htocopy, phcopy).ok()
}
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : u32, phcopy : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateHandle(htocopy, phcopy).ok()
}
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : u32, phpubout : *mut u32) -> windows_core::HRESULT);
    DRMDuplicatePubHandle(hpubin, phpubout).ok()
}
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : u32, phsessionout : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateSession(hsessionin, phsessionout).ok()
}
#[inline]
pub unsafe fn DRMEncode<P0>(wszalgid: P0, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_core::PCWSTR, udatalen : u32, pbdecodeddata : *mut u8, puencodedstringlen : *mut u32, wszencodedstring : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMEncode(wszalgid.param().abi(), udatalen, pbdecodeddata, puencodedstringlen, core::mem::transmute(wszencodedstring)).ok()
}
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : u32, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    DRMEncrypt(hcryptoprovider, iposition, cnuminbytes, pbindata, pcnumoutbytes, pboutdata).ok()
}
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : u32, uflags : u32, uindex : u32, pfsharedflag : *mut super::super::Foundation:: BOOL, pucertificatedatalen : *mut u32, wszcertificatedata : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMEnumerateLicense(hsession, uflags, uindex, pfsharedflag, pucertificatedatalen, core::mem::transmute(wszcertificatedata)).ok()
}
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: windows_core::PWSTR, puvaluelength: *mut u32, wszvalue: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : u32, uindex : u32, punamelength : *mut u32, wszname : windows_core::PWSTR, puvaluelength : *mut u32, wszvalue : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetApplicationSpecificData(hissuancelicense, uindex, punamelength, core::mem::transmute(wszname), puvaluelength, core::mem::transmute(wszvalue)).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<P0>(hqueryroot: u32, wszattribute: P0, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : u32, wszattribute : windows_core::PCWSTR, iwhich : u32, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetBoundLicenseAttribute(hqueryroot, wszattribute.param().abi(), iwhich, peencoding, pcbuffer, pbbuffer).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<P0>(hqueryroot: u32, wszattribute: P0, pcattributes: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : u32, wszattribute : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseAttributeCount(hqueryroot, wszattribute.param().abi(), pcattributes).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<P0>(hqueryroot: u32, wszsubobjecttype: P0, iwhich: u32, phsubobject: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, iwhich : u32, phsubobject : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iwhich, phsubobject).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<P0>(hqueryroot: u32, wszsubobjecttype: P0, pcsubobjects: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), pcsubobjects).ok()
}
#[inline]
pub unsafe fn DRMGetCertificateChainCount<P0>(wszchain: P0, pccertcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_core::PCWSTR, pccertcount : *mut u32) -> windows_core::HRESULT);
    DRMGetCertificateChainCount(wszchain.param().abi(), pccertcount).ok()
}
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetClientVersion(pdrmclientversioninfo : *mut DRM_CLIENT_VERSION_INFO) -> windows_core::HRESULT);
    DRMGetClientVersion(pdrmclientversioninfo).ok()
}
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<P0>(handle: u32, wszattribute: P0, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : u32, wszattribute : windows_core::PCWSTR, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetEnvironmentInfo(handle, wszattribute.param().abi(), peencoding, pcbuffer, pbbuffer).ok()
}
#[inline]
pub unsafe fn DRMGetInfo<P0>(handle: u32, wszattribute: P0, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetInfo(handle : u32, wszattribute : windows_core::PCWSTR, peencoding : *const DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetInfo(handle, wszattribute.param().abi(), peencoding, pcbuffer, pbbuffer).ok()
}
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : u32, pcdays : *mut u32) -> windows_core::HRESULT);
    DRMGetIntervalTime(hissuancelicense, pcdays).ok()
}
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: windows_core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: windows_core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : u32, psttimefrom : *mut super::super::Foundation:: SYSTEMTIME, psttimeuntil : *mut super::super::Foundation:: SYSTEMTIME, uflags : u32, pudistributionpointnamelength : *mut u32, wszdistributionpointname : windows_core::PWSTR, pudistributionpointurllength : *mut u32, wszdistributionpointurl : windows_core::PWSTR, phowner : *mut u32, pfofficial : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMGetIssuanceLicenseInfo(hissuancelicense, psttimefrom, psttimeuntil, uflags, pudistributionpointnamelength, core::mem::transmute(wszdistributionpointname), pudistributionpointurllength, core::mem::transmute(wszdistributionpointurl), phowner, pfofficial).ok()
}
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : u32, puissuancelicensetemplatelength : *mut u32, wszissuancelicensetemplate : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetIssuanceLicenseTemplate(hissuancelicense, puissuancelicensetemplatelength, core::mem::transmute(wszissuancelicensetemplate)).ok()
}
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: windows_core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: windows_core::PWSTR, puskuidlength: *mut u32, wszskuid: windows_core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: windows_core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: windows_core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : u32, pucontentidlength : *mut u32, wszcontentid : windows_core::PWSTR, pucontentidtypelength : *mut u32, wszcontentidtype : windows_core::PWSTR, puskuidlength : *mut u32, wszskuid : windows_core::PWSTR, puskuidtypelength : *mut u32, wszskuidtype : windows_core::PWSTR, pucontenttypelength : *mut u32, wszcontenttype : windows_core::PWSTR, pucontentnamelength : *mut u32, wszcontentname : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetMetaData(hissuancelicense, pucontentidlength, core::mem::transmute(wszcontentid), pucontentidtypelength, core::mem::transmute(wszcontentidtype), puskuidlength, core::mem::transmute(wszskuid), puskuidtypelength, core::mem::transmute(wszskuidtype), pucontenttypelength, core::mem::transmute(wszcontenttype), pucontentnamelength, core::mem::transmute(wszcontentname)).ok()
}
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: windows_core::PWSTR, pudescriptionlength: *mut u32, wszdescription: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : u32, uindex : u32, pulcid : *mut u32, punamelength : *mut u32, wszname : windows_core::PWSTR, pudescriptionlength : *mut u32, wszdescription : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetNameAndDescription(hissuancelicense, uindex, pulcid, punamelength, core::mem::transmute(wszname), pudescriptionlength, core::mem::transmute(wszdescription)).ok()
}
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : u32, puownerlicenselength : *mut u32, wszownerlicense : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetOwnerLicense(hissuancelicense, puownerlicenselength, core::mem::transmute(wszownerlicense)).ok()
}
#[inline]
pub unsafe fn DRMGetProcAddress<P0>(hlibrary: u32, wszprocname: P0, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : u32, wszprocname : windows_core::PCWSTR, ppfnprocaddress : *mut super::super::Foundation:: FARPROC) -> windows_core::HRESULT);
    DRMGetProcAddress(hlibrary, wszprocname.param().abi(), ppfnprocaddress).ok()
}
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: windows_core::PWSTR, puidtypelength: *mut u32, wszidtype: windows_core::PWSTR, puurllength: *mut u32, wszrl: windows_core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: windows_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : u32, puidlength : *mut u32, wszid : windows_core::PWSTR, puidtypelength : *mut u32, wszidtype : windows_core::PWSTR, puurllength : *mut u32, wszrl : windows_core::PWSTR, pstfrequency : *mut super::super::Foundation:: SYSTEMTIME, punamelength : *mut u32, wszname : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetRevocationPoint(hissuancelicense, puidlength, core::mem::transmute(wszid), puidtypelength, core::mem::transmute(wszidtype), puurllength, core::mem::transmute(wszrl), pstfrequency, punamelength, core::mem::transmute(wszname), pupublickeylength, core::mem::transmute(wszpublickey)).ok()
}
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: windows_core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : u32, uindex : u32, puextendedinfonamelength : *mut u32, wszextendedinfoname : windows_core::PWSTR, puextendedinfovaluelength : *mut u32, wszextendedinfovalue : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetRightExtendedInfo(hright, uindex, puextendedinfonamelength, core::mem::transmute(wszextendedinfoname), puextendedinfovaluelength, core::mem::transmute(wszextendedinfovalue)).ok()
}
#[inline]
pub unsafe fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: windows_core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : u32, purightnamelength : *mut u32, wszrightname : windows_core::PWSTR, pstfrom : *mut super::super::Foundation:: SYSTEMTIME, pstuntil : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::HRESULT);
    DRMGetRightInfo(hright, purightnamelength, core::mem::transmute(wszrightname), pstfrom, pstuntil).ok()
}
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: windows_core::PWSTR, pupathlen: *mut u32, wszpath: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : u32, putypelen : *mut u32, wsztype : windows_core::PWSTR, pupathlen : *mut u32, wszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetSecurityProvider(uflags, putypelen, core::mem::transmute(wsztype), pupathlen, core::mem::transmute(wszpath)).ok()
}
#[inline]
pub unsafe fn DRMGetServiceLocation<P0>(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: P0, puserviceurllength: *mut u32, wszserviceurl: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : u32, uservicetype : u32, uservicelocation : u32, wszissuancelicense : windows_core::PCWSTR, puserviceurllength : *mut u32, wszserviceurl : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetServiceLocation(hclient, uservicetype, uservicelocation, wszissuancelicense.param().abi(), puserviceurllength, core::mem::transmute(wszserviceurl)).ok()
}
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<P0, P1, P2>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: P0, wszclientlicensorcertificate: P1, pfncallback: DRMCALLBACK, wszurl: P2, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : u32, hissuancelicense : u32, uflags : u32, pbsymkey : *mut u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, wszclientlicensorcertificate : windows_core::PCWSTR, pfncallback : DRMCALLBACK, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMGetSignedIssuanceLicense(henv, hissuancelicense, uflags, pbsymkey, cbsymkey, wszsymkeytype.param().abi(), wszclientlicensorcertificate.param().abi(), pfncallback, wszurl.param().abi(), pvcontext).ok()
}
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<P0>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: Option<&[u8]>, wszsymkeytype: P0, pvreserved: Option<*const core::ffi::c_void>, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : u32, hissuancelicense : u32, uflags : u32, pbsymkey : *const u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, henablingprincipal : u32, hboundlicenseclc : u32, pfncallback : DRMCALLBACK, pvcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DRMGetSignedIssuanceLicenseEx(henv, hissuancelicense, uflags, core::mem::transmute(pbsymkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbsymkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), wszsymkeytype.param().abi(), core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())), henablingprincipal, hboundlicenseclc, pfncallback, pvcontext).ok()
}
#[inline]
pub unsafe fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetTime(henv : u32, etimeridtype : DRMTIMETYPE, potimeobject : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::HRESULT);
    DRMGetTime(henv, etimeridtype, potimeobject).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<P0>(hqueryroot: u32, wszattributetype: P0, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : u32, wszattributetype : windows_core::PCWSTR, iwhich : u32, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetUnboundLicenseAttribute(hqueryroot, wszattributetype.param().abi(), iwhich, peencoding, pcbuffer, pbbuffer).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<P0>(hqueryroot: u32, wszattributetype: P0, pcattributes: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : u32, wszattributetype : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseAttributeCount(hqueryroot, wszattributetype.param().abi(), pcattributes).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<P0>(hqueryroot: u32, wszsubobjecttype: P0, iindex: u32, phsubquery: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, iindex : u32, phsubquery : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iindex, phsubquery).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<P0>(hqueryroot: u32, wszsubobjecttype: P0, pcsubobjects: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), pcsubobjects).ok()
}
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: windows_core::PWSTR, puminversionlength: *mut u32, wszminversion: windows_core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: windows_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: windows_core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: windows_core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : u32, uindex : u32, peusagepolicytype : *mut DRM_USAGEPOLICY_TYPE, pfexclusion : *mut super::super::Foundation:: BOOL, punamelength : *mut u32, wszname : windows_core::PWSTR, puminversionlength : *mut u32, wszminversion : windows_core::PWSTR, pumaxversionlength : *mut u32, wszmaxversion : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR, pudigestalgorithmlength : *mut u32, wszdigestalgorithm : windows_core::PWSTR, pcbdigest : *mut u32, pbdigest : *mut u8) -> windows_core::HRESULT);
    DRMGetUsagePolicy(hissuancelicense, uindex, peusagepolicytype, pfexclusion, punamelength, core::mem::transmute(wszname), puminversionlength, core::mem::transmute(wszminversion), pumaxversionlength, core::mem::transmute(wszmaxversion), pupublickeylength, core::mem::transmute(wszpublickey), pudigestalgorithmlength, core::mem::transmute(wszdigestalgorithm), pcbdigest, pbdigest).ok()
}
#[inline]
pub unsafe fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: windows_core::PWSTR, puuseridlength: *mut u32, wszuserid: windows_core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : u32, puusernamelength : *mut u32, wszusername : windows_core::PWSTR, puuseridlength : *mut u32, wszuserid : windows_core::PWSTR, puuseridtypelength : *mut u32, wszuseridtype : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetUserInfo(huser, puusernamelength, core::mem::transmute(wszusername), puuseridlength, core::mem::transmute(wszuserid), puuseridtypelength, core::mem::transmute(wszuseridtype)).ok()
}
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : u32, huser : u32, uindex : u32, phright : *mut u32) -> windows_core::HRESULT);
    DRMGetUserRights(hissuancelicense, huser, uindex, phright).ok()
}
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : u32, uindex : u32, phuser : *mut u32) -> windows_core::HRESULT);
    DRMGetUsers(hissuancelicense, uindex, phuser).ok()
}
#[inline]
pub unsafe fn DRMInitEnvironment<P0, P1, P2>(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: P0, wszmanifestcredentials: P1, wszmachinecredentials: P2, phenv: *mut u32, phdefaultlibrary: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : DRMSECURITYPROVIDERTYPE, especification : DRMSPECTYPE, wszsecurityprovider : windows_core::PCWSTR, wszmanifestcredentials : windows_core::PCWSTR, wszmachinecredentials : windows_core::PCWSTR, phenv : *mut u32, phdefaultlibrary : *mut u32) -> windows_core::HRESULT);
    DRMInitEnvironment(esecurityprovidertype, especification, wszsecurityprovider.param().abi(), wszmanifestcredentials.param().abi(), wszmachinecredentials.param().abi(), phenv, phdefaultlibrary).ok()
}
#[inline]
pub unsafe fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMIsActivated(hclient : u32, uflags : u32, pactservinfo : *mut DRM_ACTSERV_INFO) -> windows_core::HRESULT);
    DRMIsActivated(hclient, uflags, pactservinfo).ok()
}
#[inline]
pub unsafe fn DRMIsWindowProtected<P0>(hwnd: P0, pfprotected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::super::Foundation:: HWND, pfprotected : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMIsWindowProtected(hwnd.param().abi(), pfprotected).ok()
}
#[inline]
pub unsafe fn DRMLoadLibrary<P0, P1>(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: P0, wszcredentials: P1, phlibrary: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : u32, especification : DRMSPECTYPE, wszlibraryprovider : windows_core::PCWSTR, wszcredentials : windows_core::PCWSTR, phlibrary : *mut u32) -> windows_core::HRESULT);
    DRMLoadLibrary(henv, especification, wszlibraryprovider.param().abi(), wszcredentials.param().abi(), phlibrary).ok()
}
#[inline]
pub unsafe fn DRMParseUnboundLicense<P0>(wszcertificate: P0, phqueryroot: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_core::PCWSTR, phqueryroot : *mut u32) -> windows_core::HRESULT);
    DRMParseUnboundLicense(wszcertificate.param().abi(), phqueryroot).ok()
}
#[inline]
pub unsafe fn DRMRegisterContent<P0>(fregister: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMRegisterContent(fregister.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRegisterProtectedWindow<P0>(henv: u32, hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : u32, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DRMRegisterProtectedWindow(henv, hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRegisterRevocationList<P0>(henv: u32, wszrevocationlist: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : u32, wszrevocationlist : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMRegisterRevocationList(henv, wszrevocationlist.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRepair() -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMRepair() -> windows_core::HRESULT);
    DRMRepair().ok()
}
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<P0, P1, P2>(hissuancelicense: u32, fdelete: P0, wszname: P1, wszvalue: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, wszname : windows_core::PCWSTR, wszvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetApplicationSpecificData(hissuancelicense, fdelete.param().abi(), wszname.param().abi(), wszvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut core::ffi::c_void, dwlen: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : DRMGLOBALOPTIONS, pvdata : *mut core::ffi::c_void, dwlen : u32) -> windows_core::HRESULT);
    DRMSetGlobalOptions(eglobaloptions, pvdata, dwlen).ok()
}
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : u32, cdays : u32) -> windows_core::HRESULT);
    DRMSetIntervalTime(hissuancelicense, cdays).ok()
}
#[inline]
pub unsafe fn DRMSetMetaData<P0, P1, P2, P3, P4, P5>(hissuancelicense: u32, wszcontentid: P0, wszcontentidtype: P1, wszskuid: P2, wszskuidtype: P3, wszcontenttype: P4, wszcontentname: P5) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : u32, wszcontentid : windows_core::PCWSTR, wszcontentidtype : windows_core::PCWSTR, wszskuid : windows_core::PCWSTR, wszskuidtype : windows_core::PCWSTR, wszcontenttype : windows_core::PCWSTR, wszcontentname : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetMetaData(hissuancelicense, wszcontentid.param().abi(), wszcontentidtype.param().abi(), wszskuid.param().abi(), wszskuidtype.param().abi(), wszcontenttype.param().abi(), wszcontentname.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetNameAndDescription<P0, P1, P2>(hissuancelicense: u32, fdelete: P0, lcid: u32, wszname: P1, wszdescription: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, lcid : u32, wszname : windows_core::PCWSTR, wszdescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetNameAndDescription(hissuancelicense, fdelete.param().abi(), lcid, wszname.param().abi(), wszdescription.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetRevocationPoint<P0, P1, P2, P3, P4, P5>(hissuancelicense: u32, fdelete: P0, wszid: P1, wszidtype: P2, wszurl: P3, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: P4, wszpublickey: P5) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, wszid : windows_core::PCWSTR, wszidtype : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pstfrequency : *mut super::super::Foundation:: SYSTEMTIME, wszname : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetRevocationPoint(hissuancelicense, fdelete.param().abi(), wszid.param().abi(), wszidtype.param().abi(), wszurl.param().abi(), pstfrequency, wszname.param().abi(), wszpublickey.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetUsagePolicy<P0, P1, P2, P3, P4, P5, P6>(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: P0, fexclusion: P1, wszname: P2, wszminversion: P3, wszmaxversion: P4, wszpublickey: P5, wszdigestalgorithm: P6, pbdigest: *mut u8, cbdigest: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : u32, eusagepolicytype : DRM_USAGEPOLICY_TYPE, fdelete : super::super::Foundation:: BOOL, fexclusion : super::super::Foundation:: BOOL, wszname : windows_core::PCWSTR, wszminversion : windows_core::PCWSTR, wszmaxversion : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR, wszdigestalgorithm : windows_core::PCWSTR, pbdigest : *mut u8, cbdigest : u32) -> windows_core::HRESULT);
    DRMSetUsagePolicy(hissuancelicense, eusagepolicytype, fdelete.param().abi(), fexclusion.param().abi(), wszname.param().abi(), wszminversion.param().abi(), wszmaxversion.param().abi(), wszpublickey.param().abi(), wszdigestalgorithm.param().abi(), pbdigest, cbdigest).ok()
}
#[inline]
pub unsafe fn DRMVerify<P0>(wszdata: P0, pcattesteddata: *mut u32, wszattesteddata: windows_core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: windows_core::PWSTR, pcmanifest: *mut u32, wszmanifest: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_core::PCWSTR, pcattesteddata : *mut u32, wszattesteddata : windows_core::PWSTR, petype : *mut DRMATTESTTYPE, pcprincipal : *mut u32, wszprincipal : windows_core::PWSTR, pcmanifest : *mut u32, wszmanifest : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMVerify(wszdata.param().abi(), pcattesteddata, core::mem::transmute(wszattesteddata), petype, pcprincipal, core::mem::transmute(wszprincipal), pcmanifest, core::mem::transmute(wszmanifest)).ok()
}
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = DRMATTESTTYPE(0i32);
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = DRMATTESTTYPE(1i32);
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
pub const DRMCALLBACKVERSION: u32 = 1u32;
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = DRMENCODINGTYPE(0i32);
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = DRMENCODINGTYPE(2i32);
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = DRMENCODINGTYPE(5i32);
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = DRMENCODINGTYPE(1i32);
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = DRMENCODINGTYPE(3i32);
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = DRMENCODINGTYPE(4i32);
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(1i32);
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(0i32);
pub const DRMHANDLE_INVALID: u32 = 0u32;
pub const DRMHSESSION_INVALID: u32 = 0u32;
pub const DRMIDVERSION: u32 = 0u32;
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = DRMSECURITYPROVIDERTYPE(0i32);
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = DRMSPECTYPE(1i32);
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = DRMSPECTYPE(0i32);
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = DRMTIMETYPE(1i32);
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = DRMTIMETYPE(0i32);
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
pub const DRM_AILT_CANCEL: u32 = 4u32;
pub const DRM_AILT_NONSILENT: u32 = 1u32;
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
pub const DRM_AL_CANCEL: u32 = 4u32;
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
pub const DRM_AL_NONSILENT: u32 = 1u32;
pub const DRM_AL_NOPERSIST: u32 = 2u32;
pub const DRM_AL_NOUI: u32 = 16u32;
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: windows_core::PCWSTR = windows_core::w!("PassportAuthProvider");
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: windows_core::PCWSTR = windows_core::w!("WindowsAuthProvider");
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(0i32);
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(1i32);
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(2i32);
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
pub const DRM_EL_EUL: u32 = 32u32;
pub const DRM_EL_EUL_LID: u32 = 64u32;
pub const DRM_EL_EXPIRED: u32 = 4096u32;
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
pub const DRM_EL_MACHINE: u32 = 1u32;
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = DRM_STATUS_MSG(3i32);
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = DRM_STATUS_MSG(5i32);
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = DRM_STATUS_MSG(6i32);
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(2i32);
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = DRM_STATUS_MSG(1i32);
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = DRM_STATUS_MSG(0i32);
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(4i32);
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
pub const DRM_REUSE_KEY: u32 = 64u32;
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
pub const DRM_SIGN_CANCEL: u32 = 4u32;
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
pub const DRM_SIGN_ONLINE: u32 = 1u32;
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(2i32);
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(0i32);
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(1i32);
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(3i32);
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMATTESTTYPE(pub i32);
impl windows_core::TypeKind for DRMATTESTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMATTESTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMATTESTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMENCODINGTYPE(pub i32);
impl windows_core::TypeKind for DRMENCODINGTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMENCODINGTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMENCODINGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMGLOBALOPTIONS(pub i32);
impl windows_core::TypeKind for DRMGLOBALOPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMGLOBALOPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMGLOBALOPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMSECURITYPROVIDERTYPE(pub i32);
impl windows_core::TypeKind for DRMSECURITYPROVIDERTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMSECURITYPROVIDERTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMSECURITYPROVIDERTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMSPECTYPE(pub i32);
impl windows_core::TypeKind for DRMSPECTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMSPECTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMSPECTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRMTIMETYPE(pub i32);
impl windows_core::TypeKind for DRMTIMETYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRMTIMETYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRMTIMETYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRM_DISTRIBUTION_POINT_INFO(pub i32);
impl windows_core::TypeKind for DRM_DISTRIBUTION_POINT_INFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRM_DISTRIBUTION_POINT_INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRM_DISTRIBUTION_POINT_INFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRM_STATUS_MSG(pub i32);
impl windows_core::TypeKind for DRM_STATUS_MSG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRM_STATUS_MSG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRM_STATUS_MSG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DRM_USAGEPOLICY_TYPE(pub i32);
impl windows_core::TypeKind for DRM_USAGEPOLICY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DRM_USAGEPOLICY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DRM_USAGEPOLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: windows_core::PWSTR,
    pub wszRightsGroup: windows_core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: windows_core::PWSTR,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for DRMBOUNDLICENSEPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: windows_core::PWSTR,
    pub wszID: windows_core::PWSTR,
}
impl windows_core::TypeKind for DRMID {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: windows_core::PWSTR,
    pub wszURL: windows_core::PWSTR,
}
impl windows_core::TypeKind for DRM_ACTSERV_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl windows_core::TypeKind for DRM_CLIENT_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: windows_core::PWSTR,
    pub wszLocalFilename: windows_core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: windows_core::PWSTR,
}
impl windows_core::TypeKind for DRM_LICENSE_ACQ_DATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRMCALLBACK = Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: windows_core::HRESULT, param2: *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT>;
