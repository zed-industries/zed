#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> windows_core::Result<()> {
    windows_targets::link!("imgutil.dll" "system" fn ComputeInvCMAP(prgbcolors : *const super::super::Graphics::Gdi:: RGBQUAD, ncolors : u32, pinvtable : *mut u8, cbtable : u32) -> windows_core::HRESULT);
    ComputeInvCMAP(prgbcolors, ncolors, pinvtable, cbtable).ok()
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateDDrawSurfaceOnDIB<P0>(hbmdib: P0) -> windows_core::Result<super::super::Graphics::DirectDraw::IDirectDrawSurface>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HBITMAP>,
{
    windows_targets::link!("imgutil.dll" "system" fn CreateDDrawSurfaceOnDIB(hbmdib : super::super::Graphics::Gdi:: HBITMAP, ppsurface : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDDrawSurfaceOnDIB(hbmdib.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateMIMEMap() -> windows_core::Result<IMapMIMEToCLSID> {
    windows_targets::link!("imgutil.dll" "system" fn CreateMIMEMap(ppmap : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateMIMEMap(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DecodeImage<P0, P1, P2>(pstream: P0, pmap: P1, peventsink: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<IMapMIMEToCLSID>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("imgutil.dll" "system" fn DecodeImage(pstream : * mut core::ffi::c_void, pmap : * mut core::ffi::c_void, peventsink : * mut core::ffi::c_void) -> windows_core::HRESULT);
    DecodeImage(pstream.param().abi(), pmap.param().abi(), peventsink.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DecodeImageEx<P0, P1, P2, P3>(pstream: P0, pmap: P1, peventsink: P2, pszmimetypeparam: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<IMapMIMEToCLSID>,
    P2: windows_core::Param<windows_core::IUnknown>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("imgutil.dll" "system" fn DecodeImageEx(pstream : * mut core::ffi::c_void, pmap : * mut core::ffi::c_void, peventsink : * mut core::ffi::c_void, pszmimetypeparam : windows_core::PCWSTR) -> windows_core::HRESULT);
    DecodeImageEx(pstream.param().abi(), pmap.param().abi(), peventsink.param().abi(), pszmimetypeparam.param().abi()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const windows_core::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> windows_core::Result<()> {
    windows_targets::link!("imgutil.dll" "system" fn DitherTo8(pdestbits : *mut u8, ndestpitch : i32, psrcbits : *mut u8, nsrcpitch : i32, bfidsrc : *const windows_core::GUID, prgbdestcolors : *mut super::super::Graphics::Gdi:: RGBQUAD, prgbsrccolors : *mut super::super::Graphics::Gdi:: RGBQUAD, pbdestinvmap : *mut u8, x : i32, y : i32, cx : i32, cy : i32, ldesttrans : i32, lsrctrans : i32) -> windows_core::HRESULT);
    DitherTo8(pdestbits, ndestpitch, psrcbits, nsrcpitch, bfidsrc, prgbdestcolors, prgbsrccolors, pbdestinvmap, x, y, cx, cy, ldesttrans, lsrctrans).ok()
}
#[inline]
pub unsafe fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("imgutil.dll" "system" fn GetMaxMIMEIDBytes(pnmaxbytes : *mut u32) -> windows_core::HRESULT);
    GetMaxMIMEIDBytes(pnmaxbytes).ok()
}
#[inline]
pub unsafe fn IEAssociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> windows_core::Result<()> {
    windows_targets::link!("ieframe.dll" "system" fn IEAssociateThreadWithTab(dwtabthreadid : u32, dwassociatedthreadid : u32) -> windows_core::HRESULT);
    IEAssociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok()
}
#[inline]
pub unsafe fn IECancelSaveFile<P0>(hstate: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ieframe.dll" "system" fn IECancelSaveFile(hstate : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    IECancelSaveFile(hstate.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IECreateDirectory<P0>(lppathname: P0, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IECreateDirectory(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: BOOL);
    IECreateDirectory(lppathname.param().abi(), lpsecurityattributes)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IECreateFile<P0, P1>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: P1) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ieframe.dll" "system" fn IECreateFile(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    IECreateFile(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes, dwcreationdisposition, dwflagsandattributes, htemplatefile.param().abi())
}
#[inline]
pub unsafe fn IEDeleteFile<P0>(lpfilename: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEDeleteFile(lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    IEDeleteFile(lpfilename.param().abi())
}
#[inline]
pub unsafe fn IEDisassociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> windows_core::Result<()> {
    windows_targets::link!("ieframe.dll" "system" fn IEDisassociateThreadWithTab(dwtabthreadid : u32, dwassociatedthreadid : u32) -> windows_core::HRESULT);
    IEDisassociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok()
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn IEFindFirstFile<P0>(lpfilename: P0, lpfindfiledata: *const super::super::Storage::FileSystem::WIN32_FIND_DATAA) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEFindFirstFile(lpfilename : windows_core::PCWSTR, lpfindfiledata : *const super::super::Storage::FileSystem:: WIN32_FIND_DATAA) -> super::super::Foundation:: HANDLE);
    IEFindFirstFile(lpfilename.param().abi(), lpfindfiledata)
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn IEGetFileAttributesEx<P0>(lpfilename: P0, finfolevelid: super::super::Storage::FileSystem::GET_FILEEX_INFO_LEVELS, lpfileinformation: *const core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEGetFileAttributesEx(lpfilename : windows_core::PCWSTR, finfolevelid : super::super::Storage::FileSystem:: GET_FILEEX_INFO_LEVELS, lpfileinformation : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    IEGetFileAttributesEx(lpfilename.param().abi(), finfolevelid, lpfileinformation)
}
#[inline]
pub unsafe fn IEGetProtectedModeCookie<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: windows_core::PWSTR, pcchcookiedata: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEGetProtectedModeCookie(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PWSTR, pcchcookiedata : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    IEGetProtectedModeCookie(lpszurl.param().abi(), lpszcookiename.param().abi(), core::mem::transmute(lpszcookiedata), pcchcookiedata, dwflags).ok()
}
#[inline]
pub unsafe fn IEGetWriteableFolderPath(clsidfolderid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("ieframe.dll" "system" fn IEGetWriteableFolderPath(clsidfolderid : *const windows_core::GUID, lppwstrpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IEGetWriteableFolderPath(clsidfolderid, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn IEGetWriteableLowHKCU() -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_targets::link!("ieframe.dll" "system" fn IEGetWriteableLowHKCU(phkey : *mut super::super::System::Registry:: HKEY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IEGetWriteableLowHKCU(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IEInPrivateFilteringEnabled() -> super::super::Foundation::BOOL {
    windows_targets::link!("ieframe.dll" "system" fn IEInPrivateFilteringEnabled() -> super::super::Foundation:: BOOL);
    IEInPrivateFilteringEnabled()
}
#[inline]
pub unsafe fn IEIsInPrivateBrowsing() -> super::super::Foundation::BOOL {
    windows_targets::link!("ieframe.dll" "system" fn IEIsInPrivateBrowsing() -> super::super::Foundation:: BOOL);
    IEIsInPrivateBrowsing()
}
#[inline]
pub unsafe fn IEIsProtectedModeProcess() -> windows_core::Result<super::super::Foundation::BOOL> {
    windows_targets::link!("ieframe.dll" "system" fn IEIsProtectedModeProcess(pbresult : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IEIsProtectedModeProcess(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IEIsProtectedModeURL<P0>(lpwstrurl: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEIsProtectedModeURL(lpwstrurl : windows_core::PCWSTR) -> windows_core::HRESULT);
    IEIsProtectedModeURL(lpwstrurl.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn IELaunchURL<P0>(lpwstrurl: P0, lpprocinfo: *mut super::super::System::Threading::PROCESS_INFORMATION, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IELaunchURL(lpwstrurl : windows_core::PCWSTR, lpprocinfo : *mut super::super::System::Threading:: PROCESS_INFORMATION, lpinfo : *const core::ffi::c_void) -> windows_core::HRESULT);
    IELaunchURL(lpwstrurl.param().abi(), lpprocinfo, core::mem::transmute(lpinfo.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn IEMoveFileEx<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEMoveFileEx(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, dwflags : u32) -> super::super::Foundation:: BOOL);
    IEMoveFileEx(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags)
}
#[inline]
pub unsafe fn IERefreshElevationPolicy() -> windows_core::Result<()> {
    windows_targets::link!("ieframe.dll" "system" fn IERefreshElevationPolicy() -> windows_core::HRESULT);
    IERefreshElevationPolicy().ok()
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn IERegCreateKeyEx<P0, P1>(lpsubkey: P0, reserved: u32, lpclass: P1, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IERegCreateKeyEx(lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32) -> windows_core::HRESULT);
    IERegCreateKeyEx(lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, core::mem::transmute(lpsecurityattributes.unwrap_or(std::ptr::null())), phkresult, lpdwdisposition).ok()
}
#[inline]
pub unsafe fn IERegSetValueEx<P0, P1>(lpsubkey: P0, lpvaluename: P1, reserved: u32, dwtype: u32, lpdata: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IERegSetValueEx(lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> windows_core::HRESULT);
    IERegSetValueEx(lpsubkey.param().abi(), lpvaluename.param().abi(), reserved, dwtype, core::mem::transmute(lpdata.as_ptr()), lpdata.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn IERegisterWritableRegistryKey<P0, P1>(guid: windows_core::GUID, lpsubkey: P0, fsubkeyallowed: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ieframe.dll" "system" fn IERegisterWritableRegistryKey(guid : windows_core::GUID, lpsubkey : windows_core::PCWSTR, fsubkeyallowed : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    IERegisterWritableRegistryKey(core::mem::transmute(guid), lpsubkey.param().abi(), fsubkeyallowed.param().abi()).ok()
}
#[inline]
pub unsafe fn IERegisterWritableRegistryValue<P0, P1>(guid: windows_core::GUID, lppath: P0, lpvaluename: P1, dwtype: u32, lpdata: Option<&[u8]>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IERegisterWritableRegistryValue(guid : windows_core::GUID, lppath : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbmaxdata : u32) -> windows_core::HRESULT);
    IERegisterWritableRegistryValue(core::mem::transmute(guid), lppath.param().abi(), lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok()
}
#[inline]
pub unsafe fn IERemoveDirectory<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IERemoveDirectory(lppathname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    IERemoveDirectory(lppathname.param().abi())
}
#[inline]
pub unsafe fn IESaveFile<P0, P1>(hstate: P0, lpwstrsourcefile: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IESaveFile(hstate : super::super::Foundation:: HANDLE, lpwstrsourcefile : windows_core::PCWSTR) -> windows_core::HRESULT);
    IESaveFile(hstate.param().abi(), lpwstrsourcefile.param().abi()).ok()
}
#[inline]
pub unsafe fn IESetProtectedModeCookie<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IESetProtectedModeCookie(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    IESetProtectedModeCookie(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags).ok()
}
#[inline]
pub unsafe fn IEShowOpenFileDialog<P0, P1, P2, P3>(hwnd: P0, lpwstrfilename: &mut [u16], lpwstrinitialdir: P1, lpwstrfilter: P2, lpwstrdefext: P3, dwfilterindex: u32, dwflags: u32, phfile: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEShowOpenFileDialog(hwnd : super::super::Foundation:: HWND, lpwstrfilename : windows_core::PWSTR, cchmaxfilename : u32, lpwstrinitialdir : windows_core::PCWSTR, lpwstrfilter : windows_core::PCWSTR, lpwstrdefext : windows_core::PCWSTR, dwfilterindex : u32, dwflags : u32, phfile : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    IEShowOpenFileDialog(hwnd.param().abi(), core::mem::transmute(lpwstrfilename.as_ptr()), lpwstrfilename.len().try_into().unwrap(), lpwstrinitialdir.param().abi(), lpwstrfilter.param().abi(), lpwstrdefext.param().abi(), dwfilterindex, dwflags, phfile).ok()
}
#[inline]
pub unsafe fn IEShowSaveFileDialog<P0, P1, P2, P3, P4>(hwnd: P0, lpwstrinitialfilename: P1, lpwstrinitialdir: P2, lpwstrfilter: P3, lpwstrdefext: P4, dwfilterindex: u32, dwflags: u32, lppwstrdestinationfilepath: *mut windows_core::PWSTR, phstate: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ieframe.dll" "system" fn IEShowSaveFileDialog(hwnd : super::super::Foundation:: HWND, lpwstrinitialfilename : windows_core::PCWSTR, lpwstrinitialdir : windows_core::PCWSTR, lpwstrfilter : windows_core::PCWSTR, lpwstrdefext : windows_core::PCWSTR, dwfilterindex : u32, dwflags : u32, lppwstrdestinationfilepath : *mut windows_core::PWSTR, phstate : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    IEShowSaveFileDialog(hwnd.param().abi(), lpwstrinitialfilename.param().abi(), lpwstrinitialdir.param().abi(), lpwstrfilter.param().abi(), lpwstrdefext.param().abi(), dwfilterindex, dwflags, lppwstrdestinationfilepath, phstate).ok()
}
#[inline]
pub unsafe fn IETrackingProtectionEnabled() -> super::super::Foundation::BOOL {
    windows_targets::link!("ieframe.dll" "system" fn IETrackingProtectionEnabled() -> super::super::Foundation:: BOOL);
    IETrackingProtectionEnabled()
}
#[inline]
pub unsafe fn IEUnregisterWritableRegistry(guid: windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ieframe.dll" "system" fn IEUnregisterWritableRegistry(guid : windows_core::GUID) -> windows_core::HRESULT);
    IEUnregisterWritableRegistry(core::mem::transmute(guid)).ok()
}
#[inline]
pub unsafe fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("imgutil.dll" "system" fn IdentifyMIMEType(pbbytes : *const u8, nbytes : u32, pnformat : *mut u32) -> windows_core::HRESULT);
    IdentifyMIMEType(pbbytes, nbytes, pnformat).ok()
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog<P0, P1, P2>(hdlg: P0, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingAccessDeniedDialog(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, pszcontentdescription : windows_core::PCSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingAccessDeniedDialog(hdlg.param().abi(), pszusername.param().abi(), pszcontentdescription.param().abi(), pratingdetails).ok()
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog2<P0, P1>(hdlg: P0, pszusername: P1, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingAccessDeniedDialog2(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingAccessDeniedDialog2(hdlg.param().abi(), pszusername.param().abi(), pratingdetails).ok()
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog2W<P0, P1>(hdlg: P0, pszusername: P1, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingAccessDeniedDialog2W(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingAccessDeniedDialog2W(hdlg.param().abi(), pszusername.param().abi(), pratingdetails).ok()
}
#[inline]
pub unsafe fn RatingAccessDeniedDialogW<P0, P1, P2>(hdlg: P0, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingAccessDeniedDialogW(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, pszcontentdescription : windows_core::PCWSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingAccessDeniedDialogW(hdlg.param().abi(), pszusername.param().abi(), pszcontentdescription.param().abi(), pratingdetails).ok()
}
#[inline]
pub unsafe fn RatingAddToApprovedSites<P0, P1, P2, P3, P4>(hdlg: P0, pbpasswordblob: &mut [u8], lpszurl: P1, falwaysnever: P2, fsitepage: P3, fapprovedsitesenforced: P4) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
    P4: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingAddToApprovedSites(hdlg : super::super::Foundation:: HWND, cbpasswordblob : u32, pbpasswordblob : *mut u8, lpszurl : windows_core::PCWSTR, falwaysnever : super::super::Foundation:: BOOL, fsitepage : super::super::Foundation:: BOOL, fapprovedsitesenforced : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RatingAddToApprovedSites(hdlg.param().abi(), pbpasswordblob.len().try_into().unwrap(), core::mem::transmute(pbpasswordblob.as_ptr()), lpszurl.param().abi(), falwaysnever.param().abi(), fsitepage.param().abi(), fapprovedsitesenforced.param().abi()).ok()
}
#[inline]
pub unsafe fn RatingCheckUserAccess<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: Option<&[u8]>, ppratingdetails: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingCheckUserAccess(pszusername : windows_core::PCSTR, pszurl : windows_core::PCSTR, pszratinginfo : windows_core::PCSTR, pdata : *const u8, cbdata : u32, ppratingdetails : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingCheckUserAccess(pszusername.param().abi(), pszurl.param().abi(), pszratinginfo.param().abi(), core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppratingdetails.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RatingCheckUserAccessW<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: Option<&[u8]>, ppratingdetails: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingCheckUserAccessW(pszusername : windows_core::PCWSTR, pszurl : windows_core::PCWSTR, pszratinginfo : windows_core::PCWSTR, pdata : *const u8, cbdata : u32, ppratingdetails : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    RatingCheckUserAccessW(pszusername.param().abi(), pszurl.param().abi(), pszratinginfo.param().abi(), core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(ppratingdetails.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RatingClickedOnPRFInternal<P0, P1, P2>(hwndowner: P0, param1: P1, lpszfilename: P2, nshow: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingClickedOnPRFInternal(hwndowner : super::super::Foundation:: HWND, param1 : super::super::Foundation:: HINSTANCE, lpszfilename : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    RatingClickedOnPRFInternal(hwndowner.param().abi(), param1.param().abi(), lpszfilename.param().abi(), nshow).ok()
}
#[inline]
pub unsafe fn RatingClickedOnRATInternal<P0, P1, P2>(hwndowner: P0, param1: P1, lpszfilename: P2, nshow: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingClickedOnRATInternal(hwndowner : super::super::Foundation:: HWND, param1 : super::super::Foundation:: HINSTANCE, lpszfilename : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    RatingClickedOnRATInternal(hwndowner.param().abi(), param1.param().abi(), lpszfilename.param().abi(), nshow).ok()
}
#[inline]
pub unsafe fn RatingEnable<P0, P1, P2>(hwndparent: P0, pszusername: P1, fenable: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingEnable(hwndparent : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, fenable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RatingEnable(hwndparent.param().abi(), pszusername.param().abi(), fenable.param().abi()).ok()
}
#[inline]
pub unsafe fn RatingEnableW<P0, P1, P2>(hwndparent: P0, pszusername: P1, fenable: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingEnableW(hwndparent : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, fenable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RatingEnableW(hwndparent.param().abi(), pszusername.param().abi(), fenable.param().abi()).ok()
}
#[inline]
pub unsafe fn RatingEnabledQuery() -> windows_core::Result<()> {
    windows_targets::link!("msrating.dll" "system" fn RatingEnabledQuery() -> windows_core::HRESULT);
    RatingEnabledQuery().ok()
}
#[inline]
pub unsafe fn RatingFreeDetails(pratingdetails: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("msrating.dll" "system" fn RatingFreeDetails(pratingdetails : *const core::ffi::c_void) -> windows_core::HRESULT);
    RatingFreeDetails(core::mem::transmute(pratingdetails.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn RatingInit() -> windows_core::Result<()> {
    windows_targets::link!("msrating.dll" "system" fn RatingInit() -> windows_core::HRESULT);
    RatingInit().ok()
}
#[inline]
pub unsafe fn RatingObtainCancel<P0>(hratingobtainquery: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingObtainCancel(hratingobtainquery : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RatingObtainCancel(hratingobtainquery.param().abi()).ok()
}
#[inline]
pub unsafe fn RatingObtainQuery<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingObtainQuery(psztargeturl : windows_core::PCSTR, dwuserdata : u32, fcallback : isize, phratingobtainquery : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RatingObtainQuery(psztargeturl.param().abi(), dwuserdata, fcallback, core::mem::transmute(phratingobtainquery.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RatingObtainQueryW<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingObtainQueryW(psztargeturl : windows_core::PCWSTR, dwuserdata : u32, fcallback : isize, phratingobtainquery : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RatingObtainQueryW(psztargeturl.param().abi(), dwuserdata, fcallback, core::mem::transmute(phratingobtainquery.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RatingSetupUI<P0, P1>(hdlg: P0, pszusername: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingSetupUI(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR) -> windows_core::HRESULT);
    RatingSetupUI(hdlg.param().abi(), pszusername.param().abi()).ok()
}
#[inline]
pub unsafe fn RatingSetupUIW<P0, P1>(hdlg: P0, pszusername: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msrating.dll" "system" fn RatingSetupUIW(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR) -> windows_core::HRESULT);
    RatingSetupUIW(hdlg.param().abi(), pszusername.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SniffStream<P0>(pinstream: P0, pnformat: *mut u32, ppoutstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("imgutil.dll" "system" fn SniffStream(pinstream : * mut core::ffi::c_void, pnformat : *mut u32, ppoutstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    SniffStream(pinstream.param().abi(), pnformat, core::mem::transmute(ppoutstream)).ok()
}
windows_core::imp::define_interface!(IActiveXUIHandlerSite, IActiveXUIHandlerSite_Vtbl, 0x30510853_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IActiveXUIHandlerSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite, windows_core::IUnknown);
impl IActiveXUIHandlerSite {
    pub unsafe fn CreateScrollableContextMenu(&self) -> windows_core::Result<IScrollableContextMenu> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateScrollableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PickFileAndGetResult<P0, P1>(&self, filepicker: P0, allowmultipleselections: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PickFileAndGetResult)(windows_core::Interface::as_raw(self), filepicker.param().abi(), allowmultipleselections.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IActiveXUIHandlerSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateScrollableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PickFileAndGetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveXUIHandlerSite2, IActiveXUIHandlerSite2_Vtbl, 0x7e3707b2_d087_4542_ac1f_a0d2fcd080fd);
impl core::ops::Deref for IActiveXUIHandlerSite2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite2, windows_core::IUnknown);
impl IActiveXUIHandlerSite2 {
    pub unsafe fn AddSuspensionExemption(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddSuspensionExemption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RemoveSuspensionExemption(&self, ullcookie: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveSuspensionExemption)(windows_core::Interface::as_raw(self), ullcookie).ok()
    }
}
#[repr(C)]
pub struct IActiveXUIHandlerSite2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSuspensionExemption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub RemoveSuspensionExemption: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActiveXUIHandlerSite3, IActiveXUIHandlerSite3_Vtbl, 0x7904009a_1238_47f4_901c_871375c34608);
impl core::ops::Deref for IActiveXUIHandlerSite3 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite3, windows_core::IUnknown);
impl IActiveXUIHandlerSite3 {
    pub unsafe fn MessageBoxW<P0, P1, P2>(&self, hwnd: P0, text: P1, caption: P2, r#type: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MessageBoxW)(windows_core::Interface::as_raw(self), hwnd.param().abi(), text.param().abi(), caption.param().abi(), r#type, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IActiveXUIHandlerSite3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MessageBoxW: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAnchorClick, IAnchorClick_Vtbl, 0x13d5413b_33b9_11d2_95a7_00c04f8ecb02);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAnchorClick {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAnchorClick, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAnchorClick {
    pub unsafe fn ProcOnClick(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcOnClick)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAnchorClick_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProcOnClick: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAudioSessionSite, IAudioSessionSite_Vtbl, 0xd7d8b684_d02d_4517_b6b7_19e3dfe29c45);
impl core::ops::Deref for IAudioSessionSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionSite, windows_core::IUnknown);
impl IAudioSessionSite {
    pub unsafe fn GetAudioSessionGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAudioSessionGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OnAudioStreamCreated<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnAudioStreamCreated)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok()
    }
    pub unsafe fn OnAudioStreamDestroyed<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnAudioStreamDestroyed)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAudioSessionSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAudioSessionGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OnAudioStreamCreated: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnAudioStreamDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICaretPositionProvider, ICaretPositionProvider_Vtbl, 0x58da43a2_108e_4d5b_9f75_e5f74f93fff5);
impl core::ops::Deref for ICaretPositionProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICaretPositionProvider, windows_core::IUnknown);
impl ICaretPositionProvider {
    pub unsafe fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCaretPosition)(windows_core::Interface::as_raw(self), pptcaret, pflheight).ok()
    }
}
#[repr(C)]
pub struct ICaretPositionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaretPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut f32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDeviceRect, IDeviceRect_Vtbl, 0x3050f6d5_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDeviceRect {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDeviceRect, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDeviceRect {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDeviceRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IDithererImpl, IDithererImpl_Vtbl, 0x7c48e840_3910_11d0_86fc_00a0c913f750);
impl core::ops::Deref for IDithererImpl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDithererImpl, windows_core::IUnknown);
impl IDithererImpl {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetDestColorTable)(windows_core::Interface::as_raw(self), ncolors, prgbcolors).ok()
    }
    pub unsafe fn SetEventSink<P0>(&self, peventsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImageDecodeEventSink>,
    {
        (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), peventsink.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDithererImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetDestColorTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetDestColorTable: usize,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDocObjectService, IDocObjectService_Vtbl, 0x3050f801_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IDocObjectService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDocObjectService, windows_core::IUnknown);
impl IDocObjectService {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireBeforeNavigate2<P0, P1, P2, P3, P4>(&self, pdispatch: P0, lpszurl: P1, dwflags: u32, lpszframename: P2, ppostdata: *const u8, cbpostdata: u32, lpszheaders: P3, fplaynavsound: P4) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireBeforeNavigate2)(windows_core::Interface::as_raw(self), pdispatch.param().abi(), lpszurl.param().abi(), dwflags, lpszframename.param().abi(), ppostdata, cbpostdata, lpszheaders.param().abi(), fplaynavsound.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn FireDownloadBegin(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireDownloadBegin)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireDownloadComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPendingUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPendingUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUrlSearchComponent(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUrlSearchComponent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsErrorUrl<P0>(&self, lpszurl: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsErrorUrl)(windows_core::Interface::as_raw(self), lpszurl.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IDocObjectService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FireBeforeNavigate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const u8, u32, windows_core::PCWSTR, super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireBeforeNavigate2: usize,
    FireNavigateComplete2: usize,
    pub FireDownloadBegin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    FireDocumentComplete: usize,
    UpdateDesktopComponent: usize,
    pub GetPendingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    ActiveElementChanged: usize,
    pub GetUrlSearchComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsErrorUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadBehavior, IDownloadBehavior_Vtbl, 0x3050f5bd_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadBehavior {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadBehavior, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadBehavior {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn startDownload<P0, P1>(&self, bstrurl: P0, pdispcallback: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).startDownload)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), pdispcallback.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadBehavior_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub startDownload: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    startDownload: usize,
}
windows_core::imp::define_interface!(IDownloadManager, IDownloadManager_Vtbl, 0x988934a4_064b_11d3_bb80_00104b35e7f9);
impl core::ops::Deref for IDownloadManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDownloadManager, windows_core::IUnknown);
impl IDownloadManager {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Download<P0, P1, P2, P3>(&self, pmk: P0, pbc: P1, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: P2, pszredir: P3, uicp: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IMoniker>,
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), dwbindverb, grfbindf, pbindinfo, pszheaders.param().abi(), pszredir.param().abi(), uicp).ok()
    }
}
#[repr(C)]
pub struct IDownloadManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, i32, *const super::super::System::Com::BINDINFO, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    Download: usize,
}
windows_core::imp::define_interface!(IEnumManagerFrames, IEnumManagerFrames_Vtbl, 0x3caa826a_9b1f_4a79_bc81_f0430ded1648);
impl core::ops::Deref for IEnumManagerFrames {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumManagerFrames, windows_core::IUnknown);
impl IEnumManagerFrames {
    pub unsafe fn Next(&self, ppwindows: &mut [*mut super::super::Foundation::HWND], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppwindows.len().try_into().unwrap(), core::mem::transmute(ppwindows.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumManagerFrames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumManagerFrames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::super::Foundation::HWND, *mut u32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumOpenServiceActivity, IEnumOpenServiceActivity_Vtbl, 0xa436d7d2_17c3_4ef4_a1e8_5c86faff26c0);
impl core::ops::Deref for IEnumOpenServiceActivity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumOpenServiceActivity, windows_core::IUnknown);
impl IEnumOpenServiceActivity {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOpenServiceActivity>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumOpenServiceActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumOpenServiceActivityCategory, IEnumOpenServiceActivityCategory_Vtbl, 0x33627a56_8c9a_4430_8fd1_b5f5c771afb6);
impl core::ops::Deref for IEnumOpenServiceActivityCategory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumOpenServiceActivityCategory, windows_core::IUnknown);
impl IEnumOpenServiceActivityCategory {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOpenServiceActivityCategory>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivityCategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumOpenServiceActivityCategory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSTATURL, IEnumSTATURL_Vtbl, 0x3c374a42_bae4_11cf_bf7d_00aa006946ee);
impl core::ops::Deref for IEnumSTATURL {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSTATURL, windows_core::IUnknown);
impl IEnumSTATURL {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATURL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFilter<P0>(&self, poszfilter: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), poszfilter.param().abi(), dwflags).ok()
    }
}
#[repr(C)]
pub struct IEnumSTATURL_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATURL, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExtensionValidation, IExtensionValidation_Vtbl, 0x7d33f73d_8525_4e0f_87db_830288baff44);
impl core::ops::Deref for IExtensionValidation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExtensionValidation, windows_core::IUnknown);
impl IExtensionValidation {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IExtensionValidation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Validate: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHTMLPersistData, IHTMLPersistData_Vtbl, 0x3050f4c5_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IHTMLPersistData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHTMLPersistData, windows_core::IUnknown);
impl IHTMLPersistData {
    pub unsafe fn save<P0>(&self, punk: P0, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).save)(windows_core::Interface::as_raw(self), punk.param().abi(), ltype, &mut result__).map(|| result__)
    }
    pub unsafe fn load<P0>(&self, punk: P0, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).load)(windows_core::Interface::as_raw(self), punk.param().abi(), ltype, &mut result__).map(|| result__)
    }
    pub unsafe fn queryType(&self, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).queryType)(windows_core::Interface::as_raw(self), ltype, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IHTMLPersistData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub queryType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IHTMLPersistDataOM, IHTMLPersistDataOM_Vtbl, 0x3050f4c0_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IHTMLPersistDataOM {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IHTMLPersistDataOM, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IHTMLPersistDataOM {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).XMLDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn getAttribute<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn setAttribute<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn removeAttribute<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IHTMLPersistDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub XMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    XMLDocument: usize,
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IHTMLUserDataOM, IHTMLUserDataOM_Vtbl, 0x3050f48f_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IHTMLUserDataOM {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IHTMLUserDataOM, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IHTMLUserDataOM {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).XMLDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn save<P0>(&self, strname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).save)(windows_core::Interface::as_raw(self), strname.param().abi()).ok()
    }
    pub unsafe fn load<P0>(&self, strname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).load)(windows_core::Interface::as_raw(self), strname.param().abi()).ok()
    }
    pub unsafe fn getAttribute<P0>(&self, name: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn setAttribute<P0, P1>(&self, name: P0, value: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()).ok()
    }
    pub unsafe fn removeAttribute<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    pub unsafe fn Setexpires<P0>(&self, bstr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Setexpires)(windows_core::Interface::as_raw(self), bstr.param().abi()).ok()
    }
    pub unsafe fn expires(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).expires)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IHTMLUserDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub XMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    XMLDocument: usize,
    pub save: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub load: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Setexpires: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub expires: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IHeaderFooter, IHeaderFooter_Vtbl, 0x3050f6ce_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IHeaderFooter {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IHeaderFooter, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter {
    pub unsafe fn htmlHead(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).htmlHead)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn htmlFoot(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).htmlFoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SettextHead<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SettextHead)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn textHead(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).textHead)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SettextFoot<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SettextFoot)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn textFoot(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).textFoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Setpage(&self, v: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Setpage)(windows_core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn page(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).page)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetpageTotal(&self, v: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetpageTotal)(windows_core::Interface::as_raw(self), v).ok()
    }
    pub unsafe fn pageTotal(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).pageTotal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetURL<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn URL(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).URL)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Settitle<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Settitle)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn title(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).title)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetdateShort<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetdateShort)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn dateShort(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).dateShort)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetdateLong<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetdateLong)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn dateLong(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).dateLong)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SettimeShort<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SettimeShort)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn timeShort(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).timeShort)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SettimeLong<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SettimeLong)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn timeLong(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).timeLong)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IHeaderFooter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub htmlHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub htmlFoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SettextHead: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub textHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SettextFoot: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub textFoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Setpage: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub page: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetpageTotal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub pageTotal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Settitle: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetdateShort: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub dateShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetdateLong: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub dateLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SettimeShort: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub timeShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SettimeLong: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub timeLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IHeaderFooter2, IHeaderFooter2_Vtbl, 0x305104a5_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IHeaderFooter2 {
    type Target = IHeaderFooter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IHeaderFooter2, windows_core::IUnknown, super::super::System::Com::IDispatch, IHeaderFooter);
#[cfg(feature = "Win32_System_Com")]
impl IHeaderFooter2 {
    pub unsafe fn Setfont<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Setfont)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn font(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).font)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IHeaderFooter2_Vtbl {
    pub base__: IHeaderFooter_Vtbl,
    pub Setfont: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IHomePage, IHomePage_Vtbl, 0x766bf2af_d650_11d1_9811_00c04fc31d2e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IHomePage {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IHomePage, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IHomePage {
    pub unsafe fn navigateHomePage(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).navigateHomePage)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn setHomePage<P0>(&self, bstrurl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).setHomePage)(windows_core::Interface::as_raw(self), bstrurl.param().abi()).ok()
    }
    pub unsafe fn isHomePage<P0>(&self, bstrurl: P0) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).isHomePage)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IHomePage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigateHomePage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub isHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHomePageSetting, IHomePageSetting_Vtbl, 0xfdfc244f_18fa_4ff2_b08e_1d618f3ffbe4);
impl core::ops::Deref for IHomePageSetting {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IHomePageSetting, windows_core::IUnknown);
impl IHomePageSetting {
    pub unsafe fn SetHomePage<P0, P1, P2>(&self, hwnd: P0, homepageuri: P1, brandingmessage: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHomePage)(windows_core::Interface::as_raw(self), hwnd.param().abi(), homepageuri.param().abi(), brandingmessage.param().abi()).ok()
    }
    pub unsafe fn IsHomePage<P0>(&self, uri: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsHomePage)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetHomePageToBrowserDefault(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHomePageToBrowserDefault)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IHomePageSetting_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetHomePageToBrowserDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIEWebDriverManager, IIEWebDriverManager_Vtbl, 0xbd1dc630_6590_4ca2_a293_6bc72b2438d8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIEWebDriverManager {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIEWebDriverManager, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverManager {
    pub unsafe fn ExecuteCommand<P0>(&self, command: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExecuteCommand)(windows_core::Interface::as_raw(self), command.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIEWebDriverManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExecuteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIEWebDriverSite, IIEWebDriverSite_Vtbl, 0xffb84444_453d_4fbc_9f9d_8db5c471ec75);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIEWebDriverSite {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIEWebDriverSite, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIEWebDriverSite {
    pub unsafe fn WindowOperation(&self, operationcode: u32, hwnd: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).WindowOperation)(windows_core::Interface::as_raw(self), operationcode, hwnd).ok()
    }
    pub unsafe fn DetachWebdriver<P0>(&self, punkwd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).DetachWebdriver)(windows_core::Interface::as_raw(self), punkwd.param().abi()).ok()
    }
    pub unsafe fn GetCapabilityValue<P0, P1>(&self, punkwd: P0, capname: P1) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCapabilityValue)(windows_core::Interface::as_raw(self), punkwd.param().abi(), capname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIEWebDriverSite_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WindowOperation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DetachWebdriver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapabilityValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageDecodeEventSink, IImageDecodeEventSink_Vtbl, 0xbaa342a0_2ded_11d0_86f4_00a0c913f750);
impl core::ops::Deref for IImageDecodeEventSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageDecodeEventSink, windows_core::IUnknown);
impl IImageDecodeEventSink {
    pub unsafe fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSurface)(windows_core::Interface::as_raw(self), nwidth, nheight, bfid, npasses, dwhints, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBeginDecode)(windows_core::Interface::as_raw(self), pdwevents, pnformats, ppformats).ok()
    }
    pub unsafe fn OnBitsComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBitsComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDecodeComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDecodeComplete)(windows_core::Interface::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn OnPalette(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnPalette)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), pbounds, bcomplete.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IImageDecodeEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBeginDecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OnBitsComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDecodeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnPalette: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageDecodeEventSink2, IImageDecodeEventSink2_Vtbl, 0x8ebd8a57_8a96_48c9_84a6_962e2db9c931);
impl core::ops::Deref for IImageDecodeEventSink2 {
    type Target = IImageDecodeEventSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageDecodeEventSink2, windows_core::IUnknown, IImageDecodeEventSink);
impl IImageDecodeEventSink2 {
    pub unsafe fn IsAlphaPremultRequired(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsAlphaPremultRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IImageDecodeEventSink2_Vtbl {
    pub base__: IImageDecodeEventSink_Vtbl,
    pub IsAlphaPremultRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageDecodeFilter, IImageDecodeFilter_Vtbl, 0xa3ccedf3_2de2_11d0_86f4_00a0c913f750);
impl core::ops::Deref for IImageDecodeFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageDecodeFilter, windows_core::IUnknown);
impl IImageDecodeFilter {
    pub unsafe fn Initialize<P0>(&self, peventsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImageDecodeEventSink>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), peventsink.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Process<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), hrstatus).ok()
    }
}
#[repr(C)]
pub struct IImageDecodeFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Process: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Process: usize,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IIntelliForms, IIntelliForms_Vtbl, 0x9b9f68e6_1aaa_11d2_bca5_00c04fd929db);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IIntelliForms {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IIntelliForms, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IIntelliForms {
    pub unsafe fn enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Setenabled<P0>(&self, bval: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).Setenabled)(windows_core::Interface::as_raw(self), bval.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IIntelliForms_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetExplorerManager, IInternetExplorerManager_Vtbl, 0xacc84351_04ff_44f9_b23f_655ed168c6d5);
impl core::ops::Deref for IInternetExplorerManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetExplorerManager, windows_core::IUnknown);
impl IInternetExplorerManager {
    pub unsafe fn CreateObject<P0, T>(&self, dwconfig: u32, pszurl: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), dwconfig, pszurl.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IInternetExplorerManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternetExplorerManager2, IInternetExplorerManager2_Vtbl, 0xdfbb5136_9259_4895_b4a7_c1934429919a);
impl core::ops::Deref for IInternetExplorerManager2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternetExplorerManager2, windows_core::IUnknown);
impl IInternetExplorerManager2 {
    pub unsafe fn EnumFrameWindows(&self) -> windows_core::Result<IEnumManagerFrames> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFrameWindows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IInternetExplorerManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumFrameWindows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ILayoutRect, ILayoutRect_Vtbl, 0x3050f665_98b5_11cf_bb82_00aa00bdce0b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ILayoutRect {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ILayoutRect, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ILayoutRect {
    pub unsafe fn SetnextRect<P0>(&self, bstrelementid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetnextRect)(windows_core::Interface::as_raw(self), bstrelementid.param().abi()).ok()
    }
    pub unsafe fn nextRect(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).nextRect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetcontentSrc<P0>(&self, varcontentsrc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetcontentSrc)(windows_core::Interface::as_raw(self), varcontentsrc.param().abi()).ok()
    }
    pub unsafe fn contentSrc(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).contentSrc)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SethonorPageBreaks<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SethonorPageBreaks)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn honorPageBreaks(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).honorPageBreaks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SethonorPageRules<P0>(&self, v: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SethonorPageRules)(windows_core::Interface::as_raw(self), v.param().abi()).ok()
    }
    pub unsafe fn honorPageRules(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).honorPageRules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetnextRectElement<P0>(&self, pelem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        (windows_core::Interface::vtable(self).SetnextRectElement)(windows_core::Interface::as_raw(self), pelem.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn nextRectElement(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).nextRectElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn contentDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).contentDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ILayoutRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetnextRect: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub nextRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetcontentSrc: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub contentSrc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SethonorPageBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub honorPageBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SethonorPageRules: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub honorPageRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetnextRectElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetnextRectElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub nextRectElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    nextRectElement: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub contentDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    contentDocument: usize,
}
windows_core::imp::define_interface!(IMapMIMEToCLSID, IMapMIMEToCLSID_Vtbl, 0xd9e89500_30fa_11d0_b724_00aa006c1a01);
impl core::ops::Deref for IMapMIMEToCLSID {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMapMIMEToCLSID, windows_core::IUnknown);
impl IMapMIMEToCLSID {
    pub unsafe fn EnableDefaultMappings<P0>(&self, benable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableDefaultMappings)(windows_core::Interface::as_raw(self), benable.param().abi()).ok()
    }
    pub unsafe fn MapMIMEToCLSID<P0>(&self, pszmimetype: P0, pclsid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).MapMIMEToCLSID)(windows_core::Interface::as_raw(self), pszmimetype.param().abi(), pclsid).ok()
    }
    pub unsafe fn SetMapping<P0>(&self, pszmimetype: P0, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetMapping)(windows_core::Interface::as_raw(self), pszmimetype.param().abi(), dwmapmode, clsid).ok()
    }
}
#[repr(C)]
pub struct IMapMIMEToCLSID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDefaultMappings: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub MapMIMEToCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaActivityNotifySite, IMediaActivityNotifySite_Vtbl, 0x8165cfef_179d_46c2_bc71_3fa726dc1f8d);
impl core::ops::Deref for IMediaActivityNotifySite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMediaActivityNotifySite, windows_core::IUnknown);
impl IMediaActivityNotifySite {
    pub unsafe fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMediaActivityStarted)(windows_core::Interface::as_raw(self), mediaactivitytype).ok()
    }
    pub unsafe fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMediaActivityStopped)(windows_core::Interface::as_raw(self), mediaactivitytype).ok()
    }
}
#[repr(C)]
pub struct IMediaActivityNotifySite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMediaActivityStarted: unsafe extern "system" fn(*mut core::ffi::c_void, MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT,
    pub OnMediaActivityStopped: unsafe extern "system" fn(*mut core::ffi::c_void, MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenService, IOpenService_Vtbl, 0xc2952ed1_6a89_4606_925f_1ed8b4be0630);
impl core::ops::Deref for IOpenService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenService, windows_core::IUnknown);
impl IOpenService {
    pub unsafe fn IsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetDefault<P0, P1>(&self, fdefault: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self), fdefault.param().abi(), hwnd.param().abi()).ok()
    }
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpenService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenServiceActivity, IOpenServiceActivity_Vtbl, 0x13645c88_221a_4905_8ed1_4f5112cfc108);
impl core::ops::Deref for IOpenServiceActivity {
    type Target = IOpenService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceActivity, windows_core::IUnknown, IOpenService);
impl IOpenServiceActivity {
    pub unsafe fn Execute<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi()).ok()
    }
    pub unsafe fn CanExecute<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanExecute)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanExecuteType)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
    }
    pub unsafe fn Preview<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        (windows_core::Interface::vtable(self).Preview)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi()).ok()
    }
    pub unsafe fn CanPreview<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanPreview)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanPreviewType)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
    }
    pub unsafe fn GetStatusText<P0>(&self, pinput: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetStatusText)(windows_core::Interface::as_raw(self), pinput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHomepageUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHomepageUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCategoryName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategoryName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon<P0>(&self, fsmallicon: P0) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), fsmallicon.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDescriptionFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescriptionFilePath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDownloadUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDownloadUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInstallUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInstallUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOpenServiceActivity_Vtbl {
    pub base__: IOpenService_Vtbl,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanExecuteType: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Preview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CanPreviewType: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHomepageUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetCategoryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetDescriptionFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDownloadUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetInstallUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenServiceActivityCategory, IOpenServiceActivityCategory_Vtbl, 0x850af9d6_7309_40b5_bdb8_786c106b2153);
impl core::ops::Deref for IOpenServiceActivityCategory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceActivityCategory, windows_core::IUnknown);
impl IOpenServiceActivityCategory {
    pub unsafe fn HasDefaultActivity(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasDefaultActivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDefaultActivity(&self) -> windows_core::Result<IOpenServiceActivity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultActivity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultActivity<P0, P1>(&self, pactivity: P0, hwnd: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenServiceActivity>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetDefaultActivity)(windows_core::Interface::as_raw(self), pactivity.param().abi(), hwnd.param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActivityEnumerator<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<IEnumOpenServiceActivity>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityEnumerator)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpenServiceActivityCategory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HasDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetActivityEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenServiceActivityInput, IOpenServiceActivityInput_Vtbl, 0x75cb4db9_6da0_4da3_83ce_422b6a433346);
impl core::ops::Deref for IOpenServiceActivityInput {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceActivityInput, windows_core::IUnknown);
impl IOpenServiceActivityInput {
    pub unsafe fn GetVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVariable)(windows_core::Interface::as_raw(self), pwzvariablename.param().abi(), pwzvariabletype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HasVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasVariable)(windows_core::Interface::as_raw(self), pwzvariablename.param().abi(), pwzvariabletype.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<OpenServiceActivityContentType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpenServiceActivityInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HasVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OpenServiceActivityContentType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenServiceActivityManager, IOpenServiceActivityManager_Vtbl, 0x8a2d0a9d_e920_4bdc_a291_d30f650bc4f1);
impl core::ops::Deref for IOpenServiceActivityManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceActivityManager, windows_core::IUnknown);
impl IOpenServiceActivityManager {
    pub unsafe fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> windows_core::Result<IEnumOpenServiceActivityCategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategoryEnumerator)(windows_core::Interface::as_raw(self), etype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActivityByID<P0>(&self, pwzactivityid: P0) -> windows_core::Result<IOpenServiceActivity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityByID)(windows_core::Interface::as_raw(self), pwzactivityid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetActivityByHomepageAndCategory<P0, P1>(&self, pwzhomepage: P0, pwzcategory: P1) -> windows_core::Result<IOpenServiceActivity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivityByHomepageAndCategory)(windows_core::Interface::as_raw(self), pwzhomepage.param().abi(), pwzcategory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVersionCookie(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersionCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpenServiceActivityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCategoryEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityByID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityByHomepageAndCategory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOpenServiceActivityOutputContext, IOpenServiceActivityOutputContext_Vtbl, 0xe289deab_f709_49a9_b99e_282364074571);
impl core::ops::Deref for IOpenServiceActivityOutputContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceActivityOutputContext, windows_core::IUnknown);
impl IOpenServiceActivityOutputContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Navigate<P0, P1, P2, P3>(&self, pwzuri: P0, pwzmethod: P1, pwzheaders: P2, ppostdata: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), pwzmethod.param().abi(), pwzheaders.param().abi(), ppostdata.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CanNavigate<P0, P1, P2, P3>(&self, pwzuri: P0, pwzmethod: P1, pwzheaders: P2, ppostdata: P3) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanNavigate)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), pwzmethod.param().abi(), pwzheaders.param().abi(), ppostdata.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOpenServiceActivityOutputContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Navigate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CanNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CanNavigate: usize,
}
windows_core::imp::define_interface!(IOpenServiceManager, IOpenServiceManager_Vtbl, 0x5664125f_4e10_4e90_98e4_e4513d955a14);
impl core::ops::Deref for IOpenServiceManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOpenServiceManager, windows_core::IUnknown);
impl IOpenServiceManager {
    pub unsafe fn InstallService<P0>(&self, pwzserviceurl: P0) -> windows_core::Result<IOpenService>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallService)(windows_core::Interface::as_raw(self), pwzserviceurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UninstallService<P0>(&self, pservice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenService>,
    {
        (windows_core::Interface::vtable(self).UninstallService)(windows_core::Interface::as_raw(self), pservice.param().abi()).ok()
    }
    pub unsafe fn GetServiceByID<P0>(&self, pwzid: P0) -> windows_core::Result<IOpenService>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetServiceByID)(windows_core::Interface::as_raw(self), pwzid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOpenServiceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InstallService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceByID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPeerFactory, IPeerFactory_Vtbl, 0x6663f9d3_b482_11d1_89c6_00c04fb6bfc4);
impl core::ops::Deref for IPeerFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPeerFactory, windows_core::IUnknown);
impl IPeerFactory {}
#[repr(C)]
pub struct IPeerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPersistHistory, IPersistHistory_Vtbl, 0x91a565c1_e38f_11d0_94bf_00a0c9055cbf);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPersistHistory {
    type Target = super::super::System::Com::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPersistHistory, windows_core::IUnknown, super::super::System::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl IPersistHistory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadHistory<P0, P1>(&self, pstream: P0, pbc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).LoadHistory)(windows_core::Interface::as_raw(self), pstream.param().abi(), pbc.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveHistory<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveHistory)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
    pub unsafe fn SetPositionCookie(&self, dwpositioncookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPositionCookie)(windows_core::Interface::as_raw(self), dwpositioncookie).ok()
    }
    pub unsafe fn GetPositionCookie(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPositionCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistHistory_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadHistory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveHistory: usize,
    pub SetPositionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPositionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskRequestFactory, IPrintTaskRequestFactory_Vtbl, 0xbb516745_8c34_4f8b_9605_684dcb144be5);
impl core::ops::Deref for IPrintTaskRequestFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTaskRequestFactory, windows_core::IUnknown);
impl IPrintTaskRequestFactory {
    pub unsafe fn CreatePrintTaskRequest<P0>(&self, pprinttaskrequesthandler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintTaskRequestHandler>,
    {
        (windows_core::Interface::vtable(self).CreatePrintTaskRequest)(windows_core::Interface::as_raw(self), pprinttaskrequesthandler.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPrintTaskRequestFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePrintTaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrintTaskRequestHandler, IPrintTaskRequestHandler_Vtbl, 0x191cd340_cf36_44ff_bd53_d1b701799d9b);
impl core::ops::Deref for IPrintTaskRequestHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrintTaskRequestHandler, windows_core::IUnknown);
impl IPrintTaskRequestHandler {
    pub unsafe fn HandlePrintTaskRequest<P0>(&self, pprinttaskrequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        (windows_core::Interface::vtable(self).HandlePrintTaskRequest)(windows_core::Interface::as_raw(self), pprinttaskrequest.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPrintTaskRequestHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandlePrintTaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollableContextMenu, IScrollableContextMenu_Vtbl, 0x30510854_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IScrollableContextMenu {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScrollableContextMenu, windows_core::IUnknown);
impl IScrollableContextMenu {
    pub unsafe fn AddItem<P0>(&self, itemtext: P0, cmdid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), itemtext.param().abi(), cmdid).ok()
    }
    pub unsafe fn ShowModal(&self, x: i32, y: i32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ShowModal)(windows_core::Interface::as_raw(self), x, y, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IScrollableContextMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub ShowModal: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollableContextMenu2, IScrollableContextMenu2_Vtbl, 0xf77e9056_8674_4936_924c_0e4a06fa634a);
impl core::ops::Deref for IScrollableContextMenu2 {
    type Target = IScrollableContextMenu;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IScrollableContextMenu2, windows_core::IUnknown, IScrollableContextMenu);
impl IScrollableContextMenu2 {
    pub unsafe fn AddSeparator(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddSeparator)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPlacement)(windows_core::Interface::as_raw(self), scmp).ok()
    }
}
#[repr(C)]
pub struct IScrollableContextMenu2_Vtbl {
    pub base__: IScrollableContextMenu_Vtbl,
    pub AddSeparator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISniffStream, ISniffStream_Vtbl, 0x4ef17940_30e0_11d0_b724_00aa006c1a01);
impl core::ops::Deref for ISniffStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISniffStream, windows_core::IUnknown);
impl ISniffStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Init<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok()
    }
    pub unsafe fn Peek(&self, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), pbuffer, nbytes, pnbytesread).ok()
    }
}
#[repr(C)]
pub struct ISniffStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Init: usize,
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISurfacePresenterFlip, ISurfacePresenterFlip_Vtbl, 0x30510848_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ISurfacePresenterFlip {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlip, windows_core::IUnknown);
impl ISurfacePresenterFlip {
    pub unsafe fn Present(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBuffer(&self, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), backbufferindex, riid, ppbuffer).ok()
    }
}
#[repr(C)]
pub struct ISurfacePresenterFlip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISurfacePresenterFlip2, ISurfacePresenterFlip2_Vtbl, 0x30510865_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ISurfacePresenterFlip2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlip2, windows_core::IUnknown);
impl ISurfacePresenterFlip2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), dxgirotation).ok()
    }
}
#[repr(C)]
pub struct ISurfacePresenterFlip2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
}
windows_core::imp::define_interface!(ISurfacePresenterFlipBuffer, ISurfacePresenterFlipBuffer_Vtbl, 0xe43f4a08_8bbc_4665_ac92_c55ce61fd7e7);
impl core::ops::Deref for ISurfacePresenterFlipBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlipBuffer, windows_core::IUnknown);
impl ISurfacePresenterFlipBuffer {
    pub unsafe fn BeginDraw(&self, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), riid, ppbuffer).ok()
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISurfacePresenterFlipBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetContainer, ITargetContainer_Vtbl, 0x7847ec01_2bec_11d0_82b4_00a0c90c29c5);
impl core::ops::Deref for ITargetContainer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetContainer, windows_core::IUnknown);
impl ITargetContainer {
    pub unsafe fn GetFrameUrl(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITargetContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
}
windows_core::imp::define_interface!(ITargetEmbedding, ITargetEmbedding_Vtbl, 0x548793c0_9e74_11cf_9655_00a0c9034923);
impl core::ops::Deref for ITargetEmbedding {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetEmbedding, windows_core::IUnknown);
impl ITargetEmbedding {
    pub unsafe fn GetTargetFrame(&self) -> windows_core::Result<ITargetFrame> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITargetEmbedding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFrame, ITargetFrame_Vtbl, 0xd5f78c80_5252_11cf_90fa_00aa0042106e);
impl core::ops::Deref for ITargetFrame {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetFrame, windows_core::IUnknown);
impl ITargetFrame {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFrameName)(windows_core::Interface::as_raw(self), pszframename.param().abi()).ok()
    }
    pub unsafe fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFrame<P0, P1>(&self, psztargetname: P0, ppunkcontextframe: P1, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFrame)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), ppunkcontextframe.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFrameSrc)(windows_core::Interface::as_raw(self), pszframesrc.param().abi()).ok()
    }
    pub unsafe fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameSrc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrameOptions)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFrameOptions(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrameMargins)(windows_core::Interface::as_raw(self), dwwidth, dwheight).ok()
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFrameMargins)(windows_core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
    pub unsafe fn RemoteNavigate(&self, puldata: &[u32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoteNavigate)(windows_core::Interface::as_raw(self), puldata.len().try_into().unwrap(), core::mem::transmute(puldata.as_ptr())).ok()
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChildFrameActivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok()
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChildFrameDeactivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITargetFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetParentFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFrame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFrameSrc: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFrameSrc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
    pub SetFrameOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFrameOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFrameMargins: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetFrameMargins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub RemoteNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32) -> windows_core::HRESULT,
    pub OnChildFrameActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnChildFrameDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFrame2, ITargetFrame2_Vtbl, 0x86d52e11_94a8_11d0_82af_00c04fd5ae38);
impl core::ops::Deref for ITargetFrame2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetFrame2, windows_core::IUnknown);
impl ITargetFrame2 {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFrameName)(windows_core::Interface::as_raw(self), pszframename.param().abi()).ok()
    }
    pub unsafe fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFrameSrc)(windows_core::Interface::as_raw(self), pszframesrc.param().abi()).ok()
    }
    pub unsafe fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameSrc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrameOptions)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFrameOptions(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFrameOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFrameMargins)(windows_core::Interface::as_raw(self), dwwidth, dwheight).ok()
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFrameMargins)(windows_core::Interface::as_raw(self), pdwwidth, pdwheight).ok()
    }
    pub unsafe fn FindFrame<P0>(&self, psztargetname: P0, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFrame)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTargetAlias<P0>(&self, psztargetname: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTargetAlias)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITargetFrame2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetParentFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFrameSrc: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetFrameSrc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
    pub SetFrameOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFrameOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFrameMargins: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetFrameMargins: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub FindFrame: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTargetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFramePriv, ITargetFramePriv_Vtbl, 0x9216e421_2bf5_11d0_82b4_00a0c90c29c5);
impl core::ops::Deref for ITargetFramePriv {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetFramePriv, windows_core::IUnknown);
impl ITargetFramePriv {
    pub unsafe fn FindFrameDownwards<P0>(&self, psztargetname: P0, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFrameDownwards)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindFrameInContext<P0, P1>(&self, psztargetname: P0, punkcontextframe: P1, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindFrameInContext)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), punkcontextframe.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChildFrameActivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok()
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnChildFrameDeactivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NavigateHack<P0, P1, P2, P3, P4>(&self, grfhlnf: u32, pbc: P0, pibsc: P1, psztargetname: P2, pszurl: P3, pszlocation: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IBindCtx>,
        P1: windows_core::Param<super::super::System::Com::IBindStatusCallback>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).NavigateHack)(windows_core::Interface::as_raw(self), grfhlnf, pbc.param().abi(), pibsc.param().abi(), psztargetname.param().abi(), pszurl.param().abi(), pszlocation.param().abi()).ok()
    }
    pub unsafe fn FindBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindBrowserByIndex)(windows_core::Interface::as_raw(self), dwid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITargetFramePriv_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFrameDownwards: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFrameInContext: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnChildFrameActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnChildFrameDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NavigateHack: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NavigateHack: usize,
    pub FindBrowserByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetFramePriv2, ITargetFramePriv2_Vtbl, 0xb2c867e6_69d6_46f2_a611_ded9a4bd7fef);
impl core::ops::Deref for ITargetFramePriv2 {
    type Target = ITargetFramePriv;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetFramePriv2, windows_core::IUnknown, ITargetFramePriv);
impl ITargetFramePriv2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AggregatedNavigation2<P0, P1, P2, P3, P4>(&self, grfhlnf: u32, pbc: P0, pibsc: P1, psztargetname: P2, puri: P3, pszlocation: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IBindCtx>,
        P1: windows_core::Param<super::super::System::Com::IBindStatusCallback>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::System::Com::IUri>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AggregatedNavigation2)(windows_core::Interface::as_raw(self), grfhlnf, pbc.param().abi(), pibsc.param().abi(), psztargetname.param().abi(), puri.param().abi(), pszlocation.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITargetFramePriv2_Vtbl {
    pub base__: ITargetFramePriv_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AggregatedNavigation2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AggregatedNavigation2: usize,
}
windows_core::imp::define_interface!(ITargetNotify, ITargetNotify_Vtbl, 0x863a99a0_21bc_11d0_82b4_00a0c90c29c5);
impl core::ops::Deref for ITargetNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetNotify, windows_core::IUnknown);
impl ITargetNotify {
    pub unsafe fn OnCreate<P0>(&self, punkdestination: P0, cbcookie: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnCreate)(windows_core::Interface::as_raw(self), punkdestination.param().abi(), cbcookie).ok()
    }
    pub unsafe fn OnReuse<P0>(&self, punkdestination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnReuse)(windows_core::Interface::as_raw(self), punkdestination.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITargetNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnReuse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITargetNotify2, ITargetNotify2_Vtbl, 0x3050f6b1_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITargetNotify2 {
    type Target = ITargetNotify;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITargetNotify2, windows_core::IUnknown, ITargetNotify);
impl ITargetNotify2 {
    pub unsafe fn GetOptionString(&self, pbstroptions: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOptionString)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstroptions)).ok()
    }
}
#[repr(C)]
pub struct ITargetNotify2_Vtbl {
    pub base__: ITargetNotify_Vtbl,
    pub GetOptionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimer, ITimer_Vtbl, 0x3050f360_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITimer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITimer, windows_core::IUnknown);
impl ITimer {
    pub unsafe fn Advise<P0, P1, P2, P3>(&self, vtimemin: P0, vtimemax: P1, vtimeinterval: P2, dwflags: u32, ptimersink: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
        P1: windows_core::Param<windows_core::VARIANT>,
        P2: windows_core::Param<windows_core::VARIANT>,
        P3: windows_core::Param<ITimerSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), vtimemin.param().abi(), vtimemax.param().abi(), vtimeinterval.param().abi(), dwflags, ptimersink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn Freeze<P0>(&self, ffreeze: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), ffreeze.param().abi()).ok()
    }
    pub unsafe fn GetTime(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITimer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::VARIANT>, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimerEx, ITimerEx_Vtbl, 0x30510414_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITimerEx {
    type Target = ITimer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITimerEx, windows_core::IUnknown, ITimer);
impl ITimerEx {
    pub unsafe fn SetMode(&self, dwmode: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), dwmode).ok()
    }
}
#[repr(C)]
pub struct ITimerEx_Vtbl {
    pub base__: ITimer_Vtbl,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimerService, ITimerService_Vtbl, 0x3050f35f_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITimerService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITimerService, windows_core::IUnknown);
impl ITimerService {
    pub unsafe fn CreateTimer<P0>(&self, preferencetimer: P0) -> windows_core::Result<ITimer>
    where
        P0: windows_core::Param<ITimer>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTimer)(windows_core::Interface::as_raw(self), preferencetimer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNamedTimer(&self, rguidname: *const windows_core::GUID) -> windows_core::Result<ITimer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNamedTimer)(windows_core::Interface::as_raw(self), rguidname, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetNamedTimerReference<P0>(&self, rguidname: *const windows_core::GUID, preferencetimer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITimer>,
    {
        (windows_core::Interface::vtable(self).SetNamedTimerReference)(windows_core::Interface::as_raw(self), rguidname, preferencetimer.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITimerService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamedTimerReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimerSink, ITimerSink_Vtbl, 0x3050f361_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITimerSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITimerSink, windows_core::IUnknown);
impl ITimerSink {
    pub unsafe fn OnTimer<P0>(&self, vtimeadvise: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).OnTimer)(windows_core::Interface::as_raw(self), vtimeadvise.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITimerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnTimer: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITridentTouchInput, ITridentTouchInput_Vtbl, 0x30510850_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITridentTouchInput {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITridentTouchInput, windows_core::IUnknown);
impl ITridentTouchInput {
    pub unsafe fn OnPointerMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnPointerMessage)(windows_core::Interface::as_raw(self), msg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITridentTouchInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPointerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITridentTouchInputSite, ITridentTouchInputSite_Vtbl, 0x30510849_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for ITridentTouchInputSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITridentTouchInputSite, windows_core::IUnknown);
impl ITridentTouchInputSite {
    pub unsafe fn ZoomToPoint(&self, x: i32, y: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ZoomToPoint)(windows_core::Interface::as_raw(self), x, y).ok()
    }
}
#[repr(C)]
pub struct ITridentTouchInputSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetManipulationMode: usize,
    pub ZoomToPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Ole")]
windows_core::imp::define_interface!(IUrlHistoryNotify, IUrlHistoryNotify_Vtbl, 0xbc40bec1_c493_11d0_831b_00c04fd5ae38);
#[cfg(feature = "Win32_System_Ole")]
impl core::ops::Deref for IUrlHistoryNotify {
    type Target = super::super::System::Ole::IOleCommandTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Ole")]
windows_core::imp::interface_hierarchy!(IUrlHistoryNotify, windows_core::IUnknown, super::super::System::Ole::IOleCommandTarget);
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryNotify {}
#[cfg(feature = "Win32_System_Ole")]
#[repr(C)]
pub struct IUrlHistoryNotify_Vtbl {
    pub base__: super::super::System::Ole::IOleCommandTarget_Vtbl,
}
windows_core::imp::define_interface!(IUrlHistoryStg, IUrlHistoryStg_Vtbl, 0x3c374a41_bae4_11cf_bf7d_00aa006946ee);
impl core::ops::Deref for IUrlHistoryStg {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlHistoryStg, windows_core::IUnknown);
impl IUrlHistoryStg {
    pub unsafe fn AddUrl<P0, P1>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), pocstitle.param().abi(), dwflags).ok()
    }
    pub unsafe fn DeleteUrl<P0>(&self, pocsurl: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), dwflags).ok()
    }
    pub unsafe fn QueryUrl<P0>(&self, pocsurl: P0, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).QueryUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), dwflags, lpstaturl).ok()
    }
    pub unsafe fn BindToObject<P0, T>(&self, pocsurl: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumUrls(&self) -> windows_core::Result<IEnumSTATURL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IUrlHistoryStg_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub QueryUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut STATURL) -> windows_core::HRESULT,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUrlHistoryStg2, IUrlHistoryStg2_Vtbl, 0xafa0dc11_c313_11d0_831a_00c04fd5ae38);
impl core::ops::Deref for IUrlHistoryStg2 {
    type Target = IUrlHistoryStg;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlHistoryStg2, windows_core::IUnknown, IUrlHistoryStg);
impl IUrlHistoryStg2 {
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn AddUrlAndNotify<P0, P1, P2, P3, P4>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32, fwritehistory: P2, poctnotify: P3, punkisfolder: P4) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
        P3: windows_core::Param<super::super::System::Ole::IOleCommandTarget>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).AddUrlAndNotify)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), pocstitle.param().abi(), dwflags, fwritehistory.param().abi(), poctnotify.param().abi(), punkisfolder.param().abi()).ok()
    }
    pub unsafe fn ClearHistory(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ClearHistory)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUrlHistoryStg2_Vtbl {
    pub base__: IUrlHistoryStg_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub AddUrlAndNotify: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, super::super::Foundation::BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    AddUrlAndNotify: usize,
    pub ClearHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewObjectPresentFlip, IViewObjectPresentFlip_Vtbl, 0x30510847_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IViewObjectPresentFlip {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlip, windows_core::IUnknown);
impl IViewObjectPresentFlip {
    pub unsafe fn NotifyRender<P0>(&self, frecreatepresenter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).NotifyRender)(windows_core::Interface::as_raw(self), frecreatepresenter.param().abi()).ok()
    }
    pub unsafe fn RenderObjectToBitmap<P0>(&self, pbitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RenderObjectToBitmap)(windows_core::Interface::as_raw(self), pbitmap.param().abi()).ok()
    }
    pub unsafe fn RenderObjectToSharedBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISurfacePresenterFlipBuffer>,
    {
        (windows_core::Interface::vtable(self).RenderObjectToSharedBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IViewObjectPresentFlip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyRender: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RenderObjectToBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RenderObjectToSharedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewObjectPresentFlip2, IViewObjectPresentFlip2_Vtbl, 0x30510856_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IViewObjectPresentFlip2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlip2, windows_core::IUnknown);
impl IViewObjectPresentFlip2 {
    pub unsafe fn NotifyLeavingView(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NotifyLeavingView)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IViewObjectPresentFlip2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyLeavingView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewObjectPresentFlipSite, IViewObjectPresentFlipSite_Vtbl, 0x30510846_98b5_11cf_bb82_00aa00bdce0b);
impl core::ops::Deref for IViewObjectPresentFlipSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlipSite, windows_core::IUnknown);
impl IViewObjectPresentFlipSite {
    pub unsafe fn GetDeviceLuid(&self) -> windows_core::Result<super::super::Foundation::LUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDeviceLuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnterFullScreen(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnterFullScreen)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExitFullScreen(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ExitFullScreen)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsFullScreen(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsFullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBoundingRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoundingRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetMetrics(&self, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), ppos, psize, pscalex, pscaley).ok()
    }
    pub unsafe fn GetFullScreenSize(&self) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFullScreenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IViewObjectPresentFlipSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateSurfacePresenterFlip: usize,
    pub GetDeviceLuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID) -> windows_core::HRESULT,
    pub EnterFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetBoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut super::super::Foundation::SIZE, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetFullScreenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewObjectPresentFlipSite2, IViewObjectPresentFlipSite2_Vtbl, 0xaad0cbf1_e7fd_4f12_8902_c78132a8e01d);
impl core::ops::Deref for IViewObjectPresentFlipSite2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlipSite2, windows_core::IUnknown);
impl IViewObjectPresentFlipSite2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotationForCurrentOutput(&self) -> windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRotationForCurrentOutput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IViewObjectPresentFlipSite2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotationForCurrentOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotationForCurrentOutput: usize,
}
windows_core::imp::define_interface!(IWebBrowserEventsService, IWebBrowserEventsService_Vtbl, 0x54a8f188_9ebd_4795_ad16_9b4945119636);
impl core::ops::Deref for IWebBrowserEventsService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebBrowserEventsService, windows_core::IUnknown);
impl IWebBrowserEventsService {
    pub unsafe fn FireBeforeNavigate2Event(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireBeforeNavigate2Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn FireNavigateComplete2Event(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireNavigateComplete2Event)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadBeginEvent(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireDownloadBeginEvent)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDownloadCompleteEvent(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireDownloadCompleteEvent)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FireDocumentCompleteEvent(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FireDocumentCompleteEvent)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWebBrowserEventsService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FireBeforeNavigate2Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FireNavigateComplete2Event: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadBeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDocumentCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebBrowserEventsUrlService, IWebBrowserEventsUrlService_Vtbl, 0x87cc5d04_eafa_4833_9820_8f986530cc00);
impl core::ops::Deref for IWebBrowserEventsUrlService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWebBrowserEventsUrlService, windows_core::IUnknown);
impl IWebBrowserEventsUrlService {
    pub unsafe fn GetUrlForEvents(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUrlForEvents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWebBrowserEventsUrlService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUrlForEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(Iwfolders, Iwfolders_Vtbl, 0xbae31f98_1b81_11d2_a97a_00c04f8ecb02);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for Iwfolders {
    type Target = super::super::System::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(Iwfolders, windows_core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl Iwfolders {
    pub unsafe fn navigate<P0>(&self, bstrurl: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).navigate)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn navigateFrame<P0, P1>(&self, bstrurl: P0, bstrtargetframe: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).navigateFrame)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), bstrtargetframe.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn navigateNoSite<P0, P1, P2>(&self, bstrurl: P0, bstrtargetframe: P1, dwhwnd: u32, pwb: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).navigateNoSite)(windows_core::Interface::as_raw(self), bstrurl.param().abi(), bstrtargetframe.param().abi(), dwhwnd, pwb.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct Iwfolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigate: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub navigateFrame: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub navigateNoSite: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ADDRESSBAND: u32 = 2u32;
pub const ADDURL_ADDTOCACHE: ADDURL_FLAG = ADDURL_FLAG(1i32);
pub const ADDURL_ADDTOHISTORYANDCACHE: ADDURL_FLAG = ADDURL_FLAG(0i32);
pub const ADDURL_FIRST: ADDURL_FLAG = ADDURL_FLAG(0i32);
pub const ADDURL_Max: ADDURL_FLAG = ADDURL_FLAG(2147483647i32);
pub const ActivityContentCount: OpenServiceActivityContentType = OpenServiceActivityContentType(3i32);
pub const ActivityContentDocument: OpenServiceActivityContentType = OpenServiceActivityContentType(0i32);
pub const ActivityContentLink: OpenServiceActivityContentType = OpenServiceActivityContentType(2i32);
pub const ActivityContentNone: OpenServiceActivityContentType = OpenServiceActivityContentType(-1i32);
pub const ActivityContentSelection: OpenServiceActivityContentType = OpenServiceActivityContentType(1i32);
pub const CATID_MSOfficeAntiVirus: windows_core::GUID = windows_core::GUID::from_u128(0x56ffcc30_d398_11d0_b2ae_00a0c908fa49);
pub const COLOR_NO_TRANSPARENT: u32 = 4294967295u32;
pub const DISPID_ACTIVEXFILTERINGENABLED: u32 = 61u32;
pub const DISPID_ADDCHANNEL: u32 = 5u32;
pub const DISPID_ADDDESKTOPCOMPONENT: u32 = 6u32;
pub const DISPID_ADDFAVORITE: u32 = 4u32;
pub const DISPID_ADDSEARCHPROVIDER: u32 = 14u32;
pub const DISPID_ADDSERVICE: u32 = 30u32;
pub const DISPID_ADDSITEMODE: u32 = 49u32;
pub const DISPID_ADDTHUMBNAILBUTTONS: u32 = 48u32;
pub const DISPID_ADDTOFAVORITESBAR: u32 = 32u32;
pub const DISPID_ADDTRACKINGPROTECTIONLIST: u32 = 57u32;
pub const DISPID_ADVANCEERROR: u32 = 10u32;
pub const DISPID_AMBIENT_OFFLINEIFNOTCONNECTED: i32 = -5501i32;
pub const DISPID_AMBIENT_SILENT: i32 = -5502i32;
pub const DISPID_AUTOCOMPLETEATTACH: u32 = 12u32;
pub const DISPID_AUTOCOMPLETESAVEFORM: u32 = 10u32;
pub const DISPID_AUTOSCAN: u32 = 11u32;
pub const DISPID_BEFORENAVIGATE: u32 = 100u32;
pub const DISPID_BEFORENAVIGATE2: u32 = 250u32;
pub const DISPID_BEFORESCRIPTEXECUTE: u32 = 290u32;
pub const DISPID_BRANDIMAGEURI: u32 = 20u32;
pub const DISPID_BUILDNEWTABPAGE: u32 = 33u32;
pub const DISPID_CANADVANCEERROR: u32 = 12u32;
pub const DISPID_CANRETREATERROR: u32 = 13u32;
pub const DISPID_CHANGEDEFAULTBROWSER: u32 = 68u32;
pub const DISPID_CLEARNOTIFICATION: u32 = 71u32;
pub const DISPID_CLEARSITEMODEICONOVERLAY: u32 = 45u32;
pub const DISPID_CLIENTTOHOSTWINDOW: u32 = 268u32;
pub const DISPID_COMMANDSTATECHANGE: u32 = 105u32;
pub const DISPID_CONTENTDISCOVERYRESET: u32 = 36u32;
pub const DISPID_COUNTVIEWTYPES: u32 = 22u32;
pub const DISPID_CREATESUBSCRIPTION: u32 = 11u32;
pub const DISPID_CUSTOMIZECLEARTYPE: u32 = 23u32;
pub const DISPID_CUSTOMIZESETTINGS: u32 = 17u32;
pub const DISPID_DEFAULTSEARCHPROVIDER: u32 = 26u32;
pub const DISPID_DELETESUBSCRIPTION: u32 = 12u32;
pub const DISPID_DEPTH: u32 = 17u32;
pub const DISPID_DIAGNOSECONNECTION: u32 = 22u32;
pub const DISPID_DIAGNOSECONNECTIONUILESS: u32 = 66u32;
pub const DISPID_DOCUMENTCOMPLETE: u32 = 259u32;
pub const DISPID_DOUBLECLICK: u32 = 3u32;
pub const DISPID_DOWNLOADBEGIN: u32 = 106u32;
pub const DISPID_DOWNLOADCOMPLETE: u32 = 104u32;
pub const DISPID_ENABLENOTIFICATIONQUEUE: u32 = 72u32;
pub const DISPID_ENABLENOTIFICATIONQUEUELARGE: u32 = 78u32;
pub const DISPID_ENABLENOTIFICATIONQUEUESQUARE: u32 = 76u32;
pub const DISPID_ENABLENOTIFICATIONQUEUEWIDE: u32 = 77u32;
pub const DISPID_ENABLESUGGESTEDSITES: u32 = 39u32;
pub const DISPID_ENUMOPTIONS: u32 = 14u32;
pub const DISPID_EXPAND: u32 = 25u32;
pub const DISPID_EXPORT: u32 = 7u32;
pub const DISPID_FAVSELECTIONCHANGE: u32 = 1u32;
pub const DISPID_FILEDOWNLOAD: u32 = 270u32;
pub const DISPID_FLAGS: u32 = 19u32;
pub const DISPID_FRAMEBEFORENAVIGATE: u32 = 200u32;
pub const DISPID_FRAMENAVIGATECOMPLETE: u32 = 201u32;
pub const DISPID_FRAMENEWWINDOW: u32 = 204u32;
pub const DISPID_GETALWAYSSHOWLOCKSTATE: u32 = 23u32;
pub const DISPID_GETCVLISTDATA: u32 = 93u32;
pub const DISPID_GETCVLISTLOCALDATA: u32 = 94u32;
pub const DISPID_GETDETAILSSTATE: u32 = 19u32;
pub const DISPID_GETEMIELISTDATA: u32 = 95u32;
pub const DISPID_GETEMIELISTLOCALDATA: u32 = 96u32;
pub const DISPID_GETERRORCHAR: u32 = 15u32;
pub const DISPID_GETERRORCODE: u32 = 16u32;
pub const DISPID_GETERRORLINE: u32 = 14u32;
pub const DISPID_GETERRORMSG: u32 = 17u32;
pub const DISPID_GETERRORURL: u32 = 18u32;
pub const DISPID_GETEXPERIMENTALFLAG: u32 = 85u32;
pub const DISPID_GETEXPERIMENTALVALUE: u32 = 87u32;
pub const DISPID_GETNEEDHVSIAUTOLAUNCHFLAG: u32 = 100u32;
pub const DISPID_GETNEEDIEAUTOLAUNCHFLAG: u32 = 89u32;
pub const DISPID_GETOSSKU: u32 = 103u32;
pub const DISPID_GETPERERRSTATE: u32 = 21u32;
pub const DISPID_HASNEEDHVSIAUTOLAUNCHFLAG: u32 = 102u32;
pub const DISPID_HASNEEDIEAUTOLAUNCHFLAG: u32 = 88u32;
pub const DISPID_IMPORT: u32 = 6u32;
pub const DISPID_IMPORTEXPORTFAVORITES: u32 = 9u32;
pub const DISPID_INITIALIZED: u32 = 4u32;
pub const DISPID_INPRIVATEFILTERINGENABLED: u32 = 37u32;
pub const DISPID_INVOKECONTEXTMENU: u32 = 8u32;
pub const DISPID_ISMETAREFERRERAVAILABLE: u32 = 83u32;
pub const DISPID_ISSEARCHMIGRATED: u32 = 25u32;
pub const DISPID_ISSEARCHPROVIDERINSTALLED: u32 = 24u32;
pub const DISPID_ISSERVICEINSTALLED: u32 = 31u32;
pub const DISPID_ISSITEMODE: u32 = 43u32;
pub const DISPID_ISSITEMODEFIRSTRUN: u32 = 59u32;
pub const DISPID_ISSUBSCRIBED: u32 = 7u32;
pub const DISPID_LAUNCHIE: u32 = 91u32;
pub const DISPID_LAUNCHINHVSI: u32 = 99u32;
pub const DISPID_LAUNCHINTERNETOPTIONS: u32 = 74u32;
pub const DISPID_LAUNCHNETWORKCLIENTHELP: u32 = 67u32;
pub const DISPID_MODE: u32 = 18u32;
pub const DISPID_MOVESELECTIONDOWN: u32 = 2u32;
pub const DISPID_MOVESELECTIONTO: u32 = 9u32;
pub const DISPID_MOVESELECTIONUP: u32 = 1u32;
pub const DISPID_NAVIGATEANDFIND: u32 = 8u32;
pub const DISPID_NAVIGATECOMPLETE: u32 = 101u32;
pub const DISPID_NAVIGATECOMPLETE2: u32 = 252u32;
pub const DISPID_NAVIGATEERROR: u32 = 271u32;
pub const DISPID_NAVIGATETOSUGGESTEDSITES: u32 = 40u32;
pub const DISPID_NEWFOLDER: u32 = 4u32;
pub const DISPID_NEWPROCESS: u32 = 284u32;
pub const DISPID_NEWWINDOW: u32 = 107u32;
pub const DISPID_NEWWINDOW2: u32 = 251u32;
pub const DISPID_NEWWINDOW3: u32 = 273u32;
pub const DISPID_NSCOLUMNS: u32 = 21u32;
pub const DISPID_ONADDRESSBAR: u32 = 261u32;
pub const DISPID_ONFULLSCREEN: u32 = 258u32;
pub const DISPID_ONMENUBAR: u32 = 256u32;
pub const DISPID_ONQUIT: u32 = 253u32;
pub const DISPID_ONSTATUSBAR: u32 = 257u32;
pub const DISPID_ONTHEATERMODE: u32 = 260u32;
pub const DISPID_ONTOOLBAR: u32 = 255u32;
pub const DISPID_ONVISIBLE: u32 = 254u32;
pub const DISPID_OPENFAVORITESPANE: u32 = 97u32;
pub const DISPID_OPENFAVORITESSETTINGS: u32 = 98u32;
pub const DISPID_PHISHINGENABLED: u32 = 19u32;
pub const DISPID_PINNEDSITESTATE: u32 = 73u32;
pub const DISPID_PRINTTEMPLATEINSTANTIATION: u32 = 225u32;
pub const DISPID_PRINTTEMPLATETEARDOWN: u32 = 226u32;
pub const DISPID_PRIVACYIMPACTEDSTATECHANGE: u32 = 272u32;
pub const DISPID_PROGRESSCHANGE: u32 = 108u32;
pub const DISPID_PROPERTYCHANGE: u32 = 112u32;
pub const DISPID_PROVISIONNETWORKS: u32 = 62u32;
pub const DISPID_QUIT: u32 = 103u32;
pub const DISPID_REDIRECTXDOMAINBLOCKED: u32 = 286u32;
pub const DISPID_REFRESHOFFLINEDESKTOP: u32 = 3u32;
pub const DISPID_REMOVESCHEDULEDTILENOTIFICATION: u32 = 80u32;
pub const DISPID_REPORTSAFEURL: u32 = 63u32;
pub const DISPID_RESETEXPERIMENTALFLAGS: u32 = 92u32;
pub const DISPID_RESETFIRSTBOOTMODE: u32 = 1u32;
pub const DISPID_RESETSAFEMODE: u32 = 2u32;
pub const DISPID_RESETSORT: u32 = 3u32;
pub const DISPID_RETREATERROR: u32 = 11u32;
pub const DISPID_ROOT: u32 = 16u32;
pub const DISPID_RUNONCEHASSHOWN: u32 = 28u32;
pub const DISPID_RUNONCEREQUIREDSETTINGSCOMPLETE: u32 = 27u32;
pub const DISPID_RUNONCESHOWN: u32 = 15u32;
pub const DISPID_SCHEDULEDTILENOTIFICATION: u32 = 79u32;
pub const DISPID_SEARCHGUIDEURL: u32 = 29u32;
pub const DISPID_SELECTEDITEM: u32 = 15u32;
pub const DISPID_SELECTEDITEMS: u32 = 24u32;
pub const DISPID_SELECTIONCHANGE: u32 = 2u32;
pub const DISPID_SETACTIVITIESVISIBLE: u32 = 35u32;
pub const DISPID_SETDETAILSSTATE: u32 = 20u32;
pub const DISPID_SETEXPERIMENTALFLAG: u32 = 84u32;
pub const DISPID_SETEXPERIMENTALVALUE: u32 = 86u32;
pub const DISPID_SETMSDEFAULTS: u32 = 104u32;
pub const DISPID_SETNEEDHVSIAUTOLAUNCHFLAG: u32 = 101u32;
pub const DISPID_SETNEEDIEAUTOLAUNCHFLAG: u32 = 90u32;
pub const DISPID_SETPERERRSTATE: u32 = 22u32;
pub const DISPID_SETPHISHINGFILTERSTATUS: u32 = 282u32;
pub const DISPID_SETRECENTLYCLOSEDVISIBLE: u32 = 34u32;
pub const DISPID_SETROOT: u32 = 13u32;
pub const DISPID_SETSECURELOCKICON: u32 = 269u32;
pub const DISPID_SETSITEMODEICONOVERLAY: u32 = 44u32;
pub const DISPID_SETSITEMODEPROPERTIES: u32 = 50u32;
pub const DISPID_SETTHUMBNAILBUTTONS: u32 = 47u32;
pub const DISPID_SETVIEWTYPE: u32 = 23u32;
pub const DISPID_SHELLUIHELPERLAST: u32 = 105u32;
pub const DISPID_SHOWBROWSERUI: u32 = 13u32;
pub const DISPID_SHOWINPRIVATEHELP: u32 = 42u32;
pub const DISPID_SHOWTABSHELP: u32 = 41u32;
pub const DISPID_SITEMODEACTIVATE: u32 = 58u32;
pub const DISPID_SITEMODEADDBUTTONSTYLE: u32 = 54u32;
pub const DISPID_SITEMODEADDJUMPLISTITEM: u32 = 52u32;
pub const DISPID_SITEMODECLEARBADGE: u32 = 65u32;
pub const DISPID_SITEMODECLEARJUMPLIST: u32 = 53u32;
pub const DISPID_SITEMODECREATEJUMPLIST: u32 = 51u32;
pub const DISPID_SITEMODEREFRESHBADGE: u32 = 64u32;
pub const DISPID_SITEMODESHOWBUTTONSTYLE: u32 = 55u32;
pub const DISPID_SITEMODESHOWJUMPLIST: u32 = 56u32;
pub const DISPID_SKIPRUNONCE: u32 = 16u32;
pub const DISPID_SKIPTABSWELCOME: u32 = 21u32;
pub const DISPID_SQMENABLED: u32 = 18u32;
pub const DISPID_STARTBADGEUPDATE: u32 = 81u32;
pub const DISPID_STARTPERIODICUPDATE: u32 = 70u32;
pub const DISPID_STARTPERIODICUPDATEBATCH: u32 = 75u32;
pub const DISPID_STATUSTEXTCHANGE: u32 = 102u32;
pub const DISPID_STOPBADGEUPDATE: u32 = 82u32;
pub const DISPID_STOPPERIODICUPDATE: u32 = 69u32;
pub const DISPID_SUBSCRIPTIONSENABLED: u32 = 10u32;
pub const DISPID_SUGGESTEDSITESENABLED: u32 = 38u32;
pub const DISPID_SYNCHRONIZE: u32 = 5u32;
pub const DISPID_THIRDPARTYURLBLOCKED: u32 = 285u32;
pub const DISPID_TITLECHANGE: u32 = 113u32;
pub const DISPID_TITLEICONCHANGE: u32 = 114u32;
pub const DISPID_TRACKINGPROTECTIONENABLED: u32 = 60u32;
pub const DISPID_TVFLAGS: u32 = 20u32;
pub const DISPID_UNSELECTALL: u32 = 26u32;
pub const DISPID_UPDATEPAGESTATUS: u32 = 227u32;
pub const DISPID_UPDATETHUMBNAILBUTTON: u32 = 46u32;
pub const DISPID_VIEWUPDATE: u32 = 281u32;
pub const DISPID_WEBWORKERFINISHED: u32 = 289u32;
pub const DISPID_WEBWORKERSTARTED: u32 = 288u32;
pub const DISPID_WINDOWACTIVATE: u32 = 111u32;
pub const DISPID_WINDOWCLOSING: u32 = 263u32;
pub const DISPID_WINDOWMOVE: u32 = 109u32;
pub const DISPID_WINDOWREGISTERED: u32 = 200u32;
pub const DISPID_WINDOWRESIZE: u32 = 110u32;
pub const DISPID_WINDOWREVOKED: u32 = 201u32;
pub const DISPID_WINDOWSETHEIGHT: u32 = 267u32;
pub const DISPID_WINDOWSETLEFT: u32 = 264u32;
pub const DISPID_WINDOWSETRESIZABLE: u32 = 262u32;
pub const DISPID_WINDOWSETTOP: u32 = 265u32;
pub const DISPID_WINDOWSETWIDTH: u32 = 266u32;
pub const DISPID_WINDOWSTATECHANGED: u32 = 283u32;
pub const E_SURFACE_DISCARDED: i32 = -2147434493i32;
pub const E_SURFACE_NODC: i32 = -2147434492i32;
pub const E_SURFACE_NOSURFACE: i32 = -2147434496i32;
pub const E_SURFACE_NOTMYDC: i32 = -2147434491i32;
pub const E_SURFACE_NOTMYPOINTER: i32 = -2147434494i32;
pub const E_SURFACE_UNKNOWN_FORMAT: i32 = -2147434495i32;
pub const ExtensionValidationContextDynamic: ExtensionValidationContexts = ExtensionValidationContexts(1i32);
pub const ExtensionValidationContextNone: ExtensionValidationContexts = ExtensionValidationContexts(0i32);
pub const ExtensionValidationContextParsed: ExtensionValidationContexts = ExtensionValidationContexts(2i32);
pub const ExtensionValidationResultArrestPageLoad: ExtensionValidationResults = ExtensionValidationResults(2i32);
pub const ExtensionValidationResultDoNotInstantiate: ExtensionValidationResults = ExtensionValidationResults(1i32);
pub const ExtensionValidationResultNone: ExtensionValidationResults = ExtensionValidationResults(0i32);
pub const FINDFRAME_INTERNAL: FINDFRAME_FLAGS = FINDFRAME_FLAGS(-2147483648i32);
pub const FINDFRAME_JUSTTESTEXISTENCE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(1i32);
pub const FINDFRAME_NONE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(0i32);
pub const FRAMEOPTIONS_BROWSERBAND: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(64i32);
pub const FRAMEOPTIONS_DESKTOP: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(32i32);
pub const FRAMEOPTIONS_NO3DBORDER: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(16i32);
pub const FRAMEOPTIONS_NORESIZE: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(8i32);
pub const FRAMEOPTIONS_SCROLL_AUTO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(4i32);
pub const FRAMEOPTIONS_SCROLL_NO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(2i32);
pub const FRAMEOPTIONS_SCROLL_YES: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(1i32);
pub const IECMDID_ARG_CLEAR_FORMS_ALL: u32 = 0u32;
pub const IECMDID_ARG_CLEAR_FORMS_ALL_BUT_PASSWORDS: u32 = 1u32;
pub const IECMDID_ARG_CLEAR_FORMS_PASSWORDS_ONLY: u32 = 2u32;
pub const IECMDID_BEFORENAVIGATE_DOEXTERNALBROWSE: u32 = 3u32;
pub const IECMDID_BEFORENAVIGATE_GETIDLIST: u32 = 4u32;
pub const IECMDID_BEFORENAVIGATE_GETSHELLBROWSE: u32 = 2u32;
pub const IECMDID_CLEAR_AUTOCOMPLETE_FOR_FORMS: u32 = 0u32;
pub const IECMDID_GET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 6u32;
pub const IECMDID_SETID_AUTOCOMPLETE_FOR_FORMS: u32 = 1u32;
pub const IECMDID_SET_INVOKE_DEFAULT_BROWSER_ON_NEW_WINDOW: u32 = 5u32;
pub const IEGetProcessModule_PROC_NAME: windows_core::PCSTR = windows_core::s!("IEGetProcessModule");
pub const IEGetTabWindowExports_PROC_NAME: windows_core::PCSTR = windows_core::s!("IEGetTabWindowExports");
pub const IELAUNCHOPTION_FORCE_COMPAT: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(2i32);
pub const IELAUNCHOPTION_FORCE_EDGE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(4i32);
pub const IELAUNCHOPTION_LOCK_ENGINE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(8i32);
pub const IELAUNCHOPTION_SCRIPTDEBUG: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(1i32);
pub const IEPROCESS_MODULE_NAME: windows_core::PCWSTR = windows_core::w!("IERtUtil.dll");
pub const IE_USE_OE_MAIL_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_MAIL_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Internet Explorer\\Mail");
pub const IE_USE_OE_MAIL_VALUE: windows_core::PCWSTR = windows_core::w!("Use Outlook Express");
pub const IE_USE_OE_NEWS_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_NEWS_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Internet Explorer\\News");
pub const IE_USE_OE_NEWS_VALUE: windows_core::PCWSTR = windows_core::w!("Use Outlook Express");
pub const IE_USE_OE_PRESENT_HKEY: i32 = -2147483646i32;
pub const IE_USE_OE_PRESENT_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\app.paths\\msimn.exe");
pub const IMGDECODE_EVENT_BEGINBITS: u32 = 4u32;
pub const IMGDECODE_EVENT_BITSCOMPLETE: u32 = 8u32;
pub const IMGDECODE_EVENT_PALETTE: u32 = 2u32;
pub const IMGDECODE_EVENT_PROGRESS: u32 = 1u32;
pub const IMGDECODE_EVENT_USEDDRAW: u32 = 16u32;
pub const IMGDECODE_HINT_BOTTOMUP: u32 = 2u32;
pub const IMGDECODE_HINT_FULLWIDTH: u32 = 4u32;
pub const IMGDECODE_HINT_TOPDOWN: u32 = 1u32;
pub const INTERNETEXPLORERCONFIGURATION_HOST: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(1i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(2i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER_EDGE: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(4i32);
pub const LINKSBAND: u32 = 4u32;
pub const MAPMIME_CLSID: u32 = 1u32;
pub const MAPMIME_DEFAULT: u32 = 0u32;
pub const MAPMIME_DEFAULT_ALWAYS: u32 = 3u32;
pub const MAPMIME_DISABLE: u32 = 2u32;
pub const MAX_SEARCH_FORMAT_STRING: u32 = 255u32;
pub const MediaCasting: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(2i32);
pub const MediaPlayback: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(0i32);
pub const MediaRecording: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(1i32);
pub const NAVIGATEFRAME_FL_AUTH_FAIL_CACHE_OK: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(16i32);
pub const NAVIGATEFRAME_FL_NO_DOC_CACHE: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(4i32);
pub const NAVIGATEFRAME_FL_NO_IMAGE_CACHE: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(8i32);
pub const NAVIGATEFRAME_FL_POST: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(2i32);
pub const NAVIGATEFRAME_FL_REALLY_SENDING_FROM_FORM: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(64i32);
pub const NAVIGATEFRAME_FL_RECORD: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(1i32);
pub const NAVIGATEFRAME_FL_SENDING_FROM_FORM: NAVIGATEFRAME_FLAGS = NAVIGATEFRAME_FLAGS(32i32);
pub const OS_E_CANCELLED: OpenServiceErrors = OpenServiceErrors(-2147471631i32);
pub const OS_E_GPDISABLED: OpenServiceErrors = OpenServiceErrors(-1072886820i32);
pub const OS_E_NOTFOUND: OpenServiceErrors = OpenServiceErrors(-2147287038i32);
pub const OS_E_NOTSUPPORTED: OpenServiceErrors = OpenServiceErrors(-2147467231i32);
pub const REGSTRA_VAL_STARTPAGE: windows_core::PCSTR = windows_core::s!("Start Page");
pub const REGSTR_PATH_CURRENT: windows_core::PCWSTR = windows_core::w!("current");
pub const REGSTR_PATH_DEFAULT: windows_core::PCWSTR = windows_core::w!("default");
pub const REGSTR_PATH_INETCPL_RESTRICTIONS: windows_core::PCWSTR = windows_core::w!("Software\\Policies\\Microsoft\\Internet Explorer\\Control Panel");
pub const REGSTR_PATH_MIME_DATABASE: windows_core::PCWSTR = windows_core::w!("MIME\\Database");
pub const REGSTR_PATH_REMOTEACCESS: windows_core::PCWSTR = windows_core::w!("RemoteAccess");
pub const REGSTR_PATH_REMOTEACESS: windows_core::PCWSTR = windows_core::w!("RemoteAccess");
pub const REGSTR_SHIFTQUICKSUFFIX: windows_core::PCWSTR = windows_core::w!("ShiftQuickCompleteSuffix");
pub const REGSTR_VAL_ACCEPT_LANGUAGE: windows_core::PCWSTR = windows_core::w!("AcceptLanguage");
pub const REGSTR_VAL_ACCESSMEDIUM: windows_core::PCWSTR = windows_core::w!("AccessMedium");
pub const REGSTR_VAL_ACCESSTYPE: windows_core::PCWSTR = windows_core::w!("AccessType");
pub const REGSTR_VAL_ALIASTO: windows_core::PCWSTR = windows_core::w!("AliasForCharset");
pub const REGSTR_VAL_ANCHORCOLOR: windows_core::PCWSTR = windows_core::w!("Anchor Color");
pub const REGSTR_VAL_ANCHORCOLORHOVER: windows_core::PCWSTR = windows_core::w!("Anchor Color Hover");
pub const REGSTR_VAL_ANCHORCOLORVISITED: windows_core::PCWSTR = windows_core::w!("Anchor Color Visited");
pub const REGSTR_VAL_ANCHORUNDERLINE: windows_core::PCWSTR = windows_core::w!("Anchor Underline");
pub const REGSTR_VAL_AUTODETECT: windows_core::PCWSTR = windows_core::w!("AutoDetect");
pub const REGSTR_VAL_AUTODIALDLLNAME: windows_core::PCWSTR = windows_core::w!("AutodialDllName");
pub const REGSTR_VAL_AUTODIALFCNNAME: windows_core::PCWSTR = windows_core::w!("AutodialFcnName");
pub const REGSTR_VAL_AUTODIAL_MONITORCLASSNAME: windows_core::PCWSTR = windows_core::w!("MS_AutodialMonitor");
pub const REGSTR_VAL_AUTODIAL_TRYONLYONCE: windows_core::PCWSTR = windows_core::w!("TryAutodialOnce");
pub const REGSTR_VAL_AUTONAVIGATE: windows_core::PCWSTR = windows_core::w!("SearchForExtensions");
pub const REGSTR_VAL_AUTOSEARCH: windows_core::PCWSTR = windows_core::w!("Do404Search");
pub const REGSTR_VAL_BACKBITMAP: windows_core::PCWSTR = windows_core::w!("BackBitmap");
pub const REGSTR_VAL_BACKGROUNDCOLOR: windows_core::PCWSTR = windows_core::w!("Background Color");
pub const REGSTR_VAL_BODYCHARSET: windows_core::PCWSTR = windows_core::w!("BodyCharset");
pub const REGSTR_VAL_BYPASSAUTOCONFIG: windows_core::PCWSTR = windows_core::w!("BypassAutoconfig");
pub const REGSTR_VAL_CACHEPREFIX: windows_core::PCWSTR = windows_core::w!("CachePrefix");
pub const REGSTR_VAL_CHECKASSOC: windows_core::PCWSTR = windows_core::w!("Check_Associations");
pub const REGSTR_VAL_CODEDOWNLOAD: windows_core::PCWSTR = windows_core::w!("Code Download");
pub const REGSTR_VAL_CODEDOWNLOAD_DEF: windows_core::PCWSTR = windows_core::w!("yes");
pub const REGSTR_VAL_CODEPAGE: windows_core::PCWSTR = windows_core::w!("CodePage");
pub const REGSTR_VAL_COVEREXCLUDE: windows_core::PCWSTR = windows_core::w!("CoverExclude");
pub const REGSTR_VAL_DAYSTOKEEP: windows_core::PCWSTR = windows_core::w!("DaysToKeep");
pub const REGSTR_VAL_DEFAULT_CODEPAGE: windows_core::PCWSTR = windows_core::w!("Default_CodePage");
pub const REGSTR_VAL_DEFAULT_SCRIPT: windows_core::PCWSTR = windows_core::w!("Default_Script");
pub const REGSTR_VAL_DEF_ENCODING: windows_core::PCWSTR = windows_core::w!("Default_Encoding");
pub const REGSTR_VAL_DEF_INETENCODING: windows_core::PCWSTR = windows_core::w!("Default_InternetEncoding");
pub const REGSTR_VAL_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const REGSTR_VAL_DIRECTORY: windows_core::PCWSTR = windows_core::w!("Directory");
pub const REGSTR_VAL_DISCONNECTIDLETIME: windows_core::PCWSTR = windows_core::w!("DisconnectIdleTime");
pub const REGSTR_VAL_ENABLEAUTODIAL: windows_core::PCWSTR = windows_core::w!("EnableAutodial");
pub const REGSTR_VAL_ENABLEAUTODIALDISCONNECT: windows_core::PCWSTR = windows_core::w!("EnableAutodisconnect");
pub const REGSTR_VAL_ENABLEAUTODISCONNECT: windows_core::PCWSTR = windows_core::w!("EnableAutodisconnect");
pub const REGSTR_VAL_ENABLEEXITDISCONNECT: windows_core::PCWSTR = windows_core::w!("EnableExitDisconnect");
pub const REGSTR_VAL_ENABLESECURITYCHECK: windows_core::PCWSTR = windows_core::w!("EnableSecurityCheck");
pub const REGSTR_VAL_ENABLEUNATTENDED: windows_core::PCWSTR = windows_core::w!("EnableUnattended");
pub const REGSTR_VAL_ENCODENAME: windows_core::PCWSTR = windows_core::w!("EncodingName");
pub const REGSTR_VAL_FAMILY: windows_core::PCWSTR = windows_core::w!("Family");
pub const REGSTR_VAL_FIXEDWIDTHFONT: windows_core::PCWSTR = windows_core::w!("FixedWidthFont");
pub const REGSTR_VAL_FIXED_FONT: windows_core::PCWSTR = windows_core::w!("IEFixedFontName");
pub const REGSTR_VAL_FONT_SCRIPT: windows_core::PCWSTR = windows_core::w!("Script");
pub const REGSTR_VAL_FONT_SCRIPTS: windows_core::PCWSTR = windows_core::w!("Scripts");
pub const REGSTR_VAL_FONT_SCRIPT_NAME: windows_core::PCWSTR = windows_core::w!("Script");
pub const REGSTR_VAL_FONT_SIZE: windows_core::PCWSTR = windows_core::w!("IEFontSize");
pub const REGSTR_VAL_FONT_SIZE_DEF: u32 = 2u32;
pub const REGSTR_VAL_HEADERCHARSET: windows_core::PCWSTR = windows_core::w!("HeaderCharset");
pub const REGSTR_VAL_HTTP_ERRORS: windows_core::PCWSTR = windows_core::w!("Friendly http errors");
pub const REGSTR_VAL_IE_CUSTOMCOLORS: windows_core::PCWSTR = windows_core::w!("Custom Colors");
pub const REGSTR_VAL_INETCPL_ADVANCEDTAB: windows_core::PCWSTR = windows_core::w!("AdvancedTab");
pub const REGSTR_VAL_INETCPL_CONNECTIONSTAB: windows_core::PCWSTR = windows_core::w!("ConnectionsTab");
pub const REGSTR_VAL_INETCPL_CONTENTTAB: windows_core::PCWSTR = windows_core::w!("ContentTab");
pub const REGSTR_VAL_INETCPL_GENERALTAB: windows_core::PCWSTR = windows_core::w!("GeneralTab");
pub const REGSTR_VAL_INETCPL_IEAK: windows_core::PCWSTR = windows_core::w!("IEAKContext");
pub const REGSTR_VAL_INETCPL_PRIVACYTAB: windows_core::PCWSTR = windows_core::w!("PrivacyTab");
pub const REGSTR_VAL_INETCPL_PROGRAMSTAB: windows_core::PCWSTR = windows_core::w!("ProgramsTab");
pub const REGSTR_VAL_INETCPL_SECURITYTAB: windows_core::PCWSTR = windows_core::w!("SecurityTab");
pub const REGSTR_VAL_INETENCODING: windows_core::PCWSTR = windows_core::w!("InternetEncoding");
pub const REGSTR_VAL_INTERNETENTRY: windows_core::PCWSTR = windows_core::w!("InternetProfile");
pub const REGSTR_VAL_INTERNETENTRYBKUP: windows_core::PCWSTR = windows_core::w!("BackupInternetProfile");
pub const REGSTR_VAL_INTERNETPROFILE: windows_core::PCWSTR = windows_core::w!("InternetProfile");
pub const REGSTR_VAL_JAVAJIT: windows_core::PCWSTR = windows_core::w!("EnableJIT");
pub const REGSTR_VAL_JAVAJIT_DEF: u32 = 0u32;
pub const REGSTR_VAL_JAVALOGGING: windows_core::PCWSTR = windows_core::w!("EnableLogging");
pub const REGSTR_VAL_JAVALOGGING_DEF: u32 = 0u32;
pub const REGSTR_VAL_LEVEL: windows_core::PCWSTR = windows_core::w!("Level");
pub const REGSTR_VAL_LOADIMAGES: windows_core::PCWSTR = windows_core::w!("Display Inline Images");
pub const REGSTR_VAL_LOCALPAGE: windows_core::PCWSTR = windows_core::w!("Local Page");
pub const REGSTR_VAL_MOSDISCONNECT: windows_core::PCWSTR = windows_core::w!("DisconnectTimeout");
pub const REGSTR_VAL_NEWDIRECTORY: windows_core::PCWSTR = windows_core::w!("NewDirectory");
pub const REGSTR_VAL_NONETAUTODIAL: windows_core::PCWSTR = windows_core::w!("NoNetAutodial");
pub const REGSTR_VAL_PLAYSOUNDS: windows_core::PCWSTR = windows_core::w!("Play_Background_Sounds");
pub const REGSTR_VAL_PLAYVIDEOS: windows_core::PCWSTR = windows_core::w!("Display Inline Videos");
pub const REGSTR_VAL_PRIVCONVERTER: windows_core::PCWSTR = windows_core::w!("PrivConverter");
pub const REGSTR_VAL_PROPORTIONALFONT: windows_core::PCWSTR = windows_core::w!("ProportionalFont");
pub const REGSTR_VAL_PROP_FONT: windows_core::PCWSTR = windows_core::w!("IEPropFontName");
pub const REGSTR_VAL_PROXYENABLE: windows_core::PCWSTR = windows_core::w!("ProxyEnable");
pub const REGSTR_VAL_PROXYOVERRIDE: windows_core::PCWSTR = windows_core::w!("ProxyOverride");
pub const REGSTR_VAL_PROXYSERVER: windows_core::PCWSTR = windows_core::w!("ProxyServer");
pub const REGSTR_VAL_REDIALATTEMPTS: windows_core::PCWSTR = windows_core::w!("RedialAttempts");
pub const REGSTR_VAL_REDIALINTERVAL: windows_core::PCWSTR = windows_core::w!("RedialWait");
pub const REGSTR_VAL_RNAINSTALLED: windows_core::PCWSTR = windows_core::w!("Installed");
pub const REGSTR_VAL_SAFETYWARNINGLEVEL: windows_core::PCWSTR = windows_core::w!("Safety Warning Level");
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const REGSTR_VAL_SCHANNELENABLEPROTOCOL_DEF: u32 = 1u32;
pub const REGSTR_VAL_SCRIPT_FIXED_FONT: windows_core::PCWSTR = windows_core::w!("IEFixedFontName");
pub const REGSTR_VAL_SCRIPT_PROP_FONT: windows_core::PCWSTR = windows_core::w!("IEPropFontName");
pub const REGSTR_VAL_SEARCHPAGE: windows_core::PCWSTR = windows_core::w!("Search Page");
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS: windows_core::PCWSTR = windows_core::w!("Security_RunScripts");
pub const REGSTR_VAL_SECURITYACTICEXSCRIPTS_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYACTIVEX: windows_core::PCWSTR = windows_core::w!("Security_RunActiveXControls");
pub const REGSTR_VAL_SECURITYACTIVEX_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYALLOWCOOKIES: windows_core::PCWSTR = windows_core::w!("AllowCookies");
pub const REGSTR_VAL_SECURITYALLOWCOOKIES_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES: windows_core::PCWSTR = windows_core::w!("DisableCachingOfSSLPages");
pub const REGSTR_VAL_SECURITYDISABLECACHINGOFSSLPAGES_DEF: u32 = 0u32;
pub const REGSTR_VAL_SECURITYJAVA: windows_core::PCWSTR = windows_core::w!("Security_RunJavaApplets");
pub const REGSTR_VAL_SECURITYJAVA_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING: windows_core::PCWSTR = windows_core::w!("WarnOnBadCertSending");
pub const REGSTR_VAL_SECURITYWARNONBADCERTSENDING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING: windows_core::PCWSTR = windows_core::w!("WarnOnBadCertRecving");
pub const REGSTR_VAL_SECURITYWARNONBADCERTVIEWING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONSEND: windows_core::PCWSTR = windows_core::w!("WarnOnPost");
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS: windows_core::PCWSTR = windows_core::w!("WarnAlwaysOnPost");
pub const REGSTR_VAL_SECURITYWARNONSENDALWAYS_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONSEND_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONVIEW: windows_core::PCWSTR = windows_core::w!("WarnOnView");
pub const REGSTR_VAL_SECURITYWARNONVIEW_DEF: u32 = 1u32;
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING: windows_core::PCWSTR = windows_core::w!("WarnOnZoneCrossing");
pub const REGSTR_VAL_SECURITYWARNONZONECROSSING_DEF: u32 = 1u32;
pub const REGSTR_VAL_SHOWADDRESSBAR: windows_core::PCWSTR = windows_core::w!("Show_URLToolBar");
pub const REGSTR_VAL_SHOWFOCUS: windows_core::PCWSTR = windows_core::w!("Tabstop - MouseDown");
pub const REGSTR_VAL_SHOWFOCUS_DEF: windows_core::PCWSTR = windows_core::w!("no");
pub const REGSTR_VAL_SHOWFULLURLS: windows_core::PCWSTR = windows_core::w!("Show_FullURL");
pub const REGSTR_VAL_SHOWTOOLBAR: windows_core::PCWSTR = windows_core::w!("Show_ToolBar");
pub const REGSTR_VAL_SMOOTHSCROLL: windows_core::PCWSTR = windows_core::w!("SmoothScroll");
pub const REGSTR_VAL_SMOOTHSCROLL_DEF: u32 = 1u32;
pub const REGSTR_VAL_STARTPAGE: windows_core::PCWSTR = windows_core::w!("Start Page");
pub const REGSTR_VAL_TEXTCOLOR: windows_core::PCWSTR = windows_core::w!("Text Color");
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_HIGH: windows_core::PCWSTR = windows_core::w!("High");
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_LOW: windows_core::PCWSTR = windows_core::w!("No Security");
pub const REGSTR_VAL_TRUSTWARNINGLEVEL_MED: windows_core::PCWSTR = windows_core::w!("Medium");
pub const REGSTR_VAL_USEAUTOAPPEND: windows_core::PCWSTR = windows_core::w!("Append Completion");
pub const REGSTR_VAL_USEAUTOCOMPLETE: windows_core::PCWSTR = windows_core::w!("Use AutoComplete");
pub const REGSTR_VAL_USEAUTOSUGGEST: windows_core::PCWSTR = windows_core::w!("AutoSuggest");
pub const REGSTR_VAL_USEDLGCOLORS: windows_core::PCWSTR = windows_core::w!("Use_DlgBox_Colors");
pub const REGSTR_VAL_USEHOVERCOLOR: windows_core::PCWSTR = windows_core::w!("Use Anchor Hover Color");
pub const REGSTR_VAL_USEIBAR: windows_core::PCWSTR = windows_core::w!("UseBar");
pub const REGSTR_VAL_USEICM: windows_core::PCWSTR = windows_core::w!("UseICM");
pub const REGSTR_VAL_USEICM_DEF: u32 = 0u32;
pub const REGSTR_VAL_USERAGENT: windows_core::PCWSTR = windows_core::w!("User Agent");
pub const REGSTR_VAL_USESTYLESHEETS: windows_core::PCWSTR = windows_core::w!("Use Stylesheets");
pub const REGSTR_VAL_USESTYLESHEETS_DEF: windows_core::PCWSTR = windows_core::w!("yes");
pub const REGSTR_VAL_VISIBLEBANDS: windows_core::PCWSTR = windows_core::w!("VisibleBands");
pub const REGSTR_VAL_VISIBLEBANDS_DEF: u32 = 7u32;
pub const REGSTR_VAL_WEBCHARSET: windows_core::PCWSTR = windows_core::w!("WebCharset");
pub const SCMP_BOTTOM: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(1i32);
pub const SCMP_FULL: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(4i32);
pub const SCMP_LEFT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(2i32);
pub const SCMP_RIGHT: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(3i32);
pub const SCMP_TOP: SCROLLABLECONTEXTMENU_PLACEMENT = SCROLLABLECONTEXTMENU_PLACEMENT(0i32);
pub const STATURLFLAG_ISCACHED: u32 = 1u32;
pub const STATURLFLAG_ISTOPLEVEL: u32 = 2u32;
pub const STATURL_QUERYFLAG_ISCACHED: u32 = 65536u32;
pub const STATURL_QUERYFLAG_NOTITLE: u32 = 262144u32;
pub const STATURL_QUERYFLAG_NOURL: u32 = 131072u32;
pub const STATURL_QUERYFLAG_TOPLEVEL: u32 = 524288u32;
pub const SURFACE_LOCK_ALLOW_DISCARD: u32 = 2u32;
pub const SURFACE_LOCK_EXCLUSIVE: u32 = 1u32;
pub const SURFACE_LOCK_WAIT: u32 = 4u32;
pub const SZBACKBITMAP: windows_core::PCSTR = windows_core::s!("BackBitmap");
pub const SZJAVAVMPATH: windows_core::PCSTR = windows_core::s!("\\Java VM");
pub const SZNOTEXT: windows_core::PCSTR = windows_core::s!("NoText");
pub const SZTOOLBAR: windows_core::PCSTR = windows_core::s!("\\Toolbar");
pub const SZTRUSTWARNLEVEL: windows_core::PCSTR = windows_core::s!("Trust Warning Level");
pub const SZVISIBLE: windows_core::PCSTR = windows_core::s!("VisibleBands");
pub const SZ_IE_DEFAULT_HTML_EDITOR: windows_core::PCSTR = windows_core::s!("Default HTML Editor");
pub const SZ_IE_IBAR: windows_core::PCSTR = windows_core::s!("Bar");
pub const SZ_IE_IBAR_BANDS: windows_core::PCSTR = windows_core::s!("Bands");
pub const SZ_IE_MAIN: windows_core::PCSTR = windows_core::s!("Main");
pub const SZ_IE_SEARCHSTRINGS: windows_core::PCSTR = windows_core::s!("UrlTemplate");
pub const SZ_IE_SECURITY: windows_core::PCSTR = windows_core::s!("Security");
pub const SZ_IE_SETTINGS: windows_core::PCSTR = windows_core::s!("Settings");
pub const SZ_IE_THRESHOLDS: windows_core::PCSTR = windows_core::s!("ErrorThresholds");
pub const S_SURFACE_DISCARDED: i32 = 49155i32;
pub const TARGET_NOTIFY_OBJECT_NAME: windows_core::PCWSTR = windows_core::w!("863a99a0-21bc-11d0-82b4-00a0c90c29c5");
pub const TF_NAVIGATE: u32 = 2142153644u32;
pub const TIMERMODE_NORMAL: u32 = 0u32;
pub const TIMERMODE_VISIBILITYAWARE: u32 = 1u32;
pub const TOOLSBAND: u32 = 1u32;
pub const TSZCALENDARPROTOCOL: windows_core::PCWSTR = windows_core::w!("unk");
pub const TSZCALLTOPROTOCOL: windows_core::PCWSTR = windows_core::w!("callto");
pub const TSZINTERNETCLIENTSPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Internet Explorer\\Unix");
pub const TSZLDAPPROTOCOL: windows_core::PCWSTR = windows_core::w!("ldap");
pub const TSZMAILTOPROTOCOL: windows_core::PCWSTR = windows_core::w!("mailto");
pub const TSZMICROSOFTPATH: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft");
pub const TSZNEWSPROTOCOL: windows_core::PCWSTR = windows_core::w!("news");
pub const TSZPROTOCOLSPATH: windows_core::PCWSTR = windows_core::w!("Protocols\\");
pub const TSZSCHANNELPATH: windows_core::PCWSTR = windows_core::w!("SYSTEM\\CurrentControlSet\\Control\\SecurityProviders\\SCHANNEL");
pub const TSZVSOURCEPROTOCOL: windows_core::PCWSTR = windows_core::w!("view source");
pub const msodsvFailed: u32 = 3u32;
pub const msodsvLowSecurityLevel: u32 = 4u32;
pub const msodsvNoMacros: u32 = 0u32;
pub const msodsvPassedTrusted: u32 = 2u32;
pub const msodsvPassedTrustedCert: u32 = 5u32;
pub const msodsvUnsigned: u32 = 1u32;
pub const msoedmDisable: u32 = 2u32;
pub const msoedmDontOpen: u32 = 3u32;
pub const msoedmEnable: u32 = 1u32;
pub const msoslHigh: u32 = 3u32;
pub const msoslMedium: u32 = 2u32;
pub const msoslNone: u32 = 1u32;
pub const msoslUndefined: u32 = 0u32;
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADDURL_FLAG(pub i32);
impl windows_core::TypeKind for ADDURL_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADDURL_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADDURL_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtensionValidationContexts(pub i32);
impl windows_core::TypeKind for ExtensionValidationContexts {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtensionValidationContexts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtensionValidationContexts").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ExtensionValidationResults(pub i32);
impl windows_core::TypeKind for ExtensionValidationResults {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ExtensionValidationResults {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ExtensionValidationResults").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FINDFRAME_FLAGS(pub i32);
impl windows_core::TypeKind for FINDFRAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FINDFRAME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FINDFRAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FRAMEOPTIONS_FLAGS(pub i32);
impl windows_core::TypeKind for FRAMEOPTIONS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FRAMEOPTIONS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FRAMEOPTIONS_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IELAUNCHOPTION_FLAGS(pub i32);
impl windows_core::TypeKind for IELAUNCHOPTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IELAUNCHOPTION_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IELAUNCHOPTION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INTERNETEXPLORERCONFIGURATION(pub i32);
impl windows_core::TypeKind for INTERNETEXPLORERCONFIGURATION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INTERNETEXPLORERCONFIGURATION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INTERNETEXPLORERCONFIGURATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEDIA_ACTIVITY_NOTIFY_TYPE(pub i32);
impl windows_core::TypeKind for MEDIA_ACTIVITY_NOTIFY_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEDIA_ACTIVITY_NOTIFY_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEDIA_ACTIVITY_NOTIFY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NAVIGATEFRAME_FLAGS(pub i32);
impl windows_core::TypeKind for NAVIGATEFRAME_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NAVIGATEFRAME_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NAVIGATEFRAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OpenServiceActivityContentType(pub i32);
impl windows_core::TypeKind for OpenServiceActivityContentType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OpenServiceActivityContentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OpenServiceActivityContentType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OpenServiceErrors(pub i32);
impl windows_core::TypeKind for OpenServiceErrors {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OpenServiceErrors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OpenServiceErrors").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SCROLLABLECONTEXTMENU_PLACEMENT(pub i32);
impl windows_core::TypeKind for SCROLLABLECONTEXTMENU_PLACEMENT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SCROLLABLECONTEXTMENU_PLACEMENT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SCROLLABLECONTEXTMENU_PLACEMENT").field(&self.0).finish()
    }
}
pub const AnchorClick: windows_core::GUID = windows_core::GUID::from_u128(0x13d5413c_33b9_11d2_95a7_00c04f8ecb02);
pub const CDeviceRect: windows_core::GUID = windows_core::GUID::from_u128(0x3050f6d4_98b5_11cf_bb82_00aa00bdce0b);
pub const CDownloadBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x3050f5be_98b5_11cf_bb82_00aa00bdce0b);
pub const CHeaderFooter: windows_core::GUID = windows_core::GUID::from_u128(0x3050f6cd_98b5_11cf_bb82_00aa00bdce0b);
pub const CLayoutRect: windows_core::GUID = windows_core::GUID::from_u128(0x3050f664_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistDataPeer: windows_core::GUID = windows_core::GUID::from_u128(0x3050f487_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistHistory: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c8_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistShortcut: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c6_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistSnapshot: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c9_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistUserData: windows_core::GUID = windows_core::GUID::from_u128(0x3050f48e_98b5_11cf_bb82_00aa00bdce0b);
pub const CoDitherToRGB8: windows_core::GUID = windows_core::GUID::from_u128(0xa860ce50_3910_11d0_86fc_00a0c913f750);
pub const CoMapMIMEToCLSID: windows_core::GUID = windows_core::GUID::from_u128(0x30c3b080_30fb_11d0_b724_00aa006c1a01);
pub const CoSniffStream: windows_core::GUID = windows_core::GUID::from_u128(0x6a01fda0_30df_11d0_b724_00aa006c1a01);
pub const HomePage: windows_core::GUID = windows_core::GUID::from_u128(0x766bf2ae_d650_11d1_9811_00c04fc31d2e);
pub const HomePageSetting: windows_core::GUID = windows_core::GUID::from_u128(0x374cede0_873a_4c4f_bc86_bcc8cf5116a3);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IELAUNCHURLINFO {
    pub cbSize: u32,
    pub dwCreationFlags: u32,
    pub dwLaunchOptionFlags: u32,
}
impl windows_core::TypeKind for IELAUNCHURLINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for IELAUNCHURLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IEWebDriverManager: windows_core::GUID = windows_core::GUID::from_u128(0x90314af2_5250_47b3_89d8_6295fc23bc22);
pub const IntelliForms: windows_core::GUID = windows_core::GUID::from_u128(0x613ab92e_16bf_11d2_bca5_00c04fd929db);
pub const InternetExplorerManager: windows_core::GUID = windows_core::GUID::from_u128(0xdf4fcc34_067a_4e0a_8352_4a1a5095346e);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NAVIGATEDATA {
    pub ulTarget: u32,
    pub ulURL: u32,
    pub ulRefURL: u32,
    pub ulPostData: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for NAVIGATEDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for NAVIGATEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OpenServiceActivityManager: windows_core::GUID = windows_core::GUID::from_u128(0xc5efd803_50f8_43cd_9ab8_aafc1394c9e0);
pub const OpenServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0x098870b6_39ea_480b_b8b5_dd0167c4db59);
pub const PeerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4cf_98b5_11cf_bb82_00aa00bdce0b);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STATURL {
    pub cbSize: u32,
    pub pwcsUrl: windows_core::PWSTR,
    pub pwcsTitle: windows_core::PWSTR,
    pub ftLastVisited: super::super::Foundation::FILETIME,
    pub ftLastUpdated: super::super::Foundation::FILETIME,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for STATURL {
    type TypeKind = windows_core::CopyType;
}
impl Default for STATURL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const wfolders: windows_core::GUID = windows_core::GUID::from_u128(0xbae31f9a_1b81_11d2_a97a_00c04f8ecb02);
#[cfg(feature = "implement")]
core::include!("impl.rs");
