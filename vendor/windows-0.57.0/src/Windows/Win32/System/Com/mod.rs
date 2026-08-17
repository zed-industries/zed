#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
#[inline]
pub unsafe fn BindMoniker<P0, T>(pmk: P0, grfopt: u32) -> windows_core::Result<T>
where
    P0: windows_core::Param<IMoniker>,
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn BindMoniker(pmk : * mut core::ffi::c_void, grfopt : u32, iidresult : *const windows_core::GUID, ppvresult : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    BindMoniker(pmk.param().abi(), grfopt, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CLSIDFromProgID<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CLSIDFromProgID(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CLSIDFromProgID(lpszprogid.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CLSIDFromProgIDEx<P0>(lpszprogid: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CLSIDFromProgIDEx(lpszprogid : windows_core::PCWSTR, lpclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CLSIDFromProgIDEx(lpszprogid.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CLSIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CLSIDFromString(lpsz : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CLSIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoAddRefServerProcess() -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CoAddRefServerProcess() -> u32);
    CoAddRefServerProcess()
}
#[inline]
pub unsafe fn CoAllowSetForegroundWindow<P0>(punk: P0, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoAllowSetForegroundWindow(punk : * mut core::ffi::c_void, lpvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    CoAllowSetForegroundWindow(punk.param().abi(), core::mem::transmute(lpvreserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CoAllowUnmarshalerCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoAllowUnmarshalerCLSID(clsid : *const windows_core::GUID) -> windows_core::HRESULT);
    CoAllowUnmarshalerCLSID(clsid).ok()
}
#[inline]
pub unsafe fn CoBuildVersion() -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CoBuildVersion() -> u32);
    CoBuildVersion()
}
#[inline]
pub unsafe fn CoCancelCall(dwthreadid: u32, ultimeout: u32) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoCancelCall(dwthreadid : u32, ultimeout : u32) -> windows_core::HRESULT);
    CoCancelCall(dwthreadid, ultimeout).ok()
}
#[inline]
pub unsafe fn CoCopyProxy<P0>(pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoCopyProxy(pproxy : * mut core::ffi::c_void, ppcopy : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoCopyProxy(pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoCreateFreeThreadedMarshaler<P0>(punkouter: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter : * mut core::ffi::c_void, ppunkmarshal : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoCreateFreeThreadedMarshaler(punkouter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoCreateGuid() -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("ole32.dll" "system" fn CoCreateGuid(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoCreateGuid(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoCreateInstance<P0, T>(rclsid: *const windows_core::GUID, punkouter: P0, dwclscontext: CLSCTX) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoCreateInstance(rclsid, punkouter.param().abi(), dwclscontext, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoCreateInstanceEx<P0>(clsid: *const windows_core::GUID, punkouter: P0, dwclsctx: CLSCTX, pserverinfo: Option<*const COSERVERINFO>, presults: &mut [MULTI_QI]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoCreateInstanceEx(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : CLSCTX, pserverinfo : *const COSERVERINFO, dwcount : u32, presults : *mut MULTI_QI) -> windows_core::HRESULT);
    CoCreateInstanceEx(clsid, punkouter.param().abi(), dwclsctx, core::mem::transmute(pserverinfo.unwrap_or(std::ptr::null())), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok()
}
#[inline]
pub unsafe fn CoCreateInstanceFromApp<P0>(clsid: *const windows_core::GUID, punkouter: P0, dwclsctx: CLSCTX, reserved: Option<*const core::ffi::c_void>, presults: &mut [MULTI_QI]) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoCreateInstanceFromApp(clsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclsctx : CLSCTX, reserved : *const core::ffi::c_void, dwcount : u32, presults : *mut MULTI_QI) -> windows_core::HRESULT);
    CoCreateInstanceFromApp(clsid, punkouter.param().abi(), dwclsctx, core::mem::transmute(reserved.unwrap_or(std::ptr::null())), presults.len().try_into().unwrap(), core::mem::transmute(presults.as_ptr())).ok()
}
#[inline]
pub unsafe fn CoDecrementMTAUsage<P0>(cookie: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<CO_MTA_USAGE_COOKIE>,
{
    windows_targets::link!("ole32.dll" "system" fn CoDecrementMTAUsage(cookie : CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    CoDecrementMTAUsage(cookie.param().abi()).ok()
}
#[inline]
pub unsafe fn CoDisableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoDisableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    CoDisableCallCancellation(core::mem::transmute(preserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CoDisconnectContext(dwtimeout: u32) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoDisconnectContext(dwtimeout : u32) -> windows_core::HRESULT);
    CoDisconnectContext(dwtimeout).ok()
}
#[inline]
pub unsafe fn CoDisconnectObject<P0>(punk: P0, dwreserved: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoDisconnectObject(punk : * mut core::ffi::c_void, dwreserved : u32) -> windows_core::HRESULT);
    CoDisconnectObject(punk.param().abi(), dwreserved).ok()
}
#[inline]
pub unsafe fn CoDosDateTimeToFileTime(ndosdate: u16, ndostime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    windows_targets::link!("ole32.dll" "system" fn CoDosDateTimeToFileTime(ndosdate : u16, ndostime : u16, lpfiletime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    CoDosDateTimeToFileTime(ndosdate, ndostime, lpfiletime)
}
#[inline]
pub unsafe fn CoEnableCallCancellation(preserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoEnableCallCancellation(preserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    CoEnableCallCancellation(core::mem::transmute(preserved.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CoFileTimeNow() -> windows_core::Result<super::super::Foundation::FILETIME> {
    windows_targets::link!("ole32.dll" "system" fn CoFileTimeNow(lpfiletime : *mut super::super::Foundation:: FILETIME) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoFileTimeNow(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoFileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpdosdate: *mut u16, lpdostime: *mut u16) -> super::super::Foundation::BOOL {
    windows_targets::link!("ole32.dll" "system" fn CoFileTimeToDosDateTime(lpfiletime : *const super::super::Foundation:: FILETIME, lpdosdate : *mut u16, lpdostime : *mut u16) -> super::super::Foundation:: BOOL);
    CoFileTimeToDosDateTime(lpfiletime, lpdosdate, lpdostime)
}
#[inline]
pub unsafe fn CoFreeAllLibraries() {
    windows_targets::link!("ole32.dll" "system" fn CoFreeAllLibraries());
    CoFreeAllLibraries()
}
#[inline]
pub unsafe fn CoFreeLibrary<P0>(hinst: P0)
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
{
    windows_targets::link!("ole32.dll" "system" fn CoFreeLibrary(hinst : super::super::Foundation:: HINSTANCE));
    CoFreeLibrary(hinst.param().abi())
}
#[inline]
pub unsafe fn CoFreeUnusedLibraries() {
    windows_targets::link!("ole32.dll" "system" fn CoFreeUnusedLibraries());
    CoFreeUnusedLibraries()
}
#[inline]
pub unsafe fn CoFreeUnusedLibrariesEx(dwunloaddelay: u32, dwreserved: u32) {
    windows_targets::link!("ole32.dll" "system" fn CoFreeUnusedLibrariesEx(dwunloaddelay : u32, dwreserved : u32));
    CoFreeUnusedLibrariesEx(dwunloaddelay, dwreserved)
}
#[inline]
pub unsafe fn CoGetApartmentType(papttype: *mut APTTYPE, paptqualifier: *mut APTTYPEQUALIFIER) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoGetApartmentType(papttype : *mut APTTYPE, paptqualifier : *mut APTTYPEQUALIFIER) -> windows_core::HRESULT);
    CoGetApartmentType(papttype, paptqualifier).ok()
}
#[inline]
pub unsafe fn CoGetCallContext<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetCallContext(riid : *const windows_core::GUID, ppinterface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoGetCallContext(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetCallerTID() -> windows_core::Result<u32> {
    windows_targets::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetCallerTID(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoGetCancelObject<T>(dwthreadid: u32) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetCancelObject(dwthreadid : u32, iid : *const windows_core::GUID, ppunk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoGetCancelObject(dwthreadid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetClassObject<T>(rclsid: *const windows_core::GUID, dwclscontext: CLSCTX, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetClassObject(rclsid : *const windows_core::GUID, dwclscontext : u32, pvreserved : *const core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoGetClassObject(rclsid, dwclscontext.0 as _, core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetContextToken() -> windows_core::Result<usize> {
    windows_targets::link!("ole32.dll" "system" fn CoGetContextToken(ptoken : *mut usize) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetContextToken(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoGetCurrentLogicalThreadId() -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("ole32.dll" "system" fn CoGetCurrentLogicalThreadId(pguid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetCurrentLogicalThreadId(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoGetCurrentProcess() -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CoGetCurrentProcess() -> u32);
    CoGetCurrentProcess()
}
#[inline]
pub unsafe fn CoGetMalloc(dwmemcontext: u32) -> windows_core::Result<IMalloc> {
    windows_targets::link!("ole32.dll" "system" fn CoGetMalloc(dwmemcontext : u32, ppmalloc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetMalloc(dwmemcontext, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetObject<P0, T>(pszname: P0, pbindoptions: Option<*const BIND_OPTS>) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetObject(pszname : windows_core::PCWSTR, pbindoptions : *const BIND_OPTS, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoGetObject(pszname.param().abi(), core::mem::transmute(pbindoptions.unwrap_or(std::ptr::null())), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetObjectContext<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CoGetObjectContext(riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CoGetObjectContext(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoGetPSClsid(riid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
    windows_targets::link!("ole32.dll" "system" fn CoGetPSClsid(riid : *const windows_core::GUID, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoGetPSClsid(riid, &mut result__).map(|| result__)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoGetSystemSecurityPermissions(comsdtype: COMSD, ppsd: *mut super::super::Security::PSECURITY_DESCRIPTOR) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoGetSystemSecurityPermissions(comsdtype : COMSD, ppsd : *mut super::super::Security:: PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    CoGetSystemSecurityPermissions(comsdtype, ppsd).ok()
}
#[inline]
pub unsafe fn CoGetTreatAsClass(clsidold: *const windows_core::GUID, pclsidnew: *mut windows_core::GUID) -> windows_core::HRESULT {
    windows_targets::link!("ole32.dll" "system" fn CoGetTreatAsClass(clsidold : *const windows_core::GUID, pclsidnew : *mut windows_core::GUID) -> windows_core::HRESULT);
    CoGetTreatAsClass(clsidold, pclsidnew)
}
#[inline]
pub unsafe fn CoImpersonateClient() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoImpersonateClient() -> windows_core::HRESULT);
    CoImpersonateClient().ok()
}
#[inline]
pub unsafe fn CoIncrementMTAUsage() -> windows_core::Result<CO_MTA_USAGE_COOKIE> {
    windows_targets::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoIncrementMTAUsage(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoInitialize(pvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
    windows_targets::link!("ole32.dll" "system" fn CoInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    CoInitialize(core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: Option<*const core::ffi::c_void>, dwcoinit: COINIT) -> windows_core::HRESULT {
    windows_targets::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    CoInitializeEx(core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())), dwcoinit.0 as _)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CoInitializeSecurity<P0>(psecdesc: P0, cauthsvc: i32, asauthsvc: Option<*const SOLE_AUTHENTICATION_SERVICE>, preserved1: Option<*const core::ffi::c_void>, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthlist: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES, preserved3: Option<*const core::ffi::c_void>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    windows_targets::link!("ole32.dll" "system" fn CoInitializeSecurity(psecdesc : super::super::Security:: PSECURITY_DESCRIPTOR, cauthsvc : i32, asauthsvc : *const SOLE_AUTHENTICATION_SERVICE, preserved1 : *const core::ffi::c_void, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthlist : *const core::ffi::c_void, dwcapabilities : u32, preserved3 : *const core::ffi::c_void) -> windows_core::HRESULT);
    CoInitializeSecurity(psecdesc.param().abi(), cauthsvc, core::mem::transmute(asauthsvc.unwrap_or(std::ptr::null())), core::mem::transmute(preserved1.unwrap_or(std::ptr::null())), dwauthnlevel, dwimplevel, core::mem::transmute(pauthlist.unwrap_or(std::ptr::null())), dwcapabilities.0 as _, core::mem::transmute(preserved3.unwrap_or(std::ptr::null()))).ok()
}
#[inline]
pub unsafe fn CoInstall<P0, P1>(pbc: P0, dwflags: u32, pclassspec: *const uCLSSPEC, pquery: *const QUERYCONTEXT, pszcodebase: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CoInstall(pbc : * mut core::ffi::c_void, dwflags : u32, pclassspec : *const uCLSSPEC, pquery : *const QUERYCONTEXT, pszcodebase : windows_core::PCWSTR) -> windows_core::HRESULT);
    CoInstall(pbc.param().abi(), dwflags, pclassspec, pquery, pszcodebase.param().abi()).ok()
}
#[inline]
pub unsafe fn CoInvalidateRemoteMachineBindings<P0>(pszmachinename: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CoInvalidateRemoteMachineBindings(pszmachinename : windows_core::PCWSTR) -> windows_core::HRESULT);
    CoInvalidateRemoteMachineBindings(pszmachinename.param().abi()).ok()
}
#[inline]
pub unsafe fn CoIsHandlerConnected<P0>(punk: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : * mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CoIsHandlerConnected(punk.param().abi())
}
#[inline]
pub unsafe fn CoIsOle1Class(rclsid: *const windows_core::GUID) -> super::super::Foundation::BOOL {
    windows_targets::link!("ole32.dll" "system" fn CoIsOle1Class(rclsid : *const windows_core::GUID) -> super::super::Foundation:: BOOL);
    CoIsOle1Class(rclsid)
}
#[inline]
pub unsafe fn CoLoadLibrary<P0, P1>(lpszlibname: P0, bautofree: P1) -> super::super::Foundation::HINSTANCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn CoLoadLibrary(lpszlibname : windows_core::PCWSTR, bautofree : super::super::Foundation:: BOOL) -> super::super::Foundation:: HINSTANCE);
    CoLoadLibrary(lpszlibname.param().abi(), bautofree.param().abi())
}
#[inline]
pub unsafe fn CoLockObjectExternal<P0, P1, P2>(punk: P0, flock: P1, flastunlockreleases: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn CoLockObjectExternal(punk : * mut core::ffi::c_void, flock : super::super::Foundation:: BOOL, flastunlockreleases : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    CoLockObjectExternal(punk.param().abi(), flock.param().abi(), flastunlockreleases.param().abi()).ok()
}
#[inline]
pub unsafe fn CoQueryAuthenticationServices(pcauthsvc: *mut u32, asauthsvc: *mut *mut SOLE_AUTHENTICATION_SERVICE) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoQueryAuthenticationServices(pcauthsvc : *mut u32, asauthsvc : *mut *mut SOLE_AUTHENTICATION_SERVICE) -> windows_core::HRESULT);
    CoQueryAuthenticationServices(pcauthsvc, asauthsvc).ok()
}
#[inline]
pub unsafe fn CoQueryClientBlanket(pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: Option<*mut windows_core::PWSTR>, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: Option<*mut *mut core::ffi::c_void>, pcapabilities: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoQueryClientBlanket(pauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pprivs : *mut *mut core::ffi::c_void, pcapabilities : *mut u32) -> windows_core::HRESULT);
    CoQueryClientBlanket(core::mem::transmute(pauthnsvc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pauthzsvc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pserverprincname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pauthnlevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pimplevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pprivs.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcapabilities.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn CoQueryProxyBlanket<P0>(pproxy: P0, pwauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: Option<*mut windows_core::PWSTR>, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pauthinfo: Option<*mut *mut core::ffi::c_void>, pcapabilites: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoQueryProxyBlanket(pproxy : * mut core::ffi::c_void, pwauthnsvc : *mut u32, pauthzsvc : *mut u32, pserverprincname : *mut windows_core::PWSTR, pauthnlevel : *mut u32, pimplevel : *mut u32, pauthinfo : *mut *mut core::ffi::c_void, pcapabilites : *mut u32) -> windows_core::HRESULT);
    CoQueryProxyBlanket(
        pproxy.param().abi(),
        core::mem::transmute(pwauthnsvc.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pauthzsvc.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pserverprincname.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pauthnlevel.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pimplevel.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pauthinfo.unwrap_or(std::ptr::null_mut())),
        core::mem::transmute(pcapabilites.unwrap_or(std::ptr::null_mut())),
    )
    .ok()
}
#[inline]
pub unsafe fn CoRegisterActivationFilter<P0>(pactivationfilter: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IActivationFilter>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterActivationFilter(pactivationfilter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoRegisterActivationFilter(pactivationfilter.param().abi()).ok()
}
#[inline]
pub unsafe fn CoRegisterChannelHook<P0>(extensionuuid: *const windows_core::GUID, pchannelhook: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IChannelHook>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterChannelHook(extensionuuid : *const windows_core::GUID, pchannelhook : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoRegisterChannelHook(extensionuuid, pchannelhook.param().abi()).ok()
}
#[inline]
pub unsafe fn CoRegisterClassObject<P0>(rclsid: *const windows_core::GUID, punk: P0, dwclscontext: CLSCTX, flags: REGCLS) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterClassObject(rclsid : *const windows_core::GUID, punk : * mut core::ffi::c_void, dwclscontext : CLSCTX, flags : u32, lpdwregister : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoRegisterClassObject(rclsid, punk.param().abi(), dwclscontext, flags.0 as _, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoRegisterDeviceCatalog<P0>(deviceinstanceid: P0) -> windows_core::Result<CO_DEVICE_CATALOG_COOKIE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterDeviceCatalog(deviceinstanceid : windows_core::PCWSTR, cookie : *mut CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoRegisterDeviceCatalog(deviceinstanceid.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoRegisterInitializeSpy<P0>(pspy: P0) -> windows_core::Result<u64>
where
    P0: windows_core::Param<IInitializeSpy>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterInitializeSpy(pspy : * mut core::ffi::c_void, pulicookie : *mut u64) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoRegisterInitializeSpy(pspy.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoRegisterMallocSpy<P0>(pmallocspy: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMallocSpy>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterMallocSpy(pmallocspy : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoRegisterMallocSpy(pmallocspy.param().abi()).ok()
}
#[inline]
pub unsafe fn CoRegisterPSClsid(riid: *const windows_core::GUID, rclsid: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoRegisterPSClsid(riid : *const windows_core::GUID, rclsid : *const windows_core::GUID) -> windows_core::HRESULT);
    CoRegisterPSClsid(riid, rclsid).ok()
}
#[inline]
pub unsafe fn CoRegisterSurrogate<P0>(psurrogate: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<ISurrogate>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRegisterSurrogate(psurrogate : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoRegisterSurrogate(psurrogate.param().abi()).ok()
}
#[inline]
pub unsafe fn CoReleaseServerProcess() -> u32 {
    windows_targets::link!("ole32.dll" "system" fn CoReleaseServerProcess() -> u32);
    CoReleaseServerProcess()
}
#[inline]
pub unsafe fn CoResumeClassObjects() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoResumeClassObjects() -> windows_core::HRESULT);
    CoResumeClassObjects().ok()
}
#[inline]
pub unsafe fn CoRevertToSelf() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoRevertToSelf() -> windows_core::HRESULT);
    CoRevertToSelf().ok()
}
#[inline]
pub unsafe fn CoRevokeClassObject(dwregister: u32) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoRevokeClassObject(dwregister : u32) -> windows_core::HRESULT);
    CoRevokeClassObject(dwregister).ok()
}
#[inline]
pub unsafe fn CoRevokeDeviceCatalog<P0>(cookie: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<CO_DEVICE_CATALOG_COOKIE>,
{
    windows_targets::link!("ole32.dll" "system" fn CoRevokeDeviceCatalog(cookie : CO_DEVICE_CATALOG_COOKIE) -> windows_core::HRESULT);
    CoRevokeDeviceCatalog(cookie.param().abi()).ok()
}
#[inline]
pub unsafe fn CoRevokeInitializeSpy(ulicookie: u64) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoRevokeInitializeSpy(ulicookie : u64) -> windows_core::HRESULT);
    CoRevokeInitializeSpy(ulicookie).ok()
}
#[inline]
pub unsafe fn CoRevokeMallocSpy() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoRevokeMallocSpy() -> windows_core::HRESULT);
    CoRevokeMallocSpy().ok()
}
#[inline]
pub unsafe fn CoSetCancelObject<P0>(punk: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoSetCancelObject(punk : * mut core::ffi::c_void) -> windows_core::HRESULT);
    CoSetCancelObject(punk.param().abi()).ok()
}
#[inline]
pub unsafe fn CoSetProxyBlanket<P0, P1>(pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P1, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CoSetProxyBlanket(pproxy : * mut core::ffi::c_void, dwauthnsvc : u32, dwauthzsvc : u32, pserverprincname : windows_core::PCWSTR, dwauthnlevel : RPC_C_AUTHN_LEVEL, dwimplevel : RPC_C_IMP_LEVEL, pauthinfo : *const core::ffi::c_void, dwcapabilities : u32) -> windows_core::HRESULT);
    CoSetProxyBlanket(pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.param().abi(), dwauthnlevel, dwimplevel, core::mem::transmute(pauthinfo.unwrap_or(std::ptr::null())), dwcapabilities.0 as _).ok()
}
#[inline]
pub unsafe fn CoSuspendClassObjects() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoSuspendClassObjects() -> windows_core::HRESULT);
    CoSuspendClassObjects().ok()
}
#[inline]
pub unsafe fn CoSwitchCallContext<P0>(pnewobject: P0) -> windows_core::Result<windows_core::IUnknown>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CoSwitchCallContext(pnewobject : * mut core::ffi::c_void, ppoldobject : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoSwitchCallContext(pnewobject.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    CoTaskMemAlloc(cb)
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: Option<*const core::ffi::c_void>) {
    windows_targets::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
    CoTaskMemFree(core::mem::transmute(pv.unwrap_or(std::ptr::null())))
}
#[inline]
pub unsafe fn CoTaskMemRealloc(pv: Option<*const core::ffi::c_void>, cb: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("ole32.dll" "system" fn CoTaskMemRealloc(pv : *const core::ffi::c_void, cb : usize) -> *mut core::ffi::c_void);
    CoTaskMemRealloc(core::mem::transmute(pv.unwrap_or(std::ptr::null())), cb)
}
#[inline]
pub unsafe fn CoTestCancel() -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoTestCancel() -> windows_core::HRESULT);
    CoTestCancel().ok()
}
#[inline]
pub unsafe fn CoTreatAsClass(clsidold: *const windows_core::GUID, clsidnew: *const windows_core::GUID) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn CoTreatAsClass(clsidold : *const windows_core::GUID, clsidnew : *const windows_core::GUID) -> windows_core::HRESULT);
    CoTreatAsClass(clsidold, clsidnew).ok()
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_targets::link!("ole32.dll" "system" fn CoUninitialize());
    CoUninitialize()
}
#[inline]
pub unsafe fn CoWaitForMultipleHandles(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
    windows_targets::link!("ole32.dll" "system" fn CoWaitForMultipleHandles(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation:: HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoWaitForMultipleHandles(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CoWaitForMultipleObjects(dwflags: u32, dwtimeout: u32, phandles: &[super::super::Foundation::HANDLE]) -> windows_core::Result<u32> {
    windows_targets::link!("ole32.dll" "system" fn CoWaitForMultipleObjects(dwflags : u32, dwtimeout : u32, chandles : u32, phandles : *const super::super::Foundation:: HANDLE, lpdwindex : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CoWaitForMultipleObjects(dwflags, dwtimeout, phandles.len().try_into().unwrap(), core::mem::transmute(phandles.as_ptr()), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn CreateAntiMoniker() -> windows_core::Result<IMoniker> {
    windows_targets::link!("ole32.dll" "system" fn CreateAntiMoniker(ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateAntiMoniker(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateBindCtx(reserved: u32) -> windows_core::Result<IBindCtx> {
    windows_targets::link!("ole32.dll" "system" fn CreateBindCtx(reserved : u32, ppbc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateBindCtx(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateClassMoniker(rclsid: *const windows_core::GUID) -> windows_core::Result<IMoniker> {
    windows_targets::link!("ole32.dll" "system" fn CreateClassMoniker(rclsid : *const windows_core::GUID, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateClassMoniker(rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateDataAdviseHolder() -> windows_core::Result<IDataAdviseHolder> {
    windows_targets::link!("ole32.dll" "system" fn CreateDataAdviseHolder(ppdaholder : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateDataAdviseHolder(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateDataCache<P0, T>(punkouter: P0, rclsid: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_targets::link!("ole32.dll" "system" fn CreateDataCache(punkouter : * mut core::ffi::c_void, rclsid : *const windows_core::GUID, iid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    CreateDataCache(punkouter.param().abi(), rclsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateFileMoniker<P0>(lpszpathname: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CreateFileMoniker(lpszpathname : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateFileMoniker(lpszpathname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateGenericComposite<P0, P1>(pmkfirst: P0, pmkrest: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
{
    windows_targets::link!("ole32.dll" "system" fn CreateGenericComposite(pmkfirst : * mut core::ffi::c_void, pmkrest : * mut core::ffi::c_void, ppmkcomposite : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateGenericComposite(pmkfirst.param().abi(), pmkrest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateIUriBuilder<P0>(piuri: P0, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUriBuilder>
where
    P0: windows_core::Param<IUri>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateIUriBuilder(piuri : * mut core::ffi::c_void, dwflags : u32, dwreserved : usize, ppiuribuilder : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateIUriBuilder(piuri.param().abi(), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateItemMoniker<P0, P1>(lpszdelim: P0, lpszitem: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn CreateItemMoniker(lpszdelim : windows_core::PCWSTR, lpszitem : windows_core::PCWSTR, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateItemMoniker(lpszdelim.param().abi(), lpszitem.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateObjrefMoniker<P0>(punk: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CreateObjrefMoniker(punk : * mut core::ffi::c_void, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateObjrefMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreatePointerMoniker<P0>(punk: P0) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("ole32.dll" "system" fn CreatePointerMoniker(punk : * mut core::ffi::c_void, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreatePointerMoniker(punk.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateStdProgressIndicator<P0, P1, P2>(hwndparent: P0, psztitle: P1, pibsccaller: P2) -> windows_core::Result<IBindStatusCallback>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IBindStatusCallback>,
{
    windows_targets::link!("ole32.dll" "system" fn CreateStdProgressIndicator(hwndparent : super::super::Foundation:: HWND, psztitle : windows_core::PCWSTR, pibsccaller : * mut core::ffi::c_void, ppibsc : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateStdProgressIndicator(hwndparent.param().abi(), psztitle.param().abi(), pibsccaller.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateUri<P0>(pwzuri: P0, dwflags: URI_CREATE_FLAGS, dwreserved: usize) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateUri(pwzuri : windows_core::PCWSTR, dwflags : URI_CREATE_FLAGS, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateUri(pwzuri.param().abi(), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateUriFromMultiByteString<P0>(pszansiinputuri: P0, dwencodingflags: u32, dwcodepage: u32, dwcreateflags: u32, dwreserved: usize) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateUriFromMultiByteString(pszansiinputuri : windows_core::PCSTR, dwencodingflags : u32, dwcodepage : u32, dwcreateflags : u32, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateUriFromMultiByteString(pszansiinputuri.param().abi(), dwencodingflags, dwcodepage, dwcreateflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn CreateUriWithFragment<P0, P1>(pwzuri: P0, pwzfragment: P1, dwflags: u32, dwreserved: usize) -> windows_core::Result<IUri>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("urlmon.dll" "system" fn CreateUriWithFragment(pwzuri : windows_core::PCWSTR, pwzfragment : windows_core::PCWSTR, dwflags : u32, dwreserved : usize, ppuri : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    CreateUriWithFragment(pwzuri.param().abi(), pwzfragment.param().abi(), dwflags, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn DcomChannelSetHResult(pvreserved: Option<*const core::ffi::c_void>, pulreserved: Option<*const u32>, appshr: windows_core::HRESULT) -> windows_core::Result<()> {
    windows_targets::link!("ole32.dll" "system" fn DcomChannelSetHResult(pvreserved : *const core::ffi::c_void, pulreserved : *const u32, appshr : windows_core::HRESULT) -> windows_core::HRESULT);
    DcomChannelSetHResult(core::mem::transmute(pvreserved.unwrap_or(std::ptr::null())), core::mem::transmute(pulreserved.unwrap_or(std::ptr::null())), appshr).ok()
}
#[inline]
pub unsafe fn GetClassFile<P0>(szfilename: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn GetClassFile(szfilename : windows_core::PCWSTR, pclsid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetClassFile(szfilename.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetErrorInfo(dwreserved: u32) -> windows_core::Result<IErrorInfo> {
    windows_targets::link!("oleaut32.dll" "system" fn GetErrorInfo(dwreserved : u32, pperrinfo : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetErrorInfo(dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn GetRunningObjectTable(reserved: u32) -> windows_core::Result<IRunningObjectTable> {
    windows_targets::link!("ole32.dll" "system" fn GetRunningObjectTable(reserved : u32, pprot : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetRunningObjectTable(reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn IIDFromString<P0>(lpsz: P0) -> windows_core::Result<windows_core::GUID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn IIDFromString(lpsz : windows_core::PCWSTR, lpiid : *mut windows_core::GUID) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    IIDFromString(lpsz.param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn MkParseDisplayName<P0, P1>(pbc: P0, szusername: P1, pcheaten: *mut u32, ppmk: *mut Option<IMoniker>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IBindCtx>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("ole32.dll" "system" fn MkParseDisplayName(pbc : * mut core::ffi::c_void, szusername : windows_core::PCWSTR, pcheaten : *mut u32, ppmk : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    MkParseDisplayName(pbc.param().abi(), szusername.param().abi(), pcheaten, core::mem::transmute(ppmk)).ok()
}
#[inline]
pub unsafe fn MonikerCommonPrefixWith<P0, P1>(pmkthis: P0, pmkother: P1) -> windows_core::Result<IMoniker>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
{
    windows_targets::link!("ole32.dll" "system" fn MonikerCommonPrefixWith(pmkthis : * mut core::ffi::c_void, pmkother : * mut core::ffi::c_void, ppmkcommon : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    MonikerCommonPrefixWith(pmkthis.param().abi(), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn MonikerRelativePathTo<P0, P1, P2>(pmksrc: P0, pmkdest: P1, ppmkrelpath: *mut Option<IMoniker>, dwreserved: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<IMoniker>,
    P1: windows_core::Param<IMoniker>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("ole32.dll" "system" fn MonikerRelativePathTo(pmksrc : * mut core::ffi::c_void, pmkdest : * mut core::ffi::c_void, ppmkrelpath : *mut * mut core::ffi::c_void, dwreserved : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    MonikerRelativePathTo(pmksrc.param().abi(), pmkdest.param().abi(), core::mem::transmute(ppmkrelpath), dwreserved.param().abi()).ok()
}
#[inline]
pub unsafe fn ProgIDFromCLSID(clsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("ole32.dll" "system" fn ProgIDFromCLSID(clsid : *const windows_core::GUID, lplpszprogid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ProgIDFromCLSID(clsid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn SetErrorInfo<P0>(dwreserved: u32, perrinfo: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IErrorInfo>,
{
    windows_targets::link!("oleaut32.dll" "system" fn SetErrorInfo(dwreserved : u32, perrinfo : * mut core::ffi::c_void) -> windows_core::HRESULT);
    SetErrorInfo(dwreserved, perrinfo.param().abi()).ok()
}
#[inline]
pub unsafe fn StringFromCLSID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("ole32.dll" "system" fn StringFromCLSID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    StringFromCLSID(rclsid, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn StringFromGUID2(rguid: *const windows_core::GUID, lpsz: &mut [u16]) -> i32 {
    windows_targets::link!("ole32.dll" "system" fn StringFromGUID2(rguid : *const windows_core::GUID, lpsz : windows_core::PWSTR, cchmax : i32) -> i32);
    StringFromGUID2(rguid, core::mem::transmute(lpsz.as_ptr()), lpsz.len().try_into().unwrap())
}
#[inline]
pub unsafe fn StringFromIID(rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("ole32.dll" "system" fn StringFromIID(rclsid : *const windows_core::GUID, lplpsz : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    StringFromIID(rclsid, &mut result__).map(|| result__)
}
windows_core::imp::define_interface!(AsyncIAdviseSink, AsyncIAdviseSink_Vtbl, 0x00000150_0000_0000_c000_000000000046);
impl core::ops::Deref for AsyncIAdviseSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIAdviseSink, windows_core::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (windows_core::Interface::vtable(self).Begin_OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        (windows_core::Interface::vtable(self).Finish_OnDataChange)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (windows_core::Interface::vtable(self).Begin_OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        (windows_core::Interface::vtable(self).Finish_OnViewChange)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).Begin_OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi())
    }
    pub unsafe fn Finish_OnRename(&self) {
        (windows_core::Interface::vtable(self).Finish_OnRename)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnSave(&self) {
        (windows_core::Interface::vtable(self).Begin_OnSave)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnSave(&self) {
        (windows_core::Interface::vtable(self).Finish_OnSave)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_OnClose(&self) {
        (windows_core::Interface::vtable(self).Begin_OnClose)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Finish_OnClose(&self) {
        (windows_core::Interface::vtable(self).Finish_OnClose)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct AsyncIAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub Begin_OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    Begin_OnDataChange: usize,
    pub Finish_OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32),
    pub Finish_OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Finish_OnRename: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Finish_OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Finish_OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(AsyncIAdviseSink2, AsyncIAdviseSink2_Vtbl, 0x00000151_0000_0000_c000_000000000046);
impl core::ops::Deref for AsyncIAdviseSink2 {
    type Target = AsyncIAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIAdviseSink2, windows_core::IUnknown, AsyncIAdviseSink);
impl AsyncIAdviseSink2 {
    pub unsafe fn Begin_OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).Begin_OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi())
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        (windows_core::Interface::vtable(self).Finish_OnLinkSrcChange)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base__: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(AsyncIMultiQI, AsyncIMultiQI_Vtbl, 0x000e0020_0000_0000_c000_000000000046);
impl core::ops::Deref for AsyncIMultiQI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIMultiQI, windows_core::IUnknown);
impl AsyncIMultiQI {
    pub unsafe fn Begin_QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())).ok()
    }
    pub unsafe fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis).ok()
    }
}
#[repr(C)]
pub struct AsyncIMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
    pub Finish_QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MULTI_QI) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIPipeByte, AsyncIPipeByte_Vtbl, 0xdb2f3acb_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for AsyncIPipeByte {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIPipeByte, windows_core::IUnknown);
impl AsyncIPipeByte {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIPipeDouble, AsyncIPipeDouble_Vtbl, 0xdb2f3acf_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for AsyncIPipeDouble {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIPipeDouble, windows_core::IUnknown);
impl AsyncIPipeDouble {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIPipeLong, AsyncIPipeLong_Vtbl, 0xdb2f3acd_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for AsyncIPipeLong {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIPipeLong, windows_core::IUnknown);
impl AsyncIPipeLong {
    pub unsafe fn Begin_Pull(&self, crequest: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Pull)(windows_core::Interface::as_raw(self), crequest).ok()
    }
    pub unsafe fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Pull)(windows_core::Interface::as_raw(self), buf, pcreturned).ok()
    }
    pub unsafe fn Begin_Push(&self, buf: &[i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Finish_Push(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_Push)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct AsyncIPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Finish_Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut u32) -> windows_core::HRESULT,
    pub Begin_Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
    pub Finish_Push: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(AsyncIUnknown, AsyncIUnknown_Vtbl, 0x000e0000_0000_0000_c000_000000000046);
impl core::ops::Deref for AsyncIUnknown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIUnknown, windows_core::IUnknown);
impl AsyncIUnknown {
    pub unsafe fn Begin_QueryInterface(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_QueryInterface)(windows_core::Interface::as_raw(self), riid).ok()
    }
    pub unsafe fn Finish_QueryInterface(&self, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Finish_QueryInterface)(windows_core::Interface::as_raw(self), ppvobject).ok()
    }
    pub unsafe fn Begin_AddRef(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_AddRef)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_AddRef(&self) -> u32 {
        (windows_core::Interface::vtable(self).Finish_AddRef)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Begin_Release(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Begin_Release)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_Release(&self) -> u32 {
        (windows_core::Interface::vtable(self).Finish_Release)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct AsyncIUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Finish_QueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Begin_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_AddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Begin_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finish_Release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(IActivationFilter, IActivationFilter_Vtbl, 0x00000017_0000_0000_c000_000000000046);
impl core::ops::Deref for IActivationFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IActivationFilter, windows_core::IUnknown);
impl IActivationFilter {
    pub unsafe fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HandleActivation)(windows_core::Interface::as_raw(self), dwactivationtype, rclsid, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IActivationFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleActivation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAddrExclusionControl, IAddrExclusionControl_Vtbl, 0x00000148_0000_0000_c000_000000000046);
impl core::ops::Deref for IAddrExclusionControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAddrExclusionControl, windows_core::IUnknown);
impl IAddrExclusionControl {
    pub unsafe fn GetCurrentAddrExclusionList(&self, riid: *const windows_core::GUID, ppenumerator: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCurrentAddrExclusionList)(windows_core::Interface::as_raw(self), riid, ppenumerator).ok()
    }
    pub unsafe fn UpdateAddrExclusionList<P0>(&self, penumerator: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).UpdateAddrExclusionList)(windows_core::Interface::as_raw(self), penumerator.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IAddrExclusionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateAddrExclusionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAddrTrackingControl, IAddrTrackingControl_Vtbl, 0x00000147_0000_0000_c000_000000000046);
impl core::ops::Deref for IAddrTrackingControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAddrTrackingControl, windows_core::IUnknown);
impl IAddrTrackingControl {
    pub unsafe fn EnableCOMDynamicAddrTracking(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableCOMDynamicAddrTracking(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DisableCOMDynamicAddrTracking)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IAddrTrackingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableCOMDynamicAddrTracking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAdviseSink, IAdviseSink_Vtbl, 0x0000010f_0000_0000_c000_000000000046);
impl core::ops::Deref for IAdviseSink {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdviseSink, windows_core::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        (windows_core::Interface::vtable(self).OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, pstgmed)
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        (windows_core::Interface::vtable(self).OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex)
    }
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi())
    }
    pub unsafe fn OnSave(&self) {
        (windows_core::Interface::vtable(self).OnSave)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn OnClose(&self) {
        (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM),
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataChange: usize,
    pub OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32),
    pub OnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IAdviseSink2, IAdviseSink2_Vtbl, 0x00000125_0000_0000_c000_000000000046);
impl core::ops::Deref for IAdviseSink2 {
    type Target = IAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdviseSink2, windows_core::IUnknown, IAdviseSink);
impl IAdviseSink2 {
    pub unsafe fn OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi())
    }
}
#[repr(C)]
pub struct IAdviseSink2_Vtbl {
    pub base__: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IAgileObject, IAgileObject_Vtbl, 0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
impl core::ops::Deref for IAgileObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAgileObject, windows_core::IUnknown);
impl IAgileObject {}
#[repr(C)]
pub struct IAgileObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IAsyncManager, IAsyncManager_Vtbl, 0x0000002a_0000_0000_c000_000000000046);
impl core::ops::Deref for IAsyncManager {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncManager, windows_core::IUnknown);
impl IAsyncManager {
    pub unsafe fn CompleteCall(&self, result: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CompleteCall)(windows_core::Interface::as_raw(self), result).ok()
    }
    pub unsafe fn GetCallContext(&self, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), riid, pinterface).ok()
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IAsyncManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompleteCall: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAsyncRpcChannelBuffer, IAsyncRpcChannelBuffer_Vtbl, 0xa5029fb6_3c34_11d1_9c99_00c04fb998aa);
impl core::ops::Deref for IAsyncRpcChannelBuffer {
    type Target = IRpcChannelBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAsyncRpcChannelBuffer, windows_core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl IAsyncRpcChannelBuffer {
    pub unsafe fn Send<P0>(&self, pmsg: *mut RPCOLEMESSAGE, psync: P0, pulstatus: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISynchronize>,
    {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg, psync.param().abi(), pulstatus).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg, pulstatus).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext, core::mem::transmute(ppvdestcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IAsyncRpcChannelBuffer_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAuthenticate, IAuthenticate_Vtbl, 0x79eac9d0_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IAuthenticate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAuthenticate, windows_core::IUnknown);
impl IAuthenticate {
    pub unsafe fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Authenticate)(windows_core::Interface::as_raw(self), phwnd, pszusername, pszpassword).ok()
    }
}
#[repr(C)]
pub struct IAuthenticate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Authenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAuthenticateEx, IAuthenticateEx_Vtbl, 0x2ad1edaf_d83d_48b5_9adf_03dbe19f53bd);
impl core::ops::Deref for IAuthenticateEx {
    type Target = IAuthenticate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAuthenticateEx, windows_core::IUnknown, IAuthenticate);
impl IAuthenticateEx {
    pub unsafe fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut windows_core::PWSTR, pszpassword: *mut windows_core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AuthenticateEx)(windows_core::Interface::as_raw(self), phwnd, pszusername, pszpassword, pauthinfo).ok()
    }
}
#[repr(C)]
pub struct IAuthenticateEx_Vtbl {
    pub base__: IAuthenticate_Vtbl,
    pub AuthenticateEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND, *mut windows_core::PWSTR, *mut windows_core::PWSTR, *const AUTHENTICATEINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindCtx, IBindCtx_Vtbl, 0x0000000e_0000_0000_c000_000000000046);
impl core::ops::Deref for IBindCtx {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RegisterObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn RevokeObjectBound<P0>(&self, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RevokeObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()).ok()
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseBoundObjects)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetBindOptions)(windows_core::Interface::as_raw(self), pbindopts).ok()
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindOptions)(windows_core::Interface::as_raw(self), pbindopts).ok()
    }
    pub unsafe fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRunningObjectTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterObjectParam<P0, P1>(&self, pszkey: P0, punk: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).RegisterObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), punk.param().abi()).ok()
    }
    pub unsafe fn GetObjectParam<P0>(&self, pszkey: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumObjectParam(&self) -> windows_core::Result<IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumObjectParam)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RevokeObjectParam<P0>(&self, pszkey: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).RevokeObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBindCtx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterObjectBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObjectBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseBoundObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBindOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const BIND_OPTS) -> windows_core::HRESULT,
    pub GetBindOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BIND_OPTS) -> windows_core::HRESULT,
    pub GetRunningObjectTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindHost, IBindHost_Vtbl, 0xfc4801a1_2ba9_11cf_a229_00aa003d7352);
impl core::ops::Deref for IBindHost {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindHost, windows_core::IUnknown);
impl IBindHost {
    pub unsafe fn CreateMoniker<P0, P1>(&self, szname: P0, pbc: P1, ppmk: *mut Option<IMoniker>, dwreserved: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IBindCtx>,
    {
        (windows_core::Interface::vtable(self).CreateMoniker)(windows_core::Interface::as_raw(self), szname.param().abi(), pbc.param().abi(), core::mem::transmute(ppmk), dwreserved).ok()
    }
    pub unsafe fn MonikerBindToStorage<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
        P1: windows_core::Param<IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
    {
        (windows_core::Interface::vtable(self).MonikerBindToStorage)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), riid, ppvobj).ok()
    }
    pub unsafe fn MonikerBindToObject<P0, P1, P2>(&self, pmk: P0, pbc: P1, pbsc: P2, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
        P1: windows_core::Param<IBindCtx>,
        P2: windows_core::Param<IBindStatusCallback>,
    {
        (windows_core::Interface::vtable(self).MonikerBindToObject)(windows_core::Interface::as_raw(self), pmk.param().abi(), pbc.param().abi(), pbsc.param().abi(), riid, ppvobj).ok()
    }
}
#[repr(C)]
pub struct IBindHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MonikerBindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonikerBindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindStatusCallback, IBindStatusCallback_Vtbl, 0x79eac9c1_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IBindStatusCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindStatusCallback, windows_core::IUnknown);
impl IBindStatusCallback {
    pub unsafe fn OnStartBinding<P0>(&self, dwreserved: u32, pib: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBinding>,
    {
        (windows_core::Interface::vtable(self).OnStartBinding)(windows_core::Interface::as_raw(self), dwreserved, pib.param().abi()).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OnLowResource(&self, reserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnLowResource)(windows_core::Interface::as_raw(self), reserved).ok()
    }
    pub unsafe fn OnProgress<P0>(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), ulprogress, ulprogressmax, ulstatuscode, szstatustext.param().abi()).ok()
    }
    pub unsafe fn OnStopBinding<P0>(&self, hresult: windows_core::HRESULT, szerror: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).OnStopBinding)(windows_core::Interface::as_raw(self), hresult, szerror.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindInfo)(windows_core::Interface::as_raw(self), grfbindf, pbindinfo).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnDataAvailable)(windows_core::Interface::as_raw(self), grfbscf, dwsize, pformatetc, pstgmed).ok()
    }
    pub unsafe fn OnObjectAvailable<P0>(&self, riid: *const windows_core::GUID, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).OnObjectAvailable)(windows_core::Interface::as_raw(self), riid, punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IBindStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStartBinding: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub OnLowResource: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnStopBinding: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfo: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub OnDataAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const FORMATETC, *const STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    OnDataAvailable: usize,
    pub OnObjectAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBindStatusCallbackEx, IBindStatusCallbackEx_Vtbl, 0xaaa74ef9_8ee7_4659_88d9_f8c504da73cc);
impl core::ops::Deref for IBindStatusCallbackEx {
    type Target = IBindStatusCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBindStatusCallbackEx, windows_core::IUnknown, IBindStatusCallback);
impl IBindStatusCallbackEx {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindInfoEx)(windows_core::Interface::as_raw(self), grfbindf, pbindinfo, grfbindf2, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IBindStatusCallbackEx_Vtbl {
    pub base__: IBindStatusCallback_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetBindInfoEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut BINDINFO, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage")))]
    GetBindInfoEx: usize,
}
windows_core::imp::define_interface!(IBinding, IBinding_Vtbl, 0x79eac9c0_baf9_11ce_8c82_00aa004ba90b);
impl core::ops::Deref for IBinding {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBinding, windows_core::IUnknown);
impl IBinding {
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Suspend(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPriority(&self, npriority: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), npriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBindResult(&self, pclsidprotocol: *mut windows_core::GUID, pdwresult: *mut u32, pszresult: *mut windows_core::PWSTR, pdwreserved: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBindResult)(windows_core::Interface::as_raw(self), pclsidprotocol, pdwresult, pszresult, pdwreserved).ok()
    }
}
#[repr(C)]
pub struct IBinding_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetBindResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBlockingLock, IBlockingLock_Vtbl, 0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
impl core::ops::Deref for IBlockingLock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IBlockingLock, windows_core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwtimeout).ok()
    }
    pub unsafe fn Unlock(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IBlockingLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICallFactory, ICallFactory_Vtbl, 0x1c733a30_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ICallFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICallFactory, windows_core::IUnknown);
impl ICallFactory {
    pub unsafe fn CreateCall<P0>(&self, riid: *const windows_core::GUID, pctrlunk: P0, riid2: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateCall)(windows_core::Interface::as_raw(self), riid, pctrlunk.param().abi(), riid2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICallFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateCall: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICancelMethodCalls, ICancelMethodCalls_Vtbl, 0x00000029_0000_0000_c000_000000000046);
impl core::ops::Deref for ICancelMethodCalls {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICancelMethodCalls, windows_core::IUnknown);
impl ICancelMethodCalls {
    pub unsafe fn Cancel(&self, ulseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), ulseconds).ok()
    }
    pub unsafe fn TestCancel(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).TestCancel)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ICancelMethodCalls_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TestCancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICatInformation, ICatInformation_Vtbl, 0x0002e013_0000_0000_c000_000000000046);
impl core::ops::Deref for ICatInformation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICatInformation, windows_core::IUnknown);
impl ICatInformation {
    pub unsafe fn EnumCategories(&self, lcid: u32) -> windows_core::Result<IEnumCATEGORYINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumCategories)(windows_core::Interface::as_raw(self), lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetCategoryDesc(&self, rcatid: *const windows_core::GUID, lcid: u32) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCategoryDesc)(windows_core::Interface::as_raw(self), rcatid, lcid, &mut result__).map(|| result__)
    }
    pub unsafe fn EnumClassesOfCategories(&self, rgcatidimpl: &[windows_core::GUID], rgcatidreq: &[windows_core::GUID]) -> windows_core::Result<IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumClassesOfCategories)(windows_core::Interface::as_raw(self), rgcatidimpl.len().try_into().unwrap(), core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len().try_into().unwrap(), core::mem::transmute(rgcatidreq.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsClassOfCategories(&self, rclsid: *const windows_core::GUID, rgcatidimpl: &[windows_core::GUID], rgcatidreq: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsClassOfCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatidimpl.len().try_into().unwrap(), core::mem::transmute(rgcatidimpl.as_ptr()), rgcatidreq.len().try_into().unwrap(), core::mem::transmute(rgcatidreq.as_ptr())).ok()
    }
    pub unsafe fn EnumImplCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumImplCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EnumReqCategoriesOfClass(&self, rclsid: *const windows_core::GUID) -> windows_core::Result<IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumReqCategoriesOfClass)(windows_core::Interface::as_raw(self), rclsid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICatInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCategoryDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub EnumClassesOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsClassOfCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub EnumImplCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumReqCategoriesOfClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICatRegister, ICatRegister_Vtbl, 0x0002e012_0000_0000_c000_000000000046);
impl core::ops::Deref for ICatRegister {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICatRegister, windows_core::IUnknown);
impl ICatRegister {
    pub unsafe fn RegisterCategories(&self, rgcategoryinfo: &[CATEGORYINFO]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterCategories)(windows_core::Interface::as_raw(self), rgcategoryinfo.len().try_into().unwrap(), core::mem::transmute(rgcategoryinfo.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterCategories(&self, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnRegisterCategories)(windows_core::Interface::as_raw(self), rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn RegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterClassImplCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnRegisterClassImplCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn RegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok()
    }
    pub unsafe fn UnRegisterClassReqCategories(&self, rclsid: *const windows_core::GUID, rgcatid: &[windows_core::GUID]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnRegisterClassReqCategories)(windows_core::Interface::as_raw(self), rclsid, rgcatid.len().try_into().unwrap(), core::mem::transmute(rgcatid.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct ICatRegister_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CATEGORYINFO) -> windows_core::HRESULT,
    pub UnRegisterCategories: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnRegisterClassImplCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub UnRegisterClassReqCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IChannelHook, IChannelHook_Vtbl, 0x1008c4a0_7613_11cf_9af1_0020af6e72f4);
impl core::ops::Deref for IChannelHook {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IChannelHook, windows_core::IUnknown);
impl IChannelHook {
    pub unsafe fn ClientGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID) -> u32 {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientGetSize)(windows_core::Interface::as_raw(self), uextent, riid, &mut result__);
        result__
    }
    pub unsafe fn ClientFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void) {
        (windows_core::Interface::vtable(self).ClientFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize, pdatabuffer)
    }
    pub unsafe fn ClientNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32, hrfault: windows_core::HRESULT) {
        (windows_core::Interface::vtable(self).ClientNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep, hrfault)
    }
    pub unsafe fn ServerNotify(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, cbdatasize: u32, pdatabuffer: *const core::ffi::c_void, ldatarep: u32) {
        (windows_core::Interface::vtable(self).ServerNotify)(windows_core::Interface::as_raw(self), uextent, riid, cbdatasize, pdatabuffer, ldatarep)
    }
    pub unsafe fn ServerGetSize(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, hrfault: windows_core::HRESULT) -> u32 {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerGetSize)(windows_core::Interface::as_raw(self), uextent, riid, hrfault, &mut result__);
        result__
    }
    pub unsafe fn ServerFillBuffer(&self, uextent: *const windows_core::GUID, riid: *const windows_core::GUID, pdatasize: *mut u32, pdatabuffer: *const core::ffi::c_void, hrfault: windows_core::HRESULT) {
        (windows_core::Interface::vtable(self).ServerFillBuffer)(windows_core::Interface::as_raw(self), uextent, riid, pdatasize, pdatabuffer, hrfault)
    }
}
#[repr(C)]
pub struct IChannelHook_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ClientGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32),
    pub ClientFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void),
    pub ClientNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32, windows_core::HRESULT),
    pub ServerNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, u32, *const core::ffi::c_void, u32),
    pub ServerGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, windows_core::HRESULT, *mut u32),
    pub ServerFillBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut u32, *const core::ffi::c_void, windows_core::HRESULT),
}
windows_core::imp::define_interface!(IClassActivator, IClassActivator_Vtbl, 0x00000140_0000_0000_c000_000000000046);
impl core::ops::Deref for IClassActivator {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IClassActivator, windows_core::IUnknown);
impl IClassActivator {
    pub unsafe fn GetClassObject<T>(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetClassObject)(windows_core::Interface::as_raw(self), rclsid, dwclasscontext, locale, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IClassActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClassFactory, IClassFactory_Vtbl, 0x00000001_0000_0000_c000_000000000046);
impl core::ops::Deref for IClassFactory {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IClassFactory, windows_core::IUnknown);
impl IClassFactory {
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LockServer<P0>(&self, flock: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LockServer)(windows_core::Interface::as_raw(self), flock.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockServer: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClientSecurity, IClientSecurity_Vtbl, 0x0000013d_0000_0000_c000_000000000046);
impl core::ops::Deref for IClientSecurity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IClientSecurity, windows_core::IUnknown);
impl IClientSecurity {
    pub unsafe fn QueryBlanket<P0>(&self, pproxy: P0, pauthnsvc: *mut u32, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: Option<*mut RPC_C_AUTHN_LEVEL>, pimplevel: Option<*mut RPC_C_IMP_LEVEL>, pauthinfo: *mut *mut core::ffi::c_void, pcapabilites: Option<*mut u32>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), pauthnsvc, core::mem::transmute(pauthzsvc.unwrap_or(std::ptr::null_mut())), pserverprincname, core::mem::transmute(pauthnlevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pimplevel.unwrap_or(std::ptr::null_mut())), pauthinfo, core::mem::transmute(pcapabilites.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetBlanket<P0, P1>(&self, pproxy: P0, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: P1, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: Option<*const core::ffi::c_void>, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetBlanket)(windows_core::Interface::as_raw(self), pproxy.param().abi(), dwauthnsvc, dwauthzsvc, pserverprincname.param().abi(), dwauthnlevel, dwimplevel, core::mem::transmute(pauthinfo.unwrap_or(std::ptr::null())), dwcapabilities.0 as _).ok()
    }
    pub unsafe fn CopyProxy<P0>(&self, pproxy: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CopyProxy)(windows_core::Interface::as_raw(self), pproxy.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IClientSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut u16, *mut RPC_C_AUTHN_LEVEL, *mut RPC_C_IMP_LEVEL, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, windows_core::PCWSTR, RPC_C_AUTHN_LEVEL, RPC_C_IMP_LEVEL, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CopyProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IComThreadingInfo, IComThreadingInfo_Vtbl, 0x000001ce_0000_0000_c000_000000000046);
impl core::ops::Deref for IComThreadingInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IComThreadingInfo, windows_core::IUnknown);
impl IComThreadingInfo {
    pub unsafe fn GetCurrentApartmentType(&self) -> windows_core::Result<APTTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentApartmentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentThreadType(&self) -> windows_core::Result<THDTYPE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentThreadType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetCurrentLogicalThreadId(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCurrentLogicalThreadId(&self, rguid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetCurrentLogicalThreadId)(windows_core::Interface::as_raw(self), rguid).ok()
    }
}
#[repr(C)]
pub struct IComThreadingInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentApartmentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut APTTYPE) -> windows_core::HRESULT,
    pub GetCurrentThreadType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut THDTYPE) -> windows_core::HRESULT,
    pub GetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetCurrentLogicalThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionPoint, IConnectionPoint_Vtbl, 0xb196b286_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IConnectionPoint {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConnectionPoint, windows_core::IUnknown);
impl IConnectionPoint {
    pub unsafe fn GetConnectionInterface(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionInterface)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnectionPointContainer(&self) -> windows_core::Result<IConnectionPointContainer> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetConnectionPointContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Advise<P0>(&self, punksink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), punksink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn EnumConnections(&self) -> windows_core::Result<IEnumConnections> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumConnections)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IConnectionPoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectionInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetConnectionPointContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumConnections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IConnectionPointContainer, IConnectionPointContainer_Vtbl, 0xb196b284_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IConnectionPointContainer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConnectionPointContainer, windows_core::IUnknown);
impl IConnectionPointContainer {
    pub unsafe fn EnumConnectionPoints(&self) -> windows_core::Result<IEnumConnectionPoints> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumConnectionPoints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn FindConnectionPoint(&self, riid: *const windows_core::GUID) -> windows_core::Result<IConnectionPoint> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FindConnectionPoint)(windows_core::Interface::as_raw(self), riid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IConnectionPointContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumConnectionPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindConnectionPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContext, IContext_Vtbl, 0x000001c0_0000_0000_c000_000000000046);
impl core::ops::Deref for IContext {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContext, windows_core::IUnknown);
impl IContext {
    pub unsafe fn SetProperty<P0>(&self, rpolicyid: *const windows_core::GUID, flags: u32, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), rpolicyid, flags, punk.param().abi()).ok()
    }
    pub unsafe fn RemoveProperty(&self, rpolicyid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveProperty)(windows_core::Interface::as_raw(self), rpolicyid).ok()
    }
    pub unsafe fn GetProperty(&self, rguid: *const windows_core::GUID, pflags: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), rguid, pflags, core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn EnumContextProps(&self) -> windows_core::Result<IEnumContextProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumContextProps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumContextProps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextCallback, IContextCallback_Vtbl, 0x000001da_0000_0000_c000_000000000046);
impl core::ops::Deref for IContextCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IContextCallback, windows_core::IUnknown);
impl IContextCallback {
    pub unsafe fn ContextCallback<P0>(&self, pfncallback: PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const windows_core::GUID, imethod: i32, punk: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ContextCallback)(windows_core::Interface::as_raw(self), pfncallback, pparam, riid, imethod, punk.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IContextCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ContextCallback: unsafe extern "system" fn(*mut core::ffi::c_void, PFNCONTEXTCALL, *const ComCallData, *const windows_core::GUID, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataAdviseHolder, IDataAdviseHolder_Vtbl, 0x00000110_0000_0000_c000_000000000046);
impl core::ops::Deref for IDataAdviseHolder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataAdviseHolder, windows_core::IUnknown);
impl IDataAdviseHolder {
    pub unsafe fn Advise<P0, P1>(&self, pdataobject: P0, pfetc: *const FORMATETC, advf: u32, padvise: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDataObject>,
        P1: windows_core::Param<IAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), pfetc, advf, padvise.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection).ok()
    }
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SendOnDataChange<P0>(&self, pdataobject: P0, dwreserved: u32, advf: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDataObject>,
    {
        (windows_core::Interface::vtable(self).SendOnDataChange)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), dwreserved, advf).ok()
    }
}
#[repr(C)]
pub struct IDataAdviseHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataObject, IDataObject_Vtbl, 0x0000010e_0000_0000_c000_000000000046);
impl core::ops::Deref for IDataObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDataObject, windows_core::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pformatetcin, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDataHere)(windows_core::Interface::as_raw(self), pformatetc, pmedium).ok()
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).QueryGetData)(windows_core::Interface::as_raw(self), pformatetc)
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).GetCanonicalFormatEtc)(windows_core::Interface::as_raw(self), pformatectin, pformatetcout)
    }
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetData<P0>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pformatetc, pmedium, frelease.param().abi()).ok()
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumFormatEtc)(windows_core::Interface::as_raw(self), dwdirection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DAdvise<P0>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IAdviseSink>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DAdvise)(windows_core::Interface::as_raw(self), pformatetc, advf, padvsink.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DUnadvise)(windows_core::Interface::as_raw(self), dwconnection).ok()
    }
    pub unsafe fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumDAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IDataObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetData: usize,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetDataHere: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    GetDataHere: usize,
    pub QueryGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC) -> windows_core::HRESULT,
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut FORMATETC) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
impl core::ops::Deref for IDispatch {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> windows_core::Result<ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), itinfo, lcid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, rgdispid).ok()
    }
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const windows_core::GUID, lcid: u32, wflags: DISPATCH_FLAGS, pdispparams: *const DISPPARAMS, pvarresult: Option<*mut windows_core::VARIANT>, pexcepinfo: Option<*mut EXCEPINFO>, puargerr: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), dispidmember, riid, lcid, wflags, pdispparams, core::mem::transmute(pvarresult.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pexcepinfo.unwrap_or(std::ptr::null_mut())), core::mem::transmute(puargerr.unwrap_or(std::ptr::null_mut()))).ok()
    }
}
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::PCWSTR, u32, u32, *mut i32) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::GUID, u32, DISPATCH_FLAGS, *const DISPPARAMS, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumCATEGORYINFO, IEnumCATEGORYINFO_Vtbl, 0x0002e011_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumCATEGORYINFO {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumCATEGORYINFO, windows_core::IUnknown);
impl IEnumCATEGORYINFO {
    pub unsafe fn Next(&self, rgelt: &mut [CATEGORYINFO], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumCATEGORYINFO> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumCATEGORYINFO_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CATEGORYINFO, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumConnectionPoints, IEnumConnectionPoints_Vtbl, 0xb196b285_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IEnumConnectionPoints {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumConnectionPoints, windows_core::IUnknown);
impl IEnumConnectionPoints {
    pub unsafe fn Next(&self, ppcp: &mut [Option<IConnectionPoint>], pcfetched: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppcp.len().try_into().unwrap(), core::mem::transmute(ppcp.as_ptr()), pcfetched)
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumConnectionPoints> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumConnectionPoints_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumConnections, IEnumConnections_Vtbl, 0xb196b287_bab4_101a_b69c_00aa00341d07);
impl core::ops::Deref for IEnumConnections {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumConnections, windows_core::IUnknown);
impl IEnumConnections {
    pub unsafe fn Next(&self, rgcd: &mut [CONNECTDATA], pcfetched: *mut u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgcd.len().try_into().unwrap(), core::mem::transmute(rgcd.as_ptr()), pcfetched)
    }
    pub unsafe fn Skip(&self, cconnections: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cconnections).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumConnections> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumConnections_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CONNECTDATA, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumContextProps, IEnumContextProps_Vtbl, 0x000001c1_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumContextProps {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumContextProps, windows_core::IUnknown);
impl IEnumContextProps {
    pub unsafe fn Next(&self, pcontextproperties: &mut [ContextProperty], pceltfetched: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), pcontextproperties.len().try_into().unwrap(), core::mem::transmute(pcontextproperties.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumContextProps> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEnumContextProps_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ContextProperty, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumFORMATETC, IEnumFORMATETC_Vtbl, 0x00000103_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumFORMATETC {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumFORMATETC, windows_core::IUnknown);
impl IEnumFORMATETC {
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumFORMATETC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumFORMATETC_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut FORMATETC, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumGUID, IEnumGUID_Vtbl, 0x0002e000_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumGUID {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumGUID, windows_core::IUnknown);
impl IEnumGUID {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::GUID], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumGUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumGUID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumMoniker, IEnumMoniker_Vtbl, 0x00000102_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumMoniker {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumMoniker, windows_core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IMoniker>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumSTATDATA, IEnumSTATDATA_Vtbl, 0x00000105_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumSTATDATA {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumSTATDATA, windows_core::IUnknown);
impl IEnumSTATDATA {
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumSTATDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumSTATDATA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATDATA, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumString, IEnumString_Vtbl, 0x00000101_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumString {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumString, windows_core::IUnknown);
impl IEnumString {
    pub unsafe fn Next(&self, rgelt: &mut [windows_core::PWSTR], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumString> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumString_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnumUnknown, IEnumUnknown_Vtbl, 0x00000100_0000_0000_c000_000000000046);
impl core::ops::Deref for IEnumUnknown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumUnknown, windows_core::IUnknown);
impl IEnumUnknown {
    pub unsafe fn Next(&self, rgelt: &mut [Option<windows_core::IUnknown>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), core::mem::transmute(pceltfetched.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEnumUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorInfo, IErrorInfo_Vtbl, 0x1cf2b120_547d_101b_8e65_08002b2bd119);
impl core::ops::Deref for IErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IErrorInfo, windows_core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetGUID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHelpFile(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelpFile)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHelpContext(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelpContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IErrorLog, IErrorLog_Vtbl, 0x3127ca40_446e_11ce_8135_00aa004bb851);
impl core::ops::Deref for IErrorLog {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IErrorLog, windows_core::IUnknown);
impl IErrorLog {
    pub unsafe fn AddError<P0>(&self, pszpropname: P0, pexcepinfo: *const EXCEPINFO) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).AddError)(windows_core::Interface::as_raw(self), pszpropname.param().abi(), pexcepinfo).ok()
    }
}
#[repr(C)]
pub struct IErrorLog_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const EXCEPINFO) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExternalConnection, IExternalConnection_Vtbl, 0x00000019_0000_0000_c000_000000000046);
impl core::ops::Deref for IExternalConnection {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IExternalConnection, windows_core::IUnknown);
impl IExternalConnection {
    pub unsafe fn AddConnection(&self, extconn: u32, reserved: u32) -> u32 {
        (windows_core::Interface::vtable(self).AddConnection)(windows_core::Interface::as_raw(self), extconn, reserved)
    }
    pub unsafe fn ReleaseConnection<P0>(&self, extconn: u32, reserved: u32, flastreleasecloses: P0) -> u32
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).ReleaseConnection)(windows_core::Interface::as_raw(self), extconn, reserved, flastreleasecloses.param().abi())
    }
}
#[repr(C)]
pub struct IExternalConnection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> u32,
    pub ReleaseConnection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Foundation::BOOL) -> u32,
}
windows_core::imp::define_interface!(IFastRundown, IFastRundown_Vtbl, 0x00000040_0000_0000_c000_000000000046);
impl core::ops::Deref for IFastRundown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFastRundown, windows_core::IUnknown);
impl IFastRundown {}
#[repr(C)]
pub struct IFastRundown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IForegroundTransfer, IForegroundTransfer_Vtbl, 0x00000145_0000_0000_c000_000000000046);
impl core::ops::Deref for IForegroundTransfer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IForegroundTransfer, windows_core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AllowForegroundTransfer)(windows_core::Interface::as_raw(self), core::mem::transmute(lpvreserved.unwrap_or(std::ptr::null()))).ok()
    }
}
#[repr(C)]
pub struct IForegroundTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalInterfaceTable, IGlobalInterfaceTable_Vtbl, 0x00000146_0000_0000_c000_000000000046);
impl core::ops::Deref for IGlobalInterfaceTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGlobalInterfaceTable, windows_core::IUnknown);
impl IGlobalInterfaceTable {
    pub unsafe fn RegisterInterfaceInGlobal<P0>(&self, punk: P0, riid: *const windows_core::GUID) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterInterfaceInGlobal)(windows_core::Interface::as_raw(self), punk.param().abi(), riid, &mut result__).map(|| result__)
    }
    pub unsafe fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevokeInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetInterfaceFromGlobal)(windows_core::Interface::as_raw(self), dwcookie, riid, ppv).ok()
    }
}
#[repr(C)]
pub struct IGlobalInterfaceTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterInterfaceInGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub RevokeInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInterfaceFromGlobal: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGlobalOptions, IGlobalOptions_Vtbl, 0x0000015b_0000_0000_c000_000000000046);
impl core::ops::Deref for IGlobalOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IGlobalOptions, windows_core::IUnknown);
impl IGlobalOptions {
    pub unsafe fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), dwproperty, dwvalue).ok()
    }
    pub unsafe fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> windows_core::Result<usize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), dwproperty, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGlobalOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, GLOBALOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInitializeSpy, IInitializeSpy_Vtbl, 0x00000034_0000_0000_c000_000000000046);
impl core::ops::Deref for IInitializeSpy {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInitializeSpy, windows_core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreInitialize)(windows_core::Interface::as_raw(self), dwcoinit, dwcurthreadaptrefs).ok()
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PostInitialize)(windows_core::Interface::as_raw(self), hrcoinit, dwcoinit, dwnewthreadaptrefs).ok()
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PreUninitialize)(windows_core::Interface::as_raw(self), dwcurthreadaptrefs).ok()
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).PostUninitialize)(windows_core::Interface::as_raw(self), dwnewthreadaptrefs).ok()
    }
}
#[repr(C)]
pub struct IInitializeSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, u32) -> windows_core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInternalUnknown, IInternalUnknown_Vtbl, 0x00000021_0000_0000_c000_000000000046);
impl core::ops::Deref for IInternalUnknown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInternalUnknown, windows_core::IUnknown);
impl IInternalUnknown {
    pub unsafe fn QueryInternalInterface(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryInternalInterface)(windows_core::Interface::as_raw(self), riid, ppv).ok()
    }
}
#[repr(C)]
pub struct IInternalUnknown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryInternalInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMachineGlobalObjectTable, IMachineGlobalObjectTable_Vtbl, 0x26d709ac_f70b_4421_a96f_d2878fafb00d);
impl core::ops::Deref for IMachineGlobalObjectTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMachineGlobalObjectTable, windows_core::IUnknown);
impl IMachineGlobalObjectTable {
    pub unsafe fn RegisterObject<P0, P1>(&self, clsid: *const windows_core::GUID, identifier: P0, object: P1) -> windows_core::Result<MachineGlobalObjectTableRegistrationToken>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegisterObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), object.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn GetObject<P0, T>(&self, clsid: *const windows_core::GUID, identifier: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), clsid, identifier.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RevokeObject<P0>(&self, token: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<MachineGlobalObjectTableRegistrationToken>,
    {
        (windows_core::Interface::vtable(self).RevokeObject)(windows_core::Interface::as_raw(self), token.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMachineGlobalObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut core::ffi::c_void, *mut MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObject: unsafe extern "system" fn(*mut core::ffi::c_void, MachineGlobalObjectTableRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMalloc, IMalloc_Vtbl, 0x00000002_0000_0000_c000_000000000046);
impl core::ops::Deref for IMalloc {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMalloc, windows_core::IUnknown);
impl IMalloc {
    pub unsafe fn Alloc(&self, cb: usize) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).Alloc)(windows_core::Interface::as_raw(self), cb)
    }
    pub unsafe fn Realloc(&self, pv: Option<*const core::ffi::c_void>, cb: usize) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).Realloc)(windows_core::Interface::as_raw(self), core::mem::transmute(pv.unwrap_or(std::ptr::null())), cb)
    }
    pub unsafe fn Free(&self, pv: Option<*const core::ffi::c_void>) {
        (windows_core::Interface::vtable(self).Free)(windows_core::Interface::as_raw(self), core::mem::transmute(pv.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn GetSize(&self, pv: Option<*const core::ffi::c_void>) -> usize {
        (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), core::mem::transmute(pv.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn DidAlloc(&self, pv: Option<*const core::ffi::c_void>) -> i32 {
        (windows_core::Interface::vtable(self).DidAlloc)(windows_core::Interface::as_raw(self), core::mem::transmute(pv.unwrap_or(std::ptr::null())))
    }
    pub unsafe fn HeapMinimize(&self) {
        (windows_core::Interface::vtable(self).HeapMinimize)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IMalloc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Alloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Realloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    pub Free: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> usize,
    pub DidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> i32,
    pub HeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IMallocSpy, IMallocSpy_Vtbl, 0x0000001d_0000_0000_c000_000000000046);
impl core::ops::Deref for IMallocSpy {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMallocSpy, windows_core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        (windows_core::Interface::vtable(self).PreAlloc)(windows_core::Interface::as_raw(self), cbrequest)
    }
    pub unsafe fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void {
        (windows_core::Interface::vtable(self).PostAlloc)(windows_core::Interface::as_raw(self), pactual)
    }
    pub unsafe fn PreFree<P0>(&self, prequest: *const core::ffi::c_void, fspyed: P0) -> *mut core::ffi::c_void
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PreFree)(windows_core::Interface::as_raw(self), prequest, fspyed.param().abi())
    }
    pub unsafe fn PostFree<P0>(&self, fspyed: P0)
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PostFree)(windows_core::Interface::as_raw(self), fspyed.param().abi())
    }
    pub unsafe fn PreRealloc<P0>(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: P0) -> usize
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PreRealloc)(windows_core::Interface::as_raw(self), prequest, cbrequest, ppnewrequest, fspyed.param().abi())
    }
    pub unsafe fn PostRealloc<P0>(&self, pactual: *const core::ffi::c_void, fspyed: P0) -> *mut core::ffi::c_void
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PostRealloc)(windows_core::Interface::as_raw(self), pactual, fspyed.param().abi())
    }
    pub unsafe fn PreGetSize<P0>(&self, prequest: *const core::ffi::c_void, fspyed: P0) -> *mut core::ffi::c_void
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PreGetSize)(windows_core::Interface::as_raw(self), prequest, fspyed.param().abi())
    }
    pub unsafe fn PostGetSize<P0>(&self, cbactual: usize, fspyed: P0) -> usize
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PostGetSize)(windows_core::Interface::as_raw(self), cbactual, fspyed.param().abi())
    }
    pub unsafe fn PreDidAlloc<P0>(&self, prequest: *const core::ffi::c_void, fspyed: P0) -> *mut core::ffi::c_void
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PreDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.param().abi())
    }
    pub unsafe fn PostDidAlloc<P0>(&self, prequest: *const core::ffi::c_void, fspyed: P0, factual: i32) -> i32
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).PostDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.param().abi(), factual)
    }
    pub unsafe fn PreHeapMinimize(&self) {
        (windows_core::Interface::vtable(self).PreHeapMinimize)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn PostHeapMinimize(&self) {
        (windows_core::Interface::vtable(self).PostHeapMinimize)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IMallocSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> *mut core::ffi::c_void,
    pub PreFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::BOOL) -> *mut core::ffi::c_void,
    pub PostFree: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL),
    pub PreRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void, super::super::Foundation::BOOL) -> usize,
    pub PostRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::BOOL) -> *mut core::ffi::c_void,
    pub PreGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::BOOL) -> *mut core::ffi::c_void,
    pub PostGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, usize, super::super::Foundation::BOOL) -> usize,
    pub PreDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::BOOL) -> *mut core::ffi::c_void,
    pub PostDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::super::Foundation::BOOL, i32) -> i32,
    pub PreHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IMoniker, IMoniker_Vtbl, 0x0000000f_0000_0000_c000_000000000046);
impl core::ops::Deref for IMoniker {
    type Target = IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMoniker, windows_core::IUnknown, IPersist, IPersistStream);
impl IMoniker {
    pub unsafe fn BindToObject<P0, P1, T>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn BindToStorage<P0, P1, T>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).BindToStorage)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Reduce<P0>(&self, pbc: P0, dwreducehowfar: u32, ppmktoleft: *mut Option<IMoniker>, ppmkreduced: *mut Option<IMoniker>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
    {
        (windows_core::Interface::vtable(self).Reduce)(windows_core::Interface::as_raw(self), pbc.param().abi(), dwreducehowfar, core::mem::transmute(ppmktoleft), core::mem::transmute(ppmkreduced)).ok()
    }
    pub unsafe fn ComposeWith<P0, P1>(&self, pmkright: P0, fonlyifnotgeneric: P1) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ComposeWith)(windows_core::Interface::as_raw(self), pmkright.param().abi(), fonlyifnotgeneric.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Enum<P0>(&self, fforward: P0) -> windows_core::Result<IEnumMoniker>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), fforward.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsEqual<P0>(&self, pmkothermoniker: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pmkothermoniker.param().abi())
    }
    pub unsafe fn Hash(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Hash)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsRunning<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pmknewlyrunning: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        P2: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pmknewlyrunning.param().abi()).ok()
    }
    pub unsafe fn GetTimeOfLastChange<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<super::super::Foundation::FILETIME>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Inverse(&self) -> windows_core::Result<IMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Inverse)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CommonPrefixWith<P0>(&self, pmkother: P0) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CommonPrefixWith)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RelativePathTo<P0>(&self, pmkother: P0) -> windows_core::Result<IMoniker>
    where
        P0: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RelativePathTo)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayName<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ParseDisplayName<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pszdisplayname: P2, pcheaten: *mut u32, ppmkout: *mut Option<IMoniker>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).ParseDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pszdisplayname.param().abi(), pcheaten, core::mem::transmute(ppmkout)).ok()
    }
    pub unsafe fn IsSystemMoniker(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsSystemMoniker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMoniker_Vtbl {
    pub base__: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reduce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComposeWith: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTimeOfLastChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub Inverse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonPrefixWith: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativePathTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ParseDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSystemMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMultiQI, IMultiQI_Vtbl, 0x00000020_0000_0000_c000_000000000046);
impl core::ops::Deref for IMultiQI {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiQI, windows_core::IUnknown);
impl IMultiQI {
    pub unsafe fn QueryMultipleInterfaces(&self, pmqis: &mut [MULTI_QI]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryMultipleInterfaces)(windows_core::Interface::as_raw(self), pmqis.len().try_into().unwrap(), core::mem::transmute(pmqis.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IMultiQI_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryMultipleInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MULTI_QI) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INoMarshal, INoMarshal_Vtbl, 0xecc8691b_c1db_4dc0_855e_65f6c551af49);
impl core::ops::Deref for INoMarshal {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(INoMarshal, windows_core::IUnknown);
impl INoMarshal {}
#[repr(C)]
pub struct INoMarshal_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(IOplockStorage, IOplockStorage_Vtbl, 0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
impl core::ops::Deref for IOplockStorage {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOplockStorage, windows_core::IUnknown);
impl IOplockStorage {
    pub unsafe fn CreateStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn OpenStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).OpenStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IOplockStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPSFactoryBuffer, IPSFactoryBuffer_Vtbl, 0xd5f569d0_593b_101a_b569_08002b2dbf7a);
impl core::ops::Deref for IPSFactoryBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPSFactoryBuffer, windows_core::IUnknown);
impl IPSFactoryBuffer {
    pub unsafe fn CreateProxy<P0>(&self, punkouter: P0, riid: *const windows_core::GUID, ppproxy: *mut Option<IRpcProxyBuffer>, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).CreateProxy)(windows_core::Interface::as_raw(self), punkouter.param().abi(), riid, core::mem::transmute(ppproxy), ppv).ok()
    }
    pub unsafe fn CreateStub<P0>(&self, riid: *const windows_core::GUID, punkserver: P0) -> windows_core::Result<IRpcStubBuffer>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateStub)(windows_core::Interface::as_raw(self), riid, punkserver.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IPSFactoryBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStub: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersist, IPersist_Vtbl, 0x0000010c_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersist {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersist, windows_core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistFile, IPersistFile_Vtbl, 0x0000010b_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistFile {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistFile, windows_core::IUnknown, IPersist);
impl IPersistFile {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pszfilename: P0, dwmode: STGM) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), dwmode).ok()
    }
    pub unsafe fn Save<P0, P1>(&self, pszfilename: P0, fremember: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), fremember.param().abi()).ok()
    }
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pszfilename.param().abi()).ok()
    }
    pub unsafe fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCurFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, STGM) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistMemory, IPersistMemory_Vtbl, 0xbd1ae5e0_a6ae_11ce_bd37_504200c10000);
impl core::ops::Deref for IPersistMemory {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistMemory, windows_core::IUnknown, IPersist);
impl IPersistMemory {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Load(&self, pmem: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), core::mem::transmute(pmem.as_ptr()), pmem.len().try_into().unwrap()).ok()
    }
    pub unsafe fn Save<P0>(&self, pmem: &mut [u8], fcleardirty: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute(pmem.as_ptr()), fcleardirty.param().abi(), pmem.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPersistMemory_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistStream, IPersistStream_Vtbl, 0x00000109_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistStream {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistStream, windows_core::IUnknown, IPersist);
impl IPersistStream {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok()
    }
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IPersistStream_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPersistStreamInit, IPersistStreamInit_Vtbl, 0x7fd52380_4e07_101b_ae2d_08002b2ec713);
impl core::ops::Deref for IPersistStreamInit {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistStreamInit, windows_core::IUnknown, IPersist);
impl IPersistStreamInit {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()).ok()
    }
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.param().abi()).ok()
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn InitNew(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IPersistStreamInit_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPipeByte, IPipeByte_Vtbl, 0xdb2f3aca_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for IPipeByte {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPipeByte, windows_core::IUnknown);
impl IPipeByte {
    pub unsafe fn Pull(&self, buf: &mut [u8], pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[u8]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IPipeByte_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPipeDouble, IPipeDouble_Vtbl, 0xdb2f3ace_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for IPipeDouble {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPipeDouble, windows_core::IUnknown);
impl IPipeDouble {
    pub unsafe fn Pull(&self, buf: &mut [f64], pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[f64]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IPipeDouble_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const f64, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPipeLong, IPipeLong_Vtbl, 0xdb2f3acc_2f86_11d1_8e04_00c04fb9989a);
impl core::ops::Deref for IPipeLong {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPipeLong, windows_core::IUnknown);
impl IPipeLong {
    pub unsafe fn Pull(&self, buf: &mut [i32], pcreturned: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pull)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), pcreturned).ok()
    }
    pub unsafe fn Push(&self, buf: &[i32]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Push)(windows_core::Interface::as_raw(self), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()).ok()
    }
}
#[repr(C)]
pub struct IPipeLong_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Pull: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, u32, *mut u32) -> windows_core::HRESULT,
    pub Push: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessInitControl, IProcessInitControl_Vtbl, 0x72380d55_8d2b_43a3_8513_2b6ef31434e9);
impl core::ops::Deref for IProcessInitControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProcessInitControl, windows_core::IUnknown);
impl IProcessInitControl {
    pub unsafe fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ResetInitializerTimeout)(windows_core::Interface::as_raw(self), dwsecondsremaining).ok()
    }
}
#[repr(C)]
pub struct IProcessInitControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ResetInitializerTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IProcessLock, IProcessLock_Vtbl, 0x000001d5_0000_0000_c000_000000000046);
impl core::ops::Deref for IProcessLock {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProcessLock, windows_core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        (windows_core::Interface::vtable(self).AddRefOnProcess)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        (windows_core::Interface::vtable(self).ReleaseRefOnProcess)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IProcessLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
windows_core::imp::define_interface!(IProgressNotify, IProgressNotify_Vtbl, 0xa9d758a0_4617_11cf_95fc_00aa00680db4);
impl core::ops::Deref for IProgressNotify {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IProgressNotify, windows_core::IUnknown);
impl IProgressNotify {
    pub unsafe fn OnProgress<P0, P1>(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: P0, fowner: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), dwprogresscurrent, dwprogressmaximum, faccurate.param().abi(), fowner.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IProgressNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IROTData, IROTData_Vtbl, 0xf29f6bc0_5021_11ce_aa15_00006901293f);
impl core::ops::Deref for IROTData {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IROTData, windows_core::IUnknown);
impl IROTData {
    pub unsafe fn GetComparisonData(&self, pbdata: &mut [u8], pcbdata: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetComparisonData)(windows_core::Interface::as_raw(self), core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap(), pcbdata).ok()
    }
}
#[repr(C)]
pub struct IROTData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetComparisonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IReleaseMarshalBuffers, IReleaseMarshalBuffers_Vtbl, 0xeb0cb9e8_7996_11d2_872e_0000f8080859);
impl core::ops::Deref for IReleaseMarshalBuffers {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReleaseMarshalBuffers, windows_core::IUnknown);
impl IReleaseMarshalBuffers {
    pub unsafe fn ReleaseMarshalBuffer<P0>(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).ReleaseMarshalBuffer)(windows_core::Interface::as_raw(self), pmsg, dwflags, pchnl.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IReleaseMarshalBuffers_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReleaseMarshalBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcChannelBuffer, IRpcChannelBuffer_Vtbl, 0xd5f56b60_593b_101a_b569_08002b2dbf7a);
impl core::ops::Deref for IRpcChannelBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer, windows_core::IUnknown);
impl IRpcChannelBuffer {
    pub unsafe fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), pmessage, riid).ok()
    }
    pub unsafe fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SendReceive)(windows_core::Interface::as_raw(self), pmessage, core::mem::transmute(pstatus.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeBuffer)(windows_core::Interface::as_raw(self), pmessage).ok()
    }
    pub unsafe fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDestCtx)(windows_core::Interface::as_raw(self), pdwdestcontext, ppvdestcontext).ok()
    }
    pub unsafe fn IsConnected(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRpcChannelBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SendReceive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    pub GetDestCtx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcChannelBuffer2, IRpcChannelBuffer2_Vtbl, 0x594f31d0_7f19_11d0_b194_00a0c90dc8bf);
impl core::ops::Deref for IRpcChannelBuffer2 {
    type Target = IRpcChannelBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer2, windows_core::IUnknown, IRpcChannelBuffer);
impl IRpcChannelBuffer2 {
    pub unsafe fn GetProtocolVersion(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRpcChannelBuffer2_Vtbl {
    pub base__: IRpcChannelBuffer_Vtbl,
    pub GetProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcChannelBuffer3, IRpcChannelBuffer3_Vtbl, 0x25b15600_0115_11d0_bf0d_00aa00b8dfd2);
impl core::ops::Deref for IRpcChannelBuffer3 {
    type Target = IRpcChannelBuffer2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcChannelBuffer3, windows_core::IUnknown, IRpcChannelBuffer, IRpcChannelBuffer2);
impl IRpcChannelBuffer3 {
    pub unsafe fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Send)(windows_core::Interface::as_raw(self), pmsg, pulstatus).ok()
    }
    pub unsafe fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Receive)(windows_core::Interface::as_raw(self), pmsg, ulsize, pulstatus).ok()
    }
    pub unsafe fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
    pub unsafe fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetCallContext)(windows_core::Interface::as_raw(self), pmsg, riid, pinterface).ok()
    }
    pub unsafe fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDestCtxEx)(windows_core::Interface::as_raw(self), pmsg, pdwdestcontext, core::mem::transmute(ppvdestcontext.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pmsg, &mut result__).map(|| result__)
    }
    pub unsafe fn RegisterAsync<P0>(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncManager>,
    {
        (windows_core::Interface::vtable(self).RegisterAsync)(windows_core::Interface::as_raw(self), pmsg, pasyncmgr.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRpcChannelBuffer3_Vtbl {
    pub base__: IRpcChannelBuffer2_Vtbl,
    pub Send: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub Receive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, u32, *mut u32) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
    pub GetCallContext: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDestCtxEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *const RPCOLEMESSAGE, *mut u32) -> windows_core::HRESULT,
    pub RegisterAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcHelper, IRpcHelper_Vtbl, 0x00000149_0000_0000_c000_000000000046);
impl core::ops::Deref for IRpcHelper {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcHelper, windows_core::IUnknown);
impl IRpcHelper {
    pub unsafe fn GetDCOMProtocolVersion(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDCOMProtocolVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetIIDFromOBJREF(&self, pobjref: *const core::ffi::c_void) -> windows_core::Result<*mut windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIIDFromOBJREF)(windows_core::Interface::as_raw(self), pobjref, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRpcHelper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDCOMProtocolVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetIIDFromOBJREF: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcOptions, IRpcOptions_Vtbl, 0x00000144_0000_0000_c000_000000000046);
impl core::ops::Deref for IRpcOptions {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcOptions, windows_core::IUnknown);
impl IRpcOptions {
    pub unsafe fn Set<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, dwvalue).ok()
    }
    pub unsafe fn Query<P0>(&self, pprx: P0, dwproperty: RPCOPT_PROPERTIES) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pprx.param().abi(), dwproperty, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRpcOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, usize) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RPCOPT_PROPERTIES, *mut usize) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRpcProxyBuffer, IRpcProxyBuffer_Vtbl, 0xd5f56a34_593b_101a_b569_08002b2dbf7a);
impl core::ops::Deref for IRpcProxyBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcProxyBuffer, windows_core::IUnknown);
impl IRpcProxyBuffer {
    pub unsafe fn Connect<P0>(&self, prpcchannelbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRpcChannelBuffer>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), prpcchannelbuffer.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IRpcProxyBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
}
windows_core::imp::define_interface!(IRpcStubBuffer, IRpcStubBuffer_Vtbl, 0xd5f56afc_593b_101a_b569_08002b2dbf7a);
impl core::ops::Deref for IRpcStubBuffer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcStubBuffer, windows_core::IUnknown);
impl IRpcStubBuffer {
    pub unsafe fn Connect<P0>(&self, punkserver: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), punkserver.param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) {
        (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Invoke<P0>(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRpcChannelBuffer>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), _prpcmsg, _prpcchannelbuffer.param().abi()).ok()
    }
    pub unsafe fn IsIIDSupported(&self, riid: *const windows_core::GUID) -> Option<IRpcStubBuffer> {
        (windows_core::Interface::vtable(self).IsIIDSupported)(windows_core::Interface::as_raw(self), riid)
    }
    pub unsafe fn CountRefs(&self) -> u32 {
        (windows_core::Interface::vtable(self).CountRefs)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn DebugServerQueryInterface(&self, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DebugServerQueryInterface)(windows_core::Interface::as_raw(self), ppv).ok()
    }
    pub unsafe fn DebugServerRelease(&self, pv: *const core::ffi::c_void) {
        (windows_core::Interface::vtable(self).DebugServerRelease)(windows_core::Interface::as_raw(self), pv)
    }
}
#[repr(C)]
pub struct IRpcStubBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIIDSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> Option<IRpcStubBuffer>,
    pub CountRefs: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub DebugServerQueryInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DebugServerRelease: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void),
}
windows_core::imp::define_interface!(IRpcSyntaxNegotiate, IRpcSyntaxNegotiate_Vtbl, 0x58a08519_24c8_4935_b482_3fd823333a4f);
impl core::ops::Deref for IRpcSyntaxNegotiate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRpcSyntaxNegotiate, windows_core::IUnknown);
impl IRpcSyntaxNegotiate {
    pub unsafe fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NegotiateSyntax)(windows_core::Interface::as_raw(self), pmsg).ok()
    }
}
#[repr(C)]
pub struct IRpcSyntaxNegotiate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NegotiateSyntax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RPCOLEMESSAGE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRunnableObject, IRunnableObject_Vtbl, 0x00000126_0000_0000_c000_000000000046);
impl core::ops::Deref for IRunnableObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRunnableObject, windows_core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRunningClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Run<P0>(&self, pbc: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBindCtx>,
    {
        (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), pbc.param().abi()).ok()
    }
    pub unsafe fn IsRunning(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn LockRunning<P0, P1>(&self, flock: P0, flastunlockcloses: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
        P1: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).LockRunning)(windows_core::Interface::as_raw(self), flock.param().abi(), flastunlockcloses.param().abi()).ok()
    }
    pub unsafe fn SetContainedObject<P0>(&self, fcontained: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetContainedObject)(windows_core::Interface::as_raw(self), fcontained.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRunnableObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRunningClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
    pub LockRunning: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetContainedObject: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRunningObjectTable, IRunningObjectTable_Vtbl, 0x00000010_0000_0000_c000_000000000046);
impl core::ops::Deref for IRunningObjectTable {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRunningObjectTable, windows_core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<P0, P1>(&self, grfflags: ROT_FLAGS, punkobject: P0, pmkobjectname: P1) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), grfflags, punkobject.param().abi(), pmkobjectname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), dwregister).ok()
    }
    pub unsafe fn IsRunning<P0>(&self, pmkobjectname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMoniker>,
    {
        (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi()).ok()
    }
    pub unsafe fn GetObject<P0>(&self, pmkobjectname: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).NoteChangeTime)(windows_core::Interface::as_raw(self), dwregister, pfiletime).ok()
    }
    pub unsafe fn GetTimeOfLastChange<P0>(&self, pmkobjectname: P0) -> windows_core::Result<super::super::Foundation::FILETIME>
    where
        P0: windows_core::Param<IMoniker>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumRunning)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IRunningObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, ROT_FLAGS, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NoteChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetTimeOfLastChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub EnumRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
impl core::ops::Deref for ISequentialStream {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv, cb, core::mem::transmute(pcbread.unwrap_or(std::ptr::null_mut())))
    }
    pub unsafe fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pv, cb, core::mem::transmute(pcbwritten.unwrap_or(std::ptr::null_mut())))
    }
}
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IServerSecurity, IServerSecurity_Vtbl, 0x0000013e_0000_0000_c000_000000000046);
impl core::ops::Deref for IServerSecurity {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServerSecurity, windows_core::IUnknown);
impl IServerSecurity {
    pub unsafe fn QueryBlanket(&self, pauthnsvc: Option<*mut u32>, pauthzsvc: Option<*mut u32>, pserverprincname: *mut *mut u16, pauthnlevel: Option<*mut u32>, pimplevel: Option<*mut u32>, pprivs: *mut *mut core::ffi::c_void, pcapabilities: Option<*mut u32>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).QueryBlanket)(windows_core::Interface::as_raw(self), core::mem::transmute(pauthnsvc.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pauthzsvc.unwrap_or(std::ptr::null_mut())), pserverprincname, core::mem::transmute(pauthnlevel.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pimplevel.unwrap_or(std::ptr::null_mut())), pprivs, core::mem::transmute(pcapabilities.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ImpersonateClient(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ImpersonateClient)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RevertToSelf(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RevertToSelf)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn IsImpersonating(&self) -> super::super::Foundation::BOOL {
        (windows_core::Interface::vtable(self).IsImpersonating)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IServerSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryBlanket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut *mut u16, *mut u32, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ImpersonateClient: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevertToSelf: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsImpersonating: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::BOOL,
}
windows_core::imp::define_interface!(IServiceProvider, IServiceProvider_Vtbl, 0x6d5140c1_7436_11ce_8034_00aa006009fa);
impl core::ops::Deref for IServiceProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IServiceProvider, windows_core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService<T>(&self, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).QueryService)(windows_core::Interface::as_raw(self), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStdMarshalInfo, IStdMarshalInfo_Vtbl, 0x00000018_0000_0000_c000_000000000046);
impl core::ops::Deref for IStdMarshalInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStdMarshalInfo, windows_core::IUnknown);
impl IStdMarshalInfo {
    pub unsafe fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetClassForHandler)(windows_core::Interface::as_raw(self), dwdestcontext, core::mem::transmute(pvdestcontext.unwrap_or(std::ptr::null())), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IStdMarshalInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassForHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStream, IStream_Vtbl, 0x0000000c_0000_0000_c000_000000000046);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
impl IStream {
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: Option<*mut u64>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), dlibmove, dworigin, core::mem::transmute(plibnewposition.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: Option<*mut u64>, pcbwritten: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStream>,
    {
        (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), pstm.param().abi(), cb, core::mem::transmute(pcbread.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pcbwritten.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: STGC) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags.0 as _).ok()
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: LOCKTYPE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype.0 as _).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: STATFLAG) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg, grfstatflag.0 as _).ok()
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<IStream> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, i64, STREAM_SEEK, *mut u64) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STATSTG, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISupportAllowLowerTrustActivation, ISupportAllowLowerTrustActivation_Vtbl, 0xe9956ef2_3828_4b4b_8fa9_7db61dee4954);
impl core::ops::Deref for ISupportAllowLowerTrustActivation {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISupportAllowLowerTrustActivation, windows_core::IUnknown);
impl ISupportAllowLowerTrustActivation {}
#[repr(C)]
pub struct ISupportAllowLowerTrustActivation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
windows_core::imp::define_interface!(ISupportErrorInfo, ISupportErrorInfo_Vtbl, 0xdf0b3d60_548f_101b_8e65_08002b2bd119);
impl core::ops::Deref for ISupportErrorInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISupportErrorInfo, windows_core::IUnknown);
impl ISupportErrorInfo {
    pub unsafe fn InterfaceSupportsErrorInfo(&self, riid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InterfaceSupportsErrorInfo)(windows_core::Interface::as_raw(self), riid).ok()
    }
}
#[repr(C)]
pub struct ISupportErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InterfaceSupportsErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISurrogate, ISurrogate_Vtbl, 0x00000022_0000_0000_c000_000000000046);
impl core::ops::Deref for ISurrogate {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISurrogate, windows_core::IUnknown);
impl ISurrogate {
    pub unsafe fn LoadDllServer(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LoadDllServer)(windows_core::Interface::as_raw(self), clsid).ok()
    }
    pub unsafe fn FreeSurrogate(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FreeSurrogate)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISurrogate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LoadDllServer: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub FreeSurrogate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISurrogateService, ISurrogateService_Vtbl, 0x000001d4_0000_0000_c000_000000000046);
impl core::ops::Deref for ISurrogateService {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISurrogateService, windows_core::IUnknown);
impl ISurrogateService {
    pub unsafe fn Init<P0>(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IProcessLock>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), rguidprocessid, pprocesslock.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplicationLaunch)(windows_core::Interface::as_raw(self), rguidapplid, apptype).ok()
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ApplicationFree)(windows_core::Interface::as_raw(self), rguidapplid).ok()
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CatalogRefresh)(windows_core::Interface::as_raw(self), ulreserved).ok()
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ProcessShutdown)(windows_core::Interface::as_raw(self), shutdowntype).ok()
    }
}
#[repr(C)]
pub struct ISurrogateService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub ApplicationLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, ApplicationType) -> windows_core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, ShutdownType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronize, ISynchronize_Vtbl, 0x00000030_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronize {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronize, windows_core::IUnknown);
impl ISynchronize {
    pub unsafe fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Wait)(windows_core::Interface::as_raw(self), dwflags, dwmilliseconds).ok()
    }
    pub unsafe fn Signal(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Signal)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISynchronize_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Wait: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Signal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronizeContainer, ISynchronizeContainer_Vtbl, 0x00000033_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeContainer {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeContainer, windows_core::IUnknown);
impl ISynchronizeContainer {
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISynchronize>,
    {
        (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()).ok()
    }
    pub unsafe fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> windows_core::Result<ISynchronize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), dwflags, dwtimeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISynchronizeContainer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronizeEvent, ISynchronizeEvent_Vtbl, 0x00000032_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeEvent {
    type Target = ISynchronizeHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeEvent, windows_core::IUnknown, ISynchronizeHandle);
impl ISynchronizeEvent {
    pub unsafe fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), ph).ok()
    }
}
#[repr(C)]
pub struct ISynchronizeEvent_Vtbl {
    pub base__: ISynchronizeHandle_Vtbl,
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronizeHandle, ISynchronizeHandle_Vtbl, 0x00000031_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeHandle {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeHandle, windows_core::IUnknown);
impl ISynchronizeHandle {
    pub unsafe fn GetHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ISynchronizeHandle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISynchronizeMutex, ISynchronizeMutex_Vtbl, 0x00000025_0000_0000_c000_000000000046);
impl core::ops::Deref for ISynchronizeMutex {
    type Target = ISynchronize;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISynchronizeMutex, windows_core::IUnknown, ISynchronize);
impl ISynchronizeMutex {
    pub unsafe fn ReleaseMutex(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ReleaseMutex)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct ISynchronizeMutex_Vtbl {
    pub base__: ISynchronize_Vtbl,
    pub ReleaseMutex: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimeAndNoticeControl, ITimeAndNoticeControl_Vtbl, 0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
impl core::ops::Deref for ITimeAndNoticeControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITimeAndNoticeControl, windows_core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SuppressChanges)(windows_core::Interface::as_raw(self), res1, res2).ok()
    }
}
#[repr(C)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SuppressChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeComp, ITypeComp_Vtbl, 0x00020403_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeComp {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeComp, windows_core::IUnknown);
impl ITypeComp {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Bind<P0>(&self, szname: P0, lhashval: u32, wflags: u16, pptinfo: *mut Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, wflags, core::mem::transmute(pptinfo), pdesckind, pbindptr).ok()
    }
    pub unsafe fn BindType<P0>(&self, szname: P0, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, pptcomp: *mut Option<ITypeComp>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).BindType)(windows_core::Interface::as_raw(self), szname.param().abi(), lhashval, core::mem::transmute(pptinfo), core::mem::transmute(pptcomp)).ok()
    }
}
#[repr(C)]
pub struct ITypeComp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u16, *mut *mut core::ffi::c_void, *mut DESCKIND, *mut BINDPTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Bind: usize,
    pub BindType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeInfo, ITypeInfo_Vtbl, 0x00020401_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeInfo {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeInfo, windows_core::IUnknown);
impl ITypeInfo {
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetTypeAttr(&self) -> windows_core::Result<*mut TYPEATTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFuncDesc(&self, index: u32) -> windows_core::Result<*mut FUNCDESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFuncDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetVarDesc(&self, index: u32) -> windows_core::Result<*mut VARDESC> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVarDesc)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetNames(&self, memid: i32, rgbstrnames: &mut [windows_core::BSTR], pcnames: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNames)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(rgbstrnames.as_ptr()), rgbstrnames.len().try_into().unwrap(), pcnames).ok()
    }
    pub unsafe fn GetRefTypeOfImplType(&self, index: u32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRefTypeOfImplType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetImplTypeFlags(&self, index: u32) -> windows_core::Result<IMPLTYPEFLAGS> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImplTypeFlags)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetIDsOfNames(&self, rgsznames: *const windows_core::PCWSTR, cnames: u32, pmemid: *mut i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), rgsznames, cnames, pmemid).ok()
    }
    pub unsafe fn Invoke(&self, pvinstance: *const core::ffi::c_void, memid: i32, wflags: DISPATCH_FLAGS, pdispparams: *mut DISPPARAMS, pvarresult: *mut windows_core::VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pvinstance, memid, wflags, pdispparams, core::mem::transmute(pvarresult), pexcepinfo, puargerr).ok()
    }
    pub unsafe fn GetDocumentation(&self, memid: i32, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), memid, core::mem::transmute(pbstrname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrdocstring.unwrap_or(std::ptr::null_mut())), pdwhelpcontext, core::mem::transmute(pbstrhelpfile.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: Option<*mut windows_core::BSTR>, pbstrname: Option<*mut windows_core::BSTR>, pwordinal: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDllEntry)(windows_core::Interface::as_raw(self), memid, invkind, core::mem::transmute(pbstrdllname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrname.unwrap_or(std::ptr::null_mut())), pwordinal).ok()
    }
    pub unsafe fn GetRefTypeInfo(&self, hreftype: u32) -> windows_core::Result<ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRefTypeInfo)(windows_core::Interface::as_raw(self), hreftype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AddressOfMember)(windows_core::Interface::as_raw(self), memid, invkind, ppv).ok()
    }
    pub unsafe fn CreateInstance<P0, T>(&self, punkouter: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetMops(&self, memid: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMops)(windows_core::Interface::as_raw(self), memid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetContainingTypeLib(&self, pptlib: *mut Option<ITypeLib>, pindex: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetContainingTypeLib)(windows_core::Interface::as_raw(self), core::mem::transmute(pptlib), pindex).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR) {
        (windows_core::Interface::vtable(self).ReleaseTypeAttr)(windows_core::Interface::as_raw(self), ptypeattr)
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC) {
        (windows_core::Interface::vtable(self).ReleaseFuncDesc)(windows_core::Interface::as_raw(self), pfuncdesc)
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn ReleaseVarDesc(&self, pvardesc: *const VARDESC) {
        (windows_core::Interface::vtable(self).ReleaseVarDesc)(windows_core::Interface::as_raw(self), pvardesc)
    }
}
#[repr(C)]
pub struct ITypeInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TYPEATTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetTypeAttr: usize,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut FUNCDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetFuncDesc: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut VARDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetVarDesc: usize,
    pub GetNames: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, u32, *mut u32) -> windows_core::HRESULT,
    pub GetRefTypeOfImplType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetImplTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut IMPLTYPEFLAGS) -> windows_core::HRESULT,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::PCWSTR, u32, *mut i32) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, i32, DISPATCH_FLAGS, *mut DISPPARAMS, *mut core::mem::MaybeUninit<windows_core::VARIANT>, *mut EXCEPINFO, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDllEntry: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u16) -> windows_core::HRESULT,
    pub GetRefTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddressOfMember: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMops: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetContainingTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReleaseTypeAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TYPEATTR),
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReleaseTypeAttr: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReleaseFuncDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FUNCDESC),
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReleaseFuncDesc: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub ReleaseVarDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *const VARDESC),
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    ReleaseVarDesc: usize,
}
windows_core::imp::define_interface!(ITypeInfo2, ITypeInfo2_Vtbl, 0x00020412_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeInfo2 {
    type Target = ITypeInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeInfo2, windows_core::IUnknown, ITypeInfo);
impl ITypeInfo2 {
    pub unsafe fn GetTypeKind(&self) -> windows_core::Result<TYPEKIND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFuncIndexOfMemId)(windows_core::Interface::as_raw(self), memid, invkind, &mut result__).map(|| result__)
    }
    pub unsafe fn GetVarIndexOfMemId(&self, memid: i32) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVarIndexOfMemId)(windows_core::Interface::as_raw(self), memid, &mut result__).map(|| result__)
    }
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFuncCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFuncCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetVarCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVarCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetImplTypeCustData(&self, index: u32, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetImplTypeCustData)(windows_core::Interface::as_raw(self), index, guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), memid, lcid, core::mem::transmute(pbstrhelpstring.unwrap_or(std::ptr::null_mut())), pdwhelpstringcontext, core::mem::transmute(pbstrhelpstringdll.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetAllFuncCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllFuncCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllParamCustData)(windows_core::Interface::as_raw(self), indexfunc, indexparam, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAllVarCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllVarCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAllImplTypeCustData(&self, index: u32) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllImplTypeCustData)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITypeInfo2_Vtbl {
    pub base__: ITypeInfo_Vtbl,
    pub GetTypeKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFuncIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32, INVOKEKIND, *mut u32) -> windows_core::HRESULT,
    pub GetVarIndexOfMemId: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
    pub GetAllFuncCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    pub GetAllParamCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    pub GetAllVarCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
    pub GetAllImplTypeCustData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CUSTDATA) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeLib, ITypeLib_Vtbl, 0x00020402_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeLib {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLib, windows_core::IUnknown);
impl ITypeLib {
    pub unsafe fn GetTypeInfoCount(&self) -> u32 {
        (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetTypeInfo(&self, index: u32) -> windows_core::Result<ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetTypeInfoType(&self, index: u32) -> windows_core::Result<TYPEKIND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfoType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeInfoOfGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<ITypeInfo> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeInfoOfGuid)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLibAttr(&self) -> windows_core::Result<*mut TLIBATTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLibAttr)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTypeComp(&self) -> windows_core::Result<ITypeComp> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTypeComp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDocumentation(&self, index: i32, pbstrname: Option<*mut windows_core::BSTR>, pbstrdocstring: Option<*mut windows_core::BSTR>, pdwhelpcontext: *mut u32, pbstrhelpfile: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDocumentation)(windows_core::Interface::as_raw(self), index, core::mem::transmute(pbstrname.unwrap_or(std::ptr::null_mut())), core::mem::transmute(pbstrdocstring.unwrap_or(std::ptr::null_mut())), pdwhelpcontext, core::mem::transmute(pbstrhelpfile.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn IsName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).IsName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, pfname).ok()
    }
    pub unsafe fn FindName(&self, sznamebuf: windows_core::PWSTR, lhashval: u32, pptinfo: *mut Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).FindName)(windows_core::Interface::as_raw(self), core::mem::transmute(sznamebuf), lhashval, core::mem::transmute(pptinfo), rgmemid, pcfound).ok()
    }
    pub unsafe fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR) {
        (windows_core::Interface::vtable(self).ReleaseTLibAttr)(windows_core::Interface::as_raw(self), ptlibattr)
    }
}
#[repr(C)]
pub struct ITypeLib_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeInfoType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut TYPEKIND) -> windows_core::HRESULT,
    pub GetTypeInfoOfGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut TLIBATTR) -> windows_core::HRESULT,
    pub GetTypeComp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDocumentation: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut *mut core::ffi::c_void, *mut i32, *mut u16) -> windows_core::HRESULT,
    pub ReleaseTLibAttr: unsafe extern "system" fn(*mut core::ffi::c_void, *const TLIBATTR),
}
windows_core::imp::define_interface!(ITypeLib2, ITypeLib2_Vtbl, 0x00020411_0000_0000_c000_000000000046);
impl core::ops::Deref for ITypeLib2 {
    type Target = ITypeLib;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLib2, windows_core::IUnknown, ITypeLib);
impl ITypeLib2 {
    pub unsafe fn GetCustData(&self, guid: *const windows_core::GUID) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetCustData)(windows_core::Interface::as_raw(self), guid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetLibStatistics)(windows_core::Interface::as_raw(self), pcuniquenames, pcchuniquenames).ok()
    }
    pub unsafe fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: Option<*mut windows_core::BSTR>, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: Option<*mut windows_core::BSTR>) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetDocumentation2)(windows_core::Interface::as_raw(self), index, lcid, core::mem::transmute(pbstrhelpstring.unwrap_or(std::ptr::null_mut())), pdwhelpstringcontext, core::mem::transmute(pbstrhelpstringdll.unwrap_or(std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllCustData(&self) -> windows_core::Result<CUSTDATA> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAllCustData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct ITypeLib2_Vtbl {
    pub base__: ITypeLib_Vtbl,
    pub GetCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub GetLibStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetDocumentation2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut core::mem::MaybeUninit<windows_core::BSTR>, *mut u32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAllCustData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CUSTDATA) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeLibRegistration, ITypeLibRegistration_Vtbl, 0x76a3e735_02df_4a12_98eb_043ad3600af3);
impl core::ops::Deref for ITypeLibRegistration {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLibRegistration, windows_core::IUnknown);
impl ITypeLibRegistration {
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetVersion(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetLcid(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetLcid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetWin32Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWin32Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetWin64Path(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetWin64Path)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetHelpDir(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHelpDir)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITypeLibRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetLcid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetWin32Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetWin64Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetHelpDir: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITypeLibRegistrationReader, ITypeLibRegistrationReader_Vtbl, 0xed6a8a2a_b160_4e77_8f73_aa7435cd5c27);
impl core::ops::Deref for ITypeLibRegistrationReader {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITypeLibRegistrationReader, windows_core::IUnknown);
impl ITypeLibRegistrationReader {
    pub unsafe fn EnumTypeLibRegistrations(&self) -> windows_core::Result<IEnumUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EnumTypeLibRegistrations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ITypeLibRegistrationReader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumTypeLibRegistrations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUri, IUri_Vtbl, 0xa39ee748_6a27_4817_a6f2_13914bef5890);
impl core::ops::Deref for IUri {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUri, windows_core::IUnknown);
impl IUri {
    pub unsafe fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut windows_core::BSTR, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyBSTR)(windows_core::Interface::as_raw(self), uriprop, core::mem::transmute(pbstrproperty), dwflags).ok()
    }
    pub unsafe fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyLength)(windows_core::Interface::as_raw(self), uriprop, pcchproperty, dwflags).ok()
    }
    pub unsafe fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPropertyDWORD)(windows_core::Interface::as_raw(self), uriprop, pdwproperty, dwflags).ok()
    }
    pub unsafe fn HasProperty(&self, uriprop: Uri_PROPERTY) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasProperty)(windows_core::Interface::as_raw(self), uriprop, &mut result__).map(|| result__)
    }
    pub unsafe fn GetAbsoluteUri(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAbsoluteUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetAuthority(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetAuthority)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDisplayUri(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDisplayUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDomain)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetExtension)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetFragment(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHost(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPassword(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetPathAndQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPathAndQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetQuery(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetRawUri(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetRawUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetSchemeName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUserInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetUserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetHostType(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetHostType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetPort(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetScheme(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetScheme)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetZone(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetZone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetProperties(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsEqual<P0>(&self, puri: P0) -> windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: windows_core::Param<IUri>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), puri.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUri_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyBSTR: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut core::mem::MaybeUninit<windows_core::BSTR>, u32) -> windows_core::HRESULT,
    pub GetPropertyLength: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub GetPropertyDWORD: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut u32, u32) -> windows_core::HRESULT,
    pub HasProperty: unsafe extern "system" fn(*mut core::ffi::c_void, Uri_PROPERTY, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetAbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPathAndQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetRawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetUserInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetHostType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetScheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetZone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriBuilder, IUriBuilder_Vtbl, 0x4221b2e1_8955_46c0_bd5b_de9897565de7);
impl core::ops::Deref for IUriBuilder {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUriBuilder, windows_core::IUnknown);
impl IUriBuilder {
    pub unsafe fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUriSimple)(windows_core::Interface::as_raw(self), dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUri)(windows_core::Interface::as_raw(self), dwcreateflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> windows_core::Result<IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUriWithFlags)(windows_core::Interface::as_raw(self), dwcreateflags, dwuribuilderflags, dwallowencodingpropertymask, dwreserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetIUri(&self) -> windows_core::Result<IUri> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetIUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetIUri<P0>(&self, piuri: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUri>,
    {
        (windows_core::Interface::vtable(self).SetIUri)(windows_core::Interface::as_raw(self), piuri.param().abi()).ok()
    }
    pub unsafe fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetFragment)(windows_core::Interface::as_raw(self), pcchfragment, ppwzfragment).ok()
    }
    pub unsafe fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetHost)(windows_core::Interface::as_raw(self), pcchhost, ppwzhost).ok()
    }
    pub unsafe fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), pcchpassword, ppwzpassword).ok()
    }
    pub unsafe fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), pcchpath, ppwzpath).ok()
    }
    pub unsafe fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), pfhasport, pdwport).ok()
    }
    pub unsafe fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetQuery)(windows_core::Interface::as_raw(self), pcchquery, ppwzquery).ok()
    }
    pub unsafe fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetSchemeName)(windows_core::Interface::as_raw(self), pcchschemename, ppwzschemename).ok()
    }
    pub unsafe fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut windows_core::PCWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetUserName)(windows_core::Interface::as_raw(self), pcchusername, ppwzusername).ok()
    }
    pub unsafe fn SetFragment<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetFragment)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetHost<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetHost)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetPath<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetPort<P0>(&self, fhasport: P0, dwnewvalue: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), fhasport.param().abi(), dwnewvalue).ok()
    }
    pub unsafe fn SetQuery<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetQuery)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetSchemeName<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetSchemeName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn SetUserName<P0>(&self, pwznewvalue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), pwznewvalue.param().abi()).ok()
    }
    pub unsafe fn RemoveProperties(&self, dwpropertymask: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveProperties)(windows_core::Interface::as_raw(self), dwpropertymask).ok()
    }
    pub unsafe fn HasBeenModified(&self) -> windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HasBeenModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IUriBuilder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateUriSimple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUri: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUriWithFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL, *mut u32) -> windows_core::HRESULT,
    pub GetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFragment: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetHost: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::BOOL, u32) -> windows_core::HRESULT,
    pub SetQuery: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetSchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub HasBeenModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUrlMon, IUrlMon_Vtbl, 0x00000026_0000_0000_c000_000000000046);
impl core::ops::Deref for IUrlMon {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUrlMon, windows_core::IUnknown);
impl IUrlMon {
    pub unsafe fn AsyncGetClassBits<P0, P1, P2, P3>(&self, rclsid: *const windows_core::GUID, psztype: P0, pszext: P1, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: P2, pbc: P3, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<IBindCtx>,
    {
        (windows_core::Interface::vtable(self).AsyncGetClassBits)(windows_core::Interface::as_raw(self), rclsid, psztype.param().abi(), pszext.param().abi(), dwfileversionms, dwfileversionls, pszcodebase.param().abi(), pbc.param().abi(), dwclasscontext, riid, flags).ok()
    }
}
#[repr(C)]
pub struct IUrlMon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWaitMultiple, IWaitMultiple_Vtbl, 0x0000002b_0000_0000_c000_000000000046);
impl core::ops::Deref for IWaitMultiple {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWaitMultiple, windows_core::IUnknown);
impl IWaitMultiple {
    pub unsafe fn WaitMultiple(&self, timeout: u32) -> windows_core::Result<ISynchronize> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WaitMultiple)(windows_core::Interface::as_raw(self), timeout, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AddSynchronize<P0>(&self, psync: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISynchronize>,
    {
        (windows_core::Interface::vtable(self).AddSynchronize)(windows_core::Interface::as_raw(self), psync.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IWaitMultiple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddSynchronize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const ADVFCACHE_FORCEBUILTIN: ADVF = ADVF(16i32);
pub const ADVFCACHE_NOHANDLER: ADVF = ADVF(8i32);
pub const ADVFCACHE_ONSAVE: ADVF = ADVF(32i32);
pub const ADVF_DATAONSTOP: ADVF = ADVF(64i32);
pub const ADVF_NODATA: ADVF = ADVF(1i32);
pub const ADVF_ONLYONCE: ADVF = ADVF(4i32);
pub const ADVF_PRIMEFIRST: ADVF = ADVF(2i32);
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(6i32);
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(1i32);
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(4i32);
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(5i32);
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(2i32);
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = APTTYPEQUALIFIER(3i32);
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = APTTYPEQUALIFIER(0i32);
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = APTTYPEQUALIFIER(7i32);
pub const APTTYPE_CURRENT: APTTYPE = APTTYPE(-1i32);
pub const APTTYPE_MAINSTA: APTTYPE = APTTYPE(3i32);
pub const APTTYPE_MTA: APTTYPE = APTTYPE(1i32);
pub const APTTYPE_NA: APTTYPE = APTTYPE(2i32);
pub const APTTYPE_STA: APTTYPE = APTTYPE(0i32);
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = BINDINFOF(2i32);
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = BINDINFOF(1i32);
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = BIND_FLAGS(2i32);
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = BIND_FLAGS(1i32);
pub const CALLTYPE_ASYNC: CALLTYPE = CALLTYPE(3i32);
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = CALLTYPE(5i32);
pub const CALLTYPE_NESTED: CALLTYPE = CALLTYPE(2i32);
pub const CALLTYPE_TOPLEVEL: CALLTYPE = CALLTYPE(1i32);
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = CALLTYPE(4i32);
pub const CC_CDECL: CALLCONV = CALLCONV(1i32);
pub const CC_FASTCALL: CALLCONV = CALLCONV(0i32);
pub const CC_FPFASTCALL: CALLCONV = CALLCONV(5i32);
pub const CC_MACPASCAL: CALLCONV = CALLCONV(3i32);
pub const CC_MAX: CALLCONV = CALLCONV(9i32);
pub const CC_MPWCDECL: CALLCONV = CALLCONV(7i32);
pub const CC_MPWPASCAL: CALLCONV = CALLCONV(8i32);
pub const CC_MSCPASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_PASCAL: CALLCONV = CALLCONV(2i32);
pub const CC_STDCALL: CALLCONV = CALLCONV(4i32);
pub const CC_SYSCALL: CALLCONV = CALLCONV(6i32);
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = CLSCTX(67108864u32);
pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
pub const COINITBASE_MULTITHREADED: COINITBASE = COINITBASE(0i32);
pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2i32);
pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4i32);
pub const COINIT_MULTITHREADED: COINIT = COINIT(0i32);
pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8i32);
pub const COLE_DEFAULT_AUTHINFO: i32 = -1i32;
pub const COLE_DEFAULT_PRINCIPAL: windows_core::PCWSTR = windows_core::PCWSTR(-1i32 as _);
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(4i32);
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(5i32);
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(8i32);
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(16i32);
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(1i32);
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = RPCOPT_PROPERTIES(2i32);
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(2i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(2i32);
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(1i32);
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = GLOBALOPT_EH_VALUES(0i32);
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(1i32);
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(8i32);
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(6i32);
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(7i32);
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(8i32);
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(16i32);
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(32i32);
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(64i32);
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(256i32);
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(512i32);
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1024i32);
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(4i32);
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(3i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(0i32);
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = GLOBALOPT_RPCTP_VALUES(1i32);
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(1i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(4i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(2i32);
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = GLOBALOPT_RO_FLAGS(128i32);
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = GLOBALOPT_PROPERTIES(5i32);
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(2i32);
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(0i32);
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = GLOBALOPT_UNMARSHALING_POLICY_VALUES(1i32);
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = COWAIT_FLAGS(2i32);
pub const COWAIT_DEFAULT: COWAIT_FLAGS = COWAIT_FLAGS(0i32);
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = COWAIT_FLAGS(8i32);
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = COWAIT_FLAGS(16i32);
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = COWAIT_FLAGS(4i32);
pub const COWAIT_WAITALL: COWAIT_FLAGS = COWAIT_FLAGS(1i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483648i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483639i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483638i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483637i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483636i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483635i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483634i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483633i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483632i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483631i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483647i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483646i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483645i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483644i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483643i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483642i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483641i32);
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(-2147483640i32);
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = CO_MARSHALING_CONTEXT_ATTRIBUTES(0i32);
pub const CWMO_DEFAULT: CWMO_FLAGS = CWMO_FLAGS(0i32);
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = CWMO_FLAGS(1i32);
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = CWMO_FLAGS(2i32);
pub const CWMO_MAX_HANDLES: u32 = 56u32;
pub const DATADIR_GET: DATADIR = DATADIR(1i32);
pub const DATADIR_SET: DATADIR = DATADIR(2i32);
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = DCOM_CALL_STATE(2i32);
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = DCOM_CALL_STATE(1i32);
pub const DCOM_NONE: DCOM_CALL_STATE = DCOM_CALL_STATE(0i32);
pub const DESCKIND_FUNCDESC: DESCKIND = DESCKIND(1i32);
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = DESCKIND(4i32);
pub const DESCKIND_MAX: DESCKIND = DESCKIND(5i32);
pub const DESCKIND_NONE: DESCKIND = DESCKIND(0i32);
pub const DESCKIND_TYPECOMP: DESCKIND = DESCKIND(3i32);
pub const DESCKIND_VARDESC: DESCKIND = DESCKIND(2i32);
pub const DISPATCH_METHOD: DISPATCH_FLAGS = DISPATCH_FLAGS(1u16);
pub const DISPATCH_PROPERTYGET: DISPATCH_FLAGS = DISPATCH_FLAGS(2u16);
pub const DISPATCH_PROPERTYPUT: DISPATCH_FLAGS = DISPATCH_FLAGS(4u16);
pub const DISPATCH_PROPERTYPUTREF: DISPATCH_FLAGS = DISPATCH_FLAGS(8u16);
pub const DMUS_ERRBASE: u32 = 4096u32;
pub const DVASPECT_CONTENT: DVASPECT = DVASPECT(1u32);
pub const DVASPECT_DOCPRINT: DVASPECT = DVASPECT(8u32);
pub const DVASPECT_ICON: DVASPECT = DVASPECT(4u32);
pub const DVASPECT_OPAQUE: DVASPECT = DVASPECT(16u32);
pub const DVASPECT_THUMBNAIL: DVASPECT = DVASPECT(2u32);
pub const DVASPECT_TRANSPARENT: DVASPECT = DVASPECT(32u32);
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4i32);
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(128i32);
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8i32);
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1024i32);
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2048i32);
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(4096i32);
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16i32);
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(64i32);
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(256i32);
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(1i32);
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(0i32);
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(8192i32);
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(512i32);
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(16384i32);
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(2i32);
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = EOLE_AUTHENTICATION_CAPABILITIES(32i32);
pub const EXTCONN_CALLABLE: EXTCONN = EXTCONN(4i32);
pub const EXTCONN_STRONG: EXTCONN = EXTCONN(1i32);
pub const EXTCONN_WEAK: EXTCONN = EXTCONN(2i32);
pub const FADF_AUTO: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(1u16);
pub const FADF_BSTR: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(256u16);
pub const FADF_DISPATCH: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(1024u16);
pub const FADF_EMBEDDED: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(4u16);
pub const FADF_FIXEDSIZE: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(16u16);
pub const FADF_HAVEIID: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(64u16);
pub const FADF_HAVEVARTYPE: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(128u16);
pub const FADF_RECORD: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(32u16);
pub const FADF_RESERVED: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(61448u16);
pub const FADF_STATIC: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(2u16);
pub const FADF_UNKNOWN: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(512u16);
pub const FADF_VARIANT: ADVANCED_FEATURE_FLAGS = ADVANCED_FEATURE_FLAGS(2048u16);
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = FUNCFLAGS(4u16);
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = FUNCFLAGS(32u16);
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = FUNCFLAGS(256u16);
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = FUNCFLAGS(16u16);
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = FUNCFLAGS(64u16);
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = FUNCFLAGS(4096u16);
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = FUNCFLAGS(1024u16);
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = FUNCFLAGS(2048u16);
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = FUNCFLAGS(8u16);
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = FUNCFLAGS(1u16);
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = FUNCFLAGS(2u16);
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = FUNCFLAGS(512u16);
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = FUNCFLAGS(128u16);
pub const FUNC_DISPATCH: FUNCKIND = FUNCKIND(4i32);
pub const FUNC_NONVIRTUAL: FUNCKIND = FUNCKIND(2i32);
pub const FUNC_PUREVIRTUAL: FUNCKIND = FUNCKIND(1i32);
pub const FUNC_STATIC: FUNCKIND = FUNCKIND(3i32);
pub const FUNC_VIRTUAL: FUNCKIND = FUNCKIND(0i32);
pub const ForcedShutdown: ShutdownType = ShutdownType(1i32);
pub const IDLFLAG_FIN: IDLFLAGS = IDLFLAGS(1u16);
pub const IDLFLAG_FLCID: IDLFLAGS = IDLFLAGS(4u16);
pub const IDLFLAG_FOUT: IDLFLAGS = IDLFLAGS(2u16);
pub const IDLFLAG_FRETVAL: IDLFLAGS = IDLFLAGS(8u16);
pub const IDLFLAG_NONE: IDLFLAGS = IDLFLAGS(0u16);
pub const IMPLTYPEFLAG_FDEFAULT: IMPLTYPEFLAGS = IMPLTYPEFLAGS(1i32);
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(8i32);
pub const IMPLTYPEFLAG_FRESTRICTED: IMPLTYPEFLAGS = IMPLTYPEFLAGS(4i32);
pub const IMPLTYPEFLAG_FSOURCE: IMPLTYPEFLAGS = IMPLTYPEFLAGS(2i32);
pub const INVOKE_FUNC: INVOKEKIND = INVOKEKIND(1i32);
pub const INVOKE_PROPERTYGET: INVOKEKIND = INVOKEKIND(2i32);
pub const INVOKE_PROPERTYPUT: INVOKEKIND = INVOKEKIND(4i32);
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = INVOKEKIND(8i32);
pub const IdleShutdown: ShutdownType = ShutdownType(0i32);
pub const LOCK_EXCLUSIVE: LOCKTYPE = LOCKTYPE(2i32);
pub const LOCK_ONLYONCE: LOCKTYPE = LOCKTYPE(4i32);
pub const LOCK_WRITE: LOCKTYPE = LOCKTYPE(1i32);
pub const LibraryApplication: ApplicationType = ApplicationType(1i32);
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
pub const MEMCTX_MACSYSTEM: MEMCTX = MEMCTX(3i32);
pub const MEMCTX_SAME: MEMCTX = MEMCTX(-2i32);
pub const MEMCTX_SHARED: MEMCTX = MEMCTX(2i32);
pub const MEMCTX_TASK: MEMCTX = MEMCTX(1i32);
pub const MEMCTX_UNKNOWN: MEMCTX = MEMCTX(-1i32);
pub const MKRREDUCE_ALL: MKRREDUCE = MKRREDUCE(0i32);
pub const MKRREDUCE_ONE: MKRREDUCE = MKRREDUCE(196608i32);
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = MKRREDUCE(65536i32);
pub const MKRREDUCE_TOUSER: MKRREDUCE = MKRREDUCE(131072i32);
pub const MKSYS_ANTIMONIKER: MKSYS = MKSYS(3i32);
pub const MKSYS_CLASSMONIKER: MKSYS = MKSYS(7i32);
pub const MKSYS_FILEMONIKER: MKSYS = MKSYS(2i32);
pub const MKSYS_GENERICCOMPOSITE: MKSYS = MKSYS(1i32);
pub const MKSYS_ITEMMONIKER: MKSYS = MKSYS(4i32);
pub const MKSYS_LUAMONIKER: MKSYS = MKSYS(10i32);
pub const MKSYS_NONE: MKSYS = MKSYS(0i32);
pub const MKSYS_OBJREFMONIKER: MKSYS = MKSYS(8i32);
pub const MKSYS_POINTERMONIKER: MKSYS = MKSYS(5i32);
pub const MKSYS_SESSIONMONIKER: MKSYS = MKSYS(9i32);
pub const MSHCTX_CONTAINER: MSHCTX = MSHCTX(5i32);
pub const MSHCTX_CROSSCTX: MSHCTX = MSHCTX(4i32);
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = MSHCTX(2i32);
pub const MSHCTX_INPROC: MSHCTX = MSHCTX(3i32);
pub const MSHCTX_LOCAL: MSHCTX = MSHCTX(0i32);
pub const MSHCTX_NOSHAREDMEM: MSHCTX = MSHCTX(1i32);
pub const MSHLFLAGS_NOPING: MSHLFLAGS = MSHLFLAGS(4i32);
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = MSHLFLAGS(0i32);
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = MSHLFLAGS(8i32);
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = MSHLFLAGS(16i32);
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = MSHLFLAGS(32i32);
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = MSHLFLAGS(64i32);
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = MSHLFLAGS(1i32);
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = MSHLFLAGS(2i32);
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = PENDINGMSG(0i32);
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = PENDINGMSG(2i32);
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = PENDINGMSG(1i32);
pub const PENDINGTYPE_NESTED: PENDINGTYPE = PENDINGTYPE(2i32);
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = PENDINGTYPE(1i32);
pub const REGCLS_AGILE: REGCLS = REGCLS(16i32);
pub const REGCLS_MULTIPLEUSE: REGCLS = REGCLS(1i32);
pub const REGCLS_MULTI_SEPARATE: REGCLS = REGCLS(2i32);
pub const REGCLS_SINGLEUSE: REGCLS = REGCLS(0i32);
pub const REGCLS_SURROGATE: REGCLS = REGCLS(8i32);
pub const REGCLS_SUSPENDED: REGCLS = REGCLS(4i32);
pub const ROTFLAGS_ALLOWANYCLIENT: ROT_FLAGS = ROT_FLAGS(2u32);
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: ROT_FLAGS = ROT_FLAGS(1u32);
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(3u32);
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(2u32);
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(0u32);
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(1u32);
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(4u32);
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(5u32);
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = RPC_C_AUTHN_LEVEL(6u32);
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(1u32);
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(0u32);
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(4u32);
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(2u32);
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = RPC_C_IMP_LEVEL(3u32);
pub const SD_ACCESSPERMISSIONS: COMSD = COMSD(1i32);
pub const SD_ACCESSRESTRICTIONS: COMSD = COMSD(3i32);
pub const SD_LAUNCHPERMISSIONS: COMSD = COMSD(0i32);
pub const SD_LAUNCHRESTRICTIONS: COMSD = COMSD(2i32);
pub const SERVERCALL_ISHANDLED: SERVERCALL = SERVERCALL(0i32);
pub const SERVERCALL_REJECTED: SERVERCALL = SERVERCALL(1i32);
pub const SERVERCALL_RETRYLATER: SERVERCALL = SERVERCALL(2i32);
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(1i32);
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(0i32);
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = RPCOPT_SERVER_LOCALITY_VALUES(2i32);
pub const STATFLAG_DEFAULT: STATFLAG = STATFLAG(0i32);
pub const STATFLAG_NONAME: STATFLAG = STATFLAG(1i32);
pub const STATFLAG_NOOPEN: STATFLAG = STATFLAG(2i32);
pub const STGC_CONSOLIDATE: STGC = STGC(8i32);
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = STGC(4i32);
pub const STGC_DEFAULT: STGC = STGC(0i32);
pub const STGC_ONLYIFCURRENT: STGC = STGC(2i32);
pub const STGC_OVERWRITE: STGC = STGC(1i32);
pub const STGM_CONVERT: STGM = STGM(131072u32);
pub const STGM_CREATE: STGM = STGM(4096u32);
pub const STGM_DELETEONRELEASE: STGM = STGM(67108864u32);
pub const STGM_DIRECT: STGM = STGM(0u32);
pub const STGM_DIRECT_SWMR: STGM = STGM(4194304u32);
pub const STGM_FAILIFTHERE: STGM = STGM(0u32);
pub const STGM_NOSCRATCH: STGM = STGM(1048576u32);
pub const STGM_NOSNAPSHOT: STGM = STGM(2097152u32);
pub const STGM_PRIORITY: STGM = STGM(262144u32);
pub const STGM_READ: STGM = STGM(0u32);
pub const STGM_READWRITE: STGM = STGM(2u32);
pub const STGM_SHARE_DENY_NONE: STGM = STGM(64u32);
pub const STGM_SHARE_DENY_READ: STGM = STGM(48u32);
pub const STGM_SHARE_DENY_WRITE: STGM = STGM(32u32);
pub const STGM_SHARE_EXCLUSIVE: STGM = STGM(16u32);
pub const STGM_SIMPLE: STGM = STGM(134217728u32);
pub const STGM_TRANSACTED: STGM = STGM(65536u32);
pub const STGM_WRITE: STGM = STGM(1u32);
pub const STGTY_LOCKBYTES: STGTY = STGTY(3i32);
pub const STGTY_PROPERTY: STGTY = STGTY(4i32);
pub const STGTY_REPEAT: i32 = 256i32;
pub const STGTY_STORAGE: STGTY = STGTY(1i32);
pub const STGTY_STREAM: STGTY = STGTY(2i32);
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
pub const STREAM_SEEK_CUR: STREAM_SEEK = STREAM_SEEK(1u32);
pub const STREAM_SEEK_END: STREAM_SEEK = STREAM_SEEK(2u32);
pub const STREAM_SEEK_SET: STREAM_SEEK = STREAM_SEEK(0u32);
pub const SYS_MAC: SYSKIND = SYSKIND(2i32);
pub const SYS_WIN16: SYSKIND = SYSKIND(0i32);
pub const SYS_WIN32: SYSKIND = SYSKIND(1i32);
pub const SYS_WIN64: SYSKIND = SYSKIND(3i32);
pub const ServerApplication: ApplicationType = ApplicationType(0i32);
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = THDTYPE(0i32);
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = THDTYPE(1i32);
pub const TKIND_ALIAS: TYPEKIND = TYPEKIND(6i32);
pub const TKIND_COCLASS: TYPEKIND = TYPEKIND(5i32);
pub const TKIND_DISPATCH: TYPEKIND = TYPEKIND(4i32);
pub const TKIND_ENUM: TYPEKIND = TYPEKIND(0i32);
pub const TKIND_INTERFACE: TYPEKIND = TYPEKIND(3i32);
pub const TKIND_MAX: TYPEKIND = TYPEKIND(8i32);
pub const TKIND_MODULE: TYPEKIND = TYPEKIND(2i32);
pub const TKIND_RECORD: TYPEKIND = TYPEKIND(1i32);
pub const TKIND_UNION: TYPEKIND = TYPEKIND(7i32);
pub const TYMED_ENHMF: TYMED = TYMED(64i32);
pub const TYMED_FILE: TYMED = TYMED(2i32);
pub const TYMED_GDI: TYMED = TYMED(16i32);
pub const TYMED_HGLOBAL: TYMED = TYMED(1i32);
pub const TYMED_ISTORAGE: TYMED = TYMED(8i32);
pub const TYMED_ISTREAM: TYMED = TYMED(4i32);
pub const TYMED_MFPICT: TYMED = TYMED(32i32);
pub const TYMED_NULL: TYMED = TYMED(0i32);
pub const TYSPEC_CLSID: TYSPEC = TYSPEC(0i32);
pub const TYSPEC_FILEEXT: TYSPEC = TYSPEC(1i32);
pub const TYSPEC_FILENAME: TYSPEC = TYSPEC(3i32);
pub const TYSPEC_MIMETYPE: TYSPEC = TYSPEC(2i32);
pub const TYSPEC_OBJECTID: TYSPEC = TYSPEC(6i32);
pub const TYSPEC_PACKAGENAME: TYSPEC = TYSPEC(5i32);
pub const TYSPEC_PROGID: TYSPEC = TYSPEC(4i32);
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4u32);
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2u32);
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1u32);
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(256u32);
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(131072u32);
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(512u32);
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(64u32);
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32u32);
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8192u32);
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = URI_CREATE_FLAGS(8u32);
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(65536u32);
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16u32);
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = URI_CREATE_FLAGS(1024u32);
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = URI_CREATE_FLAGS(128u32);
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(32768u32);
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = URI_CREATE_FLAGS(16384u32);
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(4096u32);
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = URI_CREATE_FLAGS(2048u32);
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = Uri_PROPERTY(1i32);
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = Uri_PROPERTY(2i32);
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = Uri_PROPERTY(3i32);
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = Uri_PROPERTY(18i32);
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = Uri_PROPERTY(4i32);
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = Uri_PROPERTY(5i32);
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = Uri_PROPERTY(6i32);
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = Uri_PROPERTY(15i32);
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = Uri_PROPERTY(7i32);
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = Uri_PROPERTY(8i32);
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = Uri_PROPERTY(9i32);
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = Uri_PROPERTY(16i32);
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = Uri_PROPERTY(10i32);
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = Uri_PROPERTY(11i32);
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = Uri_PROPERTY(17i32);
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = Uri_PROPERTY(12i32);
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = Uri_PROPERTY(0i32);
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = Uri_PROPERTY(13i32);
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = Uri_PROPERTY(14i32);
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = Uri_PROPERTY(18i32);
pub const VARFLAG_FBINDABLE: VARFLAGS = VARFLAGS(4u16);
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = VARFLAGS(32u16);
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = VARFLAGS(256u16);
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = VARFLAGS(16u16);
pub const VARFLAG_FHIDDEN: VARFLAGS = VARFLAGS(64u16);
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = VARFLAGS(4096u16);
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = VARFLAGS(1024u16);
pub const VARFLAG_FREADONLY: VARFLAGS = VARFLAGS(1u16);
pub const VARFLAG_FREPLACEABLE: VARFLAGS = VARFLAGS(2048u16);
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = VARFLAGS(8u16);
pub const VARFLAG_FRESTRICTED: VARFLAGS = VARFLAGS(128u16);
pub const VARFLAG_FSOURCE: VARFLAGS = VARFLAGS(2u16);
pub const VARFLAG_FUIDEFAULT: VARFLAGS = VARFLAGS(512u16);
pub const VAR_CONST: VARKIND = VARKIND(2i32);
pub const VAR_DISPATCH: VARKIND = VARKIND(3i32);
pub const VAR_PERINSTANCE: VARKIND = VARKIND(0i32);
pub const VAR_STATIC: VARKIND = VARKIND(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADVANCED_FEATURE_FLAGS(pub u16);
impl windows_core::TypeKind for ADVANCED_FEATURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADVANCED_FEATURE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADVANCED_FEATURE_FLAGS").field(&self.0).finish()
    }
}
impl ADVANCED_FEATURE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ADVANCED_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ADVANCED_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ADVANCED_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ADVF(pub i32);
impl windows_core::TypeKind for ADVF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ADVF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ADVF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APTTYPE(pub i32);
impl windows_core::TypeKind for APTTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APTTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APTTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct APTTYPEQUALIFIER(pub i32);
impl windows_core::TypeKind for APTTYPEQUALIFIER {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for APTTYPEQUALIFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("APTTYPEQUALIFIER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ApplicationType(pub i32);
impl windows_core::TypeKind for ApplicationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ApplicationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ApplicationType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BINDINFOF(pub i32);
impl windows_core::TypeKind for BINDINFOF {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BINDINFOF {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BINDINFOF").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BIND_FLAGS(pub i32);
impl windows_core::TypeKind for BIND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BIND_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BIND_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLCONV(pub i32);
impl windows_core::TypeKind for CALLCONV {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLCONV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLCONV").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CALLTYPE(pub i32);
impl windows_core::TypeKind for CALLTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CALLTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CALLTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CLSCTX(pub u32);
impl windows_core::TypeKind for CLSCTX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CLSCTX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CLSCTX").field(&self.0).finish()
    }
}
impl CLSCTX {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CLSCTX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CLSCTX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CLSCTX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CLSCTX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CLSCTX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COINIT(pub i32);
impl windows_core::TypeKind for COINIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COINIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COINIT").field(&self.0).finish()
    }
}
impl COINIT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COINIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COINIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COINIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COINIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COINIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COINITBASE(pub i32);
impl windows_core::TypeKind for COINITBASE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COINITBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COINITBASE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COMSD(pub i32);
impl windows_core::TypeKind for COMSD {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COMSD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COMSD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct COWAIT_FLAGS(pub i32);
impl windows_core::TypeKind for COWAIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for COWAIT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("COWAIT_FLAGS").field(&self.0).finish()
    }
}
impl COWAIT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for COWAIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for COWAIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for COWAIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for COWAIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for COWAIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CO_MARSHALING_CONTEXT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CWMO_FLAGS(pub i32);
impl windows_core::TypeKind for CWMO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CWMO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CWMO_FLAGS").field(&self.0).finish()
    }
}
impl CWMO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CWMO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CWMO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CWMO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CWMO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CWMO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DATADIR(pub i32);
impl windows_core::TypeKind for DATADIR {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DATADIR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DATADIR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DCOM_CALL_STATE(pub i32);
impl windows_core::TypeKind for DCOM_CALL_STATE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DCOM_CALL_STATE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DCOM_CALL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DESCKIND(pub i32);
impl windows_core::TypeKind for DESCKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DESCKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DESCKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DISPATCH_FLAGS(pub u16);
impl windows_core::TypeKind for DISPATCH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DISPATCH_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DISPATCH_FLAGS").field(&self.0).finish()
    }
}
impl DISPATCH_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DISPATCH_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DISPATCH_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DISPATCH_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DISPATCH_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DISPATCH_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DVASPECT(pub u32);
impl windows_core::TypeKind for DVASPECT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DVASPECT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DVASPECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
impl windows_core::TypeKind for EOLE_AUTHENTICATION_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EOLE_AUTHENTICATION_CAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EOLE_AUTHENTICATION_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EXTCONN(pub i32);
impl windows_core::TypeKind for EXTCONN {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EXTCONN {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EXTCONN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FUNCFLAGS(pub u16);
impl windows_core::TypeKind for FUNCFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FUNCFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FUNCFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FUNCKIND(pub i32);
impl windows_core::TypeKind for FUNCKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FUNCKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FUNCKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_EH_VALUES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GLOBALOPT_EH_VALUES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GLOBALOPT_EH_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GLOBALOPT_PROPERTIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GLOBALOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
impl windows_core::TypeKind for GLOBALOPT_RO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GLOBALOPT_RO_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RO_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_RPCTP_VALUES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GLOBALOPT_RPCTP_VALUES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GLOBALOPT_RPCTP_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GLOBALOPT_UNMARSHALING_POLICY_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IDLFLAGS(pub u16);
impl windows_core::TypeKind for IDLFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IDLFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IDLFLAGS").field(&self.0).finish()
    }
}
impl IDLFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IDLFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IDLFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IDLFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IDLFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IDLFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct IMPLTYPEFLAGS(pub i32);
impl windows_core::TypeKind for IMPLTYPEFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for IMPLTYPEFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("IMPLTYPEFLAGS").field(&self.0).finish()
    }
}
impl IMPLTYPEFLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for IMPLTYPEFLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for IMPLTYPEFLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for IMPLTYPEFLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for IMPLTYPEFLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for IMPLTYPEFLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct INVOKEKIND(pub i32);
impl windows_core::TypeKind for INVOKEKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for INVOKEKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("INVOKEKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LOCKTYPE(pub i32);
impl windows_core::TypeKind for LOCKTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LOCKTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LOCKTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MEMCTX(pub i32);
impl windows_core::TypeKind for MEMCTX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MEMCTX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MEMCTX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MKRREDUCE(pub i32);
impl windows_core::TypeKind for MKRREDUCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MKRREDUCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MKRREDUCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MKSYS(pub i32);
impl windows_core::TypeKind for MKSYS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MKSYS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MKSYS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSHCTX(pub i32);
impl windows_core::TypeKind for MSHCTX {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSHCTX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSHCTX").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MSHLFLAGS(pub i32);
impl windows_core::TypeKind for MSHLFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MSHLFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MSHLFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PENDINGMSG(pub i32);
impl windows_core::TypeKind for PENDINGMSG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PENDINGMSG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PENDINGMSG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PENDINGTYPE(pub i32);
impl windows_core::TypeKind for PENDINGTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PENDINGTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PENDINGTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct REGCLS(pub i32);
impl windows_core::TypeKind for REGCLS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for REGCLS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("REGCLS").field(&self.0).finish()
    }
}
impl REGCLS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for REGCLS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for REGCLS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for REGCLS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for REGCLS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for REGCLS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ROT_FLAGS(pub u32);
impl windows_core::TypeKind for ROT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ROT_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ROT_FLAGS").field(&self.0).finish()
    }
}
impl ROT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ROT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ROT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ROT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ROT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ROT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RPCOPT_PROPERTIES(pub i32);
impl windows_core::TypeKind for RPCOPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RPCOPT_PROPERTIES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RPCOPT_PROPERTIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
impl windows_core::TypeKind for RPCOPT_SERVER_LOCALITY_VALUES {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RPCOPT_SERVER_LOCALITY_VALUES {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RPCOPT_SERVER_LOCALITY_VALUES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
impl windows_core::TypeKind for RPC_C_AUTHN_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RPC_C_AUTHN_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RPC_C_IMP_LEVEL(pub u32);
impl windows_core::TypeKind for RPC_C_IMP_LEVEL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RPC_C_IMP_LEVEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RPC_C_IMP_LEVEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SERVERCALL(pub i32);
impl windows_core::TypeKind for SERVERCALL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SERVERCALL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SERVERCALL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STATFLAG(pub i32);
impl windows_core::TypeKind for STATFLAG {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STATFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STATFLAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STGC(pub i32);
impl windows_core::TypeKind for STGC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STGC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STGC").field(&self.0).finish()
    }
}
impl STGC {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STGC {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STGC {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STGC {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STGC {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STGC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STGM(pub u32);
impl windows_core::TypeKind for STGM {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STGM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STGM").field(&self.0).finish()
    }
}
impl STGM {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STGM {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STGM {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STGM {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STGM {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STGM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STGTY(pub i32);
impl windows_core::TypeKind for STGTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STGTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STGTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct STREAM_SEEK(pub u32);
impl windows_core::TypeKind for STREAM_SEEK {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for STREAM_SEEK {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("STREAM_SEEK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SYSKIND(pub i32);
impl windows_core::TypeKind for SYSKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SYSKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SYSKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ShutdownType(pub i32);
impl windows_core::TypeKind for ShutdownType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ShutdownType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ShutdownType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct THDTYPE(pub i32);
impl windows_core::TypeKind for THDTYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for THDTYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("THDTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TYMED(pub i32);
impl windows_core::TypeKind for TYMED {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TYMED {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TYMED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TYPEKIND(pub i32);
impl windows_core::TypeKind for TYPEKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TYPEKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TYPEKIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TYSPEC(pub i32);
impl windows_core::TypeKind for TYSPEC {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TYSPEC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TYSPEC").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct URI_CREATE_FLAGS(pub u32);
impl windows_core::TypeKind for URI_CREATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for URI_CREATE_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("URI_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl URI_CREATE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for URI_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for URI_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for URI_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for URI_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Uri_PROPERTY(pub i32);
impl windows_core::TypeKind for Uri_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Uri_PROPERTY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Uri_PROPERTY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARFLAGS(pub u16);
impl windows_core::TypeKind for VARFLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARFLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARFLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct VARKIND(pub i32);
impl windows_core::TypeKind for VARKIND {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for VARKIND {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VARKIND").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl windows_core::TypeKind for AUTHENTICATEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: windows_core::GUID,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for BINDINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for BINDINFO {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for BINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: core::mem::ManuallyDrop<Option<ITypeComp>>,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Clone for BINDPTR {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for BINDPTR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl windows_core::TypeKind for BIND_OPTS {
    type TypeKind = windows_core::CopyType;
}
impl Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl windows_core::TypeKind for BIND_OPTS2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
impl windows_core::TypeKind for BIND_OPTS3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl windows_core::TypeKind for BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl windows_core::TypeKind for BYTE_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl windows_core::TypeKind for BYTE_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
impl Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CATEGORYINFO {
    pub catid: windows_core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl windows_core::TypeKind for CATEGORYINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for COAUTHIDENTITY {
    type TypeKind = windows_core::CopyType;
}
impl Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: windows_core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl windows_core::TypeKind for COAUTHINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CONNECTDATA {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub dwCookie: u32,
}
impl Clone for CONNECTDATA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CONNECTDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: windows_core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl windows_core::TypeKind for COSERVERINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_DEVICE_CATALOG_COOKIE(pub isize);
impl CO_DEVICE_CATALOG_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for CO_DEVICE_CATALOG_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CO_DEVICE_CATALOG_COOKIE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CO_MTA_USAGE_COOKIE(pub isize);
impl CO_MTA_USAGE_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl Default for CO_MTA_USAGE_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CO_MTA_USAGE_COOKIE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl windows_core::TypeKind for CSPLATFORM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
impl windows_core::TypeKind for CUSTDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for CUSTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct CUSTDATAITEM {
    pub guid: windows_core::GUID,
    pub varValue: core::mem::ManuallyDrop<windows_core::VARIANT>,
}
impl Clone for CUSTDATAITEM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for CUSTDATAITEM {
    type TypeKind = windows_core::CopyType;
}
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl windows_core::TypeKind for CY {
    type TypeKind = windows_core::CopyType;
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl windows_core::TypeKind for CY_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for CY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for ComCallData {
    type TypeKind = windows_core::CopyType;
}
impl Default for ComCallData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct ContextProperty {
    pub policyId: windows_core::GUID,
    pub flags: u32,
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Clone for ContextProperty {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for ContextProperty {
    type TypeKind = windows_core::CopyType;
}
impl Default for ContextProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DISPPARAMS {
    pub rgvarg: *mut windows_core::VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
impl windows_core::TypeKind for DISPPARAMS {
    type TypeKind = windows_core::CopyType;
}
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl windows_core::TypeKind for DVTARGETDEVICE {
    type TypeKind = windows_core::CopyType;
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl windows_core::TypeKind for DWORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl windows_core::TypeKind for DWORD_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
impl Default for DWORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for ELEMDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for ELEMDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrDescription: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrHelpFile: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl Clone for EXCEPINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for EXCEPINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl windows_core::TypeKind for FLAGGED_BYTE_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl windows_core::TypeKind for FLAGGED_WORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for FLAG_STGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl windows_core::TypeKind for FORMATETC {
    type TypeKind = windows_core::CopyType;
}
impl Default for FORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: FUNCFLAGS,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for FUNCDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for GDI_OBJECT {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for GDI_OBJECT_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl windows_core::TypeKind for HYPER_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
impl Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
impl windows_core::TypeKind for IDLDESC {
    type TypeKind = windows_core::CopyType;
}
impl Default for IDLDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct INTERFACEINFO {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub iid: windows_core::GUID,
    pub wMethod: u16,
}
impl Clone for INTERFACEINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for INTERFACEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct MULTI_QI {
    pub pIID: *const windows_core::GUID,
    pub pItf: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub hr: windows_core::HRESULT,
}
impl Clone for MULTI_QI {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for MULTI_QI {
    type TypeKind = windows_core::CopyType;
}
impl Default for MULTI_QI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MachineGlobalObjectTableRegistrationToken(pub isize);
impl Default for MachineGlobalObjectTableRegistrationToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MachineGlobalObjectTableRegistrationToken {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl windows_core::TypeKind for QUERYCONTEXT {
    type TypeKind = windows_core::CopyType;
}
impl Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl windows_core::TypeKind for RPCOLEMESSAGE {
    type TypeKind = windows_core::CopyType;
}
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl windows_core::TypeKind for RemSTGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl windows_core::TypeKind for SAFEARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl windows_core::TypeKind for SAFEARRAYBOUND {
    type TypeKind = windows_core::CopyType;
}
impl Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SChannelHookCallInfo {
    pub iid: windows_core::GUID,
    pub cbSize: u32,
    pub uCausality: windows_core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SChannelHookCallInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: windows_core::PWSTR,
    pub hr: windows_core::HRESULT,
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_SERVICE {
    type TypeKind = windows_core::CopyType;
}
impl Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: core::mem::ManuallyDrop<Option<IAdviseSink>>,
    pub dwConnection: u32,
}
impl Clone for STATDATA {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for STATDATA {
    type TypeKind = windows_core::CopyType;
}
impl Default for STATDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct STATSTG {
    pub pwcsName: windows_core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: STGM,
    pub grfLocksSupported: u32,
    pub clsid: windows_core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl windows_core::TypeKind for STATSTG {
    type TypeKind = windows_core::CopyType;
}
impl Default for STATSTG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub u: STGMEDIUM_0,
    pub pUnkForRelease: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for STGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: super::super::Foundation::HGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: core::mem::ManuallyDrop<Option<IStream>>,
    pub pstg: core::mem::ManuallyDrop<Option<StructuredStorage::IStorage>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Clone for STGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for STGMEDIUM_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: windows_core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl windows_core::TypeKind for StorageLayout {
    type TypeKind = windows_core::CopyType;
}
impl Default for StorageLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TLIBATTR {
    pub guid: windows_core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl windows_core::TypeKind for TLIBATTR {
    type TypeKind = windows_core::CopyType;
}
impl Default for TLIBATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct TYPEATTR {
    pub guid: windows_core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: windows_core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEATTR {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::Variant::VARENUM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: windows_core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for VARDESC {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut windows_core::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for VARDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl windows_core::TypeKind for WORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
impl Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl windows_core::TypeKind for WORD_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
impl Default for WORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl windows_core::TypeKind for uCLSSPEC {
    type TypeKind = windows_core::CopyType;
}
impl Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union uCLSSPEC_0 {
    pub clsid: windows_core::GUID,
    pub pFileExt: windows_core::PWSTR,
    pub pMimeType: windows_core::PWSTR,
    pub pProgId: windows_core::PWSTR,
    pub pFileName: windows_core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl windows_core::TypeKind for uCLSSPEC_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: windows_core::PWSTR,
    pub PolicyId: windows_core::GUID,
}
impl windows_core::TypeKind for uCLSSPEC_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_core::GUID,
    pub PolicyId: windows_core::GUID,
}
impl windows_core::TypeKind for uCLSSPEC_0_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Clone for userFLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userFLAG_STGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
pub struct userSTGMEDIUM {
    pub u: userSTGMEDIUM_0,
    pub pUnkForRelease: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Clone for userSTGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy)]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPEXCEPFINO_DEFERRED_FILLIN = Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> windows_core::HRESULT>;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFNCONTEXTCALL = Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> windows_core::HRESULT>;
impl From<IDispatch> for windows_core::VARIANT {
    fn from(value: IDispatch) -> Self {
        unsafe {
            Self::from_raw(windows_core::imp::VARIANT {
                Anonymous: windows_core::imp::VARIANT_0 {
                    Anonymous: windows_core::imp::VARIANT_0_0 { vt: 9, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: windows_core::imp::VARIANT_0_0_0 { pdispVal: core::mem::transmute(value) } },
                },
            })
        }
    }
}

impl From<IDispatch> for windows_core::PROPVARIANT {
    fn from(value: IDispatch) -> Self {
        unsafe {
            Self::from_raw(windows_core::imp::PROPVARIANT {
                Anonymous: windows_core::imp::PROPVARIANT_0 {
                    Anonymous: windows_core::imp::PROPVARIANT_0_0 { vt: 9, wReserved1: 0, wReserved2: 0, wReserved3: 0, Anonymous: windows_core::imp::PROPVARIANT_0_0_0 { pdispVal: core::mem::transmute(value) } },
                },
            })
        }
    }
}

impl TryFrom<&windows_core::VARIANT> for IDispatch {
    type Error = windows_core::Error;
    fn try_from(from: &windows_core::VARIANT) -> windows_core::Result<Self> {
        let from = from.as_raw();
        unsafe {
            if from.Anonymous.Anonymous.vt == 9 && !from.Anonymous.Anonymous.Anonymous.pdispVal.is_null() {
                let dispatch: &IDispatch = core::mem::transmute(&from.Anonymous.Anonymous.Anonymous.pdispVal);
                Ok(dispatch.clone())
            } else {
                Err(windows_core::Error::from_hresult(windows_core::imp::TYPE_E_TYPEMISMATCH))
            }
        }
    }
}

impl TryFrom<&windows_core::PROPVARIANT> for IDispatch {
    type Error = windows_core::Error;
    fn try_from(from: &windows_core::PROPVARIANT) -> windows_core::Result<Self> {
        let from = from.as_raw();
        unsafe {
            if from.Anonymous.Anonymous.vt == 9 && !from.Anonymous.Anonymous.Anonymous.pdispVal.is_null() {
                let dispatch: &IDispatch = core::mem::transmute(&from.Anonymous.Anonymous.Anonymous.pdispVal);
                Ok(dispatch.clone())
            } else {
                Err(windows_core::Error::from_hresult(windows_core::imp::TYPE_E_TYPEMISMATCH))
            }
        }
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
