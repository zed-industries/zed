#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ComputeInvCMAP(prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD, ncolors: u32, pinvtable: *mut u8, cbtable: u32) -> windows_core::Result<()> {
    windows_core::link!("imgutil.dll" "system" fn ComputeInvCMAP(prgbcolors : *const super::super::Graphics::Gdi:: RGBQUAD, ncolors : u32, pinvtable : *mut u8, cbtable : u32) -> windows_core::HRESULT);
    unsafe { ComputeInvCMAP(prgbcolors, ncolors, pinvtable as _, cbtable).ok() }
}
#[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateDDrawSurfaceOnDIB(hbmdib: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<super::super::Graphics::DirectDraw::IDirectDrawSurface> {
    windows_core::link!("imgutil.dll" "system" fn CreateDDrawSurfaceOnDIB(hbmdib : super::super::Graphics::Gdi:: HBITMAP, ppsurface : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDDrawSurfaceOnDIB(hbmdib, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateMIMEMap() -> windows_core::Result<IMapMIMEToCLSID> {
    windows_core::link!("imgutil.dll" "system" fn CreateMIMEMap(ppmap : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateMIMEMap(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DecodeImage<P0, P1, P2>(pstream: P0, pmap: P1, peventsink: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<IMapMIMEToCLSID>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("imgutil.dll" "system" fn DecodeImage(pstream : * mut core::ffi::c_void, pmap : * mut core::ffi::c_void, peventsink : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DecodeImage(pstream.param().abi(), pmap.param().abi(), peventsink.param().abi()).ok() }
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
    windows_core::link!("imgutil.dll" "system" fn DecodeImageEx(pstream : * mut core::ffi::c_void, pmap : * mut core::ffi::c_void, peventsink : * mut core::ffi::c_void, pszmimetypeparam : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DecodeImageEx(pstream.param().abi(), pmap.param().abi(), peventsink.param().abi(), pszmimetypeparam.param().abi()).ok() }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DitherTo8(pdestbits: *mut u8, ndestpitch: i32, psrcbits: *mut u8, nsrcpitch: i32, bfidsrc: *const windows_core::GUID, prgbdestcolors: *mut super::super::Graphics::Gdi::RGBQUAD, prgbsrccolors: *mut super::super::Graphics::Gdi::RGBQUAD, pbdestinvmap: *mut u8, x: i32, y: i32, cx: i32, cy: i32, ldesttrans: i32, lsrctrans: i32) -> windows_core::Result<()> {
    windows_core::link!("imgutil.dll" "system" fn DitherTo8(pdestbits : *mut u8, ndestpitch : i32, psrcbits : *mut u8, nsrcpitch : i32, bfidsrc : *const windows_core::GUID, prgbdestcolors : *mut super::super::Graphics::Gdi:: RGBQUAD, prgbsrccolors : *mut super::super::Graphics::Gdi:: RGBQUAD, pbdestinvmap : *mut u8, x : i32, y : i32, cx : i32, cy : i32, ldesttrans : i32, lsrctrans : i32) -> windows_core::HRESULT);
    unsafe { DitherTo8(pdestbits as _, ndestpitch, psrcbits as _, nsrcpitch, bfidsrc, prgbdestcolors as _, prgbsrccolors as _, pbdestinvmap as _, x, y, cx, cy, ldesttrans, lsrctrans).ok() }
}
#[inline]
pub unsafe fn GetMaxMIMEIDBytes(pnmaxbytes: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("imgutil.dll" "system" fn GetMaxMIMEIDBytes(pnmaxbytes : *mut u32) -> windows_core::HRESULT);
    unsafe { GetMaxMIMEIDBytes(pnmaxbytes as _).ok() }
}
#[inline]
pub unsafe fn IEAssociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> windows_core::Result<()> {
    windows_core::link!("ieframe.dll" "system" fn IEAssociateThreadWithTab(dwtabthreadid : u32, dwassociatedthreadid : u32) -> windows_core::HRESULT);
    unsafe { IEAssociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok() }
}
#[inline]
pub unsafe fn IECancelSaveFile(hstate: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("ieframe.dll" "system" fn IECancelSaveFile(hstate : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { IECancelSaveFile(hstate).ok() }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IECreateDirectory<P0>(lppathname: P0, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IECreateDirectory(lppathname : windows_core::PCWSTR, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { IECreateDirectory(lppathname.param().abi(), lpsecurityattributes) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn IECreateFile<P0>(lpfilename: P0, dwdesiredaccess: u32, dwsharemode: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwcreationdisposition: u32, dwflagsandattributes: u32, htemplatefile: Option<super::super::Foundation::HANDLE>) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IECreateFile(lpfilename : windows_core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: HANDLE);
    unsafe { IECreateFile(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IEDeleteFile<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEDeleteFile(lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IEDeleteFile(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn IEDisassociateThreadWithTab(dwtabthreadid: u32, dwassociatedthreadid: u32) -> windows_core::Result<()> {
    windows_core::link!("ieframe.dll" "system" fn IEDisassociateThreadWithTab(dwtabthreadid : u32, dwassociatedthreadid : u32) -> windows_core::HRESULT);
    unsafe { IEDisassociateThreadWithTab(dwtabthreadid, dwassociatedthreadid).ok() }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn IEFindFirstFile<P0>(lpfilename: P0, lpfindfiledata: *const super::super::Storage::FileSystem::WIN32_FIND_DATAA) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEFindFirstFile(lpfilename : windows_core::PCWSTR, lpfindfiledata : *const super::super::Storage::FileSystem:: WIN32_FIND_DATAA) -> super::super::Foundation:: HANDLE);
    unsafe { IEFindFirstFile(lpfilename.param().abi(), lpfindfiledata) }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
#[inline]
pub unsafe fn IEGetFileAttributesEx<P0>(lpfilename: P0, finfolevelid: super::super::Storage::FileSystem::GET_FILEEX_INFO_LEVELS, lpfileinformation: *const core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEGetFileAttributesEx(lpfilename : windows_core::PCWSTR, finfolevelid : super::super::Storage::FileSystem:: GET_FILEEX_INFO_LEVELS, lpfileinformation : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { IEGetFileAttributesEx(lpfilename.param().abi(), finfolevelid, lpfileinformation) }
}
#[inline]
pub unsafe fn IEGetProtectedModeCookie<P0, P1>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: windows_core::PWSTR, pcchcookiedata: *mut u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEGetProtectedModeCookie(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PWSTR, pcchcookiedata : *mut u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { IEGetProtectedModeCookie(lpszurl.param().abi(), lpszcookiename.param().abi(), core::mem::transmute(lpszcookiedata), pcchcookiedata as _, dwflags).ok() }
}
#[inline]
pub unsafe fn IEGetWriteableFolderPath(clsidfolderid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("ieframe.dll" "system" fn IEGetWriteableFolderPath(clsidfolderid : *const windows_core::GUID, lppwstrpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IEGetWriteableFolderPath(clsidfolderid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn IEGetWriteableLowHKCU() -> windows_core::Result<super::super::System::Registry::HKEY> {
    windows_core::link!("ieframe.dll" "system" fn IEGetWriteableLowHKCU(phkey : *mut super::super::System::Registry:: HKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IEGetWriteableLowHKCU(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IEInPrivateFilteringEnabled() -> windows_core::BOOL {
    windows_core::link!("ieframe.dll" "system" fn IEInPrivateFilteringEnabled() -> windows_core::BOOL);
    unsafe { IEInPrivateFilteringEnabled() }
}
#[inline]
pub unsafe fn IEIsInPrivateBrowsing() -> windows_core::BOOL {
    windows_core::link!("ieframe.dll" "system" fn IEIsInPrivateBrowsing() -> windows_core::BOOL);
    unsafe { IEIsInPrivateBrowsing() }
}
#[inline]
pub unsafe fn IEIsProtectedModeProcess() -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("ieframe.dll" "system" fn IEIsProtectedModeProcess(pbresult : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        IEIsProtectedModeProcess(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IEIsProtectedModeURL<P0>(lpwstrurl: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEIsProtectedModeURL(lpwstrurl : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { IEIsProtectedModeURL(lpwstrurl.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn IELaunchURL<P0>(lpwstrurl: P0, lpprocinfo: *mut super::super::System::Threading::PROCESS_INFORMATION, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IELaunchURL(lpwstrurl : windows_core::PCWSTR, lpprocinfo : *mut super::super::System::Threading:: PROCESS_INFORMATION, lpinfo : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { IELaunchURL(lpwstrurl.param().abi(), lpprocinfo as _, lpinfo.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn IEMoveFileEx<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEMoveFileEx(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe { IEMoveFileEx(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn IERefreshElevationPolicy() -> windows_core::Result<()> {
    windows_core::link!("ieframe.dll" "system" fn IERefreshElevationPolicy() -> windows_core::HRESULT);
    unsafe { IERefreshElevationPolicy().ok() }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn IERegCreateKeyEx<P0, P2>(lpsubkey: P0, reserved: u32, lpclass: P2, dwoptions: u32, samdesired: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IERegCreateKeyEx(lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : u32, samdesired : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, phkresult : *mut super::super::System::Registry:: HKEY, lpdwdisposition : *mut u32) -> windows_core::HRESULT);
    unsafe { IERegCreateKeyEx(lpsubkey.param().abi(), reserved, lpclass.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition as _).ok() }
}
#[inline]
pub unsafe fn IERegSetValueEx<P0, P1>(lpsubkey: P0, lpvaluename: P1, reserved: u32, dwtype: u32, lpdata: &[u8]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IERegSetValueEx(lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> windows_core::HRESULT);
    unsafe { IERegSetValueEx(lpsubkey.param().abi(), lpvaluename.param().abi(), reserved, dwtype, core::mem::transmute(lpdata.as_ptr()), lpdata.len().try_into().unwrap()).ok() }
}
#[inline]
pub unsafe fn IERegisterWritableRegistryKey<P1>(guid: windows_core::GUID, lpsubkey: P1, fsubkeyallowed: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IERegisterWritableRegistryKey(guid : windows_core::GUID, lpsubkey : windows_core::PCWSTR, fsubkeyallowed : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { IERegisterWritableRegistryKey(core::mem::transmute(guid), lpsubkey.param().abi(), fsubkeyallowed.into()).ok() }
}
#[inline]
pub unsafe fn IERegisterWritableRegistryValue<P1, P2>(guid: windows_core::GUID, lppath: P1, lpvaluename: P2, dwtype: u32, lpdata: Option<&[u8]>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IERegisterWritableRegistryValue(guid : windows_core::GUID, lppath : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbmaxdata : u32) -> windows_core::HRESULT);
    unsafe { IERegisterWritableRegistryValue(core::mem::transmute(guid), lppath.param().abi(), lpvaluename.param().abi(), dwtype, core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())).ok() }
}
#[inline]
pub unsafe fn IERemoveDirectory<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IERemoveDirectory(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { IERemoveDirectory(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn IESaveFile<P1>(hstate: super::super::Foundation::HANDLE, lpwstrsourcefile: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IESaveFile(hstate : super::super::Foundation:: HANDLE, lpwstrsourcefile : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { IESaveFile(hstate, lpwstrsourcefile.param().abi()).ok() }
}
#[inline]
pub unsafe fn IESetProtectedModeCookie<P0, P1, P2>(lpszurl: P0, lpszcookiename: P1, lpszcookiedata: P2, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IESetProtectedModeCookie(lpszurl : windows_core::PCWSTR, lpszcookiename : windows_core::PCWSTR, lpszcookiedata : windows_core::PCWSTR, dwflags : u32) -> windows_core::HRESULT);
    unsafe { IESetProtectedModeCookie(lpszurl.param().abi(), lpszcookiename.param().abi(), lpszcookiedata.param().abi(), dwflags).ok() }
}
#[inline]
pub unsafe fn IEShowOpenFileDialog<P3, P4, P5>(hwnd: super::super::Foundation::HWND, lpwstrfilename: &mut [u16], lpwstrinitialdir: P3, lpwstrfilter: P4, lpwstrdefext: P5, dwfilterindex: u32, dwflags: u32, phfile: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEShowOpenFileDialog(hwnd : super::super::Foundation:: HWND, lpwstrfilename : windows_core::PWSTR, cchmaxfilename : u32, lpwstrinitialdir : windows_core::PCWSTR, lpwstrfilter : windows_core::PCWSTR, lpwstrdefext : windows_core::PCWSTR, dwfilterindex : u32, dwflags : u32, phfile : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { IEShowOpenFileDialog(hwnd, core::mem::transmute(lpwstrfilename.as_ptr()), lpwstrfilename.len().try_into().unwrap(), lpwstrinitialdir.param().abi(), lpwstrfilter.param().abi(), lpwstrdefext.param().abi(), dwfilterindex, dwflags, phfile as _).ok() }
}
#[inline]
pub unsafe fn IEShowSaveFileDialog<P1, P2, P3, P4>(hwnd: super::super::Foundation::HWND, lpwstrinitialfilename: P1, lpwstrinitialdir: P2, lpwstrfilter: P3, lpwstrdefext: P4, dwfilterindex: u32, dwflags: u32, lppwstrdestinationfilepath: *mut windows_core::PWSTR, phstate: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ieframe.dll" "system" fn IEShowSaveFileDialog(hwnd : super::super::Foundation:: HWND, lpwstrinitialfilename : windows_core::PCWSTR, lpwstrinitialdir : windows_core::PCWSTR, lpwstrfilter : windows_core::PCWSTR, lpwstrdefext : windows_core::PCWSTR, dwfilterindex : u32, dwflags : u32, lppwstrdestinationfilepath : *mut windows_core::PWSTR, phstate : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { IEShowSaveFileDialog(hwnd, lpwstrinitialfilename.param().abi(), lpwstrinitialdir.param().abi(), lpwstrfilter.param().abi(), lpwstrdefext.param().abi(), dwfilterindex, dwflags, lppwstrdestinationfilepath as _, phstate as _).ok() }
}
#[inline]
pub unsafe fn IETrackingProtectionEnabled() -> windows_core::BOOL {
    windows_core::link!("ieframe.dll" "system" fn IETrackingProtectionEnabled() -> windows_core::BOOL);
    unsafe { IETrackingProtectionEnabled() }
}
#[inline]
pub unsafe fn IEUnregisterWritableRegistry(guid: windows_core::GUID) -> windows_core::Result<()> {
    windows_core::link!("ieframe.dll" "system" fn IEUnregisterWritableRegistry(guid : windows_core::GUID) -> windows_core::HRESULT);
    unsafe { IEUnregisterWritableRegistry(core::mem::transmute(guid)).ok() }
}
#[inline]
pub unsafe fn IdentifyMIMEType(pbbytes: *const u8, nbytes: u32, pnformat: *mut u32) -> windows_core::Result<()> {
    windows_core::link!("imgutil.dll" "system" fn IdentifyMIMEType(pbbytes : *const u8, nbytes : u32, pnformat : *mut u32) -> windows_core::HRESULT);
    unsafe { IdentifyMIMEType(pbbytes, nbytes, pnformat as _).ok() }
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog<P1, P2>(hdlg: super::super::Foundation::HWND, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingAccessDeniedDialog(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, pszcontentdescription : windows_core::PCSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingAccessDeniedDialog(hdlg, pszusername.param().abi(), pszcontentdescription.param().abi(), pratingdetails as _).ok() }
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog2<P1>(hdlg: super::super::Foundation::HWND, pszusername: P1, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingAccessDeniedDialog2(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingAccessDeniedDialog2(hdlg, pszusername.param().abi(), pratingdetails as _).ok() }
}
#[inline]
pub unsafe fn RatingAccessDeniedDialog2W<P1>(hdlg: super::super::Foundation::HWND, pszusername: P1, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingAccessDeniedDialog2W(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingAccessDeniedDialog2W(hdlg, pszusername.param().abi(), pratingdetails as _).ok() }
}
#[inline]
pub unsafe fn RatingAccessDeniedDialogW<P1, P2>(hdlg: super::super::Foundation::HWND, pszusername: P1, pszcontentdescription: P2, pratingdetails: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingAccessDeniedDialogW(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, pszcontentdescription : windows_core::PCWSTR, pratingdetails : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingAccessDeniedDialogW(hdlg, pszusername.param().abi(), pszcontentdescription.param().abi(), pratingdetails as _).ok() }
}
#[inline]
pub unsafe fn RatingAddToApprovedSites<P3>(hdlg: super::super::Foundation::HWND, pbpasswordblob: &mut [u8], lpszurl: P3, falwaysnever: bool, fsitepage: bool, fapprovedsitesenforced: bool) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingAddToApprovedSites(hdlg : super::super::Foundation:: HWND, cbpasswordblob : u32, pbpasswordblob : *mut u8, lpszurl : windows_core::PCWSTR, falwaysnever : windows_core::BOOL, fsitepage : windows_core::BOOL, fapprovedsitesenforced : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RatingAddToApprovedSites(hdlg, pbpasswordblob.len().try_into().unwrap(), core::mem::transmute(pbpasswordblob.as_ptr()), lpszurl.param().abi(), falwaysnever.into(), fsitepage.into(), fapprovedsitesenforced.into()).ok() }
}
#[inline]
pub unsafe fn RatingCheckUserAccess<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: Option<&[u8]>, ppratingdetails: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingCheckUserAccess(pszusername : windows_core::PCSTR, pszurl : windows_core::PCSTR, pszratinginfo : windows_core::PCSTR, pdata : *const u8, cbdata : u32, ppratingdetails : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingCheckUserAccess(pszusername.param().abi(), pszurl.param().abi(), pszratinginfo.param().abi(), core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ppratingdetails.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RatingCheckUserAccessW<P0, P1, P2>(pszusername: P0, pszurl: P1, pszratinginfo: P2, pdata: Option<&[u8]>, ppratingdetails: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingCheckUserAccessW(pszusername : windows_core::PCWSTR, pszurl : windows_core::PCWSTR, pszratinginfo : windows_core::PCWSTR, pdata : *const u8, cbdata : u32, ppratingdetails : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingCheckUserAccessW(pszusername.param().abi(), pszurl.param().abi(), pszratinginfo.param().abi(), core::mem::transmute(pdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ppratingdetails.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RatingClickedOnPRFInternal<P2>(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingClickedOnPRFInternal(hwndowner : super::super::Foundation:: HWND, param1 : super::super::Foundation:: HINSTANCE, lpszfilename : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { RatingClickedOnPRFInternal(hwndowner, param1, lpszfilename.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn RatingClickedOnRATInternal<P2>(hwndowner: super::super::Foundation::HWND, param1: super::super::Foundation::HINSTANCE, lpszfilename: P2, nshow: i32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingClickedOnRATInternal(hwndowner : super::super::Foundation:: HWND, param1 : super::super::Foundation:: HINSTANCE, lpszfilename : windows_core::PCSTR, nshow : i32) -> windows_core::HRESULT);
    unsafe { RatingClickedOnRATInternal(hwndowner, param1, lpszfilename.param().abi(), nshow).ok() }
}
#[inline]
pub unsafe fn RatingEnable<P1>(hwndparent: super::super::Foundation::HWND, pszusername: P1, fenable: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingEnable(hwndparent : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR, fenable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RatingEnable(hwndparent, pszusername.param().abi(), fenable.into()).ok() }
}
#[inline]
pub unsafe fn RatingEnableW<P1>(hwndparent: super::super::Foundation::HWND, pszusername: P1, fenable: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingEnableW(hwndparent : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR, fenable : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RatingEnableW(hwndparent, pszusername.param().abi(), fenable.into()).ok() }
}
#[inline]
pub unsafe fn RatingEnabledQuery() -> windows_core::Result<()> {
    windows_core::link!("msrating.dll" "system" fn RatingEnabledQuery() -> windows_core::HRESULT);
    unsafe { RatingEnabledQuery().ok() }
}
#[inline]
pub unsafe fn RatingFreeDetails(pratingdetails: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("msrating.dll" "system" fn RatingFreeDetails(pratingdetails : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RatingFreeDetails(pratingdetails.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RatingInit() -> windows_core::Result<()> {
    windows_core::link!("msrating.dll" "system" fn RatingInit() -> windows_core::HRESULT);
    unsafe { RatingInit().ok() }
}
#[inline]
pub unsafe fn RatingObtainCancel(hratingobtainquery: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_core::link!("msrating.dll" "system" fn RatingObtainCancel(hratingobtainquery : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { RatingObtainCancel(hratingobtainquery).ok() }
}
#[inline]
pub unsafe fn RatingObtainQuery<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingObtainQuery(psztargeturl : windows_core::PCSTR, dwuserdata : u32, fcallback : isize, phratingobtainquery : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { RatingObtainQuery(psztargeturl.param().abi(), dwuserdata, fcallback, phratingobtainquery.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RatingObtainQueryW<P0>(psztargeturl: P0, dwuserdata: u32, fcallback: isize, phratingobtainquery: Option<*mut super::super::Foundation::HANDLE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingObtainQueryW(psztargeturl : windows_core::PCWSTR, dwuserdata : u32, fcallback : isize, phratingobtainquery : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    unsafe { RatingObtainQueryW(psztargeturl.param().abi(), dwuserdata, fcallback, phratingobtainquery.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn RatingSetupUI<P1>(hdlg: super::super::Foundation::HWND, pszusername: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingSetupUI(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCSTR) -> windows_core::HRESULT);
    unsafe { RatingSetupUI(hdlg, pszusername.param().abi()).ok() }
}
#[inline]
pub unsafe fn RatingSetupUIW<P1>(hdlg: super::super::Foundation::HWND, pszusername: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msrating.dll" "system" fn RatingSetupUIW(hdlg : super::super::Foundation:: HWND, pszusername : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { RatingSetupUIW(hdlg, pszusername.param().abi()).ok() }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SniffStream<P0>(pinstream: P0, pnformat: *mut u32, ppoutstream: *mut Option<super::super::System::Com::IStream>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("imgutil.dll" "system" fn SniffStream(pinstream : * mut core::ffi::c_void, pnformat : *mut u32, ppoutstream : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SniffStream(pinstream.param().abi(), pnformat as _, core::mem::transmute(ppoutstream)).ok() }
}
pub const ADDRESSBAND: u32 = 2u32;
pub const ADDURL_ADDTOCACHE: ADDURL_FLAG = ADDURL_FLAG(1i32);
pub const ADDURL_ADDTOHISTORYANDCACHE: ADDURL_FLAG = ADDURL_FLAG(0i32);
pub const ADDURL_FIRST: ADDURL_FLAG = ADDURL_FLAG(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADDURL_FLAG(pub i32);
pub const ADDURL_Max: ADDURL_FLAG = ADDURL_FLAG(2147483647i32);
pub const ActivityContentCount: OpenServiceActivityContentType = OpenServiceActivityContentType(3i32);
pub const ActivityContentDocument: OpenServiceActivityContentType = OpenServiceActivityContentType(0i32);
pub const ActivityContentLink: OpenServiceActivityContentType = OpenServiceActivityContentType(2i32);
pub const ActivityContentNone: OpenServiceActivityContentType = OpenServiceActivityContentType(-1i32);
pub const ActivityContentSelection: OpenServiceActivityContentType = OpenServiceActivityContentType(1i32);
pub const AnchorClick: windows_core::GUID = windows_core::GUID::from_u128(0x13d5413c_33b9_11d2_95a7_00c04f8ecb02);
pub const CATID_MSOfficeAntiVirus: windows_core::GUID = windows_core::GUID::from_u128(0x56ffcc30_d398_11d0_b2ae_00a0c908fa49);
pub const CDeviceRect: windows_core::GUID = windows_core::GUID::from_u128(0x3050f6d4_98b5_11cf_bb82_00aa00bdce0b);
pub const CDownloadBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x3050f5be_98b5_11cf_bb82_00aa00bdce0b);
pub const CHeaderFooter: windows_core::GUID = windows_core::GUID::from_u128(0x3050f6cd_98b5_11cf_bb82_00aa00bdce0b);
pub const CLayoutRect: windows_core::GUID = windows_core::GUID::from_u128(0x3050f664_98b5_11cf_bb82_00aa00bdce0b);
pub const COLOR_NO_TRANSPARENT: u32 = 4294967295u32;
pub const CPersistDataPeer: windows_core::GUID = windows_core::GUID::from_u128(0x3050f487_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistHistory: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c8_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistShortcut: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c6_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistSnapshot: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4c9_98b5_11cf_bb82_00aa00bdce0b);
pub const CPersistUserData: windows_core::GUID = windows_core::GUID::from_u128(0x3050f48e_98b5_11cf_bb82_00aa00bdce0b);
pub const CoDitherToRGB8: windows_core::GUID = windows_core::GUID::from_u128(0xa860ce50_3910_11d0_86fc_00a0c913f750);
pub const CoMapMIMEToCLSID: windows_core::GUID = windows_core::GUID::from_u128(0x30c3b080_30fb_11d0_b724_00aa006c1a01);
pub const CoSniffStream: windows_core::GUID = windows_core::GUID::from_u128(0x6a01fda0_30df_11d0_b724_00aa006c1a01);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtensionValidationContexts(pub i32);
pub const ExtensionValidationResultArrestPageLoad: ExtensionValidationResults = ExtensionValidationResults(2i32);
pub const ExtensionValidationResultDoNotInstantiate: ExtensionValidationResults = ExtensionValidationResults(1i32);
pub const ExtensionValidationResultNone: ExtensionValidationResults = ExtensionValidationResults(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExtensionValidationResults(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FINDFRAME_FLAGS(pub i32);
pub const FINDFRAME_INTERNAL: FINDFRAME_FLAGS = FINDFRAME_FLAGS(-2147483648i32);
pub const FINDFRAME_JUSTTESTEXISTENCE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(1i32);
pub const FINDFRAME_NONE: FINDFRAME_FLAGS = FINDFRAME_FLAGS(0i32);
pub const FRAMEOPTIONS_BROWSERBAND: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(64i32);
pub const FRAMEOPTIONS_DESKTOP: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(32i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FRAMEOPTIONS_FLAGS(pub i32);
pub const FRAMEOPTIONS_NO3DBORDER: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(16i32);
pub const FRAMEOPTIONS_NORESIZE: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(8i32);
pub const FRAMEOPTIONS_SCROLL_AUTO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(4i32);
pub const FRAMEOPTIONS_SCROLL_NO: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(2i32);
pub const FRAMEOPTIONS_SCROLL_YES: FRAMEOPTIONS_FLAGS = FRAMEOPTIONS_FLAGS(1i32);
pub const HomePage: windows_core::GUID = windows_core::GUID::from_u128(0x766bf2ae_d650_11d1_9811_00c04fc31d2e);
pub const HomePageSetting: windows_core::GUID = windows_core::GUID::from_u128(0x374cede0_873a_4c4f_bc86_bcc8cf5116a3);
windows_core::imp::define_interface!(IActiveXUIHandlerSite, IActiveXUIHandlerSite_Vtbl, 0x30510853_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite, windows_core::IUnknown);
impl IActiveXUIHandlerSite {
    pub unsafe fn CreateScrollableContextMenu(&self) -> windows_core::Result<IScrollableContextMenu> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateScrollableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn PickFileAndGetResult<P0>(&self, filepicker: P0, allowmultipleselections: bool) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PickFileAndGetResult)(windows_core::Interface::as_raw(self), filepicker.param().abi(), allowmultipleselections.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateScrollableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PickFileAndGetResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActiveXUIHandlerSite_Impl: windows_core::IUnknownImpl {
    fn CreateScrollableContextMenu(&self) -> windows_core::Result<IScrollableContextMenu>;
    fn PickFileAndGetResult(&self, filepicker: windows_core::Ref<windows_core::IUnknown>, allowmultipleselections: windows_core::BOOL) -> windows_core::Result<windows_core::IUnknown>;
}
impl IActiveXUIHandlerSite_Vtbl {
    pub const fn new<Identity: IActiveXUIHandlerSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateScrollableContextMenu<Identity: IActiveXUIHandlerSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollablecontextmenu: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveXUIHandlerSite_Impl::CreateScrollableContextMenu(this) {
                    Ok(ok__) => {
                        scrollablecontextmenu.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PickFileAndGetResult<Identity: IActiveXUIHandlerSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepicker: *mut core::ffi::c_void, allowmultipleselections: windows_core::BOOL, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveXUIHandlerSite_Impl::PickFileAndGetResult(this, core::mem::transmute_copy(&filepicker), core::mem::transmute_copy(&allowmultipleselections)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateScrollableContextMenu: CreateScrollableContextMenu::<Identity, OFFSET>,
            PickFileAndGetResult: PickFileAndGetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite {}
windows_core::imp::define_interface!(IActiveXUIHandlerSite2, IActiveXUIHandlerSite2_Vtbl, 0x7e3707b2_d087_4542_ac1f_a0d2fcd080fd);
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite2, windows_core::IUnknown);
impl IActiveXUIHandlerSite2 {
    pub unsafe fn AddSuspensionExemption(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddSuspensionExemption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveSuspensionExemption(&self, ullcookie: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveSuspensionExemption)(windows_core::Interface::as_raw(self), ullcookie).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSuspensionExemption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub RemoveSuspensionExemption: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
pub trait IActiveXUIHandlerSite2_Impl: windows_core::IUnknownImpl {
    fn AddSuspensionExemption(&self) -> windows_core::Result<u64>;
    fn RemoveSuspensionExemption(&self, ullcookie: u64) -> windows_core::Result<()>;
}
impl IActiveXUIHandlerSite2_Vtbl {
    pub const fn new<Identity: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSuspensionExemption<Identity: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullcookie: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveXUIHandlerSite2_Impl::AddSuspensionExemption(this) {
                    Ok(ok__) => {
                        pullcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSuspensionExemption<Identity: IActiveXUIHandlerSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcookie: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActiveXUIHandlerSite2_Impl::RemoveSuspensionExemption(this, core::mem::transmute_copy(&ullcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddSuspensionExemption: AddSuspensionExemption::<Identity, OFFSET>,
            RemoveSuspensionExemption: RemoveSuspensionExemption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite2 {}
windows_core::imp::define_interface!(IActiveXUIHandlerSite3, IActiveXUIHandlerSite3_Vtbl, 0x7904009a_1238_47f4_901c_871375c34608);
windows_core::imp::interface_hierarchy!(IActiveXUIHandlerSite3, windows_core::IUnknown);
impl IActiveXUIHandlerSite3 {
    pub unsafe fn MessageBoxW<P1, P2>(&self, hwnd: Option<super::super::Foundation::HWND>, text: P1, caption: P2, r#type: u32) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageBoxW)(windows_core::Interface::as_raw(self), hwnd.unwrap_or(core::mem::zeroed()) as _, text.param().abi(), caption.param().abi(), r#type, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActiveXUIHandlerSite3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MessageBoxW: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut i32) -> windows_core::HRESULT,
}
pub trait IActiveXUIHandlerSite3_Impl: windows_core::IUnknownImpl {
    fn MessageBoxW(&self, hwnd: super::super::Foundation::HWND, text: &windows_core::PCWSTR, caption: &windows_core::PCWSTR, r#type: u32) -> windows_core::Result<i32>;
}
impl IActiveXUIHandlerSite3_Vtbl {
    pub const fn new<Identity: IActiveXUIHandlerSite3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MessageBoxW<Identity: IActiveXUIHandlerSite3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, text: windows_core::PCWSTR, caption: windows_core::PCWSTR, r#type: u32, result: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActiveXUIHandlerSite3_Impl::MessageBoxW(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&text), core::mem::transmute(&caption), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        result.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MessageBoxW: MessageBoxW::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActiveXUIHandlerSite3 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActiveXUIHandlerSite3 {}
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
        unsafe { (windows_core::Interface::vtable(self).ProcOnClick)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAnchorClick_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ProcOnClick: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAnchorClick_Impl: super::super::System::Com::IDispatch_Impl {
    fn ProcOnClick(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAnchorClick_Vtbl {
    pub const fn new<Identity: IAnchorClick_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcOnClick<Identity: IAnchorClick_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAnchorClick_Impl::ProcOnClick(this).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ProcOnClick: ProcOnClick::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnchorClick as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAnchorClick {}
windows_core::imp::define_interface!(IAudioSessionSite, IAudioSessionSite_Vtbl, 0xd7d8b684_d02d_4517_b6b7_19e3dfe29c45);
windows_core::imp::interface_hierarchy!(IAudioSessionSite, windows_core::IUnknown);
impl IAudioSessionSite {
    pub unsafe fn GetAudioSessionGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioSessionGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnAudioStreamCreated<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAudioStreamCreated)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok() }
    }
    pub unsafe fn OnAudioStreamDestroyed<P0>(&self, endpointid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAudioStreamDestroyed)(windows_core::Interface::as_raw(self), endpointid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAudioSessionGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OnAudioStreamCreated: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnAudioStreamDestroyed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAudioSessionSite_Impl: windows_core::IUnknownImpl {
    fn GetAudioSessionGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn OnAudioStreamCreated(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnAudioStreamDestroyed(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAudioSessionSite_Vtbl {
    pub const fn new<Identity: IAudioSessionSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAudioSessionGuid<Identity: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionSite_Impl::GetAudioSessionGuid(this) {
                    Ok(ok__) => {
                        audiosessionguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnAudioStreamCreated<Identity: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionSite_Impl::OnAudioStreamCreated(this, core::mem::transmute(&endpointid)).into()
            }
        }
        unsafe extern "system" fn OnAudioStreamDestroyed<Identity: IAudioSessionSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionSite_Impl::OnAudioStreamDestroyed(this, core::mem::transmute(&endpointid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAudioSessionGuid: GetAudioSessionGuid::<Identity, OFFSET>,
            OnAudioStreamCreated: OnAudioStreamCreated::<Identity, OFFSET>,
            OnAudioStreamDestroyed: OnAudioStreamDestroyed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSessionSite {}
windows_core::imp::define_interface!(ICaretPositionProvider, ICaretPositionProvider_Vtbl, 0x58da43a2_108e_4d5b_9f75_e5f74f93fff5);
windows_core::imp::interface_hierarchy!(ICaretPositionProvider, windows_core::IUnknown);
impl ICaretPositionProvider {
    pub unsafe fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCaretPosition)(windows_core::Interface::as_raw(self), pptcaret as _, pflheight as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICaretPositionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCaretPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut f32) -> windows_core::HRESULT,
}
pub trait ICaretPositionProvider_Impl: windows_core::IUnknownImpl {
    fn GetCaretPosition(&self, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::Result<()>;
}
impl ICaretPositionProvider_Vtbl {
    pub const fn new<Identity: ICaretPositionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCaretPosition<Identity: ICaretPositionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICaretPositionProvider_Impl::GetCaretPosition(this, core::mem::transmute_copy(&pptcaret), core::mem::transmute_copy(&pflheight)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCaretPosition: GetCaretPosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICaretPositionProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICaretPositionProvider {}
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
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDeviceRect_Impl: super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDeviceRect_Vtbl {
    pub const fn new<Identity: IDeviceRect_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceRect as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDeviceRect {}
windows_core::imp::define_interface!(IDithererImpl, IDithererImpl_Vtbl, 0x7c48e840_3910_11d0_86fc_00a0c913f750);
windows_core::imp::interface_hierarchy!(IDithererImpl, windows_core::IUnknown);
impl IDithererImpl {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDestColorTable)(windows_core::Interface::as_raw(self), ncolors, prgbcolors).ok() }
    }
    pub unsafe fn SetEventSink<P0>(&self, peventsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImageDecodeEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), peventsink.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDithererImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetDestColorTable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetDestColorTable: usize,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDithererImpl_Impl: windows_core::IUnknownImpl {
    fn SetDestColorTable(&self, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::Result<()>;
    fn SetEventSink(&self, peventsink: windows_core::Ref<IImageDecodeEventSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IDithererImpl_Vtbl {
    pub const fn new<Identity: IDithererImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDestColorTable<Identity: IDithererImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDithererImpl_Impl::SetDestColorTable(this, core::mem::transmute_copy(&ncolors), core::mem::transmute_copy(&prgbcolors)).into()
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: IDithererImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDithererImpl_Impl::SetEventSink(this, core::mem::transmute_copy(&peventsink)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDestColorTable: SetDestColorTable::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDithererImpl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IDithererImpl {}
windows_core::imp::define_interface!(IDocObjectService, IDocObjectService_Vtbl, 0x3050f801_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IDocObjectService, windows_core::IUnknown);
impl IDocObjectService {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FireBeforeNavigate2<P0, P1, P3, P6>(&self, pdispatch: P0, lpszurl: P1, dwflags: u32, lpszframename: P3, ppostdata: *const u8, cbpostdata: u32, lpszheaders: P6, fplaynavsound: bool) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FireBeforeNavigate2)(windows_core::Interface::as_raw(self), pdispatch.param().abi(), lpszurl.param().abi(), dwflags, lpszframename.param().abi(), ppostdata, cbpostdata, lpszheaders.param().abi(), fplaynavsound.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FireDownloadBegin(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireDownloadBegin)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FireDownloadComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireDownloadComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetPendingUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPendingUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetUrlSearchComponent(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUrlSearchComponent)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsErrorUrl<P0>(&self, lpszurl: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsErrorUrl)(windows_core::Interface::as_raw(self), lpszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocObjectService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub FireBeforeNavigate2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *const u8, u32, windows_core::PCWSTR, windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FireBeforeNavigate2: usize,
    FireNavigateComplete2: usize,
    pub FireDownloadBegin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    FireDocumentComplete: usize,
    UpdateDesktopComponent: usize,
    pub GetPendingUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    ActiveElementChanged: usize,
    pub GetUrlSearchComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsErrorUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDocObjectService_Impl: windows_core::IUnknownImpl {
    fn FireBeforeNavigate2(&self, pdispatch: windows_core::Ref<super::super::System::Com::IDispatch>, lpszurl: &windows_core::PCWSTR, dwflags: u32, lpszframename: &windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: &windows_core::PCWSTR, fplaynavsound: windows_core::BOOL) -> windows_core::Result<windows_core::BOOL>;
    fn FireDownloadBegin(&self) -> windows_core::Result<()>;
    fn FireDownloadComplete(&self) -> windows_core::Result<()>;
    fn GetPendingUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetUrlSearchComponent(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsErrorUrl(&self, lpszurl: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDocObjectService_Vtbl {
    pub const fn new<Identity: IDocObjectService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FireBeforeNavigate2<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispatch: *mut core::ffi::c_void, lpszurl: windows_core::PCWSTR, dwflags: u32, lpszframename: windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: windows_core::PCWSTR, fplaynavsound: windows_core::BOOL, pfcancel: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDocObjectService_Impl::FireBeforeNavigate2(this, core::mem::transmute_copy(&pdispatch), core::mem::transmute(&lpszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute(&lpszframename), core::mem::transmute_copy(&ppostdata), core::mem::transmute_copy(&cbpostdata), core::mem::transmute(&lpszheaders), core::mem::transmute_copy(&fplaynavsound)) {
                    Ok(ok__) => {
                        pfcancel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FireDownloadBegin<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDocObjectService_Impl::FireDownloadBegin(this).into()
            }
        }
        unsafe extern "system" fn FireDownloadComplete<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDocObjectService_Impl::FireDownloadComplete(this).into()
            }
        }
        unsafe extern "system" fn GetPendingUrl<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpendingurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDocObjectService_Impl::GetPendingUrl(this) {
                    Ok(ok__) => {
                        pbstrpendingurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUrlSearchComponent<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsearch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDocObjectService_Impl::GetUrlSearchComponent(this) {
                    Ok(ok__) => {
                        pbstrsearch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsErrorUrl<Identity: IDocObjectService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszurl: windows_core::PCWSTR, pfiserror: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDocObjectService_Impl::IsErrorUrl(this, core::mem::transmute(&lpszurl)) {
                    Ok(ok__) => {
                        pfiserror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FireBeforeNavigate2: FireBeforeNavigate2::<Identity, OFFSET>,
            FireNavigateComplete2: 0,
            FireDownloadBegin: FireDownloadBegin::<Identity, OFFSET>,
            FireDownloadComplete: FireDownloadComplete::<Identity, OFFSET>,
            FireDocumentComplete: 0,
            UpdateDesktopComponent: 0,
            GetPendingUrl: GetPendingUrl::<Identity, OFFSET>,
            ActiveElementChanged: 0,
            GetUrlSearchComponent: GetUrlSearchComponent::<Identity, OFFSET>,
            IsErrorUrl: IsErrorUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDocObjectService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDocObjectService {}
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
    pub unsafe fn startDownload<P1>(&self, bstrurl: &windows_core::BSTR, pdispcallback: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).startDownload)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), pdispcallback.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadBehavior_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub startDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadBehavior_Impl: super::super::System::Com::IDispatch_Impl {
    fn startDownload(&self, bstrurl: &windows_core::BSTR, pdispcallback: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDownloadBehavior_Vtbl {
    pub const fn new<Identity: IDownloadBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn startDownload<Identity: IDownloadBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, pdispcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadBehavior_Impl::startDownload(this, core::mem::transmute(&bstrurl), core::mem::transmute_copy(&pdispcallback)).into()
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), startDownload: startDownload::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadBehavior as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDownloadBehavior {}
windows_core::imp::define_interface!(IDownloadManager, IDownloadManager_Vtbl, 0x988934a4_064b_11d3_bb80_00104b35e7f9);
windows_core::imp::interface_hierarchy!(IDownloadManager, windows_core::IUnknown);
impl IDownloadManager {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Download<P0, P1, P5, P6>(&self, pmk: P0, pbc: P1, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: P5, pszredir: P6, uicp: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IMoniker>,
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), dwbindverb, grfbindf, core::mem::transmute(pbindinfo), pszheaders.param().abi(), pszredir.param().abi(), uicp).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, i32, *const super::super::System::Com::BINDINFO, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    Download: usize,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDownloadManager_Impl: windows_core::IUnknownImpl {
    fn Download(&self, pmk: windows_core::Ref<super::super::System::Com::IMoniker>, pbc: windows_core::Ref<super::super::System::Com::IBindCtx>, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: &windows_core::PCWSTR, pszredir: &windows_core::PCWSTR, uicp: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IDownloadManager_Vtbl {
    pub const fn new<Identity: IDownloadManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Download<Identity: IDownloadManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: windows_core::PCWSTR, pszredir: windows_core::PCWSTR, uicp: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDownloadManager_Impl::Download(this, core::mem::transmute_copy(&pmk), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&dwbindverb), core::mem::transmute_copy(&grfbindf), core::mem::transmute_copy(&pbindinfo), core::mem::transmute(&pszheaders), core::mem::transmute(&pszredir), core::mem::transmute_copy(&uicp)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Download: Download::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDownloadManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IDownloadManager {}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IELAUNCHOPTION_FLAGS(pub i32);
pub const IELAUNCHOPTION_FORCE_COMPAT: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(2i32);
pub const IELAUNCHOPTION_FORCE_EDGE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(4i32);
pub const IELAUNCHOPTION_LOCK_ENGINE: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(8i32);
pub const IELAUNCHOPTION_SCRIPTDEBUG: IELAUNCHOPTION_FLAGS = IELAUNCHOPTION_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IELAUNCHURLINFO {
    pub cbSize: u32,
    pub dwCreationFlags: u32,
    pub dwLaunchOptionFlags: u32,
}
pub const IEPROCESS_MODULE_NAME: windows_core::PCWSTR = windows_core::w!("IERtUtil.dll");
pub const IEWebDriverManager: windows_core::GUID = windows_core::GUID::from_u128(0x90314af2_5250_47b3_89d8_6295fc23bc22);
pub const IE_USE_OE_MAIL_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_MAIL_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Internet Explorer\\Mail");
pub const IE_USE_OE_MAIL_VALUE: windows_core::PCWSTR = windows_core::w!("Use Outlook Express");
pub const IE_USE_OE_NEWS_HKEY: i32 = -2147483647i32;
pub const IE_USE_OE_NEWS_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Internet Explorer\\News");
pub const IE_USE_OE_NEWS_VALUE: windows_core::PCWSTR = windows_core::w!("Use Outlook Express");
pub const IE_USE_OE_PRESENT_HKEY: i32 = -2147483646i32;
pub const IE_USE_OE_PRESENT_KEY: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\app.paths\\msimn.exe");
windows_core::imp::define_interface!(IEnumManagerFrames, IEnumManagerFrames_Vtbl, 0x3caa826a_9b1f_4a79_bc81_f0430ded1648);
windows_core::imp::interface_hierarchy!(IEnumManagerFrames, windows_core::IUnknown);
impl IEnumManagerFrames {
    pub unsafe fn Next(&self, ppwindows: &mut [*mut super::super::Foundation::HWND], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppwindows.len().try_into().unwrap(), core::mem::transmute(ppwindows.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumManagerFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumManagerFrames_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::super::Foundation::HWND, *mut u32) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumManagerFrames_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Count(&self) -> windows_core::Result<u32>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumManagerFrames>;
}
impl IEnumManagerFrames_Vtbl {
    pub const fn new<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumManagerFrames_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&ppwindows), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Count<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumManagerFrames_Impl::Count(this) {
                    Ok(ok__) => {
                        pcelt.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumManagerFrames_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumManagerFrames_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumManagerFrames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumManagerFrames_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumManagerFrames as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumManagerFrames {}
windows_core::imp::define_interface!(IEnumOpenServiceActivity, IEnumOpenServiceActivity_Vtbl, 0xa436d7d2_17c3_4ef4_a1e8_5c86faff26c0);
windows_core::imp::interface_hierarchy!(IEnumOpenServiceActivity, windows_core::IUnknown);
impl IEnumOpenServiceActivity {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOpenServiceActivity>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOpenServiceActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOpenServiceActivity_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOpenServiceActivity>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivity>;
}
impl IEnumOpenServiceActivity_Vtbl {
    pub const fn new<Identity: IEnumOpenServiceActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivity_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivity_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivity_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOpenServiceActivity_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOpenServiceActivity {}
windows_core::imp::define_interface!(IEnumOpenServiceActivityCategory, IEnumOpenServiceActivityCategory_Vtbl, 0x33627a56_8c9a_4430_8fd1_b5f5c771afb6);
windows_core::imp::interface_hierarchy!(IEnumOpenServiceActivityCategory, windows_core::IUnknown);
impl IEnumOpenServiceActivityCategory {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOpenServiceActivityCategory>], pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivityCategory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumOpenServiceActivityCategory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumOpenServiceActivityCategory_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IOpenServiceActivityCategory>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumOpenServiceActivityCategory>;
}
impl IEnumOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivityCategory_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivityCategory_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumOpenServiceActivityCategory_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumOpenServiceActivityCategory_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumOpenServiceActivityCategory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumOpenServiceActivityCategory {}
windows_core::imp::define_interface!(IEnumSTATURL, IEnumSTATURL_Vtbl, 0x3c374a42_bae4_11cf_bf7d_00aa006946ee);
windows_core::imp::interface_hierarchy!(IEnumSTATURL, windows_core::IUnknown);
impl IEnumSTATURL {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _).ok() }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok() }
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATURL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFilter<P0>(&self, poszfilter: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFilter)(windows_core::Interface::as_raw(self), poszfilter.param().abi(), dwflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATURL_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATURL, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
}
pub trait IEnumSTATURL_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATURL>;
    fn SetFilter(&self, poszfilter: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
}
impl IEnumSTATURL_Vtbl {
    pub const fn new<Identity: IEnumSTATURL_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATURL_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATURL_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATURL_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATURL_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IEnumSTATURL_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poszfilter: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATURL_Impl::SetFilter(this, core::mem::transmute(&poszfilter), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATURL as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSTATURL {}
windows_core::imp::define_interface!(IExtensionValidation, IExtensionValidation_Vtbl, 0x7d33f73d_8525_4e0f_87db_830288baff44);
windows_core::imp::interface_hierarchy!(IExtensionValidation, windows_core::IUnknown);
impl IExtensionValidation {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtensionValidation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Validate: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IExtensionValidation_Impl: windows_core::IUnknownImpl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IExtensionValidation_Vtbl {
    pub const fn new<Identity: IExtensionValidation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: IExtensionValidation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExtensionValidation_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Validate: 0, DisplayName: DisplayName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtensionValidation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IExtensionValidation {}
windows_core::imp::define_interface!(IHTMLPersistData, IHTMLPersistData_Vtbl, 0x3050f4c5_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IHTMLPersistData, windows_core::IUnknown);
impl IHTMLPersistData {
    pub unsafe fn save<P0>(&self, punk: P0, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).save)(windows_core::Interface::as_raw(self), punk.param().abi(), ltype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn load<P0>(&self, punk: P0, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).load)(windows_core::Interface::as_raw(self), punk.param().abi(), ltype, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn queryType(&self, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).queryType)(windows_core::Interface::as_raw(self), ltype, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLPersistData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub queryType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
pub trait IHTMLPersistData_Impl: windows_core::IUnknownImpl {
    fn save(&self, punk: windows_core::Ref<windows_core::IUnknown>, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn load(&self, punk: windows_core::Ref<windows_core::IUnknown>, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn queryType(&self, ltype: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
impl IHTMLPersistData_Vtbl {
    pub const fn new<Identity: IHTMLPersistData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn save<Identity: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ltype: i32, fcontinuebroacast: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLPersistData_Impl::save(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ltype)) {
                    Ok(ok__) => {
                        fcontinuebroacast.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn load<Identity: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, ltype: i32, fdodefault: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLPersistData_Impl::load(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&ltype)) {
                    Ok(ok__) => {
                        fdodefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn queryType<Identity: IHTMLPersistData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltype: i32, pfsupportstype: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLPersistData_Impl::queryType(this, core::mem::transmute_copy(&ltype)) {
                    Ok(ok__) => {
                        pfsupportstype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            save: save::<Identity, OFFSET>,
            load: load::<Identity, OFFSET>,
            queryType: queryType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLPersistData as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHTMLPersistData {}
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
    pub unsafe fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XMLDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn setAttribute(&self, name: &windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLPersistDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub XMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLPersistDataOM_Impl: super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHTMLPersistDataOM_Vtbl {
    pub const fn new<Identity: IHTMLPersistDataOM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XMLDocument<Identity: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLPersistDataOM_Impl::XMLDocument(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAttribute<Identity: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLPersistDataOM_Impl::getAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLPersistDataOM_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IHTMLPersistDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLPersistDataOM_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLPersistDataOM as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IHTMLPersistDataOM {}
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
    pub unsafe fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XMLDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn save(&self, strname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).save)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname)).ok() }
    }
    pub unsafe fn load(&self, strname: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).load)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strname)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn setAttribute(&self, name: &windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).setAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)).ok() }
    }
    pub unsafe fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).removeAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)).ok() }
    }
    pub unsafe fn Setexpires(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Setexpires)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstr)).ok() }
    }
    pub unsafe fn expires(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).expires)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHTMLUserDataOM_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub XMLDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub getAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    getAttribute: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub setAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    setAttribute: usize,
    pub removeAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setexpires: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub expires: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLUserDataOM_Impl: super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn save(&self, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn load(&self, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(&self, name: &windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn removeAttribute(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Setexpires(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn expires(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHTMLUserDataOM_Vtbl {
    pub const fn new<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XMLDocument<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLUserDataOM_Impl::XMLDocument(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn save<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLUserDataOM_Impl::save(this, core::mem::transmute(&strname)).into()
            }
        }
        unsafe extern "system" fn load<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLUserDataOM_Impl::load(this, core::mem::transmute(&strname)).into()
            }
        }
        unsafe extern "system" fn getAttribute<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLUserDataOM_Impl::getAttribute(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setAttribute<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLUserDataOM_Impl::setAttribute(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn removeAttribute<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLUserDataOM_Impl::removeAttribute(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Setexpires<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHTMLUserDataOM_Impl::Setexpires(this, core::mem::transmute(&bstr)).into()
            }
        }
        unsafe extern "system" fn expires<Identity: IHTMLUserDataOM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHTMLUserDataOM_Impl::expires(this) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            XMLDocument: XMLDocument::<Identity, OFFSET>,
            save: save::<Identity, OFFSET>,
            load: load::<Identity, OFFSET>,
            getAttribute: getAttribute::<Identity, OFFSET>,
            setAttribute: setAttribute::<Identity, OFFSET>,
            removeAttribute: removeAttribute::<Identity, OFFSET>,
            Setexpires: Setexpires::<Identity, OFFSET>,
            expires: expires::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHTMLUserDataOM as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IHTMLUserDataOM {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).htmlHead)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn htmlFoot(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).htmlFoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettextHead(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettextHead)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn textHead(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).textHead)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettextFoot(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettextFoot)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn textFoot(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).textFoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Setpage(&self, v: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Setpage)(windows_core::Interface::as_raw(self), v).ok() }
    }
    pub unsafe fn page(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).page)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetpageTotal(&self, v: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetpageTotal)(windows_core::Interface::as_raw(self), v).ok() }
    }
    pub unsafe fn pageTotal(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).pageTotal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetURL(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn URL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).URL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Settitle(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Settitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetdateShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetdateShort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn dateShort(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dateShort)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetdateLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetdateLong)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn dateLong(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).dateLong)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettimeShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettimeShort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn timeShort(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).timeShort)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SettimeLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SettimeLong)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn timeLong(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).timeLong)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderFooter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub htmlHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub htmlFoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettextHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub textHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettextFoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub textFoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Setpage: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub page: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetpageTotal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub pageTotal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Settitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetdateShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dateShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetdateLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub dateLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettimeShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub timeShort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SettimeLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub timeLong: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter_Impl: super::super::System::Com::IDispatch_Impl {
    fn htmlHead(&self) -> windows_core::Result<windows_core::BSTR>;
    fn htmlFoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettextHead(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn textHead(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettextFoot(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn textFoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Setpage(&self, v: u32) -> windows_core::Result<()>;
    fn page(&self) -> windows_core::Result<u32>;
    fn SetpageTotal(&self, v: u32) -> windows_core::Result<()>;
    fn pageTotal(&self) -> windows_core::Result<u32>;
    fn SetURL(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Settitle(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdateShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn dateShort(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetdateLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn dateLong(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettimeShort(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn timeShort(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SettimeLong(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn timeLong(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHeaderFooter_Vtbl {
    pub const fn new<Identity: IHeaderFooter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn htmlHead<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::htmlHead(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn htmlFoot<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::htmlFoot(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettextHead<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SettextHead(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn textHead<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::textHead(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettextFoot<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SettextFoot(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn textFoot<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::textFoot(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setpage<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::Setpage(this, core::mem::transmute_copy(&v)).into()
            }
        }
        unsafe extern "system" fn page<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::page(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetpageTotal<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SetpageTotal(this, core::mem::transmute_copy(&v)).into()
            }
        }
        unsafe extern "system" fn pageTotal<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::pageTotal(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetURL<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SetURL(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn URL<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::URL(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Settitle<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::Settitle(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn title<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::title(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetdateShort<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SetdateShort(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn dateShort<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::dateShort(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetdateLong<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SetdateLong(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn dateLong<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::dateLong(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettimeShort<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SettimeShort(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn timeShort<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::timeShort(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SettimeLong<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter_Impl::SettimeLong(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn timeLong<Identity: IHeaderFooter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter_Impl::timeLong(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            htmlHead: htmlHead::<Identity, OFFSET>,
            htmlFoot: htmlFoot::<Identity, OFFSET>,
            SettextHead: SettextHead::<Identity, OFFSET>,
            textHead: textHead::<Identity, OFFSET>,
            SettextFoot: SettextFoot::<Identity, OFFSET>,
            textFoot: textFoot::<Identity, OFFSET>,
            Setpage: Setpage::<Identity, OFFSET>,
            page: page::<Identity, OFFSET>,
            SetpageTotal: SetpageTotal::<Identity, OFFSET>,
            pageTotal: pageTotal::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
            Settitle: Settitle::<Identity, OFFSET>,
            title: title::<Identity, OFFSET>,
            SetdateShort: SetdateShort::<Identity, OFFSET>,
            dateShort: dateShort::<Identity, OFFSET>,
            SetdateLong: SetdateLong::<Identity, OFFSET>,
            dateLong: dateLong::<Identity, OFFSET>,
            SettimeShort: SettimeShort::<Identity, OFFSET>,
            timeShort: timeShort::<Identity, OFFSET>,
            SettimeLong: SettimeLong::<Identity, OFFSET>,
            timeLong: timeLong::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderFooter as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IHeaderFooter {}
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
    pub unsafe fn Setfont(&self, v: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Setfont)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(v)).ok() }
    }
    pub unsafe fn font(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).font)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderFooter2_Vtbl {
    pub base__: IHeaderFooter_Vtbl,
    pub Setfont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub font: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter2_Impl: IHeaderFooter_Impl {
    fn Setfont(&self, v: &windows_core::BSTR) -> windows_core::Result<()>;
    fn font(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHeaderFooter2_Vtbl {
    pub const fn new<Identity: IHeaderFooter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Setfont<Identity: IHeaderFooter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHeaderFooter2_Impl::Setfont(this, core::mem::transmute(&v)).into()
            }
        }
        unsafe extern "system" fn font<Identity: IHeaderFooter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHeaderFooter2_Impl::font(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IHeaderFooter_Vtbl::new::<Identity, OFFSET>(), Setfont: Setfont::<Identity, OFFSET>, font: font::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHeaderFooter2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IHeaderFooter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IHeaderFooter2 {}
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
        unsafe { (windows_core::Interface::vtable(self).navigateHomePage)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn setHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).setHomePage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)).ok() }
    }
    pub unsafe fn isHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isHomePage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IHomePage_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigateHomePage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub isHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHomePage_Impl: super::super::System::Com::IDispatch_Impl {
    fn navigateHomePage(&self) -> windows_core::Result<()>;
    fn setHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn isHomePage(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IHomePage_Vtbl {
    pub const fn new<Identity: IHomePage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn navigateHomePage<Identity: IHomePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHomePage_Impl::navigateHomePage(this).into()
            }
        }
        unsafe extern "system" fn setHomePage<Identity: IHomePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHomePage_Impl::setHomePage(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        unsafe extern "system" fn isHomePage<Identity: IHomePage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHomePage_Impl::isHomePage(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            navigateHomePage: navigateHomePage::<Identity, OFFSET>,
            setHomePage: setHomePage::<Identity, OFFSET>,
            isHomePage: isHomePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHomePage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IHomePage {}
windows_core::imp::define_interface!(IHomePageSetting, IHomePageSetting_Vtbl, 0xfdfc244f_18fa_4ff2_b08e_1d618f3ffbe4);
windows_core::imp::interface_hierarchy!(IHomePageSetting, windows_core::IUnknown);
impl IHomePageSetting {
    pub unsafe fn SetHomePage<P1, P2>(&self, hwnd: super::super::Foundation::HWND, homepageuri: P1, brandingmessage: P2) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHomePage)(windows_core::Interface::as_raw(self), hwnd, homepageuri.param().abi(), brandingmessage.param().abi()).ok() }
    }
    pub unsafe fn IsHomePage<P0>(&self, uri: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHomePage)(windows_core::Interface::as_raw(self), uri.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHomePageToBrowserDefault(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHomePageToBrowserDefault)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHomePageSetting_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsHomePage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetHomePageToBrowserDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IHomePageSetting_Impl: windows_core::IUnknownImpl {
    fn SetHomePage(&self, hwnd: super::super::Foundation::HWND, homepageuri: &windows_core::PCWSTR, brandingmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsHomePage(&self, uri: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn SetHomePageToBrowserDefault(&self) -> windows_core::Result<()>;
}
impl IHomePageSetting_Vtbl {
    pub const fn new<Identity: IHomePageSetting_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHomePage<Identity: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, homepageuri: windows_core::PCWSTR, brandingmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHomePageSetting_Impl::SetHomePage(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&homepageuri), core::mem::transmute(&brandingmessage)).into()
            }
        }
        unsafe extern "system" fn IsHomePage<Identity: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: windows_core::PCWSTR, isdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHomePageSetting_Impl::IsHomePage(this, core::mem::transmute(&uri)) {
                    Ok(ok__) => {
                        isdefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHomePageToBrowserDefault<Identity: IHomePageSetting_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHomePageSetting_Impl::SetHomePageToBrowserDefault(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHomePage: SetHomePage::<Identity, OFFSET>,
            IsHomePage: IsHomePage::<Identity, OFFSET>,
            SetHomePageToBrowserDefault: SetHomePageToBrowserDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHomePageSetting as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IHomePageSetting {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecuteCommand)(windows_core::Interface::as_raw(self), command.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIEWebDriverManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub ExecuteCommand: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverManager_Impl: super::super::System::Com::IDispatch_Impl {
    fn ExecuteCommand(&self, command: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIEWebDriverManager_Vtbl {
    pub const fn new<Identity: IIEWebDriverManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExecuteCommand<Identity: IIEWebDriverManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, command: windows_core::PCWSTR, response: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIEWebDriverManager_Impl::ExecuteCommand(this, core::mem::transmute(&command)) {
                    Ok(ok__) => {
                        response.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), ExecuteCommand: ExecuteCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIEWebDriverManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIEWebDriverManager {}
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
        unsafe { (windows_core::Interface::vtable(self).WindowOperation)(windows_core::Interface::as_raw(self), operationcode, hwnd).ok() }
    }
    pub unsafe fn DetachWebdriver<P0>(&self, punkwd: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).DetachWebdriver)(windows_core::Interface::as_raw(self), punkwd.param().abi()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetCapabilityValue<P0, P1>(&self, punkwd: P0, capname: P1) -> windows_core::Result<super::super::System::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCapabilityValue)(windows_core::Interface::as_raw(self), punkwd.param().abi(), capname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIEWebDriverSite_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub WindowOperation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DetachWebdriver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetCapabilityValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetCapabilityValue: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverSite_Impl: super::super::System::Com::IDispatch_Impl {
    fn WindowOperation(&self, operationcode: u32, hwnd: u32) -> windows_core::Result<()>;
    fn DetachWebdriver(&self, punkwd: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCapabilityValue(&self, punkwd: windows_core::Ref<windows_core::IUnknown>, capname: &windows_core::PCWSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIEWebDriverSite_Vtbl {
    pub const fn new<Identity: IIEWebDriverSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowOperation<Identity: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationcode: u32, hwnd: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIEWebDriverSite_Impl::WindowOperation(this, core::mem::transmute_copy(&operationcode), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn DetachWebdriver<Identity: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkwd: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIEWebDriverSite_Impl::DetachWebdriver(this, core::mem::transmute_copy(&punkwd)).into()
            }
        }
        unsafe extern "system" fn GetCapabilityValue<Identity: IIEWebDriverSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkwd: *mut core::ffi::c_void, capname: windows_core::PCWSTR, capvalue: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIEWebDriverSite_Impl::GetCapabilityValue(this, core::mem::transmute_copy(&punkwd), core::mem::transmute(&capname)) {
                    Ok(ok__) => {
                        capvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WindowOperation: WindowOperation::<Identity, OFFSET>,
            DetachWebdriver: DetachWebdriver::<Identity, OFFSET>,
            GetCapabilityValue: GetCapabilityValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIEWebDriverSite as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIEWebDriverSite {}
windows_core::imp::define_interface!(IImageDecodeEventSink, IImageDecodeEventSink_Vtbl, 0xbaa342a0_2ded_11d0_86f4_00a0c913f750);
windows_core::imp::interface_hierarchy!(IImageDecodeEventSink, windows_core::IUnknown);
impl IImageDecodeEventSink {
    pub unsafe fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurface)(windows_core::Interface::as_raw(self), nwidth, nheight, bfid, npasses, dwhints, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBeginDecode)(windows_core::Interface::as_raw(self), pdwevents as _, pnformats as _, ppformats as _).ok() }
    }
    pub unsafe fn OnBitsComplete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnBitsComplete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnDecodeComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDecodeComplete)(windows_core::Interface::as_raw(self), hrstatus).ok() }
    }
    pub unsafe fn OnPalette(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnPalette)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnProgress(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), pbounds, bcomplete.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDecodeEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const windows_core::GUID, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnBeginDecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub OnBitsComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDecodeComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub OnPalette: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IImageDecodeEventSink_Impl: windows_core::IUnknownImpl {
    fn GetSurface(&self, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn OnBeginDecode(&self, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn OnBitsComplete(&self) -> windows_core::Result<()>;
    fn OnDecodeComplete(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn OnPalette(&self) -> windows_core::Result<()>;
    fn OnProgress(&self, pbounds: *const super::super::Foundation::RECT, bcomplete: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IImageDecodeEventSink_Vtbl {
    pub const fn new<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSurface<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nwidth: i32, nheight: i32, bfid: *const windows_core::GUID, npasses: u32, dwhints: u32, ppsurface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageDecodeEventSink_Impl::GetSurface(this, core::mem::transmute_copy(&nwidth), core::mem::transmute_copy(&nheight), core::mem::transmute_copy(&bfid), core::mem::transmute_copy(&npasses), core::mem::transmute_copy(&dwhints)) {
                    Ok(ok__) => {
                        ppsurface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnBeginDecode<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeEventSink_Impl::OnBeginDecode(this, core::mem::transmute_copy(&pdwevents), core::mem::transmute_copy(&pnformats), core::mem::transmute_copy(&ppformats)).into()
            }
        }
        unsafe extern "system" fn OnBitsComplete<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeEventSink_Impl::OnBitsComplete(this).into()
            }
        }
        unsafe extern "system" fn OnDecodeComplete<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeEventSink_Impl::OnDecodeComplete(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        unsafe extern "system" fn OnPalette<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeEventSink_Impl::OnPalette(this).into()
            }
        }
        unsafe extern "system" fn OnProgress<Identity: IImageDecodeEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbounds: *const super::super::Foundation::RECT, bcomplete: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeEventSink_Impl::OnProgress(this, core::mem::transmute_copy(&pbounds), core::mem::transmute_copy(&bcomplete)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSurface: GetSurface::<Identity, OFFSET>,
            OnBeginDecode: OnBeginDecode::<Identity, OFFSET>,
            OnBitsComplete: OnBitsComplete::<Identity, OFFSET>,
            OnDecodeComplete: OnDecodeComplete::<Identity, OFFSET>,
            OnPalette: OnPalette::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImageDecodeEventSink {}
windows_core::imp::define_interface!(IImageDecodeEventSink2, IImageDecodeEventSink2_Vtbl, 0x8ebd8a57_8a96_48c9_84a6_962e2db9c931);
impl core::ops::Deref for IImageDecodeEventSink2 {
    type Target = IImageDecodeEventSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageDecodeEventSink2, windows_core::IUnknown, IImageDecodeEventSink);
impl IImageDecodeEventSink2 {
    pub unsafe fn IsAlphaPremultRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAlphaPremultRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageDecodeEventSink2_Vtbl {
    pub base__: IImageDecodeEventSink_Vtbl,
    pub IsAlphaPremultRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IImageDecodeEventSink2_Impl: IImageDecodeEventSink_Impl {
    fn IsAlphaPremultRequired(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IImageDecodeEventSink2_Vtbl {
    pub const fn new<Identity: IImageDecodeEventSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsAlphaPremultRequired<Identity: IImageDecodeEventSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpremultalpha: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageDecodeEventSink2_Impl::IsAlphaPremultRequired(this) {
                    Ok(ok__) => {
                        pfpremultalpha.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IImageDecodeEventSink_Vtbl::new::<Identity, OFFSET>(), IsAlphaPremultRequired: IsAlphaPremultRequired::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeEventSink2 as windows_core::Interface>::IID || iid == &<IImageDecodeEventSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImageDecodeEventSink2 {}
windows_core::imp::define_interface!(IImageDecodeFilter, IImageDecodeFilter_Vtbl, 0xa3ccedf3_2de2_11d0_86f4_00a0c913f750);
windows_core::imp::interface_hierarchy!(IImageDecodeFilter, windows_core::IUnknown);
impl IImageDecodeFilter {
    pub unsafe fn Initialize<P0>(&self, peventsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IImageDecodeEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), peventsink.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Process<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Process)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok() }
    }
    pub unsafe fn Terminate(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), hrstatus).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
pub trait IImageDecodeFilter_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, peventsink: windows_core::Ref<IImageDecodeEventSink>) -> windows_core::Result<()>;
    fn Process(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Terminate(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IImageDecodeFilter_Vtbl {
    pub const fn new<Identity: IImageDecodeFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeFilter_Impl::Initialize(this, core::mem::transmute_copy(&peventsink)).into()
            }
        }
        unsafe extern "system" fn Process<Identity: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeFilter_Impl::Process(this, core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IImageDecodeFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageDecodeFilter_Impl::Terminate(this, core::mem::transmute_copy(&hrstatus)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Process: Process::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageDecodeFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IImageDecodeFilter {}
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
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Setenabled(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Setenabled)(windows_core::Interface::as_raw(self), bval).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIntelliForms_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Setenabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIntelliForms_Impl: super::super::System::Com::IDispatch_Impl {
    fn enabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Setenabled(&self, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIntelliForms_Vtbl {
    pub const fn new<Identity: IIntelliForms_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn enabled<Identity: IIntelliForms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIntelliForms_Impl::enabled(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Setenabled<Identity: IIntelliForms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIntelliForms_Impl::Setenabled(this, core::mem::transmute_copy(&bval)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIntelliForms as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIntelliForms {}
windows_core::imp::define_interface!(IInternetExplorerManager, IInternetExplorerManager_Vtbl, 0xacc84351_04ff_44f9_b23f_655ed168c6d5);
windows_core::imp::interface_hierarchy!(IInternetExplorerManager, windows_core::IUnknown);
impl IInternetExplorerManager {
    pub unsafe fn CreateObject<P1, T>(&self, dwconfig: u32, pszurl: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateObject)(windows_core::Interface::as_raw(self), dwconfig, pszurl.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetExplorerManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternetExplorerManager_Impl: windows_core::IUnknownImpl {
    fn CreateObject(&self, dwconfig: u32, pszurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IInternetExplorerManager_Vtbl {
    pub const fn new<Identity: IInternetExplorerManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateObject<Identity: IInternetExplorerManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: u32, pszurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInternetExplorerManager_Impl::CreateObject(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute(&pszurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetExplorerManager {}
windows_core::imp::define_interface!(IInternetExplorerManager2, IInternetExplorerManager2_Vtbl, 0xdfbb5136_9259_4895_b4a7_c1934429919a);
windows_core::imp::interface_hierarchy!(IInternetExplorerManager2, windows_core::IUnknown);
impl IInternetExplorerManager2 {
    pub unsafe fn EnumFrameWindows(&self) -> windows_core::Result<IEnumManagerFrames> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFrameWindows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternetExplorerManager2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumFrameWindows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInternetExplorerManager2_Impl: windows_core::IUnknownImpl {
    fn EnumFrameWindows(&self) -> windows_core::Result<IEnumManagerFrames>;
}
impl IInternetExplorerManager2_Vtbl {
    pub const fn new<Identity: IInternetExplorerManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumFrameWindows<Identity: IInternetExplorerManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInternetExplorerManager2_Impl::EnumFrameWindows(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EnumFrameWindows: EnumFrameWindows::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInternetExplorerManager2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInternetExplorerManager2 {}
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
    pub unsafe fn SetnextRect(&self, bstrelementid: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetnextRect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrelementid)).ok() }
    }
    pub unsafe fn nextRect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nextRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetcontentSrc(&self, varcontentsrc: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetcontentSrc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varcontentsrc)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn contentSrc(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentSrc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SethonorPageBreaks(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SethonorPageBreaks)(windows_core::Interface::as_raw(self), v).ok() }
    }
    pub unsafe fn honorPageBreaks(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).honorPageBreaks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SethonorPageRules(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SethonorPageRules)(windows_core::Interface::as_raw(self), v).ok() }
    }
    pub unsafe fn honorPageRules(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).honorPageRules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetnextRectElement<P0>(&self, pelem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetnextRectElement)(windows_core::Interface::as_raw(self), pelem.param().abi()).ok() }
    }
    pub unsafe fn nextRectElement(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).nextRectElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn contentDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).contentDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutRect_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetnextRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub nextRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetcontentSrc: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetcontentSrc: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub contentSrc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    contentSrc: usize,
    pub SethonorPageBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub honorPageBreaks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SethonorPageRules: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub honorPageRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetnextRectElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub nextRectElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub contentDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILayoutRect_Impl: super::super::System::Com::IDispatch_Impl {
    fn SetnextRect(&self, bstrelementid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn nextRect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetcontentSrc(&self, varcontentsrc: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn contentSrc(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SethonorPageBreaks(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn honorPageBreaks(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SethonorPageRules(&self, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn honorPageRules(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetnextRectElement(&self, pelem: windows_core::Ref<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn nextRectElement(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn contentDocument(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILayoutRect_Vtbl {
    pub const fn new<Identity: ILayoutRect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetnextRect<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrelementid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutRect_Impl::SetnextRect(this, core::mem::transmute(&bstrelementid)).into()
            }
        }
        unsafe extern "system" fn nextRect<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrelementid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::nextRect(this) {
                    Ok(ok__) => {
                        pbstrelementid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetcontentSrc<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varcontentsrc: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutRect_Impl::SetcontentSrc(this, core::mem::transmute(&varcontentsrc)).into()
            }
        }
        unsafe extern "system" fn contentSrc<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcontentsrc: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::contentSrc(this) {
                    Ok(ok__) => {
                        pvarcontentsrc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SethonorPageBreaks<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutRect_Impl::SethonorPageBreaks(this, core::mem::transmute_copy(&v)).into()
            }
        }
        unsafe extern "system" fn honorPageBreaks<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::honorPageBreaks(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SethonorPageRules<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutRect_Impl::SethonorPageRules(this, core::mem::transmute_copy(&v)).into()
            }
        }
        unsafe extern "system" fn honorPageRules<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::honorPageRules(this) {
                    Ok(ok__) => {
                        p.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetnextRectElement<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutRect_Impl::SetnextRectElement(this, core::mem::transmute_copy(&pelem)).into()
            }
        }
        unsafe extern "system" fn nextRectElement<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppelem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::nextRectElement(this) {
                    Ok(ok__) => {
                        ppelem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn contentDocument<Identity: ILayoutRect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILayoutRect_Impl::contentDocument(this) {
                    Ok(ok__) => {
                        pdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetnextRect: SetnextRect::<Identity, OFFSET>,
            nextRect: nextRect::<Identity, OFFSET>,
            SetcontentSrc: SetcontentSrc::<Identity, OFFSET>,
            contentSrc: contentSrc::<Identity, OFFSET>,
            SethonorPageBreaks: SethonorPageBreaks::<Identity, OFFSET>,
            honorPageBreaks: honorPageBreaks::<Identity, OFFSET>,
            SethonorPageRules: SethonorPageRules::<Identity, OFFSET>,
            honorPageRules: honorPageRules::<Identity, OFFSET>,
            SetnextRectElement: SetnextRectElement::<Identity, OFFSET>,
            nextRectElement: nextRectElement::<Identity, OFFSET>,
            contentDocument: contentDocument::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILayoutRect as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ILayoutRect {}
pub const IMGDECODE_EVENT_BEGINBITS: u32 = 4u32;
pub const IMGDECODE_EVENT_BITSCOMPLETE: u32 = 8u32;
pub const IMGDECODE_EVENT_PALETTE: u32 = 2u32;
pub const IMGDECODE_EVENT_PROGRESS: u32 = 1u32;
pub const IMGDECODE_EVENT_USEDDRAW: u32 = 16u32;
pub const IMGDECODE_HINT_BOTTOMUP: u32 = 2u32;
pub const IMGDECODE_HINT_FULLWIDTH: u32 = 4u32;
pub const IMGDECODE_HINT_TOPDOWN: u32 = 1u32;
windows_core::imp::define_interface!(IMapMIMEToCLSID, IMapMIMEToCLSID_Vtbl, 0xd9e89500_30fa_11d0_b724_00aa006c1a01);
windows_core::imp::interface_hierarchy!(IMapMIMEToCLSID, windows_core::IUnknown);
impl IMapMIMEToCLSID {
    pub unsafe fn EnableDefaultMappings(&self, benable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableDefaultMappings)(windows_core::Interface::as_raw(self), benable.into()).ok() }
    }
    pub unsafe fn MapMIMEToCLSID<P0>(&self, pszmimetype: P0, pclsid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MapMIMEToCLSID)(windows_core::Interface::as_raw(self), pszmimetype.param().abi(), pclsid).ok() }
    }
    pub unsafe fn SetMapping<P0>(&self, pszmimetype: P0, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMapping)(windows_core::Interface::as_raw(self), pszmimetype.param().abi(), dwmapmode, clsid).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapMIMEToCLSID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableDefaultMappings: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub MapMIMEToCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IMapMIMEToCLSID_Impl: windows_core::IUnknownImpl {
    fn EnableDefaultMappings(&self, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn MapMIMEToCLSID(&self, pszmimetype: &windows_core::PCWSTR, pclsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetMapping(&self, pszmimetype: &windows_core::PCWSTR, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IMapMIMEToCLSID_Vtbl {
    pub const fn new<Identity: IMapMIMEToCLSID_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableDefaultMappings<Identity: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMapMIMEToCLSID_Impl::EnableDefaultMappings(this, core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn MapMIMEToCLSID<Identity: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmimetype: windows_core::PCWSTR, pclsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMapMIMEToCLSID_Impl::MapMIMEToCLSID(this, core::mem::transmute(&pszmimetype), core::mem::transmute_copy(&pclsid)).into()
            }
        }
        unsafe extern "system" fn SetMapping<Identity: IMapMIMEToCLSID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmimetype: windows_core::PCWSTR, dwmapmode: u32, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMapMIMEToCLSID_Impl::SetMapping(this, core::mem::transmute(&pszmimetype), core::mem::transmute_copy(&dwmapmode), core::mem::transmute_copy(&clsid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnableDefaultMappings: EnableDefaultMappings::<Identity, OFFSET>,
            MapMIMEToCLSID: MapMIMEToCLSID::<Identity, OFFSET>,
            SetMapping: SetMapping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapMIMEToCLSID as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMapMIMEToCLSID {}
windows_core::imp::define_interface!(IMediaActivityNotifySite, IMediaActivityNotifySite_Vtbl, 0x8165cfef_179d_46c2_bc71_3fa726dc1f8d);
windows_core::imp::interface_hierarchy!(IMediaActivityNotifySite, windows_core::IUnknown);
impl IMediaActivityNotifySite {
    pub unsafe fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMediaActivityStarted)(windows_core::Interface::as_raw(self), mediaactivitytype).ok() }
    }
    pub unsafe fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMediaActivityStopped)(windows_core::Interface::as_raw(self), mediaactivitytype).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaActivityNotifySite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnMediaActivityStarted: unsafe extern "system" fn(*mut core::ffi::c_void, MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT,
    pub OnMediaActivityStopped: unsafe extern "system" fn(*mut core::ffi::c_void, MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT,
}
pub trait IMediaActivityNotifySite_Impl: windows_core::IUnknownImpl {
    fn OnMediaActivityStarted(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()>;
    fn OnMediaActivityStopped(&self, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::Result<()>;
}
impl IMediaActivityNotifySite_Vtbl {
    pub const fn new<Identity: IMediaActivityNotifySite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMediaActivityStarted<Identity: IMediaActivityNotifySite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaActivityNotifySite_Impl::OnMediaActivityStarted(this, core::mem::transmute_copy(&mediaactivitytype)).into()
            }
        }
        unsafe extern "system" fn OnMediaActivityStopped<Identity: IMediaActivityNotifySite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaActivityNotifySite_Impl::OnMediaActivityStopped(this, core::mem::transmute_copy(&mediaactivitytype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMediaActivityStarted: OnMediaActivityStarted::<Identity, OFFSET>,
            OnMediaActivityStopped: OnMediaActivityStopped::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaActivityNotifySite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMediaActivityNotifySite {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERNETEXPLORERCONFIGURATION(pub i32);
pub const INTERNETEXPLORERCONFIGURATION_HOST: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(1i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(2i32);
pub const INTERNETEXPLORERCONFIGURATION_WEB_DRIVER_EDGE: INTERNETEXPLORERCONFIGURATION = INTERNETEXPLORERCONFIGURATION(4i32);
windows_core::imp::define_interface!(IOpenService, IOpenService_Vtbl, 0xc2952ed1_6a89_4606_925f_1ed8b4be0630);
windows_core::imp::interface_hierarchy!(IOpenService, windows_core::IUnknown);
impl IOpenService {
    pub unsafe fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDefault)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefault(&self, fdefault: bool, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self), fdefault.into(), hwnd).ok() }
    }
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpenService_Impl: windows_core::IUnknownImpl {
    fn IsDefault(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetDefault(&self, fdefault: windows_core::BOOL, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IOpenService_Vtbl {
    pub const fn new<Identity: IOpenService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDefault<Identity: IOpenService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenService_Impl::IsDefault(this) {
                    Ok(ok__) => {
                        pfisdefault.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefault<Identity: IOpenService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdefault: windows_core::BOOL, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenService_Impl::SetDefault(this, core::mem::transmute_copy(&fdefault), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetID<Identity: IOpenService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenService_Impl::GetID(this) {
                    Ok(ok__) => {
                        pbstrid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsDefault: IsDefault::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpenService {}
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
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi()).ok() }
    }
    pub unsafe fn CanExecute<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanExecute)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanExecuteType)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Preview<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Preview)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi()).ok() }
    }
    pub unsafe fn CanPreview<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPreview)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPreviewType)(windows_core::Interface::as_raw(self), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStatusText<P0>(&self, pinput: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatusText)(windows_core::Interface::as_raw(self), pinput.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHomepageUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHomepageUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCategoryName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self, fsmallicon: bool) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), fsmallicon.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDescriptionFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescriptionFilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetDownloadUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDownloadUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetInstallUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstallUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivity_Vtbl {
    pub base__: IOpenService_Vtbl,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanExecuteType: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Preview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanPreviewType: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHomepageUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCategoryName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetDescriptionFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDownloadUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInstallUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IOpenServiceActivity_Impl: IOpenService_Impl {
    fn Execute(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>, poutput: windows_core::Ref<IOpenServiceActivityOutputContext>) -> windows_core::Result<()>;
    fn CanExecute(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>, poutput: windows_core::Ref<IOpenServiceActivityOutputContext>) -> windows_core::Result<windows_core::BOOL>;
    fn CanExecuteType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<windows_core::BOOL>;
    fn Preview(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>, poutput: windows_core::Ref<IOpenServiceActivityOutputContext>) -> windows_core::Result<()>;
    fn CanPreview(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>, poutput: windows_core::Ref<IOpenServiceActivityOutputContext>) -> windows_core::Result<windows_core::BOOL>;
    fn CanPreviewType(&self, r#type: OpenServiceActivityContentType) -> windows_core::Result<windows_core::BOOL>;
    fn GetStatusText(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>) -> windows_core::Result<windows_core::BSTR>;
    fn GetHomepageUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCategoryName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIcon(&self, fsmallicon: windows_core::BOOL) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetDescriptionFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetDownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetInstallUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnabled(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IOpenServiceActivity_Vtbl {
    pub const fn new<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Execute<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceActivity_Impl::Execute(this, core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn CanExecute<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, pfcanexecute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::CanExecute(this, core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)) {
                    Ok(ok__) => {
                        pfcanexecute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanExecuteType<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanexecute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::CanExecuteType(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pfcanexecute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Preview<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceActivity_Impl::Preview(this, core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)).into()
            }
        }
        unsafe extern "system" fn CanPreview<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, pfcanpreview: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::CanPreview(this, core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)) {
                    Ok(ok__) => {
                        pfcanpreview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanPreviewType<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanpreview: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::CanPreviewType(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pfcanpreview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStatusText<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, pbstrstatustext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetStatusText(this, core::mem::transmute_copy(&pinput)) {
                    Ok(ok__) => {
                        pbstrstatustext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHomepageUrl<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrhomepageurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetHomepageUrl(this) {
                    Ok(ok__) => {
                        pbstrhomepageurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdisplayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pbstrdisplayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategoryName<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcategoryname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetCategoryName(this) {
                    Ok(ok__) => {
                        pbstrcategoryname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstriconpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetIconPath(this) {
                    Ok(ok__) => {
                        pbstriconpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIcon<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsmallicon: windows_core::BOOL, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetIcon(this, core::mem::transmute_copy(&fsmallicon)) {
                    Ok(ok__) => {
                        phicon.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDescriptionFilePath<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxmlpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetDescriptionFilePath(this) {
                    Ok(ok__) => {
                        pbstrxmlpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDownloadUrl<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxmluri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetDownloadUrl(this) {
                    Ok(ok__) => {
                        pbstrxmluri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstallUrl<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinstalluri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::GetInstallUrl(this) {
                    Ok(ok__) => {
                        pbstrinstalluri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEnabled<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivity_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        pfisenabled.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IOpenServiceActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceActivity_Impl::SetEnabled(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self {
            base__: IOpenService_Vtbl::new::<Identity, OFFSET>(),
            Execute: Execute::<Identity, OFFSET>,
            CanExecute: CanExecute::<Identity, OFFSET>,
            CanExecuteType: CanExecuteType::<Identity, OFFSET>,
            Preview: Preview::<Identity, OFFSET>,
            CanPreview: CanPreview::<Identity, OFFSET>,
            CanPreviewType: CanPreviewType::<Identity, OFFSET>,
            GetStatusText: GetStatusText::<Identity, OFFSET>,
            GetHomepageUrl: GetHomepageUrl::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            GetCategoryName: GetCategoryName::<Identity, OFFSET>,
            GetIconPath: GetIconPath::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetDescriptionFilePath: GetDescriptionFilePath::<Identity, OFFSET>,
            GetDownloadUrl: GetDownloadUrl::<Identity, OFFSET>,
            GetInstallUrl: GetInstallUrl::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivity as windows_core::Interface>::IID || iid == &<IOpenService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IOpenServiceActivity {}
windows_core::imp::define_interface!(IOpenServiceActivityCategory, IOpenServiceActivityCategory_Vtbl, 0x850af9d6_7309_40b5_bdb8_786c106b2153);
windows_core::imp::interface_hierarchy!(IOpenServiceActivityCategory, windows_core::IUnknown);
impl IOpenServiceActivityCategory {
    pub unsafe fn HasDefaultActivity(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasDefaultActivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefaultActivity(&self) -> windows_core::Result<IOpenServiceActivity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultActivity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDefaultActivity<P0>(&self, pactivity: P0, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenServiceActivity>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultActivity)(windows_core::Interface::as_raw(self), pactivity.param().abi(), hwnd).ok() }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetActivityEnumerator<P0, P1>(&self, pinput: P0, poutput: P1) -> windows_core::Result<IEnumOpenServiceActivity>
    where
        P0: windows_core::Param<IOpenServiceActivityInput>,
        P1: windows_core::Param<IOpenServiceActivityOutputContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityEnumerator)(windows_core::Interface::as_raw(self), pinput.param().abi(), poutput.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityCategory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HasDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultActivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpenServiceActivityCategory_Impl: windows_core::IUnknownImpl {
    fn HasDefaultActivity(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetDefaultActivity(&self) -> windows_core::Result<IOpenServiceActivity>;
    fn SetDefaultActivity(&self, pactivity: windows_core::Ref<IOpenServiceActivity>, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetActivityEnumerator(&self, pinput: windows_core::Ref<IOpenServiceActivityInput>, poutput: windows_core::Ref<IOpenServiceActivityOutputContext>) -> windows_core::Result<IEnumOpenServiceActivity>;
}
impl IOpenServiceActivityCategory_Vtbl {
    pub const fn new<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HasDefaultActivity<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasdefaultactivity: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityCategory_Impl::HasDefaultActivity(this) {
                    Ok(ok__) => {
                        pfhasdefaultactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultActivity<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdefaultactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityCategory_Impl::GetDefaultActivity(this) {
                    Ok(ok__) => {
                        ppdefaultactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultActivity<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivity: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceActivityCategory_Impl::SetDefaultActivity(this, core::mem::transmute_copy(&pactivity), core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetName<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityCategory_Impl::GetName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityEnumerator<Identity: IOpenServiceActivityCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut core::ffi::c_void, poutput: *mut core::ffi::c_void, ppenumactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityCategory_Impl::GetActivityEnumerator(this, core::mem::transmute_copy(&pinput), core::mem::transmute_copy(&poutput)) {
                    Ok(ok__) => {
                        ppenumactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HasDefaultActivity: HasDefaultActivity::<Identity, OFFSET>,
            GetDefaultActivity: GetDefaultActivity::<Identity, OFFSET>,
            SetDefaultActivity: SetDefaultActivity::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetActivityEnumerator: GetActivityEnumerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityCategory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpenServiceActivityCategory {}
windows_core::imp::define_interface!(IOpenServiceActivityInput, IOpenServiceActivityInput_Vtbl, 0x75cb4db9_6da0_4da3_83ce_422b6a433346);
windows_core::imp::interface_hierarchy!(IOpenServiceActivityInput, windows_core::IUnknown);
impl IOpenServiceActivityInput {
    pub unsafe fn GetVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVariable)(windows_core::Interface::as_raw(self), pwzvariablename.param().abi(), pwzvariabletype.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HasVariable<P0, P1>(&self, pwzvariablename: P0, pwzvariabletype: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasVariable)(windows_core::Interface::as_raw(self), pwzvariablename.param().abi(), pwzvariabletype.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<OpenServiceActivityContentType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasVariable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OpenServiceActivityContentType) -> windows_core::HRESULT,
}
pub trait IOpenServiceActivityInput_Impl: windows_core::IUnknownImpl {
    fn GetVariable(&self, pwzvariablename: &windows_core::PCWSTR, pwzvariabletype: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BSTR>;
    fn HasVariable(&self, pwzvariablename: &windows_core::PCWSTR, pwzvariabletype: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GetType(&self) -> windows_core::Result<OpenServiceActivityContentType>;
}
impl IOpenServiceActivityInput_Vtbl {
    pub const fn new<Identity: IOpenServiceActivityInput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVariable<Identity: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzvariablename: windows_core::PCWSTR, pwzvariabletype: windows_core::PCWSTR, pbstrvariablecontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityInput_Impl::GetVariable(this, core::mem::transmute(&pwzvariablename), core::mem::transmute(&pwzvariabletype)) {
                    Ok(ok__) => {
                        pbstrvariablecontent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasVariable<Identity: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzvariablename: windows_core::PCWSTR, pwzvariabletype: windows_core::PCWSTR, pfhasvariable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityInput_Impl::HasVariable(this, core::mem::transmute(&pwzvariablename), core::mem::transmute(&pwzvariabletype)) {
                    Ok(ok__) => {
                        pfhasvariable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IOpenServiceActivityInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut OpenServiceActivityContentType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityInput_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVariable: GetVariable::<Identity, OFFSET>,
            HasVariable: HasVariable::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityInput as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpenServiceActivityInput {}
windows_core::imp::define_interface!(IOpenServiceActivityManager, IOpenServiceActivityManager_Vtbl, 0x8a2d0a9d_e920_4bdc_a291_d30f650bc4f1);
windows_core::imp::interface_hierarchy!(IOpenServiceActivityManager, windows_core::IUnknown);
impl IOpenServiceActivityManager {
    pub unsafe fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> windows_core::Result<IEnumOpenServiceActivityCategory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategoryEnumerator)(windows_core::Interface::as_raw(self), etype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActivityByID<P0>(&self, pwzactivityid: P0) -> windows_core::Result<IOpenServiceActivity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityByID)(windows_core::Interface::as_raw(self), pwzactivityid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActivityByHomepageAndCategory<P0, P1>(&self, pwzhomepage: P0, pwzcategory: P1) -> windows_core::Result<IOpenServiceActivity>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityByHomepageAndCategory)(windows_core::Interface::as_raw(self), pwzhomepage.param().abi(), pwzcategory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVersionCookie(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVersionCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCategoryEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, OpenServiceActivityContentType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityByID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityByHomepageAndCategory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IOpenServiceActivityManager_Impl: windows_core::IUnknownImpl {
    fn GetCategoryEnumerator(&self, etype: OpenServiceActivityContentType) -> windows_core::Result<IEnumOpenServiceActivityCategory>;
    fn GetActivityByID(&self, pwzactivityid: &windows_core::PCWSTR) -> windows_core::Result<IOpenServiceActivity>;
    fn GetActivityByHomepageAndCategory(&self, pwzhomepage: &windows_core::PCWSTR, pwzcategory: &windows_core::PCWSTR) -> windows_core::Result<IOpenServiceActivity>;
    fn GetVersionCookie(&self) -> windows_core::Result<u32>;
}
impl IOpenServiceActivityManager_Vtbl {
    pub const fn new<Identity: IOpenServiceActivityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCategoryEnumerator<Identity: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, etype: OpenServiceActivityContentType, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityManager_Impl::GetCategoryEnumerator(this, core::mem::transmute_copy(&etype)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityByID<Identity: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzactivityid: windows_core::PCWSTR, ppactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityManager_Impl::GetActivityByID(this, core::mem::transmute(&pwzactivityid)) {
                    Ok(ok__) => {
                        ppactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityByHomepageAndCategory<Identity: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzhomepage: windows_core::PCWSTR, pwzcategory: windows_core::PCWSTR, ppactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityManager_Impl::GetActivityByHomepageAndCategory(this, core::mem::transmute(&pwzhomepage), core::mem::transmute(&pwzcategory)) {
                    Ok(ok__) => {
                        ppactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVersionCookie<Identity: IOpenServiceActivityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversioncookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityManager_Impl::GetVersionCookie(this) {
                    Ok(ok__) => {
                        pdwversioncookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCategoryEnumerator: GetCategoryEnumerator::<Identity, OFFSET>,
            GetActivityByID: GetActivityByID::<Identity, OFFSET>,
            GetActivityByHomepageAndCategory: GetActivityByHomepageAndCategory::<Identity, OFFSET>,
            GetVersionCookie: GetVersionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpenServiceActivityManager {}
windows_core::imp::define_interface!(IOpenServiceActivityOutputContext, IOpenServiceActivityOutputContext_Vtbl, 0xe289deab_f709_49a9_b99e_282364074571);
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
        unsafe { (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), pwzmethod.param().abi(), pwzheaders.param().abi(), ppostdata.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CanNavigate<P0, P1, P2, P3>(&self, pwzuri: P0, pwzmethod: P1, pwzheaders: P2, ppostdata: P3) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanNavigate)(windows_core::Interface::as_raw(self), pwzuri.param().abi(), pwzmethod.param().abi(), pwzheaders.param().abi(), ppostdata.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceActivityOutputContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Navigate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CanNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CanNavigate: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpenServiceActivityOutputContext_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, pwzuri: &windows_core::PCWSTR, pwzmethod: &windows_core::PCWSTR, pwzheaders: &windows_core::PCWSTR, ppostdata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CanNavigate(&self, pwzuri: &windows_core::PCWSTR, pwzmethod: &windows_core::PCWSTR, pwzheaders: &windows_core::PCWSTR, ppostdata: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_System_Com")]
impl IOpenServiceActivityOutputContext_Vtbl {
    pub const fn new<Identity: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate<Identity: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, pwzmethod: windows_core::PCWSTR, pwzheaders: windows_core::PCWSTR, ppostdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceActivityOutputContext_Impl::Navigate(this, core::mem::transmute(&pwzuri), core::mem::transmute(&pwzmethod), core::mem::transmute(&pwzheaders), core::mem::transmute_copy(&ppostdata)).into()
            }
        }
        unsafe extern "system" fn CanNavigate<Identity: IOpenServiceActivityOutputContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzuri: windows_core::PCWSTR, pwzmethod: windows_core::PCWSTR, pwzheaders: windows_core::PCWSTR, ppostdata: *mut core::ffi::c_void, pfcannavigate: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceActivityOutputContext_Impl::CanNavigate(this, core::mem::transmute(&pwzuri), core::mem::transmute(&pwzmethod), core::mem::transmute(&pwzheaders), core::mem::transmute_copy(&ppostdata)) {
                    Ok(ok__) => {
                        pfcannavigate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, OFFSET>,
            CanNavigate: CanNavigate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceActivityOutputContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IOpenServiceActivityOutputContext {}
windows_core::imp::define_interface!(IOpenServiceManager, IOpenServiceManager_Vtbl, 0x5664125f_4e10_4e90_98e4_e4513d955a14);
windows_core::imp::interface_hierarchy!(IOpenServiceManager, windows_core::IUnknown);
impl IOpenServiceManager {
    pub unsafe fn InstallService<P0>(&self, pwzserviceurl: P0) -> windows_core::Result<IOpenService>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallService)(windows_core::Interface::as_raw(self), pwzserviceurl.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn UninstallService<P0>(&self, pservice: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOpenService>,
    {
        unsafe { (windows_core::Interface::vtable(self).UninstallService)(windows_core::Interface::as_raw(self), pservice.param().abi()).ok() }
    }
    pub unsafe fn GetServiceByID<P0>(&self, pwzid: P0) -> windows_core::Result<IOpenService>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceByID)(windows_core::Interface::as_raw(self), pwzid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOpenServiceManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InstallService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninstallService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetServiceByID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOpenServiceManager_Impl: windows_core::IUnknownImpl {
    fn InstallService(&self, pwzserviceurl: &windows_core::PCWSTR) -> windows_core::Result<IOpenService>;
    fn UninstallService(&self, pservice: windows_core::Ref<IOpenService>) -> windows_core::Result<()>;
    fn GetServiceByID(&self, pwzid: &windows_core::PCWSTR) -> windows_core::Result<IOpenService>;
}
impl IOpenServiceManager_Vtbl {
    pub const fn new<Identity: IOpenServiceManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InstallService<Identity: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzserviceurl: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceManager_Impl::InstallService(this, core::mem::transmute(&pwzserviceurl)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UninstallService<Identity: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOpenServiceManager_Impl::UninstallService(this, core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn GetServiceByID<Identity: IOpenServiceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOpenServiceManager_Impl::GetServiceByID(this, core::mem::transmute(&pwzid)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InstallService: InstallService::<Identity, OFFSET>,
            UninstallService: UninstallService::<Identity, OFFSET>,
            GetServiceByID: GetServiceByID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenServiceManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOpenServiceManager {}
windows_core::imp::define_interface!(IPeerFactory, IPeerFactory_Vtbl, 0x6663f9d3_b482_11d1_89c6_00c04fb6bfc4);
windows_core::imp::interface_hierarchy!(IPeerFactory, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IPeerFactory_Impl: windows_core::IUnknownImpl {}
impl IPeerFactory_Vtbl {
    pub const fn new<Identity: IPeerFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPeerFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPeerFactory {}
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
    pub unsafe fn LoadHistory<P0, P1>(&self, pstream: P0, pbc: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadHistory)(windows_core::Interface::as_raw(self), pstream.param().abi(), pbc.param().abi()).ok() }
    }
    pub unsafe fn SaveHistory<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveHistory)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok() }
    }
    pub unsafe fn SetPositionCookie(&self, dwpositioncookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPositionCookie)(windows_core::Interface::as_raw(self), dwpositioncookie).ok() }
    }
    pub unsafe fn GetPositionCookie(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPositionCookie)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPersistHistory_Vtbl {
    pub base__: super::super::System::Com::IPersist_Vtbl,
    pub LoadHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveHistory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPositionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetPositionCookie: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistHistory_Impl: super::super::System::Com::IPersist_Impl {
    fn LoadHistory(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>, pbc: windows_core::Ref<super::super::System::Com::IBindCtx>) -> windows_core::Result<()>;
    fn SaveHistory(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetPositionCookie(&self, dwpositioncookie: u32) -> windows_core::Result<()>;
    fn GetPositionCookie(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IPersistHistory_Vtbl {
    pub const fn new<Identity: IPersistHistory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadHistory<Identity: IPersistHistory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistHistory_Impl::LoadHistory(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pbc)).into()
            }
        }
        unsafe extern "system" fn SaveHistory<Identity: IPersistHistory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistHistory_Impl::SaveHistory(this, core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn SetPositionCookie<Identity: IPersistHistory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwpositioncookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistHistory_Impl::SetPositionCookie(this, core::mem::transmute_copy(&dwpositioncookie)).into()
            }
        }
        unsafe extern "system" fn GetPositionCookie<Identity: IPersistHistory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpositioncookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistHistory_Impl::GetPositionCookie(this) {
                    Ok(ok__) => {
                        pdwpositioncookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::super::System::Com::IPersist_Vtbl::new::<Identity, OFFSET>(),
            LoadHistory: LoadHistory::<Identity, OFFSET>,
            SaveHistory: SaveHistory::<Identity, OFFSET>,
            SetPositionCookie: SetPositionCookie::<Identity, OFFSET>,
            GetPositionCookie: GetPositionCookie::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistHistory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IPersist as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IPersistHistory {}
windows_core::imp::define_interface!(IPrintTaskRequestFactory, IPrintTaskRequestFactory_Vtbl, 0xbb516745_8c34_4f8b_9605_684dcb144be5);
windows_core::imp::interface_hierarchy!(IPrintTaskRequestFactory, windows_core::IUnknown);
impl IPrintTaskRequestFactory {
    pub unsafe fn CreatePrintTaskRequest<P0>(&self, pprinttaskrequesthandler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPrintTaskRequestHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreatePrintTaskRequest)(windows_core::Interface::as_raw(self), pprinttaskrequesthandler.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreatePrintTaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintTaskRequestFactory_Impl: windows_core::IUnknownImpl {
    fn CreatePrintTaskRequest(&self, pprinttaskrequesthandler: windows_core::Ref<IPrintTaskRequestHandler>) -> windows_core::Result<()>;
}
impl IPrintTaskRequestFactory_Vtbl {
    pub const fn new<Identity: IPrintTaskRequestFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreatePrintTaskRequest<Identity: IPrintTaskRequestFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprinttaskrequesthandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTaskRequestFactory_Impl::CreatePrintTaskRequest(this, core::mem::transmute_copy(&pprinttaskrequesthandler)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePrintTaskRequest: CreatePrintTaskRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintTaskRequestFactory {}
windows_core::imp::define_interface!(IPrintTaskRequestHandler, IPrintTaskRequestHandler_Vtbl, 0x191cd340_cf36_44ff_bd53_d1b701799d9b);
windows_core::imp::interface_hierarchy!(IPrintTaskRequestHandler, windows_core::IUnknown);
impl IPrintTaskRequestHandler {
    pub unsafe fn HandlePrintTaskRequest<P0>(&self, pprinttaskrequest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandlePrintTaskRequest)(windows_core::Interface::as_raw(self), pprinttaskrequest.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskRequestHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandlePrintTaskRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPrintTaskRequestHandler_Impl: windows_core::IUnknownImpl {
    fn HandlePrintTaskRequest(&self, pprinttaskrequest: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl IPrintTaskRequestHandler_Vtbl {
    pub const fn new<Identity: IPrintTaskRequestHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandlePrintTaskRequest<Identity: IPrintTaskRequestHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprinttaskrequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintTaskRequestHandler_Impl::HandlePrintTaskRequest(this, core::mem::transmute_copy(&pprinttaskrequest)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePrintTaskRequest: HandlePrintTaskRequest::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintTaskRequestHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPrintTaskRequestHandler {}
windows_core::imp::define_interface!(IScrollableContextMenu, IScrollableContextMenu_Vtbl, 0x30510854_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IScrollableContextMenu, windows_core::IUnknown);
impl IScrollableContextMenu {
    pub unsafe fn AddItem<P0>(&self, itemtext: P0, cmdid: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddItem)(windows_core::Interface::as_raw(self), itemtext.param().abi(), cmdid).ok() }
    }
    pub unsafe fn ShowModal(&self, x: i32, y: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowModal)(windows_core::Interface::as_raw(self), x, y, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollableContextMenu_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub ShowModal: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut u32) -> windows_core::HRESULT,
}
pub trait IScrollableContextMenu_Impl: windows_core::IUnknownImpl {
    fn AddItem(&self, itemtext: &windows_core::PCWSTR, cmdid: u32) -> windows_core::Result<()>;
    fn ShowModal(&self, x: i32, y: i32) -> windows_core::Result<u32>;
}
impl IScrollableContextMenu_Vtbl {
    pub const fn new<Identity: IScrollableContextMenu_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddItem<Identity: IScrollableContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemtext: windows_core::PCWSTR, cmdid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollableContextMenu_Impl::AddItem(this, core::mem::transmute(&itemtext), core::mem::transmute_copy(&cmdid)).into()
            }
        }
        unsafe extern "system" fn ShowModal<Identity: IScrollableContextMenu_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32, cmdid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollableContextMenu_Impl::ShowModal(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                    Ok(ok__) => {
                        cmdid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddItem: AddItem::<Identity, OFFSET>, ShowModal: ShowModal::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScrollableContextMenu {}
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
        unsafe { (windows_core::Interface::vtable(self).AddSeparator)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlacement)(windows_core::Interface::as_raw(self), scmp).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollableContextMenu2_Vtbl {
    pub base__: IScrollableContextMenu_Vtbl,
    pub AddSeparator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(*mut core::ffi::c_void, SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::HRESULT,
}
pub trait IScrollableContextMenu2_Impl: IScrollableContextMenu_Impl {
    fn AddSeparator(&self) -> windows_core::Result<()>;
    fn SetPlacement(&self, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::Result<()>;
}
impl IScrollableContextMenu2_Vtbl {
    pub const fn new<Identity: IScrollableContextMenu2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSeparator<Identity: IScrollableContextMenu2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollableContextMenu2_Impl::AddSeparator(this).into()
            }
        }
        unsafe extern "system" fn SetPlacement<Identity: IScrollableContextMenu2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollableContextMenu2_Impl::SetPlacement(this, core::mem::transmute_copy(&scmp)).into()
            }
        }
        Self {
            base__: IScrollableContextMenu_Vtbl::new::<Identity, OFFSET>(),
            AddSeparator: AddSeparator::<Identity, OFFSET>,
            SetPlacement: SetPlacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollableContextMenu2 as windows_core::Interface>::IID || iid == &<IScrollableContextMenu as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScrollableContextMenu2 {}
windows_core::imp::define_interface!(ISniffStream, ISniffStream_Vtbl, 0x4ef17940_30e0_11d0_b724_00aa006c1a01);
windows_core::imp::interface_hierarchy!(ISniffStream, windows_core::IUnknown);
impl ISniffStream {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Init<P0>(&self, pstream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::System::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pstream.param().abi()).ok() }
    }
    pub unsafe fn Peek(&self, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Peek)(windows_core::Interface::as_raw(self), pbuffer as _, nbytes, pnbytesread as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISniffStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Init: usize,
    pub Peek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISniffStream_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pstream: windows_core::Ref<super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Peek(&self, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ISniffStream_Vtbl {
    pub const fn new<Identity: ISniffStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ISniffStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISniffStream_Impl::Init(this, core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn Peek<Identity: ISniffStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISniffStream_Impl::Peek(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&nbytes), core::mem::transmute_copy(&pnbytesread)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, OFFSET>, Peek: Peek::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISniffStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISniffStream {}
windows_core::imp::define_interface!(ISurfacePresenterFlip, ISurfacePresenterFlip_Vtbl, 0x30510848_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlip, windows_core::IUnknown);
impl ISurfacePresenterFlip {
    pub unsafe fn Present(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Present)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetBuffer(&self, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), backbufferindex, riid, ppbuffer as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Present: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISurfacePresenterFlip_Impl: windows_core::IUnknownImpl {
    fn Present(&self) -> windows_core::Result<()>;
    fn GetBuffer(&self, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl ISurfacePresenterFlip_Vtbl {
    pub const fn new<Identity: ISurfacePresenterFlip_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Present<Identity: ISurfacePresenterFlip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfacePresenterFlip_Impl::Present(this).into()
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: ISurfacePresenterFlip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backbufferindex: u32, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfacePresenterFlip_Impl::GetBuffer(this, core::mem::transmute_copy(&backbufferindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Present: Present::<Identity, OFFSET>, GetBuffer: GetBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurfacePresenterFlip {}
windows_core::imp::define_interface!(ISurfacePresenterFlip2, ISurfacePresenterFlip2_Vtbl, 0x30510865_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlip2, windows_core::IUnknown);
impl ISurfacePresenterFlip2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), dxgirotation).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlip2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetRotation: usize,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ISurfacePresenterFlip2_Impl: windows_core::IUnknownImpl {
    fn SetRotation(&self, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ISurfacePresenterFlip2_Vtbl {
    pub const fn new<Identity: ISurfacePresenterFlip2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRotation<Identity: ISurfacePresenterFlip2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfacePresenterFlip2_Impl::SetRotation(this, core::mem::transmute_copy(&dxgirotation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRotation: SetRotation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlip2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for ISurfacePresenterFlip2 {}
windows_core::imp::define_interface!(ISurfacePresenterFlipBuffer, ISurfacePresenterFlipBuffer_Vtbl, 0xe43f4a08_8bbc_4665_ac92_c55ce61fd7e7);
windows_core::imp::interface_hierarchy!(ISurfacePresenterFlipBuffer, windows_core::IUnknown);
impl ISurfacePresenterFlipBuffer {
    pub unsafe fn BeginDraw(&self, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self), riid, ppbuffer as _).ok() }
    }
    pub unsafe fn EndDraw(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfacePresenterFlipBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISurfacePresenterFlipBuffer_Impl: windows_core::IUnknownImpl {
    fn BeginDraw(&self, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EndDraw(&self) -> windows_core::Result<()>;
}
impl ISurfacePresenterFlipBuffer_Vtbl {
    pub const fn new<Identity: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginDraw<Identity: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfacePresenterFlipBuffer_Impl::BeginDraw(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppbuffer)).into()
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ISurfacePresenterFlipBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurfacePresenterFlipBuffer_Impl::EndDraw(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), BeginDraw: BeginDraw::<Identity, OFFSET>, EndDraw: EndDraw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurfacePresenterFlipBuffer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurfacePresenterFlipBuffer {}
windows_core::imp::define_interface!(ITargetContainer, ITargetContainer_Vtbl, 0x7847ec01_2bec_11d0_82b4_00a0c90c29c5);
windows_core::imp::interface_hierarchy!(ITargetContainer, windows_core::IUnknown);
impl ITargetContainer {
    pub unsafe fn GetFrameUrl(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrameUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Ole")]
    pub GetFramesContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    GetFramesContainer: usize,
}
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetContainer_Impl: windows_core::IUnknownImpl {
    fn GetFrameUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetContainer_Vtbl {
    pub const fn new<Identity: ITargetContainer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrameUrl<Identity: ITargetContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetContainer_Impl::GetFrameUrl(this) {
                    Ok(ok__) => {
                        ppszframesrc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ITargetContainer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetContainer_Impl::GetFramesContainer(this) {
                    Ok(ok__) => {
                        ppcontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameUrl: GetFrameUrl::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetContainer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetContainer {}
windows_core::imp::define_interface!(ITargetEmbedding, ITargetEmbedding_Vtbl, 0x548793c0_9e74_11cf_9655_00a0c9034923);
windows_core::imp::interface_hierarchy!(ITargetEmbedding, windows_core::IUnknown);
impl ITargetEmbedding {
    pub unsafe fn GetTargetFrame(&self) -> windows_core::Result<ITargetFrame> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetEmbedding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTargetFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITargetEmbedding_Impl: windows_core::IUnknownImpl {
    fn GetTargetFrame(&self) -> windows_core::Result<ITargetFrame>;
}
impl ITargetEmbedding_Vtbl {
    pub const fn new<Identity: ITargetEmbedding_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTargetFrame<Identity: ITargetEmbedding_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetEmbedding_Impl::GetTargetFrame(this) {
                    Ok(ok__) => {
                        pptargetframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTargetFrame: GetTargetFrame::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetEmbedding as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITargetEmbedding {}
windows_core::imp::define_interface!(ITargetFrame, ITargetFrame_Vtbl, 0xd5f78c80_5252_11cf_90fa_00aa0042106e);
windows_core::imp::interface_hierarchy!(ITargetFrame, windows_core::IUnknown);
impl ITargetFrame {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFrameName)(windows_core::Interface::as_raw(self), pszframename.param().abi()).ok() }
    }
    pub unsafe fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFrame<P0, P1>(&self, psztargetname: P0, ppunkcontextframe: P1, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFrame)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), ppunkcontextframe.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFrameSrc)(windows_core::Interface::as_raw(self), pszframesrc.param().abi()).ok() }
    }
    pub unsafe fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameSrc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrameOptions)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetFrameOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrameMargins)(windows_core::Interface::as_raw(self), dwwidth, dwheight).ok() }
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameMargins)(windows_core::Interface::as_raw(self), pdwwidth as _, pdwheight as _).ok() }
    }
    pub unsafe fn RemoteNavigate(&self, puldata: &[u32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoteNavigate)(windows_core::Interface::as_raw(self), puldata.len().try_into().unwrap(), core::mem::transmute(puldata.as_ptr())).ok() }
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChildFrameActivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok() }
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChildFrameDeactivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame_Impl: windows_core::IUnknownImpl {
    fn SetFrameName(&self, pszframename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn FindFrame(&self, psztargetname: &windows_core::PCWSTR, ppunkcontextframe: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrameOptions(&self) -> windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
    fn RemoteNavigate(&self, clength: u32, puldata: *const u32) -> windows_core::Result<()>;
    fn OnChildFrameActivate(&self, punkchildframe: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame_Vtbl {
    pub const fn new<Identity: ITargetFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFrameName<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::SetFrameName(this, core::mem::transmute(&pszframename)).into()
            }
        }
        unsafe extern "system" fn GetFrameName<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::GetFrameName(this) {
                    Ok(ok__) => {
                        ppszframename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::GetParentFrame(this) {
                    Ok(ok__) => {
                        ppunkparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFrame<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, ppunkcontextframe: *mut core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::FindFrame(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&ppunkcontextframe), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppunktargetframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframesrc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::SetFrameSrc(this, core::mem::transmute(&pszframesrc)).into()
            }
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::GetFrameSrc(this) {
                    Ok(ok__) => {
                        ppszframesrc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::GetFramesContainer(this) {
                    Ok(ok__) => {
                        ppcontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::SetFrameOptions(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame_Impl::GetFrameOptions(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32, dwheight: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::SetFrameMargins(this, core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight)).into()
            }
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::GetFrameMargins(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
            }
        }
        unsafe extern "system" fn RemoteNavigate<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clength: u32, puldata: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::RemoteNavigate(this, core::mem::transmute_copy(&clength), core::mem::transmute_copy(&puldata)).into()
            }
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::OnChildFrameActivate(this, core::mem::transmute_copy(&punkchildframe)).into()
            }
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ITargetFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame_Impl::OnChildFrameDeactivate(this, core::mem::transmute_copy(&punkchildframe)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, OFFSET>,
            GetFrameName: GetFrameName::<Identity, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, OFFSET>,
            FindFrame: FindFrame::<Identity, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, OFFSET>,
            RemoteNavigate: RemoteNavigate::<Identity, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFrame as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetFrame {}
windows_core::imp::define_interface!(ITargetFrame2, ITargetFrame2_Vtbl, 0x86d52e11_94a8_11d0_82af_00c04fd5ae38);
windows_core::imp::interface_hierarchy!(ITargetFrame2, windows_core::IUnknown);
impl ITargetFrame2 {
    pub unsafe fn SetFrameName<P0>(&self, pszframename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFrameName)(windows_core::Interface::as_raw(self), pszframename.param().abi()).ok() }
    }
    pub unsafe fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentFrame)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFrameSrc<P0>(&self, pszframesrc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFrameSrc)(windows_core::Interface::as_raw(self), pszframesrc.param().abi()).ok() }
    }
    pub unsafe fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameSrc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFramesContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrameOptions)(windows_core::Interface::as_raw(self), dwflags).ok() }
    }
    pub unsafe fn GetFrameOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrameOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFrameMargins)(windows_core::Interface::as_raw(self), dwwidth, dwheight).ok() }
    }
    pub unsafe fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFrameMargins)(windows_core::Interface::as_raw(self), pdwwidth as _, pdwheight as _).ok() }
    }
    pub unsafe fn FindFrame<P0>(&self, psztargetname: P0, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFrame)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTargetAlias<P0>(&self, psztargetname: P0) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTargetAlias)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame2_Impl: windows_core::IUnknownImpl {
    fn SetFrameName(&self, pszframename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParentFrame(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetFrameSrc(&self, pszframesrc: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFrameSrc(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFramesContainer(&self) -> windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(&self, dwflags: u32) -> windows_core::Result<()>;
    fn GetFrameOptions(&self) -> windows_core::Result<u32>;
    fn SetFrameMargins(&self, dwwidth: u32, dwheight: u32) -> windows_core::Result<()>;
    fn GetFrameMargins(&self, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::Result<()>;
    fn FindFrame(&self, psztargetname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn GetTargetAlias(&self, psztargetname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ITargetFrame2_Vtbl {
    pub const fn new<Identity: ITargetFrame2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFrameName<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame2_Impl::SetFrameName(this, core::mem::transmute(&pszframename)).into()
            }
        }
        unsafe extern "system" fn GetFrameName<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetFrameName(this) {
                    Ok(ok__) => {
                        ppszframename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentFrame<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetParentFrame(this) {
                    Ok(ok__) => {
                        ppunkparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszframesrc: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame2_Impl::SetFrameSrc(this, core::mem::transmute(&pszframesrc)).into()
            }
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszframesrc: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetFrameSrc(this) {
                    Ok(ok__) => {
                        ppszframesrc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontainer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetFramesContainer(this) {
                    Ok(ok__) => {
                        ppcontainer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame2_Impl::SetFrameOptions(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetFrameOptions(this) {
                    Ok(ok__) => {
                        pdwflags.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwidth: u32, dwheight: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame2_Impl::SetFrameMargins(this, core::mem::transmute_copy(&dwwidth), core::mem::transmute_copy(&dwheight)).into()
            }
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFrame2_Impl::GetFrameMargins(this, core::mem::transmute_copy(&pdwwidth), core::mem::transmute_copy(&pdwheight)).into()
            }
        }
        unsafe extern "system" fn FindFrame<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::FindFrame(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppunktargetframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTargetAlias<Identity: ITargetFrame2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, ppsztargetalias: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFrame2_Impl::GetTargetAlias(this, core::mem::transmute(&psztargetname)) {
                    Ok(ok__) => {
                        ppsztargetalias.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFrameName: SetFrameName::<Identity, OFFSET>,
            GetFrameName: GetFrameName::<Identity, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, OFFSET>,
            FindFrame: FindFrame::<Identity, OFFSET>,
            GetTargetAlias: GetTargetAlias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFrame2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for ITargetFrame2 {}
windows_core::imp::define_interface!(ITargetFramePriv, ITargetFramePriv_Vtbl, 0x9216e421_2bf5_11d0_82b4_00a0c90c29c5);
windows_core::imp::interface_hierarchy!(ITargetFramePriv, windows_core::IUnknown);
impl ITargetFramePriv {
    pub unsafe fn FindFrameDownwards<P0>(&self, psztargetname: P0, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFrameDownwards)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFrameInContext<P0, P1>(&self, psztargetname: P0, punkcontextframe: P1, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFrameInContext)(windows_core::Interface::as_raw(self), psztargetname.param().abi(), punkcontextframe.param().abi(), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnChildFrameActivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChildFrameActivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok() }
    }
    pub unsafe fn OnChildFrameDeactivate<P0>(&self, punkchildframe: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnChildFrameDeactivate)(windows_core::Interface::as_raw(self), punkchildframe.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NavigateHack<P1, P2, P3, P4, P5>(&self, grfhlnf: u32, pbc: P1, pibsc: P2, psztargetname: P3, pszurl: P4, pszlocation: P5) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
        P2: windows_core::Param<super::super::System::Com::IBindStatusCallback>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).NavigateHack)(windows_core::Interface::as_raw(self), grfhlnf, pbc.param().abi(), pibsc.param().abi(), psztargetname.param().abi(), pszurl.param().abi(), pszlocation.param().abi()).ok() }
    }
    pub unsafe fn FindBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindBrowserByIndex)(windows_core::Interface::as_raw(self), dwid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
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
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv_Impl: windows_core::IUnknownImpl {
    fn FindFrameDownwards(&self, psztargetname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn FindFrameInContext(&self, psztargetname: &windows_core::PCWSTR, punkcontextframe: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<windows_core::IUnknown>;
    fn OnChildFrameActivate(&self, punkchildframe: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn OnChildFrameDeactivate(&self, punkchildframe: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn NavigateHack(&self, grfhlnf: u32, pbc: windows_core::Ref<super::super::System::Com::IBindCtx>, pibsc: windows_core::Ref<super::super::System::Com::IBindStatusCallback>, psztargetname: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR, pszlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FindBrowserByIndex(&self, dwid: u32) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv_Vtbl {
    pub const fn new<Identity: ITargetFramePriv_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFrameDownwards<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFramePriv_Impl::FindFrameDownwards(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppunktargetframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFrameInContext<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, punkcontextframe: *mut core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFramePriv_Impl::FindFrameInContext(this, core::mem::transmute(&psztargetname), core::mem::transmute_copy(&punkcontextframe), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppunktargetframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFramePriv_Impl::OnChildFrameActivate(this, core::mem::transmute_copy(&punkchildframe)).into()
            }
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkchildframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFramePriv_Impl::OnChildFrameDeactivate(this, core::mem::transmute_copy(&punkchildframe)).into()
            }
        }
        unsafe extern "system" fn NavigateHack<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: u32, pbc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, pszurl: windows_core::PCWSTR, pszlocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFramePriv_Impl::NavigateHack(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pibsc), core::mem::transmute(&psztargetname), core::mem::transmute(&pszurl), core::mem::transmute(&pszlocation)).into()
            }
        }
        unsafe extern "system" fn FindBrowserByIndex<Identity: ITargetFramePriv_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, ppunkbrowser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITargetFramePriv_Impl::FindBrowserByIndex(this, core::mem::transmute_copy(&dwid)) {
                    Ok(ok__) => {
                        ppunkbrowser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFrameDownwards: FindFrameDownwards::<Identity, OFFSET>,
            FindFrameInContext: FindFrameInContext::<Identity, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, OFFSET>,
            NavigateHack: NavigateHack::<Identity, OFFSET>,
            FindBrowserByIndex: FindBrowserByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITargetFramePriv {}
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
    pub unsafe fn AggregatedNavigation2<P1, P2, P3, P4, P5>(&self, grfhlnf: u32, pbc: P1, pibsc: P2, psztargetname: P3, puri: P4, pszlocation: P5) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::System::Com::IBindCtx>,
        P2: windows_core::Param<super::super::System::Com::IBindStatusCallback>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::super::System::Com::IUri>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AggregatedNavigation2)(windows_core::Interface::as_raw(self), grfhlnf, pbc.param().abi(), pibsc.param().abi(), psztargetname.param().abi(), puri.param().abi(), pszlocation.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetFramePriv2_Vtbl {
    pub base__: ITargetFramePriv_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AggregatedNavigation2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AggregatedNavigation2: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv2_Impl: ITargetFramePriv_Impl {
    fn AggregatedNavigation2(&self, grfhlnf: u32, pbc: windows_core::Ref<super::super::System::Com::IBindCtx>, pibsc: windows_core::Ref<super::super::System::Com::IBindStatusCallback>, psztargetname: &windows_core::PCWSTR, puri: windows_core::Ref<super::super::System::Com::IUri>, pszlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITargetFramePriv2_Vtbl {
    pub const fn new<Identity: ITargetFramePriv2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AggregatedNavigation2<Identity: ITargetFramePriv2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: u32, pbc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, psztargetname: windows_core::PCWSTR, puri: *mut core::ffi::c_void, pszlocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetFramePriv2_Impl::AggregatedNavigation2(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pibsc), core::mem::transmute(&psztargetname), core::mem::transmute_copy(&puri), core::mem::transmute(&pszlocation)).into()
            }
        }
        Self { base__: ITargetFramePriv_Vtbl::new::<Identity, OFFSET>(), AggregatedNavigation2: AggregatedNavigation2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetFramePriv2 as windows_core::Interface>::IID || iid == &<ITargetFramePriv as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ITargetFramePriv2 {}
windows_core::imp::define_interface!(ITargetNotify, ITargetNotify_Vtbl, 0x863a99a0_21bc_11d0_82b4_00a0c90c29c5);
windows_core::imp::interface_hierarchy!(ITargetNotify, windows_core::IUnknown);
impl ITargetNotify {
    pub unsafe fn OnCreate<P0>(&self, punkdestination: P0, cbcookie: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnCreate)(windows_core::Interface::as_raw(self), punkdestination.param().abi(), cbcookie).ok() }
    }
    pub unsafe fn OnReuse<P0>(&self, punkdestination: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnReuse)(windows_core::Interface::as_raw(self), punkdestination.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnReuse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITargetNotify_Impl: windows_core::IUnknownImpl {
    fn OnCreate(&self, punkdestination: windows_core::Ref<windows_core::IUnknown>, cbcookie: u32) -> windows_core::Result<()>;
    fn OnReuse(&self, punkdestination: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ITargetNotify_Vtbl {
    pub const fn new<Identity: ITargetNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnCreate<Identity: ITargetNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdestination: *mut core::ffi::c_void, cbcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetNotify_Impl::OnCreate(this, core::mem::transmute_copy(&punkdestination), core::mem::transmute_copy(&cbcookie)).into()
            }
        }
        unsafe extern "system" fn OnReuse<Identity: ITargetNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetNotify_Impl::OnReuse(this, core::mem::transmute_copy(&punkdestination)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCreate: OnCreate::<Identity, OFFSET>, OnReuse: OnReuse::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITargetNotify {}
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
        unsafe { (windows_core::Interface::vtable(self).GetOptionString)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstroptions)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetNotify2_Vtbl {
    pub base__: ITargetNotify_Vtbl,
    pub GetOptionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITargetNotify2_Impl: ITargetNotify_Impl {
    fn GetOptionString(&self, pbstroptions: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl ITargetNotify2_Vtbl {
    pub const fn new<Identity: ITargetNotify2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOptionString<Identity: ITargetNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstroptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITargetNotify2_Impl::GetOptionString(this, core::mem::transmute_copy(&pbstroptions)).into()
            }
        }
        Self { base__: ITargetNotify_Vtbl::new::<Identity, OFFSET>(), GetOptionString: GetOptionString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITargetNotify2 as windows_core::Interface>::IID || iid == &<ITargetNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITargetNotify2 {}
windows_core::imp::define_interface!(ITimer, ITimer_Vtbl, 0x3050f360_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ITimer, windows_core::IUnknown);
impl ITimer {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Advise<P4>(&self, vtimemin: &super::super::System::Variant::VARIANT, vtimemax: &super::super::System::Variant::VARIANT, vtimeinterval: &super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: P4) -> windows_core::Result<u32>
    where
        P4: windows_core::Param<ITimerSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtimemin), core::mem::transmute_copy(vtimemax), core::mem::transmute_copy(vtimeinterval), dwflags, ptimersink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok() }
    }
    pub unsafe fn Freeze(&self, ffreeze: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), ffreeze.into()).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, super::super::System::Variant::VARIANT, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetTime: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimer_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, vtimemin: &super::super::System::Variant::VARIANT, vtimemax: &super::super::System::Variant::VARIANT, vtimeinterval: &super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: windows_core::Ref<ITimerSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn Freeze(&self, ffreeze: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimer_Vtbl {
    pub const fn new<Identity: ITimer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: ITimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtimemin: super::super::System::Variant::VARIANT, vtimemax: super::super::System::Variant::VARIANT, vtimeinterval: super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimer_Impl::Advise(this, core::mem::transmute(&vtimemin), core::mem::transmute(&vtimemax), core::mem::transmute(&vtimeinterval), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&ptimersink)) {
                    Ok(ok__) => {
                        pdwcookie.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ITimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimer_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn Freeze<Identity: ITimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffreeze: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimer_Impl::Freeze(this, core::mem::transmute_copy(&ffreeze)).into()
            }
        }
        unsafe extern "system" fn GetTime<Identity: ITimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtime: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimer_Impl::GetTime(this) {
                    Ok(ok__) => {
                        pvtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            Freeze: Freeze::<Identity, OFFSET>,
            GetTime: GetTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITimer {}
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
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), dwmode).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerEx_Vtbl {
    pub base__: ITimer_Vtbl,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerEx_Impl: ITimer_Impl {
    fn SetMode(&self, dwmode: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimerEx_Vtbl {
    pub const fn new<Identity: ITimerEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMode<Identity: ITimerEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimerEx_Impl::SetMode(this, core::mem::transmute_copy(&dwmode)).into()
            }
        }
        Self { base__: ITimer_Vtbl::new::<Identity, OFFSET>(), SetMode: SetMode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerEx as windows_core::Interface>::IID || iid == &<ITimer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITimerEx {}
windows_core::imp::define_interface!(ITimerService, ITimerService_Vtbl, 0x3050f35f_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ITimerService, windows_core::IUnknown);
impl ITimerService {
    pub unsafe fn CreateTimer<P0>(&self, preferencetimer: P0) -> windows_core::Result<ITimer>
    where
        P0: windows_core::Param<ITimer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimer)(windows_core::Interface::as_raw(self), preferencetimer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNamedTimer(&self, rguidname: *const windows_core::GUID) -> windows_core::Result<ITimer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamedTimer)(windows_core::Interface::as_raw(self), rguidname, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetNamedTimerReference<P1>(&self, rguidname: *const windows_core::GUID, preferencetimer: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ITimer>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNamedTimerReference)(windows_core::Interface::as_raw(self), rguidname, preferencetimer.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamedTimerReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITimerService_Impl: windows_core::IUnknownImpl {
    fn CreateTimer(&self, preferencetimer: windows_core::Ref<ITimer>) -> windows_core::Result<ITimer>;
    fn GetNamedTimer(&self, rguidname: *const windows_core::GUID) -> windows_core::Result<ITimer>;
    fn SetNamedTimerReference(&self, rguidname: *const windows_core::GUID, preferencetimer: windows_core::Ref<ITimer>) -> windows_core::Result<()>;
}
impl ITimerService_Vtbl {
    pub const fn new<Identity: ITimerService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateTimer<Identity: ITimerService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preferencetimer: *mut core::ffi::c_void, ppnewtimer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimerService_Impl::CreateTimer(this, core::mem::transmute_copy(&preferencetimer)) {
                    Ok(ok__) => {
                        ppnewtimer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNamedTimer<Identity: ITimerService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidname: *const windows_core::GUID, pptimer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimerService_Impl::GetNamedTimer(this, core::mem::transmute_copy(&rguidname)) {
                    Ok(ok__) => {
                        pptimer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamedTimerReference<Identity: ITimerService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidname: *const windows_core::GUID, preferencetimer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimerService_Impl::SetNamedTimerReference(this, core::mem::transmute_copy(&rguidname), core::mem::transmute_copy(&preferencetimer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTimer: CreateTimer::<Identity, OFFSET>,
            GetNamedTimer: GetNamedTimer::<Identity, OFFSET>,
            SetNamedTimerReference: SetNamedTimerReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITimerService {}
windows_core::imp::define_interface!(ITimerSink, ITimerSink_Vtbl, 0x3050f361_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ITimerSink, windows_core::IUnknown);
impl ITimerSink {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn OnTimer(&self, vtimeadvise: &super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnTimer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vtimeadvise)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimerSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub OnTimer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    OnTimer: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerSink_Impl: windows_core::IUnknownImpl {
    fn OnTimer(&self, vtimeadvise: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITimerSink_Vtbl {
    pub const fn new<Identity: ITimerSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTimer<Identity: ITimerSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vtimeadvise: super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimerSink_Impl::OnTimer(this, core::mem::transmute(&vtimeadvise)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnTimer: OnTimer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimerSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ITimerSink {}
windows_core::imp::define_interface!(ITridentTouchInput, ITridentTouchInput_Vtbl, 0x30510850_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ITridentTouchInput, windows_core::IUnknown);
impl ITridentTouchInput {
    pub unsafe fn OnPointerMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnPointerMessage)(windows_core::Interface::as_raw(self), msg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentTouchInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnPointerMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITridentTouchInput_Impl: windows_core::IUnknownImpl {
    fn OnPointerMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<windows_core::BOOL>;
}
impl ITridentTouchInput_Vtbl {
    pub const fn new<Identity: ITridentTouchInput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPointerMessage<Identity: ITridentTouchInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfallowmanipulations: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITridentTouchInput_Impl::OnPointerMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        pfallowmanipulations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPointerMessage: OnPointerMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITridentTouchInput as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITridentTouchInput {}
windows_core::imp::define_interface!(ITridentTouchInputSite, ITridentTouchInputSite_Vtbl, 0x30510849_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(ITridentTouchInputSite, windows_core::IUnknown);
impl ITridentTouchInputSite {
    pub unsafe fn ZoomToPoint(&self, x: i32, y: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ZoomToPoint)(windows_core::Interface::as_raw(self), x, y).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITridentTouchInputSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetManipulationMode: usize,
    pub ZoomToPoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait ITridentTouchInputSite_Impl: windows_core::IUnknownImpl {
    fn ZoomToPoint(&self, x: i32, y: i32) -> windows_core::Result<()>;
}
impl ITridentTouchInputSite_Vtbl {
    pub const fn new<Identity: ITridentTouchInputSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ZoomToPoint<Identity: ITridentTouchInputSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITridentTouchInputSite_Impl::ZoomToPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetManipulationMode: 0, ZoomToPoint: ZoomToPoint::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITridentTouchInputSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITridentTouchInputSite {}
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
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryNotify_Vtbl {
    pub base__: super::super::System::Ole::IOleCommandTarget_Vtbl,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUrlHistoryNotify_Impl: super::super::System::Ole::IOleCommandTarget_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IUrlHistoryNotify_Vtbl {
    pub const fn new<Identity: IUrlHistoryNotify_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::super::System::Ole::IOleCommandTarget_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryNotify as windows_core::Interface>::IID || iid == &<super::super::System::Ole::IOleCommandTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IUrlHistoryNotify {}
windows_core::imp::define_interface!(IUrlHistoryStg, IUrlHistoryStg_Vtbl, 0x3c374a41_bae4_11cf_bf7d_00aa006946ee);
windows_core::imp::interface_hierarchy!(IUrlHistoryStg, windows_core::IUnknown);
impl IUrlHistoryStg {
    pub unsafe fn AddUrl<P0, P1>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), pocstitle.param().abi(), dwflags).ok() }
    }
    pub unsafe fn DeleteUrl<P0>(&self, pocsurl: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), dwflags).ok() }
    }
    pub unsafe fn QueryUrl<P0>(&self, pocsurl: P0, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryUrl)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), dwflags, lpstaturl as _).ok() }
    }
    pub unsafe fn BindToObject<P0, T>(&self, pocsurl: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EnumUrls(&self) -> windows_core::Result<IEnumSTATURL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryStg_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub DeleteUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub QueryUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut STATURL) -> windows_core::HRESULT,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUrlHistoryStg_Impl: windows_core::IUnknownImpl {
    fn AddUrl(&self, pocsurl: &windows_core::PCWSTR, pocstitle: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteUrl(&self, pocsurl: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn QueryUrl(&self, pocsurl: &windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::Result<()>;
    fn BindToObject(&self, pocsurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumUrls(&self) -> windows_core::Result<IEnumSTATURL>;
}
impl IUrlHistoryStg_Vtbl {
    pub const fn new<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddUrl<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, pocstitle: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg_Impl::AddUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute(&pocstitle), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DeleteUrl<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg_Impl::DeleteUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn QueryUrl<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg_Impl::QueryUrl(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&lpstaturl)).into()
            }
        }
        unsafe extern "system" fn BindToObject<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg_Impl::BindToObject(this, core::mem::transmute(&pocsurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvout)).into()
            }
        }
        unsafe extern "system" fn EnumUrls<Identity: IUrlHistoryStg_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUrlHistoryStg_Impl::EnumUrls(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddUrl: AddUrl::<Identity, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, OFFSET>,
            QueryUrl: QueryUrl::<Identity, OFFSET>,
            BindToObject: BindToObject::<Identity, OFFSET>,
            EnumUrls: EnumUrls::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUrlHistoryStg {}
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
    pub unsafe fn AddUrlAndNotify<P0, P1, P4, P5>(&self, pocsurl: P0, pocstitle: P1, dwflags: u32, fwritehistory: bool, poctnotify: P4, punkisfolder: P5) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::super::System::Ole::IOleCommandTarget>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddUrlAndNotify)(windows_core::Interface::as_raw(self), pocsurl.param().abi(), pocstitle.param().abi(), dwflags, fwritehistory.into(), poctnotify.param().abi(), punkisfolder.param().abi()).ok() }
    }
    pub unsafe fn ClearHistory(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearHistory)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlHistoryStg2_Vtbl {
    pub base__: IUrlHistoryStg_Vtbl,
    #[cfg(feature = "Win32_System_Ole")]
    pub AddUrlAndNotify: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::BOOL, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    AddUrlAndNotify: usize,
    pub ClearHistory: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IUrlHistoryStg2_Impl: IUrlHistoryStg_Impl {
    fn AddUrlAndNotify(&self, pocsurl: &windows_core::PCWSTR, pocstitle: &windows_core::PCWSTR, dwflags: u32, fwritehistory: windows_core::BOOL, poctnotify: windows_core::Ref<super::super::System::Ole::IOleCommandTarget>, punkisfolder: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearHistory(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IUrlHistoryStg2_Vtbl {
    pub const fn new<Identity: IUrlHistoryStg2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddUrlAndNotify<Identity: IUrlHistoryStg2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pocsurl: windows_core::PCWSTR, pocstitle: windows_core::PCWSTR, dwflags: u32, fwritehistory: windows_core::BOOL, poctnotify: *mut core::ffi::c_void, punkisfolder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg2_Impl::AddUrlAndNotify(this, core::mem::transmute(&pocsurl), core::mem::transmute(&pocstitle), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&fwritehistory), core::mem::transmute_copy(&poctnotify), core::mem::transmute_copy(&punkisfolder)).into()
            }
        }
        unsafe extern "system" fn ClearHistory<Identity: IUrlHistoryStg2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlHistoryStg2_Impl::ClearHistory(this).into()
            }
        }
        Self {
            base__: IUrlHistoryStg_Vtbl::new::<Identity, OFFSET>(),
            AddUrlAndNotify: AddUrlAndNotify::<Identity, OFFSET>,
            ClearHistory: ClearHistory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlHistoryStg2 as windows_core::Interface>::IID || iid == &<IUrlHistoryStg as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Ole")]
impl windows_core::RuntimeName for IUrlHistoryStg2 {}
windows_core::imp::define_interface!(IViewObjectPresentFlip, IViewObjectPresentFlip_Vtbl, 0x30510847_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlip, windows_core::IUnknown);
impl IViewObjectPresentFlip {
    pub unsafe fn NotifyRender(&self, frecreatepresenter: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyRender)(windows_core::Interface::as_raw(self), frecreatepresenter.into()).ok() }
    }
    pub unsafe fn RenderObjectToBitmap<P0>(&self, pbitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenderObjectToBitmap)(windows_core::Interface::as_raw(self), pbitmap.param().abi()).ok() }
    }
    pub unsafe fn RenderObjectToSharedBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISurfacePresenterFlipBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenderObjectToSharedBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyRender: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RenderObjectToBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RenderObjectToSharedBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IViewObjectPresentFlip_Impl: windows_core::IUnknownImpl {
    fn NotifyRender(&self, frecreatepresenter: windows_core::BOOL) -> windows_core::Result<()>;
    fn RenderObjectToBitmap(&self, pbitmap: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RenderObjectToSharedBuffer(&self, pbuffer: windows_core::Ref<ISurfacePresenterFlipBuffer>) -> windows_core::Result<()>;
}
impl IViewObjectPresentFlip_Vtbl {
    pub const fn new<Identity: IViewObjectPresentFlip_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyRender<Identity: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frecreatepresenter: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlip_Impl::NotifyRender(this, core::mem::transmute_copy(&frecreatepresenter)).into()
            }
        }
        unsafe extern "system" fn RenderObjectToBitmap<Identity: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlip_Impl::RenderObjectToBitmap(this, core::mem::transmute_copy(&pbitmap)).into()
            }
        }
        unsafe extern "system" fn RenderObjectToSharedBuffer<Identity: IViewObjectPresentFlip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlip_Impl::RenderObjectToSharedBuffer(this, core::mem::transmute_copy(&pbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyRender: NotifyRender::<Identity, OFFSET>,
            RenderObjectToBitmap: RenderObjectToBitmap::<Identity, OFFSET>,
            RenderObjectToSharedBuffer: RenderObjectToSharedBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewObjectPresentFlip {}
windows_core::imp::define_interface!(IViewObjectPresentFlip2, IViewObjectPresentFlip2_Vtbl, 0x30510856_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlip2, windows_core::IUnknown);
impl IViewObjectPresentFlip2 {
    pub unsafe fn NotifyLeavingView(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyLeavingView)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlip2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyLeavingView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IViewObjectPresentFlip2_Impl: windows_core::IUnknownImpl {
    fn NotifyLeavingView(&self) -> windows_core::Result<()>;
}
impl IViewObjectPresentFlip2_Vtbl {
    pub const fn new<Identity: IViewObjectPresentFlip2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyLeavingView<Identity: IViewObjectPresentFlip2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlip2_Impl::NotifyLeavingView(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyLeavingView: NotifyLeavingView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlip2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewObjectPresentFlip2 {}
windows_core::imp::define_interface!(IViewObjectPresentFlipSite, IViewObjectPresentFlipSite_Vtbl, 0x30510846_98b5_11cf_bb82_00aa00bdce0b);
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlipSite, windows_core::IUnknown);
impl IViewObjectPresentFlipSite {
    pub unsafe fn GetDeviceLuid(&self) -> windows_core::Result<super::super::Foundation::LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceLuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnterFullScreen(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnterFullScreen)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn ExitFullScreen(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ExitFullScreen)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsFullScreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBoundingRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundingRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMetrics(&self, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), ppos as _, psize as _, pscalex as _, pscaley as _).ok() }
    }
    pub unsafe fn GetFullScreenSize(&self) -> windows_core::Result<super::super::Foundation::SIZE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullScreenSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlipSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateSurfacePresenterFlip: usize,
    pub GetDeviceLuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::LUID) -> windows_core::HRESULT,
    pub EnterFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExitFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBoundingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINT, *mut super::super::Foundation::SIZE, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetFullScreenSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
}
pub trait IViewObjectPresentFlipSite_Impl: windows_core::IUnknownImpl {
    fn GetDeviceLuid(&self) -> windows_core::Result<super::super::Foundation::LUID>;
    fn EnterFullScreen(&self) -> windows_core::Result<()>;
    fn ExitFullScreen(&self) -> windows_core::Result<()>;
    fn IsFullScreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetBoundingRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetMetrics(&self, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> windows_core::Result<()>;
    fn GetFullScreenSize(&self) -> windows_core::Result<super::super::Foundation::SIZE>;
}
impl IViewObjectPresentFlipSite_Vtbl {
    pub const fn new<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceLuid<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pluid: *mut super::super::Foundation::LUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectPresentFlipSite_Impl::GetDeviceLuid(this) {
                    Ok(ok__) => {
                        pluid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnterFullScreen<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlipSite_Impl::EnterFullScreen(this).into()
            }
        }
        unsafe extern "system" fn ExitFullScreen<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlipSite_Impl::ExitFullScreen(this).into()
            }
        }
        unsafe extern "system" fn IsFullScreen<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffullscreen: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectPresentFlipSite_Impl::IsFullScreen(this) {
                    Ok(ok__) => {
                        pffullscreen.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoundingRect<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectPresentFlipSite_Impl::GetBoundingRect(this) {
                    Ok(ok__) => {
                        prect.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMetrics<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IViewObjectPresentFlipSite_Impl::GetMetrics(this, core::mem::transmute_copy(&ppos), core::mem::transmute_copy(&psize), core::mem::transmute_copy(&pscalex), core::mem::transmute_copy(&pscaley)).into()
            }
        }
        unsafe extern "system" fn GetFullScreenSize<Identity: IViewObjectPresentFlipSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectPresentFlipSite_Impl::GetFullScreenSize(this) {
                    Ok(ok__) => {
                        psize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSurfacePresenterFlip: 0,
            GetDeviceLuid: GetDeviceLuid::<Identity, OFFSET>,
            EnterFullScreen: EnterFullScreen::<Identity, OFFSET>,
            ExitFullScreen: ExitFullScreen::<Identity, OFFSET>,
            IsFullScreen: IsFullScreen::<Identity, OFFSET>,
            GetBoundingRect: GetBoundingRect::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetFullScreenSize: GetFullScreenSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlipSite as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IViewObjectPresentFlipSite {}
windows_core::imp::define_interface!(IViewObjectPresentFlipSite2, IViewObjectPresentFlipSite2_Vtbl, 0xaad0cbf1_e7fd_4f12_8902_c78132a8e01d);
windows_core::imp::interface_hierarchy!(IViewObjectPresentFlipSite2, windows_core::IUnknown);
impl IViewObjectPresentFlipSite2 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotationForCurrentOutput(&self) -> windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotationForCurrentOutput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewObjectPresentFlipSite2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetRotationForCurrentOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetRotationForCurrentOutput: usize,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IViewObjectPresentFlipSite2_Impl: windows_core::IUnknownImpl {
    fn GetRotationForCurrentOutput(&self) -> windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IViewObjectPresentFlipSite2_Vtbl {
    pub const fn new<Identity: IViewObjectPresentFlipSite2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRotationForCurrentOutput<Identity: IViewObjectPresentFlipSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdxgirotation: *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IViewObjectPresentFlipSite2_Impl::GetRotationForCurrentOutput(this) {
                    Ok(ok__) => {
                        pdxgirotation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRotationForCurrentOutput: GetRotationForCurrentOutput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewObjectPresentFlipSite2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IViewObjectPresentFlipSite2 {}
windows_core::imp::define_interface!(IWebBrowserEventsService, IWebBrowserEventsService_Vtbl, 0x54a8f188_9ebd_4795_ad16_9b4945119636);
windows_core::imp::interface_hierarchy!(IWebBrowserEventsService, windows_core::IUnknown);
impl IWebBrowserEventsService {
    pub unsafe fn FireBeforeNavigate2Event(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FireBeforeNavigate2Event)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FireNavigateComplete2Event(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireNavigateComplete2Event)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FireDownloadBeginEvent(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireDownloadBeginEvent)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FireDownloadCompleteEvent(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireDownloadCompleteEvent)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn FireDocumentCompleteEvent(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FireDocumentCompleteEvent)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserEventsService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FireBeforeNavigate2Event: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FireNavigateComplete2Event: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadBeginEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDownloadCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FireDocumentCompleteEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWebBrowserEventsService_Impl: windows_core::IUnknownImpl {
    fn FireBeforeNavigate2Event(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn FireNavigateComplete2Event(&self) -> windows_core::Result<()>;
    fn FireDownloadBeginEvent(&self) -> windows_core::Result<()>;
    fn FireDownloadCompleteEvent(&self) -> windows_core::Result<()>;
    fn FireDocumentCompleteEvent(&self) -> windows_core::Result<()>;
}
impl IWebBrowserEventsService_Vtbl {
    pub const fn new<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FireBeforeNavigate2Event<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcancel: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserEventsService_Impl::FireBeforeNavigate2Event(this) {
                    Ok(ok__) => {
                        pfcancel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FireNavigateComplete2Event<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserEventsService_Impl::FireNavigateComplete2Event(this).into()
            }
        }
        unsafe extern "system" fn FireDownloadBeginEvent<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserEventsService_Impl::FireDownloadBeginEvent(this).into()
            }
        }
        unsafe extern "system" fn FireDownloadCompleteEvent<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserEventsService_Impl::FireDownloadCompleteEvent(this).into()
            }
        }
        unsafe extern "system" fn FireDocumentCompleteEvent<Identity: IWebBrowserEventsService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebBrowserEventsService_Impl::FireDocumentCompleteEvent(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FireBeforeNavigate2Event: FireBeforeNavigate2Event::<Identity, OFFSET>,
            FireNavigateComplete2Event: FireNavigateComplete2Event::<Identity, OFFSET>,
            FireDownloadBeginEvent: FireDownloadBeginEvent::<Identity, OFFSET>,
            FireDownloadCompleteEvent: FireDownloadCompleteEvent::<Identity, OFFSET>,
            FireDocumentCompleteEvent: FireDocumentCompleteEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWebBrowserEventsService {}
windows_core::imp::define_interface!(IWebBrowserEventsUrlService, IWebBrowserEventsUrlService_Vtbl, 0x87cc5d04_eafa_4833_9820_8f986530cc00);
windows_core::imp::interface_hierarchy!(IWebBrowserEventsUrlService, windows_core::IUnknown);
impl IWebBrowserEventsUrlService {
    pub unsafe fn GetUrlForEvents(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUrlForEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserEventsUrlService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUrlForEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWebBrowserEventsUrlService_Impl: windows_core::IUnknownImpl {
    fn GetUrlForEvents(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IWebBrowserEventsUrlService_Vtbl {
    pub const fn new<Identity: IWebBrowserEventsUrlService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUrlForEvents<Identity: IWebBrowserEventsUrlService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebBrowserEventsUrlService_Impl::GetUrlForEvents(this) {
                    Ok(ok__) => {
                        purl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUrlForEvents: GetUrlForEvents::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebBrowserEventsUrlService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWebBrowserEventsUrlService {}
pub const IntelliForms: windows_core::GUID = windows_core::GUID::from_u128(0x613ab92e_16bf_11d2_bca5_00c04fd929db);
pub const InternetExplorerManager: windows_core::GUID = windows_core::GUID::from_u128(0xdf4fcc34_067a_4e0a_8352_4a1a5095346e);
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
    pub unsafe fn navigate(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).navigate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn navigateFrame(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).navigateFrame)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), core::mem::transmute_copy(bstrtargetframe), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn navigateNoSite<P3>(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR, dwhwnd: u32, pwb: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).navigateNoSite)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), core::mem::transmute_copy(bstrtargetframe), dwhwnd, pwb.param().abi()).ok() }
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Iwfolders_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub navigate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub navigateFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub navigateNoSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Iwfolders_Impl: super::super::System::Com::IDispatch_Impl {
    fn navigate(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn navigateFrame(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn navigateNoSite(&self, bstrurl: &windows_core::BSTR, bstrtargetframe: &windows_core::BSTR, dwhwnd: u32, pwb: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Iwfolders_Vtbl {
    pub const fn new<Identity: Iwfolders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn navigate<Identity: Iwfolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, pbstrretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Iwfolders_Impl::navigate(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        pbstrretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn navigateFrame<Identity: Iwfolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, bstrtargetframe: *mut core::ffi::c_void, pbstrretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Iwfolders_Impl::navigateFrame(this, core::mem::transmute(&bstrurl), core::mem::transmute(&bstrtargetframe)) {
                    Ok(ok__) => {
                        pbstrretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn navigateNoSite<Identity: Iwfolders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, bstrtargetframe: *mut core::ffi::c_void, dwhwnd: u32, pwb: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Iwfolders_Impl::navigateNoSite(this, core::mem::transmute(&bstrurl), core::mem::transmute(&bstrtargetframe), core::mem::transmute_copy(&dwhwnd), core::mem::transmute_copy(&pwb)).into()
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            navigate: navigate::<Identity, OFFSET>,
            navigateFrame: navigateFrame::<Identity, OFFSET>,
            navigateNoSite: navigateNoSite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Iwfolders as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for Iwfolders {}
pub const LINKSBAND: u32 = 4u32;
pub const MAPMIME_CLSID: u32 = 1u32;
pub const MAPMIME_DEFAULT: u32 = 0u32;
pub const MAPMIME_DEFAULT_ALWAYS: u32 = 3u32;
pub const MAPMIME_DISABLE: u32 = 2u32;
pub const MAX_SEARCH_FORMAT_STRING: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEDIA_ACTIVITY_NOTIFY_TYPE(pub i32);
pub const MediaCasting: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(2i32);
pub const MediaPlayback: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(0i32);
pub const MediaRecording: MEDIA_ACTIVITY_NOTIFY_TYPE = MEDIA_ACTIVITY_NOTIFY_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NAVIGATEDATA {
    pub ulTarget: u32,
    pub ulURL: u32,
    pub ulRefURL: u32,
    pub ulPostData: u32,
    pub dwFlags: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAVIGATEFRAME_FLAGS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpenServiceActivityContentType(pub i32);
pub const OpenServiceActivityManager: windows_core::GUID = windows_core::GUID::from_u128(0xc5efd803_50f8_43cd_9ab8_aafc1394c9e0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpenServiceErrors(pub i32);
pub const OpenServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0x098870b6_39ea_480b_b8b5_dd0167c4db59);
pub const PeerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x3050f4cf_98b5_11cf_bb82_00aa00bdce0b);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCROLLABLECONTEXTMENU_PLACEMENT(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STATURL {
    pub cbSize: u32,
    pub pwcsUrl: windows_core::PWSTR,
    pub pwcsTitle: windows_core::PWSTR,
    pub ftLastVisited: super::super::Foundation::FILETIME,
    pub ftLastUpdated: super::super::Foundation::FILETIME,
    pub ftExpires: super::super::Foundation::FILETIME,
    pub dwFlags: u32,
}
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
pub const wfolders: windows_core::GUID = windows_core::GUID::from_u128(0xbae31f9a_1b81_11d2_a97a_00c04f8ecb02);
