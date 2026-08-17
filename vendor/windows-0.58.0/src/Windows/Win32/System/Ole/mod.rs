#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn BstrFromVector(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn BstrFromVector(psa : *const super::Com:: SAFEARRAY, pbstr : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    BstrFromVector(psa, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ClearCustData(pcustdata: *mut super::Com::CUSTDATA) {
    windows_targets::link!("oleaut32.dll" "system" fn ClearCustData(pcustdata : *mut super::Com:: CUSTDATA));
    ClearCustData(pcustdata)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn CreateDispTypeInfo(pidata: *mut INTERFACEDATA, lcid: u32, pptinfo: *mut Option<super::Com::ITypeInfo>) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn CreateDispTypeInfo(pidata : *mut INTERFACEDATA, lcid : u32, pptinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateDispTypeInfo(pidata, lcid, core::mem::transmute(pptinfo)).ok()
}
#[inline]
pub unsafe fn CreateErrorInfo() -> windows_core::Result<ICreateErrorInfo> {
    windows_targets::link!("oleaut32.dll" "system" fn CreateErrorInfo(pperrinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateOleAdviseHolder() -> windows_core::Result<IOleAdviseHolder> {
    windows_targets::link!("ole32.dll" "system" fn CreateOleAdviseHolder(ppoaholder : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateOleAdviseHolder(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateStdDispatch<P0, P1>(punkouter: P0, pvthis: *mut core::ffi::c_void, ptinfo: P1, ppunkstddisp: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::Com::ITypeInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn CreateStdDispatch(punkouter : * mut core::ffi::c_void, pvthis : *mut core::ffi::c_void, ptinfo : * mut core::ffi::c_void, ppunkstddisp : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    CreateStdDispatch(punkouter.param().abi(), pvthis, ptinfo.param().abi(), core::mem::transmute(ppunkstddisp)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateTypeLib<P0>(syskind: super::Com::SYSKIND, szfile: P0) -> windows_core::Result<ICreateTypeLib>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn CreateTypeLib(syskind : super::Com:: SYSKIND, szfile : windows_core::PCWSTR, ppctlib : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateTypeLib(syskind, szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateTypeLib2<P0>(syskind: super::Com::SYSKIND, szfile: P0) -> windows_core::Result<ICreateTypeLib2>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn CreateTypeLib2(syskind : super::Com:: SYSKIND, szfile : windows_core::PCWSTR, ppctlib : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateTypeLib2(syskind, szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn DispCallFunc(pvinstance: Option<*const core::ffi::c_void>, ovft: usize, cc: super::Com::CALLCONV, vtreturn: super::Variant::VARENUM, cactuals: u32, prgvt: *const u16, prgpvarg: *const *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn DispCallFunc(pvinstance : *const core::ffi::c_void, ovft : usize, cc : super::Com:: CALLCONV, vtreturn : super::Variant:: VARENUM, cactuals : u32, prgvt : *const u16, prgpvarg : *const *const windows_core::VARIANT, pvargresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    DispCallFunc(core::mem::transmute(pvinstance.unwrap_or(std::ptr::null())), ovft, cc, vtreturn, cactuals, prgvt, prgpvarg, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DispGetIDsOfNames<P0>(ptinfo: P0, rgsznames: *const windows_core::PCWSTR, cnames: u32, rgdispid: *mut i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::ITypeInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn DispGetIDsOfNames(ptinfo : * mut core::ffi::c_void, rgsznames : *const windows_core::PCWSTR, cnames : u32, rgdispid : *mut i32) -> windows_core::HRESULT);
    DispGetIDsOfNames(ptinfo.param().abi(), rgsznames, cnames, rgdispid).ok()
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn DispGetParam(pdispparams: *const super::Com::DISPPARAMS, position: u32, vttarg: super::Variant::VARENUM, pvarresult: *mut windows_core::VARIANT, puargerr: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn DispGetParam(pdispparams : *const super::Com:: DISPPARAMS, position : u32, vttarg : super::Variant:: VARENUM, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >, puargerr : *mut u32) -> windows_core::HRESULT);
    DispGetParam(pdispparams, position, vttarg, core::mem::transmute(pvarresult), core::mem::transmute(puargerr.unwrap_or(std::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DispInvoke<P0>(_this: *mut core::ffi::c_void, ptinfo: P0, dispidmember: i32, wflags: u16, pparams: *mut super::Com::DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::ITypeInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn DispInvoke(_this : *mut core::ffi::c_void, ptinfo : * mut core::ffi::c_void, dispidmember : i32, wflags : u16, pparams : *mut super::Com:: DISPPARAMS, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >, pexcepinfo : *mut super::Com:: EXCEPINFO, puargerr : *mut u32) -> windows_core::HRESULT);
    DispInvoke(_this, ptinfo.param().abi(), dispidmember, wflags, pparams, core::mem::transmute(pvarresult), pexcepinfo, puargerr).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DoDragDrop<P0, P1>(pdataobj: P0, pdropsource: P1, dwokeffects: DROPEFFECT, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<IDropSource>,
{
    windows_targets::link!("ole32.dll" "system" fn DoDragDrop(pdataobj : * mut core::ffi::c_void, pdropsource : * mut core::ffi::c_void, dwokeffects : DROPEFFECT, pdweffect : *mut DROPEFFECT) -> windows_core::HRESULT);
    DoDragDrop(pdataobj.param().abi(), pdropsource.param().abi(), dwokeffects, pdweffect)
}
#[inline]
pub unsafe fn GetActiveObject(rclsid: *const windows_core::GUID, pvreserved: Option<*mut core::ffi::c_void>, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn GetActiveObject(rclsid : *const windows_core::GUID, pvreserved : *mut core::ffi::c_void, ppunk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    GetActiveObject(rclsid, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppunk)).ok()
}
#[inline]
pub unsafe fn GetAltMonthNames(lcid: u32) -> windows_core::Result<*mut windows_core::PWSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn GetAltMonthNames(lcid : u32, prgp : *mut *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetAltMonthNames(lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetRecordInfoFromGuids(rguidtypelib: *const windows_core::GUID, uvermajor: u32, uverminor: u32, lcid: u32, rguidtypeinfo: *const windows_core::GUID) -> windows_core::Result<IRecordInfo> {
    windows_targets::link!("oleaut32.dll" "system" fn GetRecordInfoFromGuids(rguidtypelib : *const windows_core::GUID, uvermajor : u32, uverminor : u32, lcid : u32, rguidtypeinfo : *const windows_core::GUID, pprecinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetRecordInfoFromGuids(rguidtypelib, uvermajor, uverminor, lcid, rguidtypeinfo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GetRecordInfoFromTypeInfo<P0>(ptypeinfo: P0) -> windows_core::Result<IRecordInfo>
where
    P0: windows_core::Param<super::Com::ITypeInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn GetRecordInfoFromTypeInfo(ptypeinfo : * mut core::ffi::c_void, pprecinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetRecordInfoFromTypeInfo(ptypeinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    windows_targets::link!("ole32.dll" "system" fn HRGN_UserFree(param0 : *const u32, param1 : *const super::super::Graphics::Gdi:: HRGN));
    HRGN_UserFree(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HRGN) {
    windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserFree64(param0 : *const u32, param1 : *const super::super::Graphics::Gdi:: HRGN));
    HRGN_UserFree64(param0, param1)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HRGN_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::Graphics::Gdi:: HRGN) -> *mut u8);
    HRGN_UserMarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::Graphics::Gdi:: HRGN) -> *mut u8);
    HRGN_UserMarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    windows_targets::link!("ole32.dll" "system" fn HRGN_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::Graphics::Gdi:: HRGN) -> u32);
    HRGN_UserSize(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HRGN) -> u32 {
    windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::Graphics::Gdi:: HRGN) -> u32);
    HRGN_UserSize64(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    windows_targets::link!("ole32.dll" "system" fn HRGN_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::Graphics::Gdi:: HRGN) -> *mut u8);
    HRGN_UserUnmarshal(param0, param1, param2)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HRGN_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HRGN) -> *mut u8 {
    windows_targets::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::Graphics::Gdi:: HRGN) -> *mut u8);
    HRGN_UserUnmarshal64(param0, param1, param2)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn IsAccelerator<P0>(haccel: P0, caccelentries: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, lpwcmd: *mut u16) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HACCEL>,
{
    windows_targets::link!("ole32.dll" "system" fn IsAccelerator(haccel : super::super::UI::WindowsAndMessaging:: HACCEL, caccelentries : i32, lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG, lpwcmd : *mut u16) -> super::super::Foundation:: BOOL);
    IsAccelerator(haccel.param().abi(), caccelentries, lpmsg, lpwcmd)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LHashValOfNameSys<P0>(syskind: super::Com::SYSKIND, lcid: u32, szname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn LHashValOfNameSys(syskind : super::Com:: SYSKIND, lcid : u32, szname : windows_core::PCWSTR) -> u32);
    LHashValOfNameSys(syskind, lcid, szname.param().abi())
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LHashValOfNameSysA<P0>(syskind: super::Com::SYSKIND, lcid: u32, szname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn LHashValOfNameSysA(syskind : super::Com:: SYSKIND, lcid : u32, szname : windows_core::PCSTR) -> u32);
    LHashValOfNameSysA(syskind, lcid, szname.param().abi())
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadRegTypeLib(rguid: *const windows_core::GUID, wvermajor: u16, wverminor: u16, lcid: u32) -> windows_core::Result<super::Com::ITypeLib> {
    windows_targets::link!("oleaut32.dll" "system" fn LoadRegTypeLib(rguid : *const windows_core::GUID, wvermajor : u16, wverminor : u16, lcid : u32, pptlib : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    LoadRegTypeLib(rguid, wvermajor, wverminor, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadTypeLib<P0>(szfile: P0) -> windows_core::Result<super::Com::ITypeLib>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn LoadTypeLib(szfile : windows_core::PCWSTR, pptlib : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    LoadTypeLib(szfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn LoadTypeLibEx<P0>(szfile: P0, regkind: REGKIND) -> windows_core::Result<super::Com::ITypeLib>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn LoadTypeLibEx(szfile : windows_core::PCWSTR, regkind : REGKIND, pptlib : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    LoadTypeLibEx(szfile.param().abi(), regkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OaBuildVersion() -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn OaBuildVersion() -> u32);
    OaBuildVersion()
}
#[inline]
pub unsafe fn OaEnablePerUserTLibRegistration() {
    windows_targets::link!("oleaut32.dll" "system" fn OaEnablePerUserTLibRegistration());
    OaEnablePerUserTLibRegistration()
}
#[inline]
pub unsafe fn OleBuildVersion() -> u32 {
    windows_targets::link!("ole32.dll" "system" fn OleBuildVersion() -> u32);
    OleBuildVersion()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorage2<P0>(lpolestream: *const super::Com::StructuredStorage::OLESTREAM, pstg: P0, ptd: Option<*const super::Com::DVTARGETDEVICE>, opt: u32, pvcallbackcontext: Option<*const core::ffi::c_void>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorage2(lpolestream : *const super::Com::StructuredStorage:: OLESTREAM, pstg : * mut core::ffi::c_void, ptd : *const super::Com:: DVTARGETDEVICE, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    OleConvertOLESTREAMToIStorage2(lpolestream, pstg.param().abi(), core::mem::transmute(ptd.unwrap_or(std::ptr::null())), opt, core::mem::transmute(pvcallbackcontext.unwrap_or(std::ptr::null())), pqueryconvertolelinkcallback).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn OleConvertOLESTREAMToIStorageEx2<P0>(polestm: *const super::Com::StructuredStorage::OLESTREAM, pstg: P0, pcfformat: *mut u16, plwwidth: *mut i32, plheight: *mut i32, pdwsize: *mut u32, pmedium: *mut super::Com::STGMEDIUM, opt: u32, pvcallbackcontext: Option<*const core::ffi::c_void>, pqueryconvertolelinkcallback: OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleConvertOLESTREAMToIStorageEx2(polestm : *const super::Com::StructuredStorage:: OLESTREAM, pstg : * mut core::ffi::c_void, pcfformat : *mut u16, plwwidth : *mut i32, plheight : *mut i32, pdwsize : *mut u32, pmedium : *mut super::Com:: STGMEDIUM, opt : u32, pvcallbackcontext : *const core::ffi::c_void, pqueryconvertolelinkcallback : OLESTREAMQUERYCONVERTOLELINKCALLBACK) -> windows_core::HRESULT);
    OleConvertOLESTREAMToIStorageEx2(polestm, pstg.param().abi(), pcfformat, plwwidth, plheight, pdwsize, pmedium, opt, core::mem::transmute(pvcallbackcontext.unwrap_or(std::ptr::null())), pqueryconvertolelinkcallback).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreate<P0, P1>(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P0, pstg: P1, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<IOleClientSite>,
    P1: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreate(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreate(rclsid, riid, renderopt.0 as _, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[inline]
pub unsafe fn OleCreateDefaultHandler<P0>(clsid: *const windows_core::GUID, punkouter: P0, riid: *const windows_core::GUID, lplpobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateDefaultHandler(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, riid : *const windows_core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateDefaultHandler(clsid, punkouter.param().abi(), riid, lplpobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleCreateEmbeddingHelper<P0, P1>(clsid: *const windows_core::GUID, punkouter: P0, flags: EMBDHLP_FLAGS, pcf: P1, riid: *const windows_core::GUID, lplpobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::Com::IClassFactory>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateEmbeddingHelper(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, flags : EMBDHLP_FLAGS, pcf : * mut core::ffi::c_void, riid : *const windows_core::GUID, lplpobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateEmbeddingHelper(clsid, punkouter.param().abi(), flags, pcf.param().abi(), riid, lplpobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateEx<P0, P1, P2>(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P0, rgdwconnection: *mut u32, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IAdviseSink>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateEx(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateEx(rclsid, riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleCreateFontIndirect(lpfontdesc: *const FONTDESC, riid: *const windows_core::GUID, lplpvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn OleCreateFontIndirect(lpfontdesc : *const FONTDESC, riid : *const windows_core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateFontIndirect(lpfontdesc, riid, lplpvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromData<P0, P1, P2>(psrcdataobj: P0, riid: *const windows_core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateFromData(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateFromData(psrcdataobj.param().abi(), riid, renderopt.0 as _, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromDataEx<P0, P1, P2, P3>(psrcdataobj: P0, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<super::Com::IAdviseSink>,
    P2: windows_core::Param<IOleClientSite>,
    P3: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateFromDataEx(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromFile<P0, P1, P2>(rclsid: *const windows_core::GUID, lpszfilename: P0, riid: *const windows_core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateFromFile(rclsid : *const windows_core::GUID, lpszfilename : windows_core::PCWSTR, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateFromFile(rclsid, lpszfilename.param().abi(), riid, renderopt.0 as _, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateFromFileEx<P0, P1, P2, P3>(rclsid: *const windows_core::GUID, lpszfilename: P0, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Com::IAdviseSink>,
    P2: windows_core::Param<IOleClientSite>,
    P3: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateFromFileEx(rclsid : *const windows_core::GUID, lpszfilename : windows_core::PCWSTR, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateFromFileEx(rclsid, lpszfilename.param().abi(), riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLink<P0, P1, P2>(pmklinksrc: P0, riid: *const windows_core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IMoniker>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLink(pmklinksrc : * mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLink(pmklinksrc.param().abi(), riid, renderopt.0 as _, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkEx<P0, P1, P2, P3>(pmklinksrc: P0, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IMoniker>,
    P1: windows_core::Param<super::Com::IAdviseSink>,
    P2: windows_core::Param<IOleClientSite>,
    P3: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLinkEx(pmklinksrc : * mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLinkEx(pmklinksrc.param().abi(), riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkFromData<P0, P1, P2>(psrcdataobj: P0, riid: *const windows_core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLinkFromData(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLinkFromData(psrcdataobj.param().abi(), riid, renderopt.0 as _, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkFromDataEx<P0, P1, P2, P3>(psrcdataobj: P0, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<super::Com::IAdviseSink>,
    P2: windows_core::Param<IOleClientSite>,
    P3: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLinkFromDataEx(psrcdataobj : * mut core::ffi::c_void, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLinkFromDataEx(psrcdataobj.param().abi(), riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkToFile<P0, P1, P2>(lpszfilename: P0, riid: *const windows_core::GUID, renderopt: OLERENDER, lpformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLinkToFile(lpszfilename : windows_core::PCWSTR, riid : *const windows_core::GUID, renderopt : u32, lpformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLinkToFile(lpszfilename.param().abi(), riid, renderopt.0 as _, lpformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateLinkToFileEx<P0, P1, P2, P3>(lpszfilename: P0, riid: *const windows_core::GUID, dwflags: OLECREATE, renderopt: OLERENDER, cformats: u32, rgadvf: *const u32, rgformatetc: *const super::Com::FORMATETC, lpadvisesink: P1, rgdwconnection: *mut u32, pclientsite: P2, pstg: P3, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Com::IAdviseSink>,
    P2: windows_core::Param<IOleClientSite>,
    P3: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateLinkToFileEx(lpszfilename : windows_core::PCWSTR, riid : *const windows_core::GUID, dwflags : OLECREATE, renderopt : u32, cformats : u32, rgadvf : *const u32, rgformatetc : *const super::Com:: FORMATETC, lpadvisesink : * mut core::ffi::c_void, rgdwconnection : *mut u32, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateLinkToFileEx(lpszfilename.param().abi(), riid, dwflags, renderopt.0 as _, cformats, rgadvf, rgformatetc, lpadvisesink.param().abi(), rgdwconnection, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleCreateMenuDescriptor<P0>(hmenucombined: P0, lpmenuwidths: *const OLEMENUGROUPWIDTHS) -> isize
where
    P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateMenuDescriptor(hmenucombined : super::super::UI::WindowsAndMessaging:: HMENU, lpmenuwidths : *const OLEMENUGROUPWIDTHS) -> isize);
    OleCreateMenuDescriptor(hmenucombined.param().abi(), lpmenuwidths)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleCreatePictureIndirect<P0, T>(lppictdesc: *const PICTDESC, fown: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    T: windows_core::Interface,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleCreatePictureIndirect(lppictdesc : *const PICTDESC, riid : *const windows_core::GUID, fown : super::super::Foundation:: BOOL, lplpvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    OleCreatePictureIndirect(lppictdesc, &T::IID, fown.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OleCreatePropertyFrame<P0, P1>(hwndowner: P0, x: u32, y: u32, lpszcaption: P1, cobjects: u32, ppunk: *const Option<windows_core::IUnknown>, cpages: u32, ppageclsid: *const windows_core::GUID, lcid: u32, dwreserved: u32, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleCreatePropertyFrame(hwndowner : super::super::Foundation:: HWND, x : u32, y : u32, lpszcaption : windows_core::PCWSTR, cobjects : u32, ppunk : *const * mut core::ffi::c_void, cpages : u32, ppageclsid : *const windows_core::GUID, lcid : u32, dwreserved : u32, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    OleCreatePropertyFrame(hwndowner.param().abi(), x, y, lpszcaption.param().abi(), cobjects, core::mem::transmute(ppunk), cpages, ppageclsid, lcid, dwreserved, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn OleCreatePropertyFrameIndirect(lpparams: *const OCPFIPARAMS) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn OleCreatePropertyFrameIndirect(lpparams : *const OCPFIPARAMS) -> windows_core::HRESULT);
    OleCreatePropertyFrameIndirect(lpparams).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleCreateStaticFromData<P0, P1, P2>(psrcdataobj: P0, iid: *const windows_core::GUID, renderopt: OLERENDER, pformatetc: *const super::Com::FORMATETC, pclientsite: P1, pstg: P2, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
    P1: windows_core::Param<IOleClientSite>,
    P2: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleCreateStaticFromData(psrcdataobj : * mut core::ffi::c_void, iid : *const windows_core::GUID, renderopt : u32, pformatetc : *const super::Com:: FORMATETC, pclientsite : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleCreateStaticFromData(psrcdataobj.param().abi(), iid, renderopt.0 as _, pformatetc, pclientsite.param().abi(), pstg.param().abi(), ppvobj).ok()
}
#[inline]
pub unsafe fn OleDestroyMenuDescriptor(holemenu: isize) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn OleDestroyMenuDescriptor(holemenu : isize) -> windows_core::HRESULT);
    OleDestroyMenuDescriptor(holemenu).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleDoAutoConvert<P0>(pstg: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<super::Com::StructuredStorage::IStorage>,
{
    windows_targets::link!("ole32.dll" "system" fn OleDoAutoConvert(pstg : * mut core::ffi::c_void, pclsidnew : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleDoAutoConvert(pstg.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleDraw<P0, P1>(punknown: P0, dwaspect: u32, hdcdraw: P1, lprcbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::Graphics::Gdi::HDC>,
{
    windows_targets::link!("ole32.dll" "system" fn OleDraw(punknown : * mut core::ffi::c_void, dwaspect : u32, hdcdraw : super::super::Graphics::Gdi:: HDC, lprcbounds : *const super::super::Foundation:: RECT) -> windows_core::HRESULT);
    OleDraw(punknown.param().abi(), dwaspect, hdcdraw.param().abi(), lprcbounds).ok()
}
#[cfg(feature = "Win32_System_Memory")]
#[inline]
pub unsafe fn OleDuplicateData<P0>(hsrc: P0, cfformat: CLIPBOARD_FORMAT, uiflags: super::Memory::GLOBAL_ALLOC_FLAGS) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("ole32.dll" "system" fn OleDuplicateData(hsrc : super::super::Foundation:: HANDLE, cfformat : CLIPBOARD_FORMAT, uiflags : super::Memory:: GLOBAL_ALLOC_FLAGS) -> super::super::Foundation:: HANDLE);
    OleDuplicateData(hsrc.param().abi(), cfformat, uiflags)
}
#[inline]
pub unsafe fn OleFlushClipboard() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn OleFlushClipboard() -> windows_core::HRESULT);
    OleFlushClipboard().ok()
}
#[inline]
pub unsafe fn OleGetAutoConvert(clsidold: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("ole32.dll" "system" fn OleGetAutoConvert(clsidold : *const windows_core::GUID, pclsidnew : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleGetAutoConvert(clsidold, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleGetClipboard() -> windows_core::Result<super::Com::IDataObject> {
    windows_targets::link!("ole32.dll" "system" fn OleGetClipboard(ppdataobj : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleGetClipboard(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleGetClipboardWithEnterpriseInfo(dataobject: *mut Option<super::Com::IDataObject>, dataenterpriseid: *mut windows_core::PWSTR, sourcedescription: *mut windows_core::PWSTR, targetdescription: *mut windows_core::PWSTR, datadescription: *mut windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn OleGetClipboardWithEnterpriseInfo(dataobject : *mut * mut core::ffi::c_void, dataenterpriseid : *mut windows_core::PWSTR, sourcedescription : *mut windows_core::PWSTR, targetdescription : *mut windows_core::PWSTR, datadescription : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    OleGetClipboardWithEnterpriseInfo(core::mem::transmute(dataobject), dataenterpriseid, sourcedescription, targetdescription, datadescription).ok()
}
#[inline]
pub unsafe fn OleGetIconOfClass<P0, P1>(rclsid: *const windows_core::GUID, lpszlabel: P0, fusetypeaslabel: P1) -> super::super::Foundation::HGLOBAL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleGetIconOfClass(rclsid : *const windows_core::GUID, lpszlabel : windows_core::PCWSTR, fusetypeaslabel : super::super::Foundation:: BOOL) -> super::super::Foundation:: HGLOBAL);
    OleGetIconOfClass(rclsid, lpszlabel.param().abi(), fusetypeaslabel.param().abi())
}
#[inline]
pub unsafe fn OleGetIconOfFile<P0, P1>(lpszpath: P0, fusefileaslabel: P1) -> super::super::Foundation::HGLOBAL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleGetIconOfFile(lpszpath : windows_core::PCWSTR, fusefileaslabel : super::super::Foundation:: BOOL) -> super::super::Foundation:: HGLOBAL);
    OleGetIconOfFile(lpszpath.param().abi(), fusefileaslabel.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleIconToCursor<P0, P1>(hinstexe: P0, hicon: P1) -> super::super::UI::WindowsAndMessaging::HCURSOR
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleIconToCursor(hinstexe : super::super::Foundation:: HINSTANCE, hicon : super::super::UI::WindowsAndMessaging:: HICON) -> super::super::UI::WindowsAndMessaging:: HCURSOR);
    OleIconToCursor(hinstexe.param().abi(), hicon.param().abi())
}
#[inline]
pub unsafe fn OleInitialize(pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    OleInitialize(core::mem::transmute(pvreserved.unwrap_or(std::ptr::null()))).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleIsCurrentClipboard<P0>(pdataobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleIsCurrentClipboard(pdataobj : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleIsCurrentClipboard(pdataobj.param().abi()).ok()
}
#[inline]
pub unsafe fn OleIsRunning<P0>(pobject: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IOleObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleIsRunning(pobject : * mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    OleIsRunning(pobject.param().abi())
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleLoad<P0, P1>(pstg: P0, riid: *const windows_core::GUID, pclientsite: P1, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::StructuredStorage::IStorage>,
    P1: windows_core::Param<IOleClientSite>,
{
    windows_targets::link!("ole32.dll" "system" fn OleLoad(pstg : * mut core::ffi::c_void, riid : *const windows_core::GUID, pclientsite : * mut core::ffi::c_void, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleLoad(pstg.param().abi(), riid, pclientsite.param().abi(), ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadFromStream<P0>(pstm: P0, iidinterface: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IStream>,
{
    windows_targets::link!("ole32.dll" "system" fn OleLoadFromStream(pstm : * mut core::ffi::c_void, iidinterface : *const windows_core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleLoadFromStream(pstm.param().abi(), iidinterface, ppvobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadPicture<P0, P1>(lpstream: P0, lsize: i32, frunmode: P1, riid: *const windows_core::GUID, lplpvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IStream>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleLoadPicture(lpstream : * mut core::ffi::c_void, lsize : i32, frunmode : super::super::Foundation:: BOOL, riid : *const windows_core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleLoadPicture(lpstream.param().abi(), lsize, frunmode.param().abi(), riid, lplpvobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadPictureEx<P0, P1>(lpstream: P0, lsize: i32, frunmode: P1, riid: *const windows_core::GUID, xsizedesired: u32, ysizedesired: u32, dwflags: LOAD_PICTURE_FLAGS, lplpvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IStream>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureEx(lpstream : * mut core::ffi::c_void, lsize : i32, frunmode : super::super::Foundation:: BOOL, riid : *const windows_core::GUID, xsizedesired : u32, ysizedesired : u32, dwflags : LOAD_PICTURE_FLAGS, lplpvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleLoadPictureEx(lpstream.param().abi(), lsize, frunmode.param().abi(), riid, xsizedesired, ysizedesired, dwflags, lplpvobj).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadPictureFile<P0>(varfilename: P0) -> windows_core::Result<super::Com::IDispatch>
where
    P0: windows_core::Param<windows_core::VARIANT>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureFile(varfilename : core::mem::MaybeUninit < windows_core::VARIANT >, lplpdisppicture : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleLoadPictureFile(varfilename.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleLoadPictureFileEx<P0>(varfilename: P0, xsizedesired: u32, ysizedesired: u32, dwflags: LOAD_PICTURE_FLAGS) -> windows_core::Result<super::Com::IDispatch>
where
    P0: windows_core::Param<windows_core::VARIANT>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleLoadPictureFileEx(varfilename : core::mem::MaybeUninit < windows_core::VARIANT >, xsizedesired : u32, ysizedesired : u32, dwflags : LOAD_PICTURE_FLAGS, lplpdisppicture : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleLoadPictureFileEx(varfilename.param().abi(), xsizedesired, ysizedesired, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OleLoadPicturePath<P0, P1>(szurlorpath: P0, punkcaller: P1, dwreserved: u32, clrreserved: u32, riid: *const windows_core::GUID, ppvret: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleLoadPicturePath(szurlorpath : windows_core::PCWSTR, punkcaller : * mut core::ffi::c_void, dwreserved : u32, clrreserved : u32, riid : *const windows_core::GUID, ppvret : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    OleLoadPicturePath(szurlorpath.param().abi(), punkcaller.param().abi(), dwreserved, clrreserved, riid, ppvret).ok()
}
#[inline]
pub unsafe fn OleLockRunning<P0, P1, P2>(punknown: P0, flock: P1, flastunlockcloses: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleLockRunning(punknown : * mut core::ffi::c_void, flock : super::super::Foundation:: BOOL, flastunlockcloses : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    OleLockRunning(punknown.param().abi(), flock.param().abi(), flastunlockcloses.param().abi()).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleMetafilePictFromIconAndLabel<P0, P1, P2>(hicon: P0, lpszlabel: P1, lpszsourcefile: P2, iiconindex: u32) -> windows_core::Result<super::super::Foundation::HGLOBAL>
where
    P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn OleMetafilePictFromIconAndLabel(hicon : super::super::UI::WindowsAndMessaging:: HICON, lpszlabel : windows_core::PCWSTR, lpszsourcefile : windows_core::PCWSTR, iiconindex : u32) -> super::super::Foundation:: HGLOBAL);
    let result__ = OleMetafilePictFromIconAndLabel(hicon.param().abi(), lpszlabel.param().abi(), lpszsourcefile.param().abi(), iiconindex);
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OleNoteObjectVisible<P0, P1>(punknown: P0, fvisible: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleNoteObjectVisible(punknown : * mut core::ffi::c_void, fvisible : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    OleNoteObjectVisible(punknown.param().abi(), fvisible.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryCreateFromData<P0>(psrcdataobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleQueryCreateFromData(psrcdataobject : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleQueryCreateFromData(psrcdataobject.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleQueryLinkFromData<P0>(psrcdataobject: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleQueryLinkFromData(psrcdataobject : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleQueryLinkFromData(psrcdataobject.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleRegEnumFormatEtc(clsid: *const windows_core::GUID, dwdirection: u32) -> windows_core::Result<super::Com::IEnumFORMATETC> {
    windows_targets::link!("ole32.dll" "system" fn OleRegEnumFormatEtc(clsid : *const windows_core::GUID, dwdirection : u32, ppenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleRegEnumFormatEtc(clsid, dwdirection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OleRegEnumVerbs(clsid: *const windows_core::GUID) -> windows_core::Result<IEnumOLEVERB> {
    windows_targets::link!("ole32.dll" "system" fn OleRegEnumVerbs(clsid : *const windows_core::GUID, ppenum : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleRegEnumVerbs(clsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn OleRegGetMiscStatus(clsid: *const windows_core::GUID, dwaspect: u32) -> windows_core::Result<u32> {
    windows_targets::link!("ole32.dll" "system" fn OleRegGetMiscStatus(clsid : *const windows_core::GUID, dwaspect : u32, pdwstatus : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleRegGetMiscStatus(clsid, dwaspect, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OleRegGetUserType(clsid: *const windows_core::GUID, dwformoftype: USERCLASSTYPE) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("ole32.dll" "system" fn OleRegGetUserType(clsid : *const windows_core::GUID, dwformoftype : u32, pszusertype : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleRegGetUserType(clsid, dwformoftype.0 as _, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn OleRun<P0>(punknown: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn OleRun(punknown : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleRun(punknown.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleSave<P0, P1, P2>(pps: P0, pstg: P1, fsameasload: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::StructuredStorage::IPersistStorage>,
    P1: windows_core::Param<super::Com::StructuredStorage::IStorage>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleSave(pps : * mut core::ffi::c_void, pstg : * mut core::ffi::c_void, fsameasload : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    OleSave(pps.param().abi(), pstg.param().abi(), fsameasload.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSavePictureFile<P0, P1>(lpdisppicture: P0, bstrfilename: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDispatch>,
    P1: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleSavePictureFile(lpdisppicture : * mut core::ffi::c_void, bstrfilename : core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    OleSavePictureFile(lpdisppicture.param().abi(), bstrfilename.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSaveToStream<P0, P1>(ppstm: P0, pstm: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IPersistStream>,
    P1: windows_core::Param<super::Com::IStream>,
{
    windows_targets::link!("ole32.dll" "system" fn OleSaveToStream(ppstm : * mut core::ffi::c_void, pstm : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleSaveToStream(ppstm.param().abi(), pstm.param().abi()).ok()
}
#[inline]
pub unsafe fn OleSetAutoConvert(clsidold: *const windows_core::GUID, clsidnew: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn OleSetAutoConvert(clsidold : *const windows_core::GUID, clsidnew : *const windows_core::GUID) -> windows_core::HRESULT);
    OleSetAutoConvert(clsidold, clsidnew).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleSetClipboard<P0>(pdataobj: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDataObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleSetClipboard(pdataobj : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleSetClipboard(pdataobj.param().abi()).ok()
}
#[inline]
pub unsafe fn OleSetContainedObject<P0, P1>(punknown: P0, fcontained: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn OleSetContainedObject(punknown : * mut core::ffi::c_void, fcontained : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    OleSetContainedObject(punknown.param().abi(), fcontained.param().abi()).ok()
}
#[inline]
pub unsafe fn OleSetMenuDescriptor<P0, P1, P2, P3>(holemenu: isize, hwndframe: P0, hwndactiveobject: P1, lpframe: P2, lpactiveobj: P3) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<IOleInPlaceFrame>,
    P3: windows_core::Param<IOleInPlaceActiveObject>,
{
    windows_targets::link!("ole32.dll" "system" fn OleSetMenuDescriptor(holemenu : isize, hwndframe : super::super::Foundation:: HWND, hwndactiveobject : super::super::Foundation:: HWND, lpframe : * mut core::ffi::c_void, lpactiveobj : * mut core::ffi::c_void) -> windows_core::HRESULT);
    OleSetMenuDescriptor(holemenu, hwndframe.param().abi(), hwndactiveobject.param().abi(), lpframe.param().abi(), lpactiveobj.param().abi()).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleTranslateAccelerator<P0>(lpframe: P0, lpframeinfo: *const OLEINPLACEFRAMEINFO, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>
where
    P0: windows_core::Param<IOleInPlaceFrame>,
{
    windows_targets::link!("ole32.dll" "system" fn OleTranslateAccelerator(lpframe : * mut core::ffi::c_void, lpframeinfo : *const OLEINPLACEFRAMEINFO, lpmsg : *const super::super::UI::WindowsAndMessaging:: MSG) -> windows_core::HRESULT);
    OleTranslateAccelerator(lpframe.param().abi(), lpframeinfo, lpmsg).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn OleTranslateColor<P0>(clr: u32, hpal: P0) -> windows_core::Result<super::super::Foundation::COLORREF>
where
    P0: windows_core::Param<super::super::Graphics::Gdi::HPALETTE>,
{
    windows_targets::link!("oleaut32.dll" "system" fn OleTranslateColor(clr : u32, hpal : super::super::Graphics::Gdi:: HPALETTE, lpcolorref : *mut super::super::Foundation:: COLORREF) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    OleTranslateColor(clr, hpal.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleUIAddVerbMenuA<P0, P1, P2, P3>(lpoleobj: P0, lpszshorttype: P1, hmenu: P2, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: P3, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IOleObject>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("oledlg.dll" "system" fn OleUIAddVerbMenuA(lpoleobj : * mut core::ffi::c_void, lpszshorttype : windows_core::PCSTR, hmenu : super::super::UI::WindowsAndMessaging:: HMENU, upos : u32, uidverbmin : u32, uidverbmax : u32, baddconvert : super::super::Foundation:: BOOL, idconvert : u32, lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU) -> super::super::Foundation:: BOOL);
    OleUIAddVerbMenuA(lpoleobj.param().abi(), lpszshorttype.param().abi(), hmenu.param().abi(), upos, uidverbmin, uidverbmax, baddconvert.param().abi(), idconvert, lphmenu)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn OleUIAddVerbMenuW<P0, P1, P2, P3>(lpoleobj: P0, lpszshorttype: P1, hmenu: P2, upos: u32, uidverbmin: u32, uidverbmax: u32, baddconvert: P3, idconvert: u32, lphmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IOleObject>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("oledlg.dll" "system" fn OleUIAddVerbMenuW(lpoleobj : * mut core::ffi::c_void, lpszshorttype : windows_core::PCWSTR, hmenu : super::super::UI::WindowsAndMessaging:: HMENU, upos : u32, uidverbmin : u32, uidverbmax : u32, baddconvert : super::super::Foundation:: BOOL, idconvert : u32, lphmenu : *mut super::super::UI::WindowsAndMessaging:: HMENU) -> super::super::Foundation:: BOOL);
    OleUIAddVerbMenuW(lpoleobj.param().abi(), lpszshorttype.param().abi(), hmenu.param().abi(), upos, uidverbmin, uidverbmax, baddconvert.param().abi(), idconvert, lphmenu)
}
#[cfg(feature = "Win32_Media")]
#[inline]
pub unsafe fn OleUIBusyA(param0: *const OLEUIBUSYA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIBusyA(param0 : *const OLEUIBUSYA) -> u32);
    OleUIBusyA(param0)
}
#[cfg(feature = "Win32_Media")]
#[inline]
pub unsafe fn OleUIBusyW(param0: *const OLEUIBUSYW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIBusyW(param0 : *const OLEUIBUSYW) -> u32);
    OleUIBusyW(param0)
}
#[inline]
pub unsafe fn OleUICanConvertOrActivateAs<P0>(rclsid: *const windows_core::GUID, fislinkedobject: P0, wformat: u16) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("oledlg.dll" "system" fn OleUICanConvertOrActivateAs(rclsid : *const windows_core::GUID, fislinkedobject : super::super::Foundation:: BOOL, wformat : u16) -> super::super::Foundation:: BOOL);
    OleUICanConvertOrActivateAs(rclsid, fislinkedobject.param().abi(), wformat)
}
#[inline]
pub unsafe fn OleUIChangeIconA(param0: *const OLEUICHANGEICONA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIChangeIconA(param0 : *const OLEUICHANGEICONA) -> u32);
    OleUIChangeIconA(param0)
}
#[inline]
pub unsafe fn OleUIChangeIconW(param0: *const OLEUICHANGEICONW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIChangeIconW(param0 : *const OLEUICHANGEICONW) -> u32);
    OleUIChangeIconW(param0)
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[inline]
pub unsafe fn OleUIChangeSourceA(param0: *const OLEUICHANGESOURCEA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIChangeSourceA(param0 : *const OLEUICHANGESOURCEA) -> u32);
    OleUIChangeSourceA(param0)
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[inline]
pub unsafe fn OleUIChangeSourceW(param0: *const OLEUICHANGESOURCEW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIChangeSourceW(param0 : *const OLEUICHANGESOURCEW) -> u32);
    OleUIChangeSourceW(param0)
}
#[inline]
pub unsafe fn OleUIConvertA(param0: *const OLEUICONVERTA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIConvertA(param0 : *const OLEUICONVERTA) -> u32);
    OleUIConvertA(param0)
}
#[inline]
pub unsafe fn OleUIConvertW(param0: *const OLEUICONVERTW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIConvertW(param0 : *const OLEUICONVERTW) -> u32);
    OleUIConvertW(param0)
}
#[inline]
pub unsafe fn OleUIEditLinksA(param0: *const OLEUIEDITLINKSA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIEditLinksA(param0 : *const OLEUIEDITLINKSA) -> u32);
    OleUIEditLinksA(param0)
}
#[inline]
pub unsafe fn OleUIEditLinksW(param0: *const OLEUIEDITLINKSW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIEditLinksW(param0 : *const OLEUIEDITLINKSW) -> u32);
    OleUIEditLinksW(param0)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleUIInsertObjectA(param0: *const OLEUIINSERTOBJECTA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIInsertObjectA(param0 : *const OLEUIINSERTOBJECTA) -> u32);
    OleUIInsertObjectA(param0)
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[inline]
pub unsafe fn OleUIInsertObjectW(param0: *const OLEUIINSERTOBJECTW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIInsertObjectW(param0 : *const OLEUIINSERTOBJECTW) -> u32);
    OleUIInsertObjectW(param0)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesA(param0: *const OLEUIOBJECTPROPSA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIObjectPropertiesA(param0 : *const OLEUIOBJECTPROPSA) -> u32);
    OleUIObjectPropertiesA(param0)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn OleUIObjectPropertiesW(param0: *const OLEUIOBJECTPROPSW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIObjectPropertiesW(param0 : *const OLEUIOBJECTPROPSW) -> u32);
    OleUIObjectPropertiesW(param0)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleUIPasteSpecialA(param0: *const OLEUIPASTESPECIALA) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIPasteSpecialA(param0 : *const OLEUIPASTESPECIALA) -> u32);
    OleUIPasteSpecialA(param0)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn OleUIPasteSpecialW(param0: *const OLEUIPASTESPECIALW) -> u32 {
    windows_targets::link!("oledlg.dll" "system" fn OleUIPasteSpecialW(param0 : *const OLEUIPASTESPECIALW) -> u32);
    OleUIPasteSpecialW(param0)
}
#[inline]
pub unsafe fn OleUIPromptUserA<P0>(ntemplate: i32, hwndparent: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oledlg.dll" "cdecl" fn OleUIPromptUserA(ntemplate : i32, hwndparent : super::super::Foundation:: HWND) -> i32);
    OleUIPromptUserA(ntemplate, hwndparent.param().abi())
}
#[inline]
pub unsafe fn OleUIPromptUserW<P0>(ntemplate: i32, hwndparent: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oledlg.dll" "cdecl" fn OleUIPromptUserW(ntemplate : i32, hwndparent : super::super::Foundation:: HWND) -> i32);
    OleUIPromptUserW(ntemplate, hwndparent.param().abi())
}
#[inline]
pub unsafe fn OleUIUpdateLinksA<P0, P1, P2>(lpoleuilinkcntr: P0, hwndparent: P1, lpsztitle: P2, clinks: i32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IOleUILinkContainerA>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("oledlg.dll" "system" fn OleUIUpdateLinksA(lpoleuilinkcntr : * mut core::ffi::c_void, hwndparent : super::super::Foundation:: HWND, lpsztitle : windows_core::PCSTR, clinks : i32) -> super::super::Foundation:: BOOL);
    OleUIUpdateLinksA(lpoleuilinkcntr.param().abi(), hwndparent.param().abi(), lpsztitle.param().abi(), clinks)
}
#[inline]
pub unsafe fn OleUIUpdateLinksW<P0, P1, P2>(lpoleuilinkcntr: P0, hwndparent: P1, lpsztitle: P2, clinks: i32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<IOleUILinkContainerW>,
    P1: windows_core::Param<super::super::Foundation::HWND>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oledlg.dll" "system" fn OleUIUpdateLinksW(lpoleuilinkcntr : * mut core::ffi::c_void, hwndparent : super::super::Foundation:: HWND, lpsztitle : windows_core::PCWSTR, clinks : i32) -> super::super::Foundation:: BOOL);
    OleUIUpdateLinksW(lpoleuilinkcntr.param().abi(), hwndparent.param().abi(), lpsztitle.param().abi(), clinks)
}
#[inline]
pub unsafe fn OleUninitialize() {
    windows_targets::link!("ole32.dll" "system" fn OleUninitialize());
    OleUninitialize()
}
#[inline]
pub unsafe fn QueryPathOfRegTypeLib(guid: *const windows_core::GUID, wmaj: u16, wmin: u16, lcid: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn QueryPathOfRegTypeLib(guid : *const windows_core::GUID, wmaj : u16, wmin : u16, lcid : u32, lpbstrpathname : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    QueryPathOfRegTypeLib(guid, wmaj, wmin, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RegisterActiveObject<P0>(punk: P0, rclsid: *const windows_core::GUID, dwflags: ACTIVEOBJECT_FLAGS, pdwregister: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("oleaut32.dll" "system" fn RegisterActiveObject(punk : * mut core::ffi::c_void, rclsid : *const windows_core::GUID, dwflags : ACTIVEOBJECT_FLAGS, pdwregister : *mut u32) -> windows_core::HRESULT);
    RegisterActiveObject(punk.param().abi(), rclsid, dwflags, pdwregister).ok()
}
#[inline]
pub unsafe fn RegisterDragDrop<P0, P1>(hwnd: P0, pdroptarget: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<IDropTarget>,
{
    windows_targets::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::super::Foundation:: HWND, pdroptarget : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RegisterDragDrop(hwnd.param().abi(), pdroptarget.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RegisterTypeLib<P0, P1, P2>(ptlib: P0, szfullpath: P1, szhelpdir: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::ITypeLib>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn RegisterTypeLib(ptlib : * mut core::ffi::c_void, szfullpath : windows_core::PCWSTR, szhelpdir : windows_core::PCWSTR) -> windows_core::HRESULT);
    RegisterTypeLib(ptlib.param().abi(), szfullpath.param().abi(), szhelpdir.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RegisterTypeLibForUser<P0, P1, P2>(ptlib: P0, szfullpath: P1, szhelpdir: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::ITypeLib>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn RegisterTypeLibForUser(ptlib : * mut core::ffi::c_void, szfullpath : windows_core::PCWSTR, szhelpdir : windows_core::PCWSTR) -> windows_core::HRESULT);
    RegisterTypeLibForUser(ptlib.param().abi(), szfullpath.param().abi(), szhelpdir.param().abi()).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM) {
    windows_targets::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : *mut super::Com:: STGMEDIUM));
    ReleaseStgMedium(param0)
}
#[inline]
pub unsafe fn RevokeActiveObject(dwregister: u32, pvreserved: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn RevokeActiveObject(dwregister : u32, pvreserved : *mut core::ffi::c_void) -> windows_core::HRESULT);
    RevokeActiveObject(dwregister, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RevokeDragDrop<P0>(hwnd: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    RevokeDragDrop(hwnd.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAccessData(psa: *const super::Com::SAFEARRAY, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAccessData(psa : *const super::Com:: SAFEARRAY, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    SafeArrayAccessData(psa, ppvdata).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAddRef(psa: *const super::Com::SAFEARRAY, ppdatatorelease: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAddRef(psa : *const super::Com:: SAFEARRAY, ppdatatorelease : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    SafeArrayAddRef(psa, ppdatatorelease).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAllocData(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocData(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayAllocData(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayAllocDescriptor(cdims: u32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptor(cdims : u32, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayAllocDescriptor(cdims, &mut result__).map(|| result__)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayAllocDescriptorEx(vt: super::Variant::VARENUM, cdims: u32) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayAllocDescriptorEx(vt : super::Variant:: VARENUM, cdims : u32, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayAllocDescriptorEx(vt, cdims, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCopy(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<*mut super::Com::SAFEARRAY> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCopy(psa : *const super::Com:: SAFEARRAY, ppsaout : *mut *mut super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayCopy(psa, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayCopyData(psasource: *const super::Com::SAFEARRAY, psatarget: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCopyData(psasource : *const super::Com:: SAFEARRAY, psatarget : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayCopyData(psasource, psatarget).ok()
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayCreate(vt: super::Variant::VARENUM, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND) -> *mut super::Com::SAFEARRAY {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreate(vt : super::Variant:: VARENUM, cdims : u32, rgsabound : *const super::Com:: SAFEARRAYBOUND) -> *mut super::Com:: SAFEARRAY);
    SafeArrayCreate(vt, cdims, rgsabound)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayCreateEx(vt: super::Variant::VARENUM, cdims: u32, rgsabound: *const super::Com::SAFEARRAYBOUND, pvextra: *const core::ffi::c_void) -> *mut super::Com::SAFEARRAY {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateEx(vt : super::Variant:: VARENUM, cdims : u32, rgsabound : *const super::Com:: SAFEARRAYBOUND, pvextra : *const core::ffi::c_void) -> *mut super::Com:: SAFEARRAY);
    SafeArrayCreateEx(vt, cdims, rgsabound, pvextra)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayCreateVector(vt: super::Variant::VARENUM, llbound: i32, celements: u32) -> *mut super::Com::SAFEARRAY {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateVector(vt : super::Variant:: VARENUM, llbound : i32, celements : u32) -> *mut super::Com:: SAFEARRAY);
    SafeArrayCreateVector(vt, llbound, celements)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayCreateVectorEx(vt: super::Variant::VARENUM, llbound: i32, celements: u32, pvextra: *const core::ffi::c_void) -> *mut super::Com::SAFEARRAY {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayCreateVectorEx(vt : super::Variant:: VARENUM, llbound : i32, celements : u32, pvextra : *const core::ffi::c_void) -> *mut super::Com:: SAFEARRAY);
    SafeArrayCreateVectorEx(vt, llbound, celements, pvextra)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroy(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroy(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayDestroy(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroyData(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroyData(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayDestroyData(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayDestroyDescriptor(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayDestroyDescriptor(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayDestroyDescriptor(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetDim(psa: *const super::Com::SAFEARRAY) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetDim(psa : *const super::Com:: SAFEARRAY) -> u32);
    SafeArrayGetDim(psa)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetElement(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, pv : *mut core::ffi::c_void) -> windows_core::HRESULT);
    SafeArrayGetElement(psa, rgindices, pv).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetElemsize(psa: *const super::Com::SAFEARRAY) -> u32 {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetElemsize(psa : *const super::Com:: SAFEARRAY) -> u32);
    SafeArrayGetElemsize(psa)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetIID(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetIID(psa : *const super::Com:: SAFEARRAY, pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayGetIID(psa, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetLBound(psa: *const super::Com::SAFEARRAY, ndim: u32) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetLBound(psa : *const super::Com:: SAFEARRAY, ndim : u32, pllbound : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayGetLBound(psa, ndim, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetRecordInfo(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<IRecordInfo> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetRecordInfo(psa : *const super::Com:: SAFEARRAY, prinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayGetRecordInfo(psa, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayGetUBound(psa: *const super::Com::SAFEARRAY, ndim: u32) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetUBound(psa : *const super::Com:: SAFEARRAY, ndim : u32, plubound : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayGetUBound(psa, ndim, &mut result__).map(|| result__)
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[inline]
pub unsafe fn SafeArrayGetVartype(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<super::Variant::VARENUM> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayGetVartype(psa : *const super::Com:: SAFEARRAY, pvt : *mut super::Variant:: VARENUM) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    SafeArrayGetVartype(psa, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayLock(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayLock(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayLock(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayPtrOfIndex(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, ppvdata: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayPtrOfIndex(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, ppvdata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    SafeArrayPtrOfIndex(psa, rgindices, ppvdata).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayPutElement(psa: *const super::Com::SAFEARRAY, rgindices: *const i32, pv: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayPutElement(psa : *const super::Com:: SAFEARRAY, rgindices : *const i32, pv : *const core::ffi::c_void) -> windows_core::HRESULT);
    SafeArrayPutElement(psa, rgindices, pv).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayRedim(psa: *mut super::Com::SAFEARRAY, psaboundnew: *const super::Com::SAFEARRAYBOUND) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayRedim(psa : *mut super::Com:: SAFEARRAY, psaboundnew : *const super::Com:: SAFEARRAYBOUND) -> windows_core::HRESULT);
    SafeArrayRedim(psa, psaboundnew).ok()
}
#[inline]
pub unsafe fn SafeArrayReleaseData(pdata: *const core::ffi::c_void) {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayReleaseData(pdata : *const core::ffi::c_void));
    SafeArrayReleaseData(pdata)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayReleaseDescriptor(psa: *const super::Com::SAFEARRAY) {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayReleaseDescriptor(psa : *const super::Com:: SAFEARRAY));
    SafeArrayReleaseDescriptor(psa)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArraySetIID(psa: *const super::Com::SAFEARRAY, guid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArraySetIID(psa : *const super::Com:: SAFEARRAY, guid : *const windows_core::GUID) -> windows_core::HRESULT);
    SafeArraySetIID(psa, guid).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArraySetRecordInfo<P0>(psa: *const super::Com::SAFEARRAY, prinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRecordInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SafeArraySetRecordInfo(psa : *const super::Com:: SAFEARRAY, prinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    SafeArraySetRecordInfo(psa, prinfo.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayUnaccessData(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayUnaccessData(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayUnaccessData(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn SafeArrayUnlock(psa: *const super::Com::SAFEARRAY) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn SafeArrayUnlock(psa : *const super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    SafeArrayUnlock(psa).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UnRegisterTypeLib(libid: *const windows_core::GUID, wvermajor: u16, wverminor: u16, lcid: u32, syskind: super::Com::SYSKIND) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn UnRegisterTypeLib(libid : *const windows_core::GUID, wvermajor : u16, wverminor : u16, lcid : u32, syskind : super::Com:: SYSKIND) -> windows_core::HRESULT);
    UnRegisterTypeLib(libid, wvermajor, wverminor, lcid, syskind).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn UnRegisterTypeLibForUser(libid: *const windows_core::GUID, wmajorvernum: u16, wminorvernum: u16, lcid: u32, syskind: super::Com::SYSKIND) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn UnRegisterTypeLibForUser(libid : *const windows_core::GUID, wmajorvernum : u16, wminorvernum : u16, lcid : u32, syskind : super::Com:: SYSKIND) -> windows_core::HRESULT);
    UnRegisterTypeLibForUser(libid, wmajorvernum, wminorvernum, lcid, syskind).ok()
}
#[inline]
pub unsafe fn VarAbs(pvarin: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarAbs(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarAbs(core::mem::transmute(pvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarAdd(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarAdd(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarAdd(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarAnd(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarAnd(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarAnd(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBoolFromCy(cyin: super::Com::CY) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromCy(cyin : super::Com:: CY, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromDate(datein: f64) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDate(datein : f64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBoolFromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromI1(cin: i8) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI1(cin : i8, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromI2(sin: i16) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI2(sin : i16, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromI4(lin: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI4(lin : i32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromI8(i64in: i64) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromI8(i64in : i64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromR4(fltin: f32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromR4(fltin : f32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromR8(dblin: f64) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromR8(dblin : f64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromUI1(bin: u8) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI1(bin : u8, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromUI2(uiin: u16) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI2(uiin : u16, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromUI4(ulin: u32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI4(ulin : u32, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBoolFromUI8(i64in: u64) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBoolFromUI8(i64in : u64, pboolout : *mut super::super::Foundation:: VARIANT_BOOL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBoolFromUI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarBstrCat<P0, P1>(bstrleft: P0, bstrright: P1) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<windows_core::BSTR>,
    P1: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrCat(bstrleft : core::mem::MaybeUninit < windows_core::BSTR >, bstrright : core::mem::MaybeUninit < windows_core::BSTR >, pbstrresult : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrCat(bstrleft.param().abi(), bstrright.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrCmp<P0, P1>(bstrleft: P0, bstrright: P1, lcid: u32, dwflags: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::BSTR>,
    P1: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrCmp(bstrleft : core::mem::MaybeUninit < windows_core::BSTR >, bstrright : core::mem::MaybeUninit < windows_core::BSTR >, lcid : u32, dwflags : u32) -> windows_core::HRESULT);
    VarBstrCmp(bstrleft.param().abi(), bstrright.param().abi(), lcid, dwflags).ok()
}
#[inline]
pub unsafe fn VarBstrFromBool<P0>(boolin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromBool(boolin.param().abi(), lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBstrFromCy(cyin: super::Com::CY, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromCy(cyin : super::Com:: CY, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromCy(core::mem::transmute(cyin), lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromDate(datein: f64, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDate(datein : f64, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromDate(datein, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromDec(pdecin: *const super::super::Foundation::DECIMAL, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDec(pdecin : *const super::super::Foundation:: DECIMAL, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromDec(pdecin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarBstrFromDisp<P0>(pdispin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromDisp(pdispin.param().abi(), lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromI1(cin: i8, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI1(cin : i8, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromI1(cin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromI2(ival: i16, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI2(ival : i16, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromI2(ival, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromI4(lin: i32, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI4(lin : i32, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromI4(lin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromI8(i64in: i64, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromI8(i64in : i64, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromI8(i64in, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromR4(fltin: f32, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromR4(fltin : f32, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromR4(fltin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromR8(dblin: f64, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromR8(dblin : f64, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromR8(dblin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromUI1(bval: u8, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI1(bval : u8, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromUI1(bval, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromUI2(uiin: u16, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI2(uiin : u16, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromUI2(uiin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromUI4(ulin: u32, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI4(ulin : u32, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromUI4(ulin, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarBstrFromUI8(ui64in: u64, lcid: u32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarBstrFromUI8(ui64in : u64, lcid : u32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarBstrFromUI8(ui64in, lcid, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarCat(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCat(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCat(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarCmp(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT, lcid: u32, dwflags: u32) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarCmp(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, lcid : u32, dwflags : u32) -> VARCMP);
    VarCmp(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), lcid, dwflags)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyAbs(cyin: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyAbs(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyAbs(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyAdd(cyleft: super::Com::CY, cyright: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyAdd(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyAdd(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyCmp(cyleft: super::Com::CY, cyright: super::Com::CY) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyCmp(cyleft : super::Com:: CY, cyright : super::Com:: CY) -> VARCMP);
    VarCyCmp(core::mem::transmute(cyleft), core::mem::transmute(cyright))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyCmpR8(cyleft: super::Com::CY, dblright: f64) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyCmpR8(cyleft : super::Com:: CY, dblright : f64) -> VARCMP);
    VarCyCmpR8(core::mem::transmute(cyleft), dblright)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFix(cyin: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFix(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFix(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromBool<P0>(boolin: P0) -> windows_core::Result<super::Com::CY>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromDate(datein: f64) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDate(datein : f64, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromDate(datein, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<super::Com::CY>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI1(cin: i8) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI1(cin : i8, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromI1(cin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI2(sin: i16) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI2(sin : i16, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromI2(sin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI4(lin: i32) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI4(lin : i32, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromI4(lin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromI8(i64in: i64) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromI8(i64in : i64, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromI8(i64in, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromR4(fltin: f32) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromR4(fltin : f32, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromR4(fltin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromR8(dblin: f64) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromR8(dblin : f64, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromR8(dblin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<super::Com::CY>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI1(bin: u8) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI1(bin : u8, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromUI1(bin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI2(uiin: u16) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI2(uiin : u16, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromUI2(uiin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI4(ulin: u32) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI4(ulin : u32, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromUI4(ulin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyFromUI8(ui64in: u64) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyFromUI8(ui64in : u64, pcyout : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyFromUI8(ui64in, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyInt(cyin: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyInt(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyInt(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMul(cyleft: super::Com::CY, cyright: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyMul(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyMul(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMulI4(cyleft: super::Com::CY, lright: i32) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyMulI4(cyleft : super::Com:: CY, lright : i32, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyMulI4(core::mem::transmute(cyleft), lright, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyMulI8(cyleft: super::Com::CY, lright: i64) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyMulI8(cyleft : super::Com:: CY, lright : i64, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyMulI8(core::mem::transmute(cyleft), lright, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyNeg(cyin: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyNeg(cyin : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyNeg(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCyRound(cyin: super::Com::CY, cdecimals: i32) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCyRound(cyin : super::Com:: CY, cdecimals : i32, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCyRound(core::mem::transmute(cyin), cdecimals, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarCySub(cyleft: super::Com::CY, cyright: super::Com::CY) -> windows_core::Result<super::Com::CY> {
    windows_targets::link!("oleaut32.dll" "system" fn VarCySub(cyleft : super::Com:: CY, cyright : super::Com:: CY, pcyresult : *mut super::Com:: CY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarCySub(core::mem::transmute(cyleft), core::mem::transmute(cyright), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromBool<P0>(boolin: P0) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDateFromCy(cyin: super::Com::CY) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromCy(cyin : super::Com:: CY, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromDec(pdecin : *const super::super::Foundation:: DECIMAL, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDateFromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromI1(cin: i8) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI1(cin : i8, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromI2(sin: i16) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI2(sin : i16, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromI4(lin: i32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI4(lin : i32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromI8(i64in: i64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromI8(i64in : i64, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromR4(fltin: f32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromR4(fltin : f32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromR8(dblin: f64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromR8(dblin : f64, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUI1(bin: u8) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI1(bin : u8, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUI2(uiin: u16) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI2(uiin : u16, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUI4(ulin: u32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI4(ulin : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUI8(ui64in: u64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUI8(ui64in : u64, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUdate(pudatein: *const UDATE, dwflags: u32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUdate(pudatein : *const UDATE, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUdate(pudatein, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDateFromUdateEx(pudatein: *const UDATE, lcid: u32, dwflags: u32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDateFromUdateEx(pudatein : *const UDATE, lcid : u32, dwflags : u32, pdateout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDateFromUdateEx(pudatein, lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecAbs(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecAbs(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecAbs(pdecin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecAdd(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecAdd(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecAdd(pdecleft, pdecright, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecCmp(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecCmp(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL) -> VARCMP);
    VarDecCmp(pdecleft, pdecright)
}
#[inline]
pub unsafe fn VarDecCmpR8(pdecleft: *const super::super::Foundation::DECIMAL, dblright: f64) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecCmpR8(pdecleft : *const super::super::Foundation:: DECIMAL, dblright : f64) -> VARCMP);
    VarDecCmpR8(pdecleft, dblright)
}
#[inline]
pub unsafe fn VarDecDiv(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecDiv(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecDiv(pdecleft, pdecright, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFix(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFix(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFix(pdecin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromBool<P0>(boolin: P0) -> windows_core::Result<super::super::Foundation::DECIMAL>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDecFromCy(cyin: super::Com::CY) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromCy(cyin : super::Com:: CY, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromDate(datein: f64) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromDate(datein : f64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromDate(datein, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarDecFromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<super::super::Foundation::DECIMAL>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromI1(cin: i8) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI1(cin : i8, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromI2(uiin: i16) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI2(uiin : i16, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromI4(lin: i32) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI4(lin : i32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromI8(i64in: i64) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromI8(i64in : i64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromR4(fltin: f32) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromR4(fltin : f32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromR8(dblin: f64) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromR8(dblin : f64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<super::super::Foundation::DECIMAL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromUI1(bin: u8) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI1(bin : u8, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromUI2(uiin: u16) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI2(uiin : u16, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromUI4(ulin: u32) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI4(ulin : u32, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecFromUI8(ui64in: u64) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecFromUI8(ui64in : u64, pdecout : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecFromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecInt(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecInt(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecInt(pdecin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecMul(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecMul(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecMul(pdecleft, pdecright, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecNeg(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecNeg(pdecin : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecNeg(pdecin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecRound(pdecin: *const super::super::Foundation::DECIMAL, cdecimals: i32) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecRound(pdecin : *const super::super::Foundation:: DECIMAL, cdecimals : i32, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecRound(pdecin, cdecimals, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDecSub(pdecleft: *const super::super::Foundation::DECIMAL, pdecright: *const super::super::Foundation::DECIMAL) -> windows_core::Result<super::super::Foundation::DECIMAL> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDecSub(pdecleft : *const super::super::Foundation:: DECIMAL, pdecright : *const super::super::Foundation:: DECIMAL, pdecresult : *mut super::super::Foundation:: DECIMAL) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDecSub(pdecleft, pdecright, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarDiv(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarDiv(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarDiv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarEqv(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarEqv(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarEqv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFix(pvarin: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarFix(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFix(core::mem::transmute(pvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFormat<P0>(pvarin: *const windows_core::VARIANT, pstrformat: P0, ifirstday: VARFORMAT_FIRST_DAY, ifirstweek: VARFORMAT_FIRST_WEEK, dwflags: u32) -> windows_core::Result<windows_core::BSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarFormat(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pstrformat : windows_core::PCWSTR, ifirstday : VARFORMAT_FIRST_DAY, ifirstweek : VARFORMAT_FIRST_WEEK, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFormat(core::mem::transmute(pvarin), pstrformat.param().abi(), ifirstday, ifirstweek, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFormatCurrency(pvarin: *const windows_core::VARIANT, inumdig: i32, iinclead: i32, iuseparens: i32, igroup: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarFormatCurrency(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, inumdig : i32, iinclead : i32, iuseparens : i32, igroup : i32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFormatCurrency(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFormatDateTime(pvarin: *const windows_core::VARIANT, inamedformat: VARFORMAT_NAMED_FORMAT, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarFormatDateTime(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, inamedformat : VARFORMAT_NAMED_FORMAT, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFormatDateTime(core::mem::transmute(pvarin), inamedformat, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFormatFromTokens<P0>(pvarin: *const windows_core::VARIANT, pstrformat: P0, pbtokcur: *const u8, dwflags: u32, pbstrout: *mut windows_core::BSTR, lcid: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarFormatFromTokens(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pstrformat : windows_core::PCWSTR, pbtokcur : *const u8, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >, lcid : u32) -> windows_core::HRESULT);
    VarFormatFromTokens(core::mem::transmute(pvarin), pstrformat.param().abi(), pbtokcur, dwflags, core::mem::transmute(pbstrout), lcid).ok()
}
#[inline]
pub unsafe fn VarFormatNumber(pvarin: *const windows_core::VARIANT, inumdig: i32, iinclead: VARFORMAT_LEADING_DIGIT, iuseparens: VARFORMAT_PARENTHESES, igroup: VARFORMAT_GROUP, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarFormatNumber(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, inumdig : i32, iinclead : VARFORMAT_LEADING_DIGIT, iuseparens : VARFORMAT_PARENTHESES, igroup : VARFORMAT_GROUP, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFormatNumber(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarFormatPercent(pvarin: *const windows_core::VARIANT, inumdig: i32, iinclead: VARFORMAT_LEADING_DIGIT, iuseparens: VARFORMAT_PARENTHESES, igroup: VARFORMAT_GROUP, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarFormatPercent(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, inumdig : i32, iinclead : VARFORMAT_LEADING_DIGIT, iuseparens : VARFORMAT_PARENTHESES, igroup : VARFORMAT_GROUP, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarFormatPercent(core::mem::transmute(pvarin), inumdig, iinclead, iuseparens, igroup, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarI1FromBool<P0>(boolin: P0, pcout: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromBool(boolin.param().abi(), core::mem::transmute(pcout)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI1FromCy(cyin: super::Com::CY, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromCy(cyin : super::Com:: CY, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromCy(core::mem::transmute(cyin), core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromDate(datein: f64, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDate(datein : f64, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromDate(datein, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromDec(pdecin: *const super::super::Foundation::DECIMAL, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromDec(pdecin, core::mem::transmute(pcout)).ok()
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI1FromDisp<P0>(pdispin: P0, lcid: u32, pcout: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromDisp(pdispin.param().abi(), lcid, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromI2(uiin: i16, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI2(uiin : i16, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromI2(uiin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromI4(lin: i32, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI4(lin : i32, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromI4(lin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromI8(i64in: i64, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromI8(i64in : i64, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromI8(i64in, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromR4(fltin: f32, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromR4(fltin : f32, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromR4(fltin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromR8(dblin: f64, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromR8(dblin : f64, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromR8(dblin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromStr<P0>(strin: P0, lcid: u32, dwflags: u32, pcout: windows_core::PSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromStr(strin.param().abi(), lcid, dwflags, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromUI1(bin: u8, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI1(bin : u8, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromUI1(bin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromUI2(uiin: u16, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI2(uiin : u16, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromUI2(uiin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromUI4(ulin: u32, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI4(ulin : u32, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromUI4(ulin, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI1FromUI8(i64in: u64, pcout: windows_core::PSTR) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI1FromUI8(i64in : u64, pcout : windows_core::PSTR) -> windows_core::HRESULT);
    VarI1FromUI8(i64in, core::mem::transmute(pcout)).ok()
}
#[inline]
pub unsafe fn VarI2FromBool<P0>(boolin: P0) -> windows_core::Result<i16>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI2FromCy(cyin: super::Com::CY, psout: *mut i16) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromCy(cyin : super::Com:: CY, psout : *mut i16) -> windows_core::HRESULT);
    VarI2FromCy(core::mem::transmute(cyin), psout).ok()
}
#[inline]
pub unsafe fn VarI2FromDate(datein: f64) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDate(datein : f64, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDec(pdecin : *const super::super::Foundation:: DECIMAL, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI2FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<i16>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromI1(cin: i8) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI1(cin : i8, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromI4(lin: i32) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI4(lin : i32, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromI8(i64in: i64) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromI8(i64in : i64, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromR4(fltin: f32) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromR4(fltin : f32, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromR8(dblin: f64) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromR8(dblin : f64, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<i16>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromUI1(bin: u8) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI1(bin : u8, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromUI2(uiin: u16) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI2(uiin : u16, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromUI4(ulin: u32) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI4(ulin : u32, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI2FromUI8(ui64in: u64) -> windows_core::Result<i16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI2FromUI8(ui64in : u64, psout : *mut i16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI2FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromBool<P0>(boolin: P0) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI4FromCy(cyin: super::Com::CY) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromCy(cyin : super::Com:: CY, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromDate(datein: f64) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDate(datein : f64, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI4FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<i32>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromI1(cin: i8) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI1(cin : i8, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromI2(sin: i16) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI2(sin : i16, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromI8(i64in: i64) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromI8(i64in : i64, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromR4(fltin: f32) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromR4(fltin : f32, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromR8(dblin: f64) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromR8(dblin : f64, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<i32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromUI1(bin: u8) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI1(bin : u8, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromUI2(uiin: u16) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI2(uiin : u16, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromUI4(ulin: u32) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI4(ulin : u32, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI4FromUI8(ui64in: u64) -> windows_core::Result<i32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI4FromUI8(ui64in : u64, plout : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI4FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromBool<P0>(boolin: P0) -> windows_core::Result<i64>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI8FromCy(cyin: super::Com::CY) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromCy(cyin : super::Com:: CY, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromDate(datein: f64) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDate(datein : f64, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarI8FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<i64>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromI1(cin: i8) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromI1(cin : i8, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromI2(sin: i16) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromI2(sin : i16, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromR4(fltin: f32) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromR4(fltin : f32, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromR8(dblin: f64) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromR8(dblin : f64, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<i64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromUI1(bin: u8) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI1(bin : u8, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromUI2(uiin: u16) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI2(uiin : u16, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromUI4(ulin: u32) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI4(ulin : u32, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarI8FromUI8(ui64in: u64) -> windows_core::Result<i64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarI8FromUI8(ui64in : u64, pi64out : *mut i64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarI8FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarIdiv(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarIdiv(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarIdiv(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarImp(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarImp(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarImp(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarInt(pvarin: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarInt(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarInt(core::mem::transmute(pvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarMod(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarMod(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarMod(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarMonthName(imonth: i32, fabbrev: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarMonthName(imonth : i32, fabbrev : i32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarMonthName(imonth, fabbrev, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarMul(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarMul(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarMul(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarNeg(pvarin: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarNeg(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarNeg(core::mem::transmute(pvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarNot(pvarin: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarNot(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarNot(core::mem::transmute(pvarin), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarNumFromParseNum(pnumprs: *const NUMPARSE, rgbdig: *const u8, dwvtbits: u32) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarNumFromParseNum(pnumprs : *const NUMPARSE, rgbdig : *const u8, dwvtbits : u32, pvar : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarNumFromParseNum(pnumprs, rgbdig, dwvtbits, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarOr(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarOr(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarOr(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarParseNumFromStr<P0>(strin: P0, lcid: u32, dwflags: u32, pnumprs: *mut NUMPARSE, rgbdig: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarParseNumFromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pnumprs : *mut NUMPARSE, rgbdig : *mut u8) -> windows_core::HRESULT);
    VarParseNumFromStr(strin.param().abi(), lcid, dwflags, pnumprs, rgbdig).ok()
}
#[inline]
pub unsafe fn VarPow(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarPow(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarPow(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarR4CmpR8(fltleft: f32, dblright: f64) -> VARCMP {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4CmpR8(fltleft : f32, dblright : f64) -> VARCMP);
    VarR4CmpR8(fltleft, dblright)
}
#[inline]
pub unsafe fn VarR4FromBool<P0>(boolin: P0) -> windows_core::Result<f32>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR4FromCy(cyin: super::Com::CY, pfltout: *mut f32) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromCy(cyin : super::Com:: CY, pfltout : *mut f32) -> windows_core::HRESULT);
    VarR4FromCy(core::mem::transmute(cyin), pfltout).ok()
}
#[inline]
pub unsafe fn VarR4FromDate(datein: f64) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDate(datein : f64, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR4FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<f32>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromI1(cin: i8) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI1(cin : i8, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromI2(sin: i16) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI2(sin : i16, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromI4(lin: i32) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI4(lin : i32, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromI8(i64in: i64) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromI8(i64in : i64, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromR8(dblin: f64) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromR8(dblin : f64, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<f32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromUI1(bin: u8) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI1(bin : u8, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromUI2(uiin: u16) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI2(uiin : u16, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromUI4(ulin: u32) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI4(ulin : u32, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR4FromUI8(ui64in: u64) -> windows_core::Result<f32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR4FromUI8(ui64in : u64, pfltout : *mut f32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR4FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromBool<P0>(boolin: P0) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR8FromCy(cyin: super::Com::CY, pdblout: *mut f64) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromCy(cyin : super::Com:: CY, pdblout : *mut f64) -> windows_core::HRESULT);
    VarR8FromCy(core::mem::transmute(cyin), pdblout).ok()
}
#[inline]
pub unsafe fn VarR8FromDate(datein: f64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDate(datein : f64, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarR8FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromI1(cin: i8, pdblout: *mut f64) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI1(cin : i8, pdblout : *mut f64) -> windows_core::HRESULT);
    VarR8FromI1(cin, pdblout).ok()
}
#[inline]
pub unsafe fn VarR8FromI2(sin: i16) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI2(sin : i16, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromI4(lin: i32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI4(lin : i32, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromI8(i64in: i64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromI8(i64in : i64, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromR4(fltin: f32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromR4(fltin : f32, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<f64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromUI1(bin: u8) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI1(bin : u8, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromUI2(uiin: u16) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI2(uiin : u16, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromUI4(ulin: u32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI4(ulin : u32, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8FromUI8(ui64in: u64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8FromUI8(ui64in : u64, pdblout : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8Pow(dblleft: f64, dblright: f64) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8Pow(dblleft : f64, dblright : f64, pdblresult : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8Pow(dblleft, dblright, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarR8Round(dblin: f64, cdecimals: i32) -> windows_core::Result<f64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarR8Round(dblin : f64, cdecimals : i32, pdblresult : *mut f64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarR8Round(dblin, cdecimals, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarRound(pvarin: *const windows_core::VARIANT, cdecimals: i32) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarRound(pvarin : *const core::mem::MaybeUninit < windows_core::VARIANT >, cdecimals : i32, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarRound(core::mem::transmute(pvarin), cdecimals, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarSub(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarSub(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarSub(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarTokenizeFormatString<P0>(pstrformat: P0, rgbtok: &mut [u8], ifirstday: VARFORMAT_FIRST_DAY, ifirstweek: VARFORMAT_FIRST_WEEK, lcid: u32, pcbactual: Option<*const i32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarTokenizeFormatString(pstrformat : windows_core::PCWSTR, rgbtok : *mut u8, cbtok : i32, ifirstday : VARFORMAT_FIRST_DAY, ifirstweek : VARFORMAT_FIRST_WEEK, lcid : u32, pcbactual : *const i32) -> windows_core::HRESULT);
    VarTokenizeFormatString(pstrformat.param().abi(), core::mem::transmute(rgbtok.as_ptr()), rgbtok.len().try_into().unwrap(), ifirstday, ifirstweek, lcid, core::mem::transmute(pcbactual.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn VarUI1FromBool<P0>(boolin: P0) -> windows_core::Result<u8>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI1FromCy(cyin: super::Com::CY) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromCy(cyin : super::Com:: CY, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromDate(datein: f64) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDate(datein : f64, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI1FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<u8>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromI1(cin: i8) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI1(cin : i8, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromI2(sin: i16) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI2(sin : i16, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromI4(lin: i32) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI4(lin : i32, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromI8(i64in: i64) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromI8(i64in : i64, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromR4(fltin: f32) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromR4(fltin : f32, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromR8(dblin: f64) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromR8(dblin : f64, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<u8>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromUI2(uiin: u16) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI2(uiin : u16, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromUI4(ulin: u32) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI4(ulin : u32, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI1FromUI8(ui64in: u64) -> windows_core::Result<u8> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI1FromUI8(ui64in : u64, pbout : *mut u8) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI1FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromBool<P0>(boolin: P0) -> windows_core::Result<u16>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI2FromCy(cyin: super::Com::CY) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromCy(cyin : super::Com:: CY, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromDate(datein: f64) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDate(datein : f64, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDec(pdecin : *const super::super::Foundation:: DECIMAL, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI2FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<u16>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromI1(cin: i8) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI1(cin : i8, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromI2(uiin: i16) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI2(uiin : i16, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromI4(lin: i32) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI4(lin : i32, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromI8(i64in: i64) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromI8(i64in : i64, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromR4(fltin: f32) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromR4(fltin : f32, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromR8(dblin: f64, puiout: *mut u16) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromR8(dblin : f64, puiout : *mut u16) -> windows_core::HRESULT);
    VarUI2FromR8(dblin, puiout).ok()
}
#[inline]
pub unsafe fn VarUI2FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<u16>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromUI1(bin: u8) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI1(bin : u8, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromUI4(ulin: u32) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI4(ulin : u32, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI2FromUI8(i64in: u64) -> windows_core::Result<u16> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI2FromUI8(i64in : u64, puiout : *mut u16) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI2FromUI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromBool<P0>(boolin: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI4FromCy(cyin: super::Com::CY) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromCy(cyin : super::Com:: CY, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromDate(datein: f64) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDate(datein : f64, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI4FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromI1(cin: i8) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI1(cin : i8, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromI2(uiin: i16) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI2(uiin : i16, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromI4(lin: i32) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI4(lin : i32, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromI4(lin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromI8(i64in: i64) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromI8(i64in : i64, plout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromI8(i64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromR4(fltin: f32) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromR4(fltin : f32, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromR8(dblin: f64) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromR8(dblin : f64, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromUI1(bin: u8) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI1(bin : u8, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromUI2(uiin: u16) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI2(uiin : u16, pulout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI4FromUI8(ui64in: u64) -> windows_core::Result<u32> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI4FromUI8(ui64in : u64, plout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI4FromUI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromBool<P0>(boolin: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromBool(boolin : super::super::Foundation:: VARIANT_BOOL, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromBool(boolin.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI8FromCy(cyin: super::Com::CY) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromCy(cyin : super::Com:: CY, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromCy(core::mem::transmute(cyin), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromDate(datein: f64) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDate(datein : f64, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromDate(datein, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromDec(pdecin: *const super::super::Foundation::DECIMAL) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDec(pdecin : *const super::super::Foundation:: DECIMAL, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromDec(pdecin, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VarUI8FromDisp<P0>(pdispin: P0, lcid: u32) -> windows_core::Result<u64>
where
    P0: windows_core::Param<super::Com::IDispatch>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromDisp(pdispin : * mut core::ffi::c_void, lcid : u32, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromDisp(pdispin.param().abi(), lcid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromI1(cin: i8) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI1(cin : i8, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromI1(cin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromI2(sin: i16) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI2(sin : i16, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromI2(sin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromI8(ui64in: i64) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromI8(ui64in : i64, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromI8(ui64in, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromR4(fltin: f32) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromR4(fltin : f32, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromR4(fltin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromR8(dblin: f64) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromR8(dblin : f64, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromR8(dblin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromStr<P0>(strin: P0, lcid: u32, dwflags: u32) -> windows_core::Result<u64>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromStr(strin : windows_core::PCWSTR, lcid : u32, dwflags : u32, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromStr(strin.param().abi(), lcid, dwflags, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromUI1(bin: u8) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI1(bin : u8, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromUI1(bin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromUI2(uiin: u16) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI2(uiin : u16, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromUI2(uiin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUI8FromUI4(ulin: u32) -> windows_core::Result<u64> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUI8FromUI4(ulin : u32, pi64out : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarUI8FromUI4(ulin, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn VarUdateFromDate(datein: f64, dwflags: u32, pudateout: *mut UDATE) -> windows_core::Result<()> {
    windows_targets::link!("oleaut32.dll" "system" fn VarUdateFromDate(datein : f64, dwflags : u32, pudateout : *mut UDATE) -> windows_core::HRESULT);
    VarUdateFromDate(datein, dwflags, pudateout).ok()
}
#[inline]
pub unsafe fn VarWeekdayName(iweekday: i32, fabbrev: i32, ifirstday: i32, dwflags: u32) -> windows_core::Result<windows_core::BSTR> {
    windows_targets::link!("oleaut32.dll" "system" fn VarWeekdayName(iweekday : i32, fabbrev : i32, ifirstday : i32, dwflags : u32, pbstrout : *mut core::mem::MaybeUninit < windows_core::BSTR >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarWeekdayName(iweekday, fabbrev, ifirstday, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn VarXor(pvarleft: *const windows_core::VARIANT, pvarright: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT> {
    windows_targets::link!("oleaut32.dll" "system" fn VarXor(pvarleft : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarright : *const core::mem::MaybeUninit < windows_core::VARIANT >, pvarresult : *mut core::mem::MaybeUninit < windows_core::VARIANT >) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VarXor(core::mem::transmute(pvarleft), core::mem::transmute(pvarright), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn VectorFromBstr<P0>(bstr: P0) -> windows_core::Result<*mut super::Com::SAFEARRAY>
where
    P0: windows_core::Param<windows_core::BSTR>,
{
    windows_targets::link!("oleaut32.dll" "system" fn VectorFromBstr(bstr : core::mem::MaybeUninit < windows_core::BSTR >, ppsa : *mut *mut super::Com:: SAFEARRAY) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    VectorFromBstr(bstr.param().abi(), &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAdviseSinkEx, IAdviseSinkEx_Vtbl, 0x3af24290_0c96_11ce_a0cf_00aa00600ab8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAdviseSinkEx {
    type Target = super::Com::IAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAdviseSinkEx, windows_core::IUnknown, super::Com::IAdviseSink);
#[cfg(feature = "Win32_System_Com")]
impl IAdviseSinkEx {
    pub unsafe fn OnViewStatusChange(&self, dwviewstatus: u32) {
        (windows_core::Interface::vtable(self).OnViewStatusChange)(windows_core::Interface::as_raw(self), dwviewstatus)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAdviseSinkEx_Vtbl {
    pub base__: super::Com::IAdviseSink_Vtbl,
    pub OnViewStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
windows_core::imp::define_interface!(ICanHandleException, ICanHandleException_Vtbl, 0xc5598e60_b307_11d1_b27d_006008c3fbfb);
impl core::ops::Deref for ICanHandleException {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICanHandleException, windows_core::IUnknown);
impl ICanHandleException {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CanHandleException(&self, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanHandleException)(windows_core::Interface::as_raw(self), pexcepinfo, core::mem::transmute(pvar)).ok()
    }
}
#[repr(C)]
pub struct ICanHandleException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CanHandleException: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::EXCEPINFO, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CanHandleException: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IClassFactory2, IClassFactory2_Vtbl, 0xb196b28f_bab4_101a_b69c_00aa00341d07);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IClassFactory2 {
    type Target = super::Com::IClassFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IClassFactory2, windows_core::IUnknown, super::Com::IClassFactory);
#[cfg(feature = "Win32_System_Com")]
impl IClassFactory2 {
    pub unsafe fn GetLicInfo(&self, plicinfo: *mut LICINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLicInfo)(windows_core::Interface::as_raw(self), plicinfo).ok()
    }
    pub unsafe fn RequestLicKey(&self, dwreserved: u32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestLicKey)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateInstanceLic<P0, P1, P2, T>(&self, punkouter: P0, punkreserved: P1, bstrkey: P2) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::BSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstanceLic)(windows_core::Interface::as_raw(self), punkouter.param().abi(), punkreserved.param().abi(), &T::IID, bstrkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IClassFactory2_Vtbl {
    pub base__: super::Com::IClassFactory_Vtbl,
    pub GetLicInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LICINFO) -> windows_core::HRESULT,
    pub RequestLicKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CreateInstanceLic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContinue, IContinue_Vtbl, 0x0000012a_0000_0000_c000_000000000046);
impl core::ops::Deref for IContinue {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContinue, windows_core::IUnknown);
impl IContinue {
    pub unsafe fn FContinue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FContinue)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IContinue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContinueCallback, IContinueCallback_Vtbl, 0xb722bcca_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IContinueCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContinueCallback, windows_core::IUnknown);
impl IContinueCallback {
    pub unsafe fn FContinue(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FContinue)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FContinuePrinting<P0>(&self, ncntprinted: i32, ncurpage: i32, pwszprintstatus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).FContinuePrinting)(windows_core::Interface::as_raw(self), ncntprinted, ncurpage, pwszprintstatus.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IContinueCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FContinue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FContinuePrinting: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateErrorInfo, ICreateErrorInfo_Vtbl, 0x22f03340_547d_101b_8e65_08002b2bd119);
impl core::ops::Deref for ICreateErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateErrorInfo, windows_core::IUnknown);
impl ICreateErrorInfo {
    pub unsafe fn SetGUID(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), rguid).ok()
    }
    pub unsafe fn SetSource<P0>(&self, szsource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), szsource.param().abi()).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, szdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), szdescription.param().abi()).ok()
    }
    pub unsafe fn SetHelpFile<P0>(&self, szhelpfile: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHelpFile)(windows_core::Interface::as_raw(self), szhelpfile.param().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext).ok()
    }
}
#[repr(C)]
pub struct ICreateErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateTypeInfo, ICreateTypeInfo_Vtbl, 0x00020405_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeInfo, windows_core::IUnknown);
impl ICreateTypeInfo {
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid).ok()
    }
    pub unsafe fn SetTypeFlags(&self, utypeflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTypeFlags)(windows_core::Interface::as_raw(self), utypeflags).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, pstrdoc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), pstrdoc.param().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddRefTypeInfo<P0>(&self, ptinfo: P0, phreftype: *const u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::ITypeInfo>,
    {
        (windows_core::Interface::vtable(self).AddRefTypeInfo)(windows_core::Interface::as_raw(self), ptinfo.param().abi(), phreftype).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn AddFuncDesc(&self, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddFuncDesc)(windows_core::Interface::as_raw(self), index, pfuncdesc).ok()
    }
    pub unsafe fn AddImplType(&self, index: u32, hreftype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddImplType)(windows_core::Interface::as_raw(self), index, hreftype).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetImplTypeFlags(&self, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetImplTypeFlags)(windows_core::Interface::as_raw(self), index, impltypeflags).ok()
    }
    pub unsafe fn SetAlignment(&self, cbalignment: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetAlignment)(windows_core::Interface::as_raw(self), cbalignment).ok()
    }
    pub unsafe fn SetSchema<P0>(&self, pstrschema: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSchema)(windows_core::Interface::as_raw(self), pstrschema.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn AddVarDesc(&self, index: u32, pvardesc: *const super::Com::VARDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddVarDesc)(windows_core::Interface::as_raw(self), index, pvardesc).ok()
    }
    pub unsafe fn SetFuncAndParamNames(&self, index: u32, rgsznames: &[windows_core::PCWSTR]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFuncAndParamNames)(windows_core::Interface::as_raw(self), index, core::mem::transmute(rgsznames.as_ptr()), rgsznames.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetVarName<P0>(&self, index: u32, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetVarName)(windows_core::Interface::as_raw(self), index, szname.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn SetTypeDescAlias(&self, ptdescalias: *const super::Com::TYPEDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTypeDescAlias)(windows_core::Interface::as_raw(self), ptdescalias).ok()
    }
    pub unsafe fn DefineFuncAsDllEntry<P0, P1>(&self, index: u32, szdllname: P0, szprocname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DefineFuncAsDllEntry)(windows_core::Interface::as_raw(self), index, szdllname.param().abi(), szprocname.param().abi()).ok()
    }
    pub unsafe fn SetFuncDocString<P0>(&self, index: u32, szdocstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFuncDocString)(windows_core::Interface::as_raw(self), index, szdocstring.param().abi()).ok()
    }
    pub unsafe fn SetVarDocString<P0>(&self, index: u32, szdocstring: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetVarDocString)(windows_core::Interface::as_raw(self), index, szdocstring.param().abi()).ok()
    }
    pub unsafe fn SetFuncHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFuncHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetVarHelpContext(&self, index: u32, dwhelpcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVarHelpContext)(windows_core::Interface::as_raw(self), index, dwhelpcontext).ok()
    }
    pub unsafe fn SetMops<P0>(&self, index: u32, bstrmops: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMops)(windows_core::Interface::as_raw(self), index, bstrmops.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTypeIdldesc(&self, pidldesc: *const super::Com::IDLDESC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetTypeIdldesc)(windows_core::Interface::as_raw(self), pidldesc).ok()
    }
    pub unsafe fn LayOut(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LayOut)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICreateTypeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddRefTypeInfo: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub AddFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Com::FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    AddFuncDesc: usize,
    pub AddImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::Com::IMPLTYPEFLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetImplTypeFlags: usize,
    pub SetAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub SetSchema: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub AddVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::Com::VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    AddVarDesc: usize,
    pub SetFuncAndParamNames: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetVarName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub SetTypeDescAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::TYPEDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    SetTypeDescAlias: usize,
    pub DefineFuncAsDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFuncDocString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetVarDocString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFuncHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetVarHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetMops: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTypeIdldesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::IDLDESC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTypeIdldesc: usize,
    pub LayOut: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateTypeInfo2, ICreateTypeInfo2_Vtbl, 0x0002040e_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeInfo2 {
    type Target = ICreateTypeInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeInfo2, windows_core::IUnknown, ICreateTypeInfo);
impl ICreateTypeInfo2 {
    pub unsafe fn DeleteFuncDesc(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteFuncDesc)(windows_core::Interface::as_raw(self), index).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteFuncDescByMemId(&self, memid: i32, invkind: super::Com::INVOKEKIND) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteFuncDescByMemId)(windows_core::Interface::as_raw(self), memid, invkind).ok()
    }
    pub unsafe fn DeleteVarDesc(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteVarDesc)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn DeleteVarDescByMemId(&self, memid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteVarDescByMemId)(windows_core::Interface::as_raw(self), memid).ok()
    }
    pub unsafe fn DeleteImplType(&self, index: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteImplType)(windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetFuncCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetVarCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVarCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext).ok()
    }
    pub unsafe fn SetFuncHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetFuncHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext).ok()
    }
    pub unsafe fn SetVarHelpStringContext(&self, index: u32, dwhelpstringcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVarHelpStringContext)(windows_core::Interface::as_raw(self), index, dwhelpstringcontext).ok()
    }
    pub unsafe fn Invalidate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICreateTypeInfo2_Vtbl {
    pub base__: ICreateTypeInfo_Vtbl,
    pub DeleteFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DeleteFuncDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::Com::INVOKEKIND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DeleteFuncDescByMemId: usize,
    pub DeleteVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteVarDescByMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DeleteImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetFuncHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SetVarHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateTypeLib, ICreateTypeLib_Vtbl, 0x00020406_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeLib {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeLib, windows_core::IUnknown);
impl ICreateTypeLib {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTypeInfo<P0>(&self, szname: P0, tkind: super::Com::TYPEKIND) -> windows_core::Result<ICreateTypeInfo>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateTypeInfo)(windows_core::Interface::as_raw(self), szname.param().abi(), tkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), szname.param().abi()).ok()
    }
    pub unsafe fn SetVersion(&self, wmajorvernum: u16, wminorvernum: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), wmajorvernum, wminorvernum).ok()
    }
    pub unsafe fn SetGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid).ok()
    }
    pub unsafe fn SetDocString<P0>(&self, szdoc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDocString)(windows_core::Interface::as_raw(self), szdoc.param().abi()).ok()
    }
    pub unsafe fn SetHelpFileName<P0>(&self, szhelpfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHelpFileName)(windows_core::Interface::as_raw(self), szhelpfilename.param().abi()).ok()
    }
    pub unsafe fn SetHelpContext(&self, dwhelpcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpContext)(windows_core::Interface::as_raw(self), dwhelpcontext).ok()
    }
    pub unsafe fn SetLcid(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLcid)(windows_core::Interface::as_raw(self), lcid).ok()
    }
    pub unsafe fn SetLibFlags(&self, ulibflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLibFlags)(windows_core::Interface::as_raw(self), ulibflags).ok()
    }
    pub unsafe fn SaveAllChanges(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveAllChanges)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICreateTypeLib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::Com::TYPEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTypeInfo: usize,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetDocString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpFileName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetLibFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveAllChanges: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICreateTypeLib2, ICreateTypeLib2_Vtbl, 0x0002040f_0000_0000_c000_000000000046);
impl core::ops::Deref for ICreateTypeLib2 {
    type Target = ICreateTypeLib;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICreateTypeLib2, windows_core::IUnknown, ICreateTypeLib);
impl ICreateTypeLib2 {
    pub unsafe fn DeleteTypeInfo<P0>(&self, szname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteTypeInfo)(windows_core::Interface::as_raw(self), szname.param().abi()).ok()
    }
    pub unsafe fn SetCustData(&self, guid: *const windows_core::GUID, pvarval: *const windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCustData)(windows_core::Interface::as_raw(self), guid, core::mem::transmute(pvarval)).ok()
    }
    pub unsafe fn SetHelpStringContext(&self, dwhelpstringcontext: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetHelpStringContext)(windows_core::Interface::as_raw(self), dwhelpstringcontext).ok()
    }
    pub unsafe fn SetHelpStringDll<P0>(&self, szfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHelpStringDll)(windows_core::Interface::as_raw(self), szfilename.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ICreateTypeLib2_Vtbl {
    pub base__: ICreateTypeLib_Vtbl,
    pub DeleteTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub SetHelpStringContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetHelpStringDll: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDispError, IDispError_Vtbl, 0xa6ef9861_c720_11d0_9337_00a0c90dcaa9);
impl core::ops::Deref for IDispError {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDispError, windows_core::IUnknown);
impl IDispError {
    pub unsafe fn QueryErrorInfo(&self, guiderrortype: windows_core::GUID) -> windows_core::Result<IDispError> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(guiderrortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNext(&self) -> windows_core::Result<IDispError> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHresult(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHresult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrfilename), pdwcontext).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDispError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHresult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDispatchEx, IDispatchEx_Vtbl, 0xa6ef9860_c720_11d0_9337_00a0c90dcaa9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDispatchEx {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDispatchEx, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDispatchEx {
    pub unsafe fn GetDispID<P0>(&self, bstrname: P0, grfdex: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDispID)(windows_core::Interface::as_raw(self), bstrname.param().abi(), grfdex, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InvokeEx<P0>(&self, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: Option<*mut windows_core::VARIANT>, pei: Option<*mut super::Com::EXCEPINFO>, pspcaller: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IServiceProvider>,
    {
        (windows_core::Interface::vtable(self).InvokeEx)(windows_core::Interface::as_raw(self), id, lcid, wflags, pdp, core::mem::transmute(pvarres.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pei.unwrap_or(std::ptr::null_mut())), pspcaller.param().abi()).ok()
    }
    pub unsafe fn DeleteMemberByName<P0>(&self, bstrname: P0, grfdex: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).DeleteMemberByName)(windows_core::Interface::as_raw(self), bstrname.param().abi(), grfdex).ok()
    }
    pub unsafe fn DeleteMemberByDispID(&self, id: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeleteMemberByDispID)(windows_core::Interface::as_raw(self), id).ok()
    }
    pub unsafe fn GetMemberProperties(&self, id: i32, grfdexfetch: u32) -> windows_core::Result<FDEX_PROP_FLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMemberProperties)(windows_core::Interface::as_raw(self), id, grfdexfetch, &mut result__).map(|| result__)
    }
    pub unsafe fn GetMemberName(&self, id: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNextDispID(&self, grfdex: u32, id: i32) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNextDispID)(windows_core::Interface::as_raw(self), grfdex, id, &mut result__).map(|| result__)
    }
    pub unsafe fn GetNameSpaceParent(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNameSpaceParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDispatchEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetDispID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InvokeEx: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, u16, *const super::Com::DISPPARAMS, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut super::Com::EXCEPINFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InvokeEx: usize,
    pub DeleteMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub DeleteMemberByDispID: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetMemberProperties: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut FDEX_PROP_FLAGS) -> windows_core::HRESULT,
    pub GetMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetNextDispID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32, *mut i32) -> windows_core::HRESULT,
    pub GetNameSpaceParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDropSource, IDropSource_Vtbl, 0x00000121_0000_0000_c000_000000000046);
impl core::ops::Deref for IDropSource {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDropSource, windows_core::IUnknown);
impl IDropSource {
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn QueryContinueDrag<P0>(&self, fescapepressed: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).QueryContinueDrag)(windows_core::Interface::as_raw(self), fescapepressed.param().abi(), grfkeystate)
    }
    pub unsafe fn GiveFeedback(&self, dweffect: DROPEFFECT) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).GiveFeedback)(windows_core::Interface::as_raw(self), dweffect)
    }
}
#[repr(C)]
pub struct IDropSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub QueryContinueDrag: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::SystemServices::MODIFIERKEYS_FLAGS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))]
    QueryContinueDrag: usize,
    pub GiveFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, DROPEFFECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDropSourceNotify, IDropSourceNotify_Vtbl, 0x0000012b_0000_0000_c000_000000000046);
impl core::ops::Deref for IDropSourceNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDropSourceNotify, windows_core::IUnknown);
impl IDropSourceNotify {
    pub unsafe fn DragEnterTarget<P0>(&self, hwndtarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DragEnterTarget)(windows_core::Interface::as_raw(self), hwndtarget.param().abi()).ok()
    }
    pub unsafe fn DragLeaveTarget(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DragLeaveTarget)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IDropSourceNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DragEnterTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub DragLeaveTarget: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDropTarget, IDropTarget_Vtbl, 0x00000122_0000_0000_c000_000000000046);
impl core::ops::Deref for IDropTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDropTarget, windows_core::IUnknown);
impl IDropTarget {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn DragEnter<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).DragEnter)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, core::mem::transmute(pt), pdweffect).ok()
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DragOver)(windows_core::Interface::as_raw(self), grfkeystate, core::mem::transmute(pt), pdweffect).ok()
    }
    pub unsafe fn DragLeave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DragLeave)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Drop<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).Drop)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, core::mem::transmute(pt), pdweffect).ok()
    }
}
#[repr(C)]
pub struct IDropTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub DragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    DragEnter: usize,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub DragOver: unsafe extern "system" fn(*mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))]
    DragOver: usize,
    pub DragLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub Drop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    Drop: usize,
}
windows_core::imp::define_interface!(IEnterpriseDropTarget, IEnterpriseDropTarget_Vtbl, 0x390e3878_fd55_4e18_819d_4682081c0cfd);
impl core::ops::Deref for IEnterpriseDropTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnterpriseDropTarget, windows_core::IUnknown);
impl IEnterpriseDropTarget {
    pub unsafe fn SetDropSourceEnterpriseId<P0>(&self, identity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetDropSourceEnterpriseId)(windows_core::Interface::as_raw(self), identity.param().abi()).ok()
    }
    pub unsafe fn IsEvaluatingEdpPolicy(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEvaluatingEdpPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnterpriseDropTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDropSourceEnterpriseId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub IsEvaluatingEdpPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumOLEVERB, IEnumOLEVERB_Vtbl, 0x00000104_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumOLEVERB {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumOLEVERB, windows_core::IUnknown);
impl IEnumOLEVERB {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn Next(&self, rgelt: &mut [OLEVERB], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOLEVERB> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumOLEVERB_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut OLEVERB, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumOleDocumentViews, IEnumOleDocumentViews_Vtbl, 0xb722bcc8_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IEnumOleDocumentViews {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumOleDocumentViews, windows_core::IUnknown);
impl IEnumOleDocumentViews {
    pub unsafe fn Next(&self, cviews: u32, rgpview: *mut Option<IOleDocumentView>, pcfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), cviews, core::mem::transmute(rgpview), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, cviews: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cviews).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOleDocumentViews> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumOleDocumentViews_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumOleUndoUnits, IEnumOleUndoUnits_Vtbl, 0xb3e7c340_ef97_11ce_9bc9_00aa00608e01);
impl core::ops::Deref for IEnumOleUndoUnits {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumOleUndoUnits, windows_core::IUnknown);
impl IEnumOleUndoUnits {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IOleUndoUnit>], pceltfetched: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched)
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumOleUndoUnits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumVARIANT, IEnumVARIANT_Vtbl, 0x00020404_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumVARIANT {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumVARIANT, windows_core::IUnknown);
impl IEnumVARIANT {
    pub unsafe fn Next(&self, rgvar: &mut [windows_core::VARIANT], pceltfetched: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgvar.len().try_into().unwrap(), core::mem::transmute(rgvar.as_ptr()), pceltfetched)
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumVARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumVARIANT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFont, IFont_Vtbl, 0xbef6e002_a874_101a_8bba_00aa00300cab);
impl core::ops::Deref for IFont {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFont, windows_core::IUnknown);
impl IFont {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), name.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Size(&self) -> windows_core::Result<super::Com::CY> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, size: super::Com::CY) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), core::mem::transmute(size)).ok()
    }
    pub unsafe fn Bold(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Bold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBold<P0>(&self, bold: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBold)(windows_core::Interface::as_raw(self), bold.param().abi()).ok()
    }
    pub unsafe fn Italic(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Italic)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetItalic<P0>(&self, italic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetItalic)(windows_core::Interface::as_raw(self), italic.param().abi()).ok()
    }
    pub unsafe fn Underline(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Underline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUnderline<P0>(&self, underline: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetUnderline)(windows_core::Interface::as_raw(self), underline.param().abi()).ok()
    }
    pub unsafe fn Strikethrough(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Strikethrough)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetStrikethrough<P0>(&self, strikethrough: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetStrikethrough)(windows_core::Interface::as_raw(self), strikethrough.param().abi()).ok()
    }
    pub unsafe fn Weight(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Weight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetWeight(&self, weight: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetWeight)(windows_core::Interface::as_raw(self), weight).ok()
    }
    pub unsafe fn Charset(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Charset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCharset(&self, charset: i16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCharset)(windows_core::Interface::as_raw(self), charset).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn hFont(&self) -> windows_core::Result<super::super::Graphics::Gdi::HFONT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).hFont)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IFont> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsEqual<P0>(&self, pfontother: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IFont>,
    {
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pfontother.param().abi()).ok()
    }
    pub unsafe fn SetRatio(&self, cylogical: i32, cyhimetric: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRatio)(windows_core::Interface::as_raw(self), cylogical, cyhimetric).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn QueryTextMetrics(&self, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryTextMetrics)(windows_core::Interface::as_raw(self), ptm).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddRefHfont<P0>(&self, hfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).AddRefHfont)(windows_core::Interface::as_raw(self), hfont.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseHfont<P0>(&self, hfont: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HFONT>,
    {
        (windows_core::Interface::vtable(self).ReleaseHfont)(windows_core::Interface::as_raw(self), hfont.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetHdc<P0>(&self, hdc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).SetHdc)(windows_core::Interface::as_raw(self), hdc.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFont_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Com::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Size: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::CY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSize: usize,
    pub Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetBold: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Italic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Underline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub Charset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub SetCharset: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub hFont: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    hFont: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRatio: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub QueryTextMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::TEXTMETRICW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    QueryTextMetrics: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddRefHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddRefHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseHfont: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HFONT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseHfont: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetHdc: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetHdc: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFontDisp, IFontDisp_Vtbl, 0xbef6e003_a874_101a_8bba_00aa00300cab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFontDisp {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFontDisp, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFontDisp {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFontDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IFontEventsDisp, IFontEventsDisp_Vtbl, 0x4ef6100a_af88_11d0_9846_00c04fc29993);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IFontEventsDisp {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IFontEventsDisp, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IFontEventsDisp {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IFontEventsDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IGetOleObject, IGetOleObject_Vtbl, 0x8a701da0_4feb_101b_a82e_08002b2b2337);
impl core::ops::Deref for IGetOleObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetOleObject, windows_core::IUnknown);
impl IGetOleObject {
    pub unsafe fn GetOleObject(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetOleObject)(windows_core::Interface::as_raw(self), riid, ppvobj).ok()
    }
}
#[repr(C)]
pub struct IGetOleObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOleObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetVBAObject, IGetVBAObject_Vtbl, 0x91733a60_3f4c_101b_a3f6_00aa0034e4e9);
impl core::ops::Deref for IGetVBAObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGetVBAObject, windows_core::IUnknown);
impl IGetVBAObject {
    pub unsafe fn GetObject(&self, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), riid, ppvobj, dwreserved).ok()
    }
}
#[repr(C)]
pub struct IGetVBAObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectIdentity, IObjectIdentity_Vtbl, 0xca04b7e6_0d21_11d1_8cc5_00c04fc2b085);
impl core::ops::Deref for IObjectIdentity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectIdentity, windows_core::IUnknown);
impl IObjectIdentity {
    pub unsafe fn IsEqualObject<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).IsEqualObject)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IObjectIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEqualObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IObjectWithSite, IObjectWithSite_Vtbl, 0xfc4801a3_2ba9_11cf_a229_00aa003d7352);
impl core::ops::Deref for IObjectWithSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IObjectWithSite, windows_core::IUnknown);
impl IObjectWithSite {
    pub unsafe fn SetSite<P0>(&self, punksite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetSite)(windows_core::Interface::as_raw(self), punksite.param().abi()).ok()
    }
    pub unsafe fn GetSite<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetSite)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IObjectWithSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSite: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleAdviseHolder, IOleAdviseHolder_Vtbl, 0x00000111_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleAdviseHolder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleAdviseHolder, windows_core::IUnknown);
impl IOleAdviseHolder {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, padvise: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::Com::IAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), padvise.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendOnRename<P0>(&self, pmk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IMoniker>,
    {
        (windows_core::Interface::vtable(self).SendOnRename)(windows_core::Interface::as_raw(self), pmk.param().abi()).ok()
    }
    pub unsafe fn SendOnSave(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOnSave)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SendOnClose(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendOnClose)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleAdviseHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendOnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendOnRename: usize,
    pub SendOnSave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOnClose: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleCache, IOleCache_Vtbl, 0x0000011e_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleCache {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleCache, windows_core::IUnknown);
impl IOleCache {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Cache(&self, pformatetc: *const super::Com::FORMATETC, advf: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Cache)(windows_core::Interface::as_raw(self), pformatetc, advf, &mut result__).map(|| result__)
    }
    pub unsafe fn Uncache(&self, dwconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Uncache)(windows_core::Interface::as_raw(self), dwconnection).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCache(&self) -> windows_core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCache)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitCache<P0>(&self, pdataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).InitCache)(windows_core::Interface::as_raw(self), pdataobject.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pformatetc, pmedium, frelease.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Cache: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::FORMATETC, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Cache: usize,
    pub Uncache: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCache: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitCache: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::Com::FORMATETC, *const super::Com::STGMEDIUM, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
}
windows_core::imp::define_interface!(IOleCache2, IOleCache2_Vtbl, 0x00000128_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleCache2 {
    type Target = IOleCache;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleCache2, windows_core::IUnknown, IOleCache);
impl IOleCache2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateCache<P0>(&self, pdataobject: P0, grfupdf: UPDFCACHE_FLAGS, preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).UpdateCache)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), grfupdf, core::mem::transmute(preserved.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn DiscardCache(&self, dwdiscardoptions: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscardCache)(windows_core::Interface::as_raw(self), dwdiscardoptions).ok()
    }
}
#[repr(C)]
pub struct IOleCache2_Vtbl {
    pub base__: IOleCache_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, UPDFCACHE_FLAGS, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateCache: usize,
    pub DiscardCache: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleCacheControl, IOleCacheControl_Vtbl, 0x00000129_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleCacheControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleCacheControl, windows_core::IUnknown);
impl IOleCacheControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnRun<P0>(&self, pdataobject: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        (windows_core::Interface::vtable(self).OnRun)(windows_core::Interface::as_raw(self), pdataobject.param().abi()).ok()
    }
    pub unsafe fn OnStop(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStop)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleCacheControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnRun: usize,
    pub OnStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleClientSite, IOleClientSite_Vtbl, 0x00000118_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleClientSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleClientSite, windows_core::IUnknown);
impl IOleClientSite {
    pub unsafe fn SaveObject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SaveObject)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMoniker(&self, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK) -> windows_core::Result<super::Com::IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), dwassign.0 as _, dwwhichmoniker.0 as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContainer(&self) -> windows_core::Result<IOleContainer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ShowObject(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowObject)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnShowWindow<P0>(&self, fshow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnShowWindow)(windows_core::Interface::as_raw(self), fshow.param().abi()).ok()
    }
    pub unsafe fn RequestNewObjectLayout(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestNewObjectLayout)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleClientSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SaveObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    pub GetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnShowWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RequestNewObjectLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleCommandTarget, IOleCommandTarget_Vtbl, 0xb722bccb_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IOleCommandTarget {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleCommandTarget, windows_core::IUnknown);
impl IOleCommandTarget {
    pub unsafe fn QueryStatus(&self, pguidcmdgroup: *const windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), pguidcmdgroup, ccmds, prgcmds, pcmdtext).ok()
    }
    pub unsafe fn Exec(&self, pguidcmdgroup: *const windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const windows_core::VARIANT, pvaout: *mut windows_core::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Exec)(windows_core::Interface::as_raw(self), pguidcmdgroup, ncmdid, ncmdexecopt, core::mem::transmute(pvain), core::mem::transmute(pvaout)).ok()
    }
}
#[repr(C)]
pub struct IOleCommandTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut OLECMD, *mut OLECMDTEXT) -> windows_core::HRESULT,
    pub Exec: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleContainer, IOleContainer_Vtbl, 0x0000011b_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleContainer {
    type Target = IParseDisplayName;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleContainer, windows_core::IUnknown, IParseDisplayName);
impl IOleContainer {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumObjects(&self, grfflags: OLECONTF) -> windows_core::Result<super::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), grfflags.0 as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LockContainer<P0>(&self, flock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LockContainer)(windows_core::Interface::as_raw(self), flock.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleContainer_Vtbl {
    pub base__: IParseDisplayName_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumObjects: usize,
    pub LockContainer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleControl, IOleControl_Vtbl, 0xb196b288_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IOleControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleControl, windows_core::IUnknown);
impl IOleControl {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetControlInfo(&self, pci: *mut CONTROLINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetControlInfo)(windows_core::Interface::as_raw(self), pci).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn OnMnemonic(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnMnemonic)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
    pub unsafe fn OnAmbientPropertyChange(&self, dispid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnAmbientPropertyChange)(windows_core::Interface::as_raw(self), dispid).ok()
    }
    pub unsafe fn FreezeEvents<P0>(&self, bfreeze: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).FreezeEvents)(windows_core::Interface::as_raw(self), bfreeze.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetControlInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CONTROLINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetControlInfo: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub OnMnemonic: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    OnMnemonic: usize,
    pub OnAmbientPropertyChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FreezeEvents: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleControlSite, IOleControlSite_Vtbl, 0xb196b289_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IOleControlSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleControlSite, windows_core::IUnknown);
impl IOleControlSite {
    pub unsafe fn OnControlInfoChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnControlInfoChanged)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockInPlaceActive<P0>(&self, flock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LockInPlaceActive)(windows_core::Interface::as_raw(self), flock.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtendedControl(&self) -> windows_core::Result<super::Com::IDispatch> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExtendedControl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransformCoords(&self, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: XFORMCOORDS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TransformCoords)(windows_core::Interface::as_raw(self), pptlhimetric, pptfcontainer, dwflags.0 as _).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg, grfmodifiers).ok()
    }
    pub unsafe fn OnFocus<P0>(&self, fgotfocus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnFocus)(windows_core::Interface::as_raw(self), fgotfocus.param().abi()).ok()
    }
    pub unsafe fn ShowPropertyFrame(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowPropertyFrame)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleControlSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnControlInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockInPlaceActive: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtendedControl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtendedControl: usize,
    pub TransformCoords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::POINTL, *mut POINTF, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG, KEYMODIFIERS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TranslateAccelerator: usize,
    pub OnFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ShowPropertyFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleDocument, IOleDocument_Vtbl, 0xb722bcc5_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IOleDocument {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleDocument, windows_core::IUnknown);
impl IOleDocument {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateView<P0, P1>(&self, pipsite: P0, pstm: P1, dwreserved: u32) -> windows_core::Result<IOleDocumentView>
    where
        P0: windows_core::Param<IOleInPlaceSite>,
        P1: windows_core::Param<super::Com::IStream>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateView)(windows_core::Interface::as_raw(self), pipsite.param().abi(), pstm.param().abi(), dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocMiscStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocMiscStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumViews(&self, ppenum: *mut Option<IEnumOleDocumentViews>, ppview: *mut Option<IOleDocumentView>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnumViews)(windows_core::Interface::as_raw(self), core::mem::transmute(ppenum), core::mem::transmute(ppview)).ok()
    }
}
#[repr(C)]
pub struct IOleDocument_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateView: usize,
    pub GetDocMiscStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleDocumentSite, IOleDocumentSite_Vtbl, 0xb722bcc7_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IOleDocumentSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleDocumentSite, windows_core::IUnknown);
impl IOleDocumentSite {
    pub unsafe fn ActivateMe<P0>(&self, pviewtoactivate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleDocumentView>,
    {
        (windows_core::Interface::vtable(self).ActivateMe)(windows_core::Interface::as_raw(self), pviewtoactivate.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleDocumentSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateMe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleDocumentView, IOleDocumentView_Vtbl, 0xb722bcc6_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IOleDocumentView {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleDocumentView, windows_core::IUnknown);
impl IOleDocumentView {
    pub unsafe fn SetInPlaceSite<P0>(&self, pipsite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleInPlaceSite>,
    {
        (windows_core::Interface::vtable(self).SetInPlaceSite)(windows_core::Interface::as_raw(self), pipsite.param().abi()).ok()
    }
    pub unsafe fn GetInPlaceSite(&self) -> windows_core::Result<IOleInPlaceSite> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInPlaceSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocument(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetRect(&self, prcview: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRect)(windows_core::Interface::as_raw(self), prcview).ok()
    }
    pub unsafe fn GetRect(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetRectComplex(&self, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetRectComplex)(windows_core::Interface::as_raw(self), prcview, prchscroll, prcvscroll, prcsizebox).ok()
    }
    pub unsafe fn Show<P0>(&self, fshow: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), fshow.param().abi()).ok()
    }
    pub unsafe fn UIActivate<P0>(&self, fuiactivate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UIActivate)(windows_core::Interface::as_raw(self), fuiactivate.param().abi()).ok()
    }
    pub unsafe fn Open(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CloseView(&self, dwreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseView)(windows_core::Interface::as_raw(self), dwreserved).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveViewState<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).SaveViewState)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplyViewState<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IStream>,
    {
        (windows_core::Interface::vtable(self).ApplyViewState)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok()
    }
    pub unsafe fn Clone<P0>(&self, pipsitenew: P0) -> windows_core::Result<IOleDocumentView>
    where
        P0: windows_core::Param<IOleInPlaceSite>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), pipsitenew.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOleDocumentView_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInPlaceSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInPlaceSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetRectComplex: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub UIActivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseView: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveViewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveViewState: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplyViewState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplyViewState: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceActiveObject, IOleInPlaceActiveObject_Vtbl, 0x00000117_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceActiveObject {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceActiveObject, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceActiveObject {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: Option<*const super::super::UI::WindowsAndMessaging::MSG>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), core::mem::transmute(lpmsg.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn OnFrameWindowActivate<P0>(&self, factivate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnFrameWindowActivate)(windows_core::Interface::as_raw(self), factivate.param().abi()).ok()
    }
    pub unsafe fn OnDocWindowActivate<P0>(&self, factivate: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnDocWindowActivate)(windows_core::Interface::as_raw(self), factivate.param().abi()).ok()
    }
    pub unsafe fn ResizeBorder<P0, P1>(&self, prcborder: *const super::super::Foundation::RECT, puiwindow: P0, fframewindow: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleInPlaceUIWindow>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ResizeBorder)(windows_core::Interface::as_raw(self), prcborder, puiwindow.param().abi(), fframewindow.param().abi()).ok()
    }
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceActiveObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TranslateAccelerator: usize,
    pub OnFrameWindowActivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnDocWindowActivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ResizeBorder: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceFrame, IOleInPlaceFrame_Vtbl, 0x00000116_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceFrame {
    type Target = IOleInPlaceUIWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceFrame, windows_core::IUnknown, IOleWindow, IOleInPlaceUIWindow);
impl IOleInPlaceFrame {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn InsertMenus<P0>(&self, hmenushared: P0, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
    {
        (windows_core::Interface::vtable(self).InsertMenus)(windows_core::Interface::as_raw(self), hmenushared.param().abi(), lpmenuwidths).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetMenu<P0, P1>(&self, hmenushared: P0, holemenu: isize, hwndactiveobject: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetMenu)(windows_core::Interface::as_raw(self), hmenushared.param().abi(), holemenu, hwndactiveobject.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn RemoveMenus<P0>(&self, hmenushared: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HMENU>,
    {
        (windows_core::Interface::vtable(self).RemoveMenus)(windows_core::Interface::as_raw(self), hmenushared.param().abi()).ok()
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetStatusText)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()).ok()
    }
    pub unsafe fn EnableModeless<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).EnableModeless)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TranslateAccelerator(&self, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), lpmsg, wid).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceFrame_Vtbl {
    pub base__: IOleInPlaceUIWindow_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub InsertMenus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HMENU, *mut OLEMENUGROUPWIDTHS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    InsertMenus: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HMENU, isize, super::super::Foundation::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub RemoveMenus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    RemoveMenus: usize,
    pub SetStatusText: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub EnableModeless: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG, u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TranslateAccelerator: usize,
}
windows_core::imp::define_interface!(IOleInPlaceObject, IOleInPlaceObject_Vtbl, 0x00000113_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceObject {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceObject, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceObject {
    pub unsafe fn InPlaceDeactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InPlaceDeactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn UIDeactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UIDeactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetObjectRects(&self, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjectRects)(windows_core::Interface::as_raw(self), lprcposrect, lprccliprect).ok()
    }
    pub unsafe fn ReactivateAndUndo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReactivateAndUndo)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceObject_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub InPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetObjectRects: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub ReactivateAndUndo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceObjectWindowless, IOleInPlaceObjectWindowless_Vtbl, 0x1c2056cc_5ef4_101b_8bc8_00aa003e3b29);
impl core::ops::Deref for IOleInPlaceObjectWindowless {
    type Target = IOleInPlaceObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceObjectWindowless, windows_core::IUnknown, IOleWindow, IOleInPlaceObject);
impl IOleInPlaceObjectWindowless {
    pub unsafe fn OnWindowMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> windows_core::Result<super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnWindowMessage)(windows_core::Interface::as_raw(self), msg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetDropTarget(&self) -> windows_core::Result<IDropTarget> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDropTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOleInPlaceObjectWindowless_Vtbl {
    pub base__: IOleInPlaceObject_Vtbl,
    pub OnWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT,
    pub GetDropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceSite, IOleInPlaceSite_Vtbl, 0x00000119_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceSite {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceSite, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceSite {
    pub unsafe fn CanInPlaceActivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanInPlaceActivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnInPlaceActivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnInPlaceActivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnUIActivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnUIActivate)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetWindowContext(&self, ppframe: *mut Option<IOleInPlaceFrame>, ppdoc: *mut Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetWindowContext)(windows_core::Interface::as_raw(self), core::mem::transmute(ppframe), core::mem::transmute(ppdoc), lprcposrect, lprccliprect, lpframeinfo).ok()
    }
    pub unsafe fn Scroll(&self, scrollextant: super::super::Foundation::SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), core::mem::transmute(scrollextant)).ok()
    }
    pub unsafe fn OnUIDeactivate<P0>(&self, fundoable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnUIDeactivate)(windows_core::Interface::as_raw(self), fundoable.param().abi()).ok()
    }
    pub unsafe fn OnInPlaceDeactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnInPlaceDeactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DiscardUndoState(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DiscardUndoState)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeactivateAndUndo(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeactivateAndUndo)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnPosRectChange(&self, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnPosRectChange)(windows_core::Interface::as_raw(self), lprcposrect).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceSite_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub CanInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnInPlaceActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetWindowContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::super::Foundation::RECT, *mut super::super::Foundation::RECT, *mut OLEINPLACEFRAMEINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetWindowContext: usize,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub OnUIDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OnInPlaceDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DiscardUndoState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeactivateAndUndo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnPosRectChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceSiteEx, IOleInPlaceSiteEx_Vtbl, 0x9c2cad80_3424_11cf_b670_00aa004cd6d8);
impl core::ops::Deref for IOleInPlaceSiteEx {
    type Target = IOleInPlaceSite;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteEx, windows_core::IUnknown, IOleWindow, IOleInPlaceSite);
impl IOleInPlaceSiteEx {
    pub unsafe fn OnInPlaceActivateEx(&self, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnInPlaceActivateEx)(windows_core::Interface::as_raw(self), pfnoredraw, dwflags).ok()
    }
    pub unsafe fn OnInPlaceDeactivateEx<P0>(&self, fnoredraw: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnInPlaceDeactivateEx)(windows_core::Interface::as_raw(self), fnoredraw.param().abi()).ok()
    }
    pub unsafe fn RequestUIActivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestUIActivate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceSiteEx_Vtbl {
    pub base__: IOleInPlaceSite_Vtbl,
    pub OnInPlaceActivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub OnInPlaceDeactivateEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub RequestUIActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceSiteWindowless, IOleInPlaceSiteWindowless_Vtbl, 0x922eada0_3424_11cf_b670_00aa004cd6d8);
impl core::ops::Deref for IOleInPlaceSiteWindowless {
    type Target = IOleInPlaceSiteEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceSiteWindowless, windows_core::IUnknown, IOleWindow, IOleInPlaceSite, IOleInPlaceSiteEx);
impl IOleInPlaceSiteWindowless {
    pub unsafe fn CanWindowlessActivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CanWindowlessActivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCapture(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCapture)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCapture<P0>(&self, fcapture: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCapture)(windows_core::Interface::as_raw(self), fcapture.param().abi()).ok()
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFocus<P0>(&self, ffocus: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self), ffocus.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, prect: *const super::super::Foundation::RECT, grfflags: u32) -> windows_core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDC)(windows_core::Interface::as_raw(self), prect, grfflags, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, hdc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).ReleaseDC)(windows_core::Interface::as_raw(self), hdc.param().abi()).ok()
    }
    pub unsafe fn InvalidateRect<P0>(&self, prect: *const super::super::Foundation::RECT, ferase: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InvalidateRect)(windows_core::Interface::as_raw(self), prect, ferase.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn InvalidateRgn<P0, P1>(&self, hrgn: P0, ferase: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HRGN>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InvalidateRgn)(windows_core::Interface::as_raw(self), hrgn.param().abi(), ferase.param().abi()).ok()
    }
    pub unsafe fn ScrollRect(&self, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ScrollRect)(windows_core::Interface::as_raw(self), dx, dy, prectscroll, prectclip).ok()
    }
    pub unsafe fn AdjustRect(&self, prc: *mut super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AdjustRect)(windows_core::Interface::as_raw(self), prc).ok()
    }
    pub unsafe fn OnDefWindowMessage<P0, P1>(&self, msg: u32, wparam: P0, lparam: P1) -> windows_core::Result<super::super::Foundation::LRESULT>
    where
        P0: windows_core::Param<super::super::Foundation::WPARAM>,
        P1: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OnDefWindowMessage)(windows_core::Interface::as_raw(self), msg, wparam.param().abi(), lparam.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOleInPlaceSiteWindowless_Vtbl {
    pub base__: IOleInPlaceSiteEx_Vtbl,
    pub CanWindowlessActivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCapture: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, u32, *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub InvalidateRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub InvalidateRgn: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HRGN, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    InvalidateRgn: usize,
    pub ScrollRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub AdjustRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub OnDefWindowMessage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleInPlaceUIWindow, IOleInPlaceUIWindow_Vtbl, 0x00000115_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleInPlaceUIWindow {
    type Target = IOleWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleInPlaceUIWindow, windows_core::IUnknown, IOleWindow);
impl IOleInPlaceUIWindow {
    pub unsafe fn GetBorder(&self) -> windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBorder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequestBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestBorderSpace)(windows_core::Interface::as_raw(self), pborderwidths).ok()
    }
    pub unsafe fn SetBorderSpace(&self, pborderwidths: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBorderSpace)(windows_core::Interface::as_raw(self), pborderwidths).ok()
    }
    pub unsafe fn SetActiveObject<P0, P1>(&self, pactiveobject: P0, pszobjname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleInPlaceActiveObject>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetActiveObject)(windows_core::Interface::as_raw(self), pactiveobject.param().abi(), pszobjname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleInPlaceUIWindow_Vtbl {
    pub base__: IOleWindow_Vtbl,
    pub GetBorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub RequestBorderSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetBorderSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub SetActiveObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleItemContainer, IOleItemContainer_Vtbl, 0x0000011c_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleItemContainer {
    type Target = IOleContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleItemContainer, windows_core::IUnknown, IParseDisplayName, IOleContainer);
impl IOleItemContainer {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObject<P0, P1, T>(&self, pszitem: P0, dwspeedneeded: u32, pbc: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Com::IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pszitem.param().abi(), dwspeedneeded, pbc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetObjectStorage<P0, P1, T>(&self, pszitem: P0, pbc: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::Com::IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetObjectStorage)(windows_core::Interface::as_raw(self), pszitem.param().abi(), pbc.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsRunning<P0>(&self, pszitem: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pszitem.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleItemContainer_Vtbl {
    pub base__: IOleContainer_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetObjectStorage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetObjectStorage: usize,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleLink, IOleLink_Vtbl, 0x0000011d_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleLink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleLink, windows_core::IUnknown);
impl IOleLink {
    pub unsafe fn SetUpdateOptions(&self, dwupdateopt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUpdateOptions)(windows_core::Interface::as_raw(self), dwupdateopt).ok()
    }
    pub unsafe fn GetUpdateOptions(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdateOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSourceMoniker<P0>(&self, pmk: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IMoniker>,
    {
        (windows_core::Interface::vtable(self).SetSourceMoniker)(windows_core::Interface::as_raw(self), pmk.param().abi(), rclsid).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSourceMoniker(&self) -> windows_core::Result<super::Com::IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceMoniker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSourceDisplayName<P0>(&self, pszstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSourceDisplayName)(windows_core::Interface::as_raw(self), pszstatustext.param().abi()).ok()
    }
    pub unsafe fn GetSourceDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSourceDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToSource<P0>(&self, bindflags: u32, pbc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).BindToSource)(windows_core::Interface::as_raw(self), bindflags, pbc.param().abi()).ok()
    }
    pub unsafe fn BindIfRunning(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).BindIfRunning)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBoundSource(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetBoundSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnbindSource(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnbindSource)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Update<P0>(&self, pbc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IBindCtx>,
    {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), pbc.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleLink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSourceMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSourceMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSourceMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSourceMoniker: usize,
    pub SetSourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSourceDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BindToSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BindToSource: usize,
    pub BindIfRunning: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBoundSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnbindSource: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Update: usize,
}
windows_core::imp::define_interface!(IOleObject, IOleObject_Vtbl, 0x00000112_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleObject, windows_core::IUnknown);
impl IOleObject {
    pub unsafe fn SetClientSite<P0>(&self, pclientsite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleClientSite>,
    {
        (windows_core::Interface::vtable(self).SetClientSite)(windows_core::Interface::as_raw(self), pclientsite.param().abi()).ok()
    }
    pub unsafe fn GetClientSite(&self) -> windows_core::Result<IOleClientSite> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClientSite)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetHostNames<P0, P1>(&self, szcontainerapp: P0, szcontainerobj: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHostNames)(windows_core::Interface::as_raw(self), szcontainerapp.param().abi(), szcontainerobj.param().abi()).ok()
    }
    pub unsafe fn Close(&self, dwsaveoption: OLECLOSE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), dwsaveoption.0 as _).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMoniker<P0>(&self, dwwhichmoniker: OLEWHICHMK, pmk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IMoniker>,
    {
        (windows_core::Interface::vtable(self).SetMoniker)(windows_core::Interface::as_raw(self), dwwhichmoniker.0 as _, pmk.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMoniker(&self, dwassign: OLEGETMONIKER, dwwhichmoniker: OLEWHICHMK) -> windows_core::Result<super::Com::IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), dwassign.0 as _, dwwhichmoniker.0 as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitFromData<P0, P1>(&self, pdataobject: P0, fcreation: P1, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).InitFromData)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), fcreation.param().abi(), dwreserved).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClipboardData(&self, dwreserved: u32) -> windows_core::Result<super::Com::IDataObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClipboardData)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn DoVerb<P0, P1>(&self, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: P0, lindex: i32, hwndparent: P1, lprcposrect: *const super::super::Foundation::RECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleClientSite>,
        P1: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).DoVerb)(windows_core::Interface::as_raw(self), iverb, lpmsg, pactivesite.param().abi(), lindex, hwndparent.param().abi(), lprcposrect).ok()
    }
    pub unsafe fn EnumVerbs(&self) -> windows_core::Result<IEnumOLEVERB> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumVerbs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Update(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsUpToDate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsUpToDate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetUserClassID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetUserType(&self, dwformoftype: USERCLASSTYPE) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserType)(windows_core::Interface::as_raw(self), dwformoftype.0 as _, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExtent(&self, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, psizel).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Advise<P0>(&self, padvsink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::Com::IAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), padvsink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<super::Com::IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMiscStatus(&self, dwaspect: super::Com::DVASPECT) -> windows_core::Result<OLEMISC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMiscStatus)(windows_core::Interface::as_raw(self), dwaspect, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetColorScheme(&self, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetColorScheme)(windows_core::Interface::as_raw(self), plogpal).ok()
    }
}
#[repr(C)]
pub struct IOleObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClientSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHostNames: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMoniker: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InitFromData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitFromData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClipboardData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClipboardData: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub DoVerb: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const super::super::UI::WindowsAndMessaging::MSG, *mut core::ffi::c_void, i32, super::super::Foundation::HWND, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    DoVerb: usize,
    pub EnumVerbs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsUpToDate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetUserType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, *const super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExtent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMiscStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, *mut OLEMISC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMiscStatus: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetColorScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetColorScheme: usize,
}
windows_core::imp::define_interface!(IOleParentUndoUnit, IOleParentUndoUnit_Vtbl, 0xa1faf330_ef97_11ce_9bc9_00aa00608e01);
impl core::ops::Deref for IOleParentUndoUnit {
    type Target = IOleUndoUnit;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleParentUndoUnit, windows_core::IUnknown, IOleUndoUnit);
impl IOleParentUndoUnit {
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), ppuu.param().abi()).ok()
    }
    pub unsafe fn Close<P0, P1>(&self, ppuu: P0, fcommit: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), ppuu.param().abi(), fcommit.param().abi()).ok()
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn FindUnit<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).FindUnit)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn GetParentState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOleParentUndoUnit_Vtbl {
    pub base__: IOleUndoUnit_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindUnit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUILinkContainerA, IOleUILinkContainerA_Vtbl, 0);
impl core::ops::Deref for IOleUILinkContainerA {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUILinkContainerA, windows_core::IUnknown);
impl IOleUILinkContainerA {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetNextLink)(windows_core::Interface::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLinkUpdateOptions)(windows_core::Interface::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLinkUpdateOptions)(windows_core::Interface::as_raw(self), dwlink, &mut result__).map(|| result__)
    }
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLinkSource)(windows_core::Interface::as_raw(self), dwlink, lpszdisplayname.param().abi(), lenfilename, pcheaten, fvalidatesource.param().abi()).ok()
    }
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: Option<*mut windows_core::PSTR>, lplenfilename: *mut u32, lplpszfulllinktype: Option<*mut windows_core::PSTR>, lplpszshortlinktype: Option<*mut windows_core::PSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLinkSource)(windows_core::Interface::as_raw(self), dwlink, core::mem::transmute(lplpszdisplayname.unwrap_or(std::ptr::null_mut())), lplenfilename, core::mem::transmute(lplpszfulllinktype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszshortlinktype.unwrap_or(std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenLinkSource)(windows_core::Interface::as_raw(self), dwlink).ok()
    }
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UpdateLink)(windows_core::Interface::as_raw(self), dwlink, ferrormessage.param().abi(), freserved.param().abi()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelLink)(windows_core::Interface::as_raw(self), dwlink).ok()
    }
}
#[repr(C)]
pub struct IOleUILinkContainerA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCSTR, u32, *mut u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PSTR, *mut u32, *mut windows_core::PSTR, *mut windows_core::PSTR, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OpenLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CancelLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUILinkContainerW, IOleUILinkContainerW_Vtbl, 0);
impl core::ops::Deref for IOleUILinkContainerW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUILinkContainerW, windows_core::IUnknown);
impl IOleUILinkContainerW {
    pub unsafe fn GetNextLink(&self, dwlink: u32) -> u32 {
        (windows_core::Interface::vtable(self).GetNextLink)(windows_core::Interface::as_raw(self), dwlink)
    }
    pub unsafe fn SetLinkUpdateOptions(&self, dwlink: u32, dwupdateopt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetLinkUpdateOptions)(windows_core::Interface::as_raw(self), dwlink, dwupdateopt).ok()
    }
    pub unsafe fn GetLinkUpdateOptions(&self, dwlink: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLinkUpdateOptions)(windows_core::Interface::as_raw(self), dwlink, &mut result__).map(|| result__)
    }
    pub unsafe fn SetLinkSource<P0, P1>(&self, dwlink: u32, lpszdisplayname: P0, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetLinkSource)(windows_core::Interface::as_raw(self), dwlink, lpszdisplayname.param().abi(), lenfilename, pcheaten, fvalidatesource.param().abi()).ok()
    }
    pub unsafe fn GetLinkSource(&self, dwlink: u32, lplpszdisplayname: Option<*mut windows_core::PWSTR>, lplenfilename: *mut u32, lplpszfulllinktype: Option<*mut windows_core::PWSTR>, lplpszshortlinktype: Option<*mut windows_core::PWSTR>, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLinkSource)(windows_core::Interface::as_raw(self), dwlink, core::mem::transmute(lplpszdisplayname.unwrap_or(std::ptr::null_mut())), lplenfilename, core::mem::transmute(lplpszfulllinktype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszshortlinktype.unwrap_or(std::ptr::null_mut())), lpfsourceavailable, lpfisselected).ok()
    }
    pub unsafe fn OpenLinkSource(&self, dwlink: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OpenLinkSource)(windows_core::Interface::as_raw(self), dwlink).ok()
    }
    pub unsafe fn UpdateLink<P0, P1>(&self, dwlink: u32, ferrormessage: P0, freserved: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).UpdateLink)(windows_core::Interface::as_raw(self), dwlink, ferrormessage.param().abi(), freserved.param().abi()).ok()
    }
    pub unsafe fn CancelLink(&self, dwlink: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CancelLink)(windows_core::Interface::as_raw(self), dwlink).ok()
    }
}
#[repr(C)]
pub struct IOleUILinkContainerW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> u32,
    pub SetLinkUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetLinkUpdateOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut super::super::Foundation::BOOL, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OpenLinkSource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub CancelLink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUILinkInfoA, IOleUILinkInfoA_Vtbl, 0);
impl core::ops::Deref for IOleUILinkInfoA {
    type Target = IOleUILinkContainerA;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUILinkInfoA, windows_core::IUnknown, IOleUILinkContainerA);
impl IOleUILinkInfoA {
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastUpdate)(windows_core::Interface::as_raw(self), dwlink, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOleUILinkInfoA_Vtbl {
    pub base__: IOleUILinkContainerA_Vtbl,
    pub GetLastUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUILinkInfoW, IOleUILinkInfoW_Vtbl, 0);
impl core::ops::Deref for IOleUILinkInfoW {
    type Target = IOleUILinkContainerW;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUILinkInfoW, windows_core::IUnknown, IOleUILinkContainerW);
impl IOleUILinkInfoW {
    pub unsafe fn GetLastUpdate(&self, dwlink: u32) -> windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastUpdate)(windows_core::Interface::as_raw(self), dwlink, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IOleUILinkInfoW_Vtbl {
    pub base__: IOleUILinkContainerW_Vtbl,
    pub GetLastUpdate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUIObjInfoA, IOleUIObjInfoA_Vtbl, 0);
impl core::ops::Deref for IOleUIObjInfoA {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUIObjInfoA, windows_core::IUnknown);
impl IOleUIObjInfoA {
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: Option<*mut windows_core::PSTR>, lplpsztype: Option<*mut windows_core::PSTR>, lplpszshorttype: Option<*mut windows_core::PSTR>, lplpszlocation: Option<*mut windows_core::PSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), dwobject, lpdwobjsize, core::mem::transmute(lplpszlabel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpsztype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszshorttype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszlocation.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConvertInfo)(windows_core::Interface::as_raw(self), dwobject, lpclassid, lpwformat, lpconvertdefaultclassid, lplpclsidexclude, core::mem::transmute(lpcclsidexclude.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertObject)(windows_core::Interface::as_raw(self), dwobject, clsidnew).ok()
    }
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: Option<*const super::super::Foundation::HGLOBAL>, pdvaspect: Option<*const u32>, pncurrentscale: Option<*const i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetViewInfo)(windows_core::Interface::as_raw(self), dwobject, core::mem::transmute(phmetapict.unwrap_or(std::ptr::null())), core::mem::transmute(pdvaspect.unwrap_or(std::ptr::null())), core::mem::transmute(pncurrentscale.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetViewInfo<P0, P1>(&self, dwobject: u32, hmetapict: P0, dvaspect: u32, ncurrentscale: i32, brelativetoorig: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HGLOBAL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetViewInfo)(windows_core::Interface::as_raw(self), dwobject, hmetapict.param().abi(), dvaspect, ncurrentscale, brelativetoorig.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleUIObjInfoA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::PSTR, *mut windows_core::PSTR, *mut windows_core::PSTR, *mut windows_core::PSTR) -> windows_core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u16, *mut windows_core::GUID, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::HGLOBAL, *const u32, *const i32) -> windows_core::HRESULT,
    pub SetViewInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HGLOBAL, u32, i32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUIObjInfoW, IOleUIObjInfoW_Vtbl, 0);
impl core::ops::Deref for IOleUIObjInfoW {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUIObjInfoW, windows_core::IUnknown);
impl IOleUIObjInfoW {
    pub unsafe fn GetObjectInfo(&self, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: Option<*mut windows_core::PWSTR>, lplpsztype: Option<*mut windows_core::PWSTR>, lplpszshorttype: Option<*mut windows_core::PWSTR>, lplpszlocation: Option<*mut windows_core::PWSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), dwobject, lpdwobjsize, core::mem::transmute(lplpszlabel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpsztype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszshorttype.unwrap_or(std::ptr::null_mut())), core::mem::transmute(lplpszlocation.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetConvertInfo(&self, dwobject: u32, lpclassid: *mut windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut windows_core::GUID, lplpclsidexclude: *mut *mut windows_core::GUID, lpcclsidexclude: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetConvertInfo)(windows_core::Interface::as_raw(self), dwobject, lpclassid, lpwformat, lpconvertdefaultclassid, lplpclsidexclude, core::mem::transmute(lpcclsidexclude.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ConvertObject(&self, dwobject: u32, clsidnew: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ConvertObject)(windows_core::Interface::as_raw(self), dwobject, clsidnew).ok()
    }
    pub unsafe fn GetViewInfo(&self, dwobject: u32, phmetapict: Option<*const super::super::Foundation::HGLOBAL>, pdvaspect: Option<*const u32>, pncurrentscale: Option<*const i32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetViewInfo)(windows_core::Interface::as_raw(self), dwobject, core::mem::transmute(phmetapict.unwrap_or(std::ptr::null())), core::mem::transmute(pdvaspect.unwrap_or(std::ptr::null())), core::mem::transmute(pncurrentscale.unwrap_or(std::ptr::null()))).ok()
    }
    pub unsafe fn SetViewInfo<P0, P1>(&self, dwobject: u32, hmetapict: P0, dvaspect: u32, ncurrentscale: i32, brelativetoorig: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HGLOBAL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetViewInfo)(windows_core::Interface::as_raw(self), dwobject, hmetapict.param().abi(), dvaspect, ncurrentscale, brelativetoorig.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleUIObjInfoW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetConvertInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u16, *mut windows_core::GUID, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub ConvertObject: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetViewInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::HGLOBAL, *const u32, *const i32) -> windows_core::HRESULT,
    pub SetViewInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::super::Foundation::HGLOBAL, u32, i32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUndoManager, IOleUndoManager_Vtbl, 0xd001f200_ef97_11ce_9bc9_00aa00608e01);
impl core::ops::Deref for IOleUndoManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUndoManager, windows_core::IUnknown);
impl IOleUndoManager {
    pub unsafe fn Open<P0>(&self, ppuu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
    {
        (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), ppuu.param().abi()).ok()
    }
    pub unsafe fn Close<P0, P1>(&self, ppuu: P0, fcommit: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IOleParentUndoUnit>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), ppuu.param().abi(), fcommit.param().abi())
    }
    pub unsafe fn Add<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn GetOpenParentState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetOpenParentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DiscardFrom<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).DiscardFrom)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn UndoTo<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).UndoTo)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn RedoTo<P0>(&self, puu: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoUnit>,
    {
        (windows_core::Interface::vtable(self).RedoTo)(windows_core::Interface::as_raw(self), puu.param().abi()).ok()
    }
    pub unsafe fn EnumUndoable(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumUndoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumRedoable(&self) -> windows_core::Result<IEnumOleUndoUnits> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRedoable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLastUndoDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastUndoDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLastRedoDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLastRedoDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enable<P0>(&self, fenable: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleUndoManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOpenParentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DiscardFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UndoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RedoTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumUndoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumRedoable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastUndoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLastRedoDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleUndoUnit, IOleUndoUnit_Vtbl, 0x894ad3b0_ef97_11ce_9bc9_00aa00608e01);
impl core::ops::Deref for IOleUndoUnit {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleUndoUnit, windows_core::IUnknown);
impl IOleUndoUnit {
    pub unsafe fn Do<P0>(&self, pundomanager: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IOleUndoManager>,
    {
        (windows_core::Interface::vtable(self).Do)(windows_core::Interface::as_raw(self), pundomanager.param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUnitType(&self, pclsid: *mut windows_core::GUID, plid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUnitType)(windows_core::Interface::as_raw(self), pclsid, plid).ok()
    }
    pub unsafe fn OnNextAdd(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnNextAdd)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOleUndoUnit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Do: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetUnitType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut i32) -> windows_core::HRESULT,
    pub OnNextAdd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOleWindow, IOleWindow_Vtbl, 0x00000114_0000_0000_c000_000000000046);
impl core::ops::Deref for IOleWindow {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOleWindow, windows_core::IUnknown);
impl IOleWindow {
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ContextSensitiveHelp<P0>(&self, fentermode: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ContextSensitiveHelp)(windows_core::Interface::as_raw(self), fentermode.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IOleWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub ContextSensitiveHelp: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IParseDisplayName, IParseDisplayName_Vtbl, 0x0000011a_0000_0000_c000_000000000046);
impl core::ops::Deref for IParseDisplayName {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IParseDisplayName, windows_core::IUnknown);
impl IParseDisplayName {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ParseDisplayName<P0, P1>(&self, pbc: P0, pszdisplayname: P1, pcheaten: *mut u32, ppmkout: *mut Option<super::Com::IMoniker>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IBindCtx>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ParseDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pszdisplayname.param().abi(), pcheaten, core::mem::transmute(ppmkout)).ok()
    }
}
#[repr(C)]
pub struct IParseDisplayName_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ParseDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ParseDisplayName: usize,
}
windows_core::imp::define_interface!(IPerPropertyBrowsing, IPerPropertyBrowsing_Vtbl, 0x376bd3aa_3845_101b_84ed_08002b2ec713);
impl core::ops::Deref for IPerPropertyBrowsing {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPerPropertyBrowsing, windows_core::IUnknown);
impl IPerPropertyBrowsing {
    pub unsafe fn GetDisplayString(&self, dispid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayString)(windows_core::Interface::as_raw(self), dispid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MapPropertyToPage(&self, dispid: i32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MapPropertyToPage)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetPredefinedStrings(&self, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPredefinedStrings)(windows_core::Interface::as_raw(self), dispid, pcastringsout, pcacookiesout).ok()
    }
    pub unsafe fn GetPredefinedValue(&self, dispid: i32, dwcookie: u32) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPredefinedValue)(windows_core::Interface::as_raw(self), dispid, dwcookie, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPerPropertyBrowsing_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDisplayString: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MapPropertyToPage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetPredefinedStrings: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut CALPOLESTR, *mut CADWORD) -> windows_core::HRESULT,
    pub GetPredefinedValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPersistPropertyBag, IPersistPropertyBag_Vtbl, 0x37d84f60_42cb_11ce_8135_00aa004bb851);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPersistPropertyBag {
    type Target = super::Com::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag, windows_core::IUnknown, super::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag {
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrorlog: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::StructuredStorage::IPropertyBag>,
        P1: windows_core::Param<super::Com::IErrorLog>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), perrorlog.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Save<P0, P1, P2>(&self, ppropbag: P0, fcleardirty: P1, fsaveallproperties: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::StructuredStorage::IPropertyBag>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), fcleardirty.param().abi(), fsaveallproperties.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistPropertyBag_Vtbl {
    pub base__: super::Com::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Save: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPersistPropertyBag2, IPersistPropertyBag2_Vtbl, 0x22f55881_280b_11d0_a8a9_00a0c90c2004);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPersistPropertyBag2 {
    type Target = super::Com::IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPersistPropertyBag2, windows_core::IUnknown, super::Com::IPersist);
#[cfg(feature = "Win32_System_Com")]
impl IPersistPropertyBag2 {
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Load<P0, P1>(&self, ppropbag: P0, perrlog: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::StructuredStorage::IPropertyBag2>,
        P1: windows_core::Param<super::Com::IErrorLog>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), perrlog.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Save<P0, P1, P2>(&self, ppropbag: P0, fcleardirty: P1, fsaveallproperties: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::StructuredStorage::IPropertyBag2>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
        P2: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), ppropbag.param().abi(), fcleardirty.param().abi(), fsaveallproperties.param().abi()).ok()
    }
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPersistPropertyBag2_Vtbl {
    pub base__: super::Com::IPersist_Vtbl,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Load: usize,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Save: usize,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPicture, IPicture_Vtbl, 0x7bf80980_bf32_101a_8bbb_00aa00300cab);
impl core::ops::Deref for IPicture {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPicture, windows_core::IUnknown);
impl IPicture {
    pub unsafe fn Handle(&self) -> windows_core::Result<OLE_HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn hPal(&self) -> windows_core::Result<OLE_HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).hPal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Type(&self) -> windows_core::Result<PICTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Render<P0>(&self, hdc: P0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc.param().abi(), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds).ok()
    }
    pub unsafe fn set_hPal<P0>(&self, hpal: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<OLE_HANDLE>,
    {
        (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal.param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CurDC(&self) -> windows_core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SelectPicture<P0>(&self, hdcin: P0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).SelectPicture)(windows_core::Interface::as_raw(self), hdcin.param().abi(), phdcout, phbmpout).ok()
    }
    pub unsafe fn KeepOriginalFormat(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeepOriginalFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeepOriginalFormat<P0>(&self, keep: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetKeepOriginalFormat)(windows_core::Interface::as_raw(self), keep.param().abi()).ok()
    }
    pub unsafe fn PictureChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PictureChanged)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveAsFile<P0, P1>(&self, pstream: P0, fsavememcopy: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPicture_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_HANDLE) -> windows_core::HRESULT,
    pub hPal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OLE_HANDLE) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PICTYPE) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC, i32, i32, i32, i32, i32, i32, i32, i32, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC, *mut super::super::Graphics::Gdi::HDC, *mut OLE_HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    pub KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetKeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPicture2, IPicture2_Vtbl, 0xf5185dd8_2012_4b0b_aad9_f052c6bd482b);
impl core::ops::Deref for IPicture2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPicture2, windows_core::IUnknown);
impl IPicture2 {
    pub unsafe fn Handle(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Handle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn hPal(&self) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).hPal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Type(&self) -> windows_core::Result<i16> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Render<P0>(&self, hdc: P0, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).Render)(windows_core::Interface::as_raw(self), hdc.param().abi(), x, y, cx, cy, xsrc, ysrc, cxsrc, cysrc, prcwbounds).ok()
    }
    pub unsafe fn set_hPal(&self, hpal: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).set_hPal)(windows_core::Interface::as_raw(self), hpal).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CurDC(&self) -> windows_core::Result<super::super::Graphics::Gdi::HDC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurDC)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SelectPicture<P0>(&self, hdcin: P0, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).SelectPicture)(windows_core::Interface::as_raw(self), hdcin.param().abi(), phdcout, phbmpout).ok()
    }
    pub unsafe fn KeepOriginalFormat(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KeepOriginalFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetKeepOriginalFormat<P0>(&self, keep: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetKeepOriginalFormat)(windows_core::Interface::as_raw(self), keep.param().abi()).ok()
    }
    pub unsafe fn PictureChanged(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PictureChanged)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveAsFile<P0, P1>(&self, pstream: P0, fsavememcopy: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::Com::IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SaveAsFile)(windows_core::Interface::as_raw(self), pstream.param().abi(), fsavememcopy.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPicture2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Handle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub hPal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Render: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC, i32, i32, i32, i32, i32, i32, i32, i32, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Render: usize,
    pub set_hPal: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CurDC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Gdi::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CurDC: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SelectPicture: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Graphics::Gdi::HDC, *mut super::super::Graphics::Gdi::HDC, *mut usize) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SelectPicture: usize,
    pub KeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetKeepOriginalFormat: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub PictureChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveAsFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveAsFile: usize,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IPictureDisp, IPictureDisp_Vtbl, 0x7bf80981_bf32_101a_8bbb_00aa00300cab);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IPictureDisp {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IPictureDisp, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IPictureDisp {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IPictureDisp_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
windows_core::imp::define_interface!(IPointerInactive, IPointerInactive_Vtbl, 0x55980ba0_35aa_11cf_b671_00aa004cd6d8);
impl core::ops::Deref for IPointerInactive {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPointerInactive, windows_core::IUnknown);
impl IPointerInactive {
    pub unsafe fn GetActivationPolicy(&self) -> windows_core::Result<POINTERINACTIVE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetActivationPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OnInactiveMouseMove(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnInactiveMouseMove)(windows_core::Interface::as_raw(self), prectbounds, x, y, grfkeystate).ok()
    }
    pub unsafe fn OnInactiveSetCursor<P0>(&self, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnInactiveSetCursor)(windows_core::Interface::as_raw(self), prectbounds, x, y, dwmousemsg, fsetalways.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPointerInactive_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivationPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut POINTERINACTIVE) -> windows_core::HRESULT,
    pub OnInactiveMouseMove: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, i32, i32, u32) -> windows_core::HRESULT,
    pub OnInactiveSetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT, i32, i32, u32, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint, IPrint_Vtbl, 0xb722bcc9_4e68_101b_a2bc_00aa00404770);
impl core::ops::Deref for IPrint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPrint, windows_core::IUnknown);
impl IPrint {
    pub unsafe fn SetInitialPageNum(&self, nfirstpage: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetInitialPageNum)(windows_core::Interface::as_raw(self), nfirstpage).ok()
    }
    pub unsafe fn GetPageInfo(&self, pnfirstpage: *mut i32, pcpages: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPageInfo)(windows_core::Interface::as_raw(self), pnfirstpage, pcpages).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Print<P0>(&self, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: P0, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IContinueCallback>,
    {
        (windows_core::Interface::vtable(self).Print)(windows_core::Interface::as_raw(self), grfflags, pptd, pppageset, pstgmoptions, pcallback.param().abi(), nfirstpage, pcpagesprinted, pnlastpage).ok()
    }
}
#[repr(C)]
pub struct IPrint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInitialPageNum: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Print: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::Com::DVTARGETDEVICE, *mut *mut PAGESET, *mut super::Com::STGMEDIUM, *mut core::ffi::c_void, i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Print: usize,
}
windows_core::imp::define_interface!(IPropertyNotifySink, IPropertyNotifySink_Vtbl, 0x9bfbbc02_eff1_101a_84ed_00aa00341d07);
impl core::ops::Deref for IPropertyNotifySink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyNotifySink, windows_core::IUnknown);
impl IPropertyNotifySink {
    pub unsafe fn OnChanged(&self, dispid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnChanged)(windows_core::Interface::as_raw(self), dispid).ok()
    }
    pub unsafe fn OnRequestEdit(&self, dispid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnRequestEdit)(windows_core::Interface::as_raw(self), dispid).ok()
    }
}
#[repr(C)]
pub struct IPropertyNotifySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub OnRequestEdit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyPage, IPropertyPage_Vtbl, 0xb196b28d_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IPropertyPage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyPage, windows_core::IUnknown);
impl IPropertyPage {
    pub unsafe fn SetPageSite<P0>(&self, ppagesite: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IPropertyPageSite>,
    {
        (windows_core::Interface::vtable(self).SetPageSite)(windows_core::Interface::as_raw(self), ppagesite.param().abi()).ok()
    }
    pub unsafe fn Activate<P0, P1>(&self, hwndparent: P0, prect: *const super::super::Foundation::RECT, bmodal: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), hwndparent.param().abi(), prect, bmodal.param().abi()).ok()
    }
    pub unsafe fn Deactivate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPageInfo(&self, ppageinfo: *mut PROPPAGEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPageInfo)(windows_core::Interface::as_raw(self), ppageinfo).ok()
    }
    pub unsafe fn SetObjects(&self, ppunk: &[Option<windows_core::IUnknown>]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetObjects)(windows_core::Interface::as_raw(self), ppunk.len().try_into().unwrap(), core::mem::transmute(ppunk.as_ptr())).ok()
    }
    pub unsafe fn Show(&self, ncmdshow: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Show)(windows_core::Interface::as_raw(self), ncmdshow).ok()
    }
    pub unsafe fn Move(&self, prect: *const super::super::Foundation::RECT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), prect).ok()
    }
    pub unsafe fn IsPageDirty(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsPageDirty)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Apply(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Apply)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Help<P0>(&self, pszhelpdir: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self), pszhelpdir.param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
}
#[repr(C)]
pub struct IPropertyPage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPageSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const super::super::Foundation::RECT, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PROPPAGEINFO) -> windows_core::HRESULT,
    pub SetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::RECT) -> windows_core::HRESULT,
    pub IsPageDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Apply: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TranslateAccelerator: usize,
}
windows_core::imp::define_interface!(IPropertyPage2, IPropertyPage2_Vtbl, 0x01e44665_24ac_101b_84ed_08002b2ec713);
impl core::ops::Deref for IPropertyPage2 {
    type Target = IPropertyPage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyPage2, windows_core::IUnknown, IPropertyPage);
impl IPropertyPage2 {
    pub unsafe fn EditProperty(&self, dispid: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EditProperty)(windows_core::Interface::as_raw(self), dispid).ok()
    }
}
#[repr(C)]
pub struct IPropertyPage2_Vtbl {
    pub base__: IPropertyPage_Vtbl,
    pub EditProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyPageSite, IPropertyPageSite_Vtbl, 0xb196b28c_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IPropertyPageSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertyPageSite, windows_core::IUnknown);
impl IPropertyPageSite {
    pub unsafe fn OnStatusChange(&self, dwflags: PROPPAGESTATUS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), dwflags.0 as _).ok()
    }
    pub unsafe fn GetLocaleID(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLocaleID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPageContainer(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPageContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn TranslateAccelerator(&self, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TranslateAccelerator)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
}
#[repr(C)]
pub struct IPropertyPageSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetLocaleID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPageContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub TranslateAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    TranslateAccelerator: usize,
}
windows_core::imp::define_interface!(IProtectFocus, IProtectFocus_Vtbl, 0xd81f90a3_8156_44f7_ad28_5abb87003274);
impl core::ops::Deref for IProtectFocus {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProtectFocus, windows_core::IUnknown);
impl IProtectFocus {
    pub unsafe fn AllowFocusChange(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowFocusChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProtectFocus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowFocusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProtectedModeMenuServices, IProtectedModeMenuServices_Vtbl, 0x73c105ee_9dff_4a07_b83c_7eff290c266e);
impl core::ops::Deref for IProtectedModeMenuServices {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProtectedModeMenuServices, windows_core::IUnknown);
impl IProtectedModeMenuServices {
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateMenu(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn LoadMenu<P0, P1>(&self, pszmodulename: P0, pszmenuname: P1) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadMenu)(windows_core::Interface::as_raw(self), pszmodulename.param().abi(), pszmenuname.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn LoadMenuID<P0>(&self, pszmodulename: P0, wresourceid: u16) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LoadMenuID)(windows_core::Interface::as_raw(self), pszmodulename.param().abi(), wresourceid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProtectedModeMenuServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub CreateMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    CreateMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenu: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenu: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub LoadMenuID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u16, *mut super::super::UI::WindowsAndMessaging::HMENU) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    LoadMenuID: usize,
}
windows_core::imp::define_interface!(IProvideClassInfo, IProvideClassInfo_Vtbl, 0xb196b283_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IProvideClassInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideClassInfo, windows_core::IUnknown);
impl IProvideClassInfo {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassInfo(&self) -> windows_core::Result<super::Com::ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IProvideClassInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClassInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClassInfo: usize,
}
windows_core::imp::define_interface!(IProvideClassInfo2, IProvideClassInfo2_Vtbl, 0xa6bc3ac0_dbaa_11ce_9de3_00aa004bb851);
impl core::ops::Deref for IProvideClassInfo2 {
    type Target = IProvideClassInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideClassInfo2, windows_core::IUnknown, IProvideClassInfo);
impl IProvideClassInfo2 {
    pub unsafe fn GetGUID(&self, dwguidkind: u32) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), dwguidkind, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IProvideClassInfo2_Vtbl {
    pub base__: IProvideClassInfo_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProvideMultipleClassInfo, IProvideMultipleClassInfo_Vtbl, 0xa7aba9c1_8983_11cf_8f20_00805f2cd064);
impl core::ops::Deref for IProvideMultipleClassInfo {
    type Target = IProvideClassInfo2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideMultipleClassInfo, windows_core::IUnknown, IProvideClassInfo, IProvideClassInfo2);
impl IProvideMultipleClassInfo {
    pub unsafe fn GetMultiTypeInfoCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMultiTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInfoOfIndex(&self, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut windows_core::GUID, piidsource: *mut windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInfoOfIndex)(windows_core::Interface::as_raw(self), iti, dwflags, core::mem::transmute(ppticoclass), pdwtiflags, pcdispidreserved, piidprimary, piidsource).ok()
    }
}
#[repr(C)]
pub struct IProvideMultipleClassInfo_Vtbl {
    pub base__: IProvideClassInfo2_Vtbl,
    pub GetMultiTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInfoOfIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, MULTICLASSINFO_FLAGS, *mut *mut core::ffi::c_void, *mut u32, *mut u32, *mut windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInfoOfIndex: usize,
}
windows_core::imp::define_interface!(IProvideRuntimeContext, IProvideRuntimeContext_Vtbl, 0x10e2414a_ec59_49d2_bc51_5add2c36febc);
impl core::ops::Deref for IProvideRuntimeContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProvideRuntimeContext, windows_core::IUnknown);
impl IProvideRuntimeContext {
    pub unsafe fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentSourceContext)(windows_core::Interface::as_raw(self), pdwcontext, pfexecutingglobalcode).ok()
    }
}
#[repr(C)]
pub struct IProvideRuntimeContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentSourceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IQuickActivate, IQuickActivate_Vtbl, 0xcf51ed10_62fe_11cf_bf86_00a0c9034836);
impl core::ops::Deref for IQuickActivate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IQuickActivate, windows_core::IUnknown);
impl IQuickActivate {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn QuickActivate(&self, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QuickActivate)(windows_core::Interface::as_raw(self), pqacontainer, pqacontrol).ok()
    }
    pub unsafe fn SetContentExtent(&self, psizel: *const super::super::Foundation::SIZE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetContentExtent)(windows_core::Interface::as_raw(self), psizel).ok()
    }
    pub unsafe fn GetContentExtent(&self) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetContentExtent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IQuickActivate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub QuickActivate: unsafe extern "system" fn(*mut core::ffi::c_void, *const QACONTAINER, *mut QACONTROL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    QuickActivate: usize,
    pub SetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::SIZE) -> windows_core::HRESULT,
    pub GetContentExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRecordInfo, IRecordInfo_Vtbl, 0x0000002f_0000_0000_c000_000000000046);
impl core::ops::Deref for IRecordInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordInit)(windows_core::Interface::as_raw(self), pvnew).ok()
    }
    pub unsafe fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordClear)(windows_core::Interface::as_raw(self), pvexisting).ok()
    }
    pub unsafe fn RecordCopy(&self, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordCopy)(windows_core::Interface::as_raw(self), pvexisting, pvnew).ok()
    }
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTypeInfo(&self) -> windows_core::Result<super::Com::ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetField<P0>(&self, pvdata: *const core::ffi::c_void, szfieldname: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFieldNoCopy<P0>(&self, pvdata: *const core::ffi::c_void, szfieldname: P0, pvarfield: *mut windows_core::VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).GetFieldNoCopy)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), core::mem::transmute(pvarfield), ppvdatacarray).ok()
    }
    pub unsafe fn PutField<P0>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P0, pvarfield: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutField)(windows_core::Interface::as_raw(self), wflags, pvdata, szfieldname.param().abi(), core::mem::transmute(pvarfield)).ok()
    }
    pub unsafe fn PutFieldNoCopy<P0>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P0, pvarfield: *const windows_core::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).PutFieldNoCopy)(windows_core::Interface::as_raw(self), wflags, pvdata, szfieldname.param().abi(), core::mem::transmute(pvarfield)).ok()
    }
    pub unsafe fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut windows_core::BSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFieldNames)(windows_core::Interface::as_raw(self), pcnames, core::mem::transmute(rgbstrnames)).ok()
    }
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> super::super::Foundation::BOOL
    where
        P0: windows_core::Param<IRecordInfo>,
    {
        (windows_core::Interface::vtable(self).IsMatchingType)(windows_core::Interface::as_raw(self), precordinfo.param().abi())
    }
    pub unsafe fn RecordCreate(&self) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).RecordCreate)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn RecordCreateCopy(&self, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordCreateCopy)(windows_core::Interface::as_raw(self), pvsource, ppvdest).ok()
    }
    pub unsafe fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RecordDestroy)(windows_core::Interface::as_raw(self), pvrecord).ok()
    }
}
#[repr(C)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RecordInit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTypeInfo: usize,
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PutField: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub PutFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetFieldNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsMatchingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleFrameSite, ISimpleFrameSite_Vtbl, 0x742b0e01_14e6_101b_914e_00aa00300cab);
impl core::ops::Deref for ISimpleFrameSite {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISimpleFrameSite, windows_core::IUnknown);
impl ISimpleFrameSite {
    pub unsafe fn PreMessageFilter<P0, P1, P2>(&self, hwnd: P0, msg: u32, wp: P1, lp: P2, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).PreMessageFilter)(windows_core::Interface::as_raw(self), hwnd.param().abi(), msg, wp.param().abi(), lp.param().abi(), plresult, pdwcookie).ok()
    }
    pub unsafe fn PostMessageFilter<P0, P1, P2>(&self, hwnd: P0, msg: u32, wp: P1, lp: P2, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<super::super::Foundation::WPARAM>,
        P2: windows_core::Param<super::super::Foundation::LPARAM>,
    {
        (windows_core::Interface::vtable(self).PostMessageFilter)(windows_core::Interface::as_raw(self), hwnd.param().abi(), msg, wp.param().abi(), lp.param().abi(), plresult, dwcookie).ok()
    }
}
#[repr(C)]
pub struct ISimpleFrameSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT, *mut u32) -> windows_core::HRESULT,
    pub PostMessageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, super::super::Foundation::WPARAM, super::super::Foundation::LPARAM, *mut super::super::Foundation::LRESULT, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISpecifyPropertyPages, ISpecifyPropertyPages_Vtbl, 0xb196b28b_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for ISpecifyPropertyPages {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpecifyPropertyPages, windows_core::IUnknown);
impl ISpecifyPropertyPages {
    pub unsafe fn GetPages(&self) -> windows_core::Result<CAUUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISpecifyPropertyPages_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CAUUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeChangeEvents, ITypeChangeEvents_Vtbl, 0x00020410_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeChangeEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeChangeEvents, windows_core::IUnknown);
impl ITypeChangeEvents {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RequestTypeChange<P0, P1>(&self, changekind: CHANGEKIND, ptinfobefore: P0, pstrname: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::Com::ITypeInfo>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequestTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfobefore.param().abi(), pstrname.param().abi(), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AfterTypeChange<P0, P1>(&self, changekind: CHANGEKIND, ptinfoafter: P0, pstrname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::ITypeInfo>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AfterTypeChange)(windows_core::Interface::as_raw(self), changekind, ptinfoafter.param().abi(), pstrname.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ITypeChangeEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RequestTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, windows_core::PCWSTR, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RequestTypeChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AfterTypeChange: unsafe extern "system" fn(*mut core::ffi::c_void, CHANGEKIND, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AfterTypeChange: usize,
}
windows_core::imp::define_interface!(ITypeFactory, ITypeFactory_Vtbl, 0x0000002e_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeFactory, windows_core::IUnknown);
impl ITypeFactory {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFromTypeInfo<P0>(&self, ptypeinfo: P0, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::Com::ITypeInfo>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateFromTypeInfo)(windows_core::Interface::as_raw(self), ptypeinfo.param().abi(), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITypeFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFromTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFromTypeInfo: usize,
}
windows_core::imp::define_interface!(ITypeMarshal, ITypeMarshal_Vtbl, 0x0000002d_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeMarshal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeMarshal, windows_core::IUnknown);
impl ITypeMarshal {
    pub unsafe fn Size(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, &mut result__).map(|| result__)
    }
    pub unsafe fn Marshal(&self, pvtype: *const core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const core::ffi::c_void, pbuffer: &mut [u8], pcbwritten: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Marshal)(windows_core::Interface::as_raw(self), pvtype, dwdestcontext, pvdestcontext, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), pcbwritten).ok()
    }
    pub unsafe fn Unmarshal(&self, pvtype: *mut core::ffi::c_void, dwflags: u32, pbuffer: &[u8], pcbread: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unmarshal)(windows_core::Interface::as_raw(self), pvtype, dwflags, pbuffer.len().try_into().unwrap(), core::mem::transmute(pbuffer.as_ptr()), pcbread).ok()
    }
    pub unsafe fn Free(&self, pvtype: *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), pvtype).ok()
    }
}
#[repr(C)]
pub struct ITypeMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Marshal: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Unmarshal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u8, *mut u32) -> windows_core::HRESULT,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVBFormat, IVBFormat_Vtbl, 0x9849fd60_3768_101b_8d72_ae6164ffe3cf);
impl core::ops::Deref for IVBFormat {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVBFormat, windows_core::IUnknown);
impl IVBFormat {
    pub unsafe fn Format<P0>(&self, vdata: *mut windows_core::VARIANT, bstrformat: P0, lpbuffer: *mut core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Format)(windows_core::Interface::as_raw(self), core::mem::transmute(vdata), bstrformat.param().abi(), lpbuffer, cb, lcid, sfirstdayofweek, sfirstweekofyear, rcb).ok()
    }
}
#[repr(C)]
pub struct IVBFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Format: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, u16, i32, i16, u16, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVBGetControl, IVBGetControl_Vtbl, 0x40a050a0_3c31_101b_a82e_08002b2b2337);
impl core::ops::Deref for IVBGetControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVBGetControl, windows_core::IUnknown);
impl IVBGetControl {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumControls(&self, dwolecontf: OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> windows_core::Result<super::Com::IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumControls)(windows_core::Interface::as_raw(self), dwolecontf.0 as _, dwwhich, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IVBGetControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumControls: unsafe extern "system" fn(*mut core::ffi::c_void, u32, ENUM_CONTROLS_WHICH_FLAGS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumControls: usize,
}
windows_core::imp::define_interface!(IVariantChangeType, IVariantChangeType_Vtbl, 0xa6ef9862_c720_11d0_9337_00a0c90dcaa9);
impl core::ops::Deref for IVariantChangeType {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVariantChangeType, windows_core::IUnknown);
impl IVariantChangeType {
    #[cfg(feature = "Win32_System_Variant")]
    pub unsafe fn ChangeType(&self, pvardst: *mut windows_core::VARIANT, pvarsrc: *const windows_core::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangeType)(windows_core::Interface::as_raw(self), core::mem::transmute(pvardst), core::mem::transmute(pvarsrc), lcid, vtnew).ok()
    }
}
#[repr(C)]
pub struct IVariantChangeType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Variant")]
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *const core::mem::MaybeUninit<windows_core::VARIANT>, u32, super::Variant::VARENUM) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Variant"))]
    ChangeType: usize,
}
windows_core::imp::define_interface!(IViewObject, IViewObject_Vtbl, 0x0000010d_0000_0000_c000_000000000046);
impl core::ops::Deref for IViewObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObject, windows_core::IUnknown);
impl IViewObject {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn Draw<P0, P1>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: Option<*const super::Com::DVTARGETDEVICE>, hdctargetdev: P0, hdcdraw: P1, lprcbounds: Option<*const super::super::Foundation::RECTL>, lprcwbounds: Option<*const super::super::Foundation::RECTL>, pfncontinue: isize, dwcontinue: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
        P1: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, core::mem::transmute(ptd.unwrap_or(std::ptr::null())), hdctargetdev.param().abi(), hdcdraw.param().abi(), core::mem::transmute(lprcbounds.unwrap_or(std::ptr::null())), core::mem::transmute(lprcwbounds.unwrap_or(std::ptr::null())), pfncontinue, dwcontinue).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetColorSet<P0>(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, ptd: Option<*const super::Com::DVTARGETDEVICE>, hictargetdev: P0, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        (windows_core::Interface::vtable(self).GetColorSet)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, core::mem::transmute(ptd.unwrap_or(std::ptr::null())), hictargetdev.param().abi(), ppcolorset).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Freeze(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut core::ffi::c_void, pdwfreeze: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Freeze)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, pvaspect, pdwfreeze).ok()
    }
    pub unsafe fn Unfreeze(&self, dwfreeze: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unfreeze)(windows_core::Interface::as_raw(self), dwfreeze).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAdvise<P0>(&self, aspects: super::Com::DVASPECT, advf: u32, padvsink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IAdviseSink>,
    {
        (windows_core::Interface::vtable(self).SetAdvise)(windows_core::Interface::as_raw(self), aspects, advf, padvsink.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAdvise(&self, paspects: Option<*mut u32>, padvf: Option<*mut u32>, ppadvsink: *mut Option<super::Com::IAdviseSink>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetAdvise)(windows_core::Interface::as_raw(self), core::mem::transmute(paspects.unwrap_or(std::ptr::null_mut())), core::mem::transmute(padvf.unwrap_or(std::ptr::null_mut())), core::mem::transmute(ppadvsink)).ok()
    }
}
#[repr(C)]
pub struct IViewObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, i32, *mut core::ffi::c_void, *const super::Com::DVTARGETDEVICE, super::super::Graphics::Gdi::HDC, super::super::Graphics::Gdi::HDC, *const super::super::Foundation::RECTL, *const super::super::Foundation::RECTL, isize, usize) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    Draw: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetColorSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, i32, *mut core::ffi::c_void, *const super::Com::DVTARGETDEVICE, super::super::Graphics::Gdi::HDC, *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetColorSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Freeze: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, i32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Freeze: usize,
    pub Unfreeze: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAdvise: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAdvise: usize,
}
windows_core::imp::define_interface!(IViewObject2, IViewObject2_Vtbl, 0x00000127_0000_0000_c000_000000000046);
impl core::ops::Deref for IViewObject2 {
    type Target = IViewObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObject2, windows_core::IUnknown, IViewObject);
impl IViewObject2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetExtent(&self, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), dwdrawaspect, lindex, ptd, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IViewObject2_Vtbl {
    pub base__: IViewObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, i32, *const super::Com::DVTARGETDEVICE, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetExtent: usize,
}
windows_core::imp::define_interface!(IViewObjectEx, IViewObjectEx_Vtbl, 0x3af24292_0c96_11ce_a0cf_00aa00600ab8);
impl core::ops::Deref for IViewObjectEx {
    type Target = IViewObject2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IViewObjectEx, windows_core::IUnknown, IViewObject, IViewObject2);
impl IViewObjectEx {
    pub unsafe fn GetRect(&self, dwaspect: u32) -> windows_core::Result<super::super::Foundation::RECTL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRect)(windows_core::Interface::as_raw(self), dwaspect, &mut result__).map(|| result__)
    }
    pub unsafe fn GetViewStatus(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetViewStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn QueryHitPoint(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHitPoint)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, core::mem::transmute(ptlloc), lclosehint, &mut result__).map(|| result__)
    }
    pub unsafe fn QueryHitRect(&self, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHitRect)(windows_core::Interface::as_raw(self), dwaspect, prectbounds, prectloc, lclosehint, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetNaturalExtent<P0>(&self, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: P0, pextentinfo: *const DVEXTENTINFO) -> windows_core::Result<super::super::Foundation::SIZE>
    where
        P0: windows_core::Param<super::super::Graphics::Gdi::HDC>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNaturalExtent)(windows_core::Interface::as_raw(self), dwaspect, lindex, ptd, hictargetdev.param().abi(), pextentinfo, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IViewObjectEx_Vtbl {
    pub base__: IViewObject2_Vtbl,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::super::Foundation::RECTL) -> windows_core::HRESULT,
    pub GetViewStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub QueryHitPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT, super::super::Foundation::POINT, i32, *mut u32) -> windows_core::HRESULT,
    pub QueryHitRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT, *const super::super::Foundation::RECT, i32, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetNaturalExtent: unsafe extern "system" fn(*mut core::ffi::c_void, super::Com::DVASPECT, i32, *const super::Com::DVTARGETDEVICE, super::super::Graphics::Gdi::HDC, *const DVEXTENTINFO, *mut super::super::Foundation::SIZE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetNaturalExtent: usize,
}
windows_core::imp::define_interface!(IZoomEvents, IZoomEvents_Vtbl, 0x41b68150_904c_4e17_a0ba_a438182e359d);
impl core::ops::Deref for IZoomEvents {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IZoomEvents, windows_core::IUnknown);
impl IZoomEvents {
    pub unsafe fn OnZoomPercentChanged(&self, ulzoompercent: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnZoomPercentChanged)(windows_core::Interface::as_raw(self), ulzoompercent).ok()
    }
}
#[repr(C)]
pub struct IZoomEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnZoomPercentChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = ACTIVATEFLAGS(1i32);
pub const ACTIVEOBJECT_STRONG: ACTIVEOBJECT_FLAGS = ACTIVEOBJECT_FLAGS(0u32);
pub const ACTIVEOBJECT_WEAK: ACTIVEOBJECT_FLAGS = ACTIVEOBJECT_FLAGS(1u32);
pub const BINDSPEED_IMMEDIATE: BINDSPEED = BINDSPEED(3i32);
pub const BINDSPEED_INDEFINITE: BINDSPEED = BINDSPEED(1i32);
pub const BINDSPEED_MODERATE: BINDSPEED = BINDSPEED(2i32);
pub const BZ_DISABLECANCELBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(1u32);
pub const BZ_DISABLERETRYBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(4u32);
pub const BZ_DISABLESWITCHTOBUTTON: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(2u32);
pub const BZ_NOTRESPONDINGDIALOG: BUSY_DIALOG_FLAGS = BUSY_DIALOG_FLAGS(8u32);
pub const CF_BITMAP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(2u16);
pub const CF_CONVERTONLY: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(256u32);
pub const CF_DIB: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(8u16);
pub const CF_DIBV5: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(17u16);
pub const CF_DIF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(5u16);
pub const CF_DISABLEACTIVATEAS: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(64u32);
pub const CF_DISABLEDISPLAYASICON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(32u32);
pub const CF_DSPBITMAP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(130u16);
pub const CF_DSPENHMETAFILE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(142u16);
pub const CF_DSPMETAFILEPICT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(131u16);
pub const CF_DSPTEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(129u16);
pub const CF_ENHMETAFILE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(14u16);
pub const CF_GDIOBJFIRST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(768u16);
pub const CF_GDIOBJLAST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(1023u16);
pub const CF_HDROP: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(15u16);
pub const CF_HIDECHANGEICON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(128u32);
pub const CF_LOCALE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(16u16);
pub const CF_MAX: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(18u16);
pub const CF_METAFILEPICT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(3u16);
pub const CF_OEMTEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(7u16);
pub const CF_OWNERDISPLAY: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(128u16);
pub const CF_PALETTE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(9u16);
pub const CF_PENDATA: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(10u16);
pub const CF_PRIVATEFIRST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(512u16);
pub const CF_PRIVATELAST: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(767u16);
pub const CF_RIFF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(11u16);
pub const CF_SELECTACTIVATEAS: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(16u32);
pub const CF_SELECTCONVERTTO: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(8u32);
pub const CF_SETACTIVATEDEFAULT: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(4u32);
pub const CF_SETCONVERTDEFAULT: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(2u32);
pub const CF_SHOWHELPBUTTON: UI_CONVERT_FLAGS = UI_CONVERT_FLAGS(1u32);
pub const CF_SYLK: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(4u16);
pub const CF_TEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(1u16);
pub const CF_TIFF: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(6u16);
pub const CF_UNICODETEXT: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(13u16);
pub const CF_WAVE: CLIPBOARD_FORMAT = CLIPBOARD_FORMAT(12u16);
pub const CHANGEKIND_ADDMEMBER: CHANGEKIND = CHANGEKIND(0i32);
pub const CHANGEKIND_CHANGEFAILED: CHANGEKIND = CHANGEKIND(6i32);
pub const CHANGEKIND_DELETEMEMBER: CHANGEKIND = CHANGEKIND(1i32);
pub const CHANGEKIND_GENERAL: CHANGEKIND = CHANGEKIND(4i32);
pub const CHANGEKIND_INVALIDATE: CHANGEKIND = CHANGEKIND(5i32);
pub const CHANGEKIND_MAX: CHANGEKIND = CHANGEKIND(7i32);
pub const CHANGEKIND_SETDOCUMENTATION: CHANGEKIND = CHANGEKIND(3i32);
pub const CHANGEKIND_SETNAMES: CHANGEKIND = CHANGEKIND(2i32);
pub const CIF_SELECTCURRENT: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(2u32);
pub const CIF_SELECTDEFAULT: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(4u32);
pub const CIF_SELECTFROMFILE: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(8u32);
pub const CIF_SHOWHELP: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(1u32);
pub const CIF_USEICONEXE: CHANGE_ICON_FLAGS = CHANGE_ICON_FLAGS(16u32);
pub const CLSID_CColorPropPage: windows_core::GUID = windows_core::GUID::from_u128(0x0be35201_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CFontPropPage: windows_core::GUID = windows_core::GUID::from_u128(0x0be35200_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CPicturePropPage: windows_core::GUID = windows_core::GUID::from_u128(0x0be35202_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_ConvertVBX: windows_core::GUID = windows_core::GUID::from_u128(0xfb8f0822_0164_101b_84ed_08002b2ec713);
pub const CLSID_PersistPropset: windows_core::GUID = windows_core::GUID::from_u128(0xfb8f0821_0164_101b_84ed_08002b2ec713);
pub const CLSID_StdFont: windows_core::GUID = windows_core::GUID::from_u128(0x0be35203_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_StdPicture: windows_core::GUID = windows_core::GUID::from_u128(0x0be35204_8f91_11ce_9de3_00aa004bb851);
pub const CONNECT_E_ADVISELIMIT: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const CONNECT_E_CANNOTCONNECT: windows_core::HRESULT = windows_core::HRESULT(0x80040202_u32 as _);
pub const CONNECT_E_FIRST: i32 = -2147220992i32;
pub const CONNECT_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const CONNECT_E_NOCONNECTION: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const CONNECT_E_OVERRIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80040203_u32 as _);
pub const CONNECT_S_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const CONNECT_S_LAST: windows_core::HRESULT = windows_core::HRESULT(0x4020F_u32 as _);
pub const CSF_EXPLORER: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(8u32);
pub const CSF_ONLYGETSOURCE: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(4u32);
pub const CSF_SHOWHELP: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(1u32);
pub const CSF_VALIDSOURCE: CHANGE_SOURCE_FLAGS = CHANGE_SOURCE_FLAGS(2u32);
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = CTRLINFO(2i32);
pub const CTRLINFO_EATS_RETURN: CTRLINFO = CTRLINFO(1i32);
pub const DD_DEFDRAGDELAY: u32 = 200u32;
pub const DD_DEFDRAGMINDIST: u32 = 2u32;
pub const DD_DEFSCROLLDELAY: u32 = 50u32;
pub const DD_DEFSCROLLINSET: u32 = 11u32;
pub const DD_DEFSCROLLINTERVAL: u32 = 50u32;
pub const DISCARDCACHE_NOSAVE: DISCARDCACHE = DISCARDCACHE(1i32);
pub const DISCARDCACHE_SAVEIFDIRTY: DISCARDCACHE = DISCARDCACHE(0i32);
pub const DISPATCH_CONSTRUCT: u32 = 16384u32;
pub const DISPID_ABOUTBOX: i32 = -552i32;
pub const DISPID_ACCELERATOR: i32 = -543i32;
pub const DISPID_ADDITEM: i32 = -553i32;
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716i32;
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715i32;
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701i32;
pub const DISPID_AMBIENT_CHARSET: i32 = -727i32;
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725i32;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713i32;
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702i32;
pub const DISPID_AMBIENT_FONT: i32 = -703i32;
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704i32;
pub const DISPID_AMBIENT_LOCALEID: i32 = -705i32;
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706i32;
pub const DISPID_AMBIENT_PALETTE: i32 = -726i32;
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732i32;
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707i32;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711i32;
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712i32;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714i32;
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708i32;
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733i32;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728i32;
pub const DISPID_AMBIENT_UIDEAD: i32 = -710i32;
pub const DISPID_AMBIENT_USERMODE: i32 = -709i32;
pub const DISPID_APPEARANCE: i32 = -520i32;
pub const DISPID_AUTOSIZE: i32 = -500i32;
pub const DISPID_BACKCOLOR: i32 = -501i32;
pub const DISPID_BACKSTYLE: i32 = -502i32;
pub const DISPID_BORDERCOLOR: i32 = -503i32;
pub const DISPID_BORDERSTYLE: i32 = -504i32;
pub const DISPID_BORDERVISIBLE: i32 = -519i32;
pub const DISPID_BORDERWIDTH: i32 = -505i32;
pub const DISPID_CAPTION: i32 = -518i32;
pub const DISPID_CLEAR: i32 = -554i32;
pub const DISPID_CLICK: i32 = -600i32;
pub const DISPID_CLICK_VALUE: i32 = -610i32;
pub const DISPID_COLLECT: i32 = -8i32;
pub const DISPID_COLUMN: i32 = -529i32;
pub const DISPID_CONSTRUCTOR: i32 = -6i32;
pub const DISPID_DBLCLICK: i32 = -601i32;
pub const DISPID_DESTRUCTOR: i32 = -7i32;
pub const DISPID_DISPLAYSTYLE: i32 = -540i32;
pub const DISPID_DOCLICK: i32 = -551i32;
pub const DISPID_DRAWMODE: i32 = -507i32;
pub const DISPID_DRAWSTYLE: i32 = -508i32;
pub const DISPID_DRAWWIDTH: i32 = -509i32;
pub const DISPID_Delete: i32 = -801i32;
pub const DISPID_ENABLED: i32 = -514i32;
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544i32;
pub const DISPID_ERROREVENT: i32 = -608i32;
pub const DISPID_EVALUATE: i32 = -5i32;
pub const DISPID_FILLCOLOR: i32 = -510i32;
pub const DISPID_FILLSTYLE: i32 = -511i32;
pub const DISPID_FONT: i32 = -512i32;
pub const DISPID_FONT_BOLD: u32 = 3u32;
pub const DISPID_FONT_CHANGED: u32 = 9u32;
pub const DISPID_FONT_CHARSET: u32 = 8u32;
pub const DISPID_FONT_ITALIC: u32 = 4u32;
pub const DISPID_FONT_NAME: u32 = 0u32;
pub const DISPID_FONT_SIZE: u32 = 2u32;
pub const DISPID_FONT_STRIKE: u32 = 6u32;
pub const DISPID_FONT_UNDER: u32 = 5u32;
pub const DISPID_FONT_WEIGHT: u32 = 7u32;
pub const DISPID_FORECOLOR: i32 = -513i32;
pub const DISPID_GROUPNAME: i32 = -541i32;
pub const DISPID_HWND: i32 = -515i32;
pub const DISPID_IMEMODE: i32 = -542i32;
pub const DISPID_KEYDOWN: i32 = -602i32;
pub const DISPID_KEYPRESS: i32 = -603i32;
pub const DISPID_KEYUP: i32 = -604i32;
pub const DISPID_LIST: i32 = -528i32;
pub const DISPID_LISTCOUNT: i32 = -531i32;
pub const DISPID_LISTINDEX: i32 = -526i32;
pub const DISPID_MAXLENGTH: i32 = -533i32;
pub const DISPID_MOUSEDOWN: i32 = -605i32;
pub const DISPID_MOUSEICON: i32 = -522i32;
pub const DISPID_MOUSEMOVE: i32 = -606i32;
pub const DISPID_MOUSEPOINTER: i32 = -521i32;
pub const DISPID_MOUSEUP: i32 = -607i32;
pub const DISPID_MULTILINE: i32 = -537i32;
pub const DISPID_MULTISELECT: i32 = -532i32;
pub const DISPID_NEWENUM: i32 = -4i32;
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539i32;
pub const DISPID_NUMBEROFROWS: i32 = -538i32;
pub const DISPID_Name: i32 = -800i32;
pub const DISPID_Object: i32 = -802i32;
pub const DISPID_PASSWORDCHAR: i32 = -534i32;
pub const DISPID_PICTURE: i32 = -523i32;
pub const DISPID_PICT_HANDLE: u32 = 0u32;
pub const DISPID_PICT_HEIGHT: u32 = 5u32;
pub const DISPID_PICT_HPAL: u32 = 2u32;
pub const DISPID_PICT_RENDER: u32 = 6u32;
pub const DISPID_PICT_TYPE: u32 = 3u32;
pub const DISPID_PICT_WIDTH: u32 = 4u32;
pub const DISPID_PROPERTYPUT: i32 = -3i32;
pub const DISPID_Parent: i32 = -803i32;
pub const DISPID_READYSTATE: i32 = -525i32;
pub const DISPID_READYSTATECHANGE: i32 = -609i32;
pub const DISPID_REFRESH: i32 = -550i32;
pub const DISPID_REMOVEITEM: i32 = -555i32;
pub const DISPID_RIGHTTOLEFT: i32 = -611i32;
pub const DISPID_SCROLLBARS: i32 = -535i32;
pub const DISPID_SELECTED: i32 = -527i32;
pub const DISPID_SELLENGTH: i32 = -548i32;
pub const DISPID_SELSTART: i32 = -547i32;
pub const DISPID_SELTEXT: i32 = -546i32;
pub const DISPID_STARTENUM: i32 = -1i32;
pub const DISPID_TABKEYBEHAVIOR: i32 = -545i32;
pub const DISPID_TABSTOP: i32 = -516i32;
pub const DISPID_TEXT: i32 = -517i32;
pub const DISPID_THIS: i32 = -613i32;
pub const DISPID_TOPTOBOTTOM: i32 = -612i32;
pub const DISPID_UNKNOWN: i32 = -1i32;
pub const DISPID_VALID: i32 = -524i32;
pub const DISPID_VALUE: u32 = 0u32;
pub const DISPID_WORDWRAP: i32 = -536i32;
pub const DOCMISC_CANCREATEMULTIPLEVIEWS: DOCMISC = DOCMISC(1i32);
pub const DOCMISC_CANTOPENEDIT: DOCMISC = DOCMISC(4i32);
pub const DOCMISC_NOFILESUPPORT: DOCMISC = DOCMISC(8i32);
pub const DOCMISC_SUPPORTCOMPLEXRECTANGLES: DOCMISC = DOCMISC(2i32);
pub const DROPEFFECT_COPY: DROPEFFECT = DROPEFFECT(1u32);
pub const DROPEFFECT_LINK: DROPEFFECT = DROPEFFECT(4u32);
pub const DROPEFFECT_MOVE: DROPEFFECT = DROPEFFECT(2u32);
pub const DROPEFFECT_NONE: DROPEFFECT = DROPEFFECT(0u32);
pub const DROPEFFECT_SCROLL: DROPEFFECT = DROPEFFECT(2147483648u32);
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = DVASPECTINFOFLAG(1i32);
pub const DVEXTENT_CONTENT: DVEXTENTMODE = DVEXTENTMODE(0i32);
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = DVEXTENTMODE(1i32);
pub const ELF_DISABLECANCELLINK: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(16u32);
pub const ELF_DISABLECHANGESOURCE: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(8u32);
pub const ELF_DISABLEOPENSOURCE: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(4u32);
pub const ELF_DISABLEUPDATENOW: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(2u32);
pub const ELF_SHOWHELP: EDIT_LINKS_FLAGS = EDIT_LINKS_FLAGS(1u32);
pub const EMBDHLP_CREATENOW: EMBDHLP_FLAGS = EMBDHLP_FLAGS(0u32);
pub const EMBDHLP_DELAYCREATE: EMBDHLP_FLAGS = EMBDHLP_FLAGS(65536u32);
pub const EMBDHLP_INPROC_HANDLER: EMBDHLP_FLAGS = EMBDHLP_FLAGS(0u32);
pub const EMBDHLP_INPROC_SERVER: EMBDHLP_FLAGS = EMBDHLP_FLAGS(1u32);
pub const GCW_WCH_SIBLING: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1u32);
pub const GC_WCH_ALL: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(4u32);
pub const GC_WCH_CONTAINED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(3u32);
pub const GC_WCH_CONTAINER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(2u32);
pub const GC_WCH_FONLYAFTER: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(268435456u32);
pub const GC_WCH_FONLYBEFORE: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(536870912u32);
pub const GC_WCH_FREVERSEDIR: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(134217728u32);
pub const GC_WCH_FSELECTED: ENUM_CONTROLS_WHICH_FLAGS = ENUM_CONTROLS_WHICH_FLAGS(1073741824u32);
pub const GC_WCH_SIBLING: i32 = 1i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = GUIDKIND(1i32);
pub const GUID_CHECKVALUEEXCLUSIVE: windows_core::GUID = windows_core::GUID::from_u128(0x6650430c_be0f_101a_8bbb_00aa00300cab);
pub const GUID_COLOR: windows_core::GUID = windows_core::GUID::from_u128(0x66504301_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTBOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6650430f_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTITALIC: windows_core::GUID = windows_core::GUID::from_u128(0x66504310_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTNAME: windows_core::GUID = windows_core::GUID::from_u128(0x6650430d_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSIZE: windows_core::GUID = windows_core::GUID::from_u128(0x6650430e_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSTRIKETHROUGH: windows_core::GUID = windows_core::GUID::from_u128(0x66504312_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTUNDERSCORE: windows_core::GUID = windows_core::GUID::from_u128(0x66504311_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HANDLE: windows_core::GUID = windows_core::GUID::from_u128(0x66504313_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HIMETRIC: windows_core::GUID = windows_core::GUID::from_u128(0x66504300_be0f_101a_8bbb_00aa00300cab);
pub const GUID_OPTIONVALUEEXCLUSIVE: windows_core::GUID = windows_core::GUID::from_u128(0x6650430b_be0f_101a_8bbb_00aa00300cab);
pub const GUID_TRISTATE: windows_core::GUID = windows_core::GUID::from_u128(0x6650430a_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOS: windows_core::GUID = windows_core::GUID::from_u128(0x66504306_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOSPIXEL: windows_core::GUID = windows_core::GUID::from_u128(0x66504302_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZE: windows_core::GUID = windows_core::GUID::from_u128(0x66504308_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZEPIXEL: windows_core::GUID = windows_core::GUID::from_u128(0x66504304_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOS: windows_core::GUID = windows_core::GUID::from_u128(0x66504307_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOSPIXEL: windows_core::GUID = windows_core::GUID::from_u128(0x66504303_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZE: windows_core::GUID = windows_core::GUID::from_u128(0x66504309_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZEPIXEL: windows_core::GUID = windows_core::GUID::from_u128(0x66504305_be0f_101a_8bbb_00aa00300cab);
pub const HITRESULT_CLOSE: HITRESULT = HITRESULT(2i32);
pub const HITRESULT_HIT: HITRESULT = HITRESULT(3i32);
pub const HITRESULT_OUTSIDE: HITRESULT = HITRESULT(0i32);
pub const HITRESULT_TRANSPARENT: HITRESULT = HITRESULT(1i32);
pub const IDC_BZ_ICON: u32 = 601u32;
pub const IDC_BZ_MESSAGE1: u32 = 602u32;
pub const IDC_BZ_RETRY: u32 = 600u32;
pub const IDC_BZ_SWITCHTO: u32 = 604u32;
pub const IDC_CI_BROWSE: u32 = 130u32;
pub const IDC_CI_CURRENT: u32 = 121u32;
pub const IDC_CI_CURRENTICON: u32 = 122u32;
pub const IDC_CI_DEFAULT: u32 = 123u32;
pub const IDC_CI_DEFAULTICON: u32 = 124u32;
pub const IDC_CI_FROMFILE: u32 = 125u32;
pub const IDC_CI_FROMFILEEDIT: u32 = 126u32;
pub const IDC_CI_GROUP: u32 = 120u32;
pub const IDC_CI_ICONDISPLAY: u32 = 131u32;
pub const IDC_CI_ICONLIST: u32 = 127u32;
pub const IDC_CI_LABEL: u32 = 128u32;
pub const IDC_CI_LABELEDIT: u32 = 129u32;
pub const IDC_CV_ACTIVATEAS: u32 = 156u32;
pub const IDC_CV_ACTIVATELIST: u32 = 154u32;
pub const IDC_CV_CHANGEICON: u32 = 153u32;
pub const IDC_CV_CONVERTLIST: u32 = 158u32;
pub const IDC_CV_CONVERTTO: u32 = 155u32;
pub const IDC_CV_DISPLAYASICON: u32 = 152u32;
pub const IDC_CV_ICONDISPLAY: u32 = 165u32;
pub const IDC_CV_OBJECTTYPE: u32 = 150u32;
pub const IDC_CV_RESULTTEXT: u32 = 157u32;
pub const IDC_EL_AUTOMATIC: u32 = 202u32;
pub const IDC_EL_CANCELLINK: u32 = 209u32;
pub const IDC_EL_CHANGESOURCE: u32 = 201u32;
pub const IDC_EL_COL1: u32 = 220u32;
pub const IDC_EL_COL2: u32 = 221u32;
pub const IDC_EL_COL3: u32 = 222u32;
pub const IDC_EL_LINKSLISTBOX: u32 = 206u32;
pub const IDC_EL_LINKSOURCE: u32 = 216u32;
pub const IDC_EL_LINKTYPE: u32 = 217u32;
pub const IDC_EL_MANUAL: u32 = 212u32;
pub const IDC_EL_OPENSOURCE: u32 = 211u32;
pub const IDC_EL_UPDATENOW: u32 = 210u32;
pub const IDC_GP_CONVERT: u32 = 1013u32;
pub const IDC_GP_OBJECTICON: u32 = 1014u32;
pub const IDC_GP_OBJECTLOCATION: u32 = 1022u32;
pub const IDC_GP_OBJECTNAME: u32 = 1009u32;
pub const IDC_GP_OBJECTSIZE: u32 = 1011u32;
pub const IDC_GP_OBJECTTYPE: u32 = 1010u32;
pub const IDC_IO_ADDCONTROL: u32 = 2115u32;
pub const IDC_IO_CHANGEICON: u32 = 2105u32;
pub const IDC_IO_CONTROLTYPELIST: u32 = 2116u32;
pub const IDC_IO_CREATEFROMFILE: u32 = 2101u32;
pub const IDC_IO_CREATENEW: u32 = 2100u32;
pub const IDC_IO_DISPLAYASICON: u32 = 2104u32;
pub const IDC_IO_FILE: u32 = 2106u32;
pub const IDC_IO_FILEDISPLAY: u32 = 2107u32;
pub const IDC_IO_FILETEXT: u32 = 2112u32;
pub const IDC_IO_FILETYPE: u32 = 2113u32;
pub const IDC_IO_ICONDISPLAY: u32 = 2110u32;
pub const IDC_IO_INSERTCONTROL: u32 = 2114u32;
pub const IDC_IO_LINKFILE: u32 = 2102u32;
pub const IDC_IO_OBJECTTYPELIST: u32 = 2103u32;
pub const IDC_IO_OBJECTTYPETEXT: u32 = 2111u32;
pub const IDC_IO_RESULTIMAGE: u32 = 2108u32;
pub const IDC_IO_RESULTTEXT: u32 = 2109u32;
pub const IDC_LP_AUTOMATIC: u32 = 1016u32;
pub const IDC_LP_BREAKLINK: u32 = 1008u32;
pub const IDC_LP_CHANGESOURCE: u32 = 1015u32;
pub const IDC_LP_DATE: u32 = 1018u32;
pub const IDC_LP_LINKSOURCE: u32 = 1012u32;
pub const IDC_LP_MANUAL: u32 = 1017u32;
pub const IDC_LP_OPENSOURCE: u32 = 1006u32;
pub const IDC_LP_TIME: u32 = 1019u32;
pub const IDC_LP_UPDATENOW: u32 = 1007u32;
pub const IDC_OLEUIHELP: u32 = 99u32;
pub const IDC_PS_CHANGEICON: u32 = 508u32;
pub const IDC_PS_DISPLAYASICON: u32 = 506u32;
pub const IDC_PS_DISPLAYLIST: u32 = 505u32;
pub const IDC_PS_ICONDISPLAY: u32 = 507u32;
pub const IDC_PS_PASTE: u32 = 500u32;
pub const IDC_PS_PASTELINK: u32 = 501u32;
pub const IDC_PS_PASTELINKLIST: u32 = 504u32;
pub const IDC_PS_PASTELIST: u32 = 503u32;
pub const IDC_PS_RESULTIMAGE: u32 = 509u32;
pub const IDC_PS_RESULTTEXT: u32 = 510u32;
pub const IDC_PS_SOURCETEXT: u32 = 502u32;
pub const IDC_PU_CONVERT: u32 = 902u32;
pub const IDC_PU_ICON: u32 = 908u32;
pub const IDC_PU_LINKS: u32 = 900u32;
pub const IDC_PU_TEXT: u32 = 901u32;
pub const IDC_UL_METER: u32 = 1029u32;
pub const IDC_UL_PERCENT: u32 = 1031u32;
pub const IDC_UL_PROGRESS: u32 = 1032u32;
pub const IDC_UL_STOP: u32 = 1030u32;
pub const IDC_VP_ASICON: u32 = 1003u32;
pub const IDC_VP_CHANGEICON: u32 = 1001u32;
pub const IDC_VP_EDITABLE: u32 = 1002u32;
pub const IDC_VP_ICONDISPLAY: u32 = 1021u32;
pub const IDC_VP_PERCENT: u32 = 1000u32;
pub const IDC_VP_RELATIVE: u32 = 1005u32;
pub const IDC_VP_RESULTIMAGE: u32 = 1033u32;
pub const IDC_VP_SCALETXT: u32 = 1034u32;
pub const IDC_VP_SPIN: u32 = 1006u32;
pub const IDD_BUSY: u32 = 1006u32;
pub const IDD_CANNOTUPDATELINK: u32 = 1008u32;
pub const IDD_CHANGEICON: u32 = 1001u32;
pub const IDD_CHANGEICONBROWSE: u32 = 1011u32;
pub const IDD_CHANGESOURCE: u32 = 1009u32;
pub const IDD_CHANGESOURCE4: u32 = 1013u32;
pub const IDD_CONVERT: u32 = 1002u32;
pub const IDD_CONVERT4: u32 = 1103u32;
pub const IDD_CONVERTONLY: u32 = 1012u32;
pub const IDD_CONVERTONLY4: u32 = 1104u32;
pub const IDD_EDITLINKS: u32 = 1004u32;
pub const IDD_EDITLINKS4: u32 = 1105u32;
pub const IDD_GNRLPROPS: u32 = 1100u32;
pub const IDD_GNRLPROPS4: u32 = 1106u32;
pub const IDD_INSERTFILEBROWSE: u32 = 1010u32;
pub const IDD_INSERTOBJECT: u32 = 1000u32;
pub const IDD_LINKPROPS: u32 = 1102u32;
pub const IDD_LINKPROPS4: u32 = 1107u32;
pub const IDD_LINKSOURCEUNAVAILABLE: u32 = 1020u32;
pub const IDD_LINKTYPECHANGED: u32 = 1022u32;
pub const IDD_LINKTYPECHANGEDA: u32 = 1026u32;
pub const IDD_LINKTYPECHANGEDW: u32 = 1022u32;
pub const IDD_OUTOFMEMORY: u32 = 1024u32;
pub const IDD_PASTESPECIAL: u32 = 1003u32;
pub const IDD_PASTESPECIAL4: u32 = 1108u32;
pub const IDD_SERVERNOTFOUND: u32 = 1023u32;
pub const IDD_SERVERNOTREG: u32 = 1021u32;
pub const IDD_SERVERNOTREGA: u32 = 1025u32;
pub const IDD_SERVERNOTREGW: u32 = 1021u32;
pub const IDD_UPDATELINKS: u32 = 1007u32;
pub const IDD_VIEWPROPS: u32 = 1101u32;
pub const ID_BROWSE_ADDCONTROL: u32 = 3u32;
pub const ID_BROWSE_CHANGEICON: u32 = 1u32;
pub const ID_BROWSE_CHANGESOURCE: u32 = 4u32;
pub const ID_BROWSE_INSERTFILE: u32 = 2u32;
pub const ID_DEFAULTINST: i32 = -2i32;
pub const IGNOREMIME_PROMPT: IGNOREMIME = IGNOREMIME(1i32);
pub const IGNOREMIME_TEXT: IGNOREMIME = IGNOREMIME(2i32);
pub const INSTALL_SCOPE_INVALID: u32 = 0u32;
pub const INSTALL_SCOPE_MACHINE: u32 = 1u32;
pub const INSTALL_SCOPE_USER: u32 = 2u32;
pub const IOF_CHECKDISPLAYASICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(16u32);
pub const IOF_CHECKLINK: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(8u32);
pub const IOF_CREATEFILEOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(64u32);
pub const IOF_CREATELINKOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(128u32);
pub const IOF_CREATENEWOBJECT: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(32u32);
pub const IOF_DISABLEDISPLAYASICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(1024u32);
pub const IOF_DISABLELINK: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(256u32);
pub const IOF_HIDECHANGEICON: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(2048u32);
pub const IOF_SELECTCREATECONTROL: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(8192u32);
pub const IOF_SELECTCREATEFROMFILE: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(4u32);
pub const IOF_SELECTCREATENEW: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(2u32);
pub const IOF_SHOWHELP: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(1u32);
pub const IOF_SHOWINSERTCONTROL: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(4096u32);
pub const IOF_VERIFYSERVERSEXIST: INSERT_OBJECT_FLAGS = INSERT_OBJECT_FLAGS(512u32);
pub const KEYMOD_ALT: KEYMODIFIERS = KEYMODIFIERS(4u32);
pub const KEYMOD_CONTROL: KEYMODIFIERS = KEYMODIFIERS(2u32);
pub const KEYMOD_SHIFT: KEYMODIFIERS = KEYMODIFIERS(1u32);
pub const LIBFLAG_FCONTROL: LIBFLAGS = LIBFLAGS(2i32);
pub const LIBFLAG_FHASDISKIMAGE: LIBFLAGS = LIBFLAGS(8i32);
pub const LIBFLAG_FHIDDEN: LIBFLAGS = LIBFLAGS(4i32);
pub const LIBFLAG_FRESTRICTED: LIBFLAGS = LIBFLAGS(1i32);
pub const LOAD_TLB_AS_32BIT: u32 = 32u32;
pub const LOAD_TLB_AS_64BIT: u32 = 64u32;
pub const LOCALE_USE_NLS: u32 = 268435456u32;
pub const LP_COLOR: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(4u32);
pub const LP_DEFAULT: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(0u32);
pub const LP_MONOCHROME: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(1u32);
pub const LP_VGACOLOR: LOAD_PICTURE_FLAGS = LOAD_PICTURE_FLAGS(2u32);
pub const MEDIAPLAYBACK_PAUSE: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(1i32);
pub const MEDIAPLAYBACK_PAUSE_AND_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(2i32);
pub const MEDIAPLAYBACK_RESUME: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(0i32);
pub const MEDIAPLAYBACK_RESUME_FROM_SUSPEND: MEDIAPLAYBACK_STATE = MEDIAPLAYBACK_STATE(3i32);
pub const MEMBERID_NIL: i32 = -1i32;
pub const MK_ALT: u32 = 32u32;
pub const MSOCMDERR_E_CANCELED: i32 = -2147221245i32;
pub const MSOCMDERR_E_DISABLED: i32 = -2147221247i32;
pub const MSOCMDERR_E_FIRST: i32 = -2147221248i32;
pub const MSOCMDERR_E_NOHELP: i32 = -2147221246i32;
pub const MSOCMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const MSOCMDERR_E_UNKNOWNGROUP: i32 = -2147221244i32;
pub const MULTICLASSINFO_GETIIDPRIMARY: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(4u32);
pub const MULTICLASSINFO_GETIIDSOURCE: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(8u32);
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(2u32);
pub const MULTICLASSINFO_GETTYPEINFO: MULTICLASSINFO_FLAGS = MULTICLASSINFO_FLAGS(1u32);
pub const NUMPRS_CURRENCY: NUMPARSE_FLAGS = NUMPARSE_FLAGS(1024u32);
pub const NUMPRS_DECIMAL: NUMPARSE_FLAGS = NUMPARSE_FLAGS(256u32);
pub const NUMPRS_EXPONENT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(2048u32);
pub const NUMPRS_HEX_OCT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(64u32);
pub const NUMPRS_INEXACT: NUMPARSE_FLAGS = NUMPARSE_FLAGS(131072u32);
pub const NUMPRS_LEADING_MINUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(16u32);
pub const NUMPRS_LEADING_PLUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(4u32);
pub const NUMPRS_LEADING_WHITE: NUMPARSE_FLAGS = NUMPARSE_FLAGS(1u32);
pub const NUMPRS_NEG: NUMPARSE_FLAGS = NUMPARSE_FLAGS(65536u32);
pub const NUMPRS_PARENS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(128u32);
pub const NUMPRS_STD: NUMPARSE_FLAGS = NUMPARSE_FLAGS(8191u32);
pub const NUMPRS_THOUSANDS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(512u32);
pub const NUMPRS_TRAILING_MINUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(32u32);
pub const NUMPRS_TRAILING_PLUS: NUMPARSE_FLAGS = NUMPARSE_FLAGS(8u32);
pub const NUMPRS_TRAILING_WHITE: NUMPARSE_FLAGS = NUMPARSE_FLAGS(2u32);
pub const NUMPRS_USE_ALL: NUMPARSE_FLAGS = NUMPARSE_FLAGS(4096u32);
pub const OCM__BASE: u32 = 8192u32;
pub const OF_GET: u32 = 2u32;
pub const OF_HANDLER: u32 = 4u32;
pub const OF_SET: u32 = 1u32;
pub const OLECLOSE_NOSAVE: OLECLOSE = OLECLOSE(1i32);
pub const OLECLOSE_PROMPTSAVE: OLECLOSE = OLECLOSE(2i32);
pub const OLECLOSE_SAVEIFDIRTY: OLECLOSE = OLECLOSE(0i32);
pub const OLECMDARGINDEX_ACTIVEXINSTALL_CLSID: u32 = 2u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_DISPLAYNAME: u32 = 1u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_INSTALLSCOPE: u32 = 3u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_PUBLISHER: u32 = 0u32;
pub const OLECMDARGINDEX_ACTIVEXINSTALL_SOURCEURL: u32 = 4u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_HWND: u32 = 0u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_X: u32 = 1u32;
pub const OLECMDARGINDEX_SHOWPAGEACTIONMENU_Y: u32 = 2u32;
pub const OLECMDERR_E_CANCELED: windows_core::HRESULT = windows_core::HRESULT(0x80040103_u32 as _);
pub const OLECMDERR_E_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80040101_u32 as _);
pub const OLECMDERR_E_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x80040100_u32 as _);
pub const OLECMDERR_E_NOHELP: windows_core::HRESULT = windows_core::HRESULT(0x80040102_u32 as _);
pub const OLECMDERR_E_NOTSUPPORTED: i32 = -2147221248i32;
pub const OLECMDERR_E_UNKNOWNGROUP: windows_core::HRESULT = windows_core::HRESULT(0x80040104_u32 as _);
pub const OLECMDEXECOPT_DODEFAULT: OLECMDEXECOPT = OLECMDEXECOPT(0i32);
pub const OLECMDEXECOPT_DONTPROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(2i32);
pub const OLECMDEXECOPT_PROMPTUSER: OLECMDEXECOPT = OLECMDEXECOPT(1i32);
pub const OLECMDEXECOPT_SHOWHELP: OLECMDEXECOPT = OLECMDEXECOPT(3i32);
pub const OLECMDF_DEFHIDEONCTXTMENU: OLECMDF = OLECMDF(32i32);
pub const OLECMDF_ENABLED: OLECMDF = OLECMDF(2i32);
pub const OLECMDF_INVISIBLE: OLECMDF = OLECMDF(16i32);
pub const OLECMDF_LATCHED: OLECMDF = OLECMDF(4i32);
pub const OLECMDF_NINCHED: OLECMDF = OLECMDF(8i32);
pub const OLECMDF_SUPPORTED: OLECMDF = OLECMDF(1i32);
pub const OLECMDIDF_BROWSERSTATE_BLOCKEDVERSION: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(64i32);
pub const OLECMDIDF_BROWSERSTATE_DESKTOPHTMLDIALOG: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(32i32);
pub const OLECMDIDF_BROWSERSTATE_EXTENSIONSOFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(1i32);
pub const OLECMDIDF_BROWSERSTATE_IESECURITY: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(2i32);
pub const OLECMDIDF_BROWSERSTATE_PROTECTEDMODE_OFF: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(4i32);
pub const OLECMDIDF_BROWSERSTATE_REQUIRESACTIVEX: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(16i32);
pub const OLECMDIDF_BROWSERSTATE_RESET: OLECMDID_BROWSERSTATEFLAG = OLECMDID_BROWSERSTATEFLAG(8i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOLAYOUT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(16i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOPERSIST: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(1i32);
pub const OLECMDIDF_OPTICAL_ZOOM_NOTRANSIENT: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(32i32);
pub const OLECMDIDF_OPTICAL_ZOOM_RELOADFORNEWTAB: OLECMDID_OPTICAL_ZOOMFLAG = OLECMDID_OPTICAL_ZOOMFLAG(64i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXDISALLOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXTRUSTFAIL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUNSAFE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(262144i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEXUSERDISABLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8i32);
pub const OLECMDIDF_PAGEACTION_ACTIVEX_EPM_INCOMPATIBLE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16777216i32);
pub const OLECMDIDF_PAGEACTION_EXTENSION_COMPAT_BLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(268435456i32);
pub const OLECMDIDF_PAGEACTION_FILEDOWNLOAD: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1i32);
pub const OLECMDIDF_PAGEACTION_GENERIC_STATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1073741824i32);
pub const OLECMDIDF_PAGEACTION_INTRANETZONEREQUEST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2097152i32);
pub const OLECMDIDF_PAGEACTION_INVALID_CERT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1048576i32);
pub const OLECMDIDF_PAGEACTION_LOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(128i32);
pub const OLECMDIDF_PAGEACTION_MIMETEXTPLAIN: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(256i32);
pub const OLECMDIDF_PAGEACTION_MIXEDCONTENT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(524288i32);
pub const OLECMDIDF_PAGEACTION_NORESETACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(536870912i32);
pub const OLECMDIDF_PAGEACTION_POPUPALLOWED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(65536i32);
pub const OLECMDIDF_PAGEACTION_POPUPWINDOW: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(64i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNDENY: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(32768i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8192i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4096i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(1024i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(16384i32);
pub const OLECMDIDF_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(2048i32);
pub const OLECMDIDF_PAGEACTION_RESET: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(-2147483648i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXINSTALL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(512i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTNAVIGATE_ACTIVEXUSERAPPROVAL: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(33554432i32);
pub const OLECMDIDF_PAGEACTION_SCRIPTPROMPT: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(131072i32);
pub const OLECMDIDF_PAGEACTION_SPOOFABLEIDNHOST: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(8388608i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(67108864i32);
pub const OLECMDIDF_PAGEACTION_WPCBLOCKED_ACTIVEX: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(134217728i32);
pub const OLECMDIDF_PAGEACTION_XSSFILTERED: OLECMDID_PAGEACTIONFLAG = OLECMDID_PAGEACTIONFLAG(4194304i32);
pub const OLECMDIDF_REFRESH_CLEARUSERINPUT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4096i32);
pub const OLECMDIDF_REFRESH_COMPLETELY: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(3i32);
pub const OLECMDIDF_REFRESH_CONTINUE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2i32);
pub const OLECMDIDF_REFRESH_IFEXPIRED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1i32);
pub const OLECMDIDF_REFRESH_LEVELMASK: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(255i32);
pub const OLECMDIDF_REFRESH_NORMAL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(0i32);
pub const OLECMDIDF_REFRESH_NO_CACHE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ACTIVEXINSTALL: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(65536i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_ALLOW_VERSION: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(134217728i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_FILEDOWNLOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(131072i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_INVALID_CERT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(67108864i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_LOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(262144i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_MIXEDCONTENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(33554432i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_POPUPWINDOW: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(524288i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTERNET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8388608i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNINTRANET: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(4194304i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNLOCALMACHINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(1048576i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNRESTRICTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16777216i32);
pub const OLECMDIDF_REFRESH_PAGEACTION_PROTLOCKDOWNTRUSTED: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(2097152i32);
pub const OLECMDIDF_REFRESH_PROMPTIFOFFLINE: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(8192i32);
pub const OLECMDIDF_REFRESH_RELOAD: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(5i32);
pub const OLECMDIDF_REFRESH_SKIPBEFOREUNLOADEVENT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(32768i32);
pub const OLECMDIDF_REFRESH_THROUGHSCRIPT: OLECMDID_REFRESHFLAG = OLECMDID_REFRESHFLAG(16384i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(2i32);
pub const OLECMDIDF_VIEWPORTMODE_EXCLUDE_VISUAL_BOTTOM_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(131072i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(1i32);
pub const OLECMDIDF_VIEWPORTMODE_FIXED_LAYOUT_WIDTH_VALID: OLECMDID_VIEWPORT_MODE_FLAG = OLECMDID_VIEWPORT_MODE_FLAG(65536i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(2i32);
pub const OLECMDIDF_WINDOWSTATE_ENABLED_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(131072i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(1i32);
pub const OLECMDIDF_WINDOWSTATE_USERVISIBLE_VALID: OLECMDID_WINDOWSTATE_FLAG = OLECMDID_WINDOWSTATE_FLAG(65536i32);
pub const OLECMDID_ACTIVEXINSTALLSCOPE: OLECMDID = OLECMDID(66i32);
pub const OLECMDID_ADDTRAVELENTRY: OLECMDID = OLECMDID(60i32);
pub const OLECMDID_ALLOWUILESSSAVEAS: OLECMDID = OLECMDID(46i32);
pub const OLECMDID_CLEARSELECTION: OLECMDID = OLECMDID(18i32);
pub const OLECMDID_CLOSE: OLECMDID = OLECMDID(45i32);
pub const OLECMDID_COPY: OLECMDID = OLECMDID(12i32);
pub const OLECMDID_CUT: OLECMDID = OLECMDID(11i32);
pub const OLECMDID_DELETE: OLECMDID = OLECMDID(33i32);
pub const OLECMDID_DONTDOWNLOADCSS: OLECMDID = OLECMDID(47i32);
pub const OLECMDID_ENABLE_INTERACTION: OLECMDID = OLECMDID(36i32);
pub const OLECMDID_ENABLE_VISIBILITY: OLECMDID = OLECMDID(77i32);
pub const OLECMDID_EXITFULLSCREEN: OLECMDID = OLECMDID(81i32);
pub const OLECMDID_FIND: OLECMDID = OLECMDID(32i32);
pub const OLECMDID_FOCUSVIEWCONTROLS: OLECMDID = OLECMDID(57i32);
pub const OLECMDID_FOCUSVIEWCONTROLSQUERY: OLECMDID = OLECMDID(58i32);
pub const OLECMDID_GETPRINTTEMPLATE: OLECMDID = OLECMDID(52i32);
pub const OLECMDID_GETUSERSCALABLE: OLECMDID = OLECMDID(75i32);
pub const OLECMDID_GETZOOMRANGE: OLECMDID = OLECMDID(20i32);
pub const OLECMDID_HIDETOOLBARS: OLECMDID = OLECMDID(24i32);
pub const OLECMDID_HTTPEQUIV: OLECMDID = OLECMDID(34i32);
pub const OLECMDID_HTTPEQUIV_DONE: OLECMDID = OLECMDID(35i32);
pub const OLECMDID_LAYOUT_VIEWPORT_WIDTH: OLECMDID = OLECMDID(71i32);
pub const OLECMDID_MEDIA_PLAYBACK: OLECMDID = OLECMDID(78i32);
pub const OLECMDID_NEW: OLECMDID = OLECMDID(2i32);
pub const OLECMDID_ONBEFOREUNLOAD: OLECMDID = OLECMDID(83i32);
pub const OLECMDID_ONTOOLBARACTIVATED: OLECMDID = OLECMDID(31i32);
pub const OLECMDID_ONUNLOAD: OLECMDID = OLECMDID(37i32);
pub const OLECMDID_OPEN: OLECMDID = OLECMDID(1i32);
pub const OLECMDID_OPTICAL_GETZOOMRANGE: OLECMDID = OLECMDID(64i32);
pub const OLECMDID_OPTICAL_ZOOM: OLECMDID = OLECMDID(63i32);
pub const OLECMDID_PAGEACTIONBLOCKED: OLECMDID = OLECMDID(55i32);
pub const OLECMDID_PAGEACTIONUIQUERY: OLECMDID = OLECMDID(56i32);
pub const OLECMDID_PAGEAVAILABLE: OLECMDID = OLECMDID(74i32);
pub const OLECMDID_PAGESETUP: OLECMDID = OLECMDID(8i32);
pub const OLECMDID_PASTE: OLECMDID = OLECMDID(13i32);
pub const OLECMDID_PASTESPECIAL: OLECMDID = OLECMDID(14i32);
pub const OLECMDID_POPSTATEEVENT: OLECMDID = OLECMDID(69i32);
pub const OLECMDID_PREREFRESH: OLECMDID = OLECMDID(39i32);
pub const OLECMDID_PRINT: OLECMDID = OLECMDID(6i32);
pub const OLECMDID_PRINT2: OLECMDID = OLECMDID(49i32);
pub const OLECMDID_PRINTPREVIEW: OLECMDID = OLECMDID(7i32);
pub const OLECMDID_PRINTPREVIEW2: OLECMDID = OLECMDID(50i32);
pub const OLECMDID_PROPERTIES: OLECMDID = OLECMDID(10i32);
pub const OLECMDID_PROPERTYBAG2: OLECMDID = OLECMDID(38i32);
pub const OLECMDID_REDO: OLECMDID = OLECMDID(16i32);
pub const OLECMDID_REFRESH: OLECMDID = OLECMDID(22i32);
pub const OLECMDID_SAVE: OLECMDID = OLECMDID(3i32);
pub const OLECMDID_SAVEAS: OLECMDID = OLECMDID(4i32);
pub const OLECMDID_SAVECOPYAS: OLECMDID = OLECMDID(5i32);
pub const OLECMDID_SCROLLCOMPLETE: OLECMDID = OLECMDID(82i32);
pub const OLECMDID_SELECTALL: OLECMDID = OLECMDID(17i32);
pub const OLECMDID_SETDOWNLOADSTATE: OLECMDID = OLECMDID(29i32);
pub const OLECMDID_SETFAVICON: OLECMDID = OLECMDID(79i32);
pub const OLECMDID_SETPRINTTEMPLATE: OLECMDID = OLECMDID(51i32);
pub const OLECMDID_SETPROGRESSMAX: OLECMDID = OLECMDID(25i32);
pub const OLECMDID_SETPROGRESSPOS: OLECMDID = OLECMDID(26i32);
pub const OLECMDID_SETPROGRESSTEXT: OLECMDID = OLECMDID(27i32);
pub const OLECMDID_SETTITLE: OLECMDID = OLECMDID(28i32);
pub const OLECMDID_SET_HOST_FULLSCREENMODE: OLECMDID = OLECMDID(80i32);
pub const OLECMDID_SHOWFIND: OLECMDID = OLECMDID(42i32);
pub const OLECMDID_SHOWMESSAGE: OLECMDID = OLECMDID(41i32);
pub const OLECMDID_SHOWMESSAGE_BLOCKABLE: OLECMDID = OLECMDID(84i32);
pub const OLECMDID_SHOWPAGEACTIONMENU: OLECMDID = OLECMDID(59i32);
pub const OLECMDID_SHOWPAGESETUP: OLECMDID = OLECMDID(43i32);
pub const OLECMDID_SHOWPRINT: OLECMDID = OLECMDID(44i32);
pub const OLECMDID_SHOWSCRIPTERROR: OLECMDID = OLECMDID(40i32);
pub const OLECMDID_SHOWTASKDLG: OLECMDID = OLECMDID(68i32);
pub const OLECMDID_SHOWTASKDLG_BLOCKABLE: OLECMDID = OLECMDID(85i32);
pub const OLECMDID_SPELL: OLECMDID = OLECMDID(9i32);
pub const OLECMDID_STOP: OLECMDID = OLECMDID(23i32);
pub const OLECMDID_STOPDOWNLOAD: OLECMDID = OLECMDID(30i32);
pub const OLECMDID_UNDO: OLECMDID = OLECMDID(15i32);
pub const OLECMDID_UPDATEBACKFORWARDSTATE: OLECMDID = OLECMDID(62i32);
pub const OLECMDID_UPDATECOMMANDS: OLECMDID = OLECMDID(21i32);
pub const OLECMDID_UPDATEPAGESTATUS: OLECMDID = OLECMDID(48i32);
pub const OLECMDID_UPDATETRAVELENTRY: OLECMDID = OLECMDID(61i32);
pub const OLECMDID_UPDATETRAVELENTRY_DATARECOVERY: OLECMDID = OLECMDID(67i32);
pub const OLECMDID_UPDATE_CARET: OLECMDID = OLECMDID(76i32);
pub const OLECMDID_USER_OPTICAL_ZOOM: OLECMDID = OLECMDID(73i32);
pub const OLECMDID_VIEWPORT_MODE: OLECMDID = OLECMDID(70i32);
pub const OLECMDID_VISUAL_VIEWPORT_EXCLUDE_BOTTOM: OLECMDID = OLECMDID(72i32);
pub const OLECMDID_WINDOWSTATECHANGED: OLECMDID = OLECMDID(65i32);
pub const OLECMDID_ZOOM: OLECMDID = OLECMDID(19i32);
pub const OLECMDTEXTF_NAME: OLECMDTEXTF = OLECMDTEXTF(1i32);
pub const OLECMDTEXTF_NONE: OLECMDTEXTF = OLECMDTEXTF(0i32);
pub const OLECMDTEXTF_STATUS: OLECMDTEXTF = OLECMDTEXTF(2i32);
pub const OLECMD_TASKDLGID_ONBEFOREUNLOAD: u32 = 1u32;
pub const OLECONTF_EMBEDDINGS: OLECONTF = OLECONTF(1i32);
pub const OLECONTF_LINKS: OLECONTF = OLECONTF(2i32);
pub const OLECONTF_ONLYIFRUNNING: OLECONTF = OLECONTF(16i32);
pub const OLECONTF_ONLYUSER: OLECONTF = OLECONTF(8i32);
pub const OLECONTF_OTHERS: OLECONTF = OLECONTF(4i32);
pub const OLECREATE_LEAVERUNNING: OLECREATE = OLECREATE(1u32);
pub const OLECREATE_ZERO: OLECREATE = OLECREATE(0u32);
pub const OLEDC_NODRAW: OLEDCFLAGS = OLEDCFLAGS(1i32);
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = OLEDCFLAGS(4i32);
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = OLEDCFLAGS(2i32);
pub const OLEGETMONIKER_FORCEASSIGN: OLEGETMONIKER = OLEGETMONIKER(2i32);
pub const OLEGETMONIKER_ONLYIFTHERE: OLEGETMONIKER = OLEGETMONIKER(1i32);
pub const OLEGETMONIKER_TEMPFORUSER: OLEGETMONIKER = OLEGETMONIKER(4i32);
pub const OLEGETMONIKER_UNASSIGN: OLEGETMONIKER = OLEGETMONIKER(3i32);
pub const OLEIVERB_DISCARDUNDOSTATE: OLEIVERB = OLEIVERB(-6i32);
pub const OLEIVERB_HIDE: OLEIVERB = OLEIVERB(-3i32);
pub const OLEIVERB_INPLACEACTIVATE: OLEIVERB = OLEIVERB(-5i32);
pub const OLEIVERB_OPEN: OLEIVERB = OLEIVERB(-2i32);
pub const OLEIVERB_PRIMARY: OLEIVERB = OLEIVERB(0i32);
pub const OLEIVERB_PROPERTIES: i32 = -7i32;
pub const OLEIVERB_SHOW: OLEIVERB = OLEIVERB(-1i32);
pub const OLEIVERB_UIACTIVATE: OLEIVERB = OLEIVERB(-4i32);
pub const OLELINKBIND_EVENIFCLASSDIFF: OLELINKBIND = OLELINKBIND(1i32);
pub const OLEMISC_ACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(256i32);
pub const OLEMISC_ACTSLIKEBUTTON: OLEMISC = OLEMISC(4096i32);
pub const OLEMISC_ACTSLIKELABEL: OLEMISC = OLEMISC(8192i32);
pub const OLEMISC_ALIGNABLE: OLEMISC = OLEMISC(32768i32);
pub const OLEMISC_ALWAYSRUN: OLEMISC = OLEMISC(2048i32);
pub const OLEMISC_CANLINKBYOLE1: OLEMISC = OLEMISC(32i32);
pub const OLEMISC_CANTLINKINSIDE: OLEMISC = OLEMISC(16i32);
pub const OLEMISC_IGNOREACTIVATEWHENVISIBLE: OLEMISC = OLEMISC(524288i32);
pub const OLEMISC_IMEMODE: OLEMISC = OLEMISC(262144i32);
pub const OLEMISC_INSERTNOTREPLACE: OLEMISC = OLEMISC(4i32);
pub const OLEMISC_INSIDEOUT: OLEMISC = OLEMISC(128i32);
pub const OLEMISC_INVISIBLEATRUNTIME: OLEMISC = OLEMISC(1024i32);
pub const OLEMISC_ISLINKOBJECT: OLEMISC = OLEMISC(64i32);
pub const OLEMISC_NOUIACTIVATE: OLEMISC = OLEMISC(16384i32);
pub const OLEMISC_ONLYICONIC: OLEMISC = OLEMISC(2i32);
pub const OLEMISC_RECOMPOSEONRESIZE: OLEMISC = OLEMISC(1i32);
pub const OLEMISC_RENDERINGISDEVICEINDEPENDENT: OLEMISC = OLEMISC(512i32);
pub const OLEMISC_SETCLIENTSITEFIRST: OLEMISC = OLEMISC(131072i32);
pub const OLEMISC_SIMPLEFRAME: OLEMISC = OLEMISC(65536i32);
pub const OLEMISC_STATIC: OLEMISC = OLEMISC(8i32);
pub const OLEMISC_SUPPORTSMULTILEVELUNDO: OLEMISC = OLEMISC(2097152i32);
pub const OLEMISC_WANTSTOMENUMERGE: OLEMISC = OLEMISC(1048576i32);
pub const OLERENDER_ASIS: OLERENDER = OLERENDER(3i32);
pub const OLERENDER_DRAW: OLERENDER = OLERENDER(1i32);
pub const OLERENDER_FORMAT: OLERENDER = OLERENDER(2i32);
pub const OLERENDER_NONE: OLERENDER = OLERENDER(0i32);
pub const OLESTDDELIM: windows_core::PCWSTR = windows_core::w!("\\");
pub const OLESTREAM_CONVERSION_DEFAULT: i32 = 0i32;
pub const OLESTREAM_CONVERSION_DISABLEOLELINK: i32 = 1i32;
pub const OLEUIPASTE_ENABLEICON: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2048i32);
pub const OLEUIPASTE_LINKANYTYPE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1024i32);
pub const OLEUIPASTE_LINKTYPE1: OLEUIPASTEFLAG = OLEUIPASTEFLAG(1i32);
pub const OLEUIPASTE_LINKTYPE2: OLEUIPASTEFLAG = OLEUIPASTEFLAG(2i32);
pub const OLEUIPASTE_LINKTYPE3: OLEUIPASTEFLAG = OLEUIPASTEFLAG(4i32);
pub const OLEUIPASTE_LINKTYPE4: OLEUIPASTEFLAG = OLEUIPASTEFLAG(8i32);
pub const OLEUIPASTE_LINKTYPE5: OLEUIPASTEFLAG = OLEUIPASTEFLAG(16i32);
pub const OLEUIPASTE_LINKTYPE6: OLEUIPASTEFLAG = OLEUIPASTEFLAG(32i32);
pub const OLEUIPASTE_LINKTYPE7: OLEUIPASTEFLAG = OLEUIPASTEFLAG(64i32);
pub const OLEUIPASTE_LINKTYPE8: OLEUIPASTEFLAG = OLEUIPASTEFLAG(128i32);
pub const OLEUIPASTE_PASTE: OLEUIPASTEFLAG = OLEUIPASTEFLAG(512i32);
pub const OLEUIPASTE_PASTEONLY: OLEUIPASTEFLAG = OLEUIPASTEFLAG(0i32);
pub const OLEUI_BZERR_HTASKINVALID: u32 = 116u32;
pub const OLEUI_BZ_CALLUNBLOCKED: u32 = 119u32;
pub const OLEUI_BZ_RETRYSELECTED: u32 = 118u32;
pub const OLEUI_BZ_SWITCHTOSELECTED: u32 = 117u32;
pub const OLEUI_CANCEL: u32 = 2u32;
pub const OLEUI_CIERR_MUSTHAVECLSID: u32 = 116u32;
pub const OLEUI_CIERR_MUSTHAVECURRENTMETAFILE: u32 = 117u32;
pub const OLEUI_CIERR_SZICONEXEINVALID: u32 = 118u32;
pub const OLEUI_CSERR_FROMNOTNULL: u32 = 118u32;
pub const OLEUI_CSERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_CSERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_CSERR_SOURCEINVALID: u32 = 121u32;
pub const OLEUI_CSERR_SOURCENULL: u32 = 120u32;
pub const OLEUI_CSERR_SOURCEPARSEERROR: u32 = 122u32;
pub const OLEUI_CSERR_SOURCEPARSERROR: u32 = 122u32;
pub const OLEUI_CSERR_TONOTNULL: u32 = 119u32;
pub const OLEUI_CTERR_CBFORMATINVALID: u32 = 119u32;
pub const OLEUI_CTERR_CLASSIDINVALID: u32 = 117u32;
pub const OLEUI_CTERR_DVASPECTINVALID: u32 = 118u32;
pub const OLEUI_CTERR_HMETAPICTINVALID: u32 = 120u32;
pub const OLEUI_CTERR_STRINGINVALID: u32 = 121u32;
pub const OLEUI_ELERR_LINKCNTRINVALID: u32 = 117u32;
pub const OLEUI_ELERR_LINKCNTRNULL: u32 = 116u32;
pub const OLEUI_ERR_CBSTRUCTINCORRECT: u32 = 103u32;
pub const OLEUI_ERR_DIALOGFAILURE: u32 = 112u32;
pub const OLEUI_ERR_FINDTEMPLATEFAILURE: u32 = 110u32;
pub const OLEUI_ERR_GLOBALMEMALLOC: u32 = 114u32;
pub const OLEUI_ERR_HINSTANCEINVALID: u32 = 107u32;
pub const OLEUI_ERR_HRESOURCEINVALID: u32 = 109u32;
pub const OLEUI_ERR_HWNDOWNERINVALID: u32 = 104u32;
pub const OLEUI_ERR_LOADSTRING: u32 = 115u32;
pub const OLEUI_ERR_LOADTEMPLATEFAILURE: u32 = 111u32;
pub const OLEUI_ERR_LOCALMEMALLOC: u32 = 113u32;
pub const OLEUI_ERR_LPFNHOOKINVALID: u32 = 106u32;
pub const OLEUI_ERR_LPSZCAPTIONINVALID: u32 = 105u32;
pub const OLEUI_ERR_LPSZTEMPLATEINVALID: u32 = 108u32;
pub const OLEUI_ERR_OLEMEMALLOC: u32 = 100u32;
pub const OLEUI_ERR_STANDARDMAX: u32 = 116u32;
pub const OLEUI_ERR_STANDARDMIN: u32 = 100u32;
pub const OLEUI_ERR_STRUCTUREINVALID: u32 = 102u32;
pub const OLEUI_ERR_STRUCTURENULL: u32 = 101u32;
pub const OLEUI_FALSE: u32 = 0u32;
pub const OLEUI_GPERR_CBFORMATINVALID: u32 = 130u32;
pub const OLEUI_GPERR_CLASSIDINVALID: u32 = 128u32;
pub const OLEUI_GPERR_LPCLSIDEXCLUDEINVALID: u32 = 129u32;
pub const OLEUI_GPERR_STRINGINVALID: u32 = 127u32;
pub const OLEUI_IOERR_ARRLINKTYPESINVALID: u32 = 118u32;
pub const OLEUI_IOERR_ARRPASTEENTRIESINVALID: u32 = 117u32;
pub const OLEUI_IOERR_CCHFILEINVALID: u32 = 125u32;
pub const OLEUI_IOERR_HICONINVALID: u32 = 118u32;
pub const OLEUI_IOERR_LPCLSIDEXCLUDEINVALID: u32 = 124u32;
pub const OLEUI_IOERR_LPFORMATETCINVALID: u32 = 119u32;
pub const OLEUI_IOERR_LPIOLECLIENTSITEINVALID: u32 = 121u32;
pub const OLEUI_IOERR_LPISTORAGEINVALID: u32 = 122u32;
pub const OLEUI_IOERR_LPSZFILEINVALID: u32 = 116u32;
pub const OLEUI_IOERR_LPSZLABELINVALID: u32 = 117u32;
pub const OLEUI_IOERR_PPVOBJINVALID: u32 = 120u32;
pub const OLEUI_IOERR_SCODEHASERROR: u32 = 123u32;
pub const OLEUI_IOERR_SRCDATAOBJECTINVALID: u32 = 116u32;
pub const OLEUI_LPERR_LINKCNTRINVALID: u32 = 134u32;
pub const OLEUI_LPERR_LINKCNTRNULL: u32 = 133u32;
pub const OLEUI_OK: u32 = 1u32;
pub const OLEUI_OPERR_DLGPROCNOTNULL: u32 = 125u32;
pub const OLEUI_OPERR_INVALIDPAGES: u32 = 123u32;
pub const OLEUI_OPERR_LINKINFOINVALID: u32 = 137u32;
pub const OLEUI_OPERR_LPARAMNOTZERO: u32 = 126u32;
pub const OLEUI_OPERR_NOTSUPPORTED: u32 = 124u32;
pub const OLEUI_OPERR_OBJINFOINVALID: u32 = 136u32;
pub const OLEUI_OPERR_PAGESINCORRECT: u32 = 122u32;
pub const OLEUI_OPERR_PROPERTYSHEET: u32 = 135u32;
pub const OLEUI_OPERR_PROPSHEETINVALID: u32 = 119u32;
pub const OLEUI_OPERR_PROPSHEETNULL: u32 = 118u32;
pub const OLEUI_OPERR_PROPSINVALID: u32 = 121u32;
pub const OLEUI_OPERR_SUBPROPINVALID: u32 = 117u32;
pub const OLEUI_OPERR_SUBPROPNULL: u32 = 116u32;
pub const OLEUI_OPERR_SUPPROP: u32 = 120u32;
pub const OLEUI_PSERR_CLIPBOARDCHANGED: u32 = 119u32;
pub const OLEUI_PSERR_GETCLIPBOARDFAILED: u32 = 120u32;
pub const OLEUI_QUERY_GETCLASSID: u32 = 65280u32;
pub const OLEUI_QUERY_LINKBROKEN: u32 = 65281u32;
pub const OLEUI_SUCCESS: u32 = 1u32;
pub const OLEUI_VPERR_DVASPECTINVALID: u32 = 132u32;
pub const OLEUI_VPERR_METAPICTINVALID: u32 = 131u32;
pub const OLEUPDATE_ALWAYS: OLEUPDATE = OLEUPDATE(1i32);
pub const OLEUPDATE_ONCALL: OLEUPDATE = OLEUPDATE(3i32);
pub const OLEVERBATTRIB_NEVERDIRTIES: OLEVERBATTRIB = OLEVERBATTRIB(1i32);
pub const OLEVERBATTRIB_ONCONTAINERMENU: OLEVERBATTRIB = OLEVERBATTRIB(2i32);
pub const OLEVERB_PRIMARY: u32 = 0u32;
pub const OLEWHICHMK_CONTAINER: OLEWHICHMK = OLEWHICHMK(1i32);
pub const OLEWHICHMK_OBJFULL: OLEWHICHMK = OLEWHICHMK(3i32);
pub const OLEWHICHMK_OBJREL: OLEWHICHMK = OLEWHICHMK(2i32);
pub const OPF_DISABLECONVERT: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(8u32);
pub const OPF_NOFILLDEFAULT: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(2u32);
pub const OPF_OBJECTISLINK: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(1u32);
pub const OPF_SHOWHELP: OBJECT_PROPERTIES_FLAGS = OBJECT_PROPERTIES_FLAGS(4u32);
pub const OT_EMBEDDED: i32 = 2i32;
pub const OT_LINK: i32 = 1i32;
pub const OT_STATIC: i32 = 3i32;
pub const PAGEACTION_UI_DEFAULT: PAGEACTION_UI = PAGEACTION_UI(0i32);
pub const PAGEACTION_UI_MODAL: PAGEACTION_UI = PAGEACTION_UI(1i32);
pub const PAGEACTION_UI_MODELESS: PAGEACTION_UI = PAGEACTION_UI(2i32);
pub const PAGEACTION_UI_SILENT: PAGEACTION_UI = PAGEACTION_UI(3i32);
pub const PARAMFLAG_FHASCUSTDATA: PARAMFLAGS = PARAMFLAGS(64u16);
pub const PARAMFLAG_FHASDEFAULT: PARAMFLAGS = PARAMFLAGS(32u16);
pub const PARAMFLAG_FIN: PARAMFLAGS = PARAMFLAGS(1u16);
pub const PARAMFLAG_FLCID: PARAMFLAGS = PARAMFLAGS(4u16);
pub const PARAMFLAG_FOPT: PARAMFLAGS = PARAMFLAGS(16u16);
pub const PARAMFLAG_FOUT: PARAMFLAGS = PARAMFLAGS(2u16);
pub const PARAMFLAG_FRETVAL: PARAMFLAGS = PARAMFLAGS(8u16);
pub const PARAMFLAG_NONE: PARAMFLAGS = PARAMFLAGS(0u16);
pub const PERPROP_E_FIRST: i32 = -2147220992i32;
pub const PERPROP_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const PERPROP_E_NOPAGEAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const PERPROP_S_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const PERPROP_S_LAST: windows_core::HRESULT = windows_core::HRESULT(0x4020F_u32 as _);
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = PICTUREATTRIBUTES(1i32);
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = PICTUREATTRIBUTES(2i32);
pub const PICTYPE_BITMAP: PICTYPE = PICTYPE(1i16);
pub const PICTYPE_ENHMETAFILE: PICTYPE = PICTYPE(4i16);
pub const PICTYPE_ICON: PICTYPE = PICTYPE(3i16);
pub const PICTYPE_METAFILE: PICTYPE = PICTYPE(2i16);
pub const PICTYPE_NONE: PICTYPE = PICTYPE(0i16);
pub const PICTYPE_UNINITIALIZED: PICTYPE = PICTYPE(-1i16);
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = POINTERINACTIVE(4i32);
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = POINTERINACTIVE(1i32);
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = POINTERINACTIVE(2i32);
pub const PRINTFLAG_DONTACTUALLYPRINT: PRINTFLAG = PRINTFLAG(16i32);
pub const PRINTFLAG_FORCEPROPERTIES: PRINTFLAG = PRINTFLAG(32i32);
pub const PRINTFLAG_MAYBOTHERUSER: PRINTFLAG = PRINTFLAG(1i32);
pub const PRINTFLAG_PRINTTOFILE: PRINTFLAG = PRINTFLAG(64i32);
pub const PRINTFLAG_PROMPTUSER: PRINTFLAG = PRINTFLAG(2i32);
pub const PRINTFLAG_RECOMPOSETODEVICE: PRINTFLAG = PRINTFLAG(8i32);
pub const PRINTFLAG_USERMAYCHANGEPRINTER: PRINTFLAG = PRINTFLAG(4i32);
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = PROPBAG2_TYPE(1i32);
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = PROPBAG2_TYPE(6i32);
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = PROPBAG2_TYPE(3i32);
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = PROPBAG2_TYPE(5i32);
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = PROPBAG2_TYPE(4i32);
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = PROPBAG2_TYPE(0i32);
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = PROPBAG2_TYPE(2i32);
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = PROPPAGESTATUS(4i32);
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = PROPPAGESTATUS(1i32);
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = PROPPAGESTATUS(2i32);
pub const PROP_HWND_CHGICONDLG: windows_core::PCWSTR = windows_core::w!("HWND_CIDLG");
pub const PSF_CHECKDISPLAYASICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(8u32);
pub const PSF_DISABLEDISPLAYASICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(16u32);
pub const PSF_HIDECHANGEICON: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(32u32);
pub const PSF_NOREFRESHDATAOBJECT: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(128u32);
pub const PSF_SELECTPASTE: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(2u32);
pub const PSF_SELECTPASTELINK: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(4u32);
pub const PSF_SHOWHELP: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(1u32);
pub const PSF_STAYONCLIPBOARDCHANGE: PASTE_SPECIAL_FLAGS = PASTE_SPECIAL_FLAGS(64u32);
pub const PS_MAXLINKTYPES: u32 = 8u32;
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = QACONTAINERFLAGS(32i32);
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = QACONTAINERFLAGS(8i32);
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = QACONTAINERFLAGS(64i32);
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = QACONTAINERFLAGS(2i32);
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = QACONTAINERFLAGS(1i32);
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = QACONTAINERFLAGS(128i32);
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = QACONTAINERFLAGS(16i32);
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = QACONTAINERFLAGS(4i32);
pub const READYSTATE_COMPLETE: READYSTATE = READYSTATE(4i32);
pub const READYSTATE_INTERACTIVE: READYSTATE = READYSTATE(3i32);
pub const READYSTATE_LOADED: READYSTATE = READYSTATE(2i32);
pub const READYSTATE_LOADING: READYSTATE = READYSTATE(1i32);
pub const READYSTATE_UNINITIALIZED: READYSTATE = READYSTATE(0i32);
pub const REGKIND_DEFAULT: REGKIND = REGKIND(0i32);
pub const REGKIND_NONE: REGKIND = REGKIND(2i32);
pub const REGKIND_REGISTER: REGKIND = REGKIND(1i32);
pub const SELFREG_E_CLASS: windows_core::HRESULT = windows_core::HRESULT(0x80040201_u32 as _);
pub const SELFREG_E_FIRST: i32 = -2147220992i32;
pub const SELFREG_E_LAST: windows_core::HRESULT = windows_core::HRESULT(0x8004020F_u32 as _);
pub const SELFREG_E_TYPELIB: windows_core::HRESULT = windows_core::HRESULT(0x80040200_u32 as _);
pub const SELFREG_S_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x40200_u32 as _);
pub const SELFREG_S_LAST: windows_core::HRESULT = windows_core::HRESULT(0x4020F_u32 as _);
pub const SF_BSTR: SF_TYPE = SF_TYPE(8i32);
pub const SF_DISPATCH: SF_TYPE = SF_TYPE(9i32);
pub const SF_ERROR: SF_TYPE = SF_TYPE(10i32);
pub const SF_HAVEIID: SF_TYPE = SF_TYPE(32781i32);
pub const SF_I1: SF_TYPE = SF_TYPE(16i32);
pub const SF_I2: SF_TYPE = SF_TYPE(2i32);
pub const SF_I4: SF_TYPE = SF_TYPE(3i32);
pub const SF_I8: SF_TYPE = SF_TYPE(20i32);
pub const SF_RECORD: SF_TYPE = SF_TYPE(36i32);
pub const SF_UNKNOWN: SF_TYPE = SF_TYPE(13i32);
pub const SF_VARIANT: SF_TYPE = SF_TYPE(12i32);
pub const SID_GetCaller: windows_core::GUID = windows_core::GUID::from_u128(0x4717cc40_bcb9_11d0_9336_00a0c90dcaa9);
pub const SID_ProvideRuntimeContext: windows_core::GUID = windows_core::GUID::from_u128(0x74a5040c_dd0c_48f0_ac85_194c3259180a);
pub const SID_VariantConversion: windows_core::GUID = windows_core::GUID::from_u128(0x1f101481_bccd_11d0_9336_00a0c90dcaa9);
pub const STDOLE2_LCID: u32 = 0u32;
pub const STDOLE2_MAJORVERNUM: u32 = 2u32;
pub const STDOLE2_MINORVERNUM: u32 = 0u32;
pub const STDOLE_LCID: u32 = 0u32;
pub const STDOLE_MAJORVERNUM: u32 = 1u32;
pub const STDOLE_MINORVERNUM: u32 = 0u32;
pub const STDOLE_TLB: windows_core::PCSTR = windows_core::s!("stdole2.tlb");
pub const STDTYPE_TLB: windows_core::PCSTR = windows_core::s!("stdole2.tlb");
pub const SZOLEUI_MSG_ADDCONTROL: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_ADDCONTROL");
pub const SZOLEUI_MSG_BROWSE: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_BROWSE");
pub const SZOLEUI_MSG_BROWSE_OFN: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_BROWSE_OFN");
pub const SZOLEUI_MSG_CHANGEICON: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_CHANGEICON");
pub const SZOLEUI_MSG_CHANGESOURCE: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_CHANGESOURCE");
pub const SZOLEUI_MSG_CLOSEBUSYDIALOG: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_CLOSEBUSYDIALOG");
pub const SZOLEUI_MSG_CONVERT: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_CONVERT");
pub const SZOLEUI_MSG_ENDDIALOG: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_ENDDIALOG");
pub const SZOLEUI_MSG_HELP: windows_core::PCWSTR = windows_core::w!("OLEUI_MSG_HELP");
pub const TIFLAGS_EXTENDDISPATCHONLY: u32 = 1u32;
pub const TYPEFLAG_FAGGREGATABLE: TYPEFLAGS = TYPEFLAGS(1024i32);
pub const TYPEFLAG_FAPPOBJECT: TYPEFLAGS = TYPEFLAGS(1i32);
pub const TYPEFLAG_FCANCREATE: TYPEFLAGS = TYPEFLAGS(2i32);
pub const TYPEFLAG_FCONTROL: TYPEFLAGS = TYPEFLAGS(32i32);
pub const TYPEFLAG_FDISPATCHABLE: TYPEFLAGS = TYPEFLAGS(4096i32);
pub const TYPEFLAG_FDUAL: TYPEFLAGS = TYPEFLAGS(64i32);
pub const TYPEFLAG_FHIDDEN: TYPEFLAGS = TYPEFLAGS(16i32);
pub const TYPEFLAG_FLICENSED: TYPEFLAGS = TYPEFLAGS(4i32);
pub const TYPEFLAG_FNONEXTENSIBLE: TYPEFLAGS = TYPEFLAGS(128i32);
pub const TYPEFLAG_FOLEAUTOMATION: TYPEFLAGS = TYPEFLAGS(256i32);
pub const TYPEFLAG_FPREDECLID: TYPEFLAGS = TYPEFLAGS(8i32);
pub const TYPEFLAG_FPROXY: TYPEFLAGS = TYPEFLAGS(16384i32);
pub const TYPEFLAG_FREPLACEABLE: TYPEFLAGS = TYPEFLAGS(2048i32);
pub const TYPEFLAG_FRESTRICTED: TYPEFLAGS = TYPEFLAGS(512i32);
pub const TYPEFLAG_FREVERSEBIND: TYPEFLAGS = TYPEFLAGS(8192i32);
pub const UAS_BLOCKED: UASFLAGS = UASFLAGS(1i32);
pub const UAS_MASK: UASFLAGS = UASFLAGS(3i32);
pub const UAS_NOPARENTENABLE: UASFLAGS = UASFLAGS(2i32);
pub const UAS_NORMAL: UASFLAGS = UASFLAGS(0i32);
pub const UPDFCACHE_ALL: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483647u32);
pub const UPDFCACHE_ALLBUTNODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483646u32);
pub const UPDFCACHE_IFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(16u32);
pub const UPDFCACHE_IFBLANKORONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(18u32);
pub const UPDFCACHE_NODATACACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(1u32);
pub const UPDFCACHE_NORMALCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(8u32);
pub const UPDFCACHE_ONLYIFBLANK: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2147483648u32);
pub const UPDFCACHE_ONSAVECACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(2u32);
pub const UPDFCACHE_ONSTOPCACHE: UPDFCACHE_FLAGS = UPDFCACHE_FLAGS(4u32);
pub const USERCLASSTYPE_APPNAME: USERCLASSTYPE = USERCLASSTYPE(3i32);
pub const USERCLASSTYPE_FULL: USERCLASSTYPE = USERCLASSTYPE(1i32);
pub const USERCLASSTYPE_SHORT: USERCLASSTYPE = USERCLASSTYPE(2i32);
pub const VARCMP_EQ: VARCMP = VARCMP(1u32);
pub const VARCMP_GT: VARCMP = VARCMP(2u32);
pub const VARCMP_LT: VARCMP = VARCMP(0u32);
pub const VARCMP_NULL: VARCMP = VARCMP(3u32);
pub const VARFORMAT_FIRST_DAY_FRIDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(5i32);
pub const VARFORMAT_FIRST_DAY_MONDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(1i32);
pub const VARFORMAT_FIRST_DAY_SATURDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(6i32);
pub const VARFORMAT_FIRST_DAY_SUNDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(7i32);
pub const VARFORMAT_FIRST_DAY_SYSTEMDEFAULT: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(0i32);
pub const VARFORMAT_FIRST_DAY_THURSDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(4i32);
pub const VARFORMAT_FIRST_DAY_TUESDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(2i32);
pub const VARFORMAT_FIRST_DAY_WEDNESDAY: VARFORMAT_FIRST_DAY = VARFORMAT_FIRST_DAY(3i32);
pub const VARFORMAT_FIRST_WEEK_CONTAINS_JANUARY_FIRST: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(1i32);
pub const VARFORMAT_FIRST_WEEK_HAS_SEVEN_DAYS: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(3i32);
pub const VARFORMAT_FIRST_WEEK_LARGER_HALF_IN_CURRENT_YEAR: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(2i32);
pub const VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT: VARFORMAT_FIRST_WEEK = VARFORMAT_FIRST_WEEK(0i32);
pub const VARFORMAT_GROUP_NOTTHOUSANDS: VARFORMAT_GROUP = VARFORMAT_GROUP(0i32);
pub const VARFORMAT_GROUP_SYSTEMDEFAULT: VARFORMAT_GROUP = VARFORMAT_GROUP(-2i32);
pub const VARFORMAT_GROUP_THOUSANDS: VARFORMAT_GROUP = VARFORMAT_GROUP(-1i32);
pub const VARFORMAT_LEADING_DIGIT_INCLUDED: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(-1i32);
pub const VARFORMAT_LEADING_DIGIT_NOTINCLUDED: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(0i32);
pub const VARFORMAT_LEADING_DIGIT_SYSTEMDEFAULT: VARFORMAT_LEADING_DIGIT = VARFORMAT_LEADING_DIGIT(-2i32);
pub const VARFORMAT_NAMED_FORMAT_GENERALDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(0i32);
pub const VARFORMAT_NAMED_FORMAT_LONGDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(1i32);
pub const VARFORMAT_NAMED_FORMAT_LONGTIME: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(3i32);
pub const VARFORMAT_NAMED_FORMAT_SHORTDATE: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(2i32);
pub const VARFORMAT_NAMED_FORMAT_SHORTTIME: VARFORMAT_NAMED_FORMAT = VARFORMAT_NAMED_FORMAT(4i32);
pub const VARFORMAT_PARENTHESES_NOTUSED: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(0i32);
pub const VARFORMAT_PARENTHESES_SYSTEMDEFAULT: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(-2i32);
pub const VARFORMAT_PARENTHESES_USED: VARFORMAT_PARENTHESES = VARFORMAT_PARENTHESES(-1i32);
pub const VAR_CALENDAR_GREGORIAN: u32 = 256u32;
pub const VAR_CALENDAR_HIJRI: u32 = 8u32;
pub const VAR_CALENDAR_THAI: u32 = 128u32;
pub const VAR_DATEVALUEONLY: u32 = 2u32;
pub const VAR_FORMAT_NOSUBSTITUTE: u32 = 32u32;
pub const VAR_FOURDIGITYEARS: u32 = 64u32;
pub const VAR_LOCALBOOL: u32 = 16u32;
pub const VAR_TIMEVALUEONLY: u32 = 1u32;
pub const VAR_VALIDDATE: u32 = 4u32;
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = VIEWSTATUS(32i32);
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = VIEWSTATUS(4i32);
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = VIEWSTATUS(8i32);
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = VIEWSTATUS(1i32);
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = VIEWSTATUS(2i32);
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = VIEWSTATUS(16i32);
pub const VPF_DISABLERELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(2u32);
pub const VPF_DISABLESCALE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(4u32);
pub const VPF_SELECTRELATIVE: VIEW_OBJECT_PROPERTIES_FLAGS = VIEW_OBJECT_PROPERTIES_FLAGS(1u32);
pub const VTDATEGRE_MAX: u32 = 2958465u32;
pub const VTDATEGRE_MIN: i32 = -657434i32;
pub const VT_BLOB_PROPSET: u32 = 75u32;
pub const VT_STORED_PROPSET: u32 = 74u32;
pub const VT_STREAMED_PROPSET: u32 = 73u32;
pub const VT_VERBOSE_ENUM: u32 = 76u32;
pub const WIN32: u32 = 100u32;
pub const WPCSETTING_FILEDOWNLOAD_BLOCKED: WPCSETTING = WPCSETTING(2i32);
pub const WPCSETTING_LOGGING_ENABLED: WPCSETTING = WPCSETTING(1i32);
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = XFORMCOORDS(8i32);
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = XFORMCOORDS(16i32);
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = XFORMCOORDS(4i32);
pub const XFORMCOORDS_POSITION: XFORMCOORDS = XFORMCOORDS(1i32);
pub const XFORMCOORDS_SIZE: XFORMCOORDS = XFORMCOORDS(2i32);
pub const fdexEnumAll: i32 = 2i32;
pub const fdexEnumDefault: i32 = 1i32;
pub const fdexNameCaseInsensitive: i32 = 8i32;
pub const fdexNameCaseSensitive: i32 = 1i32;
pub const fdexNameEnsure: i32 = 2i32;
pub const fdexNameImplicit: i32 = 4i32;
pub const fdexNameInternal: i32 = 16i32;
pub const fdexNameNoDynamicProperties: i32 = 32i32;
pub const fdexPropCanCall: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(256u32);
pub const fdexPropCanConstruct: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(1024u32);
pub const fdexPropCanGet: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(1u32);
pub const fdexPropCanPut: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(4u32);
pub const fdexPropCanPutRef: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(16u32);
pub const fdexPropCanSourceEvents: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(4096u32);
pub const fdexPropCannotCall: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(512u32);
pub const fdexPropCannotConstruct: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(2048u32);
pub const fdexPropCannotGet: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(2u32);
pub const fdexPropCannotPut: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(8u32);
pub const fdexPropCannotPutRef: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(32u32);
pub const fdexPropCannotSourceEvents: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(8192u32);
pub const fdexPropDynamicType: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(128u32);
pub const fdexPropNoSideEffects: FDEX_PROP_FLAGS = FDEX_PROP_FLAGS(64u32);
pub const triChecked: OLE_TRISTATE = OLE_TRISTATE(1i32);
pub const triGray: OLE_TRISTATE = OLE_TRISTATE(2i32);
pub const triUnchecked: OLE_TRISTATE = OLE_TRISTATE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTIVATEFLAGS(pub i32);
impl windows_core::TypeKind for ACTIVATEFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTIVATEFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTIVATEFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ACTIVEOBJECT_FLAGS(pub u32);
impl windows_core::TypeKind for ACTIVEOBJECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ACTIVEOBJECT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ACTIVEOBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ACTIVEOBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ACTIVEOBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ACTIVEOBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ACTIVEOBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDSPEED(pub i32);
impl windows_core::TypeKind for BINDSPEED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDSPEED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDSPEED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BUSY_DIALOG_FLAGS(pub u32);
impl windows_core::TypeKind for BUSY_DIALOG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BUSY_DIALOG_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BUSY_DIALOG_FLAGS").field(&self.0).finish()
    }
}
impl BUSY_DIALOG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for BUSY_DIALOG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for BUSY_DIALOG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for BUSY_DIALOG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANGEKIND(pub i32);
impl windows_core::TypeKind for CHANGEKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANGEKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANGEKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANGE_ICON_FLAGS(pub u32);
impl windows_core::TypeKind for CHANGE_ICON_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANGE_ICON_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANGE_ICON_FLAGS").field(&self.0).finish()
    }
}
impl CHANGE_ICON_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHANGE_ICON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHANGE_ICON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHANGE_ICON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CHANGE_SOURCE_FLAGS(pub u32);
impl windows_core::TypeKind for CHANGE_SOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CHANGE_SOURCE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CHANGE_SOURCE_FLAGS").field(&self.0).finish()
    }
}
impl CHANGE_SOURCE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CHANGE_SOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CHANGE_SOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CHANGE_SOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLIPBOARD_FORMAT(pub u16);
impl windows_core::TypeKind for CLIPBOARD_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLIPBOARD_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLIPBOARD_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CTRLINFO(pub i32);
impl windows_core::TypeKind for CTRLINFO {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CTRLINFO {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CTRLINFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISCARDCACHE(pub i32);
impl windows_core::TypeKind for DISCARDCACHE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISCARDCACHE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISCARDCACHE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DOCMISC(pub i32);
impl windows_core::TypeKind for DOCMISC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DOCMISC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DOCMISC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DROPEFFECT(pub u32);
impl windows_core::TypeKind for DROPEFFECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DROPEFFECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DROPEFFECT").field(&self.0).finish()
    }
}
impl DROPEFFECT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DROPEFFECT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DROPEFFECT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DROPEFFECT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DROPEFFECT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DROPEFFECT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DVASPECTINFOFLAG(pub i32);
impl windows_core::TypeKind for DVASPECTINFOFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DVASPECTINFOFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DVASPECTINFOFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DVEXTENTMODE(pub i32);
impl windows_core::TypeKind for DVEXTENTMODE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DVEXTENTMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DVEXTENTMODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EDIT_LINKS_FLAGS(pub u32);
impl windows_core::TypeKind for EDIT_LINKS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EDIT_LINKS_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EDIT_LINKS_FLAGS").field(&self.0).finish()
    }
}
impl EDIT_LINKS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EDIT_LINKS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EDIT_LINKS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EDIT_LINKS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EMBDHLP_FLAGS(pub u32);
impl windows_core::TypeKind for EMBDHLP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EMBDHLP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EMBDHLP_FLAGS").field(&self.0).finish()
    }
}
impl EMBDHLP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for EMBDHLP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for EMBDHLP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for EMBDHLP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for EMBDHLP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ENUM_CONTROLS_WHICH_FLAGS(pub u32);
impl windows_core::TypeKind for ENUM_CONTROLS_WHICH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ENUM_CONTROLS_WHICH_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ENUM_CONTROLS_WHICH_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FDEX_PROP_FLAGS(pub u32);
impl windows_core::TypeKind for FDEX_PROP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FDEX_PROP_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FDEX_PROP_FLAGS").field(&self.0).finish()
    }
}
impl FDEX_PROP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FDEX_PROP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FDEX_PROP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FDEX_PROP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FDEX_PROP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GUIDKIND(pub i32);
impl windows_core::TypeKind for GUIDKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GUIDKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GUIDKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HITRESULT(pub i32);
impl windows_core::TypeKind for HITRESULT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HITRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HITRESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IGNOREMIME(pub i32);
impl windows_core::TypeKind for IGNOREMIME {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IGNOREMIME {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IGNOREMIME").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INSERT_OBJECT_FLAGS(pub u32);
impl windows_core::TypeKind for INSERT_OBJECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INSERT_OBJECT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INSERT_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl INSERT_OBJECT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for INSERT_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for INSERT_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for INSERT_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct KEYMODIFIERS(pub u32);
impl windows_core::TypeKind for KEYMODIFIERS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for KEYMODIFIERS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("KEYMODIFIERS").field(&self.0).finish()
    }
}
impl KEYMODIFIERS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for KEYMODIFIERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for KEYMODIFIERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for KEYMODIFIERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for KEYMODIFIERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for KEYMODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LIBFLAGS(pub i32);
impl windows_core::TypeKind for LIBFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LIBFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LIBFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOAD_PICTURE_FLAGS(pub u32);
impl windows_core::TypeKind for LOAD_PICTURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOAD_PICTURE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOAD_PICTURE_FLAGS").field(&self.0).finish()
    }
}
impl LOAD_PICTURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOAD_PICTURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOAD_PICTURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOAD_PICTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEDIAPLAYBACK_STATE(pub i32);
impl windows_core::TypeKind for MEDIAPLAYBACK_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEDIAPLAYBACK_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEDIAPLAYBACK_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MULTICLASSINFO_FLAGS(pub u32);
impl windows_core::TypeKind for MULTICLASSINFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MULTICLASSINFO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MULTICLASSINFO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NUMPARSE_FLAGS(pub u32);
impl windows_core::TypeKind for NUMPARSE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NUMPARSE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NUMPARSE_FLAGS").field(&self.0).finish()
    }
}
impl NUMPARSE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NUMPARSE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NUMPARSE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NUMPARSE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NUMPARSE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OBJECT_PROPERTIES_FLAGS(pub u32);
impl windows_core::TypeKind for OBJECT_PROPERTIES_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl OBJECT_PROPERTIES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECLOSE(pub i32);
impl windows_core::TypeKind for OLECLOSE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECLOSE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECLOSE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDEXECOPT(pub i32);
impl windows_core::TypeKind for OLECMDEXECOPT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDEXECOPT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDEXECOPT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDF(pub i32);
impl windows_core::TypeKind for OLECMDF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID(pub i32);
impl windows_core::TypeKind for OLECMDID {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_BROWSERSTATEFLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_BROWSERSTATEFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_BROWSERSTATEFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_BROWSERSTATEFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_OPTICAL_ZOOMFLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_OPTICAL_ZOOMFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_OPTICAL_ZOOMFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_OPTICAL_ZOOMFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_PAGEACTIONFLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_PAGEACTIONFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_PAGEACTIONFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_PAGEACTIONFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_REFRESHFLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_REFRESHFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_REFRESHFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_REFRESHFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_VIEWPORT_MODE_FLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_VIEWPORT_MODE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_VIEWPORT_MODE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_VIEWPORT_MODE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDID_WINDOWSTATE_FLAG(pub i32);
impl windows_core::TypeKind for OLECMDID_WINDOWSTATE_FLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDID_WINDOWSTATE_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDID_WINDOWSTATE_FLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECMDTEXTF(pub i32);
impl windows_core::TypeKind for OLECMDTEXTF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECMDTEXTF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECMDTEXTF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECONTF(pub i32);
impl windows_core::TypeKind for OLECONTF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECONTF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECONTF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLECREATE(pub u32);
impl windows_core::TypeKind for OLECREATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLECREATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLECREATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEDCFLAGS(pub i32);
impl windows_core::TypeKind for OLEDCFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEDCFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEDCFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEGETMONIKER(pub i32);
impl windows_core::TypeKind for OLEGETMONIKER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEGETMONIKER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEGETMONIKER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEIVERB(pub i32);
impl windows_core::TypeKind for OLEIVERB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEIVERB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEIVERB").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLELINKBIND(pub i32);
impl windows_core::TypeKind for OLELINKBIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLELINKBIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLELINKBIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEMISC(pub i32);
impl windows_core::TypeKind for OLEMISC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEMISC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEMISC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLERENDER(pub i32);
impl windows_core::TypeKind for OLERENDER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLERENDER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLERENDER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEUIPASTEFLAG(pub i32);
impl windows_core::TypeKind for OLEUIPASTEFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEUIPASTEFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEUIPASTEFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEUPDATE(pub i32);
impl windows_core::TypeKind for OLEUPDATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEUPDATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEUPDATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEVERBATTRIB(pub i32);
impl windows_core::TypeKind for OLEVERBATTRIB {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEVERBATTRIB {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEVERBATTRIB").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLEWHICHMK(pub i32);
impl windows_core::TypeKind for OLEWHICHMK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLEWHICHMK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLEWHICHMK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OLE_TRISTATE(pub i32);
impl windows_core::TypeKind for OLE_TRISTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OLE_TRISTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OLE_TRISTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PAGEACTION_UI(pub i32);
impl windows_core::TypeKind for PAGEACTION_UI {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PAGEACTION_UI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PAGEACTION_UI").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PARAMFLAGS(pub u16);
impl windows_core::TypeKind for PARAMFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PARAMFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PARAMFLAGS").field(&self.0).finish()
    }
}
impl PARAMFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PARAMFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PARAMFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PARAMFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PARAMFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PARAMFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PASTE_SPECIAL_FLAGS(pub u32);
impl windows_core::TypeKind for PASTE_SPECIAL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PASTE_SPECIAL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PASTE_SPECIAL_FLAGS").field(&self.0).finish()
    }
}
impl PASTE_SPECIAL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PASTE_SPECIAL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PASTE_SPECIAL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PASTE_SPECIAL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PICTUREATTRIBUTES(pub i32);
impl windows_core::TypeKind for PICTUREATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PICTUREATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PICTUREATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PICTYPE(pub i16);
impl windows_core::TypeKind for PICTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PICTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PICTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct POINTERINACTIVE(pub i32);
impl windows_core::TypeKind for POINTERINACTIVE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for POINTERINACTIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("POINTERINACTIVE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PRINTFLAG(pub i32);
impl windows_core::TypeKind for PRINTFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PRINTFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PRINTFLAG").field(&self.0).finish()
    }
}
impl PRINTFLAG {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PRINTFLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PRINTFLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PRINTFLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PRINTFLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PRINTFLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPBAG2_TYPE(pub i32);
impl windows_core::TypeKind for PROPBAG2_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPBAG2_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPBAG2_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PROPPAGESTATUS(pub i32);
impl windows_core::TypeKind for PROPPAGESTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PROPPAGESTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PROPPAGESTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QACONTAINERFLAGS(pub i32);
impl windows_core::TypeKind for QACONTAINERFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QACONTAINERFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QACONTAINERFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct READYSTATE(pub i32);
impl windows_core::TypeKind for READYSTATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for READYSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("READYSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REGKIND(pub i32);
impl windows_core::TypeKind for REGKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REGKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REGKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SF_TYPE(pub i32);
impl windows_core::TypeKind for SF_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SF_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SF_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TYPEFLAGS(pub i32);
impl windows_core::TypeKind for TYPEFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TYPEFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TYPEFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UASFLAGS(pub i32);
impl windows_core::TypeKind for UASFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UASFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UASFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UI_CONVERT_FLAGS(pub u32);
impl windows_core::TypeKind for UI_CONVERT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UI_CONVERT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UI_CONVERT_FLAGS").field(&self.0).finish()
    }
}
impl UI_CONVERT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UI_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UI_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UI_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UI_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UPDFCACHE_FLAGS(pub u32);
impl windows_core::TypeKind for UPDFCACHE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UPDFCACHE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UPDFCACHE_FLAGS").field(&self.0).finish()
    }
}
impl UPDFCACHE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for UPDFCACHE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for UPDFCACHE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for UPDFCACHE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for UPDFCACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct USERCLASSTYPE(pub i32);
impl windows_core::TypeKind for USERCLASSTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for USERCLASSTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("USERCLASSTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARCMP(pub u32);
impl windows_core::TypeKind for VARCMP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARCMP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARCMP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_FIRST_DAY(pub i32);
impl windows_core::TypeKind for VARFORMAT_FIRST_DAY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_FIRST_DAY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_DAY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_FIRST_WEEK(pub i32);
impl windows_core::TypeKind for VARFORMAT_FIRST_WEEK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_FIRST_WEEK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_FIRST_WEEK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_GROUP(pub i32);
impl windows_core::TypeKind for VARFORMAT_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_GROUP {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_GROUP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_LEADING_DIGIT(pub i32);
impl windows_core::TypeKind for VARFORMAT_LEADING_DIGIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_LEADING_DIGIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_LEADING_DIGIT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_NAMED_FORMAT(pub i32);
impl windows_core::TypeKind for VARFORMAT_NAMED_FORMAT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_NAMED_FORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_NAMED_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFORMAT_PARENTHESES(pub i32);
impl windows_core::TypeKind for VARFORMAT_PARENTHESES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFORMAT_PARENTHESES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFORMAT_PARENTHESES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VIEWSTATUS(pub i32);
impl windows_core::TypeKind for VIEWSTATUS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VIEWSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VIEWSTATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VIEW_OBJECT_PROPERTIES_FLAGS(pub u32);
impl windows_core::TypeKind for VIEW_OBJECT_PROPERTIES_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VIEW_OBJECT_PROPERTIES_FLAGS").field(&self.0).finish()
    }
}
impl VIEW_OBJECT_PROPERTIES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VIEW_OBJECT_PROPERTIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VIEW_OBJECT_PROPERTIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WPCSETTING(pub i32);
impl windows_core::TypeKind for WPCSETTING {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WPCSETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WPCSETTING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct XFORMCOORDS(pub i32);
impl windows_core::TypeKind for XFORMCOORDS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for XFORMCOORDS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("XFORMCOORDS").field(&self.0).finish()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct ARRAYDESC {
    pub tdescElem: super::Com::TYPEDESC,
    pub cDims: u16,
    pub rgbounds: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for ARRAYDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for ARRAYDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
impl windows_core::TypeKind for CADWORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for CADWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
impl windows_core::TypeKind for CALPOLESTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for CALPOLESTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
impl windows_core::TypeKind for CAUUID {
    type TypeKind = windows_core::CopyType;
}
impl Default for CAUUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CLEANLOCALSTORAGE {
    pub pInterface: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pStorage: *mut core::ffi::c_void,
    pub flags: u32,
}
impl Clone for CLEANLOCALSTORAGE {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CLEANLOCALSTORAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for CLEANLOCALSTORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for CONTROLINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for CONTROLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVASPECTINFO {
    pub cb: u32,
    pub dwFlags: u32,
}
impl windows_core::TypeKind for DVASPECTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVASPECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVEXTENTINFO {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::super::Foundation::SIZE,
}
impl windows_core::TypeKind for DVEXTENTINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVEXTENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: windows_core::PWSTR,
    pub cySize: super::Com::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: super::super::Foundation::BOOL,
    pub fUnderline: super::super::Foundation::BOOL,
    pub fStrikethrough: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for FONTDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for FONTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct INTERFACEDATA {
    pub pmethdata: *mut METHODDATA,
    pub cMembers: u32,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for INTERFACEDATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for INTERFACEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: super::super::Foundation::BOOL,
    pub fLicVerified: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for LICINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for LICINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct METHODDATA {
    pub szName: windows_core::PWSTR,
    pub ppdata: *mut PARAMDATA,
    pub dispid: i32,
    pub iMeth: u32,
    pub cc: super::Com::CALLCONV,
    pub cArgs: u32,
    pub wFlags: u16,
    pub vtReturn: super::Variant::VARENUM,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for METHODDATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl Default for METHODDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NUMPARSE {
    pub cDig: i32,
    pub dwInFlags: NUMPARSE_FLAGS,
    pub dwOutFlags: NUMPARSE_FLAGS,
    pub cchUsed: i32,
    pub nBaseShift: i32,
    pub nPwr10: i32,
}
impl windows_core::TypeKind for NUMPARSE {
    type TypeKind = windows_core::CopyType;
}
impl Default for NUMPARSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OBJECTDESCRIPTOR {
    pub cbSize: u32,
    pub clsid: windows_core::GUID,
    pub dwDrawAspect: u32,
    pub sizel: super::super::Foundation::SIZE,
    pub pointl: super::super::Foundation::POINTL,
    pub dwStatus: u32,
    pub dwFullUserTypeName: u32,
    pub dwSrcOfCopy: u32,
}
impl windows_core::TypeKind for OBJECTDESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
impl Default for OBJECTDESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::super::Foundation::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: windows_core::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut Option<windows_core::IUnknown>,
    pub cPages: u32,
    pub lpPages: *mut windows_core::GUID,
    pub lcid: u32,
    pub dispidInitialProperty: i32,
}
impl windows_core::TypeKind for OCPFIPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLECMD {
    pub cmdID: u32,
    pub cmdf: u32,
}
impl windows_core::TypeKind for OLECMD {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLECMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLECMDTEXT {
    pub cmdtextf: u32,
    pub cwActual: u32,
    pub cwBuf: u32,
    pub rgwz: [u16; 1],
}
impl windows_core::TypeKind for OLECMDTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLECMDTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLEINPLACEFRAMEINFO {
    pub cb: u32,
    pub fMDIApp: super::super::Foundation::BOOL,
    pub hwndFrame: super::super::Foundation::HWND,
    pub haccel: super::super::UI::WindowsAndMessaging::HACCEL,
    pub cAccelEntries: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for OLEINPLACEFRAMEINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OLEINPLACEFRAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLEMENUGROUPWIDTHS {
    pub width: [i32; 6],
}
impl windows_core::TypeKind for OLEMENUGROUPWIDTHS {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEMENUGROUPWIDTHS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Media")]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIBUSYA {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Media")]
impl windows_core::TypeKind for OLEUIBUSYA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Media")]
impl Default for OLEUIBUSYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Media")]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIBUSYW {
    pub cbStruct: u32,
    pub dwFlags: BUSY_DIALOG_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hTask: super::super::Media::HTASK,
    pub lphWndDialog: *mut super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Media")]
impl windows_core::TypeKind for OLEUIBUSYW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Media")]
impl Default for OLEUIBUSYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OLEUICHANGEICONA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub clsid: windows_core::GUID,
    pub szIconExe: [i8; 260],
    pub cchIconExe: i32,
}
impl windows_core::TypeKind for OLEUICHANGEICONA {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUICHANGEICONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OLEUICHANGEICONW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_ICON_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub clsid: windows_core::GUID,
    pub szIconExe: [u16; 260],
    pub cchIconExe: i32,
}
impl windows_core::TypeKind for OLEUICHANGEICONW {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUICHANGEICONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[derive(Debug)]
pub struct OLEUICHANGESOURCEA {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: core::mem::ManuallyDrop<Option<IOleUILinkContainerA>>,
    pub dwLink: u32,
    pub lpszDisplayName: windows_core::PSTR,
    pub nFileLength: u32,
    pub lpszFrom: windows_core::PSTR,
    pub lpszTo: windows_core::PSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Clone for OLEUICHANGESOURCEA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl windows_core::TypeKind for OLEUICHANGESOURCEA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Default for OLEUICHANGESOURCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
#[derive(Debug)]
pub struct OLEUICHANGESOURCEW {
    pub cbStruct: u32,
    pub dwFlags: CHANGE_SOURCE_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOFN: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
    pub dwReserved1: [u32; 4],
    pub lpOleUILinkContainer: core::mem::ManuallyDrop<Option<IOleUILinkContainerW>>,
    pub dwLink: u32,
    pub lpszDisplayName: windows_core::PWSTR,
    pub nFileLength: u32,
    pub lpszFrom: windows_core::PWSTR,
    pub lpszTo: windows_core::PWSTR,
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Clone for OLEUICHANGESOURCEW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl windows_core::TypeKind for OLEUICHANGESOURCEW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
impl Default for OLEUICHANGESOURCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OLEUICONVERTA {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_core::GUID,
    pub clsidConvertDefault: windows_core::GUID,
    pub clsidActivateDefault: windows_core::GUID,
    pub clsidNew: windows_core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub lpszUserType: windows_core::PSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: windows_core::PSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
}
impl windows_core::TypeKind for OLEUICONVERTA {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUICONVERTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OLEUICONVERTW {
    pub cbStruct: u32,
    pub dwFlags: UI_CONVERT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_core::GUID,
    pub clsidConvertDefault: windows_core::GUID,
    pub clsidActivateDefault: windows_core::GUID,
    pub clsidNew: windows_core::GUID,
    pub dvAspect: u32,
    pub wFormat: u16,
    pub fIsLinkedObject: super::super::Foundation::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub lpszUserType: windows_core::PWSTR,
    pub fObjectsIconChanged: super::super::Foundation::BOOL,
    pub lpszDefLabel: windows_core::PWSTR,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
}
impl windows_core::TypeKind for OLEUICONVERTW {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUICONVERTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct OLEUIEDITLINKSA {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: core::mem::ManuallyDrop<Option<IOleUILinkContainerA>>,
}
impl Clone for OLEUIEDITLINKSA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for OLEUIEDITLINKSA {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUIEDITLINKSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct OLEUIEDITLINKSW {
    pub cbStruct: u32,
    pub dwFlags: EDIT_LINKS_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpOleUILinkContainer: core::mem::ManuallyDrop<Option<IOleUILinkContainerW>>,
}
impl Clone for OLEUIEDITLINKSW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for OLEUIEDITLINKSW {
    type TypeKind = windows_core::CopyType;
}
impl Default for OLEUIEDITLINKSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIGNRLPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIGNRLPROPSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIGNRLPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIGNRLPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIGNRLPROPSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIGNRLPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Debug)]
pub struct OLEUIINSERTOBJECTA {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_core::GUID,
    pub lpszFile: windows_core::PSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
    pub iid: windows_core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: core::mem::ManuallyDrop<Option<IOleClientSite>>,
    pub lpIStorage: core::mem::ManuallyDrop<Option<super::Com::StructuredStorage::IStorage>>,
    pub ppvObj: *mut *mut core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Clone for OLEUIINSERTOBJECTA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::TypeKind for OLEUIINSERTOBJECTA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OLEUIINSERTOBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
#[derive(Debug)]
pub struct OLEUIINSERTOBJECTW {
    pub cbStruct: u32,
    pub dwFlags: INSERT_OBJECT_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub clsid: windows_core::GUID,
    pub lpszFile: windows_core::PWSTR,
    pub cchFile: u32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
    pub iid: windows_core::GUID,
    pub oleRender: u32,
    pub lpFormatEtc: *mut super::Com::FORMATETC,
    pub lpIOleClientSite: core::mem::ManuallyDrop<Option<IOleClientSite>>,
    pub lpIStorage: core::mem::ManuallyDrop<Option<super::Com::StructuredStorage::IStorage>>,
    pub ppvObj: *mut *mut core::ffi::c_void,
    pub sc: i32,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Clone for OLEUIINSERTOBJECTW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::TypeKind for OLEUIINSERTOBJECTW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl Default for OLEUIINSERTOBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUILINKPROPSA {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUILINKPROPSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUILINKPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUILINKPROPSW {
    pub cbStruct: u32,
    pub dwFlags: u32,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUILINKPROPSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUILINKPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Debug, Eq, PartialEq)]
pub struct OLEUIOBJECTPROPSA {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERA_V2,
    pub dwObject: u32,
    pub lpObjInfo: core::mem::ManuallyDrop<Option<IOleUIObjInfoA>>,
    pub dwLink: u32,
    pub lpLinkInfo: core::mem::ManuallyDrop<Option<IOleUILinkInfoA>>,
    pub lpGP: *mut OLEUIGNRLPROPSA,
    pub lpVP: *mut OLEUIVIEWPROPSA,
    pub lpLP: *mut OLEUILINKPROPSA,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Clone for OLEUIOBJECTPROPSA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIOBJECTPROPSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIOBJECTPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Debug, Eq, PartialEq)]
pub struct OLEUIOBJECTPROPSW {
    pub cbStruct: u32,
    pub dwFlags: OBJECT_PROPERTIES_FLAGS,
    pub lpPS: *mut super::super::UI::Controls::PROPSHEETHEADERW_V2,
    pub dwObject: u32,
    pub lpObjInfo: core::mem::ManuallyDrop<Option<IOleUIObjInfoW>>,
    pub dwLink: u32,
    pub lpLinkInfo: core::mem::ManuallyDrop<Option<IOleUILinkInfoW>>,
    pub lpGP: *mut OLEUIGNRLPROPSW,
    pub lpVP: *mut OLEUIVIEWPROPSW,
    pub lpLP: *mut OLEUILINKPROPSW,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Clone for OLEUIOBJECTPROPSW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIOBJECTPROPSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIOBJECTPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLEUIPASTEENTRYA {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: windows_core::PCSTR,
    pub lpstrResultText: windows_core::PCSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for OLEUIPASTEENTRYA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTEENTRYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLEUIPASTEENTRYW {
    pub fmtetc: super::Com::FORMATETC,
    pub lpstrFormatName: windows_core::PCWSTR,
    pub lpstrResultText: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub dwScratchSpace: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for OLEUIPASTEENTRYW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTEENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug)]
pub struct OLEUIPASTESPECIALA {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: core::mem::ManuallyDrop<Option<super::Com::IDataObject>>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYA,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for OLEUIPASTESPECIALA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for OLEUIPASTESPECIALA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTESPECIALA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Debug)]
pub struct OLEUIPASTESPECIALW {
    pub cbStruct: u32,
    pub dwFlags: PASTE_SPECIAL_FLAGS,
    pub hWndOwner: super::super::Foundation::HWND,
    pub lpszCaption: windows_core::PCWSTR,
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub lpszTemplate: windows_core::PCWSTR,
    pub hResource: super::super::Foundation::HRSRC,
    pub lpSrcDataObj: core::mem::ManuallyDrop<Option<super::Com::IDataObject>>,
    pub arrPasteEntries: *mut OLEUIPASTEENTRYW,
    pub cPasteEntries: i32,
    pub arrLinkTypes: *mut u32,
    pub cLinkTypes: i32,
    pub cClsidExclude: u32,
    pub lpClsidExclude: *mut windows_core::GUID,
    pub nSelectedIndex: i32,
    pub fLink: super::super::Foundation::BOOL,
    pub hMetaPict: super::super::Foundation::HGLOBAL,
    pub sizel: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for OLEUIPASTESPECIALW {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for OLEUIPASTESPECIALW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for OLEUIPASTESPECIALW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIVIEWPROPSA {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSA,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIVIEWPROPSA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIVIEWPROPSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug)]
pub struct OLEUIVIEWPROPSW {
    pub cbStruct: u32,
    pub dwFlags: VIEW_OBJECT_PROPERTIES_FLAGS,
    pub dwReserved1: [u32; 2],
    pub lpfnHook: LPFNOLEUIHOOK,
    pub lCustData: super::super::Foundation::LPARAM,
    pub dwReserved2: [u32; 3],
    pub lpOP: *mut OLEUIOBJECTPROPSW,
    pub nScaleMin: i32,
    pub nScaleMax: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for OLEUIVIEWPROPSW {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for OLEUIVIEWPROPSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OLEVERB {
    pub lVerb: OLEIVERB,
    pub lpszVerbName: windows_core::PWSTR,
    pub fuFlags: super::super::UI::WindowsAndMessaging::MENU_ITEM_FLAGS,
    pub grfAttribs: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for OLEVERB {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OLEVERB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OLE_HANDLE(pub u32);
impl OLE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl Default for OLE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OLE_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGERANGE {
    pub nFromPage: i32,
    pub nToPage: i32,
}
impl windows_core::TypeKind for PAGERANGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for PAGERANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PAGESET {
    pub cbStruct: u32,
    pub fOddPages: super::super::Foundation::BOOL,
    pub fEvenPages: super::super::Foundation::BOOL,
    pub cPageRange: u32,
    pub rgPages: [PAGERANGE; 1],
}
impl windows_core::TypeKind for PAGESET {
    type TypeKind = windows_core::CopyType;
}
impl Default for PAGESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Variant")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PARAMDATA {
    pub szName: windows_core::PWSTR,
    pub vt: super::Variant::VARENUM,
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::TypeKind for PARAMDATA {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Variant")]
impl Default for PARAMDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PARAMDESC {
    pub pparamdescex: *mut PARAMDESCEX,
    pub wParamFlags: PARAMFLAGS,
}
impl windows_core::TypeKind for PARAMDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for PARAMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct PARAMDESCEX {
    pub cBytes: u32,
    pub varDefaultValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for PARAMDESCEX {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for PARAMDESCEX {
    type TypeKind = windows_core::CopyType;
}
impl Default for PARAMDESCEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: u32,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy)]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_3,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_1,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PICTDESC_0_1 {
    pub hemf: super::super::Graphics::Gdi::HENHMETAFILE,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PICTDESC_0_2 {
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC_0_2 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PICTDESC_0_3 {
    pub hmeta: super::super::Graphics::Gdi::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::TypeKind for PICTDESC_0_3 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl Default for PICTDESC_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTF {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for POINTF {
    type TypeKind = windows_core::CopyType;
}
impl Default for POINTF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: windows_core::PWSTR,
    pub size: super::super::Foundation::SIZE,
    pub pszDocString: windows_core::PWSTR,
    pub pszHelpFile: windows_core::PWSTR,
    pub dwHelpContext: u32,
}
impl windows_core::TypeKind for PROPPAGEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for PROPPAGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[derive(Debug, Eq, PartialEq)]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: core::mem::ManuallyDrop<Option<IOleClientSite>>,
    pub pAdviseSink: core::mem::ManuallyDrop<Option<IAdviseSinkEx>>,
    pub pPropertyNotifySink: core::mem::ManuallyDrop<Option<IPropertyNotifySink>>,
    pub pUnkEventSink: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwAmbientFlags: u32,
    pub colorFore: u32,
    pub colorBack: u32,
    pub pFont: core::mem::ManuallyDrop<Option<IFont>>,
    pub pUndoMgr: core::mem::ManuallyDrop<Option<IOleUndoManager>>,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub pBindHost: core::mem::ManuallyDrop<Option<super::Com::IBindHost>>,
    pub pOleControlSite: core::mem::ManuallyDrop<Option<IOleControlSite>>,
    pub pServiceProvider: core::mem::ManuallyDrop<Option<super::Com::IServiceProvider>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl Clone for QACONTAINER {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::TypeKind for QACONTAINER {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl Default for QACONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
impl windows_core::TypeKind for QACONTROL {
    type TypeKind = windows_core::CopyType;
}
impl Default for QACONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct SAFEARRAYUNION {
    pub sfType: u32,
    pub u: SAFEARRAYUNION_0,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAFEARRAYUNION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARRAYUNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union SAFEARRAYUNION_0 {
    pub BstrStr: SAFEARR_BSTR,
    pub UnknownStr: SAFEARR_UNKNOWN,
    pub DispatchStr: SAFEARR_DISPATCH,
    pub VariantStr: SAFEARR_VARIANT,
    pub RecordStr: SAFEARR_BRECORD,
    pub HaveIidStr: SAFEARR_HAVEIID,
    pub ByteStr: super::Com::BYTE_SIZEDARR,
    pub WordStr: super::Com::WORD_SIZEDARR,
    pub LongStr: super::Com::DWORD_SIZEDARR,
    pub HyperStr: super::Com::HYPER_SIZEDARR,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAFEARRAYUNION_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARRAYUNION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_BRECORD {
    pub Size: u32,
    pub aRecord: *mut *mut _wireBRECORD,
}
impl windows_core::TypeKind for SAFEARR_BRECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFEARR_BRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_BSTR {
    pub Size: u32,
    pub aBstr: *mut *mut super::Com::FLAGGED_WORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAFEARR_BSTR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_BSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_DISPATCH {
    pub Size: u32,
    pub apDispatch: *mut Option<super::Com::IDispatch>,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAFEARR_DISPATCH {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_DISPATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_HAVEIID {
    pub Size: u32,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
    pub iid: windows_core::GUID,
}
impl windows_core::TypeKind for SAFEARR_HAVEIID {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFEARR_HAVEIID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_UNKNOWN {
    pub Size: u32,
    pub apUnknown: *mut Option<windows_core::IUnknown>,
}
impl windows_core::TypeKind for SAFEARR_UNKNOWN {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFEARR_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARR_VARIANT {
    pub Size: u32,
    pub aVariant: *mut *mut _wireVARIANT,
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for SAFEARR_VARIANT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for SAFEARR_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UDATE {
    pub st: super::super::Foundation::SYSTEMTIME,
    pub wDayOfYear: u16,
}
impl windows_core::TypeKind for UDATE {
    type TypeKind = windows_core::CopyType;
}
impl Default for UDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct _wireBRECORD {
    pub fFlags: u32,
    pub clSize: u32,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
    pub pRecord: *mut u8,
}
impl Clone for _wireBRECORD {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for _wireBRECORD {
    type TypeKind = windows_core::CopyType;
}
impl Default for _wireBRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct _wireSAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub uArrayStructs: SAFEARRAYUNION,
    pub rgsabound: [super::Com::SAFEARRAYBOUND; 1],
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for _wireSAFEARRAY {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireSAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub struct _wireVARIANT {
    pub clSize: u32,
    pub rpcReserved: u32,
    pub vt: u16,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: _wireVARIANT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for _wireVARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for _wireVARIANT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union _wireVARIANT_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: super::super::Foundation::VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: super::Com::CY,
    pub date: f64,
    pub bstrVal: *mut super::Com::FLAGGED_WORD_BLOB,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<super::Com::IDispatch>>,
    pub parray: *mut *mut _wireSAFEARRAY,
    pub brecVal: *mut _wireBRECORD,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut super::super::Foundation::VARIANT_BOOL,
    pub pscode: *mut i32,
    pub pcyVal: *mut super::Com::CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut *mut super::Com::FLAGGED_WORD_BLOB,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<super::Com::IDispatch>,
    pub pparray: *mut *mut *mut _wireSAFEARRAY,
    pub pvarVal: *mut *mut _wireVARIANT,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub decVal: super::super::Foundation::DECIMAL,
    pub pdecVal: *mut super::super::Foundation::DECIMAL,
    pub pcVal: windows_core::PSTR,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Clone for _wireVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for _wireVARIANT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl Default for _wireVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPFNOLEUIHOOK = Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::WPARAM, param3: super::super::Foundation::LPARAM) -> u32>;
pub type OLESTREAMQUERYCONVERTOLELINKCALLBACK = Option<unsafe extern "system" fn(pclsid: *const windows_core::GUID, szclass: windows_core::PCWSTR, sztopicname: windows_core::PCWSTR, szitemname: windows_core::PCWSTR, szuncname: windows_core::PCWSTR, linkupdatingoption: u32, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
