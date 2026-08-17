#[inline]
pub unsafe fn CoGetClassObjectFromURL<P0, P1, P2>(rclassid: *const windows_core::GUID, szcode: P0, dwfileversionms: u32, dwfileversionls: u32, sztype: P1, pbindctx: P2, dwclscontext: super::CLSCTX, pvreserved: Option<*const core::ffi::c_void>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindCtx>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoGetClassObjectFromURL(rclassid : *const windows_core::GUID, szcode : windows_core::PCWSTR, dwfileversionms : u32, dwfileversionls : u32, sztype : windows_core::PCWSTR, pbindctx : * mut core::ffi::c_void, dwclscontext : super:: CLSCTX, pvreserved : *const core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    CoGetClassObjectFromURL(rclassid, szcode.param().abi(), dwfileversionms, dwfileversionls, sztype.param().abi(), pbindctx.param().abi(), dwclscontext, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())), riid, ppv).ok()
}
#[inline]
pub unsafe fn CoInternetCombineIUri<P0, P1>(pbaseuri: P0, prelativeuri: P1, dwcombineflags: u32, ppcombineduri: *mut Option<super::IUri>, dwreserved: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IUri>,
    P1: windows_core::Param<super::IUri>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCombineIUri(pbaseuri : * mut core::ffi::c_void, prelativeuri : * mut core::ffi::c_void, dwcombineflags : u32, ppcombineduri : *mut * mut core::ffi::c_void, dwreserved : usize) -> windows_core::HRESULT);
    CoInternetCombineIUri(pbaseuri.param().abi(), prelativeuri.param().abi(), dwcombineflags, core::mem::transmute(ppcombineduri), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetCombineUrl<P0, P1>(pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pszresult: &mut [u16], pcchresult: Option<*mut u32>, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCombineUrl(pwzbaseurl : windows_core::PCWSTR, pwzrelativeurl : windows_core::PCWSTR, dwcombineflags : u32, pszresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetCombineUrl(pwzbaseurl.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, core::mem::transmute(pszresult.as_ptr()), pszresult.len().try_into().unwrap(), core::mem::transmute(pcchresult.unwrap_or(std::ptr::null_mut())), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetCombineUrlEx<P0, P1>(pbaseuri: P0, pwzrelativeurl: P1, dwcombineflags: u32, ppcombineduri: *mut Option<super::IUri>, dwreserved: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IUri>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCombineUrlEx(pbaseuri : * mut core::ffi::c_void, pwzrelativeurl : windows_core::PCWSTR, dwcombineflags : u32, ppcombineduri : *mut * mut core::ffi::c_void, dwreserved : usize) -> windows_core::HRESULT);
    CoInternetCombineUrlEx(pbaseuri.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, core::mem::transmute(ppcombineduri), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetCompareUrl<P0, P1>(pwzurl1: P0, pwzurl2: P1, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCompareUrl(pwzurl1 : windows_core::PCWSTR, pwzurl2 : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    CoInternetCompareUrl(pwzurl1.param().abi(), pwzurl2.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn CoInternetCreateSecurityManager<P0>(psp: P0, ppsm: *mut Option<IInternetSecurityManager>, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IServiceProvider>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCreateSecurityManager(psp : * mut core::ffi::c_void, ppsm : *mut * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetCreateSecurityManager(psp.param().abi(), core::mem::transmute(ppsm), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetCreateZoneManager<P0>(psp: P0, ppzm: *mut Option<IInternetZoneManager>, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IServiceProvider>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetCreateZoneManager(psp : * mut core::ffi::c_void, ppzm : *mut * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetCreateZoneManager(psp.param().abi(), core::mem::transmute(ppzm), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetGetProtocolFlags<P0>(pwzurl: P0, pdwflags: *mut u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetGetProtocolFlags(pwzurl : windows_core::PCWSTR, pdwflags : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetGetProtocolFlags(pwzurl.param().abi(), pdwflags, dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetGetSecurityUrl<P0>(pwszurl: P0, ppwszsecurl: *mut windows_core::PWSTR, psuaction: PSUACTION, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrl(pwszurl : windows_core::PCWSTR, ppwszsecurl : *mut windows_core::PWSTR, psuaction : PSUACTION, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetGetSecurityUrl(pwszurl.param().abi(), ppwszsecurl, psuaction, dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetGetSecurityUrlEx<P0>(puri: P0, ppsecuri: *mut Option<super::IUri>, psuaction: PSUACTION, dwreserved: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IUri>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetGetSecurityUrlEx(puri : * mut core::ffi::c_void, ppsecuri : *mut * mut core::ffi::c_void, psuaction : PSUACTION, dwreserved : usize) -> windows_core::HRESULT);
    CoInternetGetSecurityUrlEx(puri.param().abi(), core::mem::transmute(ppsecuri), psuaction, dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetGetSession(dwsessionmode: u32, ppiinternetsession: *mut Option<IInternetSession>, dwreserved: u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn CoInternetGetSession(dwsessionmode : u32, ppiinternetsession : *mut * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetGetSession(dwsessionmode, core::mem::transmute(ppiinternetsession), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabled(featureentry: INTERNETFEATURELIST, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32) -> windows_core::HRESULT);
    CoInternetIsFeatureEnabled(featureentry, dwflags).ok()
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForIUri<P0, P1>(featureentry: INTERNETFEATURELIST, dwflags: u32, piuri: P0, psecmgr: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IUri>,
    P1: windows_core::Param<IInternetSecurityManagerEx2>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForIUri(featureentry : INTERNETFEATURELIST, dwflags : u32, piuri : * mut core::ffi::c_void, psecmgr : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoInternetIsFeatureEnabledForIUri(featureentry, dwflags, piuri.param().abi(), psecmgr.param().abi()).ok()
}
#[inline]
pub unsafe fn CoInternetIsFeatureEnabledForUrl<P0, P1>(featureentry: INTERNETFEATURELIST, dwflags: u32, szurl: P0, psecmgr: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IInternetSecurityManager>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetIsFeatureEnabledForUrl(featureentry : INTERNETFEATURELIST, dwflags : u32, szurl : windows_core::PCWSTR, psecmgr : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoInternetIsFeatureEnabledForUrl(featureentry, dwflags, szurl.param().abi(), psecmgr.param().abi()).ok()
}
#[inline]
pub unsafe fn CoInternetIsFeatureZoneElevationEnabled<P0, P1, P2>(szfromurl: P0, sztourl: P1, psecmgr: P2, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IInternetSecurityManager>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetIsFeatureZoneElevationEnabled(szfromurl : windows_core::PCWSTR, sztourl : windows_core::PCWSTR, psecmgr : * mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    CoInternetIsFeatureZoneElevationEnabled(szfromurl.param().abi(), sztourl.param().abi(), psecmgr.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn CoInternetParseIUri<P0>(piuri: P0, parseaction: PARSEACTION, dwflags: u32, pwzresult: &mut [u16], pcchresult: *mut u32, dwreserved: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IUri>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetParseIUri(piuri : * mut core::ffi::c_void, parseaction : PARSEACTION, dwflags : u32, pwzresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : usize) -> windows_core::HRESULT);
    CoInternetParseIUri(piuri.param().abi(), parseaction, dwflags, core::mem::transmute(pwzresult.as_ptr()), pwzresult.len().try_into().unwrap(), pcchresult, dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetParseUrl<P0>(pwzurl: P0, parseaction: PARSEACTION, dwflags: u32, pszresult: &mut [u16], pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetParseUrl(pwzurl : windows_core::PCWSTR, parseaction : PARSEACTION, dwflags : u32, pszresult : windows_core::PWSTR, cchresult : u32, pcchresult : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetParseUrl(pwzurl.param().abi(), parseaction, dwflags, core::mem::transmute(pszresult.as_ptr()), pszresult.len().try_into().unwrap(), pcchresult, dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetQueryInfo<P0>(pwzurl: P0, queryoptions: QUERYOPTION, dwqueryflags: u32, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuffer: Option<*mut u32>, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetQueryInfo(pwzurl : windows_core::PCWSTR, queryoptions : QUERYOPTION, dwqueryflags : u32, pvbuffer : *mut core::ffi::c_void, cbbuffer : u32, pcbbuffer : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    CoInternetQueryInfo(pwzurl.param().abi(), queryoptions, dwqueryflags, pvbuffer, cbbuffer, core::mem::transmute(pcbbuffer.unwrap_or(std::ptr::null_mut())), dwreserved).ok()
}
#[inline]
pub unsafe fn CoInternetSetFeatureEnabled<P0>(featureentry: INTERNETFEATURELIST, dwflags: u32, fenable: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("urlmon.dll" "system" fn CoInternetSetFeatureEnabled(featureentry : INTERNETFEATURELIST, dwflags : u32, fenable : super::super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    CoInternetSetFeatureEnabled(featureentry, dwflags, fenable.param().abi()).ok()
}
#[inline]
pub unsafe fn CompareSecurityIds(pbsecurityid1: &[u8], pbsecurityid2: &[u8], dwreserved: u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn CompareSecurityIds(pbsecurityid1 : *const u8, dwlen1 : u32, pbsecurityid2 : *const u8, dwlen2 : u32, dwreserved : u32) -> windows_core::HRESULT);
    CompareSecurityIds(core::mem::transmute(pbsecurityid1.as_ptr()), pbsecurityid1.len().try_into().unwrap(), core::mem::transmute(pbsecurityid2.as_ptr()), pbsecurityid2.len().try_into().unwrap(), dwreserved).ok()
}
#[inline]
pub unsafe fn CompatFlagsFromClsid(pclsid: *const windows_core::GUID, pdwcompatflags: *mut u32, pdwmiscstatusflags: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn CompatFlagsFromClsid(pclsid : *const windows_core::GUID, pdwcompatflags : *mut u32, pdwmiscstatusflags : *mut u32) -> windows_core::HRESULT);
    CompatFlagsFromClsid(pclsid, pdwcompatflags, pdwmiscstatusflags).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyBindInfo(pcbisrc: *const super::BINDINFO, pbidest: *mut super::BINDINFO) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn CopyBindInfo(pcbisrc : *const super:: BINDINFO, pbidest : *mut super:: BINDINFO) -> windows_core::HRESULT);
    CopyBindInfo(pcbisrc, pbidest).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn CopyStgMedium(pcstgmedsrc: *const super::STGMEDIUM) -> windows_core::Result<super::STGMEDIUM> {
    windows_targets::link!("urlmon.dll" "system" fn CopyStgMedium(pcstgmedsrc : *const super:: STGMEDIUM, pstgmeddest : *mut super:: STGMEDIUM) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CopyStgMedium(pcstgmedsrc, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CreateAsyncBindCtx<P0, P1>(reserved: u32, pbscb: P0, pefetc: P1) -> windows_core::Result<super::IBindCtx>
where
    P0: windows_core::Param<super::IBindStatusCallback>,
    P1: windows_core::Param<super::IEnumFORMATETC>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateAsyncBindCtx(reserved : u32, pbscb : * mut core::ffi::c_void, pefetc : * mut core::ffi::c_void, ppbc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateAsyncBindCtx(reserved, pbscb.param().abi(), pefetc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateAsyncBindCtxEx<P0, P1, P2>(pbc: P0, dwoptions: u32, pbscb: P1, penum: P2, ppbc: *mut Option<super::IBindCtx>, reserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<super::IBindStatusCallback>,
    P2: windows_core::Param<super::IEnumFORMATETC>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateAsyncBindCtxEx(pbc : * mut core::ffi::c_void, dwoptions : u32, pbscb : * mut core::ffi::c_void, penum : * mut core::ffi::c_void, ppbc : *mut * mut core::ffi::c_void, reserved : u32) -> windows_core::HRESULT);
    CreateAsyncBindCtxEx(pbc.param().abi(), dwoptions, pbscb.param().abi(), penum.param().abi(), core::mem::transmute(ppbc), reserved).ok()
}
#[inline]
pub unsafe fn CreateFormatEnumerator(rgfmtetc: &[super::FORMATETC]) -> windows_core::Result<super::IEnumFORMATETC> {
    windows_targets::link!("urlmon.dll" "system" fn CreateFormatEnumerator(cfmtetc : u32, rgfmtetc : *const super:: FORMATETC, ppenumfmtetc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateFormatEnumerator(rgfmtetc.len().try_into().unwrap(), core::mem::transmute(rgfmtetc.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateURLMoniker<P0, P1>(pmkctx: P0, szurl: P1) -> windows_core::Result<super::IMoniker>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateURLMoniker(pmkctx : * mut core::ffi::c_void, szurl : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateURLMoniker(pmkctx.param().abi(), szurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateURLMonikerEx<P0, P1>(pmkctx: P0, szurl: P1, ppmk: *mut Option<super::IMoniker>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateURLMonikerEx(pmkctx : * mut core::ffi::c_void, szurl : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    CreateURLMonikerEx(pmkctx.param().abi(), szurl.param().abi(), core::mem::transmute(ppmk), dwflags).ok()
}
#[inline]
pub unsafe fn CreateURLMonikerEx2<P0, P1>(pmkctx: P0, puri: P1, ppmk: *mut Option<super::IMoniker>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<super::IUri>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateURLMonikerEx2(pmkctx : * mut core::ffi::c_void, puri : * mut core::ffi::c_void, ppmk : *mut * mut core::ffi::c_void, dwflags : u32) -> windows_core::HRESULT);
    CreateURLMonikerEx2(pmkctx.param().abi(), puri.param().abi(), core::mem::transmute(ppmk), dwflags).ok()
}
#[inline]
pub unsafe fn FaultInIEFeature<P0>(hwnd: P0, pclassspec: *const super::uCLSSPEC, pquery: Option<*mut super::QUERYCONTEXT>, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HWND>,
{
    windows_targets::link!("urlmon.dll" "system" fn FaultInIEFeature(hwnd : super::super::super::Foundation:: HWND, pclassspec : *const super:: uCLSSPEC, pquery : *mut super:: QUERYCONTEXT, dwflags : u32) -> windows_core::HRESULT);
    FaultInIEFeature(hwnd.param().abi(), pclassspec, core::mem::transmute(pquery.unwrap_or(std::ptr::null_mut())), dwflags).ok()
}
#[inline]
pub unsafe fn FindMediaType<P0>(rgsztypes: P0) -> windows_core::Result<u16>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn FindMediaType(rgsztypes : windows_core::PCSTR, rgcftypes : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    FindMediaType(rgsztypes.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn FindMediaTypeClass<P0, P1>(pbc: P0, sztype: P1, pclsid: *mut windows_core::GUID, reserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn FindMediaTypeClass(pbc : * mut core::ffi::c_void, sztype : windows_core::PCSTR, pclsid : *mut windows_core::GUID, reserved : u32) -> windows_core::HRESULT);
    FindMediaTypeClass(pbc.param().abi(), sztype.param().abi(), pclsid, reserved).ok()
}
#[inline]
pub unsafe fn FindMimeFromData<P0, P1, P2>(pbc: P0, pwzurl: P1, pbuffer: Option<*const core::ffi::c_void>, cbsize: u32, pwzmimeproposed: P2, dwmimeflags: u32, ppwzmimeout: *mut windows_core::PWSTR, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn FindMimeFromData(pbc : * mut core::ffi::c_void, pwzurl : windows_core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, pwzmimeproposed : windows_core::PCWSTR, dwmimeflags : u32, ppwzmimeout : *mut windows_core::PWSTR, dwreserved : u32) -> windows_core::HRESULT);
    FindMimeFromData(pbc.param().abi(), pwzurl.param().abi(), core::mem::transmute(pbuffer.unwrap_or(std::ptr::null())), cbsize, pwzmimeproposed.param().abi(), dwmimeflags, ppwzmimeout, dwreserved).ok()
}
#[inline]
pub unsafe fn GetClassFileOrMime<P0, P1, P2>(pbc: P0, szfilename: P1, pbuffer: Option<*const core::ffi::c_void>, cbsize: u32, szmime: P2, dwreserved: u32) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn GetClassFileOrMime(pbc : * mut core::ffi::c_void, szfilename : windows_core::PCWSTR, pbuffer : *const core::ffi::c_void, cbsize : u32, szmime : windows_core::PCWSTR, dwreserved : u32, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetClassFileOrMime(pbc.param().abi(), szfilename.param().abi(), core::mem::transmute(pbuffer.unwrap_or(std::ptr::null())), cbsize, szmime.param().abi(), dwreserved, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetClassURL<P0>(szurl: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn GetClassURL(szurl : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetClassURL(szurl.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetComponentIDFromCLSSPEC(pclassspec: *const super::uCLSSPEC) -> windows_core::Result<windows_core::PSTR> {
    windows_targets::link!("urlmon.dll" "system" fn GetComponentIDFromCLSSPEC(pclassspec : *const super:: uCLSSPEC, ppszcomponentid : *mut windows_core::PSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetComponentIDFromCLSSPEC(pclassspec, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetSoftwareUpdateInfo<P0>(szdistunit: P0, psdi: *mut SOFTDISTINFO) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn GetSoftwareUpdateInfo(szdistunit : windows_core::PCWSTR, psdi : *mut SOFTDISTINFO) -> windows_core::HRESULT);
    GetSoftwareUpdateInfo(szdistunit.param().abi(), psdi).ok()
}
#[inline]
pub unsafe fn HlinkGoBack<P0>(punk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkGoBack(punk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    HlinkGoBack(punk.param().abi()).ok()
}
#[inline]
pub unsafe fn HlinkGoForward<P0>(punk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkGoForward(punk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    HlinkGoForward(punk.param().abi()).ok()
}
#[inline]
pub unsafe fn HlinkNavigateMoniker<P0, P1>(punk: P0, pmktarget: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::IMoniker>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkNavigateMoniker(punk : * mut core::ffi::c_void, pmktarget : * mut core::ffi::c_void) -> windows_core::HRESULT);
    HlinkNavigateMoniker(punk.param().abi(), pmktarget.param().abi()).ok()
}
#[inline]
pub unsafe fn HlinkNavigateString<P0, P1>(punk: P0, sztarget: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkNavigateString(punk : * mut core::ffi::c_void, sztarget : windows_core::PCWSTR) -> windows_core::HRESULT);
    HlinkNavigateString(punk.param().abi(), sztarget.param().abi()).ok()
}
#[inline]
pub unsafe fn HlinkSimpleNavigateToMoniker<P0, P1, P2, P3, P4, P5>(pmktarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::IBindCtx>,
    P5: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToMoniker(pmktarget : * mut core::ffi::c_void, szlocation : windows_core::PCWSTR, sztargetframename : windows_core::PCWSTR, punk : * mut core::ffi::c_void, pbc : * mut core::ffi::c_void, param5 : * mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_core::HRESULT);
    HlinkSimpleNavigateToMoniker(pmktarget.param().abi(), szlocation.param().abi(), sztargetframename.param().abi(), punk.param().abi(), pbc.param().abi(), param5.param().abi(), grfhlnf, dwreserved).ok()
}
#[inline]
pub unsafe fn HlinkSimpleNavigateToString<P0, P1, P2, P3, P4, P5>(sztarget: P0, szlocation: P1, sztargetframename: P2, punk: P3, pbc: P4, param5: P5, grfhlnf: u32, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::IUnknown>,
    P4: windows_core::Param<super::IBindCtx>,
    P5: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn HlinkSimpleNavigateToString(sztarget : windows_core::PCWSTR, szlocation : windows_core::PCWSTR, sztargetframename : windows_core::PCWSTR, punk : * mut core::ffi::c_void, pbc : * mut core::ffi::c_void, param5 : * mut core::ffi::c_void, grfhlnf : u32, dwreserved : u32) -> windows_core::HRESULT);
    HlinkSimpleNavigateToString(sztarget.param().abi(), szlocation.param().abi(), sztargetframename.param().abi(), punk.param().abi(), pbc.param().abi(), param5.param().abi(), grfhlnf, dwreserved).ok()
}
#[inline]
pub unsafe fn IEGetUserPrivateNamespaceName() -> windows_core::PWSTR {
    windows_targets::link!("urlmon.dll" "system" fn IEGetUserPrivateNamespaceName() -> windows_core::PWSTR);
    IEGetUserPrivateNamespaceName()
}
#[inline]
pub unsafe fn IEInstallScope() -> windows_core::Result<u32> {
    windows_targets::link!("urlmon.dll" "system" fn IEInstallScope(pdwscope : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IEInstallScope(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsAsyncMoniker<P0>(pmk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IMoniker>,
{
    windows_targets::link!("urlmon.dll" "system" fn IsAsyncMoniker(pmk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    IsAsyncMoniker(pmk.param().abi()).ok()
}
#[inline]
pub unsafe fn IsLoggingEnabledA<P0>(pszurl: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn IsLoggingEnabledA(pszurl : windows_core::PCSTR) -> super::super::super::Foundation:: BOOL);
    IsLoggingEnabledA(pszurl.param().abi())
}
#[inline]
pub unsafe fn IsLoggingEnabledW<P0>(pwszurl: P0) -> super::super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn IsLoggingEnabledW(pwszurl : windows_core::PCWSTR) -> super::super::super::Foundation:: BOOL);
    IsLoggingEnabledW(pwszurl.param().abi())
}
#[inline]
pub unsafe fn IsValidURL<P0, P1>(pbc: P0, szurl: P1, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn IsValidURL(pbc : * mut core::ffi::c_void, szurl : windows_core::PCWSTR, dwreserved : u32) -> windows_core::HRESULT);
    IsValidURL(pbc.param().abi(), szurl.param().abi(), dwreserved).ok()
}
#[inline]
pub unsafe fn MkParseDisplayNameEx<P0, P1>(pbc: P0, szdisplayname: P1, pcheaten: *mut u32, ppmk: *mut Option<super::IMoniker>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn MkParseDisplayNameEx(pbc : * mut core::ffi::c_void, szdisplayname : windows_core::PCWSTR, pcheaten : *mut u32, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    MkParseDisplayNameEx(pbc.param().abi(), szdisplayname.param().abi(), pcheaten, core::mem::transmute(ppmk)).ok()
}
#[inline]
pub unsafe fn ObtainUserAgentString(dwoption: u32, pszuaout: windows_core::PSTR, cbsize: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn ObtainUserAgentString(dwoption : u32, pszuaout : windows_core::PSTR, cbsize : *mut u32) -> windows_core::HRESULT);
    ObtainUserAgentString(dwoption, core::mem::transmute(pszuaout), cbsize).ok()
}
#[inline]
pub unsafe fn RegisterBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1, ppbscbprev: *mut Option<super::IBindStatusCallback>, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn RegisterBindStatusCallback(pbc : * mut core::ffi::c_void, pbscb : * mut core::ffi::c_void, ppbscbprev : *mut * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    RegisterBindStatusCallback(pbc.param().abi(), pbscb.param().abi(), core::mem::transmute(ppbscbprev), dwreserved).ok()
}
#[inline]
pub unsafe fn RegisterFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1, reserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<super::IEnumFORMATETC>,
{
    windows_targets::link!("urlmon.dll" "system" fn RegisterFormatEnumerator(pbc : * mut core::ffi::c_void, pefetc : * mut core::ffi::c_void, reserved : u32) -> windows_core::HRESULT);
    RegisterFormatEnumerator(pbc.param().abi(), pefetc.param().abi(), reserved).ok()
}
#[inline]
pub unsafe fn RegisterMediaTypeClass<P0>(pbc: P0, ctypes: u32, rgsztypes: *const windows_core::PCSTR, rgclsid: *const windows_core::GUID, reserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
{
    windows_targets::link!("urlmon.dll" "system" fn RegisterMediaTypeClass(pbc : * mut core::ffi::c_void, ctypes : u32, rgsztypes : *const windows_core::PCSTR, rgclsid : *const windows_core::GUID, reserved : u32) -> windows_core::HRESULT);
    RegisterMediaTypeClass(pbc.param().abi(), ctypes, rgsztypes, rgclsid, reserved).ok()
}
#[inline]
pub unsafe fn RegisterMediaTypes(ctypes: u32, rgsztypes: *const windows_core::PCSTR, rgcftypes: *mut u16) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn RegisterMediaTypes(ctypes : u32, rgsztypes : *const windows_core::PCSTR, rgcftypes : *mut u16) -> windows_core::HRESULT);
    RegisterMediaTypes(ctypes, rgsztypes, rgcftypes).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseBindInfo(pbindinfo: *mut super::BINDINFO) {
    windows_targets::link!("urlmon.dll" "system" fn ReleaseBindInfo(pbindinfo : *mut super:: BINDINFO));
    ReleaseBindInfo(pbindinfo)
}
#[inline]
pub unsafe fn RevokeBindStatusCallback<P0, P1>(pbc: P0, pbscb: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn RevokeBindStatusCallback(pbc : * mut core::ffi::c_void, pbscb : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RevokeBindStatusCallback(pbc.param().abi(), pbscb.param().abi()).ok()
}
#[inline]
pub unsafe fn RevokeFormatEnumerator<P0, P1>(pbc: P0, pefetc: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::IBindCtx>,
    P1: windows_core::Param<super::IEnumFORMATETC>,
{
    windows_targets::link!("urlmon.dll" "system" fn RevokeFormatEnumerator(pbc : * mut core::ffi::c_void, pefetc : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RevokeFormatEnumerator(pbc.param().abi(), pefetc.param().abi()).ok()
}
#[inline]
pub unsafe fn SetAccessForIEAppContainer<P0>(hobject: P0, ieobjecttype: IEObjectType, dwaccessmask: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::HANDLE>,
{
    windows_targets::link!("urlmon.dll" "system" fn SetAccessForIEAppContainer(hobject : super::super::super::Foundation:: HANDLE, ieobjecttype : IEObjectType, dwaccessmask : u32) -> windows_core::HRESULT);
    SetAccessForIEAppContainer(hobject.param().abi(), ieobjecttype, dwaccessmask).ok()
}
#[inline]
pub unsafe fn SetSoftwareUpdateAdvertisementState<P0>(szdistunit: P0, dwadstate: u32, dwadvertisedversionms: u32, dwadvertisedversionls: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn SetSoftwareUpdateAdvertisementState(szdistunit : windows_core::PCWSTR, dwadstate : u32, dwadvertisedversionms : u32, dwadvertisedversionls : u32) -> windows_core::HRESULT);
    SetSoftwareUpdateAdvertisementState(szdistunit.param().abi(), dwadstate, dwadvertisedversionms, dwadvertisedversionls).ok()
}
#[inline]
pub unsafe fn URLDownloadToCacheFileA<P0, P1, P2>(param0: P0, param1: P1, param2: &mut [u8], param4: u32, param5: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLDownloadToCacheFileA(param0 : * mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : windows_core::PSTR, cchfilename : u32, param4 : u32, param5 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLDownloadToCacheFileA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.as_ptr()), param2.len().try_into().unwrap(), param4, param5.param().abi()).ok()
}
#[inline]
pub unsafe fn URLDownloadToCacheFileW<P0, P1, P2>(param0: P0, param1: P1, param2: &mut [u16], param4: u32, param5: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLDownloadToCacheFileW(param0 : * mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : windows_core::PWSTR, cchfilename : u32, param4 : u32, param5 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLDownloadToCacheFileW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2.as_ptr()), param2.len().try_into().unwrap(), param4, param5.param().abi()).ok()
}
#[inline]
pub unsafe fn URLDownloadToFileA<P0, P1, P2, P3>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLDownloadToFileA(param0 : * mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : windows_core::PCSTR, param3 : u32, param4 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLDownloadToFileA(param0.param().abi(), param1.param().abi(), param2.param().abi(), param3, param4.param().abi()).ok()
}
#[inline]
pub unsafe fn URLDownloadToFileW<P0, P1, P2, P3>(param0: P0, param1: P1, param2: P2, param3: u32, param4: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLDownloadToFileW(param0 : * mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : windows_core::PCWSTR, param3 : u32, param4 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLDownloadToFileW(param0.param().abi(), param1.param().abi(), param2.param().abi(), param3, param4.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenBlockingStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: *mut Option<super::IStream>, param3: u32, param4: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenBlockingStreamA(param0 : * mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : *mut * mut core::ffi::c_void, param3 : u32, param4 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenBlockingStreamA(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2), param3, param4.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenBlockingStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: *mut Option<super::IStream>, param3: u32, param4: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenBlockingStreamW(param0 : * mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : *mut * mut core::ffi::c_void, param3 : u32, param4 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenBlockingStreamW(param0.param().abi(), param1.param().abi(), core::mem::transmute(param2), param3, param4.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenPullStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenPullStreamA(param0 : * mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : u32, param3 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenPullStreamA(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenPullStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenPullStreamW(param0 : * mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : u32, param3 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenPullStreamW(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenStreamA<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenStreamA(param0 : * mut core::ffi::c_void, param1 : windows_core::PCSTR, param2 : u32, param3 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenStreamA(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()).ok()
}
#[inline]
pub unsafe fn URLOpenStreamW<P0, P1, P2>(param0: P0, param1: P1, param2: u32, param3: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::IBindStatusCallback>,
{
    windows_targets::link!("urlmon.dll" "system" fn URLOpenStreamW(param0 : * mut core::ffi::c_void, param1 : windows_core::PCWSTR, param2 : u32, param3 : * mut core::ffi::c_void) -> windows_core::HRESULT);
    URLOpenStreamW(param0.param().abi(), param1.param().abi(), param2, param3.param().abi()).ok()
}
#[inline]
pub unsafe fn UrlMkGetSessionOption(dwoption: u32, pbuffer: Option<*mut core::ffi::c_void>, dwbufferlength: u32, pdwbufferlengthout: *mut u32, dwreserved: u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn UrlMkGetSessionOption(dwoption : u32, pbuffer : *mut core::ffi::c_void, dwbufferlength : u32, pdwbufferlengthout : *mut u32, dwreserved : u32) -> windows_core::HRESULT);
    UrlMkGetSessionOption(dwoption, core::mem::transmute(pbuffer.unwrap_or(std::ptr::null_mut())), dwbufferlength, pdwbufferlengthout, dwreserved).ok()
}
#[inline]
pub unsafe fn UrlMkSetSessionOption(dwoption: u32, pbuffer: Option<*const core::ffi::c_void>, dwbufferlength: u32, dwreserved: u32) -> windows_core::Result<()> {
    windows_targets::link!("urlmon.dll" "system" fn UrlMkSetSessionOption(dwoption : u32, pbuffer : *const core::ffi::c_void, dwbufferlength : u32, dwreserved : u32) -> windows_core::HRESULT);
    UrlMkSetSessionOption(dwoption, core::mem::transmute(pbuffer.unwrap_or(std::ptr::null())), dwbufferlength, dwreserved).ok()
}
#[inline]
pub unsafe fn WriteHitLogging(lplogginginfo: *const HIT_LOGGING_INFO) -> super::super::super::Foundation::BOOL {
    windows_targets::link!("urlmon.dll" "system" fn WriteHitLogging(lplogginginfo : *const HIT_LOGGING_INFO) -> super::super::super::Foundation:: BOOL);
    WriteHitLogging(lplogginginfo)
}
windows_core::imp::define_interface!(IBindCallbackRedirect, IBindCallbackRedirect_Vtbl, 0x11c81bc2_121e_4ed5_b9c4_b430bd54f2c0);
impl core::ops::Deref for IBindCallbackRedirect {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindCallbackRedirect, windows_core::IUnknown);
impl IBindCallbackRedirect {
    pub unsafe fn Redirect<P0>(&self, lpcurl: P0) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Redirect)(windows_core::Interface::as_raw(self), lpcurl.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBindCallbackRedirect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Redirect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindHttpSecurity, IBindHttpSecurity_Vtbl, 0xa9eda967_f50e_4a33_b358_206f6ef3086d);
impl core::ops::Deref for IBindHttpSecurity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindHttpSecurity, windows_core::IUnknown);
impl IBindHttpSecurity {
    pub unsafe fn GetIgnoreCertMask(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIgnoreCertMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IBindHttpSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIgnoreCertMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindProtocol, IBindProtocol_Vtbl, 0x79eac9cd_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IBindProtocol {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindProtocol, windows_core::IUnknown);
impl IBindProtocol {
    pub unsafe fn CreateBinding<P0, P1>(&self, szurl: P0, pbc: P1) -> windows_core::Result<super::IBinding>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::IBindCtx>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateBinding)(windows_core::Interface::as_raw(self), szurl.param().abi(), pbc.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IBindProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBinding: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICatalogFileInfo, ICatalogFileInfo_Vtbl, 0x711c7600_6b48_11d1_b403_00aa00b92af1);
impl core::ops::Deref for ICatalogFileInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICatalogFileInfo, windows_core::IUnknown);
impl ICatalogFileInfo {
    pub unsafe fn GetCatalogFile(&self) -> windows_core::Result<windows_core::PSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCatalogFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetJavaTrust(&self, ppjavatrust: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetJavaTrust)(windows_core::Interface::as_raw(self), ppjavatrust).ok()
    }
}
#[repr(C)]
pub struct ICatalogFileInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCatalogFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetJavaTrust: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICodeInstall, ICodeInstall_Vtbl, 0x79eac9d1_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for ICodeInstall {
    type Target = IWindowForBindingUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICodeInstall, windows_core::IUnknown, IWindowForBindingUI);
impl ICodeInstall {
    pub unsafe fn OnCodeInstallProblem<P0, P1>(&self, ulstatuscode: u32, szdestination: P0, szsource: P1, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnCodeInstallProblem)(windows_core::Interface::as_raw(self), ulstatuscode, szdestination.param().abi(), szsource.param().abi(), dwreserved).ok()
    }
}
#[repr(C)]
pub struct ICodeInstall_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnCodeInstallProblem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataFilter, IDataFilter_Vtbl, 0x69d14c80_c18e_11d0_a9ce_006097942311);
impl core::ops::Deref for IDataFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataFilter, windows_core::IUnknown);
impl IDataFilter {
    pub unsafe fn DoEncode(&self, dwflags: u32, pbinbuffer: &[u8], pboutbuffer: &mut [u8], linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoEncode)(windows_core::Interface::as_raw(self), dwflags, pbinbuffer.len().try_into().unwrap(), core::mem::transmute(pbinbuffer.as_ptr()), pboutbuffer.len().try_into().unwrap(), core::mem::transmute(pboutbuffer.as_ptr()), linbytesavailable, plinbytesread, ploutbyteswritten, dwreserved).ok()
    }
    pub unsafe fn DoDecode(&self, dwflags: u32, pbinbuffer: &[u8], pboutbuffer: &mut [u8], linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DoDecode)(windows_core::Interface::as_raw(self), dwflags, pbinbuffer.len().try_into().unwrap(), core::mem::transmute(pbinbuffer.as_ptr()), pboutbuffer.len().try_into().unwrap(), core::mem::transmute(pboutbuffer.as_ptr()), linbytesavailable, plinbytesread, ploutbyteswritten, dwreserved).ok()
    }
    pub unsafe fn SetEncodingLevel(&self, dwenclevel: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEncodingLevel)(windows_core::Interface::as_raw(self), dwenclevel).ok()
    }
}
#[repr(C)]
pub struct IDataFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DoEncode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const u8, i32, *mut u8, i32, *mut i32, *mut i32, u32) -> windows_core::HRESULT,
    pub DoDecode: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *const u8, i32, *mut u8, i32, *mut i32, *mut i32, u32) -> windows_core::HRESULT,
    pub SetEncodingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEncodingFilterFactory, IEncodingFilterFactory_Vtbl, 0x70bdde00_c18e_11d0_a9ce_006097942311);
impl core::ops::Deref for IEncodingFilterFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEncodingFilterFactory, windows_core::IUnknown);
impl IEncodingFilterFactory {
    pub unsafe fn FindBestFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1, info: DATAINFO) -> windows_core::Result<IDataFilter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindBestFilter)(windows_core::Interface::as_raw(self), pwzcodein.param().abi(), pwzcodeout.param().abi(), core::mem::transmute(info), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDefaultFilter<P0, P1>(&self, pwzcodein: P0, pwzcodeout: P1) -> windows_core::Result<IDataFilter>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultFilter)(windows_core::Interface::as_raw(self), pwzcodein.param().abi(), pwzcodeout.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEncodingFilterFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindBestFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, DATAINFO, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetBindHandle, IGetBindHandle_Vtbl, 0xaf0ff408_129d_4b20_91f0_02bd23d88352);
impl core::ops::Deref for IGetBindHandle {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetBindHandle, windows_core::IUnknown);
impl IGetBindHandle {
    pub unsafe fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBindHandle)(windows_core::Interface::as_raw(self), enumrequestedhandle, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGetBindHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBindHandle: unsafe extern "system" fn(*mut core::ffi::c_void, BINDHANDLETYPES, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNegotiate, IHttpNegotiate_Vtbl, 0x79eac9d2_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IHttpNegotiate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpNegotiate, windows_core::IUnknown);
impl IHttpNegotiate {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginningTransaction)(windows_core::Interface::as_raw(self), szurl.param().abi(), szheaders.param().abi(), dwreserved, &mut result__).map(|| result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnResponse)(windows_core::Interface::as_raw(self), dwresponsecode, szresponseheaders.param().abi(), szrequestheaders.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IHttpNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginningTransaction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub OnResponse: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNegotiate2, IHttpNegotiate2_Vtbl, 0x4f9f9fcb_e0f4_48eb_b7ab_fa2ea9365cb4);
impl core::ops::Deref for IHttpNegotiate2 {
    type Target = IHttpNegotiate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpNegotiate2, windows_core::IUnknown, IHttpNegotiate);
impl IHttpNegotiate2 {
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRootSecurityId)(windows_core::Interface::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IHttpNegotiate2_Vtbl {
    pub base__: IHttpNegotiate_Vtbl,
    pub GetRootSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpNegotiate3, IHttpNegotiate3_Vtbl, 0x57b6c80a_34c2_4602_bc26_66a02fc57153);
impl core::ops::Deref for IHttpNegotiate3 {
    type Target = IHttpNegotiate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpNegotiate3, windows_core::IUnknown, IHttpNegotiate, IHttpNegotiate2);
impl IHttpNegotiate3 {
    pub unsafe fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSerializedClientCertContext)(windows_core::Interface::as_raw(self), ppbcert, pcbcert).ok()
    }
}
#[repr(C)]
pub struct IHttpNegotiate3_Vtbl {
    pub base__: IHttpNegotiate2_Vtbl,
    pub GetSerializedClientCertContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHttpSecurity, IHttpSecurity_Vtbl, 0x79eac9d7_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IHttpSecurity {
    type Target = IWindowForBindingUI;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHttpSecurity, windows_core::IUnknown, IWindowForBindingUI);
impl IHttpSecurity {
    pub unsafe fn OnSecurityProblem(&self, dwproblem: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnSecurityProblem)(windows_core::Interface::as_raw(self), dwproblem).ok()
    }
}
#[repr(C)]
pub struct IHttpSecurity_Vtbl {
    pub base__: IWindowForBindingUI_Vtbl,
    pub OnSecurityProblem: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternet, IInternet_Vtbl, 0x79eac9e0_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternet {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternet, windows_core::IUnknown);
impl IInternet {}
#[repr(C)]
pub struct IInternet_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IInternetBindInfo, IInternetBindInfo_Vtbl, 0x79eac9e1_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetBindInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetBindInfo, windows_core::IUnknown);
impl IInternetBindInfo {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindInfo)(windows_core::Interface::as_raw(self), grfbindf, pbindinfo).ok()
    }
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut windows_core::PWSTR, cel: u32, pcelfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindString)(windows_core::Interface::as_raw(self), ulstringtype, ppwzstr, cel, pcelfetched).ok()
    }
}
#[repr(C)]
pub struct IInternetBindInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::BINDINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    pub GetBindString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetBindInfoEx, IInternetBindInfoEx_Vtbl, 0xa3e015b7_a82c_4dcd_a150_569aeeed36ab);
impl core::ops::Deref for IInternetBindInfoEx {
    type Target = IInternetBindInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetBindInfoEx, windows_core::IUnknown, IInternetBindInfo);
impl IInternetBindInfoEx {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindInfoEx)(windows_core::Interface::as_raw(self), grfbindf, pbindinfo, grfbindf2, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetBindInfoEx_Vtbl {
    pub base__: IInternetBindInfo_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::BINDINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
windows_core::imp::define_interface!(IInternetHostSecurityManager, IInternetHostSecurityManager_Vtbl, 0x3af280b6_cb3f_11d0_891e_00c04fb6bfc4);
impl core::ops::Deref for IInternetHostSecurityManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetHostSecurityManager, windows_core::IUnknown);
impl IInternetHostSecurityManager {
    pub unsafe fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSecurityId)(windows_core::Interface::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction(&self, dwaction: u32, ppolicy: &mut [u8], pcontext: Option<&[u8]>, dwflags: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessUrlAction)(windows_core::Interface::as_raw(self), dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), core::mem::transmute(pcontext.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pcontext.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy(&self, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: &[u8], dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryCustomPolicy)(windows_core::Interface::as_raw(self), guidkey, pppolicy, pcbpolicy, core::mem::transmute(pcontext.as_ptr()), pcontext.len().try_into().unwrap(), dwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetHostSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8, u32, *const u8, u32, u32, u32) -> windows_core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetPriority, IInternetPriority_Vtbl, 0x79eac9eb_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetPriority {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetPriority, windows_core::IUnknown);
impl IInternetPriority {
    pub unsafe fn SetPriority(&self, npriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), npriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IInternetPriority_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocol, IInternetProtocol_Vtbl, 0x79eac9e4_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocol {
    type Target = IInternetProtocolRoot;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocol, windows_core::IUnknown, IInternetProtocolRoot);
impl IInternetProtocol {
    pub unsafe fn Read(&self, pv: &mut [u8], pcbread: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), core::mem::transmute(pv.as_ptr()), pv.len().try_into().unwrap(), pcbread).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, &mut result__).map(|| result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockRequest)(windows_core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockRequest)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocol_Vtbl {
    pub base__: IInternetProtocolRoot_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i64, u32, *mut u64) -> windows_core::HRESULT,
    pub LockRequest: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UnlockRequest: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocolEx, IInternetProtocolEx_Vtbl, 0xc7a98e66_1010_492c_a1c8_c809e1f75905);
impl core::ops::Deref for IInternetProtocolEx {
    type Target = IInternetProtocol;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolEx, windows_core::IUnknown, IInternetProtocolRoot, IInternetProtocol);
impl IInternetProtocolEx {
    pub unsafe fn StartEx<P0, P1, P2, P3>(&self, puri: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IUri>,
        P1: windows_core::Param<IInternetProtocolSink>,
        P2: windows_core::Param<IInternetBindInfo>,
        P3: windows_core::Param<super::super::super::Foundation::HANDLE_PTR>,
    {
        (windows_core::Interface::vtable(self).StartEx)(windows_core::Interface::as_raw(self), puri.param().abi(), poiprotsink.param().abi(), poibindinfo.param().abi(), grfpi, dwreserved.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocolEx_Vtbl {
    pub base__: IInternetProtocol_Vtbl,
    pub StartEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocolInfo, IInternetProtocolInfo_Vtbl, 0x79eac9ec_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocolInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolInfo, windows_core::IUnknown);
impl IInternetProtocolInfo {
    pub unsafe fn ParseUrl<P0>(&self, pwzurl: P0, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: windows_core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ParseUrl)(windows_core::Interface::as_raw(self), pwzurl.param().abi(), parseaction, dwparseflags, core::mem::transmute(pwzresult), cchresult, pcchresult, dwreserved).ok()
    }
    pub unsafe fn CombineUrl<P0, P1, P2>(&self, pwzbaseurl: P0, pwzrelativeurl: P1, dwcombineflags: u32, pwzresult: P2, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CombineUrl)(windows_core::Interface::as_raw(self), pwzbaseurl.param().abi(), pwzrelativeurl.param().abi(), dwcombineflags, pwzresult.param().abi(), cchresult, pcchresult, dwreserved).ok()
    }
    pub unsafe fn CompareUrl<P0, P1>(&self, pwzurl1: P0, pwzurl2: P1, dwcompareflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).CompareUrl)(windows_core::Interface::as_raw(self), pwzurl1.param().abi(), pwzurl2.param().abi(), dwcompareflags).ok()
    }
    pub unsafe fn QueryInfo<P0>(&self, pwzurl: P0, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).QueryInfo)(windows_core::Interface::as_raw(self), pwzurl.param().abi(), oueryoption, dwqueryflags, pbuffer, cbbuffer, pcbbuf, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocolInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ParseUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, PARSEACTION, u32, windows_core::PWSTR, u32, *mut u32, u32) -> windows_core::HRESULT,
    pub CombineUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::PCWSTR, u32, *mut u32, u32) -> windows_core::HRESULT,
    pub CompareUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub QueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, QUERYOPTION, u32, *mut core::ffi::c_void, u32, *mut u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocolRoot, IInternetProtocolRoot_Vtbl, 0x79eac9e3_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocolRoot {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolRoot, windows_core::IUnknown);
impl IInternetProtocolRoot {
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IInternetProtocolSink>,
        P2: windows_core::Param<IInternetBindInfo>,
        P3: windows_core::Param<super::super::super::Foundation::HANDLE_PTR>,
    {
        (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), szurl.param().abi(), poiprotsink.param().abi(), poibindinfo.param().abi(), grfpi, dwreserved.param().abi()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: windows_core::HRESULT, dwoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocolRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::super::super::Foundation::HANDLE_PTR) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROTOCOLDATA) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocolSink, IInternetProtocolSink_Vtbl, 0x79eac9e5_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocolSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolSink, windows_core::IUnknown);
impl IInternetProtocolSink {
    pub unsafe fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Switch)(windows_core::Interface::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn ReportProgress<P0>(&self, ulstatuscode: u32, szstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), ulstatuscode, szstatustext.param().abi()).ok()
    }
    pub unsafe fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReportData)(windows_core::Interface::as_raw(self), grfbscf, ulprogress, ulprogressmax).ok()
    }
    pub unsafe fn ReportResult<P0>(&self, hrresult: windows_core::HRESULT, dwerror: u32, szresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ReportResult)(windows_core::Interface::as_raw(self), hrresult, dwerror, szresult.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocolSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Switch: unsafe extern "system" fn(*mut core::ffi::c_void, *const PROTOCOLDATA) -> windows_core::HRESULT,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ReportData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
    pub ReportResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetProtocolSinkStackable, IInternetProtocolSinkStackable_Vtbl, 0x79eac9f0_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetProtocolSinkStackable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetProtocolSinkStackable, windows_core::IUnknown);
impl IInternetProtocolSinkStackable {
    pub unsafe fn SwitchSink<P0>(&self, poiprotsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInternetProtocolSink>,
    {
        (windows_core::Interface::vtable(self).SwitchSink)(windows_core::Interface::as_raw(self), poiprotsink.param().abi()).ok()
    }
    pub unsafe fn CommitSwitch(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CommitSwitch)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RollbackSwitch(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RollbackSwitch)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternetProtocolSinkStackable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitSwitch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RollbackSwitch: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetSecurityManager, IInternetSecurityManager_Vtbl, 0x79eac9ee_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetSecurityManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityManager, windows_core::IUnknown);
impl IInternetSecurityManager {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInternetSecurityMgrSite>,
    {
        (windows_core::Interface::vtable(self).SetSecuritySite)(windows_core::Interface::as_raw(self), psite.param().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> windows_core::Result<IInternetSecurityMgrSite> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSecuritySite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).MapUrlToZone)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetSecurityId)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ProcessUrlAction)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).QueryCustomPolicy)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetZoneMapping)(windows_core::Interface::as_raw(self), dwzone, lpszpattern.param().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut Option<super::IEnumString>, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneMappings)(windows_core::Interface::as_raw(self), dwzone, core::mem::transmute(ppenumstring), dwflags).ok()
    }
}
#[repr(C)]
pub struct IInternetSecurityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSecuritySite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecuritySite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MapUrlToZone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, u32) -> windows_core::HRESULT,
    pub GetSecurityId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub ProcessUrlAction: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u8, u32, *const u8, u32, u32, u32) -> windows_core::HRESULT,
    pub QueryCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, u32) -> windows_core::HRESULT,
    pub SetZoneMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetZoneMappings: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetSecurityManagerEx, IInternetSecurityManagerEx_Vtbl, 0xf164edf1_cc7c_4f0d_9a94_34222625c393);
impl core::ops::Deref for IInternetSecurityManagerEx {
    type Target = IInternetSecurityManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityManagerEx, windows_core::IUnknown, IInternetSecurityManager);
impl IInternetSecurityManagerEx {
    pub unsafe fn ProcessUrlActionEx<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ProcessUrlActionEx)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
}
#[repr(C)]
pub struct IInternetSecurityManagerEx_Vtbl {
    pub base__: IInternetSecurityManager_Vtbl,
    pub ProcessUrlActionEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u8, u32, *const u8, u32, u32, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetSecurityManagerEx2, IInternetSecurityManagerEx2_Vtbl, 0xf1e50292_a795_4117_8e09_2b560a72ac60);
impl core::ops::Deref for IInternetSecurityManagerEx2 {
    type Target = IInternetSecurityManagerEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityManagerEx2, windows_core::IUnknown, IInternetSecurityManager, IInternetSecurityManagerEx);
impl IInternetSecurityManagerEx2 {
    pub unsafe fn MapUrlToZoneEx2<P0>(&self, puri: P0, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: Option<*mut windows_core::PWSTR>, pdwoutflags: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IUri>,
    {
        (windows_core::Interface::vtable(self).MapUrlToZoneEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), pdwzone, dwflags, core::mem::transmute(ppwszmappedurl.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pdwoutflags.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ProcessUrlActionEx2<P0>(&self, puri: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IUri>,
    {
        (windows_core::Interface::vtable(self).ProcessUrlActionEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
    pub unsafe fn GetSecurityIdEx2<P0>(&self, puri: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IUri>,
    {
        (windows_core::Interface::vtable(self).GetSecurityIdEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicyEx2<P0>(&self, puri: P0, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IUri>,
    {
        (windows_core::Interface::vtable(self).QueryCustomPolicyEx2)(windows_core::Interface::as_raw(self), puri.param().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetSecurityManagerEx2_Vtbl {
    pub base__: IInternetSecurityManagerEx_Vtbl,
    pub MapUrlToZoneEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub ProcessUrlActionEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u8, u32, *const u8, u32, u32, usize, *mut u32) -> windows_core::HRESULT,
    pub GetSecurityIdEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u8, *mut u32, usize) -> windows_core::HRESULT,
    pub QueryCustomPolicyEx2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut u8, *mut u32, *const u8, u32, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetSecurityMgrSite, IInternetSecurityMgrSite_Vtbl, 0x79eac9ed_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetSecurityMgrSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSecurityMgrSite, windows_core::IUnknown);
impl IInternetSecurityMgrSite {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInternetSecurityMgrSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetSession, IInternetSession_Vtbl, 0x79eac9e7_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetSession {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetSession, windows_core::IUnknown);
impl IInternetSession {
    pub unsafe fn RegisterNameSpace<P0, P1>(&self, pcf: P0, rclsid: *const windows_core::GUID, pwzprotocol: P1, cpatterns: u32, ppwzpatterns: *const windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterNameSpace)(windows_core::Interface::as_raw(self), pcf.param().abi(), rclsid, pwzprotocol.param().abi(), cpatterns, ppwzpatterns, dwreserved).ok()
    }
    pub unsafe fn UnregisterNameSpace<P0, P1>(&self, pcf: P0, pszprotocol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterNameSpace)(windows_core::Interface::as_raw(self), pcf.param().abi(), pszprotocol.param().abi()).ok()
    }
    pub unsafe fn RegisterMimeFilter<P0, P1>(&self, pcf: P0, rclsid: *const windows_core::GUID, pwztype: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterMimeFilter)(windows_core::Interface::as_raw(self), pcf.param().abi(), rclsid, pwztype.param().abi()).ok()
    }
    pub unsafe fn UnregisterMimeFilter<P0, P1>(&self, pcf: P0, pwztype: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IClassFactory>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterMimeFilter)(windows_core::Interface::as_raw(self), pcf.param().abi(), pwztype.param().abi()).ok()
    }
    pub unsafe fn CreateBinding<P0, P1, P2>(&self, pbc: P0, szurl: P1, punkouter: P2, ppunk: *mut Option<windows_core::IUnknown>, ppoinetprot: *mut Option<IInternetProtocol>, dwoption: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IBindCtx>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateBinding)(windows_core::Interface::as_raw(self), pbc.param().abi(), szurl.param().abi(), punkouter.param().abi(), core::mem::transmute(ppunk), core::mem::transmute(ppoinetprot), dwoption).ok()
    }
    pub unsafe fn SetSessionOption(&self, dwoption: u32, pbuffer: *const core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSessionOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer, dwbufferlength, dwreserved).ok()
    }
    pub unsafe fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSessionOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer, pdwbufferlength, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterNameSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub UnregisterNameSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RegisterMimeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterMimeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CreateBinding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSessionOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSessionOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetThreadSwitch, IInternetThreadSwitch_Vtbl, 0x79eac9e8_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetThreadSwitch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetThreadSwitch, windows_core::IUnknown);
impl IInternetThreadSwitch {
    pub unsafe fn Prepare(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Prepare)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Continue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Continue)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternetThreadSwitch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Prepare: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Continue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetZoneManager, IInternetZoneManager_Vtbl, 0x79eac9ef_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IInternetZoneManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetZoneManager, windows_core::IUnknown);
impl IInternetZoneManager {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneAttributes)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetZoneAttributes)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneCustomPolicy)(windows_core::Interface::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const windows_core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetZoneCustomPolicy)(windows_core::Interface::as_raw(self), dwzone, guidkey, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneActionPolicy)(windows_core::Interface::as_raw(self), dwzone, dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetZoneActionPolicy)(windows_core::Interface::as_raw(self), dwzone, dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), urlzonereg).ok()
    }
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PromptAction)(windows_core::Interface::as_raw(self), dwaction, hwndparent.param().abi(), pwszurl.param().abi(), pwsztext.param().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).LogAction)(windows_core::Interface::as_raw(self), dwaction, pwszurl.param().abi(), pwsztext.param().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CreateZoneEnumerator)(windows_core::Interface::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetZoneAt)(windows_core::Interface::as_raw(self), dwenum, dwindex, &mut result__).map(|| result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DestroyZoneEnumerator)(windows_core::Interface::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CopyTemplatePoliciesToZone)(windows_core::Interface::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IInternetZoneManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetZoneAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ZONEATTRIBUTES) -> windows_core::HRESULT,
    pub SetZoneAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ZONEATTRIBUTES) -> windows_core::HRESULT,
    pub GetZoneCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut u8, *mut u32, URLZONEREG) -> windows_core::HRESULT,
    pub SetZoneCustomPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const u8, u32, URLZONEREG) -> windows_core::HRESULT,
    pub GetZoneActionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, u32, URLZONEREG) -> windows_core::HRESULT,
    pub SetZoneActionPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, URLZONEREG) -> windows_core::HRESULT,
    pub PromptAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub LogAction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub CreateZoneEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, u32) -> windows_core::HRESULT,
    pub GetZoneAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
    pub DestroyZoneEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CopyTemplatePoliciesToZone: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetZoneManagerEx, IInternetZoneManagerEx_Vtbl, 0xa4c23339_8e06_431e_9bf4_7e711c085648);
impl core::ops::Deref for IInternetZoneManagerEx {
    type Target = IInternetZoneManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetZoneManagerEx, windows_core::IUnknown, IInternetZoneManager);
impl IInternetZoneManagerEx {
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneActionPolicyEx)(windows_core::Interface::as_raw(self), dwzone, dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), urlzonereg, dwflags).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetZoneActionPolicyEx)(windows_core::Interface::as_raw(self), dwzone, dwaction, core::mem::transmute(ppolicy.as_ptr()), ppolicy.len().try_into().unwrap(), urlzonereg, dwflags).ok()
    }
}
#[repr(C)]
pub struct IInternetZoneManagerEx_Vtbl {
    pub base__: IInternetZoneManager_Vtbl,
    pub GetZoneActionPolicyEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, u32, URLZONEREG, u32) -> windows_core::HRESULT,
    pub SetZoneActionPolicyEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, URLZONEREG, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetZoneManagerEx2, IInternetZoneManagerEx2_Vtbl, 0xedc17559_dd5d_4846_8eef_8becba5a4abf);
impl core::ops::Deref for IInternetZoneManagerEx2 {
    type Target = IInternetZoneManagerEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetZoneManagerEx2, windows_core::IUnknown, IInternetZoneManager, IInternetZoneManagerEx);
impl IInternetZoneManagerEx2 {
    pub unsafe fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetZoneAttributesEx)(windows_core::Interface::as_raw(self), dwzone, pzoneattributes, dwflags).ok()
    }
    pub unsafe fn GetZoneSecurityState<P0>(&self, dwzoneindex: u32, frespectpolicy: P0, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetZoneSecurityState)(windows_core::Interface::as_raw(self), dwzoneindex, frespectpolicy.param().abi(), pdwstate, pfpolicyencountered).ok()
    }
    pub unsafe fn GetIESecurityState<P0, P1>(&self, frespectpolicy: P0, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).GetIESecurityState)(windows_core::Interface::as_raw(self), frespectpolicy.param().abi(), pdwstate, pfpolicyencountered, fnocache.param().abi()).ok()
    }
    pub unsafe fn FixUnsecureSettings(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FixUnsecureSettings)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IInternetZoneManagerEx2_Vtbl {
    pub base__: IInternetZoneManagerEx_Vtbl,
    pub GetZoneAttributesEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ZONEATTRIBUTES, u32) -> windows_core::HRESULT,
    pub GetZoneSecurityState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::super::Foundation::BOOL, *mut u32, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetIESecurityState: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, *mut u32, *mut super::super::super::Foundation::BOOL, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FixUnsecureSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMonikerProp, IMonikerProp_Vtbl, 0xa5ca5f7f_1847_4d87_9c5b_918509f7511d);
impl core::ops::Deref for IMonikerProp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMonikerProp, windows_core::IUnknown);
impl IMonikerProp {
    pub unsafe fn PutProperty<P0>(&self, mkp: MONIKERPROPERTY, val: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), mkp, val.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMonikerProp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, MONIKERPROPERTY, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistMoniker, IPersistMoniker_Vtbl, 0x79eac9c9_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IPersistMoniker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistMoniker, windows_core::IUnknown);
impl IPersistMoniker {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Load<P0, P1, P2>(&self, ffullyavailable: P0, pimkname: P1, pibc: P2, grfmode: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<super::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ffullyavailable.param().abi(), pimkname.param().abi(), pibc.param().abi(), grfmode).ok()
    }
    pub unsafe fn Save<P0, P1, P2>(&self, pimkname: P0, pbc: P1, fremember: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMoniker>,
        P1: windows_core::Param<super::IBindCtx>,
        P2: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pimkname.param().abi(), pbc.param().abi(), fremember.param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<P0, P1>(&self, pimkname: P0, pibc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IMoniker>,
        P1: windows_core::Param<super::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pimkname.param().abi(), pibc.param().abi()).ok()
    }
    pub unsafe fn GetCurMoniker(&self) -> windows_core::Result<super::IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurMoniker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPersistMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftDistExt, ISoftDistExt_Vtbl, 0xb15b8dc1_c7e1_11d0_8680_00aa00bdcb71);
impl core::ops::Deref for ISoftDistExt {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISoftDistExt, windows_core::IUnknown);
impl ISoftDistExt {
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub unsafe fn ProcessSoftDist<P0, P1>(&self, szcdfurl: P0, psoftdistelement: P1, lpsdi: *mut SOFTDISTINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::super::Data::Xml::MsXml::IXMLElement>,
    {
        (windows_core::Interface::vtable(self).ProcessSoftDist)(windows_core::Interface::as_raw(self), szcdfurl.param().abi(), psoftdistelement.param().abi(), lpsdi).ok()
    }
    pub unsafe fn GetFirstCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFirstCodeBase)(windows_core::Interface::as_raw(self), szcodebase, dwmaxsize).ok()
    }
    pub unsafe fn GetNextCodeBase(&self, szcodebase: *const windows_core::PCWSTR, dwmaxsize: *const u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNextCodeBase)(windows_core::Interface::as_raw(self), szcodebase, dwmaxsize).ok()
    }
    pub unsafe fn AsyncInstallDistributionUnit<P0>(&self, pbc: P0, pvreserved: *const core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).AsyncInstallDistributionUnit)(windows_core::Interface::as_raw(self), pbc.param().abi(), pvreserved, flags, lpcbh).ok()
    }
}
#[repr(C)]
pub struct ISoftDistExt_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Data_Xml_MsXml")]
    pub ProcessSoftDist: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut SOFTDISTINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Data_Xml_MsXml"))]
    ProcessSoftDist: usize,
    pub GetFirstCodeBase: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, *const u32) -> windows_core::HRESULT,
    pub GetNextCodeBase: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, *const u32) -> windows_core::HRESULT,
    pub AsyncInstallDistributionUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const core::ffi::c_void, u32, *const CODEBASEHOLD) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriBuilderFactory, IUriBuilderFactory_Vtbl, 0xe982ce48_0b96_440c_bc37_0c869b27a29e);
impl core::ops::Deref for IUriBuilderFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUriBuilderFactory, windows_core::IUnknown);
impl IUriBuilderFactory {
    pub unsafe fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<super::IUriBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateIUriBuilder)(windows_core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> windows_core::Result<super::IUriBuilder> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateInitializedIUriBuilder)(windows_core::Interface::as_raw(self), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUriBuilderFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateIUriBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInitializedIUriBuilder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriContainer, IUriContainer_Vtbl, 0xa158a630_ed6f_45fb_b987_f68676f57752);
impl core::ops::Deref for IUriContainer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUriContainer, windows_core::IUnknown);
impl IUriContainer {
    pub unsafe fn GetIUri(&self) -> windows_core::Result<super::IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUriContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetCacheHints, IWinInetCacheHints_Vtbl, 0xdd1ec3b3_8391_4fdb_a9e6_347c3caaa7dd);
impl core::ops::Deref for IWinInetCacheHints {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetCacheHints, windows_core::IUnknown);
impl IWinInetCacheHints {
    pub unsafe fn SetCacheExtension<P0>(&self, pwzext: P0, pszcachefile: *mut core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCacheExtension)(windows_core::Interface::as_raw(self), pwzext.param().abi(), pszcachefile, pcbcachefile, pdwwinineterror, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IWinInetCacheHints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCacheExtension: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetCacheHints2, IWinInetCacheHints2_Vtbl, 0x7857aeac_d31f_49bf_884e_dd46df36780a);
impl core::ops::Deref for IWinInetCacheHints2 {
    type Target = IWinInetCacheHints;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetCacheHints2, windows_core::IUnknown, IWinInetCacheHints);
impl IWinInetCacheHints2 {
    pub unsafe fn SetCacheExtension2<P0>(&self, pwzext: P0, pwzcachefile: windows_core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetCacheExtension2)(windows_core::Interface::as_raw(self), pwzext.param().abi(), core::mem::transmute(pwzcachefile), pcchcachefile, pdwwinineterror, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IWinInetCacheHints2_Vtbl {
    pub base__: IWinInetCacheHints_Vtbl,
    pub SetCacheExtension2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetFileStream, IWinInetFileStream_Vtbl, 0xf134c4b7_b1f8_4e75_b886_74b90943becb);
impl core::ops::Deref for IWinInetFileStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetFileStream, windows_core::IUnknown);
impl IWinInetFileStream {
    pub unsafe fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHandleForUnlock)(windows_core::Interface::as_raw(self), hwininetlockhandle, dwreserved).ok()
    }
    pub unsafe fn SetDeleteFile(&self, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDeleteFile)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
}
#[repr(C)]
pub struct IWinInetFileStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHandleForUnlock: unsafe extern "system" fn(*mut core::ffi::c_void, usize, usize) -> windows_core::HRESULT,
    pub SetDeleteFile: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetHttpInfo, IWinInetHttpInfo_Vtbl, 0x79eac9d8_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IWinInetHttpInfo {
    type Target = IWinInetInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetHttpInfo, windows_core::IUnknown, IWinInetInfo);
impl IWinInetHttpInfo {
    pub unsafe fn QueryInfo(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryInfo)(windows_core::Interface::as_raw(self), dwoption, pbuffer, pcbbuf, pdwflags, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IWinInetHttpInfo_Vtbl {
    pub base__: IWinInetInfo_Vtbl,
    pub QueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetHttpTimeouts, IWinInetHttpTimeouts_Vtbl, 0xf286fa56_c1fd_4270_8e67_b3eb790a81e8);
impl core::ops::Deref for IWinInetHttpTimeouts {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetHttpTimeouts, windows_core::IUnknown);
impl IWinInetHttpTimeouts {
    pub unsafe fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetRequestTimeouts)(windows_core::Interface::as_raw(self), pdwconnecttimeout, pdwsendtimeout, pdwreceivetimeout).ok()
    }
}
#[repr(C)]
pub struct IWinInetHttpTimeouts_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRequestTimeouts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWinInetInfo, IWinInetInfo_Vtbl, 0x79eac9d6_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IWinInetInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWinInetInfo, windows_core::IUnknown);
impl IWinInetInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut core::ffi::c_void, pcbbuf: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryOption)(windows_core::Interface::as_raw(self), dwoption, pbuffer, pcbbuf).ok()
    }
}
#[repr(C)]
pub struct IWinInetInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryOption: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowForBindingUI, IWindowForBindingUI_Vtbl, 0x79eac9d5_bafa_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IWindowForBindingUI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWindowForBindingUI, windows_core::IUnknown);
impl IWindowForBindingUI {
    pub unsafe fn GetWindow(&self, rguidreason: *const windows_core::GUID) -> windows_core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), rguidreason, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWindowForBindingUI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWrappedProtocol, IWrappedProtocol_Vtbl, 0x53c84785_8425_4dc5_971b_e58d9c19f9b6);
impl core::ops::Deref for IWrappedProtocol {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWrappedProtocol, windows_core::IUnknown);
impl IWrappedProtocol {
    pub unsafe fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWrapperCode)(windows_core::Interface::as_raw(self), pncode, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IWrappedProtocol_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWrapperCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IZoneIdentifier, IZoneIdentifier_Vtbl, 0xcd45f185_1b21_48e2_967b_ead743a8914e);
impl core::ops::Deref for IZoneIdentifier {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IZoneIdentifier, windows_core::IUnknown);
impl IZoneIdentifier {
    pub unsafe fn GetId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), dwzone).ok()
    }
    pub unsafe fn Remove(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IZoneIdentifier_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IZoneIdentifier2, IZoneIdentifier2_Vtbl, 0xeb5e760c_09ef_45c0_b510_70830ce31e6a);
impl core::ops::Deref for IZoneIdentifier2 {
    type Target = IZoneIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IZoneIdentifier2, windows_core::IUnknown, IZoneIdentifier);
impl IZoneIdentifier2 {
    pub unsafe fn GetLastWriterPackageFamilyName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetLastWriterPackageFamilyName<P0>(&self, packagefamilyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self), packagefamilyname.param().abi()).ok()
    }
    pub unsafe fn RemoveLastWriterPackageFamilyName(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveLastWriterPackageFamilyName)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetAppZoneId(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAppZoneId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAppZoneId(&self, zone: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAppZoneId)(windows_core::Interface::as_raw(self), zone).ok()
    }
    pub unsafe fn RemoveAppZoneId(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAppZoneId)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IZoneIdentifier2_Vtbl {
    pub base__: IZoneIdentifier_Vtbl,
    pub GetLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveLastWriterPackageFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAppZoneId: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const AUTHENTICATEF_BASIC: AUTHENTICATEF = AUTHENTICATEF(2i32);
pub const AUTHENTICATEF_HTTP: AUTHENTICATEF = AUTHENTICATEF(4i32);
pub const AUTHENTICATEF_PROXY: AUTHENTICATEF = AUTHENTICATEF(1i32);
pub const BINDF2_ALLOW_PROXY_CRED_PROMPT: BINDF2 = BINDF2(256i32);
pub const BINDF2_DISABLEAUTOCOOKIEHANDLING: BINDF2 = BINDF2(2i32);
pub const BINDF2_DISABLEBASICOVERHTTP: BINDF2 = BINDF2(1i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_CACHING: BINDF2 = BINDF2(64i32);
pub const BINDF2_DISABLE_HTTP_REDIRECT_XSECURITYID: BINDF2 = BINDF2(8i32);
pub const BINDF2_KEEP_CALLBACK_MODULE_LOADED: BINDF2 = BINDF2(128i32);
pub const BINDF2_READ_DATA_GREATER_THAN_4GB: BINDF2 = BINDF2(4i32);
pub const BINDF2_RESERVED_1: BINDF2 = BINDF2(-2147483648i32);
pub const BINDF2_RESERVED_10: BINDF2 = BINDF2(65536i32);
pub const BINDF2_RESERVED_11: BINDF2 = BINDF2(32768i32);
pub const BINDF2_RESERVED_12: BINDF2 = BINDF2(16384i32);
pub const BINDF2_RESERVED_13: BINDF2 = BINDF2(8192i32);
pub const BINDF2_RESERVED_14: BINDF2 = BINDF2(4096i32);
pub const BINDF2_RESERVED_15: BINDF2 = BINDF2(2048i32);
pub const BINDF2_RESERVED_16: BINDF2 = BINDF2(1024i32);
pub const BINDF2_RESERVED_17: BINDF2 = BINDF2(512i32);
pub const BINDF2_RESERVED_2: BINDF2 = BINDF2(1073741824i32);
pub const BINDF2_RESERVED_3: BINDF2 = BINDF2(536870912i32);
pub const BINDF2_RESERVED_4: BINDF2 = BINDF2(268435456i32);
pub const BINDF2_RESERVED_5: BINDF2 = BINDF2(134217728i32);
pub const BINDF2_RESERVED_6: BINDF2 = BINDF2(67108864i32);
pub const BINDF2_RESERVED_7: BINDF2 = BINDF2(33554432i32);
pub const BINDF2_RESERVED_8: BINDF2 = BINDF2(16777216i32);
pub const BINDF2_RESERVED_9: BINDF2 = BINDF2(8388608i32);
pub const BINDF2_RESERVED_A: BINDF2 = BINDF2(4194304i32);
pub const BINDF2_RESERVED_B: BINDF2 = BINDF2(2097152i32);
pub const BINDF2_RESERVED_C: BINDF2 = BINDF2(1048576i32);
pub const BINDF2_RESERVED_D: BINDF2 = BINDF2(524288i32);
pub const BINDF2_RESERVED_E: BINDF2 = BINDF2(262144i32);
pub const BINDF2_RESERVED_F: BINDF2 = BINDF2(131072i32);
pub const BINDF2_SETDOWNLOADMODE: BINDF2 = BINDF2(32i32);
pub const BINDF_ASYNCHRONOUS: BINDF = BINDF(1i32);
pub const BINDF_ASYNCSTORAGE: BINDF = BINDF(2i32);
pub const BINDF_DIRECT_READ: BINDF = BINDF(131072i32);
pub const BINDF_ENFORCERESTRICTED: BINDF = BINDF(8388608i32);
pub const BINDF_FORMS_SUBMIT: BINDF = BINDF(262144i32);
pub const BINDF_FREE_THREADED: BINDF = BINDF(65536i32);
pub const BINDF_FROMURLMON: BINDF = BINDF(1048576i32);
pub const BINDF_FWD_BACK: BINDF = BINDF(2097152i32);
pub const BINDF_GETCLASSOBJECT: BINDF = BINDF(16384i32);
pub const BINDF_GETFROMCACHE_IF_NET_FAIL: BINDF = BINDF(524288i32);
pub const BINDF_GETNEWESTVERSION: BINDF = BINDF(16i32);
pub const BINDF_HYPERLINK: BINDF = BINDF(1024i32);
pub const BINDF_IGNORESECURITYPROBLEM: BINDF = BINDF(256i32);
pub const BINDF_NEEDFILE: BINDF = BINDF(64i32);
pub const BINDF_NOPROGRESSIVERENDERING: BINDF = BINDF(4i32);
pub const BINDF_NOWRITECACHE: BINDF = BINDF(32i32);
pub const BINDF_NO_UI: BINDF = BINDF(2048i32);
pub const BINDF_OFFLINEOPERATION: BINDF = BINDF(8i32);
pub const BINDF_PRAGMA_NO_CACHE: BINDF = BINDF(8192i32);
pub const BINDF_PREFERDEFAULTHANDLER: BINDF = BINDF(4194304i32);
pub const BINDF_PULLDATA: BINDF = BINDF(128i32);
pub const BINDF_RESERVED_1: BINDF = BINDF(32768i32);
pub const BINDF_RESERVED_2: BINDF = BINDF(-2147483648i32);
pub const BINDF_RESERVED_3: BINDF = BINDF(16777216i32);
pub const BINDF_RESERVED_4: BINDF = BINDF(33554432i32);
pub const BINDF_RESERVED_5: BINDF = BINDF(67108864i32);
pub const BINDF_RESERVED_6: BINDF = BINDF(134217728i32);
pub const BINDF_RESERVED_7: BINDF = BINDF(1073741824i32);
pub const BINDF_RESERVED_8: BINDF = BINDF(536870912i32);
pub const BINDF_RESYNCHRONIZE: BINDF = BINDF(512i32);
pub const BINDF_SILENTOPERATION: BINDF = BINDF(4096i32);
pub const BINDHANDLETYPES_APPCACHE: BINDHANDLETYPES = BINDHANDLETYPES(0i32);
pub const BINDHANDLETYPES_COUNT: BINDHANDLETYPES = BINDHANDLETYPES(2i32);
pub const BINDHANDLETYPES_DEPENDENCY: BINDHANDLETYPES = BINDHANDLETYPES(1i32);
pub const BINDINFO_OPTIONS_ALLOWCONNECTDATA: BINDINFO_OPTIONS = BINDINFO_OPTIONS(536870912i32);
pub const BINDINFO_OPTIONS_BINDTOOBJECT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1048576i32);
pub const BINDINFO_OPTIONS_DISABLEAUTOREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(1073741824i32);
pub const BINDINFO_OPTIONS_DISABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(262144i32);
pub const BINDINFO_OPTIONS_ENABLE_UTF8: BINDINFO_OPTIONS = BINDINFO_OPTIONS(131072i32);
pub const BINDINFO_OPTIONS_IGNOREHTTPHTTPSREDIRECTS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(16777216i32);
pub const BINDINFO_OPTIONS_IGNOREMIMETEXTPLAIN: BINDINFO_OPTIONS = BINDINFO_OPTIONS(4194304i32);
pub const BINDINFO_OPTIONS_IGNORE_SSLERRORS_ONCE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(33554432i32);
pub const BINDINFO_OPTIONS_SECURITYOPTOUT: BINDINFO_OPTIONS = BINDINFO_OPTIONS(2097152i32);
pub const BINDINFO_OPTIONS_SHDOCVW_NAVIGATE: BINDINFO_OPTIONS = BINDINFO_OPTIONS(-2147483648i32);
pub const BINDINFO_OPTIONS_USEBINDSTRINGCREDS: BINDINFO_OPTIONS = BINDINFO_OPTIONS(8388608i32);
pub const BINDINFO_OPTIONS_USE_IE_ENCODING: BINDINFO_OPTIONS = BINDINFO_OPTIONS(524288i32);
pub const BINDINFO_OPTIONS_WININETFLAG: BINDINFO_OPTIONS = BINDINFO_OPTIONS(65536i32);
pub const BINDINFO_WPC_DOWNLOADBLOCKED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(134217728i32);
pub const BINDINFO_WPC_LOGGING_ENABLED: BINDINFO_OPTIONS = BINDINFO_OPTIONS(268435456i32);
pub const BINDSTATUS_64BIT_PROGRESS: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_ACCEPTRANGES: BINDSTATUS = BINDSTATUS(33i32);
pub const BINDSTATUS_BEGINDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(7i32);
pub const BINDSTATUS_BEGINDOWNLOADDATA: BINDSTATUS = BINDSTATUS(4i32);
pub const BINDSTATUS_BEGINSYNCOPERATION: BINDSTATUS = BINDSTATUS(15i32);
pub const BINDSTATUS_BEGINUPLOADDATA: BINDSTATUS = BINDSTATUS(17i32);
pub const BINDSTATUS_CACHECONTROL: BINDSTATUS = BINDSTATUS(48i32);
pub const BINDSTATUS_CACHEFILENAMEAVAILABLE: BINDSTATUS = BINDSTATUS(14i32);
pub const BINDSTATUS_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(12i32);
pub const BINDSTATUS_CLASSINSTALLLOCATION: BINDSTATUS = BINDSTATUS(23i32);
pub const BINDSTATUS_CLSIDCANINSTANTIATE: BINDSTATUS = BINDSTATUS(28i32);
pub const BINDSTATUS_COMPACT_POLICY_RECEIVED: BINDSTATUS = BINDSTATUS(35i32);
pub const BINDSTATUS_CONNECTING: BINDSTATUS = BINDSTATUS(2i32);
pub const BINDSTATUS_CONTENTDISPOSITIONATTACH: BINDSTATUS = BINDSTATUS(26i32);
pub const BINDSTATUS_CONTENTDISPOSITIONFILENAME: BINDSTATUS = BINDSTATUS(49i32);
pub const BINDSTATUS_COOKIE_SENT: BINDSTATUS = BINDSTATUS(34i32);
pub const BINDSTATUS_COOKIE_STATE_ACCEPT: BINDSTATUS = BINDSTATUS(38i32);
pub const BINDSTATUS_COOKIE_STATE_DOWNGRADE: BINDSTATUS = BINDSTATUS(42i32);
pub const BINDSTATUS_COOKIE_STATE_LEASH: BINDSTATUS = BINDSTATUS(41i32);
pub const BINDSTATUS_COOKIE_STATE_PROMPT: BINDSTATUS = BINDSTATUS(40i32);
pub const BINDSTATUS_COOKIE_STATE_REJECT: BINDSTATUS = BINDSTATUS(39i32);
pub const BINDSTATUS_COOKIE_STATE_UNKNOWN: BINDSTATUS = BINDSTATUS(37i32);
pub const BINDSTATUS_COOKIE_SUPPRESSED: BINDSTATUS = BINDSTATUS(36i32);
pub const BINDSTATUS_DECODING: BINDSTATUS = BINDSTATUS(24i32);
pub const BINDSTATUS_DIRECTBIND: BINDSTATUS = BINDSTATUS(30i32);
pub const BINDSTATUS_DISPLAYNAMEAVAILABLE: BINDSTATUS = BINDSTATUS(52i32);
pub const BINDSTATUS_DOWNLOADINGDATA: BINDSTATUS = BINDSTATUS(5i32);
pub const BINDSTATUS_ENCODING: BINDSTATUS = BINDSTATUS(21i32);
pub const BINDSTATUS_ENDDOWNLOADCOMPONENTS: BINDSTATUS = BINDSTATUS(9i32);
pub const BINDSTATUS_ENDDOWNLOADDATA: BINDSTATUS = BINDSTATUS(6i32);
pub const BINDSTATUS_ENDSYNCOPERATION: BINDSTATUS = BINDSTATUS(16i32);
pub const BINDSTATUS_ENDUPLOADDATA: BINDSTATUS = BINDSTATUS(19i32);
pub const BINDSTATUS_FILTERREPORTMIMETYPE: BINDSTATUS = BINDSTATUS(27i32);
pub const BINDSTATUS_FINDINGRESOURCE: BINDSTATUS = BINDSTATUS(1i32);
pub const BINDSTATUS_INSTALLINGCOMPONENTS: BINDSTATUS = BINDSTATUS(8i32);
pub const BINDSTATUS_IUNKNOWNAVAILABLE: BINDSTATUS = BINDSTATUS(29i32);
pub const BINDSTATUS_LAST: BINDSTATUS = BINDSTATUS(56i32);
pub const BINDSTATUS_LAST_PRIVATE: BINDSTATUS = BINDSTATUS(77i32);
pub const BINDSTATUS_LOADINGMIMEHANDLER: BINDSTATUS = BINDSTATUS(25i32);
pub const BINDSTATUS_MIMETEXTPLAINMISMATCH: BINDSTATUS = BINDSTATUS(50i32);
pub const BINDSTATUS_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(13i32);
pub const BINDSTATUS_P3P_HEADER: BINDSTATUS = BINDSTATUS(44i32);
pub const BINDSTATUS_PERSISTENT_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(46i32);
pub const BINDSTATUS_POLICY_HREF: BINDSTATUS = BINDSTATUS(43i32);
pub const BINDSTATUS_PROTOCOLCLASSID: BINDSTATUS = BINDSTATUS(20i32);
pub const BINDSTATUS_PROXYDETECTING: BINDSTATUS = BINDSTATUS(32i32);
pub const BINDSTATUS_PUBLISHERAVAILABLE: BINDSTATUS = BINDSTATUS(51i32);
pub const BINDSTATUS_RAWMIMETYPE: BINDSTATUS = BINDSTATUS(31i32);
pub const BINDSTATUS_REDIRECTING: BINDSTATUS = BINDSTATUS(3i32);
pub const BINDSTATUS_RESERVED_0: BINDSTATUS = BINDSTATUS(57i32);
pub const BINDSTATUS_RESERVED_1: BINDSTATUS = BINDSTATUS(58i32);
pub const BINDSTATUS_RESERVED_10: BINDSTATUS = BINDSTATUS(73i32);
pub const BINDSTATUS_RESERVED_11: BINDSTATUS = BINDSTATUS(74i32);
pub const BINDSTATUS_RESERVED_12: BINDSTATUS = BINDSTATUS(75i32);
pub const BINDSTATUS_RESERVED_13: BINDSTATUS = BINDSTATUS(76i32);
pub const BINDSTATUS_RESERVED_14: BINDSTATUS = BINDSTATUS(77i32);
pub const BINDSTATUS_RESERVED_2: BINDSTATUS = BINDSTATUS(59i32);
pub const BINDSTATUS_RESERVED_3: BINDSTATUS = BINDSTATUS(60i32);
pub const BINDSTATUS_RESERVED_4: BINDSTATUS = BINDSTATUS(61i32);
pub const BINDSTATUS_RESERVED_5: BINDSTATUS = BINDSTATUS(62i32);
pub const BINDSTATUS_RESERVED_6: BINDSTATUS = BINDSTATUS(63i32);
pub const BINDSTATUS_RESERVED_7: BINDSTATUS = BINDSTATUS(64i32);
pub const BINDSTATUS_RESERVED_8: BINDSTATUS = BINDSTATUS(65i32);
pub const BINDSTATUS_RESERVED_9: BINDSTATUS = BINDSTATUS(66i32);
pub const BINDSTATUS_RESERVED_A: BINDSTATUS = BINDSTATUS(67i32);
pub const BINDSTATUS_RESERVED_B: BINDSTATUS = BINDSTATUS(68i32);
pub const BINDSTATUS_RESERVED_C: BINDSTATUS = BINDSTATUS(69i32);
pub const BINDSTATUS_RESERVED_D: BINDSTATUS = BINDSTATUS(70i32);
pub const BINDSTATUS_RESERVED_E: BINDSTATUS = BINDSTATUS(71i32);
pub const BINDSTATUS_RESERVED_F: BINDSTATUS = BINDSTATUS(72i32);
pub const BINDSTATUS_SENDINGREQUEST: BINDSTATUS = BINDSTATUS(11i32);
pub const BINDSTATUS_SERVER_MIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(54i32);
pub const BINDSTATUS_SESSION_COOKIES_ALLOWED: BINDSTATUS = BINDSTATUS(47i32);
pub const BINDSTATUS_SESSION_COOKIE_RECEIVED: BINDSTATUS = BINDSTATUS(45i32);
pub const BINDSTATUS_SNIFFED_CLASSIDAVAILABLE: BINDSTATUS = BINDSTATUS(55i32);
pub const BINDSTATUS_SSLUX_NAVBLOCKED: BINDSTATUS = BINDSTATUS(53i32);
pub const BINDSTATUS_UPLOADINGDATA: BINDSTATUS = BINDSTATUS(18i32);
pub const BINDSTATUS_USINGCACHEDCOPY: BINDSTATUS = BINDSTATUS(10i32);
pub const BINDSTATUS_VERIFIEDMIMETYPEAVAILABLE: BINDSTATUS = BINDSTATUS(22i32);
pub const BINDSTRING_ACCEPT_ENCODINGS: BINDSTRING = BINDSTRING(11i32);
pub const BINDSTRING_ACCEPT_MIMES: BINDSTRING = BINDSTRING(2i32);
pub const BINDSTRING_DOC_URL: BINDSTRING = BINDSTRING(25i32);
pub const BINDSTRING_DOWNLOADPATH: BINDSTRING = BINDSTRING(19i32);
pub const BINDSTRING_ENTERPRISE_ID: BINDSTRING = BINDSTRING(24i32);
pub const BINDSTRING_EXTRA_URL: BINDSTRING = BINDSTRING(3i32);
pub const BINDSTRING_FLAG_BIND_TO_OBJECT: BINDSTRING = BINDSTRING(16i32);
pub const BINDSTRING_HEADERS: BINDSTRING = BINDSTRING(1i32);
pub const BINDSTRING_IID: BINDSTRING = BINDSTRING(15i32);
pub const BINDSTRING_INITIAL_FILENAME: BINDSTRING = BINDSTRING(21i32);
pub const BINDSTRING_LANGUAGE: BINDSTRING = BINDSTRING(4i32);
pub const BINDSTRING_OS: BINDSTRING = BINDSTRING(9i32);
pub const BINDSTRING_PASSWORD: BINDSTRING = BINDSTRING(6i32);
pub const BINDSTRING_POST_COOKIE: BINDSTRING = BINDSTRING(12i32);
pub const BINDSTRING_POST_DATA_MIME: BINDSTRING = BINDSTRING(13i32);
pub const BINDSTRING_PROXY_PASSWORD: BINDSTRING = BINDSTRING(23i32);
pub const BINDSTRING_PROXY_USERNAME: BINDSTRING = BINDSTRING(22i32);
pub const BINDSTRING_PTR_BIND_CONTEXT: BINDSTRING = BINDSTRING(17i32);
pub const BINDSTRING_ROOTDOC_URL: BINDSTRING = BINDSTRING(20i32);
pub const BINDSTRING_SAMESITE_COOKIE_LEVEL: BINDSTRING = BINDSTRING(26i32);
pub const BINDSTRING_UA_COLOR: BINDSTRING = BINDSTRING(8i32);
pub const BINDSTRING_UA_PIXELS: BINDSTRING = BINDSTRING(7i32);
pub const BINDSTRING_URL: BINDSTRING = BINDSTRING(14i32);
pub const BINDSTRING_USERNAME: BINDSTRING = BINDSTRING(5i32);
pub const BINDSTRING_USER_AGENT: BINDSTRING = BINDSTRING(10i32);
pub const BINDSTRING_XDR_ORIGIN: BINDSTRING = BINDSTRING(18i32);
pub const BINDVERB_CUSTOM: BINDVERB = BINDVERB(3i32);
pub const BINDVERB_GET: BINDVERB = BINDVERB(0i32);
pub const BINDVERB_POST: BINDVERB = BINDVERB(1i32);
pub const BINDVERB_PUT: BINDVERB = BINDVERB(2i32);
pub const BINDVERB_RESERVED1: BINDVERB = BINDVERB(4i32);
pub const BSCF_64BITLENGTHDOWNLOAD: BSCF = BSCF(64i32);
pub const BSCF_AVAILABLEDATASIZEUNKNOWN: BSCF = BSCF(16i32);
pub const BSCF_DATAFULLYAVAILABLE: BSCF = BSCF(8i32);
pub const BSCF_FIRSTDATANOTIFICATION: BSCF = BSCF(1i32);
pub const BSCF_INTERMEDIATEDATANOTIFICATION: BSCF = BSCF(2i32);
pub const BSCF_LASTDATANOTIFICATION: BSCF = BSCF(4i32);
pub const BSCF_SKIPDRAINDATAFORFILEURLS: BSCF = BSCF(32i32);
pub const CF_NULL: u32 = 0u32;
pub const CIP_ACCESS_DENIED: CIP_STATUS = CIP_STATUS(1i32);
pub const CIP_DISK_FULL: CIP_STATUS = CIP_STATUS(0i32);
pub const CIP_EXE_SELF_REGISTERATION_TIMEOUT: CIP_STATUS = CIP_STATUS(6i32);
pub const CIP_NAME_CONFLICT: CIP_STATUS = CIP_STATUS(4i32);
pub const CIP_NEED_REBOOT: CIP_STATUS = CIP_STATUS(8i32);
pub const CIP_NEED_REBOOT_UI_PERMISSION: CIP_STATUS = CIP_STATUS(9i32);
pub const CIP_NEWER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(2i32);
pub const CIP_OLDER_VERSION_EXISTS: CIP_STATUS = CIP_STATUS(3i32);
pub const CIP_TRUST_VERIFICATION_COMPONENT_MISSING: CIP_STATUS = CIP_STATUS(5i32);
pub const CIP_UNSAFE_TO_ABORT: CIP_STATUS = CIP_STATUS(7i32);
pub const CLASSIDPROP: MONIKERPROPERTY = MONIKERPROPERTY(2i32);
pub const CONFIRMSAFETYACTION_LOADOBJECT: u32 = 1u32;
pub const E_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x8000000A_u32 as _);
pub const FEATURE_ADDON_MANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(13i32);
pub const FEATURE_BEHAVIORS: INTERNETFEATURELIST = INTERNETFEATURELIST(6i32);
pub const FEATURE_BLOCK_INPUT_PROMPTS: INTERNETFEATURELIST = INTERNETFEATURELIST(27i32);
pub const FEATURE_DISABLE_LEGACY_COMPRESSION: INTERNETFEATURELIST = INTERNETFEATURELIST(22i32);
pub const FEATURE_DISABLE_MK_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(7i32);
pub const FEATURE_DISABLE_NAVIGATION_SOUNDS: INTERNETFEATURELIST = INTERNETFEATURELIST(21i32);
pub const FEATURE_DISABLE_TELNET_PROTOCOL: INTERNETFEATURELIST = INTERNETFEATURELIST(25i32);
pub const FEATURE_ENTRY_COUNT: INTERNETFEATURELIST = INTERNETFEATURELIST(28i32);
pub const FEATURE_FEEDS: INTERNETFEATURELIST = INTERNETFEATURELIST(26i32);
pub const FEATURE_FORCE_ADDR_AND_STATUS: INTERNETFEATURELIST = INTERNETFEATURELIST(23i32);
pub const FEATURE_GET_URL_DOM_FILEPATH_UNENCODED: INTERNETFEATURELIST = INTERNETFEATURELIST(18i32);
pub const FEATURE_HTTP_USERNAME_PASSWORD_DISABLE: INTERNETFEATURELIST = INTERNETFEATURELIST(15i32);
pub const FEATURE_LOCALMACHINE_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(8i32);
pub const FEATURE_MIME_HANDLING: INTERNETFEATURELIST = INTERNETFEATURELIST(2i32);
pub const FEATURE_MIME_SNIFFING: INTERNETFEATURELIST = INTERNETFEATURELIST(3i32);
pub const FEATURE_OBJECT_CACHING: INTERNETFEATURELIST = INTERNETFEATURELIST(0i32);
pub const FEATURE_PROTOCOL_LOCKDOWN: INTERNETFEATURELIST = INTERNETFEATURELIST(14i32);
pub const FEATURE_RESTRICT_ACTIVEXINSTALL: INTERNETFEATURELIST = INTERNETFEATURELIST(10i32);
pub const FEATURE_RESTRICT_FILEDOWNLOAD: INTERNETFEATURELIST = INTERNETFEATURELIST(12i32);
pub const FEATURE_SAFE_BINDTOOBJECT: INTERNETFEATURELIST = INTERNETFEATURELIST(16i32);
pub const FEATURE_SECURITYBAND: INTERNETFEATURELIST = INTERNETFEATURELIST(9i32);
pub const FEATURE_SSLUX: INTERNETFEATURELIST = INTERNETFEATURELIST(20i32);
pub const FEATURE_TABBED_BROWSING: INTERNETFEATURELIST = INTERNETFEATURELIST(19i32);
pub const FEATURE_UNC_SAVEDFILECHECK: INTERNETFEATURELIST = INTERNETFEATURELIST(17i32);
pub const FEATURE_VALIDATE_NAVIGATE_URL: INTERNETFEATURELIST = INTERNETFEATURELIST(11i32);
pub const FEATURE_WEBOC_POPUPMANAGEMENT: INTERNETFEATURELIST = INTERNETFEATURELIST(5i32);
pub const FEATURE_WINDOW_RESTRICTIONS: INTERNETFEATURELIST = INTERNETFEATURELIST(4i32);
pub const FEATURE_XMLHTTP: INTERNETFEATURELIST = INTERNETFEATURELIST(24i32);
pub const FEATURE_ZONE_ELEVATION: INTERNETFEATURELIST = INTERNETFEATURELIST(1i32);
pub const FIEF_FLAG_FORCE_JITUI: u32 = 1u32;
pub const FIEF_FLAG_PEEK: u32 = 2u32;
pub const FIEF_FLAG_RESERVED_0: u32 = 8u32;
pub const FIEF_FLAG_SKIP_INSTALLED_VERSION_CHECK: u32 = 4u32;
pub const FMFD_DEFAULT: u32 = 0u32;
pub const FMFD_ENABLEMIMESNIFFING: u32 = 2u32;
pub const FMFD_IGNOREMIMETEXTPLAIN: u32 = 4u32;
pub const FMFD_RESERVED_1: u32 = 64u32;
pub const FMFD_RESERVED_2: u32 = 128u32;
pub const FMFD_RESPECTTEXTPLAIN: u32 = 16u32;
pub const FMFD_RETURNUPDATEDIMGMIMES: u32 = 32u32;
pub const FMFD_SERVERMIME: u32 = 8u32;
pub const FMFD_URLASFILENAME: u32 = 1u32;
pub const GET_FEATURE_FROM_PROCESS: u32 = 2u32;
pub const GET_FEATURE_FROM_REGISTRY: u32 = 4u32;
pub const GET_FEATURE_FROM_THREAD: u32 = 1u32;
pub const GET_FEATURE_FROM_THREAD_INTERNET: u32 = 64u32;
pub const GET_FEATURE_FROM_THREAD_INTRANET: u32 = 16u32;
pub const GET_FEATURE_FROM_THREAD_LOCALMACHINE: u32 = 8u32;
pub const GET_FEATURE_FROM_THREAD_RESTRICTED: u32 = 128u32;
pub const GET_FEATURE_FROM_THREAD_TRUSTED: u32 = 32u32;
pub const IE_EPM_OBJECT_EVENT: IEObjectType = IEObjectType(0i32);
pub const IE_EPM_OBJECT_FILE: IEObjectType = IEObjectType(5i32);
pub const IE_EPM_OBJECT_MUTEX: IEObjectType = IEObjectType(1i32);
pub const IE_EPM_OBJECT_NAMED_PIPE: IEObjectType = IEObjectType(6i32);
pub const IE_EPM_OBJECT_REGISTRY: IEObjectType = IEObjectType(7i32);
pub const IE_EPM_OBJECT_SEMAPHORE: IEObjectType = IEObjectType(2i32);
pub const IE_EPM_OBJECT_SHARED_MEMORY: IEObjectType = IEObjectType(3i32);
pub const IE_EPM_OBJECT_WAITABLE_TIMER: IEObjectType = IEObjectType(4i32);
pub const INET_E_AUTHENTICATION_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x800C0009_u32 as _);
pub const INET_E_BLOCKED_ENHANCEDPROTECTEDMODE: windows_core::HRESULT = windows_core::HRESULT(0x800C0506_u32 as _);
pub const INET_E_BLOCKED_PLUGGABLE_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x800C0505_u32 as _);
pub const INET_E_BLOCKED_REDIRECT_XSECURITYID: windows_core::HRESULT = windows_core::HRESULT(0x800C001B_u32 as _);
pub const INET_E_CANNOT_CONNECT: windows_core::HRESULT = windows_core::HRESULT(0x800C0004_u32 as _);
pub const INET_E_CANNOT_INSTANTIATE_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0x800C0010_u32 as _);
pub const INET_E_CANNOT_LOAD_DATA: windows_core::HRESULT = windows_core::HRESULT(0x800C000F_u32 as _);
pub const INET_E_CANNOT_LOCK_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x800C0016_u32 as _);
pub const INET_E_CANNOT_REPLACE_SFP_FILE: windows_core::HRESULT = windows_core::HRESULT(0x800C0300_u32 as _);
pub const INET_E_CODE_DOWNLOAD_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x800C0100_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_ARM: windows_core::HRESULT = windows_core::HRESULT(0x800C0504_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BITNESS: windows_core::HRESULT = windows_core::HRESULT(0x800C0507_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_BY_HASH_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x800C0500_u32 as _);
pub const INET_E_CODE_INSTALL_BLOCKED_IMMERSIVE: windows_core::HRESULT = windows_core::HRESULT(0x800C0502_u32 as _);
pub const INET_E_CODE_INSTALL_SUPPRESSED: windows_core::HRESULT = windows_core::HRESULT(0x800C0400_u32 as _);
pub const INET_E_CONNECTION_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x800C000B_u32 as _);
pub const INET_E_DATA_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x800C0007_u32 as _);
pub const INET_E_DEFAULT_ACTION: i32 = -2146697199i32;
pub const INET_E_DOMINJECTIONVALIDATION: windows_core::HRESULT = windows_core::HRESULT(0x800C001C_u32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_CSP: windows_core::HRESULT = windows_core::HRESULT(0x800C0508_u32 as _);
pub const INET_E_DOWNLOAD_BLOCKED_BY_INPRIVATE: windows_core::HRESULT = windows_core::HRESULT(0x800C0501_u32 as _);
pub const INET_E_DOWNLOAD_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x800C0008_u32 as _);
pub const INET_E_ERROR_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x800C0002_u32 as _);
pub const INET_E_ERROR_LAST: i32 = -2146695928i32;
pub const INET_E_FORBIDFRAMING: windows_core::HRESULT = windows_core::HRESULT(0x800C0503_u32 as _);
pub const INET_E_HSTS_CERTIFICATE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x800C001E_u32 as _);
pub const INET_E_INVALID_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0x800C0019_u32 as _);
pub const INET_E_INVALID_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x800C000C_u32 as _);
pub const INET_E_INVALID_URL: windows_core::HRESULT = windows_core::HRESULT(0x800C0002_u32 as _);
pub const INET_E_NO_SESSION: windows_core::HRESULT = windows_core::HRESULT(0x800C0003_u32 as _);
pub const INET_E_NO_VALID_MEDIA: windows_core::HRESULT = windows_core::HRESULT(0x800C000A_u32 as _);
pub const INET_E_OBJECT_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x800C0006_u32 as _);
pub const INET_E_QUERYOPTION_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x800C0013_u32 as _);
pub const INET_E_REDIRECTING: windows_core::HRESULT = windows_core::HRESULT(0x800C0014_u32 as _);
pub const INET_E_REDIRECT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x800C0014_u32 as _);
pub const INET_E_REDIRECT_TO_DIR: windows_core::HRESULT = windows_core::HRESULT(0x800C0015_u32 as _);
pub const INET_E_RESERVED_1: windows_core::HRESULT = windows_core::HRESULT(0x800C001A_u32 as _);
pub const INET_E_RESERVED_2: windows_core::HRESULT = windows_core::HRESULT(0x800C001F_u32 as _);
pub const INET_E_RESERVED_3: windows_core::HRESULT = windows_core::HRESULT(0x800C0020_u32 as _);
pub const INET_E_RESERVED_4: windows_core::HRESULT = windows_core::HRESULT(0x800C0021_u32 as _);
pub const INET_E_RESERVED_5: windows_core::HRESULT = windows_core::HRESULT(0x800C0022_u32 as _);
pub const INET_E_RESOURCE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x800C0005_u32 as _);
pub const INET_E_RESULT_DISPATCHED: windows_core::HRESULT = windows_core::HRESULT(0x800C0200_u32 as _);
pub const INET_E_SECURITY_PROBLEM: windows_core::HRESULT = windows_core::HRESULT(0x800C000E_u32 as _);
pub const INET_E_TERMINATED_BIND: windows_core::HRESULT = windows_core::HRESULT(0x800C0018_u32 as _);
pub const INET_E_UNKNOWN_PROTOCOL: windows_core::HRESULT = windows_core::HRESULT(0x800C000D_u32 as _);
pub const INET_E_USE_DEFAULT_PROTOCOLHANDLER: windows_core::HRESULT = windows_core::HRESULT(0x800C0011_u32 as _);
pub const INET_E_USE_DEFAULT_SETTING: windows_core::HRESULT = windows_core::HRESULT(0x800C0012_u32 as _);
pub const INET_E_USE_EXTEND_BINDING: windows_core::HRESULT = windows_core::HRESULT(0x800C0017_u32 as _);
pub const INET_E_VTAB_SWITCH_FORCE_ENGINE: windows_core::HRESULT = windows_core::HRESULT(0x800C001D_u32 as _);
pub const MAX_SIZE_SECURITY_ID: u32 = 512u32;
pub const MAX_ZONE_DESCRIPTION: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(200i32);
pub const MAX_ZONE_PATH: INET_ZONE_MANAGER_CONSTANTS = INET_ZONE_MANAGER_CONSTANTS(260i32);
pub const MIMETYPEPROP: MONIKERPROPERTY = MONIKERPROPERTY(0i32);
pub const MKSYS_URLMONIKER: u32 = 6u32;
pub const MK_S_ASYNCHRONOUS: windows_core::HRESULT = windows_core::HRESULT(0x401E8_u32 as _);
pub const MUTZ_ACCEPT_WILDCARD_SCHEME: u32 = 128u32;
pub const MUTZ_DONT_UNESCAPE: u32 = 2048u32;
pub const MUTZ_DONT_USE_CACHE: u32 = 4096u32;
pub const MUTZ_ENFORCERESTRICTED: u32 = 256u32;
pub const MUTZ_FORCE_INTRANET_FLAGS: u32 = 8192u32;
pub const MUTZ_IGNORE_ZONE_MAPPINGS: u32 = 16384u32;
pub const MUTZ_ISFILE: u32 = 2u32;
pub const MUTZ_NOSAVEDFILECHECK: u32 = 1u32;
pub const MUTZ_REQUIRESAVEDFILECHECK: u32 = 1024u32;
pub const MUTZ_RESERVED: u32 = 512u32;
pub const OIBDG_APARTMENTTHREADED: OIBDG_FLAGS = OIBDG_FLAGS(256i32);
pub const OIBDG_DATAONLY: OIBDG_FLAGS = OIBDG_FLAGS(4096i32);
pub const PARSE_ANCHOR: PARSEACTION = PARSEACTION(6i32);
pub const PARSE_CANONICALIZE: PARSEACTION = PARSEACTION(1i32);
pub const PARSE_DECODE_IS_ESCAPE: PARSEACTION = PARSEACTION(8i32);
pub const PARSE_DOCUMENT: PARSEACTION = PARSEACTION(5i32);
pub const PARSE_DOMAIN: PARSEACTION = PARSEACTION(15i32);
pub const PARSE_ENCODE_IS_UNESCAPE: PARSEACTION = PARSEACTION(7i32);
pub const PARSE_ESCAPE: PARSEACTION = PARSEACTION(18i32);
pub const PARSE_FRIENDLY: PARSEACTION = PARSEACTION(2i32);
pub const PARSE_LOCATION: PARSEACTION = PARSEACTION(16i32);
pub const PARSE_MIME: PARSEACTION = PARSEACTION(11i32);
pub const PARSE_PATH_FROM_URL: PARSEACTION = PARSEACTION(9i32);
pub const PARSE_ROOTDOCUMENT: PARSEACTION = PARSEACTION(4i32);
pub const PARSE_SCHEMA: PARSEACTION = PARSEACTION(13i32);
pub const PARSE_SECURITY_DOMAIN: PARSEACTION = PARSEACTION(17i32);
pub const PARSE_SECURITY_URL: PARSEACTION = PARSEACTION(3i32);
pub const PARSE_SERVER: PARSEACTION = PARSEACTION(12i32);
pub const PARSE_SITE: PARSEACTION = PARSEACTION(14i32);
pub const PARSE_UNESCAPE: PARSEACTION = PARSEACTION(19i32);
pub const PARSE_URL_FROM_PATH: PARSEACTION = PARSEACTION(10i32);
pub const PD_FORCE_SWITCH: PI_FLAGS = PI_FLAGS(65536i32);
pub const PI_APARTMENTTHREADED: PI_FLAGS = PI_FLAGS(256i32);
pub const PI_CLASSINSTALL: PI_FLAGS = PI_FLAGS(512i32);
pub const PI_CLSIDLOOKUP: PI_FLAGS = PI_FLAGS(32i32);
pub const PI_DATAPROGRESS: PI_FLAGS = PI_FLAGS(64i32);
pub const PI_FILTER_MODE: PI_FLAGS = PI_FLAGS(2i32);
pub const PI_FORCE_ASYNC: PI_FLAGS = PI_FLAGS(4i32);
pub const PI_LOADAPPDIRECT: PI_FLAGS = PI_FLAGS(16384i32);
pub const PI_MIMEVERIFICATION: PI_FLAGS = PI_FLAGS(16i32);
pub const PI_NOMIMEHANDLER: PI_FLAGS = PI_FLAGS(32768i32);
pub const PI_PARSE_URL: PI_FLAGS = PI_FLAGS(1i32);
pub const PI_PASSONBINDCTX: PI_FLAGS = PI_FLAGS(8192i32);
pub const PI_PREFERDEFAULTHANDLER: PI_FLAGS = PI_FLAGS(131072i32);
pub const PI_SYNCHRONOUS: PI_FLAGS = PI_FLAGS(128i32);
pub const PI_USE_WORKERTHREAD: PI_FLAGS = PI_FLAGS(8i32);
pub const POPUPLEVELPROP: MONIKERPROPERTY = MONIKERPROPERTY(4i32);
pub const PROTOCOLFLAG_NO_PICS_CHECK: u32 = 1u32;
pub const PSU_DEFAULT: PSUACTION = PSUACTION(1i32);
pub const PSU_SECURITY_URL_ONLY: PSUACTION = PSUACTION(2i32);
pub const PUAFOUT_DEFAULT: PUAFOUT = PUAFOUT(0i32);
pub const PUAFOUT_ISLOCKZONEPOLICY: PUAFOUT = PUAFOUT(1i32);
pub const PUAF_ACCEPT_WILDCARD_SCHEME: PUAF = PUAF(128i32);
pub const PUAF_CHECK_TIFS: PUAF = PUAF(16i32);
pub const PUAF_DEFAULT: PUAF = PUAF(0i32);
pub const PUAF_DEFAULTZONEPOL: PUAF = PUAF(262144i32);
pub const PUAF_DONTCHECKBOXINDIALOG: PUAF = PUAF(32i32);
pub const PUAF_DONT_USE_CACHE: PUAF = PUAF(4096i32);
pub const PUAF_DRAGPROTOCOLCHECK: PUAF = PUAF(2097152i32);
pub const PUAF_ENFORCERESTRICTED: PUAF = PUAF(256i32);
pub const PUAF_FORCEUI_FOREGROUND: PUAF = PUAF(8i32);
pub const PUAF_ISFILE: PUAF = PUAF(2i32);
pub const PUAF_LMZ_LOCKED: PUAF = PUAF(131072i32);
pub const PUAF_LMZ_UNLOCKED: PUAF = PUAF(65536i32);
pub const PUAF_NOSAVEDFILECHECK: PUAF = PUAF(512i32);
pub const PUAF_NOUI: PUAF = PUAF(1i32);
pub const PUAF_NOUIIFLOCKED: PUAF = PUAF(1048576i32);
pub const PUAF_NPL_USE_LOCKED_IF_RESTRICTED: PUAF = PUAF(524288i32);
pub const PUAF_REQUIRESAVEDFILECHECK: PUAF = PUAF(1024i32);
pub const PUAF_RESERVED1: PUAF = PUAF(8192i32);
pub const PUAF_RESERVED2: PUAF = PUAF(16384i32);
pub const PUAF_TRUSTED: PUAF = PUAF(64i32);
pub const PUAF_WARN_IF_DENIED: PUAF = PUAF(4i32);
pub const QUERY_CAN_NAVIGATE: QUERYOPTION = QUERYOPTION(7i32);
pub const QUERY_CONTENT_ENCODING: QUERYOPTION = QUERYOPTION(3i32);
pub const QUERY_CONTENT_TYPE: QUERYOPTION = QUERYOPTION(4i32);
pub const QUERY_EXPIRATION_DATE: QUERYOPTION = QUERYOPTION(1i32);
pub const QUERY_IS_CACHED: QUERYOPTION = QUERYOPTION(9i32);
pub const QUERY_IS_CACHED_AND_USABLE_OFFLINE: QUERYOPTION = QUERYOPTION(16i32);
pub const QUERY_IS_CACHED_OR_MAPPED: QUERYOPTION = QUERYOPTION(11i32);
pub const QUERY_IS_INSTALLEDENTRY: QUERYOPTION = QUERYOPTION(10i32);
pub const QUERY_IS_SAFE: QUERYOPTION = QUERYOPTION(14i32);
pub const QUERY_IS_SECURE: QUERYOPTION = QUERYOPTION(13i32);
pub const QUERY_RECOMBINE: QUERYOPTION = QUERYOPTION(6i32);
pub const QUERY_REFRESH: QUERYOPTION = QUERYOPTION(5i32);
pub const QUERY_TIME_OF_LAST_CHANGE: QUERYOPTION = QUERYOPTION(2i32);
pub const QUERY_USES_CACHE: QUERYOPTION = QUERYOPTION(12i32);
pub const QUERY_USES_HISTORYFOLDER: QUERYOPTION = QUERYOPTION(15i32);
pub const QUERY_USES_NETWORK: QUERYOPTION = QUERYOPTION(8i32);
pub const SECURITY_IE_STATE_GREEN: u32 = 0u32;
pub const SECURITY_IE_STATE_RED: u32 = 1u32;
pub const SET_FEATURE_IN_REGISTRY: u32 = 4u32;
pub const SET_FEATURE_ON_PROCESS: u32 = 2u32;
pub const SET_FEATURE_ON_THREAD: u32 = 1u32;
pub const SET_FEATURE_ON_THREAD_INTERNET: u32 = 64u32;
pub const SET_FEATURE_ON_THREAD_INTRANET: u32 = 16u32;
pub const SET_FEATURE_ON_THREAD_LOCALMACHINE: u32 = 8u32;
pub const SET_FEATURE_ON_THREAD_RESTRICTED: u32 = 128u32;
pub const SET_FEATURE_ON_THREAD_TRUSTED: u32 = 32u32;
pub const SOFTDIST_ADSTATE_AVAILABLE: u32 = 1u32;
pub const SOFTDIST_ADSTATE_DOWNLOADED: u32 = 2u32;
pub const SOFTDIST_ADSTATE_INSTALLED: u32 = 3u32;
pub const SOFTDIST_ADSTATE_NONE: u32 = 0u32;
pub const SOFTDIST_FLAG_DELETE_SUBSCRIPTION: u32 = 8u32;
pub const SOFTDIST_FLAG_USAGE_AUTOINSTALL: u32 = 4u32;
pub const SOFTDIST_FLAG_USAGE_EMAIL: u32 = 1u32;
pub const SOFTDIST_FLAG_USAGE_PRECACHE: u32 = 2u32;
pub const SZM_CREATE: SZM_FLAGS = SZM_FLAGS(0i32);
pub const SZM_DELETE: SZM_FLAGS = SZM_FLAGS(1i32);
pub const S_ASYNCHRONOUS: i32 = 262632i32;
pub const TRUSTEDDOWNLOADPROP: MONIKERPROPERTY = MONIKERPROPERTY(3i32);
pub const UAS_EXACTLEGACY: u32 = 4096u32;
pub const URLACTION_ACTIVEX_ALLOW_TDC: u32 = 4620u32;
pub const URLACTION_ACTIVEX_CONFIRM_NOOBJECTSAFETY: u32 = 4612u32;
pub const URLACTION_ACTIVEX_CURR_MAX: u32 = 4620u32;
pub const URLACTION_ACTIVEX_DYNSRC_VIDEO_AND_ANIMATION: u32 = 4618u32;
pub const URLACTION_ACTIVEX_MAX: u32 = 5119u32;
pub const URLACTION_ACTIVEX_MIN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_NO_WEBOC_SCRIPT: u32 = 4614u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DATA_SAFETY: u32 = 4610u32;
pub const URLACTION_ACTIVEX_OVERRIDE_DOMAINLIST: u32 = 4619u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OBJECT_SAFETY: u32 = 4609u32;
pub const URLACTION_ACTIVEX_OVERRIDE_OPTIN: u32 = 4616u32;
pub const URLACTION_ACTIVEX_OVERRIDE_REPURPOSEDETECTION: u32 = 4615u32;
pub const URLACTION_ACTIVEX_OVERRIDE_SCRIPT_SAFETY: u32 = 4611u32;
pub const URLACTION_ACTIVEX_RUN: u32 = 4608u32;
pub const URLACTION_ACTIVEX_SCRIPTLET_RUN: u32 = 4617u32;
pub const URLACTION_ACTIVEX_TREATASUNTRUSTED: u32 = 4613u32;
pub const URLACTION_ALLOW_ACTIVEX_FILTERING: u32 = 9986u32;
pub const URLACTION_ALLOW_ANTIMALWARE_SCANNING_OF_ACTIVEX: u32 = 9996u32;
pub const URLACTION_ALLOW_APEVALUATION: u32 = 8961u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO: u32 = 9985u32;
pub const URLACTION_ALLOW_AUDIO_VIDEO_PLUGINS: u32 = 9988u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_APPCACHE_MANIFEST: u32 = 9994u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_ACROSS_WINDOWS: u32 = 9993u32;
pub const URLACTION_ALLOW_CROSSDOMAIN_DROP_WITHIN_WINDOW: u32 = 9992u32;
pub const URLACTION_ALLOW_CSS_EXPRESSIONS: u32 = 9997u32;
pub const URLACTION_ALLOW_JSCRIPT_IE: u32 = 5133u32;
pub const URLACTION_ALLOW_RENDER_LEGACY_DXTFILTERS: u32 = 9995u32;
pub const URLACTION_ALLOW_RESTRICTEDPROTOCOLS: u32 = 8960u32;
pub const URLACTION_ALLOW_STRUCTURED_STORAGE_SNIFFING: u32 = 9987u32;
pub const URLACTION_ALLOW_VBSCRIPT_IE: u32 = 5132u32;
pub const URLACTION_ALLOW_XDOMAIN_SUBFRAME_RESIZE: u32 = 5128u32;
pub const URLACTION_ALLOW_XHR_EVALUATION: u32 = 8962u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_OPT_OUT_ADDITION: u32 = 9990u32;
pub const URLACTION_ALLOW_ZONE_ELEVATION_VIA_OPT_OUT: u32 = 9989u32;
pub const URLACTION_AUTHENTICATE_CLIENT: u32 = 6657u32;
pub const URLACTION_AUTOMATIC_ACTIVEX_UI: u32 = 8705u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI: u32 = 8704u32;
pub const URLACTION_AUTOMATIC_DOWNLOAD_UI_MIN: u32 = 8704u32;
pub const URLACTION_BEHAVIOR_MIN: u32 = 8192u32;
pub const URLACTION_BEHAVIOR_RUN: u32 = 8192u32;
pub const URLACTION_CHANNEL_SOFTDIST_MAX: u32 = 7935u32;
pub const URLACTION_CHANNEL_SOFTDIST_MIN: u32 = 7680u32;
pub const URLACTION_CHANNEL_SOFTDIST_PERMISSIONS: u32 = 7685u32;
pub const URLACTION_CLIENT_CERT_PROMPT: u32 = 6660u32;
pub const URLACTION_COOKIES: u32 = 6658u32;
pub const URLACTION_COOKIES_ENABLED: u32 = 6672u32;
pub const URLACTION_COOKIES_SESSION: u32 = 6659u32;
pub const URLACTION_COOKIES_SESSION_THIRD_PARTY: u32 = 6662u32;
pub const URLACTION_COOKIES_THIRD_PARTY: u32 = 6661u32;
pub const URLACTION_CREDENTIALS_USE: u32 = 6656u32;
pub const URLACTION_CROSS_DOMAIN_DATA: u32 = 5126u32;
pub const URLACTION_DOTNET_USERCONTROLS: u32 = 8197u32;
pub const URLACTION_DOWNLOAD_CURR_MAX: u32 = 4100u32;
pub const URLACTION_DOWNLOAD_MAX: u32 = 4607u32;
pub const URLACTION_DOWNLOAD_MIN: u32 = 4096u32;
pub const URLACTION_DOWNLOAD_SIGNED_ACTIVEX: u32 = 4097u32;
pub const URLACTION_DOWNLOAD_UNSIGNED_ACTIVEX: u32 = 4100u32;
pub const URLACTION_FEATURE_BLOCK_INPUT_PROMPTS: u32 = 8453u32;
pub const URLACTION_FEATURE_CROSSDOMAIN_FOCUS_CHANGE: u32 = 8455u32;
pub const URLACTION_FEATURE_DATA_BINDING: u32 = 8454u32;
pub const URLACTION_FEATURE_FORCE_ADDR_AND_STATUS: u32 = 8452u32;
pub const URLACTION_FEATURE_MIME_SNIFFING: u32 = 8448u32;
pub const URLACTION_FEATURE_MIN: u32 = 8448u32;
pub const URLACTION_FEATURE_SCRIPT_STATUS_BAR: u32 = 8451u32;
pub const URLACTION_FEATURE_WINDOW_RESTRICTIONS: u32 = 8450u32;
pub const URLACTION_FEATURE_ZONE_ELEVATION: u32 = 8449u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_CANVAS: u32 = 5645u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_TEXTTRACK: u32 = 5648u32;
pub const URLACTION_HTML_ALLOW_CROSS_DOMAIN_WEBWORKER: u32 = 5647u32;
pub const URLACTION_HTML_ALLOW_INDEXEDDB: u32 = 5649u32;
pub const URLACTION_HTML_ALLOW_INJECTED_DYNAMIC_HTML: u32 = 5643u32;
pub const URLACTION_HTML_ALLOW_WINDOW_CLOSE: u32 = 5646u32;
pub const URLACTION_HTML_FONT_DOWNLOAD: u32 = 5636u32;
pub const URLACTION_HTML_INCLUDE_FILE_PATH: u32 = 5642u32;
pub const URLACTION_HTML_JAVA_RUN: u32 = 5637u32;
pub const URLACTION_HTML_MAX: u32 = 6143u32;
pub const URLACTION_HTML_META_REFRESH: u32 = 5640u32;
pub const URLACTION_HTML_MIN: u32 = 5632u32;
pub const URLACTION_HTML_MIXED_CONTENT: u32 = 5641u32;
pub const URLACTION_HTML_REQUIRE_UTF8_DOCUMENT_CODEPAGE: u32 = 5644u32;
pub const URLACTION_HTML_SUBFRAME_NAVIGATE: u32 = 5639u32;
pub const URLACTION_HTML_SUBMIT_FORMS: u32 = 5633u32;
pub const URLACTION_HTML_SUBMIT_FORMS_FROM: u32 = 5634u32;
pub const URLACTION_HTML_SUBMIT_FORMS_TO: u32 = 5635u32;
pub const URLACTION_HTML_USERDATA_SAVE: u32 = 5638u32;
pub const URLACTION_INFODELIVERY_CURR_MAX: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_MAX: u32 = 7679u32;
pub const URLACTION_INFODELIVERY_MIN: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_CHANNELS: u32 = 7424u32;
pub const URLACTION_INFODELIVERY_NO_ADDING_SUBSCRIPTIONS: u32 = 7427u32;
pub const URLACTION_INFODELIVERY_NO_CHANNEL_LOGGING: u32 = 7430u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_CHANNELS: u32 = 7425u32;
pub const URLACTION_INFODELIVERY_NO_EDITING_SUBSCRIPTIONS: u32 = 7428u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_CHANNELS: u32 = 7426u32;
pub const URLACTION_INFODELIVERY_NO_REMOVING_SUBSCRIPTIONS: u32 = 7429u32;
pub const URLACTION_INPRIVATE_BLOCKING: u32 = 9984u32;
pub const URLACTION_JAVA_CURR_MAX: u32 = 7168u32;
pub const URLACTION_JAVA_MAX: u32 = 7423u32;
pub const URLACTION_JAVA_MIN: u32 = 7168u32;
pub const URLACTION_JAVA_PERMISSIONS: u32 = 7168u32;
pub const URLACTION_LOOSE_XAML: u32 = 9218u32;
pub const URLACTION_LOWRIGHTS: u32 = 9472u32;
pub const URLACTION_MIN: u32 = 4096u32;
pub const URLACTION_NETWORK_CURR_MAX: u32 = 6672u32;
pub const URLACTION_NETWORK_MAX: u32 = 7167u32;
pub const URLACTION_NETWORK_MIN: u32 = 6656u32;
pub const URLACTION_PLUGGABLE_PROTOCOL_XHR: u32 = 5131u32;
pub const URLACTION_SCRIPT_CURR_MAX: u32 = 5133u32;
pub const URLACTION_SCRIPT_JAVA_USE: u32 = 5122u32;
pub const URLACTION_SCRIPT_MAX: u32 = 5631u32;
pub const URLACTION_SCRIPT_MIN: u32 = 5120u32;
pub const URLACTION_SCRIPT_NAVIGATE: u32 = 5130u32;
pub const URLACTION_SCRIPT_OVERRIDE_SAFETY: u32 = 5121u32;
pub const URLACTION_SCRIPT_PASTE: u32 = 5127u32;
pub const URLACTION_SCRIPT_RUN: u32 = 5120u32;
pub const URLACTION_SCRIPT_SAFE_ACTIVEX: u32 = 5125u32;
pub const URLACTION_SCRIPT_XSSFILTER: u32 = 5129u32;
pub const URLACTION_SHELL_ALLOW_CROSS_SITE_SHARE: u32 = 6161u32;
pub const URLACTION_SHELL_CURR_MAX: u32 = 6162u32;
pub const URLACTION_SHELL_ENHANCED_DRAGDROP_SECURITY: u32 = 6155u32;
pub const URLACTION_SHELL_EXECUTE_HIGHRISK: u32 = 6150u32;
pub const URLACTION_SHELL_EXECUTE_LOWRISK: u32 = 6152u32;
pub const URLACTION_SHELL_EXECUTE_MODRISK: u32 = 6151u32;
pub const URLACTION_SHELL_EXTENSIONSECURITY: u32 = 6156u32;
pub const URLACTION_SHELL_FILE_DOWNLOAD: u32 = 6147u32;
pub const URLACTION_SHELL_INSTALL_DTITEMS: u32 = 6144u32;
pub const URLACTION_SHELL_MAX: u32 = 6655u32;
pub const URLACTION_SHELL_MIN: u32 = 6144u32;
pub const URLACTION_SHELL_MOVE_OR_COPY: u32 = 6146u32;
pub const URLACTION_SHELL_POPUPMGR: u32 = 6153u32;
pub const URLACTION_SHELL_PREVIEW: u32 = 6159u32;
pub const URLACTION_SHELL_REMOTEQUERY: u32 = 6158u32;
pub const URLACTION_SHELL_RTF_OBJECTS_LOAD: u32 = 6154u32;
pub const URLACTION_SHELL_SECURE_DRAGSOURCE: u32 = 6157u32;
pub const URLACTION_SHELL_SHARE: u32 = 6160u32;
pub const URLACTION_SHELL_SHELLEXECUTE: u32 = 6150u32;
pub const URLACTION_SHELL_TOCTOU_RISK: u32 = 6162u32;
pub const URLACTION_SHELL_VERB: u32 = 6148u32;
pub const URLACTION_SHELL_WEBVIEW_VERB: u32 = 6149u32;
pub const URLACTION_WINDOWS_BROWSER_APPLICATIONS: u32 = 9216u32;
pub const URLACTION_WINFX_SETUP: u32 = 9728u32;
pub const URLACTION_XPS_DOCUMENTS: u32 = 9217u32;
pub const URLMON_OPTION_URL_ENCODING: u32 = 268435460u32;
pub const URLMON_OPTION_USERAGENT: u32 = 268435457u32;
pub const URLMON_OPTION_USERAGENT_REFRESH: u32 = 268435458u32;
pub const URLMON_OPTION_USE_BINDSTRINGCREDS: u32 = 268435464u32;
pub const URLMON_OPTION_USE_BROWSERAPPSDOCUMENTS: u32 = 268435472u32;
pub const URLOSTRM_GETNEWESTVERSION: u32 = 3u32;
pub const URLOSTRM_USECACHEDCOPY: u32 = 2u32;
pub const URLOSTRM_USECACHEDCOPY_ONLY: u32 = 1u32;
pub const URLPOLICY_ACTIVEX_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_ALLOW: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_CHALLENGE_RESPONSE: u32 = 65536u32;
pub const URLPOLICY_AUTHENTICATE_CLEARTEXT_OK: u32 = 0u32;
pub const URLPOLICY_AUTHENTICATE_MUTUAL_ONLY: u32 = 196608u32;
pub const URLPOLICY_BEHAVIOR_CHECK_LIST: u32 = 65536u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_AUTOINSTALL: u32 = 196608u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PRECACHE: u32 = 131072u32;
pub const URLPOLICY_CHANNEL_SOFTDIST_PROHIBIT: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_ANONYMOUS_ONLY: u32 = 196608u32;
pub const URLPOLICY_CREDENTIALS_CONDITIONAL_PROMPT: u32 = 131072u32;
pub const URLPOLICY_CREDENTIALS_MUST_PROMPT_USER: u32 = 65536u32;
pub const URLPOLICY_CREDENTIALS_SILENT_LOGON_OK: u32 = 0u32;
pub const URLPOLICY_DISALLOW: u32 = 3u32;
pub const URLPOLICY_DONTCHECKDLGBOX: u32 = 256u32;
pub const URLPOLICY_JAVA_CUSTOM: u32 = 8388608u32;
pub const URLPOLICY_JAVA_HIGH: u32 = 65536u32;
pub const URLPOLICY_JAVA_LOW: u32 = 196608u32;
pub const URLPOLICY_JAVA_MEDIUM: u32 = 131072u32;
pub const URLPOLICY_JAVA_PROHIBIT: u32 = 0u32;
pub const URLPOLICY_LOG_ON_ALLOW: u32 = 64u32;
pub const URLPOLICY_LOG_ON_DISALLOW: u32 = 128u32;
pub const URLPOLICY_MASK_PERMISSIONS: u32 = 15u32;
pub const URLPOLICY_NOTIFY_ON_ALLOW: u32 = 16u32;
pub const URLPOLICY_NOTIFY_ON_DISALLOW: u32 = 32u32;
pub const URLPOLICY_QUERY: u32 = 1u32;
pub const URLTEMPLATE_CUSTOM: URLTEMPLATE = URLTEMPLATE(0i32);
pub const URLTEMPLATE_HIGH: URLTEMPLATE = URLTEMPLATE(73728i32);
pub const URLTEMPLATE_LOW: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLTEMPLATE_MEDHIGH: URLTEMPLATE = URLTEMPLATE(70912i32);
pub const URLTEMPLATE_MEDIUM: URLTEMPLATE = URLTEMPLATE(69632i32);
pub const URLTEMPLATE_MEDLOW: URLTEMPLATE = URLTEMPLATE(66816i32);
pub const URLTEMPLATE_PREDEFINED_MAX: URLTEMPLATE = URLTEMPLATE(131072i32);
pub const URLTEMPLATE_PREDEFINED_MIN: URLTEMPLATE = URLTEMPLATE(65536i32);
pub const URLZONEREG_DEFAULT: URLZONEREG = URLZONEREG(0i32);
pub const URLZONEREG_HKCU: URLZONEREG = URLZONEREG(2i32);
pub const URLZONEREG_HKLM: URLZONEREG = URLZONEREG(1i32);
pub const URLZONE_ESC_FLAG: u32 = 256u32;
pub const URLZONE_INTERNET: URLZONE = URLZONE(3i32);
pub const URLZONE_INTRANET: URLZONE = URLZONE(1i32);
pub const URLZONE_INVALID: URLZONE = URLZONE(-1i32);
pub const URLZONE_LOCAL_MACHINE: URLZONE = URLZONE(0i32);
pub const URLZONE_PREDEFINED_MAX: URLZONE = URLZONE(999i32);
pub const URLZONE_PREDEFINED_MIN: URLZONE = URLZONE(0i32);
pub const URLZONE_TRUSTED: URLZONE = URLZONE(2i32);
pub const URLZONE_UNTRUSTED: URLZONE = URLZONE(4i32);
pub const URLZONE_USER_MAX: URLZONE = URLZONE(10000i32);
pub const URLZONE_USER_MIN: URLZONE = URLZONE(1000i32);
pub const URL_ENCODING_DISABLE_UTF8: URL_ENCODING = URL_ENCODING(536870912i32);
pub const URL_ENCODING_ENABLE_UTF8: URL_ENCODING = URL_ENCODING(268435456i32);
pub const URL_ENCODING_NONE: URL_ENCODING = URL_ENCODING(0i32);
pub const URL_MK_LEGACY: u32 = 0u32;
pub const URL_MK_NO_CANONICALIZE: u32 = 2u32;
pub const URL_MK_UNIFORM: u32 = 1u32;
pub const USE_SRC_URL: MONIKERPROPERTY = MONIKERPROPERTY(1i32);
pub const UriBuilder_USE_ORIGINAL_FLAGS: u32 = 1u32;
pub const Uri_DISPLAY_IDN_HOST: u32 = 4u32;
pub const Uri_DISPLAY_NO_FRAGMENT: u32 = 1u32;
pub const Uri_DISPLAY_NO_PUNYCODE: u32 = 8u32;
pub const Uri_ENCODING_HOST_IS_IDN: u32 = 4u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_CP: u32 = 16u32;
pub const Uri_ENCODING_HOST_IS_PERCENT_ENCODED_UTF8: u32 = 8u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_CP: u32 = 64u32;
pub const Uri_ENCODING_QUERY_AND_FRAGMENT_IS_PERCENT_ENCODED_UTF8: u32 = 32u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_CP: u32 = 2u32;
pub const Uri_ENCODING_USER_INFO_AND_PATH_IS_PERCENT_ENCODED_UTF8: u32 = 1u32;
pub const Uri_HOST_DNS: Uri_HOST_TYPE = Uri_HOST_TYPE(1i32);
pub const Uri_HOST_IDN: Uri_HOST_TYPE = Uri_HOST_TYPE(4i32);
pub const Uri_HOST_IPV4: Uri_HOST_TYPE = Uri_HOST_TYPE(2i32);
pub const Uri_HOST_IPV6: Uri_HOST_TYPE = Uri_HOST_TYPE(3i32);
pub const Uri_HOST_UNKNOWN: Uri_HOST_TYPE = Uri_HOST_TYPE(0i32);
pub const Uri_PUNYCODE_IDN_HOST: u32 = 2u32;
pub const WININETINFO_OPTION_LOCK_HANDLE: u32 = 65534u32;
pub const ZAFLAGS_ADD_SITES: ZAFLAGS = ZAFLAGS(2i32);
pub const ZAFLAGS_CUSTOM_EDIT: ZAFLAGS = ZAFLAGS(1i32);
pub const ZAFLAGS_DETECT_INTRANET: ZAFLAGS = ZAFLAGS(256i32);
pub const ZAFLAGS_INCLUDE_INTRANET_SITES: ZAFLAGS = ZAFLAGS(16i32);
pub const ZAFLAGS_INCLUDE_PROXY_OVERRIDE: ZAFLAGS = ZAFLAGS(8i32);
pub const ZAFLAGS_NO_CACHE: ZAFLAGS = ZAFLAGS(262144i32);
pub const ZAFLAGS_NO_UI: ZAFLAGS = ZAFLAGS(32i32);
pub const ZAFLAGS_REQUIRE_VERIFICATION: ZAFLAGS = ZAFLAGS(4i32);
pub const ZAFLAGS_SUPPORTS_VERIFICATION: ZAFLAGS = ZAFLAGS(64i32);
pub const ZAFLAGS_UNC_AS_INTRANET: ZAFLAGS = ZAFLAGS(128i32);
pub const ZAFLAGS_USE_LOCKED_ZONES: ZAFLAGS = ZAFLAGS(65536i32);
pub const ZAFLAGS_VERIFY_TEMPLATE_SETTINGS: ZAFLAGS = ZAFLAGS(131072i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AUTHENTICATEF(pub i32);
impl windows_core::TypeKind for AUTHENTICATEF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AUTHENTICATEF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AUTHENTICATEF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDF(pub i32);
impl windows_core::TypeKind for BINDF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDF2(pub i32);
impl windows_core::TypeKind for BINDF2 {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDF2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDF2").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDHANDLETYPES(pub i32);
impl windows_core::TypeKind for BINDHANDLETYPES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDHANDLETYPES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDHANDLETYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDINFO_OPTIONS(pub i32);
impl windows_core::TypeKind for BINDINFO_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDINFO_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDINFO_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDSTATUS(pub i32);
impl windows_core::TypeKind for BINDSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDSTRING(pub i32);
impl windows_core::TypeKind for BINDSTRING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDSTRING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDVERB(pub i32);
impl windows_core::TypeKind for BINDVERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDVERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDVERB").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BSCF(pub i32);
impl windows_core::TypeKind for BSCF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BSCF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BSCF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CIP_STATUS(pub i32);
impl windows_core::TypeKind for CIP_STATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CIP_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CIP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IEObjectType(pub i32);
impl windows_core::TypeKind for IEObjectType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IEObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IEObjectType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INET_ZONE_MANAGER_CONSTANTS(pub i32);
impl windows_core::TypeKind for INET_ZONE_MANAGER_CONSTANTS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INET_ZONE_MANAGER_CONSTANTS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INET_ZONE_MANAGER_CONSTANTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INTERNETFEATURELIST(pub i32);
impl windows_core::TypeKind for INTERNETFEATURELIST {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INTERNETFEATURELIST {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INTERNETFEATURELIST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MONIKERPROPERTY(pub i32);
impl windows_core::TypeKind for MONIKERPROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MONIKERPROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MONIKERPROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OIBDG_FLAGS(pub i32);
impl windows_core::TypeKind for OIBDG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OIBDG_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OIBDG_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PARSEACTION(pub i32);
impl windows_core::TypeKind for PARSEACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PARSEACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PARSEACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PI_FLAGS(pub i32);
impl windows_core::TypeKind for PI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PI_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PI_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PSUACTION(pub i32);
impl windows_core::TypeKind for PSUACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PSUACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PSUACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PUAF(pub i32);
impl windows_core::TypeKind for PUAF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PUAF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PUAF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PUAFOUT(pub i32);
impl windows_core::TypeKind for PUAFOUT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PUAFOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PUAFOUT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QUERYOPTION(pub i32);
impl windows_core::TypeKind for QUERYOPTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QUERYOPTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QUERYOPTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SZM_FLAGS(pub i32);
impl windows_core::TypeKind for SZM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SZM_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SZM_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URLTEMPLATE(pub i32);
impl windows_core::TypeKind for URLTEMPLATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URLTEMPLATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URLTEMPLATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URLZONE(pub i32);
impl windows_core::TypeKind for URLZONE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URLZONE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URLZONE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URLZONEREG(pub i32);
impl windows_core::TypeKind for URLZONEREG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URLZONEREG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URLZONEREG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URL_ENCODING(pub i32);
impl windows_core::TypeKind for URL_ENCODING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URL_ENCODING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URL_ENCODING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Uri_HOST_TYPE(pub i32);
impl windows_core::TypeKind for Uri_HOST_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Uri_HOST_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Uri_HOST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ZAFLAGS(pub i32);
impl windows_core::TypeKind for ZAFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ZAFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ZAFLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CODEBASEHOLD {
    pub cbSize: u32,
    pub szDistUnit: windows_core::PWSTR,
    pub szCodeBase: windows_core::PWSTR,
    pub dwVersionMS: u32,
    pub dwVersionLS: u32,
    pub dwStyle: u32,
}
impl windows_core::TypeKind for CODEBASEHOLD {
    type TypeKind = windows_core::CopyType;
}
impl Default for CODEBASEHOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CONFIRMSAFETY {
    pub clsid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwFlags: u32,
}
impl Clone for CONFIRMSAFETY {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CONFIRMSAFETY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONFIRMSAFETY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DATAINFO {
    pub ulTotalSize: u32,
    pub ulavrPacketSize: u32,
    pub ulConnectSpeed: u32,
    pub ulProcessorSpeed: u32,
}
impl windows_core::TypeKind for DATAINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DATAINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HIT_LOGGING_INFO {
    pub dwStructSize: u32,
    pub lpszLoggedUrlName: windows_core::PSTR,
    pub StartTime: super::super::super::Foundation::SYSTEMTIME,
    pub EndTime: super::super::super::Foundation::SYSTEMTIME,
    pub lpszExtendedInfo: windows_core::PSTR,
}
impl windows_core::TypeKind for HIT_LOGGING_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for HIT_LOGGING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROTOCOLDATA {
    pub grfFlags: u32,
    pub dwState: u32,
    pub pData: *mut core::ffi::c_void,
    pub cbData: u32,
}
impl windows_core::TypeKind for PROTOCOLDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROTOCOLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct PROTOCOLFILTERDATA {
    pub cbSize: u32,
    pub pProtocolSink: core::mem::ManuallyDrop<Option<IInternetProtocolSink>>,
    pub pProtocol: core::mem::ManuallyDrop<Option<IInternetProtocol>>,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwFilterFlags: u32,
}
impl Clone for PROTOCOLFILTERDATA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for PROTOCOLFILTERDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROTOCOLFILTERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROTOCOL_ARGUMENT {
    pub szMethod: windows_core::PCWSTR,
    pub szTargetUrl: windows_core::PCWSTR,
}
impl windows_core::TypeKind for PROTOCOL_ARGUMENT {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROTOCOL_ARGUMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REMSECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: u32,
    pub bInheritHandle: super::super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for REMSECURITY_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for REMSECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct RemBINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_core::PWSTR,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: REMSECURITY_ATTRIBUTES,
    pub iid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwReserved: u32,
}
impl Clone for RemBINDINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for RemBINDINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for RemBINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemFORMATETC {
    pub cfFormat: u32,
    pub ptd: u32,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl windows_core::TypeKind for RemFORMATETC {
    type TypeKind = windows_core::CopyType;
}
impl Default for RemFORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOFTDISTINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwAdState: u32,
    pub szTitle: windows_core::PWSTR,
    pub szAbstract: windows_core::PWSTR,
    pub szHREF: windows_core::PWSTR,
    pub dwInstalledVersionMS: u32,
    pub dwInstalledVersionLS: u32,
    pub dwUpdateVersionMS: u32,
    pub dwUpdateVersionLS: u32,
    pub dwAdvertisedVersionMS: u32,
    pub dwAdvertisedVersionLS: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for SOFTDISTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOFTDISTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct StartParam {
    pub iid: windows_core::GUID,
    pub pIBindCtx: core::mem::ManuallyDrop<Option<super::IBindCtx>>,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for StartParam {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for StartParam {
    type TypeKind = windows_core::CopyType;
}
impl Default for StartParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZONEATTRIBUTES {
    pub cbSize: u32,
    pub szDisplayName: [u16; 260],
    pub szDescription: [u16; 200],
    pub szIconPath: [u16; 260],
    pub dwTemplateMinLevel: u32,
    pub dwTemplateRecommended: u32,
    pub dwTemplateCurrentLevel: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for ZONEATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl Default for ZONEATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
